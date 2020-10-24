//Replace this with the actual token and prefix
//or, you know, don't push this to github
mod private{
	pub const TOKEN: &'static str = "TOKEN HERE";
	pub const PREFIX: &'static str = "!";
}

pub struct Config {
  token: &'static str,
  prefix: &'static str,
}

impl Config {
	pub const fn new() -> Self{
		return Config {
			token:  private::TOKEN,
			prefix: private::PREFIX,
		}
	}

	pub fn token(&self) -> &'static str {
		return self.token;
	}

	pub fn prefix(&self) -> &'static str {
		return self.prefix;
	}
}