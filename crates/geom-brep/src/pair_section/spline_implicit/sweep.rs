//! **The regularity sweep of a spline × implicit pose**: stage 3 of the
//! pair's classification (C5), for `φ = f∘S`. Against a plane `φ` is
//! the height of the wall's point over it, `φ = n·(S − o)`, and
//! `∇φ = n·[S_u S_v]`, so `∇φ` bounded off zero is the wall's normal
//! bounded off `±n`; against a cylinder, sphere or torus `φ` is its
//! canonical form along the wall ([`Curved`]), and `∇φ = ∇f(S)·[S_u S_v]`.
//!
//! The wall's knot rectangle is swept by the chart lane's one
//! recursion ([`sweep_chart_regular`], shared with seeding and C3's
//! accounting), to the accounting floor. A cell is:
//!
//! - **excluded** where `φ` over it is decided past the band
//!   (`spline_sweep_excluded`): on the plane its height on the
//!   accounting's own enclosure
//!   ([`crate::ssi::exhaust::chart_plane_height`]), over the normal's
//!   length; on a curved partner `f` over the cell's box, a floor in
//!   metres ([`Curved::floor_metres`]). So is `φ − c` for every `c`
//!   within the band, the band read in metres;
//! - **regular** where the wall's slope off the partner along `u` or
//!   `v`, `∇f·S_d / (sup‖∇f‖·sup‖S_d‖)` on the closed cell's derivative
//!   boxes ([`NurbsBoxes::deriv_box_closed`], [`NurbsBoxes::speed_sup`])
//!   and, on a curved partner, `∇f` over the cell's box (`n` and its
//!   length on the plane), levered by the feature extent, is decided off
//!   zero (`spline_sweep_regular`), as the march decides
//!   transversality;
//! - neither, and halved down to the floor, where it is a critical
//!   point's to decide ([`critical`]).
//!
//! **What the verdict proves.** Let every cell be regular or excluded,
//! and every excluded cell that is not regular be joined to the
//! rectangle's boundary through cells excluded on its own side of the
//! plane (cells joined where their closed rectangles meet). Then for
//! every `c` within the band no component of `φ = c` is closed inside
//! the rectangle. Such a component avoids the excluded cells, so it lies
//! where `∇φ ≠ 0`, a simple closed curve bounding a disc `D` inside the
//! rectangle, and `φ` takes an extremum at a point `p` inside `D` where
//! `∇φ = 0`: `p` lies in a cell that is not regular. Its chain of
//! excluded cells is a connected set from `p` to the rectangle's
//! boundary, outside `D`, on which `φ − c` keeps one sign, yet it must
//! cross `∂D`, where `φ = c`. A critical point the band excludes but a
//! loop of the section encloses (a dome's top above the plane cutting
//! it) is never joined so, and declines.
//!
//! **Outside the decided touches.** A cluster of floor cells decided as a
//! critical point of value zero ([`critical`]) is exempt from the join:
//! about a touch, `φ`'s level sets at values within the band are small
//! closed curves, the touch itself at the zero level, so the verdict is
//! that no component closes outside the decided points, which are the
//! section there. An extremum `p` of the argument above lies in a
//! decided cluster, or in a cell the argument reaches.
//!
//! That each component ends where the boundary pass found a crossing or
//! a region, the sides the verdict names, rests on stage 2 having
//! refused every graze of a side ([`crate::ssi::boundary`]'s pass): a
//! component touching a side without crossing it leaves no crossing
//! there. That the verdict covers the zero set at all rests on stage 1:
//! a wall within the band of the partner everywhere has no level set the
//! band parts from its neighbours, and its cells can each read regular.
//!
//! **The crease pass.** The argument needs `∇φ` continuous, which a
//! crease (a knot of multiplicity at least the degree) breaks: each
//! side's one-sided gradient may be off zero while `φ` peaks along the
//! crease. So the rectangle is cut at its creases and each piece swept as
//! above, and each crease line is read along itself, its pieces excluded
//! or regular along the line (`∇f·S_d` off zero, `d` the line's own
//! direction, continuous across the crease), down to the floor; each
//! crossing of two creases is read as a point, which must be excluded.
//! An extremum `p` on a crease line, not at a crossing, is a critical
//! point of `φ` along it, so it lies in a piece not regular along the
//! line; at a crossing it is that point; either way the join reaches it
//! as it reaches a cell, the crease's excluded pieces and points among
//! the cells it joins.
//!
//! **A cell at the floor** neither regular nor excluded is a critical
//! point of `φ` within the band, or one the boxes cannot tell from it.
//! The cells are clustered and each cluster decided ([`critical`]): a
//! touch or a node of value zero is decided Zero; anything else declines
//! the sweep at [`Declined::Floor`], and a plane's pose falls to C3's
//! whole reading.

use std::cell::RefCell;

use geom_core::k_stats::decide;
use geom_core::{Band, Interval, Margin, Point3, Real, Sign, Vec3};

use crate::ssi::SsiError;
use crate::ssi::enclose::{NurbsBoxes, UvWindow};
use crate::ssi::exhaust::{
    REGULAR_SURVIVORS, SweepFloor, UvRect, chart_plane_height, sweep_chart_regular,
};
use crate::ssi::residual::Curved;

mod critical;

use critical::Jets;
pub(in crate::pair_section) use critical::{Critical, Kind};

/// **The partner the sweep reads `φ` of.**
#[derive(Clone, Copy)]
pub(super) enum Phi<'a> {
    /// A plane, read at its pivot: its origin and its normal as stored.
    Plane(Point3<f64>, Vec3<f64>),
    /// A cylinder, sphere or torus.
    Curved(&'a Curved),
}

/// What the sweep answers.
#[derive(Clone, Debug, PartialEq)]
pub(super) enum Sweep {
    /// The critical points of `φ` within the band, each decided Zero, and
    /// outside them no component of the section is closed in the wall's
    /// chart.
    LoopFree(Vec<Critical>),
    /// The sweep proves nothing: the pose is read whole.
    Declined(Declined),
}

/// Why the sweep declined.
#[derive(Clone, Copy, Debug, PartialEq)]
pub(super) enum Declined {
    /// The wall's gradient jumps inside a rectangle between its creases.
    Crease,
    /// A cell neither regular nor excluded at the floor, at its centre,
    /// whose critical point is not decided: not isolated, its Hessian
    /// singular, or its value not decided zero; or a piece of a crease
    /// line, or a crossing of two, neither regular along the line nor
    /// excluded.
    Floor((f64, f64)),
    /// The shared recursion's cell budget ran out, or more cells reached
    /// the floor than critical points are decided for.
    Budget,
    /// A cell that is not regular, excluded on one side, and joined to
    /// the rectangle's boundary through no cell excluded on that side: a
    /// critical point a loop of the section may enclose, as `[u0, u1,
    /// v0, v1]`.
    Enclosed([f64; 4]),
    /// The rectangle has no domain, or an enclosure refused.
    Unread,
}

/// An excluded cell: the side of the plane it lies on (`true` above),
/// and whether it is regular too.
#[derive(Clone, Copy, Debug)]
struct Excluded {
    cell: UvRect,
    above: bool,
    regular: bool,
}

/// **The sweep of a wall against its partner `phi`** to `floor`, at
/// `band` (module docs); `extent` is the feature extent the slopes are
/// levered by.
pub(super) fn sweep(
    boxes: &NurbsBoxes<'_, f64>,
    phi: Phi<'_>,
    floor: SweepFloor<UvRect>,
    band: Band,
    extent: f64,
) -> Sweep {
    let net = boxes.surface();
    let (ud, vd) = (net.knot_values_u().domain(), net.knot_values_v().domain());
    let root = UvRect { u: ud, v: vd };
    // Both callers mint the floor over the wall's own knot rectangle
    // (`PlaneSection::of`, `ImplicitWall::of`).
    debug_assert_eq!(
        floor.root(),
        root,
        "the floor is minted over the wall's rectangle"
    );
    if root.window().is_none() {
        return Sweep::Declined(Declined::Unread);
    }
    let arm = Interval::from_certified(extent);
    let off = |name, margin| {
        matches!(
            decide(name, margin, band),
            Ok(Sign::Positive | Sign::Negative)
        )
    };
    // `∇f` over a window, and an upper bound on its length there.
    let gradient = |w: UvWindow| match phi {
        Phi::Plane(_, normal) => {
            let n = normal.map(Interval::from_certified);
            let length = (n.x.powi(2) + n.y.powi(2) + n.z.powi(2)).sqrt();
            (n, length)
        }
        Phi::Curved(curved) => {
            let q = curved.framed(boxes.rect_box(w));
            (
                curved.gradient(q),
                Interval::from_certified(curved.gradient_sup(q)),
            )
        }
    };
    // The wall's slope off the partner along `u` (`along_u`) or `v` over
    // a window, decided off zero.
    let slope_off = |w: UvWindow, along_u: bool| {
        let (n, length) = gradient(w);
        let speed = boxes.speed_sup(w, along_u);
        if !(speed.is_finite() && speed > 0.0) {
            return false;
        }
        let d = boxes.deriv_box_closed(w, along_u);
        let slope = sine(
            n,
            length,
            Vec3::new(d.x, d.y, d.z),
            Interval::from_certified(speed),
        );
        off(
            "spline_sweep_regular",
            Margin::levered(slope, geom_core::Lever::unsourced(arm)),
        )
    };
    let regular = |cell: UvRect| {
        cell.window().is_some_and(|w| {
            [true, false]
                .into_iter()
                .any(|along_u| slope_off(w, along_u))
        })
    };
    // `φ` over a cell, in metres: a floor on its distance, signed.
    let value = |cell: UvRect| match phi {
        Phi::Plane(origin, normal) => {
            let n = normal.map(Interval::from_certified);
            let length = (n.x.powi(2) + n.y.powi(2) + n.z.powi(2)).sqrt();
            chart_plane_height(boxes, origin, normal, cell).map(|height| height / length)
        }
        Phi::Curved(curved) => {
            let b = cell
                .window()
                .map_or_else(crate::ssi::enclose::refused_box, |w| boxes.rect_box(w));
            let q = curved.framed(b);
            let metres = curved.floor_metres(curved.value(q), q);
            if metres.is_certified() {
                Ok(metres)
            } else {
                Err(SsiError::UnsupportedCertificate {
                    what: "a curved partner's form over a cell of the wall does not certify",
                })
            }
        }
    };
    // The side of the partner a cell is decided on (`true` above), or
    // `None` where it is not excluded.
    let side = |cell: UvRect, metres: Interval| {
        match decide("spline_sweep_excluded", Margin::of(metres), band) {
            Ok(Sign::Positive) => Some(true),
            Ok(Sign::Negative) => Some(false),
            _ => None,
        }
        .filter(|_| cell.window().is_some())
    };
    let excluded = RefCell::new(Vec::new());
    let mut survivors: Vec<((f64, f64), UvRect)> = Vec::new();
    let (cu, cv) = (
        creases(net.knot_values_u(), ud),
        creases(net.knot_values_v(), vd),
    );
    for sub in pieces(root, &cu, &cv) {
        let Some(w) = sub.window() else {
            return Sweep::Declined(Declined::Unread);
        };
        if !boxes.first_order_continuous_in(w) {
            return Sweep::Declined(Declined::Crease);
        }
        let Some(within) = floor.within(sub) else {
            return Sweep::Declined(Declined::Unread);
        };
        let read = sweep_chart_regular(
            value,
            within,
            |cell, metres| {
                let Some(above) = side(cell, metres) else {
                    return false;
                };
                excluded.borrow_mut().push(Excluded {
                    cell,
                    above,
                    regular: regular(cell),
                });
                true
            },
            &regular,
        );
        match read {
            Err(SsiError::CellBudget { .. }) => return Sweep::Declined(Declined::Budget),
            Err(_) => return Sweep::Declined(Declined::Unread),
            Ok((at, _)) => survivors.extend(at.into_iter().map(|c| (c, sub))),
        }
        if survivors.len() > REGULAR_SURVIVORS {
            return Sweep::Declined(Declined::Budget);
        }
    }
    // The crease pass: each crease line read along itself, and each
    // crossing of two read as a point.
    let jets = (!cu.is_empty() || !cv.is_empty() || !survivors.is_empty())
        .then(|| Jets::of(net))
        .flatten();
    let iv = |r: (f64, f64)| Interval::from_bounds(r.0, r.1);
    let lines = cu
        .iter()
        .map(|&k| (UvRect { u: (k, k), v: vd }, false))
        .chain(cv.iter().map(|&k| (UvRect { u: ud, v: (k, k) }, true)));
    // Each axis's interior knots, at which a line is cut before it is
    // read. A piece is read open at a knot it is cut at, one float in, so
    // the jet reads that span's polynomial alone: at a closed knot it would
    // hull the next span's too. A crossing of two creases is then read by
    // the crossing pass below and by no line piece, and an extremum at a
    // knot that is no crease has a derivative along the line that is
    // continuous and zero there, which the pieces beside it read.
    let knots = |k: &geom_core::spline::Knots<f64>, d: (f64, f64)| {
        k.runs()
            .map(|(t, _)| t)
            .filter(|&t| d.0 < t && t < d.1)
            .collect::<Vec<_>>()
    };
    let (ku, kv) = (
        knots(net.knot_values_u(), ud),
        knots(net.knot_values_v(), vd),
    );
    for (line, along_u) in lines {
        let cuts = if along_u { &ku } else { &kv };
        let mut stack: Vec<UvRect> = pieces(
            line,
            if along_u { cuts } else { &[] },
            if along_u { &[] } else { cuts },
        );
        stack.reverse();
        let mut read = 0usize;
        while let Some(piece) = stack.pop() {
            read += 1;
            if read > crate::ssi::exhaust::SSI_MAX_CELLS {
                return Sweep::Declined(Declined::Budget);
            }
            let Some(jets) = &jets else {
                return Sweep::Declined(Declined::Unread);
            };
            let open = |(a, b): (f64, f64)| {
                let a = if cuts.contains(&a) { a.next_up() } else { a };
                let b = if cuts.contains(&b) { b.next_down() } else { b };
                Interval::from_bounds(a, b.max(a))
            };
            let at = if along_u {
                (open(piece.u), iv(piece.v))
            } else {
                (iv(piece.u), open(piece.v))
            };
            let (metres, slope) = critical::along_line(jets, phi, at, along_u);
            // A reading that does not certify is halved as an undecided
            // one is, down to the floor.
            let read = metres.is_certified() && slope.is_certified();
            let along = read
                && off(
                    "spline_sweep_regular",
                    Margin::levered(slope, geom_core::Lever::unsourced(arm)),
                );
            if let Some(above) = side(piece, metres).filter(|_| read) {
                excluded.borrow_mut().push(Excluded {
                    cell: piece,
                    above,
                    regular: along,
                });
                continue;
            }
            if along {
                continue;
            }
            let (a, b) = if along_u { piece.u } else { piece.v };
            if b - a <= floor.width() {
                return Sweep::Declined(Declined::Floor((
                    0.5 * (piece.u.0 + piece.u.1),
                    0.5 * (piece.v.0 + piece.v.1),
                )));
            }
            let m = 0.5 * (a + b);
            let (lo, hi) = if along_u {
                (UvRect { u: (a, m), ..piece }, UvRect { u: (m, b), ..piece })
            } else {
                (UvRect { v: (a, m), ..piece }, UvRect { v: (m, b), ..piece })
            };
            stack.push(hi);
            stack.push(lo);
        }
    }
    for &ku in &cu {
        for &kv in &cv {
            let point = UvRect {
                u: (ku, ku),
                v: (kv, kv),
            };
            let read = jets
                .as_ref()
                .map(|j| critical::along_line(j, phi, (iv(point.u), iv(point.v)), true).0);
            let Some(above) = read.and_then(|m| side(point, m)) else {
                return Sweep::Declined(Declined::Floor((ku, kv)));
            };
            excluded.borrow_mut().push(Excluded {
                cell: point,
                above,
                regular: false,
            });
        }
    }
    // The cells no reading decided, each cluster decided as one critical
    // point.
    let mut decided = Vec::new();
    if !survivors.is_empty() {
        let Some(jets) = &jets else {
            return Sweep::Declined(Declined::Unread);
        };
        for (cluster, sub) in clusters(&survivors, floor.width()) {
            match critical::decide(jets, phi, cluster, band) {
                Some(c)
                    if c.cell.u.0 > sub.u.0
                        && c.cell.u.1 < sub.u.1
                        && c.cell.v.0 > sub.v.0
                        && c.cell.v.1 < sub.v.1 =>
                {
                    decided.push(c)
                }
                _ => {
                    return Sweep::Declined(Declined::Floor((
                        0.5 * (cluster.u.0 + cluster.u.1),
                        0.5 * (cluster.v.0 + cluster.v.1),
                    )));
                }
            }
        }
    }
    match joined(&excluded.into_inner(), root) {
        Sweep::LoopFree(_) => Sweep::LoopFree(decided),
        declined => declined,
    }
}

/// **The wall's slope off the partner along a direction `d`**, a sine:
/// `∇f·d` over an upper bound `g_len` on `‖∇f‖` times one, `d_len`, on
/// `‖d‖`. The sweep's cells and the crease pass's line pieces read it
/// alike, the one levered by the feature extent against the band.
pub(super) fn sine(
    g: Vec3<Interval>,
    g_len: Interval,
    d: Vec3<Interval>,
    d_len: Interval,
) -> Interval {
    g.dot(d) / (g_len * d_len)
}

/// How far a cluster's box reaches past each of its cells' centres, in
/// floor widths ([`clusters`]): half a width covers the cell, the rest is
/// room for the Krawczyk test.
const CLUSTER_REACH: f64 = 1.5;

/// The creases of `knots` strictly inside `domain`, ascending.
fn creases(knots: &geom_core::spline::Knots<f64>, domain: (f64, f64)) -> Vec<f64> {
    let mut out: Vec<f64> = knots
        .crease_values()
        .into_iter()
        .filter(|&k| domain.0 < k && k < domain.1)
        .collect();
    out.sort_by(f64::total_cmp);
    out.dedup();
    out
}

/// The rectangles `root` is cut into by the creases `cu` in `u` and `cv`
/// in `v`.
fn pieces(root: UvRect, cu: &[f64], cv: &[f64]) -> Vec<UvRect> {
    let cuts = |d: (f64, f64), k: &[f64]| {
        let mut b = vec![d.0];
        b.extend_from_slice(k);
        b.push(d.1);
        b.windows(2).map(|w| (w[0], w[1])).collect::<Vec<_>>()
    };
    let (us, vs) = (cuts(root.u, cu), cuts(root.v, cv));
    us.iter()
        .flat_map(|&u| vs.iter().map(move |&v| UvRect { u, v }))
        .collect()
}

/// **The floor cells clustered**: cells within two floor widths of each
/// other in both coordinates, in one sub-rectangle, are one cluster, its
/// box their cells' hull widened by a floor width each way and cut to
/// the sub-rectangle.
///
/// The box must cover every cell of its cluster, since what the Krawczyk
/// test proves is about the box (its one zero of `∇φ`, and none beside
/// it), and a cell left partly outside it would leave part of an
/// unread cell undecided. A cell reaches the floor only once its widest
/// side is at most `width` (the shared recursion's floor test,
/// `cell.width() <= width`), so it lies within half a width of its centre
/// on both axes: the reach of [`CLUSTER_REACH`] widths covers it with a
/// width to spare, the room the test needs about a zero on a cell's
/// edge.
fn clusters(cells: &[((f64, f64), UvRect)], width: f64) -> Vec<(UvRect, UvRect)> {
    let mut parent: Vec<usize> = (0..cells.len()).collect();
    for i in 0..cells.len() {
        for j in i + 1..cells.len() {
            let ((a, sa), (b, sb)) = (cells[i], cells[j]);
            if sa == sb && (a.0 - b.0).abs() <= 2.0 * width && (a.1 - b.1).abs() <= 2.0 * width {
                let (ra, rb) = (find(&mut parent, i), find(&mut parent, j));
                parent[ra] = rb;
            }
        }
    }
    let mut out: Vec<(usize, UvRect, UvRect)> = Vec::new();
    for (i, &(c, sub)) in cells.iter().enumerate() {
        let r = find(&mut parent, i);
        let reach = CLUSTER_REACH * width;
        let cell = UvRect {
            u: ((c.0 - reach).max(sub.u.0), (c.0 + reach).min(sub.u.1)),
            v: ((c.1 - reach).max(sub.v.0), (c.1 + reach).min(sub.v.1)),
        };
        match out.iter_mut().find(|(k, _, _)| *k == r) {
            Some((_, hull, _)) => {
                *hull = UvRect {
                    u: (hull.u.0.min(cell.u.0), hull.u.1.max(cell.u.1)),
                    v: (hull.v.0.min(cell.v.0), hull.v.1.max(cell.v.1)),
                };
            }
            None => out.push((r, cell, sub)),
        }
    }
    out.into_iter().map(|(_, hull, sub)| (hull, sub)).collect()
}

/// The root of `i`'s class, its path halved on the way.
fn find(parent: &mut [usize], mut i: usize) -> usize {
    while parent[i] != i {
        parent[i] = parent[parent[i]];
        i = parent[i];
    }
    i
}

/// **Whether every excluded cell that is not regular is joined to the
/// rectangle's boundary** through cells excluded on its side, by
/// union-find over the cells that meet (module docs).
fn joined(cells: &[Excluded], root: UvRect) -> Sweep {
    let mut parent: Vec<usize> = (0..cells.len()).collect();
    let meets =
        |a: UvRect, b: UvRect| a.u.0 <= b.u.1 && b.u.0 <= a.u.1 && a.v.0 <= b.v.1 && b.v.0 <= a.v.1;
    let mut order: Vec<usize> = (0..cells.len()).collect();
    order.sort_by(|&a, &b| cells[a].cell.u.0.total_cmp(&cells[b].cell.u.0));
    for (k, &i) in order.iter().enumerate() {
        for &j in &order[k + 1..] {
            if cells[j].cell.u.0 > cells[i].cell.u.1 {
                break;
            }
            if cells[i].above == cells[j].above && meets(cells[i].cell, cells[j].cell) {
                let (a, b) = (find(&mut parent, i), find(&mut parent, j));
                parent[a] = b;
            }
        }
    }
    let on_boundary = |c: UvRect| {
        c.u.0 <= root.u.0 || c.u.1 >= root.u.1 || c.v.0 <= root.v.0 || c.v.1 >= root.v.1
    };
    let mut free = vec![false; cells.len()];
    for (i, c) in cells.iter().enumerate() {
        if on_boundary(c.cell) {
            let r = find(&mut parent, i);
            free[r] = true;
        }
    }
    for (i, c) in cells.iter().enumerate() {
        if !c.regular && !free[find(&mut parent, i)] {
            return Sweep::Declined(Declined::Enclosed([
                c.cell.u.0, c.cell.u.1, c.cell.v.0, c.cell.v.1,
            ]));
        }
    }
    Sweep::LoopFree(Vec::new())
}

/// The sweep of the pair `p`, `w` at `band`, read as the arm reads it.
#[cfg(test)]
#[allow(clippy::expect_used)]
fn swept(p: &geom::Surface<f64>, w: &geom::Surface<f64>, band: Band) -> Sweep {
    let pose = super::SplineImplicitPose::of(p, w).expect("a plane and a spline");
    let pivot = pose.pivot();
    let band = pivot.band(band).expect("the shift pads the band");
    let reading = crate::PlaneSection::of(&pivot.plane, pose.net(), pose.domain(), band)
        .expect("the pair is read");
    sweep(
        reading.boxes(),
        pivot.phi(),
        reading.account_floor(),
        band,
        pose.domain().extent,
    )
}

#[cfg(test)]
mod rows;
#[cfg(test)]
mod rows_review;
