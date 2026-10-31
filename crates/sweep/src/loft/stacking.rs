//! **The stacking door** — what the loft decides about each slab before
//! the skin: its pinned corners, whether its two sections are apart
//! beyond them, and which way it stacks ([`stacking_door`]).

use geom::NurbsCurve3;
use geom_core::spline::SpanLocate;
use geom_core::{Band, Decide, Indeterminate, Margin, Point3, Sign, Vec3};
use profile::{SegmentKind, SketchPlane, ValidatedProfile};

use super::{LoftError, LoopSegment, PinnedSite};
use crate::skin::compatible_strips;
use crate::swept::decide;

/// The compatible strips `[loop][segment][section]` the door reads: each
/// segment's section curves, placed, on one knot vector.
type Strips<T> = Vec<Vec<Vec<NurbsCurve3<T>>>>;

/// **Which corners each slab pins**, `[loop][corner][slab]`: corner `j`
/// is the start of segment `j`, and it is pinned in slab `s` where its
/// world point in section `s` and in section `s + 1` decide one point
/// ([`corner_pinned`]).
pub(super) struct Pinned(Vec<Vec<Vec<bool>>>);

impl Pinned {
    /// The outer loop's corners pinned in every slab: the vertex rows
    /// the skin's parameterization leaves out of its average.
    pub(super) fn still_outer(&self) -> Vec<bool> {
        self.0
            .first()
            .map(|lp| lp.iter().map(|slabs| slabs.iter().all(|&p| p)).collect())
            .unwrap_or_default()
    }

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
    a: &NurbsCurve3<T>,
    b: &NurbsCurve3<T>,
    band: Band,
) -> Result<bool, Indeterminate> {
    for (p, q) in a.control().iter().zip(b.control()) {
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
/// 3. **The step.** The mean displacement of the outer loop's moving
///    control rows (its compatible rows, the pinned corners left out)
///    along section `s`'s normal, then along section `s + 1`'s, each
///    under `loft_stacking` and refusing at the first verdict that is
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
/// The arms above, and [`LoftError::Skin`] where the sections are not
/// compatible (the skin's own refusal, met here first), and
/// [`LoftError::SectionStructure`] for a `places`/`canonical` length
/// disagreement or a section count below two, which `build`'s
/// callers refuse before.
pub(super) fn stacking_door<T: Decide + SpanLocate>(
    places: &[SketchPlane<T>],
    canonical: &[ValidatedProfile<T>],
    v_degree: usize,
    band: Band,
) -> Result<Pinned, LoftError> {
    if places.len() < 2 || places.len() != canonical.len() {
        return Err(LoftError::SectionStructure);
    }
    let strips = compatible_strips(canonical, places).map_err(LoftError::Skin)?;
    let pinned = corner_classes(&strips, v_degree, band)?;
    for slab in 0..places.len() - 1 {
        clearance(&strips, canonical, places, &pinned, slab, band)?;
    }
    for slab in 0..places.len() - 1 {
        step(&strips, places, &pinned, slab, band)?;
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
    Ok(pinned)
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
                let corner = |i: usize| strip[i].control()[0];
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

/// One segment of one section as the clearance reads it: its curve, and
/// whether it is a line (read by its two ends) or an arc (read by its
/// control polygon, whose convex hull holds it: every weight is
/// positive).
struct Piece<'a, T: Decide> {
    at: LoopSegment,
    curve: &'a NurbsCurve3<T>,
    line: bool,
}

impl<T: Decide> Piece<'_, T> {
    fn ends(&self) -> (Point3<T>, Point3<T>) {
        let c = self.curve.control();
        (c[0], c[c.len() - 1])
    }

    /// The segment's corners: `(loop, corner)` of its start and its end.
    fn corners(&self, n: usize) -> [(usize, usize); 2] {
        let LoopSegment {
            loop_index,
            segment,
        } = self.at;
        [(loop_index, segment), (loop_index, (segment + 1) % n)]
    }
}

/// Step 2 of [`stacking_door`]: **the sections are apart beyond what they
/// share.** The census's number for a pair of cells (`[topology.census]`)
/// over every segment of section `slab` against every segment of section
/// `slab + 1`:
///
/// - **sharing nothing**, two lines are read by the census's own
///   readings of two line edges ([`topo::segments_meet`]); a pair with an
///   arc in it is apart where one curve's control polygon lies wholly on
///   one side of the other's section plane, past the band
///   (`loft_section_clearance`), which holds the curve that lies in it
///   off its hull;
/// - **sharing a pinned corner**, two lines meet beyond it exactly where
///   they are parallel (the census's sine reading,
///   [`topo::segments_parallel`]) and leave the corner the same way; a
///   pair with an arc in it is apart beyond the corner where the
///   control points of one, the shared corners left out, lie wholly on
///   one side of the other's section plane, since the hull then meets
///   that plane only at the shared corners and their chord, which the
///   arc meets only at its ends;
/// - **one segment pinned at both ends** is the edge both sections hold
///   where it is the same curve in both (a line always is), and is read
///   as the pair it is otherwise (a lune).
///
/// # Errors
///
/// [`LoftError::SectionsTouch`] naming the first pair that meets;
/// [`LoftError::ClearanceUndecided`] for one the band cannot call
/// (`Some`), or an arc pair no plane separates (`None`: the frontier
/// of the control-polygon reading, not a verdict that they meet).
fn clearance<T: Decide>(
    strips: &Strips<T>,
    canonical: &[ValidatedProfile<T>],
    places: &[SketchPlane<T>],
    pinned: &Pinned,
    slab: usize,
    band: Band,
) -> Result<(), LoftError> {
    let pieces = |i: usize| -> Vec<Piece<'_, T>> {
        strips
            .iter()
            .enumerate()
            .flat_map(|(l, lp)| {
                lp.iter().enumerate().map(move |(j, strip)| Piece {
                    at: LoopSegment {
                        loop_index: l,
                        segment: j,
                    },
                    curve: &strip[i],
                    line: matches!(
                        canonical[i].loops()[l].segments()[j].kind,
                        SegmentKind::Line
                    ),
                })
            })
            .collect()
    };
    let plane = |i: usize| (places[i].origin(), places[i].normal().get());
    let (base, far) = (pieces(slab), pieces(slab + 1));
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
            let apart = if shared.is_empty() {
                if x.line && y.line {
                    !topo::segments_meet(x.ends(), y.ends(), band)
                        .map_err(|s| undecided(Some(s)))?
                } else {
                    let sides = [
                        one_side(y.curve.control(), plane(slab), band),
                        one_side(x.curve.control(), plane(slab + 1), band),
                    ];
                    match sides {
                        [Ok(true), _] | [_, Ok(true)] => true,
                        [Err(s), _] | [_, Err(s)] => return Err(undecided(Some(s))),
                        [Ok(false), Ok(false)] => return Err(undecided(None)),
                    }
                }
            } else if x.at == y.at
                && shared.len() == 2
                && (x.line || rim_pinned(x.curve, y.curve, band).map_err(|s| undecided(Some(s)))?)
            {
                true
            } else if x.line && y.line {
                // Each line from its own copy of the shared corner.
                let away = |p: &Piece<'_, T>| {
                    let (start, end) = p.ends();
                    if p.corners(n)[0] == shared[0] {
                        end - start
                    } else {
                        start - end
                    }
                };
                if topo::segments_parallel(x.ends(), y.ends(), band)
                    .map_err(|s| undecided(Some(s)))?
                {
                    let (ux, uy) = (away(x), away(y));
                    match decide(
                        "loft_section_clearance",
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
                let beyond = |p: &Piece<'_, T>| -> Vec<Point3<T>> {
                    let c = p.curve.control();
                    let [start, end] = p.corners(n);
                    let last = c.len() - 1;
                    c.iter()
                        .enumerate()
                        .filter(|&(i, _)| {
                            !(i == 0 && shared.contains(&start)
                                || i == last && shared.contains(&end))
                        })
                        .map(|(_, q)| *q)
                        .collect()
                };
                let sides = [
                    one_side(&beyond(y), plane(slab), band),
                    one_side(&beyond(x), plane(slab + 1), band),
                ];
                match sides {
                    [Ok(true), _] | [_, Ok(true)] => true,
                    [Err(s), _] | [_, Err(s)] => return Err(undecided(Some(s))),
                    [Ok(false), Ok(false)] => return Err(undecided(None)),
                }
            };
            if !apart {
                return Err(LoftError::SectionsTouch { slab, segments });
            }
        }
    }
    Ok(())
}

/// Whether every point lies strictly on one side of a plane, past the
/// band (`loft_section_clearance`): `Ok(false)` where some point is on
/// it or the points straddle it.
///
/// # Errors
///
/// The first escalation, where no side is decided otherwise.
fn one_side<T: Decide>(
    points: &[Point3<T>],
    (origin, normal): (Point3<T>, Vec3<T>),
    band: Band,
) -> Result<bool, Indeterminate> {
    let mut side = None;
    for p in points {
        let s = decide(
            "loft_section_clearance",
            Margin::of((*p - origin).dot(normal)),
            band,
        )?;
        match (s, side) {
            (Sign::Zero, _) => return Ok(false),
            (_, None) => side = Some(s),
            (_, Some(first)) if first != s => return Ok(false),
            _ => {}
        }
    }
    Ok(side.is_some())
}

/// Step 3 of [`stacking_door`]: the slab's step along its two normals,
/// read on the outer loop's moving control rows — each strip's rows but
/// its last (the next strip's first), its first left out where that
/// corner is pinned in the slab. The mean of their displacements is
/// decided along section `slab`'s normal and then section `slab + 1`'s.
/// Pinned rows are left out because they have no step to give: read off
/// every vertex, the mean misses a hinge and dilutes a turn.
fn step<T: Decide>(
    strips: &Strips<T>,
    places: &[SketchPlane<T>],
    pinned: &Pinned,
    slab: usize,
    band: Band,
) -> Result<(), LoftError> {
    let mut d = Vec3::new(T::zero(), T::zero(), T::zero());
    let mut count = 0usize;
    for (j, strip) in strips[0].iter().enumerate() {
        let (a, b) = (strip[slab].control(), strip[slab + 1].control());
        let first = usize::from(pinned.at(0, j, slab));
        for (p, q) in a.iter().zip(b).take(a.len() - 1).skip(first) {
            d = d + (*q - *p);
            count += 1;
        }
    }
    #[allow(clippy::cast_precision_loss)]
    let count = T::from_f64(count.max(1) as f64);
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
