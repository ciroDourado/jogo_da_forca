#![allow(dead_code)]
pub mod cliente_https;
pub mod models;

use cliente_https::cliente::Cliente;

use models::deserializacao::deserializar;
use models::palavra_aleatoria::PalavraAleatoria;
use models::significados::Significados;


#[tokio::main]
async fn main() -> () {
    let dominio = "api.dicionario-aberto.net";
    let cliente = Cliente::conectar_no(dominio);

    let json_parte1 = cliente.get("/random");
    let aleatoria   = deserializar::<PalavraAleatoria>(&json_parte1);
    let buscada     = formatar( vec!["word", &aleatoria.word] );

    let json_parte2  = cliente.get(&buscada);
    let significados = deserializar::<Significados>(&json_parte2);

    println!("{}", significados[0].word);
    println!("{}", significados[0].xml);
} // main


fn formatar(partes: Vec<&str>) -> String {
    let mut novas: Vec<String> = Vec::new();

    for parte in partes {
        let nova = urlencoding::encode(parte);
        novas.push(nova.to_string());
    }
    format!("/{}/", novas.join("/"))
}
