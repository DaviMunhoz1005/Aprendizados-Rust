fn main() {

    let mut  tuple : (f64, bool, isize, &str, char) = (2.3, true, -10, "davi", 'D');
    tuple.3 = "Davi";
    println!("{:?}", tuple);

    let (float, boolean, negative_number, string, caracter) = tuple;
    println!("{}, {}, {}, {}, {}", caracter.to_ascii_lowercase(), string, negative_number, boolean, float);

    let mut array : [u16; 5] = [1, 2, 3, 4, 5];
    array[0] = 0;
    println!("{:#?}", array);

    let array = ['a'; 10];
    println!("{:?}", array);
}