use openai_http::{api::create_completion::CreateCompletionRequest, client::Client};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api_token = std::env::var("OPENAI_API_KEY")?;
    let openai = Client::new(&api_token);

    let prompt = "Some popular programming languages are ";

    let req = CreateCompletionRequest::new(prompt).model("text-davinci-003");
    let resp = openai.call(req).await?;

    let text = resp.choices[0].text.clone();

    println!("{prompt}{text}");

    Ok(())
}
