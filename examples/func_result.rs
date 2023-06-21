fn parse_string(first_number_str: &str) -> i32 {
    // Let's try using `unwrap()` to get the number out. Will it bite us?
    let first_number = first_number_str.parse::<i32>().unwrap();
    //let second_number = second_number_str.parse::<i32>().unwrap();
    //first_number * second_number
    first_number
}

fn return_str(return_str: &str) -> &str {
    return_str
}

fn main() {
    let twenty: i32 = parse_string("10");
    println!("parse_string ,double is {}", twenty);

let return_str = return_str("return_str");
println!("{}",return_str);

}