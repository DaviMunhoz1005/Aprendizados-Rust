#[derive(Debug)]
enum Types {
    Integer(i32),
    Float(f32),
    Text(String)
}

fn main() {

    /* 
    Dessa forma, criando um enum com os tipos, é possível guardar "diferentes tipos de dados"
    mas na verdade todos são do tipo Types que é o nome do Enum
    */
    let my_types = vec![
        Types::Integer(10), 
        Types::Float(3.1415), 
        Types::Text(String::from("Texto"))
    ];
    for i in &my_types {
        println!("{:?}", i);
    }
}