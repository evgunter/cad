//! The viewer binary.
//!
//! Two lines, and both are entry-point acts: commit the run's ε, then
//! hand it to the application. Everything below receives the witness
//! as a parameter (`scripts/gates/witness-not-ambient.sh`), and the
//! façade door is where a program inside this workspace mints one.

fn main() -> eframe::Result<()> {
    viewer::app::run(pncad::tolerance::witness())
}
