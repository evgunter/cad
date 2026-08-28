//! **PCURVE P-1b, reviewer R2's independent consumer probes** at the
//! `geom-brep` door — the layer where the collapsed description is
//! certified, so the claims about the ONE conventional form, the one
//! meter, the transience conversion and the authority record can all
//! be measured without a body in the way.
//!
//! These rows are deliberately NOT a re-run of the unit's own: each
//! one states a claim the PR makes in prose and measures it from
//! outside, and several of them measure WHERE a boundary falls rather
//! than asserting that it exists.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom::Curve3;
use geom::Surface;
use geom_brep::{
    CertCheck, CertifyError, EdgeAuthority, EdgeCurve, EdgeCurveSpec, EdgeDescription,
    EdgeDescriptionSpec, MappedCurve, SurfaceKey,
};
use geom_core::{Band, Point3, Tol, Vec3};
use slotmap::SlotMap;

fn band() -> Band {
    let tol = Tol::witness().get();
    Band::new(tol.eps, tol.k * tol.eps).unwrap()
}

fn p(x: f64, y: f64, z: f64) -> Point3<f64> {
    Point3::new(x, y, z)
}

/// A surface table and its resolver (the injected lookup the door
/// takes — keys never resolve inside `geom-brep`).
fn table(surfs: Vec<Surface<f64>>) -> (Vec<SurfaceKey>, impl Fn(SurfaceKey) -> Option<Surface<f64>>)
{
    let mut map: SlotMap<SurfaceKey, Surface<f64>> = SlotMap::with_key();
    let keys: Vec<SurfaceKey> = surfs.into_iter().map(|s| map.insert(s)).collect();
    (keys, move |k| map.get(k).cloned())
}

fn unit_cylinder() -> Surface<f64> {
    Surface::Cylinder {
        origin: Point3::origin(),
        axis: Vec3::unit_z(),
        radius: 1.0,
        u_ref: Vec3::unit_x(),
    }
}

/// A chord of the unit cylinder subtending `half_angle` either side of
/// the `x` axis at `z = 0`: its two endpoints are ON the cylinder and
/// every interior point is strictly inside it (sagitta
/// `1 − cos(half_angle)`).
fn secant_chord(half_angle: f64) -> (Point3<f64>, Point3<f64>) {
    let (s, c) = half_angle.sin_cos();
    (p(c, -s, 0.0), p(c, s, 0.0))
}

// ---------------------------------------------------------------
// SECANTS — the boundary the unit claims (#1116 / the fillet strut)
// ---------------------------------------------------------------

/// **R2-G1.** A straight chord between two points of a CURVED chart is
/// refused when it is described as an image of that chart — and the
/// refusal is measured, not read off its text: the row records WHICH
/// door refuses (the mint's, or the meter's).
///
/// The unit's #1116 argument is "no chart image of the surface its two
/// faces share describes it". This row is that sentence's consumer
/// test.
#[test]
fn r2_a_secant_of_a_cylinder_is_refused_as_a_chart_image_of_it() {
    let (keys, lookup) = table(vec![unit_cylinder()]);
    let (q0, q1) = secant_chord(std::f64::consts::FRAC_PI_3);
    let len = q0.distance(q1);
    let err = EdgeCurve::certify(
        EdgeCurveSpec {
            description: EdgeDescriptionSpec::chart(keys[0]),
            carrier: Curve3::Line {
                origin: q0,
                dir: (q1 - q0) / len,
            },
            param_start: 0.0,
            param_end: len,
        },
        q0,
        q1,
        &lookup,
        band(),
    )
    .expect_err("a 60-degree secant is half a radius off the wall");
    // The door that refuses is recorded rather than asserted blind:
    // `refusal-text-is-not-cause` cuts both ways, so the row prints
    // the variant it actually got and pins only that it is one of the
    // two loud ones.
    println!("[R2-G1] the secant refused through: {err:?}");
    assert!(
        matches!(
            err,
            CertifyError::ChartImageUnavailable { .. }
                | CertifyError::ResidualExceeded {
                    check: CertCheck::ChartResidual,
                    ..
                }
                | CertifyError::Escalated {
                    check: CertCheck::ChartImage | CertCheck::ChartResidual,
                    ..
                }
        ),
        "a secant must refuse through the chart-image mint or the one meter, not elsewhere:          {err:?}"
    );
}

/// **R2-G2.** The SAME chord, described in a chart that does contain
/// it, certifies. This is the other half of the boundary: the refusal
/// above is about containment, not about chords, and not about the
/// `Line` carrier kind.
#[test]
fn r2_the_same_chord_certifies_in_a_plane_that_contains_it() {
    let (q0, q1) = secant_chord(std::f64::consts::FRAC_PI_3);
    let (keys, lookup) = table(vec![Surface::Plane {
        origin: q0,
        normal: Vec3::unit_z(),
        u_ref: Vec3::unit_x(),
    }]);
    let len = q0.distance(q1);
    let edge = EdgeCurve::certify(
        EdgeCurveSpec {
            description: EdgeDescriptionSpec::chart(keys[0]),
            carrier: Curve3::Line {
                origin: q0,
                dir: (q1 - q0) / len,
            },
            param_start: 0.0,
            param_end: len,
        },
        q0,
        q1,
        &lookup,
        band(),
    )
    .expect("a chord of a plane lies in the plane");
    assert!(
        edge.description().chart().is_some(),
        "the certified form is the ONE conventional arm"
    );
}

/// **R2-G3.** WHERE the secant boundary falls, measured on the meter
/// rather than argued from the kind: a chord whose sagitta is inside
/// the witness tolerance is accepted on the very same cylinder chart
/// that refused the 60-degree one. The refusal is metric.
///
/// Three-outcome honesty: the row prints the sagitta it used and the
/// verdict, so a tolerance change reads as a fact rather than a
/// mystery.
#[test]
fn r2_the_secant_refusal_is_metric_not_categorical() {
    let (keys, lookup) = table(vec![unit_cylinder()]);
    // sagitta = 1 − cos(h) ≈ h²/2; pick h so the sagitta is ~1e-10,
    // three orders inside the witness ε.
    let h = (2.0_f64 * 1e-10).sqrt();
    let (q0, q1) = secant_chord(h);
    let sagitta = 1.0 - h.cos();
    let len = q0.distance(q1);
    let outcome = EdgeCurve::certify(
        EdgeCurveSpec {
            description: EdgeDescriptionSpec::chart(keys[0]),
            carrier: Curve3::Line {
                origin: q0,
                dir: (q1 - q0) / len,
            },
            param_start: 0.0,
            param_end: len,
        },
        q0,
        q1,
        &lookup,
        band(),
    );
    // **(evidence — this row reports, it does not gate.)** Which way it
    // lands is the finding either way: if the chord certifies, the
    // secant boundary is METRIC and "no chart image describes a
    // secant" is a statement about magnitude; if it refuses, the
    // cylinder's chart-image mint rejects every non-meridian,
    // non-section line CATEGORICALLY, before any residual is taken —
    // and then the `#1116` wording "makes ChartResidual escalate" is
    // naming the wrong check as well as the wrong cause.
    println!(
        "[R2-G3] sagitta = {sagitta:e}, outcome = {:?}",
        outcome.as_ref().err()
    );
}

// ---------------------------------------------------------------
// THE TRANSIENCE CONVERSION — `at_rest_in_chart`
// ---------------------------------------------------------------

/// **R2-G4.** The conversion the whole fence rests on: a scaffolding
/// chord that comes to rest in a chart containing it keeps its
/// carrier and interval BITWISE, moves its pushforward into the
/// authority record, and reads as `Declared` afterwards — which is
/// what keeps tier 3's prefer-intrinsic verdicts where they were.
#[test]
fn r2_the_rest_conversion_moves_only_the_description() {
    let (q0, q1) = (p(0.0, 0.0, 0.0), p(1.0, 0.0, 0.0));
    let (keys, lookup) = table(vec![Surface::Plane {
        origin: Point3::origin(),
        normal: Vec3::unit_z(),
        u_ref: Vec3::unit_x(),
    }]);
    let scaffold = EdgeCurveSpec::line_between(q0, q1);
    let (carrier_before, t_before) = (
        scaffold.carrier.clone(),
        (scaffold.param_start, scaffold.param_end),
    );
    let at_rest = scaffold.at_rest_in_chart(keys[0], false);
    assert_eq!(
        (at_rest.param_start, at_rest.param_end),
        t_before,
        "the interval is untouched"
    );
    match (&carrier_before, &at_rest.carrier) {
        (
            Curve3::Line {
                origin: o0,
                dir: d0,
            },
            Curve3::Line {
                origin: o1,
                dir: d1,
            },
        ) => {
            assert!(
                o0.x == o1.x && o0.y == o1.y && o0.z == o1.z,
                "the carrier origin is bitwise untouched"
            );
            assert!(
                d0.x == d1.x && d0.y == d1.y && d0.z == d1.z,
                "the carrier direction is bitwise untouched"
            );
        }
        _ => panic!("the carrier kind changed"),
    }
    let edge = EdgeCurve::certify(at_rest, q0, q1, &lookup, band()).expect("it rests in the plane");
    assert!(matches!(edge.description(), EdgeDescription::Chart(_)));
    assert!(
        edge.authority().is_declared(),
        "the pushforward became the authority record, so prefer-intrinsic still reads Declared"
    );
}

/// **R2-G5 — a fail-QUIET door.** `at_rest_in_chart` is documented as
/// idempotent so a construction "can call this on whatever it built
/// without first asking what that was". Measured, the idempotence is
/// wider than that sentence: on a spec that is ALREADY a chart image
/// the call silently discards BOTH of its arguments — the chart key
/// and the seam obligation — and answers the old description.
///
/// So a construction that names the wrong chart, or that means to
/// state D1's seam obligation on an edge already described without
/// it, gets no error and no change. This row records that as a
/// measured property of the one conversion door the fence depends on.
#[test]
fn r2_the_rest_door_silently_ignores_its_arguments_on_an_at_rest_spec() {
    let (keys, _lookup) = table(vec![unit_cylinder(), unit_cylinder()]);
    let spec = EdgeCurveSpec {
        description: EdgeDescriptionSpec::chart(keys[0]),
        carrier: Curve3::Line {
            origin: p(1.0, 0.0, 0.0),
            dir: Vec3::unit_z(),
        },
        param_start: 0.0,
        param_end: 1.0,
    };
    let restated = spec.at_rest_in_chart(keys[1], true);
    match restated.description {
        EdgeDescriptionSpec::Chart { surface, seam, .. } => {
            assert_eq!(
                surface, keys[0],
                "the door kept the OLD chart and dropped the one it was handed"
            );
            assert!(
                !seam,
                "the door dropped the seam obligation it was handed, silently"
            );
        }
        other => panic!("unexpected: {other:?}"),
    }
}

// ---------------------------------------------------------------
// THE AUTHORITY RECORD — can `is_declared()` be flipped quietly?
// ---------------------------------------------------------------

/// **R2-G6.** Restating a certified edge (`restated_spec`, the one
/// door consumers rebuild specs through) preserves the declaration on
/// every arm, and re-certifying the restatement reads the same
/// `is_declared`.
#[test]
fn r2_restating_a_certified_edge_preserves_is_declared() {
    let (keys, lookup) = table(vec![Surface::Plane {
        origin: Point3::origin(),
        normal: Vec3::unit_z(),
        u_ref: Vec3::unit_x(),
    }]);
    let (q0, q1) = (p(0.0, 0.0, 0.0), p(1.0, 0.0, 0.0));
    let declared = EdgeCurve::certify(
        EdgeCurveSpec::line_between(q0, q1).at_rest_in_chart(keys[0], false),
        q0,
        q1,
        &lookup,
        band(),
    )
    .expect("rests in the plane");
    assert!(declared.authority().is_declared());
    let again = EdgeCurve::certify(declared.restated_spec(), q0, q1, &lookup, band())
        .expect("a restatement re-certifies");
    assert!(
        again.authority().is_declared(),
        "a restatement must not flip the authority record"
    );
    // And the derived twin stays derived.
    let derived = EdgeCurve::certify(
        EdgeCurveSpec {
            description: EdgeDescriptionSpec::chart(keys[0]),
            carrier: Curve3::Line {
                origin: q0,
                dir: Vec3::unit_x(),
            },
            param_start: 0.0,
            param_end: 1.0,
        },
        q0,
        q1,
        &lookup,
        band(),
    )
    .expect("rests in the plane");
    assert!(!derived.authority().is_declared());
    assert!(
        !EdgeCurve::certify(derived.restated_spec(), q0, q1, &lookup, band())
            .expect("re-certifies")
            .authority()
            .is_declared()
    );
}

/// **R2-G7.** Splitting a declared chart image leaves BOTH children
/// declared — the prefer-intrinsic verdict cannot be shed by cutting
/// an edge in half.
#[test]
fn r2_splitting_a_declared_chart_image_keeps_both_children_declared() {
    let (keys, lookup) = table(vec![Surface::Plane {
        origin: Point3::origin(),
        normal: Vec3::unit_z(),
        u_ref: Vec3::unit_x(),
    }]);
    let (q0, q1) = (p(0.0, 0.0, 0.0), p(2.0, 0.0, 0.0));
    let parent = EdgeCurve::certify(
        EdgeCurveSpec::line_between(q0, q1).at_rest_in_chart(keys[0], false),
        q0,
        q1,
        &lookup,
        band(),
    )
    .expect("rests in the plane");
    let mid = p(1.0, 0.0, 0.0);
    let (a, b) = parent.split_specs(1.0);
    for (spec, s, e) in [(a, q0, mid), (b, mid, q1)] {
        let child = EdgeCurve::certify(spec, s, e, &lookup, band()).expect("the child certifies");
        assert!(
            child.authority().is_declared(),
            "a split child kept its parent's declaration"
        );
    }
}

// ---------------------------------------------------------------
// ITEM 6 — the interval checks' sentinel
// ---------------------------------------------------------------

/// **R2-G8.** `ParamSpan` refusals no longer print a sample index they
/// never had. The row reads the rendered words, which is what the
/// `step-import` pin the spec named actually reads.
#[test]
fn r2_a_param_span_refusal_says_it_is_not_a_sampled_check() {
    let (keys, lookup) = table(vec![Surface::Plane {
        origin: Point3::origin(),
        normal: Vec3::unit_z(),
        u_ref: Vec3::unit_x(),
    }]);
    let q = p(0.0, 0.0, 0.0);
    let err = EdgeCurve::certify(
        EdgeCurveSpec {
            description: EdgeDescriptionSpec::chart(keys[0]),
            carrier: Curve3::Line {
                origin: q,
                dir: Vec3::unit_x(),
            },
            // A decreasing interval: refused by the ParamSpan gate.
            param_start: 1.0,
            param_end: 0.0,
        },
        q,
        p(1.0, 0.0, 0.0),
        &lookup,
        band(),
    )
    .expect_err("a decreasing interval is refused");
    let rendered = format!("{err}");
    println!("[R2-G8] {rendered}");
    assert!(
        !rendered.contains("4294967295"),
        "the sentinel must never render as its numeric value: {rendered}"
    );
}

// ---------------------------------------------------------------
// THE ONE CONVENTIONAL FORM — no scaffolding survives certification
// as anything else, and a scaffold is still legal at the door
// ---------------------------------------------------------------

/// **R2-G9.** The scaffolding door is still open with NO surface in
/// existence — the half of the fence that must not have been closed
/// by accident.
#[test]
fn r2_the_scaffolding_door_certifies_with_no_surface_at_all() {
    let (_keys, lookup) = table(vec![]);
    let (q0, q1) = (p(0.0, 0.0, 0.0), p(0.0, 0.0, 3.0));
    let edge = EdgeCurve::certify(
        EdgeCurveSpec::line_between(q0, q1),
        q0,
        q1,
        &lookup,
        band(),
    )
    .expect("scaffolding needs no chart");
    assert!(matches!(edge.description(), EdgeDescription::Scaffold(_)));
    assert!(
        matches!(edge.authority(), EdgeAuthority::Declared(MappedCurve::ExtrudedPoint { .. })),
        "a scaffold's pushforward IS its declaration"
    );
}

/// **R2-G10 — the `#1116` diagnosis, tested against its own fixture.**
///
/// The unit declines the spec's ordered fillet conversions and files
/// #1116 with this recorded cause: *"a strut is a straight chord
/// between a support boundary vertex and its foot ... on a curved one
/// it is a SECANT ... Stating one makes `ChartResidual` escalate at
/// ε = 1e-6 on `die_fillet`."*
///
/// `die_fillet` is a UNIT CUBE with all twelve edges blended in one
/// call (`crates/editor-core/tests/corpus/die_fillet.rs`), so every
/// support face in that run is a PLANE. This row builds that fixture's
/// strut in the small: a chord between two points of the `z = 0` face
/// at the die's own radius, described as an image in that face's
/// chart, at the ε the PR names.
///
/// It certifies with residual exactly zero. So on `die_fillet` the
/// secant mechanism cannot be what escalated — whatever did was never
/// isolated. (This row falsifies the stated cause; it does not claim
/// to supply the real one.)
#[test]
fn r2_a_die_scale_strut_chord_on_a_planar_support_certifies_exactly() {
    // The die: L = 1, r = 0.15 (`m5_pr12_die_body.rs`'s constants).
    let (l, r) = (1.0_f64, 0.15_f64);
    let _ = l;
    let (keys, lookup) = table(vec![Surface::Plane {
        origin: Point3::origin(),
        normal: Vec3::unit_z(),
        u_ref: Vec3::unit_x(),
    }]);
    // A support boundary vertex of the cube's `z = 0` face and the
    // foot of its station, both in that face.
    let (q0, q1) = (p(0.0, 0.0, 0.0), p(r, r, 0.0));
    let edge = EdgeCurve::certify(
        EdgeCurveSpec::line_between(q0, q1).at_rest_in_chart(keys[0], false),
        q0,
        q1,
        &lookup,
        // The ε the PR names for the escalation.
        Band::new(1e-6, 1e-6 * Tol::witness().get().k).unwrap(),
    )
    .expect("a chord of a plane is an image of that plane's chart");
    let residual = edge.certificate().max_residual;
    println!("[R2-G10] die-scale planar strut residual = {residual:e}");
    assert!(
        residual < 1e-15,
        "a chord between two points of a plane lies in it — no secant, no residual: {residual:e}"
    );
    assert!(edge.authority().is_declared());
}
