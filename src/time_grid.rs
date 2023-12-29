use chrono::DateTime;
use chrono::Local;
use chrono::NaiveTime;
use leptos::*;
use std::fmt;
use std::cmp;

#[derive(PartialEq,Clone, Copy)]
pub enum SelectionMode  {
    Single,
    AreaSelect,
    AreaDeselect
}
#[derive(PartialEq, Clone, Copy)]
pub enum HighlightColor {
    Red,
    Green,
    Yellow,
    None
}
impl fmt::Display for HighlightColor {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            HighlightColor::Red => write!(f, "highlight-red"),
            HighlightColor::Yellow => write!(f, "highlight-yellow"),
            HighlightColor::Green => write!(f, "highlight-green"),
            HighlightColor::None => write!(f, "")
        }
    }
}
#[derive(Clone)]
struct TimeSlot {
    id: u32,
    start_time: NaiveTime,
    end_time: NaiveTime,
    dayColors: Vec<DayColor>,
    color: RwSignal<HighlightColor>,
    weekend: bool
}

#[derive(Clone)]
struct DayColor {
    date: DateTime<Local>,
    color: HighlightColor
}

#[component]
pub fn TimeGrid(select_mode: ReadSignal<SelectionMode>, select_color: ReadSignal<HighlightColor>, curr_date: Signal<DateTime<Local>>) -> impl IntoView {
    let time = vec!["07", "08", "09", "10", "11", "12", "13", "14", "15", "16", "17", "18", "19", "20", "21", "22" ];
    let (timeslots, set_timeslots) = create_signal(vec![]);
    let (select_current,set_select_current) = create_signal(0);
    let time_calc = vec!["06","07", "08", "09", "10", "11", "12", "13", "14", "15", "16", "17", "18", "19", "20", "21", "22" ];

    for i in 0..=230 {
        let index = if i >= 7 {((i+7)/7)/2} else {0} as usize;
        set_timeslots.update(|vec| {
            vec.push(TimeSlot {
                id: i,
                start_time: NaiveTime::from_hms_opt(time_calc[index].parse::<u32>().unwrap(),if (i/7)%2==1 { 0 } else { 30 },0).expect("HARDCODED VALUE: Time Start"),
                end_time: NaiveTime::from_hms_opt(time_calc[index].parse::<u32>().unwrap(),if (i/7)%2==1 { 0 } else { 30 },0).expect("HARDCODED VALUE: Time End"),
                dayColors: vec![],
                color: create_rw_signal(HighlightColor::None),
                weekend: i % 7 == 0 || i % 7 == 6
            })});
    }

    let singleSelect = move |child_id: u32, set_color: HighlightColor| {
        timeslots.with(|slots| {
            slots.into_iter().filter(|box_div| box_div.id == child_id).for_each(|div| {
                // logging::log!("{}",div.start_time);
                div.color.update(move |color| {
                    if *color != set_color {
                        *color = set_color;
                    } else {
                        *color = HighlightColor::None;
                    }
                });
            });
        })
    };
    
    let areaSelect = move |child_id: u32, set_color: HighlightColor| {
        let row_begin =move || cmp::min(child_id/7,(select_current()-1)/7);
        let row_end =move ||  cmp::max(child_id/7,(select_current()-1)/7);
        let col_begin =move ||  cmp::min(child_id%7,(select_current()-1)%7); 
        let col_end =move ||  cmp::max(child_id%7,(select_current()-1)%7);
        timeslots.with(|slots| {
            slots.into_iter().filter(|box_div| 
                row_begin() <= box_div.id/7 && box_div.id/7 <= row_end() 
                && col_begin() <= box_div.id%7 && box_div.id%7 <= col_end() ).for_each(|div| {
                div.color.update(|color| {
                    *color = set_color;
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
            class:highlight-red={move || child.color.get() == HighlightColor::Red} 
            class:highlight-yellow={move || child.color.get() == HighlightColor::Yellow} 
            class:highlight-green={move || child.color.get() == HighlightColor::Green} 
            on:click=move |_| {
                match (move || select_mode())() {
                    SelectionMode::Single => {
                        singleSelect(child.id,select_color());
                    },
                    SelectionMode::AreaSelect => {
                        if (move || select_current() == 0)() {
                            //TODO: Replace with custom highlight color single select
                            timeslots.with(|boxes| { 
                                boxes.into_iter().filter(|box_div| box_div.id == child.id).for_each(|div| {
                                    div.color.update(|color| {
                                        //TODO: Update how corner box is colored
                                        if *color == HighlightColor::None {
                                            *color = select_color();
                                        } else {
                                            *color = HighlightColor::None;
                                        }
                                    });
                                });
                            });
                            set_select_current.update(|value| {
                                *value = child.id+1;
                            });
                        } else {
                            areaSelect(child.id,select_color())
                        }
                    },
                    SelectionMode::AreaDeselect => {
                        if (move || select_current() == 0)() {
                            //TODO: Replace with custom highlight color single select
                            timeslots.with(|boxes| {
                                boxes.into_iter().filter(|box_div| box_div.id == child.id).for_each(|div| {
                                    div.color.update(|color| {
                                        *color = select_color()
                                    });
                                });
                            });
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
        <button>
            "Submit Data"
        </button>
    }
}