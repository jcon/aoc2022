pub struct Cli {
  pub path: std::path::PathBuf,
}

// NOTE: could use something like clap instead, but wanted to use only standard rust
// where possible for this.
impl Cli {
  pub fn parse() -> Self {
      let path = std::env::args().nth(1).expect("no path given");
      Cli {
          path: std::path::PathBuf::from(path),
      }
  }
}
