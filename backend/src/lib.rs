#[ic_cdk::query]
fn greet(name: String) -> String {
    format!("Hello, {}!", name)
}
// test build trigger
// build trigger 2
// build trigger 3
// build trigger 4 - full e2e
// build trigger 5 - full e2e
