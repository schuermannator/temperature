use chrono::prelude::*;
use eventsource::reqwest::Client;
use reqwest::Url;
use serde::{Deserialize, Serialize};
use serde_json::Result;
use std::thread;
#[macro_use]
extern crate rocket;
use rocket::State;
use rocket::response::NamedFile;
use std::path::Path;
use std::sync::{Arc, Mutex};

#[get("/")]
async fn index() -> NamedFile {
    NamedFile::open(Path::new("index.html"))
        .await
        .expect("index.html")
}

#[get("/dist/uPlot.iife.js")]
async fn uplot_js() -> NamedFile {
    NamedFile::open(Path::new("dist/uPlot.iife.js"))
        .await
        .expect("/dist/uPlot.iife.js")
}

#[get("/dist/uPlot.min.css")]
async fn uplot_css() -> NamedFile {
    NamedFile::open(Path::new("dist/uPlot.min.css"))
        .await
        .expect("/dist/uPlot.min.css")
}

#[get("/data")]
fn data(db: State<DB>) -> String {
    serde_json::to_string(&*db.0.lock().unwrap()).unwrap()
}

#[launch]
fn rocket() -> rocket::Rocket {
    let database: DB = DB(Arc::new(Mutex::new(Data {
        times: vec![],
        temps: vec![],
    })));
    let mut db = DB(database.0.clone());
    thread::spawn(move || db.scrape());
    rocket::ignite()
        .mount("/", routes![index, data, uplot_js, uplot_css])
        .manage(database)
}

struct DB(Arc<Mutex<Data>>);

#[derive(Serialize, Deserialize)]
struct Event {
    data: String,
    ttl: u32,
    published_at: String,
    coreid: String,
}

#[derive(Serialize, Deserialize)]
struct Data {
    times: Vec<i64>,
    temps: Vec<f32>,
}

impl DB {
    fn save(&mut self, e: Event) {
        let mut me = self.0.lock().unwrap();
        let datetime = e.published_at.parse::<DateTime<Local>>().unwrap();
        let temp: f32 = e.data.parse().unwrap();
        me.times.push(datetime.timestamp());
        me.temps.push(temp);
    }
    fn scrape(&mut self) -> Result<()> {
        let url = "https://api.particle.io/v1/devices/40003f000547353138383138/events?access_token=c8707351c482312444480faf5abe507a88d67584";
        let client = Client::new(Url::parse(url).unwrap());

        for event in client {
            let event = event.unwrap();
            if event.is_empty() {
                continue;
            }
            let e: Event = serde_json::from_str(event.data.as_str())?;
            //println!("{}", e.published_at);
            //println!("{}", e.data);
            self.save(e);
            //println!("{:?}", self.0.lock().unwrap().times);
            //println!("{:?}", self.0.lock().unwrap().temps);
        }
        Ok(())
    }
}

