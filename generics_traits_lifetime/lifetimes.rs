/* O principal alvo de lifetimes é prevenir referências soltas,
quais fazem com que o programa referencie dados quais nós não
estamos querendo referenciar. */

fn main1() {
    let r;

    {
        let x = 5;
        r = &x;
    }

    println!("r: {}", r);
    //ERRO: `x` does not live long enough
    // estamos tentando usar uma referencia de x, mas x saiu de escopo
}

//-------------------------------------------------------------------
//Tempos de Vida Génericos em Funções
fn main() {
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let resultado = maior(string1.as_str(), string2);
    println!("A string mais longa é {}", resultado);
}

fn maior1(x: &str, y: &str) -> &str { //ERRO:  missing lifetime specifier
    if x.len() > y.len() { // ^ expected named lifetime parameter
        x
    } else {
        y
    }

    /* "help: this function's return type contains a borrowed value, but the
   signature does not say whether it is borrowed from `x` or `y"

   O texto de ajuda está nos dizendo que o tipo de retorno precisa de um
   parâmetro de tempo de vida genérico nele porque o Rust não pode dizer
   se a referência que está sendo retornada se refere a x ou y.
   Atualmente, nós também não sabemos, já que o bloco if no corpo dessa
   função retorna uma referência para x e o bloco else retorna uma
   referência para y!

   Enquanto estamos definindo essa função, não sabemos os valores
   concretos que serão passados para essa função, então não sabemos
   se o caso if ou o caso else será executado. Nós também não sabemos os
   tempos de vida concretos das referências que serão passadas, então não
   podemos determinar que a referência que retornaremos sempre será válida.
   O verificador de empréstimos não consegue determinar isso também porque
   não sabe como os tempos de vida de x e y se relacionam com o tempo de
   vida do valor de retorno. Nós vamos adicionar parâmetros genéricos de
   tempo de vida que definirão a relação entre as referências para que o
   verificador de empréstimos possa fazer sua análise. */
}

//-------------------------------------------------------------------
//Sintaxe de Anotação de Tempo de Vida

/* Anotações de tempo de vida não mudam quanto tempo qualquer uma das
referências envolvidas viverão. Do mesmo modo que funções podem aceitar
qualquer tipo de assinatura que especifica um parâmetro de tipo genérico,
funções podem aceitar referências com qualquer tempo de vida quando a
assinatura especificar um parâmetro genérico de tempo de vida.
O que anotações de tempo de vida fazem é relacionar os tempos de vida de
múltiplas referências uns com os outros.

Anotações de tempo de vida tem uma sintaxe levemente incomum: os nomes dos
parâmetros de tempos de vida precisam começar com uma apóstrofe '.
Os nomes dos parâmetros dos tempos de vida são usualmente todos em caixa
baixa, e como tipos genéricos, seu nome usualmente são bem curtos. 'a é
o nome que a maior parte das pessoas usam por padrão. Parâmetros de
anotações de tempos de vida vão depois do & de uma referência, e um espaço
separa a anotação de tempo de vida do tipo da referência.

Aqui vão alguns exemplos: nós temos uma referência para um i32 sem um
parâmetro tempo de vida, uma referência para um i32 que tem um parâmetro
de tempo de vida chamado 'a: */

fn exemplo() {
    &i32        // uma referência
    &'a i32     // uma referência com um tempo de vida explícito
    &'a mut i32 // uma referência mutável com um tempo de vida explícito

    //Todos os literais de string têm um tempo de vida static
    let s: &'static str = "Eu tenho um tempo de vida estático.";
}

//-------------------------------------------------------------------
//Anotações de Tempo de Vida em Assinaturas de Funções

/* A limitação que queremos dar ao Rust é que para as referências nos
parâmetros e o valor de retorno devem ter o mesmo tempo de vida */

fn maior<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

/* A assinatura de função agora diz que pra algum tempo de vida 'a,
a função receberá dois parâmetros, ambos serão cortes de string que
vivem pelo menos tanto quanto o tempo de vida 'a. A função retornará
um corte de string que também vai durar tanto quanto o tempo de vida 'a.
Esse é o contrato que estamos dizendo ao Rust que queremos garantir.

O Rust faz isso para garantirmos que nunca vamos retornar um
endereço de memória inválido

Especificando os parâmetros de tempo de vida nessa assinatura de função,
não estamos modificando os tempos de vida de quaisquer valores passados
ou retornados, mas estamos dizendo que quaisqueres valores que não
concordem com esse contrato devem ser rejeitados pelo verificador de
empréstimos. Essa função não sabe (ou não precisa saber) exatamente
quanto tempo x e y vão viver, apenas precisa saber que existe algum
escopo que pode ser substituído por 'a que irá satisfazer essa
assinatura.

Ah mas em ./ownership/slice.rs linha 36 tem uma
função chamada primeira_palavra nós tínhamos uma
função que usava referencias e mesmo assim compilava
sem anotações de tempo de vida, PORQUE?!

resumidamente por causa de um conjunto de regras e casos chamada
Elisão de Tempo de Vida particular que o compilador irá considerar
e tentar encaixar, e se seu código se encaixa nas regras, você não
precisa escrever os tempos de vida explicitamente, ele mesmo infere

para saber mais: https://rust-br.github.io/rust-book-pt-br/ch10-03-lifetime-syntax.html#elis%C3%A3o-de-tempo-de-vida */