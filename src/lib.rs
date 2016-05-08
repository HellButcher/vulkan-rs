
#[macro_use]
pub mod basic_types;

pub use self::vk::*;


mod vk;

#[cfg(test)]
mod tests {
    use vk;
    #[test]
    fn it_works() {
        use std::mem;
        unsafe {
            let ici : vk::InstanceCreateInfo = mem::zeroed();
            let ac : vk::AllocationCallbacks = mem::zeroed();
            let mut inst : vk::Instance = mem::zeroed();
            vk::create_instance(&ici, &ac, &mut inst);
        }
    }
}
