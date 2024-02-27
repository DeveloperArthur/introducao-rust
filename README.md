#LearningRust 🦀 Leitura finalizada, gostei bastante dessa linguagem, deu pra perceber como Rust é 
uma linguagem bastante segura, e não falo do ponto de vista de segurança da informação, mas da 
segurança do programador mesmo, de produzir menos bugs e ter mais confiança.

O Rust tem essa particularidade, o Rust aponta bugs potenciais antecipadamente para eliminar 
possíveis bugs de gerenciamento de memória, representação de dados e concorrência EM TEMPO DE COMPILAÇÃO!
[A imutabilidade de variáveis no Rust contribui para escrever código seguro e de fácil concorrência](), além disso:

Em Rust se você escrever um código que acessa o index 10 em um array de 5 elementos, o código nem 
compila, [o compilador do Rust não permite, porque isso pode gerar um buffer overread, permitindo 
acesso uma memória inválida](), tem também o conceito de [ownership para evitar erros de double free 
(liberação dupla)](), o conceito de [borrowing para evitar que referências apontem para variáveis 
inválidas](), o [Rust não permite que seja usado borrowing nas threads para evitar problemas de 
concorrência](), tem os [lifetimes para garantir que nunca vamos retornar um endereço de memória 
inválido](), e o Rust não permite ter [referencia mutável e imutável sendo usada ao mesmo tempo, para 
evitar problemas de mutabilidade e garantir a segurança do acesso aos dados]().