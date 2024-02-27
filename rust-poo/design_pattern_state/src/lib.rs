pub struct Postagem {
    estado: Option<Box<dyn Estado>>,
    conteudo: String
}

impl Postagem {
    pub fn new() -> Postagem {
        Postagem {
            estado: Some(Box::new(Rascunho {})),
            conteudo: String::new(),
        }
    }

    pub fn add_texto(&mut self, text: &str) {
        self.conteudo.push_str(text);
    }

    pub fn conteudo(&self) -> &str {
        let estado = self.estado.as_ref().unwrap();
        estado.conteudo(&self)
    }

    pub fn solicitar_revisao(&mut self) {
        if let Some(s) = self.estado.take() {
            self.estado = Some(s.solicitar_revisao())
        }
    }

    pub fn aprovar(&mut self) {
        if let Some(s) = self.estado.take() {
            self.estado = Some(s.aprovar())
        }
    }

    /* take() serve para tirar o valor de Some do campo estado
    e deixar um None no lugar, porque Rust não nos permite ter
    campos não preenchidos nas estruturas. Isso nos permite mover o
    valor do estado para fora do Postagem em vez de pedir emprestado.
    Em seguida, definiremos o valor do estado da postagem como
    resultado da operação. */
}
//-------------------------------------------------------------------
trait Estado {
    fn solicitar_revisao(self: Box<Self>) -> Box<dyn Estado>;
    fn aprovar(self: Box<Self>) -> Box<dyn Estado>;
    fn conteudo<'a>(&self, post: &'a Postagem) -> &'a str {
        ""
    }
}
//-------------------------------------------------------------------
struct Rascunho {}

impl Estado for Rascunho {
    fn solicitar_revisao(self: Box<Self>) -> Box<dyn Estado> {
        Box::new(RevisaoPendente {})
    }

    fn aprovar(self: Box<Self>) -> Box<dyn Estado> {
        self
    }
}
//-------------------------------------------------------------------
struct RevisaoPendente {}

impl Estado for RevisaoPendente {
    fn solicitar_revisao(self: Box<Self>) -> Box<dyn Estado> {
        self
    }

    fn aprovar(self: Box<Self>) -> Box<dyn Estado> {
        Box::new(Publicado {})
    }
}
//-------------------------------------------------------------------
struct Publicado {}

impl Estado for Publicado {
    fn solicitar_revisao(self: Box<Self>) -> Box<dyn Estado> {
        self
    }

    fn aprovar(self: Box<Self>) -> Box<dyn Estado> {
        self
    }

    fn conteudo<'a>(&self, post: &'a Postagem) -> &'a str {
        &post.conteudo
    }
}