use serde_json;
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
	data: serde_json::Value,
}


impl Response {
	//This needs to be &str w/ a lifetime at some point
	const BASE: String = String::from("https://prices.runescape.wiki/api/v1/osrs");
	pub fn get(endpoint: Endpoint) -> Result<Self, Error> {
		unimplemented!()

	}

	fn endpoint_resolver(endpoint: Endpoint) -> Result<serde_json::Value, reqwest::Error> {
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
		}

		// At this point the reqwest should be made and matched for errors.
		//Afterwards the struct is return 
		unimplemented!()

	}
	fn resolve_time(time: Time) -> String {
		
	}
}






// trait Target {
// 	fn request(endpoint: Endpoint) -> Result<serde_json::Result<T, serde_json::Error, reqwest::Error>;
// }


