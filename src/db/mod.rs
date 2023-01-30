mod service;

use log::{info, log_enabled, trace};
use nameof::name_of;
use service::ServiceModel;
use std::collections::BTreeMap as Map;
use surrealdb::{sql, Datastore, Error, Session};

pub(in crate::db) struct DbInitObject {
	thing: sql::Thing,
	service: bool,
}

pub(in crate::db) trait DbInit {
	fn need_init(&self, init_obj: &DbInitObject) -> Option<sql::Query>;
}

pub(crate) struct DbInstance {
	pub(in crate::db) store: Datastore,
	pub(in crate::db) session: Session,
	pub service: ServiceModel,
}

impl DbInitObject {
	pub const TABLE: &str = "init";
	pub const IDENT: &str = "surrust";
	pub const THING: &str = "thing";

	fn thing_vars(thing: &sql::Thing) -> Option<Map<String, sql::Value>> {
		let vars = Map::from([(
			String::from(Self::THING),
			sql::Value::Thing(thing.clone()),
		)]);
		Some(vars)
	}

	pub async fn new(
		store: &Datastore,
		session: &Session,
	) -> Result<DbInitObject, Error> {
		let thing = sql::Thing {
			tb: Self::TABLE.to_string(),
			id: sql::Id::String(Self::IDENT.to_string()),
		};
		let mut service = false;

		let sql = format!("SELECT * FROM ${}", Self::THING);
		let query = sql::parse(sql.as_str())?;
		let vars = Self::thing_vars(&thing);
		let mut resp = store.process(query, session, vars, false).await?;

		while let Some(resp0) = resp.pop() {
			let sql::Value::Array(sql::Array(result)) = resp0.result? else {
				panic!("malformed database response");
			};
			if result.len() == 0 {
				let sql = format!("CREATE ${}", Self::THING);
				let query = sql::parse(sql.as_str())?;
				let vars = Self::thing_vars(&thing);
				resp = store.process(query, session, vars, false).await?;
				continue;
			}
			let sql::Value::Object(init_obj) = &result[0] else {
				panic!("malformed database response: not an object");
			};
			
			if log_enabled!(log::Level::Info) {
				let json = serde_json::to_string_pretty(init_obj).unwrap();
				info!("{}: {}", name_of!(type DbInitObject), json);
			}

			service = init_obj.get(name_of!(service)).map_or(false, |v| v.is_true());
			break;
		}

		let init_obj = DbInitObject { thing, service };
		Ok(init_obj)
	}
}

impl DbInstance {
	pub const NAMESPACE: &str = "surrust";
	pub const DATABASE: &str = "develop";

	pub async fn new(path: &String) -> Result<DbInstance, surrealdb::Error> {
		let store = Datastore::new(path.as_str()).await?;
		let session = Session::for_db(Self::NAMESPACE, Self::DATABASE);
		let init_obj = DbInitObject::new(&store, &session).await?;

		let service = ServiceModel::new();

		let begin = unwrap_query(sql::parse("BEGIN")?);
		let commit = unwrap_query(sql::parse("COMMIT")?);

		let db_models: Map<&str, Box<&dyn DbInit>> =
			Map::from([(name_of!(service), Box::<&dyn DbInit>::new(&service))]);
		for (name, model) in db_models {
			let Some(query) = model.need_init(&init_obj) else {
				continue;
			};
			let chain = [begin.clone(), unwrap_query(query), commit.clone()];
			let query = sql::Query(sql::Statements(chain.concat().into()));
			let resp = store.process(query, &session, None, false).await?;

			if log_enabled!(log::Level::Trace) {
				let json = serde_json::to_string_pretty(&resp).unwrap();
				trace!("db_init->{}: {}", name, json);
			}
		}

		let instance = DbInstance {
			store,
			session,
			service,
		};

		Ok(instance)
	}
}

fn unwrap_query(value: sql::Query) -> Vec<sql::Statement> {
	value.0 .0
}

// vim: noet:ts=4:sts=8
