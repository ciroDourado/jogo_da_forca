use hyper::http::Result as HttpResult;
use hyper::http::Error  as HttpError;
use std::result::Result as StdResult;
use hyper::Request;
use hyper::Method;
use hyper::Body;
use hyper::Uri;


pub struct Cliente<'a> {
    dominio: &'a str
} // struct Cliente


impl<'a> Cliente<'a> {
    pub fn conectar_no(dominio: &'a str) -> Self {
        Cliente { dominio }
    } // conectar_no


    pub fn get(&self, caminho: &'a str) -> String {
        self.montar_url(caminho)
            .and_then(montar_requisicao)
            .map(fazer_requisicao)
            .unwrap()
    } // get


    fn montar_url(&self, caminho: &'a str)
        -> StdResult<Uri, HttpError>
    {
        Uri::builder()
            .scheme("https")
            .authority(self.dominio)
            .path_and_query(caminho)
            .build()
    } // montar_url

} // impl Cliente


fn montar_requisicao(url: Uri)
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


fn fazer_requisicao(req: Request<Body>)
    -> String
{
    let future = async move {
        cliente_https().request(req).await
    };
    block_on(future)
        .map(extrair_o_body)
        .map(converter_em_bytes)
        .map(converter_em_string)
        .unwrap()
} // fazer_requisicao


fn cliente_https()
    -> Client<HttpsConnector<HttpConnector>>
{
    let https   = HttpsConnector::new();
    let cliente = Client::builder()
        .build::<_, hyper::Body>(https);
    return cliente;
} // cliente_https



use hyper::body::to_bytes;
use hyper::body::Bytes;
use hyper::Response;

fn extrair_o_body(response: Response<Body>)
    -> Body
{
    response.into_body()
} // extrair_o_body


fn converter_em_bytes(body: Body)
    -> Bytes
{
    let future = async move { to_bytes(body).await };
    match block_on(future) {
        Ok(res) => res,
        Err(_)  => Bytes::new() }
} // converter_em_bytes


fn converter_em_string(bytes: Bytes)
    -> String
{
    let em_utf8 = bytes.to_vec();
    String::from_utf8(em_utf8)
        .unwrap_or(String::new())
} // converter_em_string
