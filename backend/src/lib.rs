#[ic_cdk::query]
fn greet(name: String) -> String {
    format!("Hello, {}!", name)
}
// test build trigger
// build trigger 2
// build trigger 3
