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

use super::cone_islands::direction;
use super::*;
use geom_core::{Band, Point3, Tol, Vec3};
use test_utils::fuzz::{Rng, pinned};

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
        let z = direction(rng);
        let x0 = direction(rng).cross(z).normalize();
        let q = [x0, z.cross(x0), z];
        let o = Point3::origin() + direction(rng) * off;
        let (phi, beta) = (0.7f64, 1.3);
        let jitter = (direction(rng), jitter);
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
    let mut rng = pinned("a graze at a smooth vertex", 0x2545f4914f6cdd1d);
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
    let mut rng = pinned("a graze at a smooth vertex", 0x2545f4914f6cdd1d);
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

/// A cell of a per-arm vertex grid: the parities it decided right and
/// wrong, the first wrong one, and the cases its scale cannot resolve.
#[derive(Default)]
struct Cell {
    decided: usize,
    wrong: usize,
    unresolvable: usize,
    first: Option<String>,
}

impl Cell {
    /// One reading of an odd crossing.
    fn read(&mut self, got: Result<Option<bool>, Indeterminate>, tag: impl FnOnce() -> String) {
        match got {
            Ok(Some(true)) => self.decided += 1,
            Ok(Some(false)) => {
                self.wrong += 1;
                self.first.get_or_insert_with(tag);
            }
            _ => {}
        }
    }
}

/// Whether coordinates `size` metres from the origin resolve a band of
/// `eps`: their ulp is far below it. A case they do not resolve is
/// counted, not read.
fn resolvable(eps: f64, size: f64) -> bool {
    eps >= 64.0 * f64::EPSILON * size
}

/// **A per-arm vertex grid**: `cell` read at ε 1e-6, 1e-9 and 1e-12,
/// scales 1e-3, 1 and 1e3, offsets 0 and 1e3, and jitters none, 4 ulp
/// of the coordinates and ε/2. No decided parity is wrong anywhere; the
/// parities decided at each ε are returned, and some decide at one ε at
/// least (an extreme fixture may be wholly in the band at the finest).
fn vertex_grid(
    what: &str,
    rng: &mut Rng,
    mut cell: impl FnMut(Band, f64, f64, f64, &mut Rng, &mut Cell),
) -> [usize; 3] {
    let mut out = [0; 3];
    for (k, eps) in [1e-6, 1e-9, 1e-12].into_iter().enumerate() {
        let band = Band::linear_at(Tol::witness(), eps).unwrap();
        let mut decided = 0;
        for s in [1e-3, 1.0, 1e3] {
            for off in [0.0, 1e3] {
                for jitter in [0.0, 4.0 * f64::EPSILON * (off + s), 0.5 * eps] {
                    let mut c = Cell::default();
                    cell(band, s, off, jitter, rng, &mut c);
                    decided += c.decided;
                    assert!(
                        c.wrong == 0,
                        "{what}, ε {eps:e}, scale {s:e}, offset {off:e}, jitter {jitter:e}: {} \
                         decided-wrong parities ({} right); first {:?}",
                        c.wrong,
                        c.decided,
                        c.first
                    );
                }
            }
        }
        out[k] = decided;
    }
    assert!(out.iter().any(|&d| d > 0), "{what}: nothing decided");
    out
}

/// The sum of [`vertex_grid`]'s counts over a row's fixtures: some
/// decide at every ε.
fn decided_at_every_eps(what: &str, counts: impl IntoIterator<Item = [usize; 3]>) {
    let total = counts
        .into_iter()
        .fold([0; 3], |a, c| [a[0] + c[0], a[1] + c[1], a[2] + c[2]]);
    assert!(
        total.iter().all(|&d| d > 0),
        "{what}: decided at ε 1e-6, 1e-9, 1e-12: {total:?}"
    );
}

/// A rotation drawn from `rng`, as its three image axes.
fn frame(rng: &mut Rng) -> [Vec3<f64>; 3] {
    let z = direction(rng);
    let x = direction(rng).cross(z).normalize();
    [x, z.cross(x), z]
}

/// **A ruling through the smooth vertex of a cone's section reads it
/// once.** A section of a cone of half-angle `α`, its plane tilted `η`
/// short of a ruling's direction, split at a vertex into two half
/// ellipses, the second's carrier jittered; a ruling segment straddling
/// it through the vertex or a nudge off it, at the vertex farthest from
/// the apex and two others. It crosses the curve once, so a decided
/// parity is odd. The segment crosses the section's plane at a slope
/// that falls with `η` (a plane parallel to a ruling cuts a parabola):
/// near-parabolic sections graze, so the in-span reading must be levered
/// by that slope, and by the slope per metre of segment, which the
/// scales from 1e-3 to 1e3 put far from 1.
#[test]
fn a_ruling_through_a_sections_vertex_reads_it_once() {
    let sec_band = Band::linear_at(Tol::witness(), 1e-12).unwrap();
    let mut rng = pinned("a ruling through a vertex", 0x0f1e2d3c4b5a6978);
    let mut counts = Vec::new();
    for alpha_deg in [0.5f64, 30.0, 85.0] {
        // `η` short of the ruling, and inside the cone's opening (past
        // the far ruling the plane cuts a hyperbola).
        for eta in [0.5, 1e-2, 1e-4, 1e-6] {
            let alpha = alpha_deg.to_radians();
            if alpha + eta >= core::f64::consts::FRAC_PI_2 {
                continue;
            }
            counts.push(vertex_grid(
                &format!("α {alpha_deg}°, η {eta:e}"),
                &mut rng,
                |band, s, off, jitter, rng, cell| {
                    segment_section(band, sec_band, (alpha, eta), (s, off, jitter), rng, cell);
                },
            ));
        }
    }
    decided_at_every_eps("a ruling through a section's vertex", counts);
}

/// The section of the cone at `apex` about `q[2]` of half-angle `alpha`
/// by the plane through the axis point at height `h0` whose normal is
/// `theta` off the axis; `None` where the section lane refuses it.
fn steep_section(
    (apex, q): (Point3<f64>, [Vec3<f64>; 3]),
    alpha: f64,
    h0: f64,
    theta: f64,
    band: Band,
) -> Option<geom::Curve3<f64>> {
    let w = |v: Vec3<f64>| q[0] * v.x + q[1] * v.y + q[2] * v.z;
    let cone = geom::Surface::Cone {
        apex,
        axis: q[2],
        half_angle: alpha,
        u_ref: q[0],
    };
    let n = w(Vec3::new(theta.sin(), 0.0, theta.cos()));
    let o = apex + q[2] * h0;
    let reach = 8.0 * h0
        / (core::f64::consts::FRAC_PI_2 - alpha - theta)
            .sin()
            .max(1e-300);
    match geom_brep::plane_cone_section(
        &super::cone_islands::plane(o, n),
        &cone,
        reach.min(1e12),
        band,
    ) {
        Ok(geom_brep::PlaneConeSection::TiltedEllipse(c)) => Some(c),
        _ => None,
    }
}

/// The ellipse `c` split at parameter `v` into two half ellipses, the
/// second's carrier moved `jitter` along `dir` and turned by as much
/// over `size`.
fn split_ellipse(
    c: &geom::Curve3<f64>,
    v: f64,
    (dir, jitter): (Vec3<f64>, f64),
    size: f64,
) -> [LoopArc<f64>; 2] {
    let geom::Curve3::Ellipse {
        center,
        axis,
        major,
        minor,
        u_ref,
    } = *c
    else {
        panic!("a tilted section is an ellipse")
    };
    let pi = core::f64::consts::PI;
    [
        LoopArc::Conic {
            centre: center,
            axis,
            u_ref,
            a: major,
            b: minor,
            span: (v - pi, v),
        },
        LoopArc::Conic {
            centre: center + dir * jitter,
            axis: (axis + dir.cross(axis) * (jitter / size)).normalize(),
            u_ref,
            a: major,
            b: minor,
            span: (v, v + pi),
        },
    ]
}

/// One cell of [`a_ruling_through_a_sections_vertex_reads_it_once`].
fn segment_section(
    band: Band,
    sec_band: Band,
    (alpha, eta): (f64, f64),
    (s, off, jitter): (f64, f64, f64),
    rng: &mut Rng,
    cell: &mut Cell,
) {
    let q = frame(rng);
    let apex = Point3::origin() + direction(rng) * off;
    let theta = core::f64::consts::FRAC_PI_2 - alpha - eta;
    let Some(c) = steep_section((apex, q), alpha, s, theta, sec_band) else {
        // The section lane refuses it (a reach past its bound): no curve.
        cell.unresolvable += 1;
        return;
    };
    let geom::Curve3::Ellipse { major, .. } = c else {
        unreachable!()
    };
    if !resolvable(band.zero(), off + (c.eval(0.0) - apex).norm() + 2.0 * major) {
        cell.unresolvable += 1;
        return;
    }
    let far = (0..4096)
        .map(|i| f64::from(i) * core::f64::consts::TAU / 4096.0)
        .max_by(|x, y| {
            (c.eval(*x) - apex)
                .norm()
                .total_cmp(&(c.eval(*y) - apex).norm())
        })
        .unwrap();
    for tv in [far, far + 0.3, far + 2.0] {
        for dir in [direction(rng), q[2]] {
            let arcs = split_ellipse(&c, tv, (dir, jitter), major);
            for nudge in nudges() {
                let x = c.eval(tv + nudge);
                let e = (x - apex).normalize();
                let slant = (x - apex).norm();
                let (from, to) = (apex + e * (slant * 0.7), apex + e * (slant * 1.3));
                let path = Path {
                    pieces: vec![Piece::Segment {
                        from,
                        to,
                        ruling: e,
                        lever: slant * 0.7,
                    }],
                    arrival: e,
                };
                cell.read(path_parity(&path, &arcs, None, band), || {
                    format!("vertex {} from the farthest, nudge {nudge:e}", tv - far)
                });
            }
        }
    }
}

/// **A parallel through the vertex of a split ruling reads it once.** A
/// ruling of a cone of half-angle `α` split at a vertex into two
/// segments, the second's jittered; the arc of a parallel across it,
/// through the vertex or a nudge off it. It crosses the ruling once, so
/// a decided parity is odd. The ruling crosses the parallel's plane at
/// slope `cos α`, which falls to nothing on a near-flat cone: there the
/// arm grazes, so the in-span reading must be levered by that slope.
#[test]
fn a_parallel_through_a_split_rulings_vertex_reads_it_once() {
    let mut rng = pinned("a parallel through a vertex", 0x1234fedc5678ba90);
    let mut counts = Vec::new();
    for alpha_deg in [1e-4f64, 0.1, 30.0, 80.0, 89.0, 89.9, 89.999, 89.99999] {
        let alpha = alpha_deg.to_radians();
        counts.push(vertex_grid(
            &format!("α {alpha_deg}°"),
            &mut rng,
            |band, s, off, jitter, rng, cell| {
                parallel_ruling(band, alpha, (s, off, jitter), rng, cell);
            },
        ));
    }
    decided_at_every_eps("a parallel through a split ruling's vertex", counts);
}

/// One cell of [`a_parallel_through_a_split_rulings_vertex_reads_it_once`].
fn parallel_ruling(
    band: Band,
    alpha: f64,
    (s, off, jitter): (f64, f64, f64),
    rng: &mut Rng,
    cell: &mut Cell,
) {
    let q = frame(rng);
    let w = |v: Vec3<f64>| q[0] * v.x + q[1] * v.y + q[2] * v.z;
    let apex = Point3::origin() + direction(rng) * off;
    let z = q[2];
    let e = w(Vec3::new(alpha.sin(), 0.0, alpha.cos()));
    let l = s / alpha.sin();
    if !resolvable(band.zero(), off + 2.0 * l) {
        cell.unresolvable += 1;
        return;
    }
    for dir in [direction(rng), z, -z] {
        let arcs = [
            LoopArc::Line {
                origin: apex,
                dir: e,
                span: (l - s, l),
            },
            LoopArc::Line {
                origin: apex + dir * jitter,
                dir: (e + dir.cross(e) * (jitter / l)).normalize(),
                span: (l, l + s),
            },
        ];
        for nudge in nudges() {
            let ln = l + nudge * s;
            let piece = CircleArc {
                centre: apex + z * (ln * alpha.cos()),
                axis: z,
                radius: ln * alpha.sin(),
                u_ref: w(Vec3::new((-0.3f64).cos(), (-0.3f64).sin(), 0.0)),
                span: (0.0, 0.6),
            };
            let path = Path {
                pieces: vec![Piece::Arc(piece)],
                arrival: piece.tangent(0.6),
            };
            cell.read(path_parity(&path, &arcs, None, band), || {
                format!("nudge {nudge:e}")
            });
        }
    }
}

/// **A crossing decided out of span is passed over, though another of
/// its readings escalates.** A ruling crosses a parallel's plane a hair
/// past the arc's end — its piece-side in-span reading in the escalation
/// band — at a point of the ruling a decided half-metre short of the
/// ruling's span: the crossing is not one, and the parity is even. The
/// same ruling with the crossing inside its span says nothing (the
/// piece-side reading is all there is), so the path escalates.
#[test]
fn a_crossing_decided_out_of_span_is_passed_over_though_another_reading_escalates() {
    let band = super::cone_islands::band();
    let end = 1.0f64;
    let piece = CircleArc {
        centre: Point3::origin(),
        axis: Vec3::unit_z(),
        radius: 1.0,
        u_ref: Vec3::unit_x(),
        span: (0.0, end),
    };
    let path = Path {
        pieces: vec![Piece::Arc(piece)],
        arrival: piece.tangent(end),
    };
    // Past the end by the band's geometric middle: zero < δ < escalate.
    let delta = (band.zero() * band.escalate()).sqrt();
    let at = piece.at(end + delta);
    let ruling = |span| LoopArc::Line {
        origin: at - Vec3::unit_z() * 0.5,
        dir: Vec3::unit_z(),
        span,
    };
    assert!(
        path_parity(&path, &[ruling((0.0, 1.0))], None, band).is_err(),
        "a crossing at the arc's end, inside the ruling's span, escalates"
    );
    assert_eq!(
        path_parity(&path, &[ruling((1.0, 2.0))], None, band).unwrap(),
        Some(false),
        "a crossing decided outside the ruling's span is passed over"
    );
}
