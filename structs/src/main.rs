struct User { // são como objetos, iguais, possuim nome de atributos e valores
    name: String,
    age: u32,
    alive: bool
}

struct RGB (i32, i32, i32); // uma tupla sem os atributos apenas valores

struct AlwaysThis;

fn main() {
    
    let mut my_user = User {
        name: String::from("Davi"),
        age: 17,
        alive: true
    };

    my_user.name = String::from("Davi Marques");
    let username = &my_user.name;

    println!("{}", username);

    let user_build = build_user(String::from("Eu"), 20);
    println!("{}", user_build.name);

    let my_user2 = User {
        name: String::from("Os dois pontos (..) copia o restante do outro"),
        ..my_user
    };

    print!("{}", my_user2.age);

    let black = RGB(0, 0, 0);

    let variable = AlwaysThis;
}

fn build_user(name : String, age : u32) -> User {
    
    User {
        name,   // por conta de possuir o mesmo nome o parametro e atributo, pode colocar só o parametro
        age,
        alive: true
    }
}
