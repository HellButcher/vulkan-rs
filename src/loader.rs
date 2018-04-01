use dispatch_table::*;
use dl;
use platform::c_char;
use protos::{PFN_vkGetDeviceProcAddr, PFN_vkGetInstanceProcAddr};
use std::ffi::{CStr, OsStr};
use std::sync;
use types;
use types_raw;
use RawStruct;
use utils::VkDispatchableHandle;

use std::collections::btree_map::Entry;
type Map<T> = ::std::collections::BTreeMap<usize, T>;
type RwMap<T> = sync::RwLock<Map<T>>;

#[cfg(not(target_os = "windows"))]
const VULKAN_LIB_NAME: &str = "libvulkan.so";

#[cfg(target_os = "windows")]
const VULKAN_LIB_NAME: &str = "vulkan.dll";

static LOADER_INIT: sync::Once = sync::ONCE_INIT;
static mut LOADER_DATA: Option<LoaderData> = None;

type VoidFunctionResult = ::std::result::Result<types_raw::PFN_vkVoidFunction, &'static str>;

unsafe fn dispatch_key<T>(handle: VkDispatchableHandle<T>) -> usize {
  let handle: *const usize = ::std::mem::transmute(handle);
  if handle.is_null() {
    return 0;
  }
  return *handle;
}

unsafe fn call_gipa(gipa: PFN_vkGetInstanceProcAddr, arg: Option<types::VkInstance>, name: &str) -> VoidFunctionResult {
  let name = CStr::from_bytes_with_nul(name.as_bytes()).unwrap();
  let gipa_usize: usize = ::std::mem::transmute(gipa);
  println!(
    "loading command {:?} with vkGetInstanceProcAddr({:#x}) from instance {:?}",
    name, gipa_usize, arg
  );
  if let Some(p) = gipa(arg.map(|a| a.value()).unwrap_or(0), name.as_ptr()) {
    Ok(p)
  } else {
    println!("unable to load command {:?}", name);
    Err("unable to load command with vkGetInstanceProcAddr")
  }
}

unsafe fn call_gdpa(gdpa: PFN_vkGetDeviceProcAddr, arg: types::VkDevice, name: &str) -> VoidFunctionResult {
  let name = CStr::from_bytes_with_nul(name.as_bytes()).unwrap();
  let gdpa_usize: usize = ::std::mem::transmute(gdpa);
  println!(
    "loading command {:?} with vkGetDeviceProcAddr({:#x}) from device {:?}",
    name, gdpa_usize, arg
  );
  if let Some(p) = gdpa(arg.value(), name.as_ptr()) {
    Ok(p)
  } else {
    println!("unable to load command {:?}", name);
    Err("unable to load command with vkGetDeviceProcAddr")
  }
}

unsafe fn ext_equals(ext: *const c_char, check_ext: &[u8]) -> bool {
  if ext.is_null() {
    return false;
  }
  let ext = ::std::slice::from_raw_parts(ext, check_ext.len() + 1);
  for i in 0..check_ext.len() {
    if ext[i] as u8 != check_ext[i] {
      return false;
    }
    if check_ext[i] == 0 {
      return true;
    }
  }
  return ext[check_ext.len()] == 0;
}

unsafe fn has_ext(extension_count: u32, pp_extension_names: *const *const c_char, check_ext: &[u8]) -> bool {
  if extension_count == 0 || pp_extension_names.is_null() {
    return false;
  }
  let extension_names = ::std::slice::from_raw_parts(pp_extension_names, extension_count as usize);
  for ext in extension_names {
    if ext.is_null() {
      return false;
    }
    if ext_equals(*ext, check_ext) {
      return true;
    }
  }
  false
}

#[allow(non_snake_case)]
struct LoaderData {
  loader_dispatch_table: VkLoaderDispatchTable,
  instance_data: RwMap<InstanceData>,
  device_data: RwMap<DeviceData>,
  vkGetInstanceProcAddr: Option<PFN_vkGetInstanceProcAddr>,
}

#[allow(non_snake_case)]
struct InstanceData {
  instance_dispatch_table: VkInstanceDispatchTable,
  //instance: types::VkInstance,
  vkGetDeviceProcAddr: Option<PFN_vkGetDeviceProcAddr>,
}

struct DeviceData {
  device_dispatch_table: VkDeviceDispatchTable,
  //device: types::VkDevice,
}

impl LoaderData {
  #[inline]
  pub fn open<S: AsRef<OsStr>>(libname: &S) -> Result<LoaderData, &'static str> {
    let (dispatch_table, gipa) = VkLoaderDispatchTable::open(libname)?;
    Ok(LoaderData {
      loader_dispatch_table: dispatch_table,
      instance_data: sync::RwLock::new(Map::new()),
      device_data: sync::RwLock::new(Map::new()),
      vkGetInstanceProcAddr: Some(gipa),
    })
  }

  pub fn get_opt() -> Option<&'static LoaderData> {
    unsafe {
      LOADER_INIT.call_once(|| {
        LOADER_DATA = match Self::open(&VULKAN_LIB_NAME) {
          Ok(tab) => Some(tab),
          Err(err) => {
            println!("loading vulkan failed: {}", err);
            None
          }
        };
      });
      LOADER_DATA.as_ref()
    }
  }
}

impl InstanceData {
  #[inline]
  pub fn with<F, R>(key: usize, body: F) -> R
  where
    F: FnOnce(Option<&Self>) -> R,
  {
    let dat_guard = LoaderData::get_opt().unwrap().instance_data.read().unwrap();
    body(dat_guard.get(&key))
  }

  #[inline]
  pub fn with_mut<F, R>(key: usize, body: F) -> R
  where
    F: FnOnce(Entry<usize, Self>) -> R,
  {
    let mut dat_guard = LoaderData::get_opt()
      .unwrap()
      .instance_data
      .write()
      .unwrap();
    body(dat_guard.entry(key))
  }
}

impl DeviceData {
  #[inline]
  pub fn with<F, R>(key: usize, body: F) -> R
  where
    F: FnOnce(Option<&Self>) -> R,
  {
    let dat_guard = LoaderData::get_opt().unwrap().device_data.read().unwrap();
    body(dat_guard.get(&key))
  }

  #[inline]
  pub fn with_mut<F, R>(key: usize, body: F) -> R
  where
    F: FnOnce(Entry<usize, Self>) -> R,
  {
    let mut dat_guard = LoaderData::get_opt().unwrap().device_data.write().unwrap();
    body(dat_guard.entry(key))
  }
}

const GET_INSTANCE_PROC_ADDR_NAME: &'static str = "vkGetInstanceProcAddr\0";
const GET_DEVICE_PROC_ADDR_NAME: &'static str = "vkGetDeviceProcAddr\0";

impl VkLoaderDispatchTable {
  pub fn open<S: AsRef<OsStr>>(
    libname: &S,
  ) -> Result<(VkLoaderDispatchTable, PFN_vkGetInstanceProcAddr), &'static str> {
    unsafe {
      let lib = match dl::open(libname) {
        Ok(lib) => lib,
        Err(_) => {
          return Err("unable to load vulkan library");
        }
      };
      let gipa: PFN_vkGetInstanceProcAddr = match lib.get(GET_INSTANCE_PROC_ADDR_NAME) {
        Ok(Some(p)) => ::std::mem::transmute(p),
        _ => {
          return Err("unable to find vkGetInstanceProcAddr function");
        }
      };

      Self::load_with(gipa)
    }
  }

  #[inline]
  pub unsafe fn load_with(
    gipa: PFN_vkGetInstanceProcAddr,
  ) -> Result<(VkLoaderDispatchTable, PFN_vkGetInstanceProcAddr), &'static str> {
    let tab = Self::load(|name| call_gipa(gipa, None, name))?;
    Ok((tab, gipa))
  }

  #[inline]
  pub unsafe fn get_opt<'a>() -> Option<&'a Self> {
    LoaderData::get_opt().map(|d| &d.loader_dispatch_table)
  }

  #[inline]
  pub unsafe fn with<F, R>(body: F) -> R
  where
    F: FnOnce(&Self) -> R,
  {
    body(Self::get_opt().unwrap())
  }
}

impl VkInstanceDispatchTable {
  #[inline]
  pub unsafe fn with<T, F, R>(handle: VkDispatchableHandle<T>, body: F) -> R
  where
    F: FnOnce(&Self) -> R,
  {
    InstanceData::with(dispatch_key(handle), |data_opt| {
      body(&data_opt.unwrap().instance_dispatch_table)
    })
  }

  #[inline]
  pub unsafe fn load_with(
    create_info: &types::VkInstanceCreateInfo,
    instance: types::VkInstance,
    gipa: PFN_vkGetInstanceProcAddr,
  ) -> Result<(VkInstanceDispatchTable, PFN_vkGetDeviceProcAddr), &'static str> {
    use std::mem::transmute;
    let create_info = create_info.as_raw();
    let gipa: PFN_vkGetInstanceProcAddr = transmute(call_gipa(
      gipa,
      Some(instance),
      GET_INSTANCE_PROC_ADDR_NAME,
    )?);
    let gdpa: PFN_vkGetDeviceProcAddr = transmute(call_gipa(gipa, Some(instance), GET_DEVICE_PROC_ADDR_NAME)?);
    let tab = Self::load(
      |name| call_gipa(gipa, Some(instance), name),
      |ext| {
        has_ext(
          create_info.enabledExtensionCount,
          create_info.ppEnabledExtensionNames,
          ext.as_bytes(),
        )
      },
    )?;
    Ok((tab, gdpa))
  }

  pub unsafe fn create(
    create_info: &types::VkInstanceCreateInfo,
    _: Option<&types::VkAllocationCallbacks>,
    instance: types::VkInstance,
  ) {
    let gipa = LoaderData::get_opt()
      .unwrap()
      .vkGetInstanceProcAddr
      .unwrap();
    let (new_tab, gdpa) = Self::load_with(create_info, instance, gipa).unwrap();
    let new_data = InstanceData {
      instance_dispatch_table: new_tab,
      //instance: instance,
      vkGetDeviceProcAddr: Some(gdpa),
    };

    InstanceData::with_mut(dispatch_key(instance), |entry| {
      if let Entry::Vacant(e) = entry {
        e.insert(new_data);
      } else {
        panic!(
          "there is already an unexpected dispatch_table for instance {:?}",
          instance
        );
      }
    });
  }

  pub unsafe fn destroy(instance: types::VkInstance) {
    InstanceData::with_mut(dispatch_key(instance), |entry| {
      if let Entry::Occupied(e) = entry {
        e.remove();
      }
    });
  }
}

impl VkDeviceDispatchTable {
  #[inline]
  pub unsafe fn with<T, F, R>(handle: VkDispatchableHandle<T>, body: F) -> R
  where
    F: FnOnce(&Self) -> R,
  {
    DeviceData::with(dispatch_key(handle), |data_opt| {
      body(&data_opt.unwrap().device_dispatch_table)
    })
  }

  #[inline]
  pub unsafe fn load_with(
    create_info: &types::VkDeviceCreateInfo,
    device: types::VkDevice,
    gdpa: PFN_vkGetDeviceProcAddr,
  ) -> Result<VkDeviceDispatchTable, &'static str> {
    use std::mem::transmute;
    let create_info = create_info.as_raw();
    let gdpa: PFN_vkGetDeviceProcAddr = transmute(call_gdpa(gdpa, device, GET_DEVICE_PROC_ADDR_NAME)?);
    let tab = Self::load(
      |name| call_gdpa(gdpa, device, name),
      |ext| {
        has_ext(
          create_info.enabledExtensionCount,
          create_info.ppEnabledExtensionNames,
          ext.as_bytes(),
        )
      },
    );
    tab
  }

  pub unsafe fn create(
    physical_device: types::VkPhysicalDevice,
    create_info: &types::VkDeviceCreateInfo,
    _: Option<&types::VkAllocationCallbacks>,
    device: types::VkDevice,
  ) {
    let gdpa = InstanceData::with(dispatch_key(physical_device), |instance_data| {
      instance_data.unwrap().vkGetDeviceProcAddr.unwrap()
    });

    let new_tab = Self::load_with(create_info, device, gdpa).unwrap();
    let new_data = DeviceData {
      device_dispatch_table: new_tab,
      //device: device,
    };

    DeviceData::with_mut(dispatch_key(device), |entry| {
      if let Entry::Vacant(e) = entry {
        e.insert(new_data);
      } else {
        panic!(
          "there is already an unexpected dispatch_table for device {:?}",
          device
        );
      }
    });
  }

  pub unsafe fn destroy(device: types::VkDevice) {
    DeviceData::with_mut(dispatch_key(device), |entry| {
      if let Entry::Occupied(e) = entry {
        e.remove();
      }
    });
  }
}
