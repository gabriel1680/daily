use chrono::{DateTime, Days, Utc};
use reqwest::header::CONTENT_TYPE;
use reqwest::{Client, Method};

use crate::user_credentials::UserCredentials;
use crate::utils::get_env_var;

pub fn make_request(credentials: UserCredentials) -> reqwest::RequestBuilder {
    let (now_date, day_before_date) = get_days_range();
    let base_url = "https://api.track.toggl.com/api/v9/me/time_entries".to_owned();
    let url = base_url + "?start_date=" + &day_before_date + "&end_date=" + &now_date;
    Client::new()
        .request(Method::GET, url)
        .basic_auth(credentials.username, Some(credentials.password))
        .header(CONTENT_TYPE, "application/json")
}

fn get_days_range() -> (String, String) {
    let days = get_env_var("DAYS_BEFORE_DATE").parse::<u64>().ok().unwrap_or_default();
    let now = Utc::now();
    let day_before_date = now.checked_sub_days(Days::new(days)).unwrap_or(now);
    (format_date(now), format_date(day_before_date))
}

fn format_date(date: DateTime<Utc>) -> String {
    let fmt = "%Y-%m-%d";
    date.format(fmt).to_string()
}