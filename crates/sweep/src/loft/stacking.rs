//! **The stacking door** — what the loft decides about each slab before
//! the skin, slab by slab: its pinned corners, whether its two sections
//! touch beyond them, and which way it stacks ([`stacking_door`]). It
//! refuses contact it verified, never contact it could not rule out: an
//! arc pair the curve-pair reader cannot settle is left to the embedding
//! certificate (`work/lathe/self-overlapping-spines-build-and-validate.md`).

use geom::Curve3;
use geom_brep::curve_pair::{self, Hit, MeetKind, Span, SpanEnd};
use geom_brep::ladder::read_identity;
use geom_core::{Band, Decide, Indeterminate, Margin, Point3, Sign, Tol, Vec3};
use profile::{SegmentKind, SketchPlane, ValidatedProfile};

use super::{LoftError, LoopSegment, PinnedSite};
use crate::swept::{SweptSeg, decide, placed_segment_spec};

/// The compatible strips the door reads, `[loop][segment][section]`:
/// each segment's section curve, placed and on the strip's one knot
/// vector, as its control points. The first is the segment's start
/// corner and the last its end.
pub(super) type Strips<T> = Vec<Vec<Vec<Vec<Point3<T>>>>>;

/// **Which corners each slab pins**, `[loop][corner][slab]`: corner `j`
/// is the start of segment `j`, and it is pinned in slab `s` where its
/// world point in section `s` and in section `s + 1` decide one point
/// ([`corner_pinned`]).
///
/// With it, **which segments each slab holds flat**, `[loop][segment][slab]`:
/// segment `j` pinned at both ends and the same curve in both sections
/// ([`rim_pinned`]; a line always is), the edge both sections hold.
///
/// And **which sub-arc joints each slab pins**, `[loop][segment][joint][slab]`:
/// the C⁰ knots of segment `j`'s compatible strip, where the loft cuts
/// its wall ([`Joints`]), each decided as a corner is, so a joint that is
/// one world point in both sections is a corner of the two walls it
/// parts.
pub(super) struct Pinned {
    corners: Vec<Vec<Vec<bool>>>,
    flats: Vec<Vec<Vec<bool>>>,
    joints: Vec<Vec<Vec<Vec<bool>>>>,
}

/// The control index of each sub-arc joint of each strip,
/// `[loop][segment][joint]` (`crate::skin::crease_columns`): where the
/// strip's curves interpolate a control point at a C⁰ knot.
pub(super) type Joints = Vec<Vec<Vec<usize>>>;

impl Pinned {
    /// Whether loop `l`'s corner `j` is pinned in `slab`.
    pub(super) fn at(&self, l: usize, j: usize, slab: usize) -> bool {
        self.corners[l][j][slab]
    }

    /// Whether loop `l`'s segment `j` is held flat in `slab`.
    pub(super) fn flat(&self, l: usize, j: usize, slab: usize) -> bool {
        self.flats[l][j][slab]
    }

    /// Whether loop `l`'s corner `j` is pinned in every slab of `slabs`.
    pub(super) fn at_through(&self, l: usize, j: usize, slabs: core::ops::Range<usize>) -> bool {
        slabs.into_iter().all(|s| self.at(l, j, s))
    }

    /// Whether loop `l`'s segment `j`'s sub-arc joint `i` is pinned in
    /// every slab of `slabs`.
    pub(super) fn joint_through(
        &self,
        l: usize,
        j: usize,
        i: usize,
        slabs: core::ops::Range<usize>,
    ) -> bool {
        slabs.into_iter().all(|s| self.joints[l][j][i][s])
    }

    /// Whether loop `l`'s segment `j`'s sub-arc joint `i` is pinned in
    /// `slab`.
    pub(super) fn joint_at(&self, l: usize, j: usize, i: usize, slab: usize) -> bool {
        self.joints[l][j][i][slab]
    }

    /// Whether loop `l`'s segment `j` is held flat in every slab of
    /// `slabs`.
    pub(super) fn flat_through(&self, l: usize, j: usize, slabs: core::ops::Range<usize>) -> bool {
        slabs.into_iter().all(|s| self.flat(l, j, s))
    }
}

/// **The one predicate of a pinned corner**: whether a profile vertex
/// is the same world point in two adjacent sections, decided on the
/// distance between its two placements.
///
/// # Errors
///
/// The escalation, where the band cannot call the distance.
fn corner_pinned<T: Decide>(a: Point3<T>, b: Point3<T>, band: Band) -> Result<bool, Indeterminate> {
    Ok(decide("loft_corner_pinned", Margin::norm3(b - a), band)? == Sign::Zero)
}

/// Whether every control row of a segment's two section curves decides
/// one point: the segment is the same curve in both sections. An
/// identity: any row decided apart answers "not pinned", whatever an
/// earlier row read ([`read_identity`]).
fn rim_pinned<T: Decide>(
    a: &[Point3<T>],
    b: &[Point3<T>],
    band: Band,
) -> Result<bool, Indeterminate> {
    read_identity(
        a.iter()
            .zip(b)
            .map(|(p, q)| (LOFT_RIM_PINNED, Margin::norm3(*q - *p))),
        band,
    )
    .holds()
}

/// [`rim_pinned`]'s predicate name, carried to the funnel by
/// [`read_identity`].
const LOFT_RIM_PINNED: &str = "loft_rim_pinned";

/// **The stacking door**, at the site the skin follows ([`fn@super::build`]):
/// slab `s` is the pair of sections `(s, s + 1)`, read on their
/// placements and canonical loops in `T`. Slab by slab, in section
/// order, so the first refusal is the earliest slab's:
///
/// 1. **Corner classes.** Every corner of every loop is decided pinned
///    or moving in the slab by one predicate ([`corner_pinned`]); one
///    the band cannot call escalates ([`LoftError::CornerEscalated`]).
///    A slab whose every corner is pinned and whose every segment is the
///    same curve in both sections is coincidence
///    ([`LoftError::DegenerateStacking`]). At v-degree 2 or more, a wall
///    spans every slab, so a corner pinned in one slab read so far and
///    moving in another refuses ([`LoftError::PinnedCorner`]): the
///    wall's boundary retraces itself there.
/// 2. **The clearance** ([`clearance`]): the two sections touch nowhere
///    the door can verify beyond the pinned corners they share
///    ([`LoftError::SectionsTouch`]).
/// 3. **The step** ([`step`]): the mean displacement of the corners
///    that move, along section `s`'s normal, then along section
///    `s + 1`'s, refusing at the first verdict that is not `Positive`
///    ([`LoftError::DegenerateStacking`],
///    [`LoftError::ReversedStacking`], [`LoftError::FarSectionNotForward`]
///    and their escalations).
///
/// Each sub-arc joint ([`Joints`]) is classed with the corners in step
/// 1, by the same predicate ([`LoftError::JointEscalated`] where the band
/// cannot call it), and refuses with them in class 2.
///
/// What passes is each corner's and joint's class per slab ([`Pinned`]): a corner
/// pinned throughout the span its wall covers is one vertex there, and
/// the skin writes it as one point ([`mod@crate::skin`]'s `pin_strips`), so
/// each wall beside it has a collapsed side. A one-segment loop's vertex
/// is pinned the same way: the loft cuts its full turn at its sub-arc
/// joints, so the vertex is a corner between two quarter walls.
///
/// Every refusal names the first loop, corner or segment pair within its
/// slab.
///
/// # Errors
///
/// The arms above, [`LoftError::ClearanceUndecided`] and
/// [`LoftError::ClearanceFault`] from the clearance, and
/// [`LoftError::SectionStructure`] for a `places`/`canonical`/`strips`
/// disagreement or a section count below two, which `build`'s callers
/// refuse before.
pub(super) fn stacking_door<T: Decide>(
    places: &[SketchPlane<T>],
    canonical: &[ValidatedProfile<T>],
    strips: &Strips<T>,
    joints: &Joints,
    v_degree: usize,
    band: Band,
    tol: Tol,
) -> Result<Pinned, LoftError> {
    let k = places.len();
    let shaped = canonical.len() == k
        && strips.len() == canonical[0].loops().len()
        && strips
            .iter()
            .flatten()
            .all(|strip| strip.len() == k && strip.iter().all(|c| c.len() >= 2))
        && joints.len() == strips.len()
        && joints
            .iter()
            .zip(strips)
            .all(|(js, lp)| js.len() == lp.len());
    if k < 2 || !shaped {
        return Err(LoftError::SectionStructure);
    }
    let slabs = k - 1;
    let table = || -> Vec<Vec<Vec<bool>>> {
        strips
            .iter()
            .map(|lp| vec![vec![false; slabs]; lp.len()])
            .collect()
    };
    let mut pinned = Pinned {
        corners: table(),
        flats: table(),
        joints: joints
            .iter()
            .map(|lp| {
                lp.iter()
                    .map(|js| vec![vec![false; slabs]; js.len()])
                    .collect()
            })
            .collect(),
    };
    for slab in 0..slabs {
        classify(strips, joints, &mut pinned, slab, band)?;
        if v_degree >= 2 {
            retraced(&pinned, slab)?;
        }
        clearance(strips, canonical, places, &pinned, slab, band, tol)?;
        step(strips, places, &pinned, slab, band)?;
    }
    Ok(pinned)
}

/// Corner `j` of loop `l`, pinned in `slab`, as a refusal names it: the
/// segment it starts where the next corner is pinned there too, the one
/// it ends where the previous one is, and the corner alone otherwise.
fn site(pinned: &Pinned, l: usize, j: usize, slab: usize) -> PinnedSite {
    let n = pinned.corners[l].len();
    if pinned.at(l, (j + 1) % n, slab) {
        PinnedSite::Segment(j)
    } else if pinned.at(l, (j + n - 1) % n, slab) {
        PinnedSite::Segment((j + n - 1) % n)
    } else {
        PinnedSite::Corner(j)
    }
}

/// Step 1 of [`stacking_door`] for one slab: its corner classes, then
/// coincidence.
fn classify<T: Decide>(
    strips: &Strips<T>,
    joints: &Joints,
    pinned: &mut Pinned,
    slab: usize,
    band: Band,
) -> Result<(), LoftError> {
    for (l, lp) in strips.iter().enumerate() {
        for (j, strip) in lp.iter().enumerate() {
            for (i, &c) in joints[l][j].iter().enumerate() {
                pinned.joints[l][j][i][slab] =
                    corner_pinned(strip[slab][c], strip[slab + 1][c], band).map_err(|source| {
                        LoftError::JointEscalated {
                            loop_index: l,
                            segment: j,
                            joint: i,
                            slab,
                            source,
                        }
                    })?;
            }
        }
    }
    for (l, lp) in strips.iter().enumerate() {
        for (j, strip) in lp.iter().enumerate() {
            pinned.corners[l][j][slab] = corner_pinned(strip[slab][0], strip[slab + 1][0], band)
                .map_err(|source| LoftError::CornerEscalated {
                    loop_index: l,
                    corner: j,
                    slab,
                    source,
                })?;
        }
    }
    for (l, lp) in strips.iter().enumerate() {
        let n = lp.len();
        for (j, strip) in lp.iter().enumerate() {
            if !(pinned.corners[l][j][slab] && pinned.corners[l][(j + 1) % n][slab]) {
                continue;
            }
            let segment = LoopSegment {
                loop_index: l,
                segment: j,
            };
            pinned.flats[l][j][slab] =
                rim_pinned(&strip[slab], &strip[slab + 1], band).map_err(|source| {
                    LoftError::ClearanceUndecided {
                        slab,
                        segments: [segment, segment],
                        source,
                    }
                })?;
        }
    }
    if pinned.flats.iter().flatten().all(|slabs| slabs[slab]) {
        return Err(LoftError::DegenerateStacking { slab });
    }
    Ok(())
}

/// Step 1's class 2 at v-degree 2 or more, once slab `slab` is
/// classified: a corner or a sub-arc joint pinned in one of slabs
/// `0..=slab` and moving in another, named by the first slab that pins
/// it and the first it moves in; and a segment held flat in one and not
/// another, named as the segment, whose wall through every section holds
/// one curve twice.
fn retraced(pinned: &Pinned, slab: usize) -> Result<(), LoftError> {
    for (l, lp) in pinned.flats.iter().enumerate() {
        for (j, by_slab) in lp.iter().enumerate() {
            let read = &by_slab[..=slab];
            if let (Some(first), Some(moving)) =
                (read.iter().position(|&p| p), read.iter().position(|&p| !p))
            {
                return Err(LoftError::PinnedCorner {
                    loop_index: l,
                    at: PinnedSite::Segment(j),
                    slab: first,
                    moving,
                });
            }
        }
    }
    for (l, lp) in pinned.corners.iter().enumerate() {
        for (j, by_slab) in lp.iter().enumerate() {
            let read = &by_slab[..=slab];
            if let (Some(first), Some(moving)) =
                (read.iter().position(|&p| p), read.iter().position(|&p| !p))
            {
                return Err(LoftError::PinnedCorner {
                    loop_index: l,
                    at: site(pinned, l, j, first),
                    slab: first,
                    moving,
                });
            }
        }
    }
    for (l, lp) in pinned.joints.iter().enumerate() {
        for (j, js) in lp.iter().enumerate() {
            for (i, by_slab) in js.iter().enumerate() {
                let read = &by_slab[..=slab];
                if let (Some(first), Some(moving)) =
                    (read.iter().position(|&p| p), read.iter().position(|&p| !p))
                {
                    return Err(LoftError::PinnedCorner {
                        loop_index: l,
                        at: PinnedSite::Joint {
                            segment: j,
                            joint: i,
                        },
                        slab: first,
                        moving,
                    });
                }
            }
        }
    }
    Ok(())
}

/// One segment of one section as the clearance reads it: where it is,
/// its two corners' world points, and its placed carrier and span (the
/// cap rims' own, [`placed_segment_spec`]).
struct Piece<T: Decide> {
    at: LoopSegment,
    ends: (Point3<T>, Point3<T>),
    line: bool,
    carrier: Curve3<T>,
    span: (T, T),
}

impl<T: Decide> Piece<T> {
    /// The segment's corners: `(loop, corner)` of its start and its end.
    fn corners(&self, n: usize) -> [(usize, usize); 2] {
        let LoopSegment {
            loop_index,
            segment,
        } = self.at;
        [(loop_index, segment), (loop_index, (segment + 1) % n)]
    }

    /// The corner a reading's end of this span is.
    fn corner_at(&self, end: SpanEnd, n: usize) -> (usize, usize) {
        self.corners(n)[usize::from(end == SpanEnd::Stop)]
    }
}

/// Section `i`'s segments as [`clearance`] reads them.
fn pieces<T: Decide>(
    strips: &Strips<T>,
    canonical: &ValidatedProfile<T>,
    place: &SketchPlane<T>,
    i: usize,
    tol: Tol,
) -> Vec<Piece<T>> {
    strips
        .iter()
        .zip(canonical.loops())
        .enumerate()
        .flat_map(|(l, (lp, profile_loop))| {
            lp.iter().enumerate().map(move |(j, strip)| {
                let c = &strip[i];
                let ends = (c[0], c[c.len() - 1]);
                let seg = SweptSeg::forward(profile_loop, j);
                let spec = placed_segment_spec(&seg, place.frame(), ends.0, tol);
                Piece {
                    at: LoopSegment {
                        loop_index: l,
                        segment: j,
                    },
                    ends,
                    line: matches!(profile_loop.segments()[j].kind, SegmentKind::Line { .. }),
                    carrier: spec.carrier,
                    span: (spec.param_start, spec.param_end),
                }
            })
        })
        .collect()
}

/// Step 2 of [`stacking_door`]: **the sections touch nowhere the door
/// can verify beyond what they share.** The census's number for a pair
/// of cells (`[topology.census]`), read for every segment of section
/// `slab` against every segment of section `slab + 1`, through the
/// readers the census reads cells with:
///
/// - **two lines sharing nothing** by the census's readings of two line
///   edges ([`topo::line_segments_meet`]);
/// - **two lines sharing a pinned corner** by the census's collinear
///   overlap ([`topo::line_segments_overlap`]): two lines through one
///   point meet beyond it exactly where they share a run, which is
///   where they leave it along one ray;
/// - **a pair with an arc in it** by the curve-pair reader the census's
///   curved lane reads ([`geom_brep::curve_pair::meet`]), on the placed
///   carriers and spans: a meeting it decided on both spans that is not
///   a pinned corner the two share, read at an end of both, refuses;
/// - **one segment pinned at both ends** is the edge both sections hold
///   where it is the same curve in both (a line always is), and is read
///   as the pair it is otherwise (a lune).
///
/// **Only verified contact refuses.** Line pairs converge, so their
/// in-band readings escalate. An arc pair the reader cannot settle (in
/// band, a budget, a pose it has no closed form for) decides nothing
/// here, as before the door: whether such a pair is embedded is the
/// embedding certificate's
/// (`work/lathe/self-overlapping-spines-build-and-validate.md`). The
/// reader poses the arcs' whole carriers, so a near-tangency off both
/// spans is one such pair.
///
/// Arcs and lines agree at a shared pinned corner: two arcs leaving it
/// tangent to each other meet beyond it only where they lie on one
/// circle the same way, a shared run, as two lines along one ray do; on
/// different circles they meet at the corner alone, and are apart. A
/// touch at a point that is a shared pinned corner is legal and the same
/// touch off every corner refuses: the first is one vertex of the body,
/// the second its boundary meeting itself.
///
/// # Errors
///
/// [`LoftError::SectionsTouch`] naming the first pair that meets;
/// [`LoftError::ClearanceUndecided`] for a line pair the band cannot
/// call; [`LoftError::ClearanceFault`] where the reader's readings
/// contradict each other.
fn clearance<T: Decide>(
    strips: &Strips<T>,
    canonical: &[ValidatedProfile<T>],
    places: &[SketchPlane<T>],
    pinned: &Pinned,
    slab: usize,
    band: Band,
    tol: Tol,
) -> Result<(), LoftError> {
    let (base, far) = (
        pieces(strips, &canonical[slab], &places[slab], slab, tol),
        pieces(
            strips,
            &canonical[slab + 1],
            &places[slab + 1],
            slab + 1,
            tol,
        ),
    );
    for x in &base {
        for y in &far {
            let segments = [x.at, y.at];
            let undecided = |source| LoftError::ClearanceUndecided {
                slab,
                segments,
                source,
            };
            let n = strips[x.at.loop_index].len();
            let shared: Vec<(usize, usize)> = if x.at.loop_index == y.at.loop_index {
                x.corners(n)
                    .into_iter()
                    .filter(|c| y.corners(n).contains(c) && pinned.at(c.0, c.1, slab))
                    .collect()
            } else {
                Vec::new()
            };
            let rows = |p: &Piece<T>, i: usize| &strips[p.at.loop_index][p.at.segment][i];
            let meets = if x.at == y.at
                && shared.len() == 2
                && (x.line
                    || rim_pinned(rows(x, slab), rows(y, slab + 1), band).map_err(undecided)?)
            {
                false
            } else if x.line && y.line && shared.is_empty() {
                topo::line_segments_meet(x.ends, y.ends, band).map_err(undecided)?
            } else if x.line && y.line {
                topo::line_segments_overlap(x.ends, y.ends, band).map_err(undecided)?
            } else {
                arc_pair_meets(x, y, &shared, n, band).map_err(|readings| {
                    LoftError::ClearanceFault {
                        slab,
                        segments,
                        readings,
                    }
                })?
            };
            if meets {
                return Err(LoftError::SectionsTouch {
                    slab,
                    segments,
                    shared: shared.len(),
                });
            }
        }
    }
    Ok(())
}

/// Whether the curve-pair reader verified a meeting of two segments, one
/// an arc, that is not a pinned corner they share (`shared`) read at an
/// end of both. A refusal of the reader verifies only the meetings it
/// decided before it ([`curve_pair::MeetRefusal::found`]).
///
/// # Errors
///
/// The readings, where the reader's own readings contradicted each other
/// or it misread a carrier's kind: a kernel fault.
fn arc_pair_meets<T: Decide>(
    x: &Piece<T>,
    y: &Piece<T>,
    shared: &[(usize, usize)],
    n: usize,
    band: Band,
) -> Result<bool, &'static str> {
    let off_shared = |m: &curve_pair::Meeting<T>| match (m.on_a, m.on_b) {
        (Hit::End(ea), Hit::End(eb)) => {
            let c = x.corner_at(ea, n);
            !(c == y.corner_at(eb, n) && shared.contains(&c))
        }
        _ => true,
    };
    let (a, b) = (Span::new(&x.carrier, x.span), Span::new(&y.carrier, y.span));
    match curve_pair::meet(&a, &b, band) {
        Ok(met) => Ok(met.points.iter().any(off_shared)),
        Err(refusal) => match refusal.fault.kind() {
            MeetKind::Fault { what } => Err(what),
            MeetKind::Escalated(_) | MeetKind::Touch(_) | MeetKind::Unread(_) => {
                Ok(refusal.found.iter().any(off_shared))
            }
        },
    }
}

/// Step 3 of [`stacking_door`]: **the slab's step along its two
/// normals**, read on the corners that move. It is the mean
/// displacement of the outer loop's corners not pinned in the slab,
/// decided along section `slab`'s normal and then section `slab + 1`'s;
/// pinned corners have no step to give, and read in they would dilute
/// a turn. Where the outer loop is held (every corner pinned), only a
/// hole can step, so it reads the holes' moving corners: that answers
/// Zero where the held loop fixes the sections' common plane (three
/// corners off one line) and the holes move in it, and a direction only
/// where the held loop is a chord's two ends (a lens) and the sections
/// turn about it. Where no corner moves at all (a loop turned about the
/// chord its two corners span) each segment that moves steps on its own:
/// the mean displacement of its interior control rows, decided as the
/// corners' is, so a half disc turned about its chord steps and a lens
/// turned about its chord, whose two arcs move to opposite sides, is a
/// slab stacking against itself.
fn step<T: Decide>(
    strips: &Strips<T>,
    places: &[SketchPlane<T>],
    pinned: &Pinned,
    slab: usize,
    band: Band,
) -> Result<(), LoftError> {
    let moving = |loops: &[Vec<Vec<Vec<Point3<T>>>>]| {
        let mut d = Vec3::new(T::zero(), T::zero(), T::zero());
        let mut count = 0usize;
        for (l, lp) in loops.iter().enumerate() {
            for (j, strip) in lp.iter().enumerate() {
                if !pinned.at(l, j, slab) {
                    d = d + (strip[slab + 1][0] - strip[slab][0]);
                    count += 1;
                }
            }
        }
        (count > 0).then_some((d, count))
    };
    let outer = || moving(&strips[..1]);
    // The outer loop's corners are all pinned here, so every loop's
    // moving corners are the holes'.
    let holes_under_a_held_outer = || moving(strips);
    let Some((d, count)) = outer().or_else(holes_under_a_held_outer) else {
        for (l, lp) in strips.iter().enumerate() {
            for (j, strip) in lp.iter().enumerate() {
                if pinned.flat(l, j, slab) {
                    continue;
                }
                let (a, b) = (&strip[slab], &strip[slab + 1]);
                let inner = 1..a.len().saturating_sub(1);
                let d = inner
                    .clone()
                    .fold(Vec3::new(T::zero(), T::zero(), T::zero()), |d, i| {
                        d + (b[i] - a[i])
                    });
                facing_both(places, slab, d, inner.len(), band)?;
            }
        }
        return Ok(());
    };
    facing_both(places, slab, d, count, band)
}

/// The step `d` summed over `count` rows, decided along section `slab`'s
/// normal and then section `slab + 1`'s ([`step`]).
fn facing_both<T: Decide>(
    places: &[SketchPlane<T>],
    slab: usize,
    d: Vec3<T>,
    count: usize,
    band: Band,
) -> Result<(), LoftError> {
    #[allow(clippy::cast_precision_loss)]
    let count = T::from_f64(count.max(1) as f64);
    let facing = |place: &SketchPlane<T>| {
        decide(
            "loft_stacking",
            Margin::of(d.dot(place.normal().get()) / count),
            band,
        )
    };
    match facing(&places[slab]).map_err(|source| LoftError::StackingEscalated { slab, source })? {
        Sign::Positive => {}
        Sign::Zero => return Err(LoftError::DegenerateStacking { slab }),
        Sign::Negative => return Err(LoftError::ReversedStacking { slab }),
    }
    match facing(&places[slab + 1])
        .map_err(|source| LoftError::FarStackingEscalated { slab, source })?
    {
        Sign::Positive => Ok(()),
        Sign::Zero | Sign::Negative => Err(LoftError::FarSectionNotForward { slab }),
    }
}

/// **A row decided apart answers "not pinned" past one in band**
/// ([`rim_pinned`] reads its rows as one identity). Read through the
/// production reader.
#[cfg(test)]
#[allow(clippy::expect_used)]
mod rim_pinned_rows {
    use geom_core::{Band, Point3, Tol};

    use super::rim_pinned;

    #[test]
    fn a_row_apart_answers_past_a_row_in_band() {
        let tol = Tol::witness();
        let band = Band::linear(tol).expect("a valid band");
        let m = 0.5 * (1.0 + tol.k()) * tol.eps();
        let a = [Point3::new(0.0, 0.0, 0.0), Point3::new(1.0, 0.0, 0.0)];
        let b = |off: f64| [Point3::new(m, 0.0, 0.0), Point3::new(1.0, off, 0.0)];
        assert_eq!(
            rim_pinned(&a, &b(1.0), band),
            Ok(false),
            "a row a unit apart is not pinned, whatever the first row read"
        );
        assert!(
            rim_pinned(&a, &b(0.0), band).is_err(),
            "with every other row pinned, the row in band refuses"
        );
    }
}
