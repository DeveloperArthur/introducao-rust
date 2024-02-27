struct User {
    username: String,
    email: String, 
    sign_in_count: u64,
    active: bool,
}

fn main1() {
    let user1 = User {
        email: String::from("alguem@exemplo.com"),
        username: String::from("algumnome123"),
        active: true,
        sign_in_count: 1,
    };

    println!("{}", user1.username);

    let mut user2 = User {
        email: String::from("alguem@exemplo.com"),
        username: String::from("algumnome123"),
        active: true,
        sign_in_count: 1,
    };

    user2.email = String::from("outroemail@exemplo.com");
    println!("{}", user2.email);

    // se por acaso as variaveis tiverem o mesmo 
    //nome dos campos, podemos usar "field init shorthand":

    let email = String::from("alguem@exemplo.com");
    let username = String::from("algumnome123");

    let user3 = User {
        email, // <- 
        username, // <-
        active: true,
        sign_in_count: 1,
    };

    //struct update syntax:
    /*essa abreviação possibilita criar uma nova instância 
    a partir de uma outra instância, usando a maioria dos valores 
    da outra instância mas mudando alguns. veja abaixo o exemplo 
    da criação de uma nova instância do user3 em user4 através 
    da definição dos valores de e-mail e username mas 
    usando os mesmos valores para o resto dos campos
    do exemplo user3 que criamos: */

    let user4 = User {
        email: String::from("alguem@exemplo.com"),
        ..user3
    };

    /*dessa forma definimos apenas o valor de email
    mas usamos todos os outros valores da instancia 
    user3 */

    //struct-tupla:

    struct Cor(i32, i32, i32);
    struct Ponto(i32, i32, i32);

    let preto = Cor(0, 0, 0);
    let origem = Ponto(0, 0, 0);

    //No geral as struct-tuplas comportam-se como instâncias de tuplas

    /*Se você deseja representar uma estrutura de dados que é 
    essencialmente uma tupla de valores, mas com significado 
    semântico associado a cada posição, uma struct de tupla 
    pode ser mais apropriada. */

    /*Em alguns casos, uma struct de tupla pode ser mais concisa 
    do que uma struct tradicional, especialmente se você estiver 
    trabalhando com tipos simples e não precisa de métodos 
    associados ou implementações complexas. */

    //outro exemplo de struct-tupla:
    struct Ponto3D(i32, i32, i32);

    /*Se você deseja representar uma estrutura de dados que é 
    essencialmente uma tupla de valores, mas com significado 
    semântico associado a cada posição, uma struct de tupla 
    pode ser mais apropriada.

    No exemplo acima (struct Ponto3D(i32, i32, i32)), estamos
    criando uma representação de cor que é essencialmente uma 
    tupla de três valores i32, mas usando uma struct de tupla 
    para dar um nome significativo à combinação de valores. */
}

//ainda sobre tuplas:
//recebe o endereço de memória de uma tupla
fn print_coordinates(&(x, y): &(i32, i32)) {
    println!("Current location: ({}, {})", x, y);
}

fn main() {
    let point = (3, 5);
    print_coordinates(&point);
}