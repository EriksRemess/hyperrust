extern crate hidapi;
use clap::Parser;
use hidapi::HidApi;
use hidapi::HidDevice;
use std::thread;
use std::time::Duration;
use rand::Rng;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Rainbow puke
    #[arg(short, long)]
    rainbow: bool,
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
    for i in 0..16 {
        req[(i * 4) + 1] = 0x81;
        req[(i * 4) + 2] = ((color >> 16) & 0x000000FF) as u8;
        req[(i * 4) + 3] = ((color >> 8) & 0x000000FF) as u8;
        req[(i * 4) + 4] = (color & 0x000000FF) as u8;
    }

    device
        .send_feature_report(&req)
        .expect("Failed to send feature report");
}

fn get_color_value(color: &str) -> u32 {
  u32::from_str_radix(color, 16).expect("Invalid hex color")
}

fn send_color_chunk(device: &HidDevice, chunk: &[&str]) {
    let mut req = [0u8; 65];
    req[0x00] = 0x00;
    for (i, color) in chunk.iter().enumerate() {
        let color_value = get_color_value(color);
        req[(i * 4) + 1] = 0x81;
        req[(i * 4) + 2] = ((color_value >> 16) & 0x000000FF) as u8;
        req[(i * 4) + 3] = ((color_value >> 8) & 0x000000FF) as u8;
        req[(i * 4) + 4] = (color_value & 0x000000FF) as u8;
    }
    device
        .send_feature_report(&req)
        .expect("Failed to send feature report");
}

fn random_rgb() -> u32 {
    rand::thread_rng().gen_range(0..=0xFFFFFF)
}

fn pad_color(color: &str) -> String {
    if color.len() == 6 {
        return color.to_string();
    }
    if color.len() == 3 {
        return format!(
            "{}{}{}",
            &color[0..1].repeat(2),
            &color[1..2].repeat(2),
            &color[2..3].repeat(2)
        );
    }
    let mut color = color.to_string();
    while color.len() < 6 {
        color.push_str("0");
    }
    color
}

fn main() {
    let args = Args::parse();
    let mut rainbow_colors: [&str; 71] = [
      "ff0000", "ff1500", "ff2b00", "ff4000", "ff5600", "ff6b00", "ff8100", "ff9600",
      "ffac00", "ffc100", "ffd700", "ffec00", "f8fb00", "e6ff00", "d0ff00", "bbff00",
      "a5ff00", "90ff00", "7aff00", "65ff00", "4fff00", "3aff00", "24ff00", "0fff00",
      "03ff09", "00ff1b", "00ff31", "00ff46", "00ff5c", "00ff71", "00ff87", "00ff9c",
      "00ffb2", "00ffc8", "00ffdd", "00fff3", "00f5ff", "00dfff", "00caff", "00b4ff",
      "009fff", "0089ff", "0074ff", "005eff", "0049ff", "0033ff", "001eff", "020bff",
      "0c00ff", "2200ff", "3700ff", "4d00ff", "6200ff", "7800ff", "8d00ff", "a300ff",
      "b800ff", "ce00ff", "e300ff", "f700fd", "fe00ef", "ff00d9", "ff00c4", "ff00ae",
      "ff0099", "ff0083", "ff006d", "ff0058", "ff0042", "ff002d", "ff0017"
    ];
    let api = HidApi::new().expect("Failed to create HID API");
    let mut found = false;
    for dev in api.device_list() {
        if !found && dev.vendor_id() == 1008 && dev.product_id() == 3214 {
            found = true;
            if let Ok(device) = dev.open_device(&api) {
                loop {
                    send_init_packet(&device);
                    if args.rainbow {
                      let color_chunks: Vec<&[&str]> = rainbow_colors.chunks(16).collect();
                      for (_i, chunk) in color_chunks.iter().enumerate() {
                          send_color_chunk(&device, chunk);
                          thread::sleep(Duration::from_millis(25));
                      }
                      rainbow_colors.rotate_left(1);
                    } else if let Some(ref color) = args.color {
                      let color = pad_color(color.as_str());
                      for _ in 0..5 {
                        send_color(&device, get_color_value(&color));
                        thread::sleep(Duration::from_millis(50));
                      }
                    } else {
                      for _ in 0..5 {
                        send_color(&device, random_rgb());
                        thread::sleep(Duration::from_millis(50));
                      }
                    }
                }
            }
        }
    }
    if !found {
        println!("HyperX Alloy Origins 60 keyboard not found");
    }
}
