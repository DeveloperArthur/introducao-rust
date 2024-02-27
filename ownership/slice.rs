// conceito igual do golang: https://github.com/DeveloperArthur/golang/blob/main/src/slice/slice.go

/*Isto é similar a pegar uma referência à String inteira, mas 
com um [0..5] a mais. Em vez de uma referência à String inteira, 
trata-se de uma referência a uma porção da String. */

fn main1() {
    let s = String::from("texto longo");

    let texto = &s[0..5];
    let longo = &s[6..11];

    println!("{}", s);
    println!("{}", texto);
    println!("{}", longo);
}

/*Internamente, a estrutura de dados de uma slice armazena a 
posição inicial e o tamanho da slice, que corresponde a índice_final 
menos índice_inicial. Então, no caso do let longo = &s[6..11];, longo 
seria uma slice que contém um ponteiro para o sétimo byte de s (índice 6) 
e um tamanho igual a 5. 
(confira na imagem https://rust-br.github.io/rust-book-pt-br/img/trpl04-06.svg) */
//------------------------------------------------------------------------------------------
//Funcao que retorna a primeira palavra de uma frase

fn main2() {
    let mut s = String::from("texto longo");

    let palavra = primeira_palavra(&s);

    println!("a primeira palavra é {}", palavra)
}

//passando referencia da string pra nao ser dropada após execucao da funcao
fn primeira_palavra(s: &String) -> &str {
    let bytes = s.as_bytes();

    //iter().enumerate retorna uma tupla com 1 posicao sendo indice
    //e segunda posicao sendo endereco de memoria do valor
    for (i, &item) in bytes.iter().enumerate() {
        //b' ' é representa um espaço usando a sintaxe de byte literal
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..] //retorna a string inteira
}

fn main3() {
    let mut s = String::from("texto longo");

    let palavra = primeira_palavra(&s);

    s.clear(); // ERRO mutable borrow occurs here

    println!("{}", palavra)

    /*se temos uma referência imutável (&s) para algum valor, não 
    podemos também obter uma referência mutável do mesmo. 
    Como clear requer uma referência mutável para s e você já tem 
    uma referência mutável para s em uso (passada para primeira_palavra)
    isso causa um erro de compilação. */

    //mesmo se a referencia de s fosse mutavel, o Rust nao deixaria compilar
    //pois ele garante que não haja mais de uma referência
    //mutável para a mesma variável em uso ao mesmo tempo
    
    //Em Rust também não é possível ter uma referência 
    //mutável e outra imutável para o mesmo valor em 
    //um determinado escopo

    //mas você pode ter várias referências imutáveis 
    //para o mesmo valor em um determinado escopo
}
//------------------------------------------------------------------------------------------
//O que acontece com a string se o slice for dropado?

fn main4() {
    // Criando uma string
    let original_string = String::from("texto longo");

    // Criando um slice que aponta para a substring "texto"
    let slice = &original_string[0..5];

    // Imprimindo o slice
    println!("Slice: {}", slice);

    // Chamando drop no slice (isso não afeta a string original)
    drop(slice);

    // Imprimindo a string original (ainda está intacta)
    println!("String original: {}", original_string);

    /*Ao executar este código, você verá que a original_string permanece 
    inalterada após chamar drop no slice, pois o slice apenas referencia 
    a original_string e não possui sua própria alocação de memória. 
    E o slice apenas deixa de existir*/
}
//------------------------------------------------------------------------------------------
//Strings Literais São Slices

fn main5() {
    let minha_string_literal = "Olá, mundo!"; // s é do tipo &str (é uma slice apontando
        // para aquele ponto específico do binário. por isso strings literais são imutáveis
        // &str é uma referência imutável)

    let minha_string = String::from("Olá, mundo!");
    
    //passando slice da string inteira
    let palavra = primeira_palavra_aprimorada(&minha_string[..]);

    //passando slice da string literal inteira
    let palavra = primeira_palavra_aprimorada(&minha_string_literal[..]);

    // uma vez que strings literais são slices de strings,
    // isso também funciona, sem nem usar sintaxe de slice!
    let palavra = primeira_palavra_aprimorada(minha_string_literal);
}

/*Se temos uma slice de string, podemos passá-la diretamente. 
Se temos uma String, podemos passar uma slice da String inteira. 
Definir uma função que recebe uma slice em vez de uma referência 
para uma String deixa nossa API mais genérica e útil 
sem perder nenhuma funcionalidade */
fn primeira_palavra_aprimorada(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
//------------------------------------------------------------------------------------------
//Outras Slices

fn main6() {
    let a = [1, 2, 3, 4, 5];

    let slice = &a[1..3];
}

/*Essa slice tem o tipo &[i32]. Ela funciona da mesma forma que 
as slices de string, armazenando uma referência para o primeiro 
elemento e um tamanho. Você vai usar esse tipo de slice para 
todos os tipos de coleções. */
//------------------------------------------------------------------------------------------
//Outro exemplo de slice:

fn maior(list: &[i32]) -> i32 {
    let mut maior = list[0];

    for &item in list.iter() {
        if item > maior {
            maior = item;
        }
    }

    maior
}

fn main() {
    let lista_numero = vec![34, 50, 25, 100, 65];
    let resultado = maior(&lista_numero);
    println!("O maior número é {}", resultado);
}

/* a função maior() recebe um slice, isso aqui > &[i32] < significa slice
é uma lista de referencias de i32
mesmo que não tenha uma segunda variavel recebendo uma fatia do vetor
como na linha 144...

pois quando passamos o endereço de memória do vetor
a variavel que está dentro da função recebe a fatia do vetor inteiro
por isso ele é um slice
*/