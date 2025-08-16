use chrono::{DateTime, Datelike, Local, NaiveTime, Timelike, Utc, Weekday};
use chrono::TimeZone;

// Configuration for game night
pub struct GameNightConfig {
    pub day_of_week: Weekday,
    pub start_time: NaiveTime,
    pub duration_hours: u32,
    pub timezone: chrono_tz::Tz,
}

impl Default for GameNightConfig {
    fn default() -> Self {
        Self {
            day_of_week: Weekday::Fri,  // Friday
            start_time: NaiveTime::from_hms_opt(20, 0, 0).unwrap(), // 8:00 PM
            duration_hours: 4,
            timezone: chrono_tz::US::Eastern,
        }
    }
}

pub fn get_next_game_night(config: &GameNightConfig) -> DateTime<Utc> {
    let now = Utc::now();
    let local_now = config.timezone.from_utc_datetime(&now.naive_utc());
    
    let days_until_game_night = days_until_weekday(local_now.weekday(), config.day_of_week);
    
    let mut next_game_night = local_now.date_naive() + chrono::Duration::days(days_until_game_night as i64);
    
    // If it's game night today but the time has passed, schedule for next week
    if days_until_game_night == 0 && local_now.time() > config.start_time {
        next_game_night = next_game_night + chrono::Duration::weeks(1);
    }
    
    let game_night_datetime = config.timezone
        .from_local_datetime(&next_game_night.and_time(config.start_time))
        .unwrap();
    
    game_night_datetime.with_timezone(&Utc)
}

pub fn is_game_night_now(config: &GameNightConfig) -> bool {
    let now = Utc::now();
    let local_now = config.timezone.from_utc_datetime(&now.naive_utc());
    
    if local_now.weekday() != config.day_of_week {
        return false;
    }
    
    let current_time = local_now.time();
    let end_time = config.start_time + chrono::Duration::hours(config.duration_hours as i64);
    
    current_time >= config.start_time && current_time <= end_time
}

pub fn time_until_game_night(config: &GameNightConfig) -> chrono::Duration {
    let now = Utc::now();
    let next_game_night = get_next_game_night(config);
    next_game_night - now
}

fn days_until_weekday(from: Weekday, to: Weekday) -> u32 {
    let from_num = from.num_days_from_monday();
    let to_num = to.num_days_from_monday();
    
    if to_num >= from_num {
        to_num - from_num
    } else {
        7 - from_num + to_num
    }
}

// Format for !nextgame - detailed countdown information
pub fn format_next_game_night(config: &GameNightConfig) -> String {
    let next_game_night = get_next_game_night(config);
    let local_time = config.timezone.from_utc_datetime(&next_game_night.naive_utc());
    let duration = time_until_game_night(config);
    
    let total_seconds = duration.num_seconds();
    let days = duration.num_days();
    let hours = duration.num_hours() % 24;
    let minutes = duration.num_minutes() % 60;
    let seconds = total_seconds % 60;
    
    // Get what game to suggest
    let game_suggestion = get_next_game_suggestion(days as usize);
    
    format!(
        "📅 **Next Game Night Details**\n\
        ━━━━━━━━━━━━━━━━━━━━━\n\
        🗓️ **Date:** {}\n\
        🕐 **Start Time:** {} {}\n\
        ⏱️ **Duration:** {} hours\n\
        \n\
        ⏳ **Countdown:**\n\
        ```\n\
        {} days, {} hours, {} minutes, {} seconds\n\
        ```\n\
        \n\
        🎮 **Planned Game:** {}\n\
        \n\
        💡 **Pro tip:** Set a reminder so you don't miss it!",
        local_time.format("%A, %B %d, %Y"),
        local_time.format("%I:%M %p"),
        config.timezone,
        config.duration_hours,
        days,
        hours,
        minutes,
        seconds,
        game_suggestion
    )
}

// Format for !gamenight - quick status check
pub fn format_game_night_status(config: &GameNightConfig) -> String {
    if is_game_night_now(config) {
        let now = Utc::now();
        let local_now = config.timezone.from_utc_datetime(&now.naive_utc());
        let end_time = config.start_time + chrono::Duration::hours(config.duration_hours as i64);
        let time_remaining = end_time - local_now.time();
        
        let hours_left = time_remaining.num_hours();
        let minutes_left = time_remaining.num_minutes() % 60;
        
        format!(
            "🔴 **GAME NIGHT IS LIVE NOW!** 🔴\n\
            ━━━━━━━━━━━━━━━━━━━━━\n\
            🎮 We're currently playing!\n\
            ⏰ Time remaining: {} hours {} minutes\n\
            🔗 Hop in the voice channel!\n\
            \n\
            Use `!suggest` to see what we're playing!",
            hours_left,
            minutes_left
        )
    } else {
        // Simple status for when it's not game night
        let next_game_night = get_next_game_night(config);
        let local_time = config.timezone.from_utc_datetime(&next_game_night.naive_utc());
        let duration = time_until_game_night(config);
        
        let days = duration.num_days();
        let hours = duration.num_hours() % 24;
        
        if days == 0 && hours < 6 {
            format!(
                "⏰ **Game Night Starting Soon!**\n\
                🎮 Tonight at {} {}\n\
                ⏳ Only {} hours {} minutes away!\n\
                🔔 Get ready to game!",
                local_time.format("%I:%M %p"),
                config.timezone,
                hours,
                duration.num_minutes() % 60
            )
        } else if days == 0 {
            format!(
                "📅 **Game Night is Today!**\n\
                🕐 Starting at {} {}\n\
                ⏳ In {} hours {} minutes",
                local_time.format("%I:%M %p"),
                config.timezone,
                hours,
                duration.num_minutes() % 60
            )
        } else if days == 1 {
            format!(
                "📅 **Game Night is Tomorrow!**\n\
                🕐 {} at {} {}",
                local_time.format("%A"),
                local_time.format("%I:%M %p"),
                config.timezone
            )
        } else {
            format!(
                "📅 **Next Game Night:**\n\
                🗓️ {} (in {} days)\n\
                🕐 {} {}",
                local_time.format("%A, %B %d"),
                days,
                local_time.format("%I:%M %p"),
                config.timezone
            )
        }
    }
}

// Game suggestions based on the date
pub fn get_game_suggestion() -> &'static str {
    let suggestions = vec![
        "🎯 **Tonight's Game Suggestions:**\n• Valorant\n• CS2\n• Overwatch 2",
        "🎯 **Tonight's Game Suggestions:**\n• League of Legends\n• Dota 2\n• Heroes of the Storm",
        "🎯 **Tonight's Game Suggestions:**\n• Minecraft\n• Terraria\n• Valheim",
        "🎯 **Tonight's Game Suggestions:**\n• Among Us\n• Fall Guys\n• Jackbox Party Pack",
        "🎯 **Tonight's Game Suggestions:**\n• Rocket League\n• FIFA\n• NBA 2K",
    ];
    
    let now = Utc::now();
    let index = (now.timestamp() as usize / 86400) % suggestions.len();
    suggestions[index]
}

fn get_next_game_suggestion(days_away: usize) -> &'static str {
    let games = vec![
        "@Amaterasu is cheating in wordle"
    ];
    
    games[days_away % games.len()]
}

// Custom game night configurations for special events
pub fn get_special_game_night(date: DateTime<Utc>) -> Option<String> {
    let local_date = chrono_tz::US::Eastern.from_utc_datetime(&date.naive_utc());
    
    match (local_date.month(), local_date.day()) {
        (12, 24) => Some("🎄 **Christmas Eve Game Night!** 🎅".to_string()),
        (12, 31) => Some("🎊 **New Year's Eve Game Night!** 🥳".to_string()),
        (10, 31) => Some("🎃 **Halloween Game Night!** 👻".to_string()),
        (7, 4) => Some("🎆 **Independence Day Game Night!** 🇺🇸".to_string()),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_days_until_weekday() {
        assert_eq!(days_until_weekday(Weekday::Mon, Weekday::Fri), 4);
        assert_eq!(days_until_weekday(Weekday::Fri, Weekday::Mon), 3);
        assert_eq!(days_until_weekday(Weekday::Wed, Weekday::Wed), 0);
    }

    #[test]
    fn test_game_night_config() {
        let config = GameNightConfig::default();
        assert_eq!(config.day_of_week, Weekday::Fri);
        assert_eq!(config.start_time.hour(), 20);
        assert_eq!(config.duration_hours, 4);
    }
}
