use openai_http_client::{api::create_completion::CreateCompletionRequest, Client};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api_token = std::env::var("OPENAI_API_KEY")?;
    let openai = Client::new(&api_token);

    let prompt = "The 10 most popular programming languages are ";

    let req = CreateCompletionRequest::new(prompt)
        .model("text-davinci-003")
        .temperature(1.8);
    let resp = openai.call(req).await?;

    let text = &resp.choices[0].text;

    println!("{prompt}{text}");

    Ok(())
}
