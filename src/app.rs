use leptos::{*, ev::Event};
use leptos_meta::*;
use leptos_router::*;

use crate::time_grid::{HighlightColor, SelectionMode};

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/team_avalibity_coordinator.css"/>

        // sets the document title
        <Title text="Team Availablity Coordinator"/>

        // content for this welcome page
        <Router>
            <main>
                <Routes>
                    <Route path="" view=HomePage/>
                    <Route path="/*any" view=NotFound/>
                </Routes>
            </main>
        </Router>
    }
}

// Todo Change Singal to callback
#[component]
fn SelectMenu(
    id: &'static str, 
    options: Vec<&'static str>, 
    #[prop(into)] 
    on_change: Callback<Event>
) -> impl IntoView {
    view! {
        <select id={id} on:change=on_change>
        {
            options.into_iter().map(|name| {
                view! {
                    <option value={name}>
                       { name }
                    </option>
                }

            }).collect_view()
        }
        </select>
    }
}

/// Renders the home page of your application.
#[component]
fn HomePage() -> impl IntoView {
    // Creates a reactive value to update the button
    let id = "playerName";
    let players: Vec<&str> = vec!["Zero", "Jordan", "Sword", "Fat Choungus Fungus", "Beeman", "Noshed", "Overrider"];
    let modes: Vec<&str> = vec!["Single", "Area Select", "Area Deselect"];
    let colors: Vec<&str> = vec!["Green", "Yellow","Red" ];
    let repeating: Vec<&str> = vec!["Only this week", "Repeat Weekely"];
    let (selected_color,set_selected_color) = create_signal(HighlightColor::Green);
    let (select_mode, set_select_mode) = create_signal(SelectionMode::Single);
    let (repeat_weekly, set_repeat_weekly) = create_signal(false);

    view! {
        <h1 class="center"> "Team Availablity Coordinator" </h1>
        <div class="select-container">
            <SelectMenu id=id options=players on_change=move |_| {} />
            <SelectMenu id=id options=modes on_change=move |ev| {
                match event_target_value(&ev).as_str() {
                  "Single" => set_select_mode.update(|mode| *mode=SelectionMode::Single),
                  "Area Select" =>  set_select_mode.update(|mode| *mode=SelectionMode::AreaSelect),
                  "Area Deselect" =>  set_select_mode.update(|mode| *mode=SelectionMode::AreaDeselect),
                  _ => ()
                };
            }/>
            //TODO: Fix Ids
            <SelectMenu id=id options=colors on_change=move |ev| {
               // logging::log!("test {}",event_target_value(&ev));
                match event_target_value(&ev).as_str() {
                    "Red" => set_selected_color.update(|color| *color = HighlightColor::Red),
                    "Yellow" => set_selected_color.update(|color| *color = HighlightColor::Yellow),
                    "Green" => set_selected_color.update(|color| *color = HighlightColor::Green),
                    _ => ()
                };
            } />
            </div>
            <SelectMenu id=id options=repeating on_change=move |ev| {
                match event_target_value(&ev).as_str() {
                    "Only this week" => set_repeat_weekly.update(|val| *val = false),
                    "Repeat Weekely" => set_repeat_weekly.update(|val| *val = true),
                    _ => ()
                };
            }/>
        <crate::calendar::Calendar color=selected_color mode=select_mode repeat_weekly/>
    }
}

/// 404 - Not Found
#[component]
fn NotFound() -> impl IntoView {
    // set an HTTP status code 404
    // this is feature gated because it can only be done during
    // initial server-side rendering
    // if you navigate to the 404 page subsequently, the status
    // code will not be set because there is not a new HTTP request
    // to the server
    #[cfg(feature = "ssr")]
    {
        // this can be done inline because it's synchronous
        // if it were async, we'd use a server function
        let resp = expect_context::<leptos_actix::ResponseOptions>();
        resp.set_status(actix_web::http::StatusCode::NOT_FOUND);
    }

    view! {
        <h1>"Not Found"</h1>
    }
}
