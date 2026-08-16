//! S11 blinded-review adversarial probes, adopted verbatim in the fix
//! pass (coordinator F4): six constructions attacking the concavity
//! criterion's orientation conventions (mixed hole arcs, the S2
//! fillet-cornered eye slot as outer AND hole, asymmetric downward
//! invariance per carrier, reversed authoring, a bore-groove torus
//! band), plus the D1 guard: a TOUCHING union against a reversed-face
//! body must refuse typed until boolean splitting inherits the parent
//! face's sense (see `Body::set_face_sense` docs — the mef re-mint
//! hazard). When curved unions start answering, D1 fails loudly and
//! the inheritance fix becomes due.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

mod revolve_common;

use core::f64::consts::PI;
use profile::RawLoop;

use geom_core::{Band, Point3, Tolerance};
use geom_surfaces::Surface;
use profile::{
    ArcSide, ArcSweep, Center, Open, Profile, ProfileLoop, ProfileVertex, Radius, SketchPlane,
    Start,
};
use revolve_common::{assert_all_tiers, axis_y, p2, validated};
use sweep::{Extrusion, Revolution, extrude, revolve};
use topo::boolean::{SolidContainment, point_in_solid};
use topo::{Body, FaceKey};

fn band() -> Band {
    Band::linear().unwrap()
}

fn vol(body: &Body<f64>) -> f64 {
    topo::props::mass_properties(body).unwrap().volume
}

fn sense_of(body: &Body<f64>, f: FaceKey) -> bool {
    body.get_face(f).unwrap().sense
}

/// A1: a hole whose boundary has BOTH a concave (bite) and a convex
/// (material tab) arc relative to the hole, plus two straight walls.
/// Authored CCW (canonicalization must flip it): bottom arc bulges
/// away from the hole (bite into material -> wall sense false), top
/// arc bulges into the hole (material tab -> wall sense true).
#[test]
fn adv_mixed_convex_concave_hole() {
    let outer = ProfileLoop::polygon([p2(0.0, 0.0), p2(6.0, 0.0), p2(6.0, 6.0), p2(0.0, 6.0)]);
    let hole = <ProfileLoop<f64> as RawLoop<f64>>::new(vec![
        ProfileVertex {
            pos: p2(2.0, 2.0),
            bulge: 0.5,
        },
        ProfileVertex {
            pos: p2(4.0, 2.0),
            bulge: 0.0,
        },
        ProfileVertex {
            pos: p2(4.0, 4.0),
            bulge: -0.5,
        },
        ProfileVertex {
            pos: p2(2.0, 4.0),
            bulge: 0.0,
        },
    ]);
    let vp = Profile::new(SketchPlane::xy(), vec![outer, hole])
        .validate(Tolerance::get())
        .unwrap();
    let t = extrude(&vp, Extrusion::Distance(1.0)).unwrap();
    assert_all_tiers(&t.body);
    assert_eq!(topo::validate::validate_geometric(&t.body), Ok(()));
    // Equal-sagitta segments cancel: hole area exactly 4, volume 32.
    assert!((vol(&t.body) - 32.0).abs() < 1e-9, "vol {}", vol(&t.body));
    let mut seen = (0, 0, 0); // (false-cyl, true-cyl, true-plane)
    for &f in &t.side_faces[1] {
        let sk = t.body.get_face(f).unwrap().surface;
        match *t.body.get_surface(sk).unwrap() {
            Surface::Cylinder { origin, .. } => {
                if origin.y < 3.0 {
                    assert!(!sense_of(&t.body, f), "bite arc (center y<3) must be false");
                    seen.0 += 1;
                } else {
                    assert!(sense_of(&t.body, f), "tab arc (center y>3) must be true");
                    seen.1 += 1;
                }
            }
            _ => {
                assert!(sense_of(&t.body, f), "straight hole walls stay true");
                seen.2 += 1;
            }
        }
    }
    assert_eq!(seen, (1, 1, 2), "one bite, one tab, two planes");
    let b = band();
    for (p, want) in [
        (Point3::new(3.0, 3.0, 0.5), SolidContainment::Out), // hole center
        (Point3::new(3.0, 3.75, 0.5), SolidContainment::In), // tab material
        (Point3::new(3.0, 1.7, 0.5), SolidContainment::Out), // bite void
        (Point3::new(3.0, 1.2, 0.5), SolidContainment::In),  // below bite
        (Point3::new(5.5, 3.0, 0.5), SolidContainment::In),
    ] {
        assert_eq!(point_in_solid(&t.body, p, b).unwrap(), want, "probe {p:?}");
    }
}

fn s3() -> f64 {
    3.0_f64.sqrt()
}

/// The S2 vesica eye-slot: the radius-2 vesica about (+-1, 0), both
/// tips rounded, authored as ONE chain of fused arc-fillet verbs.
///
/// The entry rides the left circle at (1, 0); the first fused verb
/// fillets the (0, s3) tip and arrives on the right circle at
/// (-1, 0); from that on-carrier tip the second re-authors the same
/// carrier as `Radius { r: 2, side: Left }` — its centre derived from
/// the tip's own position and tangent, which reproduces (1, 0)
/// exactly — fillets the (0, -s3) tip and closes on the left circle.
/// Each carrier run is emitted by the verb that trims it, so the
/// mid-arc points at (+-1, 0) are anchors, not vertices.
fn eye_slot(radius: f64) -> ProfileLoop<f64> {
    Open.arc_fillet_arc(
        Center {
            c: p2(-1.0, 0.0),
            winding: ArcSweep::Ccw,
            p: p2(1.0, 0.0),
        },
        0.3,
        Center {
            c: p2(1.0, 0.0),
            winding: ArcSweep::Ccw,
            p: p2(-1.0, 0.0),
        },
    )
    .unwrap()
    .arc_fillet_arc(
        Radius {
            r: 2.0,
            side: ArcSide::Left,
        },
        radius,
        Center {
            c: p2(-1.0, 0.0),
            winding: ArcSweep::Ccw,
            p: Start,
        },
    )
    .unwrap()
    .loop_
}

/// A2: extrude the eye slot as an OUTER region -> every wall convex,
/// all senses true; as a HOLE in a plate -> every wall concave to the
/// plate material, all senses false (fillet arcs included: their
/// carriers hold void, material outside).
#[test]
fn adv_eye_slot_outer_and_hole_senses() {
    // Outer rocker.
    let vp = Profile::new(SketchPlane::xy(), vec![eye_slot(0.3)])
        .validate(Tolerance::get())
        .unwrap();
    let t = extrude(&vp, Extrusion::Distance(1.0)).unwrap();
    assert_all_tiers(&t.body);
    assert_eq!(topo::validate::validate_geometric(&t.body), Ok(()));
    for &f in &t.side_faces[0] {
        assert!(sense_of(&t.body, f), "outer vesica walls are all convex");
    }
    let v_outer = vol(&t.body);

    // Same loop as a hole in a 6x6 plate.
    let outer = ProfileLoop::polygon([p2(-3.0, -3.0), p2(3.0, -3.0), p2(3.0, 3.0), p2(-3.0, 3.0)]);
    let vp = Profile::new(SketchPlane::xy(), vec![outer, eye_slot(0.3)])
        .validate(Tolerance::get())
        .unwrap();
    let t = extrude(&vp, Extrusion::Distance(1.0)).unwrap();
    assert_all_tiers(&t.body);
    assert_eq!(topo::validate::validate_geometric(&t.body), Ok(()));
    for &f in &t.side_faces[1] {
        assert!(!sense_of(&t.body, f), "every eye-slot hole wall is concave");
    }
    assert!(
        (vol(&t.body) - (36.0 - v_outer)).abs() < 1e-9,
        "hole removes exactly the rocker volume: {} vs {}",
        vol(&t.body),
        36.0 - v_outer
    );
    let b = band();
    assert_eq!(
        point_in_solid(&t.body, Point3::new(0.0, 0.0, 0.5), b).unwrap(),
        SolidContainment::Out,
        "slot interior is void"
    );
    assert_eq!(
        point_in_solid(&t.body, Point3::new(2.5, 2.5, 0.5), b).unwrap(),
        SolidContainment::In
    );
}

/// A3: downward extrusion of an ASYMMETRIC profile - senses must match
/// the upward extrusion carrier-by-carrier (not merely in count).
#[test]
fn adv_asymmetric_downward_invariance() {
    let mk = || {
        <ProfileLoop<f64> as RawLoop<f64>>::new(vec![
            ProfileVertex {
                pos: p2(0.0, 0.0),
                bulge: 0.0,
            },
            // concave bite on the right edge
            ProfileVertex {
                pos: p2(3.0, 0.0),
                bulge: -0.4,
            },
            ProfileVertex {
                pos: p2(3.0, 1.0),
                bulge: 0.0,
            },
            // convex bulge on top, off-center
            ProfileVertex {
                pos: p2(3.0, 2.0),
                bulge: 0.7,
            },
            ProfileVertex {
                pos: p2(1.0, 2.0),
                bulge: 0.0,
            },
            ProfileVertex {
                pos: p2(0.0, 2.0),
                bulge: 0.0,
            },
        ])
    };
    let up = extrude(
        &Profile::new(SketchPlane::xy(), vec![mk()])
            .validate(Tolerance::get())
            .unwrap(),
        Extrusion::Distance(1.0),
    )
    .unwrap();
    let down = extrude(
        &Profile::new(SketchPlane::xy(), vec![mk()])
            .validate(Tolerance::get())
            .unwrap(),
        Extrusion::Vector(geom_core::Vec3::new(0.0, 0.0, -1.0)),
    )
    .unwrap();
    assert_all_tiers(&up.body);
    assert_all_tiers(&down.body);
    let carriers = |t: &sweep::Extruded<f64>| {
        let mut v: Vec<((i64, i64, i64), bool)> = t.side_faces[0]
            .iter()
            .map(|&f| {
                let sk = t.body.get_face(f).unwrap().surface;
                let key = match *t.body.get_surface(sk).unwrap() {
                    Surface::Cylinder { origin, radius, .. } => (
                        (origin.x * 1e6).round() as i64,
                        (origin.y * 1e6).round() as i64,
                        (radius * 1e6).round() as i64,
                    ),
                    _ => (0, 0, 0),
                };
                (key, sense_of(&t.body, f))
            })
            .collect();
        v.sort();
        v
    };
    assert_eq!(
        carriers(&up),
        carriers(&down),
        "per-carrier senses must be extrusion-direction invariant"
    );
    // And the concave bite really is the false one.
    let falses: Vec<bool> = carriers(&up)
        .iter()
        .filter(|(k, _)| *k != (0, 0, 0))
        .map(|&(_, s)| s)
        .collect();
    assert_eq!(falses.iter().filter(|s| !**s).count(), 1);
}

/// B1: authoring the washer rectangle CLOCKWISE (same geometry) must
/// mint identical senses - canonicalization owns the winding.
#[test]
fn adv_reversed_authoring_revolve_same_senses() {
    let ccw = ProfileLoop::polygon([p2(1.0, 0.0), p2(2.0, 0.0), p2(2.0, 1.0), p2(1.0, 1.0)]);
    let cw = ProfileLoop::polygon([p2(1.0, 0.0), p2(1.0, 1.0), p2(2.0, 1.0), p2(2.0, 0.0)]);
    let by_kind = |lp: ProfileLoop<f64>| {
        let t = revolve(&validated(vec![lp]), axis_y(), Revolution::Full).unwrap();
        let mut m: Vec<(String, bool)> = t.walls[0]
            .iter()
            .flatten()
            .map(|&f| {
                let sk = t.body.get_face(f).unwrap().surface;
                let key = match *t.body.get_surface(sk).unwrap() {
                    Surface::Cylinder { radius, .. } => format!("cyl{radius:.3}"),
                    Surface::Plane { origin, .. } => format!("pl{:.3}", origin.y),
                    _ => "other".into(),
                };
                (key, sense_of(&t.body, f))
            })
            .collect();
        m.sort();
        m.dedup();
        m
    };
    assert_eq!(by_kind(ccw), by_kind(cw));
}

/// B2: a torus band concave toward the axis - a semicircular groove
/// cut into the washer's bore. Turn is clockwise in the canonical
/// traversal -> sense false; exact Pappus volume.
#[test]
fn adv_bore_groove_torus_band() {
    // Only (1, 0.75) leaves on an arc: the semicircular groove cut
    // into the bore.
    let lp = <ProfileLoop<f64> as RawLoop<f64>>::new(vec![
        ProfileVertex {
            pos: p2(1.0, 0.0),
            bulge: 0.0,
        },
        ProfileVertex {
            pos: p2(2.0, 0.0),
            bulge: 0.0,
        },
        ProfileVertex {
            pos: p2(2.0, 1.0),
            bulge: 0.0,
        },
        ProfileVertex {
            pos: p2(1.0, 1.0),
            bulge: 0.0,
        },
        ProfileVertex {
            pos: p2(1.0, 0.75),
            bulge: -1.0,
        },
        ProfileVertex {
            pos: p2(1.0, 0.25),
            bulge: 0.0,
        },
    ]);
    let t = revolve(&validated(vec![lp]), axis_y(), Revolution::Full).unwrap();
    assert_all_tiers(&t.body);
    assert_eq!(topo::validate::validate_geometric(&t.body), Ok(()));
    let expect = 3.0 * PI - (PI * PI / 16.0 + PI / 48.0);
    assert!(
        (vol(&t.body) - expect).abs() < 1e-9,
        "washer minus groove: {expect}, got {}",
        vol(&t.body)
    );
    // bottom annulus F, outer cyl T, top annulus T, bore upper F,
    // groove torus F, bore lower F.
    let senses: Vec<Option<bool>> = t.walls[0]
        .iter()
        .map(|w| w.map(|f| sense_of(&t.body, f)))
        .collect();
    assert_eq!(
        senses,
        vec![
            Some(false),
            Some(true),
            Some(true),
            Some(false),
            Some(false),
            Some(false)
        ]
    );
    // The groove wall really is a torus.
    let groove = t.walls[0][4].unwrap();
    let sk = t.body.get_face(groove).unwrap().surface;
    assert!(matches!(
        t.body.get_surface(sk).unwrap(),
        Surface::Torus { .. }
    ));
}

/// D1: a TOUCHING union against a body that carries reversed faces -
/// must not silently split a reversed face through sense-true mef
/// re-mints.
///
/// **Re-aimed at M5 S12.** When S11 adopted this probe the `mef`
/// re-mints DID stamp `sense: true`, so an answer here would have been
/// a silently mis-oriented body and panicking was the right response.
/// S12 landed the inheritance fix (`mint_loop_and_face` takes the
/// parent's bit whenever the fragment lands on the parent's surface;
/// `mfkrh` likewise), so an answer is no longer prima facie wrong — the
/// row therefore AUDITS an answer instead of rejecting it, and keeps
/// accepting the typed refusal this washer/box pair still takes (its
/// door is the annulus-touching lane, not sense inheritance). The
/// mixed-sense split that S12 genuinely made reachable is pinned with
/// exact volumes in `m5_s12_curved_ops.rs`
/// (`a_boolean_that_splits_a_reversed_wall_inherits_the_parent_bit`).
#[test]
fn adv_touching_union_with_reversed_faces_refuses_typed() {
    let lp = ProfileLoop::polygon([p2(1.0, 0.0), p2(2.0, 0.0), p2(2.0, 1.0), p2(1.0, 1.0)]);
    let washer = revolve(&validated(vec![lp]), axis_y(), Revolution::Full)
        .unwrap()
        .body;
    // A box poking through the bottom annulus near x = 1.5.
    let sq = ProfileLoop::polygon([p2(1.2, -0.5), p2(1.8, -0.5), p2(1.8, 0.5), p2(1.2, 0.5)]);
    let plane = SketchPlane::from_frame(
        Point3::new(0.0, 0.0, 0.0),
        geom_core::Vec3::new(1.0, 0.0, 0.0),
        geom_core::Vec3::new(0.0, 1.0, 0.0),
    );
    let vp = Profile::new(plane, vec![sq])
        .validate(Tolerance::get())
        .unwrap();
    let boxb = extrude(&vp, Extrusion::Distance(0.4)).unwrap().body;
    match topo::boolean::union(&washer, &boxb) {
        Err(e) => println!("typed refusal (expected today): {e}"),
        Ok(r) => {
            let out = r.body().expect("non-empty");
            println!(
                "union ANSWERED: vol {} shells {}",
                vol(&out.body),
                out.body.shells().count()
            );
            // Answering is legal as of S12; being WRONG is not. The
            // washer is the full revolve of a 1x1 square at r in [1, 2]
            // about the y axis, and the box (0.6 x 1.0 x 0.4 at
            // x in [1.2, 1.8]) lies wholly under its bottom annulus,
            // so a touching union is exactly additive.
            let washer_vol = std::f64::consts::PI * (2.0f64.powi(2) - 1.0) * 1.0;
            let expect = washer_vol + 0.6 * 1.0 * 0.4;
            assert!(
                (vol(&out.body) - expect).abs() < 1e-9,
                "touching curved union answered with {} for {expect} - inspect \
                 sense inheritance",
                vol(&out.body)
            );
            assert_eq!(topo::validate_geometric(&out.body), Ok(()));
        }
    }
}
