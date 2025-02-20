use sea_orm::{entity::*, query::*, DatabaseConnection};
use teloxide::types::UserId;

use crate::entities::{gambles, user_stats, users};

use crate::entities::gambles::Entity as Gamble;
use crate::entities::user_stats::Entity as UserStats;

pub struct FullStats {
    pub user_id: i32,
    pub balance: i32,
    pub daily_limit: i32,
    pub daily_used: i32,
    pub num_of_wins: i32,
    pub num_of_losses: i32,
    pub total_won: i32,
    pub total_lost: i32,
    pub total_gambles: i32,
    pub longest_winning_streak: i32,
    pub longest_losing_streak: i32,
    pub current_streak: i32,
    pub average_bet: f32,
}

pub async fn get_user_stats(
    conn: &DatabaseConnection,
    user_id: UserId,
) -> anyhow::Result<user_stats::Model> {
    let stats = UserStats::find()
        .join(JoinType::InnerJoin, user_stats::Relation::Users.def())
        .filter(users::Column::AccountId.eq(user_id.to_string()))
        .one(conn)
        .await?
        .ok_or_else(|| anyhow::anyhow!("User stats not found"))?;

    Ok(stats)
}

pub async fn get_full_me(conn: &DatabaseConnection, user_id: UserId) -> anyhow::Result<FullStats> {
    let stats = UserStats::find()
        .join(JoinType::InnerJoin, user_stats::Relation::Users.def())
        .filter(users::Column::AccountId.eq(user_id.to_string()))
        .one(conn)
        .await?
        .ok_or_else(|| anyhow::anyhow!("User stats not found"))?;
    let all_gambles = Gamble::find()
        .join(JoinType::InnerJoin, gambles::Relation::Users.def())
        .filter(users::Column::AccountId.eq(user_id.to_string()))
        .order_by_asc(gambles::Column::CreatedAt)
        .all(conn)
        .await?;

    let mut total_won = 0;
    let mut total_lost = 0;
    let mut num_of_wins = 0;
    let mut num_of_losses = 0;
    let mut total_gambles = 0;
    let mut longest_winning_streak = 0;
    let mut longest_losing_streak = 0;
    let mut current_streak = 0i32;
    let mut average_bet = 0.0;

    for gamble in all_gambles.iter() {
        total_gambles += 1;
        if gamble.is_win {
            total_won += gamble.change.abs();
            num_of_wins += 1;
            if current_streak >= 0 {
                current_streak += 1;
            } else {
                current_streak = 1;
            }
            if current_streak > longest_winning_streak {
                longest_winning_streak = current_streak.abs();
            }
        } else {
            total_lost += gamble.change.abs();
            num_of_losses += 1;
            if current_streak <= 0 {
                current_streak -= 1;
            } else {
                current_streak = -1;
            }
            if current_streak.abs() > longest_losing_streak {
                longest_losing_streak = current_streak.abs();
            }
        }
        average_bet += gamble.bet as f32;
    }

    average_bet = if total_gambles > 0 {
        average_bet / total_gambles as f32
    } else {
        0.0
    };

    let stats = FullStats {
        user_id: stats.user_id,
        balance: stats.balance,
        daily_limit: stats.daily_limit,
        daily_used: stats.daily_used,
        total_won,
        total_lost,
        num_of_wins,
        num_of_losses,
        total_gambles,
        longest_winning_streak,
        longest_losing_streak,
        current_streak,
        average_bet,
    };

    Ok(stats)
}

pub async fn transfer_reaction_points(
    conn: &DatabaseConnection,
    sender: users::Model,
    receiver: users::Model,
    points: i32,
) -> anyhow::Result<()> {
    let txn = conn.begin().await?;

    let sender_stats = UserStats::find()
        .filter(user_stats::Column::UserId.eq(sender.id))
        .one(&txn)
        .await?
        .ok_or_else(|| anyhow::anyhow!("Sender stats not found"))?;

    let receiver_stats = UserStats::find()
        .filter(user_stats::Column::UserId.eq(receiver.id))
        .one(&txn)
        .await?
        .ok_or_else(|| anyhow::anyhow!("Receiver stats not found"))?;

    let available = sender_stats.daily_limit - sender_stats.daily_used;
    let actual = if available > points {
        points
    } else {
        available
    };

    if actual == 0 {
        txn.rollback().await?;
        return Ok(());
    }

    let mut sender_stats: user_stats::ActiveModel = sender_stats.into();
    sender_stats.daily_used = Set(sender_stats.daily_used.unwrap() + actual);
    sender_stats.update(&txn).await?;

    let mut receiver_stats: user_stats::ActiveModel = receiver_stats.into();
    receiver_stats.balance = Set(receiver_stats.balance.unwrap() + actual);
    receiver_stats.update(&txn).await?;

    txn.commit().await?;
    Ok(())
}

pub async fn update_balance(
    conn: &DatabaseConnection,
    user_id: i32,
    change: i32,
) -> anyhow::Result<()> {
    let txn = conn.begin().await?;

    let stats = UserStats::find()
        .filter(user_stats::Column::UserId.eq(user_id))
        .one(&txn)
        .await?
        .ok_or_else(|| anyhow::anyhow!("User stats not found"))?;

    let mut stats: user_stats::ActiveModel = stats.into();
    stats.balance = Set(stats.balance.unwrap() + change);
    stats.update(&txn).await?;

    txn.commit().await?;
    Ok(())
}
