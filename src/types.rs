#![allow(dead_code)]

pub struct Move{
    pub value: u16
}

pub struct Square{
    pub value: u8
}
impl Square {
    pub const NULL: Square = Square {
        value: 64
    };

    pub fn new( value: u8 ) -> Self {
        Square {
            value: value
        }
    }

    pub fn to_string( &self ) -> String{
        if self.value == 64 {
            return "NULL".to_string();
        }

        let file: u8 = self.value % 8;
        let rank: u8 = ((self.value as f32) / 8_f32).floor() as u8 + 1;
        return format!("{}{}", ('a' as u8 + file) as char, rank);
    }

    pub fn convert_to_string( value: u8 ) -> String{
        if value == 64 {
            return "NULL".to_string();
        }

        let file: u8 = value % 8;
        let rank: u8 = ((value as f32) / 8_f32).floor() as u8 + 1;
        return format!("{}{}", ('a' as u8 + file) as char, rank);
    }

    pub fn create_from_string( square: &str ) -> Square {
        let mut result = Square::new(0);
        let signatures : Vec<char> = square.chars().collect();
        let file = signatures[0] as u8 - 'a' as u8;
        let rank = signatures[1].to_string().parse::<u8>().unwrap() - 1;
        result.value = rank * 8 + file;
        result
    }

    pub fn from_string( &mut self, square: &str ) {
        let signatures : Vec<char> = square.chars().collect();
        let file = signatures[0] as u8 - 'a' as u8;
        let rank = signatures[1].to_string().parse::<u8>().unwrap() - 1;
        self.value = rank * 8 + file;
    }
}