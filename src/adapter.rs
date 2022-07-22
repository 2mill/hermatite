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

pub enum Endpoint {
	Mapping,
	Latest(Option<u32>),
	Timeseries(TimeseriesTime, u32),
	Timestamp(TimestampTime)
}
trait Time {
	fn get_time() -> String;
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
	fn get_timeseries(crate::adapter::TimeseriesTime) -> reqwest::Response;
	fn get_timestamp() -> reqwest::Response;
	fn get_latest(id: Option<u32>) -> reqwest::Response;
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

	fn get_timeseries(id, interval: TimeseriesTime) -> reqwest::blocking::Response {
		let time = match interval {
			TimeseriesTime::FIVE_MIN => "5m",
			TimeseriesTime::ONE_HOUR => "1h",
			TimeseriesTime::SIX_HOURS => "6h"
		};

		reqwest::blocking::get(format!("{}/timeserie?id="))

	}


}


