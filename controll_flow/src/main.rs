fn main() {
    
    let condition = true;
    let ten = if condition { 10 } else { 0 };
    let number = {
        if ten == 10 {
            10/2
        } else {
            10
        }
    };
    println!("{}", number);
    if number % 4 == 0 {
        println!("Divisível por 4");
    } else if number % 3 == 0 {
        println!("Divisível por 3");
    } else if number % 2 == 0 {
        println!("Divisível por 2");
    } else {
        println!("Não divisível por 4, 3 e 2");
    }
}
