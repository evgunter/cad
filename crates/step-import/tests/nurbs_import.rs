//! **M7-3 acceptance: NURBS-face import.**
//!
//! The committed non-rational loft (`loft_prism`) runs the FULL row-1
//! and fixed-point obligations through `roundtrip.rs` (it joined
//! `SOLID_FIXTURES` at this unit); this suite carries the rows that
//! are NURBS-specific:
//!
//! - the **surface_sig pin** at body level (spec item 2's trap): the
//!   four distinct walls must land on four distinct surface keys;
//! - the **rational arm** (spec item 5, Arm B): a natively built
//!   `arc_loft` (three walls non-rational, one RATIONAL) exports and
//!   then REFUSES at import with exactly the tier-3 verdict its native
//!   twin carries at rest. Arm B was "import with a typed limitation"
//!   until M7-7 wired the shared at-rest gate (#260 ruling (a)):
//!   `StepImport::Solid` may only carry a body the gate passes, and a
//!   body whose volume the kernel cannot compute is not one — so the
//!   limitation now lands as a typed refusal instead of riding out on
//!   a shipped body. Same verdict, same recourse text, earlier;
//!   the class returns to importing when the rational-wall quadrature
//!   reaches the ambient target on it;
//! - the **description state**: imported seams carry `IsoCurve`,
//!   imported cap rims `MappedCurve` — the native loft's own
//!   description classes, which is what makes one adoption pass a
//!   fixed point of the writer.
//!
//! # Coverage honesty (spec §3)
//!
//! The bodies this suite round-trips are uniformly-spaced lofts by
//! the fixture's own choice, not by refusal
//! — polyline profiles (non-rational, full tier 3) and arc-bearing
//! profiles (rational walls, which since M7-7 refuse at the import
//! gate on the banked QUADRATURE lane, the one still open). That is a statement about the
//! builder and the quadrature, not the reader: the import side reads
//! any file in the written vocabulary, and only the at-rest gate has
//! anything left to say about the rational one.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::common;
use geom_core::Point3;
use profile::SketchPlane;

use common::import_body;
use geom_core::Point2;
use geom_core::Tol;
use profile::test_support::bulge_loop;
use step_import::{ImportOptions, import_step};
use sweep::{Section, loft_body};

/// The arc-bearing profile loft the substrate measured (one bulged
/// side per section → 3 non-rational walls + 1 RATIONAL wall): the
/// writer exports it today with tiers 1/2 valid and a typed tier-3
/// volume refusal — the at-rest state its import must reproduce, and
/// therefore (the gate being shared) the refusal its import must
/// carry.
fn native_arc_loft() -> topo::Body<f64> {
    let arc_section = |s: f64| -> Section {
        let v = |x: f64, y: f64, bulge: f64| (Point2::new(x, y), bulge);
        vec![bulge_loop(vec![
            v(-s, -s, 0.0),
            // tan(π/8) on the +x side: a quarter-circle bulge-out.
            v(s, -s, 0.4142135623730951),
            v(s, s, 0.0),
            v(-s, s, 0.0),
        ])]
    };
    let sections = vec![arc_section(1.0), arc_section(1.25), arc_section(1.0)];
    let places = vec![
        SketchPlane::xy(),
        SketchPlane::xy_at(Point3::new(0.0, 0.0, 1.0)),
        SketchPlane::xy_at(Point3::new(0.0, 0.0, 2.0)),
    ];
    loft_body::<f64>(&sections, &places, 2, Tol::witness())
        .expect("the uniformly-spaced arc loft builds natively")
        .body
}

/// RW2 probe 1's reorder (adopted verbatim): reverse every
/// `#k = ...;` statement inside DATA, gluing a multi-line statement's
/// continuation lines to their opener first so a record is moved whole.
fn reverse_data_section(text: &str) -> String {
    let mut head = Vec::new();
    let mut data: Vec<String> = Vec::new();
    let mut tail = Vec::new();
    let mut in_data = false;
    let mut done = false;
    for line in text.lines() {
        if line.trim() == "DATA;" {
            in_data = true;
            head.push(line.to_owned());
            continue;
        }
        if in_data && line.trim() == "ENDSEC;" {
            in_data = false;
            done = true;
            tail.push(line.to_owned());
            continue;
        }
        if in_data {
            if line.trim_start().starts_with('#') || data.is_empty() {
                data.push(line.to_owned());
            } else {
                let last = data.last_mut().unwrap();
                last.push('\n');
                last.push_str(line);
            }
        } else if done {
            tail.push(line.to_owned());
        } else {
            head.push(line.to_owned());
        }
    }
    assert!(!data.is_empty(), "no DATA section parsed");
    data.reverse();
    format!(
        "{}\n{}\n{}",
        head.join("\n"),
        data.join("\n"),
        tail.join("\n")
    )
}

/// The honest posture of a rational-wall body's mass properties at the
/// run's ε (M8-3), read at the pinned resolution `1024·ε`: met at one ε
/// and a bracket at a tighter one are both correct; nothing else is.
/// `Certified` carries the enclosure — the caller reuses it rather than
/// paying a second rational quadrature, which is the expensive thing in
/// these rows.
///
/// The resolution is compared, not decided, so a refusal panics.
#[derive(Debug)]
enum Posture {
    /// The resolution met.
    Certified(topo::MassProperties<f64>, Vec<topo::test_support::FaceTerm>),
    /// The faces stopped short of `1024·ε`, at the floor or the work
    /// wall, with the bracket they reached.
    Short,
}

fn rational_props_posture(body: &topo::Body<f64>, who: &str) -> Posture {
    // The same walk as `topo::mass_properties`, with each face's own term
    // handed back beside the sum (the round trip compares them face by
    // face).
    let (measured, terms) = topo::test_support::mass_properties_with_face_terms(
        body,
        Tol::witness(),
        topo::test_support::pinned_resolution(Tol::witness()),
    )
    .unwrap_or_else(|err| panic!("{who}: not an honest quadrature posture: {err:?}"));
    match measured.stop {
        topo::Stop::Met => Posture::Certified(measured.enclosure, terms),
        topo::Stop::Floor | topo::Stop::Wall => {
            assert!(
                measured.enclosure.volume_pad > 0.0,
                "{who}: a bracket short of the resolution has a width"
            );
            Posture::Short
        }
        stop => panic!("{who}: a resolution decides nothing: {stop:?}"),
    }
}

/// **THE M8-3 FLIP** of `arc_loft_refuses_at_import_with_the_native_bodys_verdict`,
/// executed in full.
///
/// The row's retirement condition was the banked rational patch flux,
/// and M8-3 retires it on both sides: the native arc loft is tier-3
/// VALID with a certified volume, and the round trip imports
/// FIRST-CLASS — the arc rim mints (`Pcurve::IsoArc`) from either
/// mapped description form, so the reader reaches the same state the
/// builder does.
///
/// The writer/reader symmetry the row was written for is literal:
/// `StepImport::Solid` may only carry a body the shared at-rest gate
/// passes, so an `Ok` here IS the imported body's tier-3 verdict.
///
/// **The rational parse arm is still pinned BY this row**: a reader
/// that dropped the `RATIONAL_B_SPLINE_SURFACE` weights would produce
/// a non-rational body of a DIFFERENT volume, and the round-trip
/// volume agreement below is what catches that. (The non-rational
/// side's positive control is `loft_prism`.)
///
/// **ε posture.** Read at `1024·ε`, so at a tight enough ε the honest
/// outcome is a bracket short of it on both sides. Both outcomes are
/// pinned; neither is widened.
///
/// # Adopted arms
///
/// This row is the single home for the rational arc-loft body's
/// disposition. Two rows that built a CHARACTER-IDENTICAL
/// `native_arc_loft` and quadratured it again were merged in here;
/// each rational quadrature is the expensive thing in this class.
/// Authorship is kept, which is this project's convention for
/// adopted review probes (M7-1's "adopted BY MERGE with authorship
/// kept"):
///
/// * **the reversed-DATA arm is RW2 probe 1's** — the blinded review
///   of PR #353 (`rw2_probes.rs`), which re-imported the same export
///   from a file whose every DATA statement is REVERSED. The reader's
///   fixed-point discipline must not depend on entity order, so the
///   reordered file must import to the SAME BODY. It reuses `text`
///   rather than re-exporting, and closes on body identity — the
///   re-exported bytes plus every edge's certified description —
///   rather than on a third rational quadrature. See the arm itself
///   for why that is a strengthening and what it measured;
/// * **the refusal arm's structural match is review F1's positive
///   control's** (`review_probes_m7_3::probe_arm_b_true_arc_rim_
///   positive_control`, M7-3 review, adopted by merge): the import
///   succeeds and its certificate measures to the resolution or to a
///   definitely positive bracket — no validity verdict, and no refusal.
///   Because the import's measurement is the SAME quadrature the native
///   side runs, this also pins the native posture RW2 asserted
///   directly: a native escalation would arrive here as a refusal and
///   fail this row.
#[test]
fn arc_loft_natively_computes_its_rational_volume() {
    let native = native_arc_loft();
    assert_eq!(topo::validate(&native), Ok(()), "native tier 1");
    assert_eq!(topo::validate_closed(&native), Ok(()), "native tier 2");
    let eps = geom_core::Tol::witness().get().eps;
    // Computed ONCE on the native side. A rational quadrature is the
    // expensive thing in this row and each one costs ~8.5 s (measured
    // 2026-08-22, 4-vCPU box, CI's opt-2 env — CI's 2-vCPU runner
    // differs), so the count is a design constraint here, not a detail.
    //
    // WHAT THIS ROW ACTUALLY PAYS, so the next reader does not have to
    // re-measure it: (1) this native quadrature; (2) the at-rest gate
    // INSIDE `import_step` below — the tier-3′ door, which computes
    // the imported body's enclosure to decide its +V invariant and
    // hands it back as `StepImport::Solid`'s `enclosure`, which the
    // assertions below read; (3) the gate inside the
    // reversed-DATA re-import; (4) the imported body's own measurement,
    // through `topo::test_support::mass_properties_with_face_terms`
    // (`mass_properties`' walk with each face's term handed back), which
    // the per-face bit-identity claim below needs: the gate hands back
    // the sum, not the terms. Four, bought for that claim; (1) runs
    // through the same door, so it costs nothing extra. A
    // `mass_properties` on the reordered body was removed on
    // 2026-08-22 in favour of a body-identity comparison; see that
    // arm.
    //
    // Tier 3 consumes exactly this number through its +V invariant, and
    // the sweep suite's
    // `tier3_admits_the_rational_wall_body_and_its_volume_brackets_the_extrusion`
    // pins the verdict itself — paying for it twice here would only buy
    // the same quadrature at the same ε.
    let posture = rational_props_posture(&native, "native");
    let native_props = match &posture {
        Posture::Certified(props, terms) => Some((props, terms)),
        Posture::Short => None,
    };
    let certified = native_props.is_some();
    if let Some((want, _)) = native_props {
        assert!(
            want.volume > 12.0 && want.volume < 13.0,
            "native arc-loft volume: {}",
            want.volume,
        );
        // The pad ceiling, pinned separately (the SKINFIT two-assertion
        // shape) so a loosening enclosure cannot absorb the band above.
        // Keyed to ε, never widened: the schedule is fixed, so the pad
        // is what that schedule achieves against the run's own target.
        let ceiling = 2.0 * 1024.0 * eps;
        assert!(
            want.volume_pad < ceiling,
            "native volume pad ceiling: {} vs {ceiling} (measured 1.4503e-6 at ε=1e-9)",
            want.volume_pad,
        );
        println!(
            "M8-3 arc loft @ eps={eps:e}: Certified, {} ± {}",
            want.volume, want.volume_pad
        );
    } else {
        println!("M8-3 arc loft @ eps={eps:e}: {posture:?} (the fixed schedule's honest frontier)");
    }

    // ---- The ROUND TRIP, which the bank used to block entirely. ----
    let text = step_export::step_string(
        &crate::common::at_rest(&native),
        &step_export::StepOptions::default(),
        Tol::witness(),
    )
    .expect("the writer exports the rational-walled body");
    match import_step(&text, &ImportOptions::default(), Tol::witness()) {
        Ok(step_import::StepImport::Solid { body, .. }) => {
            // The honest frontier, reached through the import's own
            // certificate instead of the native one. The gate ADMITS the
            // body — check 7 decides a sign, and this body's is definite
            // — and the import SUCCEEDS (DESIGN import step 4: the reader
            // holds no opinion of its own about which admitted bodies
            // ship). Where the faces stop short of the resolution the
            // measurement is the bracket they reached.
            let measured = body
                .measure(topo::test_support::pinned_resolution(Tol::witness()))
                .unwrap_or_else(|err| panic!("the import's certificate measures: {err:?}"));
            if measured.stop != topo::Stop::Met {
                assert!(
                    !certified,
                    "the import may only stop short where the native body's flux also did: \
                     {measured:?}"
                );
                let bracket = measured.enclosure.enclosure();
                assert!(
                    bracket.volume_lo > 0.0,
                    "the bracket is definitely positive: {bracket:?}"
                );
                return;
            }
            let enclosure = measured.enclosure;
            assert!(
                certified,
                "the import's certificate meets the resolution only where the native \
                 body's own flux does"
            );
            // The gate's own certificate of this body, measured on, not a
            // second quadrature.
            let got = enclosure;
            let (want, want_terms) = native_props.expect("the native side certified");
            // **BIT identity where it holds, ulp identity where it
            // cannot** (R1 MINOR-2, re-stated for
            // `work/exch/arc-loft-round-trip-volume-differs-by-face-order.md`).
            // Overlap is what soundness needs, but it is not what actually
            // happens, and asserting the weaker fact would let a real
            // divergence hide inside a pad. What the reader reproduces is
            // every FACE: the same chart and trim, so each face's
            // quadrature runs on the same bits and returns the same flux.
            // What it does not reproduce is the face ARENA order: its
            // Euler-operator assembly allocates faces in its own order,
            // and the body fold sums the face terms in plain `f64` in
            // arena order (`topo::props`' `fold_runs`, D9 addendum idiom
            // 2's sequential fold), and an `f64` sum rounds differently in
            // another order. Whether the fold should be order-independent
            // is FLUX's design row
            // `the-whole-body-property-fold-depends-on-face-order`, closed by PR 5402.
            // So:
            //
            // * the import's gate enclosure IS the imported body's own
            //   measurement, bit for bit (same body, same order);
            let (imported, imported_terms) = topo::test_support::mass_properties_with_face_terms(
                body.body(),
                Tol::witness(),
                topo::test_support::pinned_resolution(Tol::witness()),
            )
            .expect("the imported body measures where its gate certified");
            assert_eq!(
                (got.volume.to_bits(), got.volume_pad.to_bits()),
                (
                    imported.enclosure.volume.to_bits(),
                    imported.enclosure.volume_pad.to_bits()
                ),
                "the gate's enclosure must BE the imported body's measurement: \
                 gate {} ± {}, measured {} ± {}",
                got.volume,
                got.volume_pad,
                imported.enclosure.volume,
                imported.enclosure.volume_pad,
            );
            // * every face's flux term and half-width, native against
            //   imported, bit for bit: the same multiset of terms, each
            //   face matched by its bits rather than its key (the keys
            //   are the two arenas' own);
            let bits = |terms: &[topo::test_support::FaceTerm]| {
                let mut bits: Vec<(u64, u64)> = terms
                    .iter()
                    .map(|&(_, flux, pad)| (flux.to_bits(), pad.to_bits()))
                    .collect();
                bits.sort_unstable();
                bits
            };
            assert_eq!(
                bits(want_terms),
                bits(&imported_terms),
                "the round trip must reproduce every face's flux bit for bit: \
                 native {want_terms:?}, imported {imported_terms:?}"
            );
            // * native against imported bodies, the same terms summed in
            //   two orders (D9: a body's storage is part of the input, so
            //   the bits may differ): the volumes, and the pads, agree to
            //   within the gap two orders of these terms may round apart,
            //   computed from the terms (`topo::test_support::fold_order_bound`:
            //   `2γₙ₋₁·Σ|tᵢ|/3` and the third's own rounding). An ulp count
            //   of the volume is the wrong shape for signed terms: a body
            //   far from the origin has terms far larger than its volume.
            //   Measured: 0 ulps before NURBS #4553 moved the rational
            //   wall's flux bits, 1 at ε = 1e-6 and 2 at 1e-9 after it,
            //   and 1 and 0 since the skin computes in `T` (PR 4726).
            let (volume_bound, pad_bound) = topo::test_support::fold_order_bound(want_terms);
            assert!(
                got.volume > 0.0
                    && want.volume > 0.0
                    && (got.volume - want.volume).abs() <= volume_bound,
                "the round trip must reproduce the volume to within its fold orders' \
                 rounding ({volume_bound:e}): imported {} ± {}, native {} ± {}",
                got.volume,
                got.volume_pad,
                want.volume,
                want.volume_pad,
            );
            assert!(
                (got.volume_pad - want.volume_pad).abs() <= pad_bound,
                "and the certified pad to within its own ({pad_bound:e}): imported {}, native {}",
                got.volume_pad,
                want.volume_pad,
            );
            println!(
                "M8-3 arc loft round trip @ eps={eps:e}: FIRST-CLASS, {} ± {}",
                got.volume, got.volume_pad
            );

            // ---- RW2 probe 1's reversed-DATA arm (adopted). ----
            // INVARIANT: entity ORDER is not information. The reader
            // resolves references, so a file whose DATA statements are
            // reversed states the same body — and the whole body, not
            // just one number off it.
            //
            // **It compares BODIES, not volumes, and that is a
            // deliberate strengthening taken for its cost.** Until
            // 2026-08-22 this arm closed by calling `mass_properties`
            // on the reordered body and comparing two f64s against
            // `got`. MEASURED (4-vCPU box, CI's opt-2 env, solo): that
            // call was 8.4 s of this row's 42.5 s — a THIRD full
            // rational quadrature, on top of the native one and the
            // imported one. The arm's own doc comment said it "reuses
            // the export and the native enclosure computed above rather
            // than recomputing either"; the export half was true and
            // the enclosure half was not.
            //
            // What replaces it is finer, not weaker. `mass_properties`
            // is a deterministic function of the body, so equal bodies
            // give equal volumes; the two comparisons below pin
            // everything that function reads, at ~1 ms:
            //
            // * **the re-exported STEP text**, byte for byte — every
            //   surface's control points, weights and knots, every
            //   face's orientation, every loop, every vertex, and the
            //   entity numbering the writer derives from the body's own
            //   traversal. (The writer's timestamp is a fixed string,
            //   never a wall-clock read, so this is deterministic.)
            // * **every edge's certified description**, which the
            //   writer does NOT emit (there are no `PCURVE` records in
            //   our dialect) and which the face quadrature's trim
            //   reads. Without this the comparison would have a blind
            //   spot exactly where this body is interesting — the arc
            //   rim's `Pcurve::IsoArc` minting.
            //
            // A volume comparison could only ever fail on the one f64
            // these two disagree about; these fail on any disagreement
            // at all, and name it.
            let reordered = reverse_data_section(&text);
            assert_ne!(reordered, text, "the DATA section really was reordered");
            let again = match import_step(&reordered, &ImportOptions::default(), Tol::witness()) {
                Ok(step_import::StepImport::Solid { body: again, .. }) => again,
                other => panic!(
                    "reversed-DATA: the reader's fixed point must not depend on entity \
                     order — the as-written file imported first-class, got {other:?}"
                ),
            };
            let rewrite = |b: &topo::Body<f64>, who: &str| {
                step_export::step_string(
                    &crate::common::at_rest(b),
                    &step_export::StepOptions::default(),
                    Tol::witness(),
                )
                .unwrap_or_else(|e| panic!("re-exporting the {who} body: {e}"))
            };
            assert_eq!(
                rewrite(again.body(), "reversed-DATA"),
                rewrite(body.body(), "as-written"),
                "a reordered file must import to the SAME BODY, and it re-exports \
                 differently — entity order became information somewhere in the reader"
            );
            // Sorted, because a permutation of arena keys is not a
            // defect: what must agree is the multiset of descriptions
            // the body carries, and the re-export above already pins
            // the pairing of geometry to topology.
            let descriptions = |b: &topo::Body<f64>| {
                let mut all: Vec<String> = b
                    .edges()
                    .map(|(_, e)| format!("{:?}", b.get_curve_geom(e.curve)))
                    .collect();
                all.sort();
                all
            };
            assert_eq!(
                descriptions(again.body()),
                descriptions(body.body()),
                "a reordered file must mint the SAME edge descriptions — this is the \
                 half the re-export cannot see, and the arc rim's IsoArc lives in it"
            );
            println!(
                "RW2 probe 1 (adopted): reversed-DATA reimport is body-identical \
                 (re-export bytes + edge descriptions), volume {}",
                got.volume
            );
        }
        other => panic!("no other round-trip posture is pinned: {other:?}"),
    }
}

/// **Spec §2 row 5 — the surface_sig pin at body level.** The
/// committed loft_prism's four walls are four DISTINCT
/// `B_SPLINE_SURFACE_WITH_KNOTS` records; with the caps that is 6
/// distinct surface keys. The pre-M7-3 `surface_sig` Nurbs arm
/// (`vec![5u64]`) would have collapsed all four walls onto ONE shared
/// key (3 keys total) — a body that assembles, but with every wall
/// claiming the same surface: the silent-wrong-body class this pin
/// exists to keep dead. (The unit pin on `surface_sig` itself lives
/// in `adopt.rs`'s test module.)
#[test]
fn loft_prism_walls_get_distinct_surface_keys() {
    let (body, _) = import_body("loft_prism");
    let keys: std::collections::BTreeSet<_> = body.faces().map(|(_, f)| f.surface).collect();
    assert_eq!(
        keys.len(),
        6,
        "4 distinct NURBS walls + 2 caps = 6 distinct surface keys"
    );
}

/// **The description state, by the distinction that matters**: which
/// CHART each image is drawn in, and which PAIR each `Intersection` is
/// certified against. Counting description classes alone would read
/// `(4, 8)` and could not tell the three populations apart.
///
/// - The four wall–wall seams are images in a **spline wall's** own
///   chart — each seam has at least one stays-NURBS wall beside it and
///   its carrier bitwise-matches that wall's boundary column (the
///   native at-rest preference, undisturbed by the neighbour's
///   promotion).
/// - The four rims on the stays-NURBS walls are cap-plane × spline-wall
///   `Intersection`s: the plane × NURBS certificate takes their line
///   carriers, and the ladder offers that rung first. They were images
///   in the cap PLANE while that certificate took a spline carrier
///   only, and the conventional rung took them.
/// - The four rims on the PROMOTED walls are honest cap-plane ×
///   wall-plane `Intersection`s — a strictly stronger description
///   (certified against both surfaces) that promotion unlocked; the
///   ruling accepts the divergence and this row still enumerates it.
///
/// So the split is 4 / 4 / 4 by the chart each seam is drawn in or
/// the PAIR each rim is certified against, and a regression that
/// merged two of the populations would still be caught.
///
/// **The principle, because the alternative was available and worse.**
/// Re-baselining this row to `(8, 0, 4)` would have kept its SHAPE and
/// thrown away its TEETH: every body in which the cap rims stopped
/// being described in the cap plane would still read `(8, 0, 4)` and
/// pass. A re-baseline over a retired taxonomy should re-express what
/// the row was actually discriminating, not adjust the number until
/// it matches.
#[test]
fn loft_prism_descriptions_land_in_the_native_classes() {
    let (body, _) = import_body("loft_prism");
    let mut on_wall_chart = 0;
    let mut on_cap_plane = 0;
    let mut on_spline_wall = 0;
    let mut on_promoted_wall = 0;
    for (_, edge) in body.edges() {
        match body.get_curve_geom(edge.curve) {
            Some(topo::CurveGeom::Certified(curve)) => match curve.description() {
                geom_brep::EdgeDescription::Chart(chart) => match body.get_surface(chart.surface) {
                    Some(topo::Surface::Nurbs(_)) => on_wall_chart += 1,
                    Some(topo::Surface::Plane { .. }) => on_cap_plane += 1,
                    other => panic!("a chart image on neither wall nor cap: {other:?}"),
                },
                geom_brep::EdgeDescription::Intersection { pair, .. } => {
                    if pair
                        .keys()
                        .iter()
                        .any(|k| matches!(body.get_surface(*k), Some(topo::Surface::Nurbs(_))))
                    {
                        on_spline_wall += 1;
                    } else {
                        on_promoted_wall += 1;
                    }
                }
                other => panic!("unexpected description class on a loft edge: {other:?}"),
            },
            other => panic!("every imported edge is certified, got: {other:?}"),
        }
    }
    assert_eq!(
        (
            on_wall_chart,
            on_cap_plane,
            on_spline_wall,
            on_promoted_wall
        ),
        (4, 0, 4, 4),
        "4 wall–wall seams as images in a spline wall's chart, 4 stays-NURBS-wall cap \
         rims as plane × spline Intersections, 4 promoted-wall cap rims as plane × plane \
         Intersections"
    );
}

/// **The IsoCurve rung reads the chart's own knot domain** (#327): the
/// column an adopted `IsoCurve` names is `u` at an END of the wall
/// payload's knot domain, never a `[0, 1]` literal.
///
/// The fixture states one stays-NURBS wall (`#111`) on `u ∈ [0, 3]`.
/// That is an affine REPARAMETERIZATION — not one point of the solid
/// moves, every seam carrier stays bitwise that wall's own boundary
/// column, and the bitwise rung still answers for both of its seams —
/// so the only thing that changes is where the chart says its
/// boundaries are. The two adopted descriptions must therefore name
/// `u = 0` and `u = 3`: under a `[0, 1]` literal the second one names
/// an INTERIOR column of this chart, certification refuses it, and the
/// seam falls to another rung (or the import dies).
#[test]
fn an_adopted_iso_column_is_a_knot_domain_end() {
    let orig = common::fixture("loft_prism", "step");
    let text = orig.replace(
        "#111 = B_SPLINE_SURFACE_WITH_KNOTS('', 1, 2, ((#105, #106, #107), (#108, #109, \
         #110)), .UNSPECIFIED., .U., .U., .U., (2, 2), (3, 3), (0.0, 1.0), (0.0, 1.0), \
         .UNSPECIFIED.);",
        "#111 = B_SPLINE_SURFACE_WITH_KNOTS('', 1, 2, ((#105, #106, #107), (#108, #109, \
         #110)), .UNSPECIFIED., .U., .U., .U., (2, 2), (3, 3), (0.0, 3.0), (0.0, 1.0), \
         .UNSPECIFIED.);",
    );
    assert_ne!(text, orig, "the reparameterization applied");
    let step_import::StepImport::Solid { body, .. } =
        import_step(&text, &ImportOptions::default(), Tol::witness())
            .expect("the reparameterized wall imports")
    else {
        panic!("loft_prism is a solid");
    };
    // The wall the reparameterization names, located by its DOMAIN so
    // the row survives arena renumbering.
    let (wall, _) = body
        .body()
        .surfaces()
        .find(|(_, s)| match s {
            geom::Surface::Nurbs(n) => n.knots_u_f64_interim().domain() == (0.0, 3.0),
            _ => false,
        })
        .expect("the reparameterized wall survives the import");
    let mut columns: Vec<f64> = body
        .body()
        .curves()
        .filter_map(|(_, geom)| match geom {
            topo::CurveGeom::Certified(c) => match c.description() {
                geom_brep::EdgeDescription::Chart(cc) if cc.surface == wall => match cc.pcurve {
                    geom_brep::Pcurve::IsoLine { p0, .. } => Some(p0.x),
                    _ => None,
                },
                _ => None,
            },
            _ => None,
        })
        .collect();
    columns.sort_by(f64::total_cmp);
    assert_eq!(
        columns,
        vec![0.0, 3.0],
        "both of the wall's seams adopt on ITS OWN knot-domain ends"
    );
    topo::validate_geometric(body.body(), Tol::witness())
        .expect("and the reparameterized body is valid at rest");
}

/// **The LINE re-export witness (#388).** A native arc prism's four
/// straight ruling seams carry the walls' own boundary-column carriers
/// — the writer states them as degree-1 `B_SPLINE_CURVE_WITH_KNOTS`
/// records, alongside the cap rims it already writes as `LINE`. The
/// import promotes each ruling to `Curve3::Line` (verified by the
/// zero-radius-cylinder certificate, reported as data), so the
/// RE-export writes exact `LINE(...)` where the incoming text spelled
/// a spline — no writer change, the promotion is what moved.
///
/// The fixture is ε-scaled like `tcost_k3`'s prism, so the row
/// compares the same thing at every ε draw.
#[test]
fn promoted_ruling_seams_reexport_as_line() {
    let tol = Tol::witness();
    let s = 1.0e5 * tol.get().eps;
    let native = loft_body::<f64>(
        &[common::arc_section(s), common::arc_section(s)],
        &common::stacked(&[0.0, 1.0], s),
        1,
        tol,
    )
    .expect("the arc prism lofts")
    .body;
    let text1 = step_export::step_string(
        &crate::common::at_rest(&native),
        &step_export::StepOptions::default(),
        tol,
    )
    .expect("the prism exports");
    let bs1 = |text: &str| {
        text.matches("B_SPLINE_CURVE_WITH_KNOTS('', 1,").count()
            + text.matches("B_SPLINE_CURVE_WITH_KNOTS('',1,").count()
    };
    let lines = |text: &str| text.matches("LINE('").count();
    assert_eq!(
        bs1(&text1),
        4,
        "the four ruling seams state degree-1 splines"
    );
    let native_lines = lines(&text1);
    assert!(native_lines > 0, "the cap rims are LINE records already");

    let import = import_step(&text1, &ImportOptions::default(), tol)
        .expect("the ε-scaled prism imports first-class");
    let line_promotions = import
        .curve_promotions()
        .iter()
        .filter(|p| p.kind == step_import::PromotedCurveKind::Line)
        .count();
    assert_eq!(
        line_promotions,
        4,
        "each ruling's carrier is reported promoted: {:?}",
        import.curve_promotions()
    );
    assert_eq!(
        step_import::PromotedCurveKind::Line.to_string(),
        "line",
        "the report names the kind"
    );
    for p in import.curve_promotions() {
        // Not the promotion gate restated (that would be a tautology
        // — the gate IS `residual ≤ ε_in`): this pins the measured
        // MARGIN. The rulings' residuals are the ring-slack floor of
        // an exact chord, ≥2 decades under the gate at every matrix
        // band, so a residual creeping toward the gate reds here
        // before it can flip a promotion.
        assert!(
            p.residual.is_finite() && p.residual <= import.eps_in() / 100.0,
            "a ruling's residual sits decades under the gate: {p:?} vs eps_in {:e}",
            import.eps_in()
        );
    }
    let step_import::StepImport::Solid { body, .. } = import else {
        panic!("the prism is a solid");
    };
    let text2 = step_export::step_string(
        &crate::common::at_rest(body.body()),
        &step_export::StepOptions::default(),
        tol,
    )
    .expect("the re-imported prism re-exports");
    assert_eq!(bs1(&text2), 0, "no degree-1 spline survives the round trip");
    assert_eq!(
        lines(&text2),
        native_lines + 4,
        "the four rulings re-export as exact LINE records"
    );
}
