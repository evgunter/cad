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
                // ([`Self::torus_circle_arc`] yields conics alone).
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
                    pieces: vec![lead, Piece::Arc(meridian), Piece::Arc(round)],
                    arrival: round.tangent(round.span.1),
                });
                // Round the start's parallel to `to`'s meridian, then round.
                let turned = centre + axis * hs + et * rs;
                let round = parallel(centre + axis * hs, axis, rs, start, turned);
                let meridian = parallel(kt, et.cross(axis), off_t, turned, to);
                paths.push(Path {
                    pieces: vec![lead, Piece::Arc(round), Piece::Arc(meridian)],
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
    /// radius less `R` (**`split_ring_torus_parallel_side`**), decided
    /// apart from the tube's top or bottom circle, which its plane
    /// touches.
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
            Sign::Zero => return Ok(None),
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
///   arrival arc, the root nearer the piece's end is the arrival.
/// - A segment meets a conic's plane where its ends' offsets from it
///   (**`split_ring_path_segment_side`**) differ in sign.
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
                    Piece::Arc(p),
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
                    // `g = rρ − |D| ≈ rρω²/2`. The pick holds while that
                    // is under a half. A decided gap is `g ≥ Kε`, and the
                    // rounding is `δ ≤ ε` wherever the coordinates
                    // resolve the band at all, so the ratio is at most
                    // `1/(4K)`: `2K` times inside the half (about 150×
                    // at ε 1e-12, offset 1e3, where δ is well under ε).
                    // No row guards the margin, because a fixture with
                    // `δ ≥ 2g` would have coordinates that cannot resolve
                    // the band. The bound is the guard.
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
                        let slope = (cb * ct - ca * st).abs();
                        let [p0, p1] = in_span(t, p.span, p.radius * slope);
                        let [a0, a1] = in_span(s, span, b * slope);
                        readings.push([p0, p1, a0, a1, on_own(own, p.at(t))]);
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
    /// parallels, and wider than half a turn. The paths run round the
    /// back of the tube and the far side of the axis, so they cross the
    /// islands' planes on each plane's other loop: those crossings are
    /// passed over by the arc's loop side. Nearly every pair is decided,
    /// and both parities are read.
    #[test]
    fn a_torus_path_crosses_an_island_by_the_oracle_parity() {
        use super::torus_islands::{islands, torus};
        let mut far = 0usize;
        for f in frames() {
            for mirror in [false, true] {
                let surface = torus(f, mirror);
                let quadric = Quadric::of(&surface).unwrap();
                let points = torus_grid(f);
                for (name, island) in islands(f) {
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
            Piece::Arc(a) => (a.at(a.span.0), a.at(a.span.1)),
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
