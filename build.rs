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
}
