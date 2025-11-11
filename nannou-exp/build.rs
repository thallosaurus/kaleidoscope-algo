fn main() {
    wesl::Wesl::new("src/shaders").build_artifact(&"package::fs".parse().unwrap(), "my_shader"); 
}