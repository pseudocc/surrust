mod db;
mod surrust;

#[tokio::main]
async fn main() {
	let settings = surrust::Surrust::settings();
	let inst_result = db::DbInstance::new(&settings.db_path).await;
	let inst = match inst_result {
		Ok(i) => i,
		Err(e) => panic!("Problem initializing the DbInstance: {:?}", e),
	};
}
// vim: noet:ts=4:sts=8
