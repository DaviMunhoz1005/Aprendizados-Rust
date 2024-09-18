use std::io;

fn main() {
    
    let mut temperature = String::new();

    println!("Digite a temperatura que deseja converter para Fahreinheit: ");
    io::stdin()
        .read_line(&mut temperature)
        .expect("failed to read line!");

    let temperature : f32 = temperature.trim().parse().expect("Insira um valor positivo ou negativo!");

    let converted_temperature = (temperature * 5.0) / 9.0;

    println!("A temperatura convertida é aproximadamente {:.2} graus em Fahreinheit", converted_temperature);
}
