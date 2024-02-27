//declarando enum
#[derive(Debug)]
enum VersaoIp {
    V4,
    V6   
}

fn main() {
    //criando instancia de enum
    let quatro = VersaoIp::V4;
    let seis = VersaoIp::V6;

    rotear(quatro);
    rotear(seis);
    rotear(VersaoIp::V4);
    rotear(VersaoIp::V6);
}

fn rotear(versao_ip: VersaoIp) {
    println!("roteando para ip {:?}", versao_ip)
}