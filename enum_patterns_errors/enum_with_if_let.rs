//Controle de Fluxo Conciso com if let

/*A sintaxe do if let permite combinar if e let em uma forma 
menos verbosa de tratar apenas os valores que casam com um 
padrão e ignorar os demais. 

Veja o programa abaixo, que confere um valor do tipo 
Option<u8>, mas só executa um código se houver um valor 
associado igual a três: */

let algum_valor_u8 = Some(0u8);
match algum_valor_u8 {
    Some(3) => println!("três"),
    _ => (),
}

/*Queremos fazer alguma coisa com o Some(3), mas não queremos 
fazer nada com nenhum outro valor, seja Some<u8> ou None. 
Pra satisfazer a expressão match, temos que colocar _ => () 
após processar apenas uma variante, ou seja, é muito código 
para pouca coisa. 

Em vez disso, poderíamos escrever o mesmo código de uma 
forma mais compacta, usando if let: */

let algum_valor_u8 = Some(0u8);

if let Some(3) = algum_valor_u8 {
    println!("três");
}

/*if let recebe um padrão e uma expressão separados por um =. 
Isso funciona da mesma forma que um match, em que a expressão 
seria passada para o match, e o padrão apareceria 
no primeiro braço. 

Usar o if let implica menos código pra digitar e menos indentação. 
Porém, perdemos a verificação exaustiva que é garantida pelo match. 
A escolhe entre match e if let depende do que você está fazendo em 
uma situação particular, e se a redução no volume de código compensa 
a perda da verificação exaustiva. */