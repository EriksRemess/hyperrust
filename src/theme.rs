use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
pub struct Theme {
  name: String,
  keys: Map<String, Value>,
}

pub fn available_themes() -> HashMap<&'static str, &'static str> {
  let mut themes: HashMap<&'static str, &'static str> = HashMap::new();
  themes.insert("default", include_str!("../themes/default.json"));
  themes
}

pub fn get_theme_names() -> Vec<&'static str> {
  let themes = available_themes();
  themes.keys().map(|k| *k).collect()
}

pub fn get_theme_colors(name: &str) -> Result<Vec<String>, Box<dyn std::error::Error>> {
  let themes = available_themes();

  if let Some(theme_str) = themes.get(name) {
    let theme: Theme = serde_json::from_str(theme_str)?;
    let parsed_theme: Vec<String> = parse_theme(theme);
    return Ok(parsed_theme);
  } else {
    return Err("Theme not found".into());
  }
}

const KEYS: [&str; 71] = [
  // first row
  "KEY_UNUSED_0",
  "KEY_ESC",
  "KEY_1",
  "KEY_2",
  "KEY_3",
  "KEY_4",
  "KEY_5",
  "KEY_6",
  "KEY_7",
  "KEY_8",
  "KEY_9",
  "KEY_0",
  "KEY_MINUS",
  "KEY_EQUALS",
  "KEY_UNUSED_1",
  "KEY_BACKSPACE",
  // second row
  "KEY_TAB",
  "KEY_Q",
  "KEY_W",
  "KEY_E",
  "KEY_R",
  "KEY_T",
  "KEY_Y",
  "KEY_U",
  "KEY_I",
  "KEY_O",
  "KEY_P",
  "KEY_LEFT_BRACKET",
  "KEY_RIGHT_BRACKET",
  "KEY_BACKSLASH",
  // third row
  "KEY_CAPSLOCK",
  "KEY_A",
  "KEY_S",
  "KEY_D",
  "KEY_F",
  "KEY_G",
  "KEY_H",
  "KEY_J",
  "KEY_K",
  "KEY_L",
  "KEY_SEMICOLON",
  "KEY_QUOTE",
  "KEY_UNUSED_2",
  "KEY_ENTER",
  // fourth row
  "KEY_LSHIFT",
  "KEY_UNUSED_3",
  "KEY_Z",
  "KEY_X",
  "KEY_C",
  "KEY_V",
  "KEY_B",
  "KEY_N",
  "KEY_M",
  "KEY_COMMA",
  "KEY_PERIOD",
  "KEY_FORWARD_SLASH",
  "KEY_UNUSED_4",
  "KEY_RSHIFT",
  // fifth row
  "KEY_LCTRL",
  "KEY_SUPER",
  "KEY_LALT",
  "KEY_LSPACE",
  "KEY_SPACE",
  "KEY_RSPACE",
  "KEY_RALT",
  "KEY_MENU",
  "KEY_RCTRL",
  "KEY_UNUSED_5",
  "KEY_UNUSED_6",
  "KEY_UNUSED_7",
  "KEY_RFUNC",
];

pub fn parse_theme(theme: Theme) -> Vec<String> {
  let mut result: Vec<String> = Vec::new();
  for key in KEYS.iter() {
    let value = theme.keys.get(&key.to_string());
    match value {
      Some(v) => match v {
        Value::String(s) => {
          result.push(s.to_string());
        }
        _ => {
          println!("Invalid value type for key: {}", key);
        }
      },
      None => {
        println!("Key not found: {}", key);
      }
    }
  }
  result
}
