//! **An arc tangent to the partner's face at a boolean vertex, curving
//! off** — the measurement fixture for
//! `work/contacthold/boolean-conic-side-code-zero-is-first-order.md`.
//!
//! A quarter round (x ∈ [0.5, 3], z ∈ [1, 3], y ∈ [0, 4]) whose bottom
//! face meets its cylindrical wall tangentially along the rim
//! x = 2, z = 1; each cap carries an arc that leaves the rim vertex
//! tangent to z = 1 and rises to z = 2. It sits on a slab whose top is
//! z = 1, in three postures that put the rim vertex at a different kind
//! of boolean vertex:
//!
//! - `wide`: x ∈ [0, 5], y ∈ [−1, 5]. The rim vertex is inside the
//!   slab's top face, so the face covers the arc's in-band stretch.
//! - `flush`: x ∈ [0, 2], y ∈ [−1, 5]. The rim vertex lies on the
//!   slab's top edge; the slab's top ends where the arc leaves.
//! - `corner`: x ∈ [0, 2], y ∈ [0, 4]. The rim vertex is a slab corner.
//!
//! Each op runs undeclared, with the resting faces declared `Rest`,
//! and with the wall declared `Tangent` to the slab top as well. The
//! right answers: the union's volume is the sum, the slab minus the
//! round is the slab, the round minus the slab is the round, and the
//! intersection is empty.
//!
//! It asserts nothing; it prints. Run:
//! `cargo test -p sweep --test all an_arc_tangent_at_a_boolean_vertex_curves_off -- --ignored --nocapture`

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
    let b90 = (core::f64::consts::PI / 8.0).tan();
    lying_extrude(
        vec![
            (Point2::new(1.0, 0.5), 0.0),
            (Point2::new(1.0, 2.0), b90),
            (Point2::new(0.0, 3.0), 0.0),
            (Point2::new(3.0, 3.0), 0.0),
            (Point2::new(3.0, 0.5), 0.0),
        ],
        0.0,
        4.0,
        "the dipping round",
    )
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

#[test]
#[ignore = "measurement: prints each op's outcome; run with --ignored --nocapture"]
fn an_arc_tangent_at_a_boolean_vertex_curves_off() {
    let round = quarter_round();
    let vr = mass_properties(&round, Tol::witness()).unwrap().volume;
    for (posture, s) in [
        ("wide", slab(5.0, -1.0, 6.0)),
        ("flush", slab(2.0, -1.0, 6.0)),
        ("corner", slab(2.0, 0.0, 4.0)),
    ] {
        let vs = mass_properties(&s, Tol::witness()).unwrap().volume;
        // `level`: 0 undeclared, 1 the resting faces `Rest`, 2 the wall
        // `Tangent` too; `slab_first` orients each pair as the op reads it.
        let decls = |level: u8, slab_first: bool| {
            let mut d = BooleanDeclarations::none();
            let mut push = |fs: topo::FaceKey, fr: topo::FaceKey, class: ContactClass| {
                let (a, b) = if slab_first { (fs, fr) } else { (fr, fs) };
                d.coincident_faces
                    .push(FacePairDeclaration::new(a, b, class));
            };
            if level >= 1 {
                push(
                    plane_face(&s, 1.0, true),
                    plane_face(&round, 1.0, false),
                    ContactClass::Rest,
                );
            }
            if level >= 2 {
                push(
                    plane_face(&s, 1.0, true),
                    cyl_face(&round),
                    ContactClass::Tangent,
                );
            }
            d
        };
        for (decl, level) in [("undeclared", 0), ("Rest", 1), ("Rest+Tangent", 2)] {
            let d = &decls(level, true);
            let tag = |op: &str| format!("{posture} / {decl} / {op}");
            eprintln!("SIDECODE-FIXTURE --- {posture} / {decl}");
            report(
                &tag("slab ∪ round"),
                topo::union_with(&s, &round, d, Tol::witness()),
                Some(vs + vr),
            );
            report(
                &tag("slab ∖ round"),
                topo::subtract_with(&s, &round, d, Tol::witness()),
                Some(vs),
            );
            report(
                &tag("round ∖ slab"),
                topo::subtract_with(&round, &s, &decls(level, false), Tol::witness()),
                Some(vr),
            );
            report(
                &tag("slab ∩ round"),
                topo::intersect_with(&s, &round, d, Tol::witness()),
                None,
            );
        }
    }
}

/// **The arc curving INTO the partner**: the dipping round over the
/// wide slab. They overlap in the lens between the slab's top and the
/// round's wall, `(1 − π/4)` per unit of extrusion over `y ∈ [0, 4]`.
/// No declaration: the wall crosses into the slab, so nothing is a
/// contact but the rim line.
#[test]
#[ignore = "measurement: prints each op's outcome; run with --ignored --nocapture"]
fn an_arc_tangent_at_a_boolean_vertex_curves_in() {
    let round = dipping_round();
    let vr = mass_properties(&round, Tol::witness()).unwrap().volume;
    eprintln!(
        "SIDECODE-FIXTURE dip: round volume {vr} (want {})",
        24.0 - core::f64::consts::PI
    );
    let s = slab(5.0, -1.0, 6.0);
    let vs = mass_properties(&s, Tol::witness()).unwrap().volume;
    let lens = 4.0 - core::f64::consts::PI;
    let d = BooleanDeclarations::none();
    report(
        "dip / slab ∪ round",
        topo::union_with(&s, &round, &d, Tol::witness()),
        Some(vs + vr - lens),
    );
    report(
        "dip / slab ∖ round",
        topo::subtract_with(&s, &round, &d, Tol::witness()),
        Some(vs - lens),
    );
    report(
        "dip / round ∖ slab",
        topo::subtract_with(&round, &s, &d, Tol::witness()),
        Some(vr - lens),
    );
    report(
        "dip / slab ∩ round",
        topo::intersect_with(&s, &round, &d, Tol::witness()),
        Some(lens),
    );
}
