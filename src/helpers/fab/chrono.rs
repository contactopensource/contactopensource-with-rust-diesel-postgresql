use ::rand::Rng;

pub fn now_as_naive_utc() -> chrono::NaiveDateTime {
    chrono::Utc::now().naive_utc()
}

pub fn today_as_naive_utc() -> chrono::NaiveDate {
    chrono::Utc::today().naive_utc()
}

pub fn yesterday_as_naive_utc() -> chrono::NaiveDate {
    chrono::Utc::today().naive_utc() - chrono::Duration::days(1)
}

pub fn tomorrow_as_naive_utc() -> chrono::NaiveDate {
    chrono::Utc::today().naive_utc() + chrono::Duration::days(1)
}

pub fn date_in_past_as_naive_utc() -> chrono::NaiveDate {
    chrono::Utc::today().naive_utc() - chrono::Duration::days(rand::thread_rng().gen_range(1, 99))
}

pub fn date_in_future_as_naive_utc() -> chrono::NaiveDate {
    chrono::Utc::today().naive_utc() + chrono::Duration::days(rand::thread_rng().gen_range(1, 99))
}

pub fn timestamp_utc() -> chrono::NaiveDateTime {
    chrono::Utc::now().naive_utc()
}

pub fn clock_counter() -> i64 {
    rand::thread_rng().gen_range(100000000000, 900000000000)
}
