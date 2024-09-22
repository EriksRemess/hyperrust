use crate::hid;
use crate::theme::get_theme_colors;
use crate::utils::sleep;
use hidapi::HidDevice;

const NUMBER_OF_KEYS: usize = 71;

pub fn apply_color(device: &HidDevice, color: &str) {
  match parse_color(color) {
    Ok(color) => {
      let color = u32_to_rgb(color);
      let colors: Vec<String> = vec![rgb_to_hex(color.0, color.1, color.2); NUMBER_OF_KEYS];
      loop {
        send_colors(&device, colors.clone());
        sleep(400);
      }
    }
    Err(err) => {
      eprintln!("Error: {}", err);
    }
  }
}

pub fn apply_gradient(device: &HidDevice, gradient1: &String, gradient2: &String) {
  match (parse_color(gradient1), parse_color(gradient2)) {
    (Ok(start_color), Ok(end_color)) => {
      let mut colors = gradient(start_color, end_color);
      loop {
        send_colors(&device, colors.clone());
        colors.rotate_left(1);
        sleep(100);
      }
    }
    (Err(err), _) | (_, Err(err)) => {
      eprintln!("Error: {}", err);
    }
  }
}

pub fn apply_rainbow(device: &HidDevice) {
  let mut rainbow_colors = rainbow_colors();
  loop {
    send_colors(&device, rainbow_colors.clone());
    rainbow_colors.rotate_left(1);
    sleep(100);
  }
}

pub fn apply_theme(device: &HidDevice, theme_name: &str) {
  if let Ok(colors) = get_theme_colors(theme_name) {
    loop {
      send_colors(&device, colors.clone());
      sleep(400);
    }
  } else {
    eprintln!("Invalid theme: {}", theme_name);
    std::process::exit(1);
  }
}

fn get_color_chunks(colors: Vec<String>, chunk_size: usize) -> Vec<Vec<String>> {
  colors
    .chunks(chunk_size)
    .map(|chunk| chunk.to_vec())
    .collect()
}

pub fn gradient(start_color: u32, end_color: u32) -> Vec<String> {
  let start_color = u32_to_rgb(start_color);
  let end_color = u32_to_rgb(end_color);
  let mut colors = Vec::new();
  for i in 0..36 {
    let factor = i as f64 / (36 as f64 - 1.0);
    let (r, g, b) = interpolate_color(start_color, end_color, factor);
    colors.push(rgb_to_hex(r, g, b));
  }
  let inverse_colors: Vec<String> = colors.iter().rev().map(|c| c.to_string()).collect();
  colors.extend(inverse_colors);
  colors
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

fn interpolate(start: f64, end: f64, factor: f64) -> f64 {
  start + (end - start) * factor
}

fn interpolate_color(start: (u8, u8, u8), end: (u8, u8, u8), factor: f64) -> (u8, u8, u8) {
  let r = interpolate(start.0 as f64, end.0 as f64, factor).round() as u8;
  let g = interpolate(start.1 as f64, end.1 as f64, factor).round() as u8;
  let b = interpolate(start.2 as f64, end.2 as f64, factor).round() as u8;
  (r, g, b)
}

pub fn is_valid_hex_color(s: &str) -> bool {
  let s = s.trim_start_matches('#');
  if s.len() != 3 && s.len() != 6 {
    return false;
  }
  s.chars().all(|c| c.is_digit(16))
}

pub fn parse_color(color_str: &str) -> Result<u32, String> {
  if !is_valid_hex_color(color_str) {
    return Err("Invalid hex color".to_string());
  }
  let mut color_str = color_str.trim_start_matches('#').to_string();
  if color_str.len() == 3 {
    color_str = color_str
      .chars()
      .flat_map(|c| std::iter::repeat(c).take(2))
      .collect();
  }
  u32::from_str_radix(&color_str, 16).map_err(|_| "Failed to parse hex color".to_string())
}

pub fn rainbow_colors() -> Vec<String> {
  let mut colors = Vec::new();
  for i in 0..NUMBER_OF_KEYS {
    let hue = (i as f64 / NUMBER_OF_KEYS as f64) * 360.0;
    let (r, g, b) = hsl_to_rgb(hue, 1.0, 0.5);
    colors.push(rgb_to_hex(r, g, b));
  }
  colors
}

fn rgb_to_hex(r: u8, g: u8, b: u8) -> String {
  format!("{:02x}{:02x}{:02x}", r, g, b)
}

fn send_colors(device: &HidDevice, colors: Vec<String>) {
  match hid::send_init_packet(&device) {
    Ok(_) => {
      let color_chunks = get_color_chunks(colors, 16);
      for (_i, chunk) in color_chunks.iter().enumerate() {
        let mut req = [0u8; 65];
        req[0x00] = 0x00;
        for (i, color) in chunk.iter().enumerate() {
          let color_value = parse_color(color).unwrap();
          req[(i * 4) + 1] = 0x81;
          req[(i * 4) + 2] = ((color_value >> 16) & 0x000000FF) as u8;
          req[(i * 4) + 3] = ((color_value >> 8) & 0x000000FF) as u8;
          req[(i * 4) + 4] = (color_value & 0x000000FF) as u8;
        }
        hid::send_colors(&device, req);
      }
    }
    Err(e) => {
      eprintln!("Failed to send init packet: {}", e);
      std::process::exit(1);
    }
  }
}

pub fn u32_to_rgb(color: u32) -> (u8, u8, u8) {
  let r = ((color >> 16) & 0xff) as u8;
  let g = ((color >> 8) & 0xff) as u8;
  let b = (color & 0xff) as u8;
  (r, g, b)
}
