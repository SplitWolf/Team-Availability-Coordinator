use leptos::*;
use leptos_meta::*;
use leptos_router::*;

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

#[component]
fn SelectMenu(id: &'static str, options: Vec<&'static str>) -> impl IntoView {
    view! {
        <select id={id}>
        {
            options.into_iter().map(|name| {
                view! {
                    <option>
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
    let (selected_color,set_selected_color) = create_signal(0);

    view! {
        <h1 class="center"> "Team Availablity Coordinator" </h1>
        <div class="select-container">
            <SelectMenu id=id options=players />
            <SelectMenu id=id options=modes />
            <SelectMenu id=id options=colors />
            </div>
        <crate::calendar::Calendar />
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
