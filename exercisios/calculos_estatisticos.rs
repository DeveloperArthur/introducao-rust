/*Dada uma lista de inteiros, use um vetor e retorne a média, a mediana    
(quando classificado, o valor na posição do meio) e modo (o valor que    
    ocorre com mais frequência; um hash map será útil aqui) da lista.*/

use std::collections::HashMap;

fn main() {
    let lista_de_inteiros = vec![85, 92, 90, 66, 64, 72, 92];

    let mut media = 0;
    calcula_media(&lista_de_inteiros, &mut media);

    let mut mediana = 0;
    calcula_mediana(&lista_de_inteiros, &mut mediana);

    let mut moda = 0;
    calcula_moda(&lista_de_inteiros, &mut moda);

    println!("media: {}", media);
    println!("mediana: {}", mediana);
    println!("moda: {}", moda);
}

fn calcula_media(lista_de_inteiros: &Vec<usize>, media: &mut usize) {
    let mut count = 0;
    for i in lista_de_inteiros {
        count = count + i;
    }
    *media = count / lista_de_inteiros.len();
}

fn calcula_mediana(lista_de_inteiros: &Vec<usize>, mediana: &mut usize) {
    let tamanho_do_vetor_eh_par: bool = lista_de_inteiros.len() % 2 == 0;

    let mut lista_ordenada = vec![];
    merge_sort(lista_de_inteiros, &mut lista_ordenada);

    if tamanho_do_vetor_eh_par {
        let posicao_primeiro_valor = (lista_ordenada.len() / 2) - 1;
        let posicao_segundo_valor = posicao_primeiro_valor + 1;

        let primeiro_valor = lista_ordenada[posicao_primeiro_valor];
        let segundo_valor = lista_ordenada[posicao_segundo_valor];

        *mediana = (primeiro_valor + segundo_valor) / 2;
    } else {
        *mediana = lista_ordenada[lista_ordenada.len() / 2];
    }
}

fn calcula_moda(lista_de_inteiros: &Vec<usize>, moda: &mut usize) {
    let mut contador_frequencia = HashMap::new();

    for i in lista_de_inteiros {
        let count = contador_frequencia.entry(i).or_insert(0);
        *count += 1;
    }

    let mut frequencia_mais_alta = 0;
    for (elemento, frequencia) in contador_frequencia {
        if frequencia > frequencia_mais_alta {
            frequencia_mais_alta = frequencia;
            *moda = *elemento;
        }
    }
}

fn merge_sort(lista_de_inteiros: &[usize], lista_ordenada: &mut Vec<usize>) {
    // *lista_ordenada = vec![64, 66, 72, 85, 90, 92, 92];

    println!("lista_de_inteiros: {:?}", lista_de_inteiros);

    if lista_de_inteiros.len() <= 1 {
        lista_ordenada.extend_from_slice(lista_de_inteiros);
        println!("lista_ordenada: {:?}", lista_ordenada);
        return;
    }

    let meio = lista_de_inteiros.len() / 2;
    let (esquerda, direita) = lista_de_inteiros.split_at(meio);

    let mut lista_esquerda_ordenada = vec![];
    merge_sort(esquerda, &mut lista_esquerda_ordenada);

    let mut lista_direita_ordenada = vec![];
    merge_sort(direita, &mut lista_direita_ordenada);

    println!("antes do merge lista_esquerda_ordenada: {:?}", lista_esquerda_ordenada);
    println!("antes do merge lista_direita_ordenada: {:?}", lista_direita_ordenada);

    merge(&lista_esquerda_ordenada, &lista_direita_ordenada, lista_ordenada);

    println!("pós merge lista_ordenada: {:?}", lista_ordenada); // [90, 92]
    println!("pós merge lista_direita_ordenada: {:?}", lista_direita_ordenada); // [90]

    /* se lista_direita_ordenada tem valor [90], porque quando volta pro [85], 
    lista_direita_ordenada fica com valor [90, 92] do nada? 

    https://chat.openai.com/share/29194936-8663-401d-9612-fd25cd67dd7b

    porque nessa ultima iteracao o valor direita era 90
    lá na iteracao do 85, o valor direita era [92, 90]
    entao ele resgata o valor de direita que tinha, 
    e esse valor foi trabalhado, foi alterado, pq nessa iteracao
    a chamada pro metodo merge_sort recebeu o 
    o endereco de memoria de lista_direita_ordenada

    a lista_ordenada do metodo merge_sort recebe o endereco 
    de memoria de lista_direita_ordenada */
}

fn merge(esquerda: &[usize], direita: &[usize], lista_ordenada: &mut Vec<usize>) {
    let mut i = 0;
    let mut j = 0;

    while i < esquerda.len() && j < direita.len() {
        if esquerda[i] <= direita[j] {
            lista_ordenada.push(esquerda[i]);
            i += 1;
        } else {
            lista_ordenada.push(direita[j]);
            j += 1;
        }
    }

    lista_ordenada.extend_from_slice(&esquerda[i..]);
    lista_ordenada.extend_from_slice(&direita[j..]);
    println!("lista_ordenada: {:?}", lista_ordenada);
}