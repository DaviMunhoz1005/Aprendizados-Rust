fn main() {
    
    // borrowing ou references, empréstimo ou referência

    let mut string = String::from("Davi");

    {
        let string2 = &mut string;
        string2.push_str("AB");
    }

    let string3 = &mut string;
    println!("{}", string3);

    let length = calculate_length(&mut string); // & simbolo para referência, mutável ou não

    println!("String: {}, Length: {}", string, length);
}

fn calculate_length(str : &mut String) -> usize { 

    // se eu quiser modificar os dados dessa referência eu preciso colocar essa referência mutável, por padrão ela é imutável

    str.push_str("-AAAA");
    str.len()
}