#![allow(non_snake_case)]
use std::collections::HashMap;
use leptos::logging::log;
use leptos::*;
use chrono::{Datelike, Duration, Days as Day};
// Submit Action Imports
use crate::time_grid::{self, HighlightColor, TimeSlot, SendSlot};
use crate::api::update_db;

#[component]
pub fn Days(numbers_from_sun: Signal<Vec<u32>>) -> impl IntoView {
    view! {
            <div class={"days"}>
                    <div class={"filler"}></div>
                    <div class={"filler"}></div>
                    <div class={"day"}> "Sun "{ move || numbers_from_sun()[0] } </div>
                    <div class={"day"}> "Mon "{ move || numbers_from_sun()[1] } </div>
                    <div class={"day"}> "Tue "{ move || numbers_from_sun()[2] } </div>
                    <div class={"day"}> "Wed "{ move || numbers_from_sun()[3] } </div>
                    <div class={"day"}> "Thu "{ move || numbers_from_sun()[4] } </div>
                    <div class={"day"}> "Fri "{ move || numbers_from_sun()[5] } </div>
                    <div class={"day"}> "Sat "{ move || numbers_from_sun()[6] } </div>
                </div>
    }
}

#[component]
pub fn Calendar(color: ReadSignal<time_grid::HighlightColor>, mode: ReadSignal<time_grid::SelectionMode>, repeat_weekly: ReadSignal<bool>) -> impl IntoView {
    // Get the date reference
    // Temp Weekoffset signinal if
    let (weekOffset, set_weekOffset) = create_signal(0);
    let date = Signal::derive(move || chrono::offset::Local::now().checked_add_signed(Duration::days(weekOffset()*7)).unwrap());
    let numbers_from_sun = Signal::derive(move || {
        let mut nums = vec![0;7];
        let curr_date = date();
        let dayOfWeekCol = curr_date.weekday().num_days_from_sunday();
        let mut iter_date = curr_date.checked_sub_days(Day::new(dayOfWeekCol.into())).expect("Should not be outside of date range");
        for i in 0..=6 {
            nums[i] = iter_date.day();
            iter_date = iter_date.checked_add_days(Day::new(1)).expect("Should not be outside of date range");
        };
        nums
    });

    let title = move || { 
        let firstDay = date().checked_sub_days(Day::new(date().weekday().num_days_from_sunday().into())).expect(" ");
        let lastDay = firstDay.checked_add_days(Day::new(7)).expect(" ");

        if firstDay.month() == lastDay.month() {
            format!("{} Sun {} - Sat {}, {}",firstDay.format("%B"),
            numbers_from_sun()[0],
            numbers_from_sun()[6],firstDay.format("%Y"))
        } else {
            format!("{} Sun {}, {} - {} Sat {}, {}",firstDay.format("%b"),
            numbers_from_sun()[0],firstDay.format("%Y"),lastDay.format("%b"),
            numbers_from_sun()[6],lastDay.format("%Y"))
        }
    };    

    let submit_action = create_action(move |input: &Vec<TimeSlot>| {
//str::replace(&date.to_string(),"-",":")
        let toSend = input.clone().iter().map(move |data: &TimeSlot| {
            log!("{:?}",dbg!(data.day_colors.get().iter().map(move |(date, color)|{
                (str::replace(&date.to_string(),"-",""), *color)
            }).collect::<HashMap<String,HighlightColor>>()));
            SendSlot { 
                id: data.id, 
                _start_time: data._start_time.to_string(), 
                _end_time: data._end_time.to_string(), 
                //TODO: Maybe make this into a vec with tuples?
                day_colors: data.day_colors.get().iter().map(move |(date, color)|{
                    (str::replace(&date.to_string(),"-","99"), *color)
                }).collect::<HashMap<String,HighlightColor>>(), 
                weekend: data.weekend
            }
        }).collect::<Vec<_>>();

        update_db("Zero".to_string(),toSend)
    });


    view! {
        
        <div class="container">
            <div class="titlebar">
            <button class="direction" key="left" on:click=move |_| { set_weekOffset.update(|n| *n-=1); }>"<"</button>
            <div id="title"> { title } </div>
            <button class="direction" key="right" on:click=move |_| { set_weekOffset.update(|n| *n+=1); }>">"</button>
            </div>
           <Days numbers_from_sun/>
        //    <br/>
           <time_grid::TimeGrid select_mode=mode select_color=color curr_date=date submit_action repeat_weekly/>
        //    { weekOffset }
        //    <br/>
        //    { reverseOffset }
        //    <br/>
        //    { offset }
        //    <br/>
        //    { numbers_from_sun }
        //    <br/>
        </div>
        //<p> {  date.weekday().num_days_from_sunday() } </p>
        //<p> { date.date_naive().weekday().to_string() } </p>
    }
}