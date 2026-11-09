//! **Two faces apart in depth about an axis, a centre or a circle, and
//! the directions those axes, centres and arcs aim across a pair** —
//! arm 1's clear for a cross-solid pair no direction separates (nested
//! walls, a cap inside a bore, a plug in a hole, a tube inside a tube),
//! and the directions arm 1's turned-direction test reads besides its
//! own.
//!
//! A reach box reads a face's extent in linear coordinates; this reads
//! it in a radial one. A *depth* is the distance from an axis line, from
//! a centre, or from a circle (a torus's core), and each is read for
//! both faces of the pair. Where one face's greatest depth lies below
//! the other's least, no point of one is a point of the other: the pair
//! is apart. That is the verdict, and the only one.
//!
//! **Which depths.** Each face of the pair proposes the depth its cover
//! lies at a level about ([`Cover::level`]: a cylinder wall's axis, a
//! sphere's centre, a torus's core circle) and the axis of every circle
//! or ellipse on its boundary ([`face_read`]). A cone's faces have no
//! one depth about its axis, and propose only their rims'.
//!
//! **The bounds.** A face's depths are read off its cover
//! ([`face_cover`]): its distance range from an axis line, from a
//! centre, or from a core circle ([`Cover::range_about`]), which on the
//! face's own level is that level wherever the trim leaves it.
//!
//! A least axis or centre depth is topo's reading of the face's nearest
//! distance from that line or point ([`face_nearest`]), which on a
//! plane face reads its boundary where the least point lies `Out`.
//!
//! The margin is the gap less both faces' [`sweep_pad`], the pad the
//! reach boxes' gap takes, a tolerance: a placement misread within the
//! band, and the boundary's vertices certified within ε of the face.
//! Only a definite Positive clears; a Zero, a Negative, a bound that is
//! not a number, an escalation or a face with no cover leaves the pair
//! to arm 1's refusal. A pair that touches has no gap and is never
//! cleared here.
//!
//! The same depths also aim directions ([`toward`]): the offset from an
//! axis or centre to the other face's nearest segment point, which arm
//! 1's turned-direction test reads; the faces' arcs and carriers aim
//! more ([`across`]). That proposes; it decides nothing.

use geom::cover::{Around, Cover, Site};
use geom_core::{Band, Decide, Margin, Point3, Sign, Vec3, is_finite_length};

use crate::body::Body;
use crate::boolean::boxes::sweep_pad;
use crate::cover::{edge_end_point, face_cover, face_nearest};
use crate::entity::FaceKey;
use crate::live::BoundaryMember;
use crate::validate::decide;

/// K name: the gap between one face's least depth and the other's
/// greatest, less both faces' pads (metres).
pub(crate) const DEPTH_GAP: &str = "census_backstop_depth_gap";

/// K name: the offset from a depth's axis or centre to the other face's
/// nearest boundary segment point (metres), before it is read as a unit
/// direction ([`toward`]).
pub(crate) const DEPTH_TOWARD: &str = "census_backstop_depth_toward";

/// K name: an arc's chord normal (its conic's unit normal crossed with
/// the chord), or the offset between two carriers' axes or centres
/// (metres), before it is read as a unit direction ([`across`]).
pub(crate) const DEPTH_ACROSS: &str = "census_backstop_depth_across";

/// A depth, and where it came from.
#[derive(Clone, Copy, Debug)]
pub(super) struct Proposed<T: geom_core::Real> {
    pub(super) depth: Around<T>,
    pub(super) source: Source<T>,
}

/// What proposed a depth.
#[derive(Clone, Copy, Debug)]
pub(super) enum Source<T: geom_core::Real> {
    /// The face's own carrier.
    Carrier,
    /// A circle or ellipse arc on the boundary, with its two ends.
    Arc(Point3<T>, Point3<T>),
}

/// A face as the depth gap reads it ([`face_read`]).
pub(super) struct Read<T: geom_core::Real> {
    /// The face's cover.
    pub(super) cover: Cover<T>,
    /// The boundary's straight edges by their ends, and its isolated
    /// vertices as points.
    pub(super) segments: Vec<(Point3<T>, Point3<T>)>,
    pub(super) depths: Vec<Proposed<T>>,
}

impl<T: geom_core::Real> Read<T> {
    /// The depth the face's own carrier proposes, where it proposes one.
    fn carrier(&self) -> Option<Around<T>> {
        self.depths
            .iter()
            .find(|p| matches!(p.source, Source::Carrier))
            .map(|p| p.depth)
    }
}

/// **`f` read for the depth gap** (module docs): its cover, its
/// straight edges, and the depths it proposes — its carrier's and the
/// axis of every conic on its boundary. `None` for a face with no cover.
///
/// # Panics
///
/// Where `f` or a record on the walk from it does not resolve, as
/// [`face_cover`]: a torn boundary is not one with no claim.
pub(super) fn face_read<T: Decide>(body: &Body<T>, f: FaceKey, band: Band) -> Option<Read<T>> {
    let face = body.proven_face(f);
    let cover = face_cover(body, f, band).ok()?;
    let mut depths: Vec<Proposed<T>> = cover
        .level()
        .map(|depth| Proposed {
            depth,
            source: Source::Carrier,
        })
        .into_iter()
        .collect();
    let mut segments = Vec::new();
    for member in face.boundary() {
        let (ek, edge) = match member {
            BoundaryMember::Isolated { point, .. } => {
                segments.push((point, point));
                continue;
            }
            BoundaryMember::Edge { ek, edge, .. } => (ek, edge),
        };
        let Some(carrier) = body
            .edge_curve_linked(ek, edge)
            .certified()
            .map(geom_brep::EdgeCurve::carrier)
        else {
            continue;
        };
        let ends = (
            edge_end_point(body, ek, edge.he_plus, "he_plus"),
            edge_end_point(body, ek, edge.he_minus, "he_minus"),
        );
        if carrier.as_line().is_some() {
            segments.push(ends);
        } else if let Some(conic) = carrier.conic() {
            depths.push(Proposed {
                depth: Around::Line(conic.frame().axis_line()),
                source: Source::Arc(ends.0, ends.1),
            });
        }
    }
    Some(Read {
        cover,
        segments,
        depths,
    })
}

/// `v` less its component along `n`'s direction, and that component.
pub(super) fn split<T: Decide>(v: Vec3<T>, n: Vec3<T>) -> (Vec3<T>, T) {
    let n = n.normalize();
    let along = v.dot(n);
    (v - n * along, along)
}

/// The offset from `depth`'s axis or centre to the nearest point of the
/// segment `p q`, in the projection that depth reads: off the axis, or
/// from the centre. Its norm is the segment's least depth. `None` for a
/// core depth.
fn nearest_offset<T: Decide>(depth: Around<T>, p: Point3<T>, q: Point3<T>) -> Option<Vec3<T>> {
    let (u, w) = match depth {
        Around::Line(geom::Line { origin, dir: axis }) => {
            let a = axis.normalize();
            (split(p - origin, a).0, split(q - p, a).0)
        }
        Around::Point(center) => (p - center, q - p),
        Around::Circle { .. } => return None,
    };
    // A point (`w = 0`) has no direction to slide along.
    let s = -(u.dot(w)) / w.dot(w);
    let s = if is_finite_length(s) {
        s.max(T::zero()).min(T::one())
    } else {
        T::zero()
    };
    Some(u + w * s)
}

/// The greatest of `bounds`, or `None` where there is none or one is
/// not a number (`max` would drop it).
pub(super) fn greatest<T: Decide>(bounds: impl Iterator<Item = Option<T>>) -> Option<T> {
    let mut acc: Option<T> = None;
    for b in bounds {
        let b = b.filter(|b| is_finite_length(*b))?;
        acc = Some(acc.map_or(b, |m| m.max(b)));
    }
    acc
}

/// Whether `lo − hi`, less both faces' pads, is definitely positive.
pub(super) fn clears<T: Decide>(hi: T, lo: T, band: Band) -> bool {
    let pad = T::from_f64(2.0 * sweep_pad(band));
    matches!(
        decide(DEPTH_GAP, Margin::of(lo - hi - pad), band),
        Ok(Sign::Positive)
    )
}

/// An upper bound on `proposed`'s depth over the face read as `read`.
fn greatest_on<T: Decide>(read: &Read<T>, proposed: &Proposed<T>) -> Option<T> {
    greatest([Some(read.cover.range_about(&proposed.depth).1)].into_iter())
}

/// A lower bound on `proposed`'s depth over the face `f`, read as
/// `read`; `None` for a face with no claim.
fn least_on<T: Decide>(
    body: &Body<T>,
    f: FaceKey,
    read: &Read<T>,
    proposed: &Proposed<T>,
    band: Band,
) -> Option<T> {
    let near = match proposed.depth {
        Around::Line(line) => face_nearest(body, f, &read.cover, &Site::Line(line), band),
        Around::Point(center) => face_nearest(body, f, &read.cover, &Site::Point(center), band),
        Around::Circle { .. } => read.cover.range_about(&proposed.depth).0,
    };
    greatest([Some(near)].into_iter())
}

/// **Whether the faces `a`, `b`, read as `ra`, `rb` ([`face_read`]),
/// are apart in some depth** either proposes (module docs).
///
/// # Panics
///
/// As [`face_read`].
pub(super) fn apart_in_depth<T: Decide>(
    body: &Body<T>,
    (a, ra): (FaceKey, &Read<T>),
    (b, rb): (FaceKey, &Read<T>),
    band: Band,
) -> bool {
    ra.depths.iter().chain(&rb.depths).any(|proposed| {
        // A least bound costs a point placement, so it is read only
        // against a greatest the other face has.
        let below = |lo: (FaceKey, &Read<T>), hi: &Read<T>| {
            greatest_on(hi, proposed).is_some_and(|hi| {
                least_on(body, lo.0, lo.1, proposed, band).is_some_and(|lo| clears(hi, lo, band))
            })
        };
        below((b, rb), ra) || below((a, ra), rb)
    })
}

/// **The directions the depths of two faces, read as `ra` and `rb`
/// ([`face_read`]), aim across the pair**: for each axis or centre one
/// face proposes, the offset from it to the other face's nearest
/// boundary segment point, off the axis or from the centre. This only
/// proposes. Arm 1 reads both faces' reaches along each direction
/// ([`crate::boolean::separating::apart_by`]), and the gap there is
/// the certificate, whatever aimed it. A core depth or a boundary with
/// no segment proposes nothing, and neither does an offset the band cannot
/// read as a unit direction.
///
/// The same axis proposed twice (a carrier's and each coaxial rim's)
/// proposes two directions that agree only to rounding. They are kept,
/// as [`crate::boolean::separating::face_axes`] keeps every normal at a
/// scalar with no bracket to key them by: a second reading of a span
/// that no verdict depends on.
pub(super) fn toward<T: Decide>(
    ra: &Read<T>,
    rb: &Read<T>,
    band: Band,
) -> Vec<geom_core::UnitVec3<T>> {
    [(ra, rb), (rb, ra)]
        .into_iter()
        .flat_map(|(own, other)| {
            own.depths.iter().filter_map(|proposed| {
                let nearest = other
                    .segments
                    .iter()
                    .filter_map(|&(p, q)| nearest_offset(proposed.depth, p, q))
                    .filter(|o| is_finite_length(o.norm()))
                    .reduce(|m, o| {
                        let nearer = o.norm() - m.norm();
                        Vec3::new(
                            nearer.select_le_zero(o.x, m.x),
                            nearer.select_le_zero(o.y, m.y),
                            nearer.select_le_zero(o.z, m.z),
                        )
                    })?;
                geom_core::UnitVec3::new(nearest, DEPTH_TOWARD, band).ok()
            })
        })
        .collect()
}

/// **The directions two faces, read as `ra` and `rb` ([`face_read`]),
/// aim across the pair from their own arcs and carriers**, which
/// [`toward`] does not read:
///
/// - each circle or ellipse arc on either boundary proposes its chord's
///   normal in the conic's plane, `n̂ × (q − p)` for the arc's ends `p`,
///   `q` and the conic's normal `n`. An arc of a conic lies on one side
///   of its chord, and a flat cut along the chord (a D's flat, a
///   half-disc's diameter, the line through a lens's tips) faces that
///   way. A closed rim (`p = q`) has no chord and proposes nothing: the
///   wall it bounds proposes through its carrier, below;
/// - the two carriers' axes or centres, one from each face, propose the
///   line between them: the offset of each axis's origin off the
///   other's (for parallel axes, the line of centres); the offset of a
///   centre off an axis; the offset between two centres. A torus's core
///   proposes nothing.
///
/// Two skew axes are not given their common normal here: a cylinder
/// wall's seam rulings run along its axis, so [`toward`]'s offset from
/// the other axis to the wall's nearest ruling point is that normal
/// wherever the nearest point lies inside the ruling, and where it lies
/// at a ruling's end the walls' nearest points are near that end, which
/// the clamped offset aims at.
///
/// That is at most one direction per arc of the two boundaries, and
/// two per pair of carriers. As [`toward`]'s, they only propose, and
/// an offset the band cannot read as a unit proposes nothing.
pub(super) fn across<T: Decide>(
    ra: &Read<T>,
    rb: &Read<T>,
    band: Band,
) -> Vec<geom_core::UnitVec3<T>> {
    let unit = |v: Vec3<T>| {
        is_finite_length(v.norm())
            .then(|| geom_core::UnitVec3::new(v, DEPTH_ACROSS, band).ok())
            .flatten()
    };
    let chords = [ra, rb].into_iter().flat_map(|read| {
        read.depths
            .iter()
            .filter_map(|proposed| match (proposed.depth, proposed.source) {
                (Around::Line(geom::Line { dir: axis, .. }), Source::Arc(p, q)) => {
                    unit(axis.normalize().cross(q - p))
                }
                _ => None,
            })
    });
    let off = |x: Point3<T>, origin: Point3<T>, axis: Vec3<T>| unit(split(x - origin, axis).0);
    let between = match (ra.carrier(), rb.carrier()) {
        (
            Some(Around::Line(geom::Line { origin: o, dir: a })),
            Some(Around::Line(geom::Line { origin: p, dir: b })),
        ) => {
            vec![off(p, o, a), off(o, p, b)]
        }
        (Some(Around::Line(geom::Line { origin, dir: axis })), Some(Around::Point(center)))
        | (Some(Around::Point(center)), Some(Around::Line(geom::Line { origin, dir: axis }))) => {
            vec![off(center, origin, axis)]
        }
        (Some(Around::Point(c)), Some(Around::Point(d))) => vec![unit(d - c)],
        _ => Vec::new(),
    };
    chords.chain(between.into_iter().flatten()).collect()
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used)]
mod tests {
    use geom::Surface;
    use geom::cover::ParamWindow;
    use geom_core::Tol;

    use super::*;

    fn band() -> Band {
        Band::linear(Tol::witness()).unwrap()
    }

    fn p(x: f64, y: f64, z: f64) -> Point3<f64> {
        Point3::new(x, y, z)
    }

    fn v(x: f64, y: f64, z: f64) -> Vec3<f64> {
        Vec3::new(x, y, z)
    }

    /// The circle of radius `r` about `c` in the plane normal to `z`.
    fn rim_about(c: Point3<f64>, r: f64) -> Cover<f64> {
        Cover::arc(
            &geom::Frame::new(c, v(0.0, 0.0, 1.0), v(1.0, 0.0, 0.0)),
            (r, r),
            ParamWindow::new(0.0, core::f64::consts::TAU),
        )
        .unwrap()
    }

    fn z_axis() -> Around<f64> {
        Around::Line(geom::Line {
            origin: p(0.0, 0.0, 0.0),
            dir: v(0.0, 0.0, 2.0),
        })
    }

    fn core(radius: f64) -> Around<f64> {
        Around::Circle {
            center: p(0.0, 0.0, 0.0),
            axis: v(0.0, 0.0, 1.0),
            radius,
        }
    }

    /// **The clear is a strict gap past the pad, and nothing less.** A
    /// gap of 0.1 clears; a band's width, none, or half the pads do not:
    /// those touch, and arm 1 keeps refusing them. Red where the pads
    /// are dropped or a Zero is read as apart.
    #[test]
    fn a_gap_clears_only_past_the_pad() {
        let band = band();
        let pad = 2.0 * sweep_pad(band);
        assert!(clears(0.85, 0.95, band), "0.1 clears");
        assert!(
            !clears(0.95 - band.zero(), 0.95, band),
            "a band's width is a touch"
        );
        assert!(!clears(0.95, 0.95, band), "no gap is a touch");
        assert!(!clears(0.95 - 0.5 * pad, 0.95, band), "inside the pads");
        assert!(!clears(0.95 - pad, 0.95, band), "exactly the pads, a Zero");
        assert!(!clears(1.05, 0.95, band), "an overlap");
        assert!(!clears(f64::NAN, 0.95, band), "not a number");
    }

    /// **A coaxial circle reads at its radius from both sides**, and one
    /// moved 0.05 off the axis reads `0.85 ± 0.05`. Red where the
    /// circle's offset is dropped or added the wrong way.
    #[test]
    fn a_rim_reads_at_its_radius_and_its_offset() {
        let depth = z_axis();
        assert_eq!(
            rim_about(p(0.0, 0.0, 3.0), 0.85).range_about(&depth),
            (0.85, 0.85),
            "coaxial"
        );
        let (lo, hi) = rim_about(p(0.05, 0.0, 0.0), 0.85).range_about(&depth);
        assert!((hi - 0.9).abs() < 1e-15, "off the axis, above: {hi}");
        assert!((lo - 0.8).abs() < 1e-15, "off the axis, below: {lo}");
    }

    /// **The offset to a segment runs off the axis to its nearest
    /// point**, or from the centre to it, and a core depth proposes none.
    /// Red where the offset keeps its part along the axis, or is read at
    /// an end the segment is not nearest at.
    #[test]
    fn a_segments_offset_runs_to_its_nearest_point() {
        let at = |depth, a, b| nearest_offset(depth, a, b).map(|o: Vec3<f64>| (o.x, o.y, o.z));
        assert_eq!(
            at(z_axis(), p(1.0, 1.0, 5.0), p(3.0, 1.0, 5.0)),
            Some((1.0, 1.0, 0.0)),
            "beside the axis, nearest at an end"
        );
        assert_eq!(
            at(z_axis(), p(-2.0, 1.0, 5.0), p(2.0, 1.0, 7.0)),
            Some((0.0, 1.0, 0.0)),
            "across the axis, nearest inside"
        );
        let centre = Around::Point(p(0.0, 0.0, 0.0));
        assert_eq!(
            at(centre, p(-2.0, 1.0, 5.0), p(2.0, 1.0, 5.0)),
            Some((0.0, 1.0, 5.0)),
            "from a centre"
        );
        assert_eq!(
            at(core(2.0), p(0.0, 0.0, 0.0), p(1.0, 0.0, 0.0)),
            None,
            "a core"
        );
    }

    /// **A torus face's tube reads its core at its minor radius, and a
    /// disc across the hole reaches the core's distance at its centre.**
    /// Red where a tube's core depth is read through its whole `(ρ, t)`
    /// box, which reads `0.45·√2` and loses the nested tubes' gap, or
    /// where a disc's convex hull is read by its rim alone.
    #[test]
    fn a_core_depth_reads_a_tube_at_its_minor_radius_and_a_disc_at_its_centre() {
        let tube = Cover::tube(
            &geom::Frame::new(p(0.0, 0.0, 0.0), v(0.0, 0.0, 1.0), v(1.0, 0.0, 0.0)),
            2.0,
            0.45,
        )
        .unwrap();
        assert_eq!(tube.range_about(&core(2.0)), (0.0, 0.45), "the tube");
        let disc = rim_about(p(0.0, 0.0, 0.0), 2.0).convex();
        assert!(disc.range_about(&core(2.0)).1 >= 2.0, "the disc's centre");
        assert_eq!(disc.range_about(&z_axis()).1, 2.0, "an axis reads the rim");
    }

    /// **A carrier's cover reads its level about its own site**: a ring
    /// or horn torus at its minor radius from its core, a sphere at its
    /// radius from its centre, each radius by its magnitude; a spindle's
    /// points come nearer its core, so it reads from zero. Red where a
    /// shell is read as its solid tube (every least reads zero), or
    /// where a spindle is given its tube's level.
    #[test]
    fn a_carriers_cover_reads_its_level_about_its_own_site() {
        let about = |surface: Surface<f64>| {
            let cover = surface.cover().expect("a bounded carrier");
            let site = cover.level().expect("a carrier's surface lies at a level");
            cover.range_about(&site)
        };
        let torus = |major_radius, minor_radius| Surface::Torus {
            center: p(0.0, 1.0, 0.0),
            axis: v(0.0, 0.0, 1.0),
            major_radius,
            minor_radius,
            u_ref: v(1.0, 0.0, 0.0),
        };
        assert_eq!(about(torus(2.0, 0.5)), (0.5, 0.5), "a ring torus");
        assert_eq!(about(torus(0.5, 0.5)), (0.5, 0.5), "a horn torus");
        assert_eq!(about(torus(0.75, -0.3)), (0.3, 0.3), "a negative tube");
        assert_eq!(about(torus(0.4, 0.5)), (0.0, 0.5), "a spindle torus");
        let sphere = Surface::Sphere {
            center: p(0.0, 1.0, 0.0),
            radius: -0.7,
            axis: v(0.0, 0.0, 1.0),
            u_ref: v(1.0, 0.0, 0.0),
        };
        assert!(
            matches!(sphere.cover().unwrap().level(), Some(Around::Point(c)) if (c - p(0.0, 1.0, 0.0)).norm() == 0.0),
            "a sphere's level is about its centre"
        );
        assert_eq!(
            about(sphere),
            (0.7, 0.7),
            "a sphere stored with a negative radius"
        );
    }

    /// **Two nested tori's faces are apart in their core's depth**: tubes
    /// of minor radius 0.4 and 0.45 about one core, each face's cover its
    /// carrier's met with a chart patch, as the face cover reads it. The
    /// inner's greatest depth, 0.4, lies 0.05 below the outer's least.
    /// Red where either shell reads from zero: the tube's least depth
    /// about its core is 0, and the pair touches.
    #[test]
    fn nested_tori_are_apart_in_their_cores_depth() {
        let frame = geom::Frame::new(p(0.0, 0.0, 0.0), v(0.0, 0.0, 1.0), v(1.0, 0.0, 0.0));
        let face = |minor: f64| {
            let carrier = Surface::Torus {
                center: frame.origin,
                axis: frame.axis,
                major_radius: 2.0,
                minor_radius: minor,
                u_ref: frame.u_ref,
            }
            .cover()
            .unwrap();
            let patch = Cover::patch(
                &frame,
                (2.0, minor),
                Some(ParamWindow::new(0.0, 1.0)),
                ParamWindow::new(-1.0, 1.0),
            )
            .unwrap();
            let cover = carrier.meet(patch);
            let depth = cover.level().expect("the patch's carrier lies at a level");
            Read {
                cover,
                segments: Vec::new(),
                depths: vec![Proposed {
                    depth,
                    source: Source::Carrier,
                }],
            }
        };
        let (inner, outer) = (face(0.4), face(0.45));
        assert_eq!(
            inner.cover.range_about(&core(2.0)),
            (0.4, 0.4),
            "the meet keeps the level"
        );
        // A core depth reads no point placement, so no face of the body
        // is resolved.
        let body = Body::<f64>::new();
        let f = FaceKey::default();
        assert!(
            apart_in_depth(&body, (f, &inner), (f, &outer), band()),
            "nested tubes 0.05 apart"
        );
        let touching = face(0.4);
        assert!(
            !apart_in_depth(&body, (f, &inner), (f, &touching), band()),
            "one tube against itself touches"
        );
    }

    /// **Two coaxial cylinder walls are apart in their axis's depth**:
    /// sheets of radius 1 and 1.05 over one height and azimuth, in one
    /// body. Each face's cover is its boundary's hull met with its
    /// carrier's wall, which reads its radius about its axis exactly. Red
    /// where the wall is dropped: the hull's least depth about the axis
    /// is its ball's, and the pair is left touching.
    #[test]
    fn coaxial_cylinder_walls_are_apart_in_their_axiss_depth() {
        use crate::test_support_fixtures::{CylFrame, cyl_wall_sheet};
        let tol = Tol::witness();
        let mut body = Body::<f64>::new();
        let wall = |body: &mut Body<f64>, radius| {
            cyl_wall_sheet(
                body,
                CylFrame::canonical(radius),
                (0.2, 1.4),
                (0.0, 1.0),
                tol,
            )
        };
        let (a, b) = (wall(&mut body, 1.0), wall(&mut body, 1.05));
        let read = |f| face_read(&body, f, band()).expect("a wall is read");
        let (ra, rb) = (read(a), read(b));
        let axis = ra.cover.level().expect("a wall lies at a level");
        assert!(
            matches!(axis, Around::Line(geom::Line { origin, dir }) if (origin - p(0.0, 0.0, 0.0)).norm() == 0.0 && dir.cross(v(0.0, 0.0, 1.0)).norm() == 0.0),
            "about its axis: {axis:?}"
        );
        assert_eq!(ra.cover.range_about(&axis), (1.0, 1.0), "the inner wall");
        assert_eq!(rb.cover.range_about(&axis), (1.05, 1.05), "the outer wall");
        assert!(
            apart_in_depth(&body, (a, &ra), (b, &rb), band()),
            "walls 0.05 apart"
        );
    }

    /// **An ellipse's cover holds the ellipse**: about its own axis an
    /// ellipse of semi-axes 2 and 1 spans depths `[1, 2]`, and its cover
    /// bounds both ends. Red where the arc is read as its major circle,
    /// which puts the least depth at 2.
    #[test]
    fn an_ellipses_cover_holds_its_minor_vertices() {
        let carrier = geom::Curve3::Ellipse(geom::Ellipse::new(
            geom::Frame::new(p(0.0, 0.0, 0.0), v(0.0, 0.0, 1.0), v(1.0, 0.0, 0.0)),
            2.0,
            1.0,
        ));
        let (lo, hi) = carrier.cover().unwrap().range_about(&z_axis());
        assert!(lo <= 1.0, "the minor vertex: {lo}");
        assert!(hi >= 2.0, "the major vertex: {hi}");
    }

    /// A unit brick's top face (`z = 1`) restated on a cone whose apex
    /// stands at `apex`, about `axis`.
    fn coned_top(apex: Point3<f64>, axis: Vec3<f64>) -> (Body<f64>, FaceKey) {
        let tol = Tol::witness();
        let mut body =
            crate::test_support_fixtures::brick::<f64>((0.0, 1.0), (0.0, 1.0), (0.0, 1.0), tol);
        let top = body
            .faces()
            .map(|(f, _)| f)
            .find(|&f| {
                let (lo, hi) = super::super::face_reach(&body, f, band()).unwrap();
                lo.z == 1.0 && hi.z == 1.0
            })
            .expect("the top face");
        let key = body.get_face(top).unwrap().surface;
        *body.surfaces.get_mut(key).expect("its surface") = Surface::Cone {
            apex,
            axis,
            half_angle: 0.3,
            u_ref: v(1.0, 0.0, 0.0),
        };
        (body, top)
    }

    /// **A cone face's cover holds its apex where its loop may wind round
    /// the axis**: the unit square restated on a cone whose apex stands
    /// 5 above its centre reads, from that centre, a greatest depth past
    /// the apex's 5. Red where the apex is dropped from the hull. That a
    /// loop that closes in the chart leaves the apex out is
    /// `boxes::tests::the_cone_arm_boxes_the_frustum_not_the_widest_ring`.
    #[test]
    fn a_cone_face_holds_its_apex_where_its_loop_may_wind_round_the_axis() {
        let centre = Around::Point(p(0.5, 0.5, 1.0));
        let (body, top) = coned_top(p(0.5, 0.5, 6.0), v(0.0, 0.0, -1.0));
        let read = face_read(&body, top, band()).expect("a cone face is read");
        assert!(read.cover.range_about(&centre).1 >= 5.0, "the apex is held");
    }

    /// **A bound that is not a number is no bound**, though `max` and
    /// `min` would drop it; and no piece gives none.
    #[test]
    fn a_poisoned_bound_is_no_bound() {
        assert_eq!(
            greatest([Some(1.0), Some(f64::NAN)].into_iter()),
            None,
            "NaN last"
        );
        assert_eq!(
            greatest([Some(f64::NAN), Some(1.0)].into_iter()),
            None,
            "NaN first"
        );
        assert_eq!(greatest([Some(1.0), None].into_iter()), None, "no claim");
        assert_eq!(greatest(std::iter::empty::<Option<f64>>()), None, "nothing");
        assert_eq!(
            greatest([Some(2.0), Some(1.0)].into_iter()),
            Some(2.0),
            "greatest"
        );
    }

    /// **A segment reads its nearest point, not its ends**: a chord
    /// passing 0.3 from the axis between ends 1 away.
    #[test]
    fn a_segment_reads_its_nearest_point() {
        let chord = Cover::segment(p(-1.0, 0.3, 0.0), p(1.0, 0.3, 5.0)).unwrap();
        let (lo, hi) = chord.range_about(&z_axis());
        assert!((lo - 0.3).abs() < 1e-15, "the axis: {lo}");
        assert!((hi - 1.09f64.sqrt()).abs() < 1e-15, "its ends: {hi}");
        let centre = Around::Point(p(0.0, 0.0, 2.5));
        assert!(
            (chord.range_about(&centre).0 - 0.3).abs() < 1e-15,
            "a centre"
        );
    }

    /// A read holding only `depths`.
    fn read_of(depths: Vec<Proposed<f64>>) -> Read<f64> {
        Read {
            cover: Cover::points(vec![p(0.0, 0.0, 0.0)]).unwrap(),
            segments: Vec::new(),
            depths,
        }
    }
    fn arc(c: Point3<f64>, n: Vec3<f64>, from: Point3<f64>, to: Point3<f64>) -> Proposed<f64> {
        Proposed {
            depth: Around::Line(geom::Line { origin: c, dir: n }),
            source: Source::Arc(from, to),
        }
    }

    fn carrier(depth: Around<f64>) -> Proposed<f64> {
        Proposed {
            depth,
            source: Source::Carrier,
        }
    }

    fn dirs(a: &Read<f64>, b: &Read<f64>) -> Vec<Vec3<f64>> {
        across(a, b, band()).into_iter().map(|u| u.get()).collect()
    }

    fn holds(dirs: &[Vec3<f64>], want: Vec3<f64>) -> bool {
        dirs.iter().any(|d| d.cross(want).norm() < 1e-12)
    }

    /// **`across` at its degenerate inputs.** A half-disc's arc (its
    /// chord through the centre) and a scaled conic normal still propose
    /// the chord's normal in the conic's plane; an ellipse arc's is
    /// perpendicular to its chord, not its conjugate diameter; a closed
    /// rim and two coincident centres propose nothing; two parallel axes,
    /// their origins staggered along them, propose the line of centres.
    /// Red where the chord direction is read from the centre to the
    /// chord's midpoint, or the offset off an axis is not projected off
    /// it.
    #[test]
    fn across_reads_its_degenerate_inputs() {
        let empty = read_of(Vec::new());
        let z = v(0.0, 0.0, 3.0);
        let half = read_of(vec![arc(
            p(0.0, 0.0, 0.0),
            z,
            p(0.0, 1.0, 0.0),
            p(0.0, -1.0, 0.0),
        )]);
        assert_eq!(dirs(&half, &empty).len(), 1, "a half-disc proposes one");
        assert!(
            holds(&dirs(&half, &empty), v(1.0, 0.0, 0.0)),
            "its diameter's normal"
        );
        // An arc of the ellipse x²/4 + y² = 1 from (2, 0) to (0, 1): its
        // chord's normal is (1, 2), its midpoint's offset (1, 0.5).
        let ellipse = read_of(vec![arc(
            p(0.0, 0.0, 0.0),
            z,
            p(2.0, 0.0, 0.0),
            p(0.0, 1.0, 0.0),
        )]);
        assert!(
            holds(&dirs(&ellipse, &empty), v(1.0, 2.0, 0.0)),
            "the chord's normal"
        );
        let rim = read_of(vec![arc(
            p(0.0, 0.0, 0.0),
            z,
            p(1.0, 0.0, 0.0),
            p(1.0, 0.0, 0.0),
        )]);
        assert!(
            dirs(&rim, &empty).is_empty(),
            "a closed rim proposes nothing"
        );
        let ball = |c| read_of(vec![carrier(Around::Point(c))]);
        assert!(
            dirs(&ball(p(1.0, 2.0, 3.0)), &ball(p(1.0, 2.0, 3.0))).is_empty(),
            "coincident centres propose nothing"
        );
        let wall = |o, a| {
            read_of(vec![carrier(Around::Line(geom::Line {
                origin: o,
                dir: a,
            }))])
        };
        let y = v(0.0, 1.0, 0.0);
        let parallel = dirs(&wall(p(0.0, 0.0, 0.0), y), &wall(p(1.0, 0.7, 1.0), y * 2.0));
        assert_eq!(parallel.len(), 2, "two offsets");
        assert!(
            parallel
                .iter()
                .all(|d| d.cross(v(1.0, 0.0, 1.0)).norm() < 1e-12),
            "the line of centres, off the axes: {parallel:?}"
        );
    }
}
