use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct DiscordMessage {
    pub author: MessageAuthor,
    pub timestamp: u64,
    pub content: String,
    pub is_edited: bool,
}

#[derive(Debug, Serialize)]
pub struct MessageAuthor {
    pub name: String,
    pub user_type: MessageAuthorType,
    pub color: u32,
    pub avatar_url: String,
}

#[derive(Debug, Serialize)]
pub enum MessageAuthorType {
    User,
    Bot,
    System,
}
