use std::io;

fn main() {
    let mut nth_number = String::new();

    println!("Insira o n-ésimo número de Fibonacci que você quer:");
    io::stdin()
        .read_line(&mut nth_number)
        .expect("Falha ao ler a linha!");

    let nth_number: u32 = nth_number.trim().parse().expect("Insira um número válido positivo");

    if nth_number == 0 {

        println!("O 0° número de Fibonacci é 0");
        return;
    } else if nth_number == 1 {
        
        println!("O 1° número de Fibonacci é 1");
        return;
    }

    let mut prev = 0;
    let mut curr = 1;

    for _ in 2..=nth_number {
        let next = prev + curr;
        prev = curr;
        curr = next;
    }

    println!("O {}° número de Fibonacci é {}", nth_number, curr);
}
