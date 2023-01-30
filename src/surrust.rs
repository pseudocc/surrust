use env_logger;

pub(crate) struct Surrust {
	pub db_path: String,
}

impl Surrust {
	pub fn settings() -> Surrust {
		env_logger::init();

		let db_path = String::from("memory");
		Surrust { db_path }
	}
}

// vim: noet:ts=4:sts=8
