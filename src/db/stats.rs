use diesel::{Connection, ExpressionMethods, JoinOnDsl, PgConnection, QueryDsl, RunQueryDsl};
use teloxide::types::UserId;

use crate::schema;

use super::models::{Gamble, User, UserStats};

pub struct FullStats {
    pub user_id: i32,
    pub balance: i32,
    pub daily_limit: i32,
    pub daily_used: i32,
    pub total_won: i32,
    pub total_lost: i32,
    pub total_gambles: i32,
    pub longest_winning_streak: i32,
    pub longest_losing_streak: i32,
    pub current_streak: i32,
    pub average_bet: f32,
}

pub async fn get_user_stats(conn: &mut PgConnection, user_id: UserId) -> anyhow::Result<UserStats> {
    let stats = schema::user_stats::table
        .inner_join(schema::users::table.on(schema::users::id.eq(schema::user_stats::user_id)))
        .filter(schema::users::account_id.eq(user_id.to_string()))
        .select(schema::user_stats::all_columns)
        .first::<UserStats>(conn)?;

    Ok(stats)
}

pub async fn get_full_me(conn: &mut PgConnection, user_id: UserId) -> anyhow::Result<FullStats> {
    let stats = schema::user_stats::table
        .inner_join(schema::users::table.on(schema::users::id.eq(schema::user_stats::user_id)))
        .filter(schema::users::account_id.eq(user_id.to_string()))
        .select(schema::user_stats::all_columns)
        .first::<UserStats>(conn)?;

    let all_gambles = schema::gambles::table
        .inner_join(schema::users::table.on(schema::users::id.eq(schema::gambles::user_id)))
        .filter(schema::users::account_id.eq(user_id.to_string()))
        .select(schema::gambles::all_columns)
        .order(schema::gambles::created_at.asc())
        .load::<Gamble>(conn)?;

    let mut total_won = 0;
    let mut total_lost = 0;
    let mut total_gambles = 0;
    let mut longest_winning_streak = 0;
    let mut longest_losing_streak = 0;
    let mut current_streak = 0i32;
    let mut average_bet = 0.0;

    for gamble in all_gambles.iter() {
        total_gambles += 1;
        if gamble.is_win {
            total_won += gamble.change.abs();
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
        total_gambles,
        longest_winning_streak,
        longest_losing_streak,
        current_streak,
        average_bet,
    };

    Ok(stats)
}

pub async fn transfer_reaction_points(
    conn: &mut PgConnection,
    sender: User,
    receiver: User,
    points: i32,
) -> anyhow::Result<()> {
    conn.transaction::<_, anyhow::Error, _>(|tx| {
        use schema::user_stats;

        let sender_stats = user_stats::table
            .filter(user_stats::user_id.eq(sender.id))
            .first::<UserStats>(tx)?;
        let receiver_stats = user_stats::table
            .filter(user_stats::user_id.eq(receiver.id))
            .first::<UserStats>(tx)?;

        let available = sender_stats.daily_limit - sender_stats.daily_used;
        let actual = if available > points {
            points
        } else {
            available
        };

        if actual == 0 {
            return Ok(());
        }

        diesel::update(user_stats::table.filter(user_stats::user_id.eq(sender.id)))
            .set(user_stats::daily_used.eq(sender_stats.daily_used + actual))
            .execute(tx)?;

        diesel::update(user_stats::table.filter(user_stats::user_id.eq(receiver.id)))
            .set(user_stats::balance.eq(receiver_stats.balance + actual))
            .execute(tx)?;

        Ok(())
    })
}
