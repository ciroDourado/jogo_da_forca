use futures::executor::block_on;
use hyper::Body;
use hyper::body::to_bytes;
use hyper::body::Bytes;
use hyper::Response;

pub fn extrair_o_body(response: Response<Body>)
    -> Body
{
    response.into_body()
} // extrair_o_body


pub fn converter_em_bytes(body: Body)
    -> Bytes
{
    match block_on( to_bytes(body) ) {
        Ok(res) => res,
        Err(_)  => Bytes::new() }
} // converter_em_bytes


pub fn converter_em_string(bytes: Bytes)
    -> String
{
    let em_utf8 = bytes.to_vec();
    String::from_utf8(em_utf8)
        .unwrap_or(String::new())
} // converter_em_string
