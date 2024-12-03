const GIFTS: [&str; 12] = [
    "A partridge in a pair tree",
    "Two Turtle Doves",
    "Three French Hens",
    "Four Calling Birds",
    "FIVE GOLD RINGS",
    "Six Geese a Laying",
    "Seven Swans a Swimming",
    "Eight Maids a Milking",
    "Nine Ladies Dancing",
    "Ten Lords a Leaping",
    "Eleven Pipers Piping",
    "Twelve Drummers Drumming",
];

fn main() {
    for i in 1..=12 {
        println!("\n{}", day_of(i));
        for j in (1..=i).rev() {
            if j == 1 && i != 1 {
                println!("AND")
            }
            println!("{}", GIFTS[j - 1])
        }
    }
}

fn day_of(number: usize) -> String {
    let day = wordify_ordinal(number);
    format!("On the {day} day of Christmas, my true love gave to me:")
}

fn wordify_ordinal(number: usize) -> String {
    if number == 0 {
        return "Zeroth".to_string();
    }

    let mut parts = get_wordify_parts(number);
    let mut last_string = parts
        .last_mut()
        .unwrap()
        .split_whitespace()
        .map(|s| s.to_string())
        .collect::<Vec<String>>();

    if let Some(last_word) = last_string.last_mut() {
        *last_word = match last_word.as_str() {
            "One" => "First".to_string(),
            "Two" => "Second".to_string(),
            "Three" => "Third".to_string(),
            "Five" => "Fifth".to_string(),
            "Eight" => "Eighth".to_string(),
            "Nine" => "Ninth".to_string(),
            "Twelve" => "Twelfth".to_string(),
            _ if last_word.ends_with("y") => format!("{}ieth", &last_word[..last_word.len() - 1]),
            _ => format!("{last_word}th"),
        };
    }
 

    *parts.last_mut().unwrap() = last_string.join(" ");

    return parts.join(", ");
}

fn wordify_number(number: usize) -> String {
    return get_wordify_parts(number).join(", ");
}

fn get_wordify_parts(number: usize) -> Vec<String> {
    if number == 0 {
        let mut parts: Vec<String> = Vec::new();
        parts.push("Zero".to_string());
        return parts;
    }

    let magnitudes = ["", "Thousand", "Million", "Billion"];
    let mut num = number;
    let mut parts = Vec::new();

    for magnitude in magnitudes {
        if num == 0 {
            break;
        }

        let chunk = num % 1000;
        if chunk > 0 {
            let words = if magnitude == "" {
                wordify_hundreds(chunk)
            } else {
                format!("{} {}", wordify_hundreds(chunk), magnitude)
            };
            parts.insert(0, words)
        }
        num /= 1000;
    }

    return parts;
}

fn wordify_hundreds(number: usize) -> String {
    let mut parts = Vec::new();
    let hundreds = number / 100;

    if hundreds > 0 {
        parts.push(format!("{} Hundred", wordify_digit(hundreds)));
    }

    let remainder = number % 100;
    if remainder > 0 {
        if hundreds > 0 {
            parts.push("and".to_string());
        }
        parts.push(wordify_tens(remainder));
    }

    parts.join(" ")
}

fn wordify_tens(number: usize) -> String {
    if number < 10 {
        return wordify_digit(number);
    }

    if number < 20 {
        return wordify_teens(number);
    }

    let tens = number / 10;
    let digits = number % 10;

    let tens_word = String::from(match tens {
        2 => "Twenty",
        3 => "Thirty",
        4 => "Forty",
        5 => "Fifty",
        6 => "Sixty",
        7 => "Seventy",
        8 => "Eighty",
        9 => "Ninety",
        _ => panic!("\"{number}\" is out of range of the tens.")
    });

    if digits > 0 {
        format!("{} {}", tens_word, wordify_digit(digits))
    } else {
        tens_word.to_string()
    }
}

fn wordify_teens(number: usize) -> String {
    match number {
        10 => "Ten",
        11 => "Eleven",
        12 => "Twelve",
        13 => "Thirteen",
        14 => "Fourteen",
        15 => "Fifteen",
        16 => "Sixteen",
        17 => "Seventeen",
        18 => "Eighteen",
        19 => "Nineteen",
        _ => panic!("\"{number}\" is not a teen."),
    }.to_string()
}

fn wordify_digit(number: usize) -> String {
    match number {
        0 => "",
        1 => "One",
        2 => "Two",
        3 => "Three",
        4 => "Four",
        5 => "Five",
        6 => "Six",
        7 => "Seven",
        8 => "Eight",
        9 => "Nine",
        _ => panic!("\"{number}\" is not a digit."),
    }.to_string()
}
