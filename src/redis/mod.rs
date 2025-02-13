use r2d2_redis::redis::Commands;
use setup::RedisStore;
use teloxide::types::{ChatId, Message, MessageId, UserId};

use crate::db::models::{Chat, TimetableEntry, User};

pub mod setup;

#[rustfmt::skip]
pub trait RedisCache {
    fn clear_all_cache(&self) -> anyhow::Result<()>;
    fn store_timetable_entries(&self, chat_id: ChatId, timetable_entries: Vec<TimetableEntry>) -> anyhow::Result<()>;
    fn get_timetable_entries(&self, chat_id: ChatId) -> anyhow::Result<Vec<TimetableEntry>>;
    fn clear_timetable_entries(&self, chat_id: ChatId) -> anyhow::Result<()>;
    fn store_message(&self, message: Message) -> anyhow::Result<()>;
    fn get_message(&self, chat_id: ChatId, message_id: MessageId) -> anyhow::Result<Message>;
    fn store_user(&self, user: User) -> anyhow::Result<()>;
    fn get_user(&self, user_id: UserId) -> anyhow::Result<User>;
    fn store_chat(&self, chat: Chat) -> anyhow::Result<()>;
    fn get_chat(&self, chat_id: ChatId) -> anyhow::Result<Chat>;
    fn store_chat_ids(&self, chat_ids: Vec<ChatId>) -> anyhow::Result<()>;
    fn get_all_chat_ids(&self) -> anyhow::Result<Vec<ChatId>>;
}

impl RedisCache for RedisStore {
    fn clear_all_cache(&self) -> anyhow::Result<()> {
        #[cfg(not(debug_assertions))]
        {
            use r2d2_redis::redis::ConnectionLike;
            let mut con = self.get_connection()?;
            con.req_packed_command(&redis::cmd("FLUSHALL").get_packed_command())?;
            tracing::info!("All cache has been deleted");
        }
        Ok(())
    }
    fn store_timetable_entries(
        &self,
        chat_id: ChatId,
        timetable_entries: Vec<TimetableEntry>,
    ) -> anyhow::Result<()> {
        let mut con = self.get_connection()?;
        let list_key = format!("timetable_entries:{}", chat_id);
        for entry in timetable_entries {
            let serialized = serde_json::to_string(&entry)?;
            let key = format!("timetable_entry:{}:{}", chat_id, entry.id);
            let _: () = con.set(&key, serialized)?;
            let _: () = con.lpush(&list_key, entry.id.to_string())?;
        }

        Ok(())
    }
    fn get_timetable_entries(&self, chat_id: ChatId) -> anyhow::Result<Vec<TimetableEntry>> {
        let mut con = self.get_connection()?;
        let list_key = format!("timetable_entries:{}", chat_id);
        if !con.exists(&list_key)? {
            return Err(anyhow::anyhow!("No timetable entries found"));
        }
        let entry_ids: Vec<String> = con.lrange(&list_key, 0, -1)?;
        let mut entries = Vec::new();
        for entry_id in entry_ids {
            let key = format!("timetable_entry:{}:{}", chat_id, entry_id);
            let serialized: String = con.get(&key)?;
            let entry: TimetableEntry = serde_json::from_str(&serialized)?;
            entries.push(entry);
        }

        Ok(entries)
    }
    fn clear_timetable_entries(&self, chat_id: ChatId) -> anyhow::Result<()> {
        let mut con = self.get_connection()?;
        let list_key = format!("timetable_entries:{}", chat_id);
        let entry_ids: Vec<String> = con.lrange(&list_key, 0, -1)?;
        for entry_id in entry_ids {
            let key = format!("timetable_entry:{}:{}", chat_id, entry_id);
            let _: () = con.del(&key)?;
        }
        let _: () = con.del(&list_key)?;

        Ok(())
    }
    fn store_message(&self, message: Message) -> anyhow::Result<()> {
        let mut con = self.get_connection()?;

        let serialized = serde_json::to_string(&message)?;
        let key = format!("message:{}:{}", message.chat.id, message.id);
        let _: () = con.set(&key, serialized)?;

        let list_key = format!("messages:{}", message.chat.id);
        let _: () = con.lpush(&list_key, message.id.to_string())?;

        if con.llen::<&String, i64>(&list_key).unwrap() > 100 {
            let removed_id: String = con.rpop(&list_key)?;
            let removed_key = format!("message:{}:{}", message.chat.id, removed_id);
            let _: () = con.del(&removed_key)?;
        }

        Ok(())
    }
    fn get_message(&self, chat_id: ChatId, message_id: MessageId) -> anyhow::Result<Message> {
        let mut con = self.get_connection()?;
        let key = format!("message:{}:{}", chat_id, message_id);
        let serialized: String = con.get(&key)?;
        let message: Message = serde_json::from_str(&serialized)?;

        Ok(message)
    }
    fn store_user(&self, user: User) -> anyhow::Result<()> {
        let mut con = self.get_connection()?;
        let serialized = serde_json::to_string(&user)?;
        let key = format!("user:{}", user.id);
        let _: () = con.set(&key, serialized)?;

        Ok(())
    }
    fn get_user(&self, user_id: UserId) -> anyhow::Result<User> {
        let mut con = self.get_connection()?;
        let key = format!("user:{}", user_id);
        let serialized: String = con.get(&key)?;
        let user: User = serde_json::from_str(&serialized)?;
        Ok(user)
    }
    fn store_chat(&self, chat: Chat) -> anyhow::Result<()> {
        let mut con = self.get_connection()?;
        let serialized = serde_json::to_string(&chat)?;
        let key = format!("chat:{}", chat.id);
        let _: () = con.set(&key, serialized)?;

        Ok(())
    }
    fn get_chat(&self, chat_id: ChatId) -> anyhow::Result<Chat> {
        let mut con = self.get_connection()?;
        let key = format!("chat:{}", chat_id);
        let serialized: String = con.get(&key)?;
        let chat: Chat = serde_json::from_str(&serialized)?;

        Ok(chat)
    }
    fn store_chat_ids(&self, chat_ids: Vec<ChatId>) -> anyhow::Result<()> {
        let mut con = self.get_connection()?;
        let key = "chat_ids";
        let serialized = serde_json::to_string(&chat_ids)?;
        let _: () = con.set(key, serialized)?;

        Ok(())
    }
    fn get_all_chat_ids(&self) -> anyhow::Result<Vec<ChatId>> {
        let mut con = self.get_connection()?;
        let key = "chat_ids";
        let serialized: String = con.get(key)?;
        let chat_ids: Vec<ChatId> = serde_json::from_str(&serialized)?;

        Ok(chat_ids)
    }
}
