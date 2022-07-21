use serde_json;
use std::future::Future;
use reqwest::{Error};








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




struct Response {
	endpoint: Endpoint ,
	data: Result<reqwest::Response, reqwest::Error>,
}

const BASE: String = String::from("https://prices.runescape.wiki/api/v1/osrs");

impl Response {
	//This needs to be &str w/ a lifetime at some point
	pub fn get(endpoint: Endpoint) -> Result<reqwest::Response, Error> {
		let url = Response::endpoint_resolver(endpoint);
		reqwest::blocking::get(url)
	}

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


