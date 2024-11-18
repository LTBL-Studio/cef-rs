#[cfg(not(feature = "dox"))]
fn main() {
    #[cfg(target_os = "windows")]
    let path = match std::env::var("CEF_PATH") {
        Ok(val) => val,
        Err(_) => panic!("Couldn't get the path of shared library: {e}"),
    };

    #[cfg(not(target_os = "windows"))]
    if std::env::var("FLATPAK").is_ok() {
        println!("cargo:rustc-link-search=/usr/lib");
    }

    #[cfg(not(target_os = "windows"))]
    println!("cargo:rustc-link-lib=cef");
    #[cfg(target_os = "windows")]
    {
        println!("cargo:rustc-link-search={path}");
        println!("cargo:rustc-link-lib=libcef");
    }
}

#[cfg(feature = "dox")]
fn main() {}
