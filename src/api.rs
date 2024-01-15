#![allow(unused_imports, non_snake_case)]

use leptos::{server, ServerFnError, logging};

use crate::time_grid::SendSlot;
use  std::fs;


#[server(Update, "/api")]
pub async fn update_db(user: String, times: Vec<SendSlot>) -> Result<String, ServerFnError> {
    let toWrite =  times.iter().map(|slot| {
        format!("ID: {}, COLOR:",slot.id)
    }).collect::<Vec<_>>();
    toWrite.iter().for_each(|data| {
        let q = fs::write("/data.txt",data);
        logging::log!("{}",q.is_ok());
    });

    Ok("".to_string())
}