fn main() {
    
    const PI : f32 = 3.1415;
    println!("{:.2}", PI);

    let number = 10;
    println!("{}", number);
    {
        let number = number + 10;
        println!("{}", number);
    }

    let spaces_in_a_string = "     ";
    println!("{}", spaces_in_a_string.len());
}
