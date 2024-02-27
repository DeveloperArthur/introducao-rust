pub struct ColecaoDeMedia {
    lista: Vec<i32>,
    media: f64,
}

/* Os métodos públicos adicionar, remover e media são
as únicas maneiras de modificar uma instância de ColecaoDeMedia */
impl ColecaoDeMedia {
    pub fn adicionar(&mut self, valor: i32) {
        self.lista.push(valor);
        self.atualizar_media();
    }

    pub fn remover(&mut self) -> Option<i32> {
        let resultado = self.lista.pop();
        match resultado {
            Some(valor) => {
                self.atualizar_media();
                Some(valor)
            },
            None => None,
        }
    }

    pub fn media(&self) -> f64 {
        self.media
    }

    fn atualizar_media(&mut self) {
        let total: i32 = self.lista.iter().sum();
        self.media = total as f64 / self.lista.len() as f64;
    }
}