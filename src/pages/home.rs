use leptos::prelude::*;
use leptos_meta::Title;
use leptos_router::components::A;

#[component]
pub fn Home() -> impl IntoView {
    view! {
        <Title text="Runling"/>
        <div class="home">
            <div class="home-grid">
                <div class="home-main">
                    <A href="/news/003">
                        <img src="images/home/001.png" alt="Latest news: New Website"/>
                    </A>
                </div>
                <div class="home-side">
                    <A href="/dev-diary/002">
                        <img
                            src="images/home/002.png"
                            alt="Dev-Diary: Chat, Friends and Rooms"
                        />
                    </A>
                    <A href="/dev-diary/001">
                        <img src="images/home/003.png" alt="Dev-Diary: Update on Multiplayer"/>
                    </A>
                </div>
            </div>
        </div>
    }
}
