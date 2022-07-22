
struct ItemList {
	items: Vec<Item>,
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
impl Alchable {
	fn new(high_alch: u32, low_alch: u32) -> Self {
		Alchable { low_alch, high_alch }
	}
	fn alch_diff(&self) -> u32 {
		self.high_alch - self.low_alch
	}
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

impl ItemList {
	pub fn from(item_list: Vec<Item> ) -> ItemList {
		ItemList {
			items: item_list
		}
	} 
	fn find_id(&self, item_id: &u32) -> Option<&Item> {
		for item in self.items.iter() {
			if item.id == *item_id {
				return Some(item)
			}
		}
		None
	}
	fn find_name(&self, item_name: &str) -> Option<&Item> {
		for item in self.items.iter() {
			if item.name == *item_name {
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