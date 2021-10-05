use futures::StreamExt;
use telegram_bot::*;
use std::process::Command;

#[tokio::main]
pub async fn start(token: String) -> Result<(), Error> {
    println!("Start bot.rs");
    println!("{}", token);
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
                    change_volume(&api, &message, &data).await.expect("Error");
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

async fn change_volume(api:&Api,message:&Message, data: &String)-> Result<(), Error>{
    let value = &data[3..];
    let int_val: i32 = value.parse().unwrap();
    api.send(message.text_reply(format!(
        "Ok, set volume to {}%",int_val
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
    
    let _hello = output.stdout;
    Ok(())
}