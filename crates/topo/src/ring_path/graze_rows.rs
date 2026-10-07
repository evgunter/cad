//! **A path grazing a loop arc's plane is decided on the gap, and never
//! decides a wrong parity.** The fixture: a closed small circle on a
//! sphere, split into two arcs at a smooth vertex `V`, each arc carrying
//! the circle on its own `u_ref` (and, in the sweeps, its own carrier a
//! few ulp off the other's), and a great-circle path whose two ends lie
//! outside the circle's cap — so the true parity is even, whatever the
//! path does near the circle — which grazes the circle within `e` of
//! `V`, missing it by `g`. A decided odd parity is a wrong one. The
//! draws are fixture identifiers, not a counterexample search.

use super::*;
use geom_core::{Band, Point3, Tol, Vec3};

struct Rng(u64);
impl Rng {
    fn next(&mut self) -> f64 {
        self.0 ^= self.0 << 13;
        self.0 ^= self.0 >> 7;
        self.0 ^= self.0 << 17;
        (self.0 >> 11) as f64 / (1u64 << 53) as f64
    }
    fn range(&mut self, a: f64, b: f64) -> f64 {
        a + (b - a) * self.next()
    }
    fn unit(&mut self) -> Vec3<f64> {
        loop {
            let v = Vec3::new(
                self.range(-1., 1.),
                self.range(-1., 1.),
                self.range(-1., 1.),
            );
            let n = v.norm();
            if n > 0.2 && n < 1.0 {
                return v / n;
            }
        }
    }
}

#[allow(clippy::too_many_arguments)]
fn case(
    q: [Vec3<f64>; 3],
    o: Point3<f64>,
    r: f64,
    phi: f64,
    beta: f64,
    g: f64,
    e: f64,
    jitter: (Vec3<f64>, f64),
) -> (Path<f64>, Vec<LoopArc<f64>>) {
    let w = |v: Vec3<f64>| q[0] * v.x + q[1] * v.y + q[2] * v.z;
    let at = |v: Vec3<f64>| o + w(v);
    let m = Vec3::new(-phi.sin(), 0.0, phi.cos());
    let t_hat = Vec3::new(phi.cos(), 0.0, phi.sin());
    let mt = m.cross(t_hat);
    let u0 = t_hat * 0.7f64.cos() - mt * 0.7f64.sin();
    let piece = CircleArc {
        centre: o,
        axis: w(m),
        radius: r,
        u_ref: w(u0),
        span: (0.0, 1.4),
    };
    let path = Path {
        pieces: vec![Piece::Arc(piece)],
        arrival: piece.tangent(1.4),
    };
    let c = r * phi.cos() + g;
    let a = (r * r - c * c).sqrt();
    let centre = at(Vec3::new(c, 0.0, 0.0));
    let x = w(Vec3::unit_x());
    let sv = core::f64::consts::FRAC_PI_2 + e / a;
    let u1 = w(Vec3::unit_y());
    let u2 = w(Vec3::new(0.0, beta.cos(), beta.sin()));
    let pi = core::f64::consts::PI;
    let arcs = vec![
        LoopArc::Conic {
            centre,
            axis: x,
            u_ref: u1,
            a,
            b: a,
            span: (sv - pi, sv),
        },
        LoopArc::Conic {
            centre: centre + jitter.0 * jitter.1,
            axis: (x + jitter.0.cross(x) * (jitter.1 / r)).normalize(),
            u_ref: u2,
            a,
            b: a,
            span: (sv - beta, sv - beta + pi),
        },
    ];
    (path, arcs)
}

/// The grazes and vertex offsets every sweep reads, relative to `r`: the
/// grazes run from inside every band to outside it, where the meeting is
/// decided and the parity must read even.
fn grazes() -> (Vec<f64>, Vec<f64>) {
    let mut gs = vec![0.0];
    for k in [
        1e-16, 1e-15, 1e-14, 1e-13, 1e-12, 1e-11, 1e-10, 1e-8, 1e-6, 1e-4,
    ] {
        gs.extend([k, -k]);
    }
    let mut es = Vec::new();
    for k in [1e-10, 3e-10, 1e-9, 3e-9, 1e-8, 3e-8, 1e-7, 3e-7, 1e-6, 1e-5] {
        es.extend([k, -k]);
    }
    (gs, es)
}

/// The decided-odd parities of the graze sweep at `band`, scale `r` and
/// offset `off`, and how many were decided at all.
fn wrong_grazes(band: Band, r: f64, off: f64) -> (Vec<String>, usize) {
    let (gs, es) = grazes();
    let mut rng = Rng(0x9e3779b97f4a7c15);
    let (mut wrong, mut decided) = (Vec::new(), 0);
    for _ in 0..30 {
        let z = rng.unit();
        let x0 = rng.unit().cross(z).normalize();
        let q = [x0, z.cross(x0), z];
        let o = Point3::origin() + rng.unit() * off;
        let phi = rng.range(0.3, 1.2);
        let beta = rng.range(0.3, 2.5);
        let jitter = (rng.unit(), 4.0 * f64::EPSILON * (off + r));
        for &g in &gs {
            for &e in &es {
                let (path, arcs) = case(q, o, r, phi, beta, g * r, e * r, jitter);
                match path_parity(&path, &arcs, None, band) {
                    Ok(Some(true)) => {
                        wrong.push(format!("phi {phi} beta {beta} g/r {g:e} e/r {e:e}"))
                    }
                    Ok(Some(false)) => decided += 1,
                    _ => {}
                }
            }
        }
    }
    (wrong, decided)
}

/// At this run's band, scale 1, no offset: no decided parity is odd.
#[test]
fn a_graze_at_a_smooth_vertex_never_decides_a_wrong_parity() {
    let (wrong, decided) = wrong_grazes(super::cone_islands::band(), 1.0, 0.0);
    assert!(
        wrong.is_empty() && decided > 0,
        "{} decided-wrong parities ({decided} decided right); first {:?}",
        wrong.len(),
        wrong.first()
    );
}

/// The same at ε 1e-6, 1e-9 and 1e-12 (the run's K), scales 1e-3, 1 and
/// 1e3, and offsets 0 and 1e3: no decided parity is odd anywhere.
#[test]
fn a_graze_at_a_smooth_vertex_never_decides_a_wrong_parity_at_any_scale() {
    for eps in [1e-6, 1e-9, 1e-12] {
        let band = Band::linear_at(Tol::witness(), eps).unwrap();
        let mut decided_here = 0;
        for r in [1e-3, 1.0, 1e3] {
            for off in [0.0, 1e3] {
                let (wrong, decided) = wrong_grazes(band, r, off);
                decided_here += decided;
                assert!(
                    wrong.is_empty(),
                    "ε {eps:e}, r {r:e}, offset {off:e}: {} decided-wrong parities ({decided} \
                     right); first {:?}",
                    wrong.len(),
                    wrong.first()
                );
            }
        }
        assert!(decided_here > 0, "ε {eps:e}: no graze decided");
    }
}

/// **A graze within the gap's band escalates on its path, and the next
/// path decides.** The path misses the circle's plane by a gap inside
/// the band's escalation zone: its reading escalates, saying nothing.
/// A path between the same ends round the far side of the sphere,
/// through the point opposite the circle's centre, clears the circle
/// and reads the even parity.
#[test]
fn a_graze_within_the_gaps_band_escalates_and_the_next_path_decides() {
    let band = super::cone_islands::band();
    let gap = (band.zero() * band.escalate()).sqrt();
    let q = [Vec3::unit_x(), Vec3::unit_y(), Vec3::unit_z()];
    let (o, r, phi, beta) = (Point3::origin(), 1.0, 0.8, 1.3);
    // `g` is the circle plane's offset past the path's reach: a gap of
    // `−g`.
    let (graze, arcs) = case(q, o, r, phi, beta, -gap, 0.3, (Vec3::unit_x(), 0.0));
    let got = path_parity(&graze, &arcs, None, band);
    assert!(
        matches!(&got, Err(diag) if diag.predicate == Some("split_ring_path_meets_plane")),
        "the graze escalates on the gap: {got:?}"
    );
    let Piece::Arc(piece) = graze.pieces[0] else {
        unreachable!("a sphere path is one arc")
    };
    let (start, end) = (piece.at(piece.span.0), piece.at(piece.span.1));
    let far = o - Vec3::unit_x() * r;
    let sphere = Quadric::Sphere {
        centre: o,
        radius: r,
    };
    let leg = |a, b| sphere.paths((a, b), band).unwrap().remove(0);
    let (out, back) = (leg(start, far), leg(far, end));
    let detour = Path {
        pieces: out.pieces.into_iter().chain(back.pieces).collect(),
        arrival: back.arrival,
    };
    assert_eq!(path_parity(&detour, &arcs, None, band), Ok(Some(false)));
}
