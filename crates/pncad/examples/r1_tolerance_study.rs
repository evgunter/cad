//! **R1 e2e exercise (M10-1 review)**: a first-time user of the new
//! distribution surface, written against the façade only, doing the
//! whole loop a tolerance study would: author parameters with
//! distributions, save, reload, read the analyzed box and the mass
//! columns, and run into the Band refusal on purpose.
//!
//! Run: `cargo run -p pncad --example r1_tolerance_study -- <out.pncad>`

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use pncad::analysis::{AnalysisPolicy, analyzed_box, box_mass, tail_mass};
use pncad::document::{
    Dimension, Distribution, DocEdit, DocParam, ParamName, ProfileDoc, apply, load, save,
};
use pncad::geom_core::Tol;

fn main() {
    let out = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "r1_tolerance_study.pncad".into());
    let tol = Tol::witness();

    // A bracket with a machined bore, a stock plate thickness, and a
    // hole count. The bore is measured (normal), the plate is vendor
    // stock with catalogue limits and no stated shape (band), the web
    // is toleranced symmetric-ish but clipped by inspection (truncated
    // normal), and the fit allowance is uniform by assumption.
    let mut doc = ProfileDoc::empty_derived("r1-tolerance-study", tol);
    let declare: &[(&str, DocParam)] = &[
        (
            "bore_r",
            DocParam::continuous_with(
                Dimension::Length,
                0.004,
                Distribution::Normal { sigma: 5e-6 },
            ),
        ),
        (
            "plate_t",
            DocParam::continuous_with(
                Dimension::Length,
                0.012,
                Distribution::Band { lo: -2e-4, hi: 2e-4 },
            ),
        ),
        (
            "web_w",
            DocParam::continuous_with(
                Dimension::Length,
                0.003,
                Distribution::TruncatedNormal {
                    sigma: 4e-5,
                    lo: -1e-4,
                    hi: 1e-4,
                },
            ),
        ),
        (
            "fit",
            DocParam::continuous_with(
                Dimension::Scalar,
                1.0,
                Distribution::Uniform { lo: -0.02, hi: 0.02 },
            ),
        ),
        ("holes", DocParam::Count { value: 4 }),
    ];
    for (name, value) in declare {
        doc = apply(
            &doc,
            &DocEdit::SetDocParam {
                name: ParamName::new(*name),
                value: value.clone(),
            },
            tol,
        )
        .expect("declaration applies")
        .doc;
    }

    // Save, forget everything, reload — the tolerance study starts
    // from the file, as a colleague's would.
    let text = save(&doc, &[], tol).expect("saves");
    std::fs::write(&out, &text).expect("writes");
    let doc = load(&std::fs::read_to_string(&out).expect("reads"), tol)
        .expect("loads")
        .doc;
    println!("reloaded {out}: {} params", doc.params().len());

    // The analyzed box under the default (±3σ) policy.
    let policy = AnalysisPolicy::default();
    let b = analyzed_box(&doc, &policy);
    println!("\nanalyzed box (quantile mass {}):", policy.quantile_mass());
    for (name, axis) in b.params() {
        let (lo, hi) = axis.absolute();
        println!(
            "  {:<8} nominal {:>8}  offsets [{:+.3e}, {:+.3e}]  absolute [{lo}, {hi}]{}",
            format!("{:?}", name.0),
            axis.nominal,
            axis.offsets.lo,
            axis.offsets.hi,
            if axis.offsets.is_fixed() { "  (FIXED)" } else { "" },
        );
    }

    // The tail column, per varying axis.
    println!("\ntail mass outside the box:");
    for (name, axis) in b.varying() {
        let dist = axis.distribution.expect("varying implies annotated");
        match tail_mass(name, &dist, &axis.offsets) {
            Ok(t) => println!("  {:<8} {t:.3e}", format!("{:?}", name.0)),
            Err(e) => println!("  {:<8} REFUSED: {e}", format!("{:?}", name.0)),
        }
    }

    // Price a driver-leaf-shaped sub-box on the measured bore.
    let bore = b.get(&ParamName::new("bore_r")).expect("axis");
    let leaf = (0.0, bore.offsets.hi / 2.0);
    let m = box_mass(
        &ParamName::new("bore_r"),
        &bore.distribution.expect("annotated"),
        leaf,
    )
    .expect("a normal prices a leaf");
    println!("\nleaf mass on bore_r over {leaf:?}: {m:.6}");

    // And the refusal a first-time user WILL hit: pricing a leaf over
    // the vendor band.
    let plate = b.get(&ParamName::new("plate_t")).expect("axis");
    match box_mass(
        &ParamName::new("plate_t"),
        &plate.distribution.expect("annotated"),
        (0.0, 1e-4),
    ) {
        Ok(m) => println!("plate_t leaf mass: {m} (unexpected!)"),
        Err(e) => println!("plate_t leaf REFUSED, as designed:\n  {e}"),
    }
}
