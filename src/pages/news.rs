use leptos::prelude::*;
use leptos_meta::Title;

use crate::nav::Link;

#[derive(Clone, Copy)]
struct NewsEntry {
    id: &'static str,
    date: &'static str,
    title: &'static str,
    description: &'static str,
}

const NEWS_ENTRIES: &[NewsEntry] = &[
    NewsEntry {
        id: "003",
        date: "September 17, 2017",
        title: "New Website",
        description: "Welcome to our new Website!",
    },
    NewsEntry {
        id: "002",
        date: "June 15, 2017",
        title: "Single Player Demo Patch 1",
        description: "This update includes bug fixes, a few UI changes, and improved balancing for the Single player Demo!",
    },
    NewsEntry {
        id: "001",
        date: "May 21, 2017",
        title: "Website Launch and Single Player Prototype",
        description: "With the launch of our new website we're happy to announce the release of the first Single Player Demo of our Runling game!",
    },
];

#[component]
pub fn NewsList() -> impl IntoView {
    view! {
        <div class="news">
            <Title text="News-Archive - Runling"/>
            <h2>"News-Archive"</h2>
            {NEWS_ENTRIES
                .iter()
                .map(|news| {
                    view! {
                        <div class="row news-item">
                            <Link href=format!("/news/{}", news.id)>
                                <div class="col-md-3 col-sm-5 col-xs-12">
                                    <img
                                        class="img-responsive"
                                        src=format!("images/news/{}.jpg", news.id)
                                        alt=news.title
                                    />
                                </div>
                                <div class="col-md-9 col-sm-7 col-xs-12">
                                    <h4>{news.title}</h4>
                                    <p class="article">{news.description}</p>
                                    <p class="date">{news.date}</p>
                                </div>
                            </Link>
                        </div>
                    }
                })
                .collect_view()}
        </div>
    }
}

#[component]
pub fn News001() -> impl IntoView {
    view! {
        <div>
            <Title text="Website Launch and Single Player Demo - Runling"/>
            <h2>"Website Launch and Single Player Demo"</h2>
            <div class="article">
                <div class="logo">
                    <img
                        class="img-responsive"
                        src="images/news/001.jpg"
                        alt="Website Launch and Single Player Demo"
                    />
                </div>
                <p>
                    "With the launch of our new website we're happy to announce the release of the first Single Player Demo of our Runling game! \
                    The game is based on the SC2 arcade games Runling Run and Survivorling Arena and our plan is to expand on them in the future."
                </p>
                <br/>
                <h4>"Currently we offer:"</h4>
                <ul>
                    <li>"A spiral map with 9 levels each with Normal and Hard difficulty (leaned closely on the Runling Run 4 map of firefly)"</li>
                    <li>"An arena map to test your skills and track your improvement (based on Dream's Survivorling Arena)"</li>
                </ul>
                <h4>"Additionally you can choose between:"</h4>
                <ul>
                    <li>"Classic Mode: one death and it's all over"</li>
                    <li>"Practice Mode: Jump to whatever level you are struggling with and practice it until it's easy"</li>
                    <li>"Time Mode (only for RLR): Try to finish the levels as fast as possible, 3 Lifes per level."</li>
                </ul>
                <h4>"Bonus Features:"</h4>
                <ul>
                    <li>"Inbuilt Autoclicker: Takes load off your fingers and you can easily change the hotkeys for it ingame. Effective and not laggy."</li>
                    <li>"Adjustable Camera: zoom, angle, follow, whatever you need, we got you covered."</li>
                    <li>"Level Records: The game keeps track of your fastest finishes for every individual level."</li>
                </ul>
                <p>"We hope you'll enjoy our game and leave us your feedback in the forum!"</p>
            </div>
            <p class="date">"May 21, 2017"</p>
        </div>
    }
}

#[component]
pub fn News002() -> impl IntoView {
    view! {
        <div>
            <Title text="Single Player Demo Patch 1 - Runling"/>
            <h2>"Single Player Demo Patch 1"</h2>
            <div class="article">
                <div class="logo">
                    <img
                        class="img-responsive"
                        src="images/news/002.jpg"
                        alt="Single Player Demo Patch 1"
                    />
                </div>
                <p>
                    "Hey guys! This update includes bug fixes, a few UI changes, and improved balancing for the Single player Demo! Thank you all for your feedback!"
                </p>
                <h3>"Additions and changes:"</h3>
                <ul>
                    <li>"Added a simple minimap."</li>
                    <li>"Redesigned the Main Menu, let us know if you like it!"</li>
                    <li>"Reworked the player movement script to be more consistent as well as working for different heights, which will be needed for Bosses and future modes."</li>
                </ul>
                <h3>"Balance changes:"</h3>
                <h4>"General:"</h4>
                <ul>
                    <li>"Increased the maximum zoom distance (50 -> 60)"</li>
                    <li>"Increased player acceleration (80 -> 100)"</li>
                </ul>
                <h4>"Arena:"</h4>
                <ul>
                    <li>"We rebalanced pretty much all Arena levels. They should now feel more consistent."</li>
                </ul>
                <h4>"Runling Run:"</h4>
                <ul>
                    <li>"Reduced Player movement speed (15 -> 13)"</li>
                    <li>"Player now spawns at the front end of the platform instead of in the middle."</li>
                    <li>"Level 6 Normal: Increased the speed of blue drones, but reduced the spawnrate."</li>
                    <li>"Level 2 Hard: Reduced the amount of grey drones."</li>
                    <li>"Level 3 Hard: Reduced the amount of blue drones slightly."</li>
                    <li>"Level 5 Hard: Reduced the amount of red drones slightly."</li>
                </ul>
                <h4>"Bug Fixes"</h4>
                <ul>
                    <li>"Fixed a bug where the remaining time didn't get converted to score after completing Level 9 in Time Mode."</li>
                    <li>"Fixed a bug where it didn't load previous Level 9 records properly."</li>
                    <li>"Fixed a bug where the wrong score screen got shown at the end of a game."</li>
                    <li>"The \"Restart Game\" button now properly loads Level 1 of the game mode you are playing."</li>
                    <li>"Fixed a bug where adjusting your camera (while it was rotated) was messing up WASD camera movement."</li>
                </ul>
                <Link href="/download">"You can get the demo here!"</Link>
                <p>"We hope you like these changes and leave us your feedback in the forum!"</p>
            </div>
            <p class="date">"June 15, 2017"</p>
        </div>
    }
}

#[component]
pub fn News003() -> impl IntoView {
    view! {
        <div>
            <Title text="New Website! - Runling"/>
            <h2>"New Website!"</h2>
            <div class="article">
                <div class="logo">
                    <img class="img-responsive" src="images/news/003.jpg" alt="New Website"/>
                </div>
                <h4>"We are happy to present our new Website!"</h4>
                <p>
                    "Unlike our old website, it is written and hosted by ourselves. It's written as a single-page application using the MEAN stack. \
                    This means we have complete control over what and how we want to display our page. \
                    No more Ads, much faster loading and responsive design to support mobile are just a few of the advantages."
                </p>
                <p>
                    "Another big advantage is that we can now handle the backend and database ourselves, \
                    so in the future we'll be able to connect the Runling game to this website. \
                    Profiles, highscores, etc. could all be used and displayed on it. \
                    These things are still quite a while away, but it's nice to have the options available!"
                </p>
                <p>
                    "We hope you like our new page. Please try out the message board and let us know what you think of it!"
                </p>
            </div>
            <p class="date">"September 17, 2017"</p>
        </div>
    }
}
