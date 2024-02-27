fn _if() {
    let numero = 3;

    if numero < 5 {
        println!("condição era verdadeira");
    } else {
        println!("condição era falsa");
    }

    if numero != 0 {
        println!("número era algo diferente de zero");
    }

    //o Rust só executa o bloco para a primeira condição verdadeira e
    //depois de encontrar um, não verifica o restante.

    if numero % 4 == 0 {
        println!("número é divisível por 4");
    } else if numero % 3 == 0 {
        println!("número é divisível por 3");
    } else if numero % 2 == 0 {
        println!("número é divisível por 2");
    } else {
        println!("número não é divisível por 4, 3 ou 2");
    }

    //Pelo fato de if ser uma expressão
    //podemos usá-la do lado direito de
    //uma declaração let:

    let condicao = true;
    let numero = if condicao {
        5
    } else {
        6
    };
}

/*A palavra-chave loop diz ao Rust para 
executar um bloco de código várias vezes 
para sempre ou até que você diga 
explicitamente para parar. */
fn _loop() {
    loop {
        println!("novamente!");
    }
}

fn _while() {
    let mut numero = 3;

    while numero != 0 {
        println!("{}!", numero);

        numero = numero - 1;
    }

    println!("LIFTOFF!!!");
}

fn while_matriz() {    
    let a = [10, 20, 30, 40, 50];
    let mut indice = 0;

    while indice < a.len() {
        println!("O valor é: {}", a[indice]);

        indice = indice + 1;
    }
}

fn for() {
    let a = [10, 20, 30, 40, 50];

    for elemento in a.iter() {
        println!("O valor é: {}", elemento);
    }
}

fn contagem_regressiva() {
    for numero in (1..4).rev() {
        println!("{}!", numero);
    }
    println!("LIFTOFF!!!");
}