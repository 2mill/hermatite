use serde_json;
use reqwest;
pub enum Endpoint{
	Mapping,
	Latest(Option<u32>),
	Timestep(u32, Time),
	Timestamp(Time)
}
enum Time {
	FiveMinutes,
	OneHour,
	SixHours, // unofficial for timestamp, but it helps the code structure.
	TwentyFourHours, //unofficial support for both, but I am going to include it anyways.
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
pub fn fetch(endpoint: Endpoint) -> Result<reqwest::blocking::Response, reqwest::Error> {
	const base: &str = "https://prices.runescape.wiki/api/v1/osrs";
	let endpoint_string: String = match endpoint {
		Endpoint::Mapping => String::from("/mapping"),
		Endpoint::Latest(opt) => if let Some(id) = opt {
			format!("/latest?id={}", id)
		} else {
			format!("/latest?")
		},
		Endpoint::Timestep(id, time) => {
			format!("/timeseries?id={}&timestep={}", id, time_resolver(time))
		},
		Endpoint::Timestamp(time) => {
			format!("/{}", time_resolver(time))
		}
	};
	reqwest::blocking::get(format!("{}{}", base, endpoint_string))
}






