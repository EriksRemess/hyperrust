use hidapi::{HidApi, HidDevice};
use std::process;

pub fn get_device(
  vendor_id: u16,
  product_id: u16
) -> Option<HidDevice> {
  let api = HidApi::new().expect("Failed to create HID API");
  for dev in api.device_list() {

    if dev.vendor_id() == vendor_id
      && dev.product_id() == product_id
    {
      if cfg!(target_os = "windows")
       && dev.interface_number() == 0x3 {
        if let Ok(device) = dev.open_device(&api) {
          return Some(device);
       }
      } else {
        if dev.usage() == 0x6
          && dev.usage_page() == 0x1 {
          if let Ok(device) = dev.open_device(&api) {
            return Some(device);
          }
        }
      }
    }
  }
  eprintln!("Device not found");
  return None;
}

pub fn send_init_packet(device: &HidDevice) -> Result<(), String> {
  let mut req = [0u8; 65];
  req[0x00] = 0x00;
  req[0x01] = 0x04;
  req[0x02] = 0xF2;
  req[0x09] = 0x05;
  match device.send_feature_report(&req) {
    Ok(_) => Ok(()),
    Err(e) => Err(format!("Failed to send feature report: {}", e)),
  }
}

pub fn send_colors(device: &HidDevice, colors: [u8; 65]) {
  match device.send_feature_report(&colors) {
    Ok(_) => (),
    Err(e) => {
      eprintln!("Failed to send feature report: {}", e);
      process::exit(1);
    }
  }
}
