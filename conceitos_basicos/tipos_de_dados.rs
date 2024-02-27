/*Tipos inteiros */

// Tamanho	Signed	Unsigned
// 8-bit	i8	u8
// 16-bit	i16	u16
// 32-bit	i32	u32
// 64-bit	i64	u64
// arch	isize	usize

/*Cada variante pode ser com ou sem sinal e 
ter tamanho explícito. Signed e unsigned refere-se 
à possibilidade do número ser negativo ou positivo
em outras palavras, se o número precisa de um sinal 
com ele (signed) ou se sempre for positivo pode ser 
representado sem um sinal (unsigned). É como escrevemos 
números no papel: Quando o sinal importa, o número é mostrado 
com um sinal de mais ou menos; contudo, quando é seguro 
assumir que o número é positivo, é mostrado sem sinal. */

//i8 pode armazenar números de -128 até 127
//u8 pode armazenar números de 0 até 255
//i16 de -32768 até 32767
//u16 de 0 até 65535
//i32 de -2147483648 até 2147483647
//u32 de 0 até 4294967295
//i64 -9223372036854775808 até 9223372036854775807
//u64 0 até 18446744073709551615

/*Tipos de ponto flutuante */

/*Os pontos flutuantes do Rust são f32 e f64, que têm 
respectivamente os tamanhos de 32 e 64 bits. O tipo padrão 
é f64 porque nos processadores modernos, a velocidade é quase 
a mesma que em um f32, mas possui maior precisão. */

/*O tipo booleano */

fn bool() {
    let t = true;
    let f: bool = false; // com tipo explícito
}

/*O tipo de caractere */

//char é específicado com aspas simples
fn char() {
    let c = 'z';
    let z = 'ℤ';
    let heart_eyed_cat = '😻';
}

/*O tipo composto */

//Rust só tem 2 tipos compostos (de listas): tupla e matriz

/*Criamos uma tupla escrevendo uma lista de valores 
separados por vírgula dentro de parênteses. Cada posição 
da tupla tem um tipo e os tipos dos elementos da tupla 
não necessitam serem iguais. */

fn tupla() {
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    //Pegando os valores da tupla individualmente fazendo desestruturação
    //quebrando uma única tupla em três partes:
    let (x, y, z) = tup;

    println!("O valor do y é: {}", y); // 6.4

    /* Além de desestruturar através da correspondência de padrões 
    podemos acessar diretamente um elemento da tupla usando um ponto
    como índice do valor que queremos acessar. Por exemplo: */

    let x: (i32, f64, u8) = (500, 6.4, 1);

    let quinhentos = x.0;

    let seis_ponto_quatro = x.1;

    let um = x.2;

    //não conseguimos adicionar valores na tupla
}

/*O tipo matriz */

/*Matrizes em Rust são diferentes de matrizes de outras linguagens
porque matrizes em Rust são de tamanhos fixos: uma vez declarado, 
eles não podem aumentar ou diminuir de tamanho.
Em Rust, os valores que entram numa matriz são escritos em uma lista 
separados por vírgulas dentro de colchetes: */

fn matriz() {
    let a = [1, 2, 3, 4, 5];

    /*Um exemplo de quando você poderia necessitar usar uma matriz no 
    lugar de um vetor é um programa em que você precisa saber o nome 
    dos meses do ano. É improvável que tal programa deseje adicionar 
    ou remover meses, então você pode usar uma matriz porque você 
    sabe que sempre conterá 12 itens: */

    let meses = ["Janeiro", "Fevereiro", "Março", "Abril", "Maio", 
        "Junho", "Julho", "Agosto", "Setembro", "Outubro", 
        "Novembro", "Dezembro"];
}

//se pedir pro rust compilar esse programa, vai resultar em erro
//pois o rust verifica se o índice especificado é menor que o tamaho da matriz

fn panic() {
    let a = [1, 2, 3, 4, 5];
    let indice = 10;

    let elemento = a[indice];

    println!("O valor do elemento é: {}", elemento);
}

/*Esse é o primeiro exemplo dos pricípios de segurança do Rust em ação. 
Em várias linguagens de baixo nível, esse tipo de verificação não é feita e 
quando você fornece um índice incorreto, memória inválida pode ser acessada. 
Rust protege você deste tipo de erro ao dar erro na compilação, em vez de 
permitir o acesso à memória */