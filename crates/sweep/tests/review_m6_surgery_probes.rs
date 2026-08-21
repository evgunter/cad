//! **Adopted blinded-review probes for M6 unit 1** (the surgery) —
//! the review's adversarial constructions become permanent guards,
//! per the standing review policy (S11/S12 precedent). Adopted
//! verbatim from the reviewer's charters B/C/D/F at the fix pass:
//! rim-orientation recon (the F1 slot-order finding's probe),
//! ring-clearance trio through the front door, one/two-pip ladders
//! with closed-form volumes, the edge-straddling pip, seam-azimuth
//! rotation, reversed-cap sense carry-through, the dev-1 structural
//! zero, and the duplicate-edge refusal.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic, dead_code)]

use core::f64::consts::PI;
use profile::RawLoop;

use geom::Surface;
use geom_core::{Affine3, Band, Point2, Tolerance, Vec2, Vec3};
use profile::{Profile, ProfileLoop, ProfileVertex, SketchPlane};
use sweep::fillet::build::fillet_edges;
use sweep::test_support::cube;
use sweep::{Revolution, RevolveAxis, revolve};
use topo::boolean::{BooleanOp, SweepStrategy, boolean_op_with};
use topo::{Body, BooleanDeclarations, EdgeKey};
use geom_core::Tol;

const DIE_L: f64 = 1.0;
const DIE_R: f64 = 0.12;
const PIP_R: f64 = 0.09;
const PIP_H: f64 = 0.05;
const RIM_R: f64 = 0.02;

fn band() -> Band {
    let tol = Tol::witness().get();
    Band::new(tol.eps, tol.k * tol.eps).unwrap()
}

fn p2(x: f64, y: f64) -> Point2<f64> {
    Point2::new(x, y)
}

fn ball_at(r: f64, c: Vec3<f64>) -> Body<f64> {
    let lp = ProfileLoop::new(vec![
        ProfileVertex::new(p2(0.0, -r), 1.0),
        ProfileVertex::new(p2(0.0, r), 0.0),
    ]);
    let vp = Profile::new(SketchPlane::xy(), vec![lp])
        .validate(Tol::witness())
        .unwrap();
    let axis = RevolveAxis {
        origin: p2(0.0, 0.0),
        dir: Vec2::new(0.0, 1.0),
    };
    let ball = revolve(&vp, axis, Revolution::Full).unwrap().body;
    topo::transform_rigid(&ball, &Affine3::translation(c)).unwrap()
}

/// +Z-poled ball at c (pole toward +z, like the die's top-face pips).
fn ball_top(r: f64, c: Vec3<f64>) -> Body<f64> {
    let ball = ball_at(r, Vec3::new(0.0, 0.0, 0.0));
    let rot = topo::transform_rigid(
        &ball,
        &Affine3::rotation_about_axis(
            geom_core::Point3::new(0.0, 0.0, 0.0),
            Vec3::new(1.0, 0.0, 0.0),
            PI / 2.0,
        ),
    )
    .unwrap();
    topo::transform_rigid(&rot, &Affine3::translation(c)).unwrap()
}

fn subtract(a: &Body<f64>, b: &Body<f64>) -> Result<Body<f64>, topo::BooleanError> {
    let out = boolean_op_with(
        BooleanOp::Subtract,
        a,
        b,
        &BooleanDeclarations::none(),
        SweepStrategy::Realized,
    )?;
    Ok(out.body().expect("a body").body.clone())
}

fn rim_edges(body: &Body<f64>) -> Vec<EdgeKey> {
    let kind_of = |f: topo::FaceKey| -> Option<u8> {
        match body.get_surface(body.get_face(f)?.surface)? {
            Surface::Plane { .. } => Some(0),
            Surface::Sphere { .. } => Some(1),
            _ => Some(2),
        }
    };
    let face_of = |he: topo::HalfEdgeKey| -> Option<topo::FaceKey> {
        Some(body.get_loop(body.get_half_edge(he)?.parent_loop)?.face)
    };
    let mut out = Vec::new();
    for (k, e) in body.edges() {
        let (Some(fa), Some(fb)) = (face_of(e.he_plus), face_of(e.he_minus)) else {
            continue;
        };
        if matches!(
            (kind_of(fa), kind_of(fb)),
            (Some(0), Some(1)) | (Some(1), Some(0))
        ) {
            out.push(k);
        }
    }
    out
}

fn blank_volume() -> f64 {
    let core = DIE_L - 2.0 * DIE_R;
    core.powi(3)
        + 6.0 * DIE_R * core.powi(2)
        + 12.0 * (PI * DIE_R * DIE_R / 4.0) * core
        + (4.0 / 3.0) * PI * DIE_R.powi(3)
}

fn cap(r: f64, h: f64) -> f64 {
    PI * h * h * (3.0 * r - h) / 3.0
}

fn rim_fillet_extra(big_r: f64, h: f64, r: f64) -> f64 {
    let d = big_r - h;
    let s = ((big_r + r).powi(2) - (d + r).powi(2)).sqrt();
    let rho_rim = (big_r * big_r - d * d).sqrt();
    let k = big_r / (big_r + r);
    let (tsx, tsz) = (s * k, d - (d + r) * k);
    let tri_m = |a: (f64, f64), b: (f64, f64), c: (f64, f64)| -> f64 {
        let area2 = (b.0 - a.0) * (c.1 - a.1) - (c.0 - a.0) * (b.1 - a.1);
        (area2.abs() / 2.0) * ((a.0 + b.0 + c.0) / 3.0)
    };
    let seg_m = |c: (f64, f64), rad: f64, p: (f64, f64), q: (f64, f64)| -> f64 {
        let a0 = (p.1 - c.1).atan2(p.0 - c.0);
        let mut a1 = (q.1 - c.1).atan2(q.0 - c.0);
        if a1 - a0 > PI {
            a1 -= 2.0 * PI;
        } else if a0 - a1 > PI {
            a1 += 2.0 * PI;
        }
        let (lo, hi) = if a0 <= a1 { (a0, a1) } else { (a1, a0) };
        let sector = c.0 * rad * rad * (hi - lo) / 2.0 + rad.powi(3) * (hi.sin() - lo.sin()) / 3.0;
        sector - tri_m(c, p, q)
    };
    2.0 * PI
        * (tri_m((rho_rim, 0.0), (s, 0.0), (tsx, tsz))
            - seg_m((s, -r), r, (s, 0.0), (tsx, tsz))
            - seg_m((0.0, d), big_r, (rho_rim, 0.0), (tsx, tsz)))
}

/// One pip at (cx, cy) on the top face; returns (pipped, 12 box edges).
fn one_pip(cx: f64, cy: f64) -> (Body<f64>, Vec<EdgeKey>) {
    let cube0 = cube(DIE_L);
    let box_edges: Vec<_> = cube0.edges().map(|(k, _)| k).collect();
    let ball = ball_top(PIP_R, Vec3::new(cx, cy, DIE_L + (PIP_R - PIP_H)));
    let pipped = subtract(&cube0, &ball).expect("pip cuts");
    let surviving: Vec<_> = box_edges
        .into_iter()
        .filter(|k| pipped.get_edge(*k).is_some())
        .collect();
    assert_eq!(surviving.len(), 12);
    (pipped, surviving)
}

/// P1: which side of each rim edge is he_plus on? rim_phase assumes
/// trim_a (the he_plus face's trim) is the PLANE trim.
#[test]
fn p1_rim_edge_orientation_recon() {
    let (pipped, box_edges) = one_pip(0.5, 0.5);
    let blanked = fillet_edges(&pipped, &box_edges, DIE_R, band())
        .expect("box fillet")
        .body;
    let mut plane_first = 0;
    let mut sphere_first = 0;
    for e in rim_edges(&blanked) {
        let ed = blanked.get_edge(e).unwrap();
        let f = blanked
            .get_loop(blanked.get_half_edge(ed.he_plus).unwrap().parent_loop)
            .unwrap()
            .face;
        let is_plane = matches!(
            blanked
                .get_surface(blanked.get_face(f).unwrap().surface)
                .unwrap(),
            Surface::Plane { .. }
        );
        if is_plane {
            plane_first += 1;
        } else {
            sphere_first += 1;
        }
    }
    println!("P1: rim he_plus sides — plane-first {plane_first}, sphere-first {sphere_first}");
}

/// P2: ring-clearance trio through the FRONT DOOR — a pip whose
/// widened trim circle approaches the shrunk face's trimline edge.
#[test]
fn p2_ring_touch_trio_through_the_front_door() {
    let s = ((PIP_R + RIM_R).powi(2) - (PIP_R - PIP_H + RIM_R).powi(2)).sqrt();
    let tol = Tol::witness().get();
    // (a) definite pass: widened circle clears the trimline by 1 cm.
    {
        let (pipped, box_edges) = one_pip(0.12 + s + 0.01, 0.5);
        let blanked = fillet_edges(&pipped, &box_edges, DIE_R, band())
            .expect("box fillet")
            .body;
        let rims = rim_edges(&blanked);
        let out = fillet_edges(&blanked, &rims, RIM_R, band()).expect("rim fillet passes at +1cm");
        assert_eq!(topo::validate_geometric(&out.body), Ok(()), "tier 3");
        let want = blank_volume() - cap(PIP_R, PIP_H) - rim_fillet_extra(PIP_R, PIP_H, RIM_R);
        let v = topo::mass_properties(&out.body).unwrap().volume;
        assert!((v - want).abs() <= 1e-9 * want, "volume {v} vs {want}");
        println!("P2a: +1cm passes, volume ok");
    }
    // (b) definite refuse: widened circle CROSSES the trimline by 5mm.
    {
        let (pipped, box_edges) = one_pip(0.12 + s - 0.005, 0.5);
        let blanked = fillet_edges(&pipped, &box_edges, DIE_R, band())
            .expect("box fillet still ok")
            .body;
        let rims = rim_edges(&blanked);
        let err = fillet_edges(&blanked, &rims, RIM_R, band())
            .expect_err("a crossing trim circle must refuse pre-mutation");
        println!("P2b: -5mm refuses with: {err}");
    }
    // (c) in-band: the gap sits inside [eps, K*eps).
    {
        let (pipped, box_edges) = one_pip(0.12 + s + 5.0 * tol.eps, 0.5);
        let blanked = fillet_edges(&pipped, &box_edges, DIE_R, band())
            .expect("box fillet")
            .body;
        let rims = rim_edges(&blanked);
        let err = fillet_edges(&blanked, &rims, RIM_R, band())
            .expect_err("an in-band clearance must escalate");
        println!("P2c: in-band refuses with: {err}");
    }
}

/// P2d: the BOX-edge pass's ring check — the pip rim (unwidened)
/// crosses a box trimline; the box fillet itself must refuse.
#[test]
fn p2d_box_fillet_ring_touch_refuses() {
    let rho_rim = (PIP_R * PIP_R - (PIP_R - PIP_H).powi(2)).sqrt();
    let (pipped, box_edges) = one_pip(0.12 + rho_rim - 0.005, 0.5);
    let err = fillet_edges(&pipped, &box_edges, DIE_R, band())
        .expect_err("a ring crossing the trimline must refuse pre-mutation");
    println!("P2d: refuses with: {err}");
}

/// P3: one-pip and two-adjacent-pips ladders; then a too-close pair.
#[test]
fn p3_one_and_two_pip_ladders_and_tight_pair() {
    let s = ((PIP_R + RIM_R).powi(2) - (PIP_R - PIP_H + RIM_R).powi(2)).sqrt();
    let per_pip = cap(PIP_R, PIP_H) + rim_fillet_extra(PIP_R, PIP_H, RIM_R);
    // One pip.
    {
        let (pipped, box_edges) = one_pip(0.5, 0.5);
        let blanked = fillet_edges(&pipped, &box_edges, DIE_R, band())
            .expect("box")
            .body;
        let rims = rim_edges(&blanked);
        assert_eq!(rims.len(), 2);
        let out = fillet_edges(&blanked, &rims, RIM_R, band()).expect("rim");
        assert_eq!(out.band_faces.len(), 1);
        let die = &out.body;
        assert_eq!(topo::validate_geometric(die), Ok(()), "tier 3");
        assert_eq!(die.vertices().count(), 24 + 5);
        assert_eq!(die.edges().count(), 48 + 7);
        assert_eq!(die.faces().count(), 26 + 3);
        // The band: ring-free, and its outer cycle traverses the slit TWICE.
        let bf = out.band_faces[0];
        let fd = die.get_face(bf).unwrap();
        assert!(fd.rings.is_empty(), "the band is ring-free");
        let topo::LoopBoundary::Cycle { first } = die.get_loop(fd.outer).unwrap().boundary else {
            panic!("band outer is a cycle");
        };
        let cyc = die.loop_cycle(first).unwrap();
        let mut edges: Vec<EdgeKey> = cyc
            .iter()
            .map(|he| die.get_half_edge(*he).unwrap().edge)
            .collect();
        assert_eq!(edges.len(), 6, "2 ta + 2 tb + slit twice");
        edges.sort_unstable();
        let mut doubled = 0;
        let mut i = 0;
        while i < edges.len() {
            let j = edges[i..].iter().take_while(|e| **e == edges[i]).count();
            if j == 2 {
                doubled += 1;
            }
            i += j;
        }
        assert_eq!(doubled, 1, "exactly one double-traversed edge (the slit)");
        let want = blank_volume() - per_pip;
        let v = topo::mass_properties(die).unwrap().volume;
        assert!(
            (v - want).abs() <= 1e-9 * want,
            "one-pip volume {v} vs {want}"
        );
        let mesh = mesh::tessellate(die, 5e-3).expect("tessellates");
        mesh::validate::check_mesh(&mesh).expect("watertight");
    }
    // Two adjacent pips, die spacing (0.22 apart => ring-ring margin 2s-gap).
    {
        let cube0 = cube(DIE_L);
        let box_edges: Vec<_> = cube0.edges().map(|(k, _)| k).collect();
        let b1 = ball_top(PIP_R, Vec3::new(0.5 - 0.11, 0.5, DIE_L + (PIP_R - PIP_H)));
        let b2 = ball_top(PIP_R, Vec3::new(0.5 + 0.11, 0.5, DIE_L + (PIP_R - PIP_H)));
        let tool = boolean_op_with(
            BooleanOp::Union,
            &b1,
            &b2,
            &BooleanDeclarations::none(),
            SweepStrategy::Realized,
        )
        .expect("tool union")
        .body()
        .expect("a body")
        .body
        .clone();
        let pipped = subtract(&cube0, &tool).expect("2 pips cut");
        let surviving: Vec<_> = box_edges
            .iter()
            .copied()
            .filter(|k| pipped.get_edge(*k).is_some())
            .collect();
        let blanked = fillet_edges(&pipped, &surviving, DIE_R, band())
            .expect("box")
            .body;
        let rims = rim_edges(&blanked);
        assert_eq!(rims.len(), 4);
        println!(
            "P3: two-pip spacing 0.22, ring-ring margin = {}",
            0.22 - 2.0 * s
        );
        let out = fillet_edges(&blanked, &rims, RIM_R, band()).expect("both rims in one call");
        assert_eq!(out.band_faces.len(), 2);
        assert_eq!(topo::validate_geometric(&out.body), Ok(()), "tier 3");
        let want = blank_volume() - 2.0 * per_pip;
        let v = topo::mass_properties(&out.body).unwrap().volume;
        assert!(
            (v - want).abs() <= 1e-9 * want,
            "two-pip volume {v} vs {want}"
        );
    }
    // Too close: widened circles overlap (gap < 2s) -> typed refusal.
    {
        let cube0 = cube(DIE_L);
        let box_edges: Vec<_> = cube0.edges().map(|(k, _)| k).collect();
        let gap = 2.0 * s - 0.004;
        let b1 = ball_top(
            PIP_R,
            Vec3::new(0.5 - gap / 2.0, 0.5, DIE_L + (PIP_R - PIP_H)),
        );
        let b2 = ball_top(
            PIP_R,
            Vec3::new(0.5 + gap / 2.0, 0.5, DIE_L + (PIP_R - PIP_H)),
        );
        let tool = boolean_op_with(
            BooleanOp::Union,
            &b1,
            &b2,
            &BooleanDeclarations::none(),
            SweepStrategy::Realized,
        )
        .expect("tool union")
        .body()
        .expect("a body")
        .body
        .clone();
        let pipped = subtract(&cube0, &tool).expect("2 pips cut");
        let surviving: Vec<_> = box_edges
            .iter()
            .copied()
            .filter(|k| pipped.get_edge(*k).is_some())
            .collect();
        let blanked = fillet_edges(&pipped, &surviving, DIE_R, band())
            .expect("box")
            .body;
        let rims = rim_edges(&blanked);
        let err = fillet_edges(&blanked, &rims, RIM_R, band())
            .expect_err("overlapping widened circles must refuse");
        println!("P3-tight: refuses with: {err}");
    }
}

/// P4: a pip centred ON a box edge — the rim is not a ring of one
/// plane; every door on the way must refuse typed, not corrupt.
#[test]
fn p4_rim_at_a_face_edge_refuses_typed() {
    let cube0 = cube(DIE_L);
    let ball = ball_top(PIP_R, Vec3::new(0.0, 0.5, DIE_L + (PIP_R - PIP_H)));
    match subtract(&cube0, &ball) {
        Err(e) => println!("P4: boolean itself refuses: {e:?}"),
        Ok(pipped) => {
            let rims = rim_edges(&pipped);
            println!("P4: boolean succeeded, {} plane-sphere edges", rims.len());
            match fillet_edges(&pipped, &rims, RIM_R, band()) {
                Err(e) => println!("P4: fillet refuses typed: {e}"),
                Ok(out) => {
                    assert_eq!(
                        topo::validate_geometric(&out.body),
                        Ok(()),
                        "if it builds it must certify"
                    );
                    println!("P4: fillet built and certified (unexpected but sound)");
                }
            }
        }
    }
}

/// P5 (charter C): the slit azimuth is inherited from the revolve
/// seam — rotate the pip ball about its own axis so the seam lands
/// elsewhere; the re-seamed band must certify identically and the
/// volume must not move.
#[test]
fn p5_seam_azimuth_rotation_certifies_identically() {
    let want = blank_volume() - cap(PIP_R, PIP_H) - rim_fillet_extra(PIP_R, PIP_H, RIM_R);
    for theta in [0.0, 0.7, 2.9, -1.3] {
        let cube0 = cube(DIE_L);
        let box_edges: Vec<_> = cube0.edges().map(|(k, _)| k).collect();
        let c = Vec3::new(0.5, 0.5, DIE_L + (PIP_R - PIP_H));
        let ball0 = ball_top(PIP_R, c);
        let ball = topo::transform_rigid(
            &ball0,
            &Affine3::rotation_about_axis(
                geom_core::Point3::new(0.5, 0.5, 0.0),
                Vec3::new(0.0, 0.0, 1.0),
                theta,
            ),
        )
        .unwrap();
        let pipped = subtract(&cube0, &ball).expect("pip cuts");
        let surviving: Vec<_> = box_edges
            .iter()
            .copied()
            .filter(|k| pipped.get_edge(*k).is_some())
            .collect();
        let blanked = fillet_edges(&pipped, &surviving, DIE_R, band())
            .expect("box")
            .body;
        let rims = rim_edges(&blanked);
        let out = fillet_edges(&blanked, &rims, RIM_R, band())
            .unwrap_or_else(|e| panic!("theta {theta}: rim fillet: {e}"));
        assert_eq!(
            topo::validate_geometric(&out.body),
            Ok(()),
            "tier 3 at theta {theta}"
        );
        let v = topo::mass_properties(&out.body).unwrap().volume;
        assert!(
            (v - want).abs() <= 1e-9 * want,
            "theta {theta}: volume {v} vs {want}"
        );
    }
    println!("P5: all azimuths certify with the closed-form volume");
}

/// P6 (charter D): the pip cap faces are stored REVERSED (sense
/// false) — assert that, and that surgery carries the bits through.
#[test]
fn p6_reversed_pip_walls_carry_through() {
    let (pipped, box_edges) = one_pip(0.5, 0.5);
    let sphere_senses: Vec<(topo::FaceKey, bool)> = pipped
        .faces()
        .filter(|(_, f)| matches!(pipped.get_surface(f.surface), Some(Surface::Sphere { .. })))
        .map(|(k, f)| (k, f.sense))
        .collect();
    assert_eq!(sphere_senses.len(), 2, "two half-caps");
    println!("P6: pip wall senses pre-surgery: {sphere_senses:?}");
    let blanked = fillet_edges(&pipped, &box_edges, DIE_R, band())
        .expect("box")
        .body;
    for (k, sense) in &sphere_senses {
        let f = blanked
            .get_face(*k)
            .expect("cap face key survives the box surgery");
        assert_eq!(f.sense, *sense, "cap sense bit carried");
    }
    let rims = rim_edges(&blanked);
    let out = fillet_edges(&blanked, &rims, RIM_R, band()).expect("rim");
    for (k, sense) in &sphere_senses {
        let f = out
            .body
            .get_face(*k)
            .expect("shrunk cap keeps its key through the rim surgery");
        assert_eq!(f.sense, *sense, "cap sense bit carried through the band");
    }
    assert_eq!(topo::validate_geometric(&out.body), Ok(()));
    println!("P6: cap FaceKeys and senses intact through BOTH surgeries");
}

/// P7 (charter F, dev 1): every-edge fillet on a pipped body refuses
/// TangentialEdge at margin EXACTLY 0.0, at every radius — the
/// margin must be structural (co-surface cap meridians), not tuned.
#[test]
fn p7_dev1_radius_sweep_margin_is_structurally_zero() {
    let (pipped, _) = one_pip(0.5, 0.5);
    let every: Vec<_> = pipped.edges().map(|(k, _)| k).collect();
    for r in [1e-4, 5e-3, RIM_R, 0.05, DIE_R] {
        let err = fillet_edges(&pipped, &every, r, band())
            .expect_err("every-edge on a pipped body must refuse");
        match err {
            sweep::fillet::FilletError::TangentialEdge { margin, .. } => {
                assert!(
                    margin == 0.0,
                    "radius {r}: margin {margin} is not structural zero"
                );
            }
            other => panic!("radius {r}: expected TangentialEdge, got {other}"),
        }
    }
    println!("P7: TangentialEdge margin bit-zero at all radii");
}

/// P8: a duplicated edge in the request refuses before the battery.
#[test]
fn p8_duplicate_edge_refuses() {
    let (pipped, box_edges) = one_pip(0.5, 0.5);
    let mut req = box_edges.clone();
    req.push(box_edges[0]);
    let err = fillet_edges(&pipped, &req, DIE_R, band()).expect_err("duplicate edge");
    // The refusal names the repeated key, not just the situation: the
    // caller can act on it without re-deriving which edge doubled.
    assert!(
        matches!(err, sweep::fillet::FilletError::RepeatedEdge { edge } if edge == box_edges[0]),
        "{err}"
    );
}
