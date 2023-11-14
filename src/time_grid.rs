use leptos::*;
use std::fmt;
use std::cmp;

#[derive(PartialEq,Clone, Copy)]
pub enum Mode  {
    Single,
    Area
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
            HighlightColor::Red => write!(f, "red"),
            HighlightColor::Yellow => write!(f, "yellow"),
            HighlightColor::Green => write!(f, "green"),
            HighlightColor::None => write!(f, "")
        }
    }
}

#[component]
pub fn TimeGrid(select_mode: ReadSignal<Mode>, select_color: ReadSignal<HighlightColor>) -> impl IntoView {
    let time = vec!["07", "08", "09", "10", "11", "12", "13", "14", "15", "16", "17", "18", "19", "20", "21", "22" ];
    let highlightStateRed: Vec<u32> = vec![];
    let highlightStateYellow: Vec<u32> = vec![];
    let highlightStateGreen: Vec<u32> = vec![];
    let (select_current,set_select_current) = create_signal(0);
    let select_mode = select_mode.get();
    let color = select_color.get();
    let toAdd = 0;

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
            {
                let n: Vec<u32> = (0..231).collect();
                n.into_iter()
                    .map(| i | {
                        if highlightStateRed.contains(&i) {
                            colorBoxDiv(HighlightColor::Red, i % 7 == 0 || i % 7 == 6, i)
                        } else if highlightStateYellow.contains(&i) {
                            colorBoxDiv(HighlightColor::Yellow, i % 7 == 0 || i % 7 == 6, i)
                        } else if highlightStateGreen.contains(&i) {
                            colorBoxDiv(HighlightColor::Green, i % 7 == 0 || i % 7 == 6, i)
                        } else {
                            colorBoxDiv(HighlightColor::None, i % 7 == 0 || i % 7 == 6, i)
                        }
                    }).collect_view()
            }
        </div>
    }
}
#[allow(non_snake_case)]
fn colorBoxDiv(color: HighlightColor, weekend: bool, id: u32) -> impl IntoView {
    return if weekend {
        view! {
            <div class=format!("box weekend highlight-{}",color) id={id.to_string()}/>
        }
    } else {
        view! {
            <div class=format!("box highlight-{}",color) id={id.to_string()}/>
        }
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