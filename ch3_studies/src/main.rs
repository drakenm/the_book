use std::{io,time, fmt::format};
fn main() {

    /************************
    * TWELVE DAY OF CHRISTMAS
    *************************/
    println!("I will not recite the twelve days of christmas, but I will print them out for you...\n");

    print_the_twelve_days_of_christmas();


    /**********************
    * FIBONACCI CALCULATOR
    ***********************/
    println!("Hello, fibo?!");
    println!("Please enter the nth value of the fibonacci sequence you would like:");

    let n: i32 = get_number_from_user();

    let now = time::Instant::now();

    let F: i64 = get_nth_fibonacci_number( n );

    let elapsed_time = now.elapsed();

    println!("The fibo number, where n = {}, is {}", n, F);
    println!("As an aside, the nth fibonacci number took {}.{}µs seconds to determine.", elapsed_time.as_secs(), elapsed_time.subsec_micros());


    /**********************
    * TEMPERATURE CONVERTER
    ***********************/
    println!("Welcome to the temperature converter...");
    ask_for_temp_to_convert()

}


fn get_day_of_christmas( n: usize ) -> [&'static str; 2] {
    let day: [[&str; 2]; 13] = [
        ["first", "a"],
        ["second", "two"],
        ["third", "three"],
        ["fourth", "four"],
        ["fifth", "five"],
        ["sixth", "six"],
        ["seventh", "seven"],
        ["eighth", "eight"],
        ["ninth", "nine"],
        ["tenth", "ten"],
        ["eleventh", "eleven"],
        ["twelfth", "twelve"],
        ["thirteenth", "thirteen"],
    ];
    return day[n];
}


fn print_the_twelve_days_of_christmas() {
    let mut song: String = String::new();
    for i in 0..=11 {
        let ordinal: &str = get_day_of_christmas(i)[0];
        let num: &str = get_day_of_christmas(i)[1];
        
        let item: String = match i {
            0 => format!("{} partridge and a pear tree.\n\n", num),
            1 => format!("{} turtle doves, and\n", num),
            2 => format!("{} french hens,\n", num),
            3 => format!("{} colly birds,\n", num),
            4 => format!("{} gold rings,\n", num),
            5 => format!("{} geese a laying,\n", num),
            6 => format!("{} swans a swimming,\n", num),
            7 => format!("{} maids a milking,\n", num),
            8 => format!("{} drummers drumming,\n", num),
            9 => format!("{} pipers piping,\n", num),
            10 => format!("{} ladies dancing,\n", num),
            11 => format!("{} lords a leaping,\n", num),
            _ => format!("Error"),
        };
        song = format!("{}{}", item, song);
        println!("{}", format!("The {} day of Christmas\nMy true love sent to me\n{}", ordinal, song));
    }
}


fn get_nth_fibonacci_number( n: i32 ) -> i64 {
    if n == 0 {
        return 0;
    } else if n == 1 {
        return 1;
    } else {
        return get_nth_fibonacci_number( n - 1 ) + get_nth_fibonacci_number( n - 2 );
    }
}


fn get_number_from_user() -> i32 {
    loop {
        let mut input: String = String::new();
        let reader = io::stdin();
        reader.read_line( &mut input )
            .ok()
            .expect("No input received...");
        let input: i32 = match input.trim().parse() {
            Ok( num ) => num,
            Err(_) => {
                println!("A number is required...");
                continue
            },
        };
        return input
    }
}


fn ask_for_temp_to_convert() {
    println!( "Enter a temperature to convert: " );
    loop {
        let mut temp: String = String::new();
        io::stdin()
            .read_line( &mut temp )
            .ok()
            .expect( "Failed to read line..." );
        let mut temp: String = temp.trim().to_string();
    
        let unit: char = match temp.pop() {
            Some( x ) => x,
            None => {
                println!( "Unable to pop from empty string..." );
                continue
            },
        };

        let unit: char = default_unit(unit);
        
        let value: i32 = match temp.trim().parse() {
            Ok( num ) => num,
            Err(_) => {
                println!( "An error occurred in the assignment of the leftover string to a i32" );
                continue
            },
        };

        let converted_temp: String = temperature_converter( value,unit );
        println!( "{converted_temp}" );
        break
    }
}


fn default_unit( unit: char ) -> char {
    if unit != 'f' && unit != 'c' {
        println!( "The unit provided is not valid, a correct value must be chosen to proceed... (f) farenheit or (c) celsius" );
        let reader = io::stdin();
        let mut unit: String = String::new();
        reader.read_line( &mut unit )
            .ok()
            .expect( "Failed to read line..." );
        let unit: char = unit.trim().chars().next().expect( "Unable to get first char from string..." );
        return default_unit( unit )
    } else {
        return unit
    }
}

fn temperature_converter( value: i32, unit: char ) -> String {
    let converted_unit: char = match unit {
        'c' => 'f',
        'f' => 'c',
        _ => unit
    };

    let converted_value: String = convert_temperature_value( value, unit ).to_string();
    format!( "The converted temperature is: {}{}", converted_value, converted_unit )
}


fn convert_temperature_value( value: i32, unit: char ) -> i32 {
    let converted_value: i32 = match unit {
        'c' => ( value * 9/5 ) + 32,
        'f' => ( value - 32 ) * 5 / 9,
        _ => value
    };
    converted_value
}