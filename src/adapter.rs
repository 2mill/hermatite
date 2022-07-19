use serde_json::Deserializer;
use reqwest;
use core::future::Future;

pub enum Endpoint {
	Mapping,
	Latest(Option<u32>),
	Timeseries(u32, String),
	Timestamp(String),
}


/**
 * This model is still not good right now. Need to work on it some more.
 */




struct Client {
	endpoint: Endpoint,
	response: Result<reqwest::Response, reqwest::Error>
}

enum ItemData {
	Mapping,
	ItemPricingDataList,
	ItemPricingData,
	TimedItemList,
	TimedItem
}

impl Client {
	pub fn request() -> Result<ItemData, reqwest::Error> {

	}
}
mod Request {
	const BASE: &str = "https://prices.runescape.wiki/api/v1/osrs";
	use crate::adapter::{
		Endpoint,
		Response
	};
	pub fn request(endpoint: Endpoint) -> impl Future<Output = Result<reqwest::Response, reqwest::Error>> {
		let endpoint_string = match endpoint {
			Endpoint::Mapping => String::from("/mapping"),
			Endpoint::Latest(opt) => {
				if let Some(id) = opt {
					format!("/latest?id={}", id.to_string())
				} else {
					format!("/latest")
				}
			},
			Endpoint::Timeseries(id, timestep) => {
				format!("/timeseries?timestep={}&id={}", timestep, id.to_string())
			},
			Endpoint::Timestamp(timestamp) => {
				format!("/{}", timestamp)
			}
		};
		let complete_request_url = format!("{}{}", BASE, endpoint_string);

		
		reqwest::get(
			complete_request_url
		)
	}

}