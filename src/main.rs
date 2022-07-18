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
impl Alchable {
	fn new(high_alch: u32, low_alch: u32) -> Self {
		Alchable { low_alch, high_alch }
	}
	fn alch_diff(&self) -> u32 {
		self.high_alch - self.low_alch
	}
}
struct Item {
	pub id: u32,
	pub examine: String,
	pub members: bool,
	pub alch: Option<Alchable>,
	pub limit: Option<u32>,
	pub icon: String,
	pub name: String,
}

impl Item {
	fn new(id: u32, examine: String, members: bool, alch: Option<Alchable>, limit: Option<u32>, icon: String, name: String) -> Self {
		Item {
			id,
			examine,
			members,
			alch,
			limit,
			icon,
			name
		}
	}
}

struct ItemList {
	items: Vec<Item>,
}

impl ItemList {
	fn new() -> Self{
		let new_list: Vec<Item> = Vec::new();
		ItemList { items: new_list }
	}

	fn add<'a>(&mut self, item: Item) {
		self.items.push(item);
	}

	fn find_id(&self, item_id: &u32) -> Option<&Item> {
		for item in self.items.iter() {
			if item.id == *item_id {
				return Some(item)
			}
		}
		None
	}

	fn member_count(&self) -> u32 {
		let mut count = 0;
		for item in self.items.iter() {
			if item.members { count += 1 }
		}
		count
	}
	fn f2p_count(&self) -> u32 {
		let mut count: u32 = 0;
		for item in self.items.iter() {
			if item.members { count += 1 }
		}
		count
	}
}











mod wiki_api;
fn main() {
    println!("Hello, world!");
	// wiki_api::untyped_example();
	wiki_api::temp::untyped_example().unwrap();
	wiki_api::temp::other_temp();

	let new_item = Item::new(4151, String::from("Something"), true, None, Some(5), String::from("something.png"), String::from("something something"));


}
