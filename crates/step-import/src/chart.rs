//! Outerness as an **interpretation act** (M7-2 Leg B), with its rule
//! stated and its ambiguity refused.
//!
//! # Why there is anything to infer
//!
//! The kernel's own writer marks a face's outer bound
//! `FACE_OUTER_BOUND` and its rings `FACE_BOUND`, so import reads the
//! answer. FreeCAD 1.1.2 never writes `FACE_OUTER_BOUND` at all (0
//! occurrences in the 13 measured files): a face's rings are
//! syntactically indistinguishable, and which one is outer is
//! **geometry**, not syntax. That measurement is this lane's whole
//! reason to exist, so it is re-read from the committed corpus on every
//! test run rather than kept as a dated observation (issue #667's Q6):
//! `tests/freecad.rs`'s `the_committed_freecad_corpus_still_says_what_chart_and_units_quote`
//! reads the committed corpus and goes red if the count moves or the
//! marker appears.
//!
//! # The rule
//!
//! - A face with a **single** bound: that bound is outer *by
//!   definition* — there is nothing for it to be inside of. (Stated,
//!   because it is a definition and not a measurement.)
//! - A face with several bounds: each ring is mapped into the face
//!   surface's own (u, v) chart by the closed-form inverse of the
//!   kernel's analytic parameterization ([`uv_of`] — the chart
//!   conventions are `geom::Surface`'s, read off its own
//!   `eval`), sampled into a chart polygon. The **outer** ring is the
//!   one whose polygon contains every other ring's; the signed
//!   (shoelace) area supplies the independent orientation signal, and
//!   the outer ring's sign must OPPOSE every inner ring's — a genuine
//!   hole reverses. Both signals must agree.
//!
//! # What refuses (D7: ambiguity is typed, never a guess)
//!
//! - **Seam dependence**: a ring on a periodic chart whose samples
//!   wrap (a hole encircling the axis, or a ring straddling the seam)
//!   has a chart image that depends on where the seam was cut. No
//!   containment answer read there is a fact about the surface, so it
//!   refuses.
//! - **Undecidability at ε**: a test point closer to a candidate
//!   polygon's boundary than the interpretation budget allows —
//!   ε_in_eff carried into the chart through the local metric
//!   (`|∂S/∂u|`, `|∂S/∂v|`), so the budget is compared against a
//!   parametric distance in the units the chart actually has.
//! - **No unique answer**: zero, or more than one, ring containing all
//!   the others.
//!
//! Where `FACE_OUTER_BOUND` IS present (the kernel's own dialect) it is
//! honored AND cross-checked against this inference; disagreement is a
//! typed error, not a preference (`entities`' face reader).

use geom::Surface;
use geom_core::{Point2, Point3, Vec3};

/// Why an outerness inference could not answer (module docs). Carries
/// no entity ids — the caller names the face.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum OuternessRefusal {
    /// A ring point does not invert into the face surface's chart (off
    /// the surface, or at a chart singularity where the inverse is not
    /// a function).
    NotOnChart,
    /// A ring's chart image is seam-dependent (module docs).
    SeamDependent,
    /// A containment test landed inside the interpretation budget.
    UndecidableAtEps,
    /// No ring, or more than one, contains all the others.
    NoUniqueOuter,
    /// Containment and orientation disagree: a ring contained in
    /// another turns the same way it does.
    OrientationContradicts,
    /// The chart is one the closed-form inverse does not cover: a
    /// NURBS face, which has no `uv_of` arm (M7-3 item 6 — the named
    /// frontier past the imported single-bound loft class; a
    /// multi-ring NURBS face refuses earlier still, at the
    /// curved-face ring gate in `entities::face`, so this refusal is
    /// the stage-1 banked backstop, not the common path).
    UnsupportedChart,
}

impl OuternessRefusal {
    /// The static description the typed import error carries.
    pub(crate) fn what(self) -> &'static str {
        match self {
            Self::NotOnChart => {
                "a face bound whose ring does not lie on the face's own surface chart \
                 — outerness cannot be inferred from a ring that is not there"
            }
            Self::SeamDependent => {
                "a face bound whose chart image depends on where the periodic seam was \
                 cut (a ring wrapping the surface's period): the containment answer \
                 would be an artifact of the seam, not a fact about the surface"
            }
            Self::UndecidableAtEps => {
                "a face whose ring containment is undecidable at the file's own \
                 interpretation budget (a ring passing within ε_in of another): \
                 refusing rather than guessing which bound is outer"
            }
            Self::NoUniqueOuter => {
                "a face with no single bound containing all the others — the STEP \
                 outer/inner reading has no unique answer here"
            }
            Self::OrientationContradicts => {
                "a face whose bound containment and bound orientation disagree (a \
                 contained ring turning the same way as its container): the two \
                 independent signals for outerness contradict"
            }
            Self::UnsupportedChart => {
                "a face on a surface whose chart has no closed-form inverse, so \
                 outerness cannot be inferred (state FACE_OUTER_BOUND instead)"
            }
        }
    }
}

/// The chart preimage of `p` on `surface` — the closed-form inverse of
/// `geom::Surface::eval`, kind by kind. `None` for a chart
/// with no closed form (NURBS) or a preimage that is not a point (the
/// cone apex, where every azimuth maps to one place).
pub(crate) fn uv_of(surface: &Surface<f64>, p: Point3<f64>) -> Option<Point2<f64>> {
    // The chart's second in-plane basis vector: `azimuth_frame`'s
    // `radial(u) = cos u · u_ref + sin u · (axis × u_ref)` for the
    // three round kinds, `normal × u_ref` for the plane.
    let uv = match *surface {
        Surface::Plane {
            origin,
            normal,
            u_ref,
        } => {
            let d = p - origin;
            Point2::new(d.dot(u_ref), d.dot(normal.cross(u_ref)))
        }
        Surface::Cylinder {
            origin,
            axis,
            u_ref,
            ..
        } => {
            let d = p - origin;
            Point2::new(azimuth(d, axis, u_ref), d.dot(axis))
        }
        Surface::Cone {
            apex,
            axis,
            half_angle,
            u_ref,
        } => {
            let d = p - apex;
            let axial = d.dot(axis);
            let cos_a = half_angle.cos();
            if cos_a == 0.0 {
                return None;
            }
            // `v` is slant arc length: the axial component divided by
            // cos α (`S = apex + axis·(v cos α) + radial·(v sin α)`).
            let v = axial / cos_a;
            if v == 0.0 {
                // The apex: every azimuth maps here, so the preimage
                // is not a point and there is no chart answer.
                return None;
            }
            // On the mirror nappe (`v < 0`) the radial term points
            // AGAINST `radial(u)`, so the raw azimuth is half a turn
            // from the parameter that generates the point.
            let u = azimuth(d, axis, u_ref);
            Point2::new(
                if v < 0.0 {
                    u + core::f64::consts::PI
                } else {
                    u
                },
                v,
            )
        }
        Surface::Sphere {
            center,
            axis,
            u_ref,
            ..
        } => {
            let d = p - center;
            let w = d.dot(axis);
            let ring = (d - axis * w).norm();
            if ring == 0.0 {
                // A pole: the chart is singular there (every longitude
                // maps to one point), so there is no chart answer.
                return None;
            }
            // `v` is latitude, `[−π/2, π/2]`.
            Point2::new(azimuth(d, axis, u_ref), w.atan2(ring))
        }
        Surface::Torus {
            center,
            axis,
            major_radius,
            u_ref,
            ..
        } => {
            let d = p - center;
            let w = d.dot(axis);
            let ring = (d - axis * w).norm() - major_radius;
            Point2::new(azimuth(d, axis, u_ref), w.atan2(ring))
        }
        // No closed-form chart inverse for a spline, nor for a fitted
        // stand-in whose chart is one.
        Surface::Nurbs(_) | Surface::Approx(_) => return None,
    };
    (uv.x.is_finite() && uv.y.is_finite()).then_some(uv)
}

/// The azimuth of `d` about `axis` from `u_ref`, in `(−π, π]`.
fn azimuth(d: Vec3<f64>, axis: Vec3<f64>, u_ref: Vec3<f64>) -> f64 {
    d.dot(axis.cross(u_ref)).atan2(d.dot(u_ref))
}

/// Whether the chart wraps in u / v — where a wrapping ring image is
/// seam-dependent (module docs).
fn periodic(surface: &Surface<f64>) -> (bool, bool) {
    match surface {
        Surface::Plane { .. } => (false, false),
        Surface::Cylinder { .. } | Surface::Cone { .. } | Surface::Sphere { .. } => (true, false),
        Surface::Torus { .. } => (true, true),
        // The conservative answer for a chart this module cannot
        // invert: assume both directions wrap, as the spline arm does.
        Surface::Nurbs(_) | Surface::Approx(_) => (true, true),
    }
}

/// A lower bound on the chart's local metric over `samples`:
/// `min(|∂S/∂u|, |∂S/∂v|)`, the factor that carries a tolerance in
/// meters into one in chart parameters.
fn metric_floor(surface: &Surface<f64>, samples: &[Point2<f64>]) -> f64 {
    samples.iter().fold(f64::INFINITY, |acc, uv| {
        // One jet per sample, two fields off it — the two partials are
        // wanted at the same `(u, v)`, and asking twice would evaluate
        // a NURBS chart's whole jet twice to keep one field of each.
        let j = surface.jet(uv.x, uv.y);
        acc.min(j.du.norm()).min(j.dv.norm())
    })
}

/// One bound's chart polygon, with its signed area.
struct Ring {
    /// The chart samples, in traversal order (u unwrapped continuously
    /// for a periodic chart).
    poly: Vec<Point2<f64>>,
    /// The shoelace signed area (chart units²).
    area: f64,
}

/// Which of `rings` is the outer bound (module docs' rule). `rings`
/// holds each bound's 3-D samples in traversal order; `eps_in_eff` is
/// the interpretation budget in meters.
///
/// # Errors
///
/// [`OuternessRefusal`] — every ambiguity D7 forbids guessing through.
pub(crate) fn infer_outer(
    surface: &Surface<f64>,
    rings: &[Vec<Point3<f64>>],
    eps_in_eff: f64,
) -> Result<usize, OuternessRefusal> {
    // Outerness is inferred in the chart, and neither spline kind has
    // the closed-form inverse this needs.
    if matches!(surface, Surface::Nurbs(_) | Surface::Approx(_)) {
        return Err(OuternessRefusal::UnsupportedChart);
    }
    let (per_u, per_v) = periodic(surface);
    let mut charted = Vec::with_capacity(rings.len());
    let mut floor = f64::INFINITY;
    for ring in rings {
        let mut poly: Vec<Point2<f64>> = Vec::with_capacity(ring.len());
        for &p in ring {
            let mut uv = uv_of(surface, p).ok_or(OuternessRefusal::NotOnChart)?;
            // Unwrap continuously from the previous sample; a jump past
            // half a period means the ring's image genuinely crosses
            // the seam, which is what makes the answer seam-dependent.
            if let Some(prev) = poly.last().copied() {
                if per_u {
                    uv.x = unwrap(prev.x, uv.x)?;
                }
                if per_v {
                    uv.y = unwrap(prev.y, uv.y)?;
                }
            }
            poly.push(uv);
        }
        // A closed ring must come back to where it started: a net
        // winding in either periodic direction is a ring that encircles
        // the axis, whose chart image is a seam artifact.
        if let (Some(first), Some(last)) = (poly.first().copied(), poly.last().copied()) {
            let closing = (last.x - first.x, last.y - first.y);
            if (per_u && closing.0.abs() > core::f64::consts::PI)
                || (per_v && closing.1.abs() > core::f64::consts::PI)
            {
                return Err(OuternessRefusal::SeamDependent);
            }
        }
        floor = floor.min(metric_floor(surface, &poly));
        let area = signed_area(&poly);
        charted.push(Ring { poly, area });
    }
    if !(floor.is_finite() && floor > 0.0) {
        return Err(OuternessRefusal::NotOnChart);
    }
    // The budget, carried into chart units by the local metric.
    let tol = eps_in_eff / floor;

    let mut outer = None;
    for (i, candidate) in charted.iter().enumerate() {
        let mut contains_all = true;
        for (j, other) in charted.iter().enumerate() {
            if i == j {
                continue;
            }
            let probe = other
                .poly
                .first()
                .copied()
                .ok_or(OuternessRefusal::NotOnChart)?;
            if boundary_distance(&candidate.poly, probe) <= tol {
                return Err(OuternessRefusal::UndecidableAtEps);
            }
            if !contains(&candidate.poly, probe) {
                contains_all = false;
                break;
            }
            // The independent orientation signal: a hole reverses.
            if candidate.area == 0.0
                || other.area == 0.0
                || candidate.area.is_sign_positive() == other.area.is_sign_positive()
            {
                return Err(OuternessRefusal::OrientationContradicts);
            }
        }
        if contains_all {
            if outer.is_some() {
                return Err(OuternessRefusal::NoUniqueOuter);
            }
            outer = Some(i);
        }
    }
    outer.ok_or(OuternessRefusal::NoUniqueOuter)
}

/// `next` moved into the branch continuous with `prev`, or the seam
/// refusal when the step itself exceeds half a period (the sampling is
/// then too coarse to tell a wrap from a jump — an honest refusal, not
/// a guessed branch).
fn unwrap(prev: f64, next: f64) -> Result<f64, OuternessRefusal> {
    let tau = core::f64::consts::TAU;
    let k = ((prev - next) / tau).round();
    let lifted = next + k * tau;
    if (lifted - prev).abs() > core::f64::consts::FRAC_PI_2 {
        return Err(OuternessRefusal::SeamDependent);
    }
    Ok(lifted)
}

/// The shoelace signed area of a closed chart polygon.
fn signed_area(poly: &[Point2<f64>]) -> f64 {
    let mut acc = 0.0;
    for i in 0..poly.len() {
        let a = poly[i];
        let b = poly[(i + 1) % poly.len()];
        acc += a.x * b.y - b.x * a.y;
    }
    acc / 2.0
}

/// Crossing-number containment of `p` in the closed chart polygon.
fn contains(poly: &[Point2<f64>], p: Point2<f64>) -> bool {
    let mut inside = false;
    for i in 0..poly.len() {
        let a = poly[i];
        let b = poly[(i + 1) % poly.len()];
        if (a.y > p.y) != (b.y > p.y) {
            let t = (p.y - a.y) / (b.y - a.y);
            if p.x < a.x + t * (b.x - a.x) {
                inside = !inside;
            }
        }
    }
    inside
}

/// The distance from `p` to the closed polygon's boundary (chart
/// units) — what the interpretation budget is spent against.
fn boundary_distance(poly: &[Point2<f64>], p: Point2<f64>) -> f64 {
    let mut best = f64::INFINITY;
    for i in 0..poly.len() {
        let a = poly[i];
        let b = poly[(i + 1) % poly.len()];
        let ab = b - a;
        let len2 = ab.dot(ab);
        let t = if len2 > 0.0 {
            ((p - a).dot(ab) / len2).clamp(0.0, 1.0)
        } else {
            0.0
        };
        best = best.min((p - (a + ab * t)).norm());
    }
    best
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used)]
mod tests {
    use super::{infer_outer, uv_of};
    // The marker gates every test in this FILE's module, these three
    // submodules included; it sits inside one of them because a bare
    // top-level `#[cfg(test)]` line is what the tree's source censuses
    // read as the production/test cut, and a second one makes that cut
    // ambiguous.
    test_utils::gated_to![
        "crates/step-import/src/recognize.rs",
        "crates/step-import/src/normalize.rs",
        "crates/geom/src/surfaces/",
        "crates/geom/src/surfaces.rs",
    ];

    use geom::Surface;
    use geom_core::{Point3, Vec3};

    fn plane() -> Surface<f64> {
        Surface::Plane {
            origin: Point3::new(0.0, 0.0, 0.0),
            normal: Vec3::new(0.0, 0.0, 1.0),
            u_ref: Vec3::new(1.0, 0.0, 0.0),
        }
    }

    /// The closed-form inverse is the inverse: `eval ∘ uv_of = id` on
    /// every analytic kind the subset carries.
    #[test]
    fn the_chart_inverse_inverts_the_kernels_own_eval() {
        let surfaces = [
            plane(),
            Surface::Cylinder {
                origin: Point3::new(1.0, 2.0, 3.0),
                axis: Vec3::new(0.0, 0.0, 1.0),
                radius: 0.5,
                u_ref: Vec3::new(1.0, 0.0, 0.0),
            },
            Surface::Cone {
                apex: Point3::new(0.0, 0.0, 1.0),
                axis: Vec3::new(0.0, 0.0, -1.0),
                half_angle: core::f64::consts::FRAC_PI_4,
                u_ref: Vec3::new(1.0, 0.0, 0.0),
            },
            Surface::Sphere {
                center: Point3::new(-1.0, 0.0, 0.5),
                radius: 2.0,
                axis: Vec3::new(0.0, 0.0, 1.0),
                u_ref: Vec3::new(1.0, 0.0, 0.0),
            },
            Surface::Torus {
                center: Point3::new(0.0, 0.0, 0.0),
                axis: Vec3::new(0.0, 0.0, 1.0),
                major_radius: 1.0,
                minor_radius: 0.25,
                u_ref: Vec3::new(1.0, 0.0, 0.0),
            },
        ];
        for s in &surfaces {
            for &(u, v) in &[(0.3, 0.7), (-1.1, 0.2), (2.5, -0.4)] {
                let p = s.eval(u, v);
                let uv = uv_of(s, p).expect("a chart preimage");
                let back = s.eval(uv.x, uv.y);
                assert!(
                    (back - p).norm() < 1e-12,
                    "{s:?} at ({u}, {v}): round trip {back:?} vs {p:?}"
                );
            }
        }
    }

    /// A square with a square hole: containment picks the outer ring,
    /// and the orientation signal agrees with it.
    #[test]
    fn a_ring_inside_a_ring_infers_the_container_as_outer() {
        let outer: Vec<Point3<f64>> = [(0.0, 0.0), (4.0, 0.0), (4.0, 4.0), (0.0, 4.0)]
            .iter()
            .map(|&(x, y)| Point3::new(x, y, 0.0))
            .collect();
        let hole: Vec<Point3<f64>> = [(1.0, 1.0), (1.0, 2.0), (2.0, 2.0), (2.0, 1.0)]
            .iter()
            .map(|&(x, y)| Point3::new(x, y, 0.0))
            .collect();
        assert_eq!(
            infer_outer(&plane(), &[outer.clone(), hole.clone()], 1e-9),
            Ok(0)
        );
        assert_eq!(infer_outer(&plane(), &[hole, outer], 1e-9), Ok(1));
    }

    /// Two disjoint rings have no container: refused, not guessed.
    #[test]
    fn disjoint_rings_refuse() {
        let a: Vec<Point3<f64>> = [(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)]
            .iter()
            .map(|&(x, y)| Point3::new(x, y, 0.0))
            .collect();
        let b: Vec<Point3<f64>> = a.iter().map(|p| Point3::new(p.x + 5.0, p.y, 0.0)).collect();
        assert_eq!(
            infer_outer(&plane(), &[a, b], 1e-9),
            Err(super::OuternessRefusal::NoUniqueOuter)
        );
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod review_fuzz {
    use super::uv_of;
    use geom::Surface;
    use geom_core::{Point3, Vec3};
    use test_utils::fuzz;

    fn frame(s: &mut fuzz::Rng) -> (Vec3<f64>, Vec3<f64>) {
        let a = Vec3::new(s.unit() - 0.5, s.unit() - 0.5, s.unit() - 0.5).normalize();
        let h = if a.x.abs() < 0.9 {
            Vec3::new(1.0, 0.0, 0.0)
        } else {
            Vec3::new(0.0, 1.0, 0.0)
        };
        let u = a.cross(h).normalize();
        (a, u)
    }

    /// Fuzz inverse-of-eval over all five kinds, tilted frames,
    /// near-seam and near-pole parameters included.
    #[test]
    fn review_fuzz_roundtrip() {
        let mut s = fuzz::start("chart::review_fuzz_roundtrip");
        let pi = core::f64::consts::PI;
        let mut worst: f64 = 0.0;
        for trial in 0..fuzz::scaled(50) {
            let (axis, u_ref) = frame(&mut s);
            let origin = Point3::new(
                s.unit() * 4.0 - 2.0,
                s.unit() * 4.0 - 2.0,
                s.unit() * 4.0 - 2.0,
            );
            let r = 0.2 + s.unit() * 3.0;
            let kinds: [Surface<f64>; 5] = [
                Surface::Plane {
                    origin,
                    normal: axis,
                    u_ref,
                },
                Surface::Cylinder {
                    origin,
                    axis,
                    radius: r,
                    u_ref,
                },
                Surface::Cone {
                    apex: origin,
                    axis,
                    half_angle: 0.05 + s.unit() * 1.4,
                    u_ref,
                },
                Surface::Sphere {
                    center: origin,
                    radius: r,
                    axis,
                    u_ref,
                },
                Surface::Torus {
                    center: origin,
                    axis,
                    major_radius: r + 1.0,
                    minor_radius: r * 0.3,
                    u_ref,
                },
            ];
            for (k, surf) in kinds.iter().enumerate() {
                // Parameter menu: generic, near-seam, near-pole/apex.
                let us = [s.unit() * 2.0 * pi - pi, pi - 1e-9, -pi + 1e-9];
                let vs = match k {
                    1 => [s.unit() * 4.0 - 2.0, 1e-9, -1e-9],
                    2 => [0.1 + s.unit() * 2.0, 1e-6, 3.0], // cone: v>0 (import never sees mirror nappe from eval side; also test v<0)
                    3 => [
                        s.unit() * 3.0 - 1.5,
                        core::f64::consts::FRAC_PI_2 - 1e-7,
                        -core::f64::consts::FRAC_PI_2 + 1e-7,
                    ],
                    4 => [s.unit() * 2.0 * pi - pi, pi - 1e-9, 1e-9],
                    _ => [s.unit() * 4.0 - 2.0, 0.0, 1.0],
                };
                for &u in &us {
                    for &v in &vs {
                        let p = surf.eval(u, v);
                        let Some(uv) = uv_of(surf, p) else {
                            // Only poles/apex may answer None.
                            assert!(
                                matches!(surf, Surface::Sphere { .. } | Surface::Cone { .. }),
                                "t{trial} k{k} ({u},{v}): unexpected None — {}",
                                fuzz::replay()
                            );
                            continue;
                        };
                        let back = surf.eval(uv.x, uv.y);
                        let err = (back - p).norm();
                        worst = worst.max(err / (1.0 + (p - origin).norm()));
                        assert!(
                            err <= 1e-9 * (1.0 + (p - origin).norm()),
                            "t{trial} k{k} ({u},{v}) -> uv({},{}) err {err:e} — {}",
                            uv.x,
                            uv.y,
                            fuzz::replay()
                        );
                    }
                }
            }
        }
        println!("review_fuzz worst rel err: {worst:e}");
        // Cone mirror nappe: eval at v<0 must round-trip too.
        let (axis, u_ref) = frame(&mut s);
        let cone = Surface::Cone {
            apex: Point3::new(0.3, -0.2, 0.9),
            axis,
            half_angle: 0.6,
            u_ref,
        };
        let nappe = fuzz::scaled(25);
        for i in 0..nappe {
            let u = ((i as f64) / (nappe as f64)) * 2.0 * pi - pi;
            let v = -(0.01 + s.unit() * 2.0);
            let p = cone.eval(u, v);
            let uv = uv_of(&cone, p).expect("mirror nappe preimage");
            let back = cone.eval(uv.x, uv.y);
            assert!(
                (back - p).norm() <= 1e-9,
                "nappe u={u} v={v}: {:?} — {}",
                (back - p).norm(),
                fuzz::replay()
            );
        }
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod review_outerness {
    use super::{OuternessRefusal, infer_outer};
    use geom::Surface;
    use geom_core::{Point3, Vec3};

    fn cyl() -> Surface<f64> {
        Surface::Cylinder {
            origin: Point3::new(0.0, 0.0, 0.0),
            axis: Vec3::new(0.0, 0.0, 1.0),
            radius: 1.0,
            u_ref: Vec3::new(1.0, 0.0, 0.0),
        }
    }
    fn ring_on_cyl(u0: f64, u1: f64, z0: f64, z1: f64, n: usize) -> Vec<Point3<f64>> {
        // Rectangle in (u,z), traversed CCW, sampled n per side.
        let mut out = Vec::new();
        let f = |u: f64, z: f64| Point3::new(u.cos(), u.sin(), z);
        for i in 0..n {
            let t = i as f64 / n as f64;
            out.push(f(u0 + (u1 - u0) * t, z0));
        }
        for i in 0..n {
            let t = i as f64 / n as f64;
            out.push(f(u1, z0 + (z1 - z0) * t));
        }
        for i in 0..n {
            let t = i as f64 / n as f64;
            out.push(f(u1 - (u1 - u0) * t, z1));
        }
        for i in 0..n {
            let t = i as f64 / n as f64;
            out.push(f(u0, z1 - (z1 - z0) * t));
        }
        out
    }
    fn lat_ring(sphere_r: f64, lat: f64, n: usize, ccw: bool) -> Vec<Point3<f64>> {
        (0..n)
            .map(|i| {
                let u = 2.0 * core::f64::consts::PI * (i as f64) / (n as f64);
                let u = if ccw { u } else { -u };
                Point3::new(
                    sphere_r * lat.cos() * u.cos(),
                    sphere_r * lat.cos() * u.sin(),
                    sphere_r * lat.sin(),
                )
            })
            .collect()
    }

    #[test]
    fn review_six_refusals_fire() {
        // 1. UnsupportedChart is nurbs-only: skipped (no nurbs ctor here);
        //    entities.rs routes nurbs faces elsewhere.
        // 2. SeamDependent: a ring encircling the cylinder axis.
        let wrap: Vec<Point3<f64>> = (0..64)
            .map(|i| {
                let u = 2.0 * core::f64::consts::PI * (i as f64) / 64.0;
                Point3::new(u.cos(), u.sin(), 0.0)
            })
            .collect();
        let inner = ring_on_cyl(0.5, 1.0, 0.2, 0.4, 8);
        assert_eq!(
            infer_outer(&cyl(), &[wrap, inner.clone()], 1e-10),
            Err(OuternessRefusal::SeamDependent),
            "wrap ring"
        );
        // 3. UndecidableAtEps: probe ring's first sample within eps of the
        //    candidate's boundary.
        let outer = ring_on_cyl(0.0, 2.0, 0.0, 1.0, 8);
        let touching = ring_on_cyl(0.0 + 1e-12, 1.0, 0.3, 0.6, 8);
        assert_eq!(
            infer_outer(&cyl(), &[outer.clone(), touching], 1e-10),
            Err(OuternessRefusal::UndecidableAtEps),
            "touching ring"
        );
        // 4. OrientationContradicts: inner ring wound the SAME way.
        let mut inner_same = ring_on_cyl(0.5, 1.0, 0.2, 0.4, 8);
        let outer2 = ring_on_cyl(0.0, 2.0, 0.0, 1.0, 8);
        // same CCW winding as outer => contradiction
        assert_eq!(
            infer_outer(&cyl(), &[outer2.clone(), inner_same.clone()], 1e-10),
            Err(OuternessRefusal::OrientationContradicts),
            "same-wound inner"
        );
        // and reversed is accepted with outer index 0:
        inner_same.reverse();
        assert_eq!(infer_outer(&cyl(), &[outer2, inner_same], 1e-10), Ok(0));
        // 5. NoUniqueOuter: disjoint rings.
        let a = ring_on_cyl(0.0, 0.5, 0.0, 0.5, 8);
        let b = ring_on_cyl(1.0, 1.5, 0.0, 0.5, 8);
        assert_eq!(
            infer_outer(&cyl(), &[a, b], 1e-10),
            Err(OuternessRefusal::NoUniqueOuter),
            "disjoint"
        );
        // 6. NotOnChart: a ring sample AT the sphere pole.
        let sph = Surface::Sphere {
            center: Point3::new(0.0, 0.0, 0.0),
            radius: 1.0,
            axis: Vec3::new(0.0, 0.0, 1.0),
            u_ref: Vec3::new(1.0, 0.0, 0.0),
        };
        let mut polar = lat_ring(1.0, 1.2, 16, true);
        polar[3] = Point3::new(0.0, 0.0, 1.0); // the pole
        let low = lat_ring(1.0, -0.5, 16, false);
        assert_eq!(
            infer_outer(&sph, &[polar, low.clone()], 1e-10),
            Err(OuternessRefusal::NotOnChart),
            "pole sample"
        );
        // 7. A ring ENCLOSING the pole (full latitude circle) wraps u:
        //    SeamDependent, refused not guessed.
        let cap = lat_ring(1.0, 1.4, 64, true);
        assert_eq!(
            infer_outer(&sph, &[cap, low], 1e-10),
            Err(OuternessRefusal::SeamDependent),
            "pole-enclosing ring"
        );
    }
}
