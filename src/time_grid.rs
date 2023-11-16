use leptos::*;
use std::fmt;
use std::cmp;

#[derive(PartialEq,Clone, Copy)]
pub enum Mode  {
    Single,
    AreaSelect,
    AreaDeselect
}
#[derive(PartialEq, Clone)]
pub enum HighlightColor {
    Red,
    Green,
    Yellow,
    None
}
impl fmt::Display for HighlightColor {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            HighlightColor::Red => write!(f, "box highlight-red"),
            HighlightColor::Yellow => write!(f, "box highlight-yellow"),
            HighlightColor::Green => write!(f, "box highlight-green"),
            HighlightColor::None => write!(f, "")
        }
    }
}
#[derive(Clone)]
struct BoxDiv {
    id: u32,
    color: RwSignal<HighlightColor>,
    weekend: bool
}

#[component]
pub fn TimeGrid(select_mode: ReadSignal<Mode>, select_color: ReadSignal<HighlightColor>) -> impl IntoView {
    let time = vec!["07", "08", "09", "10", "11", "12", "13", "14", "15", "16", "17", "18", "19", "20", "21", "22" ];
    let (box_divs, set_box_divs) = create_signal(vec![BoxDiv{id: 0, color: create_rw_signal(HighlightColor::None),weekend: false};0]);
    let (select_current,set_select_current) = create_signal(0);

    for i in 0..231 {
        if i % 7 == 0 || i % 7 == 6 {
            set_box_divs.update(|vec| {
                vec.push(BoxDiv {
                    id: i,
                    color: create_rw_signal(HighlightColor::None),
                    weekend: true
                })});
        } else {
            set_box_divs.update(|vec| {
                vec.push(BoxDiv {
                    id: i,
                    color: create_rw_signal(HighlightColor::None),
                    weekend: false
                })});
        }
    }

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
                each=box_divs
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
                box_divs.with(|boxes| {
                    match (move || select_mode())() {
                        Mode::Single => {
                            boxes.into_iter().filter(|box_div| box_div.id == child.id).for_each(|div| {
                                div.color.update(|color| {
                                    if *color == HighlightColor::None {
                                        *color = select_color();
                                    } else {
                                        *color = HighlightColor::None;
                                    }
                                });
                            });
                        },
                        Mode::AreaSelect => {
                            if (move || select_current() == 0)() {
                                boxes.into_iter().filter(|box_div| box_div.id == child.id).for_each(|div| {
                                    div.color.update(|color| {
                                        if *color == HighlightColor::None {
                                            *color = select_color();
                                        } else {
                                            *color = HighlightColor::None;
                                        }
                                    });
                                });
                                set_select_current.update(|value| {
                                    *value = child.id+1;
                                });
                            } else {
                                // Replace with create_effect?
                                let row_begin =move || cmp::min(child.id/7,(select_current()-1)/7);
                                let row_end =move ||  cmp::max(child.id/7,(select_current()-1)/7);
                                let col_begin =move ||  cmp::min(child.id%7,(select_current()-1)%7); 
                                let col_end =move ||  cmp::max(child.id%7,(select_current()-1)%7);
                                boxes.into_iter().filter(|box_div| 
                                    row_begin() <= box_div.id/7 && box_div.id/7 <= row_end() 
                                    && col_begin() <= box_div.id%7 && box_div.id%7 <= col_end() ).for_each(|div| {
                                    div.color.update(|color| {
                                        *color = select_color();
                                    });
                                });
                                set_select_current.update(|value| {
                                    *value = 0;
                                });
                            }
                        },
                        Mode::AreaDeselect => {
                            if (move || select_current() == 0)() {
                                boxes.into_iter().filter(|box_div| box_div.id == child.id).for_each(|div| {
                                    div.color.update(|color| {
                                        *color = select_color()
                                    });
                                });
                                set_select_current.update(|value| {
                                    *value = child.id+1;
                                });
                            } else {
                                // Replace with create_effect?
                                let row_begin =move || cmp::min(child.id/7,(select_current()-1)/7);
                                let row_end =move ||  cmp::max(child.id/7,(select_current()-1)/7);
                                let col_begin =move ||  cmp::min(child.id%7,(select_current()-1)%7); 
                                let col_end =move ||  cmp::max(child.id%7,(select_current()-1)%7);
                                boxes.into_iter().filter(|box_div| 
                                    row_begin() <= box_div.id/7 && box_div.id/7 <= row_end() 
                                    && col_begin() <= box_div.id%7 && box_div.id%7 <= col_end() ).for_each(|div| {
                                    div.color.update(|color| {
                                        *color = HighlightColor::None;
                                    });
                                });
                                set_select_current.update(|value| {
                                    *value = 0;
                                });
                            }
                        }
                    }

                })
            }
            id={child.id.to_string()} 
            />
            </For>
        </div>
    }
}
//Modify Code

// for n in 0..231 {
//     let mut colorVec = if color == HighlightColor::Red {
//         highlightStateRed.clone()
//     } else if color == HighlightColor::Yellow {
//         highlightStateYellow.clone()
//     } else if color == HighlightColor::Green {
//         highlightStateGreen.clone()
//     } else {
//         highlightStateGreen.clone()
//     };

//     match select_mode {
//         Mode::Area => {
//             if select_current == 0 {
//                 //TODO: Define state callback or smth
//                 select_current = toAdd+1;
//             } else {
//                 //TODO: Make this a function and redo the alg
//                 let selectRangeCol: u32 = (cmp::max(toAdd%7,(select_current-1)%7))-(cmp::min(toAdd%7,(select_current-1)%7));
//                 let selectRangeRow: u32 = (cmp::max(toAdd/7,(select_current-1)/7))-(cmp::min(toAdd/7,(select_current-1)/7));
//                 let mut toAddVec: Vec<u32> = vec![];
//                 let mut finalToAdd: Vec<u32> = vec![];
//                 let initBox = cmp::min(toAdd/7,(select_current-1)/7)*7+cmp::min(toAdd%7,(select_current-1)%7);
//                 for n in 0..selectRangeCol+1 {
//                     toAddVec.push(initBox+n);
//                     for n in 0..selectRangeRow+1 {
//                         toAddVec.iter().for_each(| value | {
//                             finalToAdd.push(value+(7*n));
//                         });
//                     }
//                 }
//             }
//         },
//         Mode::Single => {
//             if colorVec.contains(&toAdd) {
//                 colorVec.remove(colorVec.iter().position(|&r| r == toAdd).unwrap());
//             } else {
//                 colorVec.push(toAdd);
//             }
//         }
//     }
// }