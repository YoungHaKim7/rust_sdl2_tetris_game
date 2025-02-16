# Original Code
- (Dec 3, 2018)WindowsOS(Visual Studio)로 개발한 프로젝트라 (윈도우 개발자라면 참고하기 좋다.)
  - https://github.com/bigOconstant/RustTetris/

# 윈도우 개발시 `build.rs` & `Cargo.toml`세팅
- https://github.com/bigOconstant/RustTetris/

- `Cargo.toml`
```toml
[package]
name = "rusttetris"
version = "0.1.0"
authors = ["Caleb McCarthy <caleb.a.mccarthy@gmail.com>"]
build="build.rs"

[dependencies]
serde = "1.0"
serde_derive = "1.0"
serde_json = "1.0"
lazy_static = "1.1.0"
rand = "0.3"
sdl2 = { version = "0.32.0", features = ["ttf","image"] }
```


- `build.rs`
```rs
use std::env;
use std::path::PathBuf;

fn main() {
    let target = env::var("TARGET").unwrap();
    if target.contains("pc-windows") {
        let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
        let mut lib_dir = manifest_dir.clone();
        let mut dll_dir = manifest_dir.clone();
        if target.contains("msvc") {
            lib_dir.push("msvc");
            dll_dir.push("msvc");
        }
        else {
            lib_dir.push("gnu-mingw");
            dll_dir.push("gnu-mingw");
        }
        lib_dir.push("lib");
        dll_dir.push("dll");
        if target.contains("x86_64") {
            lib_dir.push("64");
            dll_dir.push("64");
        }
        else {
            lib_dir.push("32");
            dll_dir.push("32");
        }
        println!("cargo:rustc-link-search=all={}", lib_dir.display());
        for entry in std::fs::read_dir(dll_dir).expect("Can't read DLL dir")  {
            let entry_path = entry.expect("Invalid fs entry").path();
            let file_name_result = entry_path.file_name();
            let mut new_file_path = manifest_dir.clone();
            if let Some(file_name) = file_name_result {
                let file_name = file_name.to_str().unwrap();
                if file_name.ends_with(".dll") {
                    new_file_path.push(file_name);
                    std::fs::copy(&entry_path, new_file_path.as_path()).expect("Can't copy from DLL dir");
                }
            }
        }
    }
}
```
