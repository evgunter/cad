//! Review probes for PR 4797 (predicate 2 reads three ways).

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::{Band, Interval, Point3, Tol, Vec3};
use sweep::blend::{BlendError, BlendRequest, chamfer_edges, fillet_edges};
use sweep::test_support::band_reach;
use topo::{AtRestBody, Body, EdgeKey, query};

use crate::common::interval::iv;

fn tol() -> Tol {
    Tol::witness()
}

fn band() -> Band {
    Band::linear(tol()).expect("the witness band")
}

const R: f64 = 0.5;

fn block_less<T>(void: Body<T>) -> AtRestBody<T>
where
    T: geom_core::Decide + geom_core::Bounds + topo::AtRestPolicy,
{
    let t = tol();
    let block = sweep::test_support::finished(
        "block",
        sweep::test_support::brick::<T>((0.0, 4.0), (0.0, 4.0), (0.0, 4.0), t),
        t,
    );
    let void = sweep::test_support::finished("void", void, t);
    topo::subtract(&block, &void, t)
        .expect("the cut succeeds")
        .body()
        .expect("material")
        .body
        .clone()
}

fn box_void<T>(x: (f64, f64), y: (f64, f64), z: (f64, f64)) -> AtRestBody<T>
where
    T: geom_core::Decide + geom_core::Bounds + topo::AtRestPolicy,
{
    block_less(sweep::test_support::brick::<T>(x, y, z, tol()))
}

fn the_edge<T: geom_core::Bounds>(body: &Body<T>) -> EdgeKey {
    let near = |p: &Point3<T>, q: [f64; 3]| {
        [p.x, p.y, p.z]
            .iter()
            .zip(q)
            .all(|(c, w)| c.lo() <= w + 1e-12 && w - 1e-12 <= c.hi())
    };
    let (a, b) = ([0.0, 0.0, 0.0], [4.0, 0.0, 0.0]);
    query::all_edges(body)
        .into_iter()
        .find(|&e| {
            let he = body.get_edge(e).unwrap().he_plus;
            let p = |v| *body.get_point(body.get_vertex(v).unwrap().point).unwrap();
            let (s, t) = (
                p(body.get_half_edge(he).unwrap().start),
                p(body.half_edge_end(he).unwrap()),
            );
            (near(&s, a) && near(&t, b)) || (near(&s, b) && near(&t, a))
        })
        .expect("the block's edge y = z = 0")
}

fn fillet(body: &AtRestBody<f64>, size: f64) -> Result<(), BlendError> {
    fillet_edges(body, &[the_edge(body)], size, tol())
        .map(|_| ())
        .map_err(|e| e.error)
}

fn chamfer(body: &AtRestBody<f64>, size: f64) -> Result<(), BlendError> {
    chamfer_edges(body, &[the_edge(body)], size, tol())
        .map(|_| ())
        .map_err(|e| e.error)
}

fn kind(e: &Result<(), BlendError>) -> String {
    match e {
        Ok(()) => "built".into(),
        Err(BlendError::WhollyInside { margin, .. }) => format!("WhollyInside({margin})"),
        Err(BlendError::FaceClearance {
            bounded, margin, ..
        }) => {
            format!("FaceClearance(bounded={bounded}, {margin})")
        }
        Err(BlendError::RingClearance {
            bounded, margin, ..
        }) => {
            format!("RingClearance(bounded={bounded}, {margin})")
        }
        Err(other) => format!("other: {other:?}"),
    }
}

#[test]
fn probe_print_escalate() {
    eprintln!("escalate = {:e}", band().escalate());
}

/// A box void that CROSSES the fillet's arc, two of whose faces lie wholly
/// inside the removed material: `y ∈ [0.03, 0.3]`, `z ∈ [0.03, 0.08]`.
/// Corner (0.3, 0.03) is removed (|·−(.5,.5)| = 0.511), corner (0.3, 0.08)
/// is kept (0.465). The FEATURE crosses; a WhollyInside verdict would tell
/// the author it "goes with that material", which is false.
#[test]
fn p1_a_crossing_void_with_faces_wholly_inside_is_not_read_wholly_inside() {
    let body = box_void::<f64>((1.9, 2.1), (0.03, 0.3), (0.03, 0.08));
    let f = fillet(&body, R);
    eprintln!("p1 fillet: {}", kind(&f));
    // Chamfer: removed is y + z <= 0.5; (0.3, 0.08) is 0.38 -> all removed,
    // so under the chamfer this void IS wholly inside.
    let c = chamfer(&body, R);
    eprintln!("p1 chamfer: {}", kind(&c));
    assert!(
        !matches!(f, Err(BlendError::WhollyInside { .. })),
        "the void crosses the arc; it must not be read WhollyInside: {}",
        kind(&f)
    );
}

/// The same crossing void, mirrored so the faces visit in another order.
#[test]
fn p1b_mirrored_crossing_void() {
    let body = box_void::<f64>((1.9, 2.1), (0.03, 0.08), (0.03, 0.3));
    let f = fillet(&body, R);
    eprintln!("p1b fillet: {}", kind(&f));
    assert!(
        !matches!(f, Err(BlendError::WhollyInside { .. })),
        "the void crosses the arc; it must not be read WhollyInside: {}",
        kind(&f)
    );
}

/// A through-slot along the edge, its walls meeting the window's end planes.
#[test]
fn p2_a_through_slot_inside_the_band_is_a_bound_not_inside() {
    let body = box_void::<f64>((-1.0, 5.0), (0.03, 0.08), (0.03, 0.08));
    for (v, e) in [("fillet", fillet(&body, R)), ("chamfer", chamfer(&body, R))] {
        eprintln!("p2 {v}: {}", kind(&e));
        // The slot's two rings lie in each cap's sliver and its walls in
        // the band's prism: the feature IS wholly inside. The fillet's
        // round end reads the ring at the cap; the chamfer's chord end
        // has no sliver boundary and reads a bound.
        assert!(e.is_err(), "{v}: never builds");
        if v == "fillet" {
            assert!(
                matches!(
                    e,
                    Err(BlendError::WhollyInside {
                        at: topo::EntityId::Loop(_),
                        ..
                    })
                ),
                "{v}: {}",
                kind(&e)
            );
        }
    }
}

/// A void whose corner sits exactly on the fillet's arc and the chamfer's
/// chord (touching the region's boundary): never WhollyInside, never built.
#[test]
fn p3_a_void_touching_the_boundary_is_never_inside() {
    let s2 = std::f64::consts::SQRT_2;
    for (v, hi) in [("fillet", R - R / s2), ("chamfer", 0.25)] {
        let body = box_void::<f64>((1.9, 2.1), (0.03, hi), (0.03, hi));
        let e = if v == "fillet" {
            fillet(&body, R)
        } else {
            chamfer(&body, R)
        };
        eprintln!("p3 {v}: {}", kind(&e));
        assert!(
            !matches!(e, Err(BlendError::WhollyInside { .. }) | Ok(())),
            "{v}: touching the boundary: {}",
            kind(&e)
        );
    }
}

/// Voids at k·escalate either side of the arc / chord.
#[test]
fn p3b_escalate_scale_either_side() {
    let s2 = std::f64::consts::SQRT_2;
    let esc = band().escalate();
    for k in [-4.0, -1.5, -1.0, -0.5, 0.5, 1.0, 1.5, 4.0] {
        let gap = k * esc;
        for (v, hi) in [
            ("fillet", R - (R + gap) / s2),
            ("chamfer", 0.5 * (R - gap * s2)),
        ] {
            let body = box_void::<f64>((1.9, 2.1), (0.03, hi), (0.03, hi));
            let e = if v == "fillet" {
                fillet(&body, R)
            } else {
                chamfer(&body, R)
            };
            eprintln!("p3b {v} k={k}: {}", kind(&e));
            if k < 0.0 {
                assert!(
                    !matches!(e, Err(BlendError::WhollyInside { .. }) | Ok(())),
                    "{v} k={k}: past the boundary: {}",
                    kind(&e)
                );
            } else {
                assert!(e.is_err(), "{v} k={k}: never builds");
            }
        }
    }
}

/// A spherical void (a curved face) wholly inside the removed material,
/// and one centred on the arc.
#[test]
fn p4_a_spherical_void_inside_and_across() {
    for (what, c, r) in [
        ("inside", (0.07, 0.07), 0.02),
        (
            "on the arc",
            (
                R - R / std::f64::consts::SQRT_2,
                R - R / std::f64::consts::SQRT_2,
            ),
            0.02,
        ),
    ] {
        let ball = sweep::test_support::ball_poled_z(r, Vec3::new(2.0, c.0, c.1), tol());
        let body = block_less(ball);
        for (v, e) in [("fillet", fillet(&body, R)), ("chamfer", chamfer(&body, R))] {
            eprintln!("p4 {what} {v}: {}", kind(&e));
            assert!(e.is_err(), "{what} {v}: never builds");
            if what == "on the arc" && v == "fillet" {
                assert!(
                    !matches!(e, Err(BlendError::WhollyInside { .. })),
                    "{what} {v}: {}",
                    kind(&e)
                );
            }
        }
    }
}

/// Interval vs f64 at the hair cases.
#[test]
fn p5_interval_brackets_f64_near_the_boundary() {
    let s2 = std::f64::consts::SQRT_2;
    let esc = band().escalate();
    for k in [-1.0, 1.0, 2.0, 10.0] {
        let hi = R - (R + k * esc) / s2;
        let f = box_void::<f64>((1.9, 2.1), (0.03, hi), (0.03, hi));
        let i = box_void::<Interval>((1.9, 2.1), (0.03, hi), (0.03, hi));
        let ef = band_reach(
            &BlendRequest {
                body: &f,
                edges: vec![the_edge(&f)],
                size: R,
            },
            band(),
        );
        let ei = band_reach(
            &BlendRequest {
                body: &i,
                edges: vec![the_edge(&i)],
                size: iv(R),
            },
            band(),
        );
        let ef = ef.map(|_| ());
        let ei = ei.map(|_| ());
        eprintln!("p5 k={k}: f64 {} | iv {}", kind(&ef), kind(&ei));
        if matches!(ei, Err(BlendError::WhollyInside { .. })) {
            assert!(
                matches!(ef, Err(BlendError::WhollyInside { .. })),
                "Interval says inside where f64 does not"
            );
        }
    }
}

/// An L-shaped void: a lobe wholly inside the fillet's removed material
/// (`y, z ∈ [0.03, 0.08]`) joined to a lobe that rises through the arc into
/// the kept material (`z` up to `0.4`). The FEATURE crosses; three of its
/// faces lie wholly inside. Several placements along the edge, so the
/// face-box tree hands the faces over in different orders.
#[test]
fn p6_an_l_void_crossing_the_arc_is_never_read_wholly_inside() {
    let t = tol();
    let mut seen = Vec::new();
    for (x0, flip) in [0.5, 1.0, 1.3, 1.9, 2.4, 2.9, 3.3]
        .into_iter()
        .flat_map(|x| [(x, false), (x, true)])
    {
        let bx = if flip {
            (x0, x0 + 0.1)
        } else {
            (x0 + 0.1, x0 + 0.2)
        };
        let a = sweep::test_support::finished(
            "a",
            sweep::test_support::brick::<f64>((x0, x0 + 0.2), (0.03, 0.08), (0.03, 0.08), t),
            t,
        );
        let b = sweep::test_support::finished(
            "b",
            sweep::test_support::brick::<f64>(bx, (0.03, 0.08), (0.08, 0.4), t),
            t,
        );
        let l = if flip {
            topo::union(&b, &a, t)
        } else {
            topo::union(&a, &b, t)
        }
        .expect("the union")
        .body()
        .expect("material")
        .body
        .clone();
        let body = block_less((*l).clone());
        let e = fillet(&body, R);
        eprintln!("p6 x0={x0} flip={flip}: {}", kind(&e));
        seen.push((x0, flip, kind(&e)));
    }
    let bad: Vec<_> = seen
        .iter()
        .filter(|(_, _, k)| k.starts_with("WhollyInside"))
        .collect();
    assert!(
        bad.is_empty(),
        "a crossing feature read WhollyInside: {bad:?}"
    );
}

/// A through-slot that enters the cap `x = 0` wholly inside its sliver
/// (`y, z ∈ [0.03, 0.08]`) and climbs through the fillet's arc to leave
/// the cap `x = 4` in the kept material (`z ≈ 0.33..0.38`): its walls
/// CROSS the band's material. The cap meter reads the entry ring first.
#[test]
fn p7_a_tilted_through_slot_crossing_the_band_is_not_read_wholly_inside() {
    let t = tol();
    let theta = (0.3f64 / 4.0).atan();
    let slot = sweep::test_support::brick::<f64>((-1.0, 5.0), (0.03, 0.08), (0.03, 0.08), t);
    let slot = topo::transform_rigid(
        &slot,
        &geom_core::Affine3::rotation_about_axis(
            Point3::new(0.0, 0.0, 0.055),
            Vec3::new(0.0, 1.0, 0.0),
            -theta,
        ),
        t,
    )
    .expect("the rigid motion");
    let body = block_less(slot);
    let e = fillet(&body, R);
    eprintln!("p7 fillet: {}", kind(&e));
    assert!(e.is_err(), "never builds");
    assert!(
        !matches!(e, Err(BlendError::WhollyInside { .. })),
        "the slot's walls cross the band: {}",
        kind(&e)
    );
}

/// A tube (outer radius 1, bore 0.3, height 1, poled along `y`) whose
/// outer top rim is ONE closed edge: a circular link over a plane and a
/// cylinder, whose reach's core is exact. A spherical void of radius 0.01
/// at `(0.97, 0.97, 0)` lies `0.02` inside both supports and `0.37` from
/// the tube's centre circle, so wholly inside the fillet's removed ring
/// (radius 0.3); one at `(0.88, 0.88, 0)` (0.255 from the centre circle)
/// crosses nothing and lies in the kept material, and one centred on the
/// arc crosses it.
#[test]
fn p8_a_void_in_a_circular_rims_band() {
    use sweep::Revolution;
    use sweep::test_support::{ball_poled_y, corners, one_edge_rim_at, revolved_about_y};
    let t = tol();
    let tube = revolved_about_y(
        corners(&[(0.3, 0.0), (1.0, 0.0), (1.0, 1.0), (0.3, 1.0)]),
        Revolution::Full,
        t,
    );
    let tube = sweep::test_support::finished("tube", tube, t);
    let s2 = std::f64::consts::SQRT_2;
    let on_arc = 0.7 + 0.3 / s2;
    for (what, c) in [("inside", 0.97), ("kept", 0.88), ("on the arc", on_arc)] {
        let ball = sweep::test_support::finished(
            "ball",
            ball_poled_y::<f64>(0.01, Vec3::new(c, c, 0.0), t),
            t,
        );
        let body = topo::subtract(&tube, &ball, t)
            .expect("the cut")
            .body()
            .expect("material")
            .body
            .clone();
        let rim = one_edge_rim_at(&body, 1.0, 1.0);
        {
            let size = 0.3;
            let e = fillet_edges(&body, &[rim], size, t)
                .map(|_| ())
                .map_err(|e| e.error);
            eprintln!("p8 {what} fillet: {}", kind(&e));
            let e2 = chamfer_edges(&body, &[rim], size, t)
                .map(|_| ())
                .map_err(|e| e.error);
            eprintln!("p8 {what} chamfer: {}", kind(&e2));
            if what == "on the arc" {
                assert!(
                    !matches!(e, Err(BlendError::WhollyInside { .. }) | Ok(())),
                    "{what}: {}",
                    kind(&e)
                );
            }
            if what == "inside" {
                assert!(e.is_err() && e2.is_err(), "{what}: never builds");
            }
        }
    }
}

/// A blind hole from the cap `x = 0` in the KEPT material between the
/// fillet's chord and its arc (`y, z ∈ [0.2, 0.22]`, inside the ball's
/// section): the cap meter clears it, and the blend builds at the closed
/// form `ΔV = −4·section`. (Mutating the cap's crossing to need only one
/// witness, `bounded: !reaches_in && !reaches_out`, leaves every row green:
/// no row has a cap cycle with exactly one witness.)
#[test]
fn p9_a_hole_in_the_kept_material_between_chord_and_arc_builds() {
    let body = box_void::<f64>((-1.0, 1.0), (0.2, 0.22), (0.2, 0.22));
    let out = fillet_edges(&body, &[the_edge(&body)], R, tol())
        .unwrap_or_else(|e| panic!("the kept hole builds, got {:?}", e.error));
    topo::validate_geometric(&out.body, tol()).expect("tier 3");
    let v = topo::mass_properties(&out.body, tol())
        .expect("a volume")
        .volume;
    let want = 64.0 - 0.02 * 0.02 - 4.0 * R * R * (1.0 - std::f64::consts::FRAC_PI_4);
    assert!((v - want).abs() < 1e-9, "V {v}, want {want}");
}
