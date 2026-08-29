//! The viewer binary.
//!
//! Entry-point acts only: commit the run's ε, read the one optional
//! argument, hand both to the application. Everything below receives
//! the witness as a parameter (`scripts/gates/witness-not-ambient.sh`),
//! and the façade door is where a program inside this workspace mints
//! one.
//!
//! `viewer [document.pncad]` — the optional path is opened through the
//! same typed `Open` operation the dialog feeds (a CLI argument is a
//! way of choosing the `Path`, exactly as the dialog is), which is
//! also the only way to open a document where no desktop portal or
//! `zenity` exists for the dialog to fall back to.

fn main() -> eframe::Result<()> {
    viewer::app::run(
        pncad::tolerance::witness(),
        std::env::args().nth(1).map(std::path::PathBuf::from),
    )
}
