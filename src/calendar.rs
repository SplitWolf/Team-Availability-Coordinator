#![allow(non_snake_case)]
use chrono::Datelike;
use leptos::*;
use chrono::Days as Day;

use crate::time_grid;


#[component]
pub fn Days(numbers_from_sun: ReadSignal<Vec<u32>>) -> impl IntoView {
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
    let (weekOffset, set_weekOffset) = create_signal(0);
    let reverseOffset = move || weekOffset() >= 0;
    let offset = move || weekOffset();

    // let (select_mode, set_select_mode) = create_signal(time_grid::Mode::Area);
    //let (select_color, set_select_color) = create_signal(time_grid::HighlightColor::Green);
    let (numbers_from_sun, set_nums_from_sun) = create_signal(vec![0;7]);
    let title = move || {
        let date = if reverseOffset() {
            chrono::offset::Local::now().checked_add_days(Day::new(((offset()*7)).try_into().unwrap())).expect("Should not be outside of date range")
        } else {
            chrono::offset::Local::now().checked_sub_days(Day::new(((-(offset()*7))).try_into().unwrap())).expect("Should not be outside of date range")
        };
        let dayOfWeekCol = date.weekday().num_days_from_sunday();
        let mut currentDay = date.checked_sub_days(Day::new(dayOfWeekCol.into())).expect("Should not be outside of date range");
        let firstDay = currentDay;
        let mut lastDay = currentDay;
        for i in 0..=6 {
    
            if i == 6 {
                lastDay = currentDay;
            }
            set_nums_from_sun.update(move |nums| {
                nums[i] = currentDay.day();
            });
            currentDay = currentDay.checked_add_days(Day::new(1)).expect("Should not be outside of date range");
        };
        if firstDay.month() == lastDay.month() {
            format!("{} Sun {} - Sat {}, {}",firstDay.format("%B"),numbers_from_sun.get_untracked()[0],numbers_from_sun.get_untracked()[6],firstDay.format("%Y"))
        } else {
            format!("{} Sun {}, {} - {} Sat {}, {}",firstDay.format("%b"),numbers_from_sun.get_untracked()[0],firstDay.format("%Y"),lastDay.format("%b"),numbers_from_sun.get_untracked()[6],lastDay.format("%Y"))
        }
    };
    


    view! {
        
        <div class="container">
            <div class="titlebar">
            <button class="direction" key="left" on:click=move |_| { set_weekOffset.update(|n| *n-=1); }>"<"</button>
            <div id="title"> { title } </div>
            <button class="direction" key="right" on:click=move |_| { set_weekOffset.update(|n| *n+=1); }>">"</button>
           // <button class="direction" key="right" on:click=move |_| { set_select_color.update(|n| *n=time_grid::HighlightColor::Red); }>">"</button>
            </div>
           <Days numbers_from_sun/>
        //    <br/>
           <time_grid::TimeGrid select_mode=mode select_color=color/>
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