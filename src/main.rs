use std::fs;
use tokio::task;

mod database;
mod webserver;
mod route;
mod data;

#[tokio::main]
async fn main() {
    let config = String::from("config.json");

    let db_name = database::create_database().await;

    let configuration = {
        let config_file = fs::read_to_string(config).expect("QuickestRoadToWork: problem with reading config file");
        serde_json::from_str::<serde_json::Value>(&config_file).expect("QuickestRoadToWork: problem with serializing config file to JSON")
    };

    let api_key = configuration["api_key"].as_str().unwrap();

    let mut routes: Vec<data::RouteConfig> = Vec::new();

    for route_config in configuration["routes"].as_array().unwrap() {
        let mut route: data::RouteConfig = data::RouteConfig { name: "".to_string(), request: "".to_string() };
        route.name = route_config["name"].as_str().unwrap().to_string();
        route.request = serde_json::to_string(&route_config["request"]).expect("JSON serialization failed");
        routes.push(route);
    }

    let db_name2 = db_name.clone();
    task::spawn(async {
        webserver::webserver(db_name2).await
    });

    route::get_route(&db_name, api_key, routes).await;
}
