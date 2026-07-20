//! The revolve axis frame and the trilean axis-contact / half-plane
//! classification (the ratified case split — see the module docs).
//!
//! Everything topology-determining here goes through the named
//! [`super::decide`] funnel: per-vertex radial classes (on-axis /
//! off-axis / crossing / sliver), per-segment surface classes
//! (parallel / perpendicular / oblique lines; on-axis-centered /
//! off-axis-centered arcs), and the arc-interior half-plane checks
//! (span and apex margins — a vertex check alone cannot see an arc
//! bulging across the axis).

use geom_core::{Affine3, Band, Decide, Point2, Point3, Real, Sign, Vec2, Vec3};
use profile::ValidatedProfile;

use super::{RevolveAxis, RevolveError, SweptKind, SweptSeg, arc_apex, arc_span, decide};

/// The classified axis in both coordinate systems: the sketch-plane
/// line plus its placed 3-D frame. `a3`/`u3` are the **shared
/// azimuthal frame** every revolution surface of this call uses
/// (module docs): `u3` points at the angle-0 (seam) meridian
/// half-plane.
#[derive(Clone, Copy, Debug)]
pub(super) struct AxisFrame<T: Real> {
    /// Axis origin, sketch coordinates.
    pub(super) origin_sk: Point2<T>,
    /// Unit axis direction, sketch coordinates.
    pub(super) dir_sk: Vec2<T>,
    /// The sketch placement (carried for convenience).
    pub(super) place: Affine3<T>,
    /// Placed axis origin.
    pub(super) o3: Point3<T>,
    /// Placed unit axis direction (the rotation axis).
    pub(super) a3: Vec3<T>,
    /// Placed unit radial direction (the seam meridian; `u_ref` of
    /// every revolution surface).
    pub(super) u3: Vec3<T>,
    /// The sketch plane's normal (placement's third linear column).
    pub(super) n3: Vec3<T>,
}

impl<T: Decide> AxisFrame<T> {
    /// Classifies the axis direction and builds the frame.
    ///
    /// # Errors
    ///
    /// [`RevolveError::DegenerateAxis`] on a definitely-zero (or
    /// coincident-with-zero) direction; [`RevolveError::AxisEscalated`]
    /// on a sliver/poisoned length.
    pub(super) fn build(
        place: Affine3<T>,
        axis: &RevolveAxis<T>,
        band: Band,
    ) -> Result<Self, RevolveError> {
        match decide("revolve_axis_direction", axis.dir.norm(), band)
            .map_err(|source| RevolveError::AxisEscalated { source })?
        {
            Sign::Positive => {}
            Sign::Zero | Sign::Negative => return Err(RevolveError::DegenerateAxis),
        }
        let dir_sk = axis.dir.normalize();
        let e_r = Vec2::new(dir_sk.y, T::zero() - dir_sk.x);
        let o3 = place.transform_point(Point3::new(axis.origin.x, axis.origin.y, T::zero()));
        let a3 = place.linear * Vec3::new(dir_sk.x, dir_sk.y, T::zero());
        let u3 = place.linear * Vec3::new(e_r.x, e_r.y, T::zero());
        Ok(Self {
            origin_sk: axis.origin,
            dir_sk,
            place,
            o3,
            a3,
            u3,
            n3: place.linear.c2,
        })
    }
}

impl<T: Real> AxisFrame<T> {
    /// The signed radial coordinate of a sketch point (module docs).
    pub(super) fn r(&self, p: Point2<T>) -> T {
        (p - self.origin_sk).perp_dot(self.dir_sk)
    }

    /// The axial coordinate of a sketch point.
    pub(super) fn axial(&self, p: Point2<T>) -> T {
        (p - self.origin_sk).dot(self.dir_sk)
    }

    /// The placed axis point at a sketch point's axial coordinate —
    /// the axis foot, computed **on the axis line** (revolution-surface
    /// anchors and rim centers all come from here, so every anchor of
    /// one call shares the axis bitwise).
    pub(super) fn foot3(&self, p: Point2<T>) -> Point3<T> {
        let on_axis = self.origin_sk + self.dir_sk * self.axial(p);
        self.place
            .transform_point(Point3::new(on_axis.x, on_axis.y, T::zero()))
    }

    /// A sketch point placed into 3-space.
    pub(super) fn world(&self, p: Point2<T>) -> Point3<T> {
        self.place.transform_point(Point3::new(p.x, p.y, T::zero()))
    }
}

/// The profile's maximum radial extent (meters): the fold of `|r|`
/// over every loop vertex and every arc apex — the named lever arm the
/// angle predicates meter through (D4 ¶1). Comparison-free (`max`
/// lattice fold).
pub(super) fn radial_extent<T: Real>(profile: &ValidatedProfile<T>, frame: &AxisFrame<T>) -> T {
    let mut r_max = T::zero();
    for lp in profile.loops() {
        for s in lp.segments() {
            r_max = r_max.max(frame.r(s.start).abs());
            if matches!(s.kind, profile::SegmentKind::Arc { .. }) {
                let apex = apex_of(s.start, s.end, s.bulge);
                r_max = r_max.max(frame.r(apex).abs());
            }
        }
    }
    r_max
}

/// The arc apex from raw endpoint/bulge data (the same closed form as
/// [`super::arc_apex`], usable outside swept traversals).
fn apex_of<T: Real>(a: Point2<T>, b: Point2<T>, bulge: T) -> Point2<T> {
    let chord = b - a;
    let len = chord.norm();
    let u = chord.normalize();
    let nhat = Vec2::new(T::zero() - u.y, u.x);
    let mid = a.lerp(b, T::from_f64(0.5));
    mid - nhat * (len * bulge * T::from_f64(0.5))
}

/// A vertex's axis-contact class (swept order).
#[derive(Clone, Copy, Debug)]
pub(super) struct VertexClass<T: Real> {
    /// Definitely on the axis (pole/apex/shared-cap class).
    pub(super) pinned: bool,
    /// The raw radial coordinate (definitely positive when not
    /// `pinned`; kept for carrier radii).
    pub(super) r: T,
}

/// A segment's wall class in swept order: `OnAxis` (no wall), or the
/// surface of revolution it sweeps as sketch-level data
/// ([`WallKind`], placed into 3-space by `surfaces::wall_surface`).
#[derive(Clone, Copy, Debug)]
pub(super) enum WallClass<T: Real> {
    /// A line segment on the axis: sweeps to nothing.
    OnAxis,
    /// An off-axis segment: sweeps this surface of revolution.
    Wall(WallKind<T>),
}

impl<T: Real> WallClass<T> {
    /// The wall kind, if the segment sweeps one.
    pub(super) fn kind(&self) -> Option<&WallKind<T>> {
        match self {
            WallClass::OnAxis => None,
            WallClass::Wall(k) => Some(k),
        }
    }
}

/// The classified surface-of-revolution catalog (ratified in M2-PLAN
/// PR 5): line ∥ axis ⇒ cylinder, line ⊥ axis ⇒ plane annulus,
/// oblique line ⇒ cone, arc centered on axis ⇒ sphere, arc centered
/// off axis ⇒ ring torus.
#[derive(Clone, Copy, Debug)]
pub(super) enum WallKind<T: Real> {
    /// Line ⊥ axis: a plane annulus/disc.
    Plane,
    /// Line ∥ axis: a cylinder.
    Cylinder {
        /// Radius = the start vertex's radial coordinate.
        radius: T,
    },
    /// Oblique line: a cone.
    Cone {
        /// The generator line's axis crossing, sketch coordinates.
        apex_sk: Point2<T>,
        /// Half-angle α ∈ (0, π/2) between axis and generator.
        half_angle: T,
    },
    /// Arc centered on the axis: a sphere (endpoints on the axis are
    /// poles).
    Sphere {
        /// The carrier center (on the axis), sketch coordinates.
        center_sk: Point2<T>,
        /// The carrier radius.
        radius: T,
    },
    /// Arc centered off the axis: a ring torus.
    Torus {
        /// The carrier center, sketch coordinates.
        center_sk: Point2<T>,
        /// Major radius `R` = the center's radial coordinate.
        major: T,
        /// Minor radius `r` = the carrier radius (`R > r` classified).
        minor: T,
    },
}

/// One loop's classification, parallel to its swept segments.
#[derive(Debug)]
pub(super) struct LoopClasses<T: Real> {
    /// Per swept vertex `j` (= segment `j`'s start).
    pub(super) verts: Vec<VertexClass<T>>,
    /// Per swept segment `j`.
    pub(super) walls: Vec<WallClass<T>>,
}

/// Classifies one swept loop: per-vertex radial classes, per-segment
/// wall classes, and the arc-interior half-plane checks (file docs).
/// Error indices are canonical (carried by the swept segments).
pub(super) fn classify_loop<T: Decide>(
    segs: &[SweptSeg<T>],
    frame: &AxisFrame<T>,
    loop_index: usize,
    band: Band,
) -> Result<LoopClasses<T>, RevolveError> {
    let n = segs.len();
    let mut verts = Vec::with_capacity(n);
    for s in segs {
        let r = frame.r(s.a);
        let pinned = match decide("axis_vertex_radius", r, band).map_err(|source| {
            RevolveError::SliverRadius {
                loop_index,
                vertex_index: s.canonical_vertex,
                source,
            }
        })? {
            Sign::Zero => true,
            Sign::Positive => false,
            Sign::Negative => {
                return Err(RevolveError::VertexCrossesAxis {
                    loop_index,
                    vertex_index: s.canonical_vertex,
                });
            }
        };
        verts.push(VertexClass { pinned, r });
    }
    let mut walls = Vec::with_capacity(n);
    for (j, s) in segs.iter().enumerate() {
        let (va, vb) = (&verts[j], &verts[(j + 1) % n]);
        walls.push(classify_segment(s, va, vb, frame, loop_index, band)?);
    }
    Ok(LoopClasses { verts, walls })
}

/// Classifies one swept segment against the axis (file docs).
fn classify_segment<T: Decide>(
    s: &SweptSeg<T>,
    va: &VertexClass<T>,
    vb: &VertexClass<T>,
    frame: &AxisFrame<T>,
    loop_index: usize,
    band: Band,
) -> Result<WallClass<T>, RevolveError> {
    let escalated = |source| RevolveError::SliverAxisClearance {
        loop_index,
        segment_index: s.canonical_segment,
        source,
    };
    match s.kind {
        SweptKind::Line => {
            if va.pinned && vb.pinned {
                return Ok(WallClass::OnAxis);
            }
            // Radial and axial deltas of the chord (meters).
            let dr = vb.r - va.r;
            let dz = frame.axial(s.b) - frame.axial(s.a);
            if matches!(
                decide("axis_line_radial", dr, band).map_err(escalated)?,
                Sign::Zero
            ) {
                return Ok(WallClass::Wall(WallKind::Cylinder { radius: va.r }));
            }
            if matches!(
                decide("axis_line_axial", dz, band).map_err(escalated)?,
                Sign::Zero
            ) {
                return Ok(WallClass::Wall(WallKind::Plane));
            }
            // Oblique: the generator crosses the axis at the affine
            // zero of r along the chord (r is linear on a line).
            let sstar = (T::zero() - va.r) / dr;
            let apex_sk = s.a + (s.b - s.a) * sstar;
            let half_angle = (dr.abs() / dz.abs()).atan();
            Ok(WallClass::Wall(WallKind::Cone {
                apex_sk,
                half_angle,
            }))
        }
        SweptKind::Arc { center, radius, .. } => {
            let rc = frame.r(center);
            match decide("axis_arc_center", rc, band).map_err(escalated)? {
                Sign::Zero => {
                    // Sphere class. Arc-interior half-plane checks: the
                    // r ≥ 0 half of an on-axis-centered carrier is
                    // exactly half its period, so a span definitely
                    // beyond π must dip below; the apex pins which
                    // half-circle branch the arc occupies.
                    let span_margin = (T::pi() - arc_span(s.bulge)) * radius;
                    match decide("axis_arc_span", span_margin, band).map_err(escalated)? {
                        Sign::Positive | Sign::Zero => {}
                        Sign::Negative => {
                            return Err(RevolveError::ArcCrossesAxis {
                                loop_index,
                                segment_index: s.canonical_segment,
                            });
                        }
                    }
                    let r_apex = frame.r(arc_apex(s));
                    match decide("axis_arc_apex", r_apex, band).map_err(escalated)? {
                        Sign::Positive => {}
                        // On or below the axis: tangential/crossing
                        // interior contact.
                        Sign::Zero | Sign::Negative => {
                            return Err(RevolveError::ArcCrossesAxis {
                                loop_index,
                                segment_index: s.canonical_segment,
                            });
                        }
                    }
                    Ok(WallClass::Wall(WallKind::Sphere {
                        center_sk: center,
                        radius,
                    }))
                }
                Sign::Positive => {
                    // Ring-torus clearance: the tube must stay
                    // definitely clear of the axis (D3 convention).
                    match decide("axis_arc_clearance", rc - radius, band).map_err(escalated)? {
                        Sign::Positive => Ok(WallClass::Wall(WallKind::Torus {
                            center_sk: center,
                            major: rc,
                            minor: radius,
                        })),
                        Sign::Zero | Sign::Negative => Err(RevolveError::UnsupportedToroid {
                            loop_index,
                            segment_index: s.canonical_segment,
                        }),
                    }
                }
                // Center definitely on the negative side: the carrier
                // reaches across the axis (spindle) whatever the arc
                // does.
                Sign::Negative => Err(RevolveError::UnsupportedToroid {
                    loop_index,
                    segment_index: s.canonical_segment,
                }),
            }
        }
    }
}

/// A full revolve's axis-contact run: the single maximal cyclic run of
/// on-axis segments, in **swept** indices.
#[derive(Clone, Copy, Debug)]
pub(super) struct AxisRun {
    /// First on-axis segment of the run (swept index).
    pub(super) start: usize,
    /// Number of consecutive on-axis segments.
    pub(super) len: usize,
}

/// Analyzes a full revolve's axis contact (module docs): no contact ⇒
/// `None` (lamina case); one contiguous run whose closure accounts for
/// every pinned vertex ⇒ `Some(run)` (wire case); anything else is a
/// typed error.
pub(super) fn analyze_contact<T: Real>(
    segs: &[SweptSeg<T>],
    classes: &LoopClasses<T>,
    loop_index: usize,
) -> Result<Option<AxisRun>, RevolveError> {
    let n = segs.len();
    let on_axis: Vec<bool> = classes
        .walls
        .iter()
        .map(|w| matches!(w, WallClass::OnAxis))
        .collect();
    let count = on_axis.iter().filter(|&&b| b).count();
    let run = if count == 0 {
        None
    } else if count == n {
        // Every segment on-axis: a degenerate profile the validator
        // cannot produce (a loop needs area); surfaced as non-manifold
        // contact rather than trusted.
        return Err(RevolveError::NonManifoldAxisContact {
            loop_index,
            vertex_index: segs[0].canonical_vertex,
        });
    } else {
        // The single maximal cyclic run starts at an on-axis segment
        // whose predecessor is off-axis; a second such start means two
        // runs.
        let starts: Vec<usize> = (0..n)
            .filter(|&j| on_axis[j] && !on_axis[(j + n - 1) % n])
            .collect();
        if starts.len() > 1 {
            return Err(RevolveError::MultipleAxisRuns { loop_index });
        }
        let start = starts[0];
        let len = (0..n).take_while(|k| on_axis[(start + k) % n]).count();
        Some(AxisRun { start, len })
    };
    // Every pinned vertex must lie in the run's closed vertex range
    // (run start vertex … run end vertex); an isolated pinned vertex
    // revolves to a non-manifold point (module docs).
    for (j, v) in classes.verts.iter().enumerate() {
        if !v.pinned {
            continue;
        }
        let allowed = run.is_some_and(|r| {
            (0..=r.len).any(|k| (r.start + k) % n == j)
        });
        if !allowed {
            return Err(RevolveError::NonManifoldAxisContact {
                loop_index,
                vertex_index: segs[j].canonical_vertex,
            });
        }
    }
    Ok(run)
}
