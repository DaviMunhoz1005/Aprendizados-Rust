fn main() {
    
    let mut counter = 0;
    let my_array : [f32; 3] = [1.1, 1.2, 1.3];

    while counter < my_array.len() {

        println!("{} no while", my_array[counter]);
        counter += 1;
    }

    for element in my_array {

        println!("{} no for", element);
    }
}