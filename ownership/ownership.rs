/*Quando uma variável sai de escopo, o Rust automaticamente chama 
a função drop e limpa a memória da heap para esta variável */

fn main1() {
    // s não é válida aqui, ainda não está declarada

    let s = "olá"; // s é válida deste ponto em diante

    // faz alguma coisa com s

} // agora este escopo terminou, e s não é mais válida

//ou seja, Quando s entra no escopo, ela é válida.
//Permanece dessa maneira até que ela saia de escopo.

fn main2() {
    let s = String::from("texto"); // s é válida deste ponto em diante

    // faz alguma coisa com s

} // agora este escopo terminou, e s não é mais válida
//------------------------------------------------------------------------------------------
fn main3() {
    let s1 = String::from("texto");
    let s2 = s1;

    //Acabamos assumindo que a segunda linha faria uma cópia do 
    //valor em s1 e a associaria a s2. Mas não é exatamente isso que acontece.

    /*Quando atribuímos s1 a s2, os dados da String são copiados, o que 
    significa que estamos copiando o ponteiro, o tamanho e a capacidade 
    que estão na pilha. Não estamos copiando os dados que estão na heap, 
    aos quais o ponteiro se refere. */

    /*Ou seja, a memória da variável s2 tem uma cópia do ponteiro, 
    tamanho e capacidade de s1. */

    /*Ou seja, os dois ponteiros estão apontando para o mesmo lugar na memória. */

    /* Isso é um problema: quando s2 e s1 saem de escopo, os dois vão tentar 
    liberar a mesma memória. Isso é conhecido como erro de double free (liberação dupla), 
    e é um dos bugs de segurança de memória que mencionamos anteriormente. Liberar memória 
    duas vezes pode levar à corrupção da memória, o que pode, por sua vez, trazer potenciais 
    vulnerabilidades de segurança. */
}
//------------------------------------------------------------------------------------------
fn main4() {
    /*Para garantir a segurança de memória, há um outro detalhe sobre o que 
    acontece nesta situação em Rust. Em vez de tentar copiar a memória alocada, 
    o Rust considera que s1 deixa de ser válida, e portanto, o Rust não precisa 
    liberar nenhuma memória quando s1 sai de escopo. Veja só o que acontece quando 
    você tenta usar s1 depois que s2 é criada, não vai funcionar: */

    let s1 = String::from("texto");
    let s2 = s1;

    // println!("{}", s1); //ERROR value borrowed here after move (NEM COMPILA)

    /*Isso resolve o nosso problema! Tendo apenas s2 
    válida, quando ela sair de escopo, somente ela 
    vai liberar a memória, e pronto. */
}

fn main5() {
    let s1 = String::from("texto");
    let s2 = s1.clone(); //copia os dados da heap

    //Isto funciona, e é assim que você pode
    //explicitamente copiar os dados da heap
    println!("s1 = {}, s2 = {}", s1, s2);
}
//------------------------------------------------------------------------------------------
fn main6() {
    let x = 5;
    let y = x;

    println!("x = {}, y = {}", x, y);

    /*este código parece contradizer o que acabamos de aprender: 
    não temos uma chamada ao método clone, mas x ainda é válido e 
    não foi movido para y.
    O motivo é que tipos como números inteiros têm um tamanho conhecido 
    em tempo de compilação e são armazenados inteiramente na pilha, e por 
    isso, cópias desses valores são rápidas de se fazer. Isso significa que 
    não há razão para impedir x de ser válido após criarmos a variável y. 
    Em outras palavras, não há diferença entre cópia rasa e profunda aqui, 
    então chamar o método clone não faria nada diferente de uma cópia rasa, 
    por isso podemos deixá-lo de lado. */
}
//------------------------------------------------------------------------------------------
fn main7() {
    let s = String::from("texto");  // s entra em escopo.

    toma_posse(s); // move o valor de s para dentro da função...
    // ... e ele não é mais válido aqui.

    // println!("{}", s); //ERRO (NAO DEIXA COMPILAR)
    //Essas verificações estáticas nos protegem de certo enganos.
}

fn toma_posse(uma_string: String) { // uma_string entra em escopo.
    println!("{}", uma_string);
} // Aqui, uma_string sai de escopo, e o método `drop` é chamado. A memória que
  // guarda seus dados é liberada.
//------------------------------------------------------------------------------------------
fn main8() {
    let x = 5; // x entra em escopo.                

    faz_uma_copia(x);   // x seria movido para dentro da função,
                        // mas i32 é Copy, então está tudo bem em
                        // usar x daqui para a frente.
} // Aqui, x sai de escopo

fn faz_uma_copia(um_inteiro: i32) { // um_inteiro entra em escopo.
    println!("{}", um_inteiro);
} // Aqui, um_inteiro sai de escopo. Nada de especial acontece.
//------------------------------------------------------------------------------------------
fn main9() {
    let s1 = entrega_valor();           // entrega_valor move o valor retornado
                                        // para s1.

    let s2 = String::from("texto");     // s2 entra em escopo.

    let s3 = pega_e_entrega_valor(s2);  // s2 é movido para dentro da função
                                        // pega_e_entrega_valor, que também
                                        // move o valor retornado para s3.
} // Aqui, s3 sai de escopo e é destruída. s2 sai de escopo, mas já foi movida,
  // então nada demais acontece. s1 sai de escopo e é destruída.

fn entrega_valor() -> String {               // entrega_valor move o valor
                                             // retornado para dentro da função
                                             // que a chamou.

    let uma_string = String::from("olá");    // uma_string entra em escopo.

    uma_string                               // uma_string é retornada e movida
                                             // para a função que chamou
                                             // entrega_valor.
}

// pega_e_entrega_valor vai pegar uma String e retorná-la.
fn pega_e_entrega_valor(uma_string: String) -> String { // uma_string entra em
                                                        // escopo.

    uma_string  // uma_string é retornada e movida para a função que chamou
                // pega_e_entrega_valor.
}
//------------------------------------------------------------------------------------------
//É possível retornar múltiplos valores usando uma tupla:
fn main10() {
    let s1 = String::from("texto");

    let (s2, tamanho) = calcula_tamanho(s1);

    println!("O tamanho de '{}' é {}.", s2, tamanho);
}

fn calcula_tamanho(s: String) -> (String, usize) {
    let tamanho = s.len(); // len() retorna o tamanho de uma String.

    (s, tamanho)
}