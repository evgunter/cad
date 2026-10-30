//! **The split at each interior C⁰ knot** (`docs/DESIGN.md` [topology]:
//! every face chart and edge carrier is C¹ by its knot structure
//! strictly inside its cell, and import splits at its door).
//!
//! A foreign B-spline carrier may hold an interior knot run of
//! multiplicity at least its degree (`Knots::crease_values`).
//! Across that run the spline has no derivative by its structure,
//! whether or not the locus is smooth there, so the door cuts the cell
//! at it:
//!
//! - **an edge carrier** is split at each such knot into its sub-arcs,
//!   a new vertex at the knot's point;
//! - **a face chart** is split along the knot's iso line into its two
//!   sub-patches, the iso curve minted as the edge between them.
//!
//! Both splits are exact. At such a run the net already
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
use geom_core::spline::KnotVector;
use geom_core::{Bounds as _, Interval, Point3};

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
            if !lp.uses.iter().any(|u| u.edge == edge) {
                continue;
            }
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
/// spline interpolates at a crease run.
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
        let knots = carrier.knot_values().crease_values();
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
/// lines that cross it: along `u` first, at every crossing knot of the
/// face, then along `v`, at every crossing knot of each `u` tile. A
/// tile inherits the knots that cross it from the face it was cut from,
/// so each face's boundary is read once per direction.
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
        for along_v in [false, true] {
            let mut tiles = Vec::with_capacity(made.len());
            for f in made {
                let knots = crossing_knots(solid, f, along_v, eps_in);
                split_at_all(solid, f, &knots, along_v, eps_in, mint, &mut tiles)?;
            }
            made = tiles;
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

/// Splits face `f` along each of `knots` (ascending, every one
/// crossing it), halving the run at its middle knot so each piece of
/// the loop is walked once per halving, and pushes every tile.
#[allow(clippy::too_many_arguments)]
fn split_at_all(
    solid: &mut SolidSpec,
    f: usize,
    knots: &[f64],
    along_v: bool,
    eps_in: f64,
    mint: &mut dyn FnMut() -> u64,
    tiles: &mut Vec<usize>,
) -> Result<(), StepImportError> {
    let Some(&knot) = knots.get(knots.len() / 2) else {
        tiles.push(f);
        return Ok(());
    };
    let high = split_face_once(solid, f, knot, along_v, eps_in, mint)?;
    split_at_all(
        solid,
        f,
        &knots[..knots.len() / 2],
        along_v,
        eps_in,
        mint,
        tiles,
    )?;
    split_at_all(
        solid,
        high,
        &knots[knots.len() / 2 + 1..],
        along_v,
        eps_in,
        mint,
        tiles,
    )
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

/// The crease knots of face `f`'s chart along `u` (along `v` where
/// `along_v`) whose iso line crosses the face, ascending: a line
/// outside the face's trimmed domain, or along its boundary, lies inside
/// no cell, and the face keeps its chart there.
///
/// A SELECTION, sound in the direction the rule forbids: a line is left
/// out only where the face's boundary is certified clear of it on one
/// side ([`clear_of`]). The boundary's projected samples are evidence
/// alone ([`feet`]): a line they see the boundary reach past on both
/// sides crosses, and a side they do not see reached is certified
/// clear before the line is left out. A line neither seen crossing nor
/// certified clear is kept, and its split places it or refuses typed.
fn crossing_knots(solid: &SolidSpec, f: usize, along_v: bool, eps_in: f64) -> Vec<f64> {
    let Surface::Nurbs(ref s) = solid.faces[f].surface else {
        return Vec::new();
    };
    let knots = if along_v {
        s.knot_values_v().crease_values()
    } else {
        s.knot_values_u().crease_values()
    };
    if knots.is_empty() {
        return knots;
    }
    let feet = feet(solid, f, s);
    let lifted = s.map_scalar(|x| Interval::from_bounds(x, x));
    knots
        .into_iter()
        .filter(|&knot| {
            let (mut below, mut above) = (false, false);
            for foot in &feet {
                let (c, speed) = if along_v {
                    (foot[1], foot[3])
                } else {
                    (foot[0], foot[2])
                };
                let reach = eps_in / speed;
                below |= c < knot - reach;
                above |= c > knot + reach;
            }
            let clear = |high| clear_of(solid, f, s, &lifted, knot, along_v, high, eps_in);
            !((!below && clear(false)) || (!above && clear(true)))
        })
        .collect()
}

/// Face `f`'s boundary samples on its chart `s`: each edge at nine
/// points, each read by projection as `[u, v, |S_u|, |S_v|]` at its
/// foot. A sample whose projection fails is no evidence, and is left
/// out.
fn feet(solid: &SolidSpec, f: usize, s: &NurbsSurface<f64>) -> Vec<[f64; 4]> {
    let mut out = Vec::new();
    for u in solid.faces[f].loops.iter().flat_map(|l| &l.uses) {
        let spec = &solid.edges[&u.edge];
        for i in 0..=8 {
            let t = spec.t0 + (spec.t1 - spec.t0) * f64::from(i) / 8.0;
            let Ok(foot) = s.project(spec.carrier.eval(t)) else {
                continue;
            };
            let jet = s.ders(foot.u, foot.v);
            out.push([foot.u, foot.v, jet.du.norm(), jet.dv.norm()]);
        }
    }
    out
}

/// The enclosures [`clear_of`] may form for one side of one line
/// before it answers "not certified": a constant of the selection,
/// spent where the boundary meets the region or runs along its rim.
const CLEAR_BUDGET: usize = 4096;

/// The deepest [`clear_of`] halves an edge piece.
const CLEAR_DEPTH: u32 = 40;

/// A parameter rectangle of the chart and an enclosure of its image.
#[derive(Clone, Copy)]
struct Cell {
    u: (f64, f64),
    v: (f64, f64),
    image: [Interval; 3],
}

/// Whether face `f`'s boundary is CERTIFIED clear of the chart past the
/// line `knot` on one side: above it where `high`, below it otherwise.
///
/// The bound that holds where it answers `true`: every boundary point
/// either lies within `ε_in` of the iso line ([`on_the_line`]), or has
/// its foot on the chart on the near side of the line or within `ε_in`
/// of it in space, where the boundary stands within `ε_in` of its chart.
/// The region is the chart's parameter rectangle on the far side, set
/// back from the line by `ε_in` over the chart's speed bound across it,
/// so every chart point the setback leaves out is within `ε_in` of the
/// line. An edge whose every point is farther than `ε_in` from the
/// region's image has no foot in it. Read by enclosure alone: the
/// region's image cell by cell ([`NurbsSurface::rect_enclosure`]), and
/// each edge piece's box ([`piece_box`]), the two halved together where
/// they meet, until they part or [`CLEAR_BUDGET`] is spent. What it
/// cannot certify, the boundary running along the line among it, is
/// left to the split.
#[allow(clippy::too_many_arguments)]
fn clear_of(
    solid: &SolidSpec,
    f: usize,
    s: &NurbsSurface<f64>,
    lifted: &NurbsSurface<Interval>,
    knot: f64,
    along_v: bool,
    high: bool,
    eps_in: f64,
) -> bool {
    let (sup_u, sup_v) = s.stretch_sup();
    let speed = if along_v { sup_v.get() } else { sup_u.get() };
    let setback = eps_in / speed;
    if !setback.is_finite() {
        return false;
    }
    let (du, dv) = (s.knots_u_f64_interim(), s.knots_v_f64_interim());
    let across = if along_v { dv } else { du };
    let (lo, hi) = across.domain();
    let side = if high {
        (knot + setback, hi)
    } else {
        (lo, knot - setback)
    };
    if side.0 >= side.1 {
        return true;
    }
    // The region's knot cells: an enclosure is tightest inside one.
    let breaks = |kv: &KnotVector, (a, b): (f64, f64)| -> Vec<f64> {
        let mut out = vec![a];
        out.extend(kv.knot_runs().map(|(k, _)| k).filter(|&k| a < k && k < b));
        out.push(b);
        out
    };
    let (ru, rv) = if along_v {
        (du.domain(), side)
    } else {
        (side, dv.domain())
    };
    let mut budget = CLEAR_BUDGET;
    let mut cell = |u: (f64, f64), v: (f64, f64)| -> Option<Cell> {
        budget = budget.checked_sub(1)?;
        let p = lifted.rect_enclosure(u, v);
        let image = [p.x, p.y, p.z];
        image
            .iter()
            .all(|c| c.lo() <= c.hi())
            .then_some(Cell { u, v, image })
    };
    // A box within `ε_in` of the line at every point: its centre's
    // distance from a point of the line, plus its half diagonal.
    let near = |bx: &[Interval; 3]| {
        let mid = |c: &Interval| 0.5 * (c.lo() + c.hi());
        let centre = Point3::new(mid(&bx[0]), mid(&bx[1]), mid(&bx[2]));
        let half = worst(bx.iter().map(|c| 0.5 * (c.hi() - c.lo())));
        let Ok(foot) = s.project(centre) else {
            return false;
        };
        let on = if along_v {
            s.eval(foot.u, knot)
        } else {
            s.eval(knot, foot.v)
        };
        (centre - on).norm() + half * 3f64.sqrt() <= eps_in
    };
    let (bu, bv) = (breaks(du, ru), breaks(dv, rv));
    let mut cells = Vec::new();
    for u in bu.windows(2) {
        for v in bv.windows(2) {
            let Some(c) = cell((u[0], u[1]), (v[0], v[1])) else {
                return false;
            };
            cells.push(c);
        }
    }
    solid.faces[f].loops.iter().flat_map(|l| &l.uses).all(|u| {
        let spec = &solid.edges[&u.edge];
        if on_the_line(spec, s, knot, along_v, eps_in) {
            return true;
        }
        let lifted_edge = match spec.carrier {
            Curve3::Nurbs(ref c) => Some(c.map_scalar(|x| Interval::from_bounds(x, x))),
            _ => None,
        };
        let piece = |a: f64, b: f64| piece_box(&spec.carrier, lifted_edge.as_ref(), a, b);
        parts(
            &piece,
            &near,
            (spec.t0, spec.t1),
            cells.clone(),
            eps_in,
            &mut cell,
            0,
        )
    })
}

/// Whether `spec` lies within `eps_in` of the iso line `knot` of the
/// chart `s` (along `v` where `along_v`) at every point, by structure:
/// the line is the net's row (column) the spline interpolates at the
/// knot, and where that row's control points lie within `d` of the chord
/// between its ends, the line runs within `d` of every chord point (its
/// foot on the chord runs from one end to the other). An edge whose
/// convex hull (a line's two ends, a spline's control points over its
/// whole domain) lies within `e` of the chord then lies within `e + d`
/// of the line. `false` for a curved line or any other edge.
fn on_the_line(
    spec: &EdgeSpec,
    s: &NurbsSurface<f64>,
    knot: f64,
    along_v: bool,
    eps_in: f64,
) -> bool {
    let kv = if along_v {
        s.knots_v_f64_interim()
    } else {
        s.knots_u_f64_interim()
    };
    let Some(first) = kv.knots().iter().position(|&x| x == knot) else {
        return false;
    };
    let (nu, nv) = s.control_counts();
    let control = s.control();
    let row: Vec<Point3<f64>> = if along_v {
        (0..nu).map(|i| control[i * nv + first - 1]).collect()
    } else {
        control[(first - 1) * nv..first * nv].to_vec()
    };
    let (Some(&a), Some(&b)) = (row.first(), row.last()) else {
        return false;
    };
    let chord = b - a;
    let off = |p: Point3<f64>| {
        let t = ((p - a).dot(chord) / chord.dot(chord)).clamp(0.0, 1.0);
        (p - (a + chord * t)).norm()
    };
    let bent = worst(row.iter().map(|&p| off(p)));
    let hull: Vec<Point3<f64>> = match spec.carrier {
        Curve3::Line { .. } => vec![spec.carrier.eval(spec.t0), spec.carrier.eval(spec.t1)],
        Curve3::Nurbs(ref c) if c.knots_f64_interim().domain() == (spec.t0, spec.t1) => {
            c.control().to_vec()
        }
        _ => return false,
    };
    worst(hull.iter().map(|&p| off(p))) + bent <= eps_in
}

/// The largest of `xs`, NaN where any is: a poisoned reading certifies
/// nothing.
fn worst(xs: impl Iterator<Item = f64>) -> f64 {
    xs.fold(0.0, |m, x| if x.is_nan() { f64::NAN } else { m.max(x) })
}

/// Whether the edge pieces over `t` keep farther than `pad` from every
/// one of `cells`, or lie within `ε_in` of the line (`near`), halving a
/// piece and quartering each cell it meets until one holds; `false` where a piece has no box, the budget `cell`
/// spends runs out, or the piece is [`CLEAR_DEPTH`] halvings deep.
fn parts(
    piece: &dyn Fn(f64, f64) -> Option<[Interval; 3]>,
    near: &dyn Fn(&[Interval; 3]) -> bool,
    (a, b): (f64, f64),
    cells: Vec<Cell>,
    pad: f64,
    cell: &mut dyn FnMut((f64, f64), (f64, f64)) -> Option<Cell>,
    depth: u32,
) -> bool {
    let Some(bx) = piece(a, b) else {
        return false;
    };
    let meets = |c: &Cell| {
        c.image
            .iter()
            .zip(&bx)
            .all(|(i, e)| i.lo() - pad <= e.hi() && e.lo() <= i.hi() + pad)
    };
    let hit: Vec<Cell> = cells.into_iter().filter(meets).collect();
    if hit.is_empty() || near(&bx) {
        return true;
    }
    if depth == CLEAR_DEPTH {
        return false;
    }
    let mut finer = Vec::with_capacity(4 * hit.len());
    for c in &hit {
        let (um, vm) = (0.5 * (c.u.0 + c.u.1), 0.5 * (c.v.0 + c.v.1));
        for u in [(c.u.0, um), (um, c.u.1)] {
            for v in [(c.v.0, vm), (vm, c.v.1)] {
                let Some(q) = cell(u, v) else {
                    return false;
                };
                finer.push(q);
            }
        }
    }
    let m = 0.5 * (a + b);
    parts(piece, near, (a, m), finer.clone(), pad, cell, depth + 1)
        && parts(piece, near, (m, b), finer, pad, cell, depth + 1)
}

/// An enclosure of `carrier` over `[a, b]`: a line's two ends, a conic
/// arc's certified box ([`geom::curves::boxes::conic_arc_aabb`]), a
/// spline's restricted control hull (`lifted`, its net as intervals).
/// `None` for any other carrier, or a box the door poisons.
fn piece_box(
    carrier: &Curve3<f64>,
    lifted: Option<&NurbsCurve3<Interval>>,
    a: f64,
    b: f64,
) -> Option<[Interval; 3]> {
    let hull = |lo: [f64; 3], hi: [f64; 3]| -> Option<[Interval; 3]> {
        let out = [0, 1, 2].map(|i| Interval::from_bounds(lo[i], hi[i]));
        out.iter().all(|c| c.lo() <= c.hi()).then_some(out)
    };
    match carrier {
        Curve3::Nurbs(_) => {
            let p = lifted?.span_enclosure(a, b)?;
            let out = [p.x, p.y, p.z];
            out.iter().all(|c| c.lo() <= c.hi()).then_some(out)
        }
        Curve3::Line { .. } => {
            let (p, q) = (carrier.eval(a), carrier.eval(b));
            // Two rounded evaluations: pad each by a few ulps of its
            // size, outward.
            let ulp = |x: f64| 4.0 * f64::EPSILON * x.abs().max(f64::MIN_POSITIVE);
            let lo = [p.x.min(q.x), p.y.min(q.y), p.z.min(q.z)];
            let hi = [p.x.max(q.x), p.y.max(q.y), p.z.max(q.z)];
            hull(lo.map(|x| x - ulp(x)), hi.map(|x| x + ulp(x)))
        }
        _ => {
            let bx = geom::curves::boxes::conic_arc_aabb(
                carrier,
                a,
                b,
                carrier.eval(a),
                carrier.eval(b),
            )?;
            hull(
                [bx.min_x, bx.min_y, bx.min_z],
                [bx.max_x, bx.max_y, bx.max_z],
            )
        }
    }
}

/// The vertex a use starts at.
fn use_start(solid: &SolidSpec, u: EdgeUse) -> u64 {
    let spec = &solid.edges[&u.edge];
    if u.forward { spec.start } else { spec.end }
}

/// Splits face `fi` once, along the iso line of its crease knot `knot`
/// (along `v` where `along_v`), which crosses it ([`crossing_knots`]),
/// and answers the index of the new face: the low sub-patch stays at
/// `fi`, the high one is pushed.
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
    knot: f64,
    along_v: bool,
    eps_in: f64,
    mint: &mut dyn FnMut() -> u64,
) -> Result<usize, StepImportError> {
    let Surface::Nurbs(ref s) = solid.faces[fi].surface else {
        unreachable!("only a B-spline chart has crease knots to split at")
    };
    let s = Arc::clone(s);
    let face_id = solid.faces[fi].id;
    let refuse = |knot: f64, what: &'static str| StepImportError::C0KnotUnsplit {
        face: face_id,
        knot,
        what,
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
    Ok(solid.faces.len() - 1)
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used)]
mod tests {
    use std::collections::{BTreeMap, BTreeSet};
    use std::sync::Arc;

    use geom::{Curve3, NurbsSurface, Surface};
    use geom_core::spline::KnotVector;
    use geom_core::{Point3, Vec3};

    use super::{crossing_knots, split_faces};
    use crate::entities::{EdgeSpec, EdgeUse, FaceSpec, LoopSpec, SolidSpec};
    use crate::error::StepImportError;

    const EPS_IN: f64 = 1e-7;

    /// The flat unit square `z = 0`, degree 1 in `u` with a knot at
    /// `u = 1/2` (multiplicity 1 = degree: a crease run by structure),
    /// trimmed by one circle of radius `0.2` about `(cx, 0.5)`.
    fn solid(cx: f64) -> SolidSpec {
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
            center: Point3::new(cx, 0.5, 0.0),
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

    /// The square's left half, `[0, 1/2] × [0, 1]`, bounded by four
    /// lines, its right side on the knot line: one face of a surface an
    /// exporter split at its knot. The line lies along its boundary, in
    /// no cell, and the face keeps its chart.
    #[test]
    fn a_face_bounded_along_the_knot_line_keeps_its_chart() {
        let mut solid = solid(0.25);
        let corners =
            [(0.0, 0.0), (0.5, 0.0), (0.5, 1.0), (0.0, 1.0)].map(|(x, y)| Point3::new(x, y, 0.0));
        solid.vertices = (0..4).map(|i| (30 + i, corners[i as usize])).collect();
        solid.edges = (0..4u64)
            .map(|i| {
                let (a, b) = (corners[i as usize], corners[(i as usize + 1) % 4]);
                let spec = EdgeSpec {
                    start: 30 + i,
                    end: 30 + (i + 1) % 4,
                    carrier: Curve3::Line {
                        origin: a,
                        dir: b - a,
                    },
                    t0: 0.0,
                    t1: 1.0,
                    reversed: false,
                };
                (20 + i, spec)
            })
            .collect();
        solid.faces[0].loops[0].uses = (0..4)
            .map(|i| EdgeUse {
                edge: 20 + i,
                forward: true,
            })
            .collect();
        assert_eq!(crossing_knots(&solid, 0, false, EPS_IN), Vec::<f64>::new());
    }

    /// A circle about `(0.31, 0.5)` reaches `x = 0.51`, past the knot
    /// line by `0.01`, but its nine samples, from `π/8` in steps of
    /// `π/4`, reach only `0.31 + 0.2 cos(π/8) ≈ 0.4948`: the samples see
    /// one side, nothing certifies the other clear, so the line is kept
    /// and its split, which this trim cannot place, refuses typed.
    #[test]
    fn a_trim_crossing_between_samples_is_kept_and_refuses_typed() {
        let mut solid = solid(0.31);
        assert_eq!(crossing_knots(&solid, 0, false, EPS_IN), vec![0.5]);
        let mut n = 100;
        let mut mint = || {
            n += 1;
            n
        };
        let refused = split_faces(&mut solid, EPS_IN, &mut mint, &mut Vec::new());
        assert!(
            matches!(
                refused,
                Err(StepImportError::C0KnotUnsplit { face: 10, knot, .. }) if knot == 0.5
            ),
            "the uncertified line refuses typed: {refused:?}"
        );
    }

    /// A circle about `(0.3 + 2ε_in, 0.5)` crosses the line by `2ε_in`,
    /// between samples: farther than the `ε_in` the import may move a
    /// boundary, so it is kept, and its split refuses typed rather than
    /// absorb the crossing as on the line.
    #[test]
    fn a_trim_crossing_by_twice_eps_in_is_kept_and_refuses_typed() {
        let mut solid = solid(0.3 + 2.0 * EPS_IN);
        assert_eq!(crossing_knots(&solid, 0, false, EPS_IN), vec![0.5]);
        let mut n = 100;
        let mut mint = || {
            n += 1;
            n
        };
        let refused = split_faces(&mut solid, EPS_IN, &mut mint, &mut Vec::new());
        assert!(
            matches!(
                refused,
                Err(StepImportError::C0KnotUnsplit { face: 10, .. })
            ),
            "the crossing refuses typed: {refused:?}"
        );
    }

    /// The same circle about `(0.25, 0.5)` stops at `x = 0.45`, short of
    /// the line by `0.05`: the side the samples do not see is certified
    /// clear, and the face keeps its chart.
    #[test]
    fn a_trim_clear_of_the_line_is_certified_and_kept_whole() {
        let mut solid = solid(0.25);
        assert_eq!(crossing_knots(&solid, 0, false, EPS_IN), Vec::<f64>::new());
        let mut n = 100;
        let mut mint = || {
            n += 1;
            n
        };
        let mut sink = Vec::new();
        split_faces(&mut solid, EPS_IN, &mut mint, &mut sink).unwrap();
        assert_eq!((solid.faces.len(), sink.len()), (1, 0));
    }

    /// Review probes (6013 delta): the side test against crossings
    /// built between the samples. Each `kept` row asserts the line is
    /// never left out; each `info` row prints what happens.
    mod review_probes {
        use std::collections::{BTreeMap, BTreeSet};
        use std::sync::Arc;

        use geom::{Curve3, NurbsCurve3, NurbsSurface, Surface};
        use geom_core::spline::KnotVector;
        use geom_core::{Point3, Vec3};

        use super::super::crossing_knots;
        use super::EPS_IN;
        use crate::entities::{EdgeSpec, EdgeUse, FaceSpec, LoopSpec, SolidSpec};

        /// The plane `z = 0` charted degree 1 in `u` with a crease at
        /// `u = 1/2`; `xs` are the three control columns' `x`.
        fn chart(xs: [f64; 3]) -> NurbsSurface<f64> {
            NurbsSurface::new(
                KnotVector::clamped(vec![0.0, 0.0, 0.5, 1.0, 1.0], 1).unwrap(),
                KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1).unwrap(),
                xs.into_iter()
                    .flat_map(|x| [0.0, 1.0].map(|y| Point3::new(x, y, 0.0)))
                    .collect(),
                vec![1.0; 6],
            )
            .unwrap()
        }

        fn face(s: NurbsSurface<f64>, edges: Vec<(Curve3<f64>, f64, f64)>) -> SolidSpec {
            let n = edges.len() as u64;
            let mut vertices = BTreeMap::new();
            let mut map = BTreeMap::new();
            for (i, (c, t0, t1)) in edges.into_iter().enumerate() {
                let i = i as u64;
                vertices.insert(30 + i, c.eval(t0));
                map.insert(
                    20 + i,
                    EdgeSpec {
                        start: 30 + i,
                        end: 30 + (i + 1) % n,
                        carrier: c,
                        t0,
                        t1,
                        reversed: false,
                    },
                );
            }
            SolidSpec {
                id: 1,
                faces: vec![FaceSpec {
                    id: 10,
                    surface: Surface::Nurbs(Arc::new(s)),
                    sense: true,
                    loops: vec![LoopSpec {
                        outer: true,
                        uses: (0..n)
                            .map(|i| EdgeUse {
                                edge: 20 + i,
                                forward: true,
                            })
                            .collect(),
                    }],
                    band: false,
                }],
                edges: map,
                vertices,
                band_seams: BTreeSet::new(),
            }
        }

        fn circle(cx: f64, cy: f64, r: f64) -> Curve3<f64> {
            Curve3::Circle {
                center: Point3::new(cx, cy, 0.0),
                axis: Vec3::new(0.0, 0.0, 1.0),
                radius: r,
                u_ref: Vec3::new(1.0, 0.0, 0.0),
            }
        }

        /// Circles whose rightmost point passes the line `x = xk` by
        /// `δ`, the samples phased to straddle that point, at several
        /// heights (one near the chart's `v = 0` edge) and radii.
        fn circles(xs: [f64; 3], xk: f64) {
            let mut bad = Vec::new();
            for &r in &[0.2, 0.01, 1e-4] {
                for &cy in &[0.5, r + 1e-3, 1.0 - r - 1e-6] {
                    for &k in &[1.0001, 1.01, 1.5, 2.0, 5.0, 100.0] {
                        for &phase in &[core::f64::consts::FRAC_PI_8, 0.01, 0.39, 0.2] {
                            let d = k * EPS_IN;
                            let c = circle(xk + d - r, cy, r);
                            let s =
                                face(chart(xs), vec![(c, phase, phase + core::f64::consts::TAU)]);
                            if crossing_knots(&s, 0, false, EPS_IN) != vec![0.5] {
                                bad.push((r, cy, k, phase));
                            }
                        }
                    }
                }
            }
            assert!(bad.is_empty(), "left out though crossed: {bad:?}");
        }

        #[test]
        fn kept_unit_chart() {
            circles([0.0, 0.5, 1.0], 0.5);
        }

        /// The far side stretched 199×: the setback is ε_in/199.
        #[test]
        fn kept_fast_stretching_far_side() {
            circles([0.0, 0.5, 100.0], 0.5);
        }

        /// The near side stretched instead: the circle lives on a slow
        /// chart and crosses into a fast one.
        #[test]
        fn kept_fast_stretching_near_side() {
            circles([-100.0, 0.5, 1.0], 0.5);
        }

        /// Two arcs of one circle meeting at a vertex just below the
        /// line; the crossing tip sits `1e-3` rad past the vertex.
        #[test]
        fn kept_crossing_beside_a_vertex() {
            let mut bad = Vec::new();
            for &k in &[1.01, 2.0, 10.0] {
                for &a in &[1e-3, 1e-2, 0.05] {
                    let r = 0.2;
                    let c = circle(0.5 + k * EPS_IN - r, 0.5, r);
                    let s = face(
                        chart([0.0, 0.5, 1.0]),
                        vec![(c.clone(), -a, 1.0), (c, 1.0, core::f64::consts::TAU - a)],
                    );
                    if crossing_knots(&s, 0, false, EPS_IN) != vec![0.5] {
                        bad.push((k, a));
                    }
                }
            }
            assert!(bad.is_empty(), "left out though crossed: {bad:?}");
        }

        /// A quadratic B-spline trim (a rounded diamond) whose tip
        /// passes the line between its samples.
        #[test]
        fn kept_spline_trim() {
            let mut bad = Vec::new();
            for &k in &[1.01, 2.0, 10.0] {
                for &tip in &[0.37, 0.5, 0.63] {
                    let d = k * EPS_IN;
                    // A closed degree-2 curve: control polygon a diamond,
                    // tip control point placed so the curve's maximum x
                    // (at the tip's span midpoint) is 0.5 + d.
                    // For a uniform quadratic, the point at a control
                    // point's peak is (P_prev + 6 P + P_next)/8 at the
                    // knot... use an open clamped curve through (0.3,0)
                    // to (0.3,1) with the tip in between.
                    let build = |px: f64| {
                        let ctrl = vec![
                            Point3::new(0.3, 0.2, 0.0),
                            Point3::new(0.3, tip - 0.1, 0.0),
                            Point3::new(px, tip, 0.0),
                            Point3::new(0.3, tip + 0.1, 0.0),
                            Point3::new(0.3, 0.8, 0.0),
                        ];
                        let knots =
                            KnotVector::clamped(vec![0.0, 0.0, 0.0, 0.25, 0.5, 1.0, 1.0, 1.0], 2)
                                .unwrap();
                        NurbsCurve3::new(knots, ctrl, vec![1.0; 5]).unwrap()
                    };
                    let max_x = |c: &NurbsCurve3<f64>| {
                        (0..=200_000)
                            .map(|i| c.eval(f64::from(i) / 200_000.0).x)
                            .fold(f64::MIN, f64::max)
                    };
                    // Bisect the tip's control x so the curve peaks at 0.5 + d.
                    let (mut lo, mut hi) = (0.3, 2.0);
                    for _ in 0..80 {
                        let m = 0.5 * (lo + hi);
                        if max_x(&build(m)) < 0.5 + d {
                            lo = m
                        } else {
                            hi = m
                        }
                    }
                    let spl = build(hi);
                    let max_x = max_x(&spl);
                    let back = Curve3::Line {
                        origin: Point3::new(0.3, 0.8, 0.0),
                        dir: Vec3::new(0.0, -0.6, 0.0),
                    };
                    let s = face(
                        chart([0.0, 0.5, 1.0]),
                        vec![(Curve3::Nurbs(Arc::new(spl)), 0.0, 1.0), (back, 0.0, 1.0)],
                    );
                    let got = crossing_knots(&s, 0, false, EPS_IN);
                    eprintln!(
                        "spline k={k} tip={tip} max_x-0.5={:e} -> {got:?}",
                        max_x - 0.5
                    );
                    if max_x - 0.5 > EPS_IN && got != vec![0.5] {
                        bad.push((k, tip));
                    }
                }
            }
            assert!(bad.is_empty(), "left out though crossed: {bad:?}");
        }

        /// Trims that touch, stop short of, or poke past the line by less
        /// than ε_in. A `[0.5]` here is a face kept for the split, which
        /// refuses `C0KnotUnsplit` though the crossing is within ε_in.
        #[test]
        fn pinned_near_misses() {
            for &k in &[-1e5, -100.0, -2.0, -1.0, -0.5, 0.0, 0.5, 0.999] {
                for &r in &[0.2, 1e-3] {
                    let c = circle(0.5 + k * EPS_IN - r, 0.5, r);
                    let s = face(
                        chart([0.0, 0.5, 1.0]),
                        vec![(
                            c,
                            core::f64::consts::FRAC_PI_8,
                            core::f64::consts::FRAC_PI_8 + core::f64::consts::TAU,
                        )],
                    );
                    let t = std::time::Instant::now();
                    let got = crossing_knots(&s, 0, false, EPS_IN);
                    eprintln!("near-miss k={k} r={r} -> {got:?} in {:?}", t.elapsed());
                    // Pinned as observed at e5c46713: a trim stopping short
                    // or touching certifies clear; a large trim poking past
                    // the STRAIGHT line by less than ε_in is uncertified,
                    // kept, and so refused at the split.
                    if k <= 0.0 {
                        assert_eq!(got, Vec::<f64>::new(), "k={k} r={r}");
                    }
                    if r == 0.2 && k > 0.0 {
                        assert_eq!(got, vec![0.5], "k={k} r={r}");
                    }
                }
            }
        }
    }
}
