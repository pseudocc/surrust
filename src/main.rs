mod db;
mod surrust;

use clap::{ArgGroup, Parser, ColorChoice};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(name = "Surrust server")]
#[command(color = ColorChoice::Auto)]
#[command(about = "A blazingly fast and highly configurable Quiz Site")]
#[command(version, group(cli_db_group()))]
struct Cli {
	/// Database path used for storing data
	#[arg(long, value_name = "PATH")]
	db_path: Option<PathBuf>,

	/// Use a in-memory database
	#[arg(long)]
	memory: bool,
}

fn cli_db_group() -> ArgGroup {
	ArgGroup::new("db")
		.required(true)
		.args(["db_path", "memory"])
}

#[tokio::main]
async fn main() {
	let cli = Cli::parse();
	println!("{:?}", cli);

	let settings = surrust::Surrust::settings();
	let inst_result = db::DbInstance::new(&settings.db_path).await;
	let inst = match inst_result {
		Ok(i) => i,
		Err(e) => panic!("Problem initializing the DbInstance: {:?}", e),
	};
}
// vim: noet:ts=4:sts=8
