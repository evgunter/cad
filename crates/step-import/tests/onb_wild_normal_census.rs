//! **The axis-order tie census over the wild corpus** — the
//! third-party STEP bodies, the one input in the project nobody here
//! authored.
//!
//! These planes are minted by `step_import`'s recogniser
//! (`recognize.rs`, which takes the frame from
//! `Vec3::orthonormal_basis` like `newell_plane` does) over normals the
//! FILE supplies, so this is the row where a normal sitting on the
//! construction's tie set arrives from outside rather than from the
//! kernel's own arithmetic.
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
use geom_core::{Tol, Vec3};
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

/// The world-axis choice, at `f64`: which axis the normal is crossed
/// with, and whether it sits exactly ON the comparison's seam.
///
/// The seam `|n.z| = max(|n.x|, |n.y|)` is the construction's one
/// discontinuity — the 45° cone — and it is the only class an enclosure
/// of positive width can fail to decide. No axis direction and no
/// axis-aligned face is on it, which is the property the census is here
/// to measure rather than assert.
#[derive(Default, Clone, Copy)]
struct TieClasses {
    on_seam: usize,
    off_seam: usize,
    e_z_arm: usize,
    e_y_arm: usize,
    /// The same count for the rule this construction did NOT take: an
    /// order over all THREE components, whose tie set is where the two
    /// smallest magnitudes are equal. Every axis-aligned normal is on
    /// that one, which is why it is not the rule.
    three_way_tie: usize,
}

impl TieClasses {
    fn add(&mut self, n: Vec3<f64>) {
        let other = n.x.abs().max(n.y.abs());
        if n.z.abs() <= other {
            self.e_z_arm += 1;
        } else {
            self.e_y_arm += 1;
        }
        if n.z.abs() == other {
            self.on_seam += 1;
        } else {
            self.off_seam += 1;
        }
        let mut m = [n.x.abs(), n.y.abs(), n.z.abs()];
        m.sort_by(f64::total_cmp);
        if m[0] == m[1] {
            self.three_way_tie += 1;
        }
    }

    fn merge(&mut self, o: TieClasses) {
        self.on_seam += o.on_seam;
        self.off_seam += o.off_seam;
        self.e_z_arm += o.e_z_arm;
        self.e_y_arm += o.e_y_arm;
        self.three_way_tie += o.three_way_tie;
    }

    fn planes(&self) -> usize {
        self.on_seam + self.off_seam
    }
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
    let mut total = TieClasses::default();
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
        let mut c = TieClasses::default();
        for (_, surface) in body.surfaces() {
            if let Surface::Plane { normal, .. } = surface {
                c.add(*normal);
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
