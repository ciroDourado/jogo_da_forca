#![allow(dead_code)]
pub mod cliente_https;
pub mod models;

use models::deserializacao::deserializar;
use cliente_https::cliente::Cliente;
use models::palavra_aleatoria::PalavraAleatoria;


#[tokio::main]
async fn main() -> () {
    let dominio = "api.dicionario-aberto.net";
    let cliente = Cliente::conectar_no(dominio);
    let palavra = cliente.get("/random");

    let aleatoria: PalavraAleatoria = deserializar(&palavra);
    println!("{}", aleatoria.word);
} // main
