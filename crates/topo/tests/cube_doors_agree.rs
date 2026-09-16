//! **The box doors build one body, and `geometric_cube` does not** —
//! the two halves of the partition `common::brick`'s doc asserts, made
//! falsifiable.
//!
//! The box doors reach one construction by two routes: `brick` and
//! `prism` over `prism_z`'s profile chain, `mapped_cube` and `cube_into`
//! over `cube_ops`' fixed eight corners under a point map. Nothing makes
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
//! verdict intact reds nothing else, and is what these two rows catch.
//!
//! The comparison is between five live dumps in one process, with no
//! stored baseline, so a new `Body` field or a `Debug` reformat moves
//! every side identically and this file stays green.
//!
//! The second row is the negative. A test that only pins agreement goes
//! green when someone makes every door identical by deleting the
//! distinction, which is the likeliest way to break this file:
//! `geometric_cube` stops before `describe_as_intersections` and its
//! suites assert on exactly that absence.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_brep::EdgeDescription;
use geom_core::Point3;
use topo::Body;

use crate::common;

/// The affine map carrying the unit cube onto `[x] x [y] x [z]` — the
/// one map under which `cube_ops`' fixed corners and `prism_z`'s
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

/// The rectangle profile of `[x] x [y]`, counterclockwise from +z.
///
/// This restates `brick`'s own corner ladder on purpose: `brick`'s
/// claim is that ITS ladder and `cube_ops`' corners describe the same
/// box, and a guard that reached for `brick`'s expression to state the
/// profile would be comparing it against itself.
fn profile_of(x: (f64, f64), y: (f64, f64)) -> [(f64, f64); 4] {
    [(x.0, y.0), (x.1, y.0), (x.1, y.1), (x.0, y.1)]
}

/// Every field of every arena, keys and slot versions included — the
/// derived `Debug`, so nothing is left out by the author choosing what
/// to look at.
fn dump(body: &Body<f64>) -> String {
    format!("{body:#?}")
}

/// Per edge, in edge-arena order: whether its carrier's description is
/// the intrinsic `Intersection`, and whether its locus was declared.
fn carries(body: &Body<f64>) -> Vec<(bool, bool)> {
    body.edges()
        .map(|(_, e)| {
            let curve = body
                .get_curve_geom(e.curve)
                .unwrap()
                .certified()
                .expect("a box edge carries a certified curve");
            (
                matches!(curve.description(), EdgeDescription::Intersection { .. }),
                curve.authority().is_declared(),
            )
        })
        .collect()
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
