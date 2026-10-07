//! Review probes for PR #4209 (the plane–plane mitre), lane
//! `band-dual-4209-r1`. Each probe asserts soundness of whatever
//! builds (tier 3, tier 3′, Euler, an independent volume oracle) and
//! prints what refuses, so the run log is the measurement.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::{Point2, Point3, Vec3};
use sweep::blend::BlendError;
use sweep::blend::battery::TURN_OVERRUN;
use sweep::test_support::{block, prism_on, realized, sketch_from_axes};
use topo::boolean::BooleanOp;
use topo::{Body, ContactRecords, EdgeKey, validate_geometric, validate_pseudomanifold};

use crate::band_planar_cut_off::{D, Verb, edge, tol, volume_enclosure};
use crate::common::cavity::{brick, cut};

fn check_sound(what: &str, verb: Verb, out: &sweep::blend::build::Blended<f64>) {
    validate_geometric(&out.body, tol())
        .unwrap_or_else(|e| panic!("{what} {verb:?}: tier 3 {e:?}"));
    validate_pseudomanifold(&out.body, &ContactRecords::default(), tol())
        .unwrap_or_else(|e| panic!("{what} {verb:?}: tier 3′ {e:?}"));
    let c = topo::readback::euler_counts(&out.body);
    assert!(c.genus().is_ok(), "{what} {verb:?}: Euler {c:?}");
}

fn report(what: &str, verb: Verb, r: &Result<sweep::blend::build::Blended<f64>, BlendError>) {
    match r {
        Ok(_) => eprintln!("PROBE {what} {verb:?}: BUILT"),
        Err(e) => eprintln!("PROBE {what} {verb:?}: REFUSED {e}"),
    }
}

/// A refusal that is not a typed user-facing one is a defect.
fn typed(what: &str, verb: Verb, e: &BlendError) {
    assert!(
        !matches!(
            e,
            BlendError::BodyNotIntact { .. } | BlendError::SurgeryInvariant { .. }
        ),
        "{what} {verb:?}: an internal refusal {e:?}"
    );
}

/// The parallelepiped `{0 ≤ z ≤ 1, s z ≤ x ≤ 2 + s z, s z ≤ y ≤ 1.5 + s z}`,
/// two parallelogram prisms intersected; lateral edges along `(s, s, 1)`.
/// Its top corners `(s, s, 1)` and `(2 + s, 1.5 + s, 1)` are isosceles
/// turns about the lateral edge; the other two have supplementary face
/// angles.
fn parallelepiped(s: f64) -> Body<f64> {
    parallelepiped_at(s, Point3::new(0.0, 0.0, 0.0), 1.0)
}

/// [`parallelepiped`] with its low corner at `o` and height `h`.
fn parallelepiped_at(s: f64, o: Point3<f64>, h: f64) -> Body<f64> {
    let z = Vec3::new(0.0, 0.0, 1.0);
    let a = prism_on(
        sketch_from_axes(
            Point3::new(o.x, o.y + 3.5, o.z),
            Vec3::new(1.0, 0.0, 0.0),
            z,
            tol(),
        ),
        vec![
            (Point2::new(0.0, 0.0), 0.0),
            (Point2::new(2.0, 0.0), 0.0),
            (Point2::new(2.0 + s * h, h), 0.0),
            (Point2::new(s * h, h), 0.0),
        ],
        5.0,
        tol(),
    );
    let (z0, z1) = (-0.5, h + 0.5);
    let b = prism_on(
        sketch_from_axes(
            Point3::new(o.x + 3.5, o.y, o.z),
            Vec3::new(0.0, -1.0, 0.0),
            z,
            tol(),
        ),
        vec![
            (Point2::new(-1.5 - s * z0, z0), 0.0),
            (Point2::new(-s * z0, z0), 0.0),
            (Point2::new(-s * z1, z1), 0.0),
            (Point2::new(-1.5 - s * z1, z1), 0.0),
        ],
        5.0,
        tol(),
    );
    let body = realized(BooleanOp::Intersect, &a, &b, tol());
    validate_geometric(&body, tol()).expect("the parallelepiped is tier-3 valid");
    body
}

/// The half-space `(x − q)·n ≤ 0` within a 10-wide slab: a prism on the
/// plane extruded along `−n`.
fn half_space(q: Point3<f64>, n: Vec3<f64>) -> Body<f64> {
    let n = n.normalize();
    let seed = if n.x.abs() < 0.9 {
        Vec3::new(1.0, 0.0, 0.0)
    } else {
        Vec3::new(0.0, 1.0, 0.0)
    };
    let u = (seed - n * seed.dot(n)).normalize();
    let v = u.cross(n);
    let o = q - u * 5.0 - v * 5.0;
    prism_on(
        sketch_from_axes(o, u, v, tol()),
        vec![
            (Point2::new(0.0, 0.0), 0.0),
            (Point2::new(10.0, 0.0), 0.0),
            (Point2::new(10.0, 10.0), 0.0),
            (Point2::new(0.0, 10.0), 0.0),
        ],
        10.0,
        tol(),
    )
}

/// The chamfer plane of a convex edge at `p`: through the trimline
/// `p + d·m_a`, normal `n_a + n_b` (`arms::chamfer_strip`).
fn chamfer_keep(p: Point3<f64>, m_a: Vec3<f64>, n_a: Vec3<f64>, n_b: Vec3<f64>) -> Body<f64> {
    half_space(p + m_a * D, n_a.normalize() + n_b.normalize())
}

/// **Oblique isosceles turn, convex**: the parallelepiped's corner
/// `(s, s, 1)`, its two top edges chamfered and filleted. The chamfer
/// is checked against the body cut by its two chamfer half-spaces
/// (the body is convex, so that is the chamfered solid).
#[test]
fn an_oblique_isosceles_turn_on_a_parallelepiped() {
    for s in [0.3, 0.6, -0.4] {
        let body = parallelepiped(s);
        let v = [s, s, 1.0];
        let ex = edge(&body, v, [2.0 + s, s, 1.0]);
        let ey = edge(&body, v, [s, 1.5 + s, 1.0]);
        let n = (1.0 + s * s).sqrt();
        let top = Vec3::new(0.0, 0.0, 1.0);
        let keep_x = chamfer_keep(
            Point3::new(s, s, 1.0),
            Vec3::new(0.0, 1.0, 0.0),
            top,
            Vec3::new(0.0, -1.0, s) / n,
        );
        let keep_y = chamfer_keep(
            Point3::new(s, s, 1.0),
            Vec3::new(1.0, 0.0, 0.0),
            top,
            Vec3::new(-1.0, 0.0, s) / n,
        );
        let oracle = realized(
            BooleanOp::Intersect,
            &realized(BooleanOp::Intersect, &body, &keep_x, tol()),
            &keep_y,
            tol(),
        );
        for verb in [Verb::Chamfer, Verb::Fillet] {
            let what = format!("parallelepiped s={s} isosceles corner");
            let r = verb.run(&body, &[ex, ey]);
            report(&what, verb, &r);
            match r {
                Ok(out) => {
                    check_sound(&what, verb, &out);
                    let got = volume_enclosure(&out.body).0;
                    let (n_arcs, stray) = crate::band_planar_cut_off::arc_residual(&out);
                    eprintln!("PROBE {what} {verb:?}: arcs {n_arcs} stray {stray:e}");
                    assert!(stray < 1e-12, "{what} {verb:?}: stray {stray}");
                    if let Verb::Chamfer = verb {
                        let want = volume_enclosure(&oracle).0;
                        eprintln!(
                            "PROBE {what}: chamfer V {got} oracle {want} diff {}",
                            got - want
                        );
                        assert!((got - want).abs() < 1e-9, "{what}: {got} vs {want}");
                    }
                }
                Err(e) => typed(&what, verb, &e),
            }
        }
    }
}

/// **The supplementary corner** `(2 + s, s, 1)`: face angles `φ` and
/// `π − φ`. The chamfer's two trimlines meet the lateral edge at one
/// point there (`d / sin φ` both), yet the verdict refuses it.
#[test]
fn a_supplementary_turn_on_a_parallelepiped() {
    let s = 0.3;
    let body = parallelepiped(s);
    let v = [2.0 + s, s, 1.0];
    let ex = edge(&body, v, [s, s, 1.0]);
    let ey = edge(&body, v, [2.0 + s, 1.5 + s, 1.0]);
    for verb in [Verb::Chamfer, Verb::Fillet] {
        let r = verb.run(&body, &[ex, ey]);
        report("parallelepiped supplementary corner", verb, &r);
        match r {
            Err(BlendError::UnsupportedRunOut { detail, .. }) => assert_eq!(detail, TURN_OVERRUN),
            Ok(out) => check_sound("supplementary", verb, &out),
            Err(e) => typed("supplementary", verb, &e),
        }
    }
}

/// **A void under an oblique mitre**: the parallelepiped (s = 0.3) with
/// a sealed void whose vertical edge lies in the mitre plane `x = y`;
/// its top swept up toward the cap. Whatever builds must leave the void
/// untouched: ΔV equal to the void-free carve's.
#[test]
fn a_void_under_an_oblique_mitre() {
    let s = 0.3;
    let clean = parallelepiped(s);
    let v = [s, s, 1.0];
    let turn = |b: &Body<f64>| [edge(b, v, [2.0 + s, s, 1.0]), edge(b, v, [s, 1.5 + s, 1.0])];
    for verb in [Verb::Chamfer, Verb::Fillet] {
        let base = verb
            .run(&clean, &turn(&clean))
            .expect("the clean turn builds");
        let dv0 = volume_enclosure(&clean).0 - volume_enclosure(&base.body).0;
        for top in [0.85, 0.9, 0.93, 0.95, 0.96, 0.97, 0.98, 0.99] {
            for off in [0.01, 0.03, 0.05] {
                let lo = s + off;
                let body = cut(
                    "void",
                    &clean,
                    &brick(
                        Point3::new(lo, lo, 0.5),
                        Point3::new(lo + 0.25, lo + 0.25, top),
                    ),
                );
                let what = format!("oblique void top={top} off={off}");
                let r = verb.run(&body, &turn(&body));
                report(&what, verb, &r);
                match r {
                    Ok(out) => {
                        check_sound(&what, verb, &out);
                        let dv = volume_enclosure(&body).0 - volume_enclosure(&out.body).0;
                        assert!(
                            (dv - dv0).abs() < 1e-7,
                            "{what} {verb:?}: ΔV {dv} vs {dv0}: the void was cut"
                        );
                    }
                    Err(e) => typed(&what, verb, &e),
                }
            }
        }
    }
}

/// **A short third edge**: the box `2 × 1.5 × h`, its two top edges at
/// the origin corner; the foot lands at `h − d` on the vertical edge.
#[test]
fn a_turn_over_a_short_third_edge() {
    for h in [0.25, 0.15, 0.11, 0.1, 0.09, 0.05] {
        let body = block(2.0, 1.5, h, tol());
        let e = [
            edge(&body, [0.0, 0.0, h], [2.0, 0.0, h]),
            edge(&body, [0.0, 0.0, h], [0.0, 1.5, h]),
        ];
        for verb in [Verb::Chamfer, Verb::Fillet] {
            let what = format!("short L h={h}");
            let r = verb.run(&body, &e);
            report(&what, verb, &r);
            match r {
                Ok(out) => check_sound(&what, verb, &out),
                Err(e) => typed(&what, verb, &e),
            }
        }
    }
}

/// **Two turns on one third edge**: top and bottom pairs at the origin
/// corner's vertical edge, whose feet meet as `h → 2d`.
#[test]
fn two_turns_share_a_third_edge() {
    for h in [1.0, 0.25, 0.21, 0.2, 0.19, 0.15] {
        let body = block(2.0, 1.5, h, tol());
        let e = [
            edge(&body, [0.0, 0.0, h], [2.0, 0.0, h]),
            edge(&body, [0.0, 0.0, h], [0.0, 1.5, h]),
            edge(&body, [0.0, 0.0, 0.0], [2.0, 0.0, 0.0]),
            edge(&body, [0.0, 0.0, 0.0], [0.0, 1.5, 0.0]),
        ];
        for verb in [Verb::Chamfer, Verb::Fillet] {
            let what = format!("two turns on L h={h}");
            let r = verb.run(&body, &e);
            report(&what, verb, &r);
            match r {
                Ok(out) => {
                    check_sound(&what, verb, &out);
                    let dv = volume_enclosure(&body).0 - volume_enclosure(&out.body).0;
                    let ov = match verb {
                        Verb::Chamfer => D.powi(3) / 3.0,
                        Verb::Fillet => (5.0 / 3.0 - core::f64::consts::PI / 2.0) * D.powi(3),
                    };
                    let want = verb.section() * 7.0 - 2.0 * ov;
                    eprintln!("PROBE {what} {verb:?}: dV {dv} want {want}");
                    assert!(
                        (dv - want).abs() < 1e-8,
                        "{what} {verb:?}: ΔV {dv} vs {want}"
                    );
                }
                Err(e) => typed(&what, verb, &e),
            }
        }
    }
}

/// **A turn and a cut-off on one third edge**: the top pair turns at
/// the origin corner; the bottom front edge alone is cut off there, its
/// end face the left wall, splitting the vertical edge at `d`.
#[test]
fn a_turn_and_a_cut_off_share_a_third_edge() {
    for h in [1.0, 0.25, 0.21, 0.2, 0.19, 0.15] {
        let body = block(2.0, 1.5, h, tol());
        let e = [
            edge(&body, [0.0, 0.0, h], [2.0, 0.0, h]),
            edge(&body, [0.0, 0.0, h], [0.0, 1.5, h]),
            edge(&body, [0.0, 0.0, 0.0], [2.0, 0.0, 0.0]),
        ];
        for verb in [Verb::Chamfer, Verb::Fillet] {
            let what = format!("turn + cut-off on L h={h}");
            let r = verb.run(&body, &e);
            report(&what, verb, &r);
            match r {
                Ok(out) => {
                    check_sound(&what, verb, &out);
                    let dv = volume_enclosure(&body).0 - volume_enclosure(&out.body).0;
                    let ov = match verb {
                        Verb::Chamfer => D.powi(3) / 3.0,
                        Verb::Fillet => (5.0 / 3.0 - core::f64::consts::PI / 2.0) * D.powi(3),
                    };
                    let want = verb.section() * 5.5 - ov;
                    eprintln!("PROBE {what} {verb:?}: dV {dv} want {want}");
                    assert!(
                        (dv - want).abs() < 1e-8,
                        "{what} {verb:?}: ΔV {dv} vs {want}"
                    );
                }
                Err(e) => typed(&what, verb, &e),
            }
        }
    }
}

/// **A notch on the third edge below the foot**: the vertical edge at
/// the origin corner shortened by a notch `[−1, w]² × [z0, top]` cut out
/// of it, so the third edge runs from `top` to `1`.
#[test]
fn a_notch_on_the_third_edge_under_the_foot() {
    for top in [0.85, 0.89, 0.9, 0.91, 0.95] {
        for w in [0.05, 0.2] {
            let body = cut(
                "notch",
                &block(2.0, 1.5, 1.0, tol()),
                &brick(Point3::new(-1.0, -1.0, 0.3), Point3::new(w, w, top)),
            );
            let e = [
                edge(&body, [0.0, 0.0, 1.0], [2.0, 0.0, 1.0]),
                edge(&body, [0.0, 0.0, 1.0], [0.0, 1.5, 1.0]),
            ];
            for verb in [Verb::Chamfer, Verb::Fillet] {
                let what = format!("notch top={top} w={w}");
                let r = verb.run(&body, &e);
                report(&what, verb, &r);
                match r {
                    Ok(out) => {
                        check_sound(&what, verb, &out);
                        let dv = volume_enclosure(&body).0 - volume_enclosure(&out.body).0;
                        let ov = match verb {
                            Verb::Chamfer => D.powi(3) / 3.0,
                            Verb::Fillet => (5.0 / 3.0 - core::f64::consts::PI / 2.0) * D.powi(3),
                        };
                        let want = verb.section() * 3.5 - ov;
                        eprintln!("PROBE {what} {verb:?}: dV {dv} want {want}");
                        assert!(
                            (dv - want).abs() < 1e-8,
                            "{what} {verb:?}: ΔV {dv} vs {want}"
                        );
                    }
                    Err(e) => typed(&what, verb, &e),
                }
            }
        }
    }
}

/// **An oblique concave turn**: a block less a parallelepiped raised
/// through its top, the pocket's floor corner `(0, 0, 0)` an isosceles
/// concave turn about the leaning lateral edge.
#[test]
fn an_oblique_concave_turn_in_a_sheared_pocket() {
    for s in [0.3, -0.3] {
        // The floor at z = 0.5, the top clearing the block.
        let lifted = parallelepiped_at(s, Point3::new(1.0, 1.0, 0.5), 1.2);
        let blk = sweep::test_support::brick((0.0, 4.0), (0.0, 4.0), (0.0, 1.2), tol());
        let body = cut("sheared pocket", &blk, &lifted);
        let v = [1.0, 1.0, 0.5];
        let e: Vec<EdgeKey> = vec![
            edge(&body, v, [3.0, 1.0, 0.5]),
            edge(&body, v, [1.0, 2.5, 0.5]),
        ];
        let all = vec![
            e[0],
            e[1],
            edge(&body, [3.0, 1.0, 0.5], [3.0, 2.5, 0.5]),
            edge(&body, [3.0, 2.5, 0.5], [1.0, 2.5, 0.5]),
        ];
        for verb in [Verb::Chamfer, Verb::Fillet] {
            for (label, req) in [("corner", &e), ("floor rim", &all)] {
                let what = format!("sheared pocket s={s} {label}");
                let r = verb.run(&body, req);
                report(&what, verb, &r);
                match r {
                    Ok(out) => {
                        check_sound(&what, verb, &out);
                        let dv = volume_enclosure(&body).0 - volume_enclosure(&out.body).0;
                        let (n_arcs, stray) = crate::band_planar_cut_off::arc_residual(&out);
                        eprintln!("PROBE {what} {verb:?}: dV {dv} arcs {n_arcs} stray {stray:e}");
                        assert!(dv < 0.0 && stray < 1e-12, "{what} {verb:?}");
                        if let (Verb::Chamfer, "corner") = (verb, label) {
                            // The pocket is convex: its chamfered self is
                            // it cut by the two chamfer half-spaces.
                            let n = (1.0 + s * s).sqrt();
                            let p = Point3::new(1.0, 1.0, 0.5);
                            let down = Vec3::new(0.0, 0.0, -1.0);
                            let kx = chamfer_keep(
                                p,
                                Vec3::new(0.0, 1.0, 0.0),
                                down,
                                Vec3::new(0.0, -1.0, s) / n,
                            );
                            let ky = chamfer_keep(
                                p,
                                Vec3::new(1.0, 0.0, 0.0),
                                down,
                                Vec3::new(-1.0, 0.0, s) / n,
                            );
                            let lifted = parallelepiped_at(s, Point3::new(1.0, 1.0, 0.5), 1.2);
                            let shrunk = realized(
                                BooleanOp::Intersect,
                                &realized(BooleanOp::Intersect, &lifted, &kx, tol()),
                                &ky,
                                tol(),
                            );
                            let shrunk_body = cut("shrunk pocket", &blk, &shrunk);
                            let want = volume_enclosure(&body).0 - volume_enclosure(&shrunk_body).0;
                            eprintln!("PROBE {what}: concave chamfer dV {dv} oracle {want}");
                            assert!((dv - want).abs() < 1e-9, "{what}: {dv} vs {want}");
                        }
                    }
                    Err(e) => typed(&what, verb, &e),
                }
            }
        }
    }
}

/// **A void behind the turn's station, beside the foot**: at the
/// parallelepiped's corner `(0.3, 0.3, 1)` the face angles are obtuse,
/// so each band's material near the foot on `L` lies at stations
/// behind `v` along its own edge; only the reach window's pad covers
/// it. A sliver void there must refuse.
#[test]
fn a_void_behind_an_oblique_turns_station() {
    let s = 0.3;
    let clean = parallelepiped(s);
    let v = [s, s, 1.0];
    let body = cut(
        "sliver void",
        &clean,
        &brick(
            Point3::new(0.286, 0.279, 0.9),
            Point3::new(0.298, 0.284, 0.925),
        ),
    );
    let turn = [
        edge(&body, v, [2.0 + s, s, 1.0]),
        edge(&body, v, [s, 1.5 + s, 1.0]),
    ];
    let base = Verb::Chamfer
        .run(
            &clean,
            &[
                edge(&clean, v, [2.0 + s, s, 1.0]),
                edge(&clean, v, [s, 1.5 + s, 1.0]),
            ],
        )
        .unwrap();
    let dv0 = volume_enclosure(&clean).0 - volume_enclosure(&base.body).0;
    let r = Verb::Chamfer.run(&body, &turn);
    report("sliver void behind v", Verb::Chamfer, &r);
    match r {
        Ok(out) => {
            let dv = volume_enclosure(&body).0 - volume_enclosure(&out.body).0;
            panic!(
                "sliver void: built; ΔV {dv} vs {dv0}; tier 3 {:?}",
                validate_geometric(&out.body, tol())
            );
        }
        Err(e) => typed("sliver void", Verb::Chamfer, &e),
    }
}

/// **Sampled membership**: the box's top rim chamfered (four mitres)
/// against the analytic solid `box ∩ {u + (1 − z) ≥ d}` per top edge,
/// `u` the distance in from that edge's wall, over a lattice of points
/// near the top.
#[test]
fn a_mitred_rim_agrees_with_the_analytic_solid_pointwise() {
    use topo::boolean::SolidContainment;
    let body = block(2.0, 1.5, 1.0, tol());
    let rim = [
        edge(&body, [0.0, 0.0, 1.0], [2.0, 0.0, 1.0]),
        edge(&body, [2.0, 0.0, 1.0], [2.0, 1.5, 1.0]),
        edge(&body, [2.0, 1.5, 1.0], [0.0, 1.5, 1.0]),
        edge(&body, [0.0, 1.5, 1.0], [0.0, 0.0, 1.0]),
    ];
    let out = Verb::Chamfer.run(&body, &rim).expect("builds");
    let band = geom_core::Band::linear(tol()).unwrap();
    let (mut checked, mut wrong) = (0, 0);
    let n = 23;
    for i in 0..n {
        for j in 0..n {
            for k in 0..9 {
                // Points near the corners and rims, off every face.
                let x = -0.013 + 2.026 * f64::from(i) / f64::from(n - 1);
                let y = -0.011 + 1.522 * f64::from(j) / f64::from(n - 1);
                let x = if i % 2 == 0 { x * 0.08 } else { x };
                let y = if j % 2 == 0 { y * 0.08 } else { y };
                let z = 0.861 + 0.152 * f64::from(k) / 8.0;
                let u = x.min(2.0 - x).min(y).min(1.5 - y);
                let inside_box = (0.0..=2.0).contains(&x) && (0.0..=1.5).contains(&y) && z <= 1.0;
                let keep = [x, 2.0 - x, y, 1.5 - y].iter().all(|w| w + (1.0 - z) >= D);
                let want = inside_box && keep;
                let margin = [
                    u.abs(),
                    (1.0 - z).abs(),
                    ([x, 2.0 - x, y, 1.5 - y]
                        .iter()
                        .map(|w| (w + 1.0 - z - D).abs())
                        .fold(f64::MAX, f64::min)),
                ]
                .into_iter()
                .fold(f64::MAX, f64::min);
                if margin < 1e-6 {
                    continue;
                }
                let got =
                    topo::boolean::point_in_solid(&out.body, Point3::new(x, y, z), band, tol())
                        .expect("membership reads");
                checked += 1;
                let got_in = matches!(got, SolidContainment::In);
                if got_in != want || matches!(got, SolidContainment::OnBoundary) {
                    wrong += 1;
                    eprintln!("PROBE membership ({x}, {y}, {z}): got {got:?}, want in={want}");
                }
            }
        }
    }
    eprintln!("PROBE membership: {checked} points, {wrong} wrong");
    assert_eq!(wrong, 0);
}

/// **Sharp and flat cap corners**: extruded polygons with a 20° and a
/// 160° (and a 172°) interior angle, every top-rim corner a turn; the
/// pentagon row's closed form `P·d²/2 − K·d³/3` /
/// `P·(1 − π/4)·r² − K·(5/3 − π/2)·r³`.
#[test]
fn sharp_and_flat_cap_corners_mitre_at_the_closed_form() {
    use core::f64::consts::PI;
    let a20 = 20f64.to_radians();
    let polys: Vec<Vec<(f64, f64)>> = vec![
        vec![(0.0, 0.0), (3.0, 0.0), (2.8, 2.8 * a20.tan())],
        vec![
            (0.0, 0.0),
            (1.5, -0.2645),
            (3.0, 0.0),
            (3.0, 1.5),
            (0.0, 1.5),
        ],
        vec![
            (0.0, 0.0),
            (1.5, -0.105),
            (3.0, 0.0),
            (3.0, 1.5),
            (0.0, 1.5),
        ],
    ];
    for poly in polys {
        let n = poly.len();
        let c = |i: usize| Vec3::new(poly[i].0, poly[i].1, 0.0);
        let (mut per, mut k) = (0.0, 0.0);
        for i in 0..n {
            let (p, h, q) = (c((i + n - 1) % n), c(i), c((i + 1) % n));
            per += (q - h).norm();
            let th = (p - h).normalize().dot((q - h).normalize()).acos();
            eprintln!("PROBE poly {n}: angle {}", th.to_degrees());
            k += 1.0 / (th / 2.0).tan();
        }
        let body = sweep::test_support::prism(
            poly.iter()
                .map(|&(x, y)| (Point2::new(x, y), 0.0))
                .collect(),
            1.0,
            tol(),
        );
        let rim: Vec<EdgeKey> = (0..n)
            .map(|i| {
                let (a, b) = (poly[i], poly[(i + 1) % n]);
                edge(&body, [a.0, a.1, 1.0], [b.0, b.1, 1.0])
            })
            .collect();
        for verb in [Verb::Chamfer, Verb::Fillet] {
            let want = match verb {
                Verb::Chamfer => per * D * D / 2.0 - k * D.powi(3) / 3.0,
                Verb::Fillet => per * verb.section() - k * (5.0 / 3.0 - PI / 2.0) * D.powi(3),
            };
            let what = format!("cap poly {n}");
            let r = verb.run(&body, &rim);
            report(&what, verb, &r);
            match r {
                Ok(out) => {
                    check_sound(&what, verb, &out);
                    let dv = volume_enclosure(&body).0 - volume_enclosure(&out.body).0;
                    let (_, stray) = crate::band_planar_cut_off::arc_residual(&out);
                    let pad = volume_enclosure(&out.body).1;
                    eprintln!(
                        "PROBE {what} {verb:?}: dV {dv} want {want} diff {:e} pad {pad:e} stray {stray:e}",
                        dv - want
                    );
                    assert!((dv - want).abs() < 1e-8 && stray < 1e-12, "{what} {verb:?}");
                }
                Err(e) => typed(&what, verb, &e),
            }
        }
    }
}
