use indoc::indoc;
use surrealdb::sql;

use super::{DbInit, DbInitObject};

pub(crate) struct ServiceModel;

impl DbInit for ServiceModel {
	fn need_init(&self, init_obj: &DbInitObject) -> Option<sql::Query> {
		if init_obj.service {
			return None;
		}

		let query = sql::parse(indoc!{r#"
			DEFINE TABLE service SCHEMAFULL;
			DEFINE FIELD name ON service TYPE string;
			DEFINE FIELD purpose ON service TYPE string;
			DEFINE FIELD options ON service TYPE array;
			DEFINE FIELD created ON service TYPE datetime;

			DEFINE EVENT service_created ON service
				WHEN $before = NONE
				THEN ( UPDATE $after SET created = time::now() )
		"#}).unwrap();

		Some(query)
	}
}

impl ServiceModel {
	pub fn new() -> Self {
		ServiceModel
	}
}

// vim: noet:ts=4:sts=8
