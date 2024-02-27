fn main() {
    outra_funcao(5, 6);
}

fn outra_funcao(x: i32, y: i32) {
    println!("O valor de x é: {}", x);
    println!("O valor de y é: {}", y);
}

/*Declarações são instruções que executam alguma ação 
e não retornam um resultado. E expressões retornam um 
resultado. Vamos ver alguns exemplos. */

//Exemplo de Expressao:
fn expressao() {
    let x = 5;

    let y = {
        let x = 3;
        x + 1
    };

    println!("O valor de y é: {}", y); // 4
}

/*Funções com valor de retorno */

/*Funções podem retornar valores para o código que 
os chama. Não nomeamos valores de retorno, mas declaramos 
o tipo deles depois de uma seta (->). Em Rust, o valor de 
retorno da função é sinônimo do valor da expressão final 
no bloco do corpo de uma função. Você pode retornar cedo 
de uma função usando a palavra-chave return e especificando 
um valor, mas a maioria das funções retorna a última expressão 
implicitamente. Veja um exemplo de uma função que retorna um valor: */

fn cinco() -> i32 {
    // valor de retorno da função
    5

    //corpo da função é um 5 solitário sem ponto e 
    //vírgula porque é uma expressão cujo valor 
    //queremos retornar.
}

fn funcao() {
    let x = cinco();

    println!("O valor de x é: {}", x);
}

//outro exemplo:

fn funcao2() {
    let x = soma_um(5);

    println!("O valor de x é: {}", x);
}

fn soma_um(x: i32) -> i32 {
    x + 1
}
