extern crate winres;

#[cfg(windows)]
fn main() {
    let mut res = winres::WindowsResource::new();
    res.set_icon("app.ico")
        .set_language(0x0409)
        .compile()
        .expect("Error while compiling binary file");
}
