//! **The strut source of the one-sided cover on cylinder pairs**
//! (`topo::boolean`'s `tangency_certifies_side`, its strut column),
//! measured and pinned.
//!
//! A plate whose outline runs a big arc into a small one, tangent at
//! the joint, is extruded: its two walls are cylinders tangent along a
//! ruling, and the edge between them is described
//! `TangentIntersection`: the small arc is authored as the big one's
//! tangent continuation (`.tangent()`), a constructed tangency, as the
//! rounded stack's fillets are. Two such plates are stacked with
//! every flush finding declared, as the rounded stack of
//! `reach_continuation` is. The union needs the strut cover on the
//! cylinder pair: the lower plate's tangent ruling ends on the upper
//! plate's continued wall, and the cover is what lets that endpoint
//! through the crossing layer. Three stacks, in both member orders: the
//! small arc inside the big one's circle, and outside it (an S), each
//! on a plate of its own outline; and the inside joint under a plate
//! cut back to the big arc alone, which has no strut, so that the cover
//! comes from the lower plate's strut in each direction the row
//! certifies.
//!
//! The rows pin the outcome: every stack unions at its closed form,
//! its subtract and intersect stop at the fallback extent, and each op
//! undeclared is the declared one (D10); the one-profile capsule's
//! sphere × cylinder strut, met by a coaxial rod, refuses at the
//! crossing layer, save the rod ending on the joint, whose two member
//! orders part. The `#[ignore]`d row meters the covered stack against
//! point probes through its mesh, which the tessellation refuses; run
//! it with `cargo nextest run -p sweep --run-ignored only -E
//! 'test(strut_cover_on_cylinder_pairs)'`.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::common::outcomes::outcome;
use core::f64::consts::PI;
use geom::SurfaceKind;
use geom_core::{Affine3, Point2, Point3, Tol, Vec3};
use profile::{ArcSweep, Center, Open, ProfileLoop, Start};
use sweep::test_support::{extruded, finished, sketch_at};
use sweep::{ExtrudeSide, Extrusion, Revolution, extrude};
use topo::{
    AtRestBody, Body, BooleanCoincidence, BooleanDeclarations, BooleanError, BooleanResult,
    FaceKey, FacePairDeclaration,
};

fn tol() -> Tol {
    Tol::witness()
}

/// One profile segment's start and the arc leaving it: `None` for a
/// line, else the arc's centre, radius and signed sweep.
type Leg = ((f64, f64), Option<((f64, f64), f64, f64)>);

/// The big arc's radius, about the origin from −60° to 60°.
const BIG: f64 = 2.0;
/// The small arc's radius, tangent to the big one at 60°.
const SMALL: f64 = 0.5;

fn at(c: (f64, f64), r: f64, deg: f64) -> (f64, f64) {
    let a = deg.to_radians();
    (c.0 + r * a.cos(), c.1 + r * a.sin())
}

/// Which outline a plate has.
#[derive(Clone, Copy, Debug)]
enum Outline {
    /// The small arc inside the big arc's circle.
    Inside,
    /// The small arc outside it, turning the other way: an S.
    S,
    /// [`Outline::Inside`] cut back to the big arc alone: from the
    /// joint straight down to the small arc's far end's height, so its
    /// big-arc wall ends on the joint's ruling and it has no strut.
    CutBack,
}

/// The outline as legs: the big arc counter-clockwise from −60° to
/// 60°, the small arc tangent to it there (its centre on the big arc's
/// radius at 60°, inside the big circle or outside it and running
/// clockwise), then straight back round by `x = −1`.
fn legs(outline: Outline) -> Vec<Leg> {
    let (p0, p1) = (at((0.0, 0.0), BIG, -60.0), at((0.0, 0.0), BIG, 60.0));
    let big = Some(((0.0, 0.0), BIG, 120.0));
    let (centre, from, sweep) = match outline {
        Outline::Inside | Outline::CutBack => (at((0.0, 0.0), BIG - SMALL, 60.0), 60.0, 90.0),
        Outline::S => (at((0.0, 0.0), BIG + SMALL, 60.0), 240.0, -90.0),
    };
    let p2 = at(centre, SMALL, from + sweep);
    let mut l = vec![(p0, big)];
    match outline {
        Outline::CutBack => l.extend([(p1, None), ((p1.0, p2.1), None)]),
        Outline::Inside | Outline::S => {
            l.extend([(p1, Some((centre, SMALL, sweep))), (p2, None)]);
        }
    }
    l.extend([((-1.0, p2.1), None), ((-1.0, p0.1), None)]);
    l
}

/// [`legs`] authored as a user would: the big arc by its centre, the
/// small arc as the tangent continuation of it (the joint constructed,
/// D10), and lines.
fn outline(outline: Outline) -> ProfileLoop<f64> {
    let t = tol();
    let l = legs(outline);
    let pt = |i: usize| Point2::new(l[i].0.0, l[i].0.1);
    let mut path = Open
        .at(pt(0))
        .arc_to(
            Center {
                c: Point2::new(0.0, 0.0),
                winding: ArcSweep::Ccw,
                p: pt(1),
            },
            t,
        )
        .expect("the big arc authors");
    path = match l[1].1 {
        Some(_) => path
            .tangent()
            .tangent_arc_to(pt(2), t)
            .expect("the small arc continues it tangent"),
        None => path.line_to(pt(2), t).expect("the cut back"),
    };
    path.line_to(pt(3), t)
        .expect("across the top")
        .line_to(pt(4), t)
        .expect("down the west side")
        .line_to(Start, t)
        .expect("back to the big arc")
        .into()
}

/// The outline's area in closed form: the shoelace over its vertices
/// plus each arc's circular segment, signed by its sweep.
fn area(outline: Outline) -> f64 {
    let l = legs(outline);
    let shoelace: f64 = (0..l.len())
        .map(|i| {
            let (a, b) = (l[i].0, l[(i + 1) % l.len()].0);
            a.0 * b.1 - b.0 * a.1
        })
        .sum::<f64>()
        / 2.0;
    let segments: f64 = l
        .iter()
        .filter_map(|&(_, arc)| arc)
        .map(|(_, r, sweep)| {
            let t = sweep.to_radians();
            r * r / 2.0 * (t - t.sin())
        })
        .sum();
    shoelace + segments
}

/// Whether `(x, y)` is inside the outline, and its distance to the
/// outline's boundary, read off a fine polyline of the segments' own
/// data (each arc in 2000 chords, whose sagitta is below 1e-6).
fn in_outline(outline: Outline, x: f64, y: f64) -> (bool, f64) {
    let l = legs(outline);
    let (mut winding, mut clearance) = (0.0_f64, f64::INFINITY);
    for (i, &(a, arc)) in l.iter().enumerate() {
        let b = l[(i + 1) % l.len()].0;
        let n = if arc.is_some() { 2000 } else { 1 };
        let point = |k: usize| match arc {
            None => (
                a.0 + (b.0 - a.0) * k as f64 / n as f64,
                a.1 + (b.1 - a.1) * k as f64 / n as f64,
            ),
            Some((c, r, sweep)) => {
                let from = (a.1 - c.1).atan2(a.0 - c.0).to_degrees();
                at(c, r, from + sweep * k as f64 / n as f64)
            }
        };
        for k in 0..n {
            let (p, q) = (point(k), point(k + 1));
            let (dx, dy) = (q.0 - p.0, q.1 - p.1);
            let s = (((x - p.0) * dx + (y - p.1) * dy) / (dx * dx + dy * dy)).clamp(0.0, 1.0);
            clearance = clearance.min((x - p.0 - s * dx).hypot(y - p.1 - s * dy));
            let cross = (p.0 - x) * (q.1 - y) - (q.0 - x) * (p.1 - y);
            winding += cross.atan2((p.0 - x) * (q.0 - x) + (p.1 - y) * (q.1 - y));
        }
    }
    (winding.abs() > PI, clearance)
}

fn plate(shape: Outline, z0: f64) -> AtRestBody<f64> {
    finished(
        "the plate",
        extruded(sketch_at(z0), vec![outline(shape)], 1.0, tol()),
        tol(),
    )
}

/// The `TangentIntersection` edges of `b`, as the kinds of the two
/// faces each one bounds.
fn tangent_struts(b: &Body<f64>) -> Vec<(SurfaceKind, SurfaceKind)> {
    let kind = |he| {
        let f = b.face_of_half_edge(he).unwrap();
        b.get_surface(b.get_face(f).unwrap().surface)
            .unwrap()
            .kind()
    };
    b.edges()
        .filter(|(_, e)| {
            matches!(
                b.get_curve_geom(e.curve)
                    .and_then(|g| g.certified())
                    .map(|c| c.description()),
                Some(geom_brep::EdgeDescription::TangentIntersection { .. })
            )
        })
        .map(|(_, e)| (kind(e.he_plus), kind(e.he_minus)))
        .collect()
}

/// The flush detector's findings between `a` and `b`, every one
/// declared: the mating plane `Rest` and the walls continuations.
fn findings(a: &Body<f64>, b: &Body<f64>) -> BooleanDeclarations {
    let found = topo::flush::find_flush_candidates(a, b, tol()).expect("the plates decide");
    topo::flush::declare_all(&found)
}

/// The generalized winding number of the closed triangle mesh about
/// `p`: 1 inside, 0 outside, read from the triangles alone.
fn winding(mesh: &mesh::Mesh, p: [f64; 3]) -> f64 {
    let v = |i: u32| {
        let q = mesh.positions[i as usize];
        [q.x - p[0], q.y - p[1], q.z - p[2]]
    };
    let dot = |a: [f64; 3], b: [f64; 3]| a[0] * b[0] + a[1] * b[1] + a[2] * b[2];
    let norm = |a: [f64; 3]| dot(a, a).sqrt();
    let solid: f64 = mesh
        .patches
        .iter()
        .flat_map(|patch| patch.triangles.iter())
        .map(|&[i, j, k]| {
            let (a, b, c) = (v(i), v(j), v(k));
            let cross = [
                b[1] * c[2] - b[2] * c[1],
                b[2] * c[0] - b[0] * c[2],
                b[0] * c[1] - b[1] * c[0],
            ];
            let (la, lb, lc) = (norm(a), norm(b), norm(c));
            let den = la * lb * lc + dot(a, b) * lc + dot(b, c) * la + dot(c, a) * lb;
            2.0 * dot(a, cross).atan2(den)
        })
        .sum();
    solid / (4.0 * PI)
}

/// The stacks: the lower plate's outline and the upper's, the lower on
/// `z ∈ [0, 1]` and the upper on `z ∈ [1, 2]`.
const STACKS: [(Outline, Outline); 3] = [
    (Outline::Inside, Outline::Inside),
    (Outline::S, Outline::S),
    (Outline::Inside, Outline::CutBack),
];

/// One stack united in both member orders with every finding declared:
/// each order's label and its body, valid at tier 3 and 3′.
fn stacks(lower: Outline, upper: Outline) -> Vec<(String, topo::BooleanBody<f64>)> {
    let (p, q) = (plate(lower, 0.0), plate(upper, 1.0));
    assert_eq!(
        tangent_struts(&p),
        [(SurfaceKind::Cylinder, SurfaceKind::Cylinder)],
        "{lower:?}: the plate's one strut, between its two walls"
    );
    [("P ∪ Q", &p, &q), ("Q ∪ P", &q, &p)]
        .into_iter()
        .map(|(order, a, b)| {
            let label = format!("{lower:?} under {upper:?}, {order}");
            let out = topo::union_with(a, b, &findings(a, b), tol())
                .unwrap_or_else(|e| panic!("{label}: builds: {e:?}"));
            let BooleanResult::Body(bb) = out else {
                panic!("{label}: a stack is not empty");
            };
            assert_eq!(
                topo::validate_geometric(&bb.body, tol()),
                Ok(()),
                "{label}: tier 3"
            );
            assert_eq!(
                topo::validate_pseudomanifold(&bb.body, &bb.contacts, tol()),
                Ok(()),
                "{label}: tier 3′"
            );
            (label, bb)
        })
        .collect()
}

/// **Every arc-joint stack unions and stops its subtract and intersect
/// at the fallback extent**, in both member orders, every flush finding
/// declared: the lower plate's tangent ruling ends on the upper plate's
/// continued wall, and the cover takes each op past the crossing layer.
/// The union builds ([`two_cylinders_tangent_along_a_ruling_cover_the_stack`]
/// meters it); the subtract and intersect refuse
/// `FallbackExtentUnsupported`, the no-crossings path unable to certify
/// the continued walls' abutment
/// ([`the_stacks_subtract_and_intersect_stop_at_the_fallback_extent`]).
/// Undeclared, each op is the declared one bit for bit (D10).
#[test]
fn the_arc_joint_stacks_union_and_stop_at_the_fallback_extent() {
    for (lower, upper) in STACKS {
        let (p, q) = (plate(lower, 0.0), plate(upper, 1.0));
        assert_eq!(
            tangent_struts(&p),
            [(SurfaceKind::Cylinder, SurfaceKind::Cylinder)],
            "{lower:?}: the plate's one strut, between its two walls"
        );
        for (order, a, b) in [("P, Q", &p, &q), ("Q, P", &q, &p)] {
            let label = format!("{lower:?} under {upper:?}, {order}");
            let d = findings(a, b);
            let none = BooleanDeclarations::none();
            for (op, r, undeclared) in [
                (
                    "∪",
                    topo::union_with(a, b, &d, tol()),
                    topo::union_with(a, b, &none, tol()),
                ),
                (
                    "∖",
                    topo::subtract_with(a, b, &d, tol()),
                    topo::subtract_with(a, b, &none, tol()),
                ),
                (
                    "∩",
                    topo::intersect_with(a, b, &d, tol()),
                    topo::intersect_with(a, b, &none, tol()),
                ),
            ] {
                if op == "∪" {
                    assert!(
                        matches!(r, Ok(BooleanResult::Body(_))),
                        "{label}, ∪ declared: builds: {r:?}"
                    );
                } else {
                    assert!(
                        matches!(r, Err(BooleanError::FallbackExtentUnsupported { .. })),
                        "{label}, {op} declared: {r:?}"
                    );
                }
                assert_eq!(
                    outcome(&undeclared),
                    outcome(&r),
                    "{label}, {op} undeclared: the declared outcome"
                );
            }
        }
    }
}

/// **Two cylinders tangent along a ruling cover the stack**, in every
/// stack and both member orders: the union builds, valid at tier 3 and
/// 3′, at the two outlines' closed-form areas summed. The lower plate
/// carries its strut, one tangent ruling between two cylinders, and
/// nothing else tangent.
///
/// Undeclared, each is the declared body
/// ([`the_arc_joint_stacks_union_and_stop_at_the_fallback_extent`]).
#[test]
fn two_cylinders_tangent_along_a_ruling_cover_the_stack() {
    for (lower, upper) in STACKS {
        let want = area(lower) + area(upper);
        for (label, bb) in stacks(lower, upper) {
            let v = topo::mass_properties(&bb.body, tol()).unwrap().volume;
            assert!(
                (v - want).abs() <= 1e-12 * want,
                "{label}: the stack's volume {v} against the closed form {want}"
            );
        }
    }
}

/// **The covered stack is where its point probes say**: every probe the
/// outlines' own segments place at least 0.05 inside or outside the
/// plate at its height, at the plates' mid-heights, beside the mating
/// plane and above and below the stack, is where the winding number of
/// the union's watertight mesh puts it, in every stack and both member
/// orders.
#[test]
#[ignore = "the covered stack's tessellation refuses CertificateExceeded on the lower plate's wall at 5e-3; run with --run-ignored only"]
fn the_covered_stack_meets_its_point_probes() {
    for (lower, upper) in STACKS {
        for (label, bb) in stacks(lower, upper) {
            let mesh = mesh::tessellate(&bb.body, 5e-3, tol()).expect("tessellates");
            mesh::validate::check_mesh(&mesh).expect("watertight");
            let mut probed = [0_usize; 2];
            for (z, plate) in [
                (0.5, Some(lower)),
                (0.97, Some(lower)),
                (1.03, Some(upper)),
                (1.5, Some(upper)),
                (-0.1, None),
                (2.1, None),
            ] {
                for i in 0..=12 {
                    for j in 0..=12 {
                        let (x, y) = (-1.5 + 0.3 * f64::from(i), -2.5 + 0.44 * f64::from(j));
                        let (truth, clearance) =
                            plate.map_or((false, f64::INFINITY), |shape| in_outline(shape, x, y));
                        if clearance < 0.05 {
                            continue;
                        }
                        let w = winding(&mesh, [x, y, z]);
                        assert!(
                            (w - if truth { 1.0 } else { 0.0 }).abs() < 1e-6,
                            "{label}: ({x}, {y}, {z}) winds {w}, inside {truth}"
                        );
                        probed[usize::from(truth)] += 1;
                    }
                }
            }
            assert!(
                probed.iter().all(|&n| n > 50),
                "{label}: probes on both sides {probed:?}"
            );
        }
    }
}

/// **The stack's subtract and intersect stop
/// at the fallback extent**, as the rounded stack's do
/// (`reach_continuation`;
/// `rounded-stack-subtract-and-intersect-refuse-fallback-extent`): the
/// cover takes them past the crossing layer, and the no-crossings path
/// cannot certify the continued walls' abutment.
#[test]
fn the_stacks_subtract_and_intersect_stop_at_the_fallback_extent() {
    for shape in [Outline::Inside, Outline::S] {
        let (p, q) = (plate(shape, 0.0), plate(shape, 1.0));
        for (order, a, b) in [("P, Q", &p, &q), ("Q, P", &q, &p)] {
            let d = findings(a, b);
            for (op, r) in [
                ("∖", topo::subtract_with(a, b, &d, tol())),
                ("∩", topo::intersect_with(a, b, &d, tol())),
            ] {
                assert!(
                    matches!(r, Err(BooleanError::FallbackExtentUnsupported { .. })),
                    "{shape:?}, {order}, {op}: {r:?}"
                );
            }
        }
    }
}

const R: f64 = 0.5;
const H: f64 = 2.0;

/// The capsule revolved from one profile: a tube of radius [`R`] on
/// `z ∈ [0, H]` and a half ball on it, the ball's quarter arc authored
/// as the side's tangent continuation and its joint minted
/// `TangentIntersection` (two semicircles).
fn capsule() -> AtRestBody<f64> {
    let t = tol();
    let lp: ProfileLoop<f64> = Open
        .at(Point2::new(0.0, 0.0))
        .line_to(Point2::new(R, 0.0), t)
        .expect("the floor")
        .line_to(Point2::new(R, H), t)
        .expect("the side")
        .tangent()
        .tangent_arc_to(Point2::new(0.0, H + R), t)
        .expect("the quarter arc continues the side tangent")
        .line_to(Start, t)
        .expect("down the axis")
        .into();
    let pr = profile::Profile::new(profile::SketchPlane::xy(), vec![lp])
        .validate(t)
        .unwrap();
    let axis = sweep::RevolveAxis {
        origin: Point2::new(0.0, 0.0),
        dir: geom_core::Vec2::new(0.0, 1.0),
    };
    let at0 = sweep::revolve(&pr, axis, Revolution::Full, t).unwrap().body;
    let turn = Affine3::rotation_about_axis(Point3::origin(), Vec3::unit_x(), PI / 2.0);
    let mut c = topo::transform_rigid(&at0, &turn, t).unwrap();
    c.merge_coplanar_faces(t).unwrap();
    finished("the capsule", c, t)
}

/// A rod of radius [`R`] on the capsule's axis, `z ∈ [z0, z0 + len]`.
fn rod(z0: f64, len: f64) -> AtRestBody<f64> {
    let t = tol();
    let lp = profile::circle(Point2::new(0.0, 0.0), R, t).unwrap();
    let plane = profile::SketchPlane::new(Affine3::translation(Vec3::new(0.0, 0.0, z0)));
    let p = profile::Profile::new(plane, vec![lp.into()])
        .validate(t)
        .unwrap();
    let rod = extrude(
        &p,
        Extrusion::Distance {
            depth: len,
            side: ExtrudeSide::Along,
        },
        t,
    )
    .unwrap()
    .body;
    finished("the rod", rod, t)
}

/// Every cylinder face of `x` against every one of `y`, continuations.
fn walls_continued(x: &Body<f64>, y: &Body<f64>) -> BooleanDeclarations {
    let walls = |b: &Body<f64>| -> Vec<FaceKey> {
        b.faces()
            .filter(|(_, f)| {
                b.get_surface(f.surface).map(geom::Surface::kind) == Some(SurfaceKind::Cylinder)
            })
            .map(|(k, _)| k)
            .collect()
    };
    let mut d = BooleanDeclarations::none();
    for fa in walls(x) {
        for fb in walls(y) {
            d.coincident_faces.push(FacePairDeclaration::new(
                fa,
                fb,
                BooleanCoincidence::Continuation,
            ));
        }
    }
    d
}

/// **The capsule's sphere × cylinder strut has no witness yet**: a
/// coaxial rod of the capsule's radius, its wall declared a
/// continuation of the capsule's, reaching the joint circle past it
/// (into the half ball, or past the whole ball) or starting from it
/// refuses `CurvedPierceUnsupported` in both member orders. Ending ON
/// the joint, the two member orders part: rod first, the union builds,
/// the capsule with the rod's stub below it; capsule first, the sector
/// coincidence at the joint escalates on `bool_wedge_reflex`. With the
/// strut column's sphere row admitted each pose refuses at a later door
/// instead (the join's `CurvedBooleanUnsupported`, the interior-loop
/// guard's `CurvedPairUnsupported`, or an escalated sector coincidence),
/// so that row waits for a witness. Undeclared, each is the declared
/// outcome (D10). A rod stopping short of the joint, which no cover is
/// asked about, builds at its closed form.
#[test]
fn the_capsules_strut_waits_at_the_crossing_layer() {
    let cap = capsule();
    let struts = tangent_struts(&cap);
    assert!(
        struts.len() == 2
            && struts.iter().all(|&(x, y)| {
                [x, y].contains(&SurfaceKind::Cylinder) && [x, y].contains(&SurfaceKind::Sphere)
            }),
        "the capsule's joint, two semicircles between its tube and half ball: {struts:?}"
    );
    let capsule_volume = PI * R * R * H + 2.0 / 3.0 * PI * R.powi(3);
    for (pose, z0, len) in [
        ("ending on the joint", -1.0, 1.0 + H),
        ("into the half ball", -1.0, 1.0 + H + R / 2.0),
        ("over the half ball", H / 2.0, H / 2.0 + 2.0 * R),
        ("from the joint", H, 2.0 * R),
    ] {
        let rod = rod(z0, len);
        for (order, a, b) in [("capsule, rod", &cap, &rod), ("rod, capsule", &rod, &cap)] {
            let declared = topo::union_with(a, b, &walls_continued(a, b), tol());
            match (pose, order, &declared) {
                ("ending on the joint", "rod, capsule", Ok(BooleanResult::Body(bb))) => {
                    let v = topo::mass_properties(&bb.body, tol()).unwrap().volume;
                    let want = capsule_volume + PI * R * R;
                    assert!(
                        (v - want).abs() <= 1e-12 * want,
                        "{pose}, {order}: {v} against {want}"
                    );
                }
                (
                    "ending on the joint",
                    "capsule, rod",
                    Err(BooleanError::Escalated { diag, .. }),
                ) => assert_eq!(
                    diag.predicate,
                    Some("bool_wedge_reflex"),
                    "{pose}, {order}: the joint's sector coincidence: {declared:?}"
                ),
                (
                    "into the half ball" | "over the half ball" | "from the joint",
                    _,
                    Err(BooleanError::CurvedPierceUnsupported { .. }),
                ) => {}
                _ => panic!("{pose}, {order}, declared: {declared:?}"),
            }
            let r = topo::union(a, b, tol());
            assert_eq!(
                outcome(&r),
                outcome(&declared),
                "{pose}, {order}, undeclared: the declared outcome"
            );
        }
    }
    let rod = rod(-1.0, 1.0 + H / 2.0);
    let want = PI * R * R * (H + 1.0) + 2.0 / 3.0 * PI * R.powi(3);
    for (order, a, b) in [("capsule, rod", &cap, &rod), ("rod, capsule", &rod, &cap)] {
        let r = topo::union_with(a, b, &walls_continued(a, b), tol());
        let Ok(BooleanResult::Body(bb)) = r else {
            panic!("short of the joint, {order}: builds: {r:?}");
        };
        let v = topo::mass_properties(&bb.body, tol()).unwrap().volume;
        assert!(
            (v - want).abs() <= 1e-12 * want,
            "short of the joint, {order}: {v} against {want}"
        );
    }
}
