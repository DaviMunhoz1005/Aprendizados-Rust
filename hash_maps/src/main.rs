use std::collections::HashMap;

fn main() {
    /*
    hashmap guarda conjuntos de chaves associadas a valores <key, value>
    Armazena seus dados no monte (heap)
    */

    // assim se instancia e insere valores
    let mut score_teams = HashMap::new();
    score_teams.insert(String::from("Corinthians"), 7);
    score_teams.insert(String::from("São Paulo"), 5);

    // o get retorna um optional, se não existir retorna none
    // utiliza-se o copied para o Tipo ser Optional<i32> não Optional<&i32>
    // usa o unwrap_or para pegar o valor se for nulo ela vem como o número que colocamos
    // se a chave não existir retorna o mesmo do unwrap_or
    let score_corinthians = score_teams.get("Palmeiras").copied().unwrap_or(0);
    println!("{}", score_corinthians);

    // para atualizar podemos sobrescrever o valor de acordo com a chave ou incrementar ou apenas se a chave existir

    // sobrescrevendo
    score_teams.insert(String::from("São Paulo"), 1);

    // se a chave não existe ele cria, se existe ele não atualiza
    // esse entry é para checar a existência da chave
    // o or_insert é para para se existir não faz, se não existir, cria e faz
    score_teams.entry(String::from("Corinthians")).or_insert(8); // não atualiza
    score_teams.entry(String::from("Palmeiras")).or_insert(5); // cria e insere

    // atualizando com base no valor anterior
    // aqui ele pega o valor e se não existir a chave ele cria e retorna o 0 ou o valor existente
    // depois eu desreferencio e manipulo o valor real na memória
    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    // esse split_whitespace retorna un iterator através dos pedaços separando por espaço
    for word in text.split_whitespace() {

        // se a chave não existe, cria e valor padrão é zero
        let count = map.entry(word).or_insert(0);

        // desreferencia
        *count += 1;
    }

    // é possível iterar dessa forme
    for (key, value) in &score_teams {
        println!("{} {}", key, value)
    }
}
