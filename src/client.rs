use std::sync::{Arc, Mutex};

impl From<reqwest::Error> for crate::models::ResponseError {
    fn from(value: reqwest::Error) -> Self {
        return crate::models::ResponseError(value.to_string());
    }
}

pub fn trigger_send(
    request: &crate::models::Request,
    response: Arc<Mutex<Option<Result<crate::models::Response, crate::models::ResponseError>>>>,
    ctx: &egui::Context,
) {
    {
        let mut response = response.lock().unwrap();
        *response = None;
    }
    match get_request(request) {
        Ok((client, request)) => {
            std::thread::spawn({
                let ctx = ctx.clone();
                move || {
                    let result = send(client, request);
                    {
                        let mut response = response.lock().unwrap();
                        *response = Some(result);
                    }
                    ctx.request_repaint();
                }
            });
        }
        Err(error) => {
            let mut response = response.lock().unwrap();
            *response = Some(Err(error));
        }
    }
}

fn get_request(
    request: &crate::models::Request,
) -> Result<(reqwest::blocking::Client, reqwest::blocking::Request), crate::models::ResponseError> {
    let client = reqwest::blocking::Client::builder().build()?;
    let rq = client
        .request(method(request.method), request.url.clone())
        .build()?;
    return Ok((client, rq));
}

fn get_value(value: &reqwest::header::HeaderValue) -> crate::models::HeaderValue{
    match value.to_str() {
        Ok(str) => crate::models::HeaderValue::String(str.to_string()),
        Err(_) => crate::models::HeaderValue::Bytes(value.as_bytes().to_vec()),
    }
}

fn send(
    client: reqwest::blocking::Client,
    request: reqwest::blocking::Request,
) -> Result<crate::models::Response, crate::models::ResponseError> {
    let start = chrono::offset::Utc::now();
    let response = client.execute(request)?;

    let headers = response.headers().into_iter().map(|(h,v)| crate::models::Header{name: h.to_string(), value: get_value(v)}).collect();
    let status = response.status().as_u16();
    let content_length = response.content_length();
    let text = response.text()?;
    let end = chrono::offset::Utc::now();
    return Ok(crate::models::Response {
        status,
        content_length,
        start,
        end,
        text,
        headers
    });
}

fn method(m: crate::models::Method) -> reqwest::Method {
    match m {
        crate::models::Method::Get => reqwest::Method::GET,
        crate::models::Method::Post => reqwest::Method::POST,
        crate::models::Method::Put => reqwest::Method::PUT,
        crate::models::Method::Patch => reqwest::Method::PATCH,
        crate::models::Method::Delete => reqwest::Method::DELETE,
        crate::models::Method::Head => reqwest::Method::HEAD,
        crate::models::Method::Options => reqwest::Method::OPTIONS,
        crate::models::Method::Trace => reqwest::Method::TRACE,
        crate::models::Method::Connect => reqwest::Method::CONNECT,
    }
}
