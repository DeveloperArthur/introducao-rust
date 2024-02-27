use std::io;
use std::fs::File;
use std::io::Read;
use std::io::ErrorKind;

//Erros Irrecuperáveis com panic!
fn panic(){
    let v = vec![1, 2, 3];

    v[99];

    /*Isso irá resultar em um panic
     Outras linguagens, como C, vão tentar te dar exatamente
     o que você pediu nessa situação, mesmo que não seja o que
     você quer: você vai receber o que quer que esteja na
     localização na memória que corresponderia àquele elemento
     no vetor, mesmo que a memória não pertença ao vetor.

     Isso se chama um buffer overread e pode levar a vulnerabilidades
     de segurança se um agressor for capaz de manipular o índice de
     forma a ler dados guardados depois do array aos quais ele não
     deveria ter acesso.

     Para proteger seu programa desse tipo de vulnerabilidade, se você
     tentar ler um elemento em um índice que não exista, Rust vai parar
     a execução e se recusar a continuar. */
}

//--------------------------------------------------------------------------------------------
//Erros recuperáveis com Result
fn result() {
    let f = File::open("hello.txt");

    let f = match f {
        Ok(file) => file,

        //tratando erro de arquivo inexistente
        Err(ref error) if error.kind() == ErrorKind::NotFound => {
            //tentar criar o arquivo
            //File::create retorna Result<T, E>
            // e estamos usando isso no match
            match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => {
                    panic!("Tentou criar um arquivo e houve um problema: {:?}", e)
                },
            }
        }

        Err(error) => {
            panic!("Houve um problema ao abrir o arquivo: {:?}", error)
        },
    };
}

//--------------------------------------------------------------------------------------------
//Atalhos para Pânico em Erro: unwrap e expect
fn result2(){
    /* unwrap é um método de atalho que é implementado justamente
    como o match que escrevemos lá em cima.

    Se o valor de Result for da variante Ok, unwrap vai retornar o
    valor dentro de Ok. Se o Result for da variante Err, unwrap
    vai chamar a macro panic! */

    let f = File::open("hello.txt").unwrap();

    /* expect é semelhante a unwrap, nos deixa também escolher
    a mensagem de erro do panic!. Usar expect em vez de unwrap
    e fornecer boas mensagens de erros podem transmitir sua
    intenção e tornar a procura pela fonte de pânico mais fácil */

    let f = File::open("hello.txt").expect("Falhou ao abrir hello.txt");

    /* Como essa mensagem de erro começa com o texto que especificamos,
    Falhou ao abrir hello.txt, será mais fácil encontrar o trecho do
    código de onde vem essa mensagem de erro. Se usamos unwrap em
    diversos lugares, pode tomar mais tempo encontrar exatamente qual
    dos unwrap está causando o pânico, dado que todas as chamadas a
    unwrap chamam o print de pânico com a mesma mensagem.*/
}

//--------------------------------------------------------------------------------------------
//Propagando Erros
fn read_username_from_file() -> Result<String, io::Error> {
    let f = File::open("hello.txt");

    //File::open retorna Result<T, E>
    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();

    //read_to_string retorna Result<T, E>
    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

fn main1() {
    let result = read_username_from_file();

    /* A funcao main vai receber ou um valor Ok que contém
    um nome de usuário ou um valor de Err que contém um io::Error.

    Nós não sabemos o que o código que chamou nossa função fará com
    esses valores. Se o código que chamou recebe um valor de Err,
    ele poderia chamar panic! e causar um crash, usar um nome de
    usuário padrão, ou procurar o nome de usuário em outro lugar
    que não um arquivo, por exemplo. Nós não temos informação o
    suficiente sobre o que o código que chamou está de fato tentando
    fazer, então propagamos toda a informação de sucesso ou erro para
    cima para que ele a trate apropriadamente. */

    if result.is_err() {
        panic!("ERROR: {:?}", result.err().unwrap());
    }

    println!("usuario: {:?}", result.unwrap());
}

//--------------------------------------------------------------------------------------------
//Um Atalho Para Propagar Erros: ?

/* Esse padrão de propagação de erros com match retornando Ok ou Err
é tão comum em Rust que a linguagem disponibiliza o operador de
interrogação ? para tornar isso mais fácil.

abaixo uma implementação de read_username_from_file que tem a
mesma funcionalidade que tinha na implementação acima, mas esta
implementação usa o operador de interrogação: */

fn read_username_from_file2() -> Result<String, io::Error> {
    let mut f = File::open("hello.txt")?;

    let mut s = String::new();
    f.read_to_string(&mut s)?;

    Ok(s)
}

//--------------------------------------------------------------------------------------------
//Uma forma mais enchuta ainda:
fn read_username_from_file3() -> Result<String, io::Error> {
    let mut s = String::new();

    File::open("hello.txt")?.read_to_string(&mut s)?;

    Ok(s)
}

/* Lembrando que ? só pode ser usado dentro de funções
que tem um tipo de retorno de Result<T, E>

Em funções que não retornam Result, quando você chama outras
funções que retornam Result, você deve usar um match ou um dos
métodos de Result para tratá-lo em vez de usar ? para
potencialmente propagar o erro ao código que a chamou.*/

//--------------------------------------------------------------------------------------------
// Exemplo de tratamento de erros:
fn main() {
    let res: Result<u8, io::Error> = soma(10, 1);

    if res.is_err() {
        panic!("{:?}", res.err().unwrap());
    }

    println!("{}", res.unwrap())
}

fn soma(x: u8, y: u8) -> Result<u8, io::Error> {
    let res = x + y;

    if res > 10 {
        let error = io::Error::new(
            io::ErrorKind::Other,
            "Total maior que 10"
        );

        return Err(error);
    }

    return Ok(res);
}