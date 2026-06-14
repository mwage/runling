use leptos::prelude::*;

use crate::nav::Link;

#[derive(Clone, Copy)]
pub struct NavLink {
    pub name: &'static str,
    pub path: &'static str,
}

#[component]
pub fn Header() -> impl IntoView {
    // The original site also had a "Forum" entry; it is intentionally dropped in
    // this port (no backend).
    let buttons = vec![
        (
            NavLink { name: "Home", path: "/" },
            vec![
                NavLink { name: "News-Archive", path: "/news" },
                NavLink { name: "Dev-Diary", path: "/dev-diary" },
            ],
        ),
        (
            NavLink { name: "Game", path: "/game" },
            vec![
                NavLink { name: "About", path: "/game" },
                NavLink { name: "Characters", path: "/game/characters" },
                NavLink { name: "Screenshots", path: "/game/screenshots" },
            ],
        ),
        (NavLink { name: "Download", path: "/download" }, vec![]),
    ];

    view! {
        <header>
            <div class="row">
                <img src="images/Logo.png" alt="Runling"/>
            </div>
            <nav class="row">
                <div class="col-md-8 col-md-offset-2">
                    {buttons
                        .into_iter()
                        .map(|(button, submenus)| view! { <ButtonGroup button submenus/> })
                        .collect_view()}
                </div>
            </nav>
        </header>
    }
}

#[component]
fn ButtonGroup(button: NavLink, submenus: Vec<NavLink>) -> impl IntoView {
    let (selected, set_selected) = signal(false);
    let has_submenu = !submenus.is_empty();

    view! {
        <div
            class="button-group"
            on:mouseenter=move |_| set_selected.set(true)
            on:mouseleave=move |_| set_selected.set(false)
        >
            <Link href=button.path>
                <div class="navItem" class:highlighted=move || selected.get()>
                    {button.name}
                </div>
            </Link>
            {move || {
                (has_submenu && selected.get()).then(|| {
                    let items = submenus.clone();
                    view! {
                        <div class="subMenu">
                            {items
                                .into_iter()
                                .map(|sub| {
                                    view! {
                                        <Link href=sub.path>
                                            <div class="navSubItem">{sub.name}</div>
                                        </Link>
                                    }
                                })
                                .collect_view()}
                        </div>
                    }
                })
            }}
        </div>
    }
}
