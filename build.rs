use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

struct ScadAndModel<'a> {
  scad: &'a Path,
  stl: &'a Path,
}

fn needs_rebuild(scad_and_model: ScadAndModel) -> bool {
  let src_time = fs::metadata(scad_and_model.scad).and_then(|m| m.modified());
  let dst_time = fs::metadata(scad_and_model.stl).and_then(|m| m.modified());

  match (src_time, dst_time) {
    (Ok(src), Ok(dst)) => src > dst,
    _ => true, // regenerate if missing or unreadable
  }
}

fn main() {
  println!("running the script ...");
  let scad_dir = Path::new("assets/stl");
  track_scad_files(scad_dir);
  
  let scad_files = match fs::read_dir(scad_dir) {
    Ok(entries) => entries
      .filter_map(Result::ok)
      .map(|e| e.path())
      .filter(|p| p.extension().and_then(|ext| ext.to_str()) == Some("scad"))
      .collect::<Vec<PathBuf>>(),
    Err(err) => {
      panic!("Failed to read directory {:?}: {}", scad_dir, err);
    }
  };

  for scad_path in scad_files {
    let stl_path = scad_path.with_extension("stl");

    println!("cargo:rerun-if-changed={}", scad_path.display());

    if needs_rebuild(ScadAndModel {
      scad: &scad_path,
      stl: &stl_path,
    }) {
      println!("Generating {}", stl_path.display());
      let status = Command::new("openscad")
        .args(["-o", stl_path.to_str().unwrap(), scad_path.to_str().unwrap()])
        .status()
        .expect("Failed to run OpenSCAD");

      if !status.success() {
        panic!("OpenSCAD failed on {:?}", scad_path);
      }
    } else {
      println!("Up-to-date: {}", stl_path.display());
    }
  }
}

fn track_scad_files(dir: &Path) {
    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.filter_map(Result::ok) {
            let path = entry.path();
            if path.is_dir() {
                track_scad_files(&path);
            } else if path.extension().and_then(|e| e.to_str()) == Some("scad") {
                println!("cargo:rerun-if-changed={}", path.display());
            }
        }
    }
}