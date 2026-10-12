//! **A ring lane's chart-free reading on a sphere, a cone or a torus
//! face**: the
//! parity of a path on the face's surface across a closed curve of loop
//! arcs, which says whether the path's ends lie on one side of the curve
//! or on opposite sides.
//!
//! The path is made of pieces whose meeting with a loop arc is closed
//! form: on a sphere, the short great-circle arc between its ends; on a
//! cone, a segment of a ruling and an arc of a parallel (the circle at
//! one height), in either order, on the ends' nappe and never through
//! the apex; on a torus, an arc of a meridian (the tube's circle at one
//! azimuth) and an arc of a parallel, in either order, after a segment
//! that moves the start onto the tube through the end, so the pieces
//! meet exactly. Every loop arc is a plane section of the surface (a
//! circle or an ellipse: no lane mints another conic) or, on a cone, a
//! ruling. So a piece meets a section arc only where it crosses the
//! arc's plane, which a circle piece does at most twice and a segment
//! once, and it meets a ruling only where the ruling crosses a
//! parallel's plane. Two rulings meet only at the apex, which no piece
//! reaches.
//!
//! A plane cuts a torus in up to two loops, so on a torus a crossing of
//! an arc's plane lies on the arc's own circle only on that circle's
//! side of the split between the two ([`ArcLoop`]): a loop arc on a
//! torus is a meridian or a parallel, whose plane's other loop is the
//! meridian across the axis or the parallel across the tube. The split
//! is decided at the crossing, **`split_ring_path_torus_loop`**, as the
//! boolean join's section loops are told apart.
//!
//! The tube's top and bottom circles (the parallels of radius `R`) are
//! the exception: their plane touches the torus along them, so a path
//! meets it only on the circle and never crosses it. They are read
//! across the cylinder `ρ = R` about the axis instead, which cuts the
//! torus transversally in exactly those two circles, told apart by their
//! height along the axis ([`LoopArc::Crest`]). A parallel piece keeps its
//! distance from the axis and a meridian piece crosses that cylinder at
//! the top and the bottom of the tube, through the plane through its
//! centre square to its radial, so each crossing is closed form too.
//!
//! The parity does not depend on the path because the curve is
//! null-homotopic where the paths run. Every closed curve on a sphere
//! is. On a cone, the paths run on the punctured nappe, an annulus, and
//! the curve bounds a disc of it: it lies on one face, and no face winds
//! round the axis — at rest, tier 3's pcurve mint refuses a face whose
//! outer loop spans a full period or whose loop wraps
//! (`PcurveMintError::{OuterSpansPeriod, LoopWraps}`), so a face that
//! reaches round the axis is cut by its seam, and no curve on it winds
//! round the apex. On a torus, the paths run anywhere on it and the
//! curve bounds a disc: it lies on one face, and tier 3's pcurve mint
//! refuses a loop whose chart walk closes only by a period in either
//! direction (`LoopWraps`), so a face lifts to a disc of the chart
//! plane and a closed curve on it lifts closed. The parity of a path
//! across a curve that bounds is its ends' sides of the disc, whichever
//! way round the tube or the axis it goes.
//!
//! Each decision is a named trilean metered in metres. The raw picks
//! that remain — the nearer root on the arrival arc, a parameter's
//! branch, the way round a parallel — are choices every outcome of which
//! is sound, each argued at its site. A crossing is
//! passed over once any reading is decided against it, whatever its
//! other readings say: a decided rejection is decided. A reading of a
//! crossing not passed over that lands in the zero band — the path
//! grazes an arc, runs through an arc's end, or starts on an arc's
//! plane — makes the path undecided, one in the escalation band makes
//! it escalate, and the caller asks another.

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
    Torus {
        centre: Point3<T>,
        axis: Vec3<T>,
        major: T,
        minor: T,
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
            geom::Surface::Torus {
                center,
                axis,
                major_radius,
                minor_radius,
                ..
            } => Some(Self::Torus {
                centre: center,
                axis,
                major: major_radius,
                minor: minor_radius,
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
    /// sphere's radius, the radius of the cone's parallel through `p`, or
    /// the torus's tube radius or its parallel's through `p`, whichever
    /// is smaller.
    pub(crate) fn lever(&self, p: Point3<T>) -> T {
        match *self {
            Self::Sphere { radius, .. } => radius,
            Self::Cone { apex, axis, .. } => Self::axial(apex, axis, p).1.norm(),
            Self::Torus {
                centre,
                axis,
                minor,
                ..
            } => minor.min(Self::axial(centre, axis, p).1.norm()),
        }
    }

    /// The chart normal at `p` (the outward one for a face of sense
    /// `true`): the sphere's radial direction, or the cone's
    /// `ŵ·cos α − â·sin α·sgn h`, `ŵ` the radial direction. `p` is off a
    /// cone's apex ([`Self::off_apex`]); on a torus, the direction from
    /// the core circle's nearest point, off the axis and the core.
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
                w / w.norm() * c - axis * h.select_le_zero(-s, s)
            }
            Self::Torus {
                centre,
                axis,
                major,
                ..
            } => {
                let tube = p - Self::core(centre, axis, major, p);
                tube / tube.norm()
            }
        }
    }

    /// The core circle's point at `p`'s azimuth, `p` off the axis.
    fn core(centre: Point3<T>, axis: Vec3<T>, major: T, p: Point3<T>) -> Point3<T> {
        let w = Self::axial(centre, axis, p).1;
        centre + w / w.norm() * major
    }

    /// The outward normal at `p` of a face of sense `sense`
    /// ([`Self::chart_normal`] folded by the sense).
    pub(crate) fn outward(&self, p: Point3<T>, sense: bool) -> Vec3<T> {
        geom_brep::OutwardNormal::from_chart(self.chart_normal(p), sense).vec()
    }

    /// Whether `p` is decided off a cone's apex or a torus's axis (its
    /// distance from the axis, **`split_ring_path_off_apex`**): always on
    /// a sphere.
    pub(crate) fn off_apex(&self, p: Point3<T>, band: Band) -> Result<bool, Indeterminate> {
        match *self {
            Self::Sphere { .. } => Ok(true),
            Self::Cone { apex, axis, .. }
            | Self::Torus {
                centre: apex, axis, ..
            } => Ok(decide(
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
    /// - A torus's are two, from ends decided off the axis and off the
    ///   core circle (**`split_ring_path_off_core`**: the distance from
    ///   it). Each first moves `from` along its tube radius onto the
    ///   tube through `to` (a segment, as short as the ends' offsets off
    ///   the carrier), then runs round that tube's meridian through the
    ///   start to `to`'s parallel and round it, arriving along the
    ///   parallel; or round the start's parallel to `to`'s meridian and
    ///   round it, arriving along the meridian. A meridian loop arc
    ///   holds the first's arrival transversal and a parallel the
    ///   second's.
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
                let turned = apex + axis * hb + ua * rb.norm();
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
                let below = apex + axis * ha + ub * ra.norm();
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
            Self::Torus {
                centre,
                axis,
                major,
                ..
            } => {
                if !self.off_apex(from, band)? || !self.off_apex(to, band)? {
                    return Ok(Vec::new());
                }
                let core = |p| Self::core(centre, axis, major, p);
                let (kf, kt) = (core(from), core(to));
                let (off_f, off_t) = ((from - kf).norm(), (to - kt).norm());
                for off in [off_f, off_t] {
                    if decide("split_ring_path_off_core", Margin::of(off), band)? != Sign::Positive
                    {
                        return Ok(Vec::new());
                    }
                }
                // Every piece lies on the tube of radius `off_t` about the
                // core, through `to`: `from` is moved onto it first, along
                // its own tube radius, so each turn is placed on the circle
                // the next piece runs round and the pieces meet exactly,
                // as the cone's do.
                let start = kf + (from - kf) * (off_t / off_f);
                // The ruling and lever are read only against a `Line` loop
                // arc, and no loop arc on a torus is one
                // ([`Self::torus_circle_arc`] yields conics and crests).
                let lead = Piece::Segment {
                    from,
                    to: start,
                    ruling: Vec3::zero(),
                    lever: T::zero(),
                };
                let radial = |k: Point3<T>| (k - centre) / major;
                let (ef, et) = (radial(kf), radial(kt));
                let (hs, ws) = Self::axial(centre, axis, start);
                let (ht, wt) = Self::axial(centre, axis, to);
                let (rs, rt) = (ws.norm(), wt.norm());
                let mut paths = Vec::with_capacity(2);
                // Round the start's meridian to `to`'s parallel, then round.
                let turned = centre + axis * ht + ef * rt;
                let meridian = parallel(kf, ef.cross(axis), off_t, start, turned);
                let round = parallel(centre + axis * ht, axis, rt, turned, to);
                paths.push(Path {
                    pieces: vec![lead, Piece::Meridian(meridian), Piece::Arc(round)],
                    arrival: round.tangent(round.span.1),
                });
                // Round the start's parallel to `to`'s meridian, then round.
                let turned = centre + axis * hs + et * rs;
                let round = parallel(centre + axis * hs, axis, rs, start, turned);
                let meridian = parallel(kt, et.cross(axis), off_t, turned, to);
                paths.push(Path {
                    pieces: vec![lead, Piece::Arc(round), Piece::Meridian(meridian)],
                    arrival: meridian.tangent(meridian.span.1),
                });
                Ok(paths)
            }
        }
    }
}

/// The arc of the circle centred `centre` about the unit `axis`, of
/// radius `radius`, from `from` to `to` (both in its plane), the short
/// way round: a parallel, or a torus's meridian.
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
        axis: axis * phi.select_le_zero(-T::one(), T::one()),
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
    /// An arc of a great circle, or of a parallel of a cone or a torus.
    Arc(CircleArc<T>),
    /// An arc of a torus's meridian (a circle round the tube, in a plane
    /// holding the axis).
    Meridian(CircleArc<T>),
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

/// An arc of a loop on a sphere, cone or torus face, over its
/// increasing span: a section conic
/// (`centre + u_ref·a·cos t + (axis × u_ref)·b·sin t`, `a = b` on a
/// circle) or a ruling (`origin + dir·t`).
#[derive(Clone, Copy)]
pub(crate) enum LoopArc<T: Real> {
    Conic {
        centre: Point3<T>,
        axis: Vec3<T>,
        u_ref: Vec3<T>,
        a: T,
        b: T,
        span: (T, T),
        /// On a torus, the side of its plane's two loops the conic's own
        /// circle lies on; `None` where the plane cuts the surface in the
        /// conic alone.
        own: Option<ArcLoop<T>>,
    },
    Line {
        origin: Point3<T>,
        dir: Vec3<T>,
        span: (T, T),
    },
    /// An arc of a torus's top or bottom circle, the parallel of radius
    /// `R` (`centre + (u_ref·cos t + (axis × u_ref)·sin t)·radius`),
    /// whose plane touches the torus along it. A path meets it only where
    /// it crosses the cylinder `ρ = R` about the torus's axis, which cuts
    /// the torus transversally in this circle and the one across the tube
    /// (`own` tells them apart by the height along the axis).
    Crest {
        centre: Point3<T>,
        axis: Vec3<T>,
        u_ref: Vec3<T>,
        radius: T,
        span: (T, T),
        /// The torus's centre, unit axis and major radius `R`.
        core: (Point3<T>, Vec3<T>, T),
        own: ArcLoop<T>,
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
                own: None,
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
                own: None,
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

/// **Which of two loops of one plane section a point lies on**: the sign
/// of a margin in metres, definite at every point of a served section.
/// The boolean join reads it at a germ site (`boolean::join`'s
/// `SectionLoops`), and the ring lane at a crossing of a torus loop
/// arc's plane ([`ArcLoop`]).
#[derive(Clone, Copy, Debug)]
pub(crate) enum LoopSplit<T: Real> {
    /// `(p − origin)·dir`, `dir` unit.
    Side {
        /// A point of the plane between the loops.
        origin: Point3<T>,
        /// That plane's unit normal.
        dir: Vec3<T>,
    },
    /// `p`'s distance from the axis `(origin, axis)` less `radius`,
    /// `axis` unit.
    Radius {
        /// A point of the axis.
        origin: Point3<T>,
        /// The axis.
        axis: Vec3<T>,
        /// The radius between the loops.
        radius: T,
    },
}

impl<T: Real> LoopSplit<T> {
    /// The margin at `p`: positive on the split's positive loop.
    pub(crate) fn margin(&self, p: Point3<T>) -> T {
        match *self {
            Self::Side { origin, dir } => (p - origin).dot(dir),
            Self::Radius {
                origin,
                axis,
                radius,
            } => (p - origin).cross(axis).norm() - radius,
        }
    }

    /// **The split of a plane×torus section's two loops**, the one
    /// reading the boolean join's frame (`boolean::join`'s
    /// `pt_germ_frame`) and the chord (`chord_join::section_case`) both
    /// take, so that they name one loop. The torus is `(centre, axis,
    /// R)` and `n` the plane's normal.
    ///
    /// - The meridian circles and the spiric ovals: the side of the
    ///   plane through the axis square to the cut, the first loop on the
    ///   positive side.
    /// - The concentric circles: the distance from the axis against `R`,
    ///   the outer circle positive.
    ///
    /// `None` for a section with no two loops, or a meridian section
    /// whose first loop is no circle.
    pub(crate) fn plane_torus(
        section: &geom_brep::PlaneTorusSection<T>,
        (centre, axis, major): (Point3<T>, Vec3<T>, T),
        n: Vec3<T>,
    ) -> Option<Self> {
        match section {
            geom_brep::PlaneTorusSection::MeridianCircles {
                c1: geom::Curve3::Circle { center, .. },
                ..
            } => Some(Self::Side {
                origin: centre,
                dir: (*center - centre).normalize(),
            }),
            geom_brep::PlaneTorusSection::SpiricOvals { .. } => Some(Self::Side {
                origin: centre,
                dir: axis.cross(n).normalize(),
            }),
            geom_brep::PlaneTorusSection::ConcentricCircles { .. } => Some(Self::Radius {
                origin: centre,
                axis: axis.normalize(),
                radius: major,
            }),
            geom_brep::PlaneTorusSection::MeridianCircles { .. }
            | geom_brep::PlaneTorusSection::TangentCircle(_)
            | geom_brep::PlaneTorusSection::Empty => None,
        }
    }
}

/// **Which of its plane's two loops on a torus a loop arc's circle is**:
/// the split between them and the circle's side of it. A meridian's
/// plane holds the axis and cuts the meridian across it too, `2(R − r)`
/// or more away: the split is the plane through the axis square to it,
/// the circle on the side of `ê`, toward its centre. A parallel's plane
/// cuts the parallel across the tube: the split is the core radius `R`,
/// the outer parallel on its positive side. Both are the boolean join's
/// loop splits for those sections.
#[derive(Clone, Copy)]
pub(crate) struct ArcLoop<T: Real> {
    split: LoopSplit<T>,
    /// Whether the arc's circle is on the split's positive side.
    positive: bool,
}

impl<T: Decide> ArcLoop<T> {
    /// **`split_ring_path_torus_loop`** at `p`: positive on the arc's
    /// own loop.
    fn read(&self, p: Point3<T>, band: Band) -> Result<Sign, Indeterminate> {
        let m = self.split.margin(p);
        decide(
            "split_ring_path_torus_loop",
            Margin::of(if self.positive { m } else { -m }),
            band,
        )
    }
}

impl<T: Decide> Quadric<T> {
    /// **The loop arc of a circle on this torus**, over `params`: a
    /// meridian or a parallel, with its [`ArcLoop`]; `Ok(None)` for any
    /// other circle (no plane section of a torus outside the touching
    /// poses is another circle) and on a sphere or a cone. The circle's
    /// axis is decided square to the torus's
    /// (**`split_ring_torus_meridian`**: `n̂·â`, levered at the circle's
    /// radius) or along it (**`split_ring_torus_parallel`**:
    /// `‖n̂ × â‖`, levered at the radius), and a parallel's side, its
    /// radius less `R` (**`split_ring_torus_parallel_side`**). A parallel
    /// of radius `R` in that band is the tube's top or bottom circle,
    /// whose plane touches the torus: its arc is a
    /// [`LoopArc::Crest`], told apart from the other by its centre's
    /// height (**`split_ring_torus_crest`**).
    pub(crate) fn torus_circle_arc(
        &self,
        carrier: &geom::Curve3<T>,
        params: (T, T),
        band: Band,
    ) -> Result<Option<LoopArc<T>>, Indeterminate> {
        let (
            Self::Torus {
                centre,
                axis,
                major,
                ..
            },
            geom::Curve3::Circle {
                center: k,
                axis: n,
                radius,
                ..
            },
        ) = (*self, carrier)
        else {
            return Ok(None);
        };
        let (k, n, radius) = (*k, *n, *radius);
        let Some(LoopArc::Conic {
            centre: c,
            axis: a,
            u_ref,
            a: ra,
            b: rb,
            span,
            ..
        }) = LoopArc::of(carrier, params)
        else {
            return Ok(None);
        };
        let arc = |own| LoopArc::Conic {
            centre: c,
            axis: a,
            u_ref,
            a: ra,
            b: rb,
            span,
            own: Some(own),
        };
        if decide(
            "split_ring_torus_meridian",
            Margin::levered(n.dot(axis), radius),
            band,
        )? == Sign::Zero
        {
            let w = k - centre;
            let w = w - axis * w.dot(axis);
            return Ok(Some(arc(ArcLoop {
                split: LoopSplit::Side {
                    origin: centre,
                    dir: w / w.norm(),
                },
                positive: true,
            })));
        }
        if decide(
            "split_ring_torus_parallel",
            Margin::levered(n.cross(axis).norm(), radius),
            band,
        )? != Sign::Zero
        {
            return Ok(None);
        }
        let positive = match decide(
            "split_ring_torus_parallel_side",
            Margin::of(radius - major),
            band,
        )? {
            Sign::Positive => true,
            Sign::Negative => false,
            Sign::Zero => {
                // The tube's top or bottom circle: its side of the
                // equator plane (**`split_ring_torus_crest`**, the
                // centre's height, `±r` on the torus).
                let top = match decide(
                    "split_ring_torus_crest",
                    Margin::of((k - centre).dot(axis)),
                    band,
                )? {
                    Sign::Positive => true,
                    Sign::Negative => false,
                    Sign::Zero => return Ok(None),
                };
                return Ok(Some(LoopArc::Crest {
                    centre: c,
                    axis: a,
                    u_ref,
                    radius: ra,
                    span,
                    core: (centre, axis, major),
                    own: ArcLoop {
                        split: LoopSplit::Side {
                            origin: centre,
                            dir: axis,
                        },
                        positive: top,
                    },
                }));
            }
        };
        Ok(Some(arc(ArcLoop {
            split: LoopSplit::Radius {
                origin: centre,
                axis,
                radius: major,
            },
            positive,
        })))
    }
}

/// **Whether `path` crosses `arcs` an odd number of times**: whether its
/// ends lie on opposite sides of the closed curve the arcs make (module
/// docs). `arrives_on` names the arc the path's end lies on, at an
/// interior point; its crossing there is the arrival and is not
/// counted. `None` when a reading of a crossing not passed over lands
/// in the zero band; `Err` when one lands in the escalation band, which
/// says nothing about the parity either: the caller asks another path. A crossing is passed
/// over when any of its in-span readings is decided out of span, though
/// another escalate.
///
/// - A circle piece meets a conic's plane where
///   `A cos θ + B sin θ = −D/r` (`A`, `B` the plane normal's components
///   on the piece's frame, `D` the piece's centre's offset from the
///   plane): at two parameters when `r·√(A² + B²) > |D|`
///   (**`split_ring_path_meets_plane`**, on the gap `r·√(A² + B²) − |D|`
///   in metres: moving the plane by it flips the meeting). A graze inside
///   the gap's band escalates, and the caller asks the next path. On the
///   arrival arc, the root nearer the piece's end is the arrival. A gap
///   in the zero band is a piece along the plane, which meets no arc it
///   is decided apart from ([`misses_in_plane`]): a path's meridian or
///   parallel through a point of an arc's carrier, or of its plane's
///   other loop, runs so.
/// - A segment meets a conic's plane where its ends' offsets from it
///   (**`split_ring_path_segment_side`**) differ in sign. An end on the
///   plane says nothing unless the segment is decided apart from the arc
///   (**`split_ring_path_segment_apart`**: its midpoint's distance from
///   the arc less half its length), as a torus path's lead from a point
///   on an arc's plane is.
/// - A ruling meets a circle piece's plane once (it is decided across
///   it, **`split_ring_path_line_across`**: the cosine between the
///   plane's normal and the ruling, levered at the piece's radius), and
///   another ruling nowhere off the apex (the two are decided apart,
///   **`split_ring_path_rulings_apart`**: the sine between them,
///   levered at the segment's distance from the apex).
/// - On a torus, a meeting point counts only on the arc's own loop of
///   its plane ([`ArcLoop`], **`split_ring_path_torus_loop`**).
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
    // A crossing's in-span readings are kept whole, escalations included:
    // one decided out of span passes the crossing over whatever the
    // others read.
    let in_span = |t: T, (lo, hi): (T, T), arm: T| -> [Result<Sign, Indeterminate>; 2] {
        [
            decide_m("split_ring_path_in_span", Margin::levered(t - lo, arm)),
            decide_m("split_ring_path_in_span", Margin::levered(hi - t, arm)),
        ]
    };
    let inside = Ok(Sign::Positive);
    // A crossing of a torus arc's plane on the plane's other loop is no
    // crossing of the arc: its side is one more reading of the crossing.
    let on_own = |own: Option<ArcLoop<T>>, x: Point3<T>| own.map_or(inside, |l| l.read(x, band));
    let mut odd = false;
    let last = path.pieces.len().saturating_sub(1);
    for (i, piece) in path.pieces.iter().enumerate() {
        for (j, arc) in arcs.iter().enumerate() {
            let arrival = i == last && arrives_on == Some(j);
            let mut readings: Vec<[Result<Sign, Indeterminate>; 5]> = Vec::with_capacity(2);
            match (*piece, *arc) {
                (
                    Piece::Arc(p) | Piece::Meridian(p),
                    LoopArc::Conic {
                        centre,
                        axis,
                        u_ref,
                        a,
                        b,
                        span,
                        own,
                    },
                ) => {
                    let Some(roots) = plane_roots(&p, (centre, axis), arrival, band)? else {
                        let conic = Conic {
                            centre,
                            axis,
                            u_ref,
                            a,
                            b,
                            span,
                        };
                        if !arrival && misses_in_plane(&p, &conic, band)? {
                            continue;
                        }
                        return Ok(None);
                    };
                    let v_ref = axis.cross(u_ref);
                    for (t, slope) in roots {
                        let w = p.at(t) - centre;
                        let s = branch((w.dot(v_ref) / b).atan2(w.dot(u_ref) / a), span);
                        let [p0, p1] = in_span(t, p.span, p.radius * slope);
                        let [a0, a1] = in_span(s, span, b * slope);
                        readings.push([p0, p1, a0, a1, on_own(own, p.at(t))]);
                    }
                }
                // A crest is a torus's, whose paths' arcs are parallels
                // (`Piece::Arc`) and meridians. A parallel keeps its
                // distance from the axis, so it meets the cylinder
                // `ρ = R` nowhere unless it runs along it
                // (**`split_ring_path_crest_graze`**: its radius less `R`).
                (
                    Piece::Arc(p),
                    LoopArc::Crest {
                        core: (.., major), ..
                    },
                ) => {
                    if decide_m("split_ring_path_crest_graze", Margin::of(p.radius - major))?
                        == Sign::Zero
                    {
                        return Ok(None);
                    }
                }
                // A meridian lies in a plane holding the axis, where
                // `ρ − R` is the offset from its centre on the core along
                // the radial `ê`: it crosses the cylinder `ρ = R` where it
                // crosses the plane through its centre square to `ê`, on
                // the tube's top and bottom.
                (
                    Piece::Meridian(p),
                    LoopArc::Crest {
                        centre,
                        axis,
                        u_ref,
                        radius,
                        span,
                        core: (c, a, _),
                        own,
                    },
                ) => {
                    let w = p.centre - c;
                    let e = w - a * w.dot(a);
                    let e = e / e.norm();
                    let Some(roots) = plane_roots(&p, (p.centre, e), arrival, band)? else {
                        return Ok(None);
                    };
                    let v_ref = axis.cross(u_ref);
                    for (t, slope) in roots {
                        let w = p.at(t) - centre;
                        let s = branch(w.dot(v_ref).atan2(w.dot(u_ref)), span);
                        let [p0, p1] = in_span(t, p.span, p.radius * slope);
                        let [a0, a1] = in_span(s, span, radius * slope);
                        readings.push([p0, p1, a0, a1, own.read(p.at(t), band)]);
                    }
                }
                // The torus's one segment is the lead along a tube radius,
                // in a meridian half-plane, where `ρ − R` is a multiple of
                // its offset from the core point at its start, so it keeps
                // its sign (**`split_ring_path_crest_side`**). A segment
                // whose ends are decided across the cylinder is no lead,
                // and says nothing.
                (
                    Piece::Segment { from, to, .. },
                    LoopArc::Crest {
                        core: (c, a, major),
                        ..
                    },
                ) => {
                    if arrival {
                        continue;
                    }
                    let rho_less = |p: Point3<T>| {
                        let w = p - c;
                        (w - a * w.dot(a)).norm() - major
                    };
                    let s0 = decide_m("split_ring_path_crest_side", Margin::of(rho_less(from)))?;
                    let s1 = decide_m("split_ring_path_crest_side", Margin::of(rho_less(to)))?;
                    if s0 == Sign::Zero || s1 == Sign::Zero || s0 != s1 {
                        return Ok(None);
                    }
                }
                (Piece::Arc(p) | Piece::Meridian(p), LoopArc::Line { origin, dir, span }) => {
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
                    let slope = across.abs();
                    let [p0, p1] = in_span(t, p.span, p.radius * slope);
                    let [a0, a1] = in_span(s, span, slope);
                    readings.push([p0, p1, a0, a1, inside]);
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
                        own,
                    },
                ) => {
                    if arrival {
                        continue;
                    }
                    let (d0, d1) = (axis.dot(from - centre), axis.dot(to - centre));
                    let s0 = decide_m("split_ring_path_segment_side", Margin::of(d0))?;
                    let s1 = decide_m("split_ring_path_segment_side", Margin::of(d1))?;
                    if s0 == Sign::Zero || s1 == Sign::Zero {
                        let conic = Conic {
                            centre,
                            axis,
                            u_ref,
                            a,
                            b,
                            span,
                        };
                        let half = (to - from).norm() * T::from_f64(0.5);
                        let mid = from + (to - from) * T::from_f64(0.5);
                        if decide_m(
                            "split_ring_path_segment_apart",
                            Margin::of(conic.distance_floor(mid) - half),
                        )? == Sign::Positive
                        {
                            continue;
                        }
                        return Ok(None);
                    }
                    if s0 == s1 {
                        continue;
                    }
                    let x = from + (to - from) * (d0 / (d0 - d1));
                    let w = x - centre;
                    let v_ref = axis.cross(u_ref);
                    let s = branch((w.dot(v_ref) / b).atan2(w.dot(u_ref) / a), span);
                    // The plane moved by δ moves the root by δ over the
                    // slope the segment crosses it at, `|n̂·d̂|`.
                    let slope = ((d0 - d1) / (to - from).norm()).abs();
                    let [a0, a1] = in_span(s, span, b * slope);
                    readings.push([inside, inside, a0, a1, on_own(own, x)]);
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
                if reading.contains(&Ok(Sign::Negative)) {
                    continue;
                }
                if let Some(&Err(diag)) = reading.iter().find(|r| r.is_err()) {
                    return Err(diag);
                }
                if reading.contains(&Ok(Sign::Zero)) {
                    return Ok(None);
                }
                odd = !odd;
            }
        }
    }
    Ok(Some(odd))
}

/// A loop conic's arc, as [`LoopArc::Conic`] holds it.
struct Conic<T: Real> {
    centre: Point3<T>,
    axis: Vec3<T>,
    u_ref: Vec3<T>,
    a: T,
    b: T,
    span: (T, T),
}

impl<T: Decide> Conic<T> {
    fn v_ref(&self) -> Vec3<T> {
        self.axis.cross(self.u_ref)
    }

    /// The arc's point at its parameter `s`.
    fn at(&self, s: T) -> Point3<T> {
        let (sn, cs) = s.sin_cos();
        self.centre + self.u_ref * (self.a * cs) + self.v_ref() * (self.b * sn)
    }

    /// The arc's parameter of `x`'s direction from its centre, on the
    /// span's branch.
    fn param(&self, x: Point3<T>) -> T {
        let w = x - self.centre;
        branch(
            (w.dot(self.v_ref()) / self.b).atan2(w.dot(self.u_ref) / self.a),
            self.span,
        )
    }

    /// A lower bound on the distance from `x` to the arc: the distance
    /// to the arc of the circle of radius `b` over the same span, less
    /// `|a − b|`, which bounds how far a conic point lies from the
    /// circle's at the same parameter. The circle arc's nearest point is
    /// at `x`'s direction clamped to the span: on the span's branch the
    /// end on `x`'s side is the nearer.
    fn distance_floor(&self, x: Point3<T>) -> T {
        let w = x - self.centre;
        let s = branch(w.dot(self.v_ref()).atan2(w.dot(self.u_ref)), self.span);
        let s = s.max(self.span.0).min(self.span.1);
        let (sn, cs) = s.sin_cos();
        let nearest = self.centre + (self.u_ref * cs + self.v_ref() * sn) * self.b;
        (x - nearest).norm() - (self.a - self.b).abs()
    }
}

/// **Whether the circle piece `p`, which [`plane_roots`] cannot cut
/// with `conic`'s plane, misses the arc**: it runs along that plane,
/// as a path's meridian or parallel through a point of an arc's
/// carrier, or of its plane's other loop, does. Either:
/// - the two circles are decided apart
///   (**`split_ring_path_coplanar_apart`**: the annulus about the
///   conic's centre between `min(a, b)` and `max(a, b)`, which holds
///   the conic, against the piece's circle, by the centres' distance).
///   A point on both would bound that distance by the triangle
///   inequality, so the reading holds whatever the planes: a parallel
///   against its plane's other loop on a torus, a meridian against the
///   one across the axis; or
/// - they are one circle (**`split_ring_path_same_circle`**: the larger
///   of the centres' distance and each radius's difference; with
///   [`plane_roots`]' gap in the band, the planes then lie within the
///   band of each other too) and the arcs are decided disjoint on it:
///   each end of either lies outside the other's span
///   (**`split_ring_path_in_span`**, an angle levered at the radius, the
///   distance along the circle). Two arcs of one circle overlap only
///   where one holds an end of the other.
///
/// `Ok(false)` where neither is decided.
fn misses_in_plane<T: Decide>(
    p: &CircleArc<T>,
    conic: &Conic<T>,
    band: Band,
) -> Result<bool, Indeterminate> {
    let apart = (p.centre - conic.centre).norm();
    let (inner, outer) = (conic.a.min(conic.b), conic.a.max(conic.b));
    let gap = (apart - p.radius - outer)
        .max(inner - p.radius - apart)
        .max(p.radius - outer - apart);
    if decide("split_ring_path_coplanar_apart", Margin::of(gap), band)? == Sign::Positive {
        return Ok(true);
    }
    let differ = apart
        .max((p.radius - conic.a).abs())
        .max((p.radius - conic.b).abs());
    if decide("split_ring_path_same_circle", Margin::of(differ), band)? != Sign::Zero {
        return Ok(false);
    }
    // An end lies outside a span when either of its two readings is
    // decided outside; an escalation counts only where neither is.
    let outside = |t: T, (lo, hi): (T, T), arm: T| -> Result<bool, Indeterminate> {
        let readings = [
            decide(
                "split_ring_path_in_span",
                Margin::levered(t - lo, arm),
                band,
            ),
            decide(
                "split_ring_path_in_span",
                Margin::levered(hi - t, arm),
                band,
            ),
        ];
        if readings.contains(&Ok(Sign::Negative)) {
            return Ok(true);
        }
        readings.iter().find_map(|r| r.err()).map_or(Ok(false), Err)
    };
    for t in [p.span.0, p.span.1] {
        if !outside(conic.param(p.at(t)), conic.span, conic.b)? {
            return Ok(false);
        }
    }
    for s in [conic.span.0, conic.span.1] {
        if !outside(p.param(conic.at(s)), p.span, p.radius)? {
            return Ok(false);
        }
    }
    Ok(true)
}

/// **Where the circle piece `p` meets the plane through `o` with unit
/// normal `n`**: the parameters to read, each with the slope `|n̂·τ̂|`
/// the piece crosses the plane at. `A cos θ + B sin θ = −D/r` (`A`, `B`
/// the normal's components on the piece's frame, `D` the piece's
/// centre's offset from the plane) holds at two parameters when
/// `r·√(A² + B²) > |D|` (**`split_ring_path_meets_plane`**, on the gap
/// `r·√(A² + B²) − |D|` in metres: moving the plane by it flips the
/// meeting); none when it is decided short, and `None` when it grazes in
/// the band. On the arrival arc (`arrival`), the root nearer the piece's
/// end is the arrival and only the other is read.
fn plane_roots<T: Decide>(
    p: &CircleArc<T>,
    (o, n): (Point3<T>, Vec3<T>),
    arrival: bool,
    band: Band,
) -> Result<Option<Vec<(T, T)>>, Indeterminate> {
    let (ca, cb) = (n.dot(p.u_ref), n.dot(p.v_ref()));
    let d = n.dot(p.centre - o);
    let rho = (ca.powi(2) + cb.powi(2)).sqrt();
    // The gap `rρ − |D|`: moving the plane by it flips the meeting, so
    // it is the deviation the sign rests on.
    match decide(
        "split_ring_path_meets_plane",
        Margin::of(p.radius * rho - d.abs()),
        band,
    )? {
        Sign::Negative => return Ok(Some(Vec::new())),
        Sign::Zero => return Ok(None),
        Sign::Positive => {}
    }
    let phase = cb.atan2(ca);
    let offset = (-d / (p.radius * rho)).max(-T::one()).min(T::one()).acos();
    let [r0, r1] = [phase + offset, phase - offset].map(|t| branch(t, p.span));
    // On the arrival arc, the root nearer the end is the arrival; the
    // other is read. The pick is a raw comparison and needs no decision.
    // The arrival lies on the plane up to rounding δ, and a root
    // misplaced by δ in the plane's offset moves by δ over the slope it
    // crosses at (as the in-span readings are levered): near a graze,
    // with half-angle ω between the roots, the slope is `ρ·sin ω` and the
    // roots lie `2rω` apart along the arc, so the root error against
    // their separation is `δ/(2rρ·sin ω·ω) ≈ δ/(4g)` for the gap
    // `g = rρ − |D| ≈ rρω²/2`. The pick holds while that is under a half.
    // A decided gap is `g ≥ Kε`, and the rounding is `δ ≤ ε` wherever the
    // coordinates resolve the band at all, so the ratio is at most
    // `1/(4K)`: `2K` times inside the half (about 150× at ε 1e-12, offset
    // 1e3, where δ is well under ε). No row guards the margin, because a
    // fixture with `δ ≥ 2g` would have coordinates that cannot resolve
    // the band. The bound is the guard.
    let roots = if arrival {
        let nearer = (r0 - p.span.1).abs() - (r1 - p.span.1).abs();
        vec![nearer.select_le_zero(r1, r0)]
    } else {
        vec![r0, r1]
    };
    // A carrier moved by δ moves the root by δ over the slope the piece
    // crosses the plane at, `|n̂·τ̂|`: each in-span reading is levered by
    // it, so its margin is the displacement that moves the root past the
    // span's end.
    Ok(Some(
        roots
            .into_iter()
            .map(|t| {
                let (st, ct) = t.sin_cos();
                (t, (cb * ct - ca * st).abs())
            })
            .collect(),
    ))
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
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
pub(crate) mod torus_islands;

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
    /// along its normal by up to the escalation threshold. The row guards
    /// a geometric invariant of the paths, that they are continuous, and
    /// not a parity: no parity row reads a crossing in such a gap.
    #[test]
    fn a_cone_paths_pieces_meet_at_their_seams() {
        let ends = |piece: &Piece<f64>| match *piece {
            Piece::Arc(a) | Piece::Meridian(a) => (a.at(a.span.0), a.at(a.span.1)),
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

    /// The torus islands' arcs, each read as the ring lane reads an
    /// edge on the torus ([`Quadric::torus_circle_arc`]).
    fn torus_arcs(
        quadric: &Quadric<f64>,
        island: &super::torus_islands::Island,
    ) -> Vec<LoopArc<f64>> {
        island
            .edges
            .iter()
            .map(|e| {
                quadric
                    .torus_circle_arc(&e.carrier, e.params, band())
                    .unwrap()
                    .expect("a meridian or a parallel")
            })
            .collect()
    }

    /// A grid of the whole torus, off every island's meridians and
    /// parallels: 12 azimuths by 8 tube angles.
    fn torus_grid(f: super::cone_islands::Frame) -> Vec<Point3<f64>> {
        (0..12)
            .flat_map(|k| {
                (0..8).map(move |j| {
                    super::torus_islands::on_torus(
                        f,
                        -PI + (f64::from(k) + 0.31) * PI / 6.0,
                        -PI + (f64::from(j) + 0.43) * PI / 4.0,
                    )
                })
            })
            .collect()
    }

    /// **Every path on a torus reads the parity the islands' oracles
    /// give**, in three frames, with the torus's axis either way: a grid
    /// of the whole torus taken pairwise, every path [`Quadric::paths`]
    /// offers between them, against islands on the outer side, on the
    /// inner side, over the tube's top, between one plane's two
    /// parallels, and wider than half a turn, and islands bounded by the
    /// tube's top or bottom circle, read across the cylinder `ρ = R`. The paths run round the
    /// back of the tube and the far side of the axis, so they cross the
    /// islands' planes on each plane's other loop: those crossings are
    /// passed over by the arc's loop side. Nearly every pair is decided,
    /// and both parities are read.
    #[test]
    fn a_torus_path_crosses_an_island_by_the_oracle_parity() {
        use super::torus_islands::{crest_islands, islands, torus};
        let mut far = 0usize;
        for f in frames() {
            for mirror in [false, true] {
                let surface = torus(f, mirror);
                let quadric = Quadric::of(&surface).unwrap();
                let points = torus_grid(f);
                for (name, island) in islands(f).into_iter().chain(crest_islands(f)) {
                    let arcs = torus_arcs(&quadric, &island);
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
                                // A path whose crossings of the arcs'
                                // planes would miscount with the loop
                                // side dropped.
                                let blind: Vec<_> = arcs
                                    .iter()
                                    .map(|&arc| match arc {
                                        LoopArc::Conic {
                                            centre,
                                            axis,
                                            u_ref,
                                            a,
                                            b,
                                            span,
                                            ..
                                        } => LoopArc::Conic {
                                            centre,
                                            axis,
                                            u_ref,
                                            a,
                                            b,
                                            span,
                                            own: None,
                                        },
                                        other => other,
                                    })
                                    .collect();
                                if path_parity(&path, &blind, None, band()).unwrap() == Some(!want)
                                {
                                    far += 1;
                                }
                            }
                        }
                    }
                    assert!(
                        decided * 100 >= asked * 99 && odd * 100 > decided,
                        "{name}, mirror {mirror}: {decided} of {asked} paths decided, {odd} odd"
                    );
                }
            }
        }
        assert!(
            far > 1000,
            "the rows cross the islands' planes on their other loops: {far} paths would miscount"
        );
    }

    /// **A path from a point on the tube's top or bottom circle reads the
    /// oracle's parity, or nothing**, against every crest island: its
    /// lead segment runs along the cylinder `ρ = R` from its start, and
    /// one of its parallels is that circle, so each may run along a crest
    /// arc, which no crossing count reads. The points lie on the two
    /// circles off every crest island's azimuths, taken with the whole
    /// grid, in three frames, the axis either way.
    #[test]
    fn a_path_from_the_tubes_top_or_bottom_reads_the_oracle_or_nothing() {
        use super::torus_islands::{crest_islands, on_torus, torus};
        use core::f64::consts::FRAC_PI_2;
        let (mut decided, mut asked) = (0usize, 0usize);
        for f in frames() {
            for mirror in [false, true] {
                let surface = torus(f, mirror);
                let quadric = Quadric::of(&surface).unwrap();
                let grid = torus_grid(f);
                let crests: Vec<_> = [FRAC_PI_2, -FRAC_PI_2]
                    .into_iter()
                    .flat_map(|v| [1.2, 1.6, 2.6, -2.2, -1.3].map(|u| on_torus(f, u, v)))
                    .collect();
                for (name, island) in crest_islands(f) {
                    let arcs = torus_arcs(&quadric, &island);
                    for &a in &crests {
                        for &b in grid.iter().chain(&crests) {
                            if (a - b).norm() == 0.0 {
                                continue;
                            }
                            let want = (island.inside)(a) != (island.inside)(b);
                            for (from, to) in [(a, b), (b, a)] {
                                for path in quadric.paths((from, to), band()).unwrap() {
                                    asked += 1;
                                    if let Some(got) =
                                        path_parity(&path, &arcs, None, band()).unwrap()
                                    {
                                        assert_eq!(
                                            got, want,
                                            "{name}, mirror {mirror}: {from:?} → {to:?}"
                                        );
                                        decided += 1;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        assert!(decided * 5 > asked, "{decided} of {asked} paths decided");
    }

    /// Points on the carriers of `island`'s arcs and on their planes'
    /// other loops, off the island's boundary: on each meridian's circle
    /// and the circle across the axis in its plane, and on each
    /// parallel's circle and the one across the tube in its plane
    /// (`π − v`), at chart coordinates off the island's; and where two
    /// of those meet, as a box's edge pierces a torus on two box faces'
    /// planes.
    fn on_carriers((u0, u1): (f64, f64), (v0, v1): (f64, f64)) -> Vec<(f64, f64)> {
        let us = [u0, u1, u0 + PI, u1 + PI];
        let vs = [v0, v1, PI - v0, PI - v1];
        let off_u = [u0 - 0.37, u1 + 0.41, u0 + 2.3, u1 - 2.9];
        let off_v = [v0 - 0.33, v1 + 0.29, v0 + 2.7, v1 - 3.1];
        let wrap = |a: f64| a.sin().atan2(a.cos());
        // Off the island's boundary: a point on a carrier circle within
        // the arc's span, or at a corner, lies on the curve the parity
        // counts, which no path end may.
        let near = |a: f64, b: f64| wrap(a - b).abs() < 0.05;
        let within = |a: f64, (lo, hi): (f64, f64)| {
            let t = lo + (wrap(a - lo)).rem_euclid(2.0 * PI);
            t <= hi + 0.05 || wrap(a - lo).abs() < 0.05
        };
        let on_boundary = |u: f64, v: f64| {
            ((near(v, v0) || near(v, v1)) && within(u, (u0, u1)))
                || ((near(u, u0) || near(u, u1)) && within(v, (v0, v1)))
        };
        us.iter()
            .flat_map(|&u| off_v.iter().chain(&vs).map(move |&v| (u, v)))
            .chain(
                vs.iter()
                    .flat_map(|&v| off_u.iter().chain(&us).map(move |&u| (u, v))),
            )
            .filter(|&(u, v)| !on_boundary(u, v))
            .collect()
    }

    /// **A torus path from a point on an arc's carrier, or on its
    /// plane's other loop, reads the oracle's parity**, against every
    /// island, in three frames, the axis either way: from such points,
    /// to the grid and to each other (both ways). Each meridian or
    /// parallel through such a point lies in an arc's plane, where it
    /// meets no arc crossing it: it runs on the plane's other loop, or
    /// on the arc's own circle off its span, and is read apart from the
    /// arc ([`lies_apart_in_plane`]); its lead starts on the plane, and
    /// is read apart by its distance from the arc
    /// (**`split_ring_path_segment_apart`**). The far ends are the grid,
    /// points inside the island and the other carrier points. A box's
    /// edge piercing a torus leaves a ring vertex on two such planes at
    /// once, and the one-face torus's outer loop (its wrap edges) lies on
    /// arcs' carriers, so there the ring lane's paths start and end on
    /// such points. Every decided path matches the oracle, some path
    /// decides nearly every pair, and both parities are read.
    #[test]
    fn a_torus_path_from_an_arcs_carrier_reads_the_oracle_parity() {
        use super::torus_islands::{island_spans, on_torus, rectangle, torus};
        let (mut pairs, mut read, mut paths, mut decided, mut odd) = (0, 0, 0, 0, 0usize);
        for f in frames() {
            for mirror in [false, true] {
                let surface = torus(f, mirror);
                let quadric = Quadric::of(&surface).unwrap();
                let grid = torus_grid(f);
                for (name, us, vs) in island_spans() {
                    let island = rectangle(f, us, vs);
                    let arcs = torus_arcs(&quadric, &island);
                    let carriers: Vec<_> = on_carriers(us, vs)
                        .into_iter()
                        .map(|(u, v)| on_torus(f, u, v))
                        .collect();
                    let within: Vec<_> = (1..4)
                        .flat_map(|k| {
                            (1..4).map(move |j| {
                                let at = |(lo, hi): (f64, f64), i: i32| {
                                    lo + (hi - lo) * f64::from(i) / 4.0
                                };
                                on_torus(f, at(us, k), at(vs, j))
                            })
                        })
                        .collect();
                    for (i, &a) in carriers.iter().enumerate() {
                        for &b in grid
                            .iter()
                            .step_by(5)
                            .chain(&within)
                            .chain(&carriers[i + 1..])
                        {
                            if (a - b).norm() < 1e-9 {
                                continue;
                            }
                            let want = (island.inside)(a) != (island.inside)(b);
                            for (from, to) in [(a, b), (b, a)] {
                                pairs += 1;
                                let mut any = false;
                                for path in quadric.paths((from, to), band()).unwrap() {
                                    paths += 1;
                                    if let Some(got) =
                                        path_parity(&path, &arcs, None, band()).unwrap()
                                    {
                                        assert_eq!(
                                            got, want,
                                            "{name}, mirror {mirror}: {from:?} → {to:?}"
                                        );
                                        decided += 1;
                                        odd += usize::from(got);
                                        any = true;
                                    }
                                }
                                read += usize::from(any);
                            }
                        }
                    }
                }
            }
        }
        assert!(
            read * 50 >= pairs * 49 && decided * 2 > paths && odd * 20 > decided,
            "{read} of {pairs} pairs read, {decided} of {paths} paths decided, {odd} odd"
        );
    }

    /// **A circle piece on a loop arc's own circle misses it exactly
    /// where the two arcs are disjoint**, and one on its plane's other
    /// loop always does ([`misses_in_plane`]): the arc `(0.5, 1)` of a
    /// torus meridian against pieces of the same circle, either way
    /// round, and of the meridian across the axis, in three frames. A
    /// piece holding the whole arc has both ends outside its span, so it
    /// is the arc's ends inside the piece's span that read it: red
    /// against the mutant "the arc's ends not read". The oracle rows
    /// cannot see that mutant, because a path along an island's arc meets
    /// the next arc at the corner first, and reads nothing there.
    #[test]
    fn a_piece_on_an_arcs_circle_misses_it_where_their_spans_are_disjoint() {
        use super::torus_islands::{RADII, meridian};
        for f in frames() {
            let (big, small) = (RADII.0 * f.s, RADII.1 * f.s);
            let edge = meridian(f, 0.3, (0.5, 1.0));
            let LoopArc::Conic {
                centre,
                axis,
                u_ref,
                a,
                b,
                span,
                ..
            } = LoopArc::of(&edge.carrier, edge.params).unwrap()
            else {
                panic!("a meridian is a conic");
            };
            let conic = Conic {
                centre,
                axis,
                u_ref,
                a,
                b,
                span,
            };
            let on = |span| CircleArc {
                centre,
                axis,
                radius: small,
                u_ref,
                span,
            };
            // The same circle travelled the other way: its parameter
            // `t` is the arc's `−t`.
            let back = |(lo, hi): (f64, f64)| CircleArc {
                centre,
                axis: -axis,
                radius: small,
                u_ref,
                span: (-hi, -lo),
            };
            let out = centre - f.o;
            let across = CircleArc {
                centre: f.o - out,
                axis,
                radius: small,
                u_ref,
                span: (0.0, 6.0),
            };
            assert!((out.norm() - big).abs() < 1e-12, "the meridian's centre");
            for (what, piece, misses) in [
                ("disjoint, after", on((1.2, 2.0)), true),
                ("disjoint, before", on((-1.0, 0.3)), true),
                ("disjoint, round the back", on((1.3, 5.9)), true),
                ("holding the arc", on((0.2, 1.4)), false),
                ("inside the arc", on((0.6, 0.9)), false),
                ("overlapping its end", on((0.8, 1.5)), false),
                ("disjoint, the other way round", back((1.2, 2.0)), true),
                (
                    "holding the arc, the other way round",
                    back((0.2, 1.4)),
                    false,
                ),
                ("the meridian across the axis", across, true),
            ] {
                assert_eq!(
                    misses_in_plane(&piece, &conic, band()).unwrap(),
                    misses,
                    "{what}, scale {}",
                    f.s
                );
            }
        }
    }

    /// **A piece that runs along the cylinder `ρ = R` says nothing of a
    /// crest arc**: a parallel of radius `R` over the arc's span
    /// (**`split_ring_path_crest_graze`**), a segment from the arc up a
    /// tube radius, and a segment whose ends lie either side of the
    /// cylinder (**`split_ring_path_crest_side`**), each read against the
    /// tube's top arc of the island "up to the top" alone, in three
    /// frames. [`Quadric::paths`] builds none of these where it would
    /// decide a parity: the meridian or the lead its parallel at `R`
    /// follows ends on the crest circle and is read undecided first
    /// (`a_path_from_the_tubes_top_or_bottom_reads_the_oracle_or_nothing`
    /// stays green with either guard dropped), so this row holds the
    /// guards to their contract. Red against the mutants "graze verdict
    /// ignored" and "lead-side check removed".
    #[test]
    fn a_piece_along_the_crest_cylinder_says_nothing_of_a_crest_arc() {
        use super::torus_islands::{RADII, crest_islands, on_torus, torus};
        use core::f64::consts::FRAC_PI_2;
        for f in frames() {
            let quadric = Quadric::of(&torus(f, false)).unwrap();
            let (_, island) = crest_islands(f).into_iter().next().unwrap();
            let crest: Vec<_> = torus_arcs(&quadric, &island)
                .into_iter()
                .filter(|arc| matches!(arc, LoopArc::Crest { .. }))
                .collect();
            assert_eq!(crest.len(), 1, "the island's one crest arc");
            let round = CircleArc {
                centre: f.at(0.0, 0.0, RADII.1),
                axis: f.z,
                radius: RADII.0 * f.s,
                u_ref: f.x,
                span: (-1.0, 1.0),
            };
            let top = on_torus(f, 0.0, FRAC_PI_2);
            let segment = |from: Point3<f64>, to: Point3<f64>| Piece::Segment {
                from,
                to,
                ruling: Vec3::zero(),
                lever: 0.0,
            };
            for (what, piece) in [
                ("a parallel along the crest", Piece::Arc(round)),
                (
                    "a segment up from the crest",
                    segment(top, top + f.z * 0.1 * f.s),
                ),
                (
                    "a segment across the cylinder",
                    segment(on_torus(f, 0.0, 1.2), on_torus(f, 0.0, 2.0)),
                ),
            ] {
                let path = Path {
                    pieces: vec![piece],
                    arrival: f.z,
                };
                assert_eq!(
                    path_parity(&path, &crest, None, band()).unwrap(),
                    None,
                    "{what}: decided"
                );
            }
        }
    }

    /// **A torus path's pieces meet exactly**, from end to end, wherever
    /// its ends lie off the carrier within the band: the lead segment
    /// takes the start onto the tube through the end, and each turn is
    /// placed on the circle the next piece runs round. Points of the
    /// whole torus in three frames, moved off it along its normal by up
    /// to the escalation threshold.
    #[test]
    fn a_torus_paths_pieces_meet_at_their_seams() {
        use super::torus_islands::torus;
        let ends = |piece: &Piece<f64>| match *piece {
            Piece::Arc(a) | Piece::Meridian(a) => (a.at(a.span.0), a.at(a.span.1)),
            Piece::Segment { from, to, .. } => (from, to),
        };
        let b = band();
        let mut asked = 0usize;
        for f in frames() {
            for mirror in [false, true] {
                let quadric = Quadric::of(&torus(f, mirror)).unwrap();
                let points: Vec<_> = torus_grid(f)
                    .into_iter()
                    .step_by(3)
                    .enumerate()
                    .map(|(k, p)| {
                        let off = [0.0, b.zero(), -b.escalate(), b.escalate()][k % 4];
                        p + quadric.chart_normal(p) * off
                    })
                    .collect();
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
