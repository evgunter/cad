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
//! the curve bounds a disc of it: it lies on one face, and a face that
//! reaches round the axis is cut by its seam, so no curve on it winds
//! round the apex.
//!
//! Each comparison is a named trilean metered in metres. A crossing is
//! passed over once any reading is decided against it. A reading of a
//! crossing not passed over that lands in the zero band — the path
//! grazes an arc, runs through an arc's end, or starts on an arc's
//! plane — makes the path undecided, and the caller asks another.

use geom_core::{Band, Decide, Indeterminate, Margin, Point3, Real, Sign, Vec3};

use crate::chord_join::SplitJoinError;
use crate::entity::FaceKey;
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
    pub(crate) fn chart_normal(&self, p: Point3<T>) -> Vec3<T> {
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
                // Along `from`'s ruling to `to`'s height, then round.
                let turned = apex + axis * hb + ra * (hb / ha);
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
                let below = apex + axis * ha + rb * (ha / hb);
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
/// in the zero band. A reading in the escalation band escalates.
///
/// - A circle piece meets a conic's plane where
///   `A cos θ + B sin θ = −D/r` (`A`, `B` the plane normal's components
///   on the piece's frame, `D` the piece's centre's offset from the
///   plane): at two parameters when `r·√(A² + B²) > |D|`
///   (**`split_ring_path_meets_plane`**). On the arrival arc, the one
///   nearer the piece's end is the arrival.
/// - A segment meets a conic's plane where its ends' offsets from it
///   (**`split_ring_path_segment_side`**) differ in sign.
/// - A ruling meets a circle piece's plane once (it is decided across
///   it, **`split_ring_path_line_across`**: the cosine between the
///   plane's normal and the ruling, levered at the piece's radius), and
///   another ruling nowhere off the apex (the two are decided apart,
///   **`split_ring_path_rulings_apart`**: the sine between them,
///   levered at the segment's distance from the apex).
/// - Each meeting point counts when it lies strictly inside the piece's
///   span and the arc's (**`split_ring_path_in_span`**: a parameter
///   less a span end, levered at the circle's radius or a conic's
///   minor semi-axis; a line's parameter is a length).
pub(crate) fn path_parity<T: Decide>(
    face: FaceKey,
    path: &Path<T>,
    arcs: &[LoopArc<T>],
    arrives_on: Option<usize>,
    band: Band,
) -> Result<Option<bool>, SplitJoinError> {
    let decide_m = |name, margin| {
        decide(name, margin, band).map_err(|diag| SplitJoinError::Escalated { face, diag })
    };
    let in_span = |t: T, (lo, hi): (T, T), arm: T| -> Result<[Sign; 2], SplitJoinError> {
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
                    // arrival; the other is read.
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
                        let [p0, p1] = in_span(t, p.span, p.radius)?;
                        let [a0, a1] = in_span(s, span, b)?;
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
                    let [p0, p1] = in_span(t, p.span, p.radius)?;
                    let [a0, a1] = in_span(s, span, T::one())?;
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
                    let [a0, a1] = in_span(s, span, b)?;
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
pub(crate) mod cone_islands {
    //! Islands on a cone, each with its membership oracle: the fixtures
    //! the parity rows here and the ring-lane rows in
    //! `chord_join::cone_ring_rows` share.

    use core::f64::consts::{FRAC_PI_6, PI};
    use geom_core::{Band, Point3, Tol, Vec3};

    pub(crate) fn band() -> Band {
        Band::linear(Tol::witness()).unwrap()
    }

    /// A rigid frame: the cone's local `x`, `y`, `z` and its apex.
    #[derive(Clone, Copy)]
    pub(crate) struct Frame {
        pub(crate) x: Vec3<f64>,
        pub(crate) y: Vec3<f64>,
        pub(crate) z: Vec3<f64>,
        pub(crate) o: Point3<f64>,
    }

    impl Frame {
        pub(crate) fn at(&self, x: f64, y: f64, z: f64) -> Point3<f64> {
            self.o + self.dir(x, y, z)
        }
        pub(crate) fn dir(&self, x: f64, y: f64, z: f64) -> Vec3<f64> {
            self.x * x + self.y * y + self.z * z
        }
    }

    /// The identity, a frame turned off every axis, and one with its
    /// handedness kept but its axis reversed (`z ↦ −z`, `y ↦ −y`).
    pub(crate) fn frames() -> [Frame; 3] {
        let id = Frame {
            x: Vec3::unit_x(),
            y: Vec3::unit_y(),
            z: Vec3::unit_z(),
            o: Point3::origin(),
        };
        let z = Vec3::new(0.3, -0.5, 0.8).normalize();
        let x = Vec3::new(1.0, 0.2, 0.0).cross(z).normalize();
        let turned = Frame {
            x,
            y: z.cross(x),
            z,
            o: Point3::new(0.7, -1.1, 2.3),
        };
        let flipped = Frame {
            x: Vec3::unit_x(),
            y: -Vec3::unit_y(),
            z: -Vec3::unit_z(),
            o: Point3::new(-0.4, 0.2, 0.1),
        };
        [id, turned, flipped]
    }

    /// The cone of half-angle π/6 at the frame's apex, opening along its
    /// `z` — or along `−z` (`mirror`), so that the points of
    /// [`on_nappe`] lie on the carrier's negative nappe.
    pub(crate) fn cone(f: Frame, mirror: bool) -> geom::Surface<f64> {
        geom::Surface::Cone {
            apex: f.o,
            axis: if mirror { -f.z } else { f.z },
            half_angle: FRAC_PI_6,
            u_ref: f.x,
        }
    }

    /// The point of the nappe along local `+z` at height `h`, azimuth `t`.
    pub(crate) fn on_nappe(f: Frame, h: f64, t: f64) -> Point3<f64> {
        let r = h * FRAC_PI_6.tan();
        f.at(r * t.cos(), r * t.sin(), h)
    }

    /// The plane through `o` with normal `n`.
    pub(crate) fn plane(o: Point3<f64>, n: Vec3<f64>) -> geom::Surface<f64> {
        geom::Surface::Plane {
            origin: o,
            normal: n,
            u_ref: n.orthonormal_basis().0,
        }
    }

    /// The section of `cone` by the plane through `o` with normal `n`.
    fn section(cone: &geom::Surface<f64>, o: Point3<f64>, n: Vec3<f64>) -> geom::Curve3<f64> {
        match geom_brep::plane_cone_section(&plane(o, n), cone, 4.0, band()).unwrap() {
            geom_brep::PlaneConeSection::TiltedEllipse(c)
            | geom_brep::PlaneConeSection::AxisNormalCircle(c) => c,
            other => panic!("the fixture's planes cut ellipses: {other:?}"),
        }
    }

    /// The conic parameter of `p`, a point of `c`.
    fn conic_param(c: &geom::Curve3<f64>, p: Point3<f64>) -> f64 {
        let (center, axis, u, a, b) = match *c {
            geom::Curve3::Ellipse {
                center,
                axis,
                major,
                minor,
                u_ref,
            } => (center, axis, u_ref, major, minor),
            geom::Curve3::Circle {
                center,
                axis,
                radius,
                u_ref,
            } => (center, axis, u_ref, radius, radius),
            _ => unreachable!(),
        };
        let w = p - center;
        (w.dot(axis.cross(u)) / b).atan2(w.dot(u) / a)
    }

    /// One edge of an [`Island`], travelled from its corner to the next:
    /// `carrier` from `params.0` to `params.1` (either way), and the
    /// plane it is the section of, `None` for a ruling.
    #[derive(Clone)]
    pub(crate) struct IslandEdge {
        pub(crate) carrier: geom::Curve3<f64>,
        pub(crate) params: (f64, f64),
        pub(crate) plane: Option<(Point3<f64>, Vec3<f64>)>,
    }

    /// The arc of the section by `(o, n)` from `x0` to `x1` whose
    /// midpoint `keep` holds.
    fn arc_between(
        cone: &geom::Surface<f64>,
        (o, n): (Point3<f64>, Vec3<f64>),
        (x0, x1): (Point3<f64>, Point3<f64>),
        keep: &dyn Fn(Point3<f64>) -> bool,
    ) -> IslandEdge {
        let c = section(cone, o, n);
        let (t0, mut t1) = (conic_param(&c, x0), conic_param(&c, x1));
        if t1 < t0 {
            t1 += 2.0 * PI;
        }
        if !keep(c.mid_point(t0, t1)) {
            t1 -= 2.0 * PI;
        }
        assert!(keep(c.mid_point(t0, t1)), "one of the two arcs is kept");
        IslandEdge {
            carrier: c,
            params: (t0, t1),
            plane: Some((o, n)),
        }
    }

    /// The ruling segment from `x0` to `x1`, both on one ruling of the
    /// cone at `apex`.
    fn ruling(apex: Point3<f64>, (x0, x1): (Point3<f64>, Point3<f64>)) -> IslandEdge {
        let dir = (x1 - x0).normalize();
        IslandEdge {
            carrier: geom::Curve3::Line { origin: apex, dir },
            params: ((x0 - apex).dot(dir), (x1 - apex).dot(dir)),
            plane: None,
        }
    }

    /// A closed loop on a cone: its corners, the edge leaving each, and
    /// its membership oracle.
    pub(crate) struct Island {
        pub(crate) corners: Vec<Point3<f64>>,
        pub(crate) edges: Vec<IslandEdge>,
        pub(crate) inside: Box<dyn Fn(Point3<f64>) -> bool>,
    }

    /// **A lune between two ellipses**, scaled by `s` about the apex:
    /// the box quadrant `n̂₁·x > c₁, n̂₂·x > c₂` (perpendicular normals
    /// 40° and 50° off the axis), whose edge pierces the nappe at
    /// `s·(0.5, ±0.707, 1.5)` — the far side of one section and the
    /// apex side of the other. At `s = 1` it spans heights 1.3 to 2.9.
    pub(crate) fn lune(f: Frame, cone: &geom::Surface<f64>, s: f64) -> Island {
        let b = 40f64.to_radians();
        let n1 = f.dir(b.sin(), 0.0, b.cos());
        let n2 = f.dir(b.cos(), 0.0, -b.sin());
        let e = f.at(0.5 * s, 0.0, 1.5 * s);
        let (c1, c2) = (n1.dot(e - f.o), n2.dot(e - f.o));
        let y = (0.75 - 0.25f64).sqrt() * s;
        let (xa, xb) = (f.at(0.5 * s, y, 1.5 * s), f.at(0.5 * s, -y, 1.5 * s));
        let side = move |n: Vec3<f64>, c: f64| move |p: Point3<f64>| n.dot(p - f.o) > c;
        Island {
            corners: vec![xa, xb],
            edges: vec![
                arc_between(cone, (e, n1), (xa, xb), &side(n2, c2)),
                arc_between(cone, (e, n2), (xb, xa), &side(n1, c1)),
            ],
            inside: Box::new(move |p| side(n1, c1)(p) && side(n2, c2)(p)),
        }
    }

    /// **A sector of the band between a parallel and an ellipse**: below
    /// the circle at height 2.2, above the section by the plane 30° off
    /// the axis at offset 0.6, and between the rulings at azimuths −0.6
    /// and 0.9 — two arcs and two rulings, from the corner on the circle
    /// at −0.6.
    pub(crate) fn sector(f: Frame, cone: &geom::Surface<f64>) -> Island {
        let (ta, tb, top) = (-0.6f64, 0.9f64, 2.2);
        let n = f.dir(0.5, 0.0, 0.75f64.sqrt());
        let lowest = |t: f64| 0.6 / n.dot(on_nappe(f, 1.0, t) - f.o);
        let azimuth = move |p: Point3<f64>| {
            let w = p - f.o;
            w.dot(f.y).atan2(w.dot(f.x))
        };
        let between = move |p: Point3<f64>| (ta..tb).contains(&azimuth(p));
        let corners = vec![
            on_nappe(f, top, ta),
            on_nappe(f, top, tb),
            on_nappe(f, lowest(tb), tb),
            on_nappe(f, lowest(ta), ta),
        ];
        let edges = vec![
            arc_between(
                cone,
                (f.at(0.0, 0.0, top), f.z),
                (corners[0], corners[1]),
                &between,
            ),
            ruling(f.o, (corners[1], corners[2])),
            arc_between(cone, (f.o + n * 0.6, n), (corners[2], corners[3]), &between),
            ruling(f.o, (corners[3], corners[0])),
        ];
        Island {
            corners,
            edges,
            inside: Box::new(move |p| {
                between(p) && n.dot(p - f.o) > 0.6 && (p - f.o).dot(f.z) < top
            }),
        }
    }
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
                                let got =
                                    path_parity(FaceKey::default(), &path, &arcs, None, band())
                                        .unwrap();
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
}
