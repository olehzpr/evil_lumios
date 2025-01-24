use crate::state::Event;
use teloxide::{
    payloads::SendMessageSetters,
    prelude::Requester,
    types::{ChatId, InlineKeyboardMarkup, LinkPreviewOptions, Message, ParseMode},
    Bot, RequestError,
};
use tokio::sync::broadcast::Sender;

const DISABLED_LINK_PREVIEW_OPTIONS: LinkPreviewOptions = LinkPreviewOptions {
    is_disabled: true,
    url: None,
    prefer_large_media: false,
    prefer_small_media: false,
    show_above_text: false,
};

pub enum Msg<'a> {
    Regular(ChatId, &'a str),
    Temp(ChatId, &'a str, Sender<Event>),
}

#[async_trait::async_trait]
pub trait ExtendedBot {
    async fn send_extended(&self, message: Msg<'_>) -> Result<Message, RequestError>;
    async fn send_with_keyboard(
        &self,
        message: Msg<'_>,
        keyboard: InlineKeyboardMarkup,
    ) -> Result<Message, RequestError>;
}

#[async_trait::async_trait]
impl ExtendedBot for Bot {
    async fn send_extended(&self, message: Msg<'_>) -> Result<Message, RequestError> {
        match message {
            Msg::Regular(chat_id, content) => {
                self.send_message(chat_id, content)
                    .link_preview_options(DISABLED_LINK_PREVIEW_OPTIONS)
                    .parse_mode(ParseMode::MarkdownV2)
                    .await
            }
            Msg::Temp(chat_id, content, sender) => {
                let message = self
                    .send_message(chat_id, content)
                    .link_preview_options(DISABLED_LINK_PREVIEW_OPTIONS)
                    .parse_mode(ParseMode::MarkdownV2)
                    .await?;

                if let Err(e) = sender.send(Event::DeleteMessage {
                    chat_id,
                    message_id: message.id,
                }) {
                    eprintln!("Failed to send delete message event: {:?}", e);
                }

                Ok(message)
            }
        }
    }

    async fn send_with_keyboard(
        &self,
        message: Msg<'_>,
        keyboard: InlineKeyboardMarkup,
    ) -> Result<Message, RequestError> {
        match message {
            Msg::Regular(chat_id, content) => {
                self.send_message(chat_id, content)
                    .link_preview_options(DISABLED_LINK_PREVIEW_OPTIONS)
                    .parse_mode(ParseMode::MarkdownV2)
                    .reply_markup(keyboard)
                    .await
            }
            Msg::Temp(chat_id, content, sender) => {
                let message = self
                    .send_message(chat_id, content)
                    .link_preview_options(DISABLED_LINK_PREVIEW_OPTIONS)
                    .parse_mode(ParseMode::MarkdownV2)
                    .reply_markup(keyboard)
                    .await?;

                if let Err(e) = sender.send(Event::DeleteMessage {
                    chat_id,
                    message_id: message.id,
                }) {
                    eprintln!("Failed to send delete message event: {:?}", e);
                }

                Ok(message)
            }
        }
    }
}

#[macro_export]
macro_rules! send_autodelete {
    ($bot:ident, $msg:ident, $state:ident, $res:expr, $keyboard:expr) => {
        if let Err(e) = $state.sender.send(Event::DeleteMessage {
            chat_id: $msg.chat.id,
            message_id: $msg.id,
        }) {
            eprintln!("Failed to send delete message event: {:?}", e);
        }

        $bot.send_with_keyboard(
            Msg::Temp($msg.chat.id, $res, $state.sender.clone()),
            $keyboard,
        )
        .await?;
    };
    ($bot:ident, $msg:ident, $state:ident, $res:expr) => {
        if let Err(e) = $state.sender.send(Event::DeleteMessage {
            chat_id: $msg.chat.id,
            message_id: $msg.id,
        }) {
            eprintln!("Failed to send delete message event: {:?}", e);
        }

        $bot.send_extended(Msg::Temp($msg.chat.id, $res, $state.sender.clone()))
            .await?;
    };
}

#[macro_export]
macro_rules! send_message {
    ($bot:ident, $msg:ident, $res:expr) => {
        $bot.send_extended(Msg::Regular($msg.chat.id, $res)).await?;
    };
}
