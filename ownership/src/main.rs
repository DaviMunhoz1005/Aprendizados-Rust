fn main() {
    
    let mut string = String::from("Davi");
    string.push_str(" lindão");

    let second_string = string.clone(); // esse clone é caro para a performace pq ele clona o dado do heap(monte)

    println!("string: {}, second_string: {}", string, second_string);

    let word = String::from("hello");
    takes_ownership(word);
    // println!("{}", word); => esse código da erro pq a variavel foi movida para o parametro

    let number = 10;
    copy_value(number);
    println!("{}", number);

    let string_take = gives_ownership();
    println!("string_take: {}", string_take);

    let (string_lost_and_take, string_len) = takes_and_gives_back_ownership(string_take);
    println!("string_lost_and_take: {}, tamanho: {}", string_lost_and_take, string_len);
}

fn takes_ownership(word : String) {
    
    println!("takes_ownership: {}", word);
}

fn copy_value(number : i32) {

    println!("{}", number);    
}

fn gives_ownership() -> String {
    
    let s = String::from("Owner");
    s
}

fn takes_and_gives_back_ownership(str : String) -> (String, usize) { 
    // mais recomendável caso eu queria q continue válido no meu main
    let length = str.len();
    (str, length)
}
