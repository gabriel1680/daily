use chrono::{DateTime, Days, Utc};
use reqwest::header::CONTENT_TYPE;
use reqwest::{Client, Method};

use crate::get_tasks_input::GetTasksInput;

pub fn make_request(input: GetTasksInput) -> reqwest::RequestBuilder {
    let (now_date, day_before_date) = get_days_range(input.days_before_today);
    let base_url = "https://api.track.toggl.com/api/v9/me/time_entries".to_owned();
    let url = base_url + "?start_date=" + &day_before_date + "&end_date=" + &now_date;
    Client::new()
        .request(Method::GET, url)
        .basic_auth(input.username, Some(input.password))
        .header(CONTENT_TYPE, "application/json")
}

fn get_days_range(days: u64) -> (String, String) {
    let now = Utc::now();
    let day_before_date = now.checked_sub_days(Days::new(days)).unwrap_or(now);
    (format_date(now), format_date(day_before_date))
}

fn format_date(date: DateTime<Utc>) -> String {
    let fmt = "%Y-%m-%d";
    date.format(fmt).to_string()
}
