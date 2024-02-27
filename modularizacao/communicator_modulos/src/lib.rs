// network client {
//     fn connect() {
//     }
// }
//
// network network {
//     fn connect() {
//     }
//
//     network::server::connect
//     network server {
//         fn connect() {
//         }
//     }
//        Estas podem ter funcionalidades completamente
//        diferentes, e os nomes das funções não estão em
//        conflito entre si porque estão em módulos
//        diferentes.
// }
/* Se nós quisermos chamar essa função do código
fora do módulo network, nós precisaremos especificar
o módulo e usar a sintaxe do namespace ::, assim:
network::connect()*/

//separando todos esses modulos em arquivos:

//pub para tornar o modulo publico
pub mod client;
pub mod network;
/* ˆ estamos dizendo ao Rust para que procure, em
outro local, o código definido no escopo do módulo
client. Em outras palavras, a linha network client;
significa:
mod network {
    // conteúdo de network.rs
}
*/

#[cfg(test)]
mod tests {
    use super::client; // após adicionar este codigo, a linha 54 funciona

    #[test]
    fn it_works() {
        //para executar: cargo test

        /* super::client::connect(); 

        super é utilizado para voltar um modulo na hierarquia.
        o trecho client::connect() nao funciona porque estamos 
        dentro do módulo tests, o módulo tests precisa do módulo 
        client no seu escopo */

        client::connect(); // <- agora sim funciona
        /* pois utilizamos use. */
    }
}