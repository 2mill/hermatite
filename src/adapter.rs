use serde_json;
use std::future::Future;
use reqwest;


struct Item {
	name: String,
	id: u32
}
struct ItemList {
	item_list: Vec<Item>,
}

impl ItemList {
	fn from(item_list: Vec<Item>) -> Self {
		ItemList { item_list }
	}
}


enum TimeseriesTime {
	FIVE_MIN,
	ONE_HOUR,
	SIX_HOURS
}

enum TimestampTime {
	FIVE_MIN,
	ONE_HOUR,
}
pub enum Endpoint {
	Mapping,
	Latest(Option<u32>),
	Timeseries(TimeseriesTime, u32),
	Timestamp(TimestampTime)
}







pub struct Config {
	endpoint: Endpoint,
	user_agent: String,
}

impl Config {
	pub fn new(endpoint: Endpoint, user_agent: String) -> Self {
		Config { endpoint, user_agent }
	}
}

const BASE: String = String::from("https://prices.runescape.wiki/api/v1/osrs");



trait Adapter {
	fn get_mapping() -> reqwest::blocking::Response;
	fn get_timeseries(id: u32, interval: TimeseriesTime) -> reqwest::blocking::Response;
	fn get_timestamp(timestamp: TimestampTime) -> reqwest::blocking::Response;
	fn get_latest(id: Option<u32>) -> reqwest::blocking::Response;
}



struct Adaptee {
	user_agent: String,
}

impl Adaptee {
	fn new(user_agent: String) -> Self {
		Adaptee { user_agent }	
	}

}

impl Adapter for Adaptee {
	fn get_mapping() -> reqwest::blocking::Response {
		reqwest::blocking::get(format!("{}/mapping", BASE)).unwrap()
	}

	fn get_timeseries(id: u32, interval: TimeseriesTime) -> reqwest::blocking::Response {
		let time = match interval {
			TimeseriesTime::FIVE_MIN => "5m",
			TimeseriesTime::ONE_HOUR => "1h",
			TimeseriesTime::SIX_HOURS => "6h"
		};

		reqwest::blocking::get(format!("{}/timeseries?timestep={}&id={}", BASE, time, id)).unwrap()
	}

	fn get_timestamp(timestamp: TimestampTime) -> reqwest::blocking::Response {
		let time = match timestamp {
			TimestampTime::FIVE_MIN => "5m",
			TimestampTime::ONE_HOUR => "1h",
		};

		reqwest::blocking::get(format!("{}/{}", BASE, time)).unwrap()
	}

	fn get_latest(id: Option<u32>) -> reqwest::blocking::Response {
		let endpoint = match id {
			Some(id) => format!("/latest?id={}", id.to_string()),
			None => String::from("/latest")
		};

		reqwest::blocking::get(format!("{}{}", BASE, endpoint)).unwrap()
	}




}


