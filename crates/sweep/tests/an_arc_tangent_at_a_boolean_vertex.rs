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

use geom_core::{Affine3, Point2, Tol, Vec3};
use profile::{Profile, SketchPlane, test_support::bulge_loop};
use sweep::test_support::finished;
use sweep::{ExtrudeSide, Extrusion, extrude};
use topo::{
    AtRestBody, Body, BooleanDeclarations, BooleanResult, ContactClass, FacePairDeclaration,
    mass_properties,
};

/// The profile plane: `(u, v)` maps to `(x = v, z = u)`, extruded along
/// `+y` from `y0`.
fn lying_extrude(
    vertices: Vec<(Point2<f64>, f64)>,
    y0: f64,
    depth: f64,
    what: &str,
) -> AtRestBody<f64> {
    let plane = SketchPlane::new(Affine3::from_parts(
        geom_core::Mat3::from_cols(Vec3::unit_z(), Vec3::unit_x(), Vec3::unit_y()),
        Vec3::new(0.0, y0, 0.0),
    ));
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

fn quarter_round() -> AtRestBody<f64> {
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
    let plane = SketchPlane::new(Affine3::from_parts(
        geom_core::Mat3::from_cols(
            Vec3::new(0.0, -sa, ca),
            Vec3::unit_x(),
            Vec3::new(0.0, ca, sa),
        ),
        Vec3::new(0.0, sa, 1.0 - ca),
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

fn slab(x1: f64, y0: f64, depth: f64) -> AtRestBody<f64> {
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

fn plane_face(body: &Body<f64>, z: f64, up: bool) -> topo::FaceKey {
    let hits: Vec<_> = body
        .faces()
        .filter(|(_, f)| match body.get_surface(f.surface) {
            Some(geom::Surface::Plane { origin, normal, .. }) => {
                (origin.z - z).abs() < 1e-12 && normal.z.abs() > 0.5 && (normal.z > 0.5) == up
            }
            _ => false,
        })
        .map(|(k, _)| k)
        .collect();
    let [f] = hits[..] else {
        panic!("expected exactly one z = {z} face (up = {up}), got {hits:?}");
    };
    f
}

fn cyl_face(body: &Body<f64>) -> topo::FaceKey {
    let hits: Vec<_> = body
        .faces()
        .filter(|(_, f)| {
            matches!(
                body.get_surface(f.surface),
                Some(geom::Surface::Cylinder { .. })
            )
        })
        .map(|(k, _)| k)
        .collect();
    let [f] = hits[..] else {
        panic!("expected exactly one wall face, got {hits:?}");
    };
    f
}

fn report(what: &str, r: Result<BooleanResult<f64>, topo::BooleanError>, want: Option<f64>) {
    let got = match r {
        Ok(BooleanResult::Empty) => "Empty".to_string(),
        Ok(BooleanResult::Body(b)) => {
            let v = mass_properties(&b.body, Tol::witness()).map(|m| m.volume);
            format!("Body volume {v:?}")
        }
        Err(e) => format!("Err {e:?}"),
    };
    let want = want.map_or("Empty".to_string(), |w| format!("volume {w}"));
    eprintln!("SIDECODE-FIXTURE {what}: want {want}; got {got}");
}

/// The ops' declarations at `level`: 0 undeclared, 1 the resting faces
/// `Rest`, 2 the wall `Tangent` to the slab top too, each pair oriented
/// as the op reads it (`slab_first`).
fn declarations(
    s: &AtRestBody<f64>,
    round: &AtRestBody<f64>,
    level: u8,
    slab_first: bool,
) -> BooleanDeclarations {
    let mut d = BooleanDeclarations::none();
    let mut push = |fs: topo::FaceKey, fr: topo::FaceKey, class: ContactClass| {
        let (a, b) = if slab_first { (fs, fr) } else { (fr, fs) };
        d.coincident_faces
            .push(FacePairDeclaration::new(a, b, class));
    };
    if level >= 1 {
        push(
            plane_face(s, 1.0, true),
            plane_face(round, 1.0, false),
            ContactClass::Rest,
        );
    }
    if level >= 2 {
        push(
            plane_face(s, 1.0, true),
            cyl_face(round),
            ContactClass::Tangent,
        );
    }
    d
}

const LEVELS: [(&str, u8); 3] = [("undeclared", 0), ("Rest", 1), ("Rest+Tangent", 2)];

/// **The wide slab's answers, at every declaration level**: the
/// differences are the operands and the intersection is empty. The
/// round's wall touches the slab's top along the rim and lumps whole
/// as a tangent sector at the rim vertex, and the arc it shares with
/// the cap takes the lump's side.
#[test]
fn the_wide_slab_minus_and_meet_the_round_at_every_level() {
    let round = quarter_round();
    let s = slab(5.0, -1.0, 6.0);
    let vol = |b: &AtRestBody<f64>| mass_properties(b, Tol::witness()).unwrap().volume;
    let (vr, vs) = (vol(&round), vol(&s));
    let body = |what: &str, r: Result<BooleanResult<f64>, topo::BooleanError>| match r {
        Ok(BooleanResult::Body(b)) => vol(&b.body),
        other => panic!("{what}: want a body, got {other:?}"),
    };
    for (decl, level) in LEVELS {
        let fwd = declarations(&s, &round, level, true);
        let rev = declarations(&s, &round, level, false);
        let got = body(
            &format!("{decl}: slab ∖ round"),
            topo::subtract_with(&s, &round, &fwd, Tol::witness()),
        );
        assert!(
            (got - vs).abs() <= 1e-12 * vs,
            "{decl}: slab ∖ round is the slab: {got} vs {vs}"
        );
        let got = body(
            &format!("{decl}: round ∖ slab"),
            topo::subtract_with(&round, &s, &rev, Tol::witness()),
        );
        assert!(
            (got - vr).abs() <= 1e-12 * vr,
            "{decl}: round ∖ slab is the round: {got} vs {vr}"
        );
        let meet = topo::intersect_with(&s, &round, &fwd, Tol::witness());
        assert!(
            matches!(meet, Ok(BooleanResult::Empty)),
            "{decl}: a resting contact encloses no volume: {meet:?}"
        );
    }
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
    let vol = |b: &AtRestBody<f64>| mass_properties(b, Tol::witness()).unwrap().volume;
    let vs = vol(&s);
    let d = BooleanDeclarations::none();
    for alpha in [0.3f64, 0.6] {
        let round = tilted_dipping_round(alpha);
        let vr = vol(&round);
        let lens = (5.0 / 3.0 - core::f64::consts::FRAC_PI_2) / (2.0 * alpha.tan());
        for (op, got, want) in [
            (
                "slab ∪ round",
                topo::union_with(&s, &round, &d, Tol::witness()),
                vs + vr - lens,
            ),
            (
                "slab ∖ round",
                topo::subtract_with(&s, &round, &d, Tol::witness()),
                vs - lens,
            ),
            (
                "round ∖ slab",
                topo::subtract_with(&round, &s, &d, Tol::witness()),
                vr - lens,
            ),
            (
                "slab ∩ round",
                topo::intersect_with(&s, &round, &d, Tol::witness()),
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

/// **The flush and corner postures, and the untilted dip, printed.**
/// Flush and corner: the differences and the intersection refuse in
/// `insert::plan_null_pairs` with an odd count of surviving crossing
/// records at the rim vertex pair, whichever way the arc reads
/// (`work/contacthold/a-flush-quarter-round-refuses-an-odd-crossing-count-at-its-rim-vertex.md`).
/// The dip: its wall touches the slab's top along the rim and lumps
/// whole as a tangent sector, and the differences and intersection
/// refuse in `sectors::along`, a run boundary on the coplanar bottom
/// sector beside it with both its edges on the top
/// (`work/contacthold/a-dipping-round-refuses-at-a-run-boundary-on-its-coplanar-bottom-sector.md`);
/// the union refuses `TangentSlitArmUnbuilt`. It asserts nothing. Run:
/// `cargo test -p sweep --test all the_flush_and_corner_postures -- --ignored --nocapture`
#[test]
#[ignore = "measurement: prints each op's outcome; run with --ignored --nocapture"]
fn the_flush_and_corner_postures() {
    let dip = dipping_round();
    let wide = slab(5.0, -1.0, 6.0);
    let (vd, vw) = (
        mass_properties(&dip, Tol::witness()).unwrap().volume,
        mass_properties(&wide, Tol::witness()).unwrap().volume,
    );
    let lens = 4.0 - core::f64::consts::PI;
    let d = BooleanDeclarations::none();
    report(
        "dip / slab ∪ round",
        topo::union_with(&wide, &dip, &d, Tol::witness()),
        Some(vw + vd - lens),
    );
    report(
        "dip / slab ∖ round",
        topo::subtract_with(&wide, &dip, &d, Tol::witness()),
        Some(vw - lens),
    );
    report(
        "dip / round ∖ slab",
        topo::subtract_with(&dip, &wide, &d, Tol::witness()),
        Some(vd - lens),
    );
    report(
        "dip / slab ∩ round",
        topo::intersect_with(&wide, &dip, &d, Tol::witness()),
        Some(lens),
    );
    let round = quarter_round();
    let vr = mass_properties(&round, Tol::witness()).unwrap().volume;
    for (posture, s) in [
        ("flush", slab(2.0, -1.0, 6.0)),
        ("corner", slab(2.0, 0.0, 4.0)),
    ] {
        let vs = mass_properties(&s, Tol::witness()).unwrap().volume;
        for (decl, level) in LEVELS {
            let fwd = declarations(&s, &round, level, true);
            let rev = declarations(&s, &round, level, false);
            let tag = |op: &str| format!("{posture} / {decl} / {op}");
            report(
                &tag("slab ∪ round"),
                topo::union_with(&s, &round, &fwd, Tol::witness()),
                Some(vs + vr),
            );
            report(
                &tag("slab ∖ round"),
                topo::subtract_with(&s, &round, &fwd, Tol::witness()),
                Some(vs),
            );
            report(
                &tag("round ∖ slab"),
                topo::subtract_with(&round, &s, &rev, Tol::witness()),
                Some(vr),
            );
            report(
                &tag("slab ∩ round"),
                topo::intersect_with(&s, &round, &fwd, Tol::witness()),
                None,
            );
        }
    }
}

/// A slab `x ∈ [x0, x1]`, `z ∈ [0, 1]`, `y ∈ [y0, y0 + depth]`.
fn slab_between(x0: f64, x1: f64, y0: f64, depth: f64) -> AtRestBody<f64> {
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
/// (`x ∈ [0, 2]`). At the frozen head `edge` refuses
/// `Join(UnpairedLooseEnds { count: 2 })`, the first-order signature
/// (`work/contacthold/conic-ties-outside-the-vertex-on-face-pass-still-read-first-order.md`);
/// descending in `side_code` too moves it to `ClassificationInvariant`.
/// `corner` and `beside-corner` refuse `CurvedPierceUnsupported`; `beside`
/// answers. Run with `--ignored --nocapture`.
#[test]
#[ignore = "measurement: prints each op's outcome; run with --ignored --nocapture"]
fn review_tilted_dip_off_the_vertex_on_face_pass() {
    let vol = |b: &AtRestBody<f64>| mass_properties(b, Tol::witness()).unwrap().volume;
    let d = BooleanDeclarations::none();
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
                topo::union_with(&s, &round, &d, Tol::witness()),
                Some(vs + vr - meet),
            );
            report(
                &tag("slab ∖ round"),
                topo::subtract_with(&s, &round, &d, Tol::witness()),
                w(vs - meet),
            );
            report(
                &tag("round ∖ slab"),
                topo::subtract_with(&round, &s, &d, Tol::witness()),
                w(vr - meet),
            );
            report(
                &tag("slab ∩ round"),
                topo::intersect_with(&s, &round, &d, Tol::witness()),
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
    let plane = SketchPlane::new(Affine3::from_parts(
        geom_core::Mat3::from_cols(
            Vec3::new(0.0, -sa, ca),
            Vec3::unit_x(),
            Vec3::new(0.0, ca, sa),
        ),
        Vec3::new(0.0, sa, 1.0 - ca),
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
    let vol = |b: &AtRestBody<f64>| mass_properties(b, Tol::witness()).unwrap().volume;
    let d = BooleanDeclarations::none();
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
                    topo::union_with(&s, &round, &d, Tol::witness()),
                    vs + vr - meet,
                ),
                (
                    "slab ∖ round",
                    topo::subtract_with(&s, &round, &d, Tol::witness()),
                    vs - meet,
                ),
                (
                    "round ∖ slab",
                    topo::subtract_with(&round, &s, &d, Tol::witness()),
                    vr - meet,
                ),
                (
                    "slab ∩ round",
                    topo::intersect_with(&s, &round, &d, Tol::witness()),
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
