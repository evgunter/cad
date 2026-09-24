//! **The box doors build one body, and the two cube doors do not** —
//! the two halves of the partition `common::brick`'s doc asserts, made
//! falsifiable. `geometric_cube` leaves the family on the description
//! step; `declined_cube` leaves it on that step and on face geometry.
//!
//! **The box doors are one construction, and that is why the load here
//! is on the row that does not compare them.** `brick`, `prism`,
//! `prism_z`, `mapped_cube` and `cube_into` all reach
//! `common::prism_ops`, so they agree by construction: the first row
//! below still compares their full derived `Debug` dumps, but what it
//! can now catch is a door that stops delegating — a re-inlined
//! sequence, a wrapper that starts passing a different profile, z-range
//! or map — and no longer a change inside the sequence itself, which
//! moves every dump together. **That is the trade this file's rows are
//! arranged around**, and the reason the second row exists.
//!
//! **The silent case is the one this file is for.** A dropped or added
//! description step reds two dozen rows across the tree on its own; a
//! re-ordering, a re-keying or a flipped face plane that leaves every
//! count and every consumer's verdict intact reds nothing else.
//!
//! Every comparison is between live values in one process, with no
//! stored baseline, so a new `Body` field or a `Debug` reformat moves
//! every side identically and this file stays green.
//!
//! [`assert_prism_shaped`] is therefore the row that carries the claim.
//! It shares no premise with the builder: it re-derives the body from
//! `profile`, `z` and `map` alone — the corner list in profile order,
//! the arena orders the operator sequence implies, the loop sizes an
//! N-gon prism has, **each face's outward normal**, and the description
//! arm. It is also the only row that leaves the rectangle: `n = 3`,
//! `n = 5`, a reflex `n = 6`, a shear, and a second `Decide` scalar.
//!
//! **What no row here reads is the CARRIER geometry.** A face's plane
//! is checked for the side it puts the material on and nothing else, an
//! edge is read for its endpoints and its description arm and not for
//! its curve, and no certificate is opened. A carrier that moved while
//! its endpoints stayed put passes this file; ~30 rows elsewhere in the
//! tree are what stand against that.
//!
//! The negative rows are the last two. A file that only pins agreement
//! goes green when someone makes every door identical by deleting the
//! distinction, which is the likeliest way to break it:
//! `geometric_cube` stops before `describe_as_intersections` and its
//! suites assert on exactly that absence, and `declined_cube` stops
//! there and declines its face surfaces on top of it.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_brep::EdgeDescription;
use geom_core::{Band, Decide, Point3, Real, Tol, Vec3};
use topo::Body;

use crate::common;

/// The affine map carrying the unit cube onto `[x] x [y] x [z]` — the
/// one map under which the fixed cube corners and `prism_z`'s
/// profile describe the same box.
fn onto<T: Decide>(
    x: (f64, f64),
    y: (f64, f64),
    z: (f64, f64),
) -> impl Fn(f64, f64, f64) -> Point3<T> {
    move |u, v, w| {
        Point3::new(
            T::from_f64(x.0 + u * (x.1 - x.0)),
            T::from_f64(y.0 + v * (y.1 - y.0)),
            T::from_f64(z.0 + w * (z.1 - z.0)),
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
/// sizes an N-gon prism has, each face's outward normal, and the
/// description arm. A builder that re-keys, re-orders, re-corners or
/// turns a face inside out fails here with every door still agreeing
/// with every other.
///
/// `profile` may carry any `n >= 3` corners and may be reflex; `T` may
/// be any `Decide` scalar. `map` may be any point map, shears included,
/// but the normal check reads it as **affine** — it is what makes the
/// strut vector the same at every corner and what carries the
/// determinant's sign — which every caller in this tree is.
///
/// Points are compared through their derived `Debug`, which is the only
/// equality a scalar lane without `PartialEq` offers and is exact where
/// one exists. The normals are compared by a certified sign through
/// [`geom_core::Decide`], because a direction agreeing is not a bit
/// pattern agreeing.
///
/// **It does not read a carrier**, a certificate or a plane's offset;
/// see this file's header for what that leaves to the rest of the tree.
fn assert_prism_shaped<T: Decide>(
    body: &Body<T>,
    profile: &[(f64, f64)],
    z: (f64, f64),
    map: impl Fn(f64, f64, f64) -> Point3<T>,
    described: bool,
) {
    let n = profile.len();
    let bot_at: Vec<Point3<T>> = profile.iter().map(|&(x, y)| map(x, y, z.0)).collect();
    let top_at: Vec<Point3<T>> = profile.iter().map(|&(x, y)| map(x, y, z.1)).collect();
    let bot: Vec<String> = bot_at.iter().map(|p| format!("{p:?}")).collect();
    let top: Vec<String> = top_at.iter().map(|p| format!("{p:?}")).collect();

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
    //
    // The loop SIZE alone is a weak statement and at n = 4 it is no
    // statement at all — `[4, 4, 4, 4, 4, 4]` cannot tell a cap from a
    // side. What gives this walk teeth at every n is the normal
    // asserted alongside it, which is different for every face.
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

    // **Every face's OUTWARD normal points out of the material**, which
    // is the axis the counts and the arena orders cannot reach: a
    // flipped cap plane and an orientation-reversing `map` both leave
    // every key, every endpoint and every loop size exactly where they
    // were.
    //
    // Stated from the corners rather than from a plane the builder
    // computed. `up` is the strut vector, identical at every corner of
    // a prism, so it is also the displacement from the bottom cap to
    // the top one; `e x up` for a segment of a counterclockwise profile
    // is that side's outward direction. Both statements carry the
    // determinant's sign through an affine `map`: for `M` linear,
    // `(M^-T a) . (M b) = a . b`, so a map that reverses orientation
    // lands the built normal on the far side of these and nothing else
    // in this file moves.
    let band = Band::linear(Tol::witness()).unwrap();
    let up = top_at[0] - bot_at[0];
    let mut want_outward: Vec<Vec3<T>> = vec![up, -up];
    for i in 0..n {
        let j = (i + 1) % n;
        want_outward.push((bot_at[j] - bot_at[i]).cross(top_at[i] - bot_at[i]));
    }
    for ((fk, _), want) in body.faces().zip(want_outward) {
        let got = topo::face_normal::face_outward_normal(body, fk)
            .expect("every face of a prism is planar")
            .vec();
        assert_eq!(
            got.dot(want).sign_within(band),
            Ok(geom_core::Sign::Positive),
            "face {fk:?}'s outward normal must agree with the side its \
             corners put the material on — got {got:?}, outward is {want:?}"
        );
    }

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
            (
                "brick",
                dump(&common::brick::<f64>(x, y, z, Tol::witness())),
            ),
            (
                "prism_z",
                dump(&common::prism_z::<f64>(&profile, z.0, z.1, Tol::witness()).body),
            ),
            (
                "mapped_cube",
                dump(&common::mapped_cube(onto::<f64>(x, y, z), Tol::witness())),
            ),
            ("cube_into", {
                let mut body = Body::<f64>::new();
                common::cube_into(&mut body, onto::<f64>(x, y, z), Tol::witness());
                dump(&body)
            }),
        ];
        if z.0 == 0.0 {
            doors.push((
                "prism",
                dump(&common::prism::<f64>(&profile, z.1, Tol::witness()).body),
            ));
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

/// The box doors agree at a scalar that is not `f64`.
///
/// The roster is the whole family, exactly as the `f64` row above: what
/// this adds is the lane, not a narrower set of doors. A
/// construction that is right at `f64` and wrong under an enclosure —
/// a corner recomputed rather than carried, a witness taken from the
/// wrong end — moves one door and not the other here and nowhere else
/// in this file.
#[test]
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
            (
                "brick",
                dump(&common::brick::<Interval>(
                    x,
                    y,
                    z,
                    geom_core::Tol::witness(),
                )),
            ),
            (
                "prism_z",
                dump(
                    &common::prism_z::<Interval>(&profile, z.0, z.1, geom_core::Tol::witness())
                        .body,
                ),
            ),
            (
                "mapped_cube",
                dump(&common::mapped_cube(
                    onto::<Interval>(x, y, z),
                    geom_core::Tol::witness(),
                )),
            ),
            ("cube_into", {
                let mut body = Body::<Interval>::new();
                common::cube_into(
                    &mut body,
                    onto::<Interval>(x, y, z),
                    geom_core::Tol::witness(),
                );
                dump(&body)
            }),
        ];
        if z.0 == 0.0 {
            doors.push((
                "prism",
                dump(&common::prism::<Interval>(&profile, z.1, geom_core::Tol::witness()).body),
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
/// that is not `f64`.
#[test]
fn every_door_builds_the_prism_its_inputs_name() {
    let ident = |x: f64, y: f64, z: f64| Point3::new(x, y, z);

    // The profile doors, off the rectangle.
    for profile in [&TRIANGLE[..], &PENTAGON[..], &REFLEX_L[..]] {
        let z = (-0.75, 1.5);
        assert_prism_shaped(
            &common::prism_z::<f64>(profile, z.0, z.1, Tol::witness()).body,
            profile,
            z,
            ident,
            true,
        );
        assert_prism_shaped(
            &common::prism::<f64>(profile, 2.25, Tol::witness()).body,
            profile,
            (0.0, 2.25),
            ident,
            true,
        );
    }

    // The rectangle, through every door that spells it.
    let (x, y, z) = ((-1.5, 0.25), (0.5, 3.0), (-2.0, -0.5));
    let profile = profile_of(x, y);
    assert_prism_shaped(
        &common::brick::<f64>(x, y, z, Tol::witness()),
        &profile,
        z,
        ident,
        true,
    );
    assert_prism_shaped(
        &common::prism_z::<f64>(&profile, z.0, z.1, Tol::witness()).body,
        &profile,
        z,
        ident,
        true,
    );

    // The mapped doors, under a shear: the corners are the unit
    // square's, so the profile that names them is the unit square and
    // the map does the rest.
    assert_prism_shaped(
        &common::mapped_cube(sheared, Tol::witness()),
        &UNIT_SQUARE,
        (0.0, 1.0),
        sheared,
        true,
    );
    let mut body = Body::<f64>::new();
    common::cube_into(&mut body, sheared, Tol::witness());
    assert_prism_shaped(&body, &UNIT_SQUARE, (0.0, 1.0), sheared, true);

    // And `geometric_cube`, whose only difference is the last
    // argument.
    assert_prism_shaped(
        &common::geometric_cube::<f64>(Tol::witness()).body,
        &UNIT_SQUARE,
        (0.0, 1.0),
        ident,
        false,
    );
}

/// [`every_door_builds_the_prism_its_inputs_name`] at a scalar that is
/// not `f64`, over the same off-rectangle profiles. The shear is left
/// at `f64`: what the mapped doors add here is the map, and the lane is
/// what the rows above it carry.
#[test]
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
            &common::prism_z::<Interval>(profile, z.0, z.1, geom_core::Tol::witness()).body,
            profile,
            z,
            ident,
            true,
        );
    }
    let (x, y, z) = ((-1.5, 0.25), (0.5, 3.0), (-2.0, -0.5));
    assert_prism_shaped(
        &common::brick::<Interval>(x, y, z, geom_core::Tol::witness()),
        &profile_of(x, y),
        z,
        ident,
        true,
    );
    assert_prism_shaped(
        &common::geometric_cube::<Interval>(geom_core::Tol::witness()).body,
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
    let described = common::brick::<f64>(unit.0, unit.1, unit.2, Tol::witness());
    let scaffolded = common::geometric_cube::<f64>(Tol::witness()).body;

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

/// `declined_cube` is the second door that does not build the box
/// doors' body, and it leaves the family on a second axis as well as
/// the scaffolding one: every face stays on the single surface key the
/// opening `mvfs` minted. Both halves are subjects — the coincidence
/// and revert suites read that shared key, and a silent slide to
/// `Certified` would hand them six keys and a different question — so
/// both are pinned here rather than inferred from the rows riding on
/// them.
#[test]
fn declined_cube_keeps_its_scaffolding_and_its_one_surface_key() {
    let declined = common::declined_cube::<f64>(Tol::witness()).body;
    let certified = common::geometric_cube::<f64>(Tol::witness()).body;

    let declined_carries = carries(&declined);
    assert_eq!(declined_carries.len(), 12);
    assert!(
        declined_carries.iter().all(|&(isect, decl)| !isect && decl),
        "every edge of `declined_cube` is a declared conventional chord, \
         exactly as `geometric_cube`'s are: {declined_carries:?}"
    );
    assert_eq!(
        declined.surfaces().count(),
        1,
        "all six faces of `declined_cube` sit on the one `mvfs` placeholder"
    );
    assert_eq!(
        certified.surfaces().count(),
        6,
        "and one surface key per face is what the certified door builds"
    );
}
