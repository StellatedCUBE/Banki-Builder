fn main() {
    slint_build::compile("ui/app-window.slint").expect("Slint build failed");

    if std::env::var_os("CARGO_CFG_WINDOWS").is_some() {
        winresource::WindowsResource::new().set_icon("icon.ico").compile().expect("Icon assign failed");
    }
}
