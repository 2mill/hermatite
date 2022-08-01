use serde_json;
use reqwest;
pub enum Endpoint{
	Mapping,
	Latest(Option<u32>),
	Timestep(u32, Time),
	Timestamp(Time, Option<i64>) //i64 UNIX TIMESTAMP. I would like this to have better code description.
}
enum Time {
	FiveMinutes,
	OneHour,
	SixHours, // unofficial for timestamp, but it helps the code structure.
	TwentyFourHours, //unofficial support for both, but I am going to include it anyways.
}

trait Target {
	fn fetch(endpoint: Endpoint) -> Self;
	fn deseralize(&self) -> String;
}

struct Request {
	response: reqwest::blocking::Response,
	endpoint: Endpoint,
}

impl Target for Request {
	fn fetch(endpoint: Endpoint) -> Result<Self, reqwest::Error> {
		const base: &str = "https://prices.runescape.wiki/api/v1/osrs";
		let endpoint_string: String = match endpoint {
			Endpoint::Mapping => String::from("/mapping"),
			Endpoint::Latest(opt) => if let Some(id) = opt {
				format!("/latest?id={}", id)
			} else {
				format!("/latest?")
			},
			Endpoint::Timestep(id, step) => {
				format!("/timeseries?id={}&timestep={}", id, time_resolver(step))
			},
			Endpoint::Timestamp(step, timestamp) => {
				if let Some(time) = timestamp {
					format!("/{}?timestamp={}", time_resolver(step), time)
				} else {
					format!("/{}", time_resolver(step))
				}
			}
		};
		Response {
			response: reqwest::blocking::get(format!("{}{}", base, endpoint_string)).unwrap().text(),
			endpoint
		}
		// match reqwest::blocking::get(format!("{}{}", base, endpoint_string)) {
		// 	Ok(res) => {
		// 		Ok(Request { response: serde_json::from_str(res.text()), endpoint })
		// 	}
		// 	Err(err) => Err(err)
		// 	//More than likely anti pattern.
		// }

	}
}


fn time_resolver(time: Time) -> String {
	String::from(
		match time {
			Time::FiveMinutes => "5m",
			Time::OneHour => "1h",
			Time::SixHours => "6h",
			Time::TwentyFourHours => "24h"
		}
	)
}






