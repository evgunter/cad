//! Designer probes (FLUSH, round 2): what an HONEST FOLD decides. An
//! honest fold is the n-ary union as a plain left fold of the pair
//! boolean in member order, each step deciding every coincidence it
//! meets by its own margin at the accumulation's extent. Prints, does
//! not pin.
#![allow(
    clippy::panic,
    clippy::expect_used,
    clippy::unwrap_used,
    clippy::print_stdout
)]

use crate::common;
use common::{brick, finished};
use geom_core::{Point3, Tol, UnitVec3};
use topo::{AtRestBody, Body, BooleanInputs, BooleanOp, BooleanResult, FaceKey, Surface};

fn tol() -> Tol {
    Tol::witness()
}

fn union_with(
    a: &AtRestBody<f64>,
    b: &AtRestBody<f64>,
    inputs: &BooleanInputs,
) -> Result<topo::BooleanBody<f64>, topo::BooleanError> {
    match topo::boolean_op_with(
        BooleanOp::Union,
        a,
        b,
        inputs,
        topo::SweepStrategy::Realized,
        tol(),
    )? {
        BooleanResult::Body(out) => Ok(out),
        BooleanResult::Empty => panic!("a union of two bodies is not empty"),
    }
}

fn describe(out: &topo::BooleanBody<f64>) -> String {
    let v = topo::mass_properties(&out.body, tol()).map(|m| m.volume);
    let pairs = out
        .coincidences
        .iter()
        .filter(|c| c.face_pair().is_some())
        .count();
    format!(
        "BUILT volume={v:?} faces={} rows={} face-pair-rows={} verdicts={}",
        out.body.faces().count(),
        out.coincidences.len(),
        pairs,
        out.verdicts.len()
    )
}

/// Honest fold of `members` in the given order: each step decides for
/// itself. Prints each step's face-pair rows (the only record a step
/// leaves) and the result.
fn honest_fold(label: &str, members: &[&AtRestBody<f64>]) {
    let mut acc: AtRestBody<f64> = members[0].clone();
    let mut rows_total = 0usize;
    for (i, m) in members.iter().enumerate().skip(1) {
        match union_with(&acc, m, &BooleanInputs::default()) {
            Ok(out) => {
                let pairs = out
                    .coincidences
                    .iter()
                    .filter(|c| c.face_pair().is_some())
                    .count();
                rows_total += pairs;
                println!("PROBE {label}: step {i} {}", describe(&out));
                acc = out.body;
            }
            Err(e) => {
                println!("PROBE {label}: step {i} REFUSED {e:?}");
                return;
            }
        }
    }
    println!(
        "PROBE {label}: END faces={} face-pair-rows-over-all-steps={rows_total}",
        acc.faces().count()
    );
}

/// A: two blocks touching on a wall, and a third block whose material
/// holds that whole wall. The touch is a value-decided coincidence.
/// Under an honest fold it is recorded only in the orders that join
/// the two blocks before the third.
#[test]
fn covered_touch_is_recorded_only_in_some_orders() {
    let a = finished("a", brick((0.0, 1.0), (0.0, 1.0), (0.0, 1.0), tol()), tol());
    let b = finished("b", brick((1.0, 2.0), (0.0, 1.0), (0.0, 1.0), tol()), tol());
    let c = finished(
        "c",
        brick((0.5, 1.5), (-0.2, 1.2), (-0.2, 1.2), tol()),
        tol(),
    );
    honest_fold("A [a,b,c]", &[&a, &b, &c]);
    honest_fold("A [a,c,b]", &[&a, &c, &b]);
    honest_fold("A [c,a,b]", &[&c, &a, &b]);
    honest_fold("A [b,a,c]", &[&b, &a, &c]);
    // The pass judges (a, b) on its own, whatever the order.
    match union_with(&a, &b, &BooleanInputs::default()) {
        Ok(out) => println!("PROBE A pass (a,b): {}", describe(&out)),
        Err(e) => println!("PROBE A pass (a,b): REFUSED {e:?}"),
    }
}

fn face_at_z(body: &Body<f64>, z: f64) -> FaceKey {
    body.faces()
        .find(|(_, f)| {
            matches!(
                body.get_surface(f.surface),
                Some(Surface::Plane { origin, normal, .. })
                    if normal.z.abs() == 1.0 && origin.z == z
            )
        })
        .map(|(k, _)| k)
        .expect("a face on z")
}

/// B: a long plate P1, a unit plate P2 flush beside it, and a unit
/// plate P3 beside P2, tilted by θ about the x-parallel line through
/// the middle of its top edge against P2 (so the P2–P3 wall at x = 2
/// stays exactly coplanar and the tops meet at a tilt). An angular
/// margin is metered at the consumed extent (`carrier_eq`): over the
/// P2–P3 pair the displacement is θ·r_pair ≈ 0.4ε (Zero, glues); over
/// the merged P1∪P2 top it is θ·r_merged, several ε (in band).
#[test]
fn tilt_reads_zero_at_pair_extent_and_in_band_at_merged_extent() {
    let eps = tol().get().eps;
    let theta = 0.4 * eps;
    let p1 = finished("p1", brick((-9.0, 1.0), (0.0, 1.0), (0.0, 1.0), tol()), tol());
    let p2 = finished("p2", brick((1.0, 2.0), (0.0, 1.0), (0.0, 1.0), tol()), tol());
    let p3_flat: Body<f64> = brick((2.0, 3.0), (0.0, 1.0), (0.0, 1.0), tol());
    let motion = geom_core::RigidMotion::rotation_about_axis(
        Point3::new(0.0, 0.5, 1.0),
        UnitVec3::exact_x(),
        theta,
    )
    .unwrap();
    let p3 = finished(
        "p3",
        topo::transform_rigid(&p3_flat, &motion, tol()).unwrap(),
        tol(),
    );
    println!("PROBE B: eps={eps:e} theta={theta:e}");
    // The pass's own judgement of the pair that has the tilt.
    let pair = union_with(&p2, &p3, &BooleanInputs::default());
    match &pair {
        Ok(out) => {
            println!("PROBE B pass (p2,p3): {}", describe(out));
            for v in &out.verdicts {
                println!("PROBE B pass verdict: {v:?}");
            }
        }
        Err(e) => println!("PROBE B pass (p2,p3): REFUSED {e:?}"),
    }
    honest_fold("B [p2,p3,p1]", &[&p2, &p3, &p1]);
    honest_fold("B [p1,p2,p3]", &[&p1, &p2, &p3]);
    honest_fold("B [p3,p2,p1]", &[&p3, &p2, &p1]);
    // The current design in kernel terms: the step that joins p3 to
    // p1∪p2 is handed the pass's (p2,p3) verdicts, re-keyed onto the
    // accumulation's faces, and measures none of them.
    if let Ok(pass) = pair {
        if let Ok(acc) = union_with(&p1, &p2, &BooleanInputs::default()) {
            let acc_body = acc.body;
            let given: Vec<_> = pass
                .verdicts
                .iter()
                .filter_map(|v| {
                    // Re-key v.a (a p2 face) onto the accumulation face
                    // on the same plane, by the plane the p2 face is on.
                    let f = p2.get_face(v.a())?;
                    let Some(Surface::Plane { origin, normal, .. }) = p2.get_surface(f.surface)
                    else {
                        return None;
                    };
                    let at = acc_body.faces().find(|(_, g)| {
                        matches!(acc_body.get_surface(g.surface),
                            Some(Surface::Plane { origin: o2, normal: n2, .. })
                            if (n2.x, n2.y, n2.z) == (normal.x, normal.y, normal.z)
                                && (o2.x - origin.x) * normal.x + (o2.y - origin.y) * normal.y
                                    + (o2.z - origin.z) * normal.z == 0.0)
                    })?;
                    Some(v.given_at(topo::Operand::A, at.0))
                })
                .collect();
            println!(
                "PROBE B given: {} of {} verdicts re-keyed onto p1∪p2 (top key {:?})",
                given.len(),
                pass.verdicts.len(),
                face_at_z(&acc_body, 1.0)
            );
            match union_with(
                &acc_body,
                &p3,
                &BooleanInputs {
                    given: Some(given),
                    ..BooleanInputs::default()
                },
            ) {
                Ok(out) => println!("PROBE B [p1,p2,p3] with given: {}", describe(&out)),
                Err(e) => println!("PROBE B [p1,p2,p3] with given: REFUSED {e:?}"),
            }
        }
    }
}

/// Re-key `v`'s face on `from` (operand `held` of the verdict) onto the
/// face of `onto` lying on the same plane (normals and offsets within
/// 1e-6: the planes differ by at most a few ε here), as the fold's
/// lineage would.
fn rekey_plane(
    v: &topo::GlueVerdict<f64>,
    held: topo::Operand,
    from: &Body<f64>,
    onto: &Body<f64>,
) -> Option<topo::GluedPair<f64>> {
    let key = match held {
        topo::Operand::A => v.a(),
        topo::Operand::B => v.b(),
    };
    let f = from.get_face(key)?;
    let Some(Surface::Plane { origin, normal, .. }) = from.get_surface(f.surface) else {
        return None;
    };
    let (origin, normal) = (*origin, *normal);
    let d = |o: Point3<f64>, n: geom_core::Vec3<f64>| o.x * n.x + o.y * n.y + o.z * n.z;
    let at = onto.faces().find(|(_, g)| {
        matches!(onto.get_surface(g.surface),
            Some(Surface::Plane { origin: o2, normal: n2, .. })
            if (n2.x - normal.x).abs() < 1e-6 && (n2.y - normal.y).abs() < 1e-6
                && (n2.z - normal.z).abs() < 1e-6
                && (d(*o2, *n2) - d(origin, normal)).abs() < 1e-6)
    })?;
    Some(v.given_at(held, at.0))
}

/// How far the vertices of `body` lie from each of its faces' planes:
/// the largest |n·(p − o)| over every face's own vertices.
fn worst_vertex_off_its_face(body: &Body<f64>) -> f64 {
    let mut worst = 0.0f64;
    for (_, f) in body.faces() {
        let Some(Surface::Plane { origin, normal, .. }) = body.get_surface(f.surface) else {
            continue;
        };
        for lk in std::iter::once(f.outer).chain(f.rings.iter().copied()) {
            let Some(lp) = body.get_loop(lk) else { continue };
            let starts: Vec<topo::VertexKey> = match lp.boundary {
                topo::LoopBoundary::Empty { vertex } => vec![vertex],
                topo::LoopBoundary::Cycle { first } => body
                    .loop_cycle(first)
                    .unwrap_or_default()
                    .into_iter()
                    .filter_map(|he| body.get_half_edge(he).map(|h| h.start))
                    .collect(),
            };
            for vk in starts {
                let Some(v) = body.get_vertex(vk) else { continue };
                let Some(p) = body.get_point(v.point) else { continue };
                let dist = (normal.x * (p.x - origin.x)
                    + normal.y * (p.y - origin.y)
                    + normal.z * (p.z - origin.z))
                    .abs();
                worst = worst.max(dist);
            }
        }
    }
    worst
}

/// C: as B, but P3 is tilted by θ about the Y-parallel line through its
/// shared top edge (x = 2, z = 1), so the tilt grows along the row: P3's
/// own top is within θ·1 of the flat plane, but a long flat plate read
/// against P3's tilted description is θ·11 off at its far end. This
/// separates a metering artefact from a genuine order dependence.
#[test]
fn tilt_along_the_row_is_order_dependent_in_substance() {
    let eps = tol().get().eps;
    let theta = 0.4 * eps;
    let p1 = finished("p1", brick((-9.0, 1.0), (0.0, 1.0), (0.0, 1.0), tol()), tol());
    let p2 = finished("p2", brick((1.0, 2.0), (0.0, 1.0), (0.0, 1.0), tol()), tol());
    let p3_flat: Body<f64> = brick((2.0, 3.0), (0.0, 1.0), (0.0, 1.0), tol());
    let motion = geom_core::RigidMotion::rotation_about_axis(
        Point3::new(2.0, 0.0, 1.0),
        UnitVec3::exact_y(),
        theta,
    )
    .unwrap();
    let p3 = finished(
        "p3",
        topo::transform_rigid(&p3_flat, &motion, tol()).unwrap(),
        tol(),
    );
    println!("PROBE C: eps={eps:e} theta={theta:e}");
    println!(
        "PROBE C: p3's vertices off its own faces: {:e}",
        worst_vertex_off_its_face(&p3)
    );
    let pass23 = union_with(&p2, &p3, &BooleanInputs::default());
    let pass12 = union_with(&p1, &p2, &BooleanInputs::default());
    for (name, r) in [("(p2,p3)", &pass23), ("(p1,p2)", &pass12)] {
        match r {
            Ok(out) => {
                println!("PROBE C pass {name}: {}", describe(out));
                for v in &out.verdicts {
                    println!("PROBE C pass {name} verdict: {v:?}");
                }
            }
            Err(e) => println!("PROBE C pass {name}: REFUSED {e:?}"),
        }
    }
    honest_fold("C [p1,p2,p3]", &[&p1, &p2, &p3]);
    honest_fold("C [p3,p2,p1]", &[&p3, &p2, &p1]);
    honest_fold("C [p2,p3,p1]", &[&p2, &p3, &p1]);
    let (Ok(pass23), Ok(pass12)) = (pass23, pass12) else {
        return;
    };
    // Today's design, order [p3, p2, p1]: step 1 is p3 ∪ p2 given the
    // (p2,p3) verdicts (the accumulation p3 is the verdict's B side);
    // step 2 is (p3∪p2) ∪ p1 given the (p1,p2) verdicts (the
    // accumulation holds p2's faces, the verdict's B side).
    let given1: Vec<_> = pass23
        .verdicts
        .iter()
        .filter_map(|v| rekey_plane(v, topo::Operand::B, &p3, &p3))
        .collect();
    println!("PROBE C given step 1: {} of {}", given1.len(), pass23.verdicts.len());
    let step1 = union_with(
        &p3,
        &p2,
        &BooleanInputs {
            given: Some(given1),
            ..BooleanInputs::default()
        },
    );
    let acc = match step1 {
        Ok(out) => {
            println!("PROBE C [p3,p2,p1] given step 1: {}", describe(&out));
            out.body
        }
        Err(e) => {
            println!("PROBE C [p3,p2,p1] given step 1: REFUSED {e:?}");
            return;
        }
    };
    let given2: Vec<_> = pass12
        .verdicts
        .iter()
        .filter_map(|v| rekey_plane(v, topo::Operand::B, &p2, &acc))
        .collect();
    println!("PROBE C given step 2: {} of {}", given2.len(), pass12.verdicts.len());
    match union_with(
        &acc,
        &p1,
        &BooleanInputs {
            given: Some(given2),
            ..BooleanInputs::default()
        },
    ) {
        Ok(out) => {
            println!("PROBE C [p3,p2,p1] given step 2: {}", describe(&out));
            println!(
                "PROBE C [p3,p2,p1] given: vertices off their faces' descriptions: {:e} (eps {eps:e})",
                worst_vertex_off_its_face(&out.body)
            );
            for (_, f) in out.body.faces() {
                if let Some(Surface::Plane { origin, normal, .. }) = out.body.get_surface(f.surface) {
                    if normal.z > 0.999 {
                        println!(
                            "PROBE C [p3,p2,p1] given: fused top description origin={:?} normal=({:e},{:e},{:e})",
                            origin, normal.x, normal.y, normal.z
                        );
                    }
                }
            }
        }
        Err(e) => println!("PROBE C [p3,p2,p1] given step 2: REFUSED {e:?}"),
    }
    // And the mirror: order [p1, p2, p3] with the given verdicts.
    if let Ok(acc) = union_with(&p1, &p2, &BooleanInputs::default()) {
        let given: Vec<_> = pass23
            .verdicts
            .iter()
            .filter_map(|v| rekey_plane(v, topo::Operand::A, &p2, &acc.body))
            .collect();
        match union_with(
            &acc.body,
            &p3,
            &BooleanInputs {
                given: Some(given),
                ..BooleanInputs::default()
            },
        ) {
            Ok(out) => println!(
                "PROBE C [p1,p2,p3] given: {} ; vertices off faces {:e}",
                describe(&out),
                worst_vertex_off_its_face(&out.body)
            ),
            Err(e) => println!("PROBE C [p1,p2,p3] given: REFUSED {e:?}"),
        }
    }
}
