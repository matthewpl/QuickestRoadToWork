use sqlite;

pub async fn create_database() -> String {
    let db_name = String::from("routes.db");
    let database_connection = sqlite::Connection::open(&db_name).unwrap();

    let create_table = "CREATE TABLE IF NOT EXISTS RoutesDuration (id INTEGER PRIMARY KEY, name TEXT, duration INTEGER, timestamp DATETIME DEFAULT CURRENT_TIMESTAMP);";

    database_connection.execute(create_table).unwrap();

    db_name
}
