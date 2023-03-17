use std::io::Write;

use openai_sdk::{
    api::create_chat_completion::CreateChatCompletionRequest, client::Client, types::Message,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api_token = std::env::var("OPENAI_API_KEY")?;
    let client = Client::new(&api_token);

    let mut context = vec![
        Message::new(
            "system",
            "The following is a conversation with an AI assistant.",
        ),
        Message::new(
            "system",
            "The assistant is helpful, creative, clever, and very friendly.",
        ),
        Message::new("user", "Hello, how are you?"),
        Message::new(
            "assistant",
            "My name is Agent, I am an AI. How can I help you today?",
        ),
    ];

    loop {
        print!("User: ");
        let _ = std::io::stdout().flush();

        let mut user_content = String::new();
        if let Err(e) = std::io::stdin().read_line(&mut user_content) {
            eprintln!("Error: {}", e);
            break;
        }

        context.push(Message::new("user", &user_content));

        let req = CreateChatCompletionRequest::new(&context);
        let resp = client.call(req).await.unwrap();

        let message = resp.choices[0].message.clone();

        println!("Agent: {}", message.content);

        if user_content.to_lowercase().starts_with("bye") {
            break;
        }

        context.push(message);
    }

    Ok(())
}
