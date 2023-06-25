use async_std::{task, net};
use async_std::io::prelude::*;

async fn cheapo_owing_request(host: String, port: u16, path: String) -> std::io::Result<String> {
    cheapo_request(&host, port, &path).await
}

async fn cheapo_request(host: &str, port: u16, path: &str) -> std::io::Result<String> {
    let mut socket = net::TcpStream::connect((host, port)).await?;

    let request = format!("GET {} HTTP/1.1\r\nHost: {}\r\n\r\n", path, host);
    socket.write_all(request.as_bytes()).await?;
    socket.shutdown(net::Shutdown::Write)?;

    let mut response = String::new();
    socket.read_to_string(&mut response).await?;

    Ok(response)
}

async fn many_requests(requests: Vec<(String, u16, String)>) -> Vec<std::io::Result<String>> {
    let mut handles = vec![];
    for (host, port, path) in requests {
        handles.push(task::spawn(cheapo_owing_request(host, port, path)));
    }

    let mut results = vec![];
    for handle in handles {
        results.push(handle.await);
    }

    return results;
}

fn main() {
    let requests = vec![
        ("google.com".to_string(), 80, "/".to_string()),
        ("crates.io".to_string(), 80, "/".to_string()),
        ("aws.amazon.com".to_string(), 80, "/".to_string()),
    ];
    let results = async_std::task::block_on(many_requests(requests));

    for result in results {
        match result {
            Ok(response) => println!("{}", response),
            Err(err) => eprintln!("error: {}", err),
        }
    }
}
