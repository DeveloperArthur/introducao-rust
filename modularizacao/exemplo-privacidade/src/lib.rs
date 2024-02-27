mod outermost {
    pub fn middle_function() {}

    fn middle_secret_function() {}

    mod inside {
        pub fn inner_function() {
            // use crate::outermost;
            //pra p codigo abaixo dar certo teria que descomentar a linha de cima
            outermost::middle_secret_function();
        }

        fn secret_function() {}
    }
}

/*A função try_me está no módulo raiz do nosso projeto.
O módulo chamado outermost é privado, mas a segunda regra
de privacidade afirma que a função try_me pode acessar o
módulo outermost porque outermost está no módulo atual
(raiz), bem como try_me.*/
fn try_me() {
    //funciona
    outermost::middle_function();

    //ERRO (teria que transformar a funcao em publica)
    outermost::middle_secret_function();

    //ERRO (teria que transformar o submodulo em publico)
    outermost::inside::inner_function();

    //ERRO (teria que transformar o submodulo em publico e transformar a funcao em publica)
    outermost::inside::secret_function();
}

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
