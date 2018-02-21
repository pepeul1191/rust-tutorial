pub fn hello() -> String {
 	"Hello!".to_string()
}

pub fn macos_only() {
	macos_priv();
  println!("macos_only");
}

fn macos_priv() {
  println!("macos_priv");
}