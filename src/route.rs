use futures::future;
use reqwest::header::CONTENT_TYPE;
use reqwest::header::HeaderValue;
use std::time::Duration;
use tokio::time::sleep;
use serde_json::Value;
use crate::data;

pub async fn get_route(db_name: &String, api_key: &str, routes: Vec<data::RouteConfig>) {
    let db_connection = sqlite::Connection::open(&db_name).unwrap();
    let client = reqwest::Client::new();

    loop {
        let routes = &routes;
        let responses = future::join_all(routes.into_iter().map(|route| {
            let client = &client;
            async move {
                let mut route_time: data::RouteTime = data::RouteTime { name: "".to_string(), time: "".to_string() };
                route_time.name = route.name.clone();
                let resp = client.post("https://routes.googleapis.com/directions/v2:computeRoutes")
                    .header(CONTENT_TYPE, HeaderValue::from_static("application/json"))
                    .header("X-Goog-Api-Key", api_key)
                    .header("X-Goog-FieldMask", "routes.duration")
                    .body(route.request.clone()).send().await;
                match resp {
                    Ok(r) => {
                        let v = r.json::<Value>().await.unwrap();
                        route_time.time = v["routes"].as_array().unwrap().get(0).unwrap()["duration"].as_str().unwrap().to_string();
                    },
                    Err(e) => eprintln!("Got an error: {}", e),
                }
                route_time
            }
        })).await;

        for out in responses {
            let time = &out.time[0..out.time.len() - 1];
            let duration: i32 = time.parse().unwrap_or(0);
            let query = format!("INSERT INTO RoutesDuration (name, duration) VALUES ('{}', '{}')", out.name, duration);
            db_connection.execute(query).unwrap();
        }

        println!("Sleeping for 5 minutes");
        sleep(Duration::from_secs(60 * 5)).await;
    }
}