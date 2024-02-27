/* A funcionalidade final do blog será assim:

Uma postagem no blog começa como um rascunho vazio.
Quando o rascunho é concluído, é necessária uma revisão da postagem.
Quando a postagem é aprovada, ela é aprovada.
Apenas postagens de blog publicadas retornam conteúdo para impressão,
portanto, postagens não aprovadas não podem ser publicadas por acidente. */

use design_pattern_state::Postagem;

fn main() {
    let mut post = Postagem::new();

    println!("adicionando texto");
    post.add_texto("Eu comi uma salada no almoço de hoje");
    println!("post.conteudo() = `{}`", post.conteudo()); // TEM QUE SER VAZIO

    println!("solicitando revisao");
    post.solicitar_revisao();
    println!("post.conteudo() = `{}`", post.conteudo()); // TEM QUE SER VAZIO

    println!("aprovando");
    post.aprovar();
    println!("post.conteudo() = `{}`", post.conteudo()); // TEM QUE SER O TEXTO ADICIONADO NA LINHA 15
}

/* Se fôssemos criar uma implementação alternativa que não usasse o state patter
poderíamos usar instruções match nos métodos do Postagem ou mesmo no código main
que verifica o estado da postagem e muda o comportamento nesses locais.
Isso significaria que teríamos que procurar em vários lugares para entender todas
as implicações de uma postagem estar no estado publicado! Isso só aumentaria o
número de estados que adicionamos: cada uma dessas instruções match precisaria
de outra ramificação.

Com o padrão de de estados, os métodos de Postagem e os locais que usam Postagem
não precisam da instrução match e para adicionar um novo estado, apenas precisamos
adicionar uma nova estrutura e implementar os métodos trait nessa estrutura.

A implementação usando o padrão de estados é fácil de estender para adicionar mais
funcionalidades.*/