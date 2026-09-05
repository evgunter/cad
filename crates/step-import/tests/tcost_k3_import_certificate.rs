//! **The tier-3′ certificate through the READER** — one certified
//! quadrature per imported body, and the field the reader hands back
//! IS it.
//!
//! `crates/sweep/tests/tcost_k3_certificate.rs` proves the claim at the
//! doors. This file proves it at the door's one non-test consumer, and
//! it exists because the door's claim does not travel: `gate3` could be
//! rewritten as
//!
//! ```ignore
//! topo::validate_pseudomanifold(body, records, tol)?;
//! Ok(topo::mass_properties(body, tol)...)
//! ```
//!
//! — a SECOND computation behind a field documented as *not a second
//! computation* — and every value in the tree would stay green, because
//! a recomputation of the same quadrature over the same body at the
//! same band produces the same four fields bit for bit. Only a COUNT
//! sees it. So this row counts.
//!
//! The counter is `geom_core::k_stats`' verdict log, the kernel's own
//! recording channel, exactly as the `sweep` half uses it: the
//! `props_quad_*` verdicts of a call are a deterministic function of
//! the certificates it ran, so one measurement's count is the unit.
//!
//! # ε and cost
//!
//! The fixture is scaled by ε for the reason the `sweep` half is (that
//! file's module docs): a fixed size certifies at some ε rows and
//! refuses at others, and a row that tolerates both compares nothing on
//! half of them. At `1e5·ε` the prism converges in the schedule's first
//! round at every ε row, so this row is one export, one import and two
//! certified quadratures — flat across the ε draw.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

// The reader whose gate is under test, the doors it calls, and the
// quadrature they run. Gated to the same set as the `sweep` half, so
// the two halves of one claim cannot come apart on a change filter.
test_utils::gated_to![
    "crates/step-import/src/lib.rs",
    "crates/topo/src/validate.rs",
    "crates/topo/src/props.rs",
    "crates/geom-brep/src/props/",
    "crates/step-import/tests/common/",
];

use crate::common::{arc_section, stacked};
use geom_core::Tol;
use geom_core::k_stats::Bracket;
use step_import::{ImportOptions, StepImport, import_step};
use topo::MassProperties;

/// The arc PRISM at scale `s`: two identical arc sections stacked and
/// skinned at `v`-degree 1 — a SINGLE-SOLID body with a rational wall,
/// which is the shape this claim needs. Single-solid matters: a
/// multi-instance file runs the per-solid `gate` first, and it is the
/// AGGREGATE gate whose certificate the reader hands back.
fn arc_prism(s: f64) -> topo::Body<f64> {
    sweep::loft_body::<f64>(
        &[arc_section(s), arc_section(s)],
        &stacked(&[0.0, 1.0], s),
        1,
        Tol::witness(),
    )
    .expect("the arc prism lofts")
    .body
}

/// The number of quadrature-lane classifications recorded while `run`
/// executed — the certificate COUNTER (the `sweep` half's, verbatim,
/// because it is the same measurement).
fn quad_verdicts(run: impl FnOnce()) -> usize {
    let bracket = Bracket::open();
    run();
    bracket
        .finish()
        .verdicts
        .iter()
        .filter(|v| v.predicate.starts_with("props_quad"))
        .count()
}

/// The four fields as raw bits — the identity currency.
fn bits(m: &MassProperties<f64>) -> [u64; 4] {
    [
        m.volume.to_bits(),
        m.surface_area.to_bits(),
        m.volume_pad.to_bits(),
        m.area_pad.to_bits(),
    ]
}

/// **ONE CERTIFICATE / IDENTITY, through `import_step`** — the reader
/// pays one certified quadrature for the body it ships, and
/// `StepImport::Solid`'s `enclosure` is that quadrature's own object.
///
/// **ONE CERTIFICATE** is the assertion the import path could not
/// otherwise make. The `enclosure` field is documented as the gate's
/// own value rather than a second computation, and no comparison of
/// VALUES can check that — a second computation would agree bit for
/// bit. The count is the only witness, and it is exact: a single-solid
/// file skips the per-solid gate as an identity at one instance, so the
/// whole import runs exactly the certificates its aggregate tier-3′
/// gate runs, which must be one measurement's worth.
///
/// **IDENTITY** then says the value the reader hands out is that
/// certificate: bit-identical in all four fields to `mass_properties`
/// on the body the reader shipped. The row asserts the arm it takes —
/// the fixture is ε-scaled so the import certifies at every ε row, and
/// a refusal here is this row failing rather than an ε it may skip.
#[test]
fn the_readers_gate_runs_one_certificate_and_hands_it_back() {
    let tol = Tol::witness();
    let native = arc_prism(1.0e5 * tol.get().eps);
    let text = step_export::step_string(&native, &step_export::StepOptions::default(), tol)
        .expect("the rational-walled prism exports");

    let mut imported = None;
    let through_import = quad_verdicts(|| {
        imported = Some(import_step(&text, &ImportOptions::default(), tol));
    });
    let (body, enclosure) = match imported.expect("the closure ran") {
        Ok(StepImport::Solid {
            body, enclosure, ..
        }) => (body, enclosure),
        other => panic!(
            "ONE CERTIFICATE: the ε-scaled rational prism is a single-solid file that \
             certifies at every ε row, so the reader must ship a Solid: {other:?}"
        ),
    };

    let mut measured = None;
    let one = quad_verdicts(|| measured = Some(topo::mass_properties(&body, tol)));
    let measured = measured.expect("the closure ran").expect(
        "IDENTITY: the body the reader shipped passed the reader's own tier-3′ gate, so \
         its enclosure certifies by construction",
    );

    assert!(
        one > 0,
        "ONE CERTIFICATE: the counter must see this body's quadrature at all — {one} \
         quadrature verdicts for one measurement"
    );
    assert_eq!(
        through_import, one,
        "ONE CERTIFICATE: the whole import must cost ONE certificate — the aggregate \
         tier-3′ gate's — and hand it back; {through_import} verdicts against the {one} \
         of a single measurement means the reader computed the enclosure a second time \
         behind a field that says it did not"
    );
    assert!(
        measured.volume_pad > 0.0,
        "ONE CERTIFICATE: this row's claim is about the QUADRATURE lane, so the imported \
         body must carry a certified enclosure: pad {}",
        measured.volume_pad
    );
    assert_eq!(
        bits(&enclosure),
        bits(&measured),
        "IDENTITY: the reader's `enclosure` must BE the gate's certificate, in all four \
         fields: field {enclosure:?} vs measurement {measured:?}"
    );
}
