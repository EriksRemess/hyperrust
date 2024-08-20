extern crate hidapi;
use clap::Parser;
use hidapi::HidApi;
use hidapi::HidDevice;
use std::thread;
use std::time::Duration;

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

fn is_hex(color: &str) -> bool {
  color.chars().all(|c| c.is_digit(16))
}

fn get_color_value(color: &str) -> u32 {
  u32::from_str_radix(color, 16).expect("Invalid hex color")
}

fn rgb_to_hex(r: u8, g: u8, b: u8) -> String {
  format!("{:02x}{:02x}{:02x}", r, g, b)
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

fn hsl_to_rgb(h: f64, s: f64, l: f64) -> (u8, u8, u8) {
  let a = s * f64::min(l, 1.0 - l);
  let f = |n: f64| {
    let k = (n + h / 30.0) % 12.0;
    l - a * f64::max(-1.0, f64::min(f64::min(k - 3.0, 9.0 - k), 1.0))
  };
  let r = (f(0.0) * 255.0).round() as u8;
  let g = (f(8.0) * 255.0).round() as u8;
  let b = (f(4.0) * 255.0).round() as u8;
  (r, g, b)
}

fn send_init_packet(device: &HidDevice) -> Result<(), String> {
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

fn send_color(device: &HidDevice, color: u32) {
  let mut req = [0u8; 66];
  req[0x00] = 0x00;
  for i in 0..16 {
    req[(i * 4) + 1] = 0x81; // probably ignored, not really sure if changing this does anything
    req[(i * 4) + 2] = ((color >> 16) & 0x000000FF) as u8;
    req[(i * 4) + 3] = ((color >> 8) & 0x000000FF) as u8;
    req[(i * 4) + 4] = (color & 0x000000FF) as u8;
  }
  device
    .send_feature_report(&req)
    .expect("Failed to send feature report");
}

fn send_color_chunk(device: &HidDevice, chunk: &Vec<std::string::String>) {
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

fn generate_rainbow_colors(color_count: usize) -> Vec<String> {
  let mut colors = Vec::new();
  for i in 0..color_count {
    let hue = (i as f64 / color_count as f64) * 360.0;
    let (r, g, b) = hsl_to_rgb(hue, 1.0, 0.5);
    colors.push(rgb_to_hex(r, g, b));
  }
  colors
}

fn get_color_chunks(colors: Vec<String>, chunk_size: usize) -> Vec<Vec<String>> {
  colors
    .chunks(chunk_size)
    .map(|chunk| chunk.to_vec())
    .collect()
}

fn sleep(ms: u64) {
  thread::sleep(Duration::from_millis(ms));
}

#[allow(unused_assignments)]
fn main() {
  let args = Args::parse();
  let color = args.color.unwrap_or_default();
  if args.rainbow && color.len() > 0 {
    eprintln!("Invalid arguments, either use --rainbow or --color");
    return;
  }
  if !is_hex(color.as_str()) {
    eprintln!("Invalid color, must be a valid hex color");
    return;
  }
  let mut rainbow_colors = generate_rainbow_colors(71);
  let theme: Vec<String> = vec![
  // UNUSED   ESC       1         2         3         4         5         6         7         8         9         0         -         =         UNUSED    BACKSPACE
    "FF9700", "00FF6E", "FF9700", "FF9700", "FF9700", "FF9700", "00FF6E", "00FF6E", "00FF6E", "00FF6E", "FF9700", "FF9700", "FF9700", "FF9700", "00FF6E", "00FF6E",
  // TAB      Q         W         E         R         T         Y         U         I         O         P         [         ]         BACKSLASH
    "00FF6E", "FF9700", "FF9700", "FF9700", "FF9700", "FF9700", "FF9700", "FF9700", "FF9700", "FF9700", "FF9700", "FF9700", "FF9700", "00FF6E",
  // CAPSLOCK A         S         D         F         G         H         J         K         L         ;         '         UNUSED    ENTER
    "00FF6E", "FF9700", "FF9700", "FF9700", "FF9700", "FF9700", "FF9700", "FF9700", "FF9700", "FF9700", "FF9700", "FF9700", "FF9700", "00FF6E",
  // LSHIFT   UNUSED    Z         X         C         V         B         N         M         ,         .         /         UNUSED    RSHIFT
    "00FF6E", "FF9700", "FF9700", "FF9700", "FF9700", "FF9700", "FF9700", "FF9700", "FF9700", "FF9700", "FF9700", "FF9700", "FF9700", "00FF6E",
  // LCTRL    SUPER     LALT      LSPACE    SPACE     RSPACE    RALT      MENU      RCTRL     UNUSED    UNUSED    UNUSED    RFUNC
    "00FF6E", "00FF6E", "00FF6E", "FF9700", "FF9700", "FF9700", "00FF6E", "00FF6E", "00FF6E", "FF9700", "FF9700", "FF9700", "00FF6E"
  ].into_iter().map(String::from).collect();
  let api = HidApi::new().expect("Failed to create HID API");
  let mut found = false;
  for dev in api.device_list() {
    if !found
      && dev.vendor_id() == 1008
      && dev.product_id() == 3214
      && dev.usage() == 6
      && dev.usage_page() == 1
    {
      if let Ok(device) = dev.open_device(&api) {
        if let Ok(()) = send_init_packet(&device) {
          found = true;
          loop {
            if args.rainbow {
              let color_chunks: Vec<Vec<String>> = get_color_chunks(rainbow_colors.clone(), 16);
              for (_i, chunk) in color_chunks.iter().enumerate() {
                send_color_chunk(&device, chunk);
                sleep(25);
              }
              rainbow_colors.rotate_left(1);
            } else if color.len() > 0 {
              let color = pad_color(color.as_str());
              for _ in 0..5 {
                send_color(&device, get_color_value(&color));
                sleep(50);
              }
            } else {
              let color_chunks: Vec<Vec<String>> = get_color_chunks(theme.clone(), 16);
              for (_i, chunk) in color_chunks.iter().enumerate() {
                send_color_chunk(&device, chunk);
                sleep(50);
              }
            }
            let _ = send_init_packet(&device);
          }
        }
      }
    }
  }
  if !found {
    println!("HyperX Alloy Origins 60 keyboard not found");
  }
}
