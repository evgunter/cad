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
//!   the class returns to importing when the banked rational-wall
//!   quadrature lands;
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

mod common;

use common::import_body;
use geom_core::Tol;
use geom_core::{Affine3, Point2, Vec3};
use profile::RawLoop;
use profile::{ProfileLoop, ProfileVertex};
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
        let v = |x: f64, y: f64, bulge: f64| ProfileVertex::new(Point2::new(x, y), bulge);
        vec![ProfileLoop::new(vec![
            v(-s, -s, 0.0),
            // tan(π/8) on the +x side: a quarter-circle bulge-out.
            v(s, -s, 0.4142135623730951),
            v(s, s, 0.0),
            v(-s, s, 0.0),
        ])]
    };
    let sections = vec![arc_section(1.0), arc_section(1.25), arc_section(1.0)];
    let places = vec![
        Affine3::identity(),
        Affine3::translation(Vec3::new(0.0, 0.0, 1.0)),
        Affine3::translation(Vec3::new(0.0, 0.0, 2.0)),
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
/// run's ε (M8-3). The schedule is FIXED (D9) and the target is
/// `1024·ε`, so `Ok` at one ε and a typed budget refusal at a tighter
/// one are both correct; nothing else is. `Ok` carries the certified
/// enclosure — the caller reuses it rather than paying a second
/// rational quadrature, which is the expensive thing in these rows.
fn rational_props_posture(body: &topo::Body<f64>, who: &str) -> Option<topo::MassProperties<f64>> {
    let target = 1024.0 * geom_core::Tol::witness().get().eps;
    match topo::mass_properties(body, Tol::witness()) {
        Ok(props) => Some(props),
        Err(err) => {
            let topo::MassPropsError::Face { source, .. } = &err else {
                panic!("{who}: expected a per-face volume refusal, got: {err:?}");
            };
            match source {
                geom_brep::props::PropsError::QuadratureBudget {
                    width_len,
                    target_len,
                } => {
                    assert!(
                        width_len.is_finite() && width_len > target_len,
                        "{who}: a budget refusal must carry a width that really missed: \
                         {width_len:e} vs {target_len:e}"
                    );
                    assert!(
                        (target_len - target).abs() <= target * 1e-12,
                        "{who}: the refused target must BE 1024·ε: {target_len:e} vs {target:e}"
                    );
                    None
                }
                geom_brep::props::PropsError::Escalated { cause } => {
                    assert_eq!(
                        cause.predicate,
                        Some("props_quad_converged"),
                        "{who}: only the convergence predicate may escalate here: {cause:?}"
                    );
                    None
                }
                other => panic!("{who}: not an honest quadrature posture: {other:?}"),
            }
        }
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
/// **ε posture.** The schedule is fixed (D9) against a `1024·ε`
/// target, so at a tight enough ε the honest outcome is a typed
/// budget refusal on both sides. All three outcomes are pinned; none
/// is widened.
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
///   positive_control`, M7-3 review, adopted by merge). It is
///   STRICTER than the string check beside it: the verdict list must
///   be exactly one `VolumeUncomputable` whose source is a per-face
///   `QuadratureBudget` — no other verdict, no second verdict, and no
///   escalation standing in for a budget. Because the import gate is
///   the SAME `topo::validate_geometric` the native side runs, this
///   also pins the native posture RW2 asserted directly: a native
///   escalation would arrive here as an escalation and fail this
///   match.
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
    // INSIDE `import_step` below, which runs the same
    // `topo::validate_geometric` and its +V invariant; (3) the
    // `mass_properties` call on the imported body, which is what
    // produces the number the bit-identity assertion needs — the gate
    // computes the same flux but hands nothing back, so the two cannot
    // be shared without a kernel API change; (4) the gate inside the
    // reversed-DATA re-import. Four, not one. The FIFTH — a
    // `mass_properties` on the reordered body — was removed on
    // 2026-08-22 in favour of a body-identity comparison; see that arm.
    //
    // Tier 3 consumes exactly this number through its +V invariant, and
    // the sweep suite's
    // `tier3_admits_the_rational_wall_body_and_its_volume_brackets_the_extrusion`
    // pins the verdict itself — paying for it twice here would only buy
    // the same quadrature at the same ε.
    let native_props = rational_props_posture(&native, "native");
    let certified = native_props.is_some();
    if let Some(want) = &native_props {
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
            "native volume pad ceiling: {} vs {ceiling} (M8-3 measured 1.0098754e-6 at ε=1e-9)",
            want.volume_pad,
        );
        println!(
            "M8-3 arc loft @ eps={eps:e}: Certified, {} ± {}",
            want.volume, want.volume_pad
        );
    } else {
        println!("M8-3 arc loft @ eps={eps:e}: Budget (the fixed schedule's honest frontier)");
    }

    // ---- The ROUND TRIP, which the bank used to block entirely. ----
    let text = step_export::step_string(
        &native,
        &step_export::StepOptions::default(),
        Tol::witness(),
    )
    .expect("the writer exports the rational-walled body");
    match import_step(&text, &ImportOptions::default(), Tol::witness()) {
        Ok(step_import::StepImport::Solid { body, .. }) => {
            assert!(
                certified,
                "an imported Solid means the at-rest gate passed, which means the \
                 quadrature certified — it cannot happen at an ε where the native \
                 body's own flux ran out of schedule"
            );
            let got = topo::mass_properties(&body, Tol::witness())
                .expect("imported rational mass properties");
            let want = native_props.as_ref().expect("the native side certified");
            // **BIT identity, not overlap** (R1 MINOR-2). Overlap is
            // what soundness needs — two certified enclosures of one
            // solid must intersect — but it is not what actually
            // happens, and asserting the weaker fact would let a real
            // divergence hide inside a pad. The reader reconstructs the
            // same chart and the same trim, so the same quadrature runs
            // on the same bits: pin THAT, and let a single ulp of drift
            // be a finding.
            assert_eq!(
                got.volume.to_bits(),
                want.volume.to_bits(),
                "the round trip must reproduce the volume BIT-identically: \
                 imported {} ± {}, native {} ± {}",
                got.volume,
                got.volume_pad,
                want.volume,
                want.volume_pad,
            );
            assert_eq!(
                got.volume_pad.to_bits(),
                want.volume_pad.to_bits(),
                "and the same certified pad: imported {}, native {}",
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
                step_export::step_string(b, &step_export::StepOptions::default(), Tol::witness())
                    .unwrap_or_else(|e| panic!("re-exporting the {who} body: {e}"))
            };
            assert_eq!(
                rewrite(&again, "reversed-DATA"),
                rewrite(&body, "as-written"),
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
                descriptions(&again),
                descriptions(&body),
                "a reordered file must mint the SAME edge descriptions — this is the \
                 half the re-export cannot see, and the arc rim's IsoArc lives in it"
            );
            println!(
                "RW2 probe 1 (adopted): reversed-DATA reimport is body-identical \
                 (re-export bytes + edge descriptions), volume {}",
                got.volume
            );
        }
        // The fixed schedule's honest frontier, reached through the
        // import gate instead of the native one.
        Err(step_import::StepImportError::TierInvalid { solid, errors }) => {
            assert!(
                !certified,
                "the import gate may only refuse where the native body's flux also \
                 ran out of schedule: {errors:?}"
            );
            // Review F1's positive control, adopted: the VERDICT LIST
            // itself, not a substring of its prose. Exactly one
            // verdict, and it is the per-face quadrature budget.
            assert!(
                matches!(
                    errors.as_slice(),
                    [topo::ValidationError::VolumeUncomputable {
                        source: topo::MassPropsError::Face {
                            source: geom_brep::props::PropsError::QuadratureBudget { .. },
                            ..
                        },
                    }]
                ),
                "the only surviving verdict is the fixed schedule's budget: {errors:?}"
            );
            let refusal = step_import::StepImportError::TierInvalid { solid, errors };
            let msg = refusal.to_string();
            assert!(
                !msg.contains("RATIONAL patch flux"),
                "the RATIONAL patch flux bank is RETIRED — no refusal may name it: {msg}"
            );
            assert!(
                msg.contains("stalled at a mean boundary displacement"),
                "the only surviving refusal is the quadrature budget, with its number: {msg}"
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

/// **The description state, re-pinned at M7-6** (stage-1 promotion),
/// **re-expressed at PCURVE P-1b over the collapsed taxonomy**.
///
/// The three NUMBERS below are unchanged and so is every fact they
/// state; what changed is that two of the class names they used to
/// count no longer exist. U2 collapsed `IsoCurve` and `MappedCurve`
/// into the one conventional form, so counting them would now read
/// `(8, 0, 4)` — the same body, described the same way, with the
/// row's whole discriminating power thrown away. **This row therefore
/// counts the distinction that survived rather than the names that
/// did not**: which CHART each image is drawn in.
///
/// - The four wall–wall seams are images in a **spline wall's** own
///   chart — each seam has at least one stays-NURBS wall beside it and
///   its carrier bitwise-matches that wall's boundary column (the
///   native at-rest preference, undisturbed by the neighbour's
///   promotion).
/// - The four rims on the stays-NURBS walls are images in the **cap
///   PLANE**, which is the analytic side of their own
///   NURBS-plus-plane pair. Before the collapse these were the
///   conventional `MappedCurve` rung (the Nurbs-adjacency exemption);
///   they say the same thing about the same locus, in the chart the
///   rim actually lies in.
/// - The four rims on the PROMOTED walls are honest cap-plane ×
///   wall-plane `Intersection`s — a strictly stronger description
///   (certified against both surfaces) that promotion unlocked; the
///   ruling accepts the divergence and this row still enumerates it.
///
/// So the split is 4 / 4 / 4 exactly as before, and a regression that
/// merged the two chart populations would still be caught.
#[test]
fn loft_prism_descriptions_land_in_the_native_classes() {
    let (body, _) = import_body("loft_prism");
    let mut on_wall_chart = 0;
    let mut on_cap_plane = 0;
    let mut intersection = 0;
    for (_, edge) in body.edges() {
        match body.get_curve_geom(edge.curve) {
            Some(topo::CurveGeom::Certified(curve)) => match curve.description() {
                geom_brep::EdgeDescription::Chart(chart) => match body.get_surface(chart.surface) {
                    Some(topo::Surface::Nurbs(_)) => on_wall_chart += 1,
                    Some(topo::Surface::Plane { .. }) => on_cap_plane += 1,
                    other => panic!("a chart image on neither wall nor cap: {other:?}"),
                },
                geom_brep::EdgeDescription::Intersection { .. } => intersection += 1,
                other => panic!("unexpected description class on a loft edge: {other:?}"),
            },
            other => panic!("every imported edge is certified, got: {other:?}"),
        }
    }
    assert_eq!(
        (on_wall_chart, on_cap_plane, intersection),
        (4, 4, 4),
        "4 wall–wall seams as images in a spline wall's chart, 4 stays-NURBS-wall cap \
         rims as images in the cap PLANE, 4 promoted-wall cap rims as Intersection"
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
        .surfaces()
        .find(|(_, s)| match s {
            geom::Surface::Nurbs(n) => n.knots_u().domain() == (0.0, 3.0),
            _ => false,
        })
        .expect("the reparameterized wall survives the import");
    let mut columns: Vec<f64> = body
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
    topo::validate_geometric(&body, Tol::witness())
        .expect("and the reparameterized body is valid at rest");
}
