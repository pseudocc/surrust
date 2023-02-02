use clap::{ArgGroup, ColorChoice, Parser};
use env_logger;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(name = "Surrust server", version)]
#[command(color = ColorChoice::Auto)]
#[command(about = "A blazingly fast and highly configurable quiz site.")]
#[command(group(cli_db_group()))]
#[command(group(cli_surreal_group()))]
struct Cli {
	/// Database path used for storing data
	#[arg(long, value_name = "PATH")]
	db_path: Option<PathBuf>,

	/// Use a in-memory database
	#[arg(long)]
	memory: bool,

	/// SurrealDB: USE NS @ns
	#[arg(long, default_value(DbSettings::NAMESPACE))]
	ns: Option<String>,

	/// SurrealDB: USE DB @db
	#[arg(long, default_value(DbSettings::DATABASE))]
	db: Option<String>,
}

fn cli_db_group() -> ArgGroup {
	ArgGroup::new("db_kind")
		.required(true)
		.args(["db_path", "memory"])
}

fn cli_surreal_group() -> ArgGroup {
	ArgGroup::new("surreal")
		.required(false)
		.args(["ns", "db"])
		.multiple(true)
}

pub(crate) struct Surrust {
	pub db: DbSettings,
}

pub(crate) struct DbSettings {
	pub kind: DbKind,
	ns: Option<String>,
	db: Option<String>,
}

impl DbSettings {
	pub const NAMESPACE: &str = "surrust";
	pub const DATABASE: &str = "develop";

	pub(crate) fn namespace(&self) -> &str {
		self.ns.as_ref().map_or(Self::NAMESPACE, String::as_str)
	}

	pub(crate) fn database(&self) -> &str {
		self.db.as_ref().map_or(Self::DATABASE, String::as_str)
	}
}

pub(crate) enum DbKind {
	File(PathBuf),
	Memory,
}

impl Surrust {
	/// Parse the command line arguments and return the project level settings.
	pub fn settings() -> Surrust {
		let cli = Cli::parse();
		env_logger::init();

		let db_kind = if cli.memory {
			DbKind::Memory
		} else if let Some(path) = cli.db_path {
			DbKind::File(path)
		} else {
			panic!("should never be reached, might consider filing a bug to clap");
		};

		Surrust {
			db: DbSettings {
				kind: db_kind,
				ns: cli.ns,
				db: cli.db,
			},
		}
	}
}

// vim: noet:ts=4:sts=8
