#![allow(non_snake_case)]

mod calendar;

use chrono::offset;
use yew::prelude::*;
use calendar::Calendar;

#[derive(Properties, PartialEq)]
struct Players {
    players: Vec<String>
}

#[function_component]
fn PlayerSelect(Players { players }: &Players) -> Html {
    html! {
        <div class={classes!("select-container")}>
            <select id="playerName">
            {
                players.into_iter().map(|name| {
                    html!{ 
                        <option>
                            {name}
                        </option>
                    }
                }).collect::<Html>()
            }
            </select>
        </div>
    }
}

#[function_component]
fn App() -> Html {
    let offset = use_state(|| 0);
    let on_click = {
        let offset = offset.clone();
        move | newOffset: i32| {
            offset.set(newOffset);
        }
    };
    let players: Vec<String> = vec!["Jordan", "Sword", "Fat Choungus Fungus", "Beeman", "Noshed", "Overrider"].into_iter().map(String::from).collect();
    html! {
        <body>
            <h1 class={classes!("center")}> { "Team Availablity Coordinator" }</h1>

            <PlayerSelect players={players} />
            <Calendar weekOffset={*offset} {on_click}/>
            <p>{ *offset }</p>
        </body>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}