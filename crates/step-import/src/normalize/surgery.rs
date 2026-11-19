//! **The boundary surgery the shell passes share**: the C⁰-knot split
//! ([`crate::knot_split`]), the band re-mint ([`super::band_seam`]) and
//! the chart-singularity pass ([`super::singularity`]) each split edges
//! and cut a face along lines of their own choosing. What a split or a
//! cut does to the boundary graph is decided here, once:
//!
//! - which vertex a use starts and ends at ([`use_start`], [`use_end`]);
//! - a set of faces' census ([`boundary_census`]);
//! - an edge split into pieces, every use of it in the shell walked
//!   through them ([`split_edge`], [`split_at_point`]);
//! - a face's loops and its cut lines re-linked into the cycles that
//!   bound its pieces ([`relink`]).
//!
//! Each pass keeps what is its own: where it cuts, the carriers it
//! mints, and which surface each piece takes.

use std::collections::BTreeSet;
use std::sync::Arc;

use geom::Curve3;
use geom_core::Point3;

use crate::entities::{EdgeSpec, EdgeUse, SolidSpec};
use crate::error::StepImportError;

/// The vertex a use starts at.
pub(crate) fn use_start(solid: &SolidSpec, u: EdgeUse) -> u64 {
    let spec = &solid.edges[&u.edge];
    if u.forward { spec.start } else { spec.end }
}

/// The vertex a use ends at.
pub(crate) fn use_end(solid: &SolidSpec, u: EdgeUse) -> u64 {
    let spec = &solid.edges[&u.edge];
    if u.forward { spec.end } else { spec.start }
}

/// The distinct edges and vertices the loops of `faces` use.
pub(crate) fn boundary_census(solid: &SolidSpec, faces: &[usize]) -> (usize, usize) {
    let mut edges = BTreeSet::new();
    let mut vertices = BTreeSet::new();
    for &fi in faces {
        for u in solid.faces[fi].loops.iter().flat_map(|l| &l.uses) {
            edges.insert(u.edge);
            let spec = &solid.edges[&u.edge];
            vertices.insert(spec.start);
            vertices.insert(spec.end);
        }
    }
    (edges.len(), vertices.len())
}

/// Where a split places the vertex it mints.
#[derive(Clone, Copy)]
pub(crate) enum Place {
    /// At this point.
    At(Point3<f64>),
    /// At the control point a B-spline carrier's two children share,
    /// which the spline interpolates at a crease knot.
    Joint,
}

/// What [`split_edge`] made: the new vertices in parameter order, and
/// the pieces from the edge's start to its end.
pub(crate) struct Split {
    pub(crate) vertices: Vec<u64>,
    pub(crate) pieces: Vec<u64>,
}

/// Splits edge `id` at its interior parameters `at`, increasing, each
/// new vertex placed as its [`Place`] says, and walks every use of the
/// edge in the shell through the pieces. A B-spline carrier is split
/// into its children on the parent's own parameter; any other carrier
/// is shared by the pieces over their sub-intervals. Each piece is a
/// minted edge from its start to its end along the carrier's
/// increasing parameter.
///
/// `None` where the edge is not in the shell, a B-spline carrier
/// refuses a split, or a [`Place::Joint`] is asked of a carrier that
/// is not one; nothing has moved then.
pub(crate) fn split_edge(
    solid: &mut SolidSpec,
    id: u64,
    at: &[(f64, Place)],
    mint: &mut dyn FnMut() -> u64,
) -> Option<Split> {
    let spec = solid.edges.get(&id)?;
    let carriers: Vec<Curve3<f64>> = match spec.carrier {
        Curve3::Nurbs(ref c) => {
            let mut out = Vec::with_capacity(at.len() + 1);
            let mut rest = c.as_ref().clone();
            for &(t, _) in at {
                let (lo, hi) = rest.split_at(t).ok()?;
                out.push(Curve3::Nurbs(Arc::new(lo)));
                rest = hi;
            }
            out.push(Curve3::Nurbs(Arc::new(rest)));
            out
        }
        ref carrier => vec![carrier.clone(); at.len() + 1],
    };
    let points = at
        .iter()
        .zip(&carriers)
        .map(|(&(_, place), lo)| match (place, lo) {
            (Place::At(q), _) => Some(q),
            (Place::Joint, Curve3::Nurbs(c)) => c.control().last().copied(),
            (Place::Joint, _) => None,
        })
        .collect::<Option<Vec<_>>>()?;
    let Some(spec) = solid.edges.remove(&id) else {
        unreachable!("the edge was read from the map above")
    };
    let mut ends = vec![spec.start];
    for q in points {
        let v = mint();
        solid.vertices.insert(v, q);
        ends.push(v);
    }
    ends.push(spec.end);
    let params: Vec<f64> = std::iter::once(spec.t0)
        .chain(at.iter().map(|&(t, _)| t))
        .chain(std::iter::once(spec.t1))
        .collect();
    let pieces: Vec<u64> = carriers
        .into_iter()
        .enumerate()
        .map(|(i, carrier)| {
            let e = mint();
            solid.edges.insert(
                e,
                EdgeSpec {
                    start: ends[i],
                    end: ends[i + 1],
                    carrier,
                    t0: params[i],
                    t1: params[i + 1],
                    // A minted edge: its start → end is the carrier's
                    // increasing parameter, so there is no sense to
                    // compose.
                    reversed: false,
                },
            );
            e
        })
        .collect();
    expand_uses(solid, id, &pieces);
    Some(Split {
        vertices: ends[1..ends.len() - 1].to_vec(),
        pieces,
    })
}

/// Splits `edge` at its interior parameter `t`, the new vertex placed
/// at `point` exactly and both halves' intervals re-derived from their
/// endpoints, as a re-import derives them ([`split_edge`] otherwise).
/// Answers the new vertex.
pub(crate) fn split_at_point(
    solid: &mut SolidSpec,
    edge: u64,
    t: f64,
    point: Point3<f64>,
    mint: &mut dyn FnMut() -> u64,
) -> Result<u64, StepImportError> {
    let Some(Split { vertices, pieces }) = split_edge(solid, edge, &[(t, Place::At(point))], mint)
    else {
        return Err(StepImportError::Topology {
            id: edge,
            what: "internal: an edge refused its split at an interior parameter",
        });
    };
    for half in pieces {
        let Some(spec) = solid.edges.get_mut(&half) else {
            unreachable!("a piece `split_edge` minted is in `solid.edges`")
        };
        let self_loop = spec.start == spec.end;
        let (ps, pe) = (solid.vertices[&spec.start], solid.vertices[&spec.end]);
        let (t0, t1) = crate::geometry::endpoint_params(half, &spec.carrier, ps, pe, self_loop)?;
        spec.t0 = t0;
        spec.t1 = t1;
    }
    Ok(vertices[0])
}

/// Replaces every use of `edge` by `pieces`, which run from the edge's
/// start to its end: a forward use walks them in order, a reversed use
/// walks them backwards, each reversed.
fn expand_uses(solid: &mut SolidSpec, edge: u64, pieces: &[u64]) {
    for lp in solid.faces.iter_mut().flat_map(|f| &mut f.loops) {
        if !lp.uses.iter().any(|u| u.edge == edge) {
            continue;
        }
        lp.uses = lp
            .uses
            .iter()
            .flat_map(|&u| {
                let run: Vec<EdgeUse> = if u.edge != edge {
                    vec![u]
                } else if u.forward {
                    pieces
                        .iter()
                        .map(|&e| EdgeUse {
                            edge: e,
                            forward: true,
                        })
                        .collect()
                } else {
                    pieces
                        .iter()
                        .rev()
                        .map(|&e| EdgeUse {
                            edge: e,
                            forward: false,
                        })
                        .collect()
                };
                run
            })
            .collect();
    }
}

/// A position in a face's loops: `(loop, use)`.
pub(crate) type At = (usize, usize);

/// A cut line: an edge the cut walks both ways, `first` way first, and
/// where each end of that first walk meets the loops — the position of
/// the loop use that leaves its vertex there. `None` for an end off the
/// loops (a minted pole, a spoke's free end).
#[derive(Clone, Copy)]
pub(crate) struct CutLine {
    pub(crate) first: EdgeUse,
    pub(crate) ends: [Option<At>; 2],
}

/// Why a cut did not close into cycles.
#[derive(Debug)]
pub(crate) enum Unlinked {
    /// A vertex off the loops where more than one other cut leaves.
    Branch,
    /// A walk that returned to a use other than the one it started at.
    Open,
}

/// **The face cut**: re-links `loops` and `cuts`, each cut walked both
/// ways, into the closed cycles that bound the pieces. The rule at a
/// vertex:
///
/// - a loop use arriving where a cut is attached takes the cut away
///   from there (the first such cut, in order), and otherwise goes on
///   along its loop;
/// - a cut arriving at an attached end takes the loop use leaving
///   there;
/// - a cut arriving at an end off the loops takes the one other cut
///   leaving that vertex, or, where none does, comes back along itself.
///
/// The cycles are traced from `seeds` first, in order, then from each
/// use not yet traced: loop uses in loop order, then each cut's first
/// walk and its return. A cycle starts at its seed.
pub(crate) fn relink(
    solid: &SolidSpec,
    loops: &[Vec<EdgeUse>],
    cuts: &[CutLine],
    seeds: &[At],
) -> Result<Vec<Vec<EdgeUse>>, Unlinked> {
    let mut offset = Vec::with_capacity(loops.len());
    let mut all: Vec<EdgeUse> = Vec::new();
    for lp in loops {
        offset.push(all.len());
        all.extend(lp);
    }
    let n = all.len();
    for cut in cuts {
        all.push(cut.first);
        all.push(EdgeUse {
            edge: cut.first.edge,
            forward: !cut.first.forward,
        });
    }
    let index = |(r, i): At| offset[r] + i;
    let link = |k: usize| -> Result<usize, Unlinked> {
        if k < n {
            let r = offset.partition_point(|&o| o <= k) - 1;
            let i = k - offset[r];
            let after = (r, (i + 1) % loops[r].len());
            for (c, cut) in cuts.iter().enumerate() {
                if cut.ends[0] == Some(after) {
                    return Ok(n + 2 * c);
                }
                if cut.ends[1] == Some(after) {
                    return Ok(n + 2 * c + 1);
                }
            }
            return Ok(index(after));
        }
        let (c, back) = ((k - n) / 2, (k - n) % 2 == 1);
        if let Some(at) = cuts[c].ends[usize::from(!back)] {
            return Ok(index(at));
        }
        let v = use_end(solid, all[k]);
        let others: Vec<usize> = (n..all.len())
            .filter(|&o| (o - n) / 2 != c && use_start(solid, all[o]) == v)
            .collect();
        match others.as_slice() {
            [] => Ok(if back { k - 1 } else { k + 1 }),
            [o] => Ok(*o),
            _ => Err(Unlinked::Branch),
        }
    };
    let next = (0..all.len()).map(link).collect::<Result<Vec<_>, _>>()?;
    let mut seen = vec![false; all.len()];
    let mut cycles = Vec::new();
    for s in seeds.iter().map(|&at| index(at)).chain(0..all.len()) {
        if seen[s] {
            continue;
        }
        let mut cycle = Vec::new();
        let mut k = s;
        while !seen[k] {
            seen[k] = true;
            cycle.push(all[k]);
            k = next[k];
        }
        if k != s {
            return Err(Unlinked::Open);
        }
        cycles.push(cycle);
    }
    Ok(cycles)
}
