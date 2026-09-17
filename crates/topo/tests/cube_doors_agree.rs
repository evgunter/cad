//! **The box doors build one body, and `geometric_cube` does not** —
//! the two halves of the partition `common::brick`'s doc asserts, made
//! falsifiable.
//!
//! The box doors reach one construction by two routes: `brick` and
//! `prism` over `prism_z`'s profile chain, `mapped_cube` and `cube_into`
//! over the fixed eight corners under a point map. Nothing makes
//! the two routes agree except that they run the same operators at the
//! same sites in the same order, so **any change to either route's
//! operator sequence, site addressing, corner order or description step
//! moves one dump and not the other** — including a change that leaves
//! every count intact. That is what the first row is written against,
//! and why it compares the derived `Debug` rather than counts: the arena
//! contents are the runtime value a bug moves, and `v8 e12 f6` survives
//! reordering, re-keying and a dropped description step alike.
//!
//! **The silent case is the one this file is for.** A dropped or added
//! description step reds two dozen rows across the tree on its own; a
//! re-ordering or re-keying that leaves every count and every consumer's
//! verdict intact reds nothing else, and is what these rows catch.
//!
//! The comparison is between live dumps in one process, with no
//! stored baseline, so a new `Body` field or a `Debug` reformat moves
//! every side identically and this file stays green.
//!
//! **A door-against-door row cannot see a change that moves every door
//! the same way**, which is what a shared core makes possible, so
//! [`assert_prism_shaped`] re-derives the body from its inputs instead:
//! the corner list in profile order, the arena orders the operator
//! sequence implies, and the loop sizes an N-gon prism has. That row is
//! also the only one that leaves the rectangle — it runs at `n = 3`,
//! `n = 5` and a reflex `n = 6`, under a shear, and at a second
//! `Decide` scalar.
//!
//! The negative row is the last one. A file that only pins agreement
//! goes green when someone makes every door identical by deleting the
//! distinction, which is the likeliest way to break it:
//! `geometric_cube` stops before `describe_as_intersections` and its
//! suites assert on exactly that absence.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_brep::EdgeDescription;
use geom_core::{Decide, Point3, Real};
use topo::Body;

use crate::common;

/// The affine map carrying the unit cube onto `[x] x [y] x [z]` — the
/// one map under which the fixed cube corners and `prism_z`'s
/// profile describe the same box.
fn onto(x: (f64, f64), y: (f64, f64), z: (f64, f64)) -> impl Fn(f64, f64, f64) -> Point3<f64> {
    move |u, v, w| {
        Point3::new(
            x.0 + u * (x.1 - x.0),
            y.0 + v * (y.1 - y.0),
            z.0 + w * (z.1 - z.0),
        )
    }
}

/// A shear of the unit cube, exact dyadic entries and determinant
/// `1.015625 > 0` so outward stays outward — a map no `onto` can
/// spell, and the class the mapped doors exist for.
fn sheared(u: f64, v: f64, w: f64) -> Point3<f64> {
    Point3::new(1.0 + u + 0.5 * v, 2.0 + v + 0.25 * w, -1.0 + w + 0.125 * u)
}

/// The rectangle profile of `[x] x [y]`, counterclockwise from +z.
///
/// This restates `brick`'s own corner ladder on purpose: `brick`'s
/// claim is that ITS ladder and the mapped doors' corners describe the
/// same box, and a guard that reached for `brick`'s expression to state
/// the profile would be comparing it against itself.
fn profile_of(x: (f64, f64), y: (f64, f64)) -> [(f64, f64); 4] {
    [(x.0, y.0), (x.1, y.0), (x.1, y.1), (x.0, y.1)]
}

/// The unit square, counterclockwise from +z: the profile the
/// cube doors' fixed corners spell, restated here for the same reason
/// [`profile_of`] restates `brick`'s.
const UNIT_SQUARE: [(f64, f64); 4] = [(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)];

/// A triangle, a pentagon and an L with one reflex corner — the
/// profiles that leave the rectangle the box doors are confined to.
/// Every one is counterclockwise from +z with no collinear run, so
/// every join is transverse and a described prism describes all of its
/// edges.
const TRIANGLE: [(f64, f64); 3] = [(0.0, 0.0), (2.0, 0.0), (0.5, 1.5)];
const PENTAGON: [(f64, f64); 5] = [
    (0.0, 0.0),
    (2.0, 0.0),
    (2.5, 1.25),
    (1.0, 2.0),
    (-0.5, 1.25),
];
/// Reflex at `(1.0, 1.0)`: the interior angle there is 270°.
const REFLEX_L: [(f64, f64); 6] = [
    (0.0, 0.0),
    (2.0, 0.0),
    (2.0, 1.0),
    (1.0, 1.0),
    (1.0, 2.0),
    (0.0, 2.0),
];

/// Every field of every arena, keys and slot versions included — the
/// derived `Debug`, so nothing is left out by the author choosing what
/// to look at.
fn dump<T: Real>(body: &Body<T>) -> String {
    format!("{body:#?}")
}

/// Per edge, in edge-arena order: whether its carrier's description is
/// the intrinsic `Intersection`, and whether its locus was declared.
fn carries<T: Real>(body: &Body<T>) -> Vec<(bool, bool)> {
    body.edges()
        .map(|(_, e)| {
            let curve = body
                .get_curve_geom(e.curve)
                .unwrap()
                .certified()
                .expect("a prism edge carries a certified curve");
            (
                matches!(curve.description(), EdgeDescription::Intersection { .. }),
                curve.authority().is_declared(),
            )
        })
        .collect()
}

/// **What a prism over `profile` x [`z`] through `map` IS**, stated
/// from the inputs rather than from a second builder's output.
///
/// Every other row in this file compares one door's dump against
/// another's, which cannot see a change that moves every door the same
/// way — and putting the family on one core is exactly the change that
/// would. So this row re-derives the body: the corner list in profile
/// order, the arena orders the operator sequence implies, the loop
/// sizes an N-gon prism has, and the description axis. A builder that
/// re-keys, re-orders or re-corners its output fails here with every
/// door still agreeing with every other.
///
/// `profile` may carry any `n >= 3` corners and may be reflex; `map`
/// may be any point map, shears included; `T` may be any `Decide`
/// scalar. Points are compared through their derived `Debug`, which is
/// the only equality a scalar lane without `PartialEq` offers and is
/// exact where one exists.
fn assert_prism_shaped<T: Decide>(
    body: &Body<T>,
    profile: &[(f64, f64)],
    z: (f64, f64),
    map: impl Fn(f64, f64, f64) -> Point3<T>,
    described: bool,
) {
    let n = profile.len();
    let bot: Vec<String> = profile
        .iter()
        .map(|&(x, y)| format!("{:?}", map(x, y, z.0)))
        .collect();
    let top: Vec<String> = profile
        .iter()
        .map(|&(x, y)| format!("{:?}", map(x, y, z.1)))
        .collect();

    // An N-gon prism is 2n vertices, 3n edges, n + 2 faces.
    let counts = (
        body.vertices().count(),
        body.points().count(),
        body.edges().count(),
        body.curves().count(),
        body.faces().count(),
        body.loops().count(),
        body.half_edges().count(),
        body.surfaces().count(),
    );
    assert_eq!(
        counts,
        (2 * n, 2 * n, 3 * n, 3 * n, n + 2, n + 2, 6 * n, n + 2),
        "the arenas of a prism over {n} corners"
    );

    // Vertex arena order: the bottom rim in profile order (the `mvfs`
    // seed, then the rim chain), then the top rim in profile order
    // (the struts).
    let point_of = |v: topo::VertexKey| {
        format!(
            "{:?}",
            body.get_point(body.get_vertex(v).unwrap().point).unwrap()
        )
    };
    let got_vertices: Vec<String> = body.vertices().map(|(k, _)| point_of(k)).collect();
    let want_vertices: Vec<String> = bot.iter().chain(top.iter()).cloned().collect();
    assert_eq!(
        got_vertices, want_vertices,
        "vertex arena order, and the corner each vertex carries"
    );

    // Edge arena order, each edge read start-to-end along `he_plus`:
    // the rim chain, the bottom cap's closing chord, the struts, then
    // one top chord per profile segment.
    let mut want_edges: Vec<(String, String)> = Vec::new();
    for i in 0..n - 1 {
        want_edges.push((bot[i].clone(), bot[i + 1].clone()));
    }
    want_edges.push((bot[n - 1].clone(), bot[0].clone()));
    for i in 0..n {
        want_edges.push((bot[i].clone(), top[i].clone()));
    }
    for i in 0..n {
        want_edges.push((top[i].clone(), top[(i + 1) % n].clone()));
    }
    let got_edges: Vec<(String, String)> = body
        .edges()
        .map(|(_, e)| {
            let start = body.get_half_edge(e.he_plus).unwrap().start;
            let end = body.half_edge_end(e.he_plus).unwrap();
            (point_of(start), point_of(end))
        })
        .collect();
    assert_eq!(
        got_edges, want_edges,
        "edge arena order, and each edge's endpoints along `he_plus`"
    );

    // Face arena order: the `mvfs` seed face survives as the top cap,
    // then the bottom cap, then one quad per profile segment.
    let ring: Vec<usize> = body
        .faces()
        .map(|(fk, _)| {
            body.half_edges()
                .filter(|(_, he)| body.get_loop(he.parent_loop).unwrap().face == fk)
                .count()
        })
        .collect();
    let mut want_ring = vec![n, n];
    want_ring.extend(std::iter::repeat_n(4, n));
    assert_eq!(
        ring, want_ring,
        "face arena order: the top cap, the bottom cap, then one quad per segment"
    );

    // The description axis is the caller's choice, and the one thing
    // `geometric_cube` does differently from every other door. A reflex
    // corner is still a transverse join, so a profile with no collinear
    // run either describes all 3n edges or describes none.
    let carried = carries(body);
    if described {
        assert!(
            carried.iter().all(|&(isect, decl)| isect && !decl),
            "a described prism cites the intersection its two surfaces \
             determine, on no one's authority: {carried:?}"
        );
    } else {
        assert!(
            carried.iter().all(|&(isect, decl)| !isect && decl),
            "an undescribed prism keeps every conventional chord: {carried:?}"
        );
    }
}

/// The box doors agree arena for arena at every box all of them can
/// spell — not only at the unit ranges, which is what makes this a
/// claim about the domain rather than about one fixture.
///
/// The roster below is hand-written and nothing ties it to
/// `common/mod.rs`: a builder added there joins this silently as no
/// builder at all. That is a known gap with its own row, not an
/// oversight.
#[test]
fn every_box_door_builds_one_body() {
    // (x, y, z). Two rows run off-origin on every axis and two keep
    // z0 = 0, which is the only z `prism` can spell — so `prism` is in
    // the comparison on rows 1 and 4 and out of it on rows 2 and 3.
    let boxes = [
        ((0.0, 1.0), (0.0, 1.0), (0.0, 1.0)),
        ((1.0, 3.0), (0.0, 3.0), (-0.5, 0.5)),
        ((-2.5, -0.25), (0.75, 4.0), (-1.0, -0.125)),
        ((0.0, 2.0), (0.0, 0.5), (0.0, 7.25)),
    ];
    for (x, y, z) in boxes {
        let profile = profile_of(x, y);
        let mut doors: Vec<(&str, String)> = vec![
            ("brick", dump(&common::brick::<f64>(x, y, z))),
            (
                "prism_z",
                dump(&common::prism_z::<f64>(&profile, z.0, z.1).body),
            ),
            ("mapped_cube", dump(&common::mapped_cube(onto(x, y, z)))),
            ("cube_into", {
                let mut body = Body::<f64>::new();
                common::cube_into(&mut body, onto(x, y, z));
                dump(&body)
            }),
        ];
        if z.0 == 0.0 {
            doors.push(("prism", dump(&common::prism::<f64>(&profile, z.1).body)));
        }
        let (first_name, first) = &doors[0];
        for (name, other) in &doors[1..] {
            assert_eq!(
                other, first,
                "box {x:?} {y:?} {z:?}: `{name}` and `{first_name}` must build the \
                 same arenas — some operator, site, corner order or description \
                 step moved on one route and not the other"
            );
        }
    }
}

/// The generic box doors agree at a scalar that is not `f64`.
///
/// `mapped_cube` and `cube_into` are `f64`-only, so the roster here is
/// the generic half of the family; what this adds is the lane. A
/// construction that is right at `f64` and wrong under an enclosure —
/// a corner recomputed rather than carried, a witness taken from the
/// wrong end — moves one door and not the other here and nowhere else
/// in this file.
#[test]
#[cfg(feature = "interval")]
fn the_generic_box_doors_agree_at_an_interval_scalar() {
    use geom_core::Interval;
    let boxes = [
        ((0.0, 1.0), (0.0, 1.0), (0.0, 1.0)),
        ((-2.5, -0.25), (0.75, 4.0), (-1.0, -0.125)),
        ((0.0, 2.0), (0.0, 0.5), (0.0, 7.25)),
    ];
    for (x, y, z) in boxes {
        let profile = profile_of(x, y);
        let mut doors: Vec<(&str, String)> = vec![
            ("brick", dump(&common::brick::<Interval>(x, y, z))),
            (
                "prism_z",
                dump(&common::prism_z::<Interval>(&profile, z.0, z.1).body),
            ),
        ];
        if z.0 == 0.0 {
            doors.push((
                "prism",
                dump(&common::prism::<Interval>(&profile, z.1).body),
            ));
        }
        let (first_name, first) = &doors[0];
        for (name, other) in &doors[1..] {
            assert_eq!(
                other, first,
                "box {x:?} {y:?} {z:?} at Interval: `{name}` and `{first_name}` \
                 must build the same arenas"
            );
        }
    }
}

/// Every door's body is the prism its inputs name — off the rectangle,
/// off the diagonal map, and off `f64`.
///
/// This is the row that does not compare doors to each other, so it is
/// the one that survives a change moving all of them together. The
/// sample is deliberately outside what the agreement rows can reach:
/// `n = 3`, `n = 5` and a reflex `n = 6` (which only the profile doors
/// spell), a shear (which only the mapped doors spell), and a scalar
/// that is not `f64` (which only the generic doors spell).
#[test]
fn every_door_builds_the_prism_its_inputs_name() {
    let ident = |x: f64, y: f64, z: f64| Point3::new(x, y, z);

    // The profile doors, off the rectangle.
    for profile in [&TRIANGLE[..], &PENTAGON[..], &REFLEX_L[..]] {
        let z = (-0.75, 1.5);
        assert_prism_shaped(
            &common::prism_z::<f64>(profile, z.0, z.1).body,
            profile,
            z,
            ident,
            true,
        );
        assert_prism_shaped(
            &common::prism::<f64>(profile, 2.25).body,
            profile,
            (0.0, 2.25),
            ident,
            true,
        );
    }

    // The rectangle, through every door that spells it.
    let (x, y, z) = ((-1.5, 0.25), (0.5, 3.0), (-2.0, -0.5));
    let profile = profile_of(x, y);
    assert_prism_shaped(&common::brick::<f64>(x, y, z), &profile, z, ident, true);
    assert_prism_shaped(
        &common::prism_z::<f64>(&profile, z.0, z.1).body,
        &profile,
        z,
        ident,
        true,
    );

    // The mapped doors, under a shear: the corners are the unit
    // square's, so the profile that names them is the unit square and
    // the map does the rest.
    assert_prism_shaped(
        &common::mapped_cube(sheared),
        &UNIT_SQUARE,
        (0.0, 1.0),
        sheared,
        true,
    );
    let mut body = Body::<f64>::new();
    common::cube_into(&mut body, sheared);
    assert_prism_shaped(&body, &UNIT_SQUARE, (0.0, 1.0), sheared, true);

    // And `geometric_cube`, whose only difference is the last
    // argument.
    assert_prism_shaped(
        &common::geometric_cube::<f64>().body,
        &UNIT_SQUARE,
        (0.0, 1.0),
        ident,
        false,
    );
}

/// [`every_door_builds_the_prism_its_inputs_name`] at a scalar that is
/// not `f64`, over the same off-rectangle profiles.
#[test]
#[cfg(feature = "interval")]
fn every_generic_door_builds_the_prism_its_inputs_name_at_an_interval_scalar() {
    use geom_core::Interval;
    let ident = |x: f64, y: f64, z: f64| {
        Point3::new(
            Interval::from_f64(x),
            Interval::from_f64(y),
            Interval::from_f64(z),
        )
    };
    for profile in [&TRIANGLE[..], &PENTAGON[..], &REFLEX_L[..]] {
        let z = (-0.75, 1.5);
        assert_prism_shaped(
            &common::prism_z::<Interval>(profile, z.0, z.1).body,
            profile,
            z,
            ident,
            true,
        );
    }
    let (x, y, z) = ((-1.5, 0.25), (0.5, 3.0), (-2.0, -0.5));
    assert_prism_shaped(
        &common::brick::<Interval>(x, y, z),
        &profile_of(x, y),
        z,
        ident,
        true,
    );
    assert_prism_shaped(
        &common::geometric_cube::<Interval>().body,
        &UNIT_SQUARE,
        (0.0, 1.0),
        ident,
        false,
    );
}

/// `geometric_cube` is the one door in the family that builds a
/// different body, and the difference is the one its suites measure:
/// it stops before `describe_as_intersections`, so every chord is still
/// at the scaffolding door. Unify it with the box doors and
/// `assert_every_chord_named_by_both_rules` stops being reachable
/// rather than starting to fail, which is why the axis is pinned here
/// and not left to the suites that ride on it.
#[test]
fn geometric_cube_is_the_one_door_that_keeps_its_scaffolding() {
    let unit = ((0.0, 1.0), (0.0, 1.0), (0.0, 1.0));
    let described = common::brick::<f64>(unit.0, unit.1, unit.2);
    let scaffolded = common::geometric_cube::<f64>().body;

    let described_carries = carries(&described);
    let scaffolded_carries = carries(&scaffolded);
    assert_eq!(described_carries.len(), 12);
    assert_eq!(scaffolded_carries.len(), 12);
    assert!(
        described_carries
            .iter()
            .all(|&(isect, decl)| isect && !decl),
        "every edge of a described box cites the intersection its two \
         surfaces determine, on no one's authority: {described_carries:?}"
    );
    assert!(
        scaffolded_carries
            .iter()
            .all(|&(isect, decl)| !isect && decl),
        "every edge of `geometric_cube` is still a declared conventional \
         chord — the state its suites assert both at-rest rules fire on: \
         {scaffolded_carries:?}"
    );
}
