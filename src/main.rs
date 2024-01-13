use clap::App;
fn main() {
  let _matches = clap::App::new("echo")
      .version("0.1.0")
      .author("ryan")
      .about("Rust")
      .get_matches();
}
