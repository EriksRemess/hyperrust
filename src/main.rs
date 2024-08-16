extern crate hidapi;
use clap::Parser;
use hidapi::HidApi;
use hidapi::HidDevice;
use std::thread;
use std::time::Duration;
// use rand::Rng;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// RGB color
    #[arg(short, long)]
    color: Option<String>,
}

fn send_init_packet(device: &HidDevice) {
    let mut req = [0u8; 65];
    req[0x00] = 0x00;
    req[0x01] = 0x04;
    req[0x02] = 0xF2;
    req[0x09] = 0x05;
    device
        .send_feature_report(&req)
        .expect("Failed to send feature report");
}

fn send_color(device: &HidDevice, color: u32) {
    let mut req = [0u8; 65];
    req[0x00] = 0x00;
    // let mut color = color;
    for i in 0..16 {
        // color = color.rotate_left(2);
        req[(i * 4) + 1] = 0x81;
        req[(i * 4) + 2] = ((color >> 16) & 0x000000FF) as u8;
        req[(i * 4) + 3] = ((color >> 8) & 0x000000FF) as u8;
        req[(i * 4) + 4] = (color & 0x000000FF) as u8;
    }

    device
        .send_feature_report(&req)
        .expect("Failed to send feature report");
}

// fn random_rgb() -> u32 {
//     rand::thread_rng().gen_range(0..=0xFFFFFF)
// }

fn main() {
    let args = Args::parse();

    let api = HidApi::new().expect("Failed to create HID API");
    let mut found = false;
    for dev in api.device_list() {
        if !found && dev.vendor_id() == 1008 && dev.product_id() == 3214 {
            found = true;
            if let Ok(device) = dev.open_device(&api) {
                loop {
                    let mut color_value =
                        u32::from_str_radix("FFF", 16).expect("Invalid hex color");
                    if let Some(ref color_str) = args.color {
                        color_value =
                            u32::from_str_radix(&color_str, 16).expect("Invalid hex color");
                    }
                    send_init_packet(&device);
                    for _ in 0..5 {
                        send_color(&device, color_value);
                        thread::sleep(Duration::from_millis(50));
                    }
                }
            }
        }
    }
}
