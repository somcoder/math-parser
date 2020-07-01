// extern crate lalrpop_util;
use lalrpop_util::*;

lalrpop_mod!(pub calc);

mod core;

pub use self::core::{ Converter, Value, Unit };
pub use self::calc::ExpressionParser;

#[cfg(test)]
mod tests {
    use self::core::{Unit, Value, Converter};
    use crate::*;
    use std::collections::HashMap;

    #[test]
    fn it_works() {
        let converter = setup();
        let parser = calc::ExpressionParser::new();
        let result: Result<
            Value,
            lalrpop_util::ParseError<usize, lalrpop_util::lexer::Token, &str>,
        > = parser.parse(&converter, "45 - 20%");

        match result {
            Ok(value) => {
                println!("45 - 20% is {}", value);
                assert_eq!(36., value.number);
            },
            Err(error) => {
                println!("Error: {:?}", error);
            }
        }
    }

    #[test]
    fn power_works() {
        // let parser = calc::ExpressionParser::new();
        // let result = parser.parse("2.0 ^ 4");
        // assert!(result.is_ok(), true);
        // let number: Value = result.expect("Could not parse this!");
        // assert_eq!((2.0 as f64).powi(4), number.number);
        // println!("2^4 = {}", number);
    }

    #[test]
    fn units_work() {
        let converter = setup();
        let parser = calc::ExpressionParser::new();
        let value: Result<Value, _> = parser.parse(&converter, "1m + 1000mm");

        let total = value.unwrap();
        println!("1m + 1000mm = {}", total);
        assert_eq!(2., total.number);
    }

    #[test]
    fn conversion_works() {
        let converter = setup();
        let two_hours = Value::new(1200., Unit::new("second"));
        let minutes = converter.convert(&two_hours, &Unit::new("minute"));
        println!("{} to minutes: {}", two_hours, minutes);
        
        let cents = Value::new(200., Unit::new("cm"));
        let meters = converter.convert(&cents, &Unit::new("m"));
        println!("{} = {}", cents, meters);
        
        let mm = converter.convert(&meters, &Unit::new("mm"));
        println!("{} = {}", meters, mm);
    }

    fn setup<'a>() -> Converter<'a> {
        let mut conversions = HashMap::new();
        conversions.insert((Unit::new("day"), Unit::new("hour")), 24.);
        conversions.insert((Unit::new("hour"), Unit::new("minute")), 60.);
        conversions.insert((Unit::new("minute"), Unit::new("second")), 60.);
        conversions.insert((Unit::new("second"), Unit::new("millisecond")), 60.);
        conversions.insert((Unit::new("km"), Unit::new("m")), 1000.);
        conversions.insert((Unit::new("m"), Unit::new("cm")), 100.);
        conversions.insert((Unit::new("cm"), Unit::new("mm")), 10.);

        Converter::new(conversions)
    }
}
