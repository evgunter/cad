//! **The frame-seam census over the wild corpus** — the third-party
//! STEP bodies, the one input in the project nobody here authored.
//!
//! These planes are minted by `step_import`'s recogniser
//! (`recognize.rs`, which takes the frame from
//! `Vec3::orthonormal_basis` like `newell_plane` does) over normals the
//! FILE supplies, so this is the row where a normal sitting on the
//! construction's seam arrives from outside rather than from the
//! kernel's own arithmetic. The classification lives in
//! `test_utils::seam_census`, shared with the other two corpus
//! instruments; its docs say what a count of normals on the seam does
//! and does not measure.
//!
//! [`no_wild_face_sits_on_the_frame_seam`] ASSERTS this corpus's zero;
//! the instrument below it prints the per-fixture table and is
//! `#[ignore]`d.
//!
//! ```text
//! cargo test -p step-import --test all \
//!     -- --ignored --nocapture onb_wild_normal_census
//! ```

use std::path::PathBuf;

use geom::Surface;
use geom_core::Tol;
use step_import::{ImportOptions, StepImport, import_step};
use test_utils::seam_census::SeamClasses;

/// THE CORPUS, written down: `wild.rs`'s nine imports-class fixtures
/// plus the one refusal-class file that ships in the checkout. All ten
/// are committed (`git ls-files` finds every one, `nist/` and
/// `stepcode/` included, each under its own licence file); the
/// not-committed arm below is kept for a checkout that has none of
/// them, and says so rather than skipping in silence.
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
    let path: PathBuf = [
        env!("CARGO_MANIFEST_DIR"),
        "tests",
        "fixtures",
        "wild",
        name,
    ]
    .iter()
    .collect();
    std::fs::read_to_string(&path).ok()
}

/// **This corpus's half of the measurement that decided the
/// comparison, asserted**: no planar face imported from a third-party
/// file sits on the seam `|n.z| = max(|n.x|, |n.y|)/2`. It is the row
/// that speaks for geometry the kernel did not author — the earlier
/// ratio, `|n.z| = max(|n.x|, |n.y|)`, had four faces here, all in
/// `nist_ftc_09_asme1_rd`.
#[test]
fn no_wild_face_sits_on_the_frame_seam() {
    let mut total = SeamClasses::default();
    let mut read = 0usize;
    for name in WILD {
        let Some(src) = text(name) else { continue };
        let body = match import_step(&src, &ImportOptions::default(), Tol::witness()) {
            Ok(StepImport::Solid { body, .. }) => body,
            _ => continue,
        };
        let mut c = SeamClasses::default();
        for (_, surface) in body.surfaces() {
            if let Surface::Plane { normal, .. } = surface {
                c.add((normal.x, normal.y, normal.z));
            }
        }
        assert_eq!(
            c.on_seam,
            0,
            "{name}: {} of its {} planar faces are on the frame seam",
            c.on_seam,
            c.planes()
        );
        total.merge(c);
        read += 1;
    }
    println!(
        "wild corpus ({read} files read): {} planar faces, {} on the seam, \
         {} on the three-component order's tie set",
        total.planes(),
        total.on_seam,
        total.three_way_tie
    );
    // Anti-vacuity: a checkout without the fixtures would otherwise
    // pass this row having read nothing. The COUNTS are ε-dependent —
    // which files import and which faces the recogniser calls planar
    // both move with the run's tolerance (136 faces over 9 files at the
    // default ε, 75 over 8 at 1e-12) — so the floor is well below the
    // smallest row. The zero above is not ε-dependent: it is a
    // statement about directions.
    assert!(
        read >= 8 && total.planes() >= 60,
        "the wild corpus shrank: {read} files read, {} planar faces",
        total.planes()
    );
}

/// Planar faces per wild fixture, by whether the normal sits on the
/// axis order's tie set and by which axis wins.
#[test]
#[ignore = "tie census instrument; run explicitly"]
fn axis_tie_census_over_the_wild_corpus() {
    println!(
        "| fixture | planes | on the seam | off it | e_z arm | e_y arm | on a three-way tie |"
    );
    println!("| --- | --- | --- | --- | --- | --- | --- | --- |");
    let mut total = SeamClasses::default();
    for name in WILD {
        let Some(src) = text(name) else {
            println!("| {name} | (not committed) | | | | | |");
            continue;
        };
        let body = match import_step(&src, &ImportOptions::default(), Tol::witness()) {
            Ok(StepImport::Solid { body, .. }) => body,
            Ok(StepImport::Wireframe { .. }) => {
                println!("| {name} | (wireframe, no faces) | | | | | |");
                continue;
            }
            Err(e) => {
                println!("| {name} | (refused: {e}) | | | | | |");
                continue;
            }
        };
        let mut c = SeamClasses::default();
        for (_, surface) in body.surfaces() {
            if let Surface::Plane { normal, .. } = surface {
                c.add((normal.x, normal.y, normal.z));
            }
        }
        total.merge(c);
        println!(
            "| {name} | {} | {} | {} | {} | {} | {} |",
            c.planes(),
            c.on_seam,
            c.off_seam,
            c.e_z_arm,
            c.e_y_arm,
            c.three_way_tie
        );
    }
    println!(
        "| **wild corpus** | {} | {} | {} | {} | {} | {} |",
        total.planes(),
        total.on_seam,
        total.off_seam,
        total.e_z_arm,
        total.e_y_arm,
        total.three_way_tie
    );
}
