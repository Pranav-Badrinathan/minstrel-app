
#[tokio::main]
async fn main() {
    println!("Howdy.");

	let client = reqwest::Client::new();

	loop {
		let _res = client.post("http://127.0.0.1:4242")
			.body("PACKAGE!").send().await.expect("Something went wrong here...");
	}
}
