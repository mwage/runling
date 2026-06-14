use std::borrow::Cow;

use leptos::prelude::*;
use leptos_router::components::A;

/// The deployment base path (e.g. `/runling` under GitHub Pages, `""` for local
/// `trunk serve`). `App` puts this in context so [`Link`] can prepend it.
#[derive(Clone)]
pub struct BasePath(pub Cow<'static, str>);

/// Prefix an absolute in-app path with the deployment base from context.
fn with_base(path: &str) -> String {
    let base = use_context::<BasePath>()
        .map(|b| b.0)
        .unwrap_or(Cow::Borrowed(""));
    format!("{base}{path}")
}

/// In-app navigation link — use this instead of `leptos_router`'s `<A>` for
/// internal routes.
///
/// `leptos_router` applies the router `base` only to *relative* hrefs; an
/// absolute `<A href="/news">` is rendered and navigated verbatim (see
/// `use_resolved_path`), so under GitHub Pages it would hit the domain root
/// instead of `/runling/news`. `<Link>` bakes the base into the href so links
/// resolve correctly both at `/runling/` and at local root, while staying
/// absolute (robust regardless of the current route).
#[component]
pub fn Link(
    /// Absolute in-app path, e.g. `/news` or `/dev-diary/002`. Accepts `&str`
    /// or `String` (so `format!(...)` works at call sites).
    #[prop(into)]
    href: String,
    children: Children,
) -> impl IntoView {
    let href = with_base(&href);
    view! { <A href=href>{children()}</A> }
}
