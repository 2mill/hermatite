

pub mod requests {
	use https::{Request, Response};
	const API_URL: &str = "https://prices.runescape.wiki/api/v1/osrs";
	enum Endpoint {
		Mapping,
		Latest(Option<u32>)
	}

	pub struct ItemPricingData {
		id: u32,
		high: u32,
		high_time: u32,
		low: u32,
		low_time: u32,
	}

	fn request(endpoint: Endpoint) -> Result<Response, u32> {
		let real_str = match endpoint {
			Mapping => "/mapping",
			Latest => "/latest",
		};
		let uri = format!("{}{}", API_URL, real_str);
		let mut request = Request::builder();
		unimplemented!()
	}
	pub fn latest_all() -> Result<Vec<ItemPricingData> , String> {
		unimplemented!()
	}
	pub fn latest_id(id: &u32) -> Result<ItemPricingData, String> {
		unimplemented!()
	}

	



}
