fn main() {
    let word = String::from("hello");
    let second_word = String::from("aplle");
    let pig_latin_word = to_pig_latin(&word);
    let second_pig_latin_word = to_pig_latin(&second_word);
    println!("A palavra '{}' em Pig Latin é '{}'", word, pig_latin_word);
    println!("A palavra '{}' em Pig Latin é '{}'", second_word, second_pig_latin_word);
}

fn to_pig_latin(word: &String) -> String {
    
    // transformei a palavra em um array de caracteres
    let mut letters_word = word.chars();

    // peguei o primeiro caracter
    let first_letter = letters_word.next();

    // checa se existe, se sim continua
    if let Some(c) = first_letter {
        if is_consonant(c) {

            // split a palavra entre primeira letra e o restante
            let (_, rest) = word.split_at(1);

            // coloco o restante e ponho a primeira letra no final junto com "ay"
            let mut word_in_pig_latin = rest.to_string();
            word_in_pig_latin.push(c);
            word_in_pig_latin.push_str("ay"); 
            return word_in_pig_latin;
        } else {
            // apenas clono a palavra e coloco no final "way"
            let mut word_in_pig_latin = word.clone();
            word_in_pig_latin.push_str("way"); 
            return word_in_pig_latin;
        }
    }
    word.clone() 
}

fn is_consonant(caracter: char) -> bool {
    let vowels = ['a', 'e', 'i', 'o', 'u'];
    !vowels.contains(&caracter.to_ascii_lowercase())
}