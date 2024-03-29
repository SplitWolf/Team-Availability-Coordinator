use std::{fmt, cmp};
use std::collections::HashMap;
use chrono::{ DateTime, Local, NaiveDate, NaiveTime, Duration, Datelike};
use leptos::*;
use serde::{Serialize, Deserialize};
use leptos::logging::log;

#[derive(PartialEq,Clone, Copy)]
pub enum SelectionMode  {
    Single,
    AreaSelect,
    AreaDeselect
}
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Copy)]
pub enum HighlightColor {
    Red,
    Green,
    Yellow,
    Select,
    None
}
impl fmt::Display for HighlightColor {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            HighlightColor::Red =>    write!(f, "highlight-red"),
            HighlightColor::Yellow => write!(f, "highlight-yellow"),
            HighlightColor::Green =>  write!(f, "highlight-green"),
            HighlightColor::Select => write!(f, "highlight-select"),
            HighlightColor::None =>   write!(f, "")
        }
    }
}
#[derive(Clone)]
pub struct TimeSlot {
    pub id: u32,
    pub _start_time: NaiveTime,
    pub _end_time: NaiveTime,
    pub day_colors: RwSignal<HashMap<NaiveDate, HighlightColor>>,
    pub weekend: bool
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SendSlot {
    pub id: u32,
    pub _start_time: String,
    pub _end_time: String,
    #[serde(default)]
    pub day_colors: HashMap<String, HighlightColor>,
    pub weekend: bool
}




#[component]
pub fn TimeGrid(select_mode: ReadSignal<SelectionMode>, select_color: ReadSignal<HighlightColor>, curr_date: Signal<DateTime<Local>>
, submit_action: Action<Vec<TimeSlot>, Result<String, ServerFnError>> , repeat_weekly: ReadSignal<bool>
) -> impl IntoView {
    let time = vec!["07", "08", "09", "10", "11", "12", "13", "14", "15", "16", "17", "18", "19", "20", "21", "22" ];
    let (timeslots, set_timeslots) = create_signal(vec![]);
    let (select_current,set_select_current) = create_signal(0);
    let time_calc = vec!["06","07", "08", "09", "10", "11", "12", "13", "14", "15", "16", "17", "18", "19", "20", "21", "22" ];
    let weekdate = move |x: u32| {
        curr_date().date_naive().checked_add_signed(Duration::days((i64::from(x)-i64::from(curr_date().weekday().num_days_from_sunday())).into())).unwrap()
    };

    //TODO: Fix input data
    



    for i in 0..=230 {
        let index = if i >= 7 {((i+7)/7)/2} else {0} as usize;
        set_timeslots.update(|vec| {
            vec.push(TimeSlot {
                id: i,
                _start_time: NaiveTime::from_hms_opt(time_calc[index].parse::<u32>().unwrap(),if (i/7)%2==1 { 0 } else { 30 },0).expect("HARDCODED VALUE: Time Start"),
                _end_time: NaiveTime::from_hms_opt(time_calc[index].parse::<u32>().unwrap(),if (i/7)%2==1 { 0 } else { 30 },0).expect("HARDCODED VALUE: Time End"),
                day_colors: create_rw_signal(HashMap::new()),
                weekend: i % 7 == 0 || i % 7 == 6
            })});
    }

    let singleSelect = move |child_id: u32, set_color: HighlightColor| {
        timeslots.with(|slots| {
            slots.into_iter().filter(|box_div| box_div.id == child_id).for_each(|slot| {
                // logging::log!("{}",div.start_time);
                // slot.color.update(move |color| {
                //     if *color != set_color {
                //         *color = set_color;
                //     } else {
                //         *color = HighlightColor::None;
                //     }
                // });
                slot.day_colors.update(move |colors| {
                    if colors.get(&weekdate(child_id%7)).unwrap_or(&HighlightColor::None) != &set_color {
                        colors.insert(weekdate(child_id%7),set_color);
                    } else {
                        colors.insert(weekdate(child_id%7),HighlightColor::None);
                    }
                });
                
            });
        })
    };

    let areaSelect = move |child_id: u32, set_color: HighlightColor| {
        let row_begin = move || cmp::min(child_id/7,(select_current()-1)/7);
        let row_end =   move ||  cmp::max(child_id/7,(select_current()-1)/7);
        let col_begin = move ||  cmp::min(child_id%7,(select_current()-1)%7); 
        let col_end =   move ||  cmp::max(child_id%7,(select_current()-1)%7);
        timeslots.with(|slots| {
            slots.into_iter().filter(|box_div| 
                row_begin() <= box_div.id/7 && box_div.id/7 <= row_end() 
                && col_begin() <= box_div.id%7 && box_div.id%7 <= col_end() ).for_each(|slot| {
                    slot.day_colors.update(move |colors| {
                        if repeat_weekly() {
                            colors.insert(NaiveDate::from_ymd_opt(0, 1, 1).expect("HARDCODED"),set_color);
                        } 
                        colors.insert(weekdate(slot.id%7),set_color);
                    });
            });
            set_select_current.update(|value| {
                *value = 0;
            });
        })
    };

    
    //TODO: Use box divs signal to create effect and store data when sumbit btn clicked
    
    view! {
        <div class="content">
            { time.into_iter()
                .enumerate()
                .map(| (index, time) | {
                view! {
                        <div class="time" style={"grid-row:".to_owned() + &(index*2+(if index !=0 {1} else {0})).to_string()}>{ format!("{}:00", time) }</div>
                        <div class="time" style={"grid-row:".to_owned() + &((index+1)*2).to_string()}>{ format!("{}:30", time)}</div>
                }}).collect_view()
            }
            <div class="filler-box"></div>
            <div class="filler-col"></div>
            <For
                each=move || timeslots()
                key= |state| state.id.clone()
                let:child
            >
            <div 
            class="box" 
            class:weekend={ move || child.weekend}
            class:highlight-red={move || 
                (child.day_colors)().get(&weekdate(child.id%7)).unwrap_or(&HighlightColor::None) == &HighlightColor::Red ||
                (child.day_colors)().get(&NaiveDate::from_ymd_opt(0,1,1).expect("HARDCODED")).unwrap_or(&HighlightColor::None) == &HighlightColor::Red
            }
            class:highlight-yellow={move || 
                (child.day_colors)().get(&weekdate(child.id%7)).unwrap_or(&HighlightColor::None) == &HighlightColor::Yellow ||
                (child.day_colors)().get(&NaiveDate::from_ymd_opt(0,1,1).expect("HARDCODED")).unwrap_or(&HighlightColor::None) == &HighlightColor::Yellow
            }
            class:highlight-green={move || 
                (child.day_colors)().get(&weekdate(child.id%7)).unwrap_or(&HighlightColor::None) == &HighlightColor::Green ||
                (child.day_colors)().get(&NaiveDate::from_ymd_opt(0,1,1).expect("HARDCODED")).unwrap_or(&HighlightColor::None) == &HighlightColor::Green
            }
            class:highlight-select={
                move || (child.day_colors)().get(&weekdate(child.id%7)).unwrap_or(&HighlightColor::None) == &HighlightColor::Select
            }
            on:click=move |_| {
                match (move || select_mode())() {
                    SelectionMode::Single => {
                        singleSelect(child.id,select_color());
                    },
                    SelectionMode::AreaSelect => {
                        if (move || select_current() == 0)() {
                            singleSelect(child.id, HighlightColor::Select);
                            set_select_current.update(|value| {
                                *value = child.id+1;
                            });
                        } else {
                            areaSelect(child.id,select_color())
                        }
                    },
                    SelectionMode::AreaDeselect => {
                        if (move || select_current() == 0)() {
                            singleSelect(child.id, HighlightColor::Select);
                            set_select_current.update(|value| {
                                *value = child.id+1;
                            });
                        } else {
                            areaSelect(child.id,HighlightColor::None)
                        }
                    }
                }
            }
            id={child.id.to_string()} 
            />
            </For>
        </div>
        <button
        on:click=move |_| {
            submit_action.dispatch(timeslots());
            let test = submit_action.value();
            log!("{}", test().unwrap().unwrap());
        }
        >
            "Submit Data"
        </button>
    }
}