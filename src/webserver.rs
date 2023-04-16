use sqlite::State;
use std::{
    io::{prelude::*},
    net::{TcpListener, TcpStream},
};

pub async fn webserver(db_name: String) {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(&db_name, stream).await;
    }
}

async fn handle_connection(db_name: &String, mut stream: TcpStream) {
    let db_connection = sqlite::Connection::open(&db_name).unwrap();
    let query = "SELECT * FROM RoutesDuration ORDER BY timestamp ASC";
    let mut statement = db_connection.prepare(query).unwrap();

    let status_line = "HTTP/1.1 200 OK";
    let mut contents = String::from(r#"<!DOCTYPE html>
<html lang="en">
  <head>
    <meta charset="utf-8">
    <title>Quickest road to work</title>
  </head>
  <body>
    <table style="width:100%">
      <tr>
        <th>Date</th>
        <th>Route</th>
        <th>Duration</th>
      </tr>"#);

    while let Ok(State::Row) = statement.next() {
        contents.push_str("<tr>");
        let timestamp = statement.read::<String, _>("timestamp").unwrap();
        let route_name = statement.read::<String, _>("name").unwrap();
        let route_duration = statement.read::<i64, _>("duration").unwrap();
        let data_row = format!("<tr><td>{}</td><td>{}</td><td>{}</td></tr>", timestamp, route_name, route_duration);
        contents.push_str(&data_row);
    }

    contents.push_str(
        r#"
  </table>
  </body>
</html>"#);
    let length = contents.len();

    let response =
        format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

    stream.write_all(response.as_bytes()).unwrap();
}