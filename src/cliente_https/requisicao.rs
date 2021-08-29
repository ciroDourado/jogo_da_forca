use hyper::http::Result as HttpResult;
use hyper::Request;
use hyper::Method;
use hyper::Body;
use hyper::Uri;

pub fn montar_requisicao(url: Uri)
    -> HttpResult<Request<Body>>
{
    Request::builder()
        .method(Method::GET)
        .uri(url)
        .body(Body::from(r#"{"library":"hyper"}"#))
} // montar_requisicao



use hyper::client::HttpConnector;
use hyper_tls::HttpsConnector;
use hyper::Client;

use futures::executor::block_on;
use super::resposta::extrair_o_body;
use super::resposta::converter_em_bytes;
use super::resposta::converter_em_string;


pub fn fazer_requisicao(req: Request<Body>)
    -> String
{
    let future = async move {
        cliente_https().request(req).await
    };
    block_on(future)
        .map(extrair_o_body)
        .map(converter_em_bytes)
        .map(converter_em_string)
        .unwrap_or(String::new())
} // fazer_requisicao


fn cliente_https()
    -> Client<HttpsConnector<HttpConnector>>
{
    let https   = HttpsConnector::new();
    let cliente = Client::builder()
        .build::<_, hyper::Body>(https);
    return cliente;
} // cliente_https
