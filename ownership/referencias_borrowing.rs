/*O problema de usar tuplas como vimos em ./ownership.rs na main10(), precisamos retornar a 
String, de forma que ainda possamos usá-la após a chamada à função calcula_tamanho
para dentro da qual a String foi movida.

Aqui está uma forma de como você poderia definir e usar 
uma função calcula_tamanho que recebe uma referência para 
um objeto como parâmetro, em vez de pegar este valor para si: */

fn main11() {
    let s1 = String::from("texto");

    let tamanho = calcula_tamanho(&s1);

    println!("O tamanho de '{}' é {}.", s1, tamanho);
}

//Colocar referências como parâmetros de funções é chamado de borrowing
fn calcula_tamanho(s: &String) -> usize {
    s.len()
}

/*A sintaxe &s1 nos permite criar uma referência que se refere ao valor s1, 
mas não o possui. Como ela não o possui, o valor a que ela aponta não será 
destruído quando a referência sair de escopo. */
//------------------------------------------------------------------------------------------
//Alterando conteudo de uma variavel passando por referencia
fn main12() {
    let mut s = String::from("texto");

    //criando uma referência mutável
    modifica(&mut s);
}

//aceitando uma referência mutável
fn modifica(uma_string: &mut String) {
    uma_string.push_str(" longo");
}
//------------------------------------------------------------------------------------------
//Isso aqui dá erro
fn main() {
    let referencia_para_o_nada = soltar();
}

fn soltar() -> &String { // soltar retorna uma referência a uma String
    let s = String::from("texto"); // s é uma nova String

    &s  // retornamos uma referência a uma String, s
} // Aqui, s sai de escopo e é destruída. Sua memória é devolvida.
  // Perigo!

//Esse erro não acontece na função entrega_valor (./ownership.rs)
//porque a string que é retornada é destruida, a memória dela 
//é dropada, mas o valor retornado é movido para uma variável
//e neste caso a função soltar está retornando não um valor
//mas uma referencia, e essa referencia é dropada no fim da 
//função...

/*Como s é criada dentro da função soltar, quando o código 
desta função termina, s é desalocada. Mas nós tentamos retornar 
uma referência para ela. Isto significa que esta referência 
apontaria para uma String inválida! Isso não é bom. Rust não 
vai nos deixar fazer isso. */
//------------------------------------------------------------------------------------------
//A solução aqui é retornar a String diretamente:
fn soltar_corrigido() -> String {
    let s = String::from("texto");

    s
}
