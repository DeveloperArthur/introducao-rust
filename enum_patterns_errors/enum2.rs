//declarando enum
#[derive(Debug)]
enum VersaoIp {
    V4,
    V6   
}

#[derive(Debug)]
struct EnderecoIp {
    versao: VersaoIp,
    endereco: String,
}

fn main() {
    //criando instancia de enum
    let versao4 = VersaoIp::V4;

    let local = EnderecoIp {
        versao: versao4,
        endereco: String::from("1.1.1.1"),
    };

    let loopback = EnderecoIp {
        versao: VersaoIp::V6,
        endereco: String::from("::1")
    };

    rotear(local);
    rotear(loopback);
}

fn rotear(ip: EnderecoIp) {
    println!("roteando para IP{:?} {:?}", ip.versao, ip.endereco);
}