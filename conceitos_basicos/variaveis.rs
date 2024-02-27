/*Se uma parte do seu código funciona assumindo que o valor nunca será alterado 
e outra parte do seu código muda este valor, é possível que a primeira parte do 
código não faça o que foi projetada para fazer. A causa desse tipo de falha pode 
ser difícil de rastrear, especialmente quando o segundo trecho de código muda o 
valor apenas algumas vezes. */

/*A imutabilidade das variaveis é uma das maneiras que o Rust lhe dá para 
escrever o seu código de modo seguro e a fácil concorrência que Rust oferece. */

fn imutavel() {
    let x = 5;
    println!("O valor de x é: {}", x);
    x = 6; //ERROR: cannot assign twice to immutable variable
}

fn constante() {
    // exemplo de constante declarada
    const PONTOS_MAXIMOS: u32 = 100_000;
}

fn shadowing() {
    /*Esse programa primeiro vincula x ao valor 5. 
    Em seguida x é sombreado por let x = , pegando 
    o valor original e adicionando 1, então o valor 
    de x é 6. O terceiro let também sombrea x
    multiplicando o valor anterior por 2 para então 
    x ficar com o valor final de 12 */
    let x = 5;
    println!("O valor de x é: {}", x);
    let x = x + 1;
    println!("O valor de x é: {}", x);
    let x = x * 2;
    println!("O valor de x é: {}", x);
    
    /*digamos que nosso programa solicite ao 
    usuário que mostre quantos espaços deseja 
    entre um texto, inserindo caracteres de espaço, 
    mas queremos armazenar essa entrada como um número*/
    let espacos = "   ";
    let espacos = espacos.len();

    /*Essa construção é permitida, porque a primeira variável 
    espacos é do tipo string e a segunda variável, que é uma 
    nova variável que tem o mesmo nome que a primeira, é do 
    tipo numérico. Shadowing nos poupa de ter de criar nomes 
    diferentes, como str_espacos e num_espacos */

    /*No entanto, se tentassemos usar mut para isso, como 
    mostramos aqui, teremos um erro em tempo de compilação: */
    let mut espacos = "   ";
    espacos = espacos.len(); //ERROR expected &str, found usize
}




