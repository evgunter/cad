//! **The region an entity can occupy: its cover** (`docs/DESIGN.md`
//! `[one-way.places]`, "The region a carrier, an edge or a face can
//! occupy", and `[one-way.readings]`, "The region an entity can occupy is
//! a reading").
//!
//! A [`Cover`] is a value over a closed set of shapes, each with a
//! closed-form support, and the meets and hulls of them. **It has no
//! whole-space value**: a reading that cannot bound its entity answers
//! [`Uncovered`], which names the entity and what was not known and has
//! no bound method. [`Lattice`] defines `meet` and `hull` on
//! [`Covered`] (`Result<Cover<T>, Uncovered>`): a refusal is the meet's
//! identity and the hull's absorber, so a reading that meets what it
//! proves keeps every constraint it could read, and a reader that hulls
//! two entities refuses when either is unbounded.
//!
//! Every bound is a method on the cover and matches on no kind:
//! [`Cover::support`] along a direction, [`Cover::box_in`] a [`Frame`],
//! [`Cover::distance_range`] from a [`Site`], [`Cover::ball`], and at a
//! bracketed scalar [`Cover::aabb`], the one place the bracket is taken.
//! **Each encloses the region at every scalar.** A piece charges its own
//! arithmetic's rounding outward at its own scale: at `f64`, whose
//! arithmetic rounds to nearest, the charge is what makes the answer an
//! enclosure; at `Interval` it widens an enclosure already certified. So
//! no reader adds a pad to make a cover enclose.
//!
//! Geom's readings are [`Surface::cover`](crate::Surface::cover), a bounded
//! carrier whole, and [`Curve3::cover_over`](crate::Curve3::cover_over), a
//! curve over a span. They are the only kind matches.

use bvh::Aabb;
use geom_core::{Bounds, Point3, Real, UnitVec3, Vec3};

use crate::curves::boxes::ArcWindow;
use crate::{CurveKind, Line, SurfaceKind};

mod readings;
#[cfg(test)]
mod tests;

/// A cover, or the reason a reading has none.
pub type Covered<T> = Result<Cover<T>, Uncovered>;

/// **A parameter window**: the span `[lo, hi]` of a chart or axial
/// parameter, as data. It does no arithmetic: the piece that holds it
/// reads it, in either orientation.
#[derive(Clone, Copy, Debug)]
pub struct ParamWindow<T> {
    /// One end of the window.
    pub lo: T,
    /// The other end.
    pub hi: T,
}

impl<T: Real> ParamWindow<T> {
    /// The window `[lo, hi]`.
    pub fn new(lo: T, hi: T) -> Self {
        Self { lo, hi }
    }

    /// The ends, least first.
    fn ordered(self) -> (T, T) {
        (self.lo.min(self.hi), self.lo.max(self.hi))
    }

    /// The larger end magnitude: the scale an angle's rounding is read at.
    fn reach(self) -> T {
        self.lo.abs().max(self.hi.abs())
    }
}

/// **The frame a box is read in** ([`Cover::box_in`]): the world's axes,
/// or three rows. Coordinate `i` of the box is the cover's support along
/// `±` row `i`, so with orthonormal rows the box is the cover's box in
/// that frame, and the first coordinate of an aimed frame is the support
/// along its aim, which turns with the entity.
#[derive(Clone, Copy, Debug)]
pub enum Frame<T: Real> {
    /// The world's axes.
    World,
    /// Three rows: each coordinate of a point is its component along one.
    Rows([Vec3<T>; 3]),
}

impl<T: Real> Frame<T> {
    /// The frame whose first row is `aim`, the other two
    /// [`UnitVec3::orthonormal_basis`]'s, in the cyclic order
    /// `(aim, b1, b2)` of the right-handed `(b1, b2, aim)`.
    pub fn aimed(aim: UnitVec3<T>) -> Self {
        let (b1, b2) = aim.orthonormal_basis();
        Self::Rows([aim.get(), b1, b2])
    }

    /// The frame's rows.
    pub fn rows(&self) -> [Vec3<T>; 3] {
        match self {
            Self::World => [Vec3::unit_x(), Vec3::unit_y(), Vec3::unit_z()],
            Self::Rows(rows) => *rows,
        }
    }
}

/// **What a distance is measured from** ([`Cover::distance_range`]).
#[derive(Clone, Copy, Debug)]
pub enum Site<T: Real> {
    /// A point.
    Point(Point3<T>),
    /// A whole line; its direction need not be unit.
    Line(Line<T>),
}

/// **The entity a reading could not cover.** Geom's readings name a
/// carrier by its kind.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Entity {
    /// A surface carrier.
    Surface(SurfaceKind),
    /// A curve carrier.
    Curve(CurveKind),
}

/// **What was not known** about an entity a reading could not cover.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Unknown {
    /// The kind's locus is unbounded: a plane, a cylinder or a cone,
    /// whole.
    Unbounded,
    /// The control net bounds no region: a point of it carries poison in
    /// some channel, or it has no point.
    PoisonedNet,
}

/// **A reading's refusal to cover**: the entities it could not bound and
/// what it did not know about each, one pair per entity. A meet of two
/// refusals keeps both lists.
///
/// No bound is a method here. A reader reaches a bound only by stating
/// its own fail-loud direction on the refusal: a pruning reader takes
/// the box that overlaps everything, a granting reader refuses with this
/// as its reason.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Uncovered {
    unknowns: Vec<(Entity, Unknown)>,
}

impl Uncovered {
    /// The refusal for one entity.
    pub fn new(entity: Entity, what: Unknown) -> Self {
        Self {
            unknowns: vec![(entity, what)],
        }
    }

    /// Every entity refused and what was not known about it, in the order
    /// the refusals were combined.
    pub fn unknowns(&self) -> &[(Entity, Unknown)] {
        &self.unknowns
    }

    fn and(mut self, other: Self) -> Self {
        self.unknowns.extend(other.unknowns);
        self
    }
}

impl core::fmt::Display for Uncovered {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str("no cover:")?;
        for (entity, what) in &self.unknowns {
            let entity = match entity {
                Entity::Surface(kind) => kind.name(),
                Entity::Curve(kind) => kind.name(),
            };
            let what = match what {
                Unknown::Unbounded => "its locus is unbounded",
                Unknown::PoisonedNet => "its control net bounds no region",
            };
            write!(f, " the {entity}: {what};")?;
        }
        Ok(())
    }
}

impl std::error::Error for Uncovered {}

/// **`meet` and `hull` on [`Covered`]**: `Ok ∧ Err = Ok`, the meet's
/// identity, and `Ok ∨ Err = Err`, the hull's absorber; two refusals
/// combine their reasons either way.
pub trait Lattice: Sized {
    /// What lies in both.
    #[must_use]
    fn meet(self, other: Self) -> Self;
    /// What lies in either.
    #[must_use]
    fn hull(self, other: Self) -> Self;
}

impl<T: Real> Lattice for Covered<T> {
    fn meet(self, other: Self) -> Self {
        match (self, other) {
            (Ok(a), Ok(b)) => Ok(a.meet(b)),
            (Ok(a), Err(_)) | (Err(_), Ok(a)) => Ok(a),
            (Err(a), Err(b)) => Err(a.and(b)),
        }
    }

    fn hull(self, other: Self) -> Self {
        match (self, other) {
            (Ok(a), Ok(b)) => Ok(a.hull(b)),
            (Err(e), Ok(_)) | (Ok(_), Err(e)) => Err(e),
            (Err(a), Err(b)) => Err(a.and(b)),
        }
    }
}

/// **A region an entity lies in** (module docs).
#[derive(Clone, Debug)]
pub struct Cover<T: Real>(Node<T>);

#[derive(Clone, Debug)]
enum Node<T: Real> {
    Piece(Piece<T>),
    Meet(Box<[Node<T>; 2]>),
    Hull(Box<[Node<T>; 2]>),
}

/// The closed set of shapes. Each one's frame vectors are read as the
/// carrier's evaluator reads them; the pieces whose methods need a unit
/// axis say so, and the rest hold whatever frame they are handed.
#[derive(Clone, Debug)]
enum Piece<T: Real> {
    /// Exactly these points, at least one.
    Points(Vec<Point3<T>>),
    /// The segment between two points.
    Segment([Point3<T>; 2]),
    /// `center + semi_u·cos t·u_ref + semi_v·sin t·v_ref`, `t` in the
    /// window.
    Arc(Arc<T>),
    /// The solid cylinder about `origin + h·axis`, `h` in the window, of
    /// radius `radius`; the axis is unit.
    Slab(Slab<T>),
    /// The solid cone `apex + t·axis + w`, `w ⊥ axis`,
    /// `|w| ≤ |t|·tan`, `t` in the window; the axis is unit.
    Frustum(Frustum<T>),
    /// A ring torus's chart rectangle or latitude zone, or a sphere's
    /// (`major = 0`).
    Patch(Patch<T>),
    /// The solid ball `|x − center| ≤ radius`.
    Ball { center: Point3<T>, radius: T },
    /// The solid tube of radius `minor` about the circle of radius
    /// `major` round `axis` through `center`.
    Tube(Tube<T>),
    /// The convex hull of the points, at least one, widened by `slack`.
    Net { points: Vec<Point3<T>>, slack: T },
}

#[derive(Clone, Copy, Debug)]
struct Arc<T: Real> {
    center: Point3<T>,
    u_ref: Vec3<T>,
    v_ref: Vec3<T>,
    semi_u: T,
    semi_v: T,
    window: ParamWindow<T>,
}

#[derive(Clone, Copy, Debug)]
struct Slab<T: Real> {
    origin: Point3<T>,
    axis: Vec3<T>,
    radius: T,
    window: ParamWindow<T>,
}

#[derive(Clone, Copy, Debug)]
struct Frustum<T: Real> {
    apex: Point3<T>,
    axis: Vec3<T>,
    tan: T,
    window: ParamWindow<T>,
}

#[derive(Clone, Copy, Debug)]
struct Patch<T: Real> {
    tube: Tube<T>,
    u: Option<ParamWindow<T>>,
    v: ParamWindow<T>,
}

#[derive(Clone, Copy, Debug)]
struct Tube<T: Real> {
    center: Point3<T>,
    axis: Vec3<T>,
    u_ref: Vec3<T>,
    v_ref: Vec3<T>,
    major: T,
    minor: T,
}

/// The ulps of its scale a piece built of products and sums charges
/// each bound: a dot product rounds by `γ₃·Σ|xᵢeᵢ| ≤ 1.5 ε·|x||e|`, a norm
/// or a sum by an ulp more.
const LINEAR_ULPS: f64 = 4.0;

/// The ulps of its scale every other piece charges each bound: those
/// read a carrier's frame, which is unit and orthogonal only to its
/// rounding, so the carrier's evaluated locus stands a few ulps off the
/// region the frame names, and their arithmetic runs through roots,
/// `atan2` and cosines ([`ArcWindow`]'s steps), each a few ulps of the
/// term it rounds, none cancelling.
const CURVED_ULPS: f64 = 16.0;

fn linear_charge<T: Real>(scale: T) -> T {
    scale * T::from_f64(LINEAR_ULPS * f64::EPSILON)
}

fn curved_charge<T: Real>(scale: T) -> T {
    scale * T::from_f64(CURVED_ULPS * f64::EPSILON)
}

fn vec_of<T: Real>(p: Point3<T>) -> Vec3<T> {
    p - Point3::origin()
}

fn magnitude<T: Real>(p: Point3<T>) -> T {
    vec_of(p).norm()
}

/// `v`'s part off the line direction `d`.
fn off_line<T: Real>(v: Vec3<T>, d: Vec3<T>) -> Vec3<T> {
    v - d * (v.dot(d) / d.dot(d))
}

/// A ball, `(centre, radius)`.
type Ball<T> = (Point3<T>, T);

/// The ball of the two with the smaller radius: both hold the region, so
/// either serves, and where the scalar cannot tell them apart it holds
/// both candidates.
fn smaller<T: Real>(a: Ball<T>, b: Ball<T>) -> Ball<T> {
    let d = a.1 - b.1;
    let pick = |x: T, y: T| d.select_le_zero(x, y);
    (
        Point3::new(pick(a.0.x, b.0.x), pick(a.0.y, b.0.y), pick(a.0.z, b.0.z)),
        pick(a.1, b.1),
    )
}

/// The ball about a box's centre out to its corners.
fn box_ball<T: Real>((lo, hi): (Point3<T>, Point3<T>)) -> Ball<T> {
    let half = (hi - lo) * T::from_f64(0.5);
    let c = lo + half;
    let r = half.norm();
    (c, r + linear_charge(magnitude(c) + r))
}

/// The nearest and farthest a point of the world box `[lo, hi]` stands
/// from `q`.
fn box_range<T: Real>((lo, hi): (Point3<T>, Point3<T>), q: Point3<T>) -> (T, T) {
    let axis = |l: T, h: T, x: T| {
        let near = (l - x).max(x - h).max(T::zero());
        let far = (x - l).abs().max((h - x).abs());
        (near, far)
    };
    let (nx, fx) = axis(lo.x, hi.x, q.x);
    let (ny, fy) = axis(lo.y, hi.y, q.y);
    let (nz, fz) = axis(lo.z, hi.z, q.z);
    let scale = magnitude(q) + magnitude(lo).max(magnitude(hi));
    let ch = linear_charge(scale);
    let near = Vec3::new(nx, ny, nz).norm() - ch;
    (near.max(T::zero()), Vec3::new(fx, fy, fz).norm() + ch)
}

/// A ball's distance range from a site.
fn ball_range<T: Real>((c, r): Ball<T>, site: &Site<T>) -> (T, T) {
    let (delta, scale) = site.offset(c);
    let ch = curved_charge(scale + r);
    ((delta - r - ch).max(T::zero()), delta + r + ch)
}

impl<T: Real> Site<T> {
    /// How far `p` stands from the site, and the scale that distance's
    /// rounding is read at.
    fn offset(&self, p: Point3<T>) -> (T, T) {
        let (v, scale) = self.relative(vec_of(p));
        (v.norm(), scale)
    }

    /// `p`, as a vector from the site: from the point, or off the line.
    /// With the scale of the subtraction.
    fn relative(&self, p: Vec3<T>) -> (Vec3<T>, T) {
        match self {
            Site::Point(q) => (p - vec_of(*q), p.norm() + magnitude(*q)),
            Site::Line(line) => (
                off_line(p - vec_of(line.origin), line.dir),
                p.norm() + magnitude(line.origin),
            ),
        }
    }

    /// A direction, as the site reads it: whole from a point, off the
    /// line from a line.
    fn direction(&self, v: Vec3<T>) -> Vec3<T> {
        match self {
            Site::Point(_) => v,
            Site::Line(line) => off_line(v, line.dir),
        }
    }
}

impl<T: Real> Cover<T> {
    fn piece(piece: Piece<T>) -> Self {
        Self(Node::Piece(piece))
    }

    /// Exactly these points: a vertex set. `None` for no point.
    pub fn points(points: Vec<Point3<T>>) -> Option<Self> {
        (!points.is_empty()).then(|| Self::piece(Piece::Points(points)))
    }

    /// The segment from `a` to `b`.
    pub fn segment(a: Point3<T>, b: Point3<T>) -> Self {
        Self::piece(Piece::Segment([a, b]))
    }

    /// The conic arc `center + semi_u·cos t·u_ref + semi_v·sin t·v_ref`
    /// over `window`: a circle's or an ellipse's, `v_ref = axis × u_ref`
    /// as its evaluator crosses it.
    pub fn arc(
        center: Point3<T>,
        (u_ref, v_ref): (Vec3<T>, Vec3<T>),
        (semi_u, semi_v): (T, T),
        window: ParamWindow<T>,
    ) -> Self {
        Self::piece(Piece::Arc(Arc {
            center,
            u_ref,
            v_ref,
            semi_u,
            semi_v,
            window,
        }))
    }

    /// The axial slab: the solid cylinder of radius `|radius|` about the
    /// axis from `origin + lo·axis` to `origin + hi·axis`.
    pub fn slab(origin: Point3<T>, axis: UnitVec3<T>, radius: T, window: ParamWindow<T>) -> Self {
        Self::piece(Piece::Slab(Slab {
            origin,
            axis: axis.get(),
            radius: radius.abs(),
            window,
        }))
    }

    /// The cone frustum: every point `apex + t·axis + w` with `w ⊥ axis`,
    /// `|w| ≤ |t·tan_half_angle|` and `t` in `window`.
    pub fn frustum(
        apex: Point3<T>,
        axis: UnitVec3<T>,
        tan_half_angle: T,
        window: ParamWindow<T>,
    ) -> Self {
        Self::piece(Piece::Frustum(Frustum {
            apex,
            axis: axis.get(),
            tan: tan_half_angle.abs(),
            window,
        }))
    }

    /// **A ring torus's chart rectangle `u × v`**, or its latitude zone
    /// where `u` is `None` (every azimuth): the points
    /// `center + (R + r·cos v)·(cos u·u_ref + sin u·(axis × u_ref)) + r·sin v·axis`.
    /// A sphere's is `major = 0` with `v` its latitude.
    ///
    /// The closed form reads `R + r·cos v ≥ 0` over the window, so its
    /// premise is that every torus the scalar admits is a ring
    /// (`R ≥ r ≥ 0`) or a sphere's chart (`R = 0`, `r ≥ 0`,
    /// `v ⊂ [−π/2, π/2]`). The caller decides it before it builds one.
    pub fn patch(
        center: Point3<T>,
        (axis, u_ref): (Vec3<T>, Vec3<T>),
        (major, minor): (T, T),
        u: Option<ParamWindow<T>>,
        v: ParamWindow<T>,
    ) -> Self {
        Self::piece(Piece::Patch(Patch {
            tube: Tube::new(center, axis, u_ref, major, minor),
            u,
            v,
        }))
    }

    /// The whole solid ball about `center` of radius `|radius|`.
    pub fn whole_ball(center: Point3<T>, radius: T) -> Self {
        Self::piece(Piece::Ball {
            center,
            radius: radius.abs(),
        })
    }

    /// The solid tube of radius `|minor|` about the circle of radius
    /// `|major|` round `axis` through `center`, `u_ref` a direction
    /// across the axis.
    pub fn tube(center: Point3<T>, axis: Vec3<T>, u_ref: Vec3<T>, major: T, minor: T) -> Self {
        let tube = Tube::new(center, axis, u_ref, major, minor);
        Self::piece(Piece::Tube(Tube {
            major: tube.major.abs(),
            minor: tube.minor.abs(),
            ..tube
        }))
    }

    /// The convex hull of a control net's points. `None` for no point.
    pub fn net(points: Vec<Point3<T>>) -> Option<Self> {
        (!points.is_empty()).then(|| {
            Self::piece(Piece::Net {
                points,
                slack: T::zero(),
            })
        })
    }

    /// What lies in both.
    #[must_use]
    pub fn meet(self, other: Self) -> Self {
        Self(Node::Meet(Box::new([self.0, other.0])))
    }

    /// What lies in either.
    #[must_use]
    pub fn hull(self, other: Self) -> Self {
        Self(Node::Hull(Box::new([self.0, other.0])))
    }

    /// **The support along `dir`**: an upper bound on `x·dir` over the
    /// region. `dir` need not be unit; the support is linear in it.
    pub fn support(&self, dir: Vec3<T>) -> T {
        self.0.support(dir)
    }

    /// **The box in `frame`**: per row, the least and greatest coordinate
    /// any point of the region can have along it, as the low and high
    /// corners.
    pub fn box_in(&self, frame: &Frame<T>) -> (Point3<T>, Point3<T>) {
        let [a, b, c] = frame.rows();
        let span = |r: Vec3<T>| (T::zero() - self.support(-r), self.support(r));
        let ((xl, xh), (yl, yh), (zl, zh)) = (span(a), span(b), span(c));
        (Point3::new(xl, yl, zl), Point3::new(xh, yh, zh))
    }

    /// **The nearest and farthest the region stands from `site`**: a
    /// lower bound on the least distance of any of its points, never
    /// below zero, and an upper bound on the greatest.
    pub fn distance_range(&self, site: &Site<T>) -> (T, T) {
        self.0.distance_range(site)
    }

    /// **A ball holding the region**, as `(centre, radius)`.
    pub fn ball(&self) -> (Point3<T>, T) {
        self.0.ball()
    }
}

impl<T: Bounds> Cover<T> {
    /// **The world box, bracketed and padded by `pad`**: each end of
    /// [`Cover::box_in`] at [`Frame::World`] taken at its bracket's outer
    /// end, then [`Aabb::padded`]. Poison anywhere reaches a NaN bound,
    /// which overlaps everything.
    pub fn aabb(&self, pad: f64) -> Aabb {
        let (lo, hi) = self.box_in(&Frame::World);
        Aabb {
            min_x: lo.x.lo(),
            min_y: lo.y.lo(),
            min_z: lo.z.lo(),
            max_x: hi.x.hi(),
            max_y: hi.y.hi(),
            max_z: hi.z.hi(),
        }
        .padded(pad)
    }
}

impl<T: Real> Node<T> {
    fn support(&self, e: Vec3<T>) -> T {
        match self {
            Node::Piece(p) => p.support(e),
            Node::Meet(both) => both[0].support(e).min(both[1].support(e)),
            Node::Hull(both) => both[0].support(e).max(both[1].support(e)),
        }
    }

    fn world_box(&self) -> (Point3<T>, Point3<T>) {
        Cover(self.clone()).box_in(&Frame::World)
    }

    fn distance_range(&self, site: &Site<T>) -> (T, T) {
        match self {
            Node::Piece(p) => p.distance_range(site),
            Node::Meet(both) => {
                let ((n0, f0), (n1, f1)) =
                    (both[0].distance_range(site), both[1].distance_range(site));
                (n0.max(n1), f0.min(f1))
            }
            Node::Hull(both) => {
                let ((n0, f0), (n1, f1)) =
                    (both[0].distance_range(site), both[1].distance_range(site));
                (n0.min(n1), f0.max(f1))
            }
        }
    }

    fn ball(&self) -> Ball<T> {
        match self {
            Node::Piece(p) => p.ball(),
            Node::Meet(both) => smaller(both[0].ball(), both[1].ball()),
            Node::Hull(both) => {
                let ((c0, r0), (c1, r1)) = (both[0].ball(), both[1].ball());
                let c = c0 + (c1 - c0) * T::from_f64(0.5);
                let r = ((c - c0).norm() + r0).max((c - c1).norm() + r1);
                (c, r + linear_charge(magnitude(c) + r))
            }
        }
    }
}

impl<T: Real> Tube<T> {
    fn new(center: Point3<T>, axis: Vec3<T>, u_ref: Vec3<T>, major: T, minor: T) -> Self {
        Self {
            center,
            axis,
            u_ref,
            v_ref: axis.cross(u_ref),
            major,
            minor,
        }
    }

    fn scale(&self) -> T {
        magnitude(self.center) + self.major.abs() + self.minor.abs()
    }

    /// `e`'s components along `u_ref`, `v_ref` and the axis.
    fn components(&self, e: Vec3<T>) -> (T, T, T) {
        (self.u_ref.dot(e), self.v_ref.dot(e), self.axis.dot(e))
    }

    /// The whole tube's support: `c·e + |R|·M + |r|·√(M² + aₑ²)`, `M` the
    /// norm of `e`'s components across the axis. A point of the torus
    /// is `c + R·ρ + r·(cos v·ρ + sin v·axis)` with `ρ` the radial
    /// direction, and `ρ·e ≤ M`; so the bound holds for any frame and
    /// either sign, and for a unit orthogonal frame it is attained:
    /// `R·|e⊥| + r` along a unit `e`.
    fn support(&self, e: Vec3<T>) -> T {
        let (p, q, a) = self.components(e);
        let m = (p.powi(2) + q.powi(2)).sqrt();
        let reach = self.major.abs() * m + self.minor.abs() * (m.powi(2) + a.powi(2)).sqrt();
        vec_of(self.center).dot(e) + reach + curved_charge(self.scale() * e.norm())
    }

    fn ball(&self) -> Ball<T> {
        let r = self.major.abs() + self.minor.abs();
        (self.center, r + curved_charge(self.scale()))
    }

    /// From a point: the spine circle's nearest and farthest, less and
    /// plus the tube radius. Reads the axis as unit.
    fn point_range(&self, q: Point3<T>) -> (T, T) {
        let w = q - self.center;
        let h = w.dot(self.axis);
        let rho = w.cross(self.axis).norm();
        let (big, r) = (self.major.abs(), self.minor.abs());
        let ch = curved_charge(w.norm() + self.scale());
        let near = (h.powi(2) + (rho - big).powi(2)).sqrt() - r - ch;
        let far = (h.powi(2) + (rho + big).powi(2)).sqrt() + r + ch;
        (near.max(T::zero()), far)
    }

    fn range(&self, site: &Site<T>) -> (T, T) {
        match site {
            Site::Point(q) => self.point_range(*q),
            Site::Line(_) => ball_range(self.ball(), site),
        }
    }
}

impl<T: Real> Patch<T> {
    /// `c·e + R·M + r·B`: `M` the greatest `p·cos u + q·sin u` over the
    /// azimuth window and `B` the greatest `M·cos v + a·sin v` over the
    /// latitude window, the closed form whose premise
    /// [`Cover::patch`] states.
    fn support(&self, e: Vec3<T>) -> T {
        let t = &self.tube;
        let (p, q, a) = t.components(e);
        let (m, turns) = match self.u {
            Some(u) => (
                ArcWindow::new(u.lo, u.hi).extent(T::zero(), p, q).1,
                u.reach(),
            ),
            None => ((p.powi(2) + q.powi(2)).sqrt(), T::zero()),
        };
        let b = ArcWindow::new(self.v.lo, self.v.hi)
            .extent(T::zero(), m, a)
            .1;
        let scale = t.scale() * (T::one() + turns + self.v.reach());
        vec_of(t.center).dot(e) + t.major * m + t.minor * b + curved_charge(scale * e.norm())
    }
}

impl<T: Real> Slab<T> {
    fn support(&self, e: Vec3<T>) -> T {
        let (lo, hi) = self.window.ordered();
        let ae = self.axis.dot(e);
        let room = self.axis.cross(e).norm();
        let scale = magnitude(self.origin) + self.window.reach() + self.radius;
        vec_of(self.origin).dot(e)
            + (lo * ae).max(hi * ae)
            + self.radius * room
            + curved_charge(scale * e.norm())
    }

    /// The exact range of the solid cylinder from a point. Reads the axis
    /// as unit.
    fn point_range(&self, q: Point3<T>) -> (T, T) {
        let (lo, hi) = self.window.ordered();
        let w = q - self.origin;
        let h = w.dot(self.axis);
        let rho = w.cross(self.axis).norm();
        let dh = (lo - h).max(h - hi).max(T::zero());
        let fh = (h - lo).abs().max((h - hi).abs());
        let ch =
            curved_charge(w.norm() + magnitude(self.origin) + self.window.reach() + self.radius);
        let near = (dh.powi(2) + (rho - self.radius).max(T::zero()).powi(2)).sqrt() - ch;
        let far = (fh.powi(2) + (rho + self.radius).powi(2)).sqrt() + ch;
        (near.max(T::zero()), far)
    }

    fn ball(&self) -> Ball<T> {
        let (lo, hi) = self.window.ordered();
        let half = T::from_f64(0.5);
        let c = self.origin + self.axis * ((lo + hi) * half);
        let r = (((hi - lo) * half).powi(2) + self.radius.powi(2)).sqrt();
        (c, r + curved_charge(magnitude(c) + r))
    }
}

impl<T: Real> Frustum<T> {
    fn support(&self, e: Vec3<T>) -> T {
        let (lo, hi) = self.window.ordered();
        let ae = self.axis.dot(e);
        let k = self.tan * self.axis.cross(e).norm();
        let at = |t: T| t * ae + t.abs() * k;
        let kink = lo.max(T::zero()).min(hi);
        let scale = magnitude(self.apex) + self.window.reach() * (T::one() + self.tan);
        vec_of(self.apex).dot(e)
            + at(lo).max(at(hi)).max(at(kink))
            + curved_charge(scale * e.norm())
    }

    /// The slab the frustum lies in: its widest radius over the window.
    fn slab(&self) -> Slab<T> {
        Slab {
            origin: self.apex,
            axis: self.axis,
            radius: self.window.reach() * self.tan,
            window: self.window,
        }
    }
}

impl<T: Real> Arc<T> {
    fn scale(&self) -> T {
        let amp = self.semi_u.abs() * self.u_ref.norm() + self.semi_v.abs() * self.v_ref.norm();
        magnitude(self.center) + amp * (T::one() + self.window.reach())
    }

    fn support(&self, e: Vec3<T>) -> T {
        let (_, hi) = ArcWindow::new(self.window.lo, self.window.hi).extent(
            vec_of(self.center).dot(e),
            self.semi_u * self.u_ref.dot(e),
            self.semi_v * self.v_ref.dot(e),
        );
        hi + curved_charge(self.scale() * e.norm())
    }

    /// **The squared distance, bounded by its harmonics.** With `w` the
    /// centre from the site and `P`, `Q` the semi-axis vectors, all read
    /// as the site reads them, `|x − site|²` is
    /// `|w|² + (|P|² + |Q|²)/2` plus a first harmonic
    /// `2w·P cos t + 2w·Q sin t` plus a second
    /// `(|P|² − |Q|²)/2·cos 2t + P·Q·sin 2t`; each is bounded over the
    /// window by [`ArcWindow`], and their sum by the sum of the bounds,
    /// which is exact for a circle seen from a point.
    fn range(&self, site: &Site<T>) -> (T, T) {
        let (w, scale) = site.relative(vec_of(self.center));
        let p = site.direction(self.u_ref * self.semi_u);
        let q = site.direction(self.v_ref * self.semi_v);
        let two = T::from_f64(2.0);
        let half = T::from_f64(0.5);
        let k = w.dot(w) + (p.dot(p) + q.dot(q)) * half;
        let one = ArcWindow::new(self.window.lo, self.window.hi).extent(
            T::zero(),
            two * w.dot(p),
            two * w.dot(q),
        );
        let two_t = ArcWindow::new(two * self.window.lo, two * self.window.hi).extent(
            T::zero(),
            (p.dot(p) - q.dot(q)) * half,
            p.dot(q),
        );
        let ch = curved_charge((scale + self.scale()).powi(2));
        let near = (k + one.0 + two_t.0 - ch).max(T::zero()).sqrt();
        let far = (k + one.1 + two_t.1 + ch).max(T::zero()).sqrt();
        (near, far)
    }

    fn ball(&self, world: (Point3<T>, Point3<T>)) -> Ball<T> {
        let r = (self.semi_u.abs() * self.u_ref.norm()).max(self.semi_v.abs() * self.v_ref.norm());
        let whole = (self.center, r + curved_charge(self.scale()));
        smaller(whole, box_ball(world))
    }
}

impl<T: Real> Piece<T> {
    fn support(&self, e: Vec3<T>) -> T {
        match self {
            Piece::Points(points) => points_support(points, e, T::zero()),
            Piece::Net { points, slack } => points_support(points, e, *slack),
            Piece::Segment(ends) => points_support(ends, e, segment_slack(ends)),
            Piece::Arc(arc) => arc.support(e),
            Piece::Slab(slab) => slab.support(e),
            Piece::Frustum(f) => f.support(e),
            Piece::Patch(patch) => patch.support(e),
            Piece::Ball { center, radius } => {
                let n = e.norm();
                vec_of(*center).dot(e)
                    + *radius * n
                    + curved_charge((magnitude(*center) + *radius) * n)
            }
            Piece::Tube(tube) => tube.support(e),
        }
    }

    fn world_box(&self) -> (Point3<T>, Point3<T>) {
        Node::Piece(self.clone()).world_box()
    }

    fn distance_range(&self, site: &Site<T>) -> (T, T) {
        match self {
            Piece::Points(points) => points_range(points, site, T::zero()),
            Piece::Segment([a, b]) => segment_range(*a, *b, site),
            Piece::Arc(arc) => arc.range(site),
            Piece::Slab(slab) => match site {
                Site::Point(q) => slab.point_range(*q),
                Site::Line(_) => ball_range(slab.ball(), site),
            },
            Piece::Frustum(f) => match site {
                Site::Point(q) => {
                    let (n0, f0) = f.slab().point_range(*q);
                    let (n1, f1) = box_range(self.world_box(), *q);
                    (n0.max(n1), f0.min(f1))
                }
                Site::Line(_) => ball_range(self.ball(), site),
            },
            Piece::Patch(patch) => match site {
                Site::Point(q) => {
                    let (n0, f0) = patch.tube.point_range(*q);
                    let (n1, f1) = box_range(self.world_box(), *q);
                    (n0.max(n1), f0.min(f1))
                }
                Site::Line(_) => ball_range(self.ball(), site),
            },
            Piece::Ball { center, radius } => ball_range((*center, *radius), site),
            Piece::Tube(tube) => tube.range(site),
            Piece::Net { points, slack } => {
                let (_, far) = points_range(points, site, *slack);
                let near = match site {
                    Site::Point(q) => box_range(self.world_box(), *q).0,
                    Site::Line(_) => ball_range(self.ball(), site).0,
                };
                (near, far)
            }
        }
    }

    fn ball(&self) -> Ball<T> {
        match self {
            Piece::Points(points) => points_ball(points, T::zero()),
            Piece::Net { points, slack } => points_ball(points, *slack),
            Piece::Segment(ends) => points_ball(ends, segment_slack(ends)),
            Piece::Arc(arc) => arc.ball(self.world_box()),
            Piece::Slab(slab) => slab.ball(),
            Piece::Frustum(f) => smaller(f.slab().ball(), box_ball(self.world_box())),
            Piece::Patch(patch) => smaller(patch.tube.ball(), box_ball(self.world_box())),
            Piece::Ball { center, radius } => (
                *center,
                *radius + curved_charge(magnitude(*center) + *radius),
            ),
            Piece::Tube(tube) => tube.ball(),
        }
    }
}

/// A segment's ends are a carrier's points read at the scalar, each a
/// few roundings of its scale off the carrier: the slack its bounds
/// carry for that.
fn segment_slack<T: Real>([a, b]: &[Point3<T>; 2]) -> T {
    curved_charge(magnitude(*a).max(magnitude(*b)))
}

/// The greatest `p·e` over the points, widened by `slack·|e|`.
fn points_support<T: Real>(points: &[Point3<T>], e: Vec3<T>, slack: T) -> T {
    let first = vec_of(points[0]);
    let (best, scale) = points[1..]
        .iter()
        .fold((first.dot(e), first.norm()), |(best, scale), &p| {
            (best.max(vec_of(p).dot(e)), scale.max(magnitude(p)))
        });
    let n = e.norm();
    best + slack * n + linear_charge(scale * n)
}

/// The least and greatest distance of the points from the site, charged
/// for a line's projection at [`CURVED_ULPS`], the greatest widened by
/// `slack`, which is what a net's hull reaches: a
/// distance from a point or a line is convex, so its greatest over a
/// hull is at a point of it.
fn points_range<T: Real>(points: &[Point3<T>], site: &Site<T>, slack: T) -> (T, T) {
    let (d0, s0) = site.offset(points[0]);
    let (near, far, scale) = points[1..]
        .iter()
        .fold((d0, d0, s0), |(near, far, scale), &p| {
            let (d, s) = site.offset(p);
            (near.min(d), far.max(d), scale.max(s))
        });
    let ch = curved_charge(scale);
    ((near - slack - ch).max(T::zero()), far + slack + ch)
}

/// The segment's range: its farther end, and its nearest point, the
/// clamped foot of the site on it as the site reads it.
fn segment_range<T: Real>(a: Point3<T>, b: Point3<T>, site: &Site<T>) -> (T, T) {
    let (a_rel, sa) = site.relative(vec_of(a));
    let (b_rel, sb) = site.relative(vec_of(b));
    let d = b_rel - a_rel;
    let den = d.dot(d);
    let s = den.select_le_zero(
        T::zero(),
        (T::zero() - a_rel.dot(d) / den)
            .max(T::zero())
            .min(T::one()),
    );
    let ch = curved_charge(sa.max(sb));
    let near = (a_rel + d * s).norm() - ch;
    let far = a_rel.norm().max(b_rel.norm()) + ch;
    (near.max(T::zero()), far)
}

/// The ball about the points' box centre, out to the farthest, widened
/// by `slack`.
fn points_ball<T: Real>(points: &[Point3<T>], slack: T) -> Ball<T> {
    let (lo, hi) = points[1..]
        .iter()
        .fold((points[0], points[0]), |(lo, hi), &p| {
            (lo.min(p), hi.max(p))
        });
    let c = lo + (hi - lo) * T::from_f64(0.5);
    let r = points.iter().fold(T::zero(), |r, &p| r.max((p - c).norm()));
    (c, r + slack + linear_charge(magnitude(c) + r))
}
