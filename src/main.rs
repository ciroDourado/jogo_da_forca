pub mod cliente_https;
use cliente_https::cliente::Cliente;


#[tokio::main]
async fn main() -> () {
    let dominio = "api.dicionario-aberto.net";
    let cliente = Cliente::conectar_no(dominio);

    println!("{}", cliente.get("/random"));
    println!("tudo certo até aqui :3");
} // main
