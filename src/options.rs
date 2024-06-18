use once_cell::sync::Lazy;
use std::sync::{Arc, RwLock};

static OPTIONS: Lazy<Options> = Lazy::new(Options::new);

macro_rules! create_option_structs {
    ($($name:ident: $type:ty => $new_expr:expr, $option_name:expr),* $(,)?) => {
        pub struct Options {
            $(pub $name: $type,)*
        }

        impl Options {
            fn new() -> Self {
                Self {
                    $($name: $new_expr,)*
                }
            }

            pub fn set(key: &str, new_value: &str) {
                match key {
                    $($option_name => Self::update_option(&OPTIONS.$name, new_value),)*
                    _ => println!("Option {} doesn't exist.", key),
                }
            }

            pub fn print() {
                $(
                    OPTIONS.$name.print($option_name);
                )*
            }

            $(
                pub fn $name() -> <$type as OptionTrait>::ValueType {
                    OPTIONS.$name.get()
                }
            )*

            fn update_option<T: OptionTrait>(option: &T, new_value: &str) {
                option.set(new_value);
            }
        }
    };
}

create_option_structs!(
    hash: SpinOptionInt => SpinOptionInt::new(64, 1, 65536), "Hash",
    move_overhead: SpinOptionInt => SpinOptionInt::new(10, 0, 500), "MoveOverhead",
    root_pst: SpinOptionFloat => SpinOptionFloat::new(4.5, 0.1, 10.0), "RootPST",
    non_root_pst: SpinOptionFloat => SpinOptionFloat::new(1.0, 0.1, 10.0), "NonRootPST",
    root_c: SpinOptionFloat => SpinOptionFloat::new(1.41, 0.1, 10.0), "RootC",
    non_root_c: SpinOptionFloat => SpinOptionFloat::new(1.41, 0.1, 10.0), "NonRootC",
);

#[allow(dead_code)]
pub trait OptionTrait {
    type ValueType;
    fn set(&self, new_value: &str);
    fn get(&self) -> Self::ValueType;
    fn print(&self, name: &str);
}

pub struct SpinOptionInt {
    value: Arc<RwLock<i32>>,
    default: i32,
    min: i32,
    max: i32,
}

impl SpinOptionInt {
    fn new(value: i32, min: i32, max: i32) -> Self {
        Self { value: Arc::new(RwLock::new(value)), default: value, min, max }
    }

    fn set_value(&self, new_value: i32) {
        if new_value >= self.min && new_value <= self.max {
            *self.value.write().unwrap() = new_value;
        } else {
            println!("Value out of range.");
        }
    }

    fn get(&self) -> i32 {
        *self.value.read().unwrap()
    }
}

impl OptionTrait for SpinOptionInt {
    type ValueType = i32;

    fn set(&self, new_value: &str) {
        if let Ok(parsed_value) = new_value.parse::<i32>() {
            self.set_value(parsed_value);
        } else {
            println!("Invalid value for option.");
        }
    }

    fn get(&self) -> i32 {
        self.get()
    }

    fn print(&self, name: &str) {
        println!("option name {} type spin default {:?} min {:?} max {:?}", name, self.default, self.min, self.max);
    }
}

pub struct SpinOptionFloat {
    value: Arc<RwLock<f32>>,
    default: f32,
    min: f32,
    max: f32,
}

impl SpinOptionFloat {
    fn new(value: f32, min: f32, max: f32) -> Self {
        Self { value: Arc::new(RwLock::new(value)), default: value, min, max }
    }

    fn set_value(&self, new_value: i32) {
        let adjusted = new_value as f32 / 100.0;
        if adjusted >= self.min && adjusted <= self.max {
            *self.value.write().unwrap() = adjusted;
        } else {
            println!("Value out of range.");
        }
    }

    fn get(&self) -> f32 {
        *self.value.read().unwrap()
    }
}

impl OptionTrait for SpinOptionFloat {
    type ValueType = f32;

    fn set(&self, new_value: &str) {
        if let Ok(parsed_value) = new_value.parse::<i32>() {
            self.set_value(parsed_value);
        } else {
            println!("Invalid value for option.");
        }
    }

    fn get(&self) -> f32 {
        self.get()
    }

    fn print(&self, name: &str) {
        println!("option name {} type spin default {:?} min {:?} max {:?}", name, (self.default * 100.0) as i32, (self.min * 100.0) as i32, (self.max * 100.0) as i32);
    }
}

pub struct CheckOption {
    value: Arc<RwLock<bool>>,
    default: bool
}

#[allow(dead_code)]
impl CheckOption {
    fn new(value: bool) -> Self {
        Self { value: Arc::new(RwLock::new(value)), default: value }
    }

    fn set_value(&self, new_value: bool) {
        *self.value.write().unwrap() = new_value;
    }

    fn get(&self) -> bool {
        *self.value.read().unwrap()
    }
}

impl OptionTrait for CheckOption {
    type ValueType = bool;

    fn set(&self, new_value: &str) {
        if let Ok(parsed_value) = new_value.parse::<bool>() {
            self.set_value(parsed_value);
        } else {
            println!("Invalid value for option.");
        }
    }

    fn get(&self) -> bool {
        self.get()
    }

    fn print(&self, name: &str) {
        println!("option name {} type check default {}", name, self.default);
    }
}

pub struct StringOption {
    value: Arc<RwLock<String>>,
    default: String
}

#[allow(dead_code)]
impl StringOption {
    fn new(value: String) -> Self {
        Self { value: Arc::new(RwLock::new(value.clone())), default: value }
    }

    fn set_value(&self, new_value: String) {
        *self.value.write().unwrap() = new_value;
    }

    fn get(&self) -> String {
        self.value.read().unwrap().clone()
    }
}

impl OptionTrait for StringOption {
    type ValueType = String;

    fn set(&self, new_value: &str) {
        self.set_value(new_value.to_string());
    }

    fn get(&self) -> String {
        self.get()
    }

    fn print(&self, name: &str) {
        println!("option name {} type string default {}", name, self.default);
    }
}
