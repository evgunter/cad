//! PR 3 adversarial review — target 4: `point_in_loop` (F8's seed)
//! driven adversarially: concave loops, the graze-retry schedule, a
//! constructed RayExhausted witness, and boundary-pre-pass edges.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

mod common;

use common::prism;
use geom_core::{Point3, Vec3};
use topo::{LoopContainment, PointInLoopError, point_in_loop};

fn n_z() -> Vec3<f64> {
    Vec3::new(0.0, 0.0, 1.0)
}

/// The margin an escalation refused on, for the sign-blindness row.
fn escalated_margin(r: &Result<LoopContainment, PointInLoopError>) -> geom_core::MarginDiag {
    match r {
        Err(PointInLoopError::Escalated { diag, .. }) => diag.margin,
        other => panic!("expected an escalation, got {other:?}"),
    }
}

/// Concave (L-shaped) loop: points in the notch are Out even though
/// they sit inside the convex hull; points in both arms are In.
#[test]
fn concave_loop_in_out() {
    let profile = [
        (0.0, 0.0),
        (3.0, 0.0),
        (3.0, 3.0),
        (2.0, 3.0),
        (2.0, 1.0),
        (0.0, 1.0),
    ];
    let fx = prism::<f64>(&profile, 1.0);
    let top = fx.body.get_face(fx.top_face).unwrap();
    let band = geom_core::Band::linear().unwrap();
    let q = |x: f64, y: f64| Point3::new(x, y, 1.0);
    let pil = |p| point_in_loop(&fx.body, top.outer, n_z(), p, band).unwrap();
    assert_eq!(pil(q(0.5, 0.5)), LoopContainment::In); // bottom arm
    assert_eq!(pil(q(2.5, 2.0)), LoopContainment::In); // right arm
    assert_eq!(pil(q(1.0, 2.0)), LoopContainment::Out); // the notch
    assert_eq!(pil(q(-1.0, 0.5)), LoopContainment::Out);
    assert_eq!(pil(q(2.0, 2.0)), LoopContainment::OnBoundary); // notch wall
}

/// Graze-retry: the first schedule ray (+x from q) passes EXACTLY
/// through the dart vertex (3, 1) — the side predicate reads Zero and
/// the next schedule member must take over and still answer
/// correctly on both sides of the dart.
#[test]
fn ray_graze_retries_deterministically() {
    let profile = [(0.0, 0.0), (4.0, 0.0), (3.0, 1.0), (4.0, 2.0), (0.0, 2.0)];
    let fx = prism::<f64>(&profile, 1.0);
    let top = fx.body.get_face(fx.top_face).unwrap();
    let band = geom_core::Band::linear().unwrap();
    let pil = |x: f64, y: f64| {
        point_in_loop(&fx.body, top.outer, n_z(), Point3::new(x, y, 1.0), band).unwrap()
    };
    // q = (1,1): the +x ray hits (3,1) dead on; retry must say In.
    assert_eq!(pil(1.0, 1.0), LoopContainment::In);
    // q = (3.5,1): right of the dart tip, in the notch: Out (its +x
    // ray ALSO grazes the dart vertex... from behind; retry decides).
    assert_eq!(pil(3.5, 1.0), LoopContainment::Out);
    // Determinism: same answers on replay.
    assert_eq!(pil(1.0, 1.0), LoopContainment::In);
}

/// The 15-gon of the row above: one vertex on each usable schedule
/// ray line through the origin, so every member grazes.
fn all_rays_graze_profile() -> Vec<(f64, f64)> {
    // In-plane directions of the schedule for a +z normal.
    let dirs: [(f64, f64); 15] = [
        (1.0, 0.0),
        (0.0, 1.0),
        (0.5, 0.25),
        (1.0, 0.5),
        (0.25, 1.0),
        (-0.5, 1.0),
        (0.125, -0.5),
        (1.0, 0.125),
        (0.75, -1.0),
        (0.375, 0.75),
        (-1.0, 0.375),
        (0.625, 0.9375),
        (0.3125, -0.625),
        (0.9375, 0.3125),
        (-0.75, -0.25),
    ];
    // One polygon vertex per ray LINE, placed at the direction's angle
    // (radius varied to break symmetry), sorted by angle => a simple
    // star-shaped polygon around the origin.
    let mut corners: Vec<(f64, f64, f64)> = dirs
        .iter()
        .enumerate()
        .map(|(k, &(x, y))| {
            let ang = y.atan2(x);
            let r = 2.0 + 0.03 * k as f64;
            let l = (x * x + y * y).sqrt();
            (ang, r * x / l, r * y / l)
        })
        .collect();
    corners.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
    corners.iter().map(|&(_, x, y)| (x, y)).collect()
}

/// A constructed RayExhausted: a 15-gon with one vertex ON each
/// usable schedule ray line through the query point (for n = +z the
/// [0,0,1] member projects degenerately and is skipped), every ray
/// grazes and the typed exhaustion error comes back — reachable, not
/// just decorative.
#[test]
fn ray_exhausted_is_reachable() {
    let profile = all_rays_graze_profile();
    let fx = prism::<f64>(&profile, 1.0);
    let top = fx.body.get_face(fx.top_face).unwrap();
    let band = geom_core::Band::linear().unwrap();
    let err =
        point_in_loop(&fx.body, top.outer, n_z(), Point3::new(0.0, 0.0, 1.0), band).unwrap_err();
    assert!(
        matches!(err, PointInLoopError::RayExhausted { .. }),
        "got {err:?}"
    );
    // One step off the degenerate center, the same loop answers.
    let ok = point_in_loop(
        &fx.body,
        top.outer,
        n_z(),
        Point3::new(0.011, 0.017, 1.0),
        band,
    )
    .unwrap();
    assert_eq!(ok, LoopContainment::In);
}

/// Boundary pre-pass: interior of an edge, a corner, and a point one
/// band-width off the boundary (clean In — no boundary absorption).
#[test]
fn boundary_pre_pass_edges() {
    let fx = prism::<f64>(&[(0.0, 0.0), (2.0, 0.0), (2.0, 2.0), (0.0, 2.0)], 1.0);
    let top = fx.body.get_face(fx.top_face).unwrap();
    let band = geom_core::Band::linear().unwrap();
    let pil = |x: f64, y: f64| {
        point_in_loop(&fx.body, top.outer, n_z(), Point3::new(x, y, 1.0), band).unwrap()
    };
    assert_eq!(pil(1.0, 0.0), LoopContainment::OnBoundary);
    assert_eq!(pil(2.0, 2.0), LoopContainment::OnBoundary);
    // 100·K·ε off the wall: decisively In/Out at every ε row (the
    // pre-pass must not swallow a clean margin).
    let off = 1000.0 * geom_core::Tolerance::get().eps;
    assert_eq!(pil(1.0, off), LoopContainment::In);
    assert_eq!(pil(1.0, -off), LoopContainment::Out);
}

/// **The verdict is blind to the normal's sign.** `point_in_loop` uses
/// the normal to recover the loop's plane, not to orient it: the
/// in-plane projection is invariant under `n̂ ↦ −n̂` and the parity
/// walk's side axis `n̂ × d` merely negates every vertex ordinate,
/// which leaves the straddle test, the graze and the crossing advance
/// unchanged. That is what licenses `chord_join`'s ring re-homing to
/// pass the raw CHART normal (S67 / smell-scan D6) — a reversed-sense
/// planar face cannot re-home a ring differently from its twin.
///
/// Pinned over the three definite outcomes, the graze retry, an
/// oblique carrier and the typed `RayExhausted` refusal — a sign that
/// moved a REFUSAL would be just as much a defect as one that moved a
/// verdict.
///
/// **A refusal is compared by variant, predicate and band, not by its
/// `Debug` rendering**, and the difference is real: an `Indeterminate`
/// carries the SIGNED margin it refused on, so two mirrored escalations
/// legitimately render as `Value(−m)` and `Value(m)`. `Indeterminate`
/// documents its fields as diagnostic data for error messages and
/// telemetry, and no verdict in the walk reads one back; comparing the
/// rendering would red this row on a probe that merely escalated.
///
/// **What this row can and cannot catch.** It fires on a branch inside
/// the walk that reads the normal's sign, which is the drift it
/// guards; and the tilted block below pins ABSOLUTE verdicts, so gross
/// walk arithmetic reds it too. What no differential row can catch is
/// an error that moves both signs equally — the invariance argument at
/// `chord_join::face_plane_normal` is therefore stated as algebra
/// rather than left to this row. The cost of that limit is bounded:
/// the four rows above pin the walk's arithmetic absolutely, so a
/// symmetric error has to survive them first.
#[test]
fn the_verdict_is_blind_to_the_normals_sign() {
    let band = geom_core::Band::linear().unwrap();
    let flipped = Vec3::new(0.0, 0.0, -1.0);
    // A refusal's sign-INVARIANT content: the variant, and for an
    // escalation the predicate, the band and the margin's KIND. The
    // margin's VALUE is signed and is meant to differ — see
    // `point_in_loop`'s "sign of `normal`" section.
    fn shape(r: &Result<LoopContainment, PointInLoopError>) -> String {
        match r {
            Ok(v) => format!("{v:?}"),
            Err(PointInLoopError::Escalated { r#loop, diag }) => {
                let kind = match diag.margin {
                    geom_core::MarginDiag::Value(_) => "Value",
                    geom_core::MarginDiag::Enclosure { .. } => "Enclosure",
                    geom_core::MarginDiag::Invalid => "Invalid",
                };
                format!(
                    "Escalated{loop:?}/{:?}/{:?}/{kind}",
                    diag.predicate, diag.band
                )
            }
            Err(e) => format!("{e:?}"),
        }
    }
    // Returns how many of `probes` DECIDED, so the row cannot pass by
    // comparing two refusals.
    let agree = |profile: &[(f64, f64)], probes: &[(f64, f64)]| -> usize {
        let fx = prism::<f64>(profile, 1.0);
        let top = fx.body.get_face(fx.top_face).unwrap();
        let mut definite = 0;
        for &(x, y) in probes {
            let q = Point3::new(x, y, 1.0);
            let up = point_in_loop(&fx.body, top.outer, n_z(), q, band);
            let down = point_in_loop(&fx.body, top.outer, flipped, q, band);
            definite += usize::from(up.is_ok());
            assert_eq!(
                shape(&up),
                shape(&down),
                "the sign of the plane normal moved the verdict at {q:?}"
            );
        }
        definite
    };
    // The concave loop: both arms, the notch, outside, and a boundary
    // point; then the dart, whose first schedule member grazes, so the
    // retry itself runs on both signs; then a square for the clean
    // In/Out rows.
    let definite = agree(
        &[
            (0.0, 0.0),
            (3.0, 0.0),
            (3.0, 3.0),
            (2.0, 3.0),
            (2.0, 1.0),
            (0.0, 1.0),
        ],
        &[(0.5, 0.5), (2.5, 2.0), (1.0, 2.0), (-1.0, 0.5), (2.0, 2.0)],
    ) + agree(
        &[(0.0, 0.0), (4.0, 0.0), (3.0, 1.0), (4.0, 2.0), (0.0, 2.0)],
        &[(1.0, 1.0), (3.5, 1.0)],
    ) + agree(
        &[(0.0, 0.0), (2.0, 0.0), (2.0, 2.0), (0.0, 2.0)],
        &[(1.0, 1.0), (1.0, 0.0), (5.0, 5.0)],
    );
    assert_eq!(
        definite, 10,
        "every probe above must decide, or the row compares two refusals and pins nothing"
    );

    // **The escalation arm, which is why `shape` exists.** A probe
    // whose ordinate against the first schedule member lands strictly
    // inside the ambiguity band — the geometric mean of the band's two
    // thresholds, so the row survives every eps in the matrix — and
    // sits ~1 unit from every edge, so the boundary pre-pass does not
    // absorb it first. Both signs must refuse the SAME way; their
    // margins are meant to be opposite in sign, and comparing the
    // whole `Debug` here would red on that alone.
    let delta = (band.zero() * band.escalate()).sqrt();
    assert!(delta > band.zero() && delta < band.escalate());
    let fx = prism::<f64>(
        &[(0.0, 0.0), (4.0, 0.0), (3.0, 1.0), (4.0, 2.0), (0.0, 2.0)],
        1.0,
    );
    let top = fx.body.get_face(fx.top_face).unwrap();
    let q = Point3::new(1.0, 1.0 + delta, 1.0);
    let up = point_in_loop(&fx.body, top.outer, n_z(), q, band);
    let down = point_in_loop(&fx.body, top.outer, flipped, q, band);
    let geom_core::MarginDiag::Value(m_up) = escalated_margin(&up) else {
        panic!("expected an f64 escalation, got {up:?}");
    };
    let geom_core::MarginDiag::Value(m_down) = escalated_margin(&down) else {
        panic!("expected an f64 escalation, got {down:?}");
    };
    assert_eq!(m_up, -m_down, "the two margins must be exact negations");
    assert_ne!(
        format!("{up:?}"),
        format!("{down:?}"),
        "if the renderings ever agree here, this row has stopped exercising `shape`"
    );
    assert_eq!(shape(&up), shape(&down));

    // A TILTED carrier, because every probe above lies on a `+z` face
    // where the deciding schedule members have `n̂·r = 0` and the
    // projection step is invariant for free. This wall's normal is
    // oblique in xy, so `r − n̂(n̂·r)` is a real subtraction on the
    // member that decides.
    let fx = prism::<f64>(
        &[(0.0, 0.0), (4.0, 0.0), (3.0, 1.0), (4.0, 2.0), (0.0, 2.0)],
        2.0,
    );
    let wall = fx.body.get_face(fx.side_faces[1]).unwrap(); // (4,0) → (3,1)
    let Some(geom::Surface::Plane { normal, .. }) = fx.body.get_surface(wall.surface) else {
        panic!("a prism wall carries a plane");
    };
    let (n, minus_n) = (*normal, *normal * -1.0);
    assert!(
        n.x.abs() > 0.1 && n.y.abs() > 0.1,
        "the wall must be oblique for this row to mean anything, got {n:?}"
    );
    // Points in that wall's plane: the segment runs (4,0,z) → (3,1,z).
    for (s, z, expect) in [
        (0.5, 1.0, LoopContainment::In),
        (0.5, 0.0, LoopContainment::OnBoundary),
        (0.5, 3.0, LoopContainment::Out),
        (-1.0, 1.0, LoopContainment::Out),
    ] {
        let q = Point3::new(4.0 - s, s, z);
        let up = point_in_loop(&fx.body, wall.outer, n, q, band).unwrap();
        let down = point_in_loop(&fx.body, wall.outer, minus_n, q, band).unwrap();
        assert_eq!(up, expect, "the tilted row must decide as stated at {q:?}");
        assert_eq!(
            up, down,
            "the wall normal's sign moved the verdict at {q:?}"
        );
    }

    // The typed arm: all-graze exhaustion must exhaust on both signs.
    let fx = prism::<f64>(&all_rays_graze_profile(), 1.0);
    let top = fx.body.get_face(fx.top_face).unwrap();
    let centre = Point3::new(0.0, 0.0, 1.0);
    let up = point_in_loop(&fx.body, top.outer, n_z(), centre, band);
    let down = point_in_loop(&fx.body, top.outer, flipped, centre, band);
    assert!(
        matches!(up, Err(PointInLoopError::RayExhausted { .. })),
        "got {up:?}"
    );
    assert_eq!(shape(&up), shape(&down));
}
