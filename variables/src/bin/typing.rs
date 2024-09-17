fn main() {
    
    let positive_number : u32 = 10;
    let negative_number : i32 = "-10".parse().expect("Não é um Número!");
    println!("{}, {}",positive_number, negative_number);

    let floatin_number : f32 = 10.32;
    println!("{}", floatin_number);

    let yes_or_no : bool = true;
    println!("{}", yes_or_no);

    let caracter : char = 'D';
    println!("{}", caracter.to_ascii_lowercase());
}
