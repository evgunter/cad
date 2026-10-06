//! **Predicate 2's reach** — every band against every face of the body
//! that is not a support of its chain, in any shell (C8's face
//! clearance, read over the whole body rather than over the supports'
//! own boundary features).
//!
//! A band changes the material between its supports and its blend
//! surface: a convex band removes it, a concave one adds it. Call that
//! region the band's REACH. A face the chain does not blend that meets
//! the reach is a face the carve would cut through (convex) or bury
//! (concave), and the surgery has no step for either, so the request
//! refuses before any mutation: definitely ([`BlendError::FaceClearance`]
//! with `bounded: false`) when a point of the face's own boundary lies
//! inside a region that is the band's material exactly, and as
//! uncertified (`bounded: true`) whenever the face could not be
//! certified clear.
//!
//! # The region
//!
//! Each reach is an intersection of closed sets `{g ≤ 0}`, every `g`
//! 1-Lipschitz in space (a signed distance, or a 1-Lipschitz function of
//! the meridian-sheet coordinates `(ρ, z)`, themselves a 1-Lipschitz map).
//! In a link's normal sheet — the plane ⊥ a straight spine, or the
//! meridian half-plane of a circular one — the band's cross-section is
//! the curvilinear triangle between the edge point `P`, the two feet and
//! the ball's arc. It is bounded by:
//!
//! - each support, on the ball's side (the support's signed distance);
//! - the two rays from the ball centre `C` through the feet, on `P`'s
//!   side (the sector the arc subtends);
//! - the ball, outside; and the disk of radius `|P − C|` about `C`,
//!   inside.
//!
//! A chamfer's cross-section is the triangle between `P` and its feet:
//! the two supports and the strip's plane. Over plane supports these
//! bounds ARE the cross-section, which is what lets a point inside them
//! be read as interference rather than as a bound; over a curved
//! support's circular trace they enclose it.
//!
//! A straight link's reach runs along the spine over the edge's window.
//! At a cut-off, a mitre or a cap the band ends in the plane of the face
//! it runs into, which bounds it, and the window widens by the run of the
//! cross-section along that plane; at a corner patch it ends where the
//! patch begins. A corner patch at a trivalent vertex all three of
//! whose edges are requested is a reach of its own: the three supports
//! on the ball's side, the three planes through the ball's centre normal
//! to the three edges on the vertex's side, and outside the ball (the
//! chamfer's: the three supports and the patch plane). A circular
//! spine's reach is taken over the whole turn, and every bound of it is a
//! function of its own sheet: a support whose axis departs from the
//! band's by a tilt or an offset is read about the band's axis and the
//! departure is added as slack, so the reach stays a solid of revolution
//! that encloses the band.
//!
//! # What is metered, and what is not
//!
//! A face is metered against a chain's reaches unless it is a support of
//! that chain, meets one of the chain's vertices, or lies on the stored
//! surface of a support bounding that reach. A face at a chain end is the
//! face the band runs into, which predicate 6 and the surgery's own
//! meters judge; a face on a support's surface lies on that support's
//! side of the reach, which is the reach's boundary and never its inside.
//!
//! **Two bands of one request.** A support of another chain of the
//! request is replaced, between that chain's edge and its trimline, by
//! that chain's band, so it is metered by what survives: a cell that lies
//! whole in the part that band replaces is clear (`Reach::replaces`). The
//! other band's new surface is metered against the reach too, as a face
//! over that band's own reach — except a concave band's surface against a
//! convex band's reach: the convex band removes material that region
//! overlapped, and what it leaves is metered against the concave reach.
//! Two chains that meet at a vertex meet in a corner or a turn, which the
//! corner predicate and the surgery judge, so their bands are not metered
//! against each other.
//!
//! The faces a reach can touch are pruned by their certified boxes
//! ([`topo::FaceBoxes`]) against the reach's own box. Each survivor is
//! metered on cells. A face on a surface of revolution about a circular
//! reach's axis is metered in the reach's meridian sheet: the reach and
//! the surface are both solids of revolution there, so a point of one in
//! the other would rotate into the sheet; its extent along its trace is
//! read off its own boundary where that boundary says it
//! ([`sheet_clip`]), since its box may be its whole surface's (a
//! sphere's always is). Every other face is metered on
//! its box clipped to the reach's, in space. A cell is clear when a lower
//! bound over it of either the face's own surface distance (no point of
//! the face lies in it) or one of the reach's bounds (no point of it lies
//! in the reach) is positive; a cell neither clears is halved until it
//! does, or it is as small as the band, or the face's budget is spent.
//! The face's margin is the least cell bound. That margin is a BOUND: a
//! cell of the face's surface may lie outside the face's trim, so a face
//! that is clear can be refused uncertified, never the reverse.

use std::collections::VecDeque;

use geom::{Curve3, Surface};
use geom_core::{Band, Bounds, Decide, Margin, Point3, Real, Sign, Vec3};
use topo::{Body, EdgeKey, EntityId, FaceBoxes, FaceKey, SurfaceKey, VertexKey};

use super::arms::{chamfer_corner_patch, corner_ball, line_meet};
use super::battery::{Chain, Convexity, Link, carrier_of, classified, outward};
use super::surgery::not_intact;
use super::{BlendDecision, BlendError, BlendKind, BlendSite, classify};

/// How many cells one face may be cut into against one reach before it
/// is refused uncertified — the meter's cost ceiling per pair, which
/// keeps the predicate near-linear in the body.
const CELL_BUDGET: usize = 4096;

/// How many places along each boundary edge of a refused face are read
/// for a point inside the band's material.
const EDGE_SAMPLES: u32 = 9;

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

/// The nearest and farthest distance from the sheet point `(p, q)` to a
/// sheet rectangle.
fn rect_distance<T: Real>([r0, r1, z0, z1]: Rect<T>, p: T, q: T) -> (T, T) {
    let gap = |lo: T, hi: T, c: T| (lo - c).max(c - hi).max(T::zero());
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
    fn new(lo: [f64; 3], hi: [f64; 3]) -> Self {
        let p = |v: [f64; 3]| Point3::new(T::from_f64(v[0]), T::from_f64(v[1]), T::from_f64(v[2]));
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
    /// `g(x)`.
    fn at(&self, x: Point3<T>) -> T {
        match self {
            Self::Affine { n, d } => (x - Point3::origin()).dot(*n) - *d,
            Self::Tube { o, a, r, inside } => {
                let (rho, _) = sheet_of(*o, *a, x);
                if *inside { rho - *r } else { *r - rho }
            }
            Self::OutsideBall { o, r } => *r - (x - *o).norm(),
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
            Self::OutsideBall { o, r } => {
                let gap = |lo: T, hi: T, c: T| (lo - c).max(c - hi).max(T::zero());
                let near = (gap(cell.lo.x, cell.hi.x, o.x).powi(2)
                    + gap(cell.lo.y, cell.hi.y, o.y).powi(2)
                    + gap(cell.lo.z, cell.hi.z, o.z).powi(2))
                .sqrt();
                *r - near
            }
            Self::Sheet { o, a, f, slack } => f.range(sheet_box(*o, *a, cell)).1 - *slack,
            Self::Side { s, w } => match surface_range(s, cell) {
                Some((lo, hi)) => (*w * lo).max(*w * hi),
                None => self.at(cell.c) + cell.lever,
            },
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
/// surface: exact for a plane, a cylinder, a sphere and a torus; for a
/// cone, `ρ·cos α − |z|·sin α`, zero on both nappes. `None` for a
/// surface with no closed form here (NURBS, an approximating surface).
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
/// `None` with no closed form.
fn surface_range<T: Real>(s: &Surface<T>, cell: &Cell<T>) -> Option<(T, T)> {
    Some(match s {
        Surface::Plane { normal, .. } => {
            let f = surface_distance(s, cell.c)?;
            let w = cell.spread(normal.normalize());
            (f - w, f + w)
        }
        Surface::Sphere { center, radius, .. } => {
            let rho = |x: Point3<T>| (x - *center).norm();
            let gap = |lo: T, hi: T, c: T| (lo - c).max(c - hi).max(T::zero());
            let near = (gap(cell.lo.x, cell.hi.x, center.x).powi(2)
                + gap(cell.lo.y, cell.hi.y, center.y).powi(2)
                + gap(cell.lo.z, cell.hi.z, center.z).powi(2))
            .sqrt();
            (near - *radius, corner_max(cell.lo, cell.hi, rho) - *radius)
        }
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
/// within `lever(p)` of the surface's own reference point `p`, `d` the
/// surface's own signed distance. The slack is the surface's departure
/// from a surface of revolution about `(o, a)`: its reference point's
/// distance from the axis, and its own axis's tilt (the chord
/// `|â − ±a|`) levered over that reach. `None` for a surface with no
/// closed form here.
fn sheet_fn<T: Real>(
    s: &Surface<T>,
    o: Point3<T>,
    a: Vec3<T>,
    lever: impl Fn(Point3<T>) -> T,
) -> Option<(SheetFn<T>, T)> {
    let tilt = |b: Vec3<T>| {
        let b = b.normalize();
        (T::from_f64(2.0) - T::from_f64(2.0) * b.dot(a).abs())
            .max(T::zero())
            .sqrt()
    };
    let one = T::one();
    let off_axis = |p: Point3<T>| sheet_of(o, a, p);
    Some(match s {
        Surface::Plane { origin, normal, .. } => {
            let n = normal.normalize();
            let s = n.dot(a);
            let sg = s / s.abs();
            let (_, z) = off_axis(*origin);
            (
                SheetFn::Line {
                    alpha: T::zero(),
                    beta: sg,
                    gamma: -sg * z,
                },
                (n - a * sg).norm() * lever(*origin),
            )
        }
        Surface::Cylinder {
            origin,
            axis,
            radius,
            ..
        } => (
            SheetFn::Line {
                alpha: one,
                beta: T::zero(),
                gamma: -*radius,
            },
            off_axis(*origin).0 + tilt(*axis) * lever(*origin),
        ),
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
    /// replaces: for a link, within the chord from the edge to the
    /// support's trimline of the edge itself, over the edge's own extent —
    /// that is the support's strip, since the battery's support screen
    /// refuses any other boundary feature of the face within the
    /// setbacks; for a corner patch, its bounds but the supports' sides.
    replaces: Vec<(FaceKey, Vec<Bound<T>>)>,
}

/// The outward normal of `face` at `p`, or the body's refusal.
fn normal_at<T: Decide>(
    body: &Body<T>,
    face: FaceKey,
    p: Point3<T>,
) -> Result<Vec3<T>, BlendError> {
    outward(body, face, p)
        .ok_or_else(|| not_intact(EntityId::Face(face), "a face the band reach reads"))
}

fn surface_of<T: Decide>(body: &Body<T>, face: FaceKey) -> Result<Surface<T>, BlendError> {
    body.get_face(face)
        .and_then(|f| body.get_surface(f.surface))
        .cloned()
        .ok_or_else(|| not_intact(EntityId::Face(face), "a face's stored surface"))
}

fn surface_key<T: Decide>(body: &Body<T>, face: FaceKey) -> Result<SurfaceKey, BlendError> {
    body.get_face(face)
        .map(|f| f.surface)
        .ok_or_else(|| not_intact(EntityId::Face(face), "a face's stored surface"))
}

fn point_of<T: Decide>(body: &Body<T>, v: VertexKey) -> Result<Point3<T>, BlendError> {
    super::surgery::point_of(body, v)
        .ok_or_else(|| not_intact(EntityId::Vertex(v), "a chain vertex's point"))
}

fn faces_at<T: Decide>(body: &Body<T>, v: VertexKey) -> Result<Vec<FaceKey>, BlendError> {
    body.faces_of_vertex(v)
        .ok_or_else(|| not_intact(EntityId::Vertex(v), "a chain vertex's face fan"))
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
fn side<T: Real>(s: &Surface<T>, q: Point3<T>) -> Option<Bound<T>> {
    let fq = surface_distance(s, q)?;
    let w = -fq / fq.abs();
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
fn half_space<T: Real>(o: Point3<T>, n: Vec3<T>, inside: Point3<T>) -> Bound<T> {
    let n = n.normalize();
    let k = (inside - o).dot(n);
    let n = n * -(k / k.abs());
    Bound::Affine {
        n,
        d: (o - Point3::origin()).dot(n),
    }
}

/// The faces a chain's reaches do not meter by key: its supports, and
/// every face at one of its vertices.
fn excluded<T: Decide>(body: &Body<T>, chain: &Chain<T>) -> Result<Vec<FaceKey>, BlendError> {
    let mut out = Vec::new();
    for l in chain.links() {
        out.extend([l.face_a, l.face_b]);
        out.extend(faces_at(body, l.start)?);
        out.extend(faces_at(body, l.end)?);
    }
    out.sort_unstable();
    out.dedup();
    Ok(out)
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
    let unread = || not_intact(EntityId::Edge(link.edge), "a link's carrier or trimlines");
    let (carrier, t0, t1) = carrier_of(body, link.edge).ok_or_else(unread)?;
    let p = carrier.eval(geom::mid_param(t0, t1));
    let fa = foot_on(&link.blend.trim_a.0, p).ok_or_else(unread)?;
    let fb = foot_on(&link.blend.trim_b.0, p).ok_or_else(unread)?;
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
    chain: usize,
) -> Result<Reach<T>, BlendError> {
    let start = point_of(body, link.start)?;
    let end = point_of(body, link.end)?;
    let tau = (end - start).normalize();
    let q = st.inside();
    let mut cross = Vec::new();
    let mut exact = true;
    for f in [link.face_a, link.face_b] {
        let s = surface_of(body, f)?;
        exact &= matches!(s, Surface::Plane { .. });
        cross.extend(side(&s, q).map(|b| (Role::Side(f), b)));
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
    // The window along the spine. At a cut-off, a mitre or a cap the
    // band ends in the plane of the face it runs into, which bounds it,
    // and the window widens by the run of the cross-section along that
    // plane; at a corner patch it ends at the patch: the plane through
    // the ball's centre normal to the edge (the chamfer's feet).
    let mut ends = Vec::new();
    let mut core: Vec<Bound<T>> = cross.iter().map(|(_, b)| b.clone()).collect();
    let mut caps = Vec::new();
    for v in [link.start, link.end] {
        let pv = point_of(body, v)?;
        let patch = corners.iter().find(|(cv, _)| *cv == v);
        let mut pad = T::zero();
        for f in faces_at(body, v)? {
            if f == link.face_a || f == link.face_b {
                continue;
            }
            exact &= matches!(surface_of(body, f)?, Surface::Plane { .. });
            let n = normal_at(body, f, pv)?;
            let cap = half_space(pv, n, q);
            core.push(cap.clone());
            if patch.is_none() {
                caps.push((Role::Other, cap));
                pad = pad.max(reach * (n.cross(tau).norm() / n.dot(tau).abs()));
            }
        }
        let sv = (pv - start).dot(tau);
        ends.push(match patch {
            // The band ends at the patch, between its anchors' stations.
            Some((_, anchors)) => {
                let at = |x: &Point3<T>| (*x - start).dot(tau);
                let first = anchors.first().map_or(sv, at);
                let (lo, hi) = anchors
                    .iter()
                    .map(at)
                    .fold((first, first), |(l, h), x| (l.min(x), h.max(x)));
                (sv, lo, hi)
            }
            None => (sv, sv - pad, sv + pad),
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
    let w_lo = ends[0].1.min(ends[1].1);
    let w_hi = ends[0].2.max(ends[1].2);
    let edge_extent = window(ends[0].0.min(ends[1].0), ends[0].0.max(ends[1].0));
    core.extend(edge_extent.iter().map(|(_, b)| b.clone()));
    let mut bounds = cross;
    bounds.extend(caps);
    bounds.extend(window(w_lo, w_hi));
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
        sides: vec![
            surface_key(body, link.face_a)?,
            surface_key(body, link.face_b)?,
        ],
        surface: link.blend.surface.clone(),
        replaces: [(link.face_a, st.fa), (link.face_b, st.fb)]
            .into_iter()
            .map(|(f, foot)| {
                let mut b: Vec<Bound<T>> = edge_extent.iter().map(|(_, b)| b.clone()).collect();
                b.push(Bound::Tube {
                    o: st.p,
                    a: tau,
                    r: (foot - st.p).norm(),
                    inside: true,
                });
                (f, b)
            })
            .collect(),
    })
}

/// **The reach of one link about a circular spine**, over the whole
/// turn, every bound a function of the torus's own meridian sheet.
fn circular_reach<T: Decide + Bounds>(
    body: &Body<T>,
    link: &Link<T>,
    st: &Station<T>,
    chain: usize,
) -> Result<Reach<T>, BlendError> {
    let Surface::Torus {
        center,
        axis,
        major_radius,
        minor_radius,
        ..
    } = link.blend.surface
    else {
        return Err(not_intact(
            EntityId::Edge(link.edge),
            "a circular spine's band is not a torus",
        ));
    };
    let (o, a) = canonical_axis(center, axis);
    let (c, _) =
        st.c.ok_or_else(|| not_intact(EntityId::Edge(link.edge), "a torus band's spine point"))?;
    let reach = (st.p - c).norm();
    // A torus about its own axis: `big + reach` across, `reach` along.
    let ext = |k: T| major_radius * (T::one() - k.powi(2)).max(T::zero()).sqrt() + reach;
    let e = Vec3::new(ext(a.x), ext(a.y), ext(a.z));
    let (lo, hi) = (center - e, center + e);
    let lever = |p: Point3<T>| corner_max(lo, hi, |x| (x - p).norm());
    let sheet = |x: Point3<T>| sheet_of(o, a, x);
    let in_sheet = |f: SheetFn<T>, slack: T| Bound::Sheet { o, a, f, slack };
    let mut bounds = Vec::new();
    // The supports, read about the band's axis.
    let (qr, qz) = sheet(st.inside());
    let mut exact = link.start == link.end;
    for f in [link.face_a, link.face_b] {
        let s = surface_of(body, f)?;
        exact &= matches!(s, Surface::Plane { .. } | Surface::Cylinder { .. });
        if let Some((g, slack)) = sheet_fn(&s, o, a, lever) {
            let at = g.at(qr, qz);
            bounds.push((Role::Side(f), in_sheet(g.scaled(-at / at.abs()), slack)));
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
        sides: vec![
            surface_key(body, link.face_a)?,
            surface_key(body, link.face_b)?,
        ],
        surface: link.blend.surface.clone(),
        replaces: [(link.face_a, st.fa), (link.face_b, st.fb)]
            .into_iter()
            .map(|(f, foot)| {
                let ring = SheetFn::Circle {
                    p: ps.0,
                    q: ps.1,
                    r: (foot - st.p).norm(),
                    sigma: T::one(),
                };
                (f, vec![in_sheet(ring, T::zero())])
            })
            .collect(),
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
    chain: usize,
) -> Result<Option<CornerReach<T>>, BlendError> {
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
    let (q, anchors, surface) = match kind {
        BlendKind::Fillet => {
            let ball = corner_ball([pv; 3], normals, size, links[0].convexity);
            let c = ball.center;
            bounds.push((Role::Band, Bound::OutsideBall { o: c, r: size }));
            // Each band ends in the plane through the centre normal to
            // its edge; the patch lies on the vertex's side of all three.
            for l in links {
                let far = if l.start == v { l.end } else { l.start };
                let tau = point_of(body, far)? - pv;
                bounds.push((Role::Other, half_space(c, tau, pv)));
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
            surface,
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
            let k = (0..3)
                .max_by(|&i, &j| (hi[i] - lo[i]).total_cmp(&(hi[j] - lo[j])))
                .unwrap_or(0);
            let mid = 0.5 * (lo[k] + hi[k]);
            let (mut h1, mut l2) = (hi, lo);
            h1[k] = mid;
            l2[k] = mid;
            queue.push_back((lo, h1));
            queue.push_back((l2, hi));
        }
    }
    least.done(band)
}

/// A coaxial face's image in the sheet, along its own trace: an axial
/// range for a sphere, a cylinder or a cone, a radial one for a plane ⊥
/// the axis.
enum Clip<T: Real> {
    Z(T, T),
    Rho(T, T),
}

/// **The extent of a coaxial face's image along its trace**, read from
/// its boundary (`None` where it cannot be).
///
/// The sheet map restricted to a surface of revolution is open away from
/// the axis, so the image of a connected face is an arc of the trace
/// whose ends are images of its boundary — or of a singular point of the
/// surface on the axis (a sphere's pole, a cone's apex, a plane's axis
/// point) that the face contains. On a line trace or a sphere's
/// half-circle (monotone in `z`) that arc lies in the hull of those
/// points.
///
/// Whether the face contains a singular point is read off its boundary
/// when every boundary edge is a latitude circle coaxial with the axis or
/// a meridian edge, which is what a revolved face is bounded by, both
/// exactly so: the azimuth winding `W` of the boundary (oriented by the
/// face's own outward normal) counts the singular points inside, and on
/// a sphere, where `W = 0` leaves none or both poles, the sign of
/// `∮ (z − z_c) dθ = 2πR·k − A/R` (Archimedes, `A` the face's area, `k`
/// the poles inside) decides. Any other boundary leaves the face's box
/// alone. A torus's trace is a whole circle and is not clipped.
fn sheet_clip<T: Decide + Bounds>(
    body: &Body<T>,
    face: FaceKey,
    surface: &Surface<T>,
    o: Point3<T>,
    a: Vec3<T>,
    band: Band,
) -> Option<Clip<T>> {
    // How far a direction is from the axis (`|b̂ − ±a|`, the chord),
    // and how far a point is from it.
    let tilt = |b: Vec3<T>| {
        let b = b.normalize();
        (T::from_f64(2.0) - T::from_f64(2.0) * b.dot(a).abs())
            .max(T::zero())
            .sqrt()
    };
    let off = |p: Point3<T>| sheet_of(o, a, p).0;
    let near = |x: T| x.hi() <= band.zero();
    // The surface, read about the axis it must share with the band.
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
        Surface::Cylinder { origin, axis, .. } if near(off(*origin)) && near(tilt(*axis)) => {
            Kind::Cylinder
        }
        Surface::Cone { apex, axis, .. } if near(off(*apex)) && near(tilt(*axis)) => Kind::Cone {
            z_apex: sheet_of(o, a, *apex).1,
        },
        Surface::Plane { normal, .. } if near(tilt(*normal)) => Kind::Plane,
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
            let cert = body.get_curve_geom(e.curve)?.certified()?;
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
            grow([r0 - pad, r1 + pad, z0 - pad, z1 + pad]);
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
                    let lean = (T::from_f64(2.0)
                        - T::from_f64(2.0) * (T::one() - dot.powi(2)).max(T::zero()).sqrt())
                    .max(T::zero())
                    .sqrt();
                    let d_mer = off(center) + lean * radius;
                    if d_lat.hi() <= d_mer.lo() {
                        // A latitude circle: the azimuth turns with the
                        // parameter; a circle displaced by `D` from the
                        // coaxial one sees each point's azimuth move by
                        // at most `π/2·D/ρ_min`.
                        let e = pi * d_lat / (radius - d_lat).max(T::zero());
                        let z = sheet_of(o, a, center).1 - zc;
                        (
                            dot / dot.abs() * dt,
                            e,
                            (z.abs() + d_lat) * e + d_lat * dt.abs(),
                            z,
                        )
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
                    let clear = if r0.lo() > 0.0 {
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
    let lever = |p: Point3<T>| corner_max(reach.lo, reach.hi, |x| (x - p).norm());
    let (f, slack) = sheet_fn(surface, o, a, lever)?;
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

/// The point of a face's boundary deepest in a reach's core: the most
/// negative strict bound ([`Bound::at_strict`]) over [`EDGE_SAMPLES`]
/// places along each boundary edge, ends included.
fn deepest_boundary_point<T: Decide>(
    body: &Body<T>,
    face: FaceKey,
    core: &[Bound<T>],
) -> Result<Option<T>, BlendError> {
    let f = body
        .get_face(face)
        .ok_or_else(|| not_intact(EntityId::Face(face), "a face the band reach reads"))?;
    let strict = |x: Point3<T>| {
        core.iter()
            .map(|b| b.at_strict(x))
            .reduce(|m, g| m.max(g))
            .unwrap_or_else(T::zero)
    };
    let mut least: Option<T> = None;
    let mut fold = |g: T| least = Some(least.map_or(g, |l: T| l.min(g)));
    for lp in core::iter::once(f.outer).chain(f.rings.iter().copied()) {
        let l = body
            .get_loop(lp)
            .ok_or_else(|| not_intact(EntityId::Loop(lp), "a face's loop"))?;
        let first = match l.boundary {
            topo::LoopBoundary::Cycle { first } => first,
            topo::LoopBoundary::Empty { vertex } => {
                fold(strict(point_of(body, vertex)?));
                continue;
            }
        };
        for he in body
            .loop_cycle(first)
            .ok_or_else(|| not_intact(EntityId::Loop(lp), "a face's cycle"))?
        {
            let edge = body
                .get_half_edge(he)
                .ok_or_else(|| not_intact(EntityId::HalfEdge(he), "a face's boundary"))?
                .edge;
            let Some((c, t0, t1)) = carrier_of(body, edge) else {
                continue;
            };
            for i in 0..EDGE_SAMPLES {
                fold(strict(c.eval(geom_brep::schedule_param(
                    t0,
                    t1,
                    i,
                    EDGE_SAMPLES,
                ))));
            }
        }
    }
    Ok(least)
}

/// **Meter every chain's reach against every face of the body it does
/// not exclude** (module docs), before any mutation.
///
/// # Errors
///
/// [`BlendError::FaceClearance`] for a face in a reach, or not certified
/// clear of one; [`BlendError::Escalated`] for an in-band margin;
/// [`BlendError::BodyNotIntact`] for a body the meter cannot read.
pub(crate) fn band_reach<T: Decide + Bounds>(
    body: &Body<T>,
    chains: &[Chain<T>],
    size: T,
    kind: BlendKind,
    band: Band,
) -> Result<(), BlendError> {
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
    for v in verts {
        let fan = body
            .edges_of_vertex(v)
            .ok_or_else(|| not_intact(EntityId::Vertex(v), "a chain vertex's edge fan"))?;
        if !fan.iter().all(|e| requested.binary_search(e).is_ok()) {
            continue;
        }
        let here = at(v);
        let links: Vec<&Link<T>> = here.iter().map(|(_, l)| *l).collect();
        let chain = here.first().map_or(0, |(ci, _)| *ci);
        if let Some((r, anchors)) = corner_reach(body, v, &links, size, kind, chain)? {
            reaches.push(r);
            corners.push((v, anchors));
        }
    }
    for (ci, link) in &all {
        let st = station(body, link)?;
        reaches.push(if link.arm.is_coaxial_torus() {
            circular_reach(body, link, &st, *ci)?
        } else {
            straight_reach(body, link, &st, &corners, *ci)?
        });
    }
    // What each chain's reaches skip by key; a corner shared by several
    // chains skips what each of them skips.
    let mut skip: Vec<Vec<FaceKey>> = chains
        .iter()
        .map(|c| excluded(body, c))
        .collect::<Result<_, _>>()?;
    for (v, _) in &corners {
        let here = at(*v);
        let union: Vec<FaceKey> = here
            .iter()
            .flat_map(|(ci, _)| skip[*ci].iter().copied())
            .collect();
        for (ci, _) in &here {
            skip[*ci].extend(union.iter().copied());
        }
    }
    for s in &mut skip {
        s.sort_unstable();
        s.dedup();
    }
    let boxes = FaceBoxes::of(body, band).map_err(|_| BlendError::SurgeryInvariant {
        at: EntityId::Face(FaceKey::default()),
        detail: "a face's certified box could not be read for the band reach",
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
            if skip[reach.chain].binary_search(&face).is_ok()
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
                .filter(|r| r.chain != reach.chain)
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
            // Refused: a measurement where a point of the face's own
            // boundary lies definitely inside the material of a band
            // whose bounds are its own exactly, and the face is no
            // chain's support (whose boundary the request may itself be
            // carving away); a bound otherwise.
            let supports_a_band = reaches
                .iter()
                .any(|r| r.replaces.iter().any(|(f, _)| *f == face));
            for exact in reaches.iter().filter(|r| !r.core.is_empty()) {
                if supports_a_band || skip[exact.chain].binary_search(&face).is_ok() {
                    continue;
                }
                let Some(depth) = deepest_boundary_point(body, face, &exact.core)? else {
                    continue;
                };
                if classify_bound(depth)? == Sign::Negative {
                    return Err(BlendError::FaceClearance {
                        face,
                        chain: chains[exact.chain].first().convexity,
                        margin: classified(decision, depth, band, Sign::Negative),
                        bounded: false,
                    });
                }
            }
            return Err(BlendError::FaceClearance {
                face,
                chain: chains[reach.chain].first().convexity,
                margin: classified(decision, bound, band, sign),
                bounded: true,
            });
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
        for other in reaches.iter().filter(|r| {
            r.chain != reach.chain
                && !meet(r.chain, reach.chain)
                && !(removes && chains[r.chain].first().convexity == Convexity::Concave)
        }) {
            let olo = [other.lo.x.lo(), other.lo.y.lo(), other.lo.z.lo()];
            let ohi = [other.hi.x.hi(), other.hi.y.hi(), other.hi.z.hi()];
            if (0..3).any(|k| ohi[k] < rlo[k] || rhi[k] < olo[k]) {
                continue;
            }
            let banded = |role: &Role| *role != Role::Band;
            let off_patch = |cell: &Cell<T>| {
                other
                    .bounds
                    .iter()
                    .filter(|(role, _)| banded(role))
                    .map(|(_, b)| b.low(cell))
                    .reduce(|m, g| m.max(g))
            };
            let off_patch_in_sheet = |rect: Rect<T>| {
                if !same_sheet(other, reach) {
                    return None;
                }
                other
                    .bounds
                    .iter()
                    .filter(|(role, _)| banded(role))
                    .map(|(_, b)| b.low_in_sheet(rect))
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
                let face = other
                    .bounds
                    .iter()
                    .find_map(|(role, _)| match role {
                        Role::Side(f) => Some(*f),
                        _ => None,
                    })
                    .unwrap_or_default();
                return Err(BlendError::FaceClearance {
                    face,
                    chain: chains[reach.chain].first().convexity,
                    margin: classified(decision, bound, band, sign),
                    bounded: true,
                });
            }
        }
    }
    Ok(())
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

/// **One surface against one reach**: the meridian-sheet path where the
/// reach and the surface allow it, and the cells in space where it does
/// not certify; `extra` and `extra_in_sheet` are further clearing reads
/// (a lower bound that, positive, says the cell holds no point to meter).
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
    match face_bound_in_sheet(body, face, reach, surface, bounds, extra_in_sheet, band) {
        Some(m) if m.lo() >= band.escalate() => m,
        Some(m) => m.max(face_bound(reach, surface, bounds, extra, band)),
        None => face_bound(reach, surface, bounds, extra, band),
    }
}

/// The test-support door to [`band_reach`] alone, for a fillet request:
/// its links resolved and walked into chains as the battery does, and
/// no other predicate run — so a row can pin the reach on a body some
/// earlier predicate or door refuses. Compiled into no shipped build.
///
/// # Errors
///
/// [`band_reach`]'s, and whatever resolving a link refuses.
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
    let chains = super::battery::walk_chains(links);
    band_reach(req.body, &chains, req.size, BlendKind::Fillet, band)
}
