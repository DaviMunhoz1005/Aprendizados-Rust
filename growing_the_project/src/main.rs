pub mod garden; // indicando o módulo, ele vai rodar junto com a main
use crate::garden::vegetables::Aspargus; // importando do módulo

// o código dos módulos é privado por padrão

fn main() {
    let _aspargus = Aspargus::new();
    println!("Asparagus instance created!");
}
