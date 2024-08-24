extern crate hidapi;
mod colors;
mod hid;
mod theme;
mod utils;
use clap::Parser;
use colors::{apply_color, apply_gradient, apply_rainbow, apply_theme};
use hid::get_device;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
  /// Rainbow puke
  #[arg(short, long)]
  rainbow: bool,
  /// RGB color
  #[arg(short, long)]
  color: Option<String>,
  /// Gradient color 1
  #[arg(short = 'a', long)]
  gradient1: Option<String>,
  /// Gradient color 2
  #[arg(short = 'b', long)]
  gradient2: Option<String>,
  /// Theme
  #[arg(short, long, value_parser = theme::get_theme_names())]
  theme: Option<String>,
}

fn main() {
  let args = Args::parse();
  if let Some(device) = get_device(1008, 3214, 6, 1) {
    let color = args.color.unwrap_or_default();
    if let Some(ref theme_name) = args.theme {
      apply_theme(&device, theme_name);
    }
    if let (Some(ref gradient1), Some(ref gradient2)) =
      (args.gradient1.as_ref(), args.gradient2.as_ref())
    {
      apply_gradient(&device, gradient1, gradient2);
    }
    if args.rainbow {
      apply_rainbow(&device);
    }
    if color.len() > 0 {
      apply_color(&device, color.as_str());
    }
    if !args.rainbow
      && color.len() == 0
      && args.theme.is_none()
      && args.gradient1.is_none()
      && args.gradient2.is_none()
    {
      apply_theme(&device, "default");
    }
  }
}
