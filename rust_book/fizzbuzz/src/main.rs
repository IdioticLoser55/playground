fn main() {
    let mut output = String::new();
    for i in 0..=100 {
        output.clear();

        if i % 3 == 0 {
            output += "fizz";
        }

        if i % 5 == 0 {
            output += "buzz";
        }

        if output.len() == 0 {
            output += &i.to_string();
        }

        println!("{i}: {output}");
    }

}
