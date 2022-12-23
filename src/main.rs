use discord_message_to_image::message::{DiscordMessage, MessageAuthor, MessageAuthorType};
use fantoccini::{Client, ClientBuilder};
use image::{load_from_memory_with_format, DynamicImage, ImageFormat};
use inquire::{Confirm, Text};
use std::{path::Path, str::FromStr, time::Instant};

#[tokio::main]
async fn main() -> Result<(), fantoccini::error::CmdError> {
    let start_time = Instant::now();
    let client = startup().await;
    println!("{:?}", start_time.elapsed());

    loop {
        let channel = cli::create_channel();

        let start_time = Instant::now();
        update_channel(&client, &channel).await;
        let picture = screenshot(&client).await;

        println!("Saving to output.png");
        let output_path = Path::new("output.png");
        picture
            .save_with_format(output_path, ImageFormat::Png)
            .unwrap();

        println!("{:?}", start_time.elapsed());

        let should_quit = Confirm::new("quit? (y/n)").prompt().unwrap_or(true);
        if should_quit {
            break;
        }
    }

    println!("Stopping!");
    client.close().await?;

    Ok(())
}

async fn startup() -> Client {
    println!("Starting up");
    let client = ClientBuilder::native()
        .connect("http://localhost:4444")
        .await
        .expect("failed to connect to WebDriver");

    client
        .goto("http://localhost:3000")
        .await
        .expect("website server offline");

    client
}

async fn update_channel(client: &Client, channel: &[DiscordMessage]) {
    const UPDATE_MESSAGE_SCRIPT: &str = r#"
        const [channel] = arguments;
        updateChannel(channel);
    "#;

    let channel = serde_json::to_value(channel).unwrap();

    client
        .execute(UPDATE_MESSAGE_SCRIPT, vec![channel])
        .await
        .expect("layout generation failed");
}

async fn screenshot(client: &Client) -> DynamicImage {
    const GET_CHANNEL_SIZE: &str = r#"
        const channel = document.querySelector(".channel");
        return [channel.offsetWidth, channel.offsetHeight];
    "#;

    println!("Figuring out the size of the final product");
    let size = client
        .execute(GET_CHANNEL_SIZE, Vec::new())
        .await
        .expect("size detection failed");
    let width: u32 = size[0].as_u64().unwrap().try_into().unwrap();
    let height: u32 = size[1].as_u64().unwrap().try_into().unwrap();

    println!("It will be: {width}w x {height}h");

    println!("Saving to a png");
    let screenshot = client.screenshot().await.expect("screenshot failed");

    println!("Cropping the png");
    let picture = load_from_memory_with_format(&screenshot, ImageFormat::Png).unwrap();
    picture.crop_imm(0, 0, width, height)
}

mod cli {
    use super::*;

    fn pick_parsed<T: FromStr>(prompt: &str) -> Option<T> {
        let input = inquire::Text::new(prompt).prompt().unwrap();
        let value: T = input.parse().ok()?;
        Some(value)
    }

    fn create_message() -> Option<DiscordMessage> {
        let author = inquire::Text::new("Message author:").prompt().ok()?;
        let author_color = inquire::Text::new("Username color (0x...):")
            .prompt()
            .ok()?;
        let author_color = u32::from_str_radix(&author_color, 16).ok()?;
        let content = Text::new("Message content:").prompt().ok()?;
        let timestamp = pick_parsed("Timestamp (skippable):")
            .unwrap_or_else(|| chrono::Utc::now().timestamp() as u64);
        Some(DiscordMessage {
            content,
            timestamp,
            is_edited: false,
            author: MessageAuthor {
                avatar_url: "avatar.png".to_string(),
                color: author_color,
                name: author,
                user_type: MessageAuthorType::User,
            },
        })
    }

    pub fn create_channel() -> Vec<DiscordMessage> {
        let mut messages = Vec::new();

        loop {
            if let Some(message) = create_message() {
                messages.push(message);
            }

            let should_continue = Confirm::new("add another message? (y/n)")
                .prompt()
                .unwrap_or(false);
            if !should_continue {
                break;
            }
        }

        messages
    }
}
