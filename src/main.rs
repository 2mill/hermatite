use chrono::DateTime;
enum Temp {
	FIVE_MIN,
	ONE_HOUR
}

enum Timestep {
	FIVE_MIN,
	ONE_HOUR,
	SIX_HOURS
}

struct TimedItem {
	id: u32,
	avg_high_price: u32,
	avg_low_price: u32,
	avg_high_volume: u32,
	avg_low_volume: u32,
	utc: u32,
}

struct ItemPricing {
	pub id: u32,
	pub high: u32,
	pub high_time: u32,
	pub low: u32,
	pub low_time: u32
}

struct Alchable {
	pub low_alch: u32,
	pub high_alch: u32,
}













mod wiki_api;
fn main() {
    println!("Hello, world!");
	// wiki_api::untyped_example();



}
