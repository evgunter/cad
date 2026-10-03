//! A curved wall touching a plane face's interior along a ruling, with no
//! edge for the contact, reached through public doors and passed by tier 3
//! (`work/cleave/tier-3-passes-a-curved-wall-touching-a-plane-face-interior-along-a-line.md`).
//!
//! A rod (r = 0.5, z ∈ [0.25, 0.75]) rests on the side face y = 2 of a
//! 4 × 4 × 1 plate, and the union declares the wall × face pair
//! `Tangent`: two shells touching along the ruling x = 0, y = 2, which is
//! the declared-contact lane working as designed. A plain union with a box
//! bridging the rod's top to the plate's top (x ∈ [−1, 1], y ∈ [1, 2.5],
//! z ∈ [0.6, 1.6]) then fuses the two shells into one, and the wall still
//! touches the face along z ∈ [0.25, 0.6]. The door should refuse: what it
//! answers is a one-shell body whose material meets itself along a line
//! no edge carries.
//!
//! Today it answers, and these rows pin what it answers, at `f64` and at
//! `Interval`. Run with
//! `cargo nextest run -p sweep --test all --run-ignored only wall_face_tangent_reach`.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::{Affine3, Point2, Real, Tol, Vec3};
use profile::{Profile, SketchPlane, test_support::bulge_loop};
use sweep::{Extrusion, extrude};
use topo::boolean::{
    BooleanBody, BooleanDeclarations, BooleanResult, CarriedContacts, CarriedVf,
    FacePairDeclaration,
};
use topo::{
    AtRestPolicy, Body, ContactClass, ContactRecords, FaceKey, union, union_with,
    validate_geometric, validate_pseudomanifold,
};

type Loop = Vec<((f64, f64), f64)>;

fn ext<T: geom_core::Decide + AtRestPolicy>(loops: Vec<Loop>, z0: f64, h: f64) -> Body<T> {
    let tol = Tol::witness();
    let lps = loops
        .into_iter()
        .map(|l| {
            bulge_loop(
                l.into_iter()
                    .map(|((x, y), b)| (Point2::new(x, y).map(T::from_f64), T::from_f64(b)))
                    .collect(),
            )
        })
        .collect();
    let vp = Profile::new(SketchPlane::<T>::xy(), lps)
        .validate(tol)
        .unwrap();
    let b = extrude(&vp, Extrusion::Distance(T::from_f64(h)), tol)
        .unwrap()
        .body;
    let up = Affine3::translation(Vec3::new(0.0, 0.0, z0).map(T::from_f64));
    topo::transform_rigid(&b, &up, tol).unwrap()
}

fn plate<T: geom_core::Decide + AtRestPolicy>() -> Body<T> {
    let square = vec![
        ((-2.0, -2.0), 0.0),
        ((2.0, -2.0), 0.0),
        ((2.0, 2.0), 0.0),
        ((-2.0, 2.0), 0.0),
    ];
    ext(vec![square], 0.0, 1.0)
}

fn rod<T: geom_core::Decide + AtRestPolicy>() -> Body<T> {
    ext(vec![vec![((-0.5, 2.5), 1.0), ((0.5, 2.5), 1.0)]], 0.25, 0.5)
}

fn bridge<T: geom_core::Decide + AtRestPolicy>() -> Body<T> {
    let top = vec![
        ((-1.0, 1.0), 0.0),
        ((1.0, 1.0), 0.0),
        ((1.0, 2.5), 0.0),
        ((-1.0, 2.5), 0.0),
    ];
    ext(vec![top], 0.6, 1.0)
}

/// The plate's face on y = 2 and the rod's cylinder faces, read off the
/// `f64` build: every scalar builds the same arenas in the same order.
fn declared_pairs() -> Vec<FacePairDeclaration> {
    let p = plate::<f64>();
    let side = p
        .faces()
        .find(|(_, f)| {
            matches!(p.get_surface(f.surface), Some(geom::Surface::Plane { origin, normal, .. })
                if normal.y == 1.0 && origin.y == 2.0)
        })
        .unwrap()
        .0;
    let r = rod::<f64>();
    let walls: Vec<FaceKey> = r
        .faces()
        .filter(|(_, f)| {
            matches!(
                r.get_surface(f.surface),
                Some(geom::Surface::Cylinder { .. })
            )
        })
        .map(|(k, _)| k)
        .collect();
    assert!(!walls.is_empty(), "the rod has a wall");
    walls
        .into_iter()
        .map(|w| FacePairDeclaration::new(side, w, ContactClass::Tangent))
        .collect()
}

fn body_of<T: Real>(r: Result<BooleanResult<T>, topo::BooleanError>, what: &str) -> BooleanBody<T> {
    match r {
        Ok(BooleanResult::Body(b)) => b,
        other => panic!("{what}: {:?}", other.err()),
    }
}

/// The rod on the plate (declared), then the bridge: once plainly, once
/// carrying the first result's own contact records as `Tangent`.
fn bridged<T: geom_core::Decide + geom_core::Bounds + AtRestPolicy>()
-> (BooleanBody<T>, BooleanBody<T>, BooleanBody<T>) {
    let tol = Tol::witness();
    let decls = BooleanDeclarations {
        coincident_faces: declared_pairs(),
        ..Default::default()
    };
    let resting = body_of(
        union_with(&plate::<T>(), &rod::<T>(), &decls, tol),
        "the declared rest",
    );
    let plain = body_of(
        union(&resting.body, &bridge::<T>(), tol),
        "the plain bridge",
    );
    let carried = BooleanDeclarations {
        carried_a: CarriedContacts {
            vf: resting
                .contacts
                .a_on_b
                .iter()
                .chain(&resting.contacts.b_on_a)
                .map(|&rest| CarriedVf {
                    rest,
                    class: ContactClass::Tangent,
                })
                .collect(),
            ..Default::default()
        },
        ..Default::default()
    };
    let carried = body_of(
        union_with(&resting.body, &bridge::<T>(), &carried, tol),
        "the carried bridge",
    );
    (resting, plain, carried)
}

/// Edges with both vertices on the contact ruling x = 0, y = 2.
fn edges_on_ruling(b: &Body<f64>) -> usize {
    let on = |he| {
        let v = b.get_half_edge(he).unwrap().start;
        let p = b.get_point(b.get_vertex(v).unwrap().point).unwrap();
        p.x.abs() < 1e-12 && (p.y - 2.0).abs() < 1e-12
    };
    b.edges()
        .filter(|(_, e)| on(e.he_plus) && on(e.he_minus))
        .count()
}

/// Expected: the bridge refuses. Today it answers one shell of the true
/// volume, 18.2 + 0.85·π/8 = 18.5337942…, passed by tier 3; the
/// rod's wall still lies on the plane y = 2 (its axis is at y = 2.5,
/// radius 0.5) and no edge runs along the ruling. With no records the
/// census sees the rod's cap-rim vertex on the face's interior; with the
/// first result's records carried, tier 3′ over the result's own
/// records passes too.
#[test]
#[ignore = "pins a defect: a refusal is expected (work/cleave/tier-3-passes-a-curved-wall-touching-a-plane-face-interior-along-a-line.md)"]
fn a_bridge_over_a_declared_tangent_rest_answers_an_edgeless_contact() {
    let tol = Tol::witness();
    let (resting, plain, carried) = bridged::<f64>();
    assert_eq!(resting.body.shells().count(), 2, "the rest: two shells");
    // 16 (plate) + 3 (bridge) − 0.8 (bridge ∩ plate) + π/8 (rod) − π/8·0.15 (bridge ∩ rod).
    let want = 18.2 + std::f64::consts::PI / 8.0 * 0.85;
    for (label, b) in [("plain", &plain), ("carried", &carried)] {
        assert_eq!(b.body.shells().count(), 1, "{label}: one shell");
        let v = topo::mass_properties(&b.body, tol).unwrap().volume;
        assert!((v - want).abs() < 1e-9, "{label}: volume {v}, want {want}");
        assert_eq!(validate_geometric(&b.body, tol), Ok(()), "{label}: tier 3");
        assert_eq!(
            edges_on_ruling(&b.body),
            0,
            "{label}: an edge on the contact"
        );
        let walls: Vec<_> = b
            .body
            .faces()
            .filter_map(|(_, f)| match b.body.get_surface(f.surface) {
                Some(geom::Surface::Cylinder { origin, radius, .. }) => Some(origin.y - radius),
                _ => None,
            })
            .collect();
        assert!(
            !walls.is_empty() && walls.iter().all(|&y| (y - 2.0).abs() < 1e-12),
            "{label}: the wall's near ruling lies on y = 2: {walls:?}"
        );
    }
    assert_eq!(
        plain.contacts,
        ContactRecords::default(),
        "plain: no records"
    );
    assert!(
        validate_pseudomanifold(&plain.body, &ContactRecords::default(), tol).is_err(),
        "plain: the census sees the rim vertex"
    );
    assert_eq!(
        validate_pseudomanifold(&carried.body, &carried.contacts, tol),
        Ok(()),
        "carried: tier 3′ over its own records"
    );
}

/// [`a_bridge_over_a_declared_tangent_rest_answers_an_edgeless_contact`]
/// at `Interval`: the same answer, its volume enclosing the true one.
#[test]
#[ignore = "pins a defect: a refusal is expected (work/cleave/tier-3-passes-a-curved-wall-touching-a-plane-face-interior-along-a-line.md)"]
fn a_bridge_over_a_declared_tangent_rest_answers_an_edgeless_contact_at_interval() {
    use geom_core::{Bounds, Interval};
    let tol = Tol::witness();
    let (_, plain, carried) = bridged::<Interval>();
    let want = 18.2 + std::f64::consts::PI / 8.0 * 0.85;
    for (label, b) in [("plain", &plain), ("carried", &carried)] {
        assert_eq!(b.body.shells().count(), 1, "{label}: one shell");
        let v = topo::mass_properties(&b.body, tol).unwrap().volume;
        assert!(
            v.lo() - 1e-12 <= want && want <= v.hi() + 1e-12,
            "{label}: [{}, {}] misses {want}",
            v.lo(),
            v.hi()
        );
        assert_eq!(validate_geometric(&b.body, tol), Ok(()), "{label}: tier 3");
    }
    assert_eq!(
        validate_pseudomanifold(&carried.body, &carried.contacts, tol),
        Ok(()),
        "carried: tier 3′ over its own records"
    );
}
