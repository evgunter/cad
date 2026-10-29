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
//! silently. A face whose trim the split cannot place
//! ([`split_face_once`]), and an edge knot whose vertex the closing join
//! would take back ([`split_c0_knots`]), refuse typed rather than ship
//! the crease.

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
    let knot_vertices = split_edges(solid, mint, sink)?;
    split_faces(solid, eps_in, mint, sink)?;
    // A knot vertex no face's knot line reached joins only the two
    // sub-arcs it parts. The import's closing join reads such a vertex
    // as one edge cut for no reason and cannot run either sub-arc on
    // through it, so it refuses here, typed, rather than there.
    for (v, face, knot) in knot_vertices {
        let valence = solid
            .edges
            .values()
            .map(|e| usize::from(e.start == v) + usize::from(e.end == v))
            .sum::<usize>();
        if valence == 2 {
            return Err(StepImportError::C0KnotUnsplit {
                face,
                knot,
                what: "an edge carrier's C⁰ knot is a vertex no other edge meets, which the \
                       import's closing join would take back",
            });
        }
    }
    Ok(())
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
) -> Result<Vec<(u64, u64, f64)>, StepImportError> {
    let mut knot_vertices = Vec::new();
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
        for (arc, &k) in arcs.iter().zip(&knots) {
            let v = mint();
            knot_vertices.push((v, face, k));
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
    Ok(knot_vertices)
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
    /// The knot, in the split direction's parameter.
    knot: f64,
    /// The child below the knot.
    lo: NurbsSurface<f64>,
    /// The child above it.
    hi: NurbsSurface<f64>,
    /// Whether the split ran along `v`.
    along_v: bool,
}

/// The chart's first crease knot, along `u` before `v`, with the chart
/// split there; `Err(knot)` where the partition refused.
fn first_split(s: &NurbsSurface<f64>) -> Option<Result<NetSplit, f64>> {
    if let Some(&knot) = crease_knots(s.knots_u_f64_interim()).first() {
        return Some(
            split_net_u(s, knot)
                .map(|(lo, hi)| NetSplit {
                    knot,
                    lo,
                    hi,
                    along_v: false,
                })
                .ok_or(knot),
        );
    }
    let t = s.transposed();
    let &knot = crease_knots(t.knots_u_f64_interim()).first()?;
    Some(
        split_net_u(&t, knot)
            .map(|(lo, hi)| NetSplit {
                knot,
                lo: lo.transposed(),
                hi: hi.transposed(),
                along_v: true,
            })
            .ok_or(knot),
    )
}

/// The vertex a use starts at.
fn use_start(solid: &SolidSpec, u: EdgeUse) -> u64 {
    let spec = &solid.edges[&u.edge];
    if u.forward { spec.start } else { spec.end }
}

/// Splits face `fi` once, along its first crease knot's iso line, and
/// answers the index of the new face, or `None` where its chart holds
/// no crease.
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
/// (`work/exch/the-c0-knot-split-refuses-what-it-cannot-place.md`).
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
    let Some(split) = first_split(&s) else {
        return Ok(None);
    };
    let refuse = |knot: f64, what: &'static str| StepImportError::C0KnotUnsplit {
        face: face_id,
        knot,
        what,
    };
    let NetSplit {
        knot,
        lo,
        hi,
        along_v,
    } = split.map_err(|k| refuse(k, "the chart refused its split at its own crease knot"))?;
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
                 sub-patch (a trim that crosses the line elsewhere, or a line along the \
                 boundary)",
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
