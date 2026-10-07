//! **A vertex read by two sector passes builds where its pierce only
//! touches, and refuses typed where its pierce crosses or it pierces
//! twice, in every op and both operand orders.**
//!
//! A vertex-on-face pass hangs struts at its piercing vertex where the
//! vertex crosses the face, and only there. Where the other solid holds
//! its own contact at the piercing point, the vertex is read again: by
//! a pair, where a vertex of the other solid rests on the face, or by a
//! second pierce, where two of its faces meet there. A touching vertex
//! read again by a pair builds, its edges classed against the face and
//! the pair together; a crossing one, and a second pierce, refuse
//! `VertexReadTwice` before any pass writes at the vertex.
//!
//! The scenes, each at every pose, pyramids standing on their apexes
//! at `MEET` or hanging from it inside the plate:
//! - **the arches**: the plate united with three standing pyramids,
//!   which keeps their apexes on its top with no vertex of the top
//!   there, against a fourth; **one standing pyramid**, the same with
//!   one arch; the fourth **over** the arch, the two crossing;
//! - a pyramid **hanging** inside the plate below the arch;
//! - **the cavity**: the plate less a hanging pyramid, against a
//!   standing one, a hanging one crossing it, and one hanging inside
//!   its void through its floor;
//! - **the strut hangers**: a prism through the top (`meeting::wedge`,
//!   and `meeting::leaned`) beside one standing pyramid, whose minted
//!   vertex at `MEET` crosses the top before the pair reads it;
//! - **two blocks in face contact**, one body built through the Euler
//!   doors, against a standing pyramid and against the prism, whose
//!   vertex at `MEET` pierces both blocks' faces;
//! - the plate alone, and the arches without it, read once.
//!
//! The operands' own contacts do not reach a result
//! (`work/wire/a-boolean-drops-its-operands-own-contact-records.md`), so
//! tier 3′ is asserted where the result holds no such contact and
//! otherwise refuses only those, at `MEET`, or a touch there the
//! census misreads (`three_prime`). Each result's material near `MEET`
//! is the op's over the operands' (`material_holds`), and each class
//! the naming reads there is the other operand's containment
//! (`classes_hold`).
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::common;

use common::meeting::{
    MEET, PLATE, Pose, at, leaned, orders, posed_box, posed_boxes, posed_prism, posed_pyramid,
    poses, wedge,
};
use geom_core::{Band, Point3, Tol, Vec3};
use topo::{
    AtRestBody, BooleanError, BooleanResult, CensusContact, Operand, SectorRead, SideCode,
    SolidContainment, ValidationError, intersect, mass_properties, point_in_solid, readback,
    subtract, union, validate_geometric, validate_pseudomanifold,
};

fn t() -> Tol {
    Tol::witness()
}

/// A pyramid with its apex at [`MEET`] and a horizontal base `rise`
/// above it (below, where `rise` is negative): two corners at radius
/// `r` 15° either side of `bearing` (degrees) and one at `0.6 r` on it.
fn pyramid(bearing: f64, rise: f64, r: f64, pose: &Pose) -> AtRestBody<f64> {
    let corner = |d: f64, r: f64| {
        let (s, c) = (bearing + d).to_radians().sin_cos();
        [r.mul_add(c, MEET[0]), r.mul_add(s, MEET[1]), MEET[2] + rise]
    };
    // Counterclockwise seen from the apex's side.
    let turn = if rise > 0.0 { 15.0 } else { -15.0 };
    let base = [corner(turn, r), corner(-turn, r), corner(0.0, 0.6 * r)];
    posed_pyramid(&base, MEET, pose, t())
}

fn standing(bearing: f64, rise: f64, r: f64, pose: &Pose) -> AtRestBody<f64> {
    pyramid(bearing, rise, r, pose)
}

fn hanging(bearing: f64, drop: f64, r: f64, pose: &Pose) -> AtRestBody<f64> {
    pyramid(bearing, -drop, r, pose)
}

/// A pyramid hanging from [`MEET`] inside the cone of the cavity's void
/// (`hanging(120, 0.5, 0.4)`): each base corner mixes the void's three,
/// 3 : 1 : 1, so its edges at `MEET` run into the void, and it reaches
/// 1.4 times as deep, through the void's floor.
fn in_the_void(pose: &Pose) -> AtRestBody<f64> {
    let corner = |d: f64, r: f64| {
        let (s, c) = (120.0 + d).to_radians().sin_cos();
        [r * c, r * s, -0.5]
    };
    let void = [corner(-15.0, 0.4), corner(15.0, 0.4), corner(0.0, 0.24)];
    let base = [0, 1, 2].map(|i| {
        let mix =
            |k: usize| (3.0 * void[i][k] + void[(i + 1) % 3][k] + void[(i + 2) % 3][k]) * 1.4 / 5.0;
        [0, 1, 2].map(|k| MEET[k] + mix(k))
    });
    posed_pyramid(&base, MEET, pose, t())
}

fn built(what: &str, r: Result<BooleanResult<f64>, BooleanError>) -> AtRestBody<f64> {
    match r {
        Ok(BooleanResult::Body(r)) => r.body,
        other => panic!("{what}: {:?}", other.map(|_| ())),
    }
}

fn volume(b: &AtRestBody<f64>) -> f64 {
    mass_properties(b, t()).unwrap().volume
}

/// Every refusal of tier 3′ is a contact at [`MEET`] no record backs,
/// an operand's own, which the result does not carry
/// (`work/wire/a-boolean-drops-its-operands-own-contact-records.md`),
/// or a touch there the census misreads: a pyramid notched by another,
/// a saddle corner on the plate
/// (`work/contact/a-touch-at-a-saddle-corner-refuses-unanalysed.md`),
/// and a pyramid on a solid that touches itself there, read from its
/// vertex alone
/// (`work/contact/a-solid-touching-itself-at-a-vertex-reads-its-star-from-the-vertex-alone.md`).
/// [`material_holds`] reads those results right. Returns whether 3′
/// held.
fn three_prime(what: &str, r: &topo::BooleanBody<f64>, pose: &Pose) -> bool {
    let Err(errors) = validate_pseudomanifold(&r.body, &r.contacts, t()) else {
        return true;
    };
    let meet = at(pose.at(MEET));
    let at_meet = |v| at(readback::vertex_point(&r.body, v).unwrap()) == meet;
    for e in &errors {
        let contact = match e {
            ValidationError::UndeclaredContact { contact, .. } => contact,
            ValidationError::CensusUndecidable { what: class, .. }
                if class.starts_with("they touch at a corner neither convex nor concave")
                    || class.starts_with("one passes into the other where they touch") =>
            {
                continue;
            }
            _ => panic!("{what}: tier 3′ refuses only an operand's own contact, got {e:?}"),
        };
        let ok = match *contact {
            CensusContact::VertexVertex { a, b } => at_meet(a) && at_meet(b),
            CensusContact::VertexOnFace { vertex, .. } => at_meet(vertex),
            _ => false,
        };
        assert!(
            ok,
            "{what}: tier 3′ refuses a contact away from MEET, {e:?}"
        );
    }
    false
}

/// Every class the result's naming read for an edge at [`MEET`] is the
/// other operand's containment of a point along the edge.
fn classes_hold(
    what: &str,
    r: &topo::BooleanBody<f64>,
    a: &AtRestBody<f64>,
    b: &AtRestBody<f64>,
    pose: &Pose,
) {
    let meet = at(pose.at(MEET));
    let band = Band::linear(t()).unwrap();
    for row in &r.naming.edge_classes {
        let (own, other) = match row.operand {
            Operand::A => (a, b),
            Operand::B => (b, a),
        };
        // A vertex the sweep minted is no key of the operand.
        let Ok(p) = readback::vertex_point(own, row.vertex) else {
            continue;
        };
        if at(p) != meet {
            continue;
        }
        let edge = own.get_edge(row.edge).unwrap();
        let plus = own.get_half_edge(edge.he_plus).unwrap();
        let far = if row.starts {
            own.get_half_edge(plus.next).unwrap().start
        } else {
            plus.start
        };
        let d = readback::vertex_point(own, far).unwrap() - p;
        let q = p + d * (0.02 / d.norm());
        let want = match point_in_solid(other, q, band, t()).unwrap() {
            SolidContainment::In => SideCode::In,
            SolidContainment::Out => SideCode::Out,
            SolidContainment::OnBoundary => SideCode::On,
        };
        assert_eq!(row.class, want, "{what}: {row:?} against the other operand");
    }
}

/// Points a tenth from [`MEET`]: towards 32 directions spread over the
/// sphere, and towards the mean of each vertex near it of `x` or `y`
/// and that vertex's two nearest, which lands inside each pyramid
/// (its base corners lie closer to each other than to another's).
fn samples(x: &AtRestBody<f64>, y: &AtRestBody<f64>, pose: &Pose) -> Vec<Point3<f64>> {
    let meet = pose.at(MEET);
    let mut dirs: Vec<Vec3<f64>> = (0..32)
        .map(|k| {
            let z = 1.0 - (2.0 * f64::from(k) + 1.0) / 32.0;
            let (s, c) = (2.399_963 * f64::from(k)).sin_cos();
            let r = z.mul_add(-z, 1.0).sqrt();
            Vec3::new(r * c, r * s, z)
        })
        .collect();
    for body in [x, y] {
        let near: Vec<Vec3<f64>> = body
            .vertices()
            .map(|(k, _)| readback::vertex_point(body, k).unwrap() - meet)
            .filter(|d| d.norm() > 1e-6 && d.norm() < 1.0)
            .collect();
        for (i, &d) in near.iter().enumerate() {
            let mut by: Vec<Vec3<f64>> = (0..near.len())
                .filter(|&j| j != i)
                .map(|j| near[j])
                .collect();
            by.sort_by(|e, f| (*e - d).norm().total_cmp(&(*f - d).norm()));
            if let [e, f, ..] = by[..] {
                dirs.push(d + e + f);
            }
        }
    }
    dirs.into_iter()
        .map(|d| meet + d * (0.1 / d.norm()))
        .collect()
}

fn inside(body: &AtRestBody<f64>, q: Point3<f64>) -> Option<bool> {
    let band = Band::linear(t()).unwrap();
    match point_in_solid(body, q, band, t()).unwrap() {
        SolidContainment::In => Some(true),
        SolidContainment::Out => Some(false),
        SolidContainment::OnBoundary => None,
    }
}

/// The result's material at each probe `(q, in x, in y)` is `keep`'s
/// over the operands', wherever the result does not read `q` on its
/// boundary; an empty result holds none.
fn material_holds(
    what: &str,
    keep: impl Fn(bool, bool) -> bool,
    r: Option<&AtRestBody<f64>>,
    probes: &[(Point3<f64>, bool, bool)],
) {
    for &(q, x, y) in probes {
        let got = r.map_or(Some(false), |r| inside(r, q));
        if let Some(got) = got {
            assert_eq!(got, keep(x, y), "{what}: material at {q:?}");
        }
    }
}

/// Every op on `(x, y)`, in both orders, builds at tier 3, with tier 3′
/// as [`three_prime`] says, its naming's classes holding
/// ([`classes_hold`]), and volumes that add up: `|x ∪ y| + |x ∩ y| =
/// |x| + |y|` and `|x − y| + |x ∩ y| = |x|`, each alike in both orders.
/// Returns how many results held 3′.
fn builds(label: &str, x: &AtRestBody<f64>, y: &AtRestBody<f64>, pose: &Pose) -> usize {
    let mut held = 0;
    let mut v = [0.0; 6];
    // Each op's keep over (in x, in y).
    let probes: Vec<_> = samples(x, y, pose)
        .into_iter()
        .filter_map(|q| Some((q, inside(x, q)?, inside(y, q)?)))
        .collect();
    let x_less: fn(bool, bool) -> bool = |x, y| x && !y;
    let y_less: fn(bool, bool) -> bool = |x, y| y && !x;
    let or: fn(bool, bool) -> bool = |x, y| x || y;
    let and: fn(bool, bool) -> bool = |x, y| x && y;
    for (k, (what, keep, a, b, r)) in [
        ("x − y", x_less, x, y, subtract(x, y, t())),
        ("y − x", y_less, y, x, subtract(y, x, t())),
        ("x ∪ y", or, x, y, union(x, y, t())),
        ("y ∪ x", or, y, x, union(y, x, t())),
        ("x ∩ y", and, x, y, intersect(x, y, t())),
        ("y ∩ x", and, y, x, intersect(y, x, t())),
    ]
    .into_iter()
    .enumerate()
    {
        let what = format!("{label}, {}, {what}", pose.label);
        v[k] = match r {
            Ok(BooleanResult::Body(r)) => {
                assert_eq!(validate_geometric(&r.body, t()), Ok(()), "{what}: tier 3");
                material_holds(&what, keep, Some(&r.body), &probes);
                classes_hold(&what, &r, a, b, pose);
                held += usize::from(three_prime(&what, &r, pose));
                volume(&r.body)
            }
            Ok(BooleanResult::Empty) => {
                material_holds(&what, keep, None, &probes);
                0.0
            }
            Err(e) => panic!("{what}: builds, got {e:?}"),
        };
    }
    let (vx, vy) = (volume(x), volume(y));
    let what = format!("{label}, {}", pose.label);
    let [xy, yx, u, u2, i, i2] = v;
    for (sum, want, which) in [
        (u + i, vx + vy, "|x ∪ y| + |x ∩ y|"),
        (xy + i, vx, "|x − y| + |x ∩ y|"),
        (yx + i, vy, "|y − x| + |x ∩ y|"),
        (u2, u, "|y ∪ x|"),
        (i2, i, "|y ∩ x|"),
    ] {
        assert!(
            (sum - want).abs() < 1e-9,
            "{what}: {which} {sum}, want {want}"
        );
    }
    held
}

/// Every op on `(x, y)`, in both orders, refuses `VertexReadTwice` on
/// `x`'s vertex at [`MEET`], with a pierce for its first read and
/// `second` for its next. Where `minted`, the sweep mints that vertex,
/// splitting an edge of `x` there, so `x` does not hold it.
fn refuses(
    label: &str,
    x: &AtRestBody<f64>,
    y: &AtRestBody<f64>,
    pose: &Pose,
    second: &str,
    minted: bool,
) {
    let meet = at(pose.at(MEET));
    let at_meet = |body: &AtRestBody<f64>, v| at(readback::vertex_point(body, v).unwrap()) == meet;
    for (what, x_is, r) in [
        ("x − y", Operand::A, subtract(x, y, t())),
        ("y − x", Operand::B, subtract(y, x, t())),
        ("x ∪ y", Operand::A, union(x, y, t())),
        ("y ∪ x", Operand::B, union(y, x, t())),
        ("x ∩ y", Operand::A, intersect(x, y, t())),
        ("y ∩ x", Operand::B, intersect(y, x, t())),
    ] {
        let what = format!("{label}, {}, {what}", pose.label);
        match r {
            Err(BooleanError::VertexReadTwice {
                operand,
                vertex,
                reads: [SectorRead::Pierce(first), next],
            }) => {
                assert_eq!(operand, x_is, "{what}: the operand read twice");
                if minted {
                    assert!(
                        x.vertices().all(|(k, _)| k != vertex),
                        "{what}: the vertex read twice is minted"
                    );
                } else {
                    assert!(
                        at_meet(x, vertex),
                        "{what}: the vertex read twice is at MEET"
                    );
                }
                match (second, next) {
                    ("pair", SectorRead::Pair(partner)) => {
                        assert!(at_meet(y, partner), "{what}: its partner is at MEET");
                    }
                    ("pierce", SectorRead::Pierce(face)) => {
                        assert_ne!(first, face, "{what}: it pierces two faces");
                    }
                    _ => panic!("{what}: its second read is a {second}, got {next:?}"),
                }
            }
            other => panic!(
                "{what}: refuses VertexReadTwice with a pierce first, got {:?}",
                other.map(|_| ())
            ),
        }
    }
}

/// The scenes at one pose.
struct Scene {
    plate: AtRestBody<f64>,
    cone: AtRestBody<f64>,
    over: AtRestBody<f64>,
    hang: AtRestBody<f64>,
    hang_over: AtRestBody<f64>,
    in_void: AtRestBody<f64>,
    bare: AtRestBody<f64>,
    arches: AtRestBody<f64>,
    one: AtRestBody<f64>,
    cavity: AtRestBody<f64>,
    blocks: AtRestBody<f64>,
    prism: AtRestBody<f64>,
    leaned: AtRestBody<f64>,
}

impl Scene {
    fn at(pose: &Pose) -> Self {
        let plate = posed_box("the plate", PLATE, pose, t());
        let [first, rest @ ..] = [60.0, 180.0, 300.0].map(|b| standing(b, 0.5, 0.4, pose));
        let bare = rest
            .iter()
            .fold(first, |u, a| built("the arches", union(&u, a, t())));
        Self {
            cone: standing(240.0, 0.7, 0.5, pose),
            over: standing(50.0, 0.7, 0.5, pose),
            hang: hanging(240.0, 0.6, 0.5, pose),
            hang_over: hanging(130.0, 0.6, 0.5, pose),
            in_void: in_the_void(pose),
            arches: built("the plate and the arches", union(&plate, &bare, t())),
            one: built(
                "the plate and one arch",
                union(&plate, &standing(60.0, 0.5, 0.4, pose), t()),
            ),
            cavity: built(
                "the plate less a hanging pyramid",
                subtract(&plate, &hanging(120.0, 0.5, 0.4, pose), t()),
            ),
            blocks: posed_boxes(
                "two blocks in face contact",
                &[PLATE, [(0.5, 2.5), (0.5, 1.5), (1.0, 1.5)]],
                pose,
                t(),
            ),
            prism: posed_prism(&wedge(200.0, 260.0, 0), pose, t()),
            leaned: posed_prism(&leaned(200.0, 260.0, 0, 300.0), pose, t()),
            bare,
            plate,
        }
    }
}

/// **A vertex that touches a face and pairs with a vertex resting on it
/// builds sound in every op**: a standing pyramid beside the arch, one
/// crossing it, one hanging inside the plate below it, and a standing
/// and a crossing hanging pyramid against the cavity, and one running
/// into its void. At rest; every
/// pose is the slow matrix's.
#[test]
fn a_touching_vertex_paired_on_the_face_builds_sound_in_every_op() {
    let pose = &Pose::rest();
    let s = Scene::at(pose);
    builds("one standing pyramid", &s.cone, &s.one, pose);
    builds("a standing pyramid over the arch", &s.over, &s.one, pose);
    builds("a pyramid hanging below the arch", &s.hang, &s.one, pose);
    builds(
        "a standing pyramid over the cavity",
        &s.cone,
        &s.cavity,
        pose,
    );
    builds(
        "a hanging pyramid across the cavity",
        &s.hang_over,
        &s.cavity,
        pose,
    );
    builds(
        "a hanging pyramid into the void",
        &s.in_void,
        &s.cavity,
        pose,
    );
}

/// **A vertex that crosses a face it is paired on, or pierces two
/// faces, refuses typed in every op**: the prisms beside one standing
/// pyramid, and a pyramid and the prism against two blocks in face
/// contact.
#[test]
fn a_vertex_crossing_a_face_it_pairs_on_or_piercing_two_refuses_typed_in_every_op() {
    for pose in poses() {
        let s = Scene::at(&pose);
        refuses(
            "a prism through the top",
            &s.prism,
            &s.one,
            &pose,
            "pair",
            true,
        );
        refuses(
            "a leaned prism through the top",
            &s.leaned,
            &s.one,
            &pose,
            "pair",
            true,
        );
        refuses("two blocks", &s.cone, &s.blocks, &pose, "pierce", false);
        // The prism's edge crosses the contact at `MEET`, so its first
        // pierce would hang struts there: only a refusal before that
        // pass writes keeps the second pierce off the written orbit.
        refuses(
            "a prism through two blocks",
            &s.prism,
            &s.blocks,
            &pose,
            "pierce",
            true,
        );
    }
}

/// **Every scene, at every pose, in every op and both orders, builds
/// sound or refuses typed**: 21 scenes, 630 op cells.
#[test]
fn every_scene_builds_sound_or_refuses_typed_at_every_pose() {
    let mut held = 0;
    for pose in poses() {
        let s = Scene::at(&pose);
        for (label, x, y) in [
            ("the arches", &s.cone, &s.arches),
            ("one standing pyramid", &s.cone, &s.one),
            ("over the arch", &s.over, &s.one),
            ("over the arches", &s.over, &s.arches),
            ("hanging below the arch", &s.hang, &s.one),
            ("hanging across below the arch", &s.hang_over, &s.one),
            ("hanging below the arches", &s.hang, &s.arches),
            ("standing over the cavity", &s.cone, &s.cavity),
            ("hanging below the cavity", &s.hang, &s.cavity),
            ("hanging across the cavity", &s.hang_over, &s.cavity),
            ("hanging into the void", &s.in_void, &s.cavity),
            ("standing on the plate", &s.cone, &s.plate),
            ("hanging in the plate", &s.hang, &s.plate),
            ("the bare arches", &s.cone, &s.bare),
            ("hanging below the bare arches", &s.hang, &s.bare),
            ("a prism through the plate", &s.prism, &s.plate),
            ("a leaned prism through the plate", &s.leaned, &s.plate),
        ] {
            held += builds(label, x, y, &pose);
        }
        refuses(
            "a prism through the top",
            &s.prism,
            &s.one,
            &pose,
            "pair",
            true,
        );
        refuses(
            "a leaned prism through the top",
            &s.leaned,
            &s.one,
            &pose,
            "pair",
            true,
        );
        refuses("two blocks", &s.cone, &s.blocks, &pose, "pierce", false);
        refuses(
            "a prism through two blocks",
            &s.prism,
            &s.blocks,
            &pose,
            "pierce",
            true,
        );
    }
    assert!(held > 0, "some result holds tier 3′");
}

/// **Standing pyramids folded onto the plate one at a time build in
/// every member order**, at tier 3 and the closed-form volume; tier 3′
/// refuses only the earlier steps' contacts at `MEET`, which the last
/// step's record does not carry (`three_prime`).
#[test]
fn standing_pyramids_folded_onto_the_plate_build_in_every_member_order() {
    for pose in poses() {
        let mut members = vec![posed_box("the plate", PLATE, &pose, t())];
        members.extend([60.0, 240.0].map(|b| standing(b, 0.5, 0.4, &pose)));
        let want: f64 = members.iter().map(volume).sum();
        for order in orders(members.len()) {
            let what = format!("{}, member order {order:?} (0 = plate)", pose.label);
            let mut last = None;
            let mut body = members[order[0]].clone();
            for (k, &i) in order.iter().enumerate().skip(1) {
                match union(&body, &members[i], t()) {
                    Ok(BooleanResult::Body(r)) => {
                        body = r.body.clone();
                        last = Some(r);
                    }
                    other => panic!("{what}: step {k}, {:?}", other.map(|_| ())),
                }
            }
            let last = last.unwrap();
            assert_eq!(validate_geometric(&body, t()), Ok(()), "{what}: tier 3");
            three_prime(&what, &last, &pose);
            let v = volume(&body);
            assert!((v - want).abs() < 1e-9, "{what}: volume {v}, want {want}");
        }
    }
}
