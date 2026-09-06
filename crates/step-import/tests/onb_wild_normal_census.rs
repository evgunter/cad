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

/// The axis order's decision, at `f64`: which world axis the normal is
/// crossed with, and whether the two smallest magnitudes TIE exactly.
///
/// The tie set is the construction's discontinuity, so an exact tie is
/// the class that matters: at `f64` and at a point enclosure it DECIDES
/// (the tie-break keys on the value, not on a zero's sign bit), and it
/// is the only class an enclosure of positive width can fail to decide.
/// Every axis-aligned normal is on it — two components exactly zero.
#[derive(Default, Clone, Copy)]
struct TieClasses {
    exact_tie: usize,
    separated: usize,
    by_axis: [usize; 3],
}

impl TieClasses {
    fn add(&mut self, n: Vec3<f64>) {
        let (ax, ay, az) = (n.x.abs(), n.y.abs(), n.z.abs());
        let k = if az <= ay && az <= ax {
            2
        } else if ay <= ax {
            1
        } else {
            0
        };
        self.by_axis[k] += 1;
        let mut m = [ax, ay, az];
        m.sort_by(f64::total_cmp);
        if m[0] == m[1] {
            self.exact_tie += 1;
        } else {
            self.separated += 1;
        }
    }

    fn merge(&mut self, o: TieClasses) {
        self.exact_tie += o.exact_tie;
        self.separated += o.separated;
        for k in 0..3 {
            self.by_axis[k] += o.by_axis[k];
        }
    }

    fn planes(&self) -> usize {
        self.exact_tie + self.separated
    }
}

/// Planar faces per wild fixture, by whether the normal sits on the
/// axis order's tie set and by which axis wins.
#[test]
#[ignore = "tie census instrument; run explicitly"]
fn axis_tie_census_over_the_wild_corpus() {
    println!("| fixture | planes | on an exact tie | separated | k = x | k = y | k = z |");
    println!("| --- | --- | --- | --- | --- | --- | --- |");
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
            c.exact_tie,
            c.separated,
            c.by_axis[0],
            c.by_axis[1],
            c.by_axis[2]
        );
    }
    println!(
        "| **wild corpus** | {} | {} | {} | {} | {} | {} |",
        total.planes(),
        total.exact_tie,
        total.separated,
        total.by_axis[0],
        total.by_axis[1],
        total.by_axis[2]
    );
}
