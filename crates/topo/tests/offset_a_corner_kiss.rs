//! **The offset doors part a corner kiss the offset pulls apart.** A
//! union of two cubes kissing at one corner puts the two corners on one
//! point key (`boolean::ops::record_classes`). Offsetting every face of
//! both cubes, the two corners' six planes no longer concur, so the
//! doors solve each solid's corner alone (`replace_face::split_by_solid`)
//! and the corners part: the body builds at the closed-form volume,
//! whichever door and sign.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::common;

use common::{finished, mapped_cube};
use geom_core::{Point3, Tol};
use topo::{Body, BooleanResult, ChartMove, mass_properties, union};

fn tol() -> Tol {
    Tol::witness()
}

/// A unit cube on the rotated frame `r`, its low corner at `o`.
fn cube(r: [[f64; 3]; 3], o: [f64; 3]) -> Body<f64> {
    mapped_cube(
        move |x, y, z| {
            let p = [0, 1, 2].map(|i| o[i] + x * r[0][i] + y * r[1][i] + z * r[2][i]);
            Point3::new(p[0], p[1], p[2])
        },
        tol(),
    )
}

/// Two unit cubes on frame `r`, the second's low corner on the first's
/// high one, united: the kiss, its two corners on one key.
fn kiss(r: [[f64; 3]; 3]) -> Body<f64> {
    let c = [0, 1, 2].map(|i| r[0][i] + r[1][i] + r[2][i]);
    let a = finished("a", cube(r, [0.0; 3]), tol());
    let b = finished("b", cube(r, c), tol());
    let Ok(BooleanResult::Body(u)) = union(&a, &b, tol()) else {
        panic!("the kiss builds");
    };
    let [(x, y)] = u.contacts.vertex_vertex().collect::<Vec<_>>()[..] else {
        panic!("one vertex-vertex record: {:?}", u.contacts);
    };
    let key = |v| u.body.get_vertex(v).unwrap().point;
    assert_eq!(key(x), key(y), "the kissing corners share one key");
    (*u.body).clone()
}

/// The axis frame, and one turned off every axis.
fn frames() -> [(&'static str, [[f64; 3]; 3]); 2] {
    let (s, c) = (0.3f64.sin(), 0.3f64.cos());
    let (t, d) = (0.5f64.sin(), 0.5f64.cos());
    // About z by 0.3, then about x by 0.5.
    let rz = [[c, s, 0.0], [-s, c, 0.0], [0.0, 0.0, 1.0]];
    let turned = rz.map(|v| [v[0], d * v[1] - t * v[2], t * v[1] + d * v[2]]);
    [
        ("axis", [[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]]),
        ("turned", turned),
    ]
}

/// **Every face of both kissing cubes offset by `d`, through the
/// planar and the general door** (the axial door takes no planar body). Each builds, at twice the offset cube's volume. Red where
/// the doors solve the two solids' corners on their shared key only as
/// one corner (six planes that do not concur: `TogetherCorner`, or the
/// general door's `VertexDisagreement`).
#[test]
fn both_doors_offset_a_corner_kiss_at_either_sign() {
    let band = geom_core::Band::linear(tol()).unwrap();
    for (name, r) in frames() {
        for d in [0.1, -0.1, 0.05, -0.05, -0.3] {
            for door in ["planes", "surfaces"] {
                let mut body = kiss(r);
                let moves: Vec<ChartMove<f64>> = body
                    .faces()
                    .map(|(f, _)| ChartMove {
                        faces: vec![f],
                        distance: d,
                    })
                    .collect();
                let what = format!("{name} d={d} {door}");
                let out = match door {
                    "planes" => {
                        topo::offset_planes_together(&mut body, &moves, band, tol()).map(|_| ())
                    }
                    _ => topo::offset_surfaces_together(&mut body, &moves, band, tol()).map(|_| ()),
                };
                assert!(out.is_ok(), "{what}: {out:?}");
                let v = mass_properties(
                    &topo::test_support::finished("the measured body", body.clone(), tol()),
                    tol(),
                    topo::test_support::pinned_resolution(tol()),
                )
                .unwrap()
                .met()
                .expect("the pinned resolution is met")
                .volume;
                let want = 2.0 * (1.0 + 2.0 * d).powi(3);
                assert!((v - want).abs() < 1e-9, "{what}: volume {v}, want {want}");
            }
        }
    }
}

/// The kiss on the axis frame with the first cube's faces offset by
/// `0.1` and the second's by `-0.1 + g`: the two moved corners lie
/// `√3·g` apart, and their six planes concur exactly at `g = 0`.
fn split_offset(door: &str, g: f64) -> (Result<(), String>, Body<f64>) {
    let band = geom_core::Band::linear(tol()).unwrap();
    let (_, r) = frames()[0];
    let mut body = kiss(r);
    let first = body.solids().next().unwrap().0;
    let moves: Vec<ChartMove<f64>> = body
        .faces()
        .map(|(f, _)| ChartMove {
            faces: vec![f],
            distance: if body.solid_of_face(f) == Some(first) {
                0.1
            } else {
                -0.1 + g
            },
        })
        .collect();
    let out = match door {
        "planes" => topo::offset_planes_together(&mut body, &moves, band, tol()).map(|_| ()),
        _ => topo::offset_surfaces_together(&mut body, &moves, band, tol()).map(|_| ()),
    };
    (out.map_err(|e| format!("{e:?}")), body)
}

/// **A joint solve the band cannot decide escalates; it never parts.**
/// The kiss's two corners offset to points `√3·g` apart. Within eps
/// they solve as one; far outside the band they part, the corners on
/// two keys; where the gap sits in the band (`g` a few eps over √3) the
/// joint solve is undecided and both doors refuse `Escalated`. Red if a
/// door parts a cross-solid group on any refusal of its joint solve
/// rather than on a definite one.
#[test]
fn an_undecided_joint_corner_escalates_rather_than_parts() {
    let eps = tol().eps();
    let key_of = |body: &Body<f64>| {
        let mut keys: Vec<_> = body
            .vertex_points()
            .filter(|(_, p)| (p.x - p.y).abs() < 1e-6 && (p.y - p.z).abs() < 1e-6)
            .filter(|(_, p)| (p.x - 1.1).abs() < 1e-3)
            .map(|(v, _)| body.get_vertex(v).unwrap().point)
            .collect();
        keys.sort();
        keys.dedup();
        keys.len()
    };
    for door in ["planes", "surfaces"] {
        let (out, body) = split_offset(door, 0.0);
        assert_eq!(out, Ok(()), "{door}: the planes concur");
        assert_eq!(key_of(&body), 1, "{door}: one corner, one key");
        for k in [2.0, 4.0, 10.0] {
            let (out, _) = split_offset(door, k * eps / 3f64.sqrt());
            assert!(
                matches!(&out, Err(e) if e.starts_with("Escalated")),
                "{door} g={k}·eps/√3: {out:?}"
            );
        }
        let (out, body) = split_offset(door, 1e3 * eps / 3f64.sqrt());
        assert_eq!(out, Ok(()), "{door}: far apart, the corners part");
        assert_eq!(key_of(&body), 2, "{door}: two corners, two keys");
    }
}
