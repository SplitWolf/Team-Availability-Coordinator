#![allow(unused_imports, non_snake_case)]

use leptos::{server, ServerFnError, logging};

use crate::time_grid::SendSlot;
use  std::fs;


#[server(Update, "/api")]
pub async fn update_db(user: String, times: Vec<SendSlot>) -> Result<String, ServerFnError> {
    // New data structure HashMap<Color, HashMap<Date, Timeblocks>>
    // Timeblock { start: Time, end: Time }

    dbg!(&times.clone()[230].day_colors);

    for (key, val) in times.clone()[230].day_colors.iter() {
        println!("Key: {}, Val: {val}",str::replace(key, "99","-"));
    }
    for value in times.clone() {
        for (mut key, val) in value.day_colors {
            key = str::replace(&key, "99","-");
            println!("{0} {key}, {val}",value._start_time)
        }
    }

    use sqlx::Connection;
    use sqlx::Row;
    let url = "postgres://dbuser:password@127.0.0.1:5432/avalibility";
    let mut conn = sqlx::postgres::PgConnection::connect(url).await?;

    let res = sqlx::query("SELECT 1 + 1 as sum")
    .fetch_one(&mut conn)
    .await?;
    
    let sum: i32 = res.get("sum");
    println!("{}", sum);

    Ok(format!("{}", 0))
}