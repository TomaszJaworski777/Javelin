use std::{fmt::Debug, str::FromStr};

use dashmap::DashMap;
use once_cell::sync::Lazy;
use unicase::UniCase;

static OPTIONS: Lazy<Options> = Lazy::new(Options::new);
pub struct Options {
    map: DashMap<UniCase<String>, EngineOption>,
}
impl Options {
    fn new() -> Self {
        let mut options = Self { map: DashMap::new() };

        options.add_option("Hash", "64", OptionType::Spin(1, 65536));
        options.add_option("MoveOverhead", "10", OptionType::Spin(0, 500));

        options
    }

    fn add_option(&mut self, name: &'static str, value: &'static str, option_type: OptionType) {
        self.map.insert(
            UniCase::new(name.to_string()),
            EngineOption::new(name.to_string(), &value.to_string(), option_type),
        );
    }

    pub fn get<'a>(key: &'a str) -> EngineOption {
        let key: UniCase<String> = UniCase::new(key.to_string());
        if let Some(entry) = OPTIONS.map.get(&key) {
            return entry.clone();
        }
        println!("Option {key} doesn't exist.");
        return EngineOption::NONE;
    }

    pub fn set<'a>(key: String, new_value: String) {
        let key: UniCase<String> = UniCase::new(key);
        if let Some(mut entry) = OPTIONS.map.get_mut(&key) {
            entry.change_value(new_value);
        } else {
            println!("Option {key} doesn't exist.");
        }
    }

    pub fn print() {
        for entry in OPTIONS.map.iter() {
            entry.value().print();
        }
    }
}

#[derive(Clone)]
pub struct EngineOption {
    name: String,
    value: String,
    default: String,
    option_type: OptionType,
}
#[allow(unused)]
impl EngineOption {
    const NONE: Self =
        Self { name: String::new(), value: String::new(), default: String::new(), option_type: OptionType::Check };

    fn new(name: String, value: &String, option_type: OptionType) -> Self {
        Self { name, value: value.clone(), default: value.clone(), option_type }
    }

    fn change_value<'a>(&mut self, new_value: String) {
        match self.option_type {
            OptionType::Check => {
                if new_value == "true" || new_value == "false" {
                    self.value = new_value
                }
            }
            OptionType::Spin(min, max) => {
                let converted_value = new_value.parse::<i32>();
                if let Err(_) = converted_value {
                    return;
                }
                let unwrapped = converted_value.unwrap();
                if unwrapped >= min && unwrapped <= max {
                    self.value = new_value
                }
            }
            _ => self.value = new_value,
        }
    }

    pub fn get_value<T>(&self) -> T
    where
        T: FromStr + Default,
        <T as FromStr>::Err: Debug,
    {
        if let OptionType::Spin(_, _) = self.option_type {
            self.value.parse::<T>().unwrap()
        } else {
            T::default()
        }
    }

    pub fn print(&self) {
        match self.option_type {
            OptionType::Check => println!("option name {} type check default {}", self.name, self.default),
            OptionType::Spin(min, max) => {
                println!("option name {} type spin default {} min {} max {}", self.name, self.default, min, max)
            }
            OptionType::String => println!(
                "option name {} type string default {}",
                self.name,
                if self.default == "" { "<empty>" } else { self.default.as_str() }
            ),
            _ => return,
        }
    }
}

#[allow(dead_code)]
#[derive(Clone, Copy)]
enum OptionType {
    Check,
    Spin(i32, i32),
    String,
}
