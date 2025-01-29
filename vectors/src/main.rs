use std::vec;

fn main() {

    /* 
    Vec<T> ou também chamado de vectors ou vetores podem armazenar mais de um valor
    em uma única estrutura de dados com valores apenas do mesmo tipo.
    São úteis quando vc possui uma lista de itens, pode se comparar a um List<T> em java
    */

    /*
    Para criar um vetor é necessário deixar explícito o tipo que ele terá,
    no exemplo abaixo o tipo é i32
    */

    let mut vector: Vec<i32> = Vec::new(); 

    /*
    Outra forma de criar seria já instanciando com valores, 
    assim o rust infere o tipo dos valores inseridos.
    vec! é um macro que cria um vetor para guardar os valores informados
    */

    let vector_infer = vec![-1, 0, 1];

    /* 
    Para adicionar elementos usamos o método push
    */
    
    vector.push(-1);
    vector.push(0);
    vector.push(1);

    /* 
    Para ler os elementos de um vetor existem duas formas, pelo índice ou usando get(),
    utilizar o método get é mais seguro pois ele retorna um Optional<&T>
    Esse tipo de opcional é um opcional de referência para um tipo
    É uma boa prática, se for pegar o valor do vetor, utilizar referência
    senão vc perde a propriedade do valor no vetor
    */

    println!("First Element is {}", vector[0]);
    let second_element = vector_infer.get(1);

    // se o segundo elemento for presente ele printa
    if let Some(second) = second_element {
        println!("Second Element is {}", second);
    }

    /* 
    O vetor coloca os valores próximos um do outro na memória
    por conta disso se vc tentar adicionar um novo elemento no final
    significa que precisará alocar um novo local maior na memória
    copiando os elementos para o novo espaço,
    isso ocorre se não estiver mais espaço próximo para ele acrescentar.
    Nesse primeiro caso, significa que, se nós fizermos referencia a uma variável
    para um valor do vetor, e ela for imutável
    se o endereço da memória mudar, nós estaríamos fazendo referência para um local
    aonde a memória foi deslocada
    Isso resultaria em um erro na hora de compilar
    por exemplo:
    
        let third_element = &vector[2];
        vector.push(2);
        println!("Third Element {}", third_element);

    Esse exemplo daria erro pois está fazendo referência imutável a um elemento
    que pode mudar seu endereço, e por ser imutável, a variável continuaria
    apontando para a mesma memória que foi deslocada

    Essas são as regras de empréstimo que o rust possui    
    */

    /* 
    É possível iterar sobre os valores de um vetor, com uma referência mutável ou imutável
    */

    for i in &mut vector {
        *i += 1; // aqui desfaz a referência para modificar o valor
    }

    for i in &vector {
        println!("Iterating {}", i);
    }

    'removeElementsFromLast: loop {
        let element = vector.pop();
        match element {
            Some(_) => println!("Retirei o último elemento e removi do vetor"),
            None => {
                println!("Acabou os elementos"); 
                break 'removeElementsFromLast;
            }
        }
    }
}
