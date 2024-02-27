/* Traits são similares a um recurso frequentemente
chamado de 'interface' em outras linguagens,
com algumas diferenças. */

pub trait Resumir {
    // fn resumo(&self) -> String;

    /* Definindo uma implementação padrão */
    fn resumo(&self) -> String {
        String::from("(Leia mais...)")
    }
}

pub struct BlogPost {
    pub conteudo: String,
}

impl Resumir for BlogPost {}

pub struct ArtigoDeNoticia {
    pub titulo: String,
    pub local: String,
    pub autor: String,
    pub conteudo: String,
}

impl Resumir for ArtigoDeNoticia {
    fn resumo(&self) -> String {
        return format!("{}, by {} ({})", self.titulo, self.autor, self.local);
    }
}

pub struct Tweet {
    pub nomeusuario: String,
    pub conteudo: String,
    pub resposta: bool,
    pub retweet: bool
}

impl Resumir for Tweet {
    fn resumo(&self) -> String {
        return format!("{}: {}", self.nomeusuario, self.conteudo);
    }
}

fn main() {
    let tweet = Tweet {
        nomeusuario: String::from("horse_ebooks"),
        conteudo: String::from("claro, como voces provavelmente ja sabem, pessoas"),
        resposta: false,
        retweet: false,
    };

    println!("1 novo tweet: {}", tweet.resumo());

    let artigo = ArtigoDeNoticia {
        titulo: String::from("Aprimoramento e conservação da natureza humana no ponto de vista teológico"),
        local: String::from("São Paulo"),
        autor: String::from("Arthur dos Santos Almeida"),
        conteudo: String::from("De um lado, um grupo que se denomina transhumanista, acreditam no aprimoramento técnico da natureza humana, do outro lado, os bioconsevadores, acreditam que devemos conservar a vida assim como nos foi dada. Mas e Deus, de que lado está?")
    };

    println!("{}", artigo.resumo());

    let post = BlogPost {
        conteudo: String::from("sei la"),
    };

    println!("{}", post.resumo()) // vai imprimir (Leia mais...)
}