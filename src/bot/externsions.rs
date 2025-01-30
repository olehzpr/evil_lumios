use crate::state::Event;
use reqwest::Url;
use teloxide::{
    payloads::{SendMessage, SendMessageSetters, SendPhoto, SendPhotoSetters},
    prelude::Requester,
    requests::{JsonRequest, MultipartRequest},
    types::{ChatId, InlineKeyboardMarkup, InputFile, LinkPreviewOptions, Message, ParseMode},
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

enum MessageBuilder {
    Photo(MultipartRequest<SendPhoto>),
    Text(JsonRequest<SendMessage>),
}

pub enum Msg<'a> {
    Regular {
        chat_id: ChatId,
        content: &'a str,
        photo_url: Option<&'a str>,
        keyboard: Option<InlineKeyboardMarkup>,
    },
    Temp {
        chat_id: ChatId,
        content: &'a str,
        photo_url: Option<&'a str>,
        sender: Sender<Event>,
        keyboard: Option<InlineKeyboardMarkup>,
    },
}

#[async_trait::async_trait]
pub trait ExtendedBot {
    async fn send_extended(&self, message: Msg<'_>) -> Result<Message, RequestError>;
}

#[async_trait::async_trait]
impl ExtendedBot for Bot {
    async fn send_extended(&self, message: Msg<'_>) -> Result<Message, RequestError> {
        match message {
            Msg::Regular {
                chat_id,
                content,
                photo_url,
                keyboard,
            } => {
                let mut builder = if let Some(url) = photo_url {
                    MessageBuilder::Photo(
                        self.send_photo(chat_id, InputFile::url(Url::parse(url).unwrap()))
                            .caption(content),
                    )
                } else {
                    MessageBuilder::Text(
                        self.send_message(chat_id, content)
                            .link_preview_options(DISABLED_LINK_PREVIEW_OPTIONS),
                    )
                };

                match &mut builder {
                    MessageBuilder::Photo(b) => b.parse_mode = Some(ParseMode::MarkdownV2),
                    MessageBuilder::Text(b) => b.parse_mode = Some(ParseMode::MarkdownV2),
                }

                if let Some(kb) = keyboard {
                    match &mut builder {
                        MessageBuilder::Photo(b) => b.reply_markup = Some(kb.into()),
                        MessageBuilder::Text(b) => b.reply_markup = Some(kb.into()),
                    }
                }

                match builder {
                    MessageBuilder::Photo(b) => b.await,
                    MessageBuilder::Text(b) => b.await,
                }
            }
            Msg::Temp {
                chat_id,
                content,
                photo_url,
                sender,
                keyboard,
            } => {
                let mut builder = if let Some(url) = photo_url {
                    MessageBuilder::Photo(
                        self.send_photo(chat_id, InputFile::url(Url::parse(url).unwrap()))
                            .caption(content),
                    )
                } else {
                    MessageBuilder::Text(
                        self.send_message(chat_id, content)
                            .link_preview_options(DISABLED_LINK_PREVIEW_OPTIONS),
                    )
                };

                match &mut builder {
                    MessageBuilder::Photo(b) => b.parse_mode = Some(ParseMode::MarkdownV2),
                    MessageBuilder::Text(b) => b.parse_mode = Some(ParseMode::MarkdownV2),
                }

                if let Some(kb) = keyboard {
                    match &mut builder {
                        MessageBuilder::Photo(b) => b.reply_markup = Some(kb.into()),
                        MessageBuilder::Text(b) => b.reply_markup = Some(kb.into()),
                    }
                }

                let message = match builder {
                    MessageBuilder::Photo(b) => b.await?,
                    MessageBuilder::Text(b) => b.await?,
                };

                let _ = sender.send(Event::DeleteMessage {
                    chat_id,
                    message_id: message.id,
                });

                Ok(message)
            }
        }
    }
}

#[macro_export]
macro_rules! send_regular {
    ($bot:ident, $msg:ident, $content:expr $(, $key:ident = $value:expr)*) => {{
        let chat_id = $msg.chat.id;
        let photo_url = None;
        let keyboard = None;

        $(
            match stringify!($key) {
                "photo_url" => photo_url = Some($value),
                "keyboard" => keyboard = Some($value),
                _ => compile_error!(concat!("Unknown parameter: ", stringify!($key))),
            }
        )*

        $bot.send_extended(Msg::Regular {
            chat_id,
            content: $content,
            photo_url,
            keyboard,
        }).await?
    }};
}

#[macro_export]
macro_rules! send_autodelete {
    ($bot:ident, $state:ident, $msg:ident, $content:expr $(, $key:ident = $value:expr)*) => {{
        let _ = $state.sender.send($crate::Event::DeleteMessage {
            chat_id: $msg.chat.id,
            message_id: $msg.id,
        });

        let chat_id = $msg.chat.id;
        let photo_url = None;
        let keyboard = None;

        $(
            match stringify!($key) {
                "photo_url" => photo_url = Some($value),
                "keyboard" => keyboard = Some($value),
                _ => compile_error!(concat!("Unknown parameter: ", stringify!($key))),
            }
        )*

        $bot.send_extended(Msg::Temp {
            chat_id,
            content: $content,
            photo_url,
            sender: $state.sender.clone(),
            keyboard,
        }).await?
    }};
}

#[macro_export]
macro_rules! send_temp {
    ($bot:ident, $state:ident, $chat_id:ident, $content:expr $(, $key:ident = $value:expr)*) => {{

        let photo_url = None;
        let keyboard = None;

        $(
            match stringify!($key) {
                "photo_url" => photo_url = Some($value),
                "keyboard" => keyboard = Some($value),
                _ => compile_error!(concat!("Unknown parameter: ", stringify!($key))),
            }
        )*

        $bot.send_extended(Msg::Temp {
            $chat_id,
            content: $content,
            photo_url,
            sender: $state.sender.clone(),
            keyboard,
        }).await?
    }};
}
