#[derive(Debug)]
enum State {
    Alabama,
    Alaska
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(State),
}

fn main() {
    
    let penny_coin = Coin::Penny;
    let nickel_coin = Coin::Nickel;
    let dime_coin = Coin::Dime;
    let quarter_coin = Coin::Quarter(State::Alabama);
    
    let value_for_dime = value_in_cents(&dime_coin);

    println!("The value for de Dime Coin is {}", value_for_dime);

    print_value_in_cents(&penny_coin);
    print_value_in_cents(&nickel_coin);
    print_value_in_cents(&dime_coin);
    print_value_in_cents(&quarter_coin);
}

fn value_in_cents(coin: &Coin) -> u8 {

    // Esse match faz com que, se o tipo da moeda combinar, ele retorna o valor, parecido com if
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(_state) => 25,
    }
}

fn print_value_in_cents(coin: &Coin) {

    // Também é possível realizar a comparação com Optional<T>
    match coin {
        Coin::Quarter(state) => {
            println!("The state is {:?}", state);
        },

        // é um coringa (wildcard), se queremos usar o valor damos o nome other, se não queremos colocamos _
        _ => {
            println!("Isn't Quarter");
        },
    }
}
