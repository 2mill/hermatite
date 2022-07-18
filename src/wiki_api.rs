
pub mod requests {
	use reqwest::blocking::Response;
	use reqwest;
	const API_URL: &str = "https://prices.runescape.wiki/api/v1/osrs";

	pub struct ItemPricingData {
		id: u32,
		high: u32,
		high_time: u32,
		low: u32,
		low_time: u32,
	}

	enum Endpoint {
		Mapping,
		Latest(Option<u32>),
		Timeseries(String),
		Timestamp(String),
	}

	fn request(endpoint: Endpoint) -> Result<Response, reqwest::Error> {
		let real_str = match endpoint {
			Endpoint::Mapping => "/mapping",
			Endpoint::Latest(opt) => unimplemented!(),

			/**
			 * match opt {
				Some(id) => return latest with id param
				None = => just return latest.

			 }
			 */

			Endpoint::Timeseries(timestep) => unimplemented!(),
			Endpoint::Timestamp(timestamp) => unimplemented!()


			
		};
		let req_string = format!("{}{}", API_URL, real_str);
		reqwest::blocking::get(req_string.as_str())
	}


	pub fn latest_all() -> Result<Vec<ItemPricingData> , String> {
		unimplemented!()
	}
	pub fn latest_id(id: &u32) -> Result<ItemPricingData, String> {
		unimplemented!()
	}

	



}
