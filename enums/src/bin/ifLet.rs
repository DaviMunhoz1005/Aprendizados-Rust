fn main() {

    let config_max = Some(3u8);

    // Os dois códigos fazem a mesma coisa

    match config_max {
        Some(max) => println!("The max is {max}"),
        _ => (),
    }

    if let Some(max) = config_max {
        println!("The max is {max}");
    } else { // É o mesmo que '_ => ()' do pattern match
        println!("There is no max")
    }
}