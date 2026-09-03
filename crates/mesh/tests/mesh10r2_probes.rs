//! R2 review probes for MESH-10 (issue 1562, the torus meridian fold
//! keyed on split lineage). Each row measures one hypothesis about the
//! fold's KEY — the split lineage as `topo::props::loop_edges` stamps
//! it — outside the unit's own fixtures, and prints what it found:
//!
//! * the lineage after a graft: `Provenance::SplitEdge { edge }` is
//!   copied VERBATIM into the destination arena, so the recorded
//!   parent is a key of the SOURCE arena — does a split-seam donut
//!   still fold after `graft_disjoint` / a disjoint `union`?;
//! * `set_edge_curve` on a split child (the PR's disclosed unmeasured
//!   limit): a child re-parametrised on the same carrier — what does
//!   every consumer answer?;
//! * the split-donut mesh measured independently of the unit's row.
#![allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::print_stdout,
    clippy::float_cmp
)]

mod common;
use common::*;
use geom::Curve3;
use geom_core::Tol;
use profile::RawLoop;
use topo::Body;

/// The tour donut with its seam meridian (edge 0) split at `fracs`.
fn split_seam_donut(fracs: &[f64]) -> (Body<f64>, topo::EdgeKey, Vec<topo::EdgeKey>) {
    let tol = Tol::witness();
    let mut body = donut();
    let (seam, edge) = body.edges().next().unwrap();
    let curve = body
        .get_curve_geom(edge.curve)
        .unwrap()
        .certified()
        .unwrap()
        .clone();
    let (t0, t1) = curve.params();
    let mut minted = Vec::new();
    for f in fracs {
        let c = body
            .split_edge(seam, t0 + f * (t1 - t0), tol)
            .expect("splitting the seam meridian");
        minted.push(c.new_edge);
    }
    (body, seam, minted)
}

fn volume(b: &Body<f64>) -> Result<f64, topo::MassPropsError> {
    topo::mass_properties(b, Tol::witness()).map(|m| m.volume)
}

/// **The split-seam donut's mesh, measured independently.** Positions
/// as bit-pattern sets: the two meshes differ only on the seam minor
/// circle, the split one carries exactly one more position, both are
/// watertight, and every position of both lies on the torus.
#[test]
fn m10r2_split_donut_mesh_differs_only_on_the_seam_column() {
    let tol = Tol::witness();
    let (body, _, _) = split_seam_donut(&[0.5]);
    let m0 = mesh::tessellate(&donut(), 0.1, tol).unwrap();
    let m = mesh::tessellate(&body, 0.1, tol).unwrap();
    mesh::validate::check_mesh(&m0).unwrap();
    mesh::validate::check_mesh(&m).unwrap();
    let key = |p: &geom_core::Point3<f64>| [p.x.to_bits(), p.y.to_bits(), p.z.to_bits()];
    let mut a: Vec<_> = m0.positions.iter().map(key).collect();
    let mut b: Vec<_> = m.positions.iter().map(key).collect();
    a.sort_unstable();
    b.sort_unstable();
    let only_a: Vec<_> = a.iter().filter(|k| b.binary_search(k).is_err()).collect();
    let only_b: Vec<_> = b.iter().filter(|k| a.binary_search(k).is_err()).collect();
    let on_torus = |p: &geom_core::Point3<f64>| {
        // Axis +Y through the origin, R = 2, r = 0.5.
        let rho = (p.x * p.x + p.z * p.z).sqrt();
        (((rho - 2.0).powi(2) + p.y * p.y).sqrt() - 0.5).abs()
    };
    let worst = m
        .positions
        .iter()
        .chain(&m0.positions)
        .map(on_torus)
        .fold(0.0f64, f64::max);
    let seam = |k: &[u64; 3]| {
        let (x, y, z) = (
            f64::from_bits(k[0]),
            f64::from_bits(k[1]),
            f64::from_bits(k[2]),
        );
        z.abs() < 1e-12 && (((x - 2.0).powi(2) + y * y).sqrt() - 0.5).abs() < 1e-12
    };
    println!(
        "M10R2 split-donut: unsplit {} pos / split {} pos; only-unsplit {} only-split {}; worst off-torus {worst:e}",
        m0.positions.len(),
        m.positions.len(),
        only_a.len(),
        only_b.len()
    );
    assert_eq!(m.positions.len(), m0.positions.len() + 1);
    assert!(only_a.iter().all(|k| seam(k)) && only_b.iter().all(|k| seam(k)));
    assert!(worst < 1e-12);
    assert_eq!(only_b.len(), only_a.len() + 1);
}

/// **The lineage after a graft.** `graft_disjoint` (and the boolean's
/// disjoint-union path, which transplants the same way) copies each
/// edge's `Provenance::SplitEdge { edge }` record verbatim, so the
/// parent it names is a key of the SOURCE arena. Into an EMPTY
/// destination the fresh keys happen to coincide with the source's;
/// into a destination that already holds a body they do not. This
/// row measures both, through `mass_properties` and `tessellate`, and
/// prints what the stamped ids look like on the grafted face.
///
/// **Standing as a pinned FINDING (issue 1597), not flipped.** The
/// fix shape — forward every key-carrying record through the graft's
/// maps — was built and measured against editor-core's names lane:
/// its B-descent (`emit_topo.rs`'s `chase_b`) reads the result body's
/// verbatim `SplitEdge` keys to reach ancestors that DIED in B before
/// the graft (B's table still names them), and once the graft forwards
/// such a key to null that anchor is gone — eight names-lane rows on
/// the die corpus go red under every chase that reads forwarded
/// records, and B's operand body cannot be chased instead (it is not
/// the body that was grafted; a placed copy is). Forwarding
/// `SplitEdge` needs a dead-ancestor bridge on the `GraftMap` first;
/// forwarding every OTHER variant is harmless (measured) and waits
/// with it, so the graft copies as it did.
#[test]
fn m10r2_split_lineage_after_graft() {
    let tol = Tol::witness();
    let v_donut = volume(&donut()).unwrap();
    let (src, seam, minted) = split_seam_donut(&[0.5]);
    println!(
        "M10R2 source: seam {seam:?} minted {minted:?}, V = {:?}",
        volume(&src)
    );
    // (a) into an empty body.
    let mut empty = Body::<f64>::new();
    topo::graft_disjoint(&mut empty, &src, tol).expect("graft into an empty body");
    let v_empty = volume(&empty);
    let mesh_empty = mesh::tessellate(&empty, 0.1, tol).map(|m| m.positions.len());
    println!("M10R2 graft into EMPTY: V = {v_empty:?}, mesh = {mesh_empty:?}");
    // (b) into a body already holding the ball (disjoint: the ball sits
    // in the donut's hole).
    let mut held = ball();
    let v_ball = volume(&held).unwrap();
    topo::graft_disjoint(&mut held, &src, tol).expect("graft into the ball's body");
    let v_held = volume(&held);
    let mesh_held = mesh::tessellate(&held, 0.1, tol).map(|m| m.positions.len());
    println!(
        "M10R2 graft into BALL: V = {v_held:?} (expect {}), mesh = {mesh_held:?}",
        v_ball + v_donut
    );
    for (fk, f) in held.faces() {
        let Ok((outer, _)) = topo::props::loop_edges(&held, f.outer) else {
            continue;
        };
        let ids: Vec<_> = outer
            .iter()
            .filter(|e| matches!(e.carrier, Curve3::Circle { radius, .. } if (radius - 0.5).abs() < 1e-12))
            .map(|e| (e.forward, e.carrier_id, e.t0, e.t1))
            .collect();
        if !ids.is_empty() {
            println!("M10R2   face {fk:?} minor-circle arcs: {ids:?}");
        }
    }
    // (c) the boolean's disjoint union, both operand orders: with the
    // ball (a curved pair, refused before any graft) and with a far
    // box (disjoint operands, grafted).
    let u1 = topo::union(&ball(), &src, tol).map(|r| r.body().map(|b| volume(&b.body)));
    let u2 = topo::union(&src, &ball(), tol).map(|r| r.body().map(|b| volume(&b.body)));
    println!("M10R2 union(ball, split donut): {u1:?}");
    println!("M10R2 union(split donut, ball): {u2:?}");
    let far_box = || {
        sweep::extrude(
            &validated(vec![profile::ProfileLoop::<f64>::polygon([
                p2(10.0, 10.0),
                p2(11.0, 10.0),
                p2(11.0, 11.0),
                p2(10.0, 11.0),
            ])]),
            sweep::Extrusion::Distance(1.0),
            tol,
        )
        .unwrap()
        .body
    };
    let u3 = topo::union(&far_box(), &src, tol).map(|r| r.body().map(|b| volume(&b.body)));
    let u4 = topo::union(&src, &far_box(), tol).map(|r| r.body().map(|b| volume(&b.body)));
    println!(
        "M10R2 union(far box, split donut): {u3:?} (expect {})",
        1.0 + v_donut
    );
    println!("M10R2 union(split donut, far box): {u4:?}");
    // The findings are asserted as measured so a change is visible.
    assert_eq!(
        v_empty.map(f64::to_bits),
        Ok(v_donut.to_bits()),
        "empty destination"
    );
    assert!(
        matches!(&v_held, Err(topo::MassPropsError::Face { .. })),
        "the grafted split-seam donut in a held body: {v_held:?}"
    );
}

/// **`set_edge_curve` on a split child.** The second child of the
/// split seam is re-certified on the SAME minor circle with `u_ref`
/// turned by φ, so its stored interval shifts by −φ while its lineage
/// stands. This row was written to show the fold reading
/// `[first.t0, last.t1]` across two parametrisations: for φ = π/2 a
/// visibly wrong span, for φ = 1e-12 a sub-band shift every consumer
/// ANSWERED, the volume hundreds of ulps from the donut's. Inverted:
/// the pieces must meet exactly, so BOTH shifts refuse
/// `props_meridian_pieces_meet` — the sub-ε one too, because the
/// split's own `t` is a structural fact, not a value within ε — while
/// the unsplit edge shifted the same way still answers bitwise (one
/// edge's span is shift-invariant).
#[test]
fn m10r2_set_edge_curve_on_a_split_child() {
    let tol = Tol::witness();
    let v_donut = volume(&donut()).unwrap();
    // The control: the UNSPLIT seam re-parametrised the same way. One
    // edge's span is shift-invariant, so every consumer answers the
    // donut bitwise; only a fold across two parametrisations is not.
    for (split, phi) in [
        (false, 1e-12),
        (true, core::f64::consts::FRAC_PI_2),
        (true, 1e-12),
    ] {
        let (mut body, seam, minted) = if split {
            split_seam_donut(&[0.5])
        } else {
            let (b, s, _) = split_seam_donut(&[]);
            (b, s, vec![s])
        };
        let _ = seam;
        let child = minted[0];
        let edge = body.get_edge(child).unwrap().clone();
        let cert = body
            .get_curve_geom(edge.curve)
            .unwrap()
            .certified()
            .unwrap()
            .clone();
        let mut spec = cert.restated_spec();
        let Curve3::Circle {
            center,
            axis,
            radius,
            u_ref,
        } = spec.carrier
        else {
            panic!("the seam is a circle")
        };
        let w = axis.cross(u_ref);
        spec.carrier = Curve3::Circle {
            center,
            axis,
            radius,
            u_ref: u_ref * phi.cos() + w * phi.sin(),
        };
        spec.param_start -= phi;
        spec.param_end -= phi;
        println!(
            "M10R2 split={split} phi={phi:e}: edge interval {:?} -> [{}, {}], description {:?}",
            cert.params(),
            spec.param_start,
            spec.param_end,
            spec.description
        );
        let set = body.set_edge_curve(child, spec, tol);
        println!(
            "M10R2 split={split} phi={phi:e}: set_edge_curve -> {:?}",
            set.as_ref().map(|_| ())
        );
        if set.is_err() {
            continue;
        }
        let v = volume(&body);
        let m = mesh::tessellate(&body, 0.1, tol).map(|m| m.positions.len());
        println!(
            "M10R2 split={split} phi={phi:e}: V = {v:?} (donut {v_donut}), tessellate = {m:?}"
        );
        if split {
            let meet = geom_brep::props::PropsError::NotIsoRectangle {
                what: "props_meridian_pieces_meet",
            };
            assert!(
                matches!(&v, Err(topo::MassPropsError::Face { source, .. }) if *source == meet),
                "split={split} phi={phi:e}: the shifted piece no longer meets its sibling: {v:?}"
            );
            assert!(
                matches!(&m, Err(mesh::TessellateError::UnsupportedCurvedShape { source, .. }) if *source == meet),
                "split={split} phi={phi:e}: tessellate refuses by the same name: {m:?}"
            );
        } else {
            let v = v.expect("one edge's span is shift-invariant");
            let ulps = (v.to_bits() as i64 - v_donut.to_bits() as i64).abs();
            println!(
                "M10R2 split={split} phi={phi:e}: V - donut = {:e}, ulps apart: {ulps}",
                v - v_donut
            );
            assert_eq!(ulps, 0, "one edge's span is shift-invariant");
        }
    }
}
