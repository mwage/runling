// Surface RUNLING_BASE (the GitHub Pages repo subpath, set by CI) to the crate
// as a compile-time env var, and — crucially — tell cargo to rebuild whenever it
// changes. `option_env!` reads are invisible to cargo's fingerprinting, so with
// build caching a stale or empty value would otherwise get baked into the wasm.
fn main() {
    println!("cargo:rerun-if-env-changed=RUNLING_BASE");
    let base = std::env::var("RUNLING_BASE").unwrap_or_default();
    println!("cargo:rustc-env=RUNLING_BASE={base}");
}
