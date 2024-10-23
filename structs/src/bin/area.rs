#[derive(Debug)]
struct Rectangle {
    width : usize,
    heigth : usize
}

fn main() {

    let rectangle = Rectangle {
        width: dbg!(5 * 2),
        heigth: 20
    };

    println!("{:#?}", rectangle);
    dbg!(&rectangle); // outra forma de printar, ele printa o caminho e a linha na qual esta sendo printado
    println!("A área do retângulo é {}",  get_area(&rectangle));
}

fn get_area(rectangle: &Rectangle) -> usize {
    
    rectangle.heigth * rectangle.width
}