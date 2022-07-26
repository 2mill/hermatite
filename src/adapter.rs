use serde_json;
use reqwest;





pub enum Endpoint{
	Mapping,
	Latest(Option<u32>)
}
pub fn fetch(endpoint: Endpoint) -> Result<reqwest::blocking::Response, reqwest::Error> {
	const base: &str = "https://prices.runescape.wiki/api/v1/osrs";
	let endpoint_string: String = match endpoint {
		Endpoint::Mapping => String::from("/mapping"),
		Endpoint::Latest(opt) => if let Some(id) = opt {
			format!("/latest?id={}", id)
		} else {
			format!("/latest?")
		}
	};
	reqwest::blocking::get(format!("{}{}", base, endpoint_string))
}






