async fn many_requests(urls: &[String]) -> Vec<Result<String, surf::Exception>> {
    let client = surf::Client::new();

    let mut handles = vec![];
    for url in urls {
        let req = client.get(&url).recv_string();
        handles.push(async_std::task::spawn(req));
    }

    let mut results = vec![];
    for handle in handles {
        results.push(handle.await);
    }

    results
}

fn main() {
    let requests = &[
        "https://www.google.com/".to_string(),
        "https://crates.io/".to_string(),
    ];
    let results = async_std::task::block_on(many_requests(requests));
    for result in results {
        match result {
            Ok(response) => println!("*** {}\n", response),
            Err(err) => eprintln!("error: {}\n", err)
        }
    }
}
