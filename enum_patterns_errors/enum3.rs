//declarando enum com valores associados
#[derive(Debug)]
enum EnderecoIp {
    V4(u8, u8, u8, u8),
    V6(String),
}

impl EnderecoIp {
    fn get_name(&self) -> &str {
        match self {
            EnderecoIp::V6(_) => "V6",
            EnderecoIp::V4(_, _, _, _) => "V4"
        }
    }

    fn get_value(&self) -> String {
        match self {
            EnderecoIp::V6(ip) => ip.clone(),
            EnderecoIp::V4(a, b, c, d) => {
                format!("{}.{}.{}.{}", a, b, c, d)
            }
        }
    }
}

fn main() {
    //criando instancia de enum
    let local = EnderecoIp::V4(1, 1, 1, 1);
    let loopback = EnderecoIp::V6(String::from("::1"));

    rotear(local);
    rotear(loopback);
}

fn rotear(ip: EnderecoIp) {
    println!("roteando para IP{:?} {:?}", ip.get_name(), ip.get_value());
}