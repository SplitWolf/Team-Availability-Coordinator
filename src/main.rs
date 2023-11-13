#![allow(non_snake_case)]

mod calendar;

use yew::prelude::*;
use calendar::Calendar;

#[derive(Properties, PartialEq)]
struct SelectMenuProps {
    id: String,
    options: Vec<String>,
}

#[function_component]
fn SelectMenu(SelectMenuProps { id, options}: &SelectMenuProps ) -> Html {
    html! {
        <select id={id.clone()}>
        {
            options.into_iter().map(|name| {
                html!{ 
                    <option>
                        {name}
                    </option>
                }
            }).collect::<Html>()
        }
        </select>
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
    let modes: Vec<String> = vec!["Single", "Area"].into_iter().map(String::from).collect();
    let colors: Vec<String> = vec!["Green", "Yellow","Red" ].into_iter().map(String::from).collect();

    html! {
        <body>
            <h1 class={classes!("center")}> { "Team Availablity Coordinator" }</h1>

            <div class={classes!("select-container")}>
                <SelectMenu id="playerName" options={players} />
                <SelectMenu id="playerName" options={modes} />
                <SelectMenu id="playerName" options={colors} />
            </div>
            <Calendar weekOffset={*offset} {on_click}/>
            <p>{ *offset }</p>
        </body>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}