
pub use self::vk::*;

pub mod basic_types {
	type Enum = i32;
	type Flags = u32;
	struct Handle {
		value: usize
	}
	struct DispatchableHandle {
		value: u32
	}
}

mod vk;

#[test]
fn it_works() {
}
