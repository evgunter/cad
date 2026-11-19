//! **The split at each interior C⁰ knot** (`docs/DESIGN.md` `[topology]`:
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
//! - **a face chart** is split along the knot's iso line into the
//!   pieces the line parts the face into, each on its side's sub-patch,
//!   each run of the line inside the face minted as an edge between
//!   them.
//!
//! Both splits are exact. At such a run the net already
//! separates the two sub-nets: each child is a partition of the
//! parent's knots, control points and weights, sharing the row (or the
//! point) the spline interpolates at the knot, and no arithmetic is
//! done. A minted run of the line is carried by that shared row, so its
//! carrier is bitwise both sub-patches' boundary column.
//!
//! Every split that changes the census, an edge into sub-arcs or a face
//! into pieces, is reported as a [`StructureNormalization`], never done
//! silently. A face the knot line leaves whole has its chart narrowed to
//! the sub-patch on its side: an exact partition of the net that moves
//! no point and changes no census, so it is not reported. A knot vertex is one the closing join keeps
//! (`topo`'s `joinable`): two spline pieces meeting at a knot of full
//! multiplicity are never joined. A face whose trim the split cannot
//! place ([`split_face_once`]) refuses typed rather than ship the
//! crease.

use std::collections::{BTreeMap, BTreeSet};
use std::sync::Arc;

use geom::{Curve3, NurbsCurve3, NurbsSurface, Surface};
use geom_brep::curve_pair::{self, Hit, Span, SpanEnd};
use geom_brep::{Pcurve, PcurveCache};
use geom_core::spline::{KnotVector, Knots};
use geom_core::{Band, Bounds as _, Interval, Point3, Vec3};

use crate::entities::{EdgeSpec, EdgeUse, FaceSpec, LoopSpec, SolidSpec};
use crate::error::StepImportError;
use crate::interpret::FileBand;
use crate::normalize::surgery::{
    self, At, CutLine, Place, Split, boundary_census, split_edge, use_start,
};
use crate::{FaceCensus, NormalizationKind, StructureNormalization};

/// Runs both splits over the shell: edges first, so a face's knot line
/// finds the vertices its boundary edges' own splits placed. `band` is
/// the file's chart band (`crate::chart::file_band`), which a face
/// split reads the knot line's crossings with the trim at.
pub(crate) fn split_c0_knots(
    solid: &mut SolidSpec,
    file: FileBand,
    mint: &mut dyn FnMut() -> u64,
    sink: &mut Vec<StructureNormalization>,
) -> Result<(), StepImportError> {
    split_edges(solid, mint, sink)?;
    split_faces(solid, file, mint, sink)
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
        let at: Vec<(f64, Place)> = knots.iter().map(|&k| (k, Place::Joint)).collect();
        let Split { pieces, .. } =
            split_edge(solid, id, &at, mint).ok_or(StepImportError::Topology {
                id,
                what: "internal: a B-spline carrier refused its split at its own crease knot",
            })?;
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

/// Splits every B-spline face chart along each of its crease knots' iso
/// lines that cross it: along `u` first, at every crossing knot of the
/// face, then along `v`, at every crossing knot of each `u` tile. A
/// tile inherits the knots that cross it from the face it was cut from,
/// so each face's boundary is read once per direction.
fn split_faces(
    solid: &mut SolidSpec,
    file: FileBand,
    mint: &mut dyn FnMut() -> u64,
    sink: &mut Vec<StructureNormalization>,
) -> Result<(), StepImportError> {
    let (eps_in, band) = (file.eps_in(), Some(file.band()));
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
                let at = Lines {
                    along_v,
                    eps_in,
                    band,
                    file,
                };
                split_at_all(solid, f, &knots, at, mint, &mut tiles)?;
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

/// How one direction's knot lines are read: along `v` where `along_v`,
/// at the file's `eps_in` and its chart band.
#[derive(Clone, Copy)]
struct Lines {
    along_v: bool,
    eps_in: f64,
    band: Option<Band>,
    /// The file band the two above are read off, for a reading that
    /// takes it whole (the iso line's recognition).
    file: FileBand,
}

/// Splits face `f` along each of `knots` (ascending, each kept by
/// [`crossing_knots`]), halving the run at its middle knot so each piece
/// of the loop is walked once per halving, and pushes every tile. A
/// piece below the middle knot takes the knots below it, and a piece
/// above it the knots above.
fn split_at_all(
    solid: &mut SolidSpec,
    f: usize,
    knots: &[f64],
    at: Lines,
    mint: &mut dyn FnMut() -> u64,
    tiles: &mut Vec<usize>,
) -> Result<(), StepImportError> {
    let mid = knots.len() / 2;
    let Some(&knot) = knots.get(mid) else {
        tiles.push(f);
        return Ok(());
    };
    let (below, above) = (&knots[..mid], &knots[mid + 1..]);
    let Parted { low, high } = split_face_once(solid, f, knot, at, mint)?;
    for g in low {
        split_at_all(solid, g, below, at, mint, tiles)?;
    }
    for g in high {
        split_at_all(solid, g, above, at, mint, tiles)?;
    }
    Ok(())
}

/// A clamped net's two children at the crease knot `k` along `u`: the
/// knots below `k` with `k` closing the low child at `p + 1`, the knots
/// from `k` with `k` opening the high child at `p + 1`, and the rows
/// each child spans, the row interpolated at `k` shared. A partition,
/// so exact.
fn split_net_u(s: &NurbsSurface<f64>, k: f64) -> Option<(NurbsSurface<f64>, NurbsSurface<f64>)> {
    let kv = s.knot_values_u();
    let (p, knots) = (kv.degree(), kv.values());
    let first = knots.iter().position(|&x| x == k)?;
    let (_, nv) = s.control_counts();
    let (control, weights) = (s.control(), s.weights());
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
/// It bounds the halving too: each halving that does not settle a piece
/// quarters at least one cell.
const CLEAR_BUDGET: usize = 4096;

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
///
/// The premise the inflation rests on, unchecked here: every boundary
/// edge lies within `ε_in` of the face's chart, as the file states and
/// tier 3 certifies only at rest
/// (`work/importgate/import-splits-knot-lines-on-an-unchecked-incidence-premise.md`).
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
    let (sup_u, sup_v) = lifted.stretch_sup();
    let speed = if along_v { sup_v.get() } else { sup_u.get() }.hi();
    let setback = eps_in / speed;
    if !setback.is_finite() {
        return false;
    }
    let (du, dv) = (s.knot_values_u(), s.knot_values_v());
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
    let breaks = |kv: &Knots<f64>, (a, b): (f64, f64)| -> Vec<f64> {
        let mut out = vec![a];
        out.extend(kv.breakpoints().filter(|&k| a < k && k < b));
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
    // A box within `ε_in` of the line at every point, read two ways.
    // Across a chord: the line runs within `bent` of its chord (see
    // `on_the_line`), and a box's distance from the chord, convex, is
    // greatest at a corner, so a long piece lying along the line
    // settles whole. Or from one point of the line, which bounds the
    // distance from the line however it bends: the point is the line's
    // at the foot of the box's centre, a float choice, enclosed by the
    // chart's own enclosure, so how the foot rounds cannot matter.
    let row = iso_row(s, knot, along_v);
    let ends = row.as_ref().and_then(|r| Some((*r.first()?, *r.last()?)));
    let bent = match (&row, ends) {
        (Some(r), Some(ends)) => worst(r.iter().map(|&p| segment_reach(p, ends))),
        _ => f64::NAN,
    };
    let near = |bx: &[Interval; 3]| {
        let across = ends.is_some_and(|ends| {
            let corners = (0..8).map(|i: u8| {
                let pick = |c: &Interval, bit: u8| if i & bit == 0 { c.lo() } else { c.hi() };
                Point3::new(pick(&bx[0], 1), pick(&bx[1], 2), pick(&bx[2], 4))
            });
            worst(corners.map(|c| segment_reach(c, ends))) + bent <= eps_in
        });
        across || {
            let mid = |c: &Interval| geom::mid_param(c.lo(), c.hi());
            let centre = Point3::new(mid(&bx[0]), mid(&bx[1]), mid(&bx[2]));
            let Ok(foot) = s.project(centre) else {
                return false;
            };
            let on = if along_v {
                lifted.rect_enclosure((foot.u, foot.u), (knot, knot))
            } else {
                lifted.rect_enclosure((knot, knot), (foot.v, foot.v))
            };
            let gap = Vec3::new(bx[0] - on.x, bx[1] - on.y, bx[2] - on.z);
            gap.norm().hi() <= eps_in
        }
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
        if ends.is_some_and(|ends| on_the_line(spec, ends, bent, eps_in)) {
            return true;
        }
        let carrier = spec.carrier.map_scalar(|x| Interval::from_bounds(x, x));
        let piece = |a: f64, b: f64| piece_box(&carrier, a, b);
        parts(
            &piece,
            &near,
            (spec.t0, spec.t1),
            cells.clone(),
            eps_in,
            &mut cell,
        )
    })
}

/// The net's row (column, where `along_v`) the chart interpolates at
/// its crease knot `knot`: the iso line's own control points.
fn iso_row(s: &NurbsSurface<f64>, knot: f64, along_v: bool) -> Option<Vec<Point3<f64>>> {
    let kv = if along_v {
        s.knot_values_v()
    } else {
        s.knot_values_u()
    };
    let first = kv.values().iter().position(|&x| x == knot)?;
    let (nu, nv) = s.control_counts();
    let control = s.control();
    Some(if along_v {
        (0..nu).map(|i| control[i * nv + first - 1]).collect()
    } else {
        control[(first - 1) * nv..first * nv].to_vec()
    })
}

/// An upper bound on `p`'s distance from the segment `ends`: its
/// distance from one point of the segment, chosen in `f64` (any point
/// bounds it), measured in interval arithmetic. NaN for a segment of no
/// length.
fn segment_reach(p: Point3<f64>, (a, b): (Point3<f64>, Point3<f64>)) -> f64 {
    let chord = b - a;
    let t = geom::Line {
        origin: a,
        dir: chord,
    }
    .param_within(p, (0.0, 1.0));
    if !t.is_finite() {
        return f64::NAN;
    }
    let i = |x: f64| Interval::from_bounds(x, x);
    let lift = |q: Point3<f64>| Vec3::new(i(q.x), i(q.y), i(q.z));
    let gap = lift(p) - (lift(a) + (lift(b) - lift(a)) * i(t));
    gap.norm().hi()
}

/// Whether `spec` lies within `eps_in` of the iso line at every point,
/// by structure: the line's control row lies within `bent` of the chord
/// `ends` between its ends, so the line runs within `bent` of every
/// chord point (its foot on the chord runs from one end to the other).
/// An edge whose convex hull (a line's two ends, a spline's control
/// points over its whole domain) lies within `e` of the chord then lies
/// within `e + bent` of the line. `false` for any other edge.
fn on_the_line(spec: &EdgeSpec, ends: (Point3<f64>, Point3<f64>), bent: f64, eps_in: f64) -> bool {
    let hull: Vec<Point3<f64>> = match spec.carrier {
        Curve3::Line(_) => vec![spec.carrier.eval(spec.t0), spec.carrier.eval(spec.t1)],
        Curve3::Nurbs(ref c) if c.knot_values().domain() == (spec.t0, spec.t1) => {
            c.control().to_vec()
        }
        _ => return false,
    };
    worst(hull.iter().map(|&p| segment_reach(p, ends))) + bent <= eps_in
}

/// The largest of `xs`, NaN where any is: a poisoned reading certifies
/// nothing.
fn worst(xs: impl Iterator<Item = f64>) -> f64 {
    xs.fold(0.0, |m, x| if x.is_nan() { f64::NAN } else { m.max(x) })
}

/// Whether the edge pieces over `t` keep farther than `pad` from every
/// one of `cells`, or lie within `ε_in` of the line (`near`), halving a
/// piece and quartering each cell it meets until one holds; `false`
/// where a piece has no box, a piece no wider than `pad` still meets a
/// cell, or the budget `cell` spends runs out.
fn parts(
    piece: &dyn Fn(f64, f64) -> Option<[Interval; 3]>,
    near: &dyn Fn(&[Interval; 3]) -> bool,
    (a, b): (f64, f64),
    cells: Vec<Cell>,
    pad: f64,
    cell: &mut dyn FnMut((f64, f64), (f64, f64)) -> Option<Cell>,
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
    // A piece no wider than `pad` that still meets the region, and is
    // not on the line, is where the boundary may reach the far side: at
    // that size the boundary's own distance from its chart, `pad`, is
    // the larger uncertainty, and a finer cut buys nothing.
    if bx.iter().all(|c| c.hi() - c.lo() <= pad) {
        return false;
    }
    let mut finer = Vec::with_capacity(4 * hit.len());
    for c in &hit {
        let (um, vm) = (geom::mid_param(c.u.0, c.u.1), geom::mid_param(c.v.0, c.v.1));
        for u in [(c.u.0, um), (um, c.u.1)] {
            for v in [(c.v.0, vm), (vm, c.v.1)] {
                let Some(q) = cell(u, v) else {
                    return false;
                };
                finer.push(q);
            }
        }
    }
    let m = geom::mid_param(a, b);
    parts(piece, near, (a, m), finer.clone(), pad, cell)
        && parts(piece, near, (m, b), finer, pad, cell)
}

/// An enclosure of `carrier` (its data as intervals) over `[a, b]`: the
/// bracketed box of its cover over the span ([`Curve3::cover_over`]),
/// which for a spline is its pieces' nets over the span and narrows as
/// the piece halves. `None` for a carrier with no cover, or a box the
/// door poisons. Read here rather than through `topo`'s edge cover,
/// which reads a body's certified edge curve: there is no body yet.
fn piece_box(carrier: &Curve3<Interval>, a: f64, b: f64) -> Option<[Interval; 3]> {
    let window =
        geom::cover::ParamWindow::new(Interval::from_bounds(a, a), Interval::from_bounds(b, b));
    let bx = carrier.cover_over(window).ok()?.aabb(0.0);
    let out = [
        (bx.min_x, bx.max_x),
        (bx.min_y, bx.max_y),
        (bx.min_z, bx.max_z),
    ];
    out.iter()
        .all(|&(lo, hi)| lo <= hi)
        .then(|| out.map(|(lo, hi)| Interval::from_bounds(lo, hi)))
}

/// The side of a knot line a boundary piece lies on, in the chart: below
/// the knot, or above it.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Side {
    Low,
    High,
}

/// How a boundary edge of the face being split stands to the knot's iso
/// line. `w` is the line's own parameter at the edge's start and end.
#[derive(Clone, Copy, Debug)]
enum Stand {
    /// Off the line except at its ends, on one side of it.
    Off(Side),
    /// Along the line, certified within the band of it.
    Along { w: (f64, f64) },
    /// Along the chart's boundary column that a chart closed across the
    /// split wears on both sides.
    Seam { w: (f64, f64) },
}

impl Stand {
    /// The side a use of the edge bounds. A use along a line keeps the
    /// face on its left (counter-clockwise in the chart where the face's
    /// `sense` agrees with `S_u × S_v`), so a use walking the knot line
    /// up `w` bounds the low side exactly where the low sub-patch's own
    /// boundary walks it that way (`up_is_low`); on the closed column the
    /// low sub-patch walks the other way.
    fn side(self, forward: bool, up_is_low: bool) -> Side {
        let low_if = |w: (f64, f64), low_walks_up: bool| {
            if ((w.1 > w.0) == forward) == low_walks_up {
                Side::Low
            } else {
                Side::High
            }
        };
        match self {
            Self::Off(side) => side,
            Self::Along { w } => low_if(w, up_is_low),
            Self::Seam { w } => low_if(w, !up_is_low),
        }
    }
}

/// Where a loop passes from one side of the knot line to the other: at
/// the vertex before use `at` of loop `ring`, the line's parameter `w`
/// there, leaving side `from`.
#[derive(Clone, Copy, Debug)]
struct Crossing {
    w: f64,
    ring: usize,
    at: usize,
    from: Side,
}

/// An edge read off the knot line: its meetings with the line inside
/// it, each `(t, point, w)` by its parameter, and the side of each piece
/// between them.
struct Cut {
    edge: u64,
    inner: Vec<(f64, Point3<f64>, f64)>,
    sides: Vec<Side>,
}

/// What one split did to a face: the faces below the line (the first
/// at the face's own index) and those above it.
struct Parted {
    low: Vec<usize>,
    high: Vec<usize>,
}

/// Splits face `fi` along the iso line of its crease knot `knot` (along
/// `v` where `at.along_v`), kept by [`crossing_knots`], into the pieces
/// the line parts it into: below the line on the low sub-patch, above it
/// on the high one.
///
/// **Where the line meets the trim** is the kernel's curve-pair reading
/// ([`geom_brep::curve_pair::meet`]) of the iso curve against each
/// boundary edge, at the file's band. An edge along the line is
/// certified there instead, as its iso pcurve
/// ([`PcurveCache::certify`] with [`Pcurve::iso_column`] or
/// [`Pcurve::iso_row`]), since two spans sharing a run are no meeting
/// the walk can read. Each edge piece between meetings lies on one side,
/// read at its middle by the chart's foot within the band; an edge is
/// split only where its side changes, the vertex at the meeting. A
/// meeting where the side does not change is a touch.
///
/// **The loop surgery.** A loop passes from one side to the other only
/// at a vertex on the line. Ordered along the line the way the low
/// side's boundary walks it, the crossings must alternate out of the low
/// side and back into it, starting out of it: each pair bounds one run
/// of the line inside the face, which is minted as an edge on the iso
/// curve. Each side's loops are then traced: a chain of the loop on that
/// side, the run its last crossing opens, the chain the run's other end
/// starts, until the walk closes. Every traced loop is an outer loop,
/// since the line bounds each piece from outside; a loop the line does
/// not cross is a ring on its side, and joins that side's one piece.
///
/// A line that meets no cell leaves the face whole, its chart narrowed to
/// the sub-patch on its side. What refuses typed,
/// naming the face and the knot
/// (`work/importgate/the-c0-knot-split-refuses-what-it-cannot-place.md`):
/// a reading that does not settle at the band; a touch or an edge along
/// the line inside a run; crossings that do not alternate; a crossed
/// line that closes on itself; and a ring on a side the line parts into
/// several pieces, which would need containment on a chart with no
/// inverse ([`crate::chart::infer_outer`]).
fn split_face_once(
    solid: &mut SolidSpec,
    fi: usize,
    knot: f64,
    at: Lines,
    mint: &mut dyn FnMut() -> u64,
) -> Result<Parted, StepImportError> {
    let Lines {
        along_v,
        eps_in,
        band,
        file,
    } = at;
    let Surface::Nurbs(ref s) = solid.faces[fi].surface else {
        unreachable!("only a B-spline chart has crease knots to split at")
    };
    let s = Arc::clone(s);
    let face_id = solid.faces[fi].id;
    let sense = solid.faces[fi].sense;
    let refuse = |what: &'static str| StepImportError::C0KnotUnsplit {
        face: face_id,
        knot,
        what,
    };
    let band = band.ok_or(refuse(
        "the file's uncertainty makes no band to read the knot line's crossings at",
    ))?;
    let NetSplit { lo, hi } = split_net(&s, knot, along_v)
        .ok_or(refuse("the chart refused its split at its own crease knot"))?;
    // The iso curve at the knot: the high child's start column (along
    // `u`) or start row (along `v`), bitwise the low child's end.
    let iso = if along_v {
        geom_brep::boundary_iso_v(&hi, false)
    } else {
        geom_brep::boundary_iso_u(&hi, false)
    }
    .map_err(|_| refuse("the knot's iso curve did not re-wrap as a curve"))?;
    let near = |a: Point3<f64>, b: Point3<f64>| (a - b).norm() <= eps_in;
    let (Some(&q0), Some(&q1)) = (iso.control().first(), iso.control().last()) else {
        unreachable!("a validated B-spline holds at least two control points")
    };
    // A chart closed across the split (its low and high boundaries one
    // curve, as a closed patch states its seam) puts the same corners
    // on both sides.
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
    let closed = corners(&lo, true)
        .iter()
        .zip(&corners(&hi, false))
        .all(|(&l, &h)| near(l, h));
    let up_is_low = sense != along_v;
    let w_domain = iso.knot_values().domain();
    // The line as the curve-pair reader reads it: an iso the import's
    // curve recognition certifies straight is read as that line, which
    // the reader's table reads in closed form, its arc length carried
    // back to the iso's parameter by the chord map the recognition
    // certifies within `ε_in`.
    let straight = crate::recognize_curve::try_line(&iso, file).map(|(line, _)| line);
    let chord = (q1 - q0).norm();
    let (reading, read_span) = match straight {
        Some(line) => (line, (0.0, chord)),
        None => (Curve3::Nurbs(Arc::new(iso.clone())), w_domain),
    };
    let to_w = |t: f64| {
        if matches!(reading, Curve3::Line(_)) {
            w_domain.0 + t / chord * (w_domain.1 - w_domain.0)
        } else {
            t
        }
    };
    let line = Span::new(&reading, read_span);
    // The chart's side of a point: its coordinate across the line, read
    // at its foot, which must lie within the band of it.
    let side_at = |p: Point3<f64>| -> Option<Side> {
        let f = s
            .project(p)
            .ok()
            .filter(|f| f.distance <= band.escalate())?;
        let across = if along_v { f.v } else { f.u };
        if across < knot {
            Some(Side::Low)
        } else if across > knot {
            Some(Side::High)
        } else {
            None
        }
    };

    // The parameter along `column` (an iso curve of the chart) of a point
    // within the band of it.
    let on = |column: &NurbsCurve3<f64>, p: Point3<f64>| {
        let f = column.project(p).ok()?;
        (f.distance <= band.escalate()).then_some(f.t)
    };
    // The chart's iso pcurve at `at` an edge runs along, certified, as
    // its ends' parameters on that iso's curve `column`. The vertices'
    // reading on the column only proposes the pcurve's ends; the
    // certificate decides.
    let along = |spec: &EdgeSpec, at: f64, column: &NurbsCurve3<f64>| -> Option<(f64, f64)> {
        let wa = on(column, solid.vertices[&spec.start])?;
        let wb = on(column, solid.vertices[&spec.end])?;
        if wa == wb {
            return None;
        }
        let image = if along_v {
            Pcurve::iso_row(at, (wa, wb), (spec.t0, spec.t1))
        } else {
            Pcurve::iso_column(at, (wa, wb), (spec.t0, spec.t1))
        };
        let surface = &solid.faces[fi].surface;
        PcurveCache::certify(image, spec.t0, spec.t1, &spec.carrier, surface, band)
            .ok()
            .map(|_| (wa, wb))
    };
    // The chart's low boundary column across the split, which a chart
    // closed across it wears on both sides as its seam.
    let seam_at = if along_v {
        s.knot_values_v().domain().0
    } else {
        s.knot_values_u().domain().0
    };
    let seam = if along_v {
        geom_brep::boundary_iso_v(&s, false)
    } else {
        geom_brep::boundary_iso_u(&s, false)
    }
    .map_err(|_| refuse("the chart's boundary column did not re-wrap as a curve"))?;

    // Read every boundary edge against the line before anything moves.
    let mut uses_of: BTreeMap<u64, usize> = BTreeMap::new();
    for u in solid.faces[fi].loops.iter().flat_map(|l| &l.uses) {
        *uses_of.entry(u.edge).or_default() += 1;
    }
    let mut on_line: BTreeMap<u64, f64> = BTreeMap::new();
    let mut stand: BTreeMap<u64, Stand> = BTreeMap::new();
    let mut touches: Vec<f64> = Vec::new();
    let mut alongs: Vec<(f64, f64)> = Vec::new();
    let mut cuts: Vec<Cut> = Vec::new();
    for (&e, &n) in &uses_of {
        let spec = &solid.edges[&e];
        let (pa, pb) = (solid.vertices[&spec.start], solid.vertices[&spec.end]);
        if closed && (n == 2 || along(spec, seam_at, &seam).is_some()) {
            let (Some(wa), Some(wb)) = (on(&seam, pa), on(&seam, pb)) else {
                return Err(refuse(
                    "a seam edge of the chart closed across the split did not project onto it",
                ));
            };
            stand.insert(e, Stand::Seam { w: (wa, wb) });
            continue;
        }
        if let Some((wa, wb)) = along(spec, knot, &iso) {
            on_line.insert(spec.start, wa);
            on_line.insert(spec.end, wb);
            alongs.push((wa.min(wb), wa.max(wb)));
            stand.insert(e, Stand::Along { w: (wa, wb) });
            continue;
        }
        let met = curve_pair::meet(&line, &Span::new(&spec.carrier, (spec.t0, spec.t1)), band)
            .map_err(|_| {
                refuse(
                    "the knot's iso line and a boundary edge did not settle where they meet \
                     at the file's band",
                )
            })?;
        if met.shared.is_some() {
            return Err(refuse(
                "a boundary edge shares a run with the knot's iso line",
            ));
        }
        // A spline edge holds a vertex at each of its crease knots, so a
        // meeting within the band of one is that knot's point, and the
        // edge splits there by partition.
        let creases = match spec.carrier {
            Curve3::Nurbs(ref c) => c.knot_values().crease_values(),
            _ => Vec::new(),
        };
        let mut inner = Vec::new();
        for m in &met.points {
            let w = to_w(line.at(m.on_a));
            match m.on_b {
                Hit::End(SpanEnd::Start) => {
                    on_line.insert(spec.start, w);
                }
                Hit::End(SpanEnd::Stop) => {
                    on_line.insert(spec.end, w);
                }
                Hit::On { t, .. } => inner.push(
                    creases
                        .iter()
                        .map(|&k| (k, spec.carrier.eval(k)))
                        .find(|&(_, q)| (q - m.point).norm() <= band.escalate())
                        .map_or((t, m.point, w), |(k, q)| (k, q, w)),
                ),
            }
        }
        inner.sort_by(|a, b| a.0.total_cmp(&b.0));
        let stops: Vec<f64> = std::iter::once(spec.t0)
            .chain(inner.iter().map(|m| m.0))
            .chain(std::iter::once(spec.t1))
            .collect();
        let sides = stops
            .windows(2)
            .map(|p| side_at(spec.carrier.eval(geom::mid_param(p[0], p[1]))))
            .collect::<Option<Vec<Side>>>()
            .ok_or(refuse(
                "a boundary edge's piece between its meetings with the knot's iso line did \
                 not read a side of it",
            ))?;
        cuts.push(Cut {
            edge: e,
            inner,
            sides,
        });
    }
    // Split each edge where its side changes.
    for Cut { edge, inner, sides } in cuts {
        let mut rest = edge;
        for (j, &(t, q, w)) in inner.iter().enumerate() {
            if sides[j] == sides[j + 1] {
                touches.push(w);
                continue;
            }
            let Some(Split { vertices, pieces }) =
                split_edge(solid, rest, &[(t, Place::At(q))], mint)
            else {
                return Err(refuse(
                    "a boundary edge refused its split where the knot's iso line crosses it",
                ));
            };
            let (&[v], &[a, b]) = (vertices.as_slice(), pieces.as_slice()) else {
                unreachable!("one split makes one vertex and two pieces")
            };
            on_line.insert(v, w);
            stand.insert(a, Stand::Off(sides[j]));
            rest = b;
        }
        let Some(&last) = sides.last() else {
            unreachable!("an edge's pieces are one more than its inner meetings")
        };
        stand.insert(rest, Stand::Off(last));
    }

    // Where each loop crosses the line.
    let loops: Vec<Vec<EdgeUse>> = solid.faces[fi]
        .loops
        .iter()
        .map(|l| l.uses.clone())
        .collect();
    let side_of = |u: EdgeUse| stand[&u.edge].side(u.forward, up_is_low);
    let mut crossings = Vec::new();
    for (ring, uses) in loops.iter().enumerate() {
        let n = uses.len();
        for i in 0..n {
            let (from, to) = (side_of(uses[(i + n - 1) % n]), side_of(uses[i]));
            match (from == to, on_line.get(&use_start(solid, uses[i]))) {
                (true, Some(&w)) => touches.push(w),
                (true, None) => {}
                (false, Some(&w)) => crossings.push(Crossing {
                    w,
                    ring,
                    at: i,
                    from,
                }),
                (false, None) => {
                    return Err(refuse(
                        "the face's loop passes from one side of the knot's iso line to the \
                         other away from it",
                    ));
                }
            }
        }
    }
    // A loop that never passes to the other side leaves the line in no
    // cell: the face lies on one side, and narrows to that side's
    // sub-patch, which holds its whole trim.
    if crossings.is_empty() {
        let sides: BTreeSet<bool> = loops
            .iter()
            .flatten()
            .map(|&u| side_of(u) == Side::High)
            .collect();
        let [high] = sides.into_iter().collect::<Vec<_>>()[..] else {
            return Err(refuse(
                "the face's loops lie on both sides of the knot's iso line, which none \
                 crosses",
            ));
        };
        solid.faces[fi].surface = Surface::Nurbs(Arc::new(if high { hi } else { lo }));
        let (low, high) = if high {
            (Vec::new(), vec![fi])
        } else {
            (vec![fi], Vec::new())
        };
        return Ok(Parted { low, high });
    }
    if near(q0, q1) {
        return Err(refuse("the knot's iso line closes on itself"));
    }
    let key = |c: &Crossing| if up_is_low { c.w } else { -c.w };
    crossings.sort_by(|a, b| key(a).total_cmp(&key(b)));
    let alternates = crossings.len() % 2 == 0
        && crossings
            .iter()
            .enumerate()
            .all(|(j, c)| c.from == if j % 2 == 0 { Side::Low } else { Side::High })
        && crossings.windows(2).all(|p| key(&p[0]) < key(&p[1]));
    if !alternates {
        return Err(refuse(
            "the trim's crossings with the knot's iso line do not alternate into and out of \
             the face",
        ));
    }
    // The runs of the line inside the face, each minted as an edge over
    // its stretch of the iso curve: that curve whole, so bitwise both
    // sub-patches' boundary column.
    let line_carrier = Curve3::Nurbs(Arc::new(iso.clone()));
    // The loops the line crosses, and a crossing's position in them.
    let crossed: Vec<usize> = (0..loops.len())
        .filter(|&r| crossings.iter().any(|c| c.ring == r))
        .collect();
    let at_of = |c: &Crossing| -> At {
        let Ok(k) = crossed.binary_search(&c.ring) else {
            unreachable!("a crossing's loop is a crossed loop")
        };
        (k, c.at)
    };
    let mut runs = Vec::with_capacity(crossings.len() / 2);
    for pair in crossings.chunks(2) {
        let [a, b] = [pair[0], pair[1]];
        let (va, vb) = (
            use_start(solid, loops[a.ring][a.at]),
            use_start(solid, loops[b.ring][b.at]),
        );
        let ((w_lo, v_lo, c_lo), (w_hi, v_hi, c_hi)) = if a.w < b.w {
            ((a.w, va, a), (b.w, vb, b))
        } else {
            ((b.w, vb, b), (a.w, va, a))
        };
        let inside = |w: f64| w_lo < w && w < w_hi;
        if touches.iter().any(|&w| inside(w))
            || alongs.iter().any(|&(x, y)| x.max(w_lo) < y.min(w_hi))
        {
            return Err(refuse(
                "the trim touches the knot's iso line, or runs along it, inside the face",
            ));
        }
        let e = mint();
        solid.edges.insert(
            e,
            EdgeSpec {
                start: v_lo,
                end: v_hi,
                carrier: line_carrier.clone(),
                t0: w_lo,
                t1: w_hi,
                reversed: false,
            },
        );
        runs.push(CutLine {
            first: EdgeUse {
                edge: e,
                forward: true,
            },
            ends: [Some(at_of(&c_lo)), Some(at_of(&c_hi))],
        });
    }
    // Each side's pieces, traced from the crossings into it in order:
    // a chain of the loop on that side, the run its last crossing opens,
    // the chain the run's other end starts, until the walk closes.
    let crossed_loops: Vec<Vec<EdgeUse>> = crossed.iter().map(|&r| loops[r].clone()).collect();
    let seeds: Vec<At> = crossings.iter().map(at_of).collect();
    let (mut low_pieces, mut high_pieces) = (Vec::new(), Vec::new());
    for piece in surgery::relink(solid, &crossed_loops, &runs, &seeds)
        .map_err(|_| refuse("internal: the trim and the knot line's runs did not close"))?
    {
        match piece.first().and_then(|u| stand.get(&u.edge)) {
            Some(_) if side_of(piece[0]) == Side::Low => low_pieces.push(piece),
            Some(_) => high_pieces.push(piece),
            None => return Err(refuse("internal: a piece traced from no crossing")),
        }
    }
    let mut rings = [Vec::new(), Vec::new()];
    for (ring, uses) in loops.iter().enumerate() {
        if crossings.iter().any(|c| c.ring == ring) {
            continue;
        }
        if ring == 0 {
            return Err(refuse(
                "the knot's iso line crosses a ring of the face but not its outer loop",
            ));
        }
        rings[usize::from(side_of(uses[0]) == Side::High)].push(uses.clone());
    }
    let mut faces = Vec::new();
    for (pieces, rings, net) in [(low_pieces, &rings[0], lo), (high_pieces, &rings[1], hi)] {
        if !rings.is_empty() && pieces.len() != 1 {
            return Err(refuse(
                "a ring of the face lies on a side the knot's iso line parts into several \
                 pieces, which placing it needs containment on the chart to tell",
            ));
        }
        let net = Arc::new(net);
        faces.push(
            pieces
                .into_iter()
                .map(|outer| FaceSpec {
                    id: face_id,
                    surface: Surface::Nurbs(Arc::clone(&net)),
                    sense,
                    loops: std::iter::once(LoopSpec {
                        outer: true,
                        uses: outer,
                    })
                    .chain(rings.iter().map(|r| LoopSpec {
                        outer: false,
                        uses: r.clone(),
                    }))
                    .collect(),
                    band: false,
                })
                .collect::<Vec<_>>(),
        );
    }
    let mut index = |made: Vec<FaceSpec>, first: bool| -> Vec<usize> {
        made.into_iter()
            .enumerate()
            .map(|(k, face)| {
                if first && k == 0 {
                    solid.faces[fi] = face;
                    fi
                } else {
                    solid.faces.push(face);
                    solid.faces.len() - 1
                }
            })
            .collect()
    };
    let Ok([low, high]) = <[Vec<FaceSpec>; 2]>::try_from(faces) else {
        unreachable!("one list of pieces per side")
    };
    let low = index(low, true);
    let high = index(high, false);
    Ok(Parted { low, high })
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
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

    /// The file's own band at `EPS_IN`, `(ε_in, K·ε_in)`, unfloored by
    /// the run's ε: the pass reads ε_in raw beside it
    /// (`work/importgate/the-c0-knot-split-reads-eps-in-raw-not-at-the-file-band.md`),
    /// so its rows pin the two at one scale on every run.
    fn band() -> Option<geom_core::Band> {
        crate::interpret::band_at(EPS_IN, 0.0, geom_core::Tol::witness().k())
    }

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
        let circle = Curve3::circle(
            Point3::new(cx, 0.5, 0.0),
            Vec3::new(0.0, 0.0, 1.0),
            0.2,
            Vec3::new(1.0, 0.0, 0.0),
        );
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
                    carrier: Curve3::line(a, b - a),
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

    /// The left half of a chart curved along `v` (degree 2, bulging to
    /// `z = 1/4`), bounded on its right by the knot line itself. The
    /// line is not straight, so no chord certifies the edge on it whole,
    /// and the selection's budget runs out before pieces `ε_in` long
    /// would: the line is kept. The split certifies that edge along the
    /// line by its iso pcurve, finds the loop never passes to the other
    /// side, and leaves the face whole on the sub-patch below the line.
    #[test]
    fn a_face_along_a_curved_knot_line_is_certified_on_it_and_kept_whole() {
        let s = NurbsSurface::new(
            KnotVector::clamped(vec![0.0, 0.0, 0.5, 1.0, 1.0], 1).unwrap(),
            KnotVector::clamped(vec![0.0, 0.0, 0.0, 1.0, 1.0, 1.0], 2).unwrap(),
            [0.0, 0.5, 1.0]
                .into_iter()
                .flat_map(|x| {
                    [(0.0, 0.0), (0.5, 0.5), (1.0, 0.0)].map(|(y, z)| Point3::new(x, y, z))
                })
                .collect(),
            vec![1.0; 9],
        )
        .unwrap();
        let iso = |x: f64, down: bool| {
            let mut c = vec![
                Point3::new(x, 0.0, 0.0),
                Point3::new(x, 0.5, 0.5),
                Point3::new(x, 1.0, 0.0),
            ];
            if down {
                c.reverse();
            }
            Curve3::Nurbs(Arc::new(
                geom::NurbsCurve3::new(
                    KnotVector::clamped(vec![0.0, 0.0, 0.0, 1.0, 1.0, 1.0], 2).unwrap(),
                    c,
                    vec![1.0; 3],
                )
                .unwrap(),
            ))
        };
        let line = |a: Point3<f64>, b: Point3<f64>| Curve3::line(a, b - a);
        let corners =
            [(0.0, 0.0), (0.5, 0.0), (0.5, 1.0), (0.0, 1.0)].map(|(x, y)| Point3::new(x, y, 0.0));
        let carriers = [
            line(corners[0], corners[1]),
            iso(0.5, false),
            line(corners[2], corners[3]),
            iso(0.0, true),
        ];
        let mut solid = solid(0.25);
        solid.faces[0].surface = Surface::Nurbs(Arc::new(s));
        solid.vertices = (0..4).map(|i| (30 + i, corners[i as usize])).collect();
        solid.edges = carriers
            .into_iter()
            .zip(0..4u64)
            .map(|(carrier, i)| {
                let spec = EdgeSpec {
                    start: 30 + i,
                    end: 30 + (i + 1) % 4,
                    carrier,
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
        assert_eq!(crossing_knots(&solid, 0, false, EPS_IN), vec![0.5]);
        let mut n = 100;
        let mut mint = || {
            n += 1;
            n
        };
        let mut sink = Vec::new();
        let kept = split_faces(
            &mut solid,
            crate::interpret::FileBand::test(EPS_IN),
            &mut mint,
            &mut sink,
        );
        assert!(kept.is_ok(), "the face along the line imports: {kept:?}");
        assert_eq!(
            (solid.faces.len(), sink.len(), solid.edges.len()),
            (1, 0, 4),
            "the face and its four edges as stated"
        );
        let Surface::Nurbs(ref chart) = solid.faces[0].surface else {
            panic!("the face stays on a spline chart")
        };
        assert_eq!(
            chart.knot_values_u().domain(),
            (0.0, 0.5),
            "the chart narrowed to the sub-patch below the line, its crease gone"
        );
    }

    /// One bound of a face on the unit chart: a closed polygon of lines,
    /// or one circle edge, counter-clockwise where `ccw`.
    enum Bound {
        Polygon(Vec<(f64, f64)>),
        Circle { c: (f64, f64), r: f64, ccw: bool },
    }

    /// The unit chart of [`solid`] bounded by `bounds`, the first outer.
    fn bounded(bounds: &[Bound]) -> SolidSpec {
        let mut solid = solid(0.25);
        solid.edges.clear();
        solid.vertices.clear();
        let mut loops = Vec::new();
        let (mut e, mut v) = (200u64, 300u64);
        for (i, bound) in bounds.iter().enumerate() {
            let mut uses = Vec::new();
            match *bound {
                Bound::Polygon(ref pts) => {
                    let first = v;
                    for (k, &(x, y)) in pts.iter().enumerate() {
                        let (nx, ny) = pts[(k + 1) % pts.len()];
                        let (a, b) = (Point3::new(x, y, 0.0), Point3::new(nx, ny, 0.0));
                        solid.vertices.insert(v, a);
                        let end = if k + 1 == pts.len() { first } else { v + 1 };
                        solid.edges.insert(
                            e,
                            EdgeSpec {
                                start: v,
                                end,
                                carrier: Curve3::line(a, b - a),
                                t0: 0.0,
                                t1: 1.0,
                                reversed: false,
                            },
                        );
                        uses.push(EdgeUse {
                            edge: e,
                            forward: true,
                        });
                        (e, v) = (e + 1, v + 1);
                    }
                }
                Bound::Circle { c, r, ccw } => {
                    let circle = Curve3::circle(
                        Point3::new(c.0, c.1, 0.0),
                        Vec3::new(0.0, 0.0, 1.0),
                        r,
                        Vec3::new(1.0, 0.0, 0.0),
                    );
                    let t0 = core::f64::consts::FRAC_PI_8;
                    solid.vertices.insert(v, circle.eval(t0));
                    solid.edges.insert(
                        e,
                        EdgeSpec {
                            start: v,
                            end: v,
                            carrier: circle,
                            t0,
                            t1: t0 + core::f64::consts::TAU,
                            reversed: false,
                        },
                    );
                    uses.push(EdgeUse {
                        edge: e,
                        forward: ccw,
                    });
                    (e, v) = (e + 1, v + 1);
                }
            }
            loops.push(LoopSpec {
                outer: i == 0,
                uses,
            });
        }
        solid.faces[0].loops = loops;
        solid
    }

    /// Splits every face of `solid`, minting ids from 1000.
    fn split(solid: &mut SolidSpec) -> Result<Vec<crate::StructureNormalization>, StepImportError> {
        let mut n = 1000;
        let mut mint = || {
            n += 1;
            n
        };
        let mut sink = Vec::new();
        split_faces(
            solid,
            crate::interpret::FileBand::test(EPS_IN),
            &mut mint,
            &mut sink,
        )
        .map(|()| sink)
    }

    /// Points along a use, in its walking order, its last point left out.
    fn walked(solid: &SolidSpec, u: EdgeUse) -> Vec<Point3<f64>> {
        let spec = &solid.edges[&u.edge];
        (0..64)
            .map(|i| {
                let f = f64::from(i) / 64.0;
                let f = if u.forward { f } else { 1.0 - f };
                spec.carrier.eval(spec.t0 + (spec.t1 - spec.t0) * f)
            })
            .collect()
    }

    /// What the split made of the unit chart's face: per face, its
    /// loop count and the shoelace area of its sampled loops (a ring's
    /// reversed walk subtracts); asserting first that every loop closes
    /// vertex to vertex, every boundary point lies on its face's own
    /// sub-patch, and every piece keeps the face's `sense`.
    fn parts(solid: &SolidSpec, sense: bool) -> Vec<(usize, f64)> {
        solid
            .faces
            .iter()
            .map(|face| {
                assert_eq!(face.sense, sense, "a piece keeps the face's sense");
                let Surface::Nurbs(ref s) = face.surface else {
                    panic!("a split face stays on a spline chart")
                };
                let (lo, hi) = s.knot_values_u().domain();
                let mut area = 0.0;
                for lp in &face.loops {
                    let n = lp.uses.len();
                    for i in 0..n {
                        let (a, b) = (lp.uses[i], lp.uses[(i + 1) % n]);
                        let end = |u: EdgeUse| {
                            let spec = &solid.edges[&u.edge];
                            if u.forward { spec.end } else { spec.start }
                        };
                        assert_eq!(end(a), super::use_start(solid, b), "the loop closes");
                    }
                    let pts: Vec<_> = lp.uses.iter().flat_map(|&u| walked(solid, u)).collect();
                    for p in &pts {
                        assert!(
                            lo - 1e-9 <= p.x && p.x <= hi + 1e-9,
                            "{p:?} lies on the face's sub-patch [{lo}, {hi}]"
                        );
                    }
                    for i in 0..pts.len() {
                        let (a, b) = (pts[i], pts[(i + 1) % pts.len()]);
                        area += (a.x * b.y - b.x * a.y) / 2.0;
                    }
                }
                (face.loops.len(), area)
            })
            .collect()
    }

    /// The total area of `parts`, against `want` at the sampling's
    /// resolution.
    fn holds(parts: &[(usize, f64)], want: f64) {
        let got: f64 = parts.iter().map(|p| p.1).sum();
        assert!(
            (got - want).abs() < 2e-3,
            "the pieces' area {got} is the face's {want}: {parts:?}"
        );
        assert!(
            parts.iter().all(|p| p.1 > 0.0),
            "each piece is a region: {parts:?}"
        );
    }

    /// A circle about `(0.31, 0.5)` reaches `x = 0.51`, past the knot
    /// line by `0.01`, but its nine samples, from `π/8` in steps of
    /// `π/4`, reach only `0.31 + 0.2 cos(π/8) ≈ 0.4948`: the samples see
    /// one side, nothing certifies the other clear, so the line is kept.
    /// The trim crosses it twice, away from the chart's boundary rows:
    /// the split places both crossings on the circle and parts the face
    /// into the cap past the line and the rest.
    #[test]
    fn a_trim_crossing_between_samples_parts_into_two_faces() {
        let mut solid = solid(0.31);
        assert_eq!(crossing_knots(&solid, 0, false, EPS_IN), vec![0.5]);
        let sink = split(&mut solid).expect("the crossed face splits");
        assert_eq!(sink.len(), 1, "one record for the face");
        let parts = parts(&solid, true);
        assert_eq!(
            parts.iter().map(|p| p.0).collect::<Vec<_>>(),
            [1, 1],
            "two faces, one loop each"
        );
        holds(&parts, core::f64::consts::PI * 0.04);
        let half_chord = (0.2f64 * 0.2 - 0.19 * 0.19).sqrt();
        let mut on_line: Vec<f64> = solid
            .vertices
            .values()
            .filter(|p| (p.x - 0.5).abs() < 1e-9)
            .map(|p| p.y)
            .collect();
        on_line.sort_by(f64::total_cmp);
        assert_eq!(on_line.len(), 2, "a vertex at each crossing: {on_line:?}");
        assert!(
            (on_line[0] - (0.5 - half_chord)).abs() < 1e-9
                && (on_line[1] - (0.5 + half_chord)).abs() < 1e-9,
            "the crossings at y = 0.5 ± {half_chord}: {on_line:?}"
        );
    }

    /// A C-shaped trim the line crosses four times, twice inside edges
    /// away from the chart's boundary rows: one piece below the line and
    /// the C's two arms above it. Stated both ways round: on a face whose
    /// `sense` opposes `S_u × S_v` its loop turns clockwise in the chart,
    /// and each side's boundary walks the line the other way.
    #[test]
    fn a_trim_crossing_the_line_four_times_parts_into_three_faces() {
        for sense in [true, false] {
            let mut c = vec![
                (0.1, 0.1),
                (0.9, 0.1),
                (0.9, 0.3),
                (0.3, 0.3),
                (0.3, 0.7),
                (0.9, 0.7),
                (0.9, 0.9),
                (0.1, 0.9),
            ];
            if !sense {
                c.reverse();
            }
            let mut solid = bounded(&[Bound::Polygon(c)]);
            solid.faces[0].sense = sense;
            split(&mut solid).expect("the C splits");
            let turned: Vec<_> = parts(&solid, sense)
                .into_iter()
                .map(|(n, a)| (n, if sense { a } else { -a }))
                .collect();
            assert_eq!(
                turned.iter().map(|p| p.0).collect::<Vec<_>>(),
                [1, 1, 1],
                "sense {sense}: the back of the C and its two arms"
            );
            holds(&turned, 0.64 - 0.24);
        }
    }

    /// A ring the line crosses, inside a square it also crosses: each
    /// side is one piece whose outer loop walks a half of the square, a
    /// run of the line, a half of the ring and the other run.
    #[test]
    fn a_crossed_ring_parts_with_its_face() {
        let mut solid = bounded(&[
            Bound::Polygon(vec![(0.1, 0.1), (0.9, 0.1), (0.9, 0.9), (0.1, 0.9)]),
            Bound::Circle {
                c: (0.5, 0.5),
                r: 0.1,
                ccw: false,
            },
        ]);
        split(&mut solid).expect("the ringed face splits");
        let parts = parts(&solid, true);
        assert_eq!(
            parts.iter().map(|p| p.0).collect::<Vec<_>>(),
            [1, 1],
            "two faces, the ring parted into their outer loops"
        );
        holds(&parts, 0.64 - core::f64::consts::PI * 0.01);
    }

    /// A ring wholly below the line joins the one piece below it.
    #[test]
    fn an_uncrossed_ring_joins_the_one_piece_on_its_side() {
        let mut solid = bounded(&[
            Bound::Polygon(vec![(0.1, 0.1), (0.9, 0.1), (0.9, 0.9), (0.1, 0.9)]),
            Bound::Circle {
                c: (0.25, 0.5),
                r: 0.1,
                ccw: false,
            },
        ]);
        split(&mut solid).expect("the ringed face splits");
        let parts = parts(&solid, true);
        assert_eq!(
            parts.iter().map(|p| p.0).collect::<Vec<_>>(),
            [2, 1],
            "the low piece keeps the ring"
        );
        holds(&parts, 0.64 - core::f64::consts::PI * 0.01);
    }

    /// A ring in one arm of the C: the line parts the side it lies on
    /// into two pieces, and telling which holds it needs containment on
    /// the chart, so the split refuses typed.
    #[test]
    fn a_ring_on_a_side_parted_into_several_pieces_refuses_typed() {
        let mut solid = bounded(&[
            Bound::Polygon(vec![
                (0.1, 0.1),
                (0.9, 0.1),
                (0.9, 0.3),
                (0.3, 0.3),
                (0.3, 0.7),
                (0.9, 0.7),
                (0.9, 0.9),
                (0.1, 0.9),
            ]),
            Bound::Circle {
                c: (0.75, 0.8),
                r: 0.05,
                ccw: false,
            },
        ]);
        let refused = split(&mut solid);
        assert!(
            matches!(
                refused,
                Err(StepImportError::C0KnotUnsplit { face: 10, what, .. })
                    if what.contains("several pieces")
            ),
            "the ring's piece is not told: {refused:?}"
        );
    }

    /// A circle about `(0.3 + 2ε_in, 0.5)` crosses the line by `2ε_in`,
    /// between samples: farther than the `ε_in` the import may move a
    /// boundary, so it is kept, and the curve-pair reading of two
    /// crossings that close does not settle at the band: the split
    /// refuses typed rather than absorb the crossing as on the line.
    #[test]
    fn a_trim_crossing_by_twice_eps_in_is_kept_and_refuses_typed() {
        let mut solid = solid(0.3 + 2.0 * EPS_IN);
        assert_eq!(crossing_knots(&solid, 0, false, EPS_IN), vec![0.5]);
        let refused = split(&mut solid);
        assert!(
            matches!(
                refused,
                Err(StepImportError::C0KnotUnsplit { face: 10, what, .. })
                    if what.contains("did not settle where they meet")
            ),
            "the crossing refuses typed: {refused:?}"
        );
    }

    /// Whether `split` refused naming face 10 with `what` containing
    /// `reason`.
    fn refused_for(
        split: &Result<Vec<crate::StructureNormalization>, StepImportError>,
        reason: &str,
    ) -> bool {
        matches!(
            split,
            Err(StepImportError::C0KnotUnsplit { face: 10, what, .. }) if what.contains(reason)
        )
    }

    /// The square `[0.1, 0.9]²` the line crosses at `y = 0.1` and
    /// `y = 0.9`, counter-clockwise.
    const SQUARE: [(f64, f64); 4] = [(0.1, 0.1), (0.9, 0.1), (0.9, 0.9), (0.1, 0.9)];

    /// The square walked clockwise on a face whose `sense` agrees with
    /// `S_u × S_v`: along the line the loop leaves the high side first,
    /// so its crossings do not alternate into and out of the face.
    #[test]
    fn a_loop_turning_against_its_faces_sense_refuses_typed() {
        let mut cw = SQUARE.to_vec();
        cw.reverse();
        let refused = split(&mut bounded(&[Bound::Polygon(cw)]));
        assert!(
            refused_for(&refused, "do not alternate"),
            "the crossings do not alternate: {refused:?}"
        );
    }

    /// A notch from the square's left side whose tip vertex touches the
    /// line at `y = 0.5`, inside the run `0.1 < y < 0.9`: the low piece
    /// would be pinched at that vertex.
    #[test]
    fn a_notch_touching_the_line_inside_a_run_refuses_typed() {
        let notched = vec![
            (0.1, 0.1),
            (0.9, 0.1),
            (0.9, 0.9),
            (0.1, 0.9),
            (0.1, 0.6),
            (0.5, 0.5),
            (0.1, 0.4),
        ];
        let refused = split(&mut bounded(&[Bound::Polygon(notched)]));
        assert!(
            refused_for(&refused, "inside the face"),
            "the touch inside the run refuses: {refused:?}"
        );
    }

    /// A clockwise triangle ring whose vertex touches the line inside the
    /// square's run.
    #[test]
    fn a_ring_touching_the_line_inside_a_run_refuses_typed() {
        let refused = split(&mut bounded(&[
            Bound::Polygon(SQUARE.to_vec()),
            Bound::Polygon(vec![(0.3, 0.4), (0.3, 0.6), (0.5, 0.5)]),
        ]));
        assert!(
            refused_for(&refused, "inside the face"),
            "the ring's touch inside the run refuses: {refused:?}"
        );
    }

    /// A circle trim lifted off the chart by three times the band's
    /// escalation: no meeting with the line, and no foot of its pieces
    /// within the band, so no side to read it on.
    #[test]
    fn a_trim_off_its_chart_past_the_band_reads_no_side_and_refuses_typed() {
        let lift = 3.0 * band().unwrap().escalate();
        let mut solid = solid(0.4);
        let circle = Curve3::circle(
            Point3::new(0.4, 0.5, lift),
            Vec3::new(0.0, 0.0, 1.0),
            0.2,
            Vec3::new(1.0, 0.0, 0.0),
        );
        let spec = solid.edges.get_mut(&20).unwrap();
        solid.vertices.insert(30, circle.eval(spec.t0));
        spec.carrier = circle;
        assert_eq!(crossing_knots(&solid, 0, false, EPS_IN), vec![0.5]);
        let refused = split(&mut solid);
        assert!(
            refused_for(&refused, "did not read a side"),
            "a piece off the chart reads no side: {refused:?}"
        );
    }

    /// A tube closed in `u`: degree 1 in `u` over knots `0, 0, 1, 2, 3,
    /// 4, 4`, its five control columns a square's corners with the first
    /// repeated, and `z` from 0 to 1 in `v`. Every interior knot is a
    /// crease. The face is the chart polygon `poly` (in `(u, v)`), its
    /// edges the lines between its corners' images.
    fn tube(poly: &[(f64, f64)]) -> SolidSpec {
        let corners = [(1.0, 0.0), (0.0, 1.0), (-1.0, 0.0), (0.0, -1.0), (1.0, 0.0)];
        let s = NurbsSurface::new(
            KnotVector::clamped(vec![0.0, 0.0, 1.0, 2.0, 3.0, 4.0, 4.0], 1).unwrap(),
            KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1).unwrap(),
            corners
                .iter()
                .flat_map(|&(x, y)| [0.0, 1.0].map(|z| Point3::new(x, y, z)))
                .collect(),
            vec![1.0; 10],
        )
        .unwrap();
        let mut solid = bounded(&[Bound::Polygon(
            poly.iter()
                .map(|&(u, v)| {
                    let p = s.eval(u, v);
                    (p.x, p.y)
                })
                .collect(),
        )]);
        // `bounded` places its polygon at `z = 0`; lift each corner to
        // its own height and rebuild the lines between them.
        for (k, &(u, v)) in poly.iter().enumerate() {
            solid.vertices.insert(300 + k as u64, s.eval(u, v));
        }
        let n = poly.len() as u64;
        for k in 0..n {
            let (a, b) = (
                solid.vertices[&(300 + k)],
                solid.vertices[&(300 + (k + 1) % n)],
            );
            solid.edges.get_mut(&(200 + k)).unwrap().carrier = Curve3::line(a, b - a);
        }
        solid.faces[0].surface = Surface::Nurbs(Arc::new(s));
        solid
    }

    /// The tube's pieces: each loop closes, each keeps `sense`, and each
    /// piece's corners lie on its own chart within one knot span, so no
    /// crease is left inside a cell. Answers the pieces' `u` extents.
    fn tube_pieces(solid: &SolidSpec, sense: bool) -> Vec<(f64, f64)> {
        let mut out: Vec<(f64, f64)> = solid
            .faces
            .iter()
            .map(|face| {
                assert_eq!(face.sense, sense, "a piece keeps the face's sense");
                let Surface::Nurbs(ref s) = face.surface else {
                    panic!("a piece stays on a spline chart")
                };
                let (mut lo, mut hi) = (f64::INFINITY, f64::NEG_INFINITY);
                for lp in &face.loops {
                    let n = lp.uses.len();
                    for i in 0..n {
                        let (a, b) = (lp.uses[i], lp.uses[(i + 1) % n]);
                        let spec = &solid.edges[&a.edge];
                        let end = if a.forward { spec.end } else { spec.start };
                        assert_eq!(end, super::use_start(solid, b), "the loop closes");
                        let foot = s.project(solid.vertices[&end]).unwrap();
                        assert!(foot.distance < 1e-9, "a corner lies on its piece's chart");
                        (lo, hi) = (lo.min(foot.u), hi.max(foot.u));
                    }
                }
                assert!(
                    s.knot_values_u()
                        .crease_values()
                        .iter()
                        .all(|&k| k <= lo + 1e-9 || k >= hi - 1e-9),
                    "no crease inside the piece [{lo}, {hi}]"
                );
                (lo, hi)
            })
            .collect();
        out.sort_by(|a, b| a.0.total_cmp(&b.0));
        out
    }

    /// The tube's face over `u ∈ [1.5, 4]`, bounded on the right by the
    /// chart's closed column at `u = 4`, which a projection reads at
    /// `u = 0`. That edge is sided by the column it is certified on and
    /// the way it walks it, so the face parts at the knots 2 and 3.
    #[test]
    fn a_tube_face_ending_on_its_charts_closed_column_parts_at_its_knots() {
        let poly = [
            (1.5, 0.0),
            (2.0, 0.0),
            (3.0, 0.0),
            (4.0, 0.0),
            (4.0, 1.0),
            (3.0, 1.0),
            (2.0, 1.0),
            (1.5, 1.0),
        ];
        let mut solid = tube(&poly);
        split(&mut solid).expect("the face on the closed column splits");
        let pieces = tube_pieces(&solid, true);
        let want = [(1.5, 2.0), (2.0, 3.0), (3.0, 4.0)];
        assert_eq!(pieces.len(), 3, "three pieces: {pieces:?}");
        for (got, want) in pieces.iter().zip(want) {
            assert!(
                (got.0 - want.0).abs() < 1e-9 && (got.1 - want.1).abs() < 1e-9,
                "pieces {pieces:?}"
            );
        }
    }

    /// The mirror: the face over `u ∈ [0, 2.5]`, bounded on the left by
    /// the closed column at `u = 0`.
    #[test]
    fn a_tube_face_starting_on_its_charts_closed_column_parts_at_its_knots() {
        let poly = [
            (0.0, 0.0),
            (1.0, 0.0),
            (2.0, 0.0),
            (2.5, 0.0),
            (2.5, 1.0),
            (2.0, 1.0),
            (1.0, 1.0),
            (0.0, 1.0),
        ];
        let mut solid = tube(&poly);
        split(&mut solid).expect("the face on the closed column splits");
        let pieces = tube_pieces(&solid, true);
        let want = [(0.0, 1.0), (1.0, 2.0), (2.0, 2.5)];
        assert_eq!(pieces.len(), 3, "three pieces: {pieces:?}");
        for (got, want) in pieces.iter().zip(want) {
            assert!(
                (got.0 - want.0).abs() < 1e-9 && (got.1 - want.1).abs() < 1e-9,
                "pieces {pieces:?}"
            );
        }
    }

    /// The tube face over `[1.5, 4]` walked clockwise on a face whose
    /// `sense` agrees with `S_u × S_v`: walking its closed column down
    /// sides it below the knots while its neighbours lie above, so the
    /// loop changes side at a corner off every knot line.
    #[test]
    fn a_tube_loop_against_its_sense_changes_side_away_from_the_line_and_refuses_typed() {
        let mut poly = vec![
            (1.5, 0.0),
            (2.0, 0.0),
            (3.0, 0.0),
            (4.0, 0.0),
            (4.0, 1.0),
            (3.0, 1.0),
            (2.0, 1.0),
            (1.5, 1.0),
        ];
        poly.reverse();
        let refused = split(&mut tube(&poly));
        assert!(
            refused_for(&refused, "away from it"),
            "the side changes off the line: {refused:?}"
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
        split_faces(
            &mut solid,
            crate::interpret::FileBand::test(EPS_IN),
            &mut mint,
            &mut sink,
        )
        .unwrap();
        assert_eq!((solid.faces.len(), sink.len()), (1, 0));
    }

    /// The side test against crossings built between the samples: each
    /// `kept` row asserts a line crossed by more than ε_in is never left
    /// out.
    mod crossings_between_samples {
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
            Curve3::circle(
                Point3::new(cx, cy, 0.0),
                Vec3::new(0.0, 0.0, 1.0),
                r,
                Vec3::new(1.0, 0.0, 0.0),
            )
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
                    // The peak: a coarse grid, then golden sections about
                    // its best sample (the tip is one smooth bump).
                    let max_x = |c: &NurbsCurve3<f64>| {
                        let x = |t: f64| c.eval(t).x;
                        let best = (0..=1000)
                            .map(|i| f64::from(i) / 1000.0)
                            .fold(0.0, |b, t| if x(t) > x(b) { t } else { b });
                        let (mut lo, mut hi) = ((best - 1e-3).max(0.0), (best + 1e-3).min(1.0));
                        let g = geom_core::GOLDEN_SECTION;
                        for _ in 0..80 {
                            let (m1, m2) = (hi - g * (hi - lo), lo + g * (hi - lo));
                            if x(m1) < x(m2) {
                                lo = m1;
                            } else {
                                hi = m2;
                            }
                        }
                        x(best).max(x(0.5 * (lo + hi)))
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
                    let back = Curve3::line(Point3::new(0.3, 0.8, 0.0), Vec3::new(0.0, -0.6, 0.0));
                    let s = face(
                        chart([0.0, 0.5, 1.0]),
                        vec![(Curve3::Nurbs(Arc::new(spl)), 0.0, 1.0), (back, 0.0, 1.0)],
                    );
                    let got = crossing_knots(&s, 0, false, EPS_IN);
                    if max_x - 0.5 > EPS_IN && got != vec![0.5] {
                        bad.push((k, tip));
                    }
                }
            }
            assert!(bad.is_empty(), "left out though crossed: {bad:?}");
        }

        /// Trims that touch, stop short of, or poke past the line by less
        /// than ε_in certify clear.
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
                    // Stopping short, touching, or poking past the line by
                    // less than ε_in: on the line, so the face keeps its
                    // chart, a shallow tangency included.
                    assert_eq!(
                        crossing_knots(&s, 0, false, EPS_IN),
                        Vec::<f64>::new(),
                        "k={k} r={r}"
                    );
                }
            }
        }
    }
}
