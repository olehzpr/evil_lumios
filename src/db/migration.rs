use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Create `chats` table
        manager
            .create_table(
                Table::create()
                    .table(Chats::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Chats::Id)
                            .integer()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(Chats::ChatId)
                            .string()
                            .not_null()
                            .unique_key(),
                    )
                    .col(ColumnDef::new(Chats::GroupId).string().unique_key())
                    .col(ColumnDef::new(Chats::Title).string().not_null())
                    .col(ColumnDef::new(Chats::Description).text())
                    .to_owned(),
            )
            .await?;

        // Create `users` table
        manager
            .create_table(
                Table::create()
                    .table(Users::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Users::Id)
                            .integer()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(Users::Username)
                            .string()
                            .not_null()
                            .unique_key(),
                    )
                    .col(ColumnDef::new(Users::AccountId).string().not_null())
                    .col(ColumnDef::new(Users::ChatId).string().not_null())
                    .col(ColumnDef::new(Users::Name).string().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_users_chat_id")
                            .from(Users::Table, Users::ChatId)
                            .to(Chats::Table, Chats::ChatId),
                    )
                    .to_owned(),
            )
            .await?;

        // Create `user_stats` table
        manager
            .create_table(
                Table::create()
                    .table(UserStats::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(UserStats::Id)
                            .integer()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(UserStats::UserId).integer().not_null())
                    .col(ColumnDef::new(UserStats::Balance).integer().default(1000))
                    .col(ColumnDef::new(UserStats::DailyLimit).integer().default(100))
                    .col(ColumnDef::new(UserStats::DailyUsed).integer().default(0))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_user_stats_user_id")
                            .from(UserStats::Table, UserStats::UserId)
                            .to(Users::Table, Users::Id),
                    )
                    .to_owned(),
            )
            .await?;

        // Create `gambles` table
        manager
            .create_table(
                Table::create()
                    .table(Gambles::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Gambles::Id)
                            .integer()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Gambles::UserId).integer().not_null())
                    .col(
                        ColumnDef::new(Gambles::MessageId)
                            .string()
                            .not_null()
                            .unique_key(),
                    )
                    .col(ColumnDef::new(Gambles::GambleType).string().not_null())
                    .col(ColumnDef::new(Gambles::Bet).integer().not_null())
                    .col(ColumnDef::new(Gambles::Change).integer().not_null())
                    .col(ColumnDef::new(Gambles::IsWin).boolean().not_null())
                    .col(
                        ColumnDef::new(Gambles::CreatedAt)
                            .timestamp()
                            .default(Expr::current_timestamp()),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_gambles_user_id")
                            .from(Gambles::Table, Gambles::UserId)
                            .to(Users::Table, Users::Id),
                    )
                    .to_owned(),
            )
            .await?;

        // Create `queues` table
        manager
            .create_table(
                Table::create()
                    .table(Queues::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Queues::Id)
                            .integer()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Queues::Title).string().not_null())
                    .col(ColumnDef::new(Queues::ChatId).string().not_null())
                    .col(ColumnDef::new(Queues::MessageId).string().not_null())
                    .col(ColumnDef::new(Queues::IsMixed).boolean())
                    .col(ColumnDef::new(Queues::IsPriority).boolean().default(false))
                    .col(ColumnDef::new(Queues::IsDeleted).boolean().default(false))
                    .col(
                        ColumnDef::new(Queues::CreatedAt)
                            .timestamp()
                            .default(Expr::current_timestamp()),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_queues_chat_id")
                            .from(Queues::Table, Queues::ChatId)
                            .to(Chats::Table, Chats::ChatId),
                    )
                    .to_owned(),
            )
            .await?;

        // Create `queue_users` table
        manager
            .create_table(
                Table::create()
                    .table(QueueUsers::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(QueueUsers::Id)
                            .integer()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(QueueUsers::Position).integer().not_null())
                    .col(ColumnDef::new(QueueUsers::Priority).integer())
                    .col(ColumnDef::new(QueueUsers::IsFreezed).boolean())
                    .col(ColumnDef::new(QueueUsers::QueueId).integer().not_null())
                    .col(ColumnDef::new(QueueUsers::UserId).integer().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_queue_users_queue_id")
                            .from(QueueUsers::Table, QueueUsers::QueueId)
                            .to(Queues::Table, Queues::Id),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_queue_users_user_id")
                            .from(QueueUsers::Table, QueueUsers::UserId)
                            .to(Users::Table, Users::Id),
                    )
                    .index(
                        Index::create()
                            .name("idx_queue_users_queue_id_user_id")
                            .table(QueueUsers::Table)
                            .col(QueueUsers::QueueId)
                            .col(QueueUsers::UserId)
                            .unique(),
                    )
                    .to_owned(),
            )
            .await?;

        // Create `timetables` table
        manager
            .create_table(
                Table::create()
                    .table(Timetables::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Timetables::Id)
                            .integer()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Timetables::ChatId).string().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_timetables_chat_id")
                            .from(Timetables::Table, Timetables::ChatId)
                            .to(Chats::Table, Chats::ChatId),
                    )
                    .to_owned(),
            )
            .await?;

        // Create `timetable_entries` table
        manager
            .create_table(
                Table::create()
                    .table(TimetableEntries::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(TimetableEntries::Id)
                            .integer()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(TimetableEntries::Week).integer().not_null())
                    .col(ColumnDef::new(TimetableEntries::Day).integer().not_null())
                    .col(
                        ColumnDef::new(TimetableEntries::TimetableId)
                            .integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(TimetableEntries::ClassName)
                            .string()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(TimetableEntries::ClassType)
                            .string()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(TimetableEntries::ClassTime)
                            .time()
                            .not_null(),
                    )
                    .col(ColumnDef::new(TimetableEntries::Link).text())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_timetable_entries_timetable_id")
                            .from(TimetableEntries::Table, TimetableEntries::TimetableId)
                            .to(Timetables::Table, Timetables::Id),
                    )
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Chats::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Users::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(UserStats::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Gambles::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Queues::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(QueueUsers::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Timetables::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(TimetableEntries::Table).to_owned())
            .await?;
        Ok(())
    }
}

#[derive(Iden)]
pub enum Chats {
    Table,
    Id,
    ChatId,
    GroupId,
    Title,
    Description,
}

#[derive(Iden)]
pub enum Users {
    Table,
    Id,
    Username,
    AccountId,
    ChatId,
    Name,
}

#[derive(Iden)]
pub enum UserStats {
    Table,
    Id,
    UserId,
    Balance,
    DailyLimit,
    DailyUsed,
}

#[derive(Iden)]
pub enum Gambles {
    Table,
    Id,
    UserId,
    MessageId,
    GambleType,
    Bet,
    Change,
    IsWin,
    CreatedAt,
}

#[derive(Iden)]
pub enum Queues {
    Table,
    Id,
    Title,
    ChatId,
    MessageId,
    IsMixed,
    IsPriority,
    IsDeleted,
    CreatedAt,
}

#[derive(Iden)]
pub enum QueueUsers {
    Table,
    Id,
    Position,
    Priority,
    IsFreezed,
    QueueId,
    UserId,
}

#[derive(Iden)]
pub enum Timetables {
    Table,
    Id,
    ChatId,
}

#[derive(Iden)]
pub enum TimetableEntries {
    Table,
    Id,
    Week,
    Day,
    TimetableId,
    ClassName,
    ClassType,
    ClassTime,
    Link,
}

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![Box::new(Migration)]
    }
}
