//! **The stacking door** — what the loft decides about each slab before
//! the skin: its pinned corners, whether its two sections are apart
//! beyond them, and which way it stacks ([`stacking_door`]).

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
/// placements and canonical loops in `T`. In this order:
///
/// 1. **Corner classes.** Every corner of every loop is decided pinned
///    or moving in every slab by one predicate ([`corner_pinned`]); one
///    the band cannot call escalates ([`LoftError::CornerEscalated`]).
///    A slab whose every corner is pinned and whose every segment is the
///    same curve in both sections is coincidence
///    ([`LoftError::DegenerateStacking`]). At v-degree 2 or more through
///    three sections or more, a wall spans every slab, so a corner pinned
///    in some slabs and moving in others refuses
///    ([`LoftError::PinnedCorner`]): the wall's boundary retraces itself
///    there.
/// 2. **The clearance.** The two sections are apart beyond what they
///    share: every segment of one against every segment of the other,
///    the pinned corners they share left out ([`clearance`]). A pair
///    that meets refuses ([`LoftError::SectionsTouch`]).
/// 3. **The step.** The mean displacement of the corners that move
///    ([`step`]) along section `s`'s normal, then along section
///    `s + 1`'s, each under `loft_stacking` and refusing at the first verdict that is
///    not `Positive` ([`LoftError::DegenerateStacking`],
///    [`LoftError::ReversedStacking`], [`LoftError::FarSectionNotForward`]
///    and their escalations).
/// 4. **A pinned corner the kernel cannot build**
///    ([`LoftError::PinnedCornerUnsupported`]): one the classes accept,
///    pinned throughout the span its wall covers.
///
/// Every refusal names the first slab, in section order, that reaches
/// it, and the first loop, corner or segment pair within it.
///
/// # Errors
///
/// The arms above, and [`LoftError::SectionStructure`] for a
/// `places`/`canonical`/`strips` disagreement or a section count below
/// two, which `build`'s callers refuse before.
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
    let pinned = corner_classes(strips, v_degree, band)?;
    for slab in 0..places.len() - 1 {
        clearance(strips, canonical, places, &pinned, slab, band, tol)?;
    }
    for slab in 0..places.len() - 1 {
        step(strips, places, &pinned, slab, band)?;
    }
    for slab in 0..places.len() - 1 {
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

/// Step 1 of [`stacking_door`]: the corner classes, slab by slab, then
/// the corners a wall spanning every slab cannot take.
fn corner_classes<T: Decide>(
    strips: &Strips<T>,
    v_degree: usize,
    band: Band,
) -> Result<Pinned, LoftError> {
    let sections = strips[0][0].len();
    let slabs = sections - 1;
    let mut pinned: Vec<Vec<Vec<bool>>> = strips
        .iter()
        .map(|lp| vec![vec![false; slabs]; lp.len()])
        .collect();
    for slab in 0..slabs {
        for (l, lp) in strips.iter().enumerate() {
            for (j, strip) in lp.iter().enumerate() {
                let corner = |i: usize| strip[i][0];
                pinned[l][j][slab] =
                    corner_pinned(corner(slab), corner(slab + 1), band).map_err(|source| {
                        LoftError::CornerEscalated {
                            loop_index: l,
                            corner: j,
                            slab,
                            source,
                        }
                    })?;
            }
        }
        if pinned.iter().flatten().all(|slabs| slabs[slab]) {
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
                                source: Some(source),
                            }
                        })?;
                }
            }
            if coincident {
                return Err(LoftError::DegenerateStacking { slab });
            }
        }
    }
    let pinned = Pinned(pinned);
    if v_degree >= 2 && slabs >= 2 {
        for (l, lp) in pinned.0.iter().enumerate() {
            for (j, by_slab) in lp.iter().enumerate() {
                let (Some(slab), Some(moving)) = (
                    by_slab.iter().position(|&p| p),
                    by_slab.iter().position(|&p| !p),
                ) else {
                    continue;
                };
                return Err(LoftError::PinnedCorner {
                    loop_index: l,
                    at: site(&pinned, l, j, slab),
                    slab,
                    moving,
                });
            }
        }
    }
    Ok(pinned)
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

/// Step 2 of [`stacking_door`]: **the sections are apart beyond what they
/// share.** The census's number for a pair of cells (`[topology.census]`),
/// read for every segment of section `slab` against every segment of
/// section `slab + 1`, through the readers the census reads cells with:
///
/// - **two lines sharing nothing** by the census's readings of two line
///   edges ([`topo::line_segments_meet`]);
/// - **two lines sharing a pinned corner** by the census's sine reading
///   ([`topo::segments_parallel`]): they meet beyond the corner exactly
///   where they are parallel and leave it the same way
///   (`loft_shared_corner_lead`);
/// - **a pair with an arc in it** by the curve-pair reader the census's
///   curved lane reads ([`geom_brep::curve_pair::meet`]), on the placed
///   carriers: every meeting it finds must be a pinned corner the two
///   share, read at an end of both;
/// - **one segment pinned at both ends** is the edge both sections hold
///   where it is the same curve in both (a line always is), and is read
///   as the pair it is otherwise (a lune).
///
/// # Errors
///
/// [`LoftError::SectionsTouch`] naming the first pair that meets;
/// [`LoftError::ClearanceUndecided`] for one the band cannot call
/// (`Some`), or one the curve-pair reader cannot settle (`None`).
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
            let apart = if x.at == y.at
                && shared.len() == 2
                && (x.line
                    || rim_pinned(rows(x, slab), rows(y, slab + 1), band)
                        .map_err(|s| undecided(Some(s)))?)
            {
                true
            } else if x.line && y.line && shared.is_empty() {
                !topo::line_segments_meet(x.ends, y.ends, band).map_err(|s| undecided(Some(s)))?
            } else if x.line && y.line {
                // Each line from its own copy of the shared corner.
                let away = |p: &Piece<T>| {
                    if p.corners(n)[0] == shared[0] {
                        p.ends.1 - p.ends.0
                    } else {
                        p.ends.0 - p.ends.1
                    }
                };
                if topo::segments_parallel(x.ends, y.ends, band).map_err(|s| undecided(Some(s)))? {
                    let (ux, uy) = (away(x), away(y));
                    match decide(
                        "loft_shared_corner_lead",
                        Margin::of(ux.dot(uy) / uy.norm()),
                        band,
                    ) {
                        Ok(Sign::Negative) => true,
                        Ok(Sign::Positive) => false,
                        Ok(Sign::Zero) => return Err(undecided(None)),
                        Err(s) => return Err(undecided(Some(s))),
                    }
                } else {
                    true
                }
            } else {
                let (a, b) = (Span::new(&x.carrier, x.span), Span::new(&y.carrier, y.span));
                match curve_pair::meet(&a, &b, band) {
                    Ok(met) => {
                        met.shared.is_none()
                            && met.points.iter().all(|m| match (m.on_a, m.on_b) {
                                (Hit::End(ea), Hit::End(eb)) => {
                                    let c = x.corner_at(ea, n);
                                    c == y.corner_at(eb, n) && shared.contains(&c)
                                }
                                _ => false,
                            })
                    }
                    Err(refusal) => match refusal.fault {
                        MeetFault::Escalated(s) | MeetFault::Touch(s) => {
                            return Err(undecided(Some(s)));
                        }
                        _ => return Err(undecided(None)),
                    },
                }
            };
            if !apart {
                return Err(LoftError::SectionsTouch {
                    slab,
                    segments,
                    shared: !shared.is_empty(),
                });
            }
        }
    }
    Ok(())
}

/// Step 3 of [`stacking_door`]: **the slab's step along its two
/// normals**, read on the corners that move. It is the mean
/// displacement of the outer loop's corners not pinned in the slab,
/// decided along section `slab`'s normal and then section `slab + 1`'s;
/// pinned corners have no step to give, and read in they would dilute
/// a turn. Where every outer corner is pinned and a hole's corner moves,
/// it is the mean of every loop's moving corners, so sections that hold
/// their outer loop and move a hole in their plane read a zero step.
/// Where no corner moves at all (a lens turned about its chord) it
/// decides nothing: such a slab has a pinned corner, which step 4 of
/// [`stacking_door`] decides.
fn step<T: Decide>(
    strips: &Strips<T>,
    places: &[SketchPlane<T>],
    pinned: &Pinned,
    slab: usize,
    band: Band,
) -> Result<(), LoftError> {
    let moving = |loops: &[Vec<Vec<Vec<Point3<T>>>>], first: usize| {
        let mut d = Vec3::new(T::zero(), T::zero(), T::zero());
        let mut count = 0usize;
        for (l, lp) in loops.iter().enumerate() {
            for (j, strip) in lp.iter().enumerate() {
                if !pinned.at(first + l, j, slab) {
                    d = d + (strip[slab + 1][0] - strip[slab][0]);
                    count += 1;
                }
            }
        }
        (count > 0).then_some((d, count))
    };
    let Some((d, count)) = moving(&strips[..1], 0).or_else(|| moving(strips, 0)) else {
        return Ok(());
    };
    #[allow(clippy::cast_precision_loss)]
    let count = T::from_f64(count as f64);
    let facing = |place: &SketchPlane<T>| {
        geom_core::k_stats::decide(
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
