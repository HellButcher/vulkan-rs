
pub use self::vk::*;

pub mod basic_types {
	pub type Enum = i32;
	pub type Flags = u32;
}

mod vk;

#[test]
fn it_works() {
}
