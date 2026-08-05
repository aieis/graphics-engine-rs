use std::{ffi::OsStr, path::{Path, PathBuf}};

fn main() {
    let cargo_manifest_dir: PathBuf = PathBuf::from(std::env::var_os("CARGO_MANIFEST_DIR").unwrap());

	if let Err(_) = std::process::Command::new("glslc").spawn() {
		println!("cargo:warning=Could not spawn glslc. You may not be able to use the program without it.");
		return;
	}

	let target_dir = cargo_manifest_dir.join("./assets/shaders");

	let files = read_files(&target_dir);


	let mut unbuilt_shaders = 0;

	for file in files {
		if !extension_is(&file, OsStr::new("vert")) && !extension_is(&file, OsStr::new("frag")) {
			continue;
		}

		let target_spv_file = append_to_path(file.clone(), ".spv");

		if !target_spv_file.exists() {
			unbuilt_shaders += 1;

			let mut cmd = std::process::Command::new("glslc");
			cmd.arg(&file)
				.arg("-o")
				.arg(&target_spv_file);

			match cmd.spawn() {
				Ok(_) => println!("cargo:info=Shader built successfully: {:?}.", &file),
				Err(err) => println!("cargo:warning=Failed to build shader: {:?} \n\t {}", &file, err),
			}
		}
	}

	if unbuilt_shaders == 0 {
		println!("cargo:info=No shaders to build");
	}

}


fn read_files(path: &Path) -> Vec<PathBuf> {
	match std::fs::read_dir(path) {
		Ok(path_results) => {
			let mut paths = Vec::new();
			for path in path_results {
				if let Ok(path) = path {
					paths.push(path.path());
				}
			}

			paths
		},

		Err(err) => {
			println!("Err: {}", err);
			Vec::new()
		}
	}
}

fn append_to_path(p: PathBuf, s: &str) -> PathBuf {
    let mut p = p.into_os_string();
    p.push(s);
    p.into()
}


fn extension_is(p: &Path, s: &OsStr) -> bool {

	match p.extension() {
		Some(ext) => ext == s,
		None => s == ""
	}

}
