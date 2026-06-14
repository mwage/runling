use leptos::prelude::*;
use leptos_meta::Title;

use crate::nav::Link;

#[component]
pub fn Home() -> impl IntoView {
    view! {
        <Title text="Runling"/>
        <div class="home">
            <div class="home-grid">
                <div class="home-main">
                    <Link href="/news/003">
                        <img src="images/home/001.png" alt="Latest news: New Website"/>
                    </Link>
                </div>
                <div class="home-side">
                    <Link href="/dev-diary/002">
                        <img
                            src="images/home/002.png"
                            alt="Dev-Diary: Chat, Friends and Rooms"
                        />
                    </Link>
                    <Link href="/dev-diary/001">
                        <img src="images/home/003.png" alt="Dev-Diary: Update on Multiplayer"/>
                    </Link>
                </div>
            </div>
        </div>
    }
}
