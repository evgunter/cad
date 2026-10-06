//! LANE-PRIVATE PROBES (zip-chord, branch `zip/chord-probe`; never for
//! main). Every test here is `#[ignore]`d and asserts nothing: each one
//! runs a reproducer of a ZIP row that meets `rest.rs`'s `mint_chord`
//! and appends what it saw to the file `ZIP_CHORD_PROBE` names, beside
//! the kernel's own probe lines (`rest::chord_probe`). Run:
//!
//! ```text
//! ZIP_CHORD_PROBE=/path/to/log cargo test -p sweep --test all \
//!     zip_chord_probe -- --ignored --test-threads=1
//! ```

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::mate2_common::{declare_rest, plane_face, wall_decls};
use geom_core::Tol;
use std::io::Write;
use sweep::test_support::{brick, finished};
use topo::{AtRestBody, Body, BooleanDeclarations, BooleanOp, BooleanResult};

fn note(line: &str) {
    eprintln!("{line}");
    let Ok(path) = std::env::var("ZIP_CHORD_PROBE") else {
        return;
    };
    let test = std::thread::current()
        .name()
        .unwrap_or("<unnamed>")
        .to_owned();
    let mut f = std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(path)
        .unwrap();
    writeln!(f, "[{test}] {line}").unwrap();
}

fn outcome(r: &Result<BooleanResult<f64>, topo::BooleanError>) -> String {
    match r {
        Ok(BooleanResult::Body(bb)) => format!(
            "Ok(Body) volume={:.12e} shells={}",
            topo::mass_properties(&bb.body, Tol::witness())
                .map(|m| m.volume)
                .unwrap_or(f64::NAN),
            bb.body.shells().count()
        ),
        Ok(BooleanResult::Empty) => "Ok(Empty)".to_owned(),
        Err(e) => format!("Err({e:?})"),
    }
}

/// The union of `a` and `b` under `decls`, then the general join's
/// refusal on the same op (`boolean_join_refusal`), then the union
/// undeclared.
fn run(tag: &str, a: &Body<f64>, b: &Body<f64>, decls: &BooleanDeclarations) {
    let tol = Tol::witness();
    let (fa, fb) = (finished("a", a.clone(), tol), finished("b", b.clone(), tol));
    note(&format!(
        "=== {tag}: declared union ({} pairs)",
        decls.coincident_faces.len()
    ));
    let declared = topo::union_with(&fa, &fb, decls, tol);
    note(&format!("RESULT declared: {}", outcome(&declared)));
    let join = topo::test_support::boolean_join_refusal(BooleanOp::Union, a, b, decls, tol);
    note(&format!("JOIN declared (boolean_join_refusal): {join:?}"));
    let none = BooleanDeclarations::none();
    let undeclared = topo::union_with(&fa, &fb, &none, tol);
    note(&format!("RESULT undeclared: {}", outcome(&undeclared)));
    let join = topo::test_support::boolean_join_refusal(BooleanOp::Union, a, b, &none, tol);
    note(&format!("JOIN undeclared (boolean_join_refusal): {join:?}"));
    for op in [BooleanOp::Subtract, BooleanOp::Intersect] {
        let r = topo::boolean_op_with(op, &fa, &fb, decls, topo::SweepStrategy::Realized, tol);
        note(&format!("RESULT declared {op:?}: {}", outcome(&r)));
    }
}

fn both_orders(
    tag: &str,
    a: &Body<f64>,
    b: &Body<f64>,
    decls: impl Fn(&Body<f64>, &Body<f64>) -> BooleanDeclarations,
) {
    run(&format!("{tag}, a ∪ b"), a, b, &decls(a, b));
    run(&format!("{tag}, b ∪ a"), b, a, &decls(b, a));
}

/// Row `blind-shaft-in-a-full-turn-bore-revisits-the-seam-vertex`:
/// every blind span at 0°, 60° and 90°, identity pose, both orders.
#[test]
#[ignore = "zip-chord probe; asserts nothing"]
fn probe_blind_shaft_in_a_full_turn_bore() {
    use crate::full_turn_bore_mate::{collar, shaft};
    for (span, y0, h) in [
        ("blind from below", 0.5, 1.0),
        ("blind from above", 1.5, 1.0),
        ("wholly inside", 1.2, 0.6),
    ] {
        for deg in [0.0, 60.0, 90.0] {
            both_orders(
                &format!("blind shaft, {span}, azimuth {deg}"),
                &collar(),
                &shaft(deg, y0, h),
                wall_decls,
            );
        }
    }
}

/// Row `a-vertex-of-one-solid-inside-the-rest-contact-has-no-twin`:
/// the split collar against the 60° shaft at the four spans.
#[test]
#[ignore = "zip-chord probe; asserts nothing"]
fn probe_split_collar_vertex_without_twin() {
    use crate::full_turn_bore_mate::{shaft, split_collar};
    for (span, y0, h) in [
        ("through", 0.5, 2.0),
        ("flush", 1.0, 1.0),
        ("proud above", 1.0, 1.5),
        ("proud below", 0.5, 1.5),
    ] {
        both_orders(
            &format!("split collar, {span}, azimuth 60"),
            &split_collar(),
            &shaft(60.0, y0, h),
            wall_decls,
        );
    }
}

/// Row `a-boss-flush-with-a-block-edge-refuses-its-declared-union`:
/// the 40 × 20 × 10 block centred on the world xy frame, the boss
/// `x ∈ [10, 20]`, `y ∈ [−5, 5]` on its top, 4 tall, its +x wall in the
/// block's. Built at two scales (`s` = 1e-3 is millimetres in metres,
/// the viewer's; `s` = 1 is the same numbers as units), through the
/// sweep's extrusion and the brick door, under three declarations: the
/// resting cap pair alone (the boolean tool's offer), the cap pair and
/// the flush walls' continuation, and every flush finding.
#[test]
#[ignore = "zip-chord probe; asserts nothing"]
fn probe_boss_flush_with_a_block_edge() {
    let tol = Tol::witness();
    for s in [1e-3, 1.0] {
        let extruded_block = sweep::test_support::prism_at(
            sweep::test_support::corners(&[
                (-20.0 * s, -10.0 * s),
                (20.0 * s, -10.0 * s),
                (20.0 * s, 10.0 * s),
                (-20.0 * s, 10.0 * s),
            ]),
            0.0,
            10.0 * s,
            tol,
        );
        let extruded_boss = sweep::test_support::prism_at(
            sweep::test_support::corners(&[
                (10.0 * s, -5.0 * s),
                (20.0 * s, -5.0 * s),
                (20.0 * s, 5.0 * s),
                (10.0 * s, 5.0 * s),
            ]),
            10.0 * s,
            4.0 * s,
            tol,
        );
        let brick_block: Body<f64> = brick(
            (-20.0 * s, 20.0 * s),
            (-10.0 * s, 10.0 * s),
            (0.0, 10.0 * s),
            tol,
        );
        let brick_boss: Body<f64> = brick(
            (10.0 * s, 20.0 * s),
            (-5.0 * s, 5.0 * s),
            (10.0 * s, 14.0 * s),
            tol,
        );
        for (door, block, boss) in [
            ("extruded", &extruded_block, &extruded_boss),
            ("brick", &brick_block, &brick_boss),
        ] {
            let top = 10.0 * s;
            let flush = topo::flush::find_flush_candidates(block, boss, tol);
            note(&format!(
                "boss scale {s} {door}: flush findings {:?}",
                flush
                    .as_ref()
                    .map(|f| f.iter().map(|x| (x.pair, x.class)).collect::<Vec<_>>())
            ));
            let cap_only = |a: &Body<f64>, b: &Body<f64>| {
                let mut d = BooleanDeclarations::none();
                let (fa, fb) = if core::ptr::eq(a, block) {
                    (plane_face(a, top, true), plane_face(b, top, false))
                } else {
                    (plane_face(a, top, false), plane_face(b, top, true))
                };
                declare_rest(&mut d, &[fa], &[fb]);
                d
            };
            both_orders(
                &format!("boss scale {s} {door}, cap only"),
                block,
                boss,
                cap_only,
            );
            let with_continuations = |a: &Body<f64>, b: &Body<f64>| {
                let mut d = crate::mate2_common::continuations(a, b);
                d.coincident_faces.extend(cap_only(a, b).coincident_faces);
                d
            };
            both_orders(
                &format!("boss scale {s} {door}, cap + continuations"),
                block,
                boss,
                with_continuations,
            );
        }
    }
}

/// The finished operands, for a caller that wants `AtRestBody`.
#[allow(dead_code)]
fn at_rest(b: &Body<f64>) -> AtRestBody<f64> {
    finished("operand", b.clone(), Tol::witness())
}

/// The face of `body` whose plane carrier faces `n`.
fn face_facing(body: &Body<f64>, n: [f64; 3]) -> topo::FaceKey {
    let n = geom_core::Vec3::from_array(n);
    let hits: Vec<_> = body
        .faces()
        .map(|(k, _)| k)
        .filter(|&k| {
            matches!(topo::face_carrier(body, k),
                Some(topo::CarrierDesc::Plane { normal, .. }) if normal.dot(n) > 1.0 - 1e-6)
        })
        .collect();
    assert_eq!(hits.len(), 1, "one face faces {n:?}");
    hits[0]
}

/// The prose reproducer in `reduce.rs`'s
/// `a_coplanar_sectors_in_band_residue_builds_through_the_lump_where_the_door_bridges_it`:
/// a parallelepiped wedge standing on a block, its base tilted by
/// `k·ε` (that test says at `2·ε` the zip refuses
/// `ChordBetweenIsolatedPierces`). Declared `Rest` on the base pair.
#[test]
#[ignore = "zip-chord probe; asserts nothing"]
fn probe_tilted_wedge_standing_on_a_block() {
    let tol = Tol::witness();
    let band = geom_core::Band::linear(tol).unwrap();
    let phi = 5.0_f64.to_radians();
    let p = geom_core::Point3::new(0.5, 0.2, 1.0);
    let block: Body<f64> = brick((0.0, 3.0), (-2.0, 2.5), (0.0, 1.0), tol);
    for k in [-2.0, -1.2, 0.0, 1.2, 2.0] {
        let theta = k * band.zero();
        let ea = geom_core::Vec3::new(1.0, 0.0, 0.0);
        let eb = geom_core::Vec3::new(phi.cos(), phi.sin(), theta * phi.sin());
        let wedge: Body<f64> = topo::test_support::mapped_cube(
            move |u, v, w| p + ea * u + eb * v + geom_core::Vec3::new(0.0, 0.0, w),
            tol,
        );
        let decl = |a: &Body<f64>, b: &Body<f64>| {
            let mut d = BooleanDeclarations::none();
            let (fa, fb) = if core::ptr::eq(a, &block) {
                (
                    face_facing(a, [0.0, 0.0, 1.0]),
                    face_facing(b, [0.0, 0.0, -1.0]),
                )
            } else {
                (
                    face_facing(a, [0.0, 0.0, -1.0]),
                    face_facing(b, [0.0, 0.0, 1.0]),
                )
            };
            declare_rest(&mut d, &[fa], &[fb]);
            d
        };
        both_orders(
            &format!("tilted wedge standing, tilt {k}·ε"),
            &block,
            &wedge,
            decl,
        );
    }
}

/// The two `full_turn_bore_mate` reproducers at every pose of
/// `common::poses`, both orders: blind spans on the one-face bore, and
/// the split collar against the 60° shaft.
#[test]
#[ignore = "zip-chord probe; asserts nothing"]
fn probe_full_turn_bore_every_pose() {
    use crate::full_turn_bore_mate::{collar, shaft, split_collar};
    let tol = Tol::witness();
    let moved = |b: &Body<f64>, pose: &geom_core::Affine3<f64>| {
        topo::transform_rigid(b, pose, tol).unwrap()
    };
    for (pose_name, pose) in crate::common::poses::poses() {
        for (span, y0, h) in [
            ("blind from below", 0.5, 1.0),
            ("blind from above", 1.5, 1.0),
            ("wholly inside", 1.2, 0.6),
        ] {
            for deg in [0.0, 60.0, 90.0] {
                both_orders(
                    &format!("pose {pose_name}: blind shaft, {span}, azimuth {deg}"),
                    &moved(&collar(), &pose),
                    &moved(&shaft(deg, y0, h), &pose),
                    wall_decls,
                );
            }
        }
        for (span, y0, h) in [
            ("through", 0.5, 2.0),
            ("flush", 1.0, 1.0),
            ("proud above", 1.0, 1.5),
            ("proud below", 0.5, 1.5),
        ] {
            both_orders(
                &format!("pose {pose_name}: split collar, {span}, azimuth 60"),
                &moved(&split_collar(), &pose),
                &moved(&shaft(60.0, y0, h), &pose),
                wall_decls,
            );
        }
    }
}
