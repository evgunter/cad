//! **The split at each interior C⁰ knot** (`docs/DESIGN.md` [topology]:
//! every face chart and edge carrier is C¹ by its knot structure
//! strictly inside its cell, and import splits at its door).
//!
//! A foreign B-spline carrier may hold an interior knot run of
//! multiplicity equal to its degree ([`KnotStructure::crease_runs`]).
//! Across that run the spline has no derivative by its structure,
//! whether or not the locus is smooth there, so the door cuts the cell
//! at it:
//!
//! - **an edge carrier** is split at each such knot into its sub-arcs,
//!   a new vertex at the knot's point;
//! - **a face chart** is split along the knot's iso line into its two
//!   sub-patches, the iso curve minted as the edge between them.
//!
//! Both splits are exact. At a run of full multiplicity the net already
//! separates the two sub-nets: each child is a partition of the
//! parent's knots, control points and weights, sharing the row (or the
//! point) the spline interpolates at the knot, and no arithmetic is
//! done. The minted iso edge is that shared row, so its carrier is
//! bitwise both sub-patches' boundary column.
//!
//! Every split is reported as a [`StructureNormalization`], never done
//! silently. A knot vertex is one the closing join keeps
//! (`topo`'s `joinable`): two spline pieces meeting at a knot of full
//! multiplicity are never joined. A face whose trim the split cannot
//! place ([`split_face_once`]) refuses typed rather than ship the
//! crease.

use std::collections::BTreeSet;
use std::sync::Arc;

use geom::{Curve3, NurbsCurve3, NurbsSurface, Surface};
use geom_core::Point3;
use geom_core::spline::{KnotStructure, KnotVector};

use crate::entities::{EdgeSpec, EdgeUse, FaceSpec, LoopSpec, SolidSpec};
use crate::error::StepImportError;
use crate::{FaceCensus, NormalizationKind, StructureNormalization};

/// Runs both splits over the shell: edges first, so a face's knot line
/// finds the vertices its boundary edges' own splits placed.
pub(crate) fn split_c0_knots(
    solid: &mut SolidSpec,
    eps_in: f64,
    mint: &mut dyn FnMut() -> u64,
    sink: &mut Vec<StructureNormalization>,
) -> Result<(), StepImportError> {
    split_edges(solid, mint, sink)?;
    split_faces(solid, eps_in, mint, sink)
}

/// The knot values of `kv`'s crease runs, ascending.
fn crease_knots(kv: &KnotVector) -> Vec<f64> {
    let runs: Vec<f64> = kv.knot_runs().map(|(value, _)| value).collect();
    let structure: &KnotStructure = kv.structure();
    structure.crease_runs().map(|r| runs[r]).collect()
}

/// The least `ADVANCED_FACE` id whose loops use `edge`.
fn least_face_using(solid: &SolidSpec, edge: u64) -> Option<u64> {
    solid
        .faces
        .iter()
        .filter(|f| {
            f.loops
                .iter()
                .any(|l| l.uses.iter().any(|u| u.edge == edge))
        })
        .map(|f| f.id)
        .min()
}

/// Replaces every use of `edge` by `pieces`, which run from the edge's
/// start to its end: a forward use walks them in order, a reversed use
/// walks them backwards, each reversed.
fn expand_uses(solid: &mut SolidSpec, edge: u64, pieces: &[u64]) {
    for face in &mut solid.faces {
        for lp in &mut face.loops {
            lp.uses = lp
                .uses
                .iter()
                .flat_map(|u| {
                    let run: Vec<EdgeUse> = if u.edge != edge {
                        vec![*u]
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
}

/// Splits every B-spline edge carrier at each of its crease knots.
///
/// A B-spline edge spans its carrier's whole knot domain
/// ([`crate::geometry::endpoint_params`]), so every crease knot is
/// strictly inside the edge. Each sub-arc is the carrier's
/// [`NurbsCurve3::split_at`] child on the parent's own parameter, and
/// the new vertex is the control point the sub-arcs share, which the
/// spline interpolates at a run of full multiplicity.
fn split_edges(
    solid: &mut SolidSpec,
    mint: &mut dyn FnMut() -> u64,
    sink: &mut Vec<StructureNormalization>,
) -> Result<(), StepImportError> {
    let ids: Vec<u64> = solid.edges.keys().copied().collect();
    for id in ids {
        let Curve3::Nurbs(ref carrier) = solid.edges[&id].carrier else {
            continue;
        };
        let carrier = Arc::clone(carrier);
        let knots = crease_knots(carrier.knots_f64_interim());
        if knots.is_empty() {
            continue;
        }
        let face = least_face_using(solid, id).ok_or(StepImportError::Topology {
            id,
            what: "internal: an EDGE_CURVE the shell holds bounds no face",
        })?;
        let mut arcs = Vec::with_capacity(knots.len() + 1);
        let mut rest: NurbsCurve3<f64> = carrier.as_ref().clone();
        for &k in &knots {
            let (lo, hi) = rest.split_at(k).map_err(|_| StepImportError::Topology {
                id,
                what: "internal: a B-spline carrier refused its split at its own crease knot",
            })?;
            arcs.push(lo);
            rest = hi;
        }
        arcs.push(rest);
        let Some(spec) = solid.edges.remove(&id) else {
            unreachable!("`ids` are the edge map's own keys, and each is removed once")
        };
        let mut ends = vec![spec.start];
        for arc in &arcs[..arcs.len() - 1] {
            let v = mint();
            let Some(&p) = arc.control().last() else {
                unreachable!("a validated B-spline holds at least two control points")
            };
            solid.vertices.insert(v, p);
            ends.push(v);
        }
        ends.push(spec.end);
        let params: Vec<f64> = std::iter::once(spec.t0)
            .chain(knots.iter().copied())
            .chain(std::iter::once(spec.t1))
            .collect();
        let pieces: Vec<u64> = arcs
            .into_iter()
            .enumerate()
            .map(|(i, arc)| {
                let e = mint();
                solid.edges.insert(
                    e,
                    EdgeSpec {
                        start: ends[i],
                        end: ends[i + 1],
                        carrier: Curve3::Nurbs(Arc::new(arc)),
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
        sink.push(StructureNormalization {
            face,
            kind: NormalizationKind::EdgeSplitAtC0Knot,
            file_census: FaceCensus {
                faces: 0,
                edges: 1,
                vertices: 0,
            },
            kernel_census: FaceCensus {
                faces: 0,
                edges: pieces.len(),
                vertices: pieces.len() - 1,
            },
        });
    }
    Ok(())
}

/// The parameter at which `spec`'s carrier passes within `eps_in` of
/// `q`, strictly inside the edge, or `None`. An analytic carrier reads
/// it in closed form ([`Curve3::param_near`], anchored at the edge's
/// midpoint); a B-spline carrier by its projection.
fn interior_param_at(spec: &EdgeSpec, q: Point3<f64>, eps_in: f64) -> Option<f64> {
    let t = match spec.carrier {
        Curve3::Nurbs(ref c) => c.project(q).ok()?.t,
        ref carrier => carrier.param_near(q, geom::mid_param(spec.t0, spec.t1))?,
    };
    let inside = spec.t0 < t && t < spec.t1;
    (inside && (spec.carrier.eval(t) - q).norm() <= eps_in).then_some(t)
}

/// Splits edge `id` at its interior parameter `t`, the new vertex at
/// `q`, and walks every use of it through the two halves. A B-spline
/// carrier is split into its two children; any other carrier is shared
/// by both halves over their sub-intervals. Answers the new vertex.
fn split_edge_at(
    solid: &mut SolidSpec,
    id: u64,
    t: f64,
    q: Point3<f64>,
    mint: &mut dyn FnMut() -> u64,
) -> Option<u64> {
    let spec = solid.edges.remove(&id)?;
    let (lo, hi) = match spec.carrier {
        Curve3::Nurbs(ref c) => {
            let (lo, hi) = c.split_at(t).ok()?;
            (Curve3::Nurbs(Arc::new(lo)), Curve3::Nurbs(Arc::new(hi)))
        }
        ref carrier => (carrier.clone(), carrier.clone()),
    };
    let v = mint();
    solid.vertices.insert(v, q);
    let (a, b) = (mint(), mint());
    for (e, start, end, carrier, t0, t1) in [
        (a, spec.start, v, lo, spec.t0, t),
        (b, v, spec.end, hi, t, spec.t1),
    ] {
        solid.edges.insert(
            e,
            EdgeSpec {
                start,
                end,
                carrier,
                t0,
                t1,
                // A minted edge: its start → end is the carrier's
                // increasing parameter, so there is no sense to compose.
                reversed: false,
            },
        );
    }
    expand_uses(solid, id, &[a, b]);
    Some(v)
}

/// The distinct edges and vertices the loops of `faces` use.
fn boundary_census(solid: &SolidSpec, faces: &[usize]) -> (usize, usize) {
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

/// Splits every B-spline face chart along each of its crease knots' iso
/// lines, each split face splitting again until no sub-patch holds one.
fn split_faces(
    solid: &mut SolidSpec,
    eps_in: f64,
    mint: &mut dyn FnMut() -> u64,
    sink: &mut Vec<StructureNormalization>,
) -> Result<(), StepImportError> {
    for fi in 0..solid.faces.len() {
        if !matches!(solid.faces[fi].surface, Surface::Nurbs(_)) {
            continue;
        }
        let (edges, vertices) = boundary_census(solid, &[fi]);
        let mut made = vec![fi];
        let mut work = vec![fi];
        while let Some(f) = work.pop() {
            if let Some(new) = split_face_once(solid, f, eps_in, mint)? {
                work.extend([f, new]);
                made.push(new);
            }
        }
        if made.len() > 1 {
            let (kernel_edges, kernel_vertices) = boundary_census(solid, &made);
            sink.push(StructureNormalization {
                face: solid.faces[fi].id,
                kind: NormalizationKind::FaceSplitAtC0Knot,
                file_census: FaceCensus {
                    faces: 1,
                    edges,
                    vertices,
                },
                kernel_census: FaceCensus {
                    faces: made.len(),
                    edges: kernel_edges,
                    vertices: kernel_vertices,
                },
            });
        }
    }
    Ok(())
}

/// A clamped net's two children at the crease knot `k` along `u`: the
/// knots below `k` with `k` closing the low child at `p + 1`, the knots
/// from `k` with `k` opening the high child at `p + 1`, and the rows
/// each child spans, the row interpolated at `k` shared. A partition,
/// so exact.
fn split_net_u(s: &NurbsSurface<f64>, k: f64) -> Option<(NurbsSurface<f64>, NurbsSurface<f64>)> {
    let kv = s.knots_u_f64_interim();
    let (p, knots) = (kv.degree(), kv.knots());
    let first = knots.iter().position(|&x| x == k)?;
    let (_, nv) = s.control_counts();
    let (control, weights) = (s.control(), s.weights_f64_interim());
    let lo_knots: Vec<f64> = knots[..first + p].iter().copied().chain([k]).collect();
    let hi_knots: Vec<f64> = [k]
        .into_iter()
        .chain(knots[first..].iter().copied())
        .collect();
    let child = |knots: Vec<f64>, rows: core::ops::Range<usize>| {
        let net = rows.start * nv..rows.end * nv;
        NurbsSurface::new(
            KnotVector::clamped(knots, p).ok()?,
            s.knot_values_v().clone(),
            control[net.clone()].to_vec(),
            weights[net].to_vec(),
        )
        .ok()
    };
    let rows = s.control_counts().0;
    Some((
        child(lo_knots, 0..first)?,
        child(hi_knots, first - 1..rows)?,
    ))
}

/// A chart split at one crease knot: the two children, each in the
/// parent's own chart.
struct NetSplit {
    /// The child below the knot.
    lo: NurbsSurface<f64>,
    /// The child above it.
    hi: NurbsSurface<f64>,
}

/// The chart's crease knots, along `u` before `v`: `(knot, along_v)`.
fn crease_lines(s: &NurbsSurface<f64>) -> Vec<(f64, bool)> {
    let u = crease_knots(s.knots_u_f64_interim())
        .into_iter()
        .map(|k| (k, false));
    let v = crease_knots(s.knots_v_f64_interim())
        .into_iter()
        .map(|k| (k, true));
    u.chain(v).collect()
}

/// The chart split at its crease knot `knot`, along `v` where
/// `along_v`; `None` where the partition refused.
fn split_net(s: &NurbsSurface<f64>, knot: f64, along_v: bool) -> Option<NetSplit> {
    if !along_v {
        let (lo, hi) = split_net_u(s, knot)?;
        return Some(NetSplit { lo, hi });
    }
    let (lo, hi) = split_net_u(&s.transposed(), knot)?;
    Some(NetSplit {
        lo: lo.transposed(),
        hi: hi.transposed(),
    })
}

/// Which sides of the iso line `knot` (along `v` where `along_v`) the
/// face's boundary reaches: `(below, above)`. Each edge is sampled at
/// nine points and each sample's foot read on the chart by projection;
/// a foot within `eps_in` of the line, carried into the chart by the
/// local metric, is on it and on neither side. A SELECTION of which
/// cell the line lies in, never a geometry: the split it gates is exact.
fn sides(
    solid: &SolidSpec,
    fi: usize,
    s: &NurbsSurface<f64>,
    knot: f64,
    along_v: bool,
    eps_in: f64,
) -> Option<(bool, bool)> {
    let (mut below, mut above) = (false, false);
    for u in solid.faces[fi].loops.iter().flat_map(|l| &l.uses) {
        let spec = &solid.edges[&u.edge];
        for i in 0..=8 {
            let t = spec.t0 + (spec.t1 - spec.t0) * f64::from(i) / 8.0;
            let foot = s.project(spec.carrier.eval(t)).ok()?;
            let jet = s.ders(foot.u, foot.v);
            let (c, metric) = if along_v {
                (foot.v, jet.dv.norm())
            } else {
                (foot.u, jet.du.norm())
            };
            let reach = eps_in / metric;
            below |= c < knot - reach;
            above |= c > knot + reach;
        }
    }
    Some((below, above))
}

/// The vertex a use starts at.
fn use_start(solid: &SolidSpec, u: EdgeUse) -> u64 {
    let spec = &solid.edges[&u.edge];
    if u.forward { spec.start } else { spec.end }
}

/// Splits face `fi` once, along its first crease knot's iso line, and
/// answers the index of the new face, or `None` where no crease knot's
/// iso line crosses the face ([`sides`]): a line outside the face's
/// trimmed domain, or along its boundary, lies inside no cell, and the
/// face keeps its chart.
///
/// The iso line runs across the whole chart, from `Q₀` on the low
/// boundary of the other direction to `Q₁` on the high one; both are
/// control points of the net, so exact. The face's single loop must
/// pass through each within `eps_in`: at a vertex, or inside one edge,
/// which is split there first (an analytic rim, or a spline stated on
/// other knots). The two chains between them must each hold a corner of
/// one sub-patch and none of the other; on a chart closed across the
/// split, whose two sides share their corners, each must walk the
/// closed boundary the way its own sub-patch's counter-clockwise
/// boundary does. The low sub-patch takes its chain, closed by the
/// minted iso edge, and the high sub-patch the other. Anything else
/// refuses typed, naming the face and the knot
/// (`work/foreign/the-c0-knot-split-refuses-what-it-cannot-place.md`).
fn split_face_once(
    solid: &mut SolidSpec,
    fi: usize,
    eps_in: f64,
    mint: &mut dyn FnMut() -> u64,
) -> Result<Option<usize>, StepImportError> {
    let Surface::Nurbs(ref s) = solid.faces[fi].surface else {
        return Ok(None);
    };
    let s = Arc::clone(s);
    let face_id = solid.faces[fi].id;
    let refuse = |knot: f64, what: &'static str| StepImportError::C0KnotUnsplit {
        face: face_id,
        knot,
        what,
    };
    // The first knot whose line crosses the face. A line outside the
    // face's trimmed domain, or along its boundary, lies in no cell's
    // interior, and the face keeps its chart as stated.
    let mut crossing = None;
    for (knot, along_v) in crease_lines(&s) {
        let (below, above) = sides(solid, fi, &s, knot, along_v, eps_in).ok_or(refuse(
            knot,
            "a point of the face's boundary did not invert onto its chart",
        ))?;
        if below && above {
            crossing = Some((knot, along_v));
            break;
        }
    }
    let Some((knot, along_v)) = crossing else {
        return Ok(None);
    };
    let NetSplit { lo, hi, .. } = split_net(&s, knot, along_v).ok_or(refuse(
        knot,
        "the chart refused its split at its own crease knot",
    ))?;
    // The iso curve at the knot: the high child's start column (along
    // `u`) or start row (along `v`), bitwise the low child's end.
    let iso = if along_v {
        geom_brep::boundary_iso_v(&hi, false)
    } else {
        geom_brep::boundary_iso_u(&hi, false)
    }
    .map_err(|_| refuse(knot, "the knot's iso curve did not re-wrap as a curve"))?;
    if solid.faces[fi].loops.len() != 1 {
        return Err(refuse(
            knot,
            "the face has rings, which the split does not place",
        ));
    }
    let near = |a: Point3<f64>, b: Point3<f64>| (a - b).norm() <= eps_in;
    let (q0, q1) = match (iso.control().first(), iso.control().last()) {
        (Some(&a), Some(&b)) => (a, b),
        _ => unreachable!("a validated B-spline holds at least two control points"),
    };
    // The corners of each side, in the face's own chart.
    let corners = |net: &NurbsSurface<f64>, low: bool| -> [Point3<f64>; 2] {
        let (nu, nv) = net.control_counts();
        let c = net.control();
        match (along_v, low) {
            (false, true) => [c[0], c[nv - 1]],
            (false, false) => [c[(nu - 1) * nv], c[nu * nv - 1]],
            (true, true) => [c[0], c[(nu - 1) * nv]],
            (true, false) => [c[nv - 1], c[nu * nv - 1]],
        }
    };
    let (lo_corners, hi_corners) = (corners(&lo, true), corners(&hi, false));
    // Where the line meets the boundary inside an edge that does not
    // carry the knot (an analytic rim, or a spline stated on other
    // knots), that edge is split at the line's end first.
    for q in [q0, q1] {
        let loop_uses = &solid.faces[fi].loops[0].uses;
        if loop_uses
            .iter()
            .any(|&u| near(solid.vertices[&use_start(solid, u)], q))
        {
            continue;
        }
        let crossing: Vec<(u64, f64)> = loop_uses
            .iter()
            .filter_map(|u| Some((u.edge, interior_param_at(&solid.edges[&u.edge], q, eps_in)?)))
            .collect();
        if let [(edge, t)] = crossing[..] {
            split_edge_at(solid, edge, t, q, mint).ok_or(refuse(
                knot,
                "the boundary edge the knot's iso line ends on refused its split there",
            ))?;
        }
    }
    let uses = solid.faces[fi].loops[0].uses.clone();
    let starts: Vec<u64> = uses.iter().map(|&u| use_start(solid, u)).collect();
    let at = |q: Point3<f64>| -> Result<usize, StepImportError> {
        let mut hits = starts
            .iter()
            .enumerate()
            .filter(|&(_, v)| near(solid.vertices[v], q));
        match (hits.next(), hits.next()) {
            (Some((i, _)), None) => Ok(i),
            (None, _) => Err(refuse(
                knot,
                "the face's loop has no vertex where the knot's iso line meets its boundary",
            )),
            (Some(_), Some(_)) => Err(refuse(
                knot,
                "the face's loop passes more than once where the knot's iso line meets its \
                 boundary",
            )),
        }
    };
    let (i0, i1) = (at(q0)?, at(q1)?);
    if i0 == i1 {
        return Err(refuse(knot, "the knot's iso line closes on itself"));
    }
    let n = uses.len();
    let chain = |from: usize, to: usize| -> Vec<EdgeUse> {
        (0..n)
            .map(|j| uses[(from + j) % n])
            .take((to + n - from) % n)
            .collect()
    };
    let (a, b) = (chain(i0, i1), chain(i1, i0));
    let holds = |c: &[EdgeUse], corners: &[Point3<f64>; 2]| {
        c.iter().skip(1).any(|&u| {
            corners
                .iter()
                .any(|&q| near(solid.vertices[&use_start(solid, u)], q))
        })
    };
    let a_low = holds(&a, &lo_corners) && !holds(&a, &hi_corners);
    let b_low = holds(&b, &lo_corners) && !holds(&b, &hi_corners);
    let a_high = holds(&a, &hi_corners) && !holds(&a, &lo_corners);
    let b_high = holds(&b, &hi_corners) && !holds(&b, &lo_corners);
    // A chart closed across the split direction (its low and high
    // boundaries one curve, as a closed patch states its seam) puts the
    // same corners on both sides, so the chains are told apart by which
    // way they walk that boundary: counter-clockwise in the chart (the
    // loop of a face whose `sense` agrees with `S_u × S_v`), the low
    // sub-patch walks its `u₀` column from `v₁` down to `v₀`, or its
    // `v₀` row from `u₀` up to `u₁`.
    let closed = lo_corners
        .iter()
        .zip(&hi_corners)
        .all(|(&l, &h)| near(l, h));
    let walks = |c: &[EdgeUse], from: Point3<f64>, to: Point3<f64>| {
        c.iter().any(|&u| {
            let spec = &solid.edges[&u.edge];
            let end = if u.forward { spec.end } else { spec.start };
            near(solid.vertices[&use_start(solid, u)], from) && near(solid.vertices[&end], to)
        })
    };
    let (from, to) = match (along_v, solid.faces[fi].sense) {
        (false, true) | (true, false) => (lo_corners[1], lo_corners[0]),
        (false, false) | (true, true) => (lo_corners[0], lo_corners[1]),
    };
    let a_is_low = match (a_low && b_high, b_low && a_high) {
        (true, false) => true,
        (false, true) => false,
        _ if closed && walks(&a, from, to) && walks(&b, to, from) => true,
        _ if closed && walks(&b, from, to) && walks(&a, to, from) => false,
        _ => {
            return Err(refuse(
                knot,
                "the knot's iso line does not part the face's loop into one chain on each \
                 sub-patch (a trim that crosses the line more than once)",
            ));
        }
    };
    let (t0, t1) = iso.knots_f64_interim().domain();
    let e = mint();
    solid.edges.insert(
        e,
        EdgeSpec {
            start: starts[i0],
            end: starts[i1],
            carrier: Curve3::Nurbs(Arc::new(iso)),
            t0,
            t1,
            reversed: false,
        },
    );
    // Chain `a` runs Q₀ → Q₁, so the iso edge closes it backwards; `b`
    // runs Q₁ → Q₀ and closes forwards.
    let close = |mut c: Vec<EdgeUse>, forward: bool| {
        c.push(EdgeUse { edge: e, forward });
        vec![LoopSpec {
            outer: true,
            uses: c,
        }]
    };
    let (low_loop, high_loop) = if a_is_low {
        (close(a, false), close(b, true))
    } else {
        (close(b, true), close(a, false))
    };
    let sense = solid.faces[fi].sense;
    solid.faces[fi] = FaceSpec {
        id: face_id,
        surface: Surface::Nurbs(Arc::new(lo)),
        sense,
        loops: low_loop,
        band: false,
    };
    solid.faces.push(FaceSpec {
        id: face_id,
        surface: Surface::Nurbs(Arc::new(hi)),
        sense,
        loops: high_loop,
        band: false,
    });
    Ok(Some(solid.faces.len() - 1))
}

/// Review probe for PR 6013: the side test's nine samples per edge
/// miss a trim that crosses the knot line between two of them, and the
/// face keeps its crease inside its cell.
#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used)]
mod review_6013_side_test {
    use std::collections::{BTreeMap, BTreeSet};
    use std::sync::Arc;

    use geom::{Curve3, NurbsSurface, Surface};
    use geom_core::spline::KnotVector;
    use geom_core::{Point3, Vec3};

    use super::{sides, split_face_once};
    use crate::entities::{EdgeSpec, EdgeUse, FaceSpec, LoopSpec, SolidSpec};

    /// The flat unit square `z = 0`, degree 1 in `u` with a knot at
    /// `u = 1/2` (multiplicity 1 = degree: a crease run by structure),
    /// trimmed by one circle of radius `0.2` about `(0.31, 0.5)`. The
    /// circle reaches `x = 0.51`, past the knot line by `0.01`, but its
    /// nine samples, from `π/8` in steps of `π/4`, reach only
    /// `0.31 + 0.2 cos(π/8) ≈ 0.4948`.
    fn solid() -> SolidSpec {
        let s = NurbsSurface::new(
            KnotVector::clamped(vec![0.0, 0.0, 0.5, 1.0, 1.0], 1).unwrap(),
            KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1).unwrap(),
            [0.0, 0.5, 1.0]
                .into_iter()
                .flat_map(|x| [0.0, 1.0].map(|y| Point3::new(x, y, 0.0)))
                .collect(),
            vec![1.0; 6],
        )
        .unwrap();
        let circle = Curve3::Circle {
            center: Point3::new(0.31, 0.5, 0.0),
            axis: Vec3::new(0.0, 0.0, 1.0),
            radius: 0.2,
            u_ref: Vec3::new(1.0, 0.0, 0.0),
        };
        let t0 = core::f64::consts::FRAC_PI_8;
        let p = circle.eval(t0);
        SolidSpec {
            id: 1,
            faces: vec![FaceSpec {
                id: 10,
                surface: Surface::Nurbs(Arc::new(s)),
                sense: true,
                loops: vec![LoopSpec {
                    outer: true,
                    uses: vec![EdgeUse {
                        edge: 20,
                        forward: true,
                    }],
                }],
                band: false,
            }],
            edges: BTreeMap::from([(
                20,
                EdgeSpec {
                    start: 30,
                    end: 30,
                    carrier: circle,
                    t0,
                    t1: t0 + core::f64::consts::TAU,
                    reversed: false,
                },
            )]),
            vertices: BTreeMap::from([(30, p)]),
            band_seams: BTreeSet::new(),
        }
    }

    #[test]
    fn a_trim_crossing_between_samples_reads_as_one_side_and_keeps_its_crease() {
        let mut solid = solid();
        let Surface::Nurbs(ref s) = solid.faces[0].surface else {
            unreachable!()
        };
        let s = Arc::clone(s);
        // The circle truly reaches past the line.
        let reach = (0..=10_000)
            .map(|i| {
                let t =
                    core::f64::consts::FRAC_PI_8 + core::f64::consts::TAU * f64::from(i) / 10_000.0;
                solid.edges[&20].carrier.eval(t).x
            })
            .fold(f64::NEG_INFINITY, f64::max);
        assert!(reach > 0.509, "the trim crosses u = 1/2: {reach}");
        // The side test reads it below the line only.
        assert_eq!(sides(&solid, 0, &s, 0.5, false, 1e-7), Some((true, false)));
        // So the face is not split and keeps the crease in its cell.
        let mut n = 100;
        let mut mint = || {
            n += 1;
            n
        };
        assert_eq!(
            split_face_once(&mut solid, 0, 1e-7, &mut mint).unwrap(),
            None
        );
    }
}
