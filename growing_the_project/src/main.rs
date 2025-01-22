pub mod garden; // indicando o módulo, ele vai rodar junto com a main
use crate::garden::vegetables::Aspargus as AAAA; // importando do módulo
// é possível nomear para haver diferenciação se preferir

// o código dos módulos é privado por padrão
// a palavra chave use serve para deixar globalmente no escopo
// mas daria erro se o escopo mudar, por exemplo 
// tentar utilizar a importação em um módulo dentro do escopo

// se preferir e quiser utilizar a importação em todo o escopo
// apenas coloque o use como pub

// é uma boa prática, se for importar um método, importar o caminho do mod do método
// mas não o método em si, fazer isso apenas na hora de utilizar o método
// por deixa mais limpo o código e deixa claro aonde o método foi criado

// use crate::garden::vegetables
// depois disso, quando for usar o método escreve
// vegetables::eatVegetables();

// já para estruturas é melhor utilizar o caminho inteiro mesmo

// para utilizar pacotes externor temos que adicionálos ao Cargo.toml
// é possível fazer importações nested, por exemplo
// use std::cmp::Ordering;
// use std::io;
// esses dois juntos viram - use std::{cmp::Ordering, io};
// tem tbm o exemplo de repetição
// use std::io;
// use std::io::Write;
// os dois juntos ficam - use std::io::{self, Write};
// também é possível importar tudo utilizando * - use std::collections::*;

fn main() {
    let _aspargus = AAAA::new();
    println!("Asparagus instance created!");
}
