//Tirando smart pointer de escopo com trait Drop

struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        /* O corpo da função drop é onde você colocaria qualquer que
        fosse a lógica que você gostaria que rodasse quando uma instância
        do seu tipo for sair de escopo. */

        println!("Destruindo CustomSmartPointer com dados `{}`!", self.data);
    }
}

fn main() {
    let c = CustomSmartPointer { data: String::from("alocado primeiro") };
    let d = CustomSmartPointer { data: String::from("alocado por ultimo") };
    println!("CustomSmartPointers criados.");

    //Destruindo um Valor Cedo com std::mem::drop
    let e = CustomSmartPointer { data: String::from("algum dado") };
    println!("CustomSmartPointer criado.");

    /*  e.drop(); ERROR: explicit destructor calls not allowed

        O Rust não nos deixa chamar drop 👆 explicitamente porque o drop
        ainda seria chamado no valor ao final da main. Isso seria um erro
        de liberação dupla (double free) porque o Rust estaria tentando
        limpar o mesmo valor duas vezes. */

    drop(e); //std::mem::drop

    println!("CustomSmartPointer destruído antes do final da main.");
}