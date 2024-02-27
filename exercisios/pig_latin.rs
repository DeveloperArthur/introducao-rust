/*Converta strings para Pig Latin, onde a primeira consoante de cada palavra é movida    
para o final da palavra adicionado um "ay" , então “first” se torna “irst-fay”.    
Palavras que começam com uma vogal recebem “hay” adicionado ao final (“apple”    
    torna-se “apple-hay”). Lembre-se sobre a codificação UTF-8! */

fn main() {
    let mut input = String::from("abacaxi");
    let mut input2 = String::from("banana");
    let mut input3 = String::from("cachorro");
    let mut input4 = String::from("dente");
    let mut input5 = String::from("elefante");
    let mut input6 = String::from("icone");

    println!("{}", traduz_para_pig_latin(&mut input));
    println!("{}", traduz_para_pig_latin(&mut input2));
    println!("{}", traduz_para_pig_latin(&mut input3));
    println!("{}", traduz_para_pig_latin(&mut input4));
    println!("{}", traduz_para_pig_latin(&mut input5));
    println!("{}", traduz_para_pig_latin(&mut input6));
}

fn traduz_para_pig_latin(palavra: &mut str) -> String {
    let primeira_letra = &palavra[0..1];

    let mut palavra = palavra.to_string();

    if primeira_letra == "a" || primeira_letra == "e" || 
        primeira_letra == "i" || primeira_letra == "o" || 
        primeira_letra == "u" {

        palavra.push_str("-hay");

        return palavra;
    } else {
        palavra.remove(0);

        let ultima_parte = "-".to_owned() + &primeira_letra.to_string() + "ay";
        
        palavra.push_str(&ultima_parte);

        return palavra;
    }
}