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
                    // A piece of the split edge: its carrier is the file's.
                    curve: spec.curve,
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

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used)]
mod tests {
    use std::collections::{BTreeMap, BTreeSet};

    use geom::Curve3;
    use geom_core::{Point3, Vec3};

    use super::{CutLine, EdgeUse, relink, split_at_point};
    use crate::entities::{EdgeSpec, FaceSpec, LoopSpec, SolidSpec};

    fn fwd(edge: u64) -> EdgeUse {
        EdgeUse {
            edge,
            forward: true,
        }
    }

    fn rev(edge: u64) -> EdgeUse {
        EdgeUse {
            edge,
            forward: false,
        }
    }

    /// The unit square `1 → 2 → 3 → 4` on edges `10..=13`, a pole `5`
    /// off it, and the cut edges `(id, start, end)`. Only the graph is
    /// read, so every carrier is one line.
    fn square(cuts: &[(u64, u64, u64)]) -> (SolidSpec, Vec<Vec<EdgeUse>>) {
        let line = Curve3::Line(geom::Line {
            origin: Point3::new(0.0, 0.0, 0.0),
            dir: Vec3::new(1.0, 0.0, 0.0),
        });
        let spec = |start, end| EdgeSpec {
            start,
            end,
            carrier: line.clone(),
            t0: 0.0,
            t1: 1.0,
            reversed: false,
            curve: None,
        };
        let mut edges: BTreeMap<u64, EdgeSpec> = [(10, 1, 2), (11, 2, 3), (12, 3, 4), (13, 4, 1)]
            .into_iter()
            .map(|(e, a, b)| (e, spec(a, b)))
            .collect();
        for &(e, a, b) in cuts {
            edges.insert(e, spec(a, b));
        }
        let vertices = (1..=5)
            .map(|v| (v, Point3::new(f64::from(v as u32), 0.0, 0.0)))
            .collect();
        let solid = SolidSpec {
            id: 1,
            faces: Vec::new(),
            edges,
            vertices,
            surfaces: BTreeMap::new(),
            band_seams: BTreeSet::new(),
        };
        (solid, vec![vec![fwd(10), fwd(11), fwd(12), fwd(13)]])
    }

    /// A diagonal `1 → 3` attached at both ends parts the square into
    /// its two triangles, each closing through the diagonal one way;
    /// the seeded one comes first and starts at its seed.
    #[test]
    fn a_cut_attached_at_both_ends_parts_the_face_from_its_seed() {
        let (solid, loops) = square(&[(20, 1, 3)]);
        let diagonal = CutLine {
            first: fwd(20),
            ends: [Some((0, 0)), Some((0, 2))],
        };
        let cycles = relink(&solid, &loops, &[diagonal], &[(0, 2)]).unwrap();
        assert_eq!(
            cycles,
            vec![
                vec![fwd(12), fwd(13), fwd(20)],
                vec![fwd(10), fwd(11), rev(20)],
            ],
            "the seeded triangle first, from its seed, then the other"
        );
    }

    /// Two spokes out of a pole off the loop, to vertices `1` and `3`:
    /// a walk arriving at the pole along one leaves along the other.
    #[test]
    fn a_walk_at_a_pole_off_the_loop_leaves_along_the_other_spoke() {
        let (solid, loops) = square(&[(21, 5, 1), (22, 5, 3)]);
        let spokes = [
            CutLine {
                first: fwd(21),
                ends: [None, Some((0, 0))],
            },
            CutLine {
                first: fwd(22),
                ends: [None, Some((0, 2))],
            },
        ];
        let cycles = relink(&solid, &loops, &spokes, &[]).unwrap();
        assert_eq!(
            cycles,
            vec![
                vec![fwd(10), fwd(11), rev(22), fwd(21)],
                vec![fwd(12), fwd(13), rev(21), fwd(22)],
            ],
            "each piece passes the pole from one spoke to the other"
        );
    }

    /// A spoke whose far end nothing else meets is walked out and back
    /// inside the one piece.
    #[test]
    fn a_spoke_with_a_free_end_is_walked_out_and_back() {
        let (solid, loops) = square(&[(23, 1, 5)]);
        let spoke = CutLine {
            first: fwd(23),
            ends: [Some((0, 0)), None],
        };
        let cycles = relink(&solid, &loops, &[spoke], &[]).unwrap();
        assert_eq!(
            cycles,
            vec![vec![fwd(10), fwd(11), fwd(12), fwd(13), fwd(23), rev(23)]],
            "one piece, the spoke walked out and back"
        );
    }

    /// A circle edge over `[3π/2, 5π/2]` split at `π/4` past the turn:
    /// the vertex is the point handed in, the far half's interval is the
    /// one a re-import derives from its endpoints (start angle in
    /// `[0, τ)`), not the parent's representative, and both uses of the
    /// edge walk the halves.
    #[test]
    fn a_split_past_the_turn_rederives_the_far_halfs_interval() {
        use core::f64::consts::{FRAC_PI_2, FRAC_PI_4, PI, TAU};
        let circle = Curve3::circle(
            Point3::new(0.0, 0.0, 0.0),
            Vec3::new(0.0, 0.0, 1.0),
            1.0,
            Vec3::new(1.0, 0.0, 0.0),
        );
        let (t0, t1) = (1.5 * PI, 2.5 * PI);
        let face = |id, forward| FaceSpec {
            id,
            record: 900,
            sense: true,
            loops: vec![LoopSpec {
                outer: true,
                uses: vec![EdgeUse { edge: 7, forward }],
            }],
            band: false,
        };
        let mut solid = SolidSpec {
            id: 1,
            faces: vec![face(2, true), face(3, false)],
            edges: BTreeMap::from([(
                7,
                EdgeSpec {
                    start: 1,
                    end: 2,
                    carrier: circle.clone(),
                    t0,
                    t1,
                    reversed: false,
                    curve: None,
                },
            )]),
            vertices: BTreeMap::from([(1, circle.eval(t0)), (2, circle.eval(t1))]),
            surfaces: crate::entities::records_of([(
                900,
                geom::Surface::Plane {
                    frame: geom::Frame::new(
                        Point3::new(0.0, 0.0, 0.0),
                        Vec3::new(0.0, 0.0, 1.0),
                        Vec3::new(1.0, 0.0, 0.0),
                    ),
                },
            )]),
            band_seams: BTreeSet::new(),
        };
        let q = Point3::new(FRAC_PI_4.cos(), FRAC_PI_4.sin(), 0.0);
        let mut next = 100;
        let v = split_at_point(&mut solid, 7, TAU + FRAC_PI_4, q, &mut || {
            next += 1;
            next
        })
        .unwrap();
        assert_eq!(v, 101, "the vertex is minted first");
        assert!(
            solid.vertices[&v].same_bits(q),
            "the vertex is the point handed in"
        );
        let (near, far) = (&solid.edges[&102], &solid.edges[&103]);
        assert_eq!((near.start, near.end, far.start, far.end), (1, v, v, 2));
        assert!(
            (near.t0 - t0).abs() < 1e-12 && (near.t1 - (TAU + FRAC_PI_4)).abs() < 1e-12,
            "the near half keeps its start's representative: {:?}",
            (near.t0, near.t1)
        );
        assert!(
            (far.t0 - FRAC_PI_4).abs() < 1e-12 && (far.t1 - FRAC_PI_2).abs() < 1e-12,
            "the far half starts in [0, τ), as a re-import reads it: {:?}",
            (far.t0, far.t1)
        );
        assert!(!solid.edges.contains_key(&7), "the parent is gone");
        assert_eq!(
            solid.faces[0].loops[0].uses,
            vec![fwd(102), fwd(103)],
            "a forward use walks the halves in order"
        );
        assert_eq!(
            solid.faces[1].loops[0].uses,
            vec![rev(103), rev(102)],
            "a reversed use walks them backwards, each reversed"
        );
    }
}
