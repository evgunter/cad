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
//! **The values follow `[simple.conditioned]`**: at `f64` each is the
//! real-arithmetic bound computed as if exact, and nothing charges float
//! rounding; at `Interval` each is an enclosure, by the scalar's own
//! arithmetic. So no reader adds a pad of its own.
//!
//! Geom's readings are [`Surface::cover`](crate::Surface::cover), a bounded
//! carrier whole, and [`Curve3::cover_over`](crate::Curve3::cover_over), a
//! curve over a span. They are the only kind matches.

use bvh::Aabb;
use geom_core::{Bounds, Point3, Real, UnitVec3, Vec3};

use crate::curves::boxes::ArcWindow;
use crate::{CurveKind, Line, SurfaceKind};

mod readings;
pub use readings::SPLINE_PIECES;
#[cfg(test)]
mod tests;

/// A cover, or the reason a reading has none.
pub type Covered<T> = Result<Cover<T>, Uncovered>;

/// **A parameter window**: the span `[lo, hi]` of a chart or axial
/// parameter, as data, in either orientation. The piece that holds it
/// reads its ends; the window computes nothing from them.
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

    fn finite(self) -> bool {
        finite(&[self.lo, self.hi])
    }

    /// The ends, least first.
    fn ordered(self) -> (T, T) {
        (self.lo.min(self.hi), self.lo.max(self.hi))
    }

    /// The larger end magnitude.
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
    /// some channel.
    PoisonedNet,
    /// A datum of the carrier, or a window end, is not a finite number,
    /// so the region it names is not bounded.
    NonFinite,
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
                Unknown::NonFinite => "a datum of it is not a finite number",
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
    Convex(Box<Node<T>>),
}

/// The closed set of shapes. A frame a piece holds is read as the
/// carrier's evaluator reads it, whatever its lengths and angles, and
/// every method is sound for it; the slab's and the frustum's axis is a
/// decided unit direction by type.
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
    /// The solid tube of radius `minor` about the circle of radius
    /// `major` round `axis` through `center`.
    Tube(Tube<T>),
    /// The convex hull of the points, at least one.
    Net(Vec<Point3<T>>),
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

/// An arc's mean circle as [`Arc::circle_about`] reads it about a point.
struct Circle<T: Real> {
    u_hat: Vec3<T>,
    v_hat: Vec3<T>,
    rho: T,
    dev: T,
    w: Vec3<T>,
    at: (T, T, T),
}

impl<T: Real> Circle<T> {
    /// The distance from the point to the circle's point at `t`.
    fn end(&self, t: T) -> T {
        let (s, c) = t.sin_cos();
        (self.w - (self.u_hat * c + self.v_hat * s) * self.rho).norm()
    }
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

/// A patch holds its torus's tube, whose radii are magnitudes, for the
/// bounds the whole tube serves, and its own signed radii for the closed
/// form, whose rectangle the signs place.
#[derive(Clone, Copy, Debug)]
struct Patch<T: Real> {
    tube: Tube<T>,
    major: T,
    minor: T,
    u: Option<ParamWindow<T>>,
    v: ParamWindow<T>,
}

/// The whole tube: its radii are magnitudes, taken once in
/// [`Tube::new`], because the region does not depend on their signs.
#[derive(Clone, Copy, Debug)]
struct Tube<T: Real> {
    center: Point3<T>,
    axis: Vec3<T>,
    u_ref: Vec3<T>,
    v_ref: Vec3<T>,
    major: T,
    minor: T,
}

/// Whether every datum is a finite number: a constructor's screen, so a
/// cover names a bounded region whatever its data.
fn finite<T: Real>(xs: &[T]) -> bool {
    xs.iter().all(|&x| geom_core::is_finite_length(x))
}

fn finite_point<T: Real>(p: Point3<T>) -> bool {
    finite(&[p.x, p.y, p.z])
}

fn finite_vec<T: Real>(v: Vec3<T>) -> bool {
    finite(&[v.x, v.y, v.z])
}

/// `√λ_max` of the Gram matrix of `x` and `y`: the longest
/// `|cos t·x + sin t·y|`, `(A + B)/2 + √(((A − B)/2)² + C²)` under the
/// root, which is `1` for an orthonormal pair. `A` and `B` are squared
/// norms, which an enclosure keeps non-negative where a self-dot product
/// would not (`x·x` of an enclosure about zero reaches below it).
fn gram_top<T: Real>(xx: T, yy: T, xy: T) -> T {
    let half = T::from_f64(0.5);
    ((xx + yy) * half + (((xx - yy) * half).powi(2) + xy.powi(2)).sqrt()).sqrt()
}

fn vec_of<T: Real>(p: Point3<T>) -> Vec3<T> {
    p - Point3::origin()
}

/// `v`'s part off the line direction `d`.
fn off_line<T: Real>(v: Vec3<T>, d: Vec3<T>) -> Vec3<T> {
    v - d * (v.dot(d) / d.norm_squared())
}

/// The box whose row `i` spans `±` the support along row `i`.
fn rows_box<T: Real>(frame: &Frame<T>, support: impl Fn(Vec3<T>) -> T) -> (Point3<T>, Point3<T>) {
    let [a, b, c] = frame.rows();
    let span = |r: Vec3<T>| (T::zero() - support(-r), support(r));
    let ((xl, xh), (yl, yh), (zl, zh)) = (span(a), span(b), span(c));
    (Point3::new(xl, yl, zl), Point3::new(xh, yh, zh))
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
    (c, r)
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
    (Vec3::new(nx, ny, nz).norm(), Vec3::new(fx, fy, fz).norm())
}

/// A ball's distance range from a site.
fn ball_range<T: Real>((c, r): Ball<T>, site: &Site<T>) -> (T, T) {
    let delta = site.offset(c);
    ((delta - r).max(T::zero()), delta + r)
}

impl<T: Real> Site<T> {
    /// How far `p` stands from the site.
    fn offset(&self, p: Point3<T>) -> T {
        self.relative(vec_of(p)).norm()
    }

    /// `p`, as a vector from the site: from the point, or off the line.
    fn relative(&self, p: Vec3<T>) -> Vec3<T> {
        match self {
            Site::Point(q) => p - vec_of(*q),
            Site::Line(line) => off_line(p - vec_of(line.origin), line.dir),
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

    /// Exactly these points: a vertex set. `None` for no point or a
    /// coordinate that is not a finite number.
    pub fn points(points: Vec<Point3<T>>) -> Option<Self> {
        (!points.is_empty() && points.iter().all(|&p| finite_point(p)))
            .then(|| Self::piece(Piece::Points(points)))
    }

    /// The segment from `a` to `b`. `None` for a coordinate that is not a
    /// finite number.
    pub fn segment(a: Point3<T>, b: Point3<T>) -> Option<Self> {
        (finite_point(a) && finite_point(b)).then(|| Self::piece(Piece::Segment([a, b])))
    }

    /// The conic arc `center + semi_u·cos t·u_ref + semi_v·sin t·v_ref`
    /// over `window`: a circle's or an ellipse's, `v_ref = axis × u_ref`
    /// as its evaluator crosses it. Any frame and either sign. `None` for
    /// a datum that is not a finite number.
    pub fn arc(
        center: Point3<T>,
        (u_ref, v_ref): (Vec3<T>, Vec3<T>),
        (semi_u, semi_v): (T, T),
        window: ParamWindow<T>,
    ) -> Option<Self> {
        (finite_point(center)
            && finite_vec(u_ref)
            && finite_vec(v_ref)
            && finite(&[semi_u, semi_v])
            && window.finite())
        .then(|| {
            Self::piece(Piece::Arc(Arc {
                center,
                u_ref,
                v_ref,
                semi_u,
                semi_v,
                window,
            }))
        })
    }

    /// The axial slab: the solid cylinder of radius `|radius|` about the
    /// axis from `origin + lo·axis` to `origin + hi·axis`. `None` for a
    /// datum that is not a finite number.
    pub fn slab(
        origin: Point3<T>,
        axis: UnitVec3<T>,
        radius: T,
        window: ParamWindow<T>,
    ) -> Option<Self> {
        (finite_point(origin) && finite_vec(axis.get()) && finite(&[radius]) && window.finite())
            .then(|| {
                Self::piece(Piece::Slab(Slab {
                    origin,
                    axis: axis.get(),
                    radius: radius.abs(),
                    window,
                }))
            })
    }

    /// The cone frustum: every point `apex + t·axis + w` with `w ⊥ axis`,
    /// `|w| ≤ |t·tan_half_angle|` and `t` in `window`. `None` for a datum
    /// that is not a finite number.
    pub fn frustum(
        apex: Point3<T>,
        axis: UnitVec3<T>,
        tan_half_angle: T,
        window: ParamWindow<T>,
    ) -> Option<Self> {
        (finite_point(apex)
            && finite_vec(axis.get())
            && finite(&[tan_half_angle])
            && window.finite())
        .then(|| {
            Self::piece(Piece::Frustum(Frustum {
                apex,
                axis: axis.get(),
                tan: tan_half_angle.abs(),
                window,
            }))
        })
    }

    /// **A torus's chart rectangle `u × v`**, or its latitude zone where
    /// `u` is `None` (every azimuth): the points
    /// `center + R·ρ(u) + r·(cos v·ρ(u) + sin v·axis)`,
    /// `ρ(u) = cos u·u_ref + sin u·(axis × u_ref)`, for any frame and
    /// either sign of `r`. A sphere's is `major = 0` with `v` its
    /// latitude.
    ///
    /// The closed form reads the factor `R + r·cos v` as non-negative
    /// over the window: its premise is that it is, for every torus the
    /// scalar admits. A ring torus (`R ≥ |r|`) satisfies it for any
    /// window, and a sphere's latitude chart (`R = 0`, `r ≥ 0`,
    /// `v ⊂ [−π/2, π/2]`) does. The caller decides it before it builds
    /// one. `None` for a datum that is not a finite number.
    pub fn patch(
        center: Point3<T>,
        (axis, u_ref): (Vec3<T>, Vec3<T>),
        (major, minor): (T, T),
        u: Option<ParamWindow<T>>,
        v: ParamWindow<T>,
    ) -> Option<Self> {
        let tube = Tube::new(center, axis, u_ref, major, minor)?;
        (u.is_none_or(ParamWindow::finite) && v.finite()).then(|| {
            Self::piece(Piece::Patch(Patch {
                tube,
                major,
                minor,
                u,
                v,
            }))
        })
    }

    /// **The whole torus's tube**: every point
    /// `center + R·ρ(u) + r·(cos v·ρ(u) + sin v·axis)`, for any frame and
    /// either sign of either radius. `None` for a datum that is not a
    /// finite number.
    pub fn tube(
        center: Point3<T>,
        axis: Vec3<T>,
        u_ref: Vec3<T>,
        major: T,
        minor: T,
    ) -> Option<Self> {
        Tube::new(center, axis, u_ref, major, minor).map(|t| Self::piece(Piece::Tube(t)))
    }

    /// The convex hull of a control net's points. `None` for no point or
    /// a coordinate that is not a finite number.
    pub fn net(points: Vec<Point3<T>>) -> Option<Self> {
        (!points.is_empty() && points.iter().all(|&p| finite_point(p)))
            .then(|| Self::piece(Piece::Net(points)))
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

    /// **The region's convex hull**: what lies on a segment between two
    /// of its points. A face of a ruled carrier lies in its boundary's,
    /// which [`Cover::hull`] of the boundary's pieces does not hold.
    #[must_use]
    pub fn convex(self) -> Self {
        Self(Node::Convex(Box::new(self.0)))
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
        rows_box(frame, |e| self.support(e))
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

    /// **The nearest and farthest the region stands from a circle**: the
    /// circle of radius `radius` about `center` in the plane normal to
    /// `axis`, which need not be unit. A point `ρ` from the axis line and
    /// `t` along it stands `√((ρ − R)² + t²)` from the circle, so any
    /// region is bounded through its distance range from the axis line
    /// and its support along the axis; a tube about a spine is bounded
    /// through its spine's, within its minor radius.
    pub fn circle_range(&self, center: Point3<T>, axis: Vec3<T>, radius: T) -> (T, T) {
        self.0.circle_range(&Core {
            center,
            axis,
            radius,
        })
    }
}

/// A circle a distance is read from ([`Cover::circle_range`]).
struct Core<T: Real> {
    center: Point3<T>,
    axis: Vec3<T>,
    radius: T,
}

impl<T: Real> Core<T> {
    /// The circle's distance range over a region read through its range
    /// from the axis line (`line`) and its support (`support`).
    fn through(&self, line: impl Fn(&Site<T>) -> (T, T), support: impl Fn(Vec3<T>) -> T) -> (T, T) {
        let (near, far) = line(&Site::Line(Line {
            origin: self.center,
            dir: self.axis,
        }));
        let a = self.axis / self.axis.norm();
        let base = vec_of(self.center).dot(a);
        let (below, above) = (T::zero() - support(-a) - base, support(a) - base);
        let r = self.radius;
        let outside = |x: T, lo: T, hi: T| (lo - x).max(x - hi).max(T::zero());
        let radial = (near - r).abs().max((far - r).abs());
        let height = below.abs().max(above.abs());
        (
            (outside(r, near, far).powi(2) + outside(T::zero(), below, above).powi(2)).sqrt(),
            (radial.powi(2) + height.powi(2)).sqrt(),
        )
    }

    /// The exact distance of a point from the circle.
    fn offset(&self, p: Point3<T>) -> T {
        let a = self.axis / self.axis.norm();
        let w = p - self.center;
        let t = w.dot(a);
        let rho = off_line(w, a).norm();
        ((rho - self.radius).powi(2) + t.powi(2)).sqrt()
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
            Node::Convex(inner) => inner.support(e),
        }
    }

    fn world_box(&self) -> (Point3<T>, Point3<T>) {
        rows_box(&Frame::World, |e| self.support(e))
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
            // A distance from a point or a line is convex, so its
            // greatest over the hull is its greatest over the region; its
            // least is the hull's box's or ball's.
            Node::Convex(inner) => {
                let far = inner.distance_range(site).1;
                let near = match site {
                    Site::Point(q) => box_range(self.world_box(), *q)
                        .0
                        .max(ball_range(inner.ball(), site).0),
                    Site::Line(_) => ball_range(inner.ball(), site).0,
                };
                (near, far)
            }
        }
    }

    fn ball(&self) -> Ball<T> {
        match self {
            Node::Piece(p) => p.ball(),
            // Both balls hold the region, and so does the ball about the
            // meet's box, which is narrower than either piece's where the
            // two overlap only in part.
            Node::Meet(both) => smaller(
                smaller(both[0].ball(), both[1].ball()),
                box_ball(self.world_box()),
            ),
            Node::Hull(both) => {
                let ((c0, r0), (c1, r1)) = (both[0].ball(), both[1].ball());
                let c = c0 + (c1 - c0) * T::from_f64(0.5);
                let r = ((c - c0).norm() + r0).max((c - c1).norm() + r1);
                (c, r)
            }
            Node::Convex(inner) => inner.ball(),
        }
    }
}

impl<T: Real> Node<T> {
    fn circle_range(&self, core: &Core<T>) -> (T, T) {
        match self {
            Node::Piece(p) => p.circle_range(core),
            Node::Meet(both) => {
                let ((n0, f0), (n1, f1)) = (both[0].circle_range(core), both[1].circle_range(core));
                (n0.max(n1), f0.min(f1))
            }
            Node::Hull(both) => {
                let ((n0, f0), (n1, f1)) = (both[0].circle_range(core), both[1].circle_range(core));
                (n0.min(n1), f0.max(f1))
            }
            // A circle's distance is not convex, so the hull is read
            // whole.
            Node::Convex(_) => core.through(|site| self.distance_range(site), |e| self.support(e)),
        }
    }
}

impl<T: Real> Tube<T> {
    fn new(center: Point3<T>, axis: Vec3<T>, u_ref: Vec3<T>, major: T, minor: T) -> Option<Self> {
        (finite_point(center) && finite_vec(axis) && finite_vec(u_ref) && finite(&[major, minor]))
            .then(|| Self {
                center,
                axis,
                u_ref,
                v_ref: axis.cross(u_ref),
                major: major.abs(),
                minor: minor.abs(),
            })
    }

    /// `e`'s components along `u_ref`, `v_ref` and the axis.
    fn components(&self, e: Vec3<T>) -> (T, T, T) {
        (self.u_ref.dot(e), self.v_ref.dot(e), self.axis.dot(e))
    }

    /// The whole tube's support: `c·e + |R|·M + |r|·√(M² + aₑ²)`, `M` the
    /// norm of `e`'s components along `u_ref` and `v_ref`. A point of
    /// the torus is `c + R·ρ + r·(cos v·ρ + sin v·axis)` and `ρ·e`
    /// reaches `±M` round the turn, so the bound is attained for any
    /// frame and either sign: `R·|e⊥| + r` along a unit `e` in a unit
    /// orthogonal frame.
    fn support(&self, e: Vec3<T>) -> T {
        let (p, q, a) = self.components(e);
        let m = (p.powi(2) + q.powi(2)).sqrt();
        let reach = self.major * m + self.minor * (m.powi(2) + a.powi(2)).sqrt();
        vec_of(self.center).dot(e) + reach
    }

    /// The longest radial direction `|ρ(u)|`, and the longest tube offset
    /// `|cos v·ρ(u) + sin v·axis|` over the turn: Gram bounds, each `1`
    /// for a unit orthogonal frame. The second reads `|ρ|² ≤ ρ_max²` and
    /// `|ρ·axis| ≤ √((u_ref·a)² + (v_ref·a)²)`, in which the Gram bound is
    /// non-decreasing.
    fn reaches(&self) -> (T, T) {
        let (u, v, a) = (self.u_ref, self.v_ref, self.axis);
        let rho = gram_top(u.norm_squared(), v.norm_squared(), u.dot(v));
        let tilt = (u.dot(a).powi(2) + v.dot(a).powi(2)).sqrt();
        (rho, gram_top(rho.powi(2), a.norm_squared(), tilt))
    }

    fn ball(&self) -> Ball<T> {
        let (rho, off) = self.reaches();
        let r = self.major * rho + self.minor * off;
        (self.center, r)
    }

    /// The spine circle's range from a circle, read through the spine
    /// alone ([`Core::through`]), less and plus the longest tube offset.
    fn circle_range(&self, core: &Core<T>) -> (T, T) {
        let spine = self.spine();
        let (near, far) = core.through(|site| spine.range(site), |e| spine.support(e));
        let tube = self.minor * self.reaches().1;
        ((near - tube).max(T::zero()), far + tube)
    }

    /// The spine circle, whole.
    fn spine(&self) -> Arc<T> {
        Arc {
            center: self.center,
            u_ref: self.u_ref,
            v_ref: self.v_ref,
            semi_u: self.major,
            semi_v: self.major,
            window: ParamWindow::new(T::zero(), T::tau()),
        }
    }

    /// The spine circle's range ([`Arc::range`], which holds any frame),
    /// less and plus the longest tube offset.
    fn range(&self, site: &Site<T>) -> (T, T) {
        let (near, far) = self.spine().range(site);
        let tube = self.minor * self.reaches().1;
        ((near - tube).max(T::zero()), far + tube)
    }
}

impl<T: Real> Patch<T> {
    /// `c·e + R·M + B`: `M` the greatest `p·cos u + q·sin u` over the
    /// azimuth window and `B` the greatest `r·(M·cos v + a·sin v)` over
    /// the latitude window, `r`'s sign inside the harmonic. With the
    /// factor `R + r·cos v` non-negative ([`Cover::patch`]'s premise) the
    /// best `u` is the same at every `v`, so this is the rectangle's own
    /// support.
    fn support(&self, e: Vec3<T>) -> T {
        let t = &self.tube;
        let (p, q, a) = t.components(e);
        let m = match self.u {
            Some(u) => ArcWindow::new(u.lo, u.hi).extent(T::zero(), p, q).1,
            None => (p.powi(2) + q.powi(2)).sqrt(),
        };
        let b = ArcWindow::new(self.v.lo, self.v.hi)
            .extent(T::zero(), self.minor * m, self.minor * a)
            .1;
        vec_of(t.center).dot(e) + self.major * m + b
    }
}

impl<T: Real> Slab<T> {
    fn support(&self, e: Vec3<T>) -> T {
        let (lo, hi) = self.window.ordered();
        let ae = self.axis.dot(e);
        let room = self.axis.cross(e).norm();
        vec_of(self.origin).dot(e) + (lo * ae).max(hi * ae) + self.radius * room
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
        let near = (dh.powi(2) + (rho - self.radius).max(T::zero()).powi(2)).sqrt();
        let far = (fh.powi(2) + (rho + self.radius).powi(2)).sqrt();
        (near.max(T::zero()), far)
    }

    fn ball(&self) -> Ball<T> {
        let (lo, hi) = self.window.ordered();
        let half = T::from_f64(0.5);
        let c = self.origin + self.axis * ((lo + hi) * half);
        let r = (((hi - lo) * half).powi(2) + self.radius.powi(2)).sqrt();
        (c, r)
    }
}

impl<T: Real> Frustum<T> {
    fn support(&self, e: Vec3<T>) -> T {
        let (lo, hi) = self.window.ordered();
        let ae = self.axis.dot(e);
        let k = self.tan * self.axis.cross(e).norm();
        // Convex in `t`, so greatest at an end.
        let at = |t: T| t * ae + t.abs() * k;
        vec_of(self.apex).dot(e) + at(lo).max(at(hi))
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
    fn support(&self, e: Vec3<T>) -> T {
        let (_, hi) = ArcWindow::new(self.window.lo, self.window.hi).extent(
            vec_of(self.center).dot(e),
            self.semi_u * self.u_ref.dot(e),
            self.semi_v * self.v_ref.dot(e),
        );
        hi
    }

    /// **The distance range from a site.** From a point both ends are
    /// posed as distances ([`Arc::point_near`], [`Arc::point_far`]): the
    /// squared form below cancels where the point is near the arc, and
    /// on a short arc far from its centre.
    ///
    /// From a line the squared form stands, since a line's distance to a
    /// circle is a quartic's root with no closed form to pose it as: with
    /// `w` the centre from the site and `P`, `Q` the semi-axis vectors,
    /// all read as the site reads them, `|x − site|²` is
    /// `|w|² + (|P|² + |Q|²)/2` plus a first harmonic
    /// `2w·P cos t + 2w·Q sin t` plus a second
    /// `(|P|² − |Q|²)/2·cos 2t + P·Q·sin 2t`, each bounded over the window
    /// by [`ArcWindow`] and their sum by the sum of the bounds.
    fn range(&self, site: &Site<T>) -> (T, T) {
        if let Site::Point(at) = site {
            return (self.point_near(*at), self.point_far(*at));
        }
        let w = site.relative(vec_of(self.center));
        let p = site.direction(self.u_ref * self.semi_u);
        let q = site.direction(self.v_ref * self.semi_v);
        let two = T::from_f64(2.0);
        let half = T::from_f64(0.5);
        let k = w.norm_squared() + (p.norm_squared() + q.norm_squared()) * half;
        let one = ArcWindow::new(self.window.lo, self.window.hi).extent(
            T::zero(),
            two * w.dot(p),
            two * w.dot(q),
        );
        let two_t = ArcWindow::new(two * self.window.lo, two * self.window.hi).extent(
            T::zero(),
            (p.norm_squared() - q.norm_squared()) * half,
            p.dot(q),
        );
        let far = (k + one.1 + two_t.1).max(T::zero()).sqrt();
        let near = (k + one.0 + two_t.0).max(T::zero()).sqrt();
        (near, far)
    }

    /// The circle [`Arc::point_near`] and [`Arc::point_far`] read the arc
    /// against, and `at` in its frame: the orthonormalized `(û, v̂)`, the
    /// mean radius `ρ̄` as that frame reads the two semi-axes, the Gram
    /// bound `dev` of the semi-axis vectors' departures from it (zero for
    /// a circle in an orthonormal frame, half the axes' difference for an
    /// ellipse), and `at − centre`'s components `(x, y, h)` along `û`,
    /// `v̂` and their normal.
    fn circle_about(&self, at: Point3<T>) -> Circle<T> {
        let half = T::from_f64(0.5);
        let u_len = self.u_ref.norm();
        let u_hat = self.u_ref / u_len;
        let v_off = self.v_ref - u_hat * self.v_ref.dot(u_hat);
        let v_hat = v_off / v_off.norm();
        let rho = (self.semi_u * u_len + self.semi_v * self.v_ref.dot(v_hat)) * half;
        let dp = self.u_ref * self.semi_u - u_hat * rho;
        let dq = self.v_ref * self.semi_v - v_hat * rho;
        let dev = gram_top(dp.norm_squared(), dq.norm_squared(), dp.dot(dq));
        let w = at - self.center;
        Circle {
            u_hat,
            v_hat,
            rho,
            dev,
            w,
            at: (w.dot(u_hat), w.dot(v_hat), w.dot(u_hat.cross(v_hat))),
        }
    }

    /// **The nearest distance from `at`, as a distance.** The arc lies
    /// within `dev` of the circular arc of radius `ρ̄` over the same window
    /// ([`Arc::circle_about`]). That arc's nearest point is the one in
    /// `at`'s azimuth where the window holds that azimuth,
    /// `√(h² + (ρ_at − |ρ̄|)²)`, and the nearer end where it does not;
    /// every term is a distance, so none cancels near the arc.
    fn point_near(&self, at: Point3<T>) -> T {
        let c = self.circle_about(at);
        let (x, y, h) = c.at;
        let off_axis = (x.powi(2) + y.powi(2)).sqrt();
        let in_azimuth = (h.powi(2) + (off_axis - c.rho.abs()).powi(2)).sqrt();
        let ends = c.end(self.window.lo).min(c.end(self.window.hi));
        let near = ArcWindow::new(self.window.lo, self.window.hi)
            .crest_outside(c.rho * x, c.rho * y)
            .select_le_zero(in_azimuth, ends);
        (near - c.dev).max(T::zero())
    }

    /// **The farthest distance from `at`, as a distance**: the lesser of
    /// two bounds whose every term is a distance, so neither cancels on a
    /// short arc read from near it.
    ///
    /// - The circular arc of [`Arc::circle_about`], widened by `dev`: its
    ///   farthest point is the crest opposite `at`'s azimuth where the
    ///   window holds it, `√(h² + (ρ_at + |ρ̄|)²)`, and the farther end
    ///   where it does not. Exact for a circle in an orthonormal frame.
    /// - The arc cut into four pieces of half-width `h ≤ π/4` (the window
    ///   capped at one turn either way). The arc is an affine image of
    ///   the unit circle, on which a piece about its middle `m` lies in
    ///   the rectangle its chord spans out to its crest, so the piece lies
    ///   in the parallelogram on its ends shifted by its bulge
    ///   `(1 − cos h)·(x(m) − c)`, and a distance from a point is convex,
    ///   so it peaks at a corner. Past the arc by at most a piece's bulge.
    fn point_far(&self, at: Point3<T>) -> T {
        let c = self.circle_about(at);
        let (x, y, h) = c.at;
        let off_axis = (x.powi(2) + y.powi(2)).sqrt();
        let crest = (h.powi(2) + (off_axis + c.rho.abs()).powi(2)).sqrt();
        let (lo, hi) = (self.window.lo, self.window.hi);
        let ends = c.end(lo).max(c.end(hi));
        let circle = ends.max(
            ArcWindow::new(lo, hi)
                .crest_outside(T::zero() - c.rho * x, T::zero() - c.rho * y)
                .select_le_zero(crest, T::zero()),
        ) + c.dev;
        let turn = T::tau();
        let quarter = ((hi - lo).max(T::zero() - turn).min(turn)) * T::from_f64(0.25);
        let shift = T::one() - (quarter * T::from_f64(0.5)).abs().cos();
        let point = |t: T| self.point(t);
        let pieces = (0..4).fold((point(lo) - at).norm(), |far, k| {
            let s0 = lo + quarter * T::from_f64(f64::from(k));
            let s1 = s0 + quarter;
            let bulge = (point(s0 + quarter * T::from_f64(0.5)) - self.center) * shift;
            let (x0, x1) = (point(s0), point(s1));
            [x0 + bulge, x1, x1 + bulge]
                .into_iter()
                .fold(far, |far, p| far.max((p - at).norm()))
        });
        circle.min(pieces)
    }

    /// The arc's point at `t`.
    fn point(&self, t: T) -> Point3<T> {
        let (s, c) = t.sin_cos();
        self.center + self.u_ref * (self.semi_u * c) + self.v_ref * (self.semi_v * s)
    }

    fn ball(&self, world: (Point3<T>, Point3<T>)) -> Ball<T> {
        let (p, q) = (self.u_ref * self.semi_u, self.v_ref * self.semi_v);
        let r = gram_top(p.norm_squared(), q.norm_squared(), p.dot(q));
        let whole = (self.center, r);
        smaller(whole, box_ball(world))
    }
}

impl<T: Real> Piece<T> {
    fn support(&self, e: Vec3<T>) -> T {
        match self {
            Piece::Points(points) | Piece::Net(points) => points_support(points, e),
            Piece::Segment(ends) => points_support(ends, e),
            Piece::Arc(arc) => arc.support(e),
            Piece::Slab(slab) => slab.support(e),
            Piece::Frustum(f) => f.support(e),
            Piece::Patch(patch) => patch.support(e),
            Piece::Tube(tube) => tube.support(e),
        }
    }

    fn world_box(&self) -> (Point3<T>, Point3<T>) {
        rows_box(&Frame::World, |e| self.support(e))
    }

    fn distance_range(&self, site: &Site<T>) -> (T, T) {
        match self {
            Piece::Points(points) => points_range(points, site),
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
                    let (n0, f0) = patch.tube.range(site);
                    let (n1, f1) = box_range(self.world_box(), *q);
                    (n0.max(n1), f0.min(f1))
                }
                Site::Line(_) => patch.tube.range(site),
            },
            Piece::Tube(tube) => tube.range(site),
            Piece::Net(points) => {
                let (_, far) = points_range(points, site);
                let near = match site {
                    Site::Point(q) => box_range(self.world_box(), *q).0,
                    Site::Line(_) => ball_range(self.ball(), site).0,
                };
                (near, far)
            }
        }
    }

    fn circle_range(&self, core: &Core<T>) -> (T, T) {
        let whole = || core.through(|site| self.distance_range(site), |e| self.support(e));
        match self {
            Piece::Points(points) => {
                let d0 = core.offset(points[0]);
                points[1..].iter().fold((d0, d0), |(near, far), &p| {
                    let d = core.offset(p);
                    (near.min(d), far.max(d))
                })
            }
            Piece::Tube(tube) => tube.circle_range(core),
            Piece::Patch(patch) => {
                let ((n0, f0), (n1, f1)) = (patch.tube.circle_range(core), whole());
                (n0.max(n1), f0.min(f1))
            }
            _ => whole(),
        }
    }

    fn ball(&self) -> Ball<T> {
        match self {
            Piece::Points(points) | Piece::Net(points) => points_ball(points),
            Piece::Segment(ends) => points_ball(ends),
            Piece::Arc(arc) => arc.ball(self.world_box()),
            Piece::Slab(slab) => slab.ball(),
            Piece::Frustum(f) => smaller(f.slab().ball(), box_ball(self.world_box())),
            Piece::Patch(patch) => smaller(patch.tube.ball(), box_ball(self.world_box())),
            Piece::Tube(tube) => tube.ball(),
        }
    }
}

/// The greatest `p·e` over the points.
fn points_support<T: Real>(points: &[Point3<T>], e: Vec3<T>) -> T {
    points[1..]
        .iter()
        .fold(vec_of(points[0]).dot(e), |best, &p| {
            best.max(vec_of(p).dot(e))
        })
}

/// The least and greatest distance of the points from the site; the
/// greatest is also a net's hull's: a distance from a point or a line is convex, so its greatest over a
/// hull is at a point of it.
fn points_range<T: Real>(points: &[Point3<T>], site: &Site<T>) -> (T, T) {
    let d0 = site.offset(points[0]);
    points[1..].iter().fold((d0, d0), |(near, far), &p| {
        let d = site.offset(p);
        (near.min(d), far.max(d))
    })
}

/// The segment's range: its farther end, and its nearest point, the
/// clamped foot of the site on it as the site reads it.
fn segment_range<T: Real>(a: Point3<T>, b: Point3<T>, site: &Site<T>) -> (T, T) {
    let (a_rel, b_rel) = (site.relative(vec_of(a)), site.relative(vec_of(b)));
    let d = b_rel - a_rel;
    let den = d.norm_squared();
    let s = den.select_le_zero(
        T::zero(),
        (T::zero() - a_rel.dot(d) / den)
            .max(T::zero())
            .min(T::one()),
    );
    ((a_rel + d * s).norm(), a_rel.norm().max(b_rel.norm()))
}

/// The ball about the points' box centre, out to the farthest.
fn points_ball<T: Real>(points: &[Point3<T>]) -> Ball<T> {
    let (lo, hi) = points[1..]
        .iter()
        .fold((points[0], points[0]), |(lo, hi), &p| {
            (lo.min(p), hi.max(p))
        });
    let c = lo + (hi - lo) * T::from_f64(0.5);
    let r = points.iter().fold(T::zero(), |r, &p| r.max((p - c).norm()));
    (c, r)
}
