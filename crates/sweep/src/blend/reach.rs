//! **Predicate 2's reach** — every band against every face of the body
//! that is not a support of its chain, in any shell.
//!
//! A convex band removes the material between its supports and its
//! blend surface, a concave one adds it: call that region the band's
//! REACH. Each face the chain does not blend is read three ways, before
//! any mutation: clear of it; crossing a region that is exactly a
//! band's material, a point of the face's own boundary inside it and
//! another outside ([`BlendError::FaceClearance`], `bounded: false`),
//! which would need the band trimmed by the face; or wholly inside such
//! a region, every cell of the face's box that may hold a point of its
//! surface inside it, or inside the prism a straight band removes or
//! fills, read through the cycles read inside its caps' slivers or its
//! supports' strips and the edges its
//! exact core holds ([`prism_inside`]); inside a straight band's material
//! the face goes with it ([`Consumed`]; elsewhere that carve is not
//! built, and the request refuses [`BlendError::WhollyInside`]). A
//! face certified none of the three refuses [`BlendError::FaceClearance`]
//! with `bounded: true`. The readings make one verdict for the request
//! with a cut-off's end-face meter's ([`Unclear`]): a crossing anywhere
//! wins, then a bound, so the request reads wholly inside only when
//! every face and end-face cycle not clear does.
//!
//! # The region
//!
//! Each reach is an intersection of closed sets `{g ≤ 0}`, every `g`
//! 1-Lipschitz in space. A link's cross-section, in its normal sheet,
//! is bounded by each support on the ball's side, the two rays from the
//! ball's centre through the feet, the outside of the ball and the disk
//! through the edge point (a chamfer's: the supports and the strip's
//! plane). Over plane supports these bounds ARE the cross-section; over
//! a curved support they enclose it.
//!
//! - A straight link runs over the edge's window, closed at each end by
//!   the plane of the face it runs into, and the window widens by the
//!   cross-section's run along it; at a corner patch it ends at the
//!   patch's anchors. A cut-off, a cap or a mitre ends the band within
//!   that plane — at a mitre, the other band's support. At an overrun
//!   the far band runs on past the mitre's end and is cut off by that
//!   same plane down to the third edge, so its material still ends
//!   there and the cap holds.
//! - A circular link is taken over the whole turn, every bound a
//!   function of its meridian sheet; a support off the band's axis is
//!   read about it with its departure as slack.
//! - A fillet corner patch at a trivalent vertex `v` of three requested
//!   links over planes, its ball centred at `C`: the three supports,
//!   outside the ball, inside the ball about `C` through `v`, the
//!   half-space through `C` facing `v`, and each band-end plane through
//!   `C` where `C` lies ahead of `v` along that edge. A chamfer's: the
//!   three supports and the patch plane.
//!
//! # The meter
//!
//! A link's reach skips its chain's supports, the faces its own window
//! ends in, and any face on a support's stored surface whose side bounds
//! it (that face lies on the side's zero set). A face the window ends in
//! is one the band runs into, judged by predicate 6 and the surgery:
//! a plane the reach is capped by, so it lies on the reach's boundary
//! (a curved one is the battery's refusal, and the reach fails loud on
//! it), or a support of a corner patch's links. What the band takes of
//! such a face is the cut-off's sliver, which the surgery's ring pass
//! meters too (arm (c)); a feature standing in it is refused here
//! first-hand, since its own faces lie in the band's material. At an
//! overrun arm (c) reads the far band's whole sliver on the near band's
//! support, a bound larger than the overrun, so a chamfer's also
//! refuses some features the overrun clears
//! (`work/bandmeter/a-turns-overrun-meters-the-whole-cut-off-sliver.md`).
//! A face at another link's end bounds nothing here and is metered like
//! any other; a circular link's window has no ends. A support of
//! another chain of the request is metered by what survives that
//! chain's band: a cell inside the strip the band replaces is clear.
//! The other band's surface is metered too, except a concave band's
//! inside a convex band's reach, and except between chains that meet at
//! a vertex.
//!
//! Faces are pruned by their certified boxes ([`topo::FaceBoxes`]).
//! Each survivor is cut into cells, in the reach's meridian sheet or in
//! space ([`metered`]); a cell is clear when a lower bound over it of
//! the face's own surface distance or of one of the reach's bounds is
//! positive. A cell neither clears is halved until it clears, its half
//! diagonal falls within the band's `escalate`, or the face's
//! [`CELL_BUDGET`] is spent. The face's margin is the least cell bound:
//! a cell of the surface may lie outside the face's trim, so a clear
//! face can be refused uncertified, never the reverse. The inside
//! reading is the same walk from the other side ([`inside_bound`]), over
//! the face's whole box: a face whose surface leaves the material
//! outside its trim is read uncertified, never inside.

use std::collections::{BTreeMap, VecDeque};

use geom::{Curve3, Surface};
use geom_core::{Band, Bounds, Decide, Margin, Point3, Real, Sign, Vec3};
use topo::{Body, EdgeKey, EntityId, FaceBoxes, FaceKey, LoopKey, SurfaceKey, VertexKey};

use super::arms::{chamfer_corner_patch, corner_ball, line_meet};
use super::battery::{
    Chain, Convexity, Link, carrier_of, classified, face_clearance_margin, outward,
};
use super::surgery::{degenerate, disagrees, loop_walk, unresolved, unwalked};
use super::{BlendDecision, BlendError, BlendKind, BlendSite, classify};

/// How many cells one face may be cut into against one reach before it
/// is refused uncertified: the meter's cost ceiling per pair, which
/// keeps the predicate near-linear in the body. Not a knob: a spent
/// budget folds the reach's bound alone over cells that did not clear,
/// which is below `escalate`, so no value of it can pass a face, only
/// decide how far a clear one is chased before it is refused.
const CELL_BUDGET: usize = 4096;

/// How many places along an edge, ends included, a three-way reading
/// reads for a witness: a point of a face's boundary inside the band's
/// material or out of it, or of an end-face edge inside the sliver or
/// out of it. Each witness is decided where it is read, so the count
/// decides only how readily a crossing is told from a bound.
pub(in crate::blend) const EDGE_SAMPLES: u32 = 9;

/// Whether a reading is certainly positive, by `escalate`: what makes a
/// point a witness of a three-way reading. A reading that is never
/// reported is not classified.
pub(in crate::blend) fn sure<T: Bounds>(x: T, band: Band) -> bool {
    x.lo() >= band.escalate()
}

/// A rectangle of a meridian sheet: `[ρ_lo, ρ_hi, z_lo, z_hi]`.
type Rect<T> = [T; 4];

/// A 1-Lipschitz function of the meridian-sheet coordinates `(ρ, z)`.
#[derive(Clone, Copy, Debug)]
enum SheetFn<T: Real> {
    /// `α·ρ + β·z + γ`, with `α² + β² = 1`.
    Line { alpha: T, beta: T, gamma: T },
    /// `σ·(|(ρ, z) − (p, q)| − r)`, `|σ| = 1`.
    Circle { p: T, q: T, r: T, sigma: T },
    /// `σ·(ρ·cos − |z − q|·sin)`, `|σ| = 1`: a cone about the axis with
    /// its apex at `z = q`, on both nappes.
    Cone { q: T, cos: T, sin: T, sigma: T },
}

impl<T: Real> SheetFn<T> {
    fn at(&self, rho: T, z: T) -> T {
        match *self {
            Self::Line { alpha, beta, gamma } => alpha * rho + beta * z + gamma,
            Self::Circle { p, q, r, sigma } => {
                sigma * (((rho - p).powi(2) + (z - q).powi(2)).sqrt() - r)
            }
            Self::Cone { q, cos, sin, sigma } => sigma * (rho * cos - (z - q).abs() * sin),
        }
    }

    /// The function's range over a sheet rectangle, exactly: an affine
    /// function is extreme at a corner, a distance from a point is
    /// nearest where the point clamps into the rectangle and farthest at
    /// a corner, and the cone's form is monotone in `ρ` and in `|z − q|`.
    fn range(&self, [r0, r1, z0, z1]: Rect<T>) -> (T, T) {
        let signed =
            |sigma: T, a: T, b: T| ((sigma * a).min(sigma * b), (sigma * a).max(sigma * b));
        match *self {
            Self::Line { .. } => {
                let v = [
                    self.at(r0, z0),
                    self.at(r0, z1),
                    self.at(r1, z0),
                    self.at(r1, z1),
                ];
                (
                    v[0].min(v[1]).min(v[2]).min(v[3]),
                    v[0].max(v[1]).max(v[2]).max(v[3]),
                )
            }
            Self::Circle { p, q, r, sigma } => {
                let (near, far) = rect_distance([r0, r1, z0, z1], p, q);
                signed(sigma, near - r, far - r)
            }
            Self::Cone { q, cos, sin, sigma } => {
                let (a, b) = (z0 - q, z1 - q);
                let z_near = a.max(-b).max(T::zero());
                let z_far = a.abs().max(b.abs());
                signed(sigma, r0 * cos - z_far * sin, r1 * cos - z_near * sin)
            }
        }
    }

    /// `w` times this function, `|w| = 1`.
    fn scaled(self, w: T) -> Self {
        match self {
            Self::Line { alpha, beta, gamma } => Self::Line {
                alpha: alpha * w,
                beta: beta * w,
                gamma: gamma * w,
            },
            Self::Circle { p, q, r, sigma } => Self::Circle {
                p,
                q,
                r,
                sigma: sigma * w,
            },
            Self::Cone { q, cos, sin, sigma } => Self::Cone {
                q,
                cos,
                sin,
                sigma: sigma * w,
            },
        }
    }
}

/// How far `c` lies outside `[lo, hi]`; zero inside.
fn gap<T: Real>(lo: T, hi: T, c: T) -> T {
    (lo - c).max(c - hi).max(T::zero())
}

/// How far the direction `b` is from the unit axis `a` either way: the
/// chord `|b̂ − ±a|`.
fn tilt<T: Real>(b: Vec3<T>, a: Vec3<T>) -> T {
    let b = b.normalize();
    (b - a).norm().min((b + a).norm())
}

/// `±1` where the sign of `x` is decided past the bracket, `0` where it
/// is not. A bound
/// scaled by an undecided sign is `≡ 0`, which clears no cell, excuses
/// none and places no point inside a core: vacuous, never wrong.
fn sign<T: Decide + Bounds>(x: T) -> T {
    if x.lo() > 0.0 {
        T::one()
    } else if x.hi() < 0.0 {
        -T::one()
    } else {
        T::zero()
    }
}

/// The nearest and farthest distance from the sheet point `(p, q)` to a
/// sheet rectangle.
fn rect_distance<T: Real>([r0, r1, z0, z1]: Rect<T>, p: T, q: T) -> (T, T) {
    let span = |lo: T, hi: T, c: T| (c - lo).abs().max((hi - c).abs());
    (
        (gap(r0, r1, p).powi(2) + gap(z0, z1, q).powi(2)).sqrt(),
        (span(r0, r1, p).powi(2) + span(z0, z1, q).powi(2)).sqrt(),
    )
}

/// What a bound of a reach stands for: a support's side (the support's
/// own surface is its zero set), the band itself (the blend surface's
/// side: the ball, or the chamfer strip, or a corner patch), the disk
/// through the edge point, or anything else (a sector, a window, an end
/// face).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Role {
    Side(FaceKey),
    Band,
    /// The disk through the edge point, about the ball's centre.
    Disk,
    Other,
}

/// A bound with its role.
type Tagged<T> = (Role, Bound<T>);

/// One 1-Lipschitz bound `g`; the reach is `{g ≤ 0}` over all of them.
#[derive(Clone, Debug)]
enum Bound<T: Real> {
    /// `g = n·x − d`, `n` unit.
    Affine { n: Vec3<T>, d: T },
    /// The distance `ρ(x)` from the line through `o` along the unit `a`:
    /// `g = ρ − r` when `inside`, `r − ρ` otherwise.
    Tube {
        o: Point3<T>,
        a: Vec3<T>,
        r: T,
        inside: bool,
    },
    /// `g = r − |x − o|`: outside the ball of radius `r` about `o`.
    OutsideBall { o: Point3<T>, r: T },
    /// `g = |x − o| − r`: inside the ball of radius `r` about `o`.
    InsideBall { o: Point3<T>, r: T },
    /// `g = f(ρ, z) − slack` in the meridian sheet about the unit axis `a`
    /// through `o`.
    Sheet {
        o: Point3<T>,
        a: Vec3<T>,
        f: SheetFn<T>,
        slack: T,
    },
    /// `w` times a surface's own signed distance ([`surface_distance`]),
    /// `|w| = 1`.
    Side { s: Surface<T>, w: T },
}

/// The axial coordinate and the distance from the axis of `x`, about
/// the unit axis `a` through `o`, as `(ρ, z)`.
fn sheet_of<T: Real>(o: Point3<T>, a: Vec3<T>, x: Point3<T>) -> (T, T) {
    let v = x - o;
    let z = v.dot(a);
    ((v - a * z).norm(), z)
}

/// The largest of a box's eight corner values of `f`.
fn corner_max<T: Real>(lo: Point3<T>, hi: Point3<T>, f: impl Fn(Point3<T>) -> T) -> T {
    let pick = |i: usize| {
        Point3::new(
            if i & 1 == 0 { lo.x } else { hi.x },
            if i & 2 == 0 { lo.y } else { hi.y },
            if i & 4 == 0 { lo.z } else { hi.z },
        )
    };
    (1..8).fold(f(pick(0)), |m, i| m.max(f(pick(i))))
}

/// The distance from `o` to the nearest point of `cell`: `o` clamped
/// into the box.
fn nearest<T: Real>(o: Point3<T>, cell: &Cell<T>) -> T {
    (gap(cell.lo.x, cell.hi.x, o.x).powi(2)
        + gap(cell.lo.y, cell.hi.y, o.y).powi(2)
        + gap(cell.lo.z, cell.hi.z, o.z).powi(2))
    .sqrt()
}

/// A cell: its two corners, its centre, its half extents, and the length
/// of its half diagonal (the Lipschitz lever of a bound over it).
struct Cell<T: Real> {
    lo: Point3<T>,
    hi: Point3<T>,
    c: Point3<T>,
    half: Vec3<T>,
    lever: T,
}

impl<T: Real> Cell<T> {
    /// `lo` and `hi` are box arithmetic's output, so each corner
    /// coordinate enters as one computed value ([`Real::from_computed`]).
    fn new(lo: [f64; 3], hi: [f64; 3]) -> Self {
        let p = |v: [f64; 3]| {
            let [x, y, z] = v.map(T::from_computed);
            Point3::new(x, y, z)
        };
        let (lo, hi) = (p(lo), p(hi));
        let half = (hi - lo) / T::from_f64(2.0);
        Self {
            lo,
            hi,
            c: lo + half,
            half,
            lever: half.norm(),
        }
    }

    /// How far a unit-normal affine function varies from the centre.
    fn spread(&self, n: Vec3<T>) -> T {
        n.x.abs() * self.half.x + n.y.abs() * self.half.y + n.z.abs() * self.half.z
    }
}

/// **A cell's rectangle in the meridian sheet** about the unit axis `a`
/// through `o`, enclosing every point of the cell. The axial range is
/// exact (it is affine); the top of the distance from the axis is exact
/// (a convex function peaks at a corner); its bottom is the centre's,
/// less the cell's own reach across the axis.
fn sheet_box<T: Real>(o: Point3<T>, a: Vec3<T>, cell: &Cell<T>) -> Rect<T> {
    let (rc, zc) = sheet_of(o, a, cell.c);
    let dz = cell.spread(a);
    let across = corner_max(cell.lo, cell.hi, |x| {
        let v = x - cell.c;
        (v - a * v.dot(a)).norm()
    });
    let top = corner_max(cell.lo, cell.hi, |x| sheet_of(o, a, x).0);
    [(rc - across).max(T::zero()), top, zc - dz, zc + dz]
}

impl<T: Real> Bound<T> {
    /// Whether the region `g ≤ 0` is convex.
    fn convex(&self) -> bool {
        matches!(
            self,
            Self::Affine { .. } | Self::Tube { inside: true, .. } | Self::InsideBall { .. }
        )
    }

    /// `g(x)`.
    fn at(&self, x: Point3<T>) -> T {
        match self {
            Self::Affine { n, d } => (x - Point3::origin()).dot(*n) - *d,
            Self::Tube { o, a, r, inside } => {
                let (rho, _) = sheet_of(*o, *a, x);
                if *inside { rho - *r } else { *r - rho }
            }
            Self::OutsideBall { o, r } => *r - (x - *o).norm(),
            Self::InsideBall { o, r } => (x - *o).norm() - *r,
            Self::Sheet { o, a, f, slack } => {
                let (rho, z) = sheet_of(*o, *a, x);
                f.at(rho, z) - *slack
            }
            Self::Side { s, w } => match surface_distance(s, x) {
                Some(d) => *w * d,
                None => T::zero(),
            },
        }
    }

    /// `g(x)` where the bound is read as a statement about the band's
    /// own material: a sheet bound's slack, which ENCLOSES the band,
    /// counts the other way.
    fn at_strict(&self, x: Point3<T>) -> T {
        match self {
            Self::Sheet { slack, .. } => self.at(x) + *slack + *slack,
            _ => self.at(x),
        }
    }

    /// A lower bound of `g` over `cell`: exact for an affine bound, for
    /// the far side of a tube and for the outside of a ball; read off
    /// the cell's sheet rectangle ([`sheet_box`]) for a sheet bound and
    /// for the near side of a tube; from the surface's own range for a
    /// side.
    fn low(&self, cell: &Cell<T>) -> T {
        match self {
            Self::Affine { n, .. } => self.at(cell.c) - cell.spread(*n),
            Self::Tube { o, a, r, inside } => {
                let b = sheet_box(*o, *a, cell);
                if *inside { b[0] - *r } else { *r - b[1] }
            }
            Self::OutsideBall { o, r } => *r - corner_max(cell.lo, cell.hi, |x| (x - *o).norm()),
            Self::InsideBall { o, r } => nearest(*o, cell) - *r,
            Self::Sheet { o, a, f, slack } => f.range(sheet_box(*o, *a, cell)).0 - *slack,
            Self::Side { s, w } => match surface_range(s, cell) {
                Some((lo, hi)) => (*w * lo).min(*w * hi),
                None => self.at(cell.c) - cell.lever,
            },
        }
    }

    /// An upper bound of `g` over `cell`, by the same reads as [`Self::low`]
    /// from the other side: the whole cell lies in `{g ≤ 0}` when it is
    /// not positive.
    fn high(&self, cell: &Cell<T>) -> T {
        match self {
            Self::Affine { n, .. } => self.at(cell.c) + cell.spread(*n),
            Self::Tube { o, a, r, inside } => {
                let b = sheet_box(*o, *a, cell);
                if *inside { b[1] - *r } else { *r - b[0] }
            }
            Self::OutsideBall { o, r } => *r - nearest(*o, cell),
            Self::InsideBall { o, r } => corner_max(cell.lo, cell.hi, |x| (x - *o).norm()) - *r,
            Self::Sheet { o, a, f, slack } => f.range(sheet_box(*o, *a, cell)).1 - *slack,
            Self::Side { s, w } => match surface_range(s, cell) {
                Some((lo, hi)) => (*w * lo).max(*w * hi),
                None => self.at(cell.c) + cell.lever,
            },
        }
    }

    /// [`Self::high`] read as a statement about the band's own material,
    /// as [`Self::at_strict`] reads `g`: the whole cell lies in the
    /// material when it is negative.
    fn high_strict(&self, cell: &Cell<T>) -> T {
        match self {
            Self::Sheet { slack, .. } => self.high(cell) + *slack + *slack,
            _ => self.high(cell),
        }
    }

    /// [`Self::high`] over a rectangle of a sheet, for a sheet bound read
    /// in its own sheet ([`Self::frame`]); `None` for any other.
    fn high_in_sheet(&self, rect: Rect<T>) -> Option<T> {
        match self {
            Self::Sheet { f, slack, .. } => Some(f.range(rect).1 - *slack),
            _ => None,
        }
    }

    /// The sheet a sheet bound is a function of; `None` for any other.
    fn frame(&self) -> Option<(Point3<T>, Vec3<T>)> {
        match self {
            Self::Sheet { o, a, .. } => Some((*o, *a)),
            _ => None,
        }
    }

    /// A lower bound of `g` over a rectangle of the sheet about `(o, a)`,
    /// for a sheet bound about that same sheet; `None` for any other.
    fn low_in_sheet(&self, rect: Rect<T>) -> Option<T> {
        match self {
            Self::Sheet { f, slack, .. } => Some(f.range(rect).0 - *slack),
            _ => None,
        }
    }
}

/// **A surface's own signed distance**, 1-Lipschitz and zero on the
/// surface: exact for a plane, a cylinder, a sphere and a ring torus;
/// for a cone, `ρ·cos α − |z|·sin α`, zero on both nappes. `None` for a
/// surface with no closed form here (NURBS, an approximating surface).
///
/// Every torus read here is a ring torus (`R > r`): a stored face's by
/// tier 3, which refuses a horn or spindle at rest (`DegenerateTorus`),
/// and a band's by predicate 3 (the spine's radius exceeds the ball's).
/// On a spindle `|(ρ − R, z)| − r` would be no distance off the lemon.
fn surface_distance<T: Real>(s: &Surface<T>, x: Point3<T>) -> Option<T> {
    match s {
        Surface::Plane { origin, normal, .. } => Some((x - *origin).dot(normal.normalize())),
        Surface::Cylinder {
            origin,
            axis,
            radius,
            ..
        } => Some(sheet_of(*origin, axis.normalize(), x).0 - *radius),
        Surface::Sphere { center, radius, .. } => Some((x - *center).norm() - *radius),
        Surface::Cone {
            apex,
            axis,
            half_angle,
            ..
        } => {
            let (rho, z) = sheet_of(*apex, axis.normalize(), x);
            Some(rho * half_angle.cos() - z.abs() * half_angle.sin())
        }
        Surface::Torus {
            center,
            axis,
            major_radius,
            minor_radius,
            ..
        } => {
            let (rho, z) = sheet_of(*center, axis.normalize(), x);
            Some(((rho - *major_radius).powi(2) + z.powi(2)).sqrt() - *minor_radius)
        }
        Surface::Nurbs(_) | Surface::Approx(_) => None,
    }
}

/// The range over `cell` of a surface's own signed distance
/// ([`surface_distance`]), read about the surface's own axis or centre;
/// `None` with no closed form. Not `geom_brep`'s implicit enclosure:
/// that one encloses a residual rather than a 1-Lipschitz distance,
/// refuses cones and tori, and runs only at a certified scalar, where
/// this meter replays at every `T`.
fn surface_range<T: Real>(s: &Surface<T>, cell: &Cell<T>) -> Option<(T, T)> {
    Some(match s {
        Surface::Plane { normal, .. } => {
            let f = surface_distance(s, cell.c)?;
            let w = cell.spread(normal.normalize());
            (f - w, f + w)
        }
        Surface::Sphere { center, radius, .. } => (
            nearest(*center, cell) - *radius,
            corner_max(cell.lo, cell.hi, |x| (x - *center).norm()) - *radius,
        ),
        _ => {
            let (f, o, a) = sheet_fn_own(s)?;
            f.range(sheet_box(o, a, cell))
        }
    })
}

/// A cylinder's, a cone's or a torus's own distance as a sheet function
/// about its own axis, with that axis.
fn sheet_fn_own<T: Real>(s: &Surface<T>) -> Option<(SheetFn<T>, Point3<T>, Vec3<T>)> {
    let one = T::one();
    match s {
        Surface::Cylinder {
            origin,
            axis,
            radius,
            ..
        } => Some((
            SheetFn::Line {
                alpha: one,
                beta: T::zero(),
                gamma: -*radius,
            },
            *origin,
            axis.normalize(),
        )),
        Surface::Cone {
            apex,
            axis,
            half_angle,
            ..
        } => Some((
            SheetFn::Cone {
                q: T::zero(),
                cos: half_angle.cos(),
                sin: half_angle.sin(),
                sigma: one,
            },
            *apex,
            axis.normalize(),
        )),
        Surface::Torus {
            center,
            axis,
            major_radius,
            minor_radius,
            ..
        } => Some((
            SheetFn::Circle {
                p: *major_radius,
                q: T::zero(),
                r: *minor_radius,
                sigma: one,
            },
            *center,
            axis.normalize(),
        )),
        _ => None,
    }
}

/// **A surface's distance read in the sheet about the unit axis `a`
/// through `o`**, with the slack that makes it an enclosure: a sheet
/// function `f` and `δ` with `|d(x) − f(ρ(x), z(x))| ≤ δ` for every `x`
/// in the box `(lo, hi)`, `d` the surface's own signed distance. The
/// slack is the surface's departure from a surface of revolution about
/// `(o, a)`: a reference point's distance from the axis, and its own
/// axis's tilt (the chord `|â − ±a|`) levered from there over the box.
/// A cone, a sphere and a torus are read at their anchor; a plane and a
/// cylinder at the foot of the box's centre on the plane or the axis,
/// since the origin they store may be anywhere on it. `None` for a
/// surface with no closed form here.
fn sheet_fn<T: Decide + Bounds>(
    s: &Surface<T>,
    o: Point3<T>,
    a: Vec3<T>,
    (lo, hi): (Point3<T>, Point3<T>),
) -> Option<(SheetFn<T>, T)> {
    let tilt = |b: Vec3<T>| tilt(b, a);
    let one = T::one();
    let off_axis = |p: Point3<T>| sheet_of(o, a, p);
    let lever = |p: Point3<T>| corner_max(lo, hi, |x| (x - p).norm());
    let centre = lo + (hi - lo) / T::from_f64(2.0);
    Some(match s {
        Surface::Plane { origin, normal, .. } => {
            let n = normal.normalize();
            let foot = centre - n * (centre - *origin).dot(n);
            // Either sign encloses, the slack being the chord to it, so a
            // plane parallel to the axis takes one by decision.
            let sg = if n.dot(a).hi() < 0.0 { -one } else { one };
            let (_, z) = off_axis(foot);
            (
                SheetFn::Line {
                    alpha: T::zero(),
                    beta: sg,
                    gamma: -sg * z,
                },
                (n - a * sg).norm() * lever(foot),
            )
        }
        Surface::Cylinder {
            origin,
            axis,
            radius,
            ..
        } => {
            let b = axis.normalize();
            let foot = *origin + b * (centre - *origin).dot(b);
            (
                SheetFn::Line {
                    alpha: one,
                    beta: T::zero(),
                    gamma: -*radius,
                },
                off_axis(foot).0 + tilt(*axis) * lever(foot),
            )
        }
        Surface::Sphere { center, radius, .. } => {
            let (rho, z) = off_axis(*center);
            (
                SheetFn::Circle {
                    p: T::zero(),
                    q: z,
                    r: *radius,
                    sigma: one,
                },
                rho,
            )
        }
        Surface::Cone {
            apex,
            axis,
            half_angle,
            ..
        } => {
            let (rho, z) = off_axis(*apex);
            (
                SheetFn::Cone {
                    q: z,
                    cos: half_angle.cos(),
                    sin: half_angle.sin(),
                    sigma: one,
                },
                rho + T::from_f64(1.5) * tilt(*axis) * lever(*apex),
            )
        }
        Surface::Torus {
            center,
            axis,
            major_radius,
            minor_radius,
            ..
        } => {
            let (rho, z) = off_axis(*center);
            (
                SheetFn::Circle {
                    p: *major_radius,
                    q: z,
                    r: *minor_radius,
                    sigma: one,
                },
                rho + tilt(*axis) * *major_radius,
            )
        }
        Surface::Nurbs(_) | Surface::Approx(_) => return None,
    })
}

/// A lower bound over `cell` of the distance from a face's surface: no
/// point of the surface lies in the cell when it is positive. `None`
/// with no closed form.
fn off_surface<T: Real>(s: &Surface<T>, cell: &Cell<T>) -> Option<T> {
    let (lo, hi) = surface_range(s, cell)?;
    Some(lo.max(-hi))
}

/// One reach: its bounds, the bounds a point must satisfy to lie in the
/// band's own material (`core`, empty when the bounds are not the
/// band's exactly), its box, the chain it belongs to, the sheet its
/// bounds are all functions of (a circular reach's), and the stored
/// surfaces of the supports whose sides bound it — a face on one of them
/// lies on that side's zero set, which is the reach's boundary and never
/// its inside.
struct Reach<T: Real> {
    bounds: Vec<Tagged<T>>,
    core: Vec<Bound<T>>,
    lo: Point3<T>,
    hi: Point3<T>,
    chain: usize,
    sheet: Option<(Point3<T>, Vec3<T>)>,
    sides: Vec<SurfaceKey>,
    /// The blend surface the band mints over this reach (the cylinder,
    /// the torus, the strip, the corner ball, the patch plane).
    surface: Surface<T>,
    /// Per support, the region whose points of that support the band
    /// replaces: for a straight link, within the chord from the edge to
    /// the support's trimline of the edge itself, over the edge's own
    /// extent, and on the trimline's side of it where the face comes back
    /// across — that is the support's strip where [`screened`] reads
    /// no other vertex of the face there, a reading of vertices and not
    /// edges (`work/bandreach/the-strip-screen-reads-a-supports-vertices-not-its-edges.md`); for a
    /// circular link, the disk of that chord about the edge in the
    /// meridian sheet; for a corner patch, its bounds but the supports'
    /// sides.
    replaces: Vec<(FaceKey, Vec<Bound<T>>)>,
    /// The bounds that confine `surface` to the patch the band mints,
    /// where they are not simply the reach's own but the band's
    /// (a chamfer corner's triangle of feet).
    confine: Option<Vec<Bound<T>>>,
    /// Every chain this reach belongs to: its link's, or every chain at
    /// a corner patch's vertex.
    chains: Vec<usize>,
    /// The faces this reach does not meter by key, sorted: a link's,
    /// its chain's supports and the faces its own window ends in (a
    /// plane it is capped by, or a corner patch's support, which
    /// [`straight_reach`] records as it builds the ends; a circular
    /// link's window has none); a corner patch's, the faces at its
    /// vertex.
    skip: Vec<FaceKey>,
    /// The cross-section's reach from its spine point: the scale a
    /// sheet bound's slack is weighed against ([`metered`]).
    width: T,
    /// A straight link's edge and unit spine direction, which its
    /// prism reading takes ([`prism_inside`]).
    spine: Option<(EdgeKey, Vec3<T>)>,
    /// Per plane support of a straight link with an exact core, the
    /// core less that support's side, in which a cup on the support's
    /// strip is read ([`prism_inside`]).
    strips: Vec<StripCore<T>>,
}

/// The core of a straight link less one plane support's side.
struct StripCore<T: Real> {
    support: FaceKey,
    /// The support's unit normal `ν`.
    normal: Vec3<T>,
    bounds: Vec<Bound<T>>,
    /// Whether every one of `bounds` is convex.
    convex: bool,
    /// Where the one bound of `bounds` that is not convex is the outside
    /// of a tube along the spine (a fillet's ball), the closed half-space
    /// through its axis, normal to `ν`, on the support's foot's side:
    /// within it each line along `ν` meets each set of the core's points
    /// deeper than any `δ` in a segment.
    along: Option<Bound<T>>,
}

/// The outward normal of `face` at `p`, or the body's refusal.
fn normal_at<T: Decide>(
    body: &Body<T>,
    face: FaceKey,
    p: Point3<T>,
) -> Result<Vec3<T>, BlendError> {
    outward(body, face, p)
        .ok_or_else(|| unresolved(EntityId::Face(face), "a face the band reach reads"))
}

fn surface_of<T: Decide>(body: &Body<T>, face: FaceKey) -> Result<Surface<T>, BlendError> {
    body.get_face(face)
        .and_then(|f| body.get_surface(f.surface))
        .cloned()
        .ok_or_else(|| unresolved(EntityId::Face(face), "a face's stored surface"))
}

fn surface_key<T: Decide>(body: &Body<T>, face: FaceKey) -> Result<SurfaceKey, BlendError> {
    body.get_face(face)
        .map(|f| f.surface)
        .ok_or_else(|| unresolved(EntityId::Face(face), "a face's stored surface"))
}

fn point_of<T: Decide>(body: &Body<T>, v: VertexKey) -> Result<Point3<T>, BlendError> {
    super::surgery::point_of(body, v)
        .ok_or_else(|| unresolved(EntityId::Vertex(v), "a chain vertex's point"))
}

fn faces_at<T: Decide>(body: &Body<T>, v: VertexKey) -> Result<Vec<FaceKey>, BlendError> {
    body.faces_of_vertex(v)
        .ok_or_else(|| unwalked(EntityId::Vertex(v), "a chain vertex's face fan"))
}

/// The point of `curve` nearest `p`, for a line or a circle (a band's
/// trimline or spine); `None` for any other carrier.
fn foot_on<T: Real>(curve: &Curve3<T>, p: Point3<T>) -> Option<Point3<T>> {
    match *curve {
        Curve3::Line { origin, dir } => Some(origin + dir * ((p - origin).dot(dir) / dir.dot(dir))),
        Curve3::Circle {
            center,
            axis,
            radius,
            ..
        } => {
            let a = axis.normalize();
            let v = p - center;
            Some(center + (v - a * v.dot(a)).normalize() * radius)
        }
        _ => None,
    }
}

/// The mean of some points.
fn centroid<T: Real>(pts: &[Point3<T>]) -> Point3<T> {
    let sum = pts
        .iter()
        .fold(Vec3::new(T::zero(), T::zero(), T::zero()), |s, p| {
            s + (*p - Point3::origin())
        });
    Point3::origin() + sum / T::from_f64(pts.len() as f64)
}

/// The side bound of one support in space: its signed distance, signed
/// so that the reference point `q` (inside the band's cross-section) is
/// on the inside. A plane's is affine, so it is exact over a cell.
fn side<T: Decide + Bounds>(s: &Surface<T>, q: Point3<T>) -> Option<Bound<T>> {
    let fq = surface_distance(s, q)?;
    let w = -sign(fq);
    Some(match s {
        Surface::Plane { origin, normal, .. } => {
            let n = normal.normalize() * w;
            Bound::Affine {
                n,
                d: (*origin - Point3::origin()).dot(n),
            }
        }
        _ => Bound::Side { s: s.clone(), w },
    })
}

/// The half-space through `o` normal to `n`, positive on the side away
/// from `inside`.
fn half_space<T: Decide + Bounds>(o: Point3<T>, n: Vec3<T>, inside: Point3<T>) -> Bound<T> {
    let n = n.normalize();
    let n = n * -sign((inside - o).dot(n));
    Bound::Affine {
        n,
        d: (o - Point3::origin()).dot(n),
    }
}

/// **The premise [`Reach::replaces`] stands on, metered**: the strip
/// of `face` a link's band replaces holds no vertex of `face` but the
/// request's own (`ends`). `region` over-covers the strip on both sides
/// of the edge; `across`, where the link has one, is the side of the
/// edge's line the strip lies on, set back by a sliver. A vertex
/// definitely inside both is a boundary feature of the support within
/// the band's setback, and refuses as predicate 2 does
/// ([`face_clearance_margin`]): its margin the vertex's depth inside
/// them, its gap `gap(p)`, its distance from the edge's line. Most such
/// features refuse before this: the battery's support screen reads one
/// within the setbacks of a sample, on either side of the edge, and the
/// surgery's closed-form strip meter one of a plane–plane link's planar
/// support between the screen's samples. A ruled link's planar support
/// has no strip meter, so a vertex there between the samples refuses
/// here. A feature across the edge's line, which the band never
/// reaches, is clear. A ring the request takes (`taken`: a cup's mouth
/// read inside a strip, [`TakenRing`]) leaves the face before its strip
/// is carved, and is not read.
///
/// Returns whether `region` alone is vacant: no vertex of `face` but
/// the request's possibly inside it. That is a reading of the face's
/// vertices, not its edges: a curved edge can cross `region` with both
/// its ends outside
/// (`work/bandreach/the-strip-screen-reads-a-supports-vertices-not-its-edges.md`).
#[must_use = "the vacancy decides whether a straight strip keeps both sides"]
fn screened<T: Decide + Bounds>(
    body: &Body<T>,
    (ends, taken): (&[VertexKey], &[LoopKey]),
    face: FaceKey,
    region: &[Bound<T>],
    across: Option<&Bound<T>>,
    gap: impl Fn(Point3<T>) -> T,
    band: Band,
) -> Result<bool, BlendError> {
    let f = body
        .get_face(face)
        .ok_or_else(|| unresolved(EntityId::Face(face), "a support the band replaces"))?;
    let mut vacant = true;
    let kept = f.rings.iter().copied().filter(|r| !taken.contains(r));
    for lp in core::iter::once(f.outer).chain(kept) {
        let vertices: Vec<VertexKey> = match body.get_loop(lp).map(|l| l.boundary) {
            Some(topo::LoopBoundary::Cycle { first }) => body
                .loop_cycle(first)
                .ok_or_else(|| unwalked(EntityId::Loop(lp), "a support's cycle"))?
                .into_iter()
                .filter_map(|he| body.get_half_edge(he).map(|h| h.start))
                .collect(),
            Some(topo::LoopBoundary::Empty { vertex }) => vec![vertex],
            None => return Err(unresolved(EntityId::Loop(lp), "a support's loop")),
        };
        for v in vertices {
            if ends.binary_search(&v).is_ok() {
                continue;
            }
            let p = point_of(body, v)?;
            let Some(depth) = region.iter().map(|b| b.at(p)).reduce(|m, g| m.max(g)) else {
                continue;
            };
            vacant &= depth.lo() >= 0.0;
            let depth = across.map_or(depth, |b| depth.max(b.at(p)));
            if depth.hi() < -band.escalate() {
                face_clearance_margin(face, depth, gap(p), false, band)?;
            }
        }
    }
    Ok(vacant)
}

/// The geometry of one link at its middle station: the edge point, the
/// two feet, and the spine point with the ball's radius (`None` for a
/// chamfer).
struct Station<T: Real> {
    p: Point3<T>,
    fa: Point3<T>,
    fb: Point3<T>,
    c: Option<(Point3<T>, T)>,
}

impl<T: Real> Station<T> {
    /// A point inside the band's cross-section: the mean of the edge
    /// point and the two feet.
    fn inside(&self) -> Point3<T> {
        centroid(&[self.p, self.fa, self.fb])
    }
}

fn station<T: Decide>(body: &Body<T>, link: &Link<T>) -> Result<Station<T>, BlendError> {
    let at = EntityId::Edge(link.edge);
    let (carrier, t0, t1) = carrier_of(body, link.edge)?;
    let p = carrier.eval(geom::mid_param(t0, t1));
    let off_trim = || disagrees(at, "a link's trimline is neither a line nor a circle");
    let fa = foot_on(&link.blend.trim_a.0, p).ok_or_else(off_trim)?;
    let fb = foot_on(&link.blend.trim_b.0, p).ok_or_else(off_trim)?;
    let c = match link.blend.surface {
        Surface::Cylinder {
            origin,
            axis,
            radius,
            ..
        } => foot_on(&Curve3::Line { origin, dir: axis }, p).map(|c| (c, radius)),
        Surface::Torus {
            center,
            axis,
            major_radius,
            minor_radius,
            u_ref,
        } => foot_on(
            &Curve3::Circle {
                center,
                axis,
                radius: major_radius,
                u_ref,
            },
            p,
        )
        .map(|c| (c, minor_radius)),
        _ => None,
    };
    Ok(Station { p, fa, fb, c })
}

/// **The reach of one straight link** (module docs): the cross-section
/// prism over the edge's window.
fn straight_reach<T: Decide + Bounds>(
    body: &Body<T>,
    link: &Link<T>,
    st: &Station<T>,
    corners: &[(VertexKey, Vec<Point3<T>>)],
    (request, taken): (&[VertexKey], &[LoopKey]),
    chain: usize,
    band: Band,
) -> Result<Reach<T>, BlendError> {
    let start = point_of(body, link.start)?;
    let end = point_of(body, link.end)?;
    let tau = (end - start).normalize();
    let q = st.inside();
    let mut cross = Vec::new();
    let mut sides = Vec::new();
    let mut exact = true;
    for f in [link.face_a, link.face_b] {
        let s = surface_of(body, f)?;
        exact &= matches!(s, Surface::Plane { .. });
        if let Some(b) = side(&s, q) {
            cross.push((Role::Side(f), b));
            sides.push(surface_key(body, f)?);
        }
    }
    let reach = match st.c {
        Some((c, r)) => {
            // The sector: the plane through the spine and each foot, on
            // P's side.
            for f in [st.fa, st.fb] {
                let u = f - c;
                let w = (st.p - c) - u * ((st.p - c).dot(u) / u.dot(u));
                cross.push((Role::Other, half_space(c, w, st.p)));
            }
            let reach = (st.p - c).norm();
            cross.push((
                Role::Band,
                Bound::Tube {
                    o: c,
                    a: tau,
                    r,
                    inside: false,
                },
            ));
            cross.push((
                Role::Disk,
                Bound::Tube {
                    o: c,
                    a: tau,
                    r: reach,
                    inside: true,
                },
            ));
            reach
        }
        None => {
            // The chamfer strip's plane, on P's side.
            cross.push((
                Role::Band,
                half_space(st.fa, (st.fb - st.fa).cross(tau), st.p),
            ));
            (st.fa - st.p).norm().max((st.fb - st.p).norm())
        }
    };
    // The window along the spine. The plane of the face each end runs
    // into bounds the band (module docs, a mitre's included), and the
    // window widens by the run of the cross-section along that plane;
    // at a corner patch it ends at the patch: the plane through the
    // ball's centre normal to the edge (the chamfer's feet).
    let mut ends = Vec::new();
    let mut core: Vec<Bound<T>> = cross.iter().map(|(_, b)| b.clone()).collect();
    let mut caps = Vec::new();
    let mut skip = Vec::new();
    for v in [link.start, link.end] {
        let pv = point_of(body, v)?;
        let patch = corners.iter().find(|(cv, _)| *cv == v);
        let mut pad = T::zero();
        for f in faces_at(body, v)? {
            if f == link.face_a || f == link.face_b {
                continue;
            }
            let n = normal_at(body, f, pv)?;
            let cap = half_space(pv, n, q);
            core.push(cap.clone());
            if patch.is_none() {
                // A face tangent to the spine (a G1 junction's cylinder)
                // ends no window: its plane runs along the band.
                let along = n.dot(tau).abs();
                if along.lo() <= band.escalate() {
                    return Err(BlendError::UnsupportedRunOut {
                        at: EntityId::Vertex(v),
                        detail: "an end face tangent to the spine, whose plane bounds no window",
                    });
                }
                // The face lies on its cap, the reach's boundary, only
                // when it is the plane the cap is: that is the licence
                // to skip it.
                if !matches!(surface_of(body, f)?, Surface::Plane { .. }) {
                    return Err(BlendError::SurgeryInvariant {
                        at: EntityId::Face(f),
                        detail: "a straight band's window ends in a curved face, which the \
                                 battery refuses (END_FACE_CURVED) before the reach is read",
                    });
                }
                caps.push((Role::Other, cap));
                pad = pad.max(reach * (n.cross(tau).norm() / along));
            }
            skip.push(f);
        }
        let sv = (pv - start).dot(tau);
        ends.push(match patch {
            // The band ends at the patch, between its anchors' stations,
            // and its material inside the patch is the patch's: the core
            // stops at the vertex.
            Some((_, anchors)) => {
                let at = |x: &Point3<T>| (*x - start).dot(tau);
                let first = anchors.first().map_or(sv, at);
                let (lo, hi) = anchors
                    .iter()
                    .map(at)
                    .fold((first, first), |(l, h), x| (l.min(x), h.max(x)));
                (sv, sv, lo, hi)
            }
            // Ended by the plane it runs into: the cross-section within
            // that plane is the band's material across the whole pad.
            None => (sv - pad, sv + pad, sv - pad, sv + pad),
        });
    }
    let s0 = (start - Point3::origin()).dot(tau);
    let window = |lo: T, hi: T| {
        [
            (
                Role::Other,
                Bound::Affine {
                    n: -tau,
                    d: -(s0 + lo),
                },
            ),
            (Role::Other, Bound::Affine { n: tau, d: s0 + hi }),
        ]
    };
    let w_lo = ends[0].2.min(ends[1].2);
    let w_hi = ends[0].3.max(ends[1].3);
    let core_window = window(ends[0].0.min(ends[1].0), ends[0].1.max(ends[1].1));
    core.extend(core_window.iter().map(|(_, b)| b.clone()));
    // The core opens with the cross-section's bounds, in `cross`'s order.
    let mut strips = Vec::new();
    let pairs = [(link.face_a, st.fa), (link.face_b, st.fb)];
    for (f, foot) in pairs.into_iter().filter(|_| exact) {
        let Some(i) = cross.iter().position(|(role, _)| *role == Role::Side(f)) else {
            continue;
        };
        let Surface::Plane { normal, .. } = surface_of(body, f)? else {
            continue;
        };
        let normal = normal.normalize();
        let mut bounds = core.clone();
        bounds.remove(i);
        let mut bent = bounds.iter().filter(|b| !b.convex());
        let along = match (bent.next(), bent.next()) {
            (
                Some(Bound::Tube {
                    o, inside: false, ..
                }),
                None,
            ) => Some(half_space(*o, normal, foot)),
            _ => None,
        };
        strips.push(StripCore {
            support: f,
            normal,
            convex: bounds.iter().all(Bound::convex),
            bounds,
            along,
        });
    }
    let mut bounds = cross;
    bounds.extend(caps);
    bounds.extend(window(w_lo, w_hi));
    // Per support, the strip over the edge's own extent: past an end
    // the face may go on (beyond a reflex end) where no band replaces
    // it. The slab of twice the boxes' pad past each end takes no more
    // of the face than a sliver below the meter's reach, and holds a
    // cell straddling an end only where the edge runs along an axis.
    // Across the edge's line the face may come back (a peninsula of
    // its own cycle) where no band replaces it either: where `screened`
    // finds a vertex there, the strip is the trimline's side of the line
    // alone, set back by the same slab; where it finds none, the strip
    // keeps both sides, which hold a cell straddling the line at any
    // orientation. That reads vertices alone (`screened`).
    let mut replaces = Vec::new();
    for (f, foot) in [(link.face_a, st.fa), (link.face_b, st.fb)] {
        let chord = (foot - st.p).norm();
        let slab = T::from_f64(2.0 * FaceBoxes::pad(band));
        let (lo, hi) = (ends[0].0.min(ends[1].0), ends[0].0.max(ends[1].0));
        let tube = Bound::Tube {
            o: st.p,
            a: tau,
            r: chord,
            inside: true,
        };
        let mut b: Vec<Bound<T>> = window(lo - slab, hi + slab)
            .into_iter()
            .map(|(_, b)| b)
            .collect();
        b.push(tube.clone());
        let off = foot - st.p;
        let n = (off - tau * off.dot(tau)).normalize();
        let across = half_space(st.p - n * slab, n, foot);
        let gap = |p: Point3<T>| tube.at(p) + chord;
        if !screened(body, (request, taken), f, &b, Some(&across), gap, band)? {
            b.push(across);
        }
        replaces.push((f, b));
    }
    // The box: the spine's window, widened by the cross-section's reach
    // from its spine point (the edge point's for a chamfer).
    let axis_pt = st.c.map_or(st.p, |(c, _)| c);
    let base = axis_pt + tau * (start - axis_pt).dot(tau);
    let (a, b) = (base + tau * w_lo, base + tau * w_hi);
    Ok(Reach {
        bounds,
        core: if exact { core } else { Vec::new() },
        lo: Point3::new(
            a.x.min(b.x) - reach,
            a.y.min(b.y) - reach,
            a.z.min(b.z) - reach,
        ),
        hi: Point3::new(
            a.x.max(b.x) + reach,
            a.y.max(b.y) + reach,
            a.z.max(b.z) + reach,
        ),
        chain,
        sheet: None,
        sides,
        surface: link.blend.surface.clone(),
        replaces,
        confine: None,
        chains: vec![chain],
        skip,
        width: reach,
        spine: Some((link.edge, tau)),
        strips,
    })
}

/// **The reach of one link about a circular spine**, over the whole
/// turn, every bound a function of the torus's own meridian sheet.
fn circular_reach<T: Decide + Bounds>(
    body: &Body<T>,
    link: &Link<T>,
    st: &Station<T>,
    request: &[VertexKey],
    chain: usize,
    band: Band,
) -> Result<Reach<T>, BlendError> {
    let Surface::Torus {
        center,
        axis,
        major_radius,
        minor_radius,
        ..
    } = link.blend.surface
    else {
        return Err(disagrees(
            EntityId::Edge(link.edge),
            "a circular spine's band is not a torus",
        ));
    };
    let (o, a) = canonical_axis(center, axis);
    let (c, _) =
        st.c.ok_or_else(|| disagrees(EntityId::Edge(link.edge), "a torus band's spine point"))?;
    let reach = (st.p - c).norm();
    // A torus about its own axis: `big + reach` across, `reach` along.
    let ext = |p: T, q: T| major_radius * (p.powi(2) + q.powi(2)).sqrt() + reach;
    let e = Vec3::new(ext(a.y, a.z), ext(a.z, a.x), ext(a.x, a.y));
    let (lo, hi) = (center - e, center + e);
    let sheet = |x: Point3<T>| sheet_of(o, a, x);
    let in_sheet = |f: SheetFn<T>, slack: T| Bound::Sheet { o, a, f, slack };
    let mut bounds = Vec::new();
    // The supports, read about the band's axis.
    let (qr, qz) = sheet(st.inside());
    let mut exact = link.start == link.end;
    let mut sides = Vec::new();
    for f in [link.face_a, link.face_b] {
        let s = surface_of(body, f)?;
        exact &= matches!(s, Surface::Plane { .. } | Surface::Cylinder { .. });
        if let Some((g, slack)) = sheet_fn(&s, o, a, (lo, hi)) {
            let at = g.at(qr, qz);
            bounds.push((Role::Side(f), in_sheet(g.scaled(-sign(at)), slack)));
            sides.push(surface_key(body, f)?);
        }
    }
    // The sector, on P's side.
    let (pc, ps) = (sheet(c), sheet(st.p));
    for f in [st.fa, st.fb] {
        let pf = sheet(f);
        let u = (pf.0 - pc.0, pf.1 - pc.1);
        let v = (ps.0 - pc.0, ps.1 - pc.1);
        let k = (v.0 * u.0 + v.1 * u.1) / (u.0.powi(2) + u.1.powi(2));
        let w = (v.0 - u.0 * k, v.1 - u.1 * k);
        let len = (w.0.powi(2) + w.1.powi(2)).sqrt();
        let (alpha, beta) = (-w.0 / len, -w.1 / len);
        let line = SheetFn::Line {
            alpha,
            beta,
            gamma: -(alpha * pc.0 + beta * pc.1),
        };
        bounds.push((Role::Other, in_sheet(line, T::zero())));
    }
    // Outside the ball, inside the disk through P.
    let zc = sheet(center).1;
    for (role, r, sigma) in [
        (Role::Band, minor_radius, -T::one()),
        (Role::Disk, reach, T::one()),
    ] {
        let circle = SheetFn::Circle {
            p: major_radius,
            q: zc,
            r,
            sigma,
        };
        bounds.push((role, in_sheet(circle, T::zero())));
    }
    // Per support, the disk of the chord about the rim in the meridian
    // sheet: a closed rim bounds its planar support, so the face lies on
    // one side of the rim alone.
    let mut replaces = Vec::new();
    for (f, foot) in [(link.face_a, st.fa), (link.face_b, st.fb)] {
        let chord = (foot - st.p).norm();
        let ring = in_sheet(
            SheetFn::Circle {
                p: ps.0,
                q: ps.1,
                r: chord,
                sigma: T::one(),
            },
            T::zero(),
        );
        let gap = |p: Point3<T>| ring.at(p) + chord;
        // The disk has no one-sided form to narrow to: `screened`
        // refuses a vertex in it, and its vacancy changes nothing here.
        let _vacant = screened(
            body,
            (request, &[]),
            f,
            core::slice::from_ref(&ring),
            None,
            gap,
            band,
        )?;
        replaces.push((f, vec![ring]));
    }
    // Over line traces (a plane, a cylinder) a whole closed rim's bounds
    // are its band's own cross-section revolved, and `at_strict` takes
    // the slack the other way; a cone is read on both nappes and a
    // sphere's or a torus's trace is a circle, so those stay bounds.
    Ok(Reach {
        core: if exact {
            bounds.iter().map(|(_, b)| b.clone()).collect()
        } else {
            Vec::new()
        },
        bounds,
        lo,
        hi,
        chain,
        sheet: Some((o, a)),
        sides,
        surface: link.blend.surface.clone(),
        replaces,
        confine: None,
        chains: vec![chain],
        skip: Vec::new(),
        width: reach,
        spine: None,
        strips: Vec::new(),
    })
}

/// **An axis's canonical frame**: the point of the line nearest the
/// origin, and its unit direction signed so that its largest component is
/// positive. Coaxial bands share it, so the bounds of one are functions
/// of the other's sheet too ([`Reach::sheet`]); a frame that differs in
/// a bit only loses that sharing.
fn canonical_axis<T: Bounds>(p: Point3<T>, axis: Vec3<T>) -> (Point3<T>, Vec3<T>) {
    let a = axis.normalize();
    let k = [a.x, a.y, a.z]
        .into_iter()
        .max_by(|x, y| x.lo().abs().total_cmp(&y.lo().abs()))
        .unwrap_or(a.x);
    let a = if k.lo() < 0.0 { -a } else { a };
    let v = p - Point3::origin();
    (p - a * v.dot(a), a)
}

/// A corner patch's reach, and the points its incident links' windows
/// widen to.
type CornerReach<T> = (Reach<T>, Vec<Point3<T>>);

/// **The reach of a corner patch** at `v` (module docs), and the points
/// whose run along each incident edge widens that link's window: the
/// ball's centre, or the chamfer's three feet. `None` where the corner is
/// not three plane supports with three requested links.
fn corner_reach<T: Decide + Bounds>(
    body: &Body<T>,
    v: VertexKey,
    links: &[&Link<T>],
    size: T,
    kind: BlendKind,
    chains: Vec<usize>,
) -> Result<Option<CornerReach<T>>, BlendError> {
    let chain = chains.first().copied().unwrap_or(0);
    let pv = point_of(body, v)?;
    let faces = faces_at(body, v)?;
    if faces.len() != 3 || links.len() != 3 {
        return Ok(None);
    }
    let mut surfaces = Vec::new();
    for f in &faces {
        let s = surface_of(body, *f)?;
        if !matches!(s, Surface::Plane { .. }) {
            return Ok(None);
        }
        surfaces.push(s);
    }
    let normals = [
        normal_at(body, faces[0], pv)?,
        normal_at(body, faces[1], pv)?,
        normal_at(body, faces[2], pv)?,
    ];
    let mut bounds = Vec::new();
    let mut confine = None;
    let (q, anchors, surface) = match kind {
        BlendKind::Fillet => {
            let ball = corner_ball([pv; 3], normals, size, links[0].convexity);
            let c = ball.center;
            bounds.push((Role::Band, Bound::OutsideBall { o: c, r: size }));
            // Every extreme point of the corner's material — the vertex,
            // the feet, where each edge meets its band's end plane — lies
            // within the centre's distance from the vertex, and on the
            // vertex's side of the plane through the centre normal to it.
            bounds.push((
                Role::Other,
                Bound::InsideBall {
                    o: c,
                    r: (pv - c).norm(),
                },
            ));
            bounds.push((Role::Other, half_space(c, c - pv, pv)));
            // Each band ends in the plane through the centre normal to its
            // edge, and where the centre lies ahead of the vertex along
            // that edge the patch lies on the vertex's side of it. Where
            // it lies behind (an oblique corner), the band runs past the
            // vertex and the plane bounds nothing here.
            for l in links {
                let far = if l.start == v { l.end } else { l.start };
                let tau = point_of(body, far)? - pv;
                if (c - pv).dot(tau).lo() > 0.0 {
                    bounds.push((Role::Other, half_space(c, tau, pv)));
                }
            }
            (pv + (c - pv) * T::from_f64(0.25), vec![c], ball.surface)
        }
        BlendKind::Chamfer => {
            // Each support's foot: where the two trimlines on it cross.
            let mut feet = Vec::new();
            for (f, n) in faces.iter().zip(normals) {
                let trims: Vec<&Curve3<T>> = links
                    .iter()
                    .filter_map(|l| l.trim_on(*f).map(|t| &t.0))
                    .collect();
                let [
                    Curve3::Line {
                        origin: o1,
                        dir: d1,
                    },
                    Curve3::Line {
                        origin: o2,
                        dir: d2,
                    },
                ] = trims[..]
                else {
                    return Ok(None);
                };
                feet.push(line_meet(*o1, *d1, *o2, *d2, n));
            }
            let feet = [feet[0], feet[1], feet[2]];
            let patch = chamfer_corner_patch(feet, normals);
            let Surface::Plane { origin, normal, .. } = patch else {
                return Ok(None);
            };
            bounds.push((Role::Band, half_space(origin, normal, pv)));
            // The patch is the triangle of the feet: each strip meets it
            // along the segment between the two feet on its supports.
            let mid = centroid(&feet);
            confine = Some(
                links
                    .iter()
                    .filter_map(|l| match l.blend.surface {
                        Surface::Plane { origin, normal, .. } => {
                            Some(half_space(origin, normal, mid))
                        }
                        _ => None,
                    })
                    .collect::<Vec<_>>(),
            );
            (
                centroid(&[pv, feet[0], feet[1], feet[2]]),
                vec![pv, feet[0], feet[1], feet[2]],
                patch,
            )
        }
    };
    for (f, s) in faces.iter().zip(&surfaces) {
        bounds.extend(side(s, q).map(|b| (Role::Side(*f), b)));
    }
    let kept: Vec<Bound<T>> = bounds
        .iter()
        .filter(|(role, _)| !matches!(role, Role::Side(_)))
        .map(|(_, b)| b.clone())
        .collect();
    let far = anchors
        .iter()
        .fold(T::zero(), |m, x| m.max((*x - pv).norm()));
    let far = far + far;
    let e = Vec3::new(far, far, far);
    Ok(Some((
        Reach {
            bounds,
            core: Vec::new(),
            lo: pv - e,
            hi: pv + e,
            chain,
            sheet: None,
            sides: faces
                .iter()
                .map(|f| surface_key(body, *f))
                .collect::<Result<_, _>>()?,
            replaces: faces.iter().map(|f| (*f, kept.clone())).collect(),
            confine,
            chains,
            surface,
            skip: {
                let mut at_v = faces.clone();
                at_v.sort_unstable();
                at_v
            },
            width: far,
            spine: None,
            strips: Vec::new(),
        },
        anchors,
    )))
}

/// A cell's verdict, and the face's least bound so far.
struct Least<T: Real> {
    least: Option<T>,
    spent: usize,
}

impl<T: Bounds> Least<T> {
    fn new() -> Self {
        Self {
            least: None,
            spent: 0,
        }
    }

    fn fold(&mut self, m: T) {
        self.least = Some(self.least.map_or(m, |l| l.min(m)));
    }

    /// Read one cell: `inner` is the reach's bound over it, `off` the
    /// face surface's; `small` says it cannot usefully be halved. True
    /// when the cell is to be halved.
    fn read(&mut self, inner: T, off: Option<T>, small: bool, band: Band) -> bool {
        let m = off.map_or(inner, |off| inner.max(off));
        if m.lo() >= band.escalate() {
            self.fold(m);
            return false;
        }
        // A cell neither bound clears may hold a point of the face, so
        // what it gives the face is the reach's bound alone.
        self.spent += 1;
        if small || self.spent >= CELL_BUDGET {
            self.fold(inner);
            return false;
        }
        true
    }

    fn done(self, band: Band) -> T {
        self.least.unwrap_or_else(|| T::from_f64(band.escalate()))
    }
}

/// **The least cell bound of one face against one reach in space**
/// (module docs): positive certifies the face clear of it.
fn face_bound<T: Decide + Bounds>(
    reach: &Reach<T>,
    surface: &Surface<T>,
    (lo, hi): ([f64; 3], [f64; 3]),
    extra: &dyn Fn(&Cell<T>) -> Option<T>,
    band: Band,
) -> T {
    // Breadth first, so that when the budget runs out every cell left is
    // as small as the ones already read.
    let mut queue = VecDeque::from([(lo, hi)]);
    let mut least = Least::new();
    while let Some((lo, hi)) = queue.pop_front() {
        let cell = Cell::<T>::new(lo, hi);
        let inner = reach
            .bounds
            .iter()
            .map(|(_, b)| b.low(&cell))
            .reduce(|m, g| m.max(g))
            .unwrap_or_else(T::zero);
        let small = cell.lever.hi() <= band.escalate();
        let off = match (off_surface(surface, &cell), extra(&cell)) {
            (Some(a), Some(b)) => Some(a.max(b)),
            (a, b) => a.or(b),
        };
        if least.read(inner, off, small, band) {
            queue.extend(halves(lo, hi));
        }
    }
    least.done(band)
}

/// A box's two halves across its longest side.
fn halves(lo: [f64; 3], hi: [f64; 3]) -> [([f64; 3], [f64; 3]); 2] {
    let k = (0..3)
        .max_by(|&i, &j| (hi[i] - lo[i]).total_cmp(&(hi[j] - lo[j])))
        .unwrap_or(0);
    let mid = 0.5 * (lo[k] + hi[k]);
    let (mut h1, mut l2) = (hi, lo);
    h1[k] = mid;
    l2[k] = mid;
    [(lo, h1), (l2, hi)]
}

/// **How surely a whole face lies inside a band's own material**: the
/// most of the core's strict upper bounds over the cells of the face's
/// box that may hold a point of its surface, `Some` only when each of
/// those lies in the material by `escalate` — which puts every point of
/// the face there, the box holding the face. `None` when a cell is
/// neither off the surface nor inside before it can usefully be halved
/// or the face's [`CELL_BUDGET`] is spent, or for a poison box.
fn inside_bound<T: Bounds>(
    core: &[Bound<T>],
    surface: &Surface<T>,
    (lo, hi): ([f64; 3], [f64; 3]),
    band: Band,
) -> Option<T> {
    if lo.iter().chain(&hi).any(|x| x.is_nan()) {
        return None;
    }
    let mut queue = VecDeque::from([(lo, hi)]);
    let mut most: Option<T> = None;
    let mut spent = 0;
    while let Some((lo, hi)) = queue.pop_front() {
        let cell = Cell::<T>::new(lo, hi);
        if off_surface(surface, &cell).is_some_and(|off| sure(off, band)) {
            continue;
        }
        let inner = core
            .iter()
            .map(|b| b.high_strict(&cell))
            .reduce(|m, g| m.max(g))?;
        if sure(T::zero() - inner, band) {
            most = Some(most.map_or(inner, |m| m.max(inner)));
            continue;
        }
        spent += 1;
        if spent >= CELL_BUDGET || cell.lever.hi() <= band.escalate() {
            return None;
        }
        queue.extend(halves(lo, hi));
    }
    most
}

/// A face's image in the sheet, along its own trace: an axial range for
/// a sphere, a cylinder or a cone, a radial one for a plane.
enum Clip<T: Real> {
    Z(T, T),
    Rho(T, T),
}

/// **The extent of a face's image along its trace**, read from its
/// boundary (`None` where it cannot be): a sphere and a cone coaxial
/// with the trace, a cylinder and a plane at any pose.
///
/// The image of a connected face is an arc of the trace whose ends are
/// images of its boundary or a singular point on the axis (a pole, an
/// apex, a plane's axis point) the face contains. Over latitude and
/// meridian boundary edges the azimuth winding `W` counts those points;
/// on a sphere, where `W = 0` leaves none or both poles, the sign of
/// `∮ (z − z_c) dθ = 2πR·k − A/R` (`k` the poles inside) decides. Any
/// other boundary leaves the box alone; a torus is not clipped.
///
/// A cylinder and a plane need no shared axis. Along a cylinder's
/// ruling `z` is affine, so the ruling through any point of a face
/// leaves it at a boundary point no lower, and one no higher: the
/// boundary's span is the face's, at every pose of the cylinder. On a
/// plane `ρ` is convex, so its maximum over a face is on the boundary;
/// a plane the axis crosses has its one minimum `0` there, inside the
/// face exactly when the boundary winds once about the axis, and on a
/// plane parallel to the axis `ρ` is constant along the axis's
/// direction, so the boundary holds the minimum too.
fn sheet_clip<T: Decide + Bounds>(
    body: &Body<T>,
    face: FaceKey,
    surface: &Surface<T>,
    o: Point3<T>,
    a: Vec3<T>,
    band: Band,
) -> Option<Clip<T>> {
    let tilt = |b: Vec3<T>| tilt(b, a);
    let off = |p: Point3<T>| sheet_of(o, a, p).0;
    let near = |x: T| x.hi() <= band.zero();
    // The surface, read about the band's axis.
    enum Kind<T: Real> {
        Sphere { zc: T, r: T, center: Point3<T> },
        Cylinder,
        Cone { z_apex: T },
        Plane,
    }
    let kind = match surface {
        Surface::Sphere { center, radius, .. } if near(off(*center)) => Kind::Sphere {
            zc: sheet_of(o, a, *center).1,
            r: *radius,
            center: *center,
        },
        Surface::Cylinder { .. } => Kind::Cylinder,
        Surface::Cone { apex, axis, .. } if near(off(*apex)) && near(tilt(*axis)) => Kind::Cone {
            z_apex: sheet_of(o, a, *apex).1,
        },
        Surface::Plane { .. } => Kind::Plane,
        _ => return None,
    };
    let pad = T::from_f64(band.escalate());
    let f = body.get_face(face)?;
    let mut range: Option<[T; 4]> = None;
    let mut grow = |b: [T; 4]| {
        range = Some(range.map_or(b, |r: [T; 4]| {
            [
                r[0].min(b[0]),
                r[1].max(b[1]),
                r[2].min(b[2]),
                r[3].max(b[3]),
            ]
        }));
    };
    // The azimuth the boundary turns through and `∮ (z − z_c) dθ`, each
    // with a bound on its error (module docs of [`sheet_clip`]).
    let mut turn = [T::zero(); 4];
    let zc = match kind {
        Kind::Sphere { zc, .. } => zc,
        _ => T::zero(),
    };
    for lp in core::iter::once(f.outer).chain(f.rings.iter().copied()) {
        let first = match body.get_loop(lp)?.boundary {
            topo::LoopBoundary::Cycle { first } => first,
            topo::LoopBoundary::Empty { vertex } => {
                let (r, z) = sheet_of(o, a, super::surgery::point_of(body, vertex)?);
                grow([r - pad, r + pad, z - pad, z + pad]);
                continue;
            }
        };
        for he in body.loop_cycle(first)? {
            let h = body.get_half_edge(he)?;
            let e = body.get_edge(h.edge)?;
            let forward = e.he_plus == he;
            let cert = body.stored_curve(h.edge).ok()?;
            let (t0, t1) = cert.params();
            let carrier = cert.carrier();
            let (p0, p1) = (carrier.eval(t0), carrier.eval(t1));
            let aabb = match carrier {
                Curve3::Line { .. } => {
                    let lo = |k: fn(&Point3<T>) -> T| k(&p0).min(k(&p1));
                    let hi = |k: fn(&Point3<T>) -> T| k(&p0).max(k(&p1));
                    [
                        [lo(|p| p.x).lo(), lo(|p| p.y).lo(), lo(|p| p.z).lo()],
                        [hi(|p| p.x).hi(), hi(|p| p.y).hi(), hi(|p| p.z).hi()],
                    ]
                }
                Curve3::Circle { .. } => {
                    let b = geom::curves::boxes::circle_arc_aabb(carrier, t0, t1, p0, p1)?;
                    [[b.min_x, b.min_y, b.min_z], [b.max_x, b.max_y, b.max_z]]
                }
                Curve3::Ellipse { .. } => {
                    let b = geom::curves::boxes::ellipse_arc_aabb(carrier, t0, t1, p0, p1)?;
                    [[b.min_x, b.min_y, b.min_z], [b.max_x, b.max_y, b.max_z]]
                }
                Curve3::Nurbs(n) => {
                    let b = geom::curves::boxes::nurbs_curve_aabb(n);
                    [[b.min_x, b.min_y, b.min_z], [b.max_x, b.max_y, b.max_z]]
                }
                Curve3::Spiric { .. } => return None,
            };
            if aabb.iter().flatten().any(|v| !v.is_finite()) {
                return None;
            }
            let [r0, r1, z0, z1] = sheet_box(o, a, &Cell::<T>::new(aabb[0], aabb[1]));
            // The edge's sheet image: its box's, narrowed below for a
            // latitude circle, whose box holds the axis it rings.
            let mut image = [r0, r1, z0, z1];
            let dt = if forward { t1 - t0 } else { t0 - t1 };
            let pi = T::pi();
            let (z0e, z1e) = (sheet_of(o, a, p0).1 - zc, sheet_of(o, a, p1).1 - zc);
            let z_far = z0e
                .abs()
                .max(z1e.abs())
                .max((z0 - zc).abs())
                .max((z1 - zc).abs());
            let (d_theta, e_theta, e_j, z_e) = match *carrier {
                Curve3::Circle {
                    center,
                    axis,
                    radius,
                    ..
                } => {
                    let ax = axis.normalize();
                    let dot = ax.dot(a);
                    let d_lat = off(center) + tilt(ax) * radius;
                    // The chord to the nearest ⊥ direction,
                    // `√(2 − 2·cos φ)` with `sin φ = |dot|`, spelt
                    // without the cancellation near ⊥.
                    let lean = dot.abs() * T::from_f64(2.0).sqrt()
                        / (T::one() + (T::one() - dot.powi(2)).max(T::zero()).sqrt()).sqrt();
                    let d_mer = off(center) + lean * radius;
                    if d_lat.hi() <= d_mer.lo() {
                        // A latitude circle: the azimuth turns with the
                        // parameter; a circle displaced by `D` from the
                        // coaxial one sees each point's azimuth move by
                        // at most `π/2·D/ρ_min`.
                        let e = pi * d_lat / (radius - d_lat).max(T::zero());
                        let z = sheet_of(o, a, center).1 - zc;
                        // Every point lies within `d_lat` of the
                        // coaxial circle's one sheet point.
                        let zs = z + zc;
                        image = [
                            image[0].max(radius - d_lat),
                            image[1].min(radius + d_lat),
                            image[2].max(zs - d_lat),
                            image[3].min(zs + d_lat),
                        ];
                        // `|dot| ≥ 1/√2` on this arm, so its sign is
                        // decided; were it not, no clip is read.
                        let along = if dot.lo() > 0.0 {
                            dt
                        } else if dot.hi() < 0.0 {
                            -dt
                        } else {
                            return None;
                        };
                        (along, e, (z.abs() + d_lat) * e + d_lat * dt.abs(), z)
                    } else {
                        // A meridian edge clear of the axis: no turn,
                        // within the same bound about the meridian it
                        // is displaced from.
                        let e = pi * d_mer / (r0 - d_mer).max(T::zero());
                        (T::zero(), e, (z_far + d_mer) * e, T::zero())
                    }
                }
                Curve3::Line { .. } => {
                    // A segment clear of the axis turns through the
                    // angle between its ends' radial directions.
                    let (v0, v1) = (p0 - o, p1 - o);
                    let (u0, u1) = (v0 - a * v0.dot(a), v1 - a * v1.dot(a));
                    let turn = u0.cross(u1).dot(a).atan2(u0.dot(u1));
                    let turn = if forward { turn } else { -turn };
                    let span = (z0e - z1e).abs() / T::from_f64(2.0);
                    // Its sheet image: no nearer the axis than its line
                    // (an axial segment's reads `0`), no farther than an
                    // end, and between its ends' heights.
                    let d = u1 - u0;
                    let near_axis =
                        u0.cross(d).norm() / d.norm().max(T::from_f64(f64::MIN_POSITIVE));
                    image = [
                        image[0].max(near_axis),
                        image[1].min(u0.norm().max(u1.norm())),
                        image[2].max((z0e + zc).min(z1e + zc)),
                        image[3].min((z0e + zc).max(z1e + zc)),
                    ];
                    let clear = if image[0].lo() > 0.0 {
                        T::zero()
                    } else {
                        T::from_f64(f64::NAN)
                    };
                    (
                        turn,
                        clear,
                        span * turn.abs(),
                        (z0e + z1e) / T::from_f64(2.0),
                    )
                }
                _ => {
                    let nan = T::from_f64(f64::NAN);
                    (nan, nan, nan, nan)
                }
            };
            grow([
                image[0] - pad,
                image[1] + pad,
                image[2] - pad,
                image[3] + pad,
            ]);
            turn = [
                turn[0] + d_theta,
                turn[1] + e_theta,
                turn[2] + z_e * d_theta,
                turn[3] + e_j,
            ];
        }
    }
    let [r_lo, r_hi, z_lo, z_hi] = range?;
    let tau = T::tau();
    let (w, ew) = (turn[0] / tau, turn[1] / tau);
    let none = (w.abs() + ew).hi() < 0.25;
    let one = ((w.abs() - T::one()).abs() + ew).hi() < 0.25;
    Some(match kind {
        Kind::Cylinder => Clip::Z(z_lo, z_hi),
        Kind::Plane => match (none, one) {
            (true, _) => Clip::Rho(r_lo, r_hi),
            (_, true) => Clip::Rho(T::zero(), r_hi),
            _ => return None,
        },
        Kind::Cone { z_apex } => match (none, one) {
            (true, _) => Clip::Z(z_lo, z_hi),
            (_, true) => Clip::Z(z_lo.min(z_apex), z_hi.max(z_apex)),
            _ => return None,
        },
        Kind::Sphere { zc, r, center } => {
            // Orient by the face's own outward normal against the radial.
            let sigma = outward(body, face, center + a * r)?.dot(a);
            let (w, j, ej) = (w * sigma, turn[2] * sigma, turn[3]);
            let (north, south) = (zc + r, zc - r);
            let zero = T::from_f64(band.zero());
            if ((w - T::one()).abs() + ew).hi() < 0.25 {
                Clip::Z(z_lo, north)
            } else if ((w + T::one()).abs() + ew).hi() < 0.25 {
                Clip::Z(south, z_hi)
            } else if none && (j + ej + zero).hi() < 0.0 {
                Clip::Z(z_lo, z_hi)
            } else if none && (j - ej - zero).lo() > 0.0 {
                Clip::Z(south, north)
            } else {
                return None;
            }
        }
    })
}

/// **The least cell bound of one face against a circular reach, in the
/// reach's meridian sheet** (module docs): the face's surface read about
/// the reach's axis with its slack, over the sheet rectangle of the
/// face's clipped box. `None` where the surface has no sheet form or a
/// bound of the reach is not a sheet bound.
fn face_bound_in_sheet<T: Decide + Bounds>(
    body: &Body<T>,
    face: Option<FaceKey>,
    reach: &Reach<T>,
    surface: &Surface<T>,
    (lo, hi): ([f64; 3], [f64; 3]),
    extra: &dyn Fn(Rect<T>) -> Option<T>,
    band: Band,
) -> Option<T> {
    let (o, a) = reach.sheet?;
    let (f, slack) = sheet_fn(surface, o, a, (reach.lo, reach.hi))?;
    let mut rect = sheet_box(o, a, &Cell::<T>::new(lo, hi));
    // The face's own extent along its trace, where its boundary says it.
    match face.and_then(|face| sheet_clip(body, face, surface, o, a, band)) {
        Some(Clip::Z(z0, z1)) => {
            rect[2] = rect[2].max(z0);
            rect[3] = rect[3].min(z1);
        }
        Some(Clip::Rho(r0, r1)) => {
            rect[0] = rect[0].max(r0);
            rect[1] = rect[1].min(r1);
        }
        None => {}
    }
    if (rect[1] - rect[0]).hi() < 0.0 || (rect[3] - rect[2]).hi() < 0.0 {
        // The face's image misses the reach's box: nothing to meter.
        return Some(T::from_f64(band.escalate()));
    }
    let mut queue = VecDeque::from([rect]);
    let mut least = Least::new();
    while let Some(rect) = queue.pop_front() {
        let [r0, r1, z0, z1] = rect;
        let mut inner: Option<T> = None;
        for (_, b) in &reach.bounds {
            let g = b.low_in_sheet(rect)?;
            inner = Some(inner.map_or(g, |m| m.max(g)));
        }
        let (flo, fhi) = f.range(rect);
        let off = flo.max(-fhi) - slack;
        let off = extra(rect).map_or(off, |e| off.max(e));
        let diag = ((r1 - r0).powi(2) + (z1 - z0).powi(2)).sqrt();
        let small = diag.hi() <= band.escalate();
        if least.read(inner.unwrap_or_else(T::zero), Some(off), small, band) {
            let two = T::from_f64(2.0);
            if (r1 - r0).hi() >= (z1 - z0).hi() {
                let m = (r0 + r1) / two;
                queue.push_back([r0, m, z0, z1]);
                queue.push_back([m, r1, z0, z1]);
            } else {
                let m = (z0 + z1) / two;
                queue.push_back([r0, r1, z0, m]);
                queue.push_back([r0, r1, m, z1]);
            }
        }
    }
    Some(least.done(band))
}

/// **A face's boundary read against a reach's core**, at
/// [`EDGE_SAMPLES`] places along each boundary edge, ends included:
/// the least strict bound ([`Bound::at_strict`]) of a place — negative,
/// it is in the band's material — and the most plain one
/// ([`Bound::at`]) — positive, it is out of it. `None` for a face with
/// no boundary place.
fn boundary_reading<T: Decide>(
    body: &Body<T>,
    face: FaceKey,
    core: &[Bound<T>],
) -> Result<Option<(T, T)>, BlendError> {
    let f = body
        .get_face(face)
        .ok_or_else(|| unresolved(EntityId::Face(face), "a face the band reach reads"))?;
    let most = |x: Point3<T>, g: fn(&Bound<T>, Point3<T>) -> T| {
        core.iter()
            .map(|b| g(b, x))
            .reduce(|m, g| m.max(g))
            .unwrap_or_else(T::zero)
    };
    let mut reading: Option<(T, T)> = None;
    let mut fold = |x: Point3<T>| {
        let (inner, outer) = (most(x, Bound::at_strict), most(x, Bound::at));
        reading = Some(reading.map_or((inner, outer), |(i, o)| (i.min(inner), o.max(outer))));
    };
    for lp in core::iter::once(f.outer).chain(f.rings.iter().copied()) {
        let l = body
            .get_loop(lp)
            .ok_or_else(|| unresolved(EntityId::Loop(lp), "a face's loop"))?;
        let first = match l.boundary {
            topo::LoopBoundary::Cycle { first } => first,
            topo::LoopBoundary::Empty { vertex } => {
                fold(point_of(body, vertex)?);
                continue;
            }
        };
        for he in body
            .loop_cycle(first)
            .ok_or_else(|| unwalked(EntityId::Loop(lp), "a face's cycle"))?
        {
            let edge = body
                .get_half_edge(he)
                .ok_or_else(|| unresolved(EntityId::HalfEdge(he), "a face's boundary"))?
                .edge;
            let (c, t0, t1) = carrier_of(body, edge)?;
            for i in 0..EDGE_SAMPLES {
                fold(c.eval(geom_brep::schedule_param(t0, t1, i, EDGE_SAMPLES)));
            }
        }
    }
    Ok(reading)
}

/// What a face is read against for the prism a straight band removes or
/// fills ([`prism_inside`]): the spine's unit direction, the cycles its
/// caps' meter read inside the sliver or those its strip meter read
/// inside one support's strip, whether the section is convex (with that
/// support's side dropped, for the strip's), the depth of a curve over a
/// parameter range inside the band's exact core ([`curve_in_core`]),
/// and, for the strip's, the depth of a curve inside the core less its
/// side and, where the face lies in [`StripCore::along`], that support's
/// unit normal.
struct Prism<'a, T: Real> {
    tau: Vec3<T>,
    rings: &'a [TakenRing<T>],
    convex: bool,
    in_core: &'a CurveDepth<'a, T>,
    strip: Option<(&'a CurveDepth<'a, T>, Option<Vec3<T>>)>,
}

/// The depth of a curve over a parameter range inside a core.
type CurveDepth<'a, T> = dyn Fn(&Curve3<T>, (T, T)) -> Option<T> + 'a;

/// How many pieces one edge may be cut into against a core before it is
/// read uncertified ([`curve_in_core`]): a cost ceiling, as
/// [`CELL_BUDGET`] is, which decides only how far an edge is chased.
const EDGE_PIECES: usize = 1024;

/// Certify `k` at `deep`, keeping the deepest of its certifications.
fn certify<K: Ord, T: Real>(all: &mut BTreeMap<K, T>, k: K, deep: T) {
    all.entry(k)
        .and_modify(|d| *d = d.max(deep))
        .or_insert(deep);
}

/// **How deep inside a straight band's exact core every point of the
/// curve `eval` over `[t0, t1]` lies**: positive when certified, and
/// then a lower bound on each point's distance from the core's
/// complement; `None` for a curve not certified.
///
/// The curve is cut into pieces by parameter; a piece of parameter width
/// `w` lies within `σ·w/2` of the point at its middle parameter, `σ` the
/// bound `speed` on the carrier's speed, so in that point's box widened
/// by that much,
/// over which [`Bound::high_strict`] bounds each core bound from above.
/// A piece that does not certify is halved until it does, its radius
/// falls within the band's `escalate`, or [`EDGE_PIECES`] are spent.
/// What pieces remain then tighten the reading, the least of the
/// pieces' depths: the shallowest piece is halved while its radius
/// exceeds an eighth of its depth, each half reading the lesser of its
/// own bound and its parent's, both of which hold over it.
fn curve_in_core<T: Bounds>(
    speed: T,
    eval: &dyn Fn(T) -> Point3<T>,
    (t0, t1): (T, T),
    core: &[Bound<T>],
    band: Band,
) -> Option<T> {
    if core.is_empty() {
        return None;
    }
    let half = T::from_f64(0.5);
    // A piece's greatest core bound over its box, and the box's radius.
    let read = |a: T, b: T| {
        let m = eval((a + b) * half);
        let r = Bounds::hi(speed * (b - a).abs() * half);
        let cell = Cell::<T>::new(
            [m.x, m.y, m.z].map(|x| Bounds::lo(x) - r),
            [m.x, m.y, m.z].map(|x| Bounds::hi(x) + r),
        );
        let inner = core
            .iter()
            .map(|g| g.high_strict(&cell))
            .reduce(|m, g| m.max(g))?;
        Some((inner, r))
    };
    let mut queue = VecDeque::from([(t0, t1)]);
    let mut pieces: Vec<(T, T, T, f64)> = Vec::new();
    let mut spent = 0;
    while let Some((a, b)) = queue.pop_front() {
        let (inner, r) = read(a, b)?;
        if sure(T::zero() - inner, band) {
            pieces.push((a, b, inner, r));
            continue;
        }
        spent += 1;
        if spent >= EDGE_PIECES || r <= band.escalate() {
            return None;
        }
        let mid = (a + b) * half;
        queue.extend([(a, mid), (mid, b)]);
    }
    while spent < EDGE_PIECES {
        let Some((i, &(a, b, inner, r))) = pieces
            .iter()
            .enumerate()
            .max_by(|x, y| Bounds::hi(x.1.2).total_cmp(&Bounds::hi(y.1.2)))
        else {
            break;
        };
        if r <= band.escalate() || 8.0 * r <= -Bounds::hi(inner) {
            break;
        }
        spent += 1;
        pieces.swap_remove(i);
        let mid = (a + b) * half;
        for (p, q) in [(a, mid), (mid, b)] {
            let (own, r) = read(p, q)?;
            pieces.push((p, q, own.min(inner), r));
        }
    }
    pieces
        .iter()
        .map(|p| p.2)
        .reduce(|m, g| m.max(g))
        .map(|m| T::zero() - m)
}

/// **How surely a face lies wholly inside the prism a straight band
/// removes or fills**: positive when it certainly does, and then a
/// lower bound on how far inside; `None` when the face is not of the
/// shape the reading takes.
///
/// The band's material `M` is the prism over its cross-section `S⊥`
/// along the spine `τ`, between the two cap planes. A point is
/// certified at depth `δ` when it lies between the cap planes and its
/// projection along `τ` lies at least `δ` inside `S⊥`. Each edge of the
/// face's boundary is certified one of three ways:
/// - an edge of one of the `rings`: on a cap of normal `n`, at its depth
///   in the cap times `|n·τ|`; on a support's strip, read in the core
///   less that support's side ([`Prism`]'s `strip`);
/// - an edge read inside the band's exact core ([`edge_in_core`]);
/// - a line `u → w` along `d` between two certified vertices, each of
///   whose points lies between the cap planes and within
///   `|w − u|·|d × τ|` of an end's projection (the lines' slack).
///
/// The face then reads at the least of those depths:
/// - less the slack, on a plane whose `|n·τ|` is certainly positive
///   (`by_boundary`, the caller's reading): it
///   projects one-to-one onto `S⊥`'s plane, onto the region its boundary
///   encloses there, and each component of `S⊥`'s points deeper than
///   `δ` is simply connected (`S⊥`'s complement is connected), so holds
///   all it encloses;
/// - less the slack and its box's diagonal times `|a × τ|`, on a surface
///   ruled along `a` (a cylinder's axis, or a plane's direction nearest
///   `τ`, for which `|a × τ| = |n·τ|`): each point lies on an `a`-line
///   between two points of its boundary;
/// - as it is, where `S⊥` is `convex`: depth inside a convex set is
///   concave, and each point of a line or a face is a convex combination
///   of boundary points.
///
/// **On a support's strip** `S⊥` is `S⁺`, the section of the core less
/// that support's side; every edge not on a ring is read in the whole
/// core. Where the support's normal `ν` is given, a line's or a ruling's
/// slack along `d` is the lesser of `|d × τ|` and `2·|d × ν|` per unit
/// of its length.
fn prism_inside<T: Decide>(
    body: &Body<T>,
    face: FaceKey,
    surface: &Surface<T>,
    (flo, fhi): ([f64; 3], [f64; 3]),
    prism: &Prism<'_, T>,
    by_boundary: bool,
) -> Result<Option<T>, BlendError> {
    let Prism {
        tau,
        rings,
        convex,
        strip,
        ..
    } = *prism;
    if rings.is_empty() || flo.iter().chain(&fhi).any(|x| !x.is_finite()) {
        return Ok(None);
    }
    let nu = strip.and_then(|(_, nu)| nu);
    // How far a line or ruling strays per unit of its length, from its
    // misalignment with `τ` and with `ν`.
    let charge = |off_tau: T, off_nu: &dyn Fn(Vec3<T>) -> T| {
        nu.map_or(off_tau, |nu| off_tau.min(T::from_f64(2.0) * off_nu(nu)))
    };
    let stray = |a: Vec3<T>| {
        let a = a.normalize();
        charge(a.cross(tau).norm(), &|nu| a.cross(nu).norm())
    };
    // The ruling's stray, or `None` for a plane read by its boundary: a
    // plane is ruled along its direction nearest `τ`, or nearest `ν`.
    let ruled = match surface {
        Surface::Cylinder { axis, .. } => Some(stray(*axis)),
        Surface::Plane { normal, .. } => (!by_boundary).then(|| {
            let n = normal.normalize();
            charge(n.dot(tau).abs(), &|nu| n.dot(nu).abs())
        }),
        _ => return Ok(None),
    };
    let mut edges = BTreeMap::new();
    let mut vertices = BTreeMap::new();
    let mut least: Option<T> = None;
    for r in rings {
        let Some(walk) = loop_walk(body, r.ring) else {
            return Ok(None);
        };
        match (r.on, strip) {
            (RingOn::Cap { .. }, None) => {
                let Surface::Plane { normal, .. } = surface_of(body, r.face)? else {
                    return Ok(None);
                };
                let deep = (T::zero() - r.depth) * normal.normalize().dot(tau).abs();
                least = Some(least.map_or(deep, |l| l.min(deep)));
                for (_, start, edge) in walk {
                    certify(&mut edges, edge, deep);
                    certify(&mut vertices, start, deep);
                }
            }
            (RingOn::Strip, Some((beside, _))) => {
                for (he, u, edge) in walk {
                    let w = body
                        .half_edge_end(he)
                        .ok_or_else(|| unresolved(EntityId::HalfEdge(he), "a ring edge's end"))?;
                    let carrier = carrier_of(body, edge)?;
                    let Some(deep) = edge_in_core(body, &carrier, [u, w], beside, &mut vertices)?
                    else {
                        return Ok(None);
                    };
                    least = Some(least.map_or(deep, |l| l.min(deep)));
                    certify(&mut edges, edge, deep);
                }
            }
            _ => return Ok(None),
        }
    }
    let Some(mut least) = least else {
        return Ok(None);
    };
    let f = body
        .get_face(face)
        .ok_or_else(|| unresolved(EntityId::Face(face), "a face the band reach reads"))?;
    // Every edge but a line first: its ends certify the lines.
    let mut lines = Vec::new();
    for lp in core::iter::once(f.outer).chain(f.rings.iter().copied()) {
        let Some(walk) = loop_walk(body, lp) else {
            return Ok(None);
        };
        for (he, u, edge) in walk {
            if let Some(&deep) = edges.get(&edge) {
                least = least.min(deep);
                continue;
            }
            let w = body
                .half_edge_end(he)
                .ok_or_else(|| unresolved(EntityId::HalfEdge(he), "a face edge's end"))?;
            let carrier = carrier_of(body, edge)?;
            if let Curve3::Line { dir, .. } = carrier.0 {
                lines.push((carrier, u, w, dir));
                continue;
            }
            let Some(deep) = edge_in_core(body, &carrier, [u, w], prism.in_core, &mut vertices)?
            else {
                return Ok(None);
            };
            least = least.min(deep);
        }
    }
    // A line between two certified vertices takes the slack; any other
    // is read in the core, its ends certified with it, or waits for its
    // ends.
    let certified = |v: &BTreeMap<VertexKey, T>, u, w| v.get(&u).copied().zip(v.get(&w).copied());
    let mut pending = Vec::new();
    for (carrier, u, w, dir) in lines {
        if certified(&vertices, u, w).is_some() {
            pending.push((u, w, dir));
        } else if let Some(deep) =
            edge_in_core(body, &carrier, [u, w], prism.in_core, &mut vertices)?
        {
            least = least.min(deep);
        } else {
            pending.push((u, w, dir));
        }
    }
    let mut slack = T::zero();
    for (u, w, dir) in pending {
        let Some((du, dw)) = certified(&vertices, u, w) else {
            return Ok(None);
        };
        least = least.min(du).min(dw);
        let run = (point_of(body, w)? - point_of(body, u)?).norm();
        slack = slack.max(run * stray(dir));
    }
    if convex {
        return Ok(Some(least));
    }
    let Some(tilt) = ruled else {
        return Ok(Some(least - slack));
    };
    let corner = |v: [f64; 3]| {
        let [x, y, z] = v.map(T::from_computed);
        Point3::new(x, y, z)
    };
    let diagonal = (corner(fhi) - corner(flo)).norm();
    Ok(Some(least - slack - diagonal * tilt))
}

/// **An edge read inside a straight band's exact core**, or the core
/// less a support's side: its depth (`in_core`), its ends `u` and `w`
/// certified at that depth less each one's distance from the carrier's
/// nearer end — every point of the curve lies at that depth, and depth
/// is 1-Lipschitz — and `None` when the curve is not certified.
fn edge_in_core<T: Decide>(
    body: &Body<T>,
    (c, t0, t1): &(Curve3<T>, T, T),
    ends: [VertexKey; 2],
    in_core: &CurveDepth<'_, T>,
    vertices: &mut BTreeMap<VertexKey, T>,
) -> Result<Option<T>, BlendError> {
    let Some(deep) = in_core(c, (*t0, *t1)) else {
        return Ok(None);
    };
    let (a, b) = (c.eval(*t0), c.eval(*t1));
    for v in ends {
        let p = point_of(body, v)?;
        certify(vertices, v, deep - (p - a).norm().min((p - b).norm()));
    }
    Ok(Some(deep))
}

/// **The request's three-way verdict**, over every face and end-face
/// cycle that predicate 2's reach and a cut-off's end-face meter read
/// not clear. A crossing refuses at once, nothing outranking it; a bound
/// outranks an inside reading; so the request reads wholly inside only
/// when every reading that is not clear does, whatever order the faces
/// and cycles are read in. Of several bounds it names the least margin,
/// and of several inside readings the shallowest, which bounds the depth
/// of every one; a tie on the margin's upper end names the least entity
/// ([`entity_order`]), so the refusal is the same in every read order.
///
/// An inside reading the carve consumes is a cycle on a straight band's
/// cap or support strip ([`TakenRing`]), a face of the prism over such rings
/// ([`prism_inside`]) or a face inside a straight band's exact core:
/// only when every inside reading is one does the request build
/// ([`Consumed`]); otherwise it refuses `WhollyInside`.
pub(crate) struct Unclear<T> {
    bound: Option<(T, BlendError)>,
    inside: Option<(T, BlendError)>,
    rings: Vec<TakenRing<T>>,
    faces: Vec<(FaceKey, EdgeKey)>,
    unconsumed: bool,
}

/// **A cycle a straight band takes, read wholly inside the face it lies
/// on's part of the band's region**: the crease whose band removes or
/// buries it, that face, the cycle, its depth (negative; its size a
/// lower bound on the cycle's distance from that part's boundary in the
/// face's plane), and where it lies ([`RingOn`]).
#[derive(Clone, Copy, Debug)]
pub(crate) struct TakenRing<T> {
    pub(crate) crease: EdgeKey,
    pub(crate) face: FaceKey,
    pub(crate) ring: LoopKey,
    pub(crate) depth: T,
    pub(crate) on: RingOn,
}

/// Where a [`TakenRing`] lies.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum RingOn {
    /// On a cap, inside its sliver, which was read as its exact triangle
    /// (`convex`) or not.
    Cap { convex: bool },
    /// On a support, inside the strip the band takes off it.
    Strip,
}

/// **What a request whose verdict is inside consumes**: the cycles
/// taken ([`TakenRing`]) and the faces read wholly inside the material
/// a straight band removes or fills, each with the crease of every band it was read
/// inside, and the refusal they would owe were the carve not to take
/// them ([`BlendError::WhollyInside`], the shallowest).
pub(crate) struct Consumed<T> {
    pub(crate) rings: Vec<TakenRing<T>>,
    /// Each face and a crease whose band's material it was read in.
    pub(crate) read: Vec<(FaceKey, EdgeKey)>,
    pub(crate) refusal: BlendError,
}

/// A total order on the entities refusals name: by kind, then by key.
fn entity_order(a: EntityId, b: EntityId) -> core::cmp::Ordering {
    use EntityId as E;
    let kind = |x: EntityId| match x {
        E::Solid(_) => 0,
        E::Shell(_) => 1,
        E::Face(_) => 2,
        E::Loop(_) => 3,
        E::HalfEdge(_) => 4,
        E::Edge(_) => 5,
        E::Vertex(_) => 6,
    };
    match (a, b) {
        (E::Solid(x), E::Solid(y)) => x.cmp(&y),
        (E::Shell(x), E::Shell(y)) => x.cmp(&y),
        (E::Face(x), E::Face(y)) => x.cmp(&y),
        (E::Loop(x), E::Loop(y)) => x.cmp(&y),
        (E::HalfEdge(x), E::HalfEdge(y)) => x.cmp(&y),
        (E::Edge(x), E::Edge(y)) => x.cmp(&y),
        (E::Vertex(x), E::Vertex(y)) => x.cmp(&y),
        _ => kind(a).cmp(&kind(b)),
    }
}

/// The entity a reach or end-face refusal names.
fn named(e: &BlendError) -> Option<EntityId> {
    match *e {
        BlendError::FaceClearance { at, .. } | BlendError::WhollyInside { at, .. } => Some(at),
        BlendError::RingClearance { face, .. } => Some(EntityId::Face(face)),
        _ => None,
    }
}

/// Whether `(margin, refusal)` takes the place of `held`: its margin's
/// upper end on the `better` side, or equal and naming a lesser entity.
fn displaces<T: Bounds>(
    held: Option<&(T, BlendError)>,
    margin: T,
    refusal: &BlendError,
    better: core::cmp::Ordering,
) -> bool {
    let Some((m, r)) = held else {
        return true;
    };
    match margin.hi().total_cmp(&m.hi()) {
        core::cmp::Ordering::Equal => match (named(refusal), named(r)) {
            (Some(a), Some(b)) => entity_order(a, b).is_lt(),
            _ => false,
        },
        o => o == better,
    }
}

impl<T: Bounds> Unclear<T> {
    /// Nothing read yet.
    pub(crate) const fn new() -> Self {
        Self {
            bound: None,
            inside: None,
            rings: Vec::new(),
            faces: Vec::new(),
            unconsumed: false,
        }
    }

    /// A reading certified neither clear, nor inside, nor crossing, its
    /// margin `margin`.
    pub(crate) fn bound(&mut self, margin: T, refusal: BlendError) {
        if displaces(
            self.bound.as_ref(),
            margin,
            &refusal,
            core::cmp::Ordering::Less,
        ) {
            self.bound = Some((margin, refusal));
        }
    }

    /// A reading certified wholly inside, `margin` its (negative) depth,
    /// that the carve does not consume.
    pub(crate) fn unconsumed_inside(&mut self, margin: T, refusal: BlendError) {
        self.unconsumed = true;
        self.deepen(margin, refusal);
    }

    /// A cycle certified wholly inside a straight band's cap sliver or
    /// support strip, which the carve consumes with the feature it
    /// bounds.
    pub(crate) fn taken_ring(&mut self, ring: TakenRing<T>, refusal: BlendError) {
        self.rings.push(ring);
        self.deepen(ring.depth, refusal);
    }

    /// A face certified wholly inside the prism the band of `crease`
    /// removes or fills ([`prism_inside`], or its exact core), which the
    /// carve consumes.
    fn prism_face(&mut self, face: FaceKey, crease: EdgeKey, depth: T, refusal: BlendError) {
        if !self.faces.contains(&(face, crease)) {
            self.faces.push((face, crease));
        }
        self.deepen(depth, refusal);
    }

    /// The cycles taken so far.
    fn taken(&self) -> &[TakenRing<T>] {
        &self.rings
    }

    fn deepen(&mut self, margin: T, refusal: BlendError) {
        if displaces(
            self.inside.as_ref(),
            margin,
            &refusal,
            core::cmp::Ordering::Greater,
        ) {
            self.inside = Some((margin, refusal));
        }
    }

    /// The request's refusal — a bound if any, else wholly inside where
    /// an inside reading is one the carve does not consume — or what it
    /// consumes (`None` where no reading was inside: every consumed
    /// reading is an inside one).
    ///
    /// # Errors
    ///
    /// That refusal.
    pub(crate) fn verdict(self) -> Result<Option<Consumed<T>>, BlendError> {
        if let Some((_, refusal)) = self.bound {
            return Err(refusal);
        }
        match self.inside {
            None => Ok(None),
            Some((_, refusal)) if self.unconsumed => Err(refusal),
            Some((_, refusal)) => Ok(Some(Consumed {
                rings: self.rings,
                read: self.faces,
                refusal,
            })),
        }
    }
}

/// **Meter every chain's reach against every face of the body it does
/// not exclude** (module docs), before any mutation, completing the
/// request's three-way verdict `unclear` (the end-face meter's readings
/// so far).
///
/// # Errors
///
/// [`BlendError::FaceClearance`] for a face crossing a reach, or not
/// certified clear of one; [`BlendError::WhollyInside`] when every face
/// and cycle read not clear lies wholly inside ([`Unclear`]);
/// [`BlendError::Escalated`] for an in-band margin;
/// [`BlendError::BodyNotIntact`] for a body the meter cannot read.
pub(crate) fn band_reach<T: Decide + Bounds>(
    body: &Body<T>,
    chains: &[Chain<T>],
    size: T,
    kind: BlendKind,
    band: Band,
    mut unclear: Unclear<T>,
) -> Result<Option<Consumed<T>>, BlendError> {
    let all: Vec<(usize, &Link<T>)> = chains
        .iter()
        .enumerate()
        .flat_map(|(ci, c)| c.links().map(move |l| (ci, l)))
        .collect();
    let mut requested: Vec<EdgeKey> = all.iter().map(|(_, l)| l.edge).collect();
    requested.sort_unstable();
    let at = |v: VertexKey| -> Vec<(usize, &Link<T>)> {
        all.iter()
            .filter(|(_, l)| l.start == v || l.end == v)
            .copied()
            .collect()
    };
    // Corner patches first: a link's window ends at its corner's.
    let mut reaches: Vec<Reach<T>> = Vec::new();
    let mut corners: Vec<(VertexKey, Vec<Point3<T>>)> = Vec::new();
    let mut verts: Vec<VertexKey> = all.iter().flat_map(|(_, l)| [l.start, l.end]).collect();
    verts.sort_unstable();
    verts.dedup();
    let ends_all = verts.clone();
    for v in verts {
        let fan = body
            .edges_of_vertex(v)
            .ok_or_else(|| unwalked(EntityId::Vertex(v), "a chain vertex's edge fan"))?;
        if !fan.iter().all(|e| requested.binary_search(e).is_ok()) {
            continue;
        }
        let here = at(v);
        let links: Vec<&Link<T>> = here.iter().map(|(_, l)| *l).collect();
        let mut on: Vec<usize> = here.iter().map(|(ci, _)| *ci).collect();
        on.sort_unstable();
        on.dedup();
        if let Some((r, anchors)) = corner_reach(body, v, &links, size, kind, on)? {
            reaches.push(r);
            corners.push((v, anchors));
        }
    }
    let taken: Vec<LoopKey> = unclear
        .taken()
        .iter()
        .filter(|r| r.on == RingOn::Strip)
        .map(|r| r.ring)
        .collect();
    let supports: Vec<Vec<FaceKey>> = chains
        .iter()
        .map(|c| c.links().flat_map(|l| [l.face_a, l.face_b]).collect())
        .collect();
    for (ci, link) in &all {
        let st = station(body, link)?;
        let mut r = if link.arm.is_coaxial_torus() {
            circular_reach(body, link, &st, &ends_all, *ci, band)?
        } else {
            straight_reach(body, link, &st, &corners, (&ends_all, &taken), *ci, band)?
        };
        r.skip.extend(&supports[*ci]);
        r.skip.sort_unstable();
        r.skip.dedup();
        reaches.push(r);
    }
    let boxes = FaceBoxes::of(body, band).map_err(|e| {
        degenerate(
            EntityId::Face(e.0),
            "a face's stored surface, which its box reads",
        )
    })?;
    // Two chains that share a vertex meet there in a corner or a turn,
    // which the corner predicate and the surgery judge: their bands
    // touch by construction.
    let ends: Vec<Vec<VertexKey>> = chains
        .iter()
        .map(|c| c.links().flat_map(|l| [l.start, l.end]).collect())
        .collect();
    let meet = |a: usize, b: usize| ends[a].iter().any(|v| ends[b].contains(v));
    let decision = BlendDecision::FaceClearance;
    let classify_bound = |bound: T| classify(BlendSite::Chain, decision, Margin::of(bound), band);
    for reach in &reaches {
        let rlo = [reach.lo.x.lo(), reach.lo.y.lo(), reach.lo.z.lo()];
        let rhi = [reach.hi.x.hi(), reach.hi.y.hi(), reach.hi.z.hi()];
        // A box clipped to the reach's: what of it lies in the reach lies
        // in both. A poison box reads as the reach's own.
        let clipped = |(flo, fhi): ([f64; 3], [f64; 3])| {
            let clip = |f: f64, r: f64, keep: fn(f64, f64) -> f64| {
                if f.is_nan() { r } else { keep(f, r) }
            };
            let lo: [f64; 3] = core::array::from_fn(|k| clip(flo[k], rlo[k], f64::max));
            let hi: [f64; 3] = core::array::from_fn(|k| clip(fhi[k], rhi[k], f64::min));
            (lo, hi)
        };
        for (face, fbox) in boxes.meeting(rlo, rhi) {
            if reach.skip.binary_search(&face).is_ok()
                || reach.sides.contains(&surface_key(body, face)?)
            {
                continue;
            }
            let surface = surface_of(body, face)?;
            // A support of another chain of the request is replaced
            // between that chain's edge and its trimline by that chain's
            // band (whose own surface is metered below): what survives is
            // the face outside that band's reach, so a cell that lies in it
            // whole is clear.
            let others: Vec<&[Bound<T>]> = reaches
                .iter()
                .filter(|r| r.chains.iter().all(|c| !reach.chains.contains(c)))
                .flat_map(|r| r.replaces.iter())
                .filter(|(f, _)| *f == face)
                .map(|(_, b)| b.as_slice())
                .collect();
            let replaced = |cell: &Cell<T>| {
                others
                    .iter()
                    .filter_map(|b| b.iter().map(|b| b.high(cell)).reduce(|m, g| m.max(g)))
                    .map(|h| -h)
                    .reduce(|m, g| m.max(g))
            };
            let replaced_in_sheet = |rect: Rect<T>| {
                reach.sheet?;
                others
                    .iter()
                    .filter_map(|b| {
                        b.iter()
                            .map(|b| {
                                let mine = same_frame(b.frame()?, reach.sheet?);
                                if mine { b.high_in_sheet(rect) } else { None }
                            })
                            .collect::<Option<Vec<T>>>()?
                            .into_iter()
                            .reduce(|m, g| m.max(g))
                    })
                    .map(|h| -h)
                    .reduce(|m, g| m.max(g))
            };
            let bounds = clipped(fbox);
            let bound = metered(
                body,
                Some(face),
                reach,
                &surface,
                bounds,
                &replaced,
                &replaced_in_sheet,
                band,
            );
            let sign = classify_bound(bound)?;
            if sign == Sign::Positive {
                continue;
            }
            // Not clear: read against each reach whose bounds are its band's
            // material exactly, unless the face is a chain's support (whose
            // boundary the request may itself be carving away) or one that
            // reach skips. CROSSING, a measurement, where a point of the
            // face's boundary is certainly in that material and another
            // certainly out of it; WHOLLY INSIDE where every point of the
            // face is certainly in it; otherwise a bound.
            let supports_a_band = reaches
                .iter()
                .any(|r| r.replaces.iter().any(|(f, _)| *f == face));
            let exacts: Vec<&Reach<T>> = if supports_a_band {
                Vec::new()
            } else {
                reaches
                    .iter()
                    .filter(|r| !r.core.is_empty() && r.skip.binary_search(&face).is_err())
                    .collect()
            };
            for exact in &exacts {
                let Some((depth, out)) = boundary_reading(body, face, &exact.core)? else {
                    continue;
                };
                if classify_bound(depth)? == Sign::Negative && sure(out, band) {
                    return Err(BlendError::FaceClearance {
                        at: EntityId::Face(face),
                        chain: chains[exact.chain].first().convexity,
                        margin: classified(decision, depth, band, Sign::Negative),
                        bounded: false,
                    });
                }
            }
            let inside = exacts.iter().find_map(|exact| {
                inside_bound(&exact.core, &surface, fbox, band).map(|deep| (exact, deep))
            });
            if let Some((exact, deep)) = inside {
                let sign = classify_bound(deep)?;
                let refusal = BlendError::WhollyInside {
                    at: EntityId::Face(face),
                    chain: chains[exact.chain].first().convexity,
                    margin: classified(decision, deep, band, sign),
                };
                // A straight band's exact core lies in its prism.
                match exact.spine {
                    Some((crease, _)) => unclear.prism_face(face, crease, deep, refusal),
                    None => unclear.unconsumed_inside(deep, refusal),
                }
                continue;
            }
            // Inside the prism a straight band removes or fills, read through
            // the cycles its caps' meter read inside the sliver, or through
            // those the strip meter read inside one support's strip.
            if let Some((edge, tau)) = reach.spine.filter(|_| !supports_a_band) {
                let mine = |on: &dyn Fn(RingOn) -> bool| -> Vec<TakenRing<T>> {
                    unclear
                        .taken()
                        .iter()
                        .filter(|r| r.crease == edge && on(r.on))
                        .copied()
                        .collect()
                };
                let caps = mine(&|on| matches!(on, RingOn::Cap { .. }));
                // The prism's slice in a cap's plane is an affine image
                // of `S⊥` (projection along `τ`), so `S⊥` is convex
                // exactly when that slice is; a ring read through the
                // exact triangle certifies it for its cap.
                let convex = caps.iter().all(|r| r.on == RingOn::Cap { convex: true });
                let depth_in = |core: &[Bound<T>], c: &Curve3<T>, span| {
                    let speed = geom::carrier_speed::speed_range(c).1.get();
                    curve_in_core(speed, &|t| c.eval(t), span, core, band)
                };
                let in_core = |c: &Curve3<T>, span| depth_in(&reach.core, c, span);
                let by_boundary = match &surface {
                    Surface::Plane { normal, .. } => sure(normal.normalize().dot(tau).abs(), band),
                    _ => false,
                };
                let mut readings = vec![prism_inside(
                    body,
                    face,
                    &surface,
                    fbox,
                    &Prism {
                        tau,
                        rings: &caps,
                        convex,
                        in_core: &in_core,
                        strip: None,
                    },
                    by_boundary,
                )?];
                for strip in &reach.strips {
                    let rings = mine(&|on| on == RingOn::Strip);
                    let rings: Vec<TakenRing<T>> = rings
                        .into_iter()
                        .filter(|r| r.face == strip.support)
                        .collect();
                    let in_beside = |c: &Curve3<T>, span| depth_in(&strip.bounds, c, span);
                    let corner = |v: [f64; 3]| {
                        let [x, y, z] = v.map(T::from_computed);
                        Point3::new(x, y, z)
                    };
                    let (lo, hi) = (corner(fbox.0), corner(fbox.1));
                    let nu = strip
                        .along
                        .as_ref()
                        .filter(|h| sure(T::zero() - corner_max(lo, hi, |x| h.at(x)), band))
                        .map(|_| strip.normal);
                    readings.push(prism_inside(
                        body,
                        face,
                        &surface,
                        fbox,
                        &Prism {
                            tau,
                            rings: &rings,
                            convex: strip.convex,
                            in_core: &in_core,
                            strip: Some((&in_beside, nu)),
                        },
                        by_boundary,
                    )?);
                }
                let deepest = readings
                    .into_iter()
                    .flatten()
                    .filter(|x| sure(*x, band))
                    .reduce(|m, x| if x.hi() > m.hi() { x } else { m });
                if let Some(excess) = deepest {
                    let deep = T::zero() - excess;
                    let sign = classify_bound(deep)?;
                    unclear.prism_face(
                        face,
                        edge,
                        deep,
                        BlendError::WhollyInside {
                            at: EntityId::Face(face),
                            chain: chains[reach.chain].first().convexity,
                            margin: classified(decision, deep, band, sign),
                        },
                    );
                    continue;
                }
            }
            unclear.bound(
                bound,
                BlendError::FaceClearance {
                    at: EntityId::Face(face),
                    chain: chains[reach.chain].first().convexity,
                    margin: classified(decision, bound, band, sign),
                    bounded: true,
                },
            );
        }
        // The bands of the request's other chains: each mints its blend
        // surface over its own reach, a face this reach must clear too —
        // except a concave band's surface inside a convex band's reach.
        // There the convex band removes material the concave one's region
        // already overlapped: the concave surface is the boundary of what
        // it adds, and what the convex band leaves is metered against the
        // concave reach (its faces' surviving parts above, its own band
        // surface here, the other way round).
        let removes = chains[reach.chain].first().convexity == Convexity::Convex;
        let apart = |r: &Reach<T>| {
            r.chains
                .iter()
                .all(|x| reach.chains.iter().all(|y| x != y && !meet(*x, *y)))
        };
        for other in reaches.iter().filter(|r| {
            apart(r) && !(removes && chains[r.chain].first().convexity == Convexity::Concave)
        }) {
            let olo = [other.lo.x.lo(), other.lo.y.lo(), other.lo.z.lo()];
            let ohi = [other.hi.x.hi(), other.hi.y.hi(), other.hi.z.hi()];
            if (0..3).any(|k| ohi[k] < rlo[k] || rhi[k] < olo[k]) {
                continue;
            }
            let banded = |role: &Role| *role != Role::Band;
            let confining: Vec<&Bound<T>> = match &other.confine {
                Some(c) => c.iter().collect(),
                None => other
                    .bounds
                    .iter()
                    .filter(|(role, _)| banded(role))
                    .map(|(_, b)| b)
                    .collect(),
            };
            let off_patch = |cell: &Cell<T>| {
                confining
                    .iter()
                    .map(|b| b.low(cell))
                    .reduce(|m, g| m.max(g))
            };
            let off_patch_in_sheet = |rect: Rect<T>| {
                if !same_sheet(other, reach) {
                    return None;
                }
                confining
                    .iter()
                    .map(|b| b.low_in_sheet(rect))
                    .collect::<Option<Vec<T>>>()?
                    .into_iter()
                    .reduce(|m, g| m.max(g))
            };
            let bound = metered(
                body,
                None,
                reach,
                &other.surface,
                clipped((olo, ohi)),
                &off_patch,
                &off_patch_in_sheet,
                band,
            );
            let sign = classify_bound(bound)?;
            if sign != Sign::Positive {
                // The other band has no face yet: name its chain.
                unclear.bound(
                    bound,
                    BlendError::FaceClearance {
                        at: EntityId::Edge(chains[other.chain].first().edge),
                        chain: chains[reach.chain].first().convexity,
                        margin: classified(decision, bound, band, sign),
                        bounded: true,
                    },
                );
            }
        }
    }
    unclear.verdict()
}

/// Whether two reaches are read in one and the same meridian sheet, so
/// that the bounds of one are functions of the other's sheet.
fn same_sheet<T: Bounds>(a: &Reach<T>, b: &Reach<T>) -> bool {
    matches!((a.sheet, b.sheet), (Some(x), Some(y)) if same_frame(x, y))
}

/// Whether two sheet frames are one and the same, bit for bit.
fn same_frame<T: Bounds>(x: (Point3<T>, Vec3<T>), y: (Point3<T>, Vec3<T>)) -> bool {
    let bits = |(o, a): (Point3<T>, Vec3<T>)| {
        [o.x, o.y, o.z, a.x, a.y, a.z].map(|v| (v.lo().to_bits(), v.hi().to_bits()))
    };
    bits(x) == bits(y)
}

/// **One surface against one reach**, in the meridian sheet and in
/// space, each a lower bound. A surface whose sheet slack is below the
/// reach's width is near coaxial, so the sheet is read first; any other
/// is read in space first, where it clears in a few cells while the
/// sheet, its slack wider than the band, would spend the whole budget.
/// The second path runs only where the first does not certify. `extra`
/// and `extra_in_sheet` are further clearing reads (a lower bound that,
/// positive, says the cell holds no point to meter).
#[allow(clippy::too_many_arguments)]
fn metered<T: Decide + Bounds>(
    body: &Body<T>,
    face: Option<FaceKey>,
    reach: &Reach<T>,
    surface: &Surface<T>,
    bounds: ([f64; 3], [f64; 3]),
    extra: &dyn Fn(&Cell<T>) -> Option<T>,
    extra_in_sheet: &dyn Fn(Rect<T>) -> Option<T>,
    band: Band,
) -> T {
    let sheet = || face_bound_in_sheet(body, face, reach, surface, bounds, extra_in_sheet, band);
    let space = || face_bound(reach, surface, bounds, extra, band);
    let certified = |m: T| m.lo() >= band.escalate();
    let coaxial = reach
        .sheet
        .and_then(|(o, a)| sheet_fn(surface, o, a, (reach.lo, reach.hi)))
        .is_some_and(|(_, slack)| slack.hi() < reach.width.lo());
    if coaxial {
        match sheet() {
            Some(m) if certified(m) => m,
            Some(m) => m.max(space()),
            None => space(),
        }
    } else {
        let m = space();
        if certified(m) {
            return m;
        }
        sheet().map_or(m, |s| m.max(s))
    }
}

/// The test-support door to [`band_reach`] alone, for a fillet request:
/// its links resolved, walked into chains and broken at their turns as
/// the battery builds the verdict's chains, and no other predicate run
/// but the carve's plan of what the verdict consumes
/// (`consume::plan`) — so a row can pin the reach on a body some earlier
/// predicate or door refuses, over the chains production meters.
/// Compiled into no shipped build.
///
/// # Errors
///
/// [`band_reach`]'s and the plan's, and whatever resolving a link or
/// classifying a junction refuses.
#[cfg(any(test, feature = "test-support"))]
pub fn band_reach_for_tests<T: Decide + Bounds>(
    req: &super::BlendRequest<'_, T>,
    band: Band,
) -> Result<(), BlendError> {
    let mut links = Vec::with_capacity(req.edges.len());
    for edge in &req.edges {
        links.push(super::battery::resolve_link(
            req.body,
            *edge,
            req.size,
            band,
            BlendKind::Fillet,
        )?);
    }
    let (chains, _) =
        super::battery::broken_at_turns(req.body, super::battery::walk_chains(links), band)?;
    band_reach(
        req.body,
        &chains,
        req.size,
        BlendKind::Fillet,
        band,
        Unclear::new(),
    )?
    .map_or(Ok(()), |consumed| {
        super::consume::plan(req.body, &consumed).map(|_| ())
    })
}

#[cfg(test)]
mod tests {
    use geom_core::sym::revalue::revalue;
    use geom_core::sym::with_session;
    use geom_core::{ParamSymbol, Sym, SymBudget};

    use geom_core::{Point3, Vec3};

    use super::{Bound, Cell, SheetFn, Unclear};

    /// **A tie on the margin names the least entity, in every read
    /// order**: two faces read inside at one depth, and two read as
    /// bounds at one margin, refuse naming the lesser key whichever is
    /// read first.
    #[test]
    #[allow(clippy::expect_used, clippy::panic)]
    fn a_margin_tie_names_the_least_entity_in_either_order() {
        use super::super::{BlendDecision, BlendError, Convexity, battery::classified};
        use geom_core::{Band, Sign, Tol};
        use topo::{EntityId, FaceKey};
        let band = Band::linear(Tol::witness()).expect("the witness band");
        let body = crate::test_support::cube::<f64>(1.0, Tol::witness());
        let mut faces: Vec<FaceKey> = body.faces().map(|(k, _)| k).collect();
        faces.sort_unstable();
        let (lo, hi) = (faces[0], faces[1]);
        let at = |e: &BlendError| match e {
            BlendError::WhollyInside { at, .. } | BlendError::FaceClearance { at, .. } => *at,
            other => panic!("not a reach refusal: {other:?}"),
        };
        let decision = BlendDecision::FaceClearance;
        let inside = |f: FaceKey| BlendError::WhollyInside {
            at: EntityId::Face(f),
            chain: Convexity::Convex,
            margin: classified(decision, -0.25, band, Sign::Negative),
        };
        let bound = |f: FaceKey| BlendError::FaceClearance {
            at: EntityId::Face(f),
            chain: Convexity::Convex,
            margin: classified(decision, -0.25, band, Sign::Negative),
            bounded: true,
        };
        for order in [[lo, hi], [hi, lo]] {
            let mut u = Unclear::<f64>::new();
            for f in order {
                u.unconsumed_inside(-0.25, inside(f));
            }
            let Err(e) = u.verdict() else {
                panic!("two inside readings the carve does not take refuse")
            };
            assert_eq!(at(&e), EntityId::Face(lo), "inside, read {order:?}");
            let mut u = Unclear::<f64>::new();
            for f in order {
                u.bound(-0.25, bound(f));
            }
            let Err(e) = u.verdict() else {
                panic!("two bounds refuse")
            };
            assert_eq!(at(&e), EntityId::Face(lo), "bound, read {order:?}");
        }
    }

    /// **A cell's corners are computed values**: box arithmetic over the
    /// faces' certified boxes produced them in `f64`, so at `Sym` each
    /// is an unknown, and what the cell derives from them (its centre,
    /// its lever) re-values to nothing rather than to the build's own
    /// numbers.
    #[test]
    fn a_cells_corners_enter_as_computed_values() {
        let budget = SymBudget {
            max_terms: 64,
            max_degree: 8,
        };
        let none = |_: ParamSymbol| None::<f64>;
        let (revalued, _) = with_session(budget, || {
            let cell = Cell::<Sym<f64>>::new([0.0, -1.0, 0.5], [1.0, 2.0, 3.5]);
            [cell.lo.x, cell.hi.z, cell.c.y, cell.lever].map(|v| revalue(v.node(), &none))
        });
        assert_eq!(
            revalued, [None; 4],
            "a corner, the centre and the lever are unknowns, not the constants the box read"
        );
    }

    /// The rod `ρ ≤ ½` about `z` over `z ∈ [0, 1]`, a wall face of it,
    /// and that face's stored cylinder.
    #[allow(clippy::expect_used)]
    fn rod<T: topo::AtRestPolicy>() -> (topo::Body<T>, topo::FaceKey, geom::Surface<T>) {
        use geom_core::{Point2, Tol};
        let half = T::from_f64(0.5);
        let body = crate::test_support::prism_at(
            vec![
                (Point2::new(-half, T::zero()), T::one()),
                (Point2::new(half, T::zero()), T::one()),
            ],
            T::zero(),
            T::one(),
            Tol::witness(),
        );
        let mut faces: Vec<topo::FaceKey> = body.faces().map(|(k, _)| k).collect();
        faces.sort_unstable();
        let (wall, stored) = faces
            .into_iter()
            .find_map(|f| {
                let s = body.get_surface(body.get_face(f)?.surface)?;
                matches!(s, geom::Surface::Cylinder { .. }).then(|| (f, s.clone()))
            })
            .expect("the rod's wall");
        (body, wall, stored)
    }

    /// The unit axis `cos t · z + sin t · x` through the origin.
    fn leaning<T: geom_core::Real>(t: f64) -> (Point3<T>, Vec3<T>) {
        (
            Point3::new(T::zero(), T::zero(), T::zero()),
            Vec3::new(T::from_f64(t.sin()), T::zero(), T::from_f64(t.cos())),
        )
    }

    /// The witness band.
    #[allow(clippy::expect_used)]
    fn band() -> geom_core::Band {
        geom_core::Band::linear(geom_core::Tol::witness()).expect("the witness band")
    }

    /// The rod's wall stored with its origin `along` metres up its axis.
    #[allow(clippy::expect_used, clippy::panic)]
    fn stored_at<T: geom_core::Real>(s: &geom::Surface<T>, along: f64) -> geom::Surface<T> {
        let geom::Surface::Cylinder {
            origin,
            axis,
            radius,
            u_ref,
        } = s.clone()
        else {
            panic!("the rod's wall is a cylinder")
        };
        geom::Surface::Cylinder {
            origin: origin + axis * T::from_f64(along),
            axis,
            radius,
            u_ref,
        }
    }

    /// **A cylinder's clip does not move with where its origin is
    /// stored**: the rod's wall read about an axis leaning half the zero
    /// band off its own, with its origin stored at the base and up to
    /// 1e5 m along, clips to its boundary's span at every storage, at
    /// `f64` and at `Interval`. Stored 3 m along, the origin stood
    /// `1.5·zero` off the axis and the face's clip was lost.
    #[test]
    #[allow(clippy::panic)]
    fn the_clip_reads_one_span_at_every_stored_origin() {
        use super::{Clip, sheet_clip};
        use geom_core::Bounds;
        let band = band();
        let storages = [0.0, -3.0, 1.0e3, 1.0e5];
        macro_rules! spans {
            ($t:ty) => {{
                let (body, wall, stored) = rod::<$t>();
                let (o, a) = leaning::<$t>(0.5 * band.zero());
                storages.map(|along| {
                    match sheet_clip(&body, wall, &stored_at(&stored, along), o, a, band) {
                        Some(Clip::Z(z0, z1)) => (z0.lo(), z1.hi()),
                        Some(Clip::Rho(..)) => {
                            panic!("stored {along} m along: an axial clip, not a radial one")
                        }
                        None => panic!("stored {along} m along: the wall clips"),
                    }
                })
            }};
        }
        for (scalar, spans) in [
            ("f64", spans!(f64)),
            ("Interval", spans!(geom_core::interval::Interval)),
        ] {
            for (along, span) in storages.into_iter().zip(spans) {
                assert_eq!(
                    span, spans[0],
                    "{scalar}, stored {along} m along: the base storage's span"
                );
            }
            let (z0, z1) = spans[0];
            assert!(
                z0 <= 0.0 && z1 >= 1.0 && z1 - z0 < 1.0 + 4.0 * band.escalate(),
                "{scalar}: the span holds the wall's [0, 1] and little else: {z0}..{z1}"
            );
        }
    }

    /// A profile loop as an `(x, y, bulge)` vertex chain.
    type Chain = Vec<(f64, f64, f64)>;

    /// The clip corpus: each fixture a list of profile loops (outer
    /// first) as `(x, y, bulge)` chains, extruded over `z ∈ [0, 1]` —
    /// a full rod, a half-rod, a 270° and a 90° circular segment (walls
    /// that do and do not wrap past a tilted axis's extremes), a disc
    /// with an off-centre bore (a holed cap, an off-axis bore wall), and
    /// a square (planar walls).
    fn clip_fixtures() -> Vec<(&'static str, Vec<Chain>)> {
        let q = core::f64::consts::FRAC_PI_4;
        let big = (3.0 * core::f64::consts::PI / 8.0).tan();
        let (c, s) = (0.3f64.cos(), 0.3f64.sin());
        vec![
            ("rod", vec![vec![(-0.5, 0.0, 1.0), (0.5, 0.0, 1.0)]]),
            ("half-rod", vec![vec![(-0.5, 0.0, 1.0), (0.5, 0.0, 0.0)]]),
            (
                "major-segment",
                vec![vec![
                    (0.5 * q.cos(), 0.5 * q.sin(), big),
                    (0.5 * q.cos(), -0.5 * q.sin(), 0.0),
                ]],
            ),
            (
                "minor-segment",
                vec![vec![
                    (0.5 * q.cos(), -0.5 * q.sin(), 1.0 / big),
                    (0.5 * q.cos(), 0.5 * q.sin(), 0.0),
                ]],
            ),
            (
                "bored",
                vec![
                    vec![(c, s, 1.0), (-c, -s, 1.0)],
                    vec![(0.6, 0.0, -1.0), (0.2, 0.0, -1.0)],
                ],
            ),
            (
                "square",
                vec![vec![
                    (-0.5, -0.5, 0.0),
                    (0.5, -0.5, 0.0),
                    (0.5, 0.5, 0.0),
                    (-0.5, 0.5, 0.0),
                ]],
            ),
        ]
    }

    fn extrude<T: topo::AtRestPolicy>(loops: &[Chain]) -> topo::Body<T> {
        use geom_core::Point2;
        let loops = loops
            .iter()
            .map(|l| {
                profile::test_support::bulge_loop(
                    l.iter()
                        .map(|&(x, y, b)| {
                            (Point2::new(T::from_f64(x), T::from_f64(y)), T::from_f64(b))
                        })
                        .collect(),
                )
            })
            .collect();
        crate::test_support::extruded(
            profile::SketchPlane::xy(),
            loops,
            T::one(),
            geom_core::Tol::witness(),
        )
    }

    /// Ordered samples of each boundary loop of `face`.
    #[allow(clippy::expect_used)]
    fn boundary_samples(body: &topo::Body<f64>, face: topo::FaceKey) -> Vec<Vec<Point3<f64>>> {
        let f = body.get_face(face).expect("the face");
        let mut out = Vec::new();
        for lp in core::iter::once(f.outer).chain(f.rings.iter().copied()) {
            let topo::LoopBoundary::Cycle { first } = body.get_loop(lp).expect("its loop").boundary
            else {
                continue;
            };
            let mut pts = Vec::new();
            for he in body.loop_cycle(first).expect("its cycle") {
                let h = body.get_half_edge(he).expect("a half-edge");
                let e = body.get_edge(h.edge).expect("its edge");
                let cert = body
                    .get_curve_geom(e.curve)
                    .and_then(|g| g.certified())
                    .expect("a certified curve");
                let (t0, t1) = cert.params();
                for k in 0..48 {
                    let s = f64::from(k) / 48.0;
                    let s = if e.he_plus == he { s } else { 1.0 - s };
                    pts.push(cert.carrier().eval(t0 + (t1 - t0) * s));
                }
            }
            out.push(pts);
        }
        out
    }

    /// Points of a face of a z-extrusion: a wall's boundary points
    /// spread over `z ∈ [0, 1]`; a cap's grid points the boundary winds
    /// about, clear of it by 0.01.
    fn face_samples(
        body: &topo::Body<f64>,
        face: topo::FaceKey,
        s: &geom::Surface<f64>,
    ) -> Vec<Point3<f64>> {
        let loops = boundary_samples(body, face);
        let all: Vec<Point3<f64>> = loops.iter().flatten().copied().collect();
        let mut out = all.clone();
        match s {
            geom::Surface::Cylinder { .. } => {
                for p in &all {
                    out.extend((0..=4).map(|j| Point3::new(p.x, p.y, f64::from(j) / 4.0)));
                }
            }
            geom::Surface::Plane { origin, .. } => {
                let pi = core::f64::consts::PI;
                for i in 0..=40 {
                    for j in 0..=40 {
                        let (x, y) = (-1.0 + f64::from(i) / 20.0, -1.0 + f64::from(j) / 20.0);
                        let clear = all.iter().all(|p| (p.x - x).hypot(p.y - y) > 0.01);
                        let mut w = 0.0;
                        for l in &loops {
                            for k in 0..l.len() {
                                let (p, q) = (l[k], l[(k + 1) % l.len()]);
                                let d = (q.y - y).atan2(q.x - x) - (p.y - y).atan2(p.x - x);
                                w += (d + pi).rem_euclid(2.0 * pi) - pi;
                            }
                        }
                        if clear && w.abs() > 3.0 {
                            out.push(Point3::new(x, y, origin.z));
                        }
                    }
                }
            }
            _ => {}
        }
        out
    }

    /// The unit axis at polar angle `t` from `z`, azimuth `phi`.
    fn polar(t: f64, phi: f64) -> Vec3<f64> {
        Vec3::new(t.sin() * phi.cos(), t.sin() * phi.sin(), t.cos())
    }

    /// Leans of the trace's axis off `z`: coaxial, within the zero
    /// band, through ⊥ (exactly, and either side of it within and past
    /// a band), to antiparallel.
    fn clip_leans() -> Vec<f64> {
        let h = core::f64::consts::FRAC_PI_2;
        let pi = core::f64::consts::PI;
        vec![
            0.0,
            0.9 * band().zero(),
            1.0e-3,
            0.3,
            1.2,
            1.5,
            h - 1.0e-3,
            h - 1.0e-6,
            h - 3.0 * band().escalate(),
            h,
            h + 1.0e-4,
            2.0,
            3.0,
            pi - 1.0e-9,
            pi,
        ]
    }

    /// **A clip holds its face at every pose**: on every cylinder and
    /// plane face of the corpus, about axes at every lean, three
    /// azimuths and seven origins (through a cap, near a rim, inside the
    /// bore, outside the body, far below, at a cap corner), every
    /// sampled face point lies in the clip `sheet_clip` reads, at `f64`
    /// and at `Interval`. A wall ⊥ the axis is clipped too: `z` is
    /// constant along its rulings.
    #[allow(clippy::expect_used)]
    #[test]
    fn a_clip_holds_its_face_at_every_pose() {
        use super::{Clip, sheet_clip};
        use geom_core::interval::Interval;
        use geom_core::{Bounds, Real};
        let band = band();
        let origins = [
            Point3::new(0.0, 0.0, 0.0),
            Point3::new(0.1, -0.2, 0.3),
            Point3::new(0.45, 0.0, 0.5),
            Point3::new(0.4, 0.0, 0.5),
            Point3::new(2.0, 1.0, 0.0),
            Point3::new(0.0, 0.0, -7.0),
            Point3::new(0.49, 0.49, 1.0),
        ];
        macro_rules! read {
            ($c:expr) => {
                $c.map(|c| match c {
                    Clip::Z(lo, hi) => ("Z", lo.lo(), hi.hi()),
                    Clip::Rho(lo, hi) => ("Rho", lo.lo(), hi.hi()),
                })
            };
        }
        #[allow(clippy::expect_used)]
        fn surface<T: geom_core::Real>(b: &topo::Body<T>, k: topo::FaceKey) -> geom::Surface<T> {
            b.get_face(k)
                .and_then(|face| b.get_surface(face.surface))
                .cloned()
                .expect("the face's surface")
        }
        let iv = |x: f64| Interval::from_f64(x);
        let (mut clipped, mut perpendicular_walls, mut bad) = (0usize, 0usize, Vec::new());
        for (name, loops) in clip_fixtures() {
            let body = extrude::<f64>(&loops);
            let ibody = extrude::<Interval>(&loops);
            let mut faces: Vec<_> = body.faces().map(|(k, _)| k).collect();
            faces.sort_unstable();
            let mut ifaces: Vec<_> = ibody.faces().map(|(k, _)| k).collect();
            ifaces.sort_unstable();
            assert_eq!(faces.len(), ifaces.len(), "{name}: the twin bodies' faces");
            for (&f, &fi) in faces.iter().zip(&ifaces) {
                let (s, si) = (surface(&body, f), surface(&ibody, fi));
                if !matches!(
                    s,
                    geom::Surface::Cylinder { .. } | geom::Surface::Plane { .. }
                ) {
                    continue;
                }
                let pts = face_samples(&body, f, &s);
                assert!(pts.len() > 50, "{name} {f:?}: the face's samples");
                for t in clip_leans() {
                    for phi in [0.0, 0.7, 2.0] {
                        let a = polar(t, phi);
                        let ai = Vec3::new(iv(a.x), iv(a.y), iv(a.z));
                        for o in origins {
                            let oi = Point3::new(iv(o.x), iv(o.y), iv(o.z));
                            for (scalar, clip) in [
                                ("f64", read!(sheet_clip(&body, f, &s, o, a, band))),
                                ("Interval", read!(sheet_clip(&ibody, fi, &si, oi, ai, band))),
                            ] {
                                let Some((kind, lo, hi)) = clip else {
                                    continue;
                                };
                                clipped += 1;
                                let wall = matches!(s, geom::Surface::Cylinder { .. });
                                perpendicular_walls +=
                                    usize::from(wall && t == core::f64::consts::FRAC_PI_2);
                                let worst = pts
                                    .iter()
                                    .map(|p| {
                                        let v = *p - o;
                                        let z = v.dot(a);
                                        let x = if kind == "Z" { z } else { (v - a * z).norm() };
                                        (lo - x).max(x - hi)
                                    })
                                    .fold(f64::NEG_INFINITY, f64::max);
                                if worst > 0.0 {
                                    bad.push(format!(
                                        "{name} {f:?} {scalar} lean {t} phi {phi} o {o:?}: \
                                         its {kind} clip {lo}..{hi} misses a face point by {worst}"
                                    ));
                                }
                            }
                        }
                    }
                }
            }
        }
        assert!(clipped > 5000, "the corpus reads clips: {clipped}");
        assert!(
            perpendicular_walls > 0,
            "walls ⊥ the axis are clipped: {perpendicular_walls}"
        );
        assert!(
            bad.is_empty(),
            "{} clips miss their face:\n{}",
            bad.len(),
            bad[..bad.len().min(40)].join("\n")
        );
    }

    /// **A sheet slack does not move with where a plane's or a
    /// cylinder's origin is stored**: about an axis leaning half the zero
    /// band, over a box of half-width 2 about the origin, the rod's wall
    /// stored up to 1e5 m along its axis, and a plane `z = 1` stored up to
    /// 1e5 m across itself, each read one slack within rounding, a few
    /// zero bands and not none. Read at the stored origin, the far
    /// storage's slack was about 1e5 zero bands.
    #[test]
    #[allow(clippy::panic)]
    fn a_sheet_slack_reads_one_value_at_every_stored_origin() {
        use super::sheet_fn;
        let band = band();
        let (_, _, wall) = rod::<f64>();
        let (o, a) = leaning::<f64>(0.5 * band.zero());
        let reach = (Point3::new(-2.0, -2.0, -2.0), Point3::new(2.0, 2.0, 2.0));
        let plane_at = |x: f64| geom::Surface::Plane {
            origin: Point3::new(x, 0.0, 1.0),
            normal: Vec3::new(0.0, 0.0, 1.0),
            u_ref: Vec3::new(1.0, 0.0, 0.0),
        };
        for (what, at) in [
            (
                "the wall",
                &(|along: f64| stored_at(&wall, along)) as &dyn Fn(f64) -> geom::Surface<f64>,
            ),
            ("the plane", &plane_at),
        ] {
            let slack = |along: f64| match sheet_fn(&at(along), o, a, reach) {
                Some((_, slack)) => slack,
                None => panic!("{what} stored {along} m along: a sheet form"),
            };
            let base = slack(0.0);
            assert!(
                base > 0.0 && base < 10.0 * band.zero(),
                "{what}: a slack of a few zero bands, the lean read: {base}"
            );
            for along in [-3.0, 1.0e3, 1.0e5] {
                let s = slack(along);
                assert!(
                    (s - base).abs() <= 0.1 * band.zero(),
                    "{what} stored {along} m along: the base storage's slack {base}, read {s}"
                );
            }
        }
    }

    /// **A sheet slack encloses the distance it stands for**: for a
    /// cylinder and a plane at seven leans of their own, stored at four
    /// places along or across themselves, in three reach boxes (one 1e4 m
    /// out), about axes at every lean, `|d(x) − f(ρ, z)| ≤ slack` at the
    /// box's 5×5×5 grid points.
    #[test]
    fn a_sheet_slack_encloses_the_distance() {
        use super::sheet_fn;
        let boxes = [
            (Point3::new(-2.0, -2.0, -2.0), Point3::new(2.0, 2.0, 2.0)),
            (Point3::new(50.0, -3.0, 40.0), Point3::new(51.0, -2.0, 41.0)),
            (
                Point3::new(-1e4, 0.0, 1e4),
                Point3::new(-1e4 + 0.1, 0.1, 1e4 + 0.1),
            ),
        ];
        let (mut read, mut bad) = (0usize, Vec::new());
        let o = Point3::origin();
        for t in clip_leans() {
            let a = polar(t, 0.4);
            for own in [
                0.0,
                1e-9,
                1e-6,
                1e-3,
                0.5,
                2.0,
                core::f64::consts::PI - 1e-7,
            ] {
                let b = polar(own, 1.3);
                let in_plane = b.cross(Vec3::new(1.0, 0.2, 0.0));
                for along in [0.0, -3.0, 1e3, 1e5] {
                    let cylinder = geom::Surface::Cylinder {
                        origin: Point3::new(0.3, -0.1, 0.0) + b * along,
                        axis: b * 3.0,
                        radius: 0.5,
                        u_ref: b.cross(Vec3::new(0.3, 1.0, 0.1)).normalize(),
                    };
                    let plane = geom::Surface::Plane {
                        origin: Point3::new(0.0, 0.0, 1.0) + in_plane * along,
                        normal: b,
                        u_ref: in_plane.normalize(),
                    };
                    let distance = |s: &geom::Surface<f64>, x: Point3<f64>| match *s {
                        geom::Surface::Cylinder {
                            origin,
                            axis,
                            radius,
                            ..
                        } => {
                            let (v, n) = (x - origin, axis.normalize());
                            (v - n * v.dot(n)).norm() - radius
                        }
                        geom::Surface::Plane { origin, normal, .. } => {
                            (x - origin).dot(normal.normalize())
                        }
                        _ => f64::NAN,
                    };
                    for s in [cylinder, plane] {
                        for (lo, hi) in boxes {
                            let Some((g, slack)) = sheet_fn(&s, o, a, (lo, hi)) else {
                                continue;
                            };
                            read += 1;
                            // The row's own f64 rounding at these magnitudes.
                            let round = 1e-12 * (1.0 + lo.x.abs() + lo.z.abs() + along.abs());
                            let at = |l: f64, h: f64, m: i32| l + (h - l) * f64::from(m) / 4.0;
                            let mut worst = f64::NEG_INFINITY;
                            for i in 0..=4 {
                                for j in 0..=4 {
                                    for k in 0..=4 {
                                        let x = Point3::new(
                                            at(lo.x, hi.x, i),
                                            at(lo.y, hi.y, j),
                                            at(lo.z, hi.z, k),
                                        );
                                        let v = x - o;
                                        let z = v.dot(a);
                                        let rho = (v - a * z).norm();
                                        let gap = (distance(&s, x) - g.at(rho, z)).abs() - slack;
                                        worst = worst.max(gap - round);
                                    }
                                }
                            }
                            if worst > 0.0 || worst.is_nan() {
                                bad.push(format!(
                                    "lean {t}, own lean {own}, stored {along} along, {:?}, box {lo:?}: \
                                     slack {slack} short by {worst}",
                                    core::mem::discriminant(&s)
                                ));
                            }
                        }
                    }
                }
            }
        }
        assert!(read > 1000, "the row reads slacks: {read}");
        assert!(
            bad.is_empty(),
            "{} slacks short:\n{}",
            bad.len(),
            bad[..bad.len().min(40)].join("\n")
        );
    }

    /// **`tilt` reads the chord to the nearer of `±a`**: `2·sin(θ/2)`, θ
    /// that angle, near 0, ⊥ and π, for a unit, a short and a long `b` —
    /// within 4 ulp at `f64`, enclosed at `Interval`.
    #[test]
    fn tilt_reads_the_chord() {
        use super::tilt;
        use geom_core::interval::Interval;
        use geom_core::{Bounds, Real};
        let pi = core::f64::consts::PI;
        let iv = |v: Vec3<f64>| {
            Vec3::new(
                Interval::from_f64(v.x),
                Interval::from_f64(v.y),
                Interval::from_f64(v.z),
            )
        };
        let a = Vec3::new(0.0, 0.0, 1.0);
        for th in [
            0.0,
            1e-15,
            1e-12,
            1e-9,
            1e-6,
            0.3,
            pi / 2.0 - 1e-9,
            pi / 2.0,
            2.0,
            pi - 1e-6,
            pi - 1e-9,
            pi - 1e-12,
            pi,
        ] {
            for scale in [1.0, 1e-3, 7.0e3] {
                let b = Vec3::new(th.sin(), 0.0, th.cos()) * scale;
                let truth = 2.0 * (th.min(pi - th) / 2.0).sin();
                let f = tilt(b, a);
                assert!(
                    (f - truth).abs() <= 4.0 * f64::EPSILON * (1.0 + truth) + 1e-16,
                    "f64 tilt at θ {th}, ×{scale}: {f}, the chord {truth}"
                );
                // `b` is the rounded input: an outward margin of its rounding.
                let g = tilt(iv(b), iv(a));
                let m = 4.0 * f64::EPSILON;
                assert!(
                    g.lo() <= truth + m && g.hi() >= truth - m,
                    "Interval tilt at θ {th}, ×{scale}: [{}, {}] holds the chord {truth}",
                    g.lo(),
                    g.hi()
                );
            }
        }
    }

    /// **A sheet bound read as the band's own material takes its slack
    /// the other way**: `ρ − 1` about the `z` axis with slack `0.1`
    /// encloses the material `ρ ≤ 0.9` by `ρ ≤ 1.1`. A point at `ρ =
    /// 0.95`, and the cell `ρ ∈ [0.92, 0.95]`, lie in the enclosure but
    /// not certainly in the material: the plain readings are negative,
    /// the strict ones positive.
    #[test]
    fn a_sheet_bounds_strict_reading_takes_its_slack_the_other_way() {
        let sheet = Bound::Sheet {
            o: Point3::new(0.0, 0.0, 0.0),
            a: Vec3::new(0.0, 0.0, 1.0),
            f: SheetFn::Line {
                alpha: 1.0,
                beta: 0.0,
                gamma: -1.0,
            },
            slack: 0.1,
        };
        let x = Point3::new(0.95, 0.0, 0.0);
        let cell = Cell::<f64>::new([0.92, -0.01, -0.01], [0.95, 0.0, 0.01]);
        let (at, strict) = (sheet.at(x), sheet.at_strict(x));
        let (high, high_strict) = (sheet.high(&cell), sheet.high_strict(&cell));
        assert!(
            (at + 0.15).abs() < 1e-12 && (strict - 0.05).abs() < 1e-12,
            "at ρ = 0.95: plain {at}, strict {strict}"
        );
        assert!(
            high < 0.0 && high_strict > 0.0,
            "over the cell: plain {high}, strict {high_strict}"
        );
    }

    /// **A curve is read in a core over every point of each piece, and
    /// the reading tightens**: a unit circle about the origin against
    /// the half-space `x ≤ 1 − h` refuses for `h = 10⁻³`, its point at
    /// the parameter's seam a hair out (a piece's box must reach `σ·w/2`
    /// from its middle, which it does not at half that, nor at none);
    /// against `x ≤ 1.5` it reads within `[0.4, 0.5]`, a lower bound on
    /// the true `0.5` that the first quarter-circles to certify, at
    /// `≈ 0.008`, would leave far below.
    #[test]
    #[allow(clippy::expect_used)]
    fn a_curve_is_read_in_a_core_piecewise_and_tightened() {
        let circle = geom::Curve3::Circle {
            center: Point3::new(0.0, 0.0, 0.0),
            axis: Vec3::new(0.0, 0.0, 1.0),
            radius: 1.0,
            u_ref: Vec3::new(1.0, 0.0, 0.0),
        };
        let core = |d: f64| {
            [Bound::Affine {
                n: Vec3::new(1.0, 0.0, 0.0),
                d,
            }]
        };
        let tau = core::f64::consts::TAU;
        assert_eq!(
            super::curve_in_core(
                1.0,
                &|t| circle.eval(t),
                (0.0, tau),
                &core(1.0 - 1e-3),
                band()
            ),
            None,
            "a circle a hair out of the core at its seam is not certified"
        );
        let deep = super::curve_in_core(1.0, &|t| circle.eval(t), (0.0, tau), &core(1.5), band())
            .expect("a circle half a unit inside the core certifies");
        assert!(
            (0.4..=0.5).contains(&deep),
            "the reading {deep} is a lower bound within a fifth of the true 0.5"
        );
    }
}
