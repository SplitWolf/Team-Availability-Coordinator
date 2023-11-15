use leptos::*;
use leptos_meta::*;
use leptos_router::*;

use crate::time_grid::HighlightColor;

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/team_avalibity_coordinator.css"/>

        // sets the document title
        <Title text="Welcome to Leptos"/>

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
    #[prop(default=create_signal(HighlightColor::Red).1)]
    on_change: WriteSignal<HighlightColor>
) -> impl IntoView {
    view! {
        <select id={id} on:change=move |ev| {
            logging::log!("test {}",event_target_value(&ev));
            match event_target_value(&ev).as_str() {
                "Red" => on_change.update(|color| *color = HighlightColor::Red),
                "Yellow" => on_change.update(|color| *color = HighlightColor::Yellow),
                "Green" => on_change.update(|color| *color = HighlightColor::Green),
                _ => ()
            };
        }>
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
    let players: Vec<&str> = vec!["Jordan", "Sword", "Fat Choungus Fungus", "Beeman", "Noshed", "Overrider"];
    let modes: Vec<&str> = vec!["Single", "Area"];
    let colors: Vec<&str> = vec!["Green", "Yellow","Red" ];
    let (selected_color,set_selected_color) = create_signal(HighlightColor::Green);

    view! {
        <h1 class="center"> "Team Availablity Coordinator" </h1>
        <div class="select-container">
            <SelectMenu id=id options=players />
            <SelectMenu id=id options=modes />
            <SelectMenu id=id options=colors on_change=set_selected_color />
            </div>
        <crate::calendar::Calendar color=selected_color/>
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
