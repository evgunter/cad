//! **A D pocket whose wall crosses the block's side face.** The block
//! `[−1, 1]² × [0, 1]` against the `rod_chord_at` D prism, turned and
//! offset so its cylindrical wall leaves the block through a side face,
//! entering from the top, the bottom and through, under ∪, ∖ and ∩ in
//! both operand orders.
//!
//! The 57 poses (19 placements at three entries each) are JOIN-3's R2
//! pocket battery's (`join3_r2_probes::j3r2_pocket_battery`) poses whose
//! wall crosses a side face, where it refused: ring re-homing on the
//! divided wall, then the wall window of a face an earlier chord had
//! divided. The battery prints them with no oracle — the cutter
//! leaves the block, so its own area is not the pocket's — and this
//! suite supplies one: the D's profile is a disc cut by a half-plane, so
//! the pocket's cross-section is that disc against a convex polygon (the
//! block's square clipped by the same half-plane), whose area is closed
//! form per edge — a chord triangle and a circular sector.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::f64::consts::{FRAC_PI_2, PI};

use geom_core::{Affine3, Point2, Tol, Vec3};
use profile::{Profile, SketchPlane, test_support::bulge_loop};
use sweep::test_support::{ROD_R, rod_chord_at};
use sweep::{ExtrudeSide, Extrusion, extrude};
use topo::Body;

use crate::common::differential::{area, ccw, clip_convex, disc_clip_area, outcome};

/// The block's half-width, metres.
const HALF: f64 = 1.0;

fn tol() -> Tol {
    Tol::witness()
}

/// One pose: the D's flat (`rod_chord_at`), the turn in radians, the
/// offset of the D's centre in the block's top, and the entry — the
/// sketch plane's `z` and the prism's height, as the battery names them.
struct Pose {
    flat: f64,
    phi: f64,
    off: (f64, f64),
    entry: &'static str,
    z0: f64,
    h: f64,
}

/// The pose table: every pocket pose of JOIN-3's R2 battery whose wall
/// crosses a side face of the block, by (flat, turn, offset), at each of
/// the three entries that reach the join (the two flush entries refuse
/// at the undeclared-coincidence gate before it, outside this suite).
fn poses() -> Vec<Pose> {
    const GROUPS: [(f64, f64, (f64, f64)); 19] = [
        (0.3, 2.3, (0.8, 0.1)),
        (0.3, PI, (0.8, 0.1)),
        (0.3, 4.0, (0.8, 0.1)),
        (0.45, 2.3, (0.8, 0.1)),
        (0.45, PI, (0.75, 0.75)),
        (0.45, PI, (0.8, 0.1)),
        (0.45, 4.0, (0.8, 0.1)),
        (0.49, FRAC_PI_2, (0.8, 0.1)),
        (0.49, 2.3, (0.8, 0.1)),
        (0.49, PI, (0.75, 0.75)),
        (0.49, PI, (0.8, 0.1)),
        (0.49, 4.0, (0.75, 0.75)),
        (0.49, 4.0, (0.8, 0.1)),
        (0.4999, FRAC_PI_2, (0.8, 0.1)),
        (0.4999, 2.3, (0.8, 0.1)),
        (0.4999, PI, (0.75, 0.75)),
        (0.4999, PI, (0.8, 0.1)),
        (0.4999, 4.0, (0.75, 0.75)),
        (0.4999, 4.0, (0.8, 0.1)),
    ];
    const ENTRIES: [(&str, f64, f64); 3] = [
        ("top", 0.5, 1.0),
        ("bottom", -0.5, 1.0),
        ("through", -0.5, 2.0),
    ];
    GROUPS
        .iter()
        .flat_map(|&(flat, phi, off)| {
            ENTRIES.map(|(entry, z0, h)| Pose {
                flat,
                phi,
                off,
                entry,
                z0,
                h,
            })
        })
        .collect()
}

/// The D prism of `pose`: `rod_chord_at`'s two-vertex profile (the
/// standing wall arc and the flat's chord), turned and offset, extruded
/// from its sketch plane.
fn cutter(pose: &Pose) -> Body<f64> {
    let c = rod_chord_at(pose.flat);
    let (s, co) = pose.phi.sin_cos();
    let turn =
        |(x, y): (f64, f64)| Point2::new(co * x - s * y + pose.off.0, s * x + co * y + pose.off.1);
    let lp = bulge_loop(vec![
        (turn((pose.flat, c.half)), c.wall_bulge),
        (turn((pose.flat, -c.half)), 0.0),
    ]);
    let plane = SketchPlane::new(Affine3::translation(Vec3::new(0.0, 0.0, pose.z0)));
    let profile = Profile::new(plane, vec![lp]).validate(tol()).unwrap();
    extrude(
        &profile,
        Extrusion::Distance {
            depth: pose.h,
            side: ExtrudeSide::Along,
        },
        tol(),
    )
    .unwrap()
    .body
}

/// The block `[−1, 1]² × [0, 1]`.
fn block() -> Body<f64> {
    let lp = bulge_loop(
        square()
            .into_iter()
            .map(|(x, y)| (Point2::new(x, y), 0.0))
            .collect(),
    );
    let profile = Profile::new(SketchPlane::xy(), vec![lp])
        .validate(tol())
        .unwrap();
    extrude(
        &profile,
        Extrusion::Distance {
            depth: 1.0,
            side: ExtrudeSide::Along,
        },
        tol(),
    )
    .unwrap()
    .body
}

/// The block's cross-section, counter-clockwise.
fn square() -> Vec<(f64, f64)> {
    ccw(vec![
        (-HALF, -HALF),
        (HALF, -HALF),
        (HALF, HALF),
        (-HALF, HALF),
    ])
}

/// The pocket's cross-section: the D's disc against the block's square
/// clipped by the D's flat, both about the disc's centre.
fn pocket_area(pose: &Pose) -> f64 {
    let (s, co) = pose.phi.sin_cos();
    // The flat's half-plane as a far rectangle whose near edge IS the
    // flat line: the image of `x = flat` under the pose's own turn, so
    // the clip line is the profile's own.
    let far = 8.0;
    let rect: Vec<(f64, f64)> = [
        (pose.flat, -far),
        (pose.flat, far),
        (pose.flat - far, far),
        (pose.flat - far, -far),
    ]
    .into_iter()
    .map(|(x, y)| (co * x - s * y + pose.off.0, s * x + co * y + pose.off.1))
    .collect();
    let poly: Vec<(f64, f64)> = clip_convex(&square(), &ccw(rect))
        .into_iter()
        .map(|(x, y)| (x - pose.off.0, y - pose.off.1))
        .collect();
    if poly.len() < 3 {
        return 0.0;
    }
    disc_clip_area(ROD_R, &poly).abs()
}

/// The D's own cross-section: the disc less the cap the flat cuts off.
fn cutter_area(flat: f64) -> f64 {
    let cap = ROD_R * ROD_R * (flat / ROD_R).acos() - flat * (ROD_R * ROD_R - flat * flat).sqrt();
    PI * ROD_R * ROD_R - cap
}

#[test]
fn a_pocket_whose_wall_crosses_a_side_face_builds_at_its_closed_form_volume() {
    let blk = block();
    let v_blk = 4.0;
    let mut bad = Vec::new();
    for pose in poses() {
        let cut = cutter(&pose);
        // The cutter's own volume, closed form against the kernel's:
        // the oracle's disc is the profile the prism was built from.
        let v_cut = cutter_area(pose.flat) * pose.h;
        let measured = topo::mass_properties(&cut, tol()).unwrap().volume;
        assert!(
            (measured - v_cut).abs() < 1e-9,
            "the D prism's closed form is its volume: flat={} h={} closed form {v_cut} vs \
             measured {measured}",
            pose.flat,
            pose.h,
        );
        let overlap = (1.0f64.min(pose.z0 + pose.h) - 0.0f64.max(pose.z0)).max(0.0);
        let v_i = pocket_area(&pose) * overlap;
        let tag = format!(
            "D{} phi={:.3} off={:?} {}",
            pose.flat, pose.phi, pose.off, pose.entry
        );
        for (op, want_ab, want_ba) in [
            ("U", v_blk + v_cut - v_i, v_blk + v_cut - v_i),
            ("S", v_blk - v_i, v_cut - v_i),
            ("I", v_i, v_i),
        ] {
            for (order, want) in [("AB", want_ab), ("BA", want_ba)] {
                let (l, r) = if order == "AB" {
                    (&blk, &cut)
                } else {
                    (&cut, &blk)
                };
                let got = match op {
                    "U" => topo::union(l, r, tol()),
                    "S" => topo::subtract(l, r, tol()),
                    _ => topo::intersect(l, r, tol()),
                };
                let line = outcome(got, want, tol());
                if !line.starts_with("OK SOUND") {
                    bad.push(format!("{tag} {op} {order} => {line}"));
                }
            }
        }
    }
    assert!(
        bad.is_empty(),
        "every pose builds a sound body at its closed-form volume; {} did not:\n{}",
        bad.len(),
        bad.join("\n")
    );
}

#[test]
fn the_pocket_oracle_is_the_disc_against_the_square() {
    // A D centred in the block takes the whole profile: the oracle's
    // clip is the identity there.
    let whole = Pose {
        flat: 0.3,
        phi: 0.0,
        off: (0.0, 0.0),
        entry: "top",
        z0: 0.5,
        h: 1.0,
    };
    assert!(
        (pocket_area(&whole) - cutter_area(0.3)).abs() < 1e-12,
        "a D inside the square clips to its own area: {} vs {}",
        pocket_area(&whole),
        cutter_area(0.3)
    );
    // A D pushed clear of the block takes none of it.
    let clear = Pose {
        off: (3.0, 0.0),
        ..whole
    };
    assert!(
        pocket_area(&clear).abs() < 1e-12,
        "a D clear of the square clips to nothing: {}",
        pocket_area(&clear)
    );
    // Half the disc, from a flat on the axis centred on the block's
    // own edge: a quarter of the disc's area.
    let halved = Pose {
        flat: 0.0,
        off: (HALF, 0.0),
        phi: FRAC_PI_2,
        ..whole
    };
    let quarter = PI * ROD_R * ROD_R / 4.0;
    assert!(
        (pocket_area(&halved) - quarter).abs() < 1e-12,
        "a half-disc halved again by the block's edge is a quarter disc: {} vs {quarter}",
        pocket_area(&halved)
    );
    // The square's area is the polygon oracle's own, so a disc larger
    // than the block takes all of it.
    assert!((area(&square()) - 4.0).abs() < 1e-12, "the block's section");
}
