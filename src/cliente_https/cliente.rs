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
    pub fn novo(dominio: &'a str) -> Self {
        Cliente { dominio }
    } // novo


    pub fn get(&self, caminho: &'a str) -> String {
        self.montar_url(caminho)
            .and_then(montar_requisicao)
            .map(fazer_requisicao);
        "a".to_string()
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


use futures::executor::block_on;
fn fazer_requisicao(req: Request<Body>)
    -> String
{
    let future = async move {
        cliente_https().request(req).await
    };
    match block_on(future) {
        Ok(res) => String::new() , // continuar daqui!!
        Err(_)  => String::new() }
} // fazer_requisicao



use hyper::client::HttpConnector;
use hyper_tls::HttpsConnector;
use hyper::Client;
fn cliente_https()
    -> Client<HttpsConnector<HttpConnector>>
{
    let https   = HttpsConnector::new();
    let cliente = Client::builder()
        .build::<_, hyper::Body>(https);
    return cliente;
} // cliente_https
