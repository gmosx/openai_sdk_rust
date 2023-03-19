use openai_http_client::{api::create_edit::CreateEditRequest, Client};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api_token = std::env::var("OPENAI_API_KEY")?;
    let openai = Client::new(&api_token);

    let input = "Here is a tuxt with tipos, hopefully the asssistant can fix them.";
    let instruction = "Fix the spelling mistakes";
    let req = CreateEditRequest::new(input, instruction);
    let resp = openai.call(req).await?;

    let text = &resp.choices[0].text;

    println!("{text}");

    Ok(())
}
