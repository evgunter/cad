//! Null-edge insertion at an ON vertex: one null edge per **corner of
//! the section** there, hung as the tree those corners make.
//!
//! # The section's corners at a vertex
//!
//! A **cut** is a place in the vertex's sector array where the side
//! changes: cut `c` lies between `entries[c]` and `entries[c + 1]`, inside
//! the sector between them, and that sector's face leaves the plane along
//! the cut's **germ**, a direction in the plane. Round the vertex in the
//! plane the solid's material lies between the germs in pairs: each pair
//! is a corner of the section there, and its null edge carries the
//! section's boundary across the vertex from the one germ's face to the
//! other's. The corners pair the cuts as chords of the orbit that do not
//! cross, so they cut the orbit into regions, one per piece of the
//! material round the vertex on one side of the plane, and each region's
//! edges belong at one vertex: the regions are the nodes of a tree whose
//! edges are the corners ([`crate::null::ring_tree`], the boolean pierce
//! ring's reading).
//!
//! Where the runs lie apart, every corner pairs one Above run's two cuts,
//! and the tree is a star: the ON vertex keeps the Below side, and each
//! run's copy takes the run. Where Above runs nest (or Below runs do), a
//! corner pairs cuts of two different runs, and a star would close each
//! section loop across the corners of others.
//!
//! # Reading the corners
//!
//! With one run there is one corner, its two cuts. With more, the
//! corners are read from the order of the germs about the plane's
//! normal ([`section_chords`]), each germ the material starts at paired
//! with the next. The topology decides what it can: a germ beside an
//! entry the plane holds lies along that entry, and the two germs either
//! side of one such entry are ordered with no reading. Only germs of
//! different sectors are ordered by reading their directions, and two
//! that leave along one tangent by reading, one order down, how each
//! face's section curves off it in the plane.
//!
//! # Minting
//!
//! The tree is rooted at the region holding the first Below entry (the
//! **anchor**), which the ON vertex keeps. Each corner hangs, parent
//! first, at the vertex holding its parent region: its copy takes the
//! corner's arc of the orbit, the side away from the anchor, so the copy
//! stands for the child region and is that side's end of the null edge.
//! The fan site is resolved **at execution time**: `he1` = the arc's
//! first real half-edge, `he2` = the orbit successor of its last
//! ([`Body::run_site`]) in the current body, which counts the splices of
//! the corners minted before it. An arc with no real half-edge (only
//! subdivision duplicates of one sector) becomes the **dangling** null
//! edge: the strut site `Fan { he, he }` at the sector's next real
//! half-edge. A sector holds at most one such arc: the plane's side
//! changes sign at most twice inside it. An
//! arc holding every half-edge of the vertex is the mirror case
//! ([`crate::euler::RunSite::WholeOrbit`]): a strut inside the one sector
//! the arc leaves, whose tip stands for the parent region, while the
//! vertex keeps the arc and stands for the child.
//!
//! Orientation is **data** (F9): the end standing for an Above region is
//! the above end ([`Body::mev_null_run`]). Each copy's side verdict is
//! cached ON (the old vertex's own point: structural coincidence).

use geom_core::k_stats::NonzeroSign;
use geom_core::{Band, Decide, Margin, Real, Sign, Vec3};
use slotmap::SecondaryMap;

use super::neighborhood::{EntryRay, sector_face};
use super::{
    ApexSectionReading, NullEdgeRecord, PlaneSide, SectionGermFault, SectionGermReading,
    SectorEntry, SectorEntryKind, SplitPlane, SplitReduceError,
};
use crate::body::Body;
use crate::chord_join::{SectionCase, SplitJoinError, WallSection};
use crate::entity::{EntityId, HalfEdgeKey, VertexKey};
use crate::euler::{MevSite, RunSite};
use crate::live::{Proven, proven};
use crate::null::NewVertexSide;
use crate::sector_face::VertexFace;
use crate::validate::{decide, decide_nonzero};

/// A maximal cyclic run of ABOVE entries, by entry index.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) struct AboveRun {
    /// Index of the run's first entry.
    pub start: usize,
    /// Number of entries in the run (≥ 1).
    pub len: usize,
}

/// Enumerates the maximal cyclic ABOVE runs of a fully reclassified
/// entry array, in deterministic order (first run after the anchor
/// BELOW entry with the lowest index). A one-sided neighborhood (no
/// ABOVE, or no BELOW — a vertex the plane merely touches) has no runs:
/// nothing to separate, no null edge (ch. 14 §14.6).
pub(super) fn above_runs(entries: &[SectorEntry]) -> Vec<AboveRun> {
    let n = entries.len();
    let Some(anchor) = entries.iter().position(|e| e.class == PlaneSide::Below) else {
        return Vec::new(); // no BELOW anywhere: one-sided
    };
    let mut runs = Vec::new();
    let mut k = 0;
    while k < n {
        let idx = (anchor + 1 + k) % n;
        if entries[idx].class == PlaneSide::Above {
            let start = idx;
            let mut len = 0;
            while k < n && entries[(anchor + 1 + k) % n].class == PlaneSide::Above {
                len += 1;
                k += 1;
            }
            runs.push(AboveRun { start, len });
        } else {
            k += 1;
        }
    }
    runs
}

/// **The section's corners at the vertex** (module docs): `cuts` (two or
/// more pairs) paired as the material lies between their germs round
/// the plane's `normal`.
///
/// Cut `c`'s germ lies in the plane inside the sector between
/// `rays[c]` and `rays[c + 1]`. Where one bound lies in the plane (its
/// side was On before rules (a) and (b)) the germ is that bound, read
/// off the topology. Where the sector is a cone face at its apex it is
/// `apex_germs[c]`, the section ruling inside the cut's sub-sector
/// ([`apex_germ`]): the fan of rulings there is not the positive cone of
/// its bounds. Otherwise it is `|ŵ·n| û + |û·n| ŵ`: a positive
/// combination of the bounds, so inside the sector (every tangent-plane
/// sector of the array is under 180°), and in the plane, the bounds
/// lying on opposite sides of it. The sector sweeps counterclockwise
/// about its face's outward normal from `ŵ` to `û` (the `neighborhood` module's
/// convention), so the material lies counterclockwise of the germ about
/// `n` exactly when `entries[c]` is Above: there it starts, and it ends
/// at the next germ.
///
/// The two germs either side of one in-plane entry share its direction
/// and are paired with no reading. Their order is the one the
/// alternation of starts and ends fixes from any other germ. Where every
/// germ is one of such a pair, each pair is taken end first: rule (b)'s
/// reflex row sends the entry across only where material lies on both
/// sides of it in the plane. Its safety-default row (a smooth edge or
/// duplicate, no graze) can reach this too, with no such guarantee.
/// Every other germ is placed by reading its sine about `n` from the
/// first germ (`split_germ_side`), its cosine where that is zero
/// (`split_germ_sense`), or its sine from another germ in the same
/// half-turn (`split_germ_order`), each levered at the shortest reach it
/// reads. Two germs of different sectors that a reading puts on one line
/// leave the vertex along one tangent, and are ordered one order down
/// (`curving`): by which way each face's section curves off that
/// tangent in the plane. Two that curve alike there too are a
/// coincidence to the tolerance.
///
/// # Errors
///
/// [`SplitReduceError::SectionGermsEscalated`] where a reading lands in
/// the band; [`SplitReduceError::SectionGermsUnordered`] where a germ has
/// no line, two germs of different sectors lie on one line and curve
/// alike or a face's curvature there does not read, or the order read
/// does not alternate starts and ends.
fn section_chords<T: Decide>(
    body: &Body<T>,
    normal: Vec3<T>,
    hood: &Neighborhood<'_, T>,
    cuts: &[usize],
    apex_germs: &[Option<Vec3<T>>],
    band: Band,
) -> Result<Vec<[usize; 2]>, SplitReduceError> {
    let Neighborhood {
        vertex,
        entries,
        rays,
    } = *hood;
    let n = entries.len();
    let escalated = |reading, diag| SplitReduceError::SectionGermsEscalated {
        vertex,
        reading,
        diag,
    };
    let unordered = |fault| SplitReduceError::SectionGermsUnordered { vertex, fault };
    // Each germ: its unit direction, its reach, and the in-plane entry
    // it lies along, if any.
    let germs = cuts
        .iter()
        .map(|&c| {
            let (a, b) = (c, (c + 1) % n);
            let (u, w) = (rays[a], rays[b]);
            let arm = u.dir.norm().min(w.dir.norm());
            match (u.in_plane, w.in_plane) {
                (true, false) => return Ok((u.dir.normalize(), arm, Some(a))),
                (false, true) => return Ok((w.dir.normalize(), arm, Some(b))),
                (true, true) => return Err(unordered(SectionGermFault::NoLine)),
                (false, false) => {}
            }
            if let Some(germ) = apex_germs[c] {
                return Ok((germ, arm, None));
            }
            let (u, w) = (u.dir.normalize(), w.dir.normalize());
            let d = u * w.dot(normal).abs() + w * u.dot(normal).abs();
            match decide("split_germ_line", Margin::levered(d.norm(), arm), band) {
                Ok(Sign::Positive) => Ok((d.normalize(), arm, None)),
                Ok(_) => Err(unordered(SectionGermFault::NoLine)),
                Err(diag) => Err(escalated(SectionGermReading::Line, diag)),
            }
        })
        .collect::<Result<Vec<_>, _>>()?;
    let order = |diag| escalated(SectionGermReading::Order, diag);
    let point = body.resolve_vertex_point(vertex, Proven);
    // How far cut `c`'s face's section curves off the tangent `t`
    // toward `across`, the plane's quarter turn of `t`: poison or
    // unbounded where the surface is singular at the vertex or the
    // plane is tangent to it there, which refuses. At a cone's apex the
    // section is a ruling, straight: it bends by nothing.
    let section_bend = |c: usize, t: Vec3<T>, across: Vec3<T>| -> Result<T, SplitReduceError> {
        let (face, at_vertex, _) = sector_face(body, vertex, entries[cuts[c]].he, band)?;
        if let VertexFace::Apex(_) = at_vertex {
            return Ok(T::zero());
        }
        let surface = body.face_surface_linked(face, proven(&body.faces, face, EntityId::Face));
        Ok(geom_brep::implicit_graph_acceleration(
            surface, point, t, across,
        ))
    };
    // Whether germ `b`, tied with germ `a` along `a`'s tangent, curves
    // counterclockwise of it about `normal` (Positive) or clockwise.
    let curving = |a: usize, b: usize, arm: T| -> Result<Sign, SplitReduceError> {
        let t = germs[a].0;
        let across = normal.cross(t);
        let bends = [section_bend(a, t, across)?, section_bend(b, t, across)?];
        match geom_brep::tied_germ_turn(normal, t, bends, arm, band) {
            Ok(Sign::Zero) => Err(unordered(SectionGermFault::Coincident)),
            Ok(sign) => Ok(sign),
            Err(geom_brep::TiedGermsError::Singular) => Err(unordered(SectionGermFault::Singular)),
            Err(geom_brep::TiedGermsError::Lever(escalation)) => Err(order(escalation.diag())),
        }
    };
    // Each germ's slot counterclockwise from germ 0: along it (0), the
    // open half-turn after it (1), opposite it (2), the half-turn back
    // to it (3).
    let (r, r_arm, _) = germs[0];
    let slot = |g: Vec3<T>, arm: T| -> Result<u8, SplitReduceError> {
        let arm = arm.min(r_arm);
        match decide(
            "split_germ_side",
            Margin::levered(r.cross(g).dot(normal), arm),
            band,
        )
        .map_err(order)?
        {
            Sign::Positive => Ok(1),
            Sign::Negative => Ok(3),
            Sign::Zero => {
                match decide_nonzero("split_germ_sense", Margin::levered(r.dot(g), arm), band)
                    .map_err(order)?
                {
                    NonzeroSign::Positive => Ok(0),
                    NonzeroSign::Negative => Ok(2),
                }
            }
        }
    };
    // The germs in counterclockwise order, the two along one in-plane
    // entry grouped with no reading.
    let mut groups: Vec<(u8, Vec<usize>)> = Vec::new();
    for (i, &(g, g_arm, along)) in germs.iter().enumerate() {
        if let Some(j) = groups
            .iter()
            .position(|(_, m)| along.is_some() && germs[m[0]].2 == along)
        {
            groups[j].1.push(i);
            continue;
        }
        let s = if i == 0 { 0 } else { slot(g, g_arm)? };
        let mut at = groups.len();
        for (j, (t, members)) in groups.iter().enumerate() {
            if *t < s {
                continue;
            }
            if *t > s {
                at = j;
                break;
            }
            let (h, h_arm, _) = germs[members[0]];
            let arm = g_arm.min(h_arm);
            let sine = Margin::levered(h.cross(g).dot(normal), arm);
            let sign = match decide("split_germ_order", sine, band).map_err(order)? {
                Sign::Zero => curving(members[0], i, arm)?,
                sign => sign,
            };
            if sign == Sign::Negative {
                at = j;
                break;
            }
        }
        groups.insert(at, (s, vec![i]));
    }
    // Each pair along one in-plane entry in the order the alternation
    // fixes from the first lone germ, or end first where there is none.
    let starts = |i: usize| entries[cuts[i]].class == PlaneSide::Above;
    let first = groups.iter().position(|(_, m)| m.len() == 1).unwrap_or(0);
    let mut start = groups[first].1.len() == 1 && starts(groups[first].1[0]);
    let mut seq = Vec::with_capacity(cuts.len());
    for k in 0..groups.len() {
        match groups[(first + k) % groups.len()].1[..] {
            [a] => seq.push(a),
            [a, b] => seq.extend(if starts(a) == start { [a, b] } else { [b, a] }),
            _ => unreachable!("only the two germs beside one entry share a group"),
        }
        start = !starts(
            *seq.last()
                .unwrap_or_else(|| unreachable!("a group has a germ")),
        );
    }
    let m = seq.len();
    if (0..m).any(|j| starts(seq[j]) == starts(seq[(j + 1) % m])) {
        return Err(unordered(SectionGermFault::Alternation));
    }
    Ok((0..m)
        .filter(|&j| starts(seq[j]))
        .map(|j| [cuts[seq[j]], cuts[seq[(j + 1) % m]]])
        .collect())
}

/// **Cut `c`'s germ where its sector is a cone face at its apex**, or
/// `None` where the face has a tangent plane there. The sector's
/// interior is a fan of rulings, not the positive cone of its bounds, so
/// the germ is the ruling of the plane's section that lies inside the
/// cut's sub-sector, `rays[c]` back to `rays[c + 1]`: one of the
/// section's two rulings through the apex (or its one, where the plane
/// is tangent), read off the same section table the join pairs the
/// face's crossings along ([`crate::chord_join::wall_section`]), and
/// turned onto the face's nappe. The walk subdivided the sector where
/// the plane's side along its rulings turns, so the sub-sector's bounds
/// lie on opposite sides and exactly one section ruling lies inside it;
/// which one is read as its turn from each bound into the sweep
/// (`split_germ_apex`, [`ConeApex::turn`](crate::sector_face::ConeApex::turn) levered at the sub-sector's
/// arm).
///
/// # Errors
///
/// [`SplitReduceError::SectionGermsEscalated`] where the table or the
/// reading escalates; [`SplitReduceError::ApexSection`] where the table
/// and the walk disagree on the apex ([`ApexSectionReading`]): the table
/// refuses, or reads the section clear of the apex, or not exactly one of
/// its rulings lies inside the sub-sector the walk read the plane
/// crossing.
fn apex_germ<T: Decide>(
    body: &Body<T>,
    plane: &SplitPlane<T>,
    hood: &Neighborhood<'_, T>,
    c: usize,
    band: Band,
) -> Result<Option<Vec3<T>>, SplitReduceError> {
    let Neighborhood {
        vertex,
        entries,
        rays,
    } = *hood;
    let (u, w) = (rays[c], rays[(c + 1) % entries.len()]);
    let Some(apex) = u.apex else {
        return Ok(None);
    };
    let escalated = |diag| SplitReduceError::SectionGermsEscalated {
        vertex,
        reading: SectionGermReading::Line,
        diag,
    };
    let face = crate::sector_face::face_of(body, vertex, entries[c].he);
    let disagrees = |reading| SplitReduceError::ApexSection {
        vertex,
        face,
        reading,
    };
    let lines =
        match crate::chord_join::wall_section(body, band, plane.origin, plane.normal, face, vertex)
        {
            Ok(Some(WallSection {
                case: SectionCase::Straight([l1, l2]),
                ..
            })) => vec![l1, l2],
            Ok(Some(WallSection {
                case: SectionCase::Tangent(l),
                ..
            })) => vec![l],
            Err(SplitJoinError::Escalated { diag, .. }) => return Err(escalated(diag)),
            Err(section) => {
                return Err(disagrees(ApexSectionReading::Refused(Box::new(section))));
            }
            Ok(Some(WallSection {
                case: SectionCase::Conic(_),
                ..
            })) => return Err(disagrees(ApexSectionReading::ClosedOffApex)),
            Ok(_) => unreachable!(
                "{face:?} carries a cone, whose section table reads a conic, a line pair or a \
                 tangent line"
            ),
        };
    let arm = u.dir.norm().min(w.dir.norm());
    let mut inside = None;
    for line in lines {
        let geom::Curve3::Line { dir, .. } = line else {
            unreachable!("{face:?}: a cone's section ruling through its apex is a line")
        };
        let z = apex.onto_nappe_of(dir, u.dir);
        let into = apex.turn(w.dir, z).min(apex.turn(z, u.dir));
        match decide("split_germ_apex", Margin::levered(into, arm), band).map_err(escalated)? {
            Sign::Positive if inside.is_none() => inside = Some(z),
            Sign::Positive => return Err(disagrees(ApexSectionReading::BothRulingsInside)),
            Sign::Zero | Sign::Negative => {}
        }
    }
    inside
        .map(Some)
        .ok_or_else(|| disagrees(ApexSectionReading::NoRulingInside))
}

/// One ON vertex's classified neighborhood: its entries, and each
/// entry's ray (`neighborhood::classify_neighborhood_rays`).
#[derive(Clone, Copy)]
pub(super) struct Neighborhood<'a, T: Real> {
    pub vertex: VertexKey,
    pub entries: &'a [SectorEntry],
    pub rays: &'a [EntryRay<T>],
}

/// Mints the vertex's null edges (module docs): one per corner of the
/// section, F9 attributes recorded, each copy cached ON. The rays are
/// read only where two or more runs need their corners ordered.
///
/// # Errors
///
/// [`section_chords`]'s refusals, [`SplitReduceError::SectionGermsUnordered`]
/// where the corners cross as chords of the orbit, and the mint's own.
pub(super) fn insert_null_edges<T: Decide>(
    body: &mut Body<T>,
    plane: &SplitPlane<T>,
    hood: &Neighborhood<'_, T>,
    sides: &mut SecondaryMap<VertexKey, PlaneSide>,
    records: &mut Vec<NullEdgeRecord>,
    germs: &mut SecondaryMap<HalfEdgeKey, Vec3<T>>,
    band: Band,
) -> Result<(), SplitReduceError> {
    let (vertex, entries) = (hood.vertex, hood.entries);
    let n = entries.len();
    let Some(anchor) = entries.iter().position(|e| e.class == PlaneSide::Below) else {
        return Ok(());
    };
    let above = |c: usize| entries[c % n].class == PlaneSide::Above;
    let cuts: Vec<usize> = (0..n).filter(|&c| above(c) != above(c + 1)).collect();
    if cuts.is_empty() {
        return Ok(());
    }
    // Each cut's germ where its sector is a cone face at its apex, read
    // once: the direction the section leaves the apex in there, which
    // orders the corners and puts each null-edge half on its section
    // ruling at the join.
    let mut apex_germs = vec![None; n];
    for &c in &cuts {
        let (u, w) = (hood.rays[c], hood.rays[(c + 1) % n]);
        if u.apex.is_none() {
            continue;
        }
        apex_germs[c] = match (u.in_plane, w.in_plane) {
            (true, false) => Some(u.dir.normalize()),
            (false, true) => Some(w.dir.normalize()),
            (true, true) => None,
            (false, false) => apex_germ(body, plane, hood, c, band)?,
        };
    }
    let corners = match cuts[..] {
        [a, b] => vec![[a, b]],
        _ => section_chords(body, plane.normal.get(), hood, &cuts, &apex_germs, band)?,
    };
    let unordered = |fault| SplitReduceError::SectionGermsUnordered { vertex, fault };
    // Each corner as its arc after the anchor, `(p, q)`: the entries
    // `anchor + p + 1 ..= anchor + q`.
    let mut arcs: Vec<(usize, usize)> = corners
        .iter()
        .map(|&[a, b]| {
            let (p, q) = ((a + n - anchor) % n, (b + n - anchor) % n);
            (p.min(q), p.max(q))
        })
        .collect();
    arcs.sort_unstable();
    let mut order = vec![None; n];
    for (i, &(p, q)) in arcs.iter().enumerate() {
        order[p] = Some(2 * i);
        order[q] = Some(2 * i + 1);
    }
    let order: Vec<usize> = order.into_iter().flatten().collect();
    let tree = crate::null::ring_tree(&order, Some(0))
        .ok_or_else(|| unordered(SectionGermFault::Crossing))?;
    // The vertex standing for each corner's child region, once minted.
    let mut child: Vec<Option<VertexKey>> = vec![None; arcs.len()];
    for strut in tree {
        let (p, q) = arcs[strut.run];
        let at = match strut.parent {
            None => vertex,
            Some(parent) => child[parent].unwrap_or_else(|| {
                unreachable!("ring_tree mints each strut after the one it hangs at")
            }),
        };
        let arc = |j: usize| &entries[(anchor + p + 1 + j) % n];
        let side = arc(0).class;
        // A corner pairs a start with an end, and either way round the
        // orbit its arc leaves the one and reaches the other on one side.
        if arc(q - p - 1).class != side {
            unreachable!(
                "the corner {:?} at {vertex:?} has an arc from {:?} to {:?}",
                (p, q),
                side,
                arc(q - p - 1).class
            );
        }
        let mut real = (0..q - p)
            .map(arc)
            .filter(|e| e.kind == SectorEntryKind::Edge);
        let first = real.next();
        let last = real.next_back().or(first);
        // `whole_orbit`: the copy is the strut tip standing for the
        // parent region.
        let (site, dangling, whole_orbit) = match (first, last) {
            // he2 at execution time (module docs).
            (Some(first), Some(last)) => {
                match body.run_site(first.he, last.he).unwrap_or_else(|| {
                    unreachable!(
                        "the arc {:?} ..= {:?} at {at:?} has no orbit successor: its halves come \
                     from the orbit the reduction just walked, the reduction kills nothing, and \
                     {}",
                        first.he,
                        last.he,
                        crate::live::NAMES_ONLY_LIVE
                    )
                }) {
                    site @ RunSite::Fan { .. } => (site.mev_site(), false, false),
                    site @ RunSite::WholeOrbit { .. } => (site.mev_site(), true, true),
                }
            }
            // Duplicates alone: the strut inside their sector P, before the
            // sector's next real edge Q, the first `Edge` entry past the arc
            // (an apex sector may hold a second duplicate between the arc
            // and Q). A sector holds at most one such arc: its arc is the
            // stretch between two sign changes of the plane's side inside
            // the sector, and that side changes sign at most twice in a
            // sector (once in a tangent plane's wide sector; twice in a
            // slit tip's full turn, which a plane through the tip meets
            // along one line; at a cone's apex, `c + k cos(φ − φ₀)` has at
            // most two zeros). P and Q lie
            // just outside the arc, in the parent region, and another
            // corner's mint splices only at its own arc's ends, so Q's
            // corner is still at `at`.
            _ => {
                let next_real = body.strut_before_next_real(
                    (at, arc(0).he),
                    (q + 1..q + 1 + n).map(|j| {
                        let e = &entries[(anchor + j) % n];
                        (e.he, e.kind == SectorEntryKind::Edge)
                    }),
                    || format!("the duplicate arc {:?}", (p, q)),
                );
                (
                    MevSite::Fan {
                        he1: next_real,
                        he2: next_real,
                    },
                    true,
                    false,
                )
            }
        };
        let side = match side {
            PlaneSide::Above => NewVertexSide::Above,
            _ => NewVertexSide::Below,
        };
        let mint = body.mev_null_run(site, at, side, !whole_orbit)?;
        let (created, attr) = (mint.created, mint.attr);
        child[strut.run] = Some(if whole_orbit { at } else { created.vertex });
        sides.insert(created.vertex, PlaneSide::On);
        // The half leaving `at` lies in the sector the orbit reaches just
        // before it, its mate in the one just after: the arc's first cut
        // and its last where the copy takes the arc, the other way round
        // where `at` keeps it.
        let edge = proven(&body.edges, created.edge, EntityId::Edge);
        let (from_at, to_at) =
            if proven(&body.half_edges, edge.he_plus, EntityId::HalfEdge).start == at {
                (edge.he_plus, edge.he_minus)
            } else {
                (edge.he_minus, edge.he_plus)
            };
        let [first, last] = [apex_germs[(anchor + p) % n], apex_germs[(anchor + q) % n]];
        let (from_at_germ, to_at_germ) = if whole_orbit {
            (last, first)
        } else {
            (first, last)
        };
        for (half, germ) in [(from_at, from_at_germ), (to_at, to_at_germ)] {
            if let Some(germ) = germ {
                germs.insert(half, germ);
            }
        }
        records.push(NullEdgeRecord {
            at_vertex: at,
            edge: created.edge,
            attr,
            dangling,
        });
    }
    Ok(())
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod tests {
    use super::*;
    use crate::entity::HalfEdgeKey;
    use crate::live::Proven;

    fn entries(classes: &[PlaneSide]) -> Vec<SectorEntry> {
        classes
            .iter()
            .map(|&class| SectorEntry {
                he: HalfEdgeKey::default(),
                kind: SectorEntryKind::Edge,
                class,
            })
            .collect()
    }

    use PlaneSide::{Above as A, Below as B};

    /// One-sided neighborhoods produce no runs (tangent vertex — no
    /// surgery); mixed neighborhoods produce one run per maximal ABOVE
    /// stretch, including the wrap-around and ≥2 disjoint runs
    /// (Fig. 14.9 / the Program 14.7 erratum case).
    #[test]
    fn run_enumeration() {
        assert_eq!(above_runs(&entries(&[A, A, A])), vec![]);
        assert_eq!(above_runs(&entries(&[B, B])), vec![]);
        // Single run, wrapping the seam: entries 3,0 are one run.
        assert_eq!(
            above_runs(&entries(&[A, B, B, A])),
            vec![AboveRun { start: 3, len: 2 }]
        );
        // Two disjoint runs around one vertex.
        assert_eq!(
            above_runs(&entries(&[A, B, A, A, B, B])),
            vec![AboveRun { start: 2, len: 2 }, AboveRun { start: 0, len: 1 },]
        );
        // Three runs, each a single entry.
        assert_eq!(above_runs(&entries(&[A, B, A, B, A, B])).len(), 3);
    }

    /// **A reflex corner whose two Below edges bound different pieces
    /// hangs its corners as a path.** A plane through the L-prism's
    /// reflex corner, leaning across it, leaves Above the reflex sector's
    /// duplicate and the vertical edge, one piece, and Below the two top
    /// edges, each bounding a piece of its own. At each of the corner's
    /// three orbit starts the reduction builds, and the three edges leave
    /// three vertices: the vertical edge's is the above end of both null
    /// edges, each top edge's the below end of one. A null edge per Above
    /// run would keep both top edges at the corner.
    #[test]
    fn a_reflex_corner_whose_below_edges_bound_two_pieces_hangs_a_path() {
        use crate::test_support_fixtures::{prism, split_plane};
        use geom_core::{Point3, Tol, Vec3};
        let tol = Tol::witness();
        let profile = [
            (0.0, 0.0),
            (2.0, 0.0),
            (2.0, 1.0),
            (1.0, 1.0),
            (1.0, 2.0),
            (0.0, 2.0),
        ];
        let (theta, phi) = (std::f64::consts::TAU * 0.37 / 12.0, 0.05_f64);
        let plane = split_plane(
            Point3::new(1.0, 1.0, 1.0),
            -Vec3::new(theta.cos() * phi.cos(), theta.sin() * phi.cos(), phi.sin()),
            tol,
        );
        let built = prism::<f64>(&profile, 1.0, tol);
        let corner = built.top[3];
        let starts = built.body.vertex_orbit_linked(corner);
        assert_eq!(starts.len(), 3, "the reflex corner has three edges");
        for start in starts.iter().copied() {
            let mut body = built.body.clone();
            body.get_vertex_mut(corner).unwrap().emanating = Some(start);
            let red = crate::splitting::reduce(&body, &plane, tol).unwrap();
            // The vertex each of the corner's edges leaves now, keyed by
            // its far end: +x, +y, −z.
            let at = |far: (f64, f64, f64)| {
                let he = starts
                    .iter()
                    .copied()
                    .find(|&he| {
                        let p = red
                            .body
                            .resolve_vertex_point(red.body.proven_half_edge_end(he), Proven);
                        (p.x, p.y, p.z) == far
                    })
                    .unwrap();
                red.body.get_half_edge(he).unwrap().start
            };
            let (x, y, z) = (
                at((2.0, 1.0, 1.0)),
                at((1.0, 2.0, 1.0)),
                at((1.0, 1.0, 0.0)),
            );
            let ours: Vec<_> = red
                .null_edges
                .iter()
                .filter(|r| [x, y, z].contains(&r.attr.above_end))
                .map(|r| (r.attr.above_end, r.attr.below_end, r.dangling))
                .collect();
            let mut belows = [ours[0].1, ours[1].1];
            belows.sort_unstable();
            let mut want = [x, y];
            want.sort_unstable();
            assert!(
                x != y
                    && ours.len() == 2
                    && ours.iter().all(|&(a, _, dangling)| a == z && !dangling)
                    && belows == want,
                "start {start:?}: +x at {x:?}, +y at {y:?}, −z at {z:?}; null edges (above, \
                 below, dangling) {ours:?}"
            );
        }
    }
}
