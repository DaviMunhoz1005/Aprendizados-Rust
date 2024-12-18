fn main() {

    let mut some_number = Some(5); // aqui é como se fosse opcional de algo
    // se tentarmos adicionar (a + b) com um número não Option dará um erro

    let number = some_number.take(); // quando usa take, você pega mas não devolve, ele fica none de quem tu pegou

    let absent_number : Option<i32> = None; //aqui é como se fosse opcional de nada
    println!("{:?}", some_number);
    println!("{:?}", absent_number);
    println!("{:?}", number);
}