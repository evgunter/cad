//! **The line-clearance fuzz**: a line segment against a wall or a
//! sphere with both its endpoint residuals positive, where no endpoint
//! datum bounds the residual between them (it is convex along the line,
//! so it can dip through zero and come back, the BELLY crossing). The
//! crossing reading ([`super::cross()`]) is held to the closed-form
//! quadratic, solved in double-double on the stored data: two roots
//! inside the span answer two crossings; none, or roots past the span,
//! answer none; a graze or a root at an end in the band may refuse,
//! never answer wrong.

use geom::Curve3;
use geom_core::{Band, Point3, Tol, Vec3};

use super::conic_oracle::exact::Dd;
use super::conic_oracle::unit;
use super::{MeetingKind, OnSpan, cross};
use crate::curve_pair::Span;

fn band() -> Band {
    Band::linear(Tol::witness()).unwrap_or_else(|e| panic!("the witness band: {e:?}"))
}

/// `a · b` of two double-double vectors.
fn dot(a: [Dd; 3], b: [Dd; 3]) -> Dd {
    a[0].mul(b[0]).add(a[1].mul(b[1])).add(a[2].mul(b[2]))
}

fn dd(v: Vec3<f64>) -> [Dd; 3] {
    [Dd::of(v.x), Dd::of(v.y), Dd::of(v.z)]
}

/// `o − p`, exactly, as a double-double vector.
fn diff(o: Point3<f64>, p: Point3<f64>) -> [Dd; 3] {
    [
        Dd::of(o.x).sub(Dd::of(p.x)),
        Dd::of(o.y).sub(Dd::of(p.y)),
        Dd::of(o.z).sub(Dd::of(p.z)),
    ]
}

/// `v` less its part along the unit `n`.
fn across(v: [Dd; 3], n: [Dd; 3]) -> [Dd; 3] {
    let along = dot(v, n);
    [0, 1, 2].map(|i| v[i].sub(along.mul(n[i])))
}

/// The truth of the line `o + t·d` against the surface: the
/// quadratic `qa t² + qb t + qc` whose sign is the residual's, its real
/// roots (none where its discriminant is negative), and the residual at
/// its vertex in metres (`(|w⊥|² − r²)/2r` there).
struct Quadratic {
    roots: Option<(f64, f64)>,
    vertex_depth: f64,
}

fn quadratic(o: Point3<f64>, d: Vec3<f64>, s: &geom::Surface<f64>) -> Quadratic {
    let (w, dir, r) = match *s {
        geom::Surface::Sphere { frame, radius, .. } => (diff(o, frame.origin()), dd(d), radius),
        geom::Surface::Cylinder { frame, radius, .. } => {
            let n = dd(frame.axis());
            (across(diff(o, frame.origin()), n), across(dd(d), n), radius)
        }
        _ => unreachable!("the fuzz reads walls and spheres"),
    };
    let qa = dot(dir, dir);
    let qb = dot(dir, w).mul(Dd::of(2.0));
    let qc = dot(w, w).sub(Dd::of(r).mul(Dd::of(r)));
    let disc = qb.mul(qb).sub(Dd::of(4.0).mul(qa).mul(qc));
    let (qa, qb, disc_f) = (qa.to_f64(), qb.to_f64(), disc.to_f64());
    let vertex_depth = -disc_f / (4.0 * qa) / (2.0 * r);
    let roots = (disc_f >= 0.0).then(|| {
        let root = disc.sqrt().to_f64();
        ((-qb - root) / (2.0 * qa), (-qb + root) / (2.0 * qa))
    });
    Quadratic {
        roots,
        vertex_depth,
    }
}

/// **The line-clearance fuzz** (the retired `bool_line_cylinder_clearance`'s
/// pairs, read through the pose's discriminant): a segment past a wall
/// or a sphere at both ends, its closest approach to the axis or the
/// centre `r + δ` with `δ` log-uniform across the band both ways, so the
/// residual dips inside between the ends (two crossings), grazes, or
/// clears. Wherever the double-double quadratic's vertex lies 10·`Kε`
/// past the band either way, with its roots 10·`Kε` clear of the span's
/// ends, `cross` answers its truth: two crossings strictly inside, or
/// none and no touch.
///
/// Mutant: the pose's discriminant read on the endpoint chord (the
/// decision taken on the start's residual, positive here), red: every
/// belly crossing reads `Apart`.
#[test]
fn a_segment_outside_at_both_ends_is_read_against_the_closed_form() {
    let b = band();
    let clear = 10.0 * b.escalate();
    let mut rng = test_utils::fuzz::start("line-clearance");
    let (mut bellies, mut clears) = (0, 0);
    for case in 0..test_utils::fuzz::scaled(3000) {
        let origin = Point3::new(
            rng.range(-2.0, 2.0),
            rng.range(-2.0, 2.0),
            rng.range(-2.0, 2.0),
        );
        let axis = unit(|lo, hi| rng.range(lo, hi));
        let frame = geom::Frame::new(origin, axis, axis.orthonormal_basis().0);
        let r = rng.range(0.3, 3.0);
        let wall = case % 2 == 0;
        let s = if wall {
            geom::Surface::Cylinder { frame, radius: r }
        } else {
            geom::Surface::Sphere { frame, radius: r }
        };
        let d = unit(|lo, hi| rng.range(lo, hi));
        // The closest approach, `r + δ` off the axis or the centre, square
        // to the line.
        let delta = rng.log_uniform(1e-13, 0.3) * if rng.unit() < 0.5 { -1.0 } else { 1.0 };
        let off = if wall {
            let dt = d - axis * d.dot(axis);
            if dt.norm() < 0.1 {
                continue;
            }
            axis.cross(dt).normalize()
        } else {
            unit(|lo, hi| rng.range(lo, hi)).cross(d).normalize()
        };
        let near =
            origin + axis * if wall { rng.range(-1.0, 1.0) } else { 0.0 } + off * (r + delta);
        let (before, after) = (rng.range(0.05, 3.0), rng.range(0.05, 3.0));
        let start = near - d * before;
        let line = Curve3::Line(geom::Line {
            origin: start,
            dir: geom::stated_direction(d),
        });
        let span = (0.0, before + after);
        let truth = quadratic(start, d, &s);
        // Both ends definitely outside, by the exact quadratic.
        let outside = |t: f64| {
            truth
                .roots
                .is_none_or(|(lo, hi)| t < lo - clear || t > hi + clear)
        };
        if !(outside(span.0) && outside(span.1)) {
            continue;
        }
        let got = cross(&Span::new(&line, span), &s, b);
        let inside = |t: f64| t > span.0 + clear && t < span.1 - clear;
        match truth.roots {
            Some((lo, hi)) if truth.vertex_depth < -clear && inside(lo) && inside(hi) => {
                bellies += 1;
                let Ok(found) = got else {
                    continue;
                };
                let crossings = found
                    .meetings
                    .iter()
                    .filter(|m| {
                        matches!(
                            (m.on_span, m.kind),
                            (OnSpan::Inside { .. }, MeetingKind::Cross)
                        )
                    })
                    .count();
                assert_eq!(
                    crossings,
                    2,
                    "case {case}: a belly crossing, roots {lo}, {hi} inside {span:?}, read {found:?} ({})",
                    test_utils::fuzz::replay()
                );
            }
            None if truth.vertex_depth > clear => {
                clears += 1;
                if let Ok(found) = got {
                    assert!(
                        found.meetings.is_empty(),
                        "case {case}: clear by {:e}, read {found:?} ({})",
                        truth.vertex_depth,
                        test_utils::fuzz::replay()
                    );
                }
            }
            _ => {}
        }
    }
    assert!(
        bellies > 100 && clears > 100,
        "{bellies} belly crossings, {clears} clear"
    );
}
