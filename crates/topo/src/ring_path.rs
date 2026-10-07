//! **A ring lane's chart-free reading on a sphere or a cone face**: the
//! parity of a path on the face's surface across a closed curve of loop
//! arcs, which says whether the path's ends lie on one side of the curve
//! or on opposite sides.
//!
//! The path is made of pieces whose meeting with a loop arc is closed
//! form: on a sphere, the short great-circle arc between its ends; on a
//! cone, a segment of a ruling and an arc of a parallel (the circle at
//! one height), in either order, on the ends' nappe and never through
//! the apex. Every loop arc is a plane section of the surface (a circle
//! or an ellipse: no lane mints another conic) or, on a cone, a ruling.
//! So a piece meets a section arc only where it crosses the arc's plane,
//! which a circle piece does at most twice and a segment once, and it
//! meets a ruling only where the ruling crosses a parallel's plane. Two
//! rulings meet only at the apex, which no piece reaches.
//!
//! The parity does not depend on the path because the curve is
//! null-homotopic where the paths run. Every closed curve on a sphere
//! is. On a cone, the paths run on the punctured nappe, an annulus, and
//! the curve bounds a disc of it: it lies on one face, and no face winds
//! round the axis — at rest, tier 3's pcurve mint refuses a face whose
//! outer loop spans a full period or whose loop wraps
//! (`PcurveMintError::{OuterSpansPeriod, LoopWraps}`), so a face that
//! reaches round the axis is cut by its seam, and no curve on it winds
//! round the apex.
//!
//! Each decision is a named trilean metered in metres. The raw picks
//! that remain — the nearer root on the arrival arc, a parameter's
//! branch, the way round a parallel — are choices every outcome of which
//! is sound, each argued at its site. A crossing is
//! passed over once any reading is decided against it. A reading of a
//! crossing not passed over that lands in the zero band — the path
//! grazes an arc, runs through an arc's end, or starts on an arc's
//! plane — makes the path undecided, and the caller asks another.

use geom_core::{Band, Decide, Indeterminate, Margin, Point3, Real, Sign, Vec3};

use crate::validate::decide;

/// The surface a path runs on: the kinds the ring lane reads without a
/// chart.
#[derive(Clone, Copy)]
pub(crate) enum Quadric<T: Real> {
    Sphere {
        centre: Point3<T>,
        radius: T,
    },
    Cone {
        apex: Point3<T>,
        axis: Vec3<T>,
        half_angle: T,
    },
}

impl<T: Decide> Quadric<T> {
    /// The quadric of `surface`; `None` for any other kind.
    pub(crate) fn of(surface: &geom::Surface<T>) -> Option<Self> {
        match *surface {
            geom::Surface::Sphere { center, radius, .. } => Some(Self::Sphere {
                centre: center,
                radius,
            }),
            geom::Surface::Cone {
                apex,
                axis,
                half_angle,
                ..
            } => Some(Self::Cone {
                apex,
                axis,
                half_angle,
            }),
            _ => None,
        }
    }

    /// `p`'s height along a cone's axis and its offset from the axis.
    fn axial(apex: Point3<T>, axis: Vec3<T>, p: Point3<T>) -> (T, Vec3<T>) {
        let w = p - apex;
        let h = w.dot(axis);
        (h, w - axis * h)
    }

    /// The length a dimensionless reading at `p` is levered by: the
    /// sphere's radius, or the radius of the cone's parallel through `p`.
    pub(crate) fn lever(&self, p: Point3<T>) -> T {
        match *self {
            Self::Sphere { radius, .. } => radius,
            Self::Cone { apex, axis, .. } => Self::axial(apex, axis, p).1.norm(),
        }
    }

    /// The chart normal at `p` (the outward one for a face of sense
    /// `true`): the sphere's radial direction, or the cone's
    /// `ŵ·cos α − â·sin α·sgn h`, `ŵ` the radial direction. `p` is off a
    /// cone's apex ([`Self::off_apex`]).
    fn chart_normal(&self, p: Point3<T>) -> Vec3<T> {
        match *self {
            Self::Sphere { centre, radius } => (p - centre) / radius,
            Self::Cone {
                apex,
                axis,
                half_angle,
            } => {
                let (h, w) = Self::axial(apex, axis, p);
                let (s, c) = half_angle.sin_cos();
                w / w.norm() * c - axis * s.copysign(h)
            }
        }
    }

    /// The outward normal at `p` of a face of sense `sense`
    /// ([`Self::chart_normal`] folded by the sense).
    pub(crate) fn outward(&self, p: Point3<T>, sense: bool) -> Vec3<T> {
        geom_brep::OutwardNormal::from_chart(self.chart_normal(p), sense).vec()
    }

    /// Whether `p` is decided off a cone's apex (its distance from the
    /// axis, **`split_ring_path_off_apex`**): always on a sphere.
    pub(crate) fn off_apex(&self, p: Point3<T>, band: Band) -> Result<bool, Indeterminate> {
        match *self {
            Self::Sphere { .. } => Ok(true),
            Self::Cone { apex, axis, .. } => Ok(decide(
                "split_ring_path_off_apex",
                Margin::of(Self::axial(apex, axis, p).1.norm()),
                band,
            )? == Sign::Positive),
        }
    }

    /// **The paths from `from` to `to`** this quadric reads, in the order
    /// they are asked; none where no path is decided to exist.
    ///
    /// - A sphere's is the short great-circle arc, which exists when the
    ///   ends are decided neither equal nor antipodal
    ///   (**`split_ring_path_span`**: the sine between them, levered at
    ///   the radius).
    /// - A cone's are two, on the ends' one nappe (each end's height
    ///   along the axis decided nonzero and of one sign,
    ///   **`split_ring_path_nappe`**) and off the apex: along `from`'s
    ///   ruling to `to`'s height and round that parallel, arriving along
    ///   it; and round `from`'s parallel to `to`'s ruling and along it,
    ///   arriving along the ruling where the ends' slant distances are
    ///   decided apart (**`split_ring_path_rise`**). The arrival across
    ///   the closing arc's section is transversal on one of them
    ///   wherever it is on either.
    pub(crate) fn paths(
        &self,
        (from, to): (Point3<T>, Point3<T>),
        band: Band,
    ) -> Result<Vec<Path<T>>, Indeterminate> {
        match *self {
            Self::Sphere { centre, radius } => {
                let a = (from - centre) / radius;
                let b = (to - centre) / radius;
                let m = a.cross(b);
                let sine = m.norm();
                if decide("split_ring_path_span", Margin::levered(sine, radius), band)?
                    != Sign::Positive
                {
                    return Ok(Vec::new());
                }
                let axis = m / sine;
                let arc = CircleArc {
                    centre,
                    axis,
                    radius,
                    u_ref: a,
                    span: (T::zero(), sine.atan2(a.dot(b))),
                };
                Ok(vec![Path {
                    pieces: vec![Piece::Arc(arc)],
                    arrival: axis.cross(b),
                }])
            }
            Self::Cone { apex, axis, .. } => {
                if !self.off_apex(from, band)? || !self.off_apex(to, band)? {
                    return Ok(Vec::new());
                }
                let (ha, ra) = Self::axial(apex, axis, from);
                let (hb, rb) = Self::axial(apex, axis, to);
                let nappe = |h| decide("split_ring_path_nappe", Margin::of(h), band);
                let side = nappe(ha)?;
                if side == Sign::Zero || nappe(hb)? != side {
                    return Ok(Vec::new());
                }
                let ruling = |p: Point3<T>| {
                    let e = p - apex;
                    (e / e.norm(), e.norm())
                };
                let mut paths = Vec::with_capacity(2);
                // Each turn is placed on the parallel the next piece runs
                // round, at that parallel's own radius, so the pieces meet
                // exactly: the ends lie on the carrier only up to the
                // band, and a turn at the carrier's ratio `|r|·h'/h` would
                // leave a gap of that offset at the seam, where a crossing
                // could go uncounted. The segment then leans off the
                // ruling by the same offset: its crossings of a section
                // are read from its ends, and the rulings-apart reading
                // is decided past the band that offset lies in.
                let (ua, ub) = (ra / ra.norm(), rb / rb.norm());
                // Along `from`'s ruling to `to`'s height, then round.
                let turned = if r4_mut("seam") { apex + axis * hb + ra * (hb / ha) } else { apex + axis * hb + ua * rb.norm() };
                let (e, slant) = ruling(from);
                let round = parallel(apex + axis * hb, axis, rb.norm(), turned, to);
                paths.push(Path {
                    pieces: vec![
                        Piece::Segment {
                            from,
                            to: turned,
                            ruling: e,
                            lever: slant,
                        },
                        Piece::Arc(round),
                    ],
                    arrival: round.tangent(round.span.1),
                });
                // Round `from`'s parallel to `to`'s ruling, then along it.
                let (e, slant) = ruling(to);
                let below = if r4_mut("seam") { apex + axis * ha + rb * (ha / hb) } else { apex + axis * ha + ub * ra.norm() };
                let rise = slant - (below - apex).norm();
                let along = decide("split_ring_path_rise", Margin::of(rise), band)?;
                if along != Sign::Zero {
                    let round = parallel(apex + axis * ha, axis, ra.norm(), from, below);
                    paths.push(Path {
                        pieces: vec![
                            Piece::Arc(round),
                            Piece::Segment {
                                from: below,
                                to,
                                ruling: e,
                                lever: slant,
                            },
                        ],
                        arrival: if along == Sign::Positive { e } else { -e },
                    });
                }
                Ok(paths)
            }
        }
    }
}

/// The arc of the parallel centred `centre` (on the axis `axis`) of
/// radius `radius` from `from` to `to`, the short way round.
fn parallel<T: Decide>(
    centre: Point3<T>,
    axis: Vec3<T>,
    radius: T,
    from: Point3<T>,
    to: Point3<T>,
) -> CircleArc<T> {
    let u = from - centre;
    let u = u / u.norm();
    let w = to - centre;
    let phi = w.dot(axis.cross(u)).atan2(w.dot(u));
    // The way round is a raw pick: either is a path between the same
    // ends, and the parity is the same on every path.
    CircleArc {
        centre,
        axis: axis * T::one().copysign(phi),
        radius,
        u_ref: u,
        span: (T::zero(), phi.abs()),
    }
}

/// A circle arc, travelled with its parameter over the increasing
/// `span`: `centre + (u_ref·cos t + (axis × u_ref)·sin t)·radius`.
#[derive(Clone, Copy)]
struct CircleArc<T: Real> {
    centre: Point3<T>,
    axis: Vec3<T>,
    radius: T,
    u_ref: Vec3<T>,
    span: (T, T),
}

impl<T: Real> CircleArc<T> {
    fn v_ref(&self) -> Vec3<T> {
        self.axis.cross(self.u_ref)
    }

    fn at(&self, t: T) -> Point3<T> {
        let (s, c) = t.sin_cos();
        self.centre + (self.u_ref * c + self.v_ref() * s) * self.radius
    }

    /// The unit direction of travel at `t`.
    fn tangent(&self, t: T) -> Vec3<T> {
        let (s, c) = t.sin_cos();
        self.v_ref() * c - self.u_ref * s
    }

    /// The parameter of `p` (a point of the circle) on the branch of the
    /// span's middle.
    fn param(&self, p: Point3<T>) -> T {
        let w = p - self.centre;
        branch(w.dot(self.v_ref()).atan2(w.dot(self.u_ref)), self.span)
    }
}

/// `t` moved by whole turns to the branch of `span`'s middle.
///
/// A raw pick, sound whichever way it falls: every span is shorter than a
/// turn, so a parameter inside it lies within half a span of the middle
/// and lands on its own branch. Only a parameter half a turn from the
/// middle is in doubt, and that one is at least half a turn less half
/// the span outside the span on either branch, which the in-span reading
/// then decides, or puts in its band where the span nears a whole turn.
fn branch<T: Real>(t: T, (lo, hi): (T, T)) -> T {
    let mid = (lo + hi) * T::from_f64(0.5);
    let tau = T::tau();
    t + (mid - t).periodic_branch(tau) * tau
}

/// A piece of a [`Path`].
#[derive(Clone, Copy)]
enum Piece<T: Real> {
    /// An arc of a great circle or of a cone's parallel.
    Arc(CircleArc<T>),
    /// A segment of the cone's ruling `ruling` (unit, from the apex),
    /// whose distance from the apex at `from` is `lever`.
    Segment {
        from: Point3<T>,
        to: Point3<T>,
        ruling: Vec3<T>,
        lever: T,
    },
}

/// A path on a [`Quadric`] between two points, and its unit direction
/// of travel where it arrives.
pub(crate) struct Path<T: Real> {
    pieces: Vec<Piece<T>>,
    pub(crate) arrival: Vec3<T>,
}

/// An arc of a loop on a sphere or cone face, over its increasing span:
/// a section conic (`centre + u_ref·a·cos t + (axis × u_ref)·b·sin t`,
/// `a = b` on a circle) or a ruling (`origin + dir·t`).
#[derive(Clone, Copy)]
pub(crate) enum LoopArc<T: Real> {
    Conic {
        centre: Point3<T>,
        axis: Vec3<T>,
        u_ref: Vec3<T>,
        a: T,
        b: T,
        span: (T, T),
    },
    Line {
        origin: Point3<T>,
        dir: Vec3<T>,
        span: (T, T),
    },
}

impl<T: Real> LoopArc<T> {
    /// The arc of `carrier` between the parameters `t0` and `t1`;
    /// `None` on a carrier that is neither a circle, an ellipse nor a
    /// line.
    pub(crate) fn of(carrier: &geom::Curve3<T>, (t0, t1): (T, T)) -> Option<Self> {
        let span = (t0.min(t1), t0.max(t1));
        match *carrier {
            geom::Curve3::Circle {
                center,
                axis,
                radius,
                u_ref,
            } => Some(Self::Conic {
                centre: center,
                axis,
                u_ref,
                a: radius,
                b: radius,
                span,
            }),
            geom::Curve3::Ellipse {
                center,
                axis,
                major,
                minor,
                u_ref,
            } => Some(Self::Conic {
                centre: center,
                axis,
                u_ref,
                a: major,
                b: minor,
                span,
            }),
            geom::Curve3::Line { origin, dir } => Some(Self::Line { origin, dir, span }),
            _ => None,
        }
    }

    /// The straight arc from `from` to `to`.
    pub(crate) fn segment(from: Point3<T>, to: Point3<T>) -> Self {
        let d = to - from;
        let length = d.norm();
        Self::Line {
            origin: from,
            dir: d / length,
            span: (T::zero(), length),
        }
    }
}

/// **Whether `path` crosses `arcs` an odd number of times**: whether its
/// ends lie on opposite sides of the closed curve the arcs make (module
/// docs). `arrives_on` names the arc the path's end lies on, at an
/// interior point; its crossing there is the arrival and is not
/// counted. `None` when a reading of a crossing not passed over lands
/// in the zero band; `Err` when one lands in the escalation band, which
/// says nothing about the parity either: the caller asks another path.
///
/// - A circle piece meets a conic's plane where
///   `A cos θ + B sin θ = −D/r` (`A`, `B` the plane normal's components
///   on the piece's frame, `D` the piece's centre's offset from the
///   plane): at two parameters when `r·√(A² + B²) > |D|`
///   (**`split_ring_path_meets_plane`**, on the gap `r·√(A² + B²) − |D|`
///   in metres: moving the plane by it flips the meeting). A graze inside
///   the gap's band escalates, and the caller asks the next path. On the
///   arrival arc, the root nearer the piece's end is the arrival.
/// - A segment meets a conic's plane where its ends' offsets from it
///   (**`split_ring_path_segment_side`**) differ in sign.
/// - A ruling meets a circle piece's plane once (it is decided across
///   it, **`split_ring_path_line_across`**: the cosine between the
///   plane's normal and the ruling, levered at the piece's radius), and
///   another ruling nowhere off the apex (the two are decided apart,
///   **`split_ring_path_rulings_apart`**: the sine between them,
///   levered at the segment's distance from the apex).
/// - Each meeting point counts when it lies strictly inside the piece's
///   span and the arc's (**`split_ring_path_in_span`**: the distance
///   along the piece or the arc to a span end — a parameter levered at a
///   circle's radius or a conic's minor semi-axis, or a line's length —
///   times the slope the piece crosses the other's plane at. That is the
///   carrier displacement that moves the root past the end: near a graze
///   a root moves by the displacement over the slope, so a span end a
///   decided arc length away can still be inside its reach.)
pub(crate) fn path_parity<T: Decide>(
    path: &Path<T>,
    arcs: &[LoopArc<T>],
    arrives_on: Option<usize>,
    band: Band,
) -> Result<Option<bool>, Indeterminate> {
    let decide_m = |name, margin| decide(name, margin, band);
    let in_span = |t: T, (lo, hi): (T, T), arm: T| -> Result<[Sign; 2], Indeterminate> {
        Ok([
            decide_m("split_ring_path_in_span", Margin::levered(t - lo, arm))?,
            decide_m("split_ring_path_in_span", Margin::levered(hi - t, arm))?,
        ])
    };
    let mut odd = false;
    let last = path.pieces.len().saturating_sub(1);
    for (i, piece) in path.pieces.iter().enumerate() {
        for (j, arc) in arcs.iter().enumerate() {
            let arrival = i == last && arrives_on == Some(j);
            let mut readings: Vec<[Sign; 4]> = Vec::with_capacity(2);
            match (*piece, *arc) {
                (
                    Piece::Arc(p),
                    LoopArc::Conic {
                        centre,
                        axis,
                        u_ref,
                        a,
                        b,
                        span,
                    },
                ) => {
                    let (ca, cb) = (axis.dot(p.u_ref), axis.dot(p.v_ref()));
                    let d = axis.dot(p.centre - centre);
                    let rho = (ca.powi(2) + cb.powi(2)).sqrt();
                    // The gap `rρ − |D|`: moving the plane by it flips the
                    // meeting, so it is the deviation the sign rests on.
                    match decide_m(
                        "split_ring_path_meets_plane",
                        Margin::of(p.radius * rho - d.abs()),
                    )? {
                        Sign::Negative => continue,
                        Sign::Zero => return Ok(None),
                        Sign::Positive => {}
                    }
                    let phase = cb.atan2(ca);
                    let offset = (-d / (p.radius * rho)).max(-T::one()).min(T::one()).acos();
                    let [r0, r1] = [phase + offset, phase - offset].map(|t| branch(t, p.span));
                    // On the arrival arc, the root nearer the end is the
                    // arrival; the other is read. The pick is a raw
                    // comparison and needs no decision. The arrival lies
                    // on the plane up to rounding δ, and a root misplaced
                    // by δ in the plane's offset moves by δ over the
                    // slope it crosses at (as the in-span readings below
                    // are levered): near a graze, with half-angle ω
                    // between the roots, the slope is `ρ·sin ω` and the
                    // roots lie `2rω` apart along the arc, so the root
                    // error against their separation is
                    // `δ/(2rρ·sin ω·ω) ≈ δ/(4g)` for the gap
                    // `g = rρ − |D| ≈ rρω²/2`. A decided gap `g ≥ Kε`
                    // keeps that far below the half the pick can absorb
                    // (about 150× headroom at ε 1e-12, offset 1e3).
                    let nearer = (r0 - p.span.1).abs() - (r1 - p.span.1).abs();
                    let roots = if arrival {
                        [nearer.select_le_zero(r1, r0), r1]
                    } else {
                        [r0, r1]
                    };
                    let v_ref = axis.cross(u_ref);
                    for &t in &roots[..if arrival { 1 } else { 2 }] {
                        let w = p.at(t) - centre;
                        let s = branch((w.dot(v_ref) / b).atan2(w.dot(u_ref) / a), span);
                        // A carrier moved by δ moves the root by δ over the
                        // slope the piece crosses the plane at, `|n̂·τ̂|`:
                        // each in-span reading is levered by it, so its
                        // margin is the displacement that moves the root
                        // past the span's end.
                        let (st, ct) = t.sin_cos();
                        let slope = if r4_mut("sign") { (cb * ct + ca * st).abs() } else if r4_mut("conic_len") { T::one() } else { (cb * ct - ca * st).abs() };
                        if std::env::var("R4_LOG").is_ok() {
                            std::eprintln!("R4 conic: r {:?} b {:?} rho {:?} gap {:?} slope {:?} t {:?} pspan {:?} s {:?} span {:?}", p.radius, b, rho, p.radius * rho - d.abs(), slope, t, p.span, s, span);
                        }
                        let [p0, p1] = in_span(t, p.span, if r4_mut("circ_b") { b * slope } else { p.radius * slope })?;
                        let [a0, a1] = in_span(s, span, b * slope)?;
                        readings.push([p0, p1, a0, a1]);
                    }
                }
                (Piece::Arc(p), LoopArc::Line { origin, dir, span }) => {
                    let across = p.axis.dot(dir);
                    if decide_m(
                        "split_ring_path_line_across",
                        Margin::levered(across, p.radius),
                    )? == Sign::Zero
                    {
                        return Ok(None);
                    }
                    if arrival {
                        continue;
                    }
                    let s = p.axis.dot(p.centre - origin) / across;
                    let t = p.param(origin + dir * s);
                    // The plane moved by δ moves the root by δ over the
                    // slope the ruling crosses it at, `|m̂·d̂|`.
                    let slope = if r4_mut("line_len") { T::one() } else { across.abs() };
                    let [p0, p1] = in_span(t, p.span, p.radius * slope)?;
                    let [a0, a1] = in_span(s, span, slope)?;
                    readings.push([p0, p1, a0, a1]);
                }
                (
                    Piece::Segment { from, to, .. },
                    LoopArc::Conic {
                        centre,
                        axis,
                        u_ref,
                        a,
                        b,
                        span,
                    },
                ) => {
                    if arrival {
                        continue;
                    }
                    let (d0, d1) = (axis.dot(from - centre), axis.dot(to - centre));
                    let s0 = decide_m("split_ring_path_segment_side", Margin::of(d0))?;
                    let s1 = decide_m("split_ring_path_segment_side", Margin::of(d1))?;
                    if s0 == Sign::Zero || s1 == Sign::Zero {
                        return Ok(None);
                    }
                    if s0 == s1 {
                        continue;
                    }
                    let w = from + (to - from) * (d0 / (d0 - d1)) - centre;
                    let v_ref = axis.cross(u_ref);
                    let s = branch((w.dot(v_ref) / b).atan2(w.dot(u_ref) / a), span);
                    // The plane moved by δ moves the root by δ over the
                    // slope the segment crosses it at, `|n̂·d̂|`.
                    let slope = if r4_mut("segden") { (d0 - d1).abs() } else if r4_mut("seg_len") { T::one() } else { ((d0 - d1) / (to - from).norm()).abs() };
                    let [a0, a1] = in_span(s, span, b * slope)?;
                    readings.push([Sign::Positive, Sign::Positive, a0, a1]);
                }
                (Piece::Segment { ruling, lever, .. }, LoopArc::Line { dir, .. }) => {
                    if arrival {
                        continue;
                    }
                    if decide_m(
                        "split_ring_path_rulings_apart",
                        Margin::levered(ruling.cross(dir).norm(), lever),
                    )? != Sign::Positive
                    {
                        return Ok(None);
                    }
                }
            }
            for reading in readings {
                if reading.contains(&Sign::Negative) {
                    continue;
                }
                if reading.contains(&Sign::Zero) {
                    return Ok(None);
                }
                odd = !odd;
            }
        }
    }
    Ok(Some(odd))
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
pub(crate) mod cone_islands;

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod cone_path_grid;

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod graze_rows;

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic, clippy::print_stdout, clippy::too_many_arguments)]
mod review4_probes;

/// Review-4 probe mutant toggle (scratch).
fn r4_mut(name: &str) -> bool {
    std::env::var("R4_MUT").map(|v| v.split(',').any(|x| x == name)).unwrap_or(false)
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod tests {
    use super::cone_islands::{band, cone, frames, lune, on_nappe, sector};
    use super::*;
    use core::f64::consts::PI;

    /// **Every path on a cone reads the parity the islands' oracles
    /// give**, on both nappes, in three frames, and near the apex: a grid
    /// of points (heights 0.05 to 2.9, at 16 azimuths) taken pairwise,
    /// every path [`Quadric::paths`] offers between them. Nearly every
    /// pair is decided (the undecided ones run through a loop vertex),
    /// and both parities are read.
    #[test]
    fn a_cone_path_crosses_an_island_by_the_oracle_parity() {
        for f in frames() {
            for mirror in [false, true] {
                let cone = cone(f, mirror);
                let quadric = Quadric::of(&cone).unwrap();
                let mut points = Vec::new();
                for h in [0.05, 0.17, 0.6, 1.3, 1.6, 2.1, 2.9] {
                    for k in 0..16 {
                        points.push(on_nappe(f, h, -PI + (f64::from(k) + 0.5) * PI / 8.0));
                    }
                }
                for (name, island) in [
                    ("lune", lune(f, &cone, 1.0)),
                    ("lune by the apex", lune(f, &cone, 0.1)),
                    ("sector", sector(f, &cone)),
                ] {
                    let arcs: Vec<_> = island
                        .edges
                        .iter()
                        .map(|e| LoopArc::of(&e.carrier, e.params).unwrap())
                        .collect();
                    let (mut decided, mut asked, mut odd) = (0usize, 0usize, 0usize);
                    for (i, &a) in points.iter().enumerate() {
                        for &b in &points[i + 1..] {
                            let want = (island.inside)(a) != (island.inside)(b);
                            for path in quadric.paths((a, b), band()).unwrap() {
                                asked += 1;
                                let got = path_parity(&path, &arcs, None, band()).unwrap();
                                if let Some(got) = got {
                                    assert_eq!(got, want, "{name}, mirror {mirror}: {a:?} → {b:?}");
                                    decided += 1;
                                    odd += usize::from(got);
                                }
                            }
                        }
                    }
                    assert!(
                        decided * 100 >= asked * 99 && odd * 20 > decided,
                        "{name}, mirror {mirror}: {decided} of {asked} paths decided, {odd} odd"
                    );
                }
            }
        }
    }

    /// **A cone path's pieces meet exactly**, from end to end, wherever
    /// its ends lie off the carrier within the band: a vertex is on its
    /// face up to the band, not to rounding, so a piece aimed at the
    /// other end's parallel by the carrier's own ratio would miss the
    /// next by that offset, and a crossing in the gap would go uncounted.
    /// Points on a grid of both nappes in three frames, moved off the cone
    /// along its normal by up to the escalation threshold.
    #[test]
    fn a_cone_paths_pieces_meet_at_their_seams() {
        let ends = |piece: &Piece<f64>| match *piece {
            Piece::Arc(a) => (a.at(a.span.0), a.at(a.span.1)),
            Piece::Segment { from, to, .. } => (from, to),
        };
        let b = band();
        let mut asked = 0usize;
        for f in frames() {
            for mirror in [false, true] {
                let cone = cone(f, mirror);
                let quadric = Quadric::of(&cone).unwrap();
                let mut points = Vec::new();
                for (k, h) in [0.17, 0.6, 1.3, 2.9].into_iter().enumerate() {
                    for j in 0..6u32 {
                        let p = on_nappe(f, h, -PI + (f64::from(j) + 0.5) * PI / 3.0);
                        let off =
                            [0.0, b.zero(), -b.escalate(), b.escalate()][(k + j as usize) % 4];
                        points.push(p + quadric.chart_normal(p) * off);
                    }
                }
                for &a in &points {
                    for &z in &points {
                        for path in quadric.paths((a, z), b).unwrap() {
                            asked += 1;
                            let mut at = a;
                            for piece in &path.pieces {
                                let (from, to) = ends(piece);
                                assert!(
                                    (from - at).norm() <= 1e-13,
                                    "a seam is open by {:e} from {a:?} to {z:?}",
                                    (from - at).norm()
                                );
                                at = to;
                            }
                            assert!(
                                (at - z).norm() <= 1e-13,
                                "the path ends {:e} off {z:?}",
                                (at - z).norm()
                            );
                        }
                    }
                }
            }
        }
        assert!(asked > 1000, "{asked} paths");
    }
}
