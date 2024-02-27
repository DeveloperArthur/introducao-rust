#LearningRust 🦀 Leitura finalizada, gostei bastante dessa linguagem, deu pra perceber como Rust é 
uma linguagem bastante segura, e não falo do ponto de vista de segurança da informação, mas da 
segurança do programador mesmo, de produzir menos bugs e ter mais confiança.

O Rust tem essa particularidade, o Rust aponta bugs potenciais antecipadamente para eliminar 
possíveis bugs de gerenciamento de memória, representação de dados e concorrência EM TEMPO DE COMPILAÇÃO!
[A imutabilidade de variáveis no Rust contribui para escrever código seguro e de fácil concorrência](https://github.com/DeveloperArthur/introducao-rust/blob/main/conceitos_basicos/variaveis.rs), além disso:

Em Rust se você escrever um código que acessa o index 10 em um array de 5 elementos, o código nem 
compila, [o compilador do Rust não permite, porque isso pode gerar um buffer overread, permitindo 
acesso uma memória inválida](https://github.com/DeveloperArthur/introducao-rust/blob/main/conceitos_basicos/tipos_de_dados.rs), tem também o conceito de [ownership para evitar erros de double free 
(liberação dupla)](https://github.com/DeveloperArthur/introducao-rust/blob/main/ownership/ownership.rs), o conceito de [borrowing para evitar que referências apontem para variáveis 
inválidas](https://github.com/DeveloperArthur/introducao-rust/blob/main/ownership/referencias_borrowing.rs), o [Rust não permite que seja usado borrowing nas threads para evitar problemas de 
concorrência](https://github.com/DeveloperArthur/introducao-rust/blob/main/concorrencia/threads2_with_closures.rs), tem os [lifetimes para garantir que nunca vamos retornar um endereço de memória 
inválido](https://github.com/DeveloperArthur/introducao-rust/blob/main/generics_traits_lifetime/lifetimes.rs), e o Rust não permite ter [referencia mutável e imutável sendo usada ao mesmo tempo, para 
evitar problemas de mutabilidade e garantir a segurança do acesso aos dados](https://github.com/DeveloperArthur/introducao-rust/blob/main/linguagem_funcional/closure3.rs).
