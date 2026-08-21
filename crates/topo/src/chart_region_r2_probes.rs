//! Blinded review R2 probes for M9-2 PR-1's chart-region predicate.
//! Adversarial only — nothing here ships; the module is `cfg(test)`.

use super::*;
use crate::euler::{FaceSurface, MefSite, MevSite};
use crate::source::GeomSource;
use geom_brep::{EdgeCurveSpec, Pcurve};
use geom_core::{Point3, Vec3};
use geom_core::Tol;

fn band() -> Band {
    Band::new(1e-9, 1e-8).unwrap()
}
fn pt(x: f64, y: f64) -> Point2<f64> {
    Point2::new(x, y)
}
fn rect(x0: f64, y0: f64, x1: f64, y1: f64) -> Vec<Point2<f64>> {
    vec![pt(x0, y0), pt(x1, y0), pt(x1, y1), pt(x0, y1)]
}
fn face_of(outer: Vec<Point2<f64>>, rings: &[Vec<Point2<f64>>]) -> ScaledFace<f64> {
    let (a2, p) = loop_measures(&outer);
    assert!(a2 > 0.0, "probe polygons are CCW");
    ScaledFace {
        outer,
        outer_2a: a2,
        outer_p: p,
        rings: rings
            .iter()
            .map(|r| {
                let (a2, p) = loop_measures(r);
                (a2.abs(), p)
            })
            .collect(),
    }
}
fn xy_plane() -> Surface<f64> {
    Surface::Plane {
        origin: Point3::origin(),
        normal: Vec3::unit_z(),
        u_ref: Vec3::unit_x(),
    }
}
/// Same plane LOCUS, a different chart frame (u_ref rotated 90°).
fn xy_plane_rotated() -> Surface<f64> {
    Surface::Plane {
        origin: Point3::origin(),
        normal: Vec3::unit_z(),
        u_ref: Vec3::unit_y(),
    }
}
fn sheet(
    body: &mut Body<f64>,
    x0: f64,
    y0: f64,
    x1: f64,
    y1: f64,
    surface: FaceSurface<f64>,
    tol: Tol,
) -> FaceKey {
    let c = |x: f64, y: f64| Point3::new(x, y, 0.0);
    let (a, b, cc, d) = (c(x0, y0), c(x1, y0), c(x1, y1), c(x0, y1));
    let seed = body.mvfs(a).unwrap();
    let e_ab = body
        .mev_line(
            MevSite::Lone {
                r#loop: seed.r#loop,
            },
            b,
            tol,
        )
        .unwrap();
    let e_bc = body
        .mev_line(
            MevSite::Fan {
                he1: e_ab.he_minus,
                he2: e_ab.he_minus,
            },
            cc,
            tol,
        )
        .unwrap();
    let e_cd = body
        .mev_line(
            MevSite::Fan {
                he1: e_bc.he_minus,
                he2: e_bc.he_minus,
            },
            d,
            tol,
        )
        .unwrap();
    let he_dc = body
        .find_half_edge(seed.face, e_cd.vertex, e_bc.vertex)
        .unwrap();
    body.mef(
        MefSite::Chords {
            he1: he_dc,
            he2: e_ab.he_plus,
        },
        EdgeCurveSpec::line_between(d, a),
        surface,
        tol,
    )
    .unwrap()
    .face
}

// ---------------------------------------------------------------
// CLAIM 1 — the structural inventory gate
// ---------------------------------------------------------------

fn harmonic(pa: (f64, f64), pb: (f64, f64)) -> Pcurve<f64> {
    Pcurve::Harmonic {
        p0: pt(0.0, 0.0),
        pa: Vec2::new(pa.0, pa.1),
        pb: Vec2::new(pb.0, pb.1),
        pl: Vec2::new(1.0, 0.0),
    }
}

#[test]
fn probe_every_almost_zero_channel_refuses() {
    // A denormal, the smallest normal, a negative denormal, and a
    // "line-like" 1e-300 — each in each of the four channels.
    let tinies = [1e-300_f64, f64::MIN_POSITIVE, -1e-300, 5e-324, 1e-16];
    for t in tinies {
        for slot in 0..4 {
            let (mut pa, mut pb) = ((0.0, 0.0), (0.0, 0.0));
            match slot {
                0 => pa.0 = t,
                1 => pa.1 = t,
                2 => pb.0 = t,
                _ => pb.1 = t,
            }
            assert!(
                pcurve_entry(&harmonic(pa, pb), 0.0, 1.0, true).is_err(),
                "channel {slot} = {t:e} must REFUSE (no scalar zero-test)"
            );
        }
    }
    // Signed structural zeros are still structural zeros.
    assert!(pcurve_entry(&harmonic((0.0, -0.0), (-0.0, 0.0)), 0.0, 1.0, true).is_ok());
}

#[test]
fn probe_line_like_but_not_structurally_zero_refuses() {
    // pa alive, pl DEAD: the UV image is genuinely a straight segment
    // (u = cos t, v const) yet sinusoidally parameterized. A chord
    // read would accept it; the variant/structure gate refuses. This
    // is the conservative direction, and it is what "no scalar
    // zero-test on T" costs.
    let p = Pcurve::Harmonic {
        p0: pt(0.0, 0.0),
        pa: Vec2::new(1.0, 0.0),
        pb: Vec2::new(0.0, 0.0),
        pl: Vec2::new(0.0, 0.0),
    };
    assert!(pcurve_entry(&p, 0.0, 1.0, true).is_err());
    // A genuinely curved image (u sinusoid, v linear) also refuses.
    let q = Pcurve::Harmonic {
        p0: pt(0.0, 0.0),
        pa: Vec2::new(1.0, 0.0),
        pb: Vec2::new(0.0, 0.0),
        pl: Vec2::new(0.0, 1.0),
    };
    assert!(pcurve_entry(&q, 0.0, 1.0, true).is_err());
}

// ---------------------------------------------------------------
// CLAIM 2 — the same-chart lane
// ---------------------------------------------------------------

#[test]
fn probe_same_locus_different_chart_frame_never_certifies() {
    let tol = Tol::witness();
    // Two bodies, the SAME plane locus, different chart frames
    // (u_ref x vs y). No sources: must diverge.
    let mut ba = Body::<f64>::new();
    let fa = sheet(&mut ba, 0.0, 0.0, 2.0, 2.0, FaceSurface::New(xy_plane()), tol);
    let mut bb = Body::<f64>::new();
    let fb = sheet(
        &mut bb,
        0.0,
        0.0,
        2.0,
        2.0,
        FaceSurface::New(xy_plane_rotated()),
        tol,
    );
    match chart_region_overlap(&ba, fa, &bb, fb, band()) {
        Err(ChartRegionError::ChartDivergence { .. }) => {}
        other => panic!("different chart frames must diverge, got {other:?}"),
    }
    // Even the BIT-IDENTICAL surface across two sourceless bodies
    // diverges — the structural rung is the whole test.
    let mut bc = Body::<f64>::new();
    let fc = sheet(&mut bc, 0.0, 0.0, 2.0, 2.0, FaceSurface::New(xy_plane()), tol);
    match chart_region_overlap(&ba, fa, &bc, fc, band()) {
        Err(ChartRegionError::ChartDivergence { .. }) => {}
        other => panic!("sourceless cross-body must diverge, got {other:?}"),
    }
}

/// The ADVERSARIAL rung-2 attack: the same `GeomSource` attached to
/// two DIFFERENT chart frames of one locus. N6 says this cannot
/// happen; the module trusts N6 rather than re-checking the bits.
#[test]
fn probe_forged_source_on_divergent_charts() {
    let tol = Tol::witness();
    let mut ba = Body::<f64>::new();
    let fa = sheet(&mut ba, 0.0, 0.0, 2.0, 2.0, FaceSurface::New(xy_plane()), tol);
    let ka = ba.get_face(fa).unwrap().surface;
    ba.set_surface_source(ka, GeomSource::minted(7, 0)).unwrap();
    let mut bb = Body::<f64>::new();
    let fb = sheet(
        &mut bb,
        0.0,
        0.0,
        2.0,
        2.0,
        FaceSurface::New(xy_plane_rotated()),
        tol,
    );
    let kb = bb.get_face(fb).unwrap().surface;
    bb.set_surface_source(kb, GeomSource::minted(7, 0)).unwrap();
    let got = chart_region_overlap(&ba, fa, &bb, fb, band());
    // The union fix (U1): the rung-2 lane VERIFIES N6's bit-identity
    // claim instead of assuming it — a forged same-source pair on two
    // different chart frames refuses typed. (Pre-fix this probe
    // recorded the hole: the pair answered Ok(PositiveArea).)
    println!("forged-source divergent charts => {got:?}");
    assert!(matches!(got, Err(ChartRegionError::ChartDivergence { .. })));
}

#[test]
fn probe_reverted_and_placed_sources_diverge() {
    let tol = Tol::witness();
    let mut ba = Body::<f64>::new();
    let fa = sheet(&mut ba, 0.0, 0.0, 2.0, 2.0, FaceSurface::New(xy_plane()), tol);
    let ka = ba.get_face(fa).unwrap().surface;
    let src = GeomSource::minted(3, 1);
    ba.set_surface_source(ka, src.clone()).unwrap();
    for (name, other) in [
        ("reverted", src.reverted()),
        ("placed", src.placed(9, 0)),
        ("other-node", GeomSource::minted(4, 1)),
        ("other-index", GeomSource::minted(3, 2)),
    ] {
        let mut bb = Body::<f64>::new();
        let fb = sheet(&mut bb, 0.0, 0.0, 2.0, 2.0, FaceSurface::New(xy_plane()), tol);
        let kb = bb.get_face(fb).unwrap().surface;
        bb.set_surface_source(kb, other).unwrap();
        match chart_region_overlap(&ba, fa, &bb, fb, band()) {
            Err(ChartRegionError::ChartDivergence { .. }) => {}
            got => panic!("{name} source must diverge, got {got:?}"),
        }
    }
}

// ---------------------------------------------------------------
// CLAIM 3/4 — arms, dimensions, mm-vs-metre twins
// ---------------------------------------------------------------

#[test]
fn probe_cone_and_torus_arms_refuse() {
    for (s, name) in [
        (
            Surface::Cone {
                apex: Point3::origin(),
                axis: Vec3::unit_z(),
                half_angle: 0.4,
                u_ref: Vec3::unit_x(),
            },
            "cone",
        ),
        (
            Surface::Torus {
                center: Point3::origin(),
                axis: Vec3::unit_z(),
                major_radius: 2.0,
                minor_radius: 0.5,
                u_ref: Vec3::unit_x(),
            },
            "torus",
        ),
    ] {
        match exact_arms(&s) {
            Err(ChartRegionError::ArmUnbounded { chart }) => assert_eq!(chart, name),
            other => panic!("{name} must refuse, got {other:?}"),
        }
    }
}

/// mm-vs-metre twin: the SAME shape at 1000× and 1/1000× scale. Every
/// row is metres, so the verdict must track the ABSOLUTE band — a
/// scale-invariant answer would prove a dimensionless (dishonest) row.
#[test]
fn probe_metre_scale_twins_move_the_verdict() {
    let w = 3e-9; // in-band at metre scale
    let run = |s: f64| {
        let a = face_of(rect(0.0, 0.0, 1.0 * s, 1.0 * s), &[]);
        let b = face_of(rect(1.0 * s - w * s, -0.3 * s, 2.0 * s, 1.3 * s), &[]);
        overlap_of_regions(&a, &b, false, band())
    };
    // Metre scale: in band ⇒ escalate.
    assert!(matches!(run(1.0), Err(ChartRegionError::Escalated(_))));
    // Millimetre-sized model (1e-3): the strip is 3e-12 m ⇒ still not
    // certifiable, and NEVER silently "Empty".
    let small = run(1e-3);
    assert!(
        !matches!(small, Ok(ChartOverlap::PositiveArea)),
        "a 3e-12 m strip must never certify: {small:?}"
    );
    println!("1e-3 scale => {small:?}");
    // Kilometre-sized model (1e3): the strip is 3e-6 m ⇒ certifies.
    assert!(matches!(run(1e3), Ok(ChartOverlap::PositiveArea)));
}

/// The band is what bites: the SAME sliver certifies under a tighter
/// band and escalates under the shipped one.
#[test]
fn probe_mutating_the_band_moves_the_sliver_row() {
    let a = face_of(rect(0.0, 0.0, 1.0, 1.0), &[]);
    let b = face_of(rect(1.0 - 3e-9, -0.3, 2.0, 1.3), &[]);
    assert!(matches!(
        overlap_of_regions(&a, &b, false, band()),
        Err(ChartRegionError::Escalated(_))
    ));
    let tight = Band::new(1e-12, 1e-11).unwrap();
    assert!(matches!(
        overlap_of_regions(&a, &b, false, tight),
        Ok(ChartOverlap::PositiveArea)
    ));
    let loose = Band::new(1e-6, 1e-5).unwrap();
    assert!(overlap_of_regions(&a, &b, false, loose).is_err());
}

/// The mean-width derivation, checked by hand on a known intersection.
#[test]
fn probe_mean_width_matches_the_hand_derivation() {
    // A = [0,1]², B = [0.9,3]×[-1,2]: intersection is 0.1 × 1.
    let a = face_of(rect(0.0, 0.0, 1.0, 1.0), &[]);
    let b = face_of(rect(0.9, -1.0, 3.0, 2.0), &[]);
    let crossings = proper_crossings(&a.outer, &b.outer, band()).unwrap();
    let pieces = intersection_pieces(&a.outer, &b.outer, &crossings, band()).unwrap();
    let (mut a2, mut p) = (0.0, 0.0);
    for pc in &pieces {
        let (x, y) = loop_measures(pc);
        a2 += x;
        p += y;
    }
    assert!((a2 - 0.2).abs() < 1e-12, "2A = 2·(0.1·1) = 0.2, got {a2}");
    assert!((p - 2.2).abs() < 1e-12, "P = 2(0.1+1) = 2.2, got {p}");
    // mean width 2A/P = 0.2/2.2 = 0.0909… — NOT the 0.1 true width, it
    // UNDER-states it. Under-stating is the safe direction.
    assert!((a2 / p - 0.090_909_090_909).abs() < 1e-9);
}

// ---------------------------------------------------------------
// CLAIM 7 — the clip walk under adversarial loops
// ---------------------------------------------------------------

#[test]
fn probe_corner_touch_and_edge_share_refuse_typed() {
    let a = face_of(rect(0.0, 0.0, 1.0, 1.0), &[]);
    // Corner-only touch.
    let corner = face_of(rect(1.0, 1.0, 2.0, 2.0), &[]);
    assert!(matches!(
        overlap_of_regions(&a, &corner, false, band()),
        Err(ChartRegionError::TouchingBoundary)
    ));
    // Full shared edge.
    let edge = face_of(rect(1.0, 0.0, 2.0, 1.0), &[]);
    assert!(matches!(
        overlap_of_regions(&a, &edge, false, band()),
        Err(ChartRegionError::TouchingBoundary)
    ));
    // Partial shared edge (offset).
    let part = face_of(rect(1.0, 0.5, 2.0, 1.5), &[]);
    assert!(matches!(
        overlap_of_regions(&a, &part, false, band()),
        Err(ChartRegionError::TouchingBoundary)
    ));
    // Contained-but-edge-flush: B shares A's left edge line.
    let flush = face_of(rect(0.0, 0.25, 0.5, 0.75), &[]);
    let got = overlap_of_regions(&a, &flush, false, band());
    println!("edge-flush containment => {got:?}");
    assert!(matches!(got, Err(ChartRegionError::TouchingBoundary)));
}

#[test]
fn probe_collinear_runs_do_not_disturb_the_walk() {
    // A's bottom edge carries three extra COLLINEAR vertices.
    let a = face_of(
        vec![
            pt(0.0, 0.0),
            pt(0.25, 0.0),
            pt(0.5, 0.0),
            pt(0.75, 0.0),
            pt(1.0, 0.0),
            pt(1.0, 1.0),
            pt(0.0, 1.0),
        ],
        &[],
    );
    let b = face_of(rect(0.5, 0.5, 1.5, 1.5), &[]);
    assert_eq!(
        overlap_of_regions(&a, &b, false, band()).unwrap(),
        ChartOverlap::PositiveArea
    );
    let crossings = proper_crossings(&a.outer, &b.outer, band()).unwrap();
    let pieces = intersection_pieces(&a.outer, &b.outer, &crossings, band()).unwrap();
    let (a2, _) = loop_measures(&pieces[0]);
    assert!((a2 - 2.0 * 0.25).abs() < 1e-12, "2A of 0.5×0.5, got {a2}");
}

#[test]
fn probe_comb_yields_three_pieces_with_exact_area() {
    // A three-tooth comb crossed by a horizontal bar: three pieces.
    let comb = vec![
        pt(0.0, 0.0),
        pt(5.0, 0.0),
        pt(5.0, 3.0),
        pt(4.0, 3.0),
        pt(4.0, 1.0),
        pt(3.0, 1.0),
        pt(3.0, 3.0),
        pt(2.0, 3.0),
        pt(2.0, 1.0),
        pt(1.0, 1.0),
        pt(1.0, 3.0),
        pt(0.0, 3.0),
    ];
    let a = face_of(comb, &[]);
    let bar = face_of(rect(-1.0, 2.0, 6.0, 2.5), &[]);
    let crossings = proper_crossings(&a.outer, &bar.outer, band()).unwrap();
    assert_eq!(
        crossings.len(),
        12,
        "three teeth ⇒ each tooth's two walls crossed twice"
    );
    let pieces = intersection_pieces(&a.outer, &bar.outer, &crossings, band()).unwrap();
    assert_eq!(pieces.len(), 3, "three teeth ⇒ three pieces");
    let total: f64 = pieces.iter().map(|p| loop_measures(p).0).sum();
    assert!(
        (total - 2.0 * 3.0 * 0.5).abs() < 1e-12,
        "2A = 2·(3 teeth × 1 × 0.5), got {total}"
    );
    assert_eq!(
        overlap_of_regions(&a, &bar, false, band()).unwrap(),
        ChartOverlap::PositiveArea
    );
}

#[test]
fn probe_crossing_on_edge_zero_and_the_order_wrap() {
    // B's boundary crosses A's edge 0 twice, forcing the a_order wrap
    // branch of `push_between`.
    let a = face_of(rect(0.0, 0.0, 2.0, 2.0), &[]);
    let b = face_of(rect(0.5, -1.0, 1.5, 1.0), &[]);
    let crossings = proper_crossings(&a.outer, &b.outer, band()).unwrap();
    assert_eq!(crossings.len(), 2);
    assert!(crossings.iter().all(|c| c.ai == 0), "both on A's edge 0");
    let pieces = intersection_pieces(&a.outer, &b.outer, &crossings, band()).unwrap();
    assert_eq!(pieces.len(), 1);
    let (a2, _) = loop_measures(&pieces[0]);
    assert!((a2 - 2.0 * 1.0).abs() < 1e-12, "2A of 1×1, got {a2}");
}

#[test]
fn probe_ring_swallowing_the_overlap_never_answers_empty() {
    // A hole in A that dwarfs the outer intersection: the true answer
    // is empty, but "empty" is not decidable this way — must escalate.
    let a = face_of(rect(0.0, 0.0, 4.0, 4.0), &[rect(0.5, 0.5, 3.5, 3.5)]);
    let b = face_of(rect(1.0, 1.0, 2.0, 2.0), &[]);
    let got = overlap_of_regions(&a, &b, false, band());
    println!("hole-swallowed overlap => {got:?}");
    assert!(matches!(got, Err(ChartRegionError::Escalated(_))));
}

#[test]
fn probe_bit_identical_fast_path_is_rotation_stable() {
    let a = face_of(rect(0.0, 0.0, 1.0, 1.0), &[]);
    // Same cycle, rotated start vertex.
    let b = face_of(
        vec![pt(1.0, 0.0), pt(1.0, 1.0), pt(0.0, 1.0), pt(0.0, 0.0)],
        &[],
    );
    assert!(bit_equal_cyclic(&a.outer, &b.outer));
    assert_eq!(
        overlap_of_regions(&a, &b, false, band()).unwrap(),
        ChartOverlap::PositiveArea
    );
    // One coordinate perturbed by 1 ulp: the fast path must NOT fire,
    // and the honest fallthrough is a touching boundary.
    let eps = f64::from_bits(1.0_f64.to_bits() + 1);
    let c = face_of(rect(0.0, 0.0, eps, 1.0), &[]);
    assert!(!bit_equal_cyclic(&a.outer, &c.outer));
    let got = overlap_of_regions(&a, &c, false, band());
    println!("1-ulp-perturbed twin => {got:?}");
    assert!(matches!(got, Err(ChartRegionError::TouchingBoundary)));
}

// ---------------------------------------------------------------
// CLAIM 6 — determinism replay
// ---------------------------------------------------------------

#[test]
fn probe_replay_determinism() {
    let tol = Tol::witness();
    let mut body = Body::<f64>::new();
    let f1 = sheet(&mut body, 0.0, 0.0, 2.0, 2.0, FaceSurface::New(xy_plane()), tol);
    let key = body.get_face(f1).unwrap().surface;
    let f2 = sheet(&mut body, 1.0, 1.0, 3.0, 3.0, FaceSurface::Shared(key), tol);
    let first = chart_region_overlap(&body, f1, &body, f2, band()).unwrap();
    for _ in 0..64 {
        assert_eq!(
            chart_region_overlap(&body, f1, &body, f2, band()).unwrap(),
            first
        );
    }
    // Argument order symmetry.
    assert_eq!(
        chart_region_overlap(&body, f2, &body, f1, band()).unwrap(),
        first
    );
}

// ---------------------------------------------------------------
// CLAIM 5 — the seam branch
// ---------------------------------------------------------------

#[test]
fn probe_seam_straddle_and_exact_full_wrap() {
    let cyl = Surface::Cylinder {
        origin: Point3::origin(),
        axis: Vec3::unit_z(),
        radius: 2.0,
        u_ref: Vec3::unit_x(),
    };
    let uv = |o: Vec<Point2<f64>>| FaceUv {
        outer: o,
        rings: vec![],
    };
    let tau = std::f64::consts::TAU;
    // Same branch: fine.
    let a = uv(rect(0.1, 0.0, 1.0, 1.0));
    let b = uv(rect(0.5, 0.0, 1.5, 1.0));
    assert!(seam_gate(&cyl, &a, &b, band()).is_ok());
    // τ apart: different pinned branches.
    let c = uv(rect(0.1 + tau, 0.0, 1.0 + tau, 1.0));
    assert!(matches!(
        seam_gate(&cyl, &a, &c, band()),
        Err(ChartRegionError::SeamBranch)
    ));
    // EXACT full wrap: span is exactly τ ⇒ the Zero outcome passes.
    let w = uv(rect(0.0, 0.0, tau, 1.0));
    assert!(seam_gate(&cyl, &w, &w, band()).is_ok());
    // A hair over τ, well outside the band at r = 2: refuses.
    let over = uv(rect(0.0, 0.0, tau + 1e-6, 1.0));
    assert!(matches!(
        seam_gate(&cyl, &over, &over, band()),
        Err(ChartRegionError::SeamBranch)
    ));
    // A hair over τ but INSIDE the band (3e-9 rad × 2 m = 6e-9 m):
    // three-outcome honesty says this must not silently pass.
    let inband = uv(rect(0.0, 0.0, tau + 3e-9, 1.0));
    let got = seam_gate(&cyl, &inband, &inband, band());
    println!("in-band seam excess => {got:?}");
    assert!(matches!(got, Err(ChartRegionError::Escalated(_))));
}

/// The one theoretical false-`Empty`: `polygon_relation(a, b)` answering
/// `None` (every A vertex on B's boundary) falls into the `_` arm and
/// can reach `Ok(Empty)` via the b-side `Some(Out)`. Attempts to build
/// a witness — every configuration refuses earlier, at the crossing
/// rows — so the arm is unreachable in practice but undefended.
#[test]
fn probe_all_vertices_on_the_other_boundary_never_answers_empty() {
    let b = face_of(rect(0.0, 0.0, 2.0, 2.0), &[]);
    // (i) the inscribed diamond on B's edge midpoints.
    let diamond = face_of(
        vec![pt(1.0, 0.0), pt(2.0, 1.0), pt(1.0, 2.0), pt(0.0, 1.0)],
        &[],
    );
    let got = overlap_of_regions(&diamond, &b, false, band());
    println!("inscribed diamond => {got:?}");
    assert!(!matches!(got, Ok(ChartOverlap::Empty)), "{got:?}");
    // (ii) A ⊂ B sharing three whole edges.
    let flush = face_of(rect(0.0, 0.0, 1.0, 2.0), &[]);
    let got2 = overlap_of_regions(&flush, &b, false, band());
    println!("three-edge-flush subset => {got2:?}");
    assert!(!matches!(got2, Ok(ChartOverlap::Empty)), "{got2:?}");
    // (iii) A ⊂ B touching B's boundary at one corner only.
    let corner = face_of(rect(0.0, 0.0, 1.0, 1.0), &[]);
    let got3 = overlap_of_regions(&corner, &b, false, band());
    println!("corner-anchored subset => {got3:?}");
    assert!(!matches!(got3, Ok(ChartOverlap::Empty)), "{got3:?}");
}
