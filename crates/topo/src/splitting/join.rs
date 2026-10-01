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
//! pairing, and the role resolution and area certification it layers
//! on the core's side-agnostic outcome.
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
//!   the exit that ends the arc inside it ([`conic_pairs`]). The
//!   partner of such a half is consumed only by it, and it consumes
//!   only its partner.
//! - **Elsewhere, the book's rule**: the first registered half in the
//!   *same face* (at the time of the scan) with the *opposite* up/down
//!   sense, among the halves with no fixed partner.
//!
//! # What this lane adds to the core's `cut`
//!
//! When [`ChordJoiner::cut_core`] reports the polygon COMPLETE, the
//! 2-loop null face comes back with its roles unresolved. This lane
//! resolves them by membership of the minted above-copy vertex set,
//! certifies the polygon's area definitely-positive
//! (**`split_section_area`**, margin 2·A/P) — a zero-area section
//! polygon is the one-sided tangency residue PR 2's adjudication
//! record promised to refuse here, typed
//! [`SplitJoinError::DegenerateSection`] — refuses a positive-area
//! polygon carrying a tangent contact as a zero-width spur
//! ([`SplitJoinError::SectionSpur`], `split_section_spur`;
//! `Sweep::refuse_section_spur`), and writes the F9 record.

use geom_core::{Band, Decide, Margin, Point3, Sign};
use slotmap::SecondaryMap;

use super::order;
use super::{SplitPlane, SplitReduction};
use crate::body::Body;
use crate::chord_join::{
    ChordJoiner, ConicCrossingsCase, CutOutcome, FragmentRows, JoinLane, SectionCase, SectionCtx,
    SplitJoinError, WallSection, corrupt_edge, corrupt_face, corrupt_he, corrupt_loop,
    vertex_point, wall_section,
};
use crate::entity::{EdgeKey, FaceKey, HalfEdgeKey, LoopBoundary, LoopKey, VertexKey};
use crate::null::{CurveGeom, NullFacePair};
use crate::validate::decide;
use geom_core::Tol;

/// One completed section polygon: the null face and its role loops
/// (mirrors the body's [`NullFacePair`] record; carried separately so
/// the finish step consumes explicit keys in completion order).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct CompletedSection {
    /// The null face (two coincident loops).
    pub(crate) face: FaceKey,
    /// The loop through the minted above copies.
    pub(crate) above_loop: LoopKey,
    /// The loop through the original below-side vertices.
    pub(crate) below_loop: LoopKey,
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
pub(super) fn split_connect<T: Decide>(
    red: &mut SplitReduction<T>,
    band: Band,
    tol: Tol,
) -> Result<(Vec<CompletedSection>, FragmentRows), SplitJoinError> {
    let exact = order::exact_band().map_err(SplitJoinError::Band)?;

    // The minted above-copy set (role resolution is key membership).
    let mut above_set: SecondaryMap<VertexKey, ()> = SecondaryMap::new();
    for r in &red.null_edges {
        above_set.insert(r.attr.above_end, ());
    }

    // Sort points: each null edge's coincident-copy position.
    let mut points = Vec::with_capacity(red.null_edges.len());
    for r in &red.null_edges {
        points.push(vertex_point(&red.body, r.attr.below_end)?);
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
            plane: red.plane,
            plane_key: None,
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
                let Sweep {
                    joiner, section, ..
                } = &mut st;
                joiner.join(&mut red.body, end, half, JoinLane::Split(section), tol)?;
                joined[slot] = true;
                // Retire the consumed end's edge if its other half is
                // no longer loose.
                let end_edge = he_edge(&red.body, end)?;
                let mate = red.body.mate(end).ok_or_else(|| corrupt_he(end))?;
                if !st.is_loose(mate) {
                    st.cut(&mut red.body, end_edge)?;
                }
            }
        }
        if joined[0] && joined[1] {
            st.cut(&mut red.body, record.edge)?;
        }
    }

    if !st.ends.is_empty() {
        return Err(SplitJoinError::UnpairedLooseEnds {
            count: st.ends.len(),
        });
    }
    let fragments = st.joiner.take_fragments();
    Ok((st.completed, fragments))
}

/// The edge of a half-edge.
fn he_edge<T: Decide>(body: &Body<T>, he: HalfEdgeKey) -> Result<EdgeKey, SplitJoinError> {
    Ok(body.get_half_edge(he).ok_or_else(|| corrupt_he(he))?.edge)
}

/// The face owning a half-edge's loop.
fn he_face<T: Decide>(body: &Body<T>, he: HalfEdgeKey) -> Result<FaceKey, SplitJoinError> {
    let l = body
        .get_half_edge(he)
        .ok_or_else(|| corrupt_he(he))?
        .parent_loop;
    Ok(body.get_loop(l).ok_or_else(|| corrupt_loop(l))?.face)
}

/// The fixed partners of the null-edge halves on faces with more than
/// two crossings (module docs), both ways round: a planar face's along
/// its section line ([`line_pairs`]), a curved face's along its section
/// conic ([`conic_pairs`]).
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
/// [`line_pairs`]' and [`conic_pairs`]', each naming the face.
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
        let mut crossings = Vec::with_capacity(halves.len());
        for &h in &halves {
            let start = body.get_half_edge(h).ok_or_else(|| corrupt_he(h))?.start;
            crossings.push(Crossing {
                half: h,
                point: vertex_point(body, start)?,
                // The half's up/down sense, read as the sweep reads it
                // (`Sweep::is_down`).
                down: above_set.contains_key(start),
            });
        }
        let surface = body
            .get_face(face)
            .ok_or_else(|| corrupt_face(face))?
            .surface;
        let pairs = match body.get_surface(surface) {
            Some(&geom::Surface::Plane { normal, .. }) => {
                line_pairs(red, face, normal, &crossings, band)?
            }
            _ => conic_pairs(red, face, &crossings, band)?,
        };
        for (a, b) in pairs {
            partner.insert(a, b);
            partner.insert(b, a);
        }
    }
    Ok(partner)
}

/// One null-edge half on a face, as the pairing reads it.
struct Crossing<T: Decide> {
    half: HalfEdgeKey,
    point: Point3<T>,
    /// Starts at an above copy (`Sweep::is_down`).
    down: bool,
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
    let d = normal.cross(red.plane.normal);
    match decide(
        "split_join_face_line",
        Margin::levered(d.norm(), spread),
        band,
    ) {
        Ok(Sign::Positive) => {}
        Ok(_) => return Ok(Vec::new()),
        Err(diag) => return Err(SplitJoinError::Escalated { face, diag }),
    }
    let d = d.normalize();
    let keys: Vec<T> = crossings
        .iter()
        .map(|c| (c.point - red.plane.origin).dot(d))
        .collect();
    let order = super::order::sort_along_line(&keys, band)
        .map_err(|diag| SplitJoinError::Escalated { face, diag })?;
    let mut pairs = Vec::new();
    let mut loose: Vec<&Crossing<T>> = Vec::new();
    for i in order {
        let c = &crossings[i];
        match loose.iter().position(|e| e.down != c.down) {
            Some(j) => pairs.push((loose.remove(j).half, c.half)),
            None => loose.push(c),
        }
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
/// cylinder, a polar circle on a sphere) `h` vanishes nowhere, so one
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
/// A planar face, a straight section and the kinds the gate refuses
/// pair nothing here: they keep the book's rule.
///
/// **Unpinned**: no shipped fixture reaches [`ConicCrossingsCase::Grazing`]
/// or [`ConicCrossingsCase::NotAlternating`]; both are typed and read
/// here only.
///
/// # Errors
///
/// [`SplitJoinError::Escalated`] naming the face, where a heading or a
/// gap is undecided; [`SplitJoinError::SectionCrossings`] where a
/// heading is in the band ([`ConicCrossingsCase::Grazing`]) or the
/// senses do not alternate in walk order
/// ([`ConicCrossingsCase::NotAlternating`]); the C5 table's refusals.
fn conic_pairs<T: Decide>(
    red: &SplitReduction<T>,
    face: FaceKey,
    crossings: &[Crossing<T>],
    band: Band,
) -> Result<Vec<(HalfEdgeKey, HalfEdgeKey)>, SplitJoinError> {
    let body = &red.body;
    let first = crossings[0].half;
    let at = body
        .get_half_edge(first)
        .ok_or_else(|| corrupt_he(first))?
        .start;
    let Some(WallSection {
        wall,
        case: SectionCase::Conic(conic),
    }) = wall_section(body, band, &red.plane, face, at)?
    else {
        return Ok(Vec::new());
    };
    let sense = body.get_face(face).ok_or_else(|| corrupt_face(face))?.sense;
    let refuse = |case| SplitJoinError::SectionCrossings { face, case, band };
    let escalate = |diag| SplitJoinError::Escalated { face, diag };
    let mut heading = None;
    let mut walk = Vec::with_capacity(crossings.len());
    for c in crossings {
        let theta = conic.param(c.point);
        let out = geom_brep::implicit_outward_normal(&wall, sense, c.point).vec();
        let tangent = conic.tangent(theta);
        let sine = red.plane.normal.cross(out).dot(tangent) / tangent.norm();
        let arm = geom_brep::curvature_lever_arm(&wall, c.point);
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
    let order = super::order::sort_along(n, |a, b| gap(&walk, a, b), band).map_err(escalate)?;
    let (lo, hi) = (order[0], order[n - 1]);
    let mut across = walk.clone();
    across[lo] = across[lo] + T::tau();
    let order = if decide(
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
        super::order::group_coincident(cycled, |a, b| gap(&walk, a, b), band).map_err(escalate)?
    } else {
        order
    };
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
    /// Minted above copies (role membership).
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
    fn cut(&mut self, body: &mut Body<T>, edge: EdgeKey) -> Result<(), SplitJoinError> {
        match self.joiner.cut_core(body, edge)? {
            CutOutcome::Merged => Ok(()),
            CutOutcome::Completed { face, ring } => {
                let outer_loop = body.get_face(face).ok_or_else(|| corrupt_face(face))?.outer;
                let (above_loop, below_loop) = self.resolve_roles(body, face, outer_loop, ring)?;
                self.certify_section_area(body, face, below_loop)?;
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

    /// Certify the completed polygon's area definitely positive
    /// (margin 2·|A|/P — mean width in meters, profile's
    /// `loop_orientation` lever-arm story); Zero ⇒ the degenerate
    /// one-sided-tangency section, refused typed.
    ///
    /// **Conic boundary edges (M5 PR 5)**: the vertex shoelace below is
    /// exact for straight chords and stays BIT-IDENTICAL for all-planar
    /// sections; each circle/ellipse section edge then adds its exact
    /// closed-form segment excess
    /// `[(c − O)×(B − A)]·n̂ + s_a·s_b·(axis·n̂)·Δt − [(A − O)×(B − O)]·n̂`
    /// (the eccentric-anomaly parameterization's constant areal rate —
    /// the same fact as `loop_vector_area`'s ellipse arm), and its
    /// perimeter contribution is raised from the chord to the
    /// conservative arc-length bound `s_a·|Δt|` (a larger perimeter
    /// only shrinks the mean-width margin — refuses more, never less).
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
            twice_area = twice_area + a.cross(b).dot(self.plane.normal);
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
        for he in body.loop_cycle(first).ok_or_else(|| corrupt_he(first))? {
            let he_data = body.get_half_edge(he).ok_or_else(|| corrupt_he(he))?;
            let edge = body
                .get_edge(he_data.edge)
                .ok_or_else(|| corrupt_edge(he_data.edge))?;
            let Some(CurveGeom::Certified(curve)) = body.get_curve_geom(edge.curve) else {
                continue;
            };
            let (c_e, axis_e, sa, sb) = match *curve.carrier() {
                geom::Curve3::Circle {
                    center,
                    axis,
                    radius,
                    ..
                } => (center, axis, radius, radius),
                geom::Curve3::Ellipse {
                    center,
                    axis,
                    major,
                    minor,
                    ..
                } => (center, axis, major, minor),
                geom::Curve3::Line { .. }
                | geom::Curve3::Spiric { .. }
                | geom::Curve3::Nurbs(_) => {
                    continue;
                }
            };
            let (t0, t1) = curve.params();
            let span = t1 - t0;
            let forward = edge.he_plus == he;
            let a_pt = vertex_point(body, he_data.start)?;
            let b_pt = vertex_point(body, body.half_edge_end(he).ok_or_else(|| corrupt_he(he))?)?;
            let dt_signed = if forward { span } else { T::zero() - span };
            let a = a_pt - origin;
            let b = b_pt - origin;
            let excess = (c_e - origin).cross(b_pt - a_pt).dot(self.plane.normal)
                + sa * sb * axis_e.dot(self.plane.normal) * dt_signed
                - a.cross(b).dot(self.plane.normal);
            twice_area = twice_area + excess;
            perimeter = perimeter + (sa * span.abs() - (b - a).norm());
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
        let margin = Margin::over_lever(twice_area.abs(), perimeter);
        match decide("split_section_area", margin, self.band) {
            // A positive NET area can still carry a zero-area spur.
            Ok(Sign::Positive) => self.refuse_section_spur(body, face, first),
            Ok(_) => Err(SplitJoinError::DegenerateSection { face }),
            Err(diag) => Err(SplitJoinError::Escalated { face, diag }),
        }
    }
}

impl<T: Decide> Sweep<T> {
    /// Refuse a completed polygon that carries a **spur**: a vertex at
    /// which the loop runs out along a straight edge and straight back,
    /// so the vertex before it and the vertex after it coincide
    /// ([`SplitJoinError::SectionSpur`]).
    ///
    /// Where it comes from: a plane tangent to the solid along an edge
    /// mints null edges along that edge. When the tangent side is the
    /// run's above side the contact's null edges close a polygon of
    /// their own, zero-area, and [`Self::certify_section_area`] refuses
    /// it ([`SplitJoinError::DegenerateSection`]). [`super::split`]
    /// reads that refusal as a below-side pinch and reruns under the
    /// mirrored plane; if the plane also cuts the solid somewhere the
    /// contact reaches, the mirrored run joins the contact's null edges
    /// into that real section's loop as an out-and-back excursion. Its
    /// net area is the real section's, positive, and the area test
    /// passes it — the pinch lane would turn the one-sided-tangency
    /// refusal into a success whose halves carry a zero-width slit.
    /// This refuses that excursion, so the tangency stays refused (the
    /// public `split` then surfaces the direct run's
    /// `DegenerateSection`).
    ///
    /// The margin is the distance between the tip's two neighbours
    /// (`split_section_spur`, a length through [`Margin::norm3`]).
    /// **Only straight tips are decided.** A curved out-and-back — the
    /// loop running out along an arc and back along the same arc — is
    /// a spur too (its two excesses cancel; it bounds nothing), but
    /// telling it from two DIFFERENT arcs between one pair of points,
    /// which do bound area, needs a carrier comparison this check does
    /// not make. That gap is filed as
    /// `work/hone/split-section-spur-guard-skips-curved-spurs.md`.
    ///
    /// `first` is a half-edge of the below loop's cycle, as
    /// [`Self::certify_section_area`] resolved it.
    fn refuse_section_spur(
        &self,
        body: &Body<T>,
        face: FaceKey,
        first: HalfEdgeKey,
    ) -> Result<(), SplitJoinError> {
        let hes: Vec<HalfEdgeKey> = body.loop_cycle(first).ok_or_else(|| corrupt_he(first))?;
        let n = hes.len();
        if n < 3 {
            return Ok(());
        }
        let straight = |he: HalfEdgeKey| -> Result<bool, SplitJoinError> {
            let e = he_edge(body, he)?;
            let edge = body.get_edge(e).ok_or_else(|| corrupt_edge(e))?;
            Ok(matches!(
                body.get_curve_geom(edge.curve),
                Some(CurveGeom::Certified(c)) if matches!(c.carrier(), geom::Curve3::Line { .. })
            ))
        };
        let start = |he: HalfEdgeKey| -> Result<Point3<T>, SplitJoinError> {
            vertex_point(
                body,
                body.get_half_edge(he).ok_or_else(|| corrupt_he(he))?.start,
            )
        };
        for i in 0..n {
            let (inbound, outbound) = (hes[(i + n - 1) % n], hes[i]);
            if !(straight(inbound)? && straight(outbound)?) {
                continue;
            }
            let (before, after) = (start(inbound)?, start(hes[(i + 1) % n])?);
            match decide(
                "split_section_spur",
                Margin::norm3(after - before),
                self.band,
            ) {
                Ok(Sign::Zero) => return Err(SplitJoinError::SectionSpur { face }),
                Ok(_) => {}
                Err(diag) => return Err(SplitJoinError::Escalated { face, diag }),
            }
        }
        Ok(())
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
    starts.into_iter().map(|v| vertex_point(body, v)).collect()
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
}
