fn main() {
    // instancia uma string para guardar dados
    let mut my_name = String::new();
    
    // coloca valor nessa instância de string
    my_name = "Davi".to_string();

    // também é possivel instanciar com um valor já
    let last_name = String::from(" Munhoz");

    // podemos aumentar uma string / atualizar com push_str ou push
    my_name.push_str(" Marques de Andrade"); // apenas String
    my_name.push('1'); // apenas caracter

    // é possível concatenar com o operador + ou usando format!
    let junta_tudo = format!("{my_name}{last_name}");
    
    // a razão pelo qual o primeiro não é referência e o segundo é está na declaração do método
    let my_full_name = my_name + &last_name;

    println!("{}", my_full_name);
    println!("{}", junta_tudo);

    // a linguagem rust não permite indexar uma string, exemplo my_name[0]
    // mas permite criar corter/slices, mas os slices são dos bytes, 
    // exemplo s = my_name[0..4] pegaria a primeia e segunda letra que ocupa 2 bytes cada

    // existem formas de percorrer uma string com char
    for c in "DM".chars() {
        println!("{c}");
    }

    // ou pelo seus bytes
    for b in "DM".bytes() {
        println!("{b}");
    }

    // também possui métodos como contains ou replace
}
