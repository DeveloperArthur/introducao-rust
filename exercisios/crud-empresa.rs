/*Usando um hash map e vetores, crie uma interface de texto para
permitir que um usuário adicione nomes de funcionários para um
departamento da empresa. Por exemplo, “Add Sally to Engineering”
ou “Add Amir to Sales”. Em seguida, deixe o usuário recuperar
uma lista de todas as pessoas de um departamento ou todas as
pessoas na empresa por departamento, ordenadas alfabeticamente. */

use std::collections::HashMap;
use std::io;

fn main() {
    let mut empresa = HashMap::<String, Vec<String>>::new();

    loop {
        println!("1-cadastrar funcionario");
        println!("2-listar funcionarios por departamento");
        println!("3-listar todos os funcionarios");
        println!("para sair digite CTRL+C");

        let mut input = String::new();

        io::stdin().read_line(&mut input)
            .expect("Falha ao ler entrada");

        if input == String::from("1\n") {
            cadastra(&mut empresa)
        } else if input.to_string() == String::from("2\n") {
            lista_por_departamento(&mut empresa)
        } else if input.to_string() == String::from("3\n") {
            lista_todos(&mut empresa)
        } else {
            println!("digite um número válido\n");
        }
    }
}

fn cadastra(empresa: &mut HashMap::<String, Vec<String>>) {
    println!("Escreva nesse formato: Add <nome funcionario> to <nome departamento>");
    let mut input = String::new();

    io::stdin().read_line(&mut input)
        .expect("Falha ao ler entrada");

    let input: Vec<&str> = input.split(' ').collect();

    let novo_funcionario = input[1];
    let novo_departamento = input[3];

    /*Usamos entry para obter uma referência mutável para o vetor associado à chave
    de novo_departamento. Se a chave não existir, or_insert cria um novo vetor vazio
    e retorna uma referência mutável para ele. */
    let mut funcionarios = empresa
        .entry(novo_departamento.to_string())
        .or_insert(Vec::new());

    funcionarios.push(novo_funcionario.to_string());
}

fn lista_por_departamento(empresa: &mut HashMap::<String, Vec<String>>) {
    println!("Escreva o nome do departamento");
    let mut departamento = String::new();

    io::stdin().read_line(&mut departamento)
        .expect("Falha ao ler entrada");

    let mut funcionarios = empresa.get_mut(&departamento);

    print!("Departamento: {}", departamento);
    if funcionarios.is_some() {
        let mut funcionarios = funcionarios.unwrap();
        funcionarios.sort();
        for funcionario in funcionarios {
            println!("Nome do funcionário: {}", funcionario);
        }
    }
    println!("\n");
}

fn lista_todos(empresa: &mut HashMap::<String, Vec<String>>) {
    for (departamento, funcionarios) in empresa {
        print!("Departamento: {}", departamento);
        funcionarios.sort();
        for funcionario in funcionarios {
            println!("Nome do funcionário: {}", funcionario);
        }
    }
    println!("\n");
}