#[cfg(windows)]
fn set_exe_icon() {
    let mut res = winres::WindowsResource::new();
    res.set_icon("assets/icon.ico");
    res.compile().unwrap();
}

#[cfg(not(windows))]
fn set_exe_icon() {
    // NOTE: do nothing. We're not on Windos so we're not going to set
    // the icon.
}

fn main() {
    set_exe_icon();

    // Make sure we build for this version:
    //
    // This is necessary for cargo bundle
    //
    // NOTE: if you want to change it, update the `osx_minimum_system_version`
    // field in `Cargo.toml` as well!
    println!("cargo:rustc-env=MACOSX_DEPLOYMENT_TARGET=10.12");
}
