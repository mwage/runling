use leptos::prelude::*;
use leptos_meta::Title;

// The original site offered separate Dropbox links for Windows and Mac. Those are
// replaced with a single Google Drive folder.
const DOWNLOAD_URL: &str =
    "https://drive.google.com/drive/folders/1Lrmf6LWiAx570GZx-chiyjZ1DuJYzP-I?usp=sharing";

#[component]
pub fn Download() -> impl IntoView {
    view! {
        <div class="download">
            <Title text="Download - Runling"/>
            <h2>"Get our Single-Player demo!"</h2>
            <h4>"Single-Player demo Patch 1:"</h4>
            <a target="_blank" rel="noopener noreferrer" href=DOWNLOAD_URL>
                <button class="btn btn-primary" style="width: auto">
                    "Download (Google Drive)"
                </button>
            </a>
        </div>
    }
}
