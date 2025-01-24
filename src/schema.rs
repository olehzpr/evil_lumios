// @generated automatically by Diesel CLI.

diesel::table! {
    chats (id) {
        id -> Int4,
        #[max_length = 255]
        chat_id -> Varchar,
        #[max_length = 255]
        group_id -> Nullable<Varchar>,
        #[max_length = 255]
        title -> Varchar,
        description -> Nullable<Text>,
    }
}

diesel::table! {
    gambles (id) {
        id -> Int4,
        user_id -> Int4,
        #[max_length = 255]
        message_id -> Varchar,
        #[max_length = 255]
        gamble_type -> Varchar,
        bet -> Int4,
        change -> Int4,
        is_win -> Bool,
        created_at -> Timestamp,
    }
}

diesel::table! {
    queue_users (id) {
        id -> Int4,
        position -> Int4,
        priority -> Nullable<Int4>,
        is_freezed -> Bool,
        queue_id -> Int4,
        user_id -> Int4,
    }
}

diesel::table! {
    queues (id) {
        id -> Int4,
        #[max_length = 255]
        title -> Varchar,
        #[max_length = 255]
        message_id -> Varchar,
        is_mixed -> Nullable<Bool>,
        is_deleted -> Bool,
        created_at -> Timestamp,
    }
}

diesel::table! {
    timetable_entries (id) {
        id -> Int4,
        week -> Int4,
        day -> Int4,
        timetable_id -> Int4,
        #[max_length = 255]
        class_name -> Varchar,
        #[max_length = 255]
        class_type -> Varchar,
        class_time -> Time,
        link -> Nullable<Text>,
    }
}

diesel::table! {
    timetables (id) {
        id -> Int4,
        #[max_length = 255]
        chat_id -> Varchar,
    }
}

diesel::table! {
    user_stats (id) {
        id -> Int4,
        user_id -> Int4,
        balance -> Int4,
        daily_limit -> Int4,
        daily_used -> Int4,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        #[max_length = 255]
        username -> Varchar,
        #[max_length = 255]
        account_id -> Varchar,
        #[max_length = 255]
        chat_id -> Varchar,
        #[max_length = 255]
        name -> Varchar,
    }
}

diesel::joinable!(gambles -> users (user_id));
diesel::joinable!(queue_users -> queues (queue_id));
diesel::joinable!(queue_users -> users (user_id));
diesel::joinable!(timetable_entries -> timetables (timetable_id));
diesel::joinable!(user_stats -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    chats,
    gambles,
    queue_users,
    queues,
    timetable_entries,
    timetables,
    user_stats,
    users,
);
