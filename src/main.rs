use chrono::DateTime;
use serde::{Deserialize, Serialize};
use std::fs::File;

#[derive(Debug, Deserialize, Serialize)]
struct Record {
    account_id: i32,
    name: String,
    date: String,
    account_url: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct Res {
    success: bool,
    data: Vec<Record>,
}

fn parse_results(challenge: i32) {
    let file = File::open(format!("json/{}.json", challenge)).unwrap();
    let result: Res = serde_json::from_reader(file).unwrap();

    let mut count = 0;

    for x in result.data {
        let date = x.date.clone();

        let dt = DateTime::parse_from_rfc3339(date.as_str()).unwrap();
        let dt2 = DateTime::parse_from_rfc3339(date.as_str())
            .unwrap()
            .date()
            .and_hms(7, 0, 0);
        //let duration = dt.signed_duration_since(dt2).to_std();

        let sm = dt.timestamp_millis();
        let st = dt2.timestamp_millis();

        println!(
            "2020\t{:?}\t1\t0\t{}\t{}",
            challenge,
            x.name,
            (sm - st) / 1000
        );

        count += 1;
        if count >= 20 {
            break;
        }
    }
}

fn main() {
    println!("year	day	stars	position	name	seconds");

    for i in 1..15 {
        parse_results(i);
    }
}
