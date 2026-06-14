use leptos::prelude::*;
use leptos_meta::Title;

#[component]
pub fn GameAbout() -> impl IntoView {
    view! {
        <div class="row about">
            <Title text="About - Runling"/>
            <div class="col-sm-8">
                <h2>"About"</h2>
                <div class="article">
                    <p>
                        "Runling is based off the SC2 arcade game Runling Run. \
                        The goal of the game is to reach the center of the big spiral to advance to the next level. \
                        Different kind of drones are moving around on the map. Try to avoid all of them to reach the middle alive!"
                    </p>
                    <p>
                        "The arena mode (based off Survivorling Arena) is a Survival Game that will put your dodging skills to the test. \
                        Try to survive for as long as possible, surrounded by more and more drones and beat your personal highscores!"
                    </p>
                </div>
            </div>
            <div class="col-sm-4">
                <img
                    src="images/news/003.jpg"
                    class="img-responsive"
                    alt="Runling gameplay screenshot"
                />
            </div>
        </div>
    }
}

#[component]
pub fn GameCharacters() -> impl IntoView {
    view! {
        <div class="row">
            <Title text="Characters - Runling"/>
            <div class="col-sm-12">
                <h2>"Characters"</h2>
                <div class="article">
                    <p>"In the future we'll introduce our different Characters here!"</p>
                </div>
            </div>
        </div>
    }
}

const SCREENSHOTS: &[&str] = &[
    "Runling Run Hard Level 3",
    "Runling Run Hard Level 9",
    "Runling Run Hard Level 8",
    "Arena Level 4",
    "Arena Level 7",
    "Arena Difficulty Voting",
];

#[component]
pub fn GameScreenshots() -> impl IntoView {
    // Holds the name of the currently zoomed screenshot, if any.
    let (selected, set_selected) = signal::<Option<&'static str>>(None);

    view! {
        <div class="gallery">
            <Title text="Screenshots - Runling"/>
            <h2>"Screenshots"</h2>
            <div class="row">
                {SCREENSHOTS
                    .iter()
                    .map(|name| {
                        view! {
                            <div class="col-md-4 col-sm-6 col-sm-offset-0">
                                <a on:click=move |_| set_selected.set(Some(*name))>
                                    <img
                                        src=format!("images/screenshots/preview/{}.jpg", name)
                                        class="img-responsive"
                                        alt=*name
                                    />
                                </a>
                                <p>{*name}</p>
                            </div>
                        }
                    })
                    .collect_view()}
            </div>
            {move || {
                selected
                    .get()
                    .map(|name| {
                        view! {
                            <a>
                                <div class="backdrop" on:click=move |_| set_selected.set(None)>
                                    <img
                                        src=format!("images/screenshots/full/{}.jpg", name)
                                        class="img-responsive"
                                        alt=name
                                    />
                                </div>
                            </a>
                        }
                    })
            }}
        </div>
    }
}
