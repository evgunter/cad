//! **The regularity sweep of a plane × spline pose**: stage 3 of the
//! pair's classification (C5), for `φ = n·(S − o)`, the height of the
//! wall's point over the plane. Over the plane, `∇φ = n·[S_u S_v]`,
//! so `∇φ` bounded off zero is the wall's normal bounded off `±n`.
//!
//! The wall's knot rectangle is swept cell by cell, starting from its
//! span cells, in certification arithmetic. A cell is:
//!
//! - **regular** where `n·S_u` or `n·S_v` is certified one-signed over
//!   it, the derivative boxes the closed cell's own
//!   ([`NurbsBoxes::deriv_box_closed`]);
//! - **excluded** where `φ` is certified past the band's escalation
//!   over it, read on the wall's first-order box over the cell
//!   ([`NurbsBoxes::rect_box`], C3's exclusion): so is `φ − c` for every
//!   height `c` within the band;
//! - neither, and halved in both axes, to [`FLOOR`] halvings of its span
//!   cell; a cell that is neither at the floor declines the sweep.
//!
//! **What the verdict proves.** Let every cell be regular or excluded,
//! and every cell that is not regular be joined to the rectangle's
//! boundary through cells excluded on its own side of the plane (cells
//! joined where their closed rectangles meet). Then for every `c` within
//! the band no component of `φ = c` is closed inside the rectangle. Such
//! a component avoids the excluded cells, so it lies where `∇φ ≠ 0`, a
//! simple closed curve bounding a disc `D` inside the rectangle, and `φ`
//! takes an extremum at a point `p` inside `D` where `∇φ = 0`: `p` lies
//! in a cell that is not regular. Its chain of excluded cells is a
//! connected set from `p` to the rectangle's boundary, outside `D`, on
//! which `φ − c` keeps one sign, yet it must cross `∂D`, where `φ = c`.
//! A critical point the band excludes but a loop of the section
//! encloses (a dome's top above the plane cutting it) is never joined
//! so, and declines.
//!
//! The argument needs `∇φ` continuous: at a crease (a knot of
//! multiplicity at least the degree) each side's one-sided gradient may
//! be off zero while `φ` peaks along the crease, so a wall with a crease
//! inside its rectangle declines before any cell is read.
//!
//! **Where the zero set may end**: on a side of the rectangle unless
//! the side's edge is excluded, read as the cells are, halved to the
//! floor where it is not. So every component of
//! the section is open in the wall's chart, its ends on those sides
//! ([`super::super::Components::Reaching`]).
//!
//! **What a declined cell is**: a cell neither regular nor excluded at
//! the floor holds a critical point of `φ` within the band of the plane,
//! or one the boxes cannot tell from it. Isolating it (Krawczyk on
//! `∇φ = 0`) and deciding its Hessian (a touch or a node) is the rest of
//! this stage; it plugs in here, at [`Declined::Floor`], deciding the
//! cell. Until it does, a declined sweep leaves the pose to C3's whole
//! reading.

use geom::NurbsSurface;
use geom_core::{Band, Bounds, Interval};

use super::super::ChartReach;
use super::Heights;
use crate::ssi::enclose::{NurbsBoxes, UvWindow};
use crate::ssi::section::sign;
use crate::ssi::{ChartAxis, ChartEnd, ChartSide};

/// The most halvings of a span cell the sweep takes before a cell that
/// is neither regular nor excluded declines it.
pub(super) const FLOOR: u32 = 10;

/// The most cells the sweep reads before it declines: it bounds the
/// sweep's cost where the floor alone would not, a wall whose every
/// span holds a near-critical stretch.
pub(super) const BUDGET: usize = 1 << 14;

/// What the sweep answers.
#[derive(Clone, Copy, Debug, PartialEq)]
pub(super) enum Sweep {
    /// No component of the section is closed in the wall's chart; each
    /// ends on these sides.
    LoopFree(ChartReach),
    /// The sweep proves nothing: the pose is read whole.
    Declined(Declined),
}

/// Why the sweep declined.
#[derive(Clone, Copy, Debug, PartialEq)]
pub(super) enum Declined {
    /// A crease lies inside the rectangle.
    Crease,
    /// A cell neither regular nor excluded at the floor, `[u0, u1, v0,
    /// v1]`: where an isolated critical point is decided.
    Floor([f64; 4]),
    /// The sweep read [`BUDGET`] cells.
    Budget,
    /// A cell that is not regular, excluded on one side, and joined to
    /// the rectangle's boundary through no cell excluded on that side:
    /// a critical point a loop of the section may enclose.
    Enclosed([f64; 4]),
    /// The rectangle has no span cell, or an enclosure refused.
    Unread,
}

/// A cell of the rectangle and how many halvings of its span cell it is.
#[derive(Clone, Copy, Debug)]
struct Cell {
    u: (f64, f64),
    v: (f64, f64),
    depth: u32,
}

impl Cell {
    fn window(self) -> Option<UvWindow> {
        UvWindow::new(self.u.0, self.u.1, self.v.0, self.v.1)
    }

    fn corners(self) -> [f64; 4] {
        [self.u.0, self.u.1, self.v.0, self.v.1]
    }

    /// The four halves, or `None` where a side has no `f64` between its
    /// ends.
    fn halves(self) -> Option<[Self; 4]> {
        let (um, vm) = (0.5 * (self.u.0 + self.u.1), 0.5 * (self.v.0 + self.v.1));
        let inside = |m: f64, (a, b): (f64, f64)| a < m && m < b;
        if !(inside(um, self.u) && inside(vm, self.v)) {
            return None;
        }
        let depth = self.depth + 1;
        Some(
            [
                ((self.u.0, um), (self.v.0, vm)),
                ((um, self.u.1), (self.v.0, vm)),
                ((self.u.0, um), (vm, self.v.1)),
                ((um, self.u.1), (vm, self.v.1)),
            ]
            .map(|(u, v)| Self { u, v, depth }),
        )
    }

    /// The closed cells meet.
    fn meets(self, o: Self) -> bool {
        self.u.0 <= o.u.1 && o.u.0 <= self.u.1 && self.v.0 <= o.v.1 && o.v.0 <= self.v.1
    }
}

/// A cell read: whether it is regular, and the side of the plane it is
/// excluded on (`Some(true)` above), if it is.
#[derive(Clone, Copy, Debug)]
struct Leaf {
    cell: Cell,
    regular: bool,
    excluded: Option<bool>,
}

/// The breakpoints of a knot vector inside its domain, ascending and
/// distinct: the ends of its nonempty spans.
fn breakpoints(knots: &geom_core::spline::Knots<f64>) -> Vec<f64> {
    let (lo, hi) = knots.domain();
    let mut out: Vec<f64> = knots
        .runs()
        .map(|(t, _)| t)
        .filter(|&t| lo <= t && t <= hi)
        .collect();
    out.dedup();
    out
}

/// **The sweep of `net` against the plane `heights` reads**, at `band`
/// (module docs).
pub(super) fn sweep(net: &NurbsSurface<f64>, heights: Heights, band: Band) -> Sweep {
    let (us, vs) = (
        breakpoints(net.knot_values_u()),
        breakpoints(net.knot_values_v()),
    );
    let (Some(&u_lo), Some(&u_hi), Some(&v_lo), Some(&v_hi)) =
        (us.first(), us.last(), vs.first(), vs.last())
    else {
        return Sweep::Declined(Declined::Unread);
    };
    let boxes = NurbsBoxes::new(net);
    let Some(domain) = UvWindow::new(u_lo, u_hi, v_lo, v_hi) else {
        return Sweep::Declined(Declined::Unread);
    };
    if !boxes.first_order_continuous_in(domain) {
        return Sweep::Declined(Declined::Crease);
    }
    let m = heights.normal.map(Interval::from_certified);
    // `φ` is read in the stored normal's units; a height past the
    // escalation in metres is one past it times the normal's length,
    // bounded above.
    let length =
        (heights.normal.x.abs() + heights.normal.y.abs() + heights.normal.z.abs()) * (1.0 + 1e-12);
    let reach = band.escalate() * length;
    let read = |cell: Cell| -> Option<Leaf> {
        let w = cell.window()?;
        let (su, sv) = (
            boxes.deriv_box_closed(w, true),
            boxes.deriv_box_closed(w, false),
        );
        let gu = su.x * m.x + su.y * m.y + su.z * m.z;
        let gv = sv.x * m.x + sv.y * m.y + sv.z * m.z;
        let phi = heights.over(boxes.rect_box(w));
        if !(gu.is_certified() && gv.is_certified() && phi.is_certified()) {
            return None;
        }
        let excluded = if phi.lo() >= reach {
            Some(true)
        } else if phi.hi() <= -reach {
            Some(false)
        } else {
            None
        };
        Some(Leaf {
            cell,
            regular: sign(gu).is_some() || sign(gv).is_some(),
            excluded,
        })
    };
    let mut stack: Vec<Cell> = us
        .windows(2)
        .flat_map(|u| {
            vs.windows(2).map(move |v| Cell {
                u: (u[0], u[1]),
                v: (v[0], v[1]),
                depth: 0,
            })
        })
        .filter(|c| c.u.0 < c.u.1 && c.v.0 < c.v.1)
        .collect();
    let mut leaves = Vec::new();
    let mut spent = 0;
    while let Some(cell) = stack.pop() {
        spent += 1;
        if spent > BUDGET {
            return Sweep::Declined(Declined::Budget);
        }
        let Some(leaf) = read(cell) else {
            return Sweep::Declined(Declined::Unread);
        };
        if leaf.regular || leaf.excluded.is_some() {
            leaves.push(leaf);
            continue;
        }
        match cell.halves() {
            Some(halves) if cell.depth < FLOOR => stack.extend(halves),
            _ => return Sweep::Declined(Declined::Floor(cell.corners())),
        }
    }
    if leaves.is_empty() {
        return Sweep::Declined(Declined::Unread);
    }
    let on_boundary = |c: Cell| c.u.0 <= u_lo || c.u.1 >= u_hi || c.v.0 <= v_lo || c.v.1 >= v_hi;
    for side in [true, false] {
        // Only the cells excluded on this side join; the walk is over
        // them alone.
        let on: Vec<usize> = (0..leaves.len())
            .filter(|&i| leaves[i].excluded == Some(side))
            .collect();
        if on.iter().all(|&i| leaves[i].regular) {
            continue;
        }
        let mut joined: Vec<bool> = on.iter().map(|&i| on_boundary(leaves[i].cell)).collect();
        let mut frontier: Vec<usize> = (0..on.len()).filter(|&k| joined[k]).collect();
        while let Some(k) = frontier.pop() {
            let here = leaves[on[k]].cell;
            for (j, &i) in on.iter().enumerate() {
                if !joined[j] && leaves[i].cell.meets(here) {
                    joined[j] = true;
                    frontier.push(j);
                }
            }
        }
        if let Some(&i) = on
            .iter()
            .zip(&joined)
            .find(|&(&i, &j)| !leaves[i].regular && !j)
            .map(|(i, _)| i)
        {
            return Sweep::Declined(Declined::Enclosed(leaves[i].cell.corners()));
        }
    }
    // A side is reached where its edge is not excluded: each edge read
    // whole, then halved to the floor where it is not.
    let excluded = |w: UvWindow| {
        let phi = heights.over(boxes.rect_box(w));
        phi.is_certified() && (phi.lo() >= reach || phi.hi() <= -reach)
    };
    let edge = |fixed: ChartAxis, at: f64, (a, b): (f64, f64)| match fixed {
        ChartAxis::U => UvWindow::new(at, at, a, b),
        ChartAxis::V => UvWindow::new(a, b, at, at),
    };
    let side = |fixed, end| ChartSide { fixed, end };
    let sides = [
        (side(ChartAxis::U, ChartEnd::Low), u_lo, (v_lo, v_hi)),
        (side(ChartAxis::U, ChartEnd::High), u_hi, (v_lo, v_hi)),
        (side(ChartAxis::V, ChartEnd::Low), v_lo, (u_lo, u_hi)),
        (side(ChartAxis::V, ChartEnd::High), v_hi, (u_lo, u_hi)),
    ];
    let mut reaches = ChartReach::default();
    for (s, at, span) in sides {
        let mut stack = vec![(span, 0)];
        while let Some(((a, b), depth)) = stack.pop() {
            let Some(w) = edge(s.fixed, at, (a, b)) else {
                return Sweep::Declined(Declined::Unread);
            };
            if excluded(w) {
                continue;
            }
            let m = 0.5 * (a + b);
            if depth >= FLOOR + 4 || !(a < m && m < b) {
                reaches = reaches.or(ChartReach::side(s));
                break;
            }
            stack.extend([((a, m), depth + 1), ((m, b), depth + 1)]);
        }
    }
    Sweep::LoopFree(reaches)
}

#[cfg(test)]
mod rows;
#[cfg(test)]
mod rows_review;
