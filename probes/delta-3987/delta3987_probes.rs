//! PR #3987 delta review (fix pass 1, head 07ca5a8d) — in-crate probes.
//! Mounted as a child of `boolean::refusal_routes`, beside `offer_rows`:
//! `#[cfg(test)] #[path = "delta3987_probes.rs"] mod delta3987_probes;`
//!
//! The F1 class at duals: the inside-out operand in a several-solid body,
//! through the public reduction, and the stranded operand; plus the F9
//! fence at f64.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic, clippy::print_stderr)]

use crate::boolean::{BooleanDeclarations, BooleanError, BooleanOp, BooleanResult, Operand};
use crate::body::Body;
use crate::props::AtRestPolicy;
use crate::test_support_fixtures as fx;
use geom_core::{Decide, Dual64, Point3, Tol, Vec3};

fn wedge(ccw: bool) -> [(f64, f64); 3] {
    let at = |deg: f64| (deg.to_radians().cos(), deg.to_radians().sin());
    if ccw {
        [(0.0, 0.0), at(80.0), at(190.0)]
    } else {
        [(0.0, 0.0), at(190.0), at(80.0)]
    }
}

type R<T> = Result<BooleanResult<T>, BooleanError>;

fn every_op<T: Decide + geom_core::Bounds + AtRestPolicy>(
    a: &crate::AtRestBody<T>,
    b: &crate::AtRestBody<T>,
    decls: &BooleanDeclarations,
    tol: Tol,
) -> [(&'static str, R<T>); 3] {
    [
        ("∪", crate::union_with(a, b, decls, tol)),
        ("∖", crate::subtract_with(a, b, decls, tol)),
        ("∩", crate::intersect_with(a, b, decls, tol)),
    ]
}

fn show<T: geom_core::Real>(r: &R<T>) -> String {
    match r {
        Ok(BooleanResult::Body(b)) => format!("Ok(Body, outcome {:?})", b.body.outcome()),
        Ok(other) => format!("Ok({:?})", core::mem::discriminant(other)),
        Err(e) => {
            let s = format!("{e:?}");
            s[..s.len().min(220)].to_string()
        }
    }
}

/// A box (0,2)²×(0,2) and, at x+5, a clockwise wedge prism: one body,
/// two solids, total volume positive.
fn two_parts<T: Decide + AtRestPolicy>(tol: Tol) -> Body<T> {
    let mut parts = Body::<T>::new();
    let shifted: Vec<(f64, f64)> = wedge(false).iter().map(|&(x, y)| (x + 5.0, y)).collect();
    for (profile, z) in [
        ([(0.0, 0.0), (2.0, 0.0), (2.0, 2.0), (0.0, 2.0)].to_vec(), (0.0, 2.0)),
        (shifted, (0.5, 1.0)),
    ] {
        fx::prism_ops(&mut parts, &profile, z, fx::identity_map, fx::FaceGeometry::Certified, tol);
    }
    fx::describe_as_intersections(&mut parts, tol);
    parts
}

/// F1 class: an inside-out SOLID inside a two-solid operand at a dual —
/// the read must be per solid, before `one_solid`.
#[test]
fn d_inside_out_part_of_a_two_solid_operand_at_a_dual() {
    let tol = Tol::witness();
    let parts = two_parts::<Dual64>(tol);
    assert_eq!(parts.solids().count(), 2);
    let brick = fx::brick::<Dual64>((0.5, 1.5), (0.5, 1.5), (1.0, 3.0), tol);
    let (p, b) = (
        Dual64::gate_at_rest_kept(parts, tol).unwrap(),
        Dual64::gate_at_rest_kept(brick, tol).unwrap(),
    );
    let none = BooleanDeclarations::none();
    let mut bad = Vec::new();
    for (x, y, want) in [(&p, &b, Operand::A), (&b, &p, Operand::B)] {
        for (name, r) in every_op(x, y, &none, tol) {
            eprintln!("two-part {name} (inside-out {want:?}): {}", show(&r));
            if !matches!(r, Err(BooleanError::InsideOutOperand { operand, .. }) if operand == want) {
                bad.push(format!("{name} {want:?}: {}", show(&r)));
            }
        }
    }
    assert!(bad.is_empty(), "not refused InsideOutOperand: {bad:#?}");
}

/// F1 + F9: the public reduction at a dual reads orientation too.
#[test]
fn d_public_boolean_reduce_at_a_dual_refuses_the_inside_out_wedge() {
    let tol = Tol::witness();
    let brick = fx::brick::<Dual64>((0.0, 1.0), (0.0, 1.0), (0.5, 1.5), tol);
    let w = fx::prism_z::<Dual64>(&wedge(false), 0.5, 1.0, tol).body;
    let decls = fx::flush_declarations(&brick, &w, tol);
    let (b, w) = (
        Dual64::gate_at_rest_kept(brick, tol).unwrap(),
        Dual64::gate_at_rest_kept(w, tol).unwrap(),
    );
    let mut bad = Vec::new();
    for op in [BooleanOp::Union, BooleanOp::Subtract, BooleanOp::Intersect] {
        for (x, y, want) in [(&b, &w, Operand::B), (&w, &b, Operand::A)] {
            let plain = crate::boolean::boolean_reduce(op, x, y, tol);
            let declared = crate::boolean::boolean_reduce_declared(op, x, y, &decls, tol);
            for (door, r) in [("reduce", plain), ("reduce_declared", declared)] {
                let ok = matches!(&r, Err(BooleanError::InsideOutOperand { operand, .. }) if *operand == want);
                let txt = match &r {
                    Ok(_) => "Ok(reduction)".to_string(),
                    Err(e) => format!("{e:?}"),
                };
                eprintln!("{door} {op:?} {want:?}: {}", &txt[..txt.len().min(160)]);
                if !ok {
                    bad.push(format!("{door} {op:?} {want:?}: {txt}"));
                }
            }
        }
    }
    assert!(bad.is_empty(), "{bad:#?}");
}

/// The stranded split top, generic over the scalar: the fixture of
/// `refusal_routes::tests::top_split_redescribed_face`, the plane lifted
/// `offset` above the top.
fn stranded<T: Decide + AtRestPolicy>(offset: f64) -> Body<T> {
    let tol = Tol::witness();
    let square = [(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)];
    let prism = fx::prism_z::<T>(&square, 0.0, 1.0, tol);
    let mut body = prism.body;
    let outer = body.get_face(prism.top_face).unwrap().outer;
    let crate::LoopBoundary::Cycle { first } = body.get_loop(outer).unwrap().boundary else {
        panic!()
    };
    let cycle = body.loop_cycle(first).unwrap();
    let at = |body: &Body<T>, he| {
        let v = body.get_half_edge(he).unwrap().start;
        *body.get_point(body.get_vertex(v).unwrap().point).unwrap()
    };
    let (p0, p1) = (at(&body, cycle[0]), at(&body, cycle[2]));
    let half = body
        .mef_chord(crate::euler::MefSite::Chords { he1: cycle[0], he2: cycle[2] }, tol)
        .unwrap();
    let up = Vec3::new(T::zero(), T::zero(), T::from_f64(1.0));
    let along = (p1 - p0) * (T::from_f64(1.0) / (p1 - p0).norm());
    let q0: Point3<T> = p0 + up * T::from_f64(offset);
    let surface = fx::plane(&[q0, q0 + along, q0 + up.cross(along)], tol);
    body.set_face_surface_stranding_for_tests(
        half.face,
        crate::euler::FaceSurface::New { surface, sense: true },
    )
    .unwrap();
    let chart = body.get_face(prism.top_face).unwrap().surface;
    body.set_edge_curve(
        half.edge,
        geom_brep::EdgeCurveSpec::line_between(p0, p1).at_rest_in_chart(chart, false),
        tol,
    )
    .unwrap();
    body
}

/// F1 class / F9 at a dual: where does the stranded operand end? Main
/// (r1/r2 evidence) ended it at `ClassificationInvariant` at every scalar;
/// at f64 the head refuses it at the at-rest gate. Reported, then
/// asserted only that no op SHIPS a body.
#[test]
fn d_stranded_operand_at_a_dual() {
    let tol = Tol::witness();
    let offset = 1e3 * tol.get().eps;
    let f = crate::AtRestBody::validate(stranded::<f64>(offset), tol);
    eprintln!("f64 at-rest gate on the generic stranded copy: {:?}", f.as_ref().map(|_| ()).map_err(|e| e.len()));
    assert!(f.is_err(), "the generic copy strands at f64 as the original does");
    let s = Dual64::gate_at_rest_kept(stranded::<Dual64>(offset), tol).unwrap();
    let brick = Dual64::gate_at_rest_kept(
        fx::brick::<Dual64>((0.3, 2.0), (0.2, 0.7), (0.5, 1.5), tol),
        tol,
    )
    .unwrap();
    let none = BooleanDeclarations::none();
    let mut shipped = Vec::new();
    for (x, y, tag) in [(&s, &brick, "stranded×brick"), (&brick, &s, "brick×stranded")] {
        for (name, r) in every_op(x, y, &none, tol) {
            eprintln!("dual {tag} {name}: {}", show(&r));
            if matches!(r, Ok(BooleanResult::Body(_))) {
                shipped.push(format!("{tag} {name}"));
            }
        }
        for op in [BooleanOp::Union, BooleanOp::Subtract, BooleanOp::Intersect] {
            let r = crate::boolean::boolean_reduce(op, x, y, tol);
            let txt = match &r {
                Ok(_) => "Ok(reduction)".to_string(),
                Err(e) => format!("{e:?}"),
            };
            eprintln!("dual reduce {tag} {op:?}: {}", &txt[..txt.len().min(200)]);
        }
    }
    assert!(shipped.is_empty(), "a stranded dual operand shipped a body: {shipped:?}");
}

/// F9 at f64: the r1 probe's stranded operand can no longer reach
/// `boolean_reduce` — the only door to `&AtRestBody<f64>` refuses it.
#[test]
fn d_stranded_operand_cannot_be_finished_at_f64() {
    let up = Vec3::new(0.0, 0.0, 1.0);
    let offset = 1e3 * Tol::witness().get().eps;
    let body = super::tests::top_split_redescribed(|p0, along, _| {
        fx::plane(&[p0 + up * offset, p0 + up * offset + along, p0 + up * offset + up.cross(along)], Tol::witness())
    });
    assert!(crate::AtRestBody::validate(body.clone(), Tol::witness()).is_err());
    assert!(f64::gate_at_rest_kept(body, Tol::witness()).is_err());
}

/// The lane's claim (lost-coverage item and `neighbours_across_a_closed_edge`):
/// no finished operand reaches the maximal-faces gate. Two candidates whose
/// shared edge is described at rest in one face's chart rather than left a
/// scaffold: the split top, both halves on the top's plane (no shared
/// source), and the disc planted in a top, its circle at rest in the top's
/// chart. If either finishes, the gate is reachable through a public door.
#[test]
fn d_coplanar_neighbours_with_an_at_rest_edge_against_the_finished_gate() {
    let tol = Tol::witness();
    let up = Vec3::new(0.0, 0.0, 1.0);
    let flat = super::tests::top_split_redescribed(|p0, along, _| {
        fx::plane(&[p0, p0 + along, p0 + up.cross(along)], tol)
    });
    // The disc in a 4×4×1 top, its circle at rest in the top's chart.
    let p = fx::prism_z::<f64>(&[(0.0, 0.0), (4.0, 0.0), (4.0, 4.0), (0.0, 4.0)], 0.0, 1.0, tol);
    let mut disc = p.body;
    let outer = disc.get_face(p.top_face).unwrap().outer;
    let crate::LoopBoundary::Cycle { first } = disc.get_loop(outer).unwrap().boundary else {
        panic!()
    };
    let made = fx::plant_disc_face(&mut disc, first, Point3::new(2.0, 2.0, 1.0), 1.0, tol);
    let chart = disc.get_face(p.top_face).unwrap().surface;
    let carrier = geom::Curve3::Circle {
        center: Point3::new(2.0, 2.0, 1.0),
        axis: up,
        radius: 1.0,
        u_ref: Vec3::new(1.0, 0.0, 0.0),
    };
    let spec = geom_brep::EdgeCurveSpec::arc_of_circle(carrier, 0.0, core::f64::consts::TAU)
        .unwrap()
        .at_rest_in_chart(chart, false);
    let mut disc_rev = disc.clone();
    let set = disc.set_edge_curve(made.edge, spec, tol);
    eprintln!("disc circle set at rest in the top's chart: {:?}", set.as_ref().map(|_| ()));
    // The same circle traversed the other way (axis −z).
    let rev = geom_brep::EdgeCurveSpec::arc_of_circle(
        geom::Curve3::Circle {
            center: Point3::new(2.0, 2.0, 1.0),
            axis: -up,
            radius: 1.0,
            u_ref: Vec3::new(1.0, 0.0, 0.0),
        },
        0.0,
        core::f64::consts::TAU,
    )
    .unwrap()
    .at_rest_in_chart(chart, false);
    eprintln!("reversed circle set: {:?}", disc_rev.set_edge_curve(made.edge, rev, tol).as_ref().map(|_| ()));
    let far = crate::AtRestBody::validate(fx::brick::<f64>((10.0, 11.0), (0.0, 1.0), (0.0, 1.0), tol), tol)
        .unwrap();
    let mut finished = Vec::new();
    for (what, body) in [("flat split top", flat), ("disc at rest", disc), ("disc at rest, reversed", disc_rev)] {
        match crate::AtRestBody::validate(body.clone(), tol) {
            Err(e) => eprintln!("{what}: not finished: {e:?}"),
            Ok(kept) => {
                let gate = crate::boolean::maximal_faces_gate(&body, Operand::A, tol);
                let r = crate::union(&kept, &far, tol);
                eprintln!("{what}: FINISHED; maximal-faces gate {:?}; ∪ far: {}", gate.as_ref().map(|_| ()), show(&r));
                finished.push(what);
            }
        }
    }
    eprintln!("finished coplanar-neighbour operands: {finished:?}");
}

/// The structural arm: a top split by a chord with both halves on the
/// top's own surface key, the chord at rest in that chart. Finished?
#[test]
fn d_split_top_sharing_one_surface_key_against_the_finished_gate() {
    let tol = Tol::witness();
    let prism = fx::prism_z::<f64>(&[(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)], 0.0, 1.0, tol);
    let mut body = prism.body;
    let outer = body.get_face(prism.top_face).unwrap().outer;
    let crate::LoopBoundary::Cycle { first } = body.get_loop(outer).unwrap().boundary else {
        panic!()
    };
    let cycle = body.loop_cycle(first).unwrap();
    let at = |body: &Body<f64>, he| {
        let v = body.get_half_edge(he).unwrap().start;
        *body.get_point(body.get_vertex(v).unwrap().point).unwrap()
    };
    let (p0, p1) = (at(&body, cycle[0]), at(&body, cycle[2]));
    let half = body
        .mef_chord(crate::euler::MefSite::Chords { he1: cycle[0], he2: cycle[2] }, tol)
        .unwrap();
    let chart = body.get_face(prism.top_face).unwrap().surface;
    eprintln!(
        "halves share a surface key: {}",
        body.get_face(half.face).unwrap().surface == chart
    );
    for seam in [false, true] {
        let mut b = body.clone();
        let set = b.set_edge_curve(
            half.edge,
            geom_brep::EdgeCurveSpec::line_between(p0, p1).at_rest_in_chart(chart, seam),
            tol,
        );
        let far =
            crate::AtRestBody::validate(fx::brick::<f64>((10.0, 11.0), (0.0, 1.0), (0.0, 1.0), tol), tol).unwrap();
        match crate::AtRestBody::validate(b.clone(), tol) {
            Err(e) => eprintln!("seam {seam} (set {:?}): not finished: {e:?}", set.map(|_| ())),
            Ok(kept) => eprintln!(
                "seam {seam}: FINISHED; ∪ far: {}",
                show(&crate::union(&kept, &far, tol))
            ),
        }
    }
}

/// HONE's note: "no dual construction strands a face". The public
/// `set_face_surface` with the lifted plane, at f64 and at a dual.
fn strand_publicly<T: Decide + AtRestPolicy>() -> String {
    let tol = Tol::witness();
    let square = [(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)];
    let prism = fx::prism_z::<T>(&square, 0.0, 1.0, tol);
    let mut body = prism.body;
    let outer = body.get_face(prism.top_face).unwrap().outer;
    let crate::LoopBoundary::Cycle { first } = body.get_loop(outer).unwrap().boundary else {
        panic!()
    };
    let cycle = body.loop_cycle(first).unwrap();
    let at = |body: &Body<T>, he| {
        let v = body.get_half_edge(he).unwrap().start;
        *body.get_point(body.get_vertex(v).unwrap().point).unwrap()
    };
    let (p0, p1) = (at(&body, cycle[0]), at(&body, cycle[2]));
    let half = body
        .mef_chord(crate::euler::MefSite::Chords { he1: cycle[0], he2: cycle[2] }, tol)
        .unwrap();
    let up = Vec3::new(T::zero(), T::zero(), T::from_f64(1.0));
    let along = (p1 - p0) * (T::from_f64(1.0) / (p1 - p0).norm());
    let q0: Point3<T> = p0 + up * T::from_f64(1e3 * tol.get().eps);
    let surface = fx::plane(&[q0, q0 + along, q0 + up.cross(along)], tol);
    format!(
        "{:?}",
        body.set_face_surface(half.face, crate::euler::FaceSurface::New { surface, sense: true })
            .map(|_| ())
    )
}

#[test]
fn d_does_a_public_rechart_strand_at_a_dual() {
    eprintln!("f64  set_face_surface(lifted): {}", strand_publicly::<f64>());
    eprintln!("dual set_face_surface(lifted): {}", strand_publicly::<Dual64>());
}

/// The in-band kink (the coincv5 pose: walls turning by `theta` at a
/// short edge) with its one scaffold, the kink edge, set at rest in one
/// wall's chart instead. Finished? Then the in-band `CoplanarNeighbours`
/// lever is reachable through a public door from a finished operand.
#[test]
fn d_kinked_prism_with_its_kink_at_rest() {
    let tol = Tol::witness();
    for theta in [5.5e-10, 1.1e-9, 1e-6] {
        let h = 0.1;
        let profile = [(0.0, 0.0), (0.1, 0.0), (10.1, 10.0 * f64::tan(theta)), (10.1, 1.0), (0.0, 1.0)];
        let mut body = fx::prism_z::<f64>(&profile, 0.0, h, tol).body;
        let Err(errors) = crate::AtRestBody::validate(body.clone(), tol) else {
            eprintln!("theta {theta:e}: the raw kinked prism already finishes");
            continue;
        };
        let scaff: Vec<_> = errors
            .iter()
            .filter_map(|e| match e {
                crate::ValidationError::ScaffoldAtRest { edge } => Some(*edge),
                _ => None,
            })
            .collect();
        eprintln!("theta {theta:e}: raw refusals {errors:?}");
        for edge in scaff {
            let he = body.get_edge(edge).unwrap().he_plus;
            let h0 = body.get_half_edge(he).unwrap();
            let face = body.get_loop(h0.parent_loop).unwrap().face;
            let chart = body.get_face(face).unwrap().surface;
            let twin = body.get_edge(edge).unwrap().he_minus;
            let p = |v| *body.get_point(body.get_vertex(v).unwrap().point).unwrap();
            let (p0, p1) = (p(h0.start), p(body.get_half_edge(twin).unwrap().start));
            let r = body.set_edge_curve(
                edge,
                geom_brep::EdgeCurveSpec::line_between(p0, p1).at_rest_in_chart(chart, false),
                tol,
            );
            eprintln!("  set kink edge at rest: {:?}", r.map(|_| ()));
        }
        let far = crate::AtRestBody::validate(fx::brick::<f64>((20.0, 21.0), (0.0, 1.0), (0.0, 1.0), tol), tol).unwrap();
        match crate::AtRestBody::validate(body, tol) {
            Err(e) => eprintln!("  not finished: {e:?}"),
            Ok(kept) => eprintln!("  FINISHED; ∪ far: {}", show(&crate::union(&kept, &far, tol))),
        }
    }
}
