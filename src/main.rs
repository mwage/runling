use std::borrow::Cow;

use leptos::mount::mount_to_body;
use leptos::prelude::*;
use leptos_meta::{provide_meta_context, Title};
use leptos_router::components::{Route, Router, Routes};
use leptos_router::path;

mod components;
mod pages;

use components::header::Header;
use pages::dev_diary::{DevDiary001, DevDiary002, DevDiaryList};
use pages::download::Download;
use pages::game::{GameAbout, GameCharacters, GameScreenshots};
use pages::home::Home;
use pages::news::{News001, News002, News003, NewsList};

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(App);
}

#[component]
fn App() -> impl IntoView {
    provide_meta_context();

    // For deployment under a GitHub Pages subpath (e.g. /runling/), CI sets the
    // RUNLING_BASE env var (to the repo name) at build time and builds with
    // `trunk build --release --public-url "/$RUNLING_BASE/"`. Unset for local dev,
    // so `trunk serve` keeps working at `/`.
    let base: Cow<'static, str> = Cow::Borrowed(option_env!("RUNLING_BASE").unwrap_or(""));

    view! {
        // Default document title; each page overrides it with its own <Title/>.
        <Title text="Runling"/>
        <Router base=base>
            <div class="container">
                <Header/>
            </div>
            <div class="content container">
                <Routes fallback=|| view! { <h2>"Page not found"</h2> }>
                    <Route path=path!("/") view=Home/>
                    <Route path=path!("/news") view=NewsList/>
                    <Route path=path!("/news/001") view=News001/>
                    <Route path=path!("/news/002") view=News002/>
                    <Route path=path!("/news/003") view=News003/>
                    <Route path=path!("/dev-diary") view=DevDiaryList/>
                    <Route path=path!("/dev-diary/001") view=DevDiary001/>
                    <Route path=path!("/dev-diary/002") view=DevDiary002/>
                    <Route path=path!("/game") view=GameAbout/>
                    <Route path=path!("/game/characters") view=GameCharacters/>
                    <Route path=path!("/game/screenshots") view=GameScreenshots/>
                    <Route path=path!("/download") view=Download/>
                </Routes>
            </div>
        </Router>
    }
}
