use serde_json;
use std::future::Future;
use reqwest;








enum Endpoint {
	Mapping,
	Latest(Option<u32>),
	Timeseries(Time, u32),
	Timestamp(Time),
}



enum Time {
	FIVE_MIN,
	ONE_HOUR,
	SIX_HOURS
}





struct Config {
	endpoint: Endpoint,
	user_agent: String,
}

const BASE: String = String::from("https://prices.runescape.wiki/api/v1/osrs");

struct ItemData {
	config: Config,
	result: reqwest::blocking::Response
}

trait Adapter {
	fn new(config: Config) -> Self;
	fn deseralize(&self) -> ();
}

impl Adapter for ItemData {
	//This needs to be &str w/ a lifetime at some point
	fn new(config: Config) -> Self {
		let url = ItemData::endpoint_resolver(config.endpoint);
		ItemData {
			config,
			result: reqwest::blocking::get(url).unwrap()
		}
	}

	fn deseralize(&self) -> () { 
		let text = self.result.text().unwrap().as_str();
		serde_json::from_str(text).unwrap()
	}

}

impl ItemData {
	fn endpoint_resolver(endpoint: Endpoint) -> String {
		let endpoint_str = match endpoint {
			Endpoint::Mapping => String::from("/mapping"),
			Endpoint::Latest(opt) => {
				if let Some(id) = opt {
					format!("/latest?id={}", id.to_string())
				} else {
					String::from("/latest")
				}
			},
			Endpoint::Timeseries(timestep, u32) => {
				let time_str = match timestep {
					Time::FIVE_MIN => String::from("5m"),
					Time::ONE_HOUR => String:: from("1h"),
					Time::SIX_HOURS => String::from("6h")
				};
				format!("/timeseries?timestep={}?id={}", time_str, u32.to_string())
			},
			Endpoint::Timestamp(timestamp) => {
				let time_str = match timestamp{
					Time::FIVE_MIN => String::from("5m"),
					Time::ONE_HOUR => String::from("1h"),
					Time::SIX_HOURS => String::from("6h")
				};
				format!("/{}", time_str)
			}
		};

		// At this point the reqwest should be made and matched for errors.
		//Afterwards the struct is return 

		format!("{}{}", BASE, endpoint_str)
	}

}










// trait Target {
// 	fn request(endpoint: Endpoint) -> Result<serde_json::Result<T, serde_json::Error, reqwest::Error>;
// }


