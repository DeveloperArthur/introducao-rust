use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Red"), 50);

    //---------------------------------------------------
    /* usando o método collect para transformar 
    um vetor de tuplas em um HashMap */

    let teams = vec![String::from("Blue"), String::from("Red")];
    let initial_scores = vec![10, 50];

    let scores: HashMap<_, _> = teams
        .iter()
        .zip(initial_scores.iter())
        .collect();

    println!("{:?}", scores);

    //---------------------------------------------------
    //Hash Maps e Ownership

    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // field_name e field_value saem de escopo...

    //---------------------------------------------------
    //Acessando Valores em um Hash Map

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Red"), 50);

    let field_value = String::from("Blue");

    let option = scores.get(&field_value); // assim field_value nao sai de escopo
    println!("{:?}", option);

    let option = scores.get("Red");
    println!("{:?}", option);

    //---------------------------------------------------
    //Iterando por Hash Map

    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    //---------------------------------------------------
    //Insira Apenas se a Chave Não Possui Valor

    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);

    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);

    println!("{:?}", scores);

    /*Este código imprimirá {"Yellow": 50, "Blue": 10}
    porque entry o método entry adiciona valor apenas
    se a chave não possuir, como Blue já possuia valor
    ele não sobrescreveu para 50e manteve 10, e como 
    Yellow nao tinha valor ele inseriu 50 */

    //---------------------------------------------------
    //Atualize um Valor com Base no Valor Antigo

    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {

        //insere 0 para a chave 'word' caso já nao tenha valor
        let count = map.entry(word).or_insert(0);

        /*map.entry retorna uma referencia mutavel do valor
        ou seja, um ponteiro, desreferenciando o ponteiro
        e atribuindo valor a ele, alteramos o valor no
        elemento do map */
        *count += 1;
    }

    println!("{:?}", map);
    //print: {"world": 2, "hello": 1, "wonderful": 1}
}