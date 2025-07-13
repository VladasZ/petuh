use anyhow::Result;
use reqwest::Client;
use serde::Serialize;

#[derive(Serialize)]
struct ChatRequest {
    model: String,
    input: String,
}

#[tokio::test]
async fn test_gpt() -> Result<()> {
    dotenv::dotenv()?;

    // dbg!(query_zul("Как пожарить котлеты?").await?);
    // dbg!(query_petuh("Как пожарить котлеты?").await?);
    dbg!(query_denis("Как пожарить котлеты?").await?);

    Ok(())
}

pub(crate) async fn query_petuh(input: &str) -> Result<String> {
    query_gpt(&format!("{} {input}", std::env::var("PETUH_QUERY")?)).await
}

pub(crate) async fn query_zul(input: &str) -> Result<String> {
    if input.contains("panic") {
        panic!("Zul panics! {input}");
    }
    query_gpt(&format!("{} {input}", std::env::var("ZUL_QUERY")?)).await
}

pub(crate) async fn query_denis(input: &str) -> Result<String> {
    query_gpt(&format!("{} {input}", std::env::var("DENIS_QUERY")?)).await
}

async fn query_gpt(input: &str) -> Result<String> {
    dotenv::dotenv()?;

    let api_key = std::env::var("CHAT_GPT_API_KEY")?;

    let client = Client::new();
    let url = "https://api.openai.com/v1/responses";

    let request_body = ChatRequest {
        model: "gpt-4o-mini".to_string(),
        input: input.to_string(),
    };

    let response = client
        .post(url)
        .bearer_auth(api_key)
        .header("Content-Type", "application/json")
        .json(&request_body)
        .send()
        .await?;

    dbg!(&response);

    let chat_response: serde_json::Value = response.json().await?;

    let response = chat_response["output"].as_array().unwrap().first().unwrap()["content"]
        .as_array()
        .unwrap()
        .first()
        .unwrap()["text"]
        .as_str()
        .unwrap();

    Ok(response.to_string())
}
