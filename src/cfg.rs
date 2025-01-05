use crate::*;
use color_output::*;
use http_type::ArcMutex;
use std_macro_extensions::*;

#[allow(dead_code)]
fn output(title: &str, msg: &str, color: Color) {
    OutputListBuilder::new()
        .add(
            OutputBuilder::new()
                .text(title)
                .bg_color(ColorType::Use(Color::Cyan))
                .blod(true)
                .build(),
        )
        .add(
            OutputBuilder::new()
                .text(msg)
                .bg_color(ColorType::Use(color))
                .blod(true)
                .endl(true)
                .build(),
        )
        .run();
}

#[test]
fn test_http_post_request() {
    let mut header: HashMap<&str, &str> = HashMap::new();
    header.insert("Accept", "*/*");
    header.insert("Content-Type", "application/json");
    header.insert("Connection", "keep-alive");
    header.insert("Accept-Encoding", "gzip, deflate");
    let mut body: HashMap<&str, &str> = HashMap::new();
    body.insert("code", "hello");
    body.insert("language", "rust");
    body.insert("testin", "");
    let mut _request_builder = RequestBuilder::new()
        .post("http://localhost:80/rust?hello=rust")
        .json(body)
        .headers(header)
        .timeout(6000)
        .redirect()
        .buffer(4096)
        .max_redirect_times(8)
        .http1_1_only()
        .build();
    _request_builder
        .send()
        .and_then(|response| {
            output(
                "ResponseTrait => ",
                &format!("{:?}", response.text()),
                Color::Green,
            );
            Ok(())
        })
        .unwrap_or_else(|e| output("Error => ", &format!("{:?}", e), Color::Red));
}

#[test]
fn test_http_get_request() {
    let mut header: HashMap<&str, &str> = HashMap::new();
    header.insert("header-key", "header-value");
    let mut body: HashMap<&str, &str> = HashMap::new();
    body.insert("body-key", "body-value");
    let mut _request_builder = RequestBuilder::new()
        .get("http://localhost:80")
        .headers(header)
        .timeout(6000)
        .redirect()
        .buffer(4096)
        .max_redirect_times(8)
        .http1_1_only()
        .build();
    _request_builder
        .send()
        .and_then(|response| {
            output(
                "ResponseTrait => ",
                &format!("{:?}", response.text()),
                Color::Green,
            );
            Ok(())
        })
        .unwrap_or_else(|e| output("Error => ", &format!("{:?}", e), Color::Red));
}

#[test]
fn test_https_post_request() {
    let mut header: HashMap<&str, &str> = HashMap::new();
    header.insert("Accept", "*/*");
    header.insert("Content-Type", "application/json");
    header.insert("Connection", "keep-alive");
    header.insert("Accept-Encoding", "gzip, deflate");
    let mut body: HashMap<&str, &str> = HashMap::new();
    body.insert("code", "fn main() {\r\n    println!(\"hello world\");\r\n}");
    body.insert("language", "rust");
    body.insert("testin", "");
    let mut _request_builder = RequestBuilder::new()
        .post("https://code.ltpp.vip/")
        .json(body)
        .headers(header)
        .timeout(4000)
        .redirect()
        .buffer(4096)
        .max_redirect_times(8)
        .http1_1_only()
        .build();
    _request_builder
        .send()
        .and_then(|response| {
            output(
                "ResponseTrait => ",
                &format!("{:?}", response.text()),
                Color::Green,
            );
            Ok(())
        })
        .unwrap_or_else(|e| output("Error => ", &format!("{:?}", e), Color::Red));
}

#[test]
fn test_https_get_request() {
    let mut header: HashMap<&str, &str> = HashMap::new();
    header.insert("header-key", "header-value");
    let mut body: HashMap<&str, &str> = HashMap::new();
    body.insert("body-key", "body-value");
    let mut _request_builder = RequestBuilder::new()
        .get("https://code.ltpp.vip/")
        .headers(header)
        .timeout(4000)
        .redirect()
        .buffer(4096)
        .max_redirect_times(8)
        .http1_1_only()
        .build();
    _request_builder
        .send()
        .and_then(|response| {
            output(
                "ResponseTrait => ",
                &format!("{:?}", response.text()),
                Color::Green,
            );
            Ok(())
        })
        .unwrap_or_else(|e| output("Error => ", &format!("{:?}", e), Color::Red));
}

#[test]
fn test_http_post_text_request() {
    let mut header: HashMap<&str, &str> = HashMap::new();
    header.insert("Accept", "*/*");
    header.insert("Content-Type", "application/json");
    let mut _request_builder = RequestBuilder::new()
        .post("http://localhost:80")
        .text("hello")
        .headers(header)
        .timeout(6000)
        .redirect()
        .buffer(4096)
        .max_redirect_times(8)
        .http1_1_only()
        .build();
    _request_builder
        .send()
        .and_then(|response| {
            output(
                "ResponseTrait => ",
                &format!("{:?}", response.text()),
                Color::Green,
            );
            Ok(())
        })
        .unwrap_or_else(|e| output("Error => ", &format!("{:?}", e), Color::Red));
}

#[test]
fn test_http_post_binary_request() {
    let mut header: HashMap<&str, &str> = HashMap::new();
    header.insert("Accept", "*/*");
    header.insert("Content-Type", "application/json");
    let mut _request_builder = RequestBuilder::new()
        .post("http://localhost:80")
        .body("hello".as_bytes())
        .headers(header)
        .timeout(6000)
        .redirect()
        .buffer(4096)
        .max_redirect_times(8)
        .http1_1_only()
        .build();
    _request_builder
        .send()
        .and_then(|response| {
            output(
                "ResponseTrait => ",
                &format!("{:?}", response.text()),
                Color::Green,
            );
            Ok(())
        })
        .unwrap_or_else(|e| output("Error => ", &format!("{:?}", e), Color::Red));
}

#[test]
fn test_auto_gzip_get() {
    let mut _request_builder = RequestBuilder::new()
        .get("https://proxy.ltpp.vip/visit/add?origin=https://docs.ltpp.vip/")
        .timeout(4000)
        .redirect()
        .max_redirect_times(8)
        .decode()
        .buffer(4096)
        .http1_1_only()
        .build();
    _request_builder
        .send()
        .and_then(|response| {
            output(
                "ResponseTrait => ",
                &format!("{:?}", response.text()),
                Color::Green,
            );
            Ok(())
        })
        .unwrap_or_else(|e| output("Error => ", &format!("{:?}", e), Color::Red));
}

#[test]
fn test_gzip_get() {
    let mut _request_builder = RequestBuilder::new()
        .get("https://proxy.ltpp.vip/visit/add?origin=https://docs.ltpp.vip/")
        .timeout(4000)
        .redirect()
        .max_redirect_times(8)
        .buffer(4096)
        .http1_1_only()
        .build();
    _request_builder
        .send()
        .and_then(|response| {
            output(
                "ResponseTrait => ",
                &format!("{:?}", response.decode(4096).text()),
                Color::Green,
            );
            Ok(())
        })
        .unwrap_or_else(|e| output("Error => ", &format!("{:?}", e), Color::Red));
}

#[test]
fn test_unredirect_get() {
    let mut _request_builder = RequestBuilder::new()
        .post("https://proxy.ltpp.vip/visit/add?origin=https://docs.ltpp.vip/")
        .timeout(4000)
        .max_redirect_times(8)
        .buffer(4096)
        .unredirect()
        .http1_1_only()
        .build();
    _request_builder
        .send()
        .and_then(|response| {
            output(
                "ResponseTrait => ",
                &format!("{:?}", response.decode(4096).text()),
                Color::Green,
            );
            Ok(())
        })
        .unwrap_or_else(|e| output("Error => ", &format!("{:?}", e), Color::Red));
}

#[test]
fn test_thread_https_get_request() {
    use std::thread;
    use std::time::Instant;
    let header_key = "header-key";
    let header_value = "header-value";
    let body_key = "body-key";
    let body_value = "body-value";
    let mut body: HashMap<&str, &str> = HashMap::new();
    body.insert(body_key, body_value);
    let num_threads: i32 = 10;
    let mut handles: Vec<thread::JoinHandle<()>> = Vec::new();
    let mut header: HashMap<&str, &str> = HashMap::new();
    header.insert(header_key, header_value);
    let request_builder: ArcMutex<BoxRequestTrait> = Arc::new(Mutex::new(
        RequestBuilder::new()
            .get("https://code.ltpp.vip/")
            .headers(header.clone())
            .timeout(4000)
            .redirect()
            .buffer(4096)
            .max_redirect_times(8)
            .http1_1_only()
            .build(),
    ));
    for _ in 0..num_threads {
        let request_builder = Arc::clone(&request_builder);
        let handle = thread::spawn(move || {
            let mut request_builder = request_builder.lock().unwrap();
            let start_time: Instant = Instant::now();
            match request_builder.send() {
                Ok(response) => {
                    let duration: std::time::Duration = start_time.elapsed();
                    let response_text: HttpResponseText = response.text();
                    output(
                        "Thread finished in: ",
                        &format!("{:?}", duration),
                        Color::Blue,
                    );
                    output(
                        "ResponseTrait => ",
                        &format!("{:?}", response_text),
                        Color::Green,
                    );
                }
                Err(e) => {
                    let duration: std::time::Duration = start_time.elapsed();
                    output(
                        "Thread finished in: ",
                        &format!("{:?}", duration),
                        Color::Blue,
                    );
                    output("Error => ", &format!("{:?}", e), Color::Red);
                }
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
}

#[test]
fn test_thread_http_get_request() {
    use std::thread;
    use std::time::Instant;
    let num_threads: i32 = 1;
    let mut handles: Vec<thread::JoinHandle<()>> = Vec::new();
    let request_builder: ArcMutex<BoxRequestTrait> = Arc::new(Mutex::new(
        RequestBuilder::new()
            .get("http://127.0.0.1:8080/")
            .timeout(10)
            .redirect()
            .buffer(100)
            .max_redirect_times(0)
            .http2_only()
            .build(),
    ));
    for _ in 0..num_threads {
        let request_builder = Arc::clone(&request_builder);
        let handle = thread::spawn(move || {
            let mut request_builder = request_builder.lock().unwrap();
            let start_time: Instant = Instant::now();
            match request_builder.send() {
                Ok(response) => {
                    let duration: std::time::Duration = start_time.elapsed();
                    let response_text: HttpResponseText = response.text();
                    output(
                        "Thread finished in: ",
                        &format!("{:?}", duration),
                        Color::Blue,
                    );
                    output(
                        "ResponseTrait => ",
                        &format!("{:?}", response_text),
                        Color::Green,
                    );
                }
                Err(e) => {
                    let duration: std::time::Duration = start_time.elapsed();
                    output(
                        "Thread finished in: ",
                        &format!("{:?}", duration),
                        Color::Blue,
                    );
                    output("Error => ", &format!("{:?}", e), Color::Red);
                }
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
}
