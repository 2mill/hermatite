
pub mod requests {
	use reqwest::blocking::Response;
	use reqwest;
use serde_json::Deserializer;
	const API_URL: &str = "https://prices.runescape.wiki/api/v1/osrs";

	pub struct ItemPricingData {
		id: u32,
		high: u32,
		high_time: u32,
		low: u32,
		low_time: u32,
	}

}


	
	fn url(endpoint: Endpoint) -> String {
		format!(
			"{}{}", 
			API_URL, 
			match endpoint {
				Endpoint::Mapping => String::from("/mapping"),
				Endpoint::Latest(opt) => 
					if let Some(id) = opt {
						format!("/latest?id={}", id.to_string())
					} else {
						String::from("/latest")
					}
				,
				Endpoint::Timeseries(id, timestep) => {
					format!("timeseries?timestep={}&id={}", timestep, id.to_string())
				},
				Endpoint::Timestamp(timestamp) => {
					format!("/{}", timestamp)
				}
			}
		)
	}

	fn request(endpoint: Endpoint) -> Result<Response, reqwest::Error> {
		reqwest::blocking::get(
			url(endpoint)
		)
	}

	fn deseralize_and_format_response(response: Response, endpoint: Endpoint) -> () {
		match endpoint {
			Endpoint::Mapping => {

			},
			Endpoint::Latest(_) => {

			},
			Endpoint::Timeseries(_,_) => {
				
			},
			Endpoint::Timestamp(_) => {

			}
		}
		 unimplemented!()
	}


	pub fn latest_all() -> Result<Vec<ItemPricingData> , String> {
		unimplemented!()
	}
	pub fn latest_id(id: &u32) -> Result<ItemPricingData, String> {
		unimplemented!()
	}
mod Requester {

	use serde_json;
	enum Endpoint {
		Mapping,
		Latest(Option<u32>),
		Timeseries(u32, String),
		Timestamp(String),
	}
	struct Request {
		request_endpoint: Endpoint

	}
	impl Request {
		fn new(endpoint: Endpoint) -> Result<RequestData, String> {
			unimplemented!()
		}
	}

}