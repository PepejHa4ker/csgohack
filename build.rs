use winres::WindowsResource;

fn main() {
    let mut res = WindowsResource::new();
    res.set_icon("app.ico")

        .compile()
        .expect("Failed to compile binary file.");
}