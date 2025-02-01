CREATE TABLE chats (
  id SERIAL PRIMARY KEY,
  chat_id VARCHAR(255) NOT NULL UNIQUE,
  group_id VARCHAR(255) UNIQUE,
  title VARCHAR(255) NOT NULL,
  description TEXT
);

CREATE TABLE users (
  id SERIAL PRIMARY KEY,
  username VARCHAR(255) NOT NULL UNIQUE,
  account_id VARCHAR(255) NOT NULL,
  chat_id VARCHAR(255) NOT NULL REFERENCES chats(chat_id),
  name VARCHAR(255) NOT NULL,
  CONSTRAINT unique_account_chat UNIQUE(account_id, chat_id)
);

CREATE TABLE user_stats (
  id SERIAL PRIMARY KEY,
  user_id INTEGER NOT NULL REFERENCES users(id),
  balance INTEGER NOT NULL DEFAULT 1000,
  daily_limit INTEGER NOT NULL DEFAULT 100,
  daily_used INTEGER NOT NULL DEFAULT 0,
  CONSTRAINT unique_user_id UNIQUE(user_id)
);

CREATE TABLE gambles (
  id SERIAL PRIMARY KEY,
  user_id INTEGER NOT NULL REFERENCES users(id),
  message_id VARCHAR(255) NOT NULL UNIQUE,
  gamble_type VARCHAR(255) NOT NULL,
  bet INTEGER NOT NULL,
  change INTEGER NOT NULL,
  is_win BOOLEAN NOT NULL,
  created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE queues (
  id SERIAL PRIMARY KEY,
  title VARCHAR(255) NOT NULL,
  chat_id VARCHAR(255) NOT NULL,
  message_id VARCHAR(255) NOT NULL,
  is_mixed BOOLEAN,
  is_priority BOOLEAN NOT NULL DEFAULT FALSE,
  is_deleted BOOLEAN NOT NULL DEFAULT FALSE,
  created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE queue_users (
  id SERIAL PRIMARY KEY,
  position INTEGER NOT NULL,
  priority INTEGER DEFAULT NULL,
  is_freezed BOOLEAN DEFAULT NULL,
  queue_id INTEGER NOT NULL REFERENCES queues(id),
  user_id INTEGER NOT NULL REFERENCES users(id),
  CONSTRAINT unique_queue_user UNIQUE(queue_id, user_id)
);

CREATE TABLE timetables (
  id SERIAL PRIMARY KEY,
  chat_id VARCHAR(255) NOT NULL REFERENCES chats(chat_id)
);

CREATE TABLE timetable_entries (
  id SERIAL PRIMARY KEY,
  week INTEGER NOT NULL,
  day INTEGER NOT NULL,
  timetable_id INTEGER NOT NULL REFERENCES timetables(id),
  class_name VARCHAR(255) NOT NULL,
  class_type VARCHAR(255) NOT NULL,
  class_time TIME NOT NULL,
  link TEXT
);