//! CERT-M2 R1 probe (both-tree compatible: compiles at the merge base
//! f3c035579 and at the head 8f5384515). Dumps the verdicts of the two
//! passes the PR body claims byte-identical (`validate_pseudomanifold`,
//! `contact_marks`) plus `validate_geometric` at the certifying scalars
//! and `mass_properties` over a corpus of valid and corrupt bodies at
//! f64 / Dual64 / Interval. Run with `--nocapture`, grep `M2R1|`.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic, dead_code)]

use core::f64::consts::{FRAC_PI_2, PI};
use geom_core::{Band, Point2, Point3, Real, Tol, Vec2, Vec3};
use profile::{Profile, ProfileLoop, RawLoop, SketchPlane, test_support::bulge_loop};
use sweep::{Extrusion, Revolution, RevolveAxis, extrude, revolve};
use topo::{Body, ContactRecords, SplitPart, SplitPlane, split};

fn v<T: Real>(x: f64, y: f64, b: f64) -> (Point2<T>, T) {
    (Point2::new(T::from_f64(x), T::from_f64(y)), T::from_f64(b))
}

fn profile<T: geom_core::Decide>(lp: ProfileLoop<T>) -> profile::ValidatedProfile<T> {
    Profile::new(SketchPlane::<T>::xy(), vec![lp])
        .validate(Tol::witness())
        .unwrap()
}

/// The generic corpus: name, body. Valid bodies first, then their
/// reverted (NegativeVolume) twins.
pub(crate) fn corpus<T: topo::AtRestPolicy>() -> Vec<(String, Body<T>)> {
    let tol = Tol::witness();
    let mut out: Vec<(String, Body<T>)> = Vec::new();
    // L-prism (planar, closed form).
    let l = bulge_loop(vec![
        v(0.0, 0.0, 0.0),
        v(2.0, 0.0, 0.0),
        v(2.0, 1.0, 0.0),
        v(1.0, 1.0, 0.0),
        v(1.0, 2.0, 0.0),
        v(0.0, 2.0, 0.0),
    ]);
    let l_prism = extrude(&profile(l), Extrusion::Distance(T::from_f64(1.0)), tol)
        .unwrap()
        .body;
    out.push(("l_prism".into(), l_prism));
    // Cylinder (two semicircular arcs), closed form.
    let c = bulge_loop(vec![v(-1.0, 0.0, 1.0), v(1.0, 0.0, 1.0)]);
    let cyl = extrude(&profile(c), Extrusion::Distance(T::from_f64(2.0)), tol)
        .unwrap()
        .body;
    out.push(("cylinder".into(), cyl.clone()));
    // Cylinder cut by an oblique plane: the ellipse-trimmed face needs the
    // quadrature lane (DL3's `cut_cylinder` class).
    let phi = 0.4;
    let plane = SplitPlane {
        origin: Point3::new(T::from_f64(0.0), T::from_f64(0.0), T::from_f64(1.0)),
        normal: Vec3::new(
            T::from_f64(phi.sin()),
            T::from_f64(0.0),
            T::from_f64(phi.cos()),
        ),
    };
    let res = split(&cyl, &plane, tol).unwrap();
    if let SplitPart::Body(above) = &res.above {
        out.push(("cut_cylinder_above".into(), above.clone()));
    }
    if let SplitPart::Body(below) = &res.below {
        out.push(("cut_cylinder_below".into(), below.clone()));
    }
    // Washer: rectangle revolved a full turn (torus-free, cylinder walls).
    let w = bulge_loop(vec![
        v(1.0, 0.0, 0.0),
        v(2.0, 0.0, 0.0),
        v(2.0, 1.0, 0.0),
        v(1.0, 1.0, 0.0),
    ]);
    let axis = RevolveAxis {
        origin: Point2::new(T::from_f64(0.0), T::from_f64(0.0)),
        dir: Vec2::new(T::from_f64(0.0), T::from_f64(1.0)),
    };
    let washer = revolve(&profile(w), axis, Revolution::Full, tol)
        .unwrap()
        .body;
    out.push(("washer".into(), washer));
    // Quarter washer (partial revolve — wedge caps).
    let w2 = bulge_loop(vec![
        v(1.0, 0.0, 0.0),
        v(2.0, 0.0, 0.0),
        v(2.0, 1.0, 0.0),
        v(1.0, 1.0, 0.0),
    ]);
    let axis2 = RevolveAxis {
        origin: Point2::new(T::from_f64(0.0), T::from_f64(0.0)),
        dir: Vec2::new(T::from_f64(0.0), T::from_f64(1.0)),
    };
    let quarter = revolve(
        &profile(w2),
        axis2,
        Revolution::Partial(T::from_f64(FRAC_PI_2)),
        tol,
    )
    .unwrap()
    .body;
    out.push(("quarter_washer".into(), quarter));
    // Grooved washer: a semicircular arc in the outer wall (torus face,
    // quadrature at certifying scalars).
    let g = bulge_loop(vec![
        v(1.0, 0.0, 0.0),
        v(3.0, 0.0, 0.0),
        v(3.0, 0.5, -1.0),
        v(3.0, 1.5, 0.0),
        v(3.0, 2.0, 0.0),
        v(1.0, 2.0, 0.0),
    ]);
    let axis3 = RevolveAxis {
        origin: Point2::new(T::from_f64(0.0), T::from_f64(0.0)),
        dir: Vec2::new(T::from_f64(0.0), T::from_f64(1.0)),
    };
    if let Ok(t) = revolve(&profile(g), axis3, Revolution::Full, tol) {
        out.push(("grooved_washer".into(), t.body));
    }
    // Reverted twins.
    let reverted: Vec<(String, Body<T>)> = out
        .iter()
        .filter_map(|(n, b)| b.revert().ok().map(|r| (format!("{n}~reverted"), r)))
        .collect();
    out.extend(reverted);
    let _ = PI;
    out
}

/// The three passes at one scalar, as the door table [`dump`] walks:
/// the certified names ([`certified`]) or their `_structural` twins
/// ([`structural`]), whichever the scalar's bound can form. One dump
/// over a table rather than a dump per family, so the two families
/// print under one set of labels and the rows compare across scalars.
struct Doors<T: geom_core::Decide> {
    pseudomanifold: PseudomanifoldDoor<T>,
    marks: MarksDoor<T>,
    mass: MassDoor<T>,
}

type PseudomanifoldDoor<T> =
    fn(&Body<T>, &ContactRecords, Tol) -> Result<(), Vec<topo::ValidationError>>;
type MarksDoor<T> = fn(&Body<T>, Tol) -> Result<Vec<String>, Vec<topo::ValidationError>>;
type MassDoor<T> = fn(&Body<T>, Tol) -> Result<topo::MassProperties<T>, topo::MassPropsError>;

/// The marks pass's map, rendered in one stable order.
fn sorted_marks<K: core::fmt::Debug, V: core::fmt::Debug>(
    marks: impl IntoIterator<Item = (K, V)>,
) -> Vec<String> {
    let mut v: Vec<String> = marks
        .into_iter()
        .map(|(k, m)| format!("{k:?}={m:?}"))
        .collect();
    v.sort();
    v
}

/// The certified doors — every scalar whose bound names the right.
fn certified<T: geom_core::CertifiedBounds + topo::AtRestPolicy>() -> Doors<T> {
    Doors {
        pseudomanifold: topo::validate_pseudomanifold::<T>,
        marks: |body, tol| topo::contact_marks(body, tol).map(|m| sorted_marks(m.iter())),
        mass: topo::mass_properties::<T>,
    }
}

/// The `_structural` twins — the doors a scalar without certification
/// rights measures through.
fn structural<T: geom_core::Bounds + topo::AtRestPolicy>() -> Doors<T> {
    Doors {
        pseudomanifold: topo::validate_pseudomanifold_structural::<T>,
        marks: |body, tol| {
            topo::contact_marks_structural(body, tol).map(|m| sorted_marks(m.iter()))
        },
        mass: topo::mass_properties_structural::<T>,
    }
}

/// The three passes at one scalar through `doors`, under one set of
/// labels, so the rows compare across scalars and across the two door
/// families.
fn dump<T: geom_core::Decide + core::fmt::Debug>(
    scalar: &str,
    name: &str,
    body: &Body<T>,
    doors: &Doors<T>,
) {
    let tol = Tol::witness();
    println!(
        "M2R1|{scalar}|{name}|pseudomanifold|{:?}",
        (doors.pseudomanifold)(body, &ContactRecords::default(), tol)
    );
    println!(
        "M2R1|{scalar}|{name}|contact_marks|{:?}",
        (doors.marks)(body, tol)
    );
    println!(
        "M2R1|{scalar}|{name}|mass_properties|{:?}",
        (doors.mass)(body, tol)
    );
}

fn dump_composed<T: geom_core::CertifiedBounds + core::fmt::Debug + topo::AtRestPolicy>(
    scalar: &str,
    name: &str,
    body: &Body<T>,
) {
    println!(
        "M2R1|{scalar}|{name}|validate_geometric|{:?}",
        topo::validate_geometric(body, Tol::witness())
    );
}

#[test]
fn m2r1_passes_f64() {
    for (n, b) in corpus::<f64>() {
        dump("f64", &n, &b, &certified());
        dump_composed("f64", &n, &b);
    }
    for (n, b) in f64_only_corpus() {
        dump("f64", &n, &b, &certified());
        dump_composed("f64", &n, &b);
    }
}

#[test]
fn m2r1_passes_dual64() {
    for (n, b) in corpus::<geom_core::Dual64>() {
        dump("dual64", &n, &b, &structural());
    }
}

/// The declared straddle seat at `f64` through BOTH door families —
/// the one dump row whose `_structural` line carries a
/// `CensusLaneUnsupported`. The certified door examines the declared
/// pair and certifies the seat (`Ok`); the `_structural` door holds no
/// region door at any scalar, refuses the pair typed and leaves the two
/// crossings the declaration backs as `UndeclaredContact`.
#[test]
fn m2r1_declared_seat_f64() {
    let tol = Tol::witness();
    let seat = topo::test_support::straddle_seat(tol);
    let records = ContactRecords {
        patches: vec![topo::PatchContact {
            face_a: seat.post_top,
            face_b: seat.shelf_bottom,
        }],
        ..ContactRecords::default()
    };
    for (family, doors) in [
        ("certified", certified::<f64>()),
        ("structural", structural::<f64>()),
    ] {
        println!(
            "M2R1|f64|straddle_seat~declared|pseudomanifold~{family}|{:?}",
            (doors.pseudomanifold)(&seat.body, &records, tol)
        );
    }
}

/// **The `_structural` door at `Dual64` answers the certified door's
/// `f64` measurement on every closed-form body, and refuses exactly
/// where the lane would have enclosed** — the content of the `None`
/// path, body by body over the corpus, against a door that reaches it
/// by another route. A dual's value channel is bit-identical to the
/// `f64` build's (the dual contract), and a closed-form face computes
/// the same through either door, so on every body the closed form
/// covers the two agree to the bit with pads of `0` on both sides; on
/// the oblique-cut cylinders, whose ellipse-trimmed face needs the
/// quadrature, the certified door encloses (pads above `0`) and the
/// structural one refuses typed at the props layer. A `None` path that
/// decided anything the closed form does not, or a closed form that
/// drifted from the certified walk on a closed-form face, reds here.
#[test]
fn m2r1_structural_at_dual64_is_the_f64_closed_form_and_refuses_where_the_lane_would_enclose() {
    let tol = Tol::witness();
    let base = corpus::<f64>();
    let dual = corpus::<geom_core::Dual64>();
    assert_eq!(base.len(), dual.len(), "the corpus builds at both scalars");
    let mut refused = Vec::new();
    for ((name, b), (dname, d)) in base.iter().zip(&dual) {
        assert_eq!(name, dname);
        let certified = topo::mass_properties(b, tol)
            .unwrap_or_else(|e| panic!("{name}: the certified door refused a corpus body: {e:?}"));
        match topo::mass_properties_structural(d, tol) {
            Ok(s) => {
                assert_eq!(
                    (s.volume.value.to_bits(), s.surface_area.value.to_bits()),
                    (certified.volume.to_bits(), certified.surface_area.to_bits()),
                    "{name}: the dual's value channel is the f64 certified measurement"
                );
                assert_eq!(
                    (
                        s.volume_pad.to_bits(),
                        s.area_pad.to_bits(),
                        certified.volume_pad.to_bits(),
                        certified.area_pad.to_bits()
                    ),
                    (0, 0, 0, 0),
                    "{name}: a closed-form body carries pads of 0 through both doors"
                );
            }
            Err(topo::MassPropsError::Face { .. }) => {
                assert!(
                    certified.volume_pad > 0.0,
                    "{name}: the structural door refused a face the certified door did not                      need the quadrature for"
                );
                refused.push(name.clone());
            }
            Err(other) => panic!("{name}: the structural door refused with {other:?}"),
        }
    }
    let mut expect = vec![
        "cut_cylinder_above".to_string(),
        "cut_cylinder_below".to_string(),
        "cut_cylinder_above~reverted".to_string(),
        "cut_cylinder_below~reverted".to_string(),
    ];
    refused.sort();
    expect.sort();
    assert_eq!(
        refused, expect,
        "exactly the ellipse-trimmed bodies separate the two measurement doors"
    );
}

#[test]
fn m2r1_passes_interval() {
    for (n, b) in corpus::<geom_core::Interval>() {
        dump("interval", &n, &b, &certified());
        dump_composed("interval", &n, &b);
    }
}

// ---- f64-only corrupt constructions (check 8 / check 9 failures). ----

fn p2(x: f64, y: f64) -> Point2<f64> {
    Point2::new(x, y)
}

fn plane_chart_at_y(body: &Body<f64>, y: f64) -> Vec<topo::FaceKey> {
    body.faces()
        .filter(|(_, f)| {
            matches!(body.get_surface(f.surface),
                Some(geom::Surface::Plane { origin, .. }) if (origin.y - y).abs() < 1e-12)
        })
        .map(|(k, _)| k)
        .collect()
}

fn revolved(lp: ProfileLoop<f64>) -> Body<f64> {
    revolve(
        &profile(lp),
        RevolveAxis {
            origin: p2(0.0, 0.0),
            dir: Vec2::new(0.0, 1.0),
        },
        Revolution::Full,
        Tol::witness(),
    )
    .unwrap()
    .body
}

/// A ring standing on its own outer loop (check 9), built the way
/// `verbs_shell.rs::a_ring_standing_on_its_outer_loop_refuses_at_tier_3`
/// builds it; plus its reverted twin so check 7 WOULD also fire.
pub(crate) fn f64_only_corpus() -> Vec<(String, Body<f64>)> {
    let tol = Tol::witness();
    let band = Band::linear(tol).unwrap();
    let mut out = Vec::new();
    let vessel = revolved(bulge_loop(vec![
        v(0.0, 0.0, 0.0),
        v(0.5, 0.0, 0.0),
        v(0.5, 0.4, 0.0),
        v(0.0, 0.4, 0.0),
    ]));
    let tube = revolved(bulge_loop(vec![
        v(0.30, 0.0, 0.0),
        v(0.50, 0.0, 0.0),
        v(0.50, 0.40, 0.0),
        v(0.30, 0.40, 0.0),
    ]));
    let t = 0.05;
    for (what, body, y) in [
        ("ring_on_outer_vessel", vessel, 0.4),
        ("ring_on_outer_tube", tube, 0.40),
    ] {
        let mut sealed = topo::shell(&body, t, tol).expect("sealed shell").body;
        let mouth = plane_chart_at_y(&sealed, y);
        let counterpart = plane_chart_at_y(&sealed, y - t);
        let plane_of =
            |b: &Body<f64>, f: topo::FaceKey| match b.get_surface(b.get_face(f).unwrap().surface) {
                Some(geom::Surface::Plane { origin, normal, .. }) => (*origin, *normal),
                other => panic!("non-planar cap {other:?}"),
            };
        let (o_from, n_from) = plane_of(&sealed, counterpart[0]);
        let (o_onto, _) = plane_of(&sealed, mouth[0]);
        let back = (o_onto - o_from).dot(n_from);
        topo::replace_faces_offset(&mut sealed, &counterpart, back, band, tol).unwrap();
        for (&rim, &source) in mouth.iter().zip(&counterpart) {
            sealed.kfmrh(rim, source).unwrap();
        }
        if let Ok(r) = sealed.revert() {
            out.push((format!("{what}~reverted"), r));
        }
        out.push((what.to_string(), sealed));
    }
    // Diagonal chord split of a quarter washer wall (check 2 + check 8).
    let lp = ProfileLoop::polygon([p2(1.0, 0.0), p2(2.0, 0.0), p2(2.0, 1.0), p2(1.0, 1.0)]);
    let tq = revolve(
        &profile(lp),
        RevolveAxis {
            origin: p2(0.0, 0.0),
            dir: Vec2::new(0.0, 1.0),
        },
        Revolution::Partial(FRAC_PI_2),
        tol,
    )
    .unwrap();
    let mut body = tq.body;
    let wall = tq.walls[0][1].expect("outer wall");
    let outer = body.get_face(wall).unwrap().outer;
    let topo::LoopBoundary::Cycle { first } = body.get_loop(outer).unwrap().boundary else {
        panic!("cycle");
    };
    let cycle = body.loop_cycle(first).unwrap();
    let (he1, he2) = (cycle[0], cycle[2]);
    let point_of = |body: &Body<f64>, he| {
        let vv = body.get_half_edge(he).unwrap().start;
        *body.get_point(body.get_vertex(vv).unwrap().point).unwrap()
    };
    let (a, b) = (point_of(&body, he1), point_of(&body, he2));
    body.mef(
        topo::MefSite::Chords { he1, he2 },
        topo::EdgeCurveSpec::line_between(a, b),
        topo::FaceSurface::Inherit,
        tol,
    )
    .unwrap();
    if let Ok(r) = body.revert() {
        out.push(("chord_split~reverted".into(), r));
    }
    out.push(("chord_split".into(), body));
    out
}
