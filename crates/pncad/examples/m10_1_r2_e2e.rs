//! **M10-1 R2 review exercise** — a first-time user of the
//! distribution surface, driving it end to end through `pncad` and
//! nothing else.
//!
//! Run: `cargo run -p pncad --example m10_1_r2_e2e`
//!
//! The point is ergonomics, not coverage: what a caller who has read
//! `ERROR-DESIGN` E1/E2 and nothing of the implementation has to
//! discover in order to annotate two parameters, persist them, read
//! them back, and get the three consumables out. Every friction the
//! run hit is recorded in the review report rather than papered over
//! here — the code below is what I actually had to write.
//!
//! REVIEW ARTEFACT: written for the M10-1 fix pass. It asserts, so it
//! is not dead weight, but it is scheduled for deletion once the fix
//! pass has read it — it duplicates no gate.

use pncad::analysis::{AnalysisPolicy, MeasureUnavailable, analyzed_box, box_mass, tail_mass};
use pncad::document::{
    Dimension, Distribution, DocEdit, DocParam, ParamName, ProfileDoc, apply, load, save,
};
use pncad::geom_core::Tol;

fn set(doc: &ProfileDoc, name: &str, value: DocParam) -> ProfileDoc {
    apply(
        doc,
        &DocEdit::SetDocParam {
            name: ParamName::new(name),
            value,
        },
        Tol::witness(),
    )
    .expect("the parameter declaration applies")
    .doc
}

fn main() {
    let tol = Tol::witness();

    // ---- 1. Author. A bore whose spread I know, a plate thickness
    // whose limits I know but whose shape I do not, and a plain
    // parameter I have said nothing about.
    let doc = ProfileDoc::empty_derived("r2-e2e", tol);
    let doc = set(
        &doc,
        "bore_r",
        DocParam::continuous_with(
            Dimension::Length,
            0.004,
            Distribution::Normal { sigma: 5e-6 },
        ),
    );
    let doc = set(
        &doc,
        "plate_t",
        DocParam::continuous_with(
            Dimension::Length,
            0.012,
            Distribution::Band {
                lo: -2e-4,
                hi: 2e-4,
            },
        ),
    );
    let doc = set(
        &doc,
        "boss_h",
        DocParam::continuous(Dimension::Length, 0.006),
    );
    println!("authored 3 parameters");

    // ---- 2. Round trip.
    let text = save(&doc, &[], tol).expect("saves");
    println!(
        "saved, {} bytes, header {:?}",
        text.len(),
        text.lines().next().unwrap()
    );
    let back = load(&text, tol).expect("loads").doc;
    assert!(back.bit_eq(&doc), "the annotations survive the round trip");
    println!("reloaded, bit-identical");

    // ---- 3. The analyzed box.
    let policy = AnalysisPolicy::default();
    println!("policy: quantile_mass = {}", policy.quantile_mass());
    let boxed = analyzed_box(&back, &policy);
    for (name, axis) in boxed.params() {
        let (lo, hi) = axis.absolute();
        println!(
            "  {:<8} nominal {:<8} box [{:.8}, {:.8}] width {:.3e} dist {:?}",
            name.0,
            axis.nominal,
            lo,
            hi,
            axis.offsets.width(),
            axis.distribution
        );
    }
    let varying: Vec<_> = boxed.varying().map(|(n, _)| n.0.clone()).collect();
    println!("  varying axes: {varying:?}");
    assert_eq!(varying.len(), 2, "the unannotated parameter is fixed");

    // ---- 4. The mass columns. NOTE the shape a caller must learn:
    // the box hands back an `AnalyzedParam` carrying an
    // `Option<Distribution>`, and both mass doors want the NAME and
    // the DISTRIBUTION as separate arguments — so reading a column off
    // an axis is a three-step unpack, done once per axis.
    for (name, axis) in boxed.params() {
        let Some(dist) = axis.distribution else {
            println!("  {:<8} no distribution: fixed, mass 1", name.0);
            continue;
        };
        match tail_mass(name, &dist, &axis.offsets) {
            Ok(t) => println!("  {:<8} tail mass {t:.6e}", name.0),
            Err(e) => println!("  {:<8} tail REFUSED: {e}", name.0),
        }
    }

    // ---- 5. Leaf pricing: the normal answers, the band refuses.
    let bore = boxed.get(&ParamName::new("bore_r")).expect("axis");
    let bore_dist = bore.distribution.expect("annotated");
    let upper_half = box_mass(
        &ParamName::new("bore_r"),
        &bore_dist,
        (0.0, bore.offsets.hi),
    )
    .expect("a normal prices a leaf");
    println!("  bore_r upper half of the box holds {upper_half:.6}");

    let plate = boxed.get(&ParamName::new("plate_t")).expect("axis");
    let plate_dist = plate.distribution.expect("annotated");
    match box_mass(&ParamName::new("plate_t"), &plate_dist, (0.0, 1e-4)) {
        Err(MeasureUnavailable::BandHasNoMeasure { param }) => {
            println!("  plate_t leaf REFUSED (as designed), naming {:?}", param.0);
        }
        other => panic!("a band must refuse a leaf, got {other:?}"),
    }
    // ... but the WHOLE box is priced at 1, which is the thing worth
    // knowing about the refusal's edges.
    println!(
        "  plate_t whole box prices at {:?}",
        box_mass(
            &ParamName::new("plate_t"),
            &plate_dist,
            (plate.offsets.lo, plate.offsets.hi)
        )
    );

    // ---- 6. The trap a first-time user walks into: change the bore's
    // nominal the obvious way and the distribution is gone, silently.
    let dim = back.params()[&ParamName::new("bore_r")].dim();
    let edited = set(&back, "bore_r", DocParam::continuous(dim, 0.0045));
    println!(
        "  after a plain value edit, bore_r distribution = {:?}",
        edited.params()[&ParamName::new("bore_r")].distribution()
    );
    assert!(
        edited.params()[&ParamName::new("bore_r")]
            .distribution()
            .is_none(),
        "silently dropped"
    );
    println!("DONE");
}
