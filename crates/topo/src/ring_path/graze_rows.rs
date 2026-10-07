//! **A path grazing a loop arc's plane is decided on the gap and its
//! roots' in-span readings on the slope, and never decides a wrong
//! parity.** The fixture: a closed small circle on a
//! sphere, split into two arcs at a smooth vertex `V`, each arc carrying
//! the circle on its own `u_ref` (and, in the sweeps, its own carrier a
//! few ulp off the other's), and a great-circle path whose two ends lie
//! outside the circle's cap — so the true parity is even, whatever the
//! path does near the circle — which grazes the circle with one root at
//! or within its own error of `V`. A decided odd parity is a wrong one.
//! The draws are fixture identifiers, not a counterexample search.

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

/// The decided-odd parities of a graze whose root sits at the smooth
/// vertex, at `band`, scale `r` and offset `off`, under carrier jitter of
/// `jitter` metres, and how many were decided at all. The graze is sized
/// from the band (a gap 1.01 to 10⁶ times its escalation threshold, the
/// meeting decided), and one root is put at the vertex and then nudged
/// by `±m·10ᵏ` of the half-chord, down to below a root's own error.
fn wrong_at_the_vertex(
    band: Band,
    r: f64,
    off: f64,
    jitter: f64,
    rng: &mut Rng,
) -> (usize, usize, Option<String>) {
    let (mut wrong, mut decided, mut first) = (0, 0, None);
    for _ in 0..6 {
        let z = rng.unit();
        let x0 = rng.unit().cross(z).normalize();
        let q = [x0, z.cross(x0), z];
        let o = Point3::origin() + rng.unit() * off;
        let (phi, beta) = (0.7f64, 1.3);
        let jitter = (rng.unit(), jitter);
        for gk in [1.01, 1.5, 3.0, 10.0, 100.0, 1e4, 1e6] {
            let gap = gk * band.escalate();
            let h = (2.0 * gap * r / phi.cos()).sqrt();
            if h > 0.3 * r {
                continue;
            }
            for sgn in [1.0, -1.0] {
                for k in -16..=0 {
                    for m in [1.0, 3.0] {
                        for ss in [1.0, -1.0] {
                            let e = sgn * h + ss * m * 10f64.powi(k) * h;
                            let (path, arcs) = case(q, o, r, phi, beta, -gap, e, jitter);
                            match path_parity(&path, &arcs, None, band) {
                                Ok(Some(true)) => {
                                    wrong += 1;
                                    first.get_or_insert(format!("gap/esc {gk}, e/h {}", e / h));
                                }
                                Ok(Some(false)) => decided += 1,
                                _ => {}
                            }
                        }
                    }
                }
            }
        }
    }
    (wrong, decided, first)
}

/// At this run's band, scale 1, no offset, under no jitter, 4-ulp jitter
/// and ε/2 jitter: no decided parity is odd, and some decide.
#[test]
fn a_graze_at_a_smooth_vertex_never_decides_a_wrong_parity() {
    let band = super::cone_islands::band();
    let mut rng = Rng(0x2545f4914f6cdd1d);
    for jitter in [0.0, 4.0 * f64::EPSILON, 0.5 * band.zero()] {
        let (wrong, decided, first) = wrong_at_the_vertex(band, 1.0, 0.0, jitter, &mut rng);
        assert!(
            wrong == 0 && decided > 0,
            "jitter {jitter:e}: {wrong} decided-wrong parities ({decided} right); first {first:?}"
        );
    }
}

/// The same at ε 1e-6, 1e-9 and 1e-12 (the run's K), scales 1e-3, 1 and
/// 1e3, offsets 0 and 1e3, and each jitter: no decided parity is odd
/// anywhere, and some decide at every ε.
#[test]
fn a_graze_at_a_smooth_vertex_never_decides_a_wrong_parity_at_any_scale() {
    let mut rng = Rng(0x2545f4914f6cdd1d);
    for eps in [1e-6, 1e-9, 1e-12] {
        let band = Band::linear_at(Tol::witness(), eps).unwrap();
        let mut decided_here = 0;
        for r in [1e-3, 1.0, 1e3] {
            for off in [0.0, 1e3] {
                for jitter in [0.0, 4.0 * f64::EPSILON * (off + r), 0.5 * eps] {
                    let (wrong, decided, first) =
                        wrong_at_the_vertex(band, r, off, jitter, &mut rng);
                    decided_here += decided;
                    assert!(
                        wrong == 0,
                        "ε {eps:e}, r {r:e}, offset {off:e}, jitter {jitter:e}: {wrong} \
                         decided-wrong parities ({decided} right); first {first:?}"
                    );
                }
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

/// The nudges a root is moved off a shared vertex by, relative to a unit
/// of length along the curve: `±m·10ᵏ`, from a tenth down to below an
/// ulp, and zero.
fn nudges() -> Vec<f64> {
    let mut out = vec![0.0];
    for k in -16..=-1 {
        for m in [1.0, 3.0] {
            out.extend([m * 10f64.powi(k), -m * 10f64.powi(k)]);
        }
    }
    out
}

/// The jitters a second arc's carrier is moved by: none, 4 ulp, and ε/2.
fn jitters(band: Band) -> [f64; 3] {
    [0.0, 4.0 * f64::EPSILON, 0.5 * band.zero()]
}

/// **A ruling through the smooth vertex of a cone's section reads it
/// once.** A tilted ellipse of the cone split at a vertex into two arcs,
/// the second's carrier jittered; a ruling segment straddling the
/// ellipse, through the vertex or a nudge off it. It crosses the curve
/// once, so a decided parity is odd. The segment's slope across the
/// ellipse's plane is bounded below on a cone (a plane parallel to a
/// ruling cuts a parabola, which no lane mints), so this arm never
/// grazes.
#[test]
fn a_ruling_through_a_sections_vertex_reads_it_once() {
    use super::cone_islands::{cone, frames, section};
    let band = super::cone_islands::band();
    let f = frames()[0];
    let surface = cone(f, false);
    let n = Vec3::new(40f64.to_radians().sin(), 0.0, 40f64.to_radians().cos());
    let ellipse = section(&surface, (Point3::new(0.5, 0.0, 1.5), n), 1.0);
    let geom::Curve3::Ellipse {
        center,
        axis,
        major,
        minor,
        u_ref,
    } = ellipse
    else {
        panic!("a tilted section is an ellipse")
    };
    let tv = 0.4;
    let mut rng = Rng(0x0f1e2d3c4b5a6978);
    let (mut decided, mut wrong) = (0, Vec::new());
    for jitter in jitters(band) {
        let dir = rng.unit();
        let arcs = [
            LoopArc::Conic {
                centre: center,
                axis,
                u_ref,
                a: major,
                b: minor,
                span: (tv - 1.0, tv),
            },
            LoopArc::Conic {
                centre: center + dir * jitter,
                axis: (axis + dir.cross(axis) * jitter).normalize(),
                u_ref,
                a: major,
                b: minor,
                span: (tv, tv + 1.0),
            },
        ];
        for nudge in nudges() {
            let x = ellipse.eval(tv + nudge);
            let e = (x - Point3::origin()).normalize();
            let slant = (x - Point3::origin()).norm();
            let (from, to) = (
                Point3::origin() + e * (slant - 0.3),
                Point3::origin() + e * (slant + 0.3),
            );
            let path = Path {
                pieces: vec![Piece::Segment {
                    from,
                    to,
                    ruling: e,
                    lever: slant,
                }],
                arrival: e,
            };
            match path_parity(&path, &arcs, None, band) {
                Ok(Some(true)) => decided += 1,
                Ok(Some(false)) => wrong.push((jitter, nudge)),
                _ => {}
            }
        }
    }
    assert!(
        wrong.is_empty() && decided > 0,
        "{decided} right, wrong {wrong:?}"
    );
}

/// **A parallel through the vertex of a split ruling reads it once.** A
/// ruling of the cone split at a vertex into two segments, the second's
/// jittered; the arc of a parallel across it, through the vertex or a
/// nudge off it. It crosses the ruling once, so a decided parity is odd.
/// A ruling's slope across a parallel's plane is the cosine of the
/// half-angle, so this arm never grazes either.
#[test]
fn a_parallel_through_a_split_rulings_vertex_reads_it_once() {
    let band = super::cone_islands::band();
    let tan = core::f64::consts::FRAC_PI_6.tan();
    let e = Vec3::new(tan, 0.0, 1.0).normalize();
    let slant_v = 1.5 / e.z;
    let mut rng = Rng(0x1234fedc5678ba90);
    let (mut decided, mut wrong) = (0, Vec::new());
    for jitter in jitters(band) {
        let dir = rng.unit();
        let arcs = [
            LoopArc::Line {
                origin: Point3::origin(),
                dir: e,
                span: (slant_v - 1.0, slant_v),
            },
            LoopArc::Line {
                origin: Point3::origin() + dir * jitter,
                dir: (e + dir.cross(e) * jitter).normalize(),
                span: (slant_v, slant_v + 1.0),
            },
        ];
        for nudge in nudges() {
            let h = 1.5 + nudge;
            let piece = CircleArc {
                centre: Point3::new(0.0, 0.0, h),
                axis: Vec3::unit_z(),
                radius: h * tan,
                u_ref: Vec3::new((-0.3f64).cos(), (-0.3f64).sin(), 0.0),
                span: (0.0, 0.6),
            };
            let path = Path {
                pieces: vec![Piece::Arc(piece)],
                arrival: piece.tangent(0.6),
            };
            match path_parity(&path, &arcs, None, band) {
                Ok(Some(true)) => decided += 1,
                Ok(Some(false)) => wrong.push((jitter, nudge)),
                _ => {}
            }
        }
    }
    assert!(
        wrong.is_empty() && decided > 0,
        "{decided} right, wrong {wrong:?}"
    );
}
