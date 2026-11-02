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
//!   (`plane_spline_sweep_excluded`): on the plane its height on the
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
//!   zero (`plane_spline_sweep_regular`), as the march decides
//!   transversality;
//! - neither, and halved; one such cell at the floor declines the sweep.
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
//! That each component ends where the boundary pass found a crossing or
//! a region, the sides the verdict names, rests on stage 2 having
//! refused every graze of a side ([`crate::ssi::boundary`]'s pass): a
//! component touching a side without crossing it leaves no crossing
//! there. That the verdict covers the zero set at all rests on stage 1:
//! a wall within the band of the partner everywhere has no level set the
//! band parts from its neighbours, and its cells can each read regular.
//!
//! The argument needs `∇φ` continuous: at a crease (a knot of
//! multiplicity at least the degree) each side's one-sided gradient may
//! be off zero while `φ` peaks along the crease, so a wall with a crease
//! inside its rectangle declines before any cell is read.
//!
//! **A declined cell** is one neither regular nor excluded at the floor:
//! a critical point of `φ` within the band of the plane, or one the
//! boxes cannot tell from it. Isolating it (Krawczyk on `∇φ = 0`) and
//! deciding its Hessian (a touch or a node) is the rest of this stage;
//! it plugs in at [`Declined::Floor`], deciding the cell. Until it does,
//! a declined sweep leaves the pose to C3's whole reading.

use std::cell::RefCell;

use geom_core::k_stats::decide;
use geom_core::{Band, Interval, Margin, Point3, Real, Sign, Vec3};

use crate::ssi::SsiError;
use crate::ssi::enclose::{NurbsBoxes, UvWindow};
use crate::ssi::exhaust::{SweepFloor, UvRect, chart_plane_height, sweep_chart_regular};
use crate::ssi::residual::Curved;

/// **The partner the sweep reads `φ` of.**
#[derive(Clone, Copy)]
pub(super) enum Phi<'a> {
    /// A plane, read at its pivot: its origin and its normal as stored.
    Plane(Point3<f64>, Vec3<f64>),
    /// A cylinder, sphere or torus.
    Curved(&'a Curved),
}

/// What the sweep answers.
#[derive(Clone, Copy, Debug, PartialEq)]
pub(super) enum Sweep {
    /// No component of the section is closed in the wall's chart.
    LoopFree,
    /// The sweep proves nothing: the pose is read whole.
    Declined(Declined),
}

/// Why the sweep declined.
#[derive(Clone, Copy, Debug, PartialEq)]
pub(super) enum Declined {
    /// A crease lies inside the rectangle.
    Crease,
    /// A cell neither regular nor excluded at the floor, at its centre:
    /// where an isolated critical point is decided.
    Floor((f64, f64)),
    /// The shared recursion's cell budget ran out.
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
    let Some(domain) = UvWindow::new(ud.0, ud.1, vd.0, vd.1) else {
        return Sweep::Declined(Declined::Unread);
    };
    if !boxes.first_order_continuous_in(domain) {
        return Sweep::Declined(Declined::Crease);
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
    let regular = |cell: UvRect| {
        let Some(w) = cell.window() else {
            return false;
        };
        let (n, length) = gradient(w);
        [true, false].into_iter().any(|along_u| {
            let speed = boxes.speed_sup(w, along_u);
            if !(speed.is_finite() && speed > 0.0) {
                return false;
            }
            let d = boxes.deriv_box_closed(w, along_u);
            let slope =
                (d.x * n.x + d.y * n.y + d.z * n.z) / (length * Interval::from_certified(speed));
            off("plane_spline_sweep_regular", Margin::levered(slope, arm))
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
    let excluded = RefCell::new(Vec::new());
    let declined = sweep_chart_regular(
        value,
        floor,
        |cell, metres| {
            let above = match decide("plane_spline_sweep_excluded", Margin::of(metres), band) {
                Ok(Sign::Positive) => true,
                Ok(Sign::Negative) => false,
                _ => return false,
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
    match declined {
        Err(SsiError::CellBudget { .. }) => Sweep::Declined(Declined::Budget),
        Err(_) => Sweep::Declined(Declined::Unread),
        Ok((Some(at), _)) => Sweep::Declined(Declined::Floor(at)),
        Ok((None, _regular)) => joined(&excluded.into_inner(), UvRect { u: ud, v: vd }),
    }
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
    Sweep::LoopFree
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
