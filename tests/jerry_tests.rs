#[test]
fn request_200() {
    match reqwest::blocking::get("http://127.0.0.1:7878/") {
        Ok(response) => {
            assert_eq!(200, response.status());
        }
        Err(err) => {
            panic!("Something went wrong with error - {}", err)
        }
    }
}

#[test]
fn request_404() {
    match reqwest::blocking::get("http://127.0.0.1:7878/incorrect_url") {
        Ok(response) => {
            assert_eq!(404, response.status());
        }
        Err(err) => {
            panic!("Something went wrong with error - {}", err)
        }
    }
}
