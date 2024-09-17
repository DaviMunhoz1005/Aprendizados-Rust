fn main() {
    
    let mut counter = 0;

    let result = loop {
        
        counter += 1;
        if counter == 5 {

            break counter * 2; //retorno para o result
        }
    };
    println!("{}", result);

    counter = 0;

    'loop1: loop {
        
        let mut remaining = 10;

        'loop2: loop {
            
            counter += 1;
            remaining -= 1;

            if counter == remaining {

                println!("loop 1");
                break 'loop1;
            } else if remaining < counter{
                
                println!("loop 2");
                break 'loop2;
            }
        }
    }
}