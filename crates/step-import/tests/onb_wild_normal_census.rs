//! **Wall-normal `z` census over the wild corpus** — the third-party
//! STEP bodies, the one input in the project nobody here authored.
//!
//! These planes are minted by `step_import`'s recogniser
//! (`recognize.rs`, which takes the frame from
//! `Vec3::orthonormal_basis` like `newell_plane` does) over normals the
//! FILE supplies, so this is the row where a `-0.0` can arrive from
//! outside rather than from the kernel's own arithmetic.
//!
//! `#[ignore]`d: asserts nothing, gates nothing, prints. The corpus is
//! written down as literals below — the nine imports-class fixtures.
//!
//! ```text
//! cargo test -p step-import --test all \
//!     -- --ignored --nocapture onb_wild_normal_census
//! ```

use std::path::PathBuf;

use geom::Surface;
use geom_core::Tol;
use step_import::{ImportOptions, StepImport, import_step};

/// THE CORPUS, written down: `wild.rs`'s nine imports-class fixtures
/// plus the one refusal-class file that ships in the checkout. Four of
/// the nine (the `nist/` and `stepcode/` entries) are license-excluded
/// and are not committed — the row says so rather than skipping.
const WILD: [&str; 10] = [
    "adafruit/328_2500mAh_battery.step",
    "adafruit/1982_MPR121.step",
    "adafruit/805_slide_switch.step",
    "adafruit/931_OLED_128x32_I2C.step",
    "adafruit/64_Halfsize_Breadboard.step",
    "nist/nist_ftc_09_asme1_rd.stp",
    "stepcode/sg1-c5-214.stp",
    "nist/nist_ftc_11_asme1_rb.stp",
    "occ-oss/cq_red_cube_blue_cylinder.step",
    "occ-oss/b123d_nema17_bracket.step",
];

fn text(name: &str) -> Option<String> {
    let path: PathBuf = [env!("CARGO_MANIFEST_DIR"), "tests", "fixtures", "wild", name]
        .iter()
        .collect();
    std::fs::read_to_string(&path).ok()
}

/// **Table 2 (wild row)** — planar faces per fixture, by the class of
/// the stored normal's `z`.
#[test]
#[ignore = "wall-normal census instrument; run explicitly"]
fn wall_normal_z_census_over_the_wild_corpus() {
    println!("| fixture | planes | z = +0.0 | z = -0.0 | 0 < |z| < 1e-12 | other |");
    println!("| --- | --- | --- | --- | --- | --- |");
    let (mut tp, mut tn, mut tt, mut to) = (0usize, 0usize, 0usize, 0usize);
    for name in WILD {
        let Some(src) = text(name) else {
            println!("| {name} | (not committed) | | | | |");
            continue;
        };
        let body = match import_step(&src, &ImportOptions::default(), Tol::witness()) {
            Ok(StepImport::Solid { body, .. }) => body,
            Ok(StepImport::Wireframe { .. }) => {
                println!("| {name} | (wireframe, no faces) | | | | |");
                continue;
            }
            Err(e) => {
                println!("| {name} | (refused: {e}) | | | | |");
                continue;
            }
        };
        let (mut p, mut n, mut t, mut o) = (0usize, 0usize, 0usize, 0usize);
        for (_, surface) in body.surfaces() {
            let Surface::Plane { normal, .. } = surface else {
                continue;
            };
            let z = normal.z;
            if z == 0.0 {
                if z.is_sign_negative() {
                    n += 1;
                } else {
                    p += 1;
                }
            } else if z.abs() < 1e-12 {
                t += 1;
            } else {
                o += 1;
            }
        }
        println!("| {name} | {} | {p} | {n} | {t} | {o} |", p + n + t + o);
        tp += p;
        tn += n;
        tt += t;
        to += o;
    }
    println!(
        "| **wild corpus** | {} | {tp} | {tn} | {tt} | {to} |",
        tp + tn + tt + to
    );
}
