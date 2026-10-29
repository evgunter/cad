//! Joining of null edges (ch. 14 §14.7, `splitconnect`): pair loose
//! null-edge halves into section-polygon chains, connect them with
//! real on-plane edges, and close each chain into a 2-loop **null
//! face** — recorded as F9 data
//! ([`NullFacePair::Split`](crate::null::NullFacePair)) with the
//! above/below roles determined by **vertex-key membership**, never by
//! `floops` list position or he1/he2 slot convention.
//!
//! The chord `mef`/`mekr` insertion, the ring re-homing and the
//! null-edge retirement this sweep drives are NOT here: they are
//! [`crate::chord_join`], one implementation shared with the boolean
//! lane's joining. What is here is the split lane's
//! **sweep** — the order it visits null edges in, the loose-end
//! pairing, and the role resolution, area certification and spur
//! reading it layers on the core's side-agnostic outcome, with the one
//! Euler operation it runs itself: the `kev` of a section strut.
//!
//! # The sweep (Programs 14.9/14.10, re-derived)
//!
//! Null edges are processed in the total lexicographic order of their
//! (coincident-copy) points ([`super::order`] — the ε-banded book sort
//! engineered out). Each edge offers its two halves in a fixed data
//! order (**up half first** — the half starting at `below_end`; the
//! book's "he1 first" as data, not slot). A half either consumes a
//! **loose end** or becomes one (growable typed collection; the book's
//! `ends[30]` is gone).
//!
//! Which loose end a half consumes is decided two ways:
//!
//! - **On a face with more than two crossings, its partner, fixed
//!   before the sweep** ([`fixed_partners`]). A planar face's crossings
//!   are taken in order along the face's own section line
//!   ([`super::order::sort_along_line`]), each taking the first
//!   unpaired one before it of opposite sense: the sweep's global order
//!   is monotone along that line only for exact points, and computed
//!   crossings on a line parallel to the order's `v` axis come out in
//!   rounding order (`super::order`'s module docs). A curved face's
//!   crossings lie on a conic, which no lexicographic order follows:
//!   they are paired along the conic, each entry into the face with
//!   the exit that ends the arc inside it ([`conic_pairs`]), or, where
//!   the conic is a pair of rulings, along each ruling
//!   ([`ruling_pairs`]). The partner of such a half is consumed only by
//!   it, and it consumes only its partner.
//! - **Elsewhere, the book's rule**: the first registered half in the
//!   *same face* (at the time of the scan) with the *opposite* up/down
//!   sense, among the halves with no fixed partner.
//!
//! # What this lane adds to the core's `cut`
//!
//! When [`ChordJoiner::cut_core`] reports the polygon COMPLETE, the
//! 2-loop null face comes back with its roles unresolved. This lane
//! resolves them by membership of the null edges' above-end vertex set,
//! and reads from the loops' topology whether the polygon is a **slit
//! trace** ([`SectionKind::Slit`]: one loop walks round a tree of
//! straight edges, running along each both ways; the finish kills it
//! whole and nests the other loop as a zero-area ring). Every other
//! polygon it certifies definitely of
//! positive area (**`split_section_area`**, margin 2·A/P) — a zero-area
//! section polygon is refused typed,
//! [`SplitJoinError::DegenerateSection`] — then reads its zero-width
//! spurs in pairs, a strut on one loop under a slit on the other, and
//! kills the struts (`split_section_spur`; `Sweep::strip_spurs`), so
//! the finish and the section query read one loop. Last, it writes the
//! F9 record.

use geom_core::k_stats::NonzeroSign;
use geom_core::{Band, Decide, Margin, Point3, Sign};
use slotmap::SecondaryMap;

use super::order;
use super::{SplitPlane, SplitReduction};
use crate::body::Body;
use crate::chord_join::{
    ChordJoiner, ConicCrossingsCase, CutOutcome, Datum, FragmentRows, JoinLane, Leave, SectionCase,
    SectionConic, SectionCtx, SegmentEdge, SplitJoinError, WallSection, corrupt_edge, corrupt_face,
    corrupt_he, corrupt_loop, he_face, lone_site_placeholder, vertex_point, wall_section,
};
use crate::entity::{EdgeKey, FaceKey, HalfEdgeKey, LoopBoundary, LoopKey, VertexKey};
use crate::null::{CurveGeom, NullFacePair};
use crate::validate::{decide, decide_nonzero};
use geom_core::Tol;

/// One completed section polygon: the null face and its role loops
/// (mirrors the body's [`NullFacePair`] record; carried separately so
/// the finish step consumes explicit keys in completion order).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct CompletedSection {
    /// The null face (two coincident loops).
    pub(crate) face: FaceKey,
    /// The loop through the null edges' above ends.
    pub(crate) above_loop: LoopKey,
    /// The loop through their below ends.
    pub(crate) below_loop: LoopKey,
    /// Whether the polygon bounds area or is a slit trace.
    pub(crate) kind: SectionKind,
}

/// What a completed section polygon is, read from its topology before
/// its area ([`Sweep::section_kind`]).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum SectionKind {
    /// A polygon of positive area: both loops bound the section's region.
    Area,
    /// The trace of a tree of straight edges of the operand lying in the
    /// plane, whose other side's material wraps round its every end.
    /// `strut` is the loop of the side that holds the tree, walking round
    /// it along each edge both ways, which bounds nothing on that side
    /// and dies whole; the other loop is a slit of that side's piece, two
    /// distinct edges over each of the tree's, which the finish nests as
    /// a zero-area ring of the section face that contains it.
    Slit {
        /// The loop that walks round the tree.
        strut: LoopKey,
    },
}

/// The joining sweep (module docs). Mutates `red.body` in place;
/// returns the completed section polygons in completion order, with
/// the F9 `NullFacePair::Split` records set on the body, and the
/// chord-mef fragment rows the core logged.
///
/// # Errors
///
/// [`SplitJoinError`] — the body may be left mid-surgery on `Err`
/// (callers operate on a scratch clone; the public ops discard it).
pub(super) fn split_connect<T: Decide + crate::props::AtRestPolicy>(
    red: &mut SplitReduction<T>,
    band: Band,
    tol: Tol,
) -> Result<(Vec<CompletedSection>, FragmentRows), SplitJoinError> {
    let exact = order::exact_band().map_err(SplitJoinError::Band)?;

    // The null edges' above ends (role resolution is key membership).
    let mut above_set: SecondaryMap<VertexKey, ()> = SecondaryMap::new();
    for r in &red.null_edges {
        above_set.insert(r.attr.above_end, ());
    }

    // Sort points: each null edge's coincident-copy position.
    let mut points = Vec::with_capacity(red.null_edges.len());
    for r in &red.null_edges {
        points.push(vertex_point(&red.body, r.attr.below_end));
    }
    let sorted = order::sort_indices_by_point(&points, &red.plane, band, exact)
        .map_err(|diag| SplitJoinError::OrderEscalated { diag })?;

    let partner = fixed_partners(red, &above_set, band)?;
    let mut st = Sweep {
        ends: Vec::new(),
        partner,
        joiner: ChordJoiner::new(band),
        completed: Vec::new(),
        above_set,
        plane: red.plane,
        band,
        section: SectionCtx {
            origin: red.plane.origin,
            normal: red.plane.normal,
            plane_key: None,
            branch: None,
        },
    };

    for idx in sorted {
        let record = red.null_edges[idx];
        let edge = red
            .body
            .get_edge(record.edge)
            .ok_or_else(|| corrupt_edge(record.edge))?
            .clone();
        // Data order: the up half (starting at below_end) first — the
        // book's he1-first as data (module docs).
        let (up, down) = {
            let plus_start = red
                .body
                .get_half_edge(edge.he_plus)
                .ok_or_else(|| corrupt_he(edge.he_plus))?
                .start;
            if plus_start == record.attr.below_end {
                (edge.he_plus, edge.he_minus)
            } else {
                (edge.he_minus, edge.he_plus)
            }
        };
        let mut joined = [false, false];
        for (slot, half) in [(0, up), (1, down)] {
            if let Some(end) = st.take_neighbor(&red.body, half)? {
                // A site at a cone's apex has no outward normal there: its
                // section leaves along its germ, a ruling, which the chord
                // between the two sites runs along.
                let leave = match (red.apex_germs.get(end), red.apex_germs.get(half)) {
                    (None, None) => Leave {
                        at: [
                            (
                                end,
                                split_leave(&red.body, red.plane.normal, &st.above_set, end)?,
                            ),
                            (
                                half,
                                split_leave(&red.body, red.plane.normal, &st.above_set, half)?,
                            ),
                        ],
                        datum: Datum::Section,
                    },
                    (Some(&g), None) => Leave {
                        at: [(end, g), (half, -g)],
                        datum: Datum::Germ,
                    },
                    (None, Some(&g)) => Leave {
                        at: [(end, -g), (half, g)],
                        datum: Datum::Germ,
                    },
                    (Some(&g), Some(&h)) => Leave {
                        at: [(end, g), (half, h)],
                        datum: Datum::Germ,
                    },
                };
                let Sweep {
                    joiner, section, ..
                } = &mut st;
                joiner.place_pending(&mut red.body, (end, half))?;
                let plan = joiner.plan(&red.body, (end, half), SegmentEdge::InPlane(section))?;
                let lane = JoinLane::Split(section);
                if let Some(curve) = joiner.segment_curve(&mut red.body, &plan, lane, leave)? {
                    joiner.join(&mut red.body, &plan, &curve, tol)?;
                }
                joined[slot] = true;
                // Retire the consumed end's edge if its other half is
                // no longer loose.
                let end_edge = he_edge(&red.body, end)?;
                let mate = red.body.mate(end).ok_or_else(|| corrupt_he(end))?;
                if !st.is_loose(mate) {
                    st.cut(&mut red.body, end_edge, tol)?;
                }
            }
        }
        if joined[0] && joined[1] {
            st.cut(&mut red.body, record.edge, tol)?;
        }
    }

    if !st.ends.is_empty() {
        return Err(SplitJoinError::UnpairedLooseEnds {
            count: st.ends.len(),
        });
    }
    st.joiner.finish(&red.body)?;
    let fragments = st.joiner.take_fragments();
    Ok((st.completed, fragments))
}

/// The edge of a half-edge.
fn he_edge<T: Decide>(body: &Body<T>, he: HalfEdgeKey) -> Result<EdgeKey, SplitJoinError> {
    Ok(body.get_half_edge(he).ok_or_else(|| corrupt_he(he))?.edge)
}

/// The fixed partners of the null-edge halves on faces with more than
/// two crossings (module docs), both ways round. A planar face's are
/// paired along its section line ([`line_pairs`]). A curved face's are
/// paired by what the C5 table makes of the plane against its wall
/// ([`wall_section`]): along a section conic ([`conic_pairs`]), or along
/// each of two rulings ([`ruling_pairs`]); a tangent ruling pairs
/// nothing here and keeps the book's rule.
///
/// A face's halves are taken in insertion order (null-edge record
/// order, up half first — the half starting at `below_end`, the order
/// the sweep offers them in), each at its start vertex's point.
///
/// The partners are fixed on the faces the sweep starts with. A chord
/// minted earlier in the sweep can divide a face so that two partners
/// end up in different faces; `Sweep::take_neighbor` re-checks the
/// pair's face at use and, where they parted, returns both halves to
/// the book's rule.
///
/// # Errors
///
/// [`wall_section`]'s, and [`line_pairs`]', [`conic_pairs`]' and
/// [`ruling_pairs`]', each naming the face.
///
/// # Panics
///
/// Where a face's surface does not resolve: the reduction's body is
/// mid-operation, where a face's surface is a link
/// ([`crate::live::OPERATORS_KEEP_LINKS`]), and a torn one is not
/// curved.
fn fixed_partners<T: Decide>(
    red: &SplitReduction<T>,
    above_set: &SecondaryMap<VertexKey, ()>,
    band: Band,
) -> Result<SecondaryMap<HalfEdgeKey, HalfEdgeKey>, SplitJoinError> {
    let body = &red.body;
    let mut faces: Vec<(FaceKey, Vec<HalfEdgeKey>)> = Vec::new();
    for r in &red.null_edges {
        let edge = body.get_edge(r.edge).ok_or_else(|| corrupt_edge(r.edge))?;
        let plus_start = body
            .get_half_edge(edge.he_plus)
            .ok_or_else(|| corrupt_he(edge.he_plus))?
            .start;
        let up_first = if plus_start == r.attr.below_end {
            [edge.he_plus, edge.he_minus]
        } else {
            [edge.he_minus, edge.he_plus]
        };
        for half in up_first {
            let face = he_face(body, half)?;
            match faces.iter_mut().find(|(f, _)| *f == face) {
                Some((_, halves)) => halves.push(half),
                None => faces.push((face, vec![half])),
            }
        }
    }
    let mut partner = SecondaryMap::new();
    for (face, halves) in faces {
        if halves.len() <= 2 {
            continue;
        }
        let face_data = body.get_face(face).ok_or_else(|| corrupt_face(face))?;
        let wall = body.face_surface_linked(face, face_data);
        let mut crossings = Vec::with_capacity(halves.len());
        for &h in &halves {
            let start = body.get_half_edge(h).ok_or_else(|| corrupt_he(h))?.start;
            let germ = red.apex_germs.get(h).copied();
            crossings.push(Crossing {
                half: h,
                point: vertex_point(body, start),
                // The half's up/down sense, read as the sweep reads it
                // (`Sweep::is_down`).
                down: above_set.contains_key(start),
                // At a cone's apex the face has no outward normal, and the
                // section leaves along the half's germ.
                leave: match germ {
                    Some(germ) => germ,
                    None => leave_on(
                        body,
                        red.plane.normal,
                        above_set,
                        h,
                        (wall, face_data.sense),
                    )?,
                },
                germ,
            });
        }
        let pairs = match *wall {
            geom::Surface::Plane { normal, .. } => line_pairs(red, face, normal, &crossings, band)?,
            _ => {
                let at = body
                    .get_half_edge(halves[0])
                    .ok_or_else(|| corrupt_he(halves[0]))?
                    .start;
                match wall_section(body, band, red.plane.origin, red.plane.normal, face, at)? {
                    Some(WallSection {
                        wall,
                        case: SectionCase::Conic(conic),
                    }) => conic_pairs(face, &wall, &conic, &crossings, band)?,
                    Some(WallSection {
                        case: SectionCase::Straight(rulings),
                        ..
                    }) => ruling_pairs(face, &rulings, &crossings, band)?,
                    Some(WallSection {
                        case: SectionCase::Loops { .. },
                        ..
                    }) => {
                        return Err(SplitJoinError::SectionInvariant {
                            face,
                            what: "a split reached a torus face, which its reduce refuses",
                        });
                    }
                    Some(WallSection {
                        case: SectionCase::Tangent(_),
                        ..
                    })
                    | None => Vec::new(),
                }
            }
        };
        for (a, b) in pairs {
            partner.insert(a, b);
            partner.insert(b, a);
        }
    }
    Ok(partner)
}

/// The direction the section leaves a null-edge half's site in, into
/// the half's face: `n_plane × n_out` at a down half, where the face's
/// boundary runs down through the plane and the section enters the
/// face, and its reverse at an up half, where the section leaves it
/// ([`conic_pairs`]' walk). `n_out` is the face's outward normal at the
/// site, read off its carrier ([`geom_brep::implicit_outward_normal`]):
/// a crossing lies on the face's boundary, so on its carrier. Its length
/// is the sine between the plane and the carrier there.
///
/// One reading for both of its consumers, the pairing of a curved
/// face's crossings and the chord a join mints between two of them
/// ([`crate::chord_join::Leave`]), so the chord takes the arc the
/// pairing walked.
///
/// The face's surface is a link, mid-operation
/// ([`crate::live::OPERATORS_KEEP_LINKS`]), and its miss panics.
fn split_leave<T: Decide>(
    body: &Body<T>,
    plane_normal: geom_core::UnitVec3<T>,
    above_set: &SecondaryMap<VertexKey, ()>,
    half: HalfEdgeKey,
) -> Result<geom_core::Vec3<T>, SplitJoinError> {
    let face = he_face(body, half)?;
    let face_data = body.get_face(face).ok_or_else(|| corrupt_face(face))?;
    let wall = body.face_surface_linked(face, face_data);
    leave_on(body, plane_normal, above_set, half, (wall, face_data.sense))
}

/// [`split_leave`] past its face's lookup: `wall` and `sense` are the
/// face's.
fn leave_on<T: Decide>(
    body: &Body<T>,
    plane_normal: geom_core::UnitVec3<T>,
    above_set: &SecondaryMap<VertexKey, ()>,
    half: HalfEdgeKey,
    (wall, sense): (&geom::Surface<T>, bool),
) -> Result<geom_core::Vec3<T>, SplitJoinError> {
    let start = body
        .get_half_edge(half)
        .ok_or_else(|| corrupt_he(half))?
        .start;
    let out = geom_brep::implicit_outward_normal(wall, sense, vertex_point(body, start));
    // A wall with no normal at the site (a cone's apex, where the half
    // carries its germ instead) has no direction to leave it in.
    let n = out.vec();
    if n.x.is_poison() || n.y.is_poison() || n.z.is_poison() {
        return Err(SplitJoinError::SectionInvariant {
            face: he_face(body, half)?,
            what: "a null-edge half at a point where its wall has no normal carries no germ",
        });
    }
    let heading = plane_normal.get().cross(out.vec());
    Ok(if above_set.contains_key(start) {
        heading
    } else {
        -heading
    })
}

/// One null-edge half on a face, as the pairing reads it.
struct Crossing<T: Decide> {
    half: HalfEdgeKey,
    point: Point3<T>,
    /// Starts at an above end (`Sweep::is_down`).
    down: bool,
    /// The section's direction into the face there ([`split_leave`]).
    leave: geom_core::Vec3<T>,
    /// Where the half sits at its face's cone apex, the section ruling
    /// it leaves the apex along
    /// ([`SplitReduction::apex_germs`](super::SplitReduction)).
    germ: Option<geom_core::Vec3<T>>,
}

/// A planar face's crossings paired along its section line: keyed by
/// their along-line coordinate `(p − origin)·d̂`, `d = n_face × n_plane`,
/// ordered by [`super::order::sort_along_line`], each taking the first
/// unpaired one before it of opposite sense. A crossing left unpaired
/// on its line keeps the book's rule.
///
/// **A face whose line the band cannot certify keeps the book's rule,
/// silently** (**`split_join_face_line`**: `|d|` levered by the
/// crossings' spread is Zero). That is a face lying in the plane, or
/// within the band of it, whose crossings are the plane's contact with
/// it rather than a section line; its pairing is the sweep order's.
///
/// # Errors
///
/// [`SplitJoinError::Escalated`] naming the face, where its line or
/// the order of two of its crossings along it is undecided.
fn line_pairs<T: Decide>(
    red: &SplitReduction<T>,
    face: FaceKey,
    normal: geom_core::Vec3<T>,
    crossings: &[Crossing<T>],
    band: Band,
) -> Result<Vec<(HalfEdgeKey, HalfEdgeKey)>, SplitJoinError> {
    let mut spread = T::zero();
    for c in crossings {
        spread = spread.max((c.point - crossings[0].point).norm());
    }
    let d = normal.cross(red.plane.normal.get());
    match decide(
        "split_join_face_line",
        Margin::levered(d.norm(), spread),
        band,
    ) {
        Ok(Sign::Positive) => {}
        Ok(_) => return Ok(Vec::new()),
        Err(diag) => return Err(SplitJoinError::Escalated { face, diag }),
    }
    let along: Vec<&Crossing<T>> = crossings.iter().collect();
    Ok(pair_along_line(face, &along, red.plane.origin, d.normalize(), band)?.0)
}

/// Crossings on one line through `origin` along unit `d`, paired along
/// it: keyed by `(p − origin)·d`, ordered by
/// [`super::order::sort_along_line`], each taking the first unpaired
/// one before it of opposite sense. Returns the pairs and the number
/// of crossings left unpaired.
///
/// # Errors
///
/// [`SplitJoinError::Escalated`] naming the face, where the order of
/// two crossings along the line is undecided.
fn pair_along_line<T: Decide>(
    face: FaceKey,
    crossings: &[&Crossing<T>],
    origin: Point3<T>,
    d: geom_core::Vec3<T>,
    band: Band,
) -> Result<(Vec<(HalfEdgeKey, HalfEdgeKey)>, usize), SplitJoinError> {
    let keys: Vec<T> = crossings
        .iter()
        .map(|c| (c.point - origin).dot(d))
        .collect();
    let order = super::order::sort_along_line(&keys, band)
        .map_err(|diag| SplitJoinError::Escalated { face, diag })?;
    let mut pairs = Vec::new();
    let mut loose: Vec<&Crossing<T>> = Vec::new();
    for i in order {
        let c = crossings[i];
        match loose.iter().position(|e| e.down != c.down) {
            Some(j) => pairs.push((loose.remove(j).half, c.half)),
            None => loose.push(c),
        }
    }
    Ok((pairs, loose.len()))
}

/// A curved face's crossings paired where the plane meets its wall
/// along two rulings (a cylinder's parallel pair, a cone's pair through
/// the apex): each crossing goes to the ruling it lies on, by
/// **`split_join_ruling_side`** (its distance from the other ruling
/// less its distance from this one, m), and each ruling's crossings
/// are paired along it ([`pair_along_line`]). An arc of the section
/// inside the face runs along one ruling, so no pair spans the two; the
/// sweep's lexicographic order interleaves two rulings' crossings
/// wherever the rulings are not parallel to its `v` axis. A crossing
/// left unpaired on its ruling refuses rather than falling to the
/// book's rule, which could pair it across to the other ruling.
///
/// **At a cone's apex** both rulings pass through the crossing, so its
/// position names neither: it goes to the ruling its section leaves the
/// apex along, its germ (**`split_join_ruling_germ`**: the germ's sine
/// from the other ruling less its sine from this one, levered at the
/// face's farthest crossing from the apex). The germ was read off the
/// same section table at the reduction (`insert`).
///
/// **Unpinned**: no shipped fixture reaches either refusal. A side is
/// undecided where a crossing lies as near one ruling as the other; a
/// ruling with a crossing left over is a face whose sections along it
/// do not alternate in and out.
///
/// # Errors
///
/// [`SplitJoinError::Escalated`] naming the face, where a crossing
/// lies as near one ruling as the other, or where [`pair_along_line`]
/// escalates; [`SplitJoinError::SectionCrossings`] with
/// [`ConicCrossingsCase::NotAlternating`] where a ruling's crossings do
/// not pair off; [`SplitJoinError::SectionInvariant`] where the
/// section's carrier is not a line.
fn ruling_pairs<T: Decide>(
    face: FaceKey,
    rulings: &[geom::Curve3<T>; 2],
    crossings: &[Crossing<T>],
    band: Band,
) -> Result<Vec<(HalfEdgeKey, HalfEdgeKey)>, SplitJoinError> {
    let mut lines = Vec::with_capacity(2);
    for ruling in rulings {
        let geom::Curve3::Line { origin, dir } = *ruling else {
            return Err(SplitJoinError::SectionInvariant {
                face,
                what: "a ruling section carried a non-line",
            });
        };
        lines.push((origin, dir.normalize()));
    }
    let off = |p: Point3<T>, (o, d): (Point3<T>, geom_core::Vec3<T>)| (p - o).cross(d).norm();
    let mut on: [Vec<&Crossing<T>>; 2] = [Vec::new(), Vec::new()];
    for c in crossings {
        let reading = match c.germ {
            // At the apex both rulings pass through the crossing: it lies
            // on the ruling its section leaves along, its germ's sine from
            // each levered at the face's farthest crossing from it.
            Some(germ) => decide_nonzero(
                "split_join_ruling_germ",
                Margin::levered(
                    germ.cross(lines[1].1).norm() - germ.cross(lines[0].1).norm(),
                    crossings
                        .iter()
                        .map(|o| (o.point - c.point).norm())
                        .fold(T::zero(), T::max),
                ),
                band,
            ),
            None => decide_nonzero(
                "split_join_ruling_side",
                Margin::of(off(c.point, lines[1]) - off(c.point, lines[0])),
                band,
            ),
        };
        match reading.map_err(|diag| SplitJoinError::Escalated { face, diag })? {
            NonzeroSign::Positive => on[0].push(c),
            NonzeroSign::Negative => on[1].push(c),
        }
    }
    let mut pairs = Vec::new();
    for (side, &(origin, d)) in on.iter().zip(&lines) {
        let (along, loose) = pair_along_line(face, side, origin, d, band)?;
        if loose > 0 {
            return Err(SplitJoinError::SectionCrossings {
                face,
                case: ConicCrossingsCase::NotAlternating,
                band,
            });
        }
        pairs.extend(along);
    }
    Ok(pairs)
}

/// A curved face's crossings paired along its section conic.
///
/// Walk the conic in the direction `h = n_plane × n_out` (the face's
/// outward normal at the crossing). The face lies to the left of its
/// boundary's direction `b` about `n_out`, so the walk enters the face
/// where `h·(n_out × b) > 0`, and that is `−(n_plane·b)`: it enters
/// exactly where the boundary runs down through the plane — at a down
/// half — and leaves at an up half. So, in walk order, each down
/// crossing is paired with the next crossing, the end of the arc that
/// lies in the face. The book's rule, pairing by the sweep's
/// lexicographic order, can instead pair across an arc OUTSIDE the
/// face: both ends of an exit-then-entry arc are of opposite sense too.
///
/// **The heading** is read at every crossing, as the sign of
/// `h·Ĉ′(θ)` against the conic's unit tangent in its eccentric anomaly
/// (**`split_join_conic_heading`**: a sine, levered by the wall's
/// radius — [`geom_brep::curvature_lever_arm`], the chart's own length
/// scale — so the margin is how far the section runs from tangent to
/// the wall there; on a sphere it is the section circle's radius). On
/// the conics that reach here (a tilted ellipse or a rim circle on a
/// cylinder, a polar circle on a sphere, an ellipse or an axis-normal
/// circle on a cone) `h` vanishes nowhere, so one
/// sign holds all round; opposite definite signs are a broken
/// invariant. The outward normal is the wall's gradient at the
/// crossing ([`geom_brep::implicit_outward_normal`]): a crossing lies
/// on the face's boundary, so on its wall.
///
/// **The order** is along the walk coordinate `w = ±θ`, by
/// [`super::order::sort_along`] with the gap between two crossings read
/// as `Δw·|C′(w_mid)|` — the arc between them at the conic's speed
/// halfway, which is the arc length to second order in `Δw`. A verdict
/// is close only where that gap is a few band widths, so `Δw` is a few
/// band widths over the minor semi-axis and the reading's error is far
/// inside the band; a larger `Δw` is decided apart either way, since the
/// speed is at least the minor semi-axis. Neither reading understates
/// the arc the way a fixed semi-axis would near the other vertex.
///
/// **The cycle.** `θ` is read in `(−π, π]`, so the sorted order starts
/// at the conic's branch cut. A run of coincident crossings straddling
/// it would be split across both ends: where the gap across the cut
/// reads Zero, the cut moves to the first gap that does not, those
/// crossings' `w` take a period, and the runs are regrouped. The pairing
/// reads the order cyclically, so where the cycle starts changes no
/// pair.
///
/// **The touches.** A run of crossings at one point is where the section
/// touches the face's boundary, and [`touch_order`] orders it by the
/// alternation round it. The hole-rim and bore-rim tangent splits
/// reach it from inside (`a_split_through_a_wrap_edges_vertex.rs`,
/// `split_across_a_revolve_seam.rs`).
///
/// **Unpinned**: no shipped fixture reaches [`ConicCrossingsCase::Grazing`]
/// or [`ConicCrossingsCase::NotAlternating`]; both are typed and read
/// here only. The same heading read Zero at a chord's start is the
/// chord's own refusal, [`SplitJoinError::SectionDepartureGrazes`],
/// which a needle cone cut near its apex reaches.
///
/// # Errors
///
/// [`SplitJoinError::Escalated`] naming the face, where a heading or a
/// gap is undecided; [`SplitJoinError::SectionCrossings`] where a
/// heading is in the band ([`ConicCrossingsCase::Grazing`]) or the
/// senses do not alternate in walk order
/// ([`ConicCrossingsCase::NotAlternating`]).
fn conic_pairs<T: Decide>(
    face: FaceKey,
    wall: &geom::Surface<T>,
    conic: &SectionConic<T>,
    crossings: &[Crossing<T>],
    band: Band,
) -> Result<Vec<(HalfEdgeKey, HalfEdgeKey)>, SplitJoinError> {
    let refuse = |case| SplitJoinError::SectionCrossings { face, case, band };
    let escalate = |diag| SplitJoinError::Escalated { face, diag };
    let mut heading = None;
    let mut walk = Vec::with_capacity(crossings.len());
    for c in crossings {
        let theta = conic.param(c.point);
        let tangent = conic.tangent(theta);
        let h = if c.down { c.leave } else { -c.leave };
        let sine = h.dot(tangent) / tangent.norm();
        let arm = geom_brep::curvature_lever_arm(wall, c.point);
        let sign = match decide("split_join_conic_heading", Margin::levered(sine, arm), band) {
            Ok(Sign::Zero) => return Err(refuse(ConicCrossingsCase::Grazing)),
            Ok(sign) => sign,
            Err(diag) => return Err(escalate(diag)),
        };
        if *heading.get_or_insert(sign) != sign {
            return Err(SplitJoinError::SectionInvariant {
                face,
                what: "crossings of one curved face read opposite headings along a section \
                       conic the face meets transversally all round",
            });
        }
        walk.push(theta);
    }
    if heading == Some(Sign::Negative) {
        for w in &mut walk {
            *w = T::zero() - *w;
        }
    }
    // How far crossing `a` lies past `b` along the walk (fn docs). The
    // conic's speed is even and 2π-periodic in `θ`, so it reads the
    // same at `w` as at the `θ` it came from.
    let gap = |w: &[T], a: usize, b: usize| {
        (w[a] - w[b]) * conic.tangent((w[a] + w[b]) * T::from_f64(0.5)).norm()
    };
    let n = crossings.len();
    let runs = super::order::sort_along_runs(n, |a, b| gap(&walk, a, b), band).map_err(escalate)?;
    let order = runs.concat();
    let (lo, hi) = (order[0], order[n - 1]);
    let mut across = walk.clone();
    across[lo] = across[lo] + T::tau();
    let runs = if decide(
        "split_join_line_gap",
        Margin::of(gap(&across, lo, hi)),
        band,
    )
    .map_err(escalate)?
        == Sign::Zero
    {
        let mut cut = None;
        for k in 1..n {
            let step = gap(&walk, order[k], order[k - 1]);
            if decide("split_join_line_gap", Margin::of(step), band).map_err(escalate)?
                != Sign::Zero
            {
                cut = Some(k);
                break;
            }
        }
        // Every crossing of the face at one point: nothing alternates.
        let Some(k) = cut else {
            return Err(refuse(ConicCrossingsCase::NotAlternating));
        };
        for &i in &order[..k] {
            walk[i] = walk[i] + T::tau();
        }
        let mut cycled = order;
        cycled.rotate_left(k);
        super::order::coincident_runs(cycled, |a, b| gap(&walk, a, b), band).map_err(escalate)?
    } else {
        runs
    };
    let order = touch_order(&runs, |i| crossings[i].down);
    let down = |k: usize| crossings[order[k % n]].down;
    if (0..n).any(|k| down(k) == down(k + 1)) {
        return Err(refuse(ConicCrossingsCase::NotAlternating));
    }
    let entry = usize::from(!down(0));
    Ok((0..n / 2)
        .map(|m| {
            let k = entry + 2 * m;
            (
                crossings[order[k % n]].half,
                crossings[order[(k + 1) % n]].half,
            )
        })
        .collect())
}

/// A cycle of crossings along a section, from its `runs` at one point
/// each, with each run of two or more put in the order its context
/// fixes. A run is where the section touches the face's boundary: from
/// inside, the arc arriving ends before the one leaving begins (exits
/// first); from outside, the section enters the face and leaves it
/// there, an arc of no length (entries first). Entries and exits
/// alternate round the section, so the sense of the crossing before a
/// run says which it is. The cycle is read from a run of one crossing;
/// where there is none, every crossing a touch, each run is read as a
/// touch from inside, the one of the two that leaves the face an arc of
/// the section. A run that cannot alternate keeps its order, for the
/// caller's alternation check to refuse.
fn touch_order(runs: &[Vec<usize>], down: impl Fn(usize) -> bool) -> Vec<usize> {
    let Some(lone) = runs.iter().position(|run| run.len() == 1) else {
        return runs
            .iter()
            .flat_map(|run| {
                let mut run = run.clone();
                run.sort_by_key(|&i| down(i));
                run
            })
            .collect();
    };
    let mut prev = runs[lone][0];
    let mut order = vec![prev];
    for k in 1..runs.len() {
        let mut run = runs[(lone + k) % runs.len()].clone();
        while !run.is_empty() {
            let j = run.iter().position(|&i| down(i) != down(prev)).unwrap_or(0);
            prev = run.remove(j);
            order.push(prev);
        }
    }
    order
}

/// The sweep state.
struct Sweep<T: Decide> {
    /// Loose ends, in registration order (growable — no `ends[30]`).
    ends: Vec<HalfEdgeKey>,
    /// Fixed partners of the halves on faces crossed more than twice
    /// ([`fixed_partners`]).
    partner: SecondaryMap<HalfEdgeKey, HalfEdgeKey>,
    /// The shared chord-join core.
    joiner: ChordJoiner,
    /// Completed polygons, in completion order.
    completed: Vec<CompletedSection>,
    /// The null edges' above ends (role membership).
    above_set: SecondaryMap<VertexKey, ()>,
    /// The split plane (section-area certification).
    plane: SplitPlane<T>,
    /// The run band.
    band: Band,
    /// The section-geometry context (M5 PR 5: the conic chord lane).
    section: SectionCtx<T>,
}

impl<T: Decide> Sweep<T> {
    /// Is `he` currently a loose end?
    fn is_loose(&self, he: HalfEdgeKey) -> bool {
        self.ends.contains(&he)
    }

    /// The up/down sense of a null-edge half — data from the vertex
    /// key (`start ∈ above_set` ⇒ down half).
    fn is_down<B: Decide>(&self, body: &Body<B>, he: HalfEdgeKey) -> Result<bool, SplitJoinError> {
        let start = body.get_half_edge(he).ok_or_else(|| corrupt_he(he))?.start;
        Ok(self.above_set.contains_key(start))
    }

    /// `canjoin`: consume and return `half`'s loose partner — its fixed
    /// one, or by the book's rule a loose end with no fixed partner in
    /// the same face with the opposite sense (module docs) — or register
    /// `half` as a new loose end and return `None`.
    fn take_neighbor(
        &mut self,
        body: &Body<T>,
        half: HalfEdgeKey,
    ) -> Result<Option<HalfEdgeKey>, SplitJoinError> {
        let face = he_face(body, half)?;
        if let Some(&mate) = self.partner.get(half) {
            match self.ends.iter().position(|&e| e == mate) {
                // The partner waits in this half's face: the line's pair.
                Some(i) if he_face(body, mate)? == face => {
                    self.ends.remove(i);
                    return Ok(Some(mate));
                }
                // An earlier chord divided the face between them (the
                // fixed pairing is of the faces the sweep started with):
                // both return to the book's rule below.
                Some(_) => {
                    self.partner.remove(half);
                    self.partner.remove(mate);
                }
                None => {
                    self.ends.push(half);
                    return Ok(None);
                }
            }
        }
        let down = self.is_down(body, half)?;
        for i in 0..self.ends.len() {
            let end = self.ends[i];
            if !self.partner.contains_key(end)
                && he_face(body, end)? == face
                && self.is_down(body, end)? != down
            {
                self.ends.remove(i);
                return Ok(Some(end));
            }
        }
        self.ends.push(half);
        Ok(None)
    }
}

impl<T: Decide> Sweep<T> {
    /// `cut` with the split lane's role resolution, area certification,
    /// and F9 record-keeping layered on the shared core.
    fn cut(&mut self, body: &mut Body<T>, edge: EdgeKey, tol: Tol) -> Result<(), SplitJoinError> {
        match self.joiner.cut_core(body, edge, tol)? {
            CutOutcome::Merged => Ok(()),
            CutOutcome::Completed { face, ring } => {
                let outer_loop = body.get_face(face).ok_or_else(|| corrupt_face(face))?.outer;
                let (above_loop, below_loop) = self.resolve_roles(body, face, outer_loop, ring)?;
                let kind = self.section_kind(body, face, above_loop, below_loop)?;
                if kind == SectionKind::Area {
                    self.certify_section_area(body, face, below_loop)?;
                    self.strip_spurs(body, face, above_loop, below_loop)?;
                }
                body.set_null_face_pair(
                    face,
                    NullFacePair::Split {
                        above_loop,
                        below_loop,
                    },
                )?;
                self.completed.push(CompletedSection {
                    face,
                    above_loop,
                    below_loop,
                    kind,
                });
                Ok(())
            }
        }
    }

    /// Resolve which of the null face's two loops is the above loop —
    /// by membership of the minted-copy vertex set, uniform across the
    /// loop (mixed ⇒ typed kernel-bug error).
    fn resolve_roles(
        &self,
        body: &Body<T>,
        face: FaceKey,
        outer: LoopKey,
        ring: LoopKey,
    ) -> Result<(LoopKey, LoopKey), SplitJoinError> {
        let classify = |l: LoopKey| -> Result<bool, SplitJoinError> {
            let starts = loop_starts(body, l)?;
            let above = starts
                .iter()
                .filter(|v| self.above_set.contains_key(**v))
                .count();
            if above == starts.len() {
                Ok(true)
            } else if above == 0 {
                Ok(false)
            } else {
                Err(SplitJoinError::SectionLoopMixed { face })
            }
        };
        match (classify(outer)?, classify(ring)?) {
            (true, false) => Ok((outer, ring)),
            (false, true) => Ok((ring, outer)),
            _ => Err(SplitJoinError::SectionLoopMixed { face }),
        }
    }

    /// Reads whether a completed polygon is a slit trace, from its
    /// topology alone: one of its loops runs along each of its edges both
    /// ways (each half's mate is in the loop), so it walks round a tree of
    /// edges and bounds nothing. That is the trace of edges of the operand
    /// in the plane, on the loop of the side that holds them, where the
    /// other side's material joins round every end of the tree (one null
    /// edge at each): for one edge, a loop of two struts
    /// ([`strut_turn`]). The other loop runs round the same tree in
    /// distinct edges: as many halves as the walk, and over each of the
    /// tree's edges exactly two of them, between the same two points (a
    /// vertex's copies share its point key). Every other polygon is an
    /// area polygon, certified next ([`Self::certify_section_area`]).
    ///
    /// A tree with an edge that is not a line refuses: its slit would be
    /// two coincident curved edges, which no contact record backs and
    /// tier 3′ refuses
    /// (`work/cleave/a-curved-slit-fails-tier-3-prime-on-its-two-coincident-arcs.md`).
    ///
    /// # Errors
    ///
    /// [`SplitJoinError::CurvedSlitTrace`];
    /// [`SplitJoinError::SectionInvariant`] where the other loop does not
    /// run twice over each of the tree's edges, which the two coincident
    /// loops of one trace cannot fail to; corruption on a torn loop.
    fn section_kind(
        &self,
        body: &Body<T>,
        face: FaceKey,
        above_loop: LoopKey,
        below_loop: LoopKey,
    ) -> Result<SectionKind, SplitJoinError> {
        let cycle = |l: LoopKey| -> Result<Vec<HalfEdgeKey>, SplitJoinError> {
            match body.get_loop(l).ok_or_else(|| corrupt_loop(l))?.boundary {
                LoopBoundary::Cycle { first } => {
                    body.loop_cycle(first).ok_or_else(|| corrupt_he(first))
                }
                LoopBoundary::Empty { .. } => Ok(Vec::new()),
            }
        };
        let walks_back = |hes: &[HalfEdgeKey]| -> Result<bool, SplitJoinError> {
            for &he in hes {
                let mate = body.mate(he).ok_or_else(|| corrupt_he(he))?;
                if !hes.contains(&mate) {
                    return Ok(false);
                }
            }
            Ok(!hes.is_empty())
        };
        let (above, below) = (cycle(above_loop)?, cycle(below_loop)?);
        let (strut, tree, slit) = match (walks_back(&above)?, walks_back(&below)?) {
            (true, false) => (above_loop, above, below),
            (false, true) => (below_loop, below, above),
            _ => return Ok(SectionKind::Area),
        };
        // Each edge's ends as point keys, in key order.
        let ends = |he: HalfEdgeKey| -> Result<(EdgeKey, [crate::PointKey; 2]), SplitJoinError> {
            let data = body.get_half_edge(he).ok_or_else(|| corrupt_he(he))?;
            let end = body.half_edge_end(he).ok_or_else(|| corrupt_he(he))?;
            let [Some(a), Some(b)] = [data.start, end].map(|v| body.get_vertex(v).map(|d| d.point))
            else {
                return Err(corrupt_he(he));
            };
            Ok((data.edge, [a.min(b), a.max(b)]))
        };
        let mut tree_edges: Vec<(EdgeKey, [crate::PointKey; 2])> = Vec::new();
        for &he in &tree {
            let (edge, pair) = ends(he)?;
            if tree_edges.iter().any(|&(e, _)| e == edge) {
                continue;
            }
            let data = body.get_edge(edge).ok_or_else(|| corrupt_edge(edge))?;
            if !matches!(
                body.edge_curve_linked(edge, data),
                CurveGeom::Certified(c) if matches!(c.carrier(), geom::Curve3::Line { .. })
            ) {
                return Err(SplitJoinError::CurvedSlitTrace { face });
            }
            tree_edges.push((edge, pair));
        }
        let mut slit_pairs = Vec::with_capacity(slit.len());
        for &he in &slit {
            slit_pairs.push(ends(he)?.1);
        }
        let twice = tree_edges
            .iter()
            .all(|(_, pair)| slit_pairs.iter().filter(|p| *p == pair).count() == 2);
        if slit.len() != tree.len() || !twice {
            return Err(SplitJoinError::SectionInvariant {
                face,
                what: "a section loop walks round a tree of edges, and its coincident loop \
                       does not run twice over each of them",
            });
        }
        Ok(SectionKind::Slit { strut })
    }

    /// Certify the completed polygon's area definitely positive
    /// (margin 2·|A|/P — mean width in meters, profile's
    /// `loop_orientation` lever-arm story); Zero ⇒ a degenerate
    /// section, refused typed.
    ///
    /// **Conic boundary edges (M5 PR 5)**: the vertex shoelace below is
    /// exact for straight chords and stays BIT-IDENTICAL for all-planar
    /// sections; each circle/ellipse section edge then adds its exact
    /// closed-form segment excess
    /// `[(c − O)×(B − A)]·n̂ + s_a·s_b·(axis·n̂)·Δt − [(A − O)×(B − O)]·n̂`
    /// (the eccentric-anomaly parameterization's constant areal rate,
    /// the fact `geom_brep::props::arc_segment_term` reads), and its
    /// perimeter contribution is raised from the chord to the
    /// conservative arc-length bound `s_a·|Δt|` (a larger perimeter
    /// only shrinks the mean-width margin — refuses more, never less).
    ///
    /// The join's body is mid-operation, and an edge's curve is a link
    /// there ([`crate::live::OPERATORS_KEEP_LINKS`]): a torn one panics,
    /// where null scaffolding is skipped.
    fn certify_section_area(
        &self,
        body: &Body<T>,
        face: FaceKey,
        below_loop: LoopKey,
    ) -> Result<(), SplitJoinError> {
        let points = loop_points_of(body, below_loop)?;
        let origin = points[0];
        let mut twice_area = T::zero();
        let mut perimeter = T::zero();
        for i in 0..points.len() {
            let a = points[i] - origin;
            let b = points[(i + 1) % points.len()] - origin;
            twice_area = twice_area + a.cross(b).dot(self.plane.normal.get());
            perimeter = perimeter + (b - a).norm();
        }
        // The conic excess pass (adds nothing for all-planar loops).
        let LoopBoundary::Cycle { first } = body
            .get_loop(below_loop)
            .ok_or_else(|| corrupt_loop(below_loop))?
            .boundary
        else {
            // The key RESOLVED; the shape is wrong. A completed section
            // polygon's below loop is a cycle by construction, so an
            // `Empty` one is this lane's own invariant, not a dangling
            // reference — and `Corrupt` is corruption only.
            return Err(SplitJoinError::SectionInvariant {
                face,
                what: "a completed section polygon's below loop holds a lone vertex \
                       instead of a cycle",
            });
        };
        let hes = body.loop_cycle(first).ok_or_else(|| corrupt_he(first))?;
        let mut placeholders = 0;
        for &he in &hes {
            let he_data = body.get_half_edge(he).ok_or_else(|| corrupt_he(he))?;
            let edge = body
                .get_edge(he_data.edge)
                .ok_or_else(|| corrupt_edge(he_data.edge))?;
            let CurveGeom::Certified(curve) = body.edge_curve_linked(he_data.edge, edge) else {
                continue;
            };
            let end = body.half_edge_end(he).ok_or_else(|| corrupt_he(he))?;
            // A placeholder bounds nothing, whatever plane its circle
            // lies in; a loop of placeholders alone is a point.
            if lone_site_placeholder(he_data.start, end, curve) {
                placeholders += 1;
                continue;
            }
            let Some(crate::loop_winding::ConicFrame {
                center: c_e,
                axis: axis_e,
                sa,
                sb,
                reach,
                ..
            }) = crate::loop_winding::ConicFrame::of(curve.carrier())
            else {
                continue;
            };
            let (t0, t1) = curve.params();
            let span = t1 - t0;
            let forward = edge.he_plus == he;
            let a_pt = vertex_point(body, he_data.start);
            let b_pt = vertex_point(body, end);
            let dt_signed = if forward { span } else { T::zero() - span };
            let a = a_pt - origin;
            let b = b_pt - origin;
            let excess = (c_e - origin)
                .cross(b_pt - a_pt)
                .dot(self.plane.normal.get())
                + sa * sb * axis_e.dot(self.plane.normal.get()) * dt_signed
                - a.cross(b).dot(self.plane.normal.get());
            twice_area = twice_area + excess;
            // The arc's length over-stated by its larger semi-axis
            // magnitude (`reach`), whichever is stored first.
            perimeter = perimeter + (reach * span.abs() - (b - a).norm());
        }
        // `twice_area` IS 2A (shoelace), so dividing by the full
        // perimeter yields the documented margin 2·|A|/P — the mean
        // width in meters — the dimensional argument stated once at
        // `Margin::over_lever`'s door, and the factor itself held as
        // a contract by this module's head docs and by
        // `docs/predicate-dimension-audit.md`'s row for this
        // predicate. `chart_region_area` asks the same question two
        // dimensions down through the same door; the accumulators
        // stay separate, for the reasons written at that site.
        if placeholders == hes.len() {
            return Err(SplitJoinError::DegenerateSection { face });
        }
        let margin = Margin::over_lever(twice_area.abs(), perimeter);
        match decide("split_section_area", margin, self.band) {
            Ok(Sign::Positive) => Ok(()),
            Ok(_) => Err(SplitJoinError::DegenerateSection { face }),
            Err(diag) => Err(SplitJoinError::Escalated { face, diag }),
        }
    }
}

impl<T: Decide> Sweep<T> {
    /// Reads a completed polygon's **spurs** ([`spur_tips`]) and kills
    /// its struts. A spur is a vertex at which a loop runs out along an
    /// edge and back over the same curve. Every spur of a
    /// polygon of positive area is the trace of one edge of the operand
    /// lying in the plane, and comes as a pair, one on each loop:
    ///
    /// - on the loop of the side the edge belongs to, a **strut**: the
    ///   edge itself, one half followed by the other ([`strut_turn`]), out
    ///   to a vertex no other edge reaches. That side's material lies
    ///   under the edge, so its section runs on across it: the strut is
    ///   killed, tip and all, and every reader of the section (the
    ///   finish, [`super::plane_section`]) reads the loop without it.
    /// - on the other side's loop, a **slit**: two distinct edges over
    ///   it, from two copies of its far end to one vertex at its tip.
    ///   That side's material lies on both sides of the edge, and its
    ///   piece touches itself along it, through distinct entities. The
    ///   slit stays in that side's section face; the section query takes
    ///   it out of the region's outline, inside which it lies
    ///   (`section::drop_slits`).
    ///
    /// The edge is one rule (b) reads reflex: both its faces leave it
    /// into one side, which holds material on both sides of it, and the
    /// edge goes to the other. Where that side's runs either side of it
    /// lie apart at both its ends, it bounds two polygons and is no
    /// spur. Where they join round one end, in one copy of the vertex
    /// there, it is a spur of one polygon. Where they join round both
    /// ends, its trace is a polygon of its own, a slit trace
    /// ([`SectionKind::Slit`]), which never reaches this.
    ///
    /// A contact the plane only touches makes no spur, because none mints
    /// a null edge of its own: rule (b) sends an edge touched along a
    /// convex corner with its material; rule (a) refuses a wall that
    /// bends away from its material ([`crate::SplitReduceError::KnifeEdge`]);
    /// and a cusp, which rule (b)'s safety default could otherwise send
    /// across, has a zero-width sector the neighbourhood classifier
    /// refuses first ([`crate::SplitReduceError::SliverSector`] on
    /// `sector_straight`). So at the straight tips, where the pairing is
    /// read, the struts and the slits are as many: an unpaired one is
    /// that invariant broken, and panics. A tip the band cannot tell from
    /// a corner escalates.
    ///
    /// **Only straight spurs are paired**; every strut is killed, curved
    /// or straight, since [`strut_turn`] reads topology alone, and every
    /// slit, curved or straight, is read for the section query. Pairing
    /// the curved ones is `work/honejoin/split-section-spur-guard-skips-curved-spurs.md`.
    fn strip_spurs(
        &self,
        body: &mut Body<T>,
        face: FaceKey,
        above_loop: LoopKey,
        below_loop: LoopKey,
    ) -> Result<(), SplitJoinError> {
        let (mut struts, mut slits) = (Vec::new(), Vec::new());
        for l in [above_loop, below_loop] {
            for tip in spur_tips(body, face, l, self.band)? {
                match tip {
                    Spur::Strut { at, straight: true } => struts.push(at),
                    Spur::Slit {
                        at, straight: true, ..
                    } => slits.push(at),
                    Spur::Strut { .. } | Spur::Slit { .. } => {}
                }
            }
        }
        if struts.len() != slits.len() {
            unreachable!(
                "section polygon {face:?} has {} strut(s) at {struts:?} and {} slit(s) at \
                 {slits:?} among its straight tips: every spur is the trace of one edge of the \
                 operand in the plane, the edge itself on the loop of the side that holds it \
                 and two chords on the other's, and a contact the plane only touches mints no \
                 null edge to trace one without the other",
                struts.len(),
                slits.len()
            );
        }
        for l in [above_loop, below_loop] {
            kill_struts(body, l)?;
        }
        Ok(())
    }
}

/// A spur of a section loop ([`Sweep::strip_spurs`]): a tip at which
/// the loop runs out along an edge and back over the same curve, at its
/// point. `straight` says both edges are lines.
#[derive(Clone, Copy, Debug)]
pub(super) enum Spur<T: Decide> {
    /// The loop runs out along one edge and back along the same edge.
    Strut { at: Point3<T>, straight: bool },
    /// The loop runs out along one edge, `out` its half leaving the tip,
    /// and back along another over the same line or conic arc, to a copy
    /// of the vertex it left.
    Slit {
        at: Point3<T>,
        out: HalfEdgeKey,
        straight: bool,
    },
}

/// What a half of a section loop runs along, as [`spur_tips`] compares
/// it.
enum Run<'a, T: Decide> {
    Line,
    Conic(&'a geom_brep::EdgeCurve<T>, bool),
    Unread,
}

/// The spurs of the loop `l` of the section polygon `face`, in cycle
/// order:
///
/// - a **strut** wherever a half is followed by its own mate
///   ([`strut_turn`]);
/// - a **slit** wherever two distinct edges run out from a vertex and
///   back to a vertex at its point (`split_section_spur`, the distance
///   between the two through [`Margin::norm3`]) over one curve: both
///   lines, or both conic arcs that agree at a quarter, a half and three
///   quarters of the way along. Five points of a conic fix it, and the
///   middle one fixes the arc of it between the ends, so two conic arcs
///   that agree there are one; a line and an arc between one pair of
///   points bound area, and so do two arcs that disagree.
///
/// A section edge is a line or a conic (the carrier gate refuses
/// spirics and splines in the plane's reach), so no tip is left unread.
///
/// # Errors
///
/// [`SplitJoinError::Escalated`] where the band cannot tell a tip from
/// a corner; corruption on a torn loop.
pub(super) fn spur_tips<T: Decide>(
    body: &Body<T>,
    face: FaceKey,
    l: LoopKey,
    band: Band,
) -> Result<Vec<Spur<T>>, SplitJoinError> {
    let LoopBoundary::Cycle { first } = body.get_loop(l).ok_or_else(|| corrupt_loop(l))?.boundary
    else {
        return Ok(Vec::new());
    };
    let hes = body.loop_cycle(first).ok_or_else(|| corrupt_he(first))?;
    let n = hes.len();
    let mut members = Vec::with_capacity(n);
    for &he in &hes {
        let he_data = body.get_half_edge(he).ok_or_else(|| corrupt_he(he))?;
        let edge = body
            .get_edge(he_data.edge)
            .ok_or_else(|| corrupt_edge(he_data.edge))?;
        let run = match body.edge_curve_linked(he_data.edge, edge) {
            CurveGeom::Certified(c) if matches!(c.carrier(), geom::Curve3::Line { .. }) => {
                Run::Line
            }
            CurveGeom::Certified(c)
                if crate::loop_winding::ConicFrame::of(c.carrier()).is_some() =>
            {
                Run::Conic(c, edge.he_plus == he)
            }
            _ => Run::Unread,
        };
        members.push((vertex_point(body, he_data.start), run));
    }
    let zero = |d: geom_core::Vec3<T>| -> Result<bool, SplitJoinError> {
        match decide("split_section_spur", Margin::norm3(d), band) {
            Ok(sign) => Ok(sign == Sign::Zero),
            Err(diag) => Err(SplitJoinError::Escalated { face, diag }),
        }
    };
    // The point `f` of the way along a conic half, in its own direction.
    let along = |c: &geom_brep::EdgeCurve<T>, forward: bool, f: T| {
        let (t0, t1) = c.params();
        let (a, b) = if forward { (t0, t1) } else { (t1, t0) };
        c.carrier().eval(a + (b - a) * f)
    };
    let mut tips = Vec::new();
    for index in 0..n {
        let inbound = (index + n - 1) % n;
        let ((before, into), (at, out)) = (&members[inbound], &members[index]);
        let (before, at) = (*before, *at);
        if strut_turn(body, hes[inbound])? {
            let straight = matches!(into, Run::Line);
            tips.push(Spur::Strut { at, straight });
            continue;
        }
        let same_curve = match (into, out) {
            (Run::Line, Run::Line) => Some(true),
            (Run::Conic(..), Run::Conic(..)) => Some(false),
            _ => None,
        };
        let Some(straight) = same_curve else {
            continue;
        };
        if !zero(members[(index + 1) % n].0 - before)? {
            continue;
        }
        if let (Run::Conic(a, fa), Run::Conic(b, fb)) = (into, out) {
            let mut agree = true;
            for f in [T::from_f64(0.25), T::from_f64(0.5), T::from_f64(0.75)] {
                // `into` from `before` to the tip; `out` from the tip
                // back, so its point `1 − f` of the way along.
                if !zero(along(a, *fa, f) - along(b, *fb, T::one() - f))? {
                    agree = false;
                    break;
                }
            }
            if !agree {
                continue;
            }
        }
        tips.push(Spur::Slit {
            at,
            out: hes[index],
            straight,
        });
    }
    Ok(tips)
}

/// Whether `he` turns back at its end: the next half of its loop is its
/// own mate and the edge's two ends are two vertices, so its edge is a
/// **strut** of that loop, out to a vertex no other edge reaches and
/// back — the validator's valence-1 strut. An edge closed on one vertex
/// is never one, whatever follows it.
fn strut_turn<T: Decide>(body: &Body<T>, he: HalfEdgeKey) -> Result<bool, SplitJoinError> {
    let he_data = body.get_half_edge(he).ok_or_else(|| corrupt_he(he))?;
    let next = body
        .get_half_edge(he_data.next)
        .ok_or_else(|| corrupt_he(he_data.next))?;
    Ok(next.edge == he_data.edge && next.start != he_data.start)
}

/// Kills every strut of the loop `l` ([`strut_turn`]) with the vertex at
/// its tip ([`Body::kev`](crate::construction::Construct::kev)), repeating until none is left, so a chain of
/// struts goes from its tip back.
fn kill_struts<T: Decide>(body: &mut Body<T>, l: LoopKey) -> Result<(), SplitJoinError> {
    loop {
        let LoopBoundary::Cycle { first } =
            body.get_loop(l).ok_or_else(|| corrupt_loop(l))?.boundary
        else {
            return Ok(());
        };
        let mut strut = None;
        for he in body.loop_cycle(first).ok_or_else(|| corrupt_he(first))? {
            if strut_turn(body, he)? {
                strut = Some(he);
                break;
            }
        }
        let Some(he) = strut else {
            return Ok(());
        };
        body.kev(he)?;
    }
}

/// The start vertices of a cycle loop.
pub(crate) fn loop_starts<T: Decide>(
    body: &Body<T>,
    l: LoopKey,
) -> Result<Vec<VertexKey>, SplitJoinError> {
    let loop_data = body.get_loop(l).ok_or_else(|| corrupt_loop(l))?;
    let LoopBoundary::Cycle { first } = loop_data.boundary else {
        // Resolved, but shaped wrong: an `Empty` loop has no starts to
        // walk. That is a caller invariant, not a corrupt arena.
        return Err(SplitJoinError::SectionInvariant {
            face: loop_data.face,
            what: "a loop asked for its start vertices holds a lone vertex instead of \
                   a cycle",
        });
    };
    let mut out = Vec::new();
    for he in body.loop_cycle(first).ok_or_else(|| corrupt_he(first))? {
        out.push(body.get_half_edge(he).ok_or_else(|| corrupt_he(he))?.start);
    }
    Ok(out)
}

/// The start points of a cycle loop, in cycle order.
pub(super) fn loop_points_of<T: Decide>(
    body: &Body<T>,
    l: LoopKey,
) -> Result<Vec<Point3<T>>, SplitJoinError> {
    let starts = loop_starts(body, l)?;
    Ok(starts.into_iter().map(|v| vertex_point(body, v)).collect())
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;
    use crate::EntityId;
    use crate::entity::LoopKey;
    use crate::fixtures::pillow;
    use geom_core::Tol;

    /// The refusal [`Body::face_of_half_edge`]'s docs reserve to a
    /// caller's own walk: which key went stale, per hop.
    #[test]
    fn he_face_names_the_key_that_went_stale() {
        let mut t = pillow(Tol::witness());
        let he = t.hes_a[0];
        assert_eq!(he_face(&t.body, he).ok(), Some(t.face_a));
        assert!(matches!(
            he_face(&t.body, HalfEdgeKey::default()),
            Err(SplitJoinError::Corrupt {
                entity: EntityId::HalfEdge(_)
            })
        ));
        // A live half-edge with a dead loop back-pointer: the SECOND
        // hop fails, and the refusal is about the loop.
        t.body.get_half_edge_mut(he).unwrap().parent_loop = LoopKey::default();
        assert!(matches!(
            he_face(&t.body, he),
            Err(SplitJoinError::Corrupt {
                entity: EntityId::Loop(_)
            })
        ));
    }

    /// **A touch is ordered by the crossing before it.** Crossings `0..`
    /// with `down` (an entry) as given; each case's runs are listed
    /// with the touch's pair in insertion order against the answer.
    #[test]
    fn a_touch_alternates_with_the_crossing_before_it() {
        let down = [true, false, true, false];
        let d = |i: usize| down[i];
        // Inside: an entry, then the touch (exit 1 and entry 2 at one
        // point, inserted entry first), then the exit.
        assert_eq!(
            touch_order(&[vec![0], vec![2, 1], vec![3]], d),
            [0, 1, 2, 3]
        );
        // Outside: an exit, then the touch (entry 0 and exit 3), then
        // the entry; the touch takes its entry first.
        assert_eq!(
            touch_order(&[vec![1], vec![3, 0], vec![2]], d),
            [1, 0, 3, 2]
        );
        // The cycle starts at a lone crossing wherever it lies.
        assert_eq!(
            touch_order(&[vec![3, 0], vec![2], vec![1]], d),
            [2, 1, 0, 3]
        );
        // Every crossing a touch: each from inside, exits first.
        assert_eq!(touch_order(&[vec![0, 1], vec![2, 3]], d), [1, 0, 3, 2]);
        // A run of two entries cannot alternate: it keeps its insertion
        // order, for the caller to refuse.
        assert_eq!(touch_order(&[vec![1], vec![2, 0]], d), [1, 2, 0]);
    }
}

/// **The sweep's reads past a face or a loop it resolved panic on a
/// torn link** (the split's body is mid-operation, so the panic names
/// `OPERATORS_KEEP_LINKS`). Each row reads the sound answer first, then
/// tears one record, and the body is deep-unchanged after the panic.
#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod torn_hop_rows {
    use super::*;
    use crate::entity::{EntityId, GeomRef};
    use crate::live::OPERATORS_KEEP_LINKS;
    use crate::review_d18::{ROW_FOUR, assert_torn_op_panics};
    use geom_core::Vec3;

    fn band() -> Band {
        Band::linear(Tol::witness()).unwrap()
    }

    fn reduced(body: &Body<f64>, origin: Point3<f64>, normal: Vec3<f64>) -> SplitReduction<f64> {
        let tol = Tol::witness();
        let plane = crate::test_support_fixtures::split_plane(origin, normal, tol);
        let mut described = body.clone();
        crate::test_support_fixtures::describe_as_intersections(&mut described, tol);
        let operand = crate::test_support::finished("the operand", described, tol);
        crate::splitting::split_reduce(&operand, &plane, tol).unwrap()
    }

    fn above_set(red: &SplitReduction<f64>) -> SecondaryMap<VertexKey, ()> {
        red.null_edges
            .iter()
            .map(|r| (r.attr.above_end, ()))
            .collect()
    }

    /// Drops `face`'s surface, and names the link that dangles.
    fn drop_surface(body: &mut Body<f64>, face: FaceKey) -> String {
        let surface = body.get_face(face).unwrap().surface;
        body.surfaces.remove(surface);
        format!(
            "{}'s surface names {}",
            EntityId::Face(face),
            GeomRef::Surface(surface)
        )
    }

    /// `fixed_partners`: a U prism cut across both arms, whose top face
    /// holds four crossings. Its torn surface panics, where it read as a
    /// curved face (and, first, refused `Corrupt` in the crossing's
    /// leaving direction).
    #[test]
    fn the_fixed_partners_panic_on_a_torn_surface() {
        let u = [
            (0.0, 0.0),
            (3.0, 0.0),
            (3.0, 2.0),
            (2.0, 2.0),
            (2.0, 1.0),
            (1.0, 1.0),
            (1.0, 2.0),
            (0.0, 2.0),
        ];
        let prism = crate::test_support_fixtures::prism::<f64>(&u, 1.0, Tol::witness());
        let mut red = reduced(&prism.body, Point3::new(0.0, 1.5, 0.0), Vec3::unit_y());
        let above = above_set(&red);
        assert_eq!(
            fixed_partners(&red, &above, band()).unwrap().len(),
            8,
            "the top and bottom faces' four crossings each pair along their lines"
        );
        let named = drop_surface(&mut red.body, prism.top_face);
        let (plane, null_edges) = (red.plane, red.null_edges.clone());
        assert_torn_op_panics(
            "fixed_partners",
            &mut red.body,
            &[&named, ROW_FOUR, OPERATORS_KEEP_LINKS],
            |b| {
                let red = SplitReduction {
                    body: b.clone(),
                    plane,
                    sides: SecondaryMap::new(),
                    on_vertices: Vec::new(),
                    null_edges: null_edges.clone(),
                    apex_germs: SecondaryMap::new(),
                    coincidences: Vec::new(),
                    edge_pieces: crate::lineage::SplitEdgeDivisions::new(
                        crate::lineage::LineageRows::EdgeDivisions(None),
                    ),
                };
                fixed_partners(&red, &above, band()).map(|p| p.len())
            },
        );
    }

    /// `split_leave`: a crossing on a cube's side face, whose torn
    /// surface panics where it refused `Corrupt`.
    #[test]
    fn the_leaving_direction_panics_on_a_torn_surface() {
        let cube = crate::test_support_fixtures::geometric_cube::<f64>(Tol::witness()).body;
        let mut red = reduced(&cube, Point3::new(0.0, 0.0, 0.5), Vec3::unit_z());
        let above = above_set(&red);
        let half = red.body.get_edge(red.null_edges[0].edge).unwrap().he_plus;
        let normal = red.plane.normal;
        let leave = |b: &Body<f64>| split_leave(b, normal, &above, half).map(|d| d.norm());
        assert!(
            (leave(&red.body).unwrap() - 1.0).abs() < 1e-12,
            "a side face's leaving direction is the unit in-plane one"
        );
        let face = he_face(&red.body, half).unwrap();
        let named = drop_surface(&mut red.body, face);
        assert_torn_op_panics(
            "split_leave",
            &mut red.body,
            &[&named, ROW_FOUR, OPERATORS_KEEP_LINKS],
            |b| leave(b),
        );
    }

    /// `certify_section_area`: a torn curve on a completed section's
    /// below loop panics, where it was stepped over as null scaffolding
    /// and its edge read as no line to the spur check.
    #[test]
    fn the_section_area_panics_on_a_torn_curve() {
        let tol = Tol::witness();
        let cube = crate::test_support_fixtures::geometric_cube::<f64>(tol).body;
        let mut red = reduced(&cube, Point3::new(0.0, 0.0, 0.5), Vec3::unit_z());
        let (completed, _) = split_connect(&mut red, band(), tol).unwrap();
        let [section] = completed[..] else {
            panic!("a cube cut through its middle has one section");
        };
        let sweep = Sweep {
            ends: Vec::new(),
            partner: SecondaryMap::new(),
            joiner: ChordJoiner::new(band()),
            completed: Vec::new(),
            above_set: above_set(&red),
            plane: red.plane,
            band: band(),
            section: SectionCtx {
                origin: red.plane.origin,
                normal: red.plane.normal,
                plane_key: None,
                branch: None,
            },
        };
        let certify =
            |b: &Body<f64>| sweep.certify_section_area(b, section.face, section.below_loop);
        assert!(certify(&red.body).is_ok(), "the sound section certifies");
        let LoopBoundary::Cycle { first } = red.body.get_loop(section.below_loop).unwrap().boundary
        else {
            panic!("the below loop is a cycle");
        };
        let edge = red.body.get_half_edge(first).unwrap().edge;
        let curve = red.body.get_edge(edge).unwrap().curve;
        red.body.curves.remove(curve);
        let named = format!(
            "{}'s curve names {}",
            EntityId::Edge(edge),
            GeomRef::Curve(curve)
        );
        assert_torn_op_panics(
            "certify_section_area",
            &mut red.body,
            &[&named, ROW_FOUR, OPERATORS_KEEP_LINKS],
            |b| certify(b),
        );
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod placeholder_rows {
    use super::*;
    use geom_core::Vec3;

    /// **A loop of lone-site placeholders bounds nothing in any plane.**
    /// `mef`'s lone site certifies its edge as the unit circle about
    /// `p + x̂` in the plane `z = 0`, so a section plane of normal `±ẑ`
    /// would read it as `π` of area. The area check refuses the loop as
    /// the point it is.
    #[test]
    fn a_loop_of_placeholders_is_a_degenerate_section_in_every_plane() {
        let tol = Tol::witness();
        let band = Band::linear(tol).unwrap();
        let mut body = Body::<f64>::new();
        let seed = body.mvfs(Point3::new(0.0, 0.0, 0.0), true).unwrap();
        let lone = body
            .mef_chord(
                crate::euler::MefSite::Lone {
                    r#loop: seed.r#loop,
                },
                tol,
            )
            .unwrap();
        for normal in [Vec3::unit_z(), -Vec3::unit_z(), Vec3::unit_y()] {
            let plane =
                crate::test_support_fixtures::split_plane(Point3::new(0.0, 0.0, 0.0), normal, tol);
            let sweep = Sweep {
                ends: Vec::new(),
                partner: SecondaryMap::new(),
                joiner: ChordJoiner::new(band),
                completed: Vec::new(),
                above_set: SecondaryMap::new(),
                plane,
                band,
                section: SectionCtx {
                    origin: plane.origin,
                    normal: plane.normal,
                    plane_key: None,
                    branch: None,
                },
            };
            for (face, lp) in [(seed.face, seed.r#loop), (lone.face, lone.r#loop)] {
                let got = sweep.certify_section_area(&body, face, lp);
                assert!(
                    matches!(got, Err(SplitJoinError::DegenerateSection { face: f }) if f == face),
                    "normal {normal:?}, loop {lp:?}: {got:?}"
                );
            }
        }
    }
}
