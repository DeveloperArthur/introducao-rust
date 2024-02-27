pub fn connect() {
}

pub mod server;

/*Regras dos Módulos e Seus Arquivos (https://rust-br.github.io/rust-book-pt-br/ch07-01-mod-and-the-filesystem.html#regras-dos-m%C3%B3dulos-e-seus-arquivos)
Vamos resumir as regras dos módulos em relação aos arquivos:

Se um módulo chamado foo não possui submódulos, você deve colocar as declarações    para foo em um arquivo chamado foo.rs.
Se um módulo chamado foo possui submódulos, você deve colocar as declarações    para foo em um arquivo chamado foo/mod.rs.
Essas regras se aplicam de forma recursiva, então, se um módulo chamado foo tiver um submódulo chamado bar e bar não possui submódulos, você deve ter os seguintes arquivos no seu diretório src:


├── foo
│   ├── bar.rs (contém as declarações em `foo::bar`)
│   └── mod.rs (contém as declarações em `foo`, incluindo `mod bar`)
Os módulos devem ser declarados no arquivo do módulo pai usando a palavra-chave mod.*/