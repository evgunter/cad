//! **The stacking door** — what the loft decides about each slab before
//! the skin, slab by slab: its pinned corners, whether its two sections
//! touch beyond them, and which way it stacks ([`stacking_door`]). It
//! refuses contact it verified, never contact it could not rule out: an
//! arc pair the curve-pair reader cannot settle is left to the embedding
//! certificate (`work/carve/self-overlapping-spines-build-and-validate.md`).

use geom::Curve3;
use geom_brep::curve_pair::{self, Hit, MeetFault, Span, SpanEnd};
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
struct Pinned(Vec<Vec<Vec<bool>>>);

impl Pinned {
    fn at(&self, l: usize, j: usize, slab: usize) -> bool {
        self.0[l][j][slab]
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
/// one point: the segment is the same curve in both sections.
fn rim_pinned<T: Decide>(
    a: &[Point3<T>],
    b: &[Point3<T>],
    band: Band,
) -> Result<bool, Indeterminate> {
    for (p, q) in a.iter().zip(b) {
        if decide("loft_rim_pinned", Margin::norm3(*q - *p), band)? != Sign::Zero {
            return Ok(false);
        }
    }
    Ok(true)
}

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
/// Then, once every slab has passed, **a pinned corner the kernel cannot
/// build** ([`LoftError::PinnedCornerUnsupported`]): one pinned
/// throughout the span its wall covers. It waits for every slab, because
/// a corner pinned in an early slab is class 2 if a later one moves it,
/// and because a kernel limit is reported only where no slab holds a
/// fault of the sections.
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
    v_degree: usize,
    band: Band,
    tol: Tol,
) -> Result<(), LoftError> {
    let k = places.len();
    let shaped = canonical.len() == k
        && strips.len() == canonical[0].loops().len()
        && strips
            .iter()
            .flatten()
            .all(|strip| strip.len() == k && strip.iter().all(|c| c.len() >= 2));
    if k < 2 || !shaped {
        return Err(LoftError::SectionStructure);
    }
    let slabs = k - 1;
    let mut pinned = Pinned(
        strips
            .iter()
            .map(|lp| vec![vec![false; slabs]; lp.len()])
            .collect(),
    );
    for slab in 0..slabs {
        classify(strips, &mut pinned, slab, band)?;
        if v_degree >= 2 {
            retraced(&pinned, slab)?;
        }
        clearance(strips, canonical, places, &pinned, slab, band, tol)?;
        step(strips, places, &pinned, slab, band)?;
    }
    for slab in 0..slabs {
        for (l, lp) in pinned.0.iter().enumerate() {
            if let Some(j) = lp.iter().position(|slabs| slabs[slab]) {
                return Err(LoftError::PinnedCornerUnsupported {
                    loop_index: l,
                    at: site(&pinned, l, j, slab),
                    slab,
                });
            }
        }
    }
    Ok(())
}

/// Corner `j` of loop `l`, pinned in `slab`, as a refusal names it: the
/// segment it starts where the next corner is pinned there too, the one
/// it ends where the previous one is, and the corner alone otherwise.
fn site(pinned: &Pinned, l: usize, j: usize, slab: usize) -> PinnedSite {
    let n = pinned.0[l].len();
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
    pinned: &mut Pinned,
    slab: usize,
    band: Band,
) -> Result<(), LoftError> {
    for (l, lp) in strips.iter().enumerate() {
        for (j, strip) in lp.iter().enumerate() {
            pinned.0[l][j][slab] = corner_pinned(strip[slab][0], strip[slab + 1][0], band)
                .map_err(|source| LoftError::CornerEscalated {
                    loop_index: l,
                    corner: j,
                    slab,
                    source,
                })?;
        }
    }
    if pinned.0.iter().flatten().all(|slabs| slabs[slab]) {
        let mut coincident = true;
        for (l, lp) in strips.iter().enumerate() {
            for (j, strip) in lp.iter().enumerate() {
                let segment = LoopSegment {
                    loop_index: l,
                    segment: j,
                };
                coincident &=
                    rim_pinned(&strip[slab], &strip[slab + 1], band).map_err(|source| {
                        LoftError::ClearanceUndecided {
                            slab,
                            segments: [segment, segment],
                            source,
                        }
                    })?;
            }
        }
        if coincident {
            return Err(LoftError::DegenerateStacking { slab });
        }
    }
    Ok(())
}

/// Step 1's class 2 at v-degree 2 or more, once slab `slab` is
/// classified: a corner pinned in one of slabs `0..=slab` and moving in
/// another, named by the first slab that pins it and the first it moves
/// in.
fn retraced(pinned: &Pinned, slab: usize) -> Result<(), LoftError> {
    for (l, lp) in pinned.0.iter().enumerate() {
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
    let normal = place.normal().get();
    strips
        .iter()
        .zip(canonical.loops())
        .enumerate()
        .flat_map(|(l, (lp, profile_loop))| {
            lp.iter().enumerate().map(move |(j, strip)| {
                let c = &strip[i];
                let ends = (c[0], c[c.len() - 1]);
                let seg = SweptSeg::forward(profile_loop, j);
                let spec =
                    placed_segment_spec(&seg, place.placement(), normal, ends.0, ends.1, tol);
                Piece {
                    at: LoopSegment {
                        loop_index: l,
                        segment: j,
                    },
                    ends,
                    line: matches!(profile_loop.segments()[j].kind, SegmentKind::Line),
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
/// (`work/carve/self-overlapping-spines-build-and-validate.md`). The
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
        Err(refusal) => match refusal.fault {
            MeetFault::Contradiction { readings } => Err(readings),
            MeetFault::Desync => Err("a carrier's kind against the arm the pair table chose"),
            MeetFault::Escalated(_)
            | MeetFault::Touch(_)
            | MeetFault::Unsettled(_)
            | MeetFault::Unsupported { .. } => Ok(refusal.found.iter().any(off_shared)),
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
/// turn about it. Where no corner moves at all (a lens turned about its
/// chord) it decides nothing: such a slab has a pinned corner, which
/// [`stacking_door`] decides once every slab has passed.
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
        return Ok(());
    };
    #[allow(clippy::cast_precision_loss)]
    let count = T::from_f64(count as f64);
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
