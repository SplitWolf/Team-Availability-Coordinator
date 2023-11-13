use yew::prelude::*;
use chrono::Datelike;
use chrono::Days as Day;
use std::fmt;
use std::cmp;

#[derive(Properties, PartialEq)]
pub struct CalendarProps {
   pub weekOffset: i32,
   pub on_click: Callback<i32>
}
#[derive(Properties, PartialEq)]
struct DayNumbers {
    numbers_from_sun: Vec<u32>
}
#[derive(Properties, PartialEq)]
struct TimeGridProps {
    SelectMode: Mode,
    Color: HighlightColor
}
#[derive(PartialEq,Clone, Copy)]
enum Mode  {
    Single,
    Area
}
#[derive(PartialEq)]
enum HighlightColor {
    Red,
    Green,
    Yellow,
    None
}

impl fmt::Display for HighlightColor {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            HighlightColor::Red => write!(f, "red"),
            HighlightColor::Yellow => write!(f, "yellow"),
            HighlightColor::Green => write!(f, "green"),
            HighlightColor::None => write!(f, "")
        }
    }
}

#[function_component]
fn TimeGrid(TimeGridProps { SelectMode, Color }: &TimeGridProps) -> Html {
    let time = vec!["07", "08", "09", "10", "11", "12", "13", "14", "15", "16", "17", "18", "19", "20", "21", "22" ];
    let highlightStateRed: UseStateHandle<Vec<u32>> = use_state(|| vec![]);
    let highlightStateYellow: UseStateHandle<Vec<u32>> = use_state(|| vec![]);
    let highlightStateGreen: UseStateHandle<Vec<u32>> = use_state(|| vec![]);
    let selectState = use_state(|| 0);
    let selectMode = SelectMode;
    let color = Color;

    let n: Vec<u32> = (0..231).collect();
    html! {
        <>
            <div class={classes!("content")}>
                { time.into_iter()
                    .enumerate()
                    .map(| (index, time) | {
                    html! {
                        <>
                            <div class={classes!("time")} style={"grid-row:".to_owned() + &(index*2+(if index !=0 {1} else {0})).to_string()}>{ format!("{}:00", time) }</div>
                            <div class={classes!("time")} style={"grid-row:".to_owned() + &((index+1)*2).to_string()}>{ format!("{}:30", time)}</div>
                        </>
                    }}).collect::<Html>()
                }
                <div class={classes!("filler-box")}></div>
                <div class={classes!("filler-col")}></div>
                {
                    n.into_iter()
                    .map(| i | {
                        let on_click = {
                            let state = if color == &HighlightColor::Red {
                                highlightStateRed.clone()
                            } else if color == &HighlightColor::Yellow {
                                highlightStateYellow.clone()
                            } else if color == &HighlightColor::Green {
                                highlightStateGreen.clone()
                            } else {
                                highlightStateGreen.clone()
                            };
                            let mode = selectMode.clone();
                            let firstSelect = selectState.clone();

                            Callback::from(move | toAdd: u32| { 
                                let mut newSate = state.to_vec().clone();
                                if mode == Mode::Area {
                                    if *firstSelect == 0 {
                                        firstSelect.set(toAdd+1);
                                       newSate.push(toAdd);
                                    } else {
                                        let selectRangeCol: u32 = (cmp::max(toAdd%7,(*firstSelect-1)%7))-(cmp::min(toAdd%7,(*firstSelect-1)%7));
                                        let selectRangeRow: u32 = (cmp::max(toAdd/7,(*firstSelect-1)/7))-(cmp::min(toAdd/7,(*firstSelect-1)/7));
                                        let mut toAddVec: Vec<u32> = vec![];
                                        let mut finalToAdd: Vec<u32> = vec![];
                                        let initBox = cmp::min(toAdd/7,(*firstSelect-1)/7)*7+cmp::min(toAdd%7,(*firstSelect-1)%7);
                                        for n in 0..selectRangeCol+1 {
                                            toAddVec.push(initBox+n);
                                            for n in 0..selectRangeRow+1 {
                                                toAddVec.iter().for_each(| value | {
                                                    finalToAdd.push(value+(7*n));
                                                });
                                            }
                                        }
                                        newSate.append(&mut finalToAdd);
                                        firstSelect.set(0);
                                    }
                                } else {
                                    if newSate.contains(&toAdd) {
                                        newSate.remove(newSate.iter().position(|&r| r == toAdd).unwrap());
                                    } else {
                                        newSate.push(toAdd);
                                    }
                                }
                                state.set(newSate);
                            })
                        };
                        if highlightStateRed.contains(&i) {
                            colorBoxDiv(HighlightColor::Red, i % 7 == 0 || i % 7 == 6, i, on_click)
                        } else if highlightStateYellow.contains(&i) {
                            colorBoxDiv(HighlightColor::Yellow, i % 7 == 0 || i % 7 == 6, i, on_click)
                        } else if highlightStateGreen.contains(&i) {
                            colorBoxDiv(HighlightColor::Green, i % 7 == 0 || i % 7 == 6, i, on_click)
                        } else {
                            colorBoxDiv(HighlightColor::None, i % 7 == 0 || i % 7 == 6, i, on_click)
                        }
                    }).collect::<Html>()
                }
            </div>
        </>
    }
}

fn colorBoxDiv(color: HighlightColor, weekend: bool, id: u32, on_click: Callback<u32>) -> Html {
    return if weekend {
        html! {
            <div class={classes!("box","weekend",format!("highlight-{}",color))} id={id.to_string()} onclick={move |_| on_click.emit(id)}/>
        }
    } else {
        html! {
            <div class={classes!("box",format!("highlight-{}",color))} id={id.to_string()} onclick={move |_| on_click.emit(id)}/>
        }
    }
}

#[function_component]
fn Days(DayNumbers{ numbers_from_sun }: &DayNumbers) -> Html {
    html! {
            <div class={classes!("days")}>
                <div class={classes!("filler")}></div>
                <div class={classes!("filler")}></div>
                <div class={classes!("day")}> { format!("Sun {}",numbers_from_sun[0].to_string()) } </div>
                <div class={classes!("day")}> { format!("Mon {}",numbers_from_sun[1].to_string()) } </div>
                <div class={classes!("day")}> { format!("Tue {}",numbers_from_sun[2].to_string()) } </div>
                <div class={classes!("day")}> { format!("Wed {}",numbers_from_sun[3].to_string()) } </div>
                <div class={classes!("day")}> { format!("Thu {}",numbers_from_sun[4].to_string()) } </div>
                <div class={classes!("day")}> { format!("Fri {}",numbers_from_sun[5].to_string()) } </div>
                <div class={classes!("day")}> { format!("Sat {}",numbers_from_sun[6].to_string()) } </div>
            </div>
    }
}


#[function_component]
pub fn Calendar(CalendarProps { weekOffset, on_click }: &CalendarProps) -> Html {
    let date = if weekOffset > &0 {
        chrono::offset::Local::now().checked_add_days(Day::new(((weekOffset*7)).try_into().unwrap())).expect("Should not be outside of date range")
    } else {
        chrono::offset::Local::now().checked_sub_days(Day::new(((-weekOffset*7)).try_into().unwrap())).expect("Should not be outside of date range")
    };
    let dayOfWeekCol = date.weekday().num_days_from_sunday();
    let mut numss: Vec<u32> = vec![];
    let mut currentDay = date.checked_sub_days(Day::new(dayOfWeekCol.into())).expect("Should not be outside of date range");
    let firstDay = currentDay;
    let mut lastDay = currentDay;
    for i in 0..7 {
        if i == 6 {
            lastDay = currentDay;
        }
        numss.push(currentDay.day());
        currentDay = currentDay.checked_add_days(Day::new(1)).expect("Should not be outside of date range");
    };

    let title = if firstDay.month() == lastDay.month() {
        format!("{} Sun {} - Sat {}, {}",firstDay.format("%B"),numss[0],numss[6],firstDay.format("%Y"))
    } else {
        format!("{} Sun {}, {} - {} Sat {}, {}",firstDay.format("%b"),numss[0],firstDay.format("%Y"),lastDay.format("%b"),numss[6],lastDay.format("%Y"))
    };
    let on_left_clicked = {
        let on_click = on_click.clone();
        let newOffset = weekOffset.clone() - 1;
        Callback::from(move |_| {
            on_click.emit(newOffset)
        })
    };
    let on_right_clicked = {
        let on_click = on_click.clone();
        let newOffset = weekOffset.clone() + 1;
        Callback::from(move |_| {
            on_click.emit(newOffset)
        })
    };

    html! {
        <>
            <div class="container">
                <div class="titlebar">
                <button class="direction" key="left" onclick={on_left_clicked.clone()}>{ "<" }</button>
                <div id="title"> { title } </div>
                <button class="direction" key="right" onclick={on_right_clicked.clone()}>{ ">" }</button>
                </div>
                <Days numbers_from_sun={numss}/>
                <TimeGrid SelectMode={Mode::Area} Color={HighlightColor::Yellow} />
            </div>
            <p> {  date.weekday().num_days_from_sunday() } </p>
            <p> { date.date_naive().weekday().to_string() } </p>
        </>
    }
}
