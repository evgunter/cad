//! **An arc tangent to the partner's face at a boolean vertex.**
//!
//! A quarter round (x ∈ [0.5, 3], z ∈ [1, 3], y ∈ [0, 4]) whose bottom
//! face meets its cylindrical wall tangentially along the rim
//! x = 2, z = 1; each cap carries an arc that leaves the rim vertex
//! tangent to z = 1 and rises to z = 2. It sits on a slab whose top is
//! z = 1, in three postures that put the rim vertex at a different kind
//! of boolean vertex:
//!
//! - `wide`: x ∈ [0, 5], y ∈ [−1, 5]. The rim vertex is inside the
//!   slab's top face.
//! - `flush`: x ∈ [0, 2], y ∈ [−1, 5]. The rim vertex lies on the
//!   slab's top edge; the slab's top ends where the arc leaves.
//! - `corner`: x ∈ [0, 2], y ∈ [0, 4]. The rim vertex is a slab corner.
//!
//! The `dip` mirrors the round below its rim, so the arc bends down into
//! the slab; the `tilted dip` turns it about the rim's tangent line, so
//! its wall crosses the slab's top instead of touching it.
//!
//! In every posture the arc's tangent lies in the slab's top plane at
//! the rim vertex, so its side of that plane is its bend's.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::Point3;
use geom_core::{Affine3, Point2, Tol, Vec3};
use profile::{Profile, SketchPlane, test_support::bulge_loop};
use sweep::test_support::finished;
use sweep::{ExtrudeSide, Extrusion, extrude};
use topo::{AtRestBody, BooleanResult, mass_properties};

/// The profile plane: `(u, v)` maps to `(x = v, z = u)`, extruded along
/// `+y` from `y0`.
pub(crate) fn lying_extrude(
    vertices: Vec<(Point2<f64>, f64)>,
    y0: f64,
    depth: f64,
    what: &str,
) -> AtRestBody<f64> {
    let plane = SketchPlane::from_frame(geom_core::RigidMotion::axes_zx(Point3::new(0.0, y0, 0.0)));
    let profile = Profile::new(plane, vec![bulge_loop(vertices)])
        .validate(Tol::witness())
        .unwrap();
    let body = extrude(
        &profile,
        Extrusion::Distance {
            depth,
            side: ExtrudeSide::Along,
        },
        Tol::witness(),
    )
    .unwrap()
    .body;
    finished(what, body, Tol::witness())
}

pub(crate) fn quarter_round() -> AtRestBody<f64> {
    let b90 = (core::f64::consts::PI / 8.0).tan();
    lying_extrude(
        vec![
            (Point2::new(1.0, 0.5), 0.0),
            (Point2::new(1.0, 2.0), -b90),
            (Point2::new(2.0, 3.0), 0.0),
            (Point2::new(3.0, 3.0), 0.0),
            (Point2::new(3.0, 0.5), 0.0),
        ],
        0.0,
        4.0,
        "the quarter round",
    )
}

/// The round's mirror below the rim: the arc leaves the rim vertex
/// tangent to z = 1 and curves DOWN to (3, ·, 0), into a slab under it.
fn dipping_round() -> AtRestBody<f64> {
    tilted_dipping_round(0.0)
}

/// [`dipping_round`] turned by `alpha` about the line y = 0, z = 1 (the
/// rim vertex's tangent line, along x): the arc still leaves the rim
/// vertex along x, tangent to z = 1, and bends down off it, while the
/// wall no longer touches z = 1 there.
fn tilted_dipping_round(alpha: f64) -> AtRestBody<f64> {
    let (sa, ca) = alpha.sin_cos();
    let plane = SketchPlane::from_frame(geom_core::test_support::rigid(
        &Affine3::from_parts(
            geom_core::Mat3::from_cols(
                Vec3::new(0.0, -sa, ca),
                Vec3::unit_x(),
                Vec3::new(0.0, ca, sa),
            ),
            Vec3::new(0.0, sa, 1.0 - ca),
        ),
        geom_core::Tol::witness(),
    ));
    let b90 = (core::f64::consts::PI / 8.0).tan();
    let profile = Profile::new(
        plane,
        vec![bulge_loop(vec![
            (Point2::new(1.0, 0.5), 0.0),
            (Point2::new(1.0, 2.0), b90),
            (Point2::new(0.0, 3.0), 0.0),
            (Point2::new(3.0, 3.0), 0.0),
            (Point2::new(3.0, 0.5), 0.0),
        ])],
    )
    .validate(Tol::witness())
    .unwrap();
    let body = extrude(
        &profile,
        Extrusion::Distance {
            depth: 4.0,
            side: ExtrudeSide::Along,
        },
        Tol::witness(),
    )
    .unwrap()
    .body;
    finished("the dipping round", body, Tol::witness())
}

pub(crate) fn slab(x1: f64, y0: f64, depth: f64) -> AtRestBody<f64> {
    lying_extrude(
        vec![
            (Point2::new(0.0, 0.0), 0.0),
            (Point2::new(1.0, 0.0), 0.0),
            (Point2::new(1.0, x1), 0.0),
            (Point2::new(0.0, x1), 0.0),
        ],
        y0,
        depth,
        "the slab",
    )
}

fn report(what: &str, r: Result<BooleanResult<f64>, topo::BooleanError>, want: Option<f64>) {
    let got = match r {
        Ok(BooleanResult::Empty) => "Empty".to_string(),
        Ok(BooleanResult::Body(b)) => {
            let v = mass_properties(
                &b.body,
                Tol::witness(),
                topo::test_support::pinned_resolution(Tol::witness()),
            )
            .map(|m| m.enclosure.volume);
            format!("Body volume {v:?}")
        }
        Err(e) => format!("Err {e:?}"),
    };
    let want = want.map_or("Empty".to_string(), |w| format!("volume {w}"));
    eprintln!("SIDECODE-FIXTURE {what}: want {want}; got {got}");
}

/// **The wide slab's answers**: the differences are the operands and the intersection is empty. The
/// round's wall touches the slab's top along the rim and lumps whole
/// as a tangent sector at the rim vertex, and the arc it shares with
/// the cap takes the lump's side.
#[test]
fn the_wide_slab_minus_and_meet_the_round() {
    let round = quarter_round();
    let s = slab(5.0, -1.0, 6.0);
    let vol = |b: &AtRestBody<f64>| {
        mass_properties(
            b,
            Tol::witness(),
            topo::test_support::pinned_resolution(Tol::witness()),
        )
        .unwrap()
        .met()
        .expect("the pinned resolution is met")
        .volume
    };
    let (vr, vs) = (vol(&round), vol(&s));
    let body = |what: &str, r: Result<BooleanResult<f64>, topo::BooleanError>| match r {
        Ok(BooleanResult::Body(b)) => vol(&b.body),
        other => panic!("{what}: want a body, got {other:?}"),
    };
    let got = body("slab ∖ round", topo::subtract(&s, &round, Tol::witness()));
    assert!(
        (got - vs).abs() <= 1e-12 * vs,
        "slab ∖ round is the slab: {got} vs {vs}"
    );
    let got = body("round ∖ slab", topo::subtract(&round, &s, Tol::witness()));
    assert!(
        (got - vr).abs() <= 1e-12 * vr,
        "round ∖ slab is the round: {got} vs {vr}"
    );
    let meet = topo::intersect(&s, &round, Tol::witness());
    assert!(
        matches!(meet, Ok(BooleanResult::Empty)),
        "a resting contact encloses no volume: {meet:?}"
    );
}

/// **The tilted dip meets the slab in its closed form.** Turned by `α`
/// about the rim vertex's tangent line, the dipping round's arc still
/// leaves the rim vertex tangent to the slab's top and bends down into
/// it, and nothing else of the round touches the top there: its wall,
/// cap and bottom face cross it. Neither face the arc bounds lumps as a
/// tangent sector, so the arc's side is its own reading. To first
/// order it reads `On`, its corner's germ is wrong, and every op
/// refused at the join with two loose ends. Read by its bend it is
/// `In`, and the ops answer.
///
/// The overlap, in the round's own frame: at each `x` the section is
/// `[0, 4] × [h(x), 3]` in `(y, z)`, turned by `α` about `(0, 1)`, and
/// it lies below `z = 1` where `y·sin α + (z − 1)·cos α < 0`, which for
/// `α > 0` is the triangle `z ∈ [h, 1]`, `y < (1 − z)/tan α` (inside
/// `y ≤ 4` for both turns below). With `h(x) = sqrt(1 − (x − 2)²)` on
/// `[2, 3]`, its area is `(1 − h)²/(2·tan α)`, and its volume
/// `∫₀¹ (1 − sqrt(1 − t²))² dt / (2·tan α) = (5/3 − π/2)/(2·tan α)`.
#[test]
fn a_tilted_dipping_round_meets_the_slab_in_its_closed_form() {
    let s = slab(5.0, -6.0, 12.0);
    let vol = |b: &AtRestBody<f64>| {
        mass_properties(
            b,
            Tol::witness(),
            topo::test_support::pinned_resolution(Tol::witness()),
        )
        .unwrap()
        .met()
        .expect("the pinned resolution is met")
        .volume
    };
    let vs = vol(&s);
    for alpha in [0.3f64, 0.6] {
        let round = tilted_dipping_round(alpha);
        let vr = vol(&round);
        let lens = (5.0 / 3.0 - core::f64::consts::FRAC_PI_2) / (2.0 * alpha.tan());
        for (op, got, want) in [
            (
                "slab ∪ round",
                topo::union(&s, &round, Tol::witness()),
                vs + vr - lens,
            ),
            (
                "slab ∖ round",
                topo::subtract(&s, &round, Tol::witness()),
                vs - lens,
            ),
            (
                "round ∖ slab",
                topo::subtract(&round, &s, Tol::witness()),
                vr - lens,
            ),
            (
                "slab ∩ round",
                topo::intersect(&s, &round, Tol::witness()),
                lens,
            ),
        ] {
            let v = match got {
                Ok(BooleanResult::Body(b)) => vol(&b.body),
                other => panic!("α = {alpha}, {op}: want a body, got {other:?}"),
            };
            // The pieces' volume is integrated over curved faces, not
            // closed form; it agrees with the closed form to within a
            // thousand bands (a millionth of a cubic metre at 1e-9).
            assert!(
                (v - want).abs() <= 1e3 * Tol::witness().eps(),
                "α = {alpha}, {op}: {v} vs the closed form {want}"
            );
        }
    }
}

/// **The flush and corner slabs' answers**: the union is the two
/// operands' sum, the differences are the operands, and the
/// intersection is empty. At each rim vertex the round's rim runs along
/// the slab's top edge, the slab's top and the round's bottom coincide,
/// and the slab's top and the round's wall are tangent along the rim,
/// leaving it on opposite sides, so they meet only along it: the rim is
/// a crossing exactly where the coplanar contact beside it lumps In,
/// which is the union alone.
#[test]
fn the_flush_and_corner_slabs_answer() {
    let round = quarter_round();
    let vol = |b: &AtRestBody<f64>| {
        mass_properties(
            b,
            Tol::witness(),
            topo::test_support::pinned_resolution(Tol::witness()),
        )
        .unwrap()
        .met()
        .expect("the pinned resolution is met")
        .volume
    };
    let vr = vol(&round);
    let body = |what: &str, r: Result<BooleanResult<f64>, topo::BooleanError>| match r {
        Ok(BooleanResult::Body(b)) => vol(&b.body),
        other => panic!("{what}: want a body, got {other:?}"),
    };
    for (posture, s) in [
        ("flush", slab(2.0, -1.0, 6.0)),
        ("corner", slab(2.0, 0.0, 4.0)),
    ] {
        let vs = vol(&s);
        for (op, got, want) in [
            (
                "slab ∪ round",
                topo::union(&s, &round, Tol::witness()),
                vs + vr,
            ),
            (
                "slab ∖ round",
                topo::subtract(&s, &round, Tol::witness()),
                vs,
            ),
            (
                "round ∖ slab",
                topo::subtract(&round, &s, Tol::witness()),
                vr,
            ),
        ] {
            let got = body(&format!("{posture} / {op}"), got);
            assert!(
                (got - want).abs() <= 1e-12 * want,
                "{posture} / {op}: {got} vs {want}"
            );
        }
        let meet = topo::intersect(&s, &round, Tol::witness());
        assert!(
            matches!(meet, Ok(BooleanResult::Empty)),
            "{posture} / slab ∩ round: a resting contact encloses no volume: {meet:?}"
        );
    }
}

/// **The untilted dip, printed.** Its wall is tangent to the slab's top
/// along the rim with the lens between them in both materials; the glue
/// door glues the pair `Tangent`. The differences and the intersection
/// refuse at the run boundary on the round's lumped sectors at the rim
/// vertex
/// (`work/contacthold/a-dipping-round-refuses-at-a-run-boundary-on-its-coplanar-bottom-sector.md`),
/// and the union at the round's own wedge-0 corner, which lies in the
/// slab's bottom face (`the_dip_on_a_deep_slab_unions_into_one_solid`
/// moves it inside). It asserts nothing. Run:
/// `cargo test -p sweep --test all the_untilted_dip -- --ignored --nocapture`
#[test]
#[ignore = "measurement: prints each op's outcome; run with --ignored --nocapture"]
fn the_untilted_dip() {
    let dip = dipping_round();
    let wide = slab(5.0, -1.0, 6.0);
    let (vd, vw) = (
        mass_properties(
            &dip,
            Tol::witness(),
            topo::test_support::pinned_resolution(Tol::witness()),
        )
        .unwrap()
        .met()
        .expect("the pinned resolution is met")
        .volume,
        mass_properties(
            &wide,
            Tol::witness(),
            topo::test_support::pinned_resolution(Tol::witness()),
        )
        .unwrap()
        .met()
        .expect("the pinned resolution is met")
        .volume,
    );
    let lens = 4.0 - core::f64::consts::PI;
    report(
        "dip / slab ∪ round",
        topo::union(&wide, &dip, Tol::witness()),
        Some(vw + vd - lens),
    );
    report(
        "dip / slab ∖ round",
        topo::subtract(&wide, &dip, Tol::witness()),
        Some(vw - lens),
    );
    report(
        "dip / round ∖ slab",
        topo::subtract(&dip, &wide, Tol::witness()),
        Some(vd - lens),
    );
    report(
        "dip / slab ∩ round",
        topo::intersect(&wide, &dip, Tol::witness()),
        Some(lens),
    );
}

/// **The dip on a slab deep enough to hold its whole lens**
/// (z ∈ [−1, 1]): the round's own wedge-0 corner at (x = 3, z = 0) lies
/// inside the slab, so the only junction the ops meet is the rim, where
/// the wall leaves the slab's top tangent and bends into it. The glue
/// door glues the pair `Tangent`, claiming nothing of the materials, and
/// the lump reads the wall `In`: the union is one solid with the rim
/// interior.
///
/// The differences and the intersection are WRONG today: each should
/// answer at its closed form, and each refuses with the kernel-bug kind
/// `ClassificationInvariant` instead, an open defect. The rows below
/// pin that defect so a change to it is seen; they are not the intended
/// answer.
#[test]
fn the_dip_on_a_deep_slab_unions_into_one_solid() {
    let dip = dipping_round();
    let deep = slab_deep(0.0, 5.0, -1.0, 6.0, -1.0);
    let vol = |b: &AtRestBody<f64>| {
        mass_properties(
            b,
            Tol::witness(),
            topo::test_support::pinned_resolution(Tol::witness()),
        )
        .unwrap()
        .met()
        .expect("the pinned resolution is met")
        .volume
    };
    let lens = 4.0 - core::f64::consts::PI;
    let want = vol(&deep) + vol(&dip) - lens;
    let body = match topo::union(&deep, &dip, Tol::witness()) {
        Ok(BooleanResult::Body(b)) => b.body,
        other => panic!("the deep dip, slab ∪ round: want one solid, got {other:?}"),
    };
    let v = vol(&body);
    assert!(
        (v - want).abs() < 1e-9,
        "slab ∪ round: volume {v}, want {want}"
    );
    assert_eq!(body.solids().count(), 1, "slab ∪ round: one solid");
    assert_eq!(
        topo::validate_geometric(&body, Tol::witness()),
        Ok(()),
        "slab ∪ round: tier 3"
    );
    for (what, got) in [
        ("slab ∖ round", topo::subtract(&deep, &dip, Tol::witness())),
        ("round ∖ slab", topo::subtract(&dip, &deep, Tol::witness())),
        ("slab ∩ round", topo::intersect(&deep, &dip, Tol::witness())),
    ] {
        // OPEN DEFECT, not the intended answer: the run boundary on the
        // round's lumped sectors at the rim vertex takes no germ, tracked by
        // `work/contacthold/a-dipping-round-refuses-at-a-run-boundary-on-its-coplanar-bottom-sector.md`.
        // When that row lands, this becomes the closed-form assertion.
        assert!(
            matches!(
                got,
                Err(topo::BooleanError::ClassificationInvariant {
                    what: "a germ sector with both edge bounds on the partner face"
                })
            ),
            "the deep dip, {what}: the dip row's open defect moved, got {got:?}"
        );
    }
}

/// A slab `x ∈ [x0, x1]`, `z ∈ [0, 1]`, `y ∈ [y0, y0 + depth]`.
pub(crate) fn slab_between(x0: f64, x1: f64, y0: f64, depth: f64) -> AtRestBody<f64> {
    lying_extrude(
        vec![
            (Point2::new(0.0, x0), 0.0),
            (Point2::new(1.0, x0), 0.0),
            (Point2::new(1.0, x1), 0.0),
            (Point2::new(0.0, x1), 0.0),
        ],
        y0,
        depth,
        "the slab",
    )
}

/// **The tilted dip off the vertex-on-face pass, printed** (review probe,
/// R1 of PR 4575): its rim vertex on the slab's top EDGE (`x ∈ [2, 5]`)
/// and at a slab CORNER (`y ∈ [0, 6]`), and with the slab beside it
/// (`x ∈ [0, 2]`). Measured at 5ca970782b, `edge` refuses
/// `Join(UnpairedLooseEnds { count: 2 })`, the first-order signature
/// (`work/contacthold/conic-ties-outside-the-vertex-on-face-pass-still-read-first-order.md`);
/// descending in `side_code` too moves it to `ClassificationInvariant`.
/// `corner` and `beside-corner` refuse `CurvedPierceUnsupported`; `beside`
/// answers. It asserts nothing. Run:
/// `cargo test -p sweep --test all review_tilted_dip_off_the_vertex_on_face_pass -- --ignored --nocapture`
#[test]
#[ignore = "measurement: prints each op's outcome; run with --ignored --nocapture"]
fn review_tilted_dip_off_the_vertex_on_face_pass() {
    let vol = |b: &AtRestBody<f64>| {
        mass_properties(
            b,
            Tol::witness(),
            topo::test_support::pinned_resolution(Tol::witness()),
        )
        .unwrap()
        .met()
        .expect("the pinned resolution is met")
        .volume
    };
    for alpha in [0.3f64, 0.6] {
        let round = tilted_dipping_round(alpha);
        let vr = vol(&round);
        let lens = (5.0 / 3.0 - core::f64::consts::FRAC_PI_2) / (2.0 * alpha.tan());
        for (posture, s, meet) in [
            ("edge", slab_between(2.0, 5.0, -6.0, 12.0), lens),
            ("corner", slab_between(2.0, 5.0, 0.0, 6.0), lens),
            ("beside", slab_between(0.0, 2.0, -6.0, 12.0), 0.0),
            ("beside-corner", slab_between(0.0, 2.0, 0.0, 6.0), 0.0),
        ] {
            let vs = vol(&s);
            let tag = |op: &str| format!("α={alpha} {posture} / {op}");
            let w = |v: f64| if v == 0.0 { None } else { Some(v) };
            report(
                &tag("slab ∪ round"),
                topo::union(&s, &round, Tol::witness()),
                Some(vs + vr - meet),
            );
            report(
                &tag("slab ∖ round"),
                topo::subtract(&s, &round, Tol::witness()),
                w(vs - meet),
            );
            report(
                &tag("round ∖ slab"),
                topo::subtract(&round, &s, Tol::witness()),
                w(vr - meet),
            );
            report(
                &tag("slab ∩ round"),
                topo::intersect(&s, &round, Tol::witness()),
                w(meet),
            );
        }
    }
}

/// A slab `x ∈ [x0, x1]`, `z ∈ [z0, 1]`, `y ∈ [y0, y0 + depth]`.
fn slab_deep(x0: f64, x1: f64, y0: f64, depth: f64, z0: f64) -> AtRestBody<f64> {
    lying_extrude(
        vec![
            (Point2::new(z0, x0), 0.0),
            (Point2::new(1.0, x0), 0.0),
            (Point2::new(1.0, x1), 0.0),
            (Point2::new(z0, x1), 0.0),
        ],
        y0,
        depth,
        "the slab",
    )
}

/// [`quarter_round`] (arc bending UP off z = 1) turned by `alpha` about
/// the rim's tangent line y = 0, z = 1.
fn tilted_round(alpha: f64) -> AtRestBody<f64> {
    let (sa, ca) = alpha.sin_cos();
    let plane = SketchPlane::from_frame(geom_core::test_support::rigid(
        &Affine3::from_parts(
            geom_core::Mat3::from_cols(
                Vec3::new(0.0, -sa, ca),
                Vec3::unit_x(),
                Vec3::new(0.0, ca, sa),
            ),
            Vec3::new(0.0, sa, 1.0 - ca),
        ),
        geom_core::Tol::witness(),
    ));
    let b90 = (core::f64::consts::PI / 8.0).tan();
    let profile = Profile::new(
        plane,
        vec![bulge_loop(vec![
            (Point2::new(1.0, 0.5), 0.0),
            (Point2::new(1.0, 2.0), -b90),
            (Point2::new(2.0, 3.0), 0.0),
            (Point2::new(3.0, 3.0), 0.0),
            (Point2::new(3.0, 0.5), 0.0),
        ])],
    )
    .validate(Tol::witness())
    .unwrap();
    let body = extrude(
        &profile,
        Extrusion::Distance {
            depth: 4.0,
            side: ExtrudeSide::Along,
        },
        Tol::witness(),
    )
    .unwrap()
    .body;
    finished("the tilted round", body, Tol::witness())
}

/// **A round tilted so its arc bends OUT of the slab answers at every op**
/// (review row, R1 of PR 4575). The quarter round turned by a negative
/// `α` about the rim's tangent line: its cap arc still leaves the rim
/// vertex tangent to `z = 1` and bends up, out of the slab, while its
/// bottom face and wall dip into a deep slab, so no face the arc bounds
/// lumps and the arc's own reading decides its side. Read to first order
/// (`On`) every op refused `Join(UnpairedLooseEnds { count: 2 })`; read by
/// its bend (`Out`) every op answers. `edge-off` puts the rim vertex on
/// the slab's top edge with the slab under the bottom face only, and
/// answers either way.
///
/// The overlap, in the round's frame `(x, u, w)`: below `z = 1` where
/// `w > (u − 1)/t`, `t = tan|α|`; at each `x` the section
/// `u ∈ [g(x), 1 + 4t]` gives the triangle `max(0, 4t − c)²/(2t)`,
/// `c = g − 1` (`g = 1` on `[0.5, 2]`, `2 − sqrt(1 − (x − 2)²)` on
/// `[2, 3]`), integrated by the midpoint rule on the arc's span.
#[test]
fn a_round_tilted_to_bend_its_arc_out_of_the_slab_answers() {
    let vol = |b: &AtRestBody<f64>| {
        mass_properties(
            b,
            Tol::witness(),
            topo::test_support::pinned_resolution(Tol::witness()),
        )
        .unwrap()
        .met()
        .expect("the pinned resolution is met")
        .volume
    };
    for alpha in [-0.3f64, -0.2] {
        let t = alpha.abs().tan();
        let round = tilted_round(alpha);
        let vr = vol(&round);
        let n = 200_000;
        let h = 1.0 / n as f64;
        let arc_part: f64 = (0..n)
            .map(|i| {
                let s = (i as f64 + 0.5) * h;
                let c = 1.0 - (1.0 - s * s).sqrt();
                (4.0 * t - c).max(0.0).powi(2) / (2.0 * t) * h
            })
            .sum();
        let flat_part = 12.0 * t;
        for (posture, s, meet) in [
            (
                "wide",
                slab_deep(0.0, 5.0, -6.0, 12.0, -3.0),
                flat_part + arc_part,
            ),
            ("edge-off", slab_deep(0.0, 2.0, -6.0, 12.0, -3.0), flat_part),
        ] {
            let vs = vol(&s);
            for (op, got, want) in [
                (
                    "slab ∪ round",
                    topo::union(&s, &round, Tol::witness()),
                    vs + vr - meet,
                ),
                (
                    "slab ∖ round",
                    topo::subtract(&s, &round, Tol::witness()),
                    vs - meet,
                ),
                (
                    "round ∖ slab",
                    topo::subtract(&round, &s, Tol::witness()),
                    vr - meet,
                ),
                (
                    "slab ∩ round",
                    topo::intersect(&s, &round, Tol::witness()),
                    meet,
                ),
            ] {
                let v = match got {
                    Ok(BooleanResult::Body(b)) => vol(&b.body),
                    other => panic!("α = {alpha}, {posture}, {op}: want a body, got {other:?}"),
                };
                assert!(
                    (v - want).abs() <= 1e3 * Tol::witness().eps(),
                    "α = {alpha}, {posture}, {op}: {v} vs {want}"
                );
            }
        }
    }
}
