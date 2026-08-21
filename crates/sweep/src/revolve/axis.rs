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

use geom_core::{Affine3, Band, Decide, Margin, Point2, Point3, Real, Sign, Vec2, Vec3};
use profile::ValidatedProfile;

use super::{RevolveAxis, RevolveError, SweptSeg};
use crate::swept::{SweptKind, arc_apex, arc_span, decide};

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
        match decide("revolve_axis_direction", Margin::norm2(axis.dir), band)
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
/// over every loop vertex, every arc apex, and every **arc-interior
/// radial extremum** — the named lever arm the angle predicates meter
/// through (D4 ¶1).
///
/// An arc's `|r|` maximum can sit strictly inside the arc away from
/// the apex: on the carrier, `r` is extremal at `c ± R·ê_r` (carrier
/// angles 0 and π of the radial direction), and whichever of those
/// two points lies on the arc is folded in. Membership is the chord
/// half-plane test the arc classes use (the chord splits the carrier
/// into exactly two arcs; ours is the one opposite the bulge normal),
/// folded **comparison-free** via `copysign`: the candidate enters
/// the `max` lattice with the membership margin's sign, so an
/// off-arc candidate is negated and never wins (`r_max ≥ 0`). A zero
/// margin means the extremum is an endpoint, already folded.
///
/// Never topology-determining — this only meters angle-sliver margins
/// (`revolve_angle`, `revolve_angle_headroom`), so it stays outside
/// the `decide` funnel; deterministic per D9 (a pure `max`/`copysign`
/// fold, no branching on values).
pub(super) fn radial_extent<T: Real>(profile: &ValidatedProfile<T>, frame: &AxisFrame<T>) -> T {
    // Unit radial direction in sketch coordinates (the gradient of
    // `frame.r`; same construction as `AxisFrame::build`'s `e_r`).
    let e_r = Vec2::new(frame.dir_sk.y, T::zero() - frame.dir_sk.x);
    let mut r_max = T::zero();
    for lp in profile.loops() {
        for s in lp.segments() {
            r_max = r_max.max(frame.r(s.start).abs());
            if let profile::SegmentKind::Arc { center, radius, .. } = s.kind {
                let apex = arc_apex(s.start, s.end, s.bulge);
                r_max = r_max.max(frame.r(apex).abs());
                // Arc-interior radial extrema: the carrier points
                // c ± R·ê_r, each folded in iff on the arc. The arc is
                // the chord side of sign −bulge (apex side; see
                // `arc_apex`), so with the chord normal
                // n = (−chord.y, chord.x) the membership margin is
                // −bulge·((p − a)·n) ≥ 0.
                let chord = s.end - s.start;
                let n = Vec2::new(T::zero() - chord.y, chord.x);
                for dir in [T::one(), T::zero() - T::one()] {
                    let p = center + e_r * (radius * dir);
                    let margin = T::zero() - s.bulge * (p - s.start).dot(n);
                    r_max = r_max.max(frame.r(p).abs().copysign(margin));
                }
            }
        }
    }
    r_max
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
/// ([`WallKind`], placed into 3-space by `surfaces::wall_surface`)
/// plus the wall face's orientation sense (M5 S11).
#[derive(Clone, Copy, Debug)]
pub(super) enum WallClass<T: Real> {
    /// A line segment on the axis: sweeps to nothing.
    OnAxis,
    /// An off-axis segment: sweeps this surface of revolution.
    Wall {
        /// The classified surface of revolution.
        kind: WallKind<T>,
        /// The wall face's orientation sense (`topo::Face::sense`,
        /// M5 S11): `false` iff the material lies AGAINST the placed
        /// surface's chart normal. Derived from the profile's stored
        /// winding structure in the meridian half-plane (material is
        /// left of the canonical traversal; the profile lives at
        /// r ≥ 0 — see `classify_segment`), never from sampled
        /// normals.
        sense: bool,
    },
}

impl<T: Real> WallClass<T> {
    /// The wall kind, if the segment sweeps one.
    pub(super) fn kind(&self) -> Option<&WallKind<T>> {
        match self {
            WallClass::OnAxis => None,
            WallClass::Wall { kind, .. } => Some(kind),
        }
    }

    /// The wall face's orientation sense, if the segment sweeps one.
    pub(super) fn sense(&self) -> Option<bool> {
        match self {
            WallClass::OnAxis => None,
            WallClass::Wall { sense, .. } => Some(*sense),
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
    reverse: bool,
    band: Band,
) -> Result<LoopClasses<T>, RevolveError> {
    let n = segs.len();
    let mut verts = Vec::with_capacity(n);
    for s in segs {
        let r = frame.r(s.a);
        let pinned = match decide("axis_vertex_radius", Margin::of(r), band).map_err(|source| {
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
        walls.push(classify_segment(
            s, va, vb, frame, loop_index, reverse, band,
        )?);
    }
    Ok(LoopClasses { verts, walls })
}

/// Classifies one swept segment against the axis (file docs), and
/// derives its wall face's orientation sense (M5 S11).
///
/// **The sense derivation** (exact stored structure, never sampled
/// normals). Work in the meridian half-plane with frame coordinates
/// `(r, z)` — an orientation-preserving image of the sketch plane
/// (`ê_r = d̂ rotated −90°`, so `(ê_r, d̂)` is right-handed) with the
/// profile at `r ≥ 0`. The profile's canonical winding is
/// material-left (outers counterclockwise, holes clockwise), so the
/// material side of a wall segment is the LEFT of its canonical
/// traversal; the swept traversal (reversed for θ > 0) only relabels
/// the same stored signs, undone here via `reverse`. Every placed
/// revolution surface's chart normal points AWAY from the axis at
/// physical points (cylinder/cone: the outward radial family —
/// including the cone's far nappe, whose physical points sit at
/// azimuth u+π; sphere/torus: outward from the on-meridian center),
/// or along `+a₃` for plane annuli. `sense` is `true` iff the chart
/// normal points away from the material:
///
/// - **Cylinder & cone** (chart normal away from the axis,
///   ⊥ generator): material is at smaller radius exactly when the
///   canonical traversal climbs, so `sense = (canonical Δz > 0)` —
///   for the cone this is nappe-independent (the normal's meridian
///   form is `(cos α, −s_z·sin α)` with `s_z` the nappe side, and the
///   algebra collapses to the axial sign; the cylinder is the α → 0
///   case).
/// - **Plane annulus** (chart normal `+a₃`, i.e. `+z`): material is
///   above exactly when the canonical traversal runs outward, so
///   `sense = (canonical Δr < 0)`.
/// - **Sphere & torus** (chart normal away from the on-meridian
///   carrier center): the sense is
///   [`crate::swept::centre_on_material_side`] of the canonical turn —
///   one body, shared with extrude's cylinder walls, which carries the
///   derivation and the `Zero` totality posture.
///
/// Unreachable `Zero` signs on the LINE arms (a degenerate segment
/// survives to here only as a kernel bug) keep the convex/outward arm,
/// `sense: true` — the `turn_axis` totality posture.
#[allow(clippy::too_many_arguments)] // one internal call site (classify_loop).
fn classify_segment<T: Decide>(
    s: &SweptSeg<T>,
    va: &VertexClass<T>,
    vb: &VertexClass<T>,
    frame: &AxisFrame<T>,
    loop_index: usize,
    reverse: bool,
    band: Band,
) -> Result<WallClass<T>, RevolveError> {
    let escalated = |source| RevolveError::SliverAxisClearance {
        loop_index,
        segment_index: s.canonical_segment,
        source,
    };
    // A stored sign in the CANONICAL basis: the swept reversal negated
    // chord deltas and flipped turns, so `reverse` undoes it exactly.
    let canonical = |sign: Sign| if reverse { sign.flip() } else { sign };
    match s.kind {
        SweptKind::Line => {
            if va.pinned && vb.pinned {
                return Ok(WallClass::OnAxis);
            }
            // Radial and axial deltas of the chord (meters).
            let dr = vb.r - va.r;
            let dz = frame.axial(s.b) - frame.axial(s.a);
            let sr = decide("axis_line_radial", Margin::of(dr), band).map_err(escalated)?;
            let sz = decide("axis_line_axial", Margin::of(dz), band).map_err(escalated)?;
            // Line walls' sense (doc above): cylinder and cone read
            // the canonical axial direction, the plane annulus the
            // canonical radial one.
            if matches!(sr, Sign::Zero) {
                return Ok(WallClass::Wall {
                    kind: WallKind::Cylinder { radius: va.r },
                    sense: !matches!(canonical(sz), Sign::Negative),
                });
            }
            if matches!(sz, Sign::Zero) {
                return Ok(WallClass::Wall {
                    kind: WallKind::Plane,
                    sense: !matches!(canonical(sr), Sign::Positive),
                });
            }
            // Oblique: the generator crosses the axis at the affine
            // zero of r along the chord (r is linear on a line).
            let sstar = (T::zero() - va.r) / dr;
            let apex_sk = s.a + (s.b - s.a) * sstar;
            let half_angle = (dr.abs() / dz.abs()).atan();
            Ok(WallClass::Wall {
                kind: WallKind::Cone {
                    apex_sk,
                    half_angle,
                },
                sense: !matches!(canonical(sz), Sign::Negative),
            })
        }
        SweptKind::Arc {
            center,
            radius,
            turn,
        } => {
            // Arc walls' sense (doc above), through the shared rule —
            // the same body extrude's cylinder walls read.
            let sense = crate::swept::centre_on_material_side(canonical(turn));
            let rc = frame.r(center);
            match decide("axis_arc_center", Margin::of(rc), band).map_err(escalated)? {
                Sign::Zero => {
                    // Sphere class. Arc-interior half-plane checks: the
                    // r ≥ 0 half of an on-axis-centered carrier is
                    // exactly half its period, so a span definitely
                    // beyond π must dip below; the apex pins which
                    // half-circle branch the arc occupies.
                    let span_margin = Margin::levered(T::pi() - arc_span(s.bulge), radius);
                    match decide("axis_arc_span", span_margin, band).map_err(escalated)? {
                        Sign::Positive | Sign::Zero => {}
                        Sign::Negative => {
                            return Err(RevolveError::ArcCrossesAxis {
                                loop_index,
                                segment_index: s.canonical_segment,
                            });
                        }
                    }
                    let r_apex = frame.r(arc_apex(s.a, s.b, s.bulge));
                    match decide("axis_arc_apex", Margin::of(r_apex), band).map_err(escalated)? {
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
                    Ok(WallClass::Wall {
                        kind: WallKind::Sphere {
                            center_sk: center,
                            radius,
                        },
                        sense,
                    })
                }
                Sign::Positive => {
                    // Ring-torus clearance: the tube must stay
                    // definitely clear of the axis (D3 convention).
                    match decide("axis_arc_clearance", Margin::of(rc - radius), band)
                        .map_err(escalated)?
                    {
                        Sign::Positive => Ok(WallClass::Wall {
                            kind: WallKind::Torus {
                                center_sk: center,
                                major: rc,
                                minor: radius,
                            },
                            sense,
                        }),
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
        let allowed = run.is_some_and(|r| (0..=r.len).any(|k| (r.start + k) % n == j));
        if !allowed {
            return Err(RevolveError::NonManifoldAxisContact {
                loop_index,
                vertex_index: segs[j].canonical_vertex,
            });
        }
    }
    Ok(run)
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod tests {
    use geom_core::Tol;

    use profile::RawLoop;
    use profile::{Profile, ProfileLoop, ProfileVertex, SketchPlane};

    use super::*;

    /// Circular-segment profile on the carrier centered at (2, 0) with
    /// radius 1: an arc from carrier angle `phi_a` counterclockwise to
    /// `phi_b`, closed by its chord. Axis = the sketch y-axis, so
    /// `r(p) = p.x`.
    fn segment_profile(phi_a: f64, phi_b: f64) -> ValidatedProfile<f64> {
        let (cx, r) = (2.0, 1.0);
        let at = |phi: f64| Point2::new(cx + r * phi.cos(), r * phi.sin());
        let span = phi_b - phi_a; // counterclockwise, radians
        let lp = ProfileLoop::new(vec![
            ProfileVertex::new(at(phi_a), (span / 4.0).tan()),
            ProfileVertex::new(at(phi_b), 0.0),
        ]);
        Profile::new(SketchPlane::xy(), vec![lp])
            .validate(Tol::witness())
            .unwrap()
    }

    fn frame_y(vp: &ValidatedProfile<f64>) -> AxisFrame<f64> {
        let axis = RevolveAxis {
            origin: Point2::new(0.0, 0.0),
            dir: Vec2::new(0.0, 1.0),
        };
        AxisFrame::build(
            vp.plane().placement,
            &axis,
            Band::linear(Tol::witness()).unwrap(),
        )
        .unwrap()
    }

    /// NIT-1 regression: an arc from −90° to 130° contains the carrier
    /// angle-0 point (3, 0) — the interior radial maximum, r = 3 —
    /// which is neither an endpoint (r = 2, ≈1.357) nor the apex (at
    /// 20°, r = 2 + cos 20° ≈ 2.9397). The fold must report 3.
    #[test]
    fn radial_extent_sees_arc_interior_maximum() {
        let vp = segment_profile(-90f64.to_radians(), 130f64.to_radians());
        let ext = radial_extent(&vp, &frame_y(&vp));
        assert!((ext - 3.0).abs() < 1e-9, "extent {ext}, want 3.0");
    }

    /// The membership gate: an arc from 30° to 150° does NOT contain
    /// either carrier radial extremum (3, 0) or (1, 0); the fold stays
    /// at the true maximum — the 30° endpoint, r = 2 + cos 30°.
    #[test]
    fn radial_extent_gates_off_arc_extremum() {
        let vp = segment_profile(30f64.to_radians(), 150f64.to_radians());
        let ext = radial_extent(&vp, &frame_y(&vp));
        let want = 2.0 + 30f64.to_radians().cos();
        assert!((ext - want).abs() < 1e-9, "extent {ext}, want {want}");
    }
}
