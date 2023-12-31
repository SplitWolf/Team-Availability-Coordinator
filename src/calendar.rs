#![allow(non_snake_case)]
use chrono::{Datelike, Duration};
use leptos::*;
use chrono::Days as Day;

use crate::time_grid;


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
pub fn Calendar(color: ReadSignal<time_grid::HighlightColor>, mode: ReadSignal<time_grid::SelectionMode>) -> impl IntoView {
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


    view! {
        
        <div class="container">
            <div class="titlebar">
            <button class="direction" key="left" on:click=move |_| { set_weekOffset.update(|n| *n-=1); }>"<"</button>
            <div id="title"> { title } </div>
            <button class="direction" key="right" on:click=move |_| { set_weekOffset.update(|n| *n+=1); }>">"</button>
            </div>
           <Days numbers_from_sun/>
        //    <br/>
           <time_grid::TimeGrid select_mode=mode select_color=color curr_date=date/>
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