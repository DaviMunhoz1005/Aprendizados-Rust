fn main() {
    
    // slices é fatias

    let string = String::from("Davi Marques");
    let first_word = first_word(&string);
    
    println!("{}", first_word);

    let array = [1, 2, 3, 4];
    let slice = &array[..2];
    println!("{:?}", slice)
}

fn first_word(str : &str) -> &str {
    
    let bytes = str.as_bytes();

    for(i, &item) in bytes.iter().enumerate() {

        if item == b' ' {

            return &str[..i];
        }
    }

    &str[..]
    // escrever [0..10] ou [..10] é a mesma coisa, [2..(último index))] [2..]  [..]
}