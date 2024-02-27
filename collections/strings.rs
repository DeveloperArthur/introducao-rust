fn main() {
    //Anexando a uma String com Push

    let mut s1 = String::from("foo");
    let s2 = String::from("bar");
    s1.push_str(&s2);
    //saida: foobar

    let mut s = String::from("lo");
    s.push('l');
    //saida: lol

    //----------------------------------------------
    //A concatenação de strings
    let s1 = String::from("Hello, ");
    let s2 = String::from("World!");

    let s3 = s1 + &s2;

    println!("{}", s3);

    /*Uma coisa importante de perceber é que
    a variavel s1 não poderá mais ser utilizada
    ela é removida de escopo, isso porque 
    o operador + utiliza o método:

    fn add(self, s: &str) -> String {}
    
    ou seja, internamente isso ocorre:    */
    s1.add(&s2);

    /*por isso estamos passando uma string
    e uma referencia de memória
    e consequentemente s1 nao sera mais 
    valido depois disso...
    
    o método add() faz basicamente:  */
    s1.push_str(&s2);
    
    /* a diferença é que s1 continua válida 
    com push_str, mas com add não...

    isso acontece porque o método add toma 
    posse de self:
    
    fn add(self, s: &str) -> String {}
    
    enquando push_str usa a referencia:  
    
    fn push_str(&mut self, s: &str) {}
    
    detalhe: push_str nao retorna nada, pois
    modifica o valor por referencia, o método
    add, como ele tira a string self de escopo
    ele retorna self
    ou seja, poderiamos fazer isso:  */

    let s3 = s1.add(&s2);

    /* o método add modifica o valor de 
    s1 e o retorna, s1 fica invalida, mas 
    s3 contem o valor retornado por add */

    //----------------------------------------------
    //Concatenando varias string e usando format

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s4 = format!("{}-{}-{}", s1, s2, s3);

    println!("{}", s4);

    //----------------------------------------------
    //Acessando caracteres de uma string

    let s = String::from("abcde");
    let a = s[0]; // <- ERRO, nem compila

    /* o Rust não permite fazer esse acesso devido
    a forma como as string são armazenadas na memória
    
    Como as strings em Rust são codificadas em UTF-8
    nem todos os bytes correspondem a um único caractere 
    visível. Alguns caracteres podem ocupar mais de um byte 
    na codificação UTF-8, e para determinar o índice correto 
    de um caractere em uma string UTF-8, o Rust precisaria 
    analisar a string byte a byte para encontrar 
    o caractere correto.

    Devido a essas considerações de desempenho e complexidade, 
    o Rust optou por não permitir o acesso direto aos caracteres 
    de uma String por índice*/

    //----------------------------------------------
    //Fatiando Strings

    let hello = "Здравствуйте";
    /* Uma pessoa que pergunte pelo comprimento da string 
    pode dizer que ela deva ter 12.No entanto, a resposta de 
    Rust é 24. Este é o número de bytes que é necessário para 
    codificar “Здравствуйте“ em UTF-8, uma vez que cada valor 
    escalar Unicode leva dois bytes de armazenamento. 
    
    Assim sendo, um índice nos bytes da string NEM SEMPRE se 
    correlaciona com um valor escalar Unicode válido. */

    let s = &hello[0..4];
    
    println!("{}", s); 
    /* s = "Зд" porque 4 bytes equivalem a 2 letras.
    se fizessemos:  */
    let s = &hello[0..1];

    /* A resposta: entrará em pânico em tempo de execução, 
    da mesma maneira que acessar um índice inválido 
    em um vetor */

    let s = String::from("abcde");
    let a = s[0..1];

    //----------------------------------------------
    //Métodos para Interagir Sobre Strings
    
    //chars
    let abcde = String::from("abcde");

    for c in abcde.chars() {
        println!("{}", c);
    }

    //O método bytes retorna cada byte bruto
    for b in abcde.bytes() {
        println!("{}", b);
    }
}
