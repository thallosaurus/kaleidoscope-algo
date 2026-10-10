/// Compiles the WESL shader artifact for this example.
fn main() {
    wesl::Wesl::new("src/wesl").build_artifact(&"package::main".parse().unwrap(), "my_shader"); 
}