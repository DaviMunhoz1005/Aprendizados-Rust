fn main() {

    println!("Hello!");
    name_to_dysplay(my_name(), -17);

    let x = {
        let y = 3;
        y + 2
    };

    println!("{}", x + number_five() + plus_ten(x));
}

fn name_to_dysplay(name : &str, number : i32){

    println!("{}, {}", name, number);
}

fn number_five() -> u8 {
    5
}

fn plus_ten(x : u8) -> u8 {
    x + 10
}

fn my_name() -> &'static str  {
    "Davi"
}
