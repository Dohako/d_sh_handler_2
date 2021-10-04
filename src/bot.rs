use std::env;

use futures::StreamExt;
use telegram_bot::*;
use dotenv::dotenv;
use std::process::Command;

#[tokio::main]
pub async fn main() -> Result<(), Error> {
    dotenv().expect(".env file not found");
    let token = env::var("TELEGRAM_BOT_TOKEN").expect("TELEGRAM_BOT_TOKEN not set");
    let api = Api::new(token);

    // Fetch new updates via long poll method
    let mut stream = api.stream();
    while let Some(update) = stream.next().await {
        // If the received update contains a new message...
        let update = update?;
        if let UpdateKind::Message(message) = update.kind {
            if let MessageKind::Text { ref data, .. } = message.kind {
                // Print received text message to stdout.
                println!("<{}>: {}", &message.from.first_name, data);

                
                if data.contains("/v"){
                    let value = &data[3..];
                    let int_val: i32 = value.parse().unwrap();
                    api.send(message.text_reply(format!(
                        "Ok, set volume to {}",int_val
                    )))
                    .await?;
                    
                    let sound_command = format!("amixer -D pulse sset Master {}%",value);
                    let output = if cfg!(target_os = "windows") {
                        Command::new("cmd")
                                .args(["/C", "echo wrong system"])
                                .output()
                                .expect("failed to execute process")
                    } else {
                        Command::new("sh")
                                .arg("-c")
                                .arg(sound_command)
                                .output()
                                .expect("failed to execute process")
                    };
                    
                    let hello = output.stdout;
                }
                else {
                    // Answer message with "Hi".
                    api.send(message.text_reply(format!(
                        "Hi, {}! You just wrote '{}'",
                        &message.from.first_name, data
                    )))
                    .await?;
                }
            }
        }
    }
    Ok(())
}