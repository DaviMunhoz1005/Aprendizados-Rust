use std::collections::HashMap;

fn main() {

    // mediana é 4 e 5 e moda é 2
    let mut vector = vec![5, 3, 6, 2, 7, 4, 1, 9, 8, 2, 1, 4, 3, 6, 5, 7, 8, 9, 2, 0];
    
    let length_vector = vector.len() as isize;
    if length_vector > 0 {
        // por conta da constante manipulação de índides - 1 então é recomendável utilizar isize
        quick_sort(&mut vector, 0, length_vector - 1);
        println!("{:?}", vector);
    }

    let median = median(&vector);
    let mode = mode(&vector);

    println!("A mediana é {}, e a moda é {}", median, mode);
}

fn median(vector: &Vec<i32>) -> f32 {
    let mid_index = vector.len() / 2;
    if vector.len() % 2 == 0 {
        (vector[mid_index] as f32 + vector[mid_index - 1] as f32) / 2.0
    } else {
        vector[mid_index] as f32
    }
}

fn mode(vector: &Vec<i32>) -> i32 {
    let mut frequency_values = HashMap::new();

    for &value in vector {
        // ou insere o valor ou incrementa um, por isso desreferencia
        *frequency_values.entry(value).or_insert(0) += 1;
    }
    // o iter e max_by_key pega o maior/ que aparece mais vezes
    // o unwrap pega a chave do maior valor em primeiro lugar
    *frequency_values.iter().max_by_key(|entry| entry.1).unwrap().0
}

fn quick_sort(vector: &mut Vec<i32>, low: isize, high: isize){
    if low < high {
        let pivot_index = partition(vector, low, high);
        quick_sort(vector, low, pivot_index - 1);
        quick_sort(vector, pivot_index + 1, high);
    }
}

fn partition(vector: &mut Vec<i32>, low: isize, high: isize) -> isize {
    let pivot = vector[high as usize];
    let mut i = low - 1;
    for j in low..high {
        if vector[j as usize] <= pivot {
            i += 1;
            vector.swap(i as usize, j as usize);
        }
    }
    vector.swap((i + 1) as usize, high as usize);
    i + 1
}