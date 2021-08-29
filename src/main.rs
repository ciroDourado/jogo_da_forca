pub mod cliente_https;
use cliente_https::cliente::Cliente;


#[tokio::main]
async fn main() -> () {
    let no_dominio = "api.dicionario-aberto.net";
    let cliente    = Cliente::novo(no_dominio);

    println!("{}", cliente.get("/random"));
    println!("tudo certo até aqui :3");
} // main
