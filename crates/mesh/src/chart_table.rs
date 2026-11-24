//! **One row per chart**: what the mesh knows about the chart a curved
//! face is meshed on. A row is built once per face ([`ChartRow::of`]),
//! and the curved lane, the trimmed lane and the chord pass read it:
//!
//! - its **steps** for a deviation target ([`ChartRow::steps`]): on each
//!   chart axis, the step the chart's cell bound needs, `+∞` along a
//!   ruling (a cylinder's or a cone's `v`), where no cell bound reads the
//!   extent. The chord pass's one rule (`chords::tighten`) holds every
//!   boundary chord to one step;
//! - on an analytic chart, the **lattice** a lane lays from those steps
//!   ([`AnalyticRow::lattice_steps`]: rows every `ρ_max·h_u` on a cone,
//!   square in 3-D at the face's widest radius; periodic steps capped at
//!   `sizing::MAX_ANGULAR_STEP`), the rows the trimmed lane lays over a
//!   face's `v` window ([`AnalyticRow::lattice_rows`]: that step's, on the
//!   cone graded to the row's own radius, and none on the cylinder) and the
//!   plane the trimmed
//!   lane's triangulation runs in: the chart in its **metric**
//!   ([`AnalyticRow::trimmed_metric`]; the curved lane's is the lattice's
//!   own, [`AnalyticRow::cell_metric`]), or on the cone its **development**
//!   ([`AnalyticRow::development`]);
//! - the per-triangle **certificate** ([`ChartRow::cert`], `crate::cert`
//!   and the spline cell table), with the magnitude it rounds at;
//! - on an analytic chart, its **singular points** and the rows they are
//!   ([`AnalyticRow::singular_points`], below), its **outward normal**
//!   ([`AnalyticRow::outward`]) and the per-edge chord budget its arm
//!   holds ([`AnalyticRow::chord_budget`]).
//!
//! A row carries the face's own window where a chart's steps read one: a
//! cone's largest radius and a sphere's latitude nearest the equator
//! ([`ChartRow::of`]). The torus's steps read no window
//! (`sizing::torus_grid_steps` says why), and the cylinder's and the
//! spline's need none.
//!
//! # Singular points and the rows they are
//!
//! A chart's singular point is one point `S(u, v_s)` for every `u`: a
//! sphere's poles at `v = ±π/2`, a cone's apex at `v = 0`, a spindle
//! sheet's apexes at `v = ±v*` (a horn's one cusp, at `π`). The row reads
//! them from `topo::chart::Chart::poles`, their one home beneath which is
//! `geom_brep::chart_singularity`, and adds what is the mesh's: where the
//! row sits in the chart's `v` domain ([`RowEnd`]). A cylinder and a
//! torus have none. A spline chart's are its collapsed sides, which the
//! row does not carry: the trimmed lane reads them from the home
//! (`trimmed::CollapsedSides`).
//!
//! Whether a loop vertex IS a singular point is a fact about the body,
//! decided once, where the kernel decides it:
//! `geom_brep::chart_singularity::vertex_at_singular_point`, their 3-D
//! distance against the run's band ([`AnalyticRow::is_singular`]). It is
//! the decision the loop's lift takes when it turns the face there
//! (`topo::pcurves`' `singular_at`), so the trimmed lane reads the lift's
//! own structure and no threshold of its own, in a quantity that moves
//! with neither scale nor pose. The trimmed lane's rule at such a vertex
//! is `trimmed::singular_row_shape`'s. The curved lane meets the same
//! points through its walk (`walk`'s pole entries, read off
//! `Chart::poles` at `eps`).

use geom::Surface;
use geom_brep::OutwardNormal;
use geom_core::Indeterminate;
use geom_core::Point3;
use geom_core::predicate::Band;
use topo::{Body, FaceKey};

use crate::cert;
use crate::nurbs_cert::{FaceBounds, NurbsCellGrid, face_cells, lattice_budget, lattice_metric};
use crate::sizing::{
    cap_angular, ceil_count, cone_grid_step, sagitta_step, sphere_grid_steps, torus_grid_steps,
};
use crate::types::TessellateError;

/// What the mesh knows about one face's chart (module docs). The spline
/// row borrows the face's certified cell table from the chord pass's
/// memo ([`FaceBounds`]).
pub(crate) enum ChartRow<'a> {
    /// A cylinder, cone, sphere, torus or spindle sheet.
    Analytic(AnalyticRow<'a>),
    /// A described NURBS face, or an approximating surface meshed on its
    /// fit: the certified cell table.
    Spline { grid: &'a NurbsCellGrid },
}

/// An analytic chart's row: its surface, the face's window on it, and
/// its singular points. What the steps and the certificate read is the
/// surface's own data ([`Self::kind`]), stored once, so a row's kind and
/// its surface cannot disagree.
pub(crate) struct AnalyticRow<'a> {
    surface: &'a Surface<f64>,
    /// The face's window where the chart's steps read one: a cone face's
    /// largest radius, a sphere face's latitude cosine nearest the
    /// equator; unread on every other chart ([`Self::kind`]).
    window: f64,
    singular: Vec<SingularPoint>,
}

/// **Whether `surface` is ruled in its chart's `v`**: a cylinder or a
/// cone, whose lines `u = const` are straight rulings, so a line edge of
/// one of its faces is one of them and a lattice needs no step along it
/// ([`AnalyticRow::steps`]'s `hv = ∞`).
pub(crate) fn ruled(surface: &Surface<f64>) -> bool {
    matches!(surface, Surface::Cylinder { .. } | Surface::Cone { .. })
}

/// The analytic charts, each with what its steps and certificate read.
#[derive(Clone, Copy, Debug)]
pub(crate) enum Analytic {
    /// A cylinder: `v` axial metres, ruled in `v`.
    Cylinder { radius: f64 },
    /// A cone: `v` slant metres from the apex, ruled in `v`, with the
    /// face's largest radius.
    Cone { half_angle: f64, rho_max: f64 },
    /// A sphere: `v` latitude, with the cosine of the face's latitude
    /// nearest the equator.
    Sphere { radius: f64, widest: f64 },
    /// A ring torus: `v` the minor angle.
    Torus { major: f64, minor: f64 },
    /// A spindle sheet: the torus's chart with a signed major radius.
    Spindle { major: f64, minor: f64 },
}

/// Where a singular point's row sits in the chart's `v` domain: at its
/// top end, its bottom end, or inside it (a cone's apex, between its two
/// nappes).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum RowEnd {
    Top,
    Bottom,
    Inside,
}

/// One singular point of a chart (module docs).
#[derive(Clone, Copy, Debug)]
pub(crate) struct SingularPoint {
    /// The point in model space.
    pub point: Point3<f64>,
    /// The row's `v`.
    pub v: f64,
    /// Where the row sits in the chart's `v` domain.
    pub end: RowEnd,
}

impl SingularPoint {
    /// Whether chart `v` lies past this row's end of the domain by more
    /// than the rounding of the chart's own evaluation there, `4ε`
    /// relative to the row's angle: a point no image the lane reads can
    /// reach but through the row. A row inside the domain has no past.
    /// Read on a sphere's and a spindle sheet's `v`, angles both.
    pub(crate) fn passed_by(self, v: f64) -> bool {
        let reach = self.v.abs() * (1.0 + 4.0 * f64::EPSILON);
        match self.end {
            RowEnd::Top => v > reach,
            RowEnd::Bottom => v < -reach,
            RowEnd::Inside => false,
        }
    }
}

impl<'a> ChartRow<'a> {
    /// The row of face `fk`, on `surface`: an analytic chart's
    /// ([`AnalyticRow::of`]), or a spline face's cell table from `bounds`,
    /// which the chord pass has filled.
    ///
    /// # Errors
    ///
    /// [`TessellateError::UnsupportedSurface`] for a NURBS face with no
    /// description yet; [`TessellateError::MissingEntity`] for a plane,
    /// which has no chart row, and for a spline face whose cell table the
    /// chord pass did not leave; the window readers' errors.
    pub(crate) fn of(
        body: &Body<f64>,
        fk: FaceKey,
        surface: &'a Surface<f64>,
        bounds: &'a FaceBounds,
    ) -> Result<Self, TessellateError> {
        match *surface {
            // The mvfs "no description yet" state (types docs). An
            // approximating surface has no such state: its fit is
            // certified by construction.
            Surface::Nurbs(ref payload) if payload.is_placeholder() => {
                Err(TessellateError::UnsupportedSurface { face: fk })
            }
            Surface::Nurbs(_) | Surface::Approx(_) => Ok(Self::Spline {
                grid: face_cells(bounds, fk)?,
            }),
            Surface::Plane { .. }
            | Surface::Cylinder { .. }
            | Surface::Cone { .. }
            | Surface::Sphere { .. }
            | Surface::Torus { .. }
            | Surface::Spindle { .. } => Ok(Self::Analytic(AnalyticRow::of(body, fk, surface)?)),
        }
    }

    /// The steps `(h_u, h_v)` the chart's cell bound needs at deviation
    /// `target` (module docs): `+∞` along a ruling; uncapped, so a reader
    /// stepping a periodic coordinate caps them itself (why the chord pass
    /// does not is at [`torus_grid_steps`]). A spline face's are its
    /// whole-patch certificate's at the lane's lattice budget
    /// ([`lattice_budget`]); its lattice is laid per cell.
    pub(crate) fn steps(&self, target: f64) -> (f64, f64) {
        match self {
            Self::Analytic(row) => row.steps(target),
            Self::Spline { grid } => grid.patch().grid_steps(lattice_budget(target)),
        }
    }

    /// The certificate of one triangle, with its 3-D corners `tri`, its
    /// chart corners `uv` and which corners are a singular point, and the
    /// magnitude the bound rounds at (`cert::verdict`'s `scale`).
    pub(crate) fn cert(
        &self,
        tri: [Point3<f64>; 3],
        uv: [[f64; 2]; 3],
        singular: [bool; 3],
    ) -> (f64, f64) {
        match self {
            Self::Analytic(row) => row.cert(tri, uv, singular),
            // The cell(s) covering the triangle's chart box: one band's
            // cells in `v`, since rows land on the `v` cuts, but possibly
            // several in `u`, whose columns do not (`NurbsCellGrid::cert`
            // argues the straddling and half-open boundary cases).
            Self::Spline { grid } => {
                let bound = grid.cert(uv);
                (bound, cert::uv_scale(bound, uv))
            }
        }
    }
}

impl<'a> AnalyticRow<'a> {
    /// The row of face `fk` on the analytic `surface`, reading its window
    /// from its outer loop ([`crate::sizing::cone_face_rho_max`],
    /// [`crate::sizing::sphere_face_widest_cos`]) and its singular points
    /// from its chart.
    ///
    /// # Errors
    ///
    /// [`TessellateError::MissingEntity`] for a plane or a spline surface,
    /// which have no analytic row; the window readers' errors.
    pub(crate) fn of(
        body: &Body<f64>,
        fk: FaceKey,
        surface: &'a Surface<f64>,
    ) -> Result<Self, TessellateError> {
        let window = match *surface {
            Surface::Plane { .. } | Surface::Nurbs(_) | Surface::Approx(_) => {
                return Err(TessellateError::MissingEntity {
                    what: "an analytic chart row for a plane or a spline surface",
                });
            }
            Surface::Cone { half_angle, .. } => {
                crate::sizing::cone_face_rho_max(body, fk, half_angle)?
            }
            Surface::Sphere { .. } => crate::sizing::sphere_face_widest_cos(body, fk, surface)?,
            Surface::Cylinder { .. } | Surface::Torus { .. } | Surface::Spindle { .. } => f64::NAN,
        };
        Self::new(surface, window)
    }

    /// **The row of an analytic `surface` with the face's `window`** (a
    /// cone's largest radius, a sphere's widest latitude cosine; read on
    /// no other chart): its singular points from
    /// `topo::chart::Chart::poles`, each with where its row sits in the
    /// chart's `v` domain. [`Self::of`] reads the window off the face; a
    /// unit row hands one in.
    ///
    /// # Errors
    ///
    /// [`TessellateError::MissingEntity`] for a plane or a spline surface,
    /// which have no analytic row, so [`Self::kind`] and [`Self::cert`]
    /// meet only the analytic curved surfaces.
    pub(crate) fn new(surface: &'a Surface<f64>, window: f64) -> Result<Self, TessellateError> {
        if matches!(
            surface,
            Surface::Plane { .. } | Surface::Nurbs(_) | Surface::Approx(_)
        ) {
            return Err(TessellateError::MissingEntity {
                what: "an analytic chart row for a plane or a spline surface",
            });
        }
        let chart = topo::chart::Chart::of(surface).ok_or(TessellateError::MissingEntity {
            what: "an analytic surface's chart",
        })?;
        let singular = chart
            .poles()
            .into_iter()
            .map(|(point, v)| SingularPoint {
                point,
                v,
                end: if v > 0.0 {
                    RowEnd::Top
                } else if v < 0.0 {
                    RowEnd::Bottom
                } else {
                    RowEnd::Inside
                },
            })
            .collect();
        Ok(Self {
            surface,
            window,
            singular,
        })
    }

    /// **The chart's kind, with what its steps and certificate read**,
    /// from the surface's own data and the face's window.
    pub(crate) fn kind(&self) -> Analytic {
        match *self.surface {
            Surface::Cylinder { radius, .. } => Analytic::Cylinder { radius },
            Surface::Cone { half_angle, .. } => Analytic::Cone {
                half_angle,
                rho_max: self.window,
            },
            Surface::Sphere { radius, .. } => Analytic::Sphere {
                radius,
                widest: self.window,
            },
            Surface::Torus {
                major_radius,
                minor_radius,
                ..
            } => Analytic::Torus {
                major: major_radius,
                minor: minor_radius,
            },
            Surface::Spindle {
                major_radius,
                minor_radius,
                ..
            } => Analytic::Spindle {
                major: major_radius,
                minor: minor_radius,
            },
            Surface::Plane { .. } | Surface::Nurbs(_) | Surface::Approx(_) => {
                unreachable!("AnalyticRow::new holds an analytic curved surface")
            }
        }
    }

    /// The steps the chart's cell bound needs at `target` ([`ChartRow::steps`]).
    pub(crate) fn steps(&self, target: f64) -> (f64, f64) {
        match self.kind() {
            Analytic::Cylinder { radius } => (sagitta_step(target, radius), f64::INFINITY),
            Analytic::Cone {
                half_angle,
                rho_max,
            } => (cone_grid_step(target, half_angle, rho_max), f64::INFINITY),
            Analytic::Sphere { radius, widest } => sphere_grid_steps(target, radius, widest),
            Analytic::Torus { major, minor } | Analytic::Spindle { major, minor } => {
                torus_grid_steps(target, major, minor)
            }
        }
    }

    /// The steps a lane's lattice lays from the cell steps `(hu, hv)`
    /// ([`Self::steps`]): on a cone, rows every `ρ_max·hu` metres, square
    /// in 3-D at the face's largest radius, which shape the triangulation
    /// and move no bound; on a cylinder none (the steps as they are, `hv`
    /// unbounded: [`Self::lattice_rows`] lays no row there); on a torus
    /// and a spindle sheet both periodic steps capped at
    /// `sizing::MAX_ANGULAR_STEP`; on a sphere the steps as they are, both
    /// capped already.
    ///
    /// A cone's row step has one reader: the trimmed lane lays
    /// [`Self::lattice_rows`] from it, graded. The curved lane lays no
    /// rows on a ruled chart (`curved::grid_counts`), and the trimmed lane
    /// lays none on a face it walks (`crate::ruled_walk`).
    pub(crate) fn lattice_steps(&self, (hu, hv): (f64, f64)) -> (f64, f64) {
        match self.kind() {
            Analytic::Cylinder { .. } => (hu, hv),
            Analytic::Cone { rho_max: lever, .. } => (hu, lever * hu),
            Analytic::Torus { .. } | Analytic::Spindle { .. } => (cap_angular(hu), cap_angular(hv)),
            Analytic::Sphere { .. } => (hu, hv),
        }
    }

    /// **The interior rows of the trimmed lane's lattice over the face's
    /// `v` window `(v0, v1)`**, from the lattice steps `(hu, hv)`
    /// ([`Self::lattice_steps`]), ascending.
    ///
    /// **The cylinder: none.** Argued: every line `u = const` is straight
    /// on the surface, so a triangle's deviation is set by its span round
    /// the axis (`cert::cert_cylinder`), which a row does not shorten,
    /// while rows multiply the triangles. With no row there is no lattice
    /// node, the lane triangulates the face's boundary points, and nothing
    /// bounds a triangle's span but where those points lie: a triangle
    /// joining two boundary features across free wall can be wider than
    /// the target's column, its certificate refuses, and the lane's
    /// correction (`trimmed::TrimmedFace::attempt`) puts a point inside it
    /// and triangulates again; `mesh`'s
    /// `rowless_cylinders::two_holes_across_a_column_and_a_half_mesh_by_the_correction`
    /// holds that. Measured, debug, against the rows this replaced: over
    /// the `mesh` suite (ε 10⁻⁹, 10⁻⁶ and 10⁻¹²) and `sweep`'s meshing
    /// modules (the default ε) no cylinder face newly refuses, and 1,751
    /// faces fall from 3.2 million triangles to 0.2 million; on notched
    /// and holed pipe walls a face takes 1.0 to 1.5 times the rectangle
    /// lane's triangles over the iso-rectangles it divides into, where the
    /// rows made 1.2 to 223 times.
    ///
    /// The sphere, the torus and the spindle sheet: uniform,
    /// `⌈(v1 − v0)/hv⌉` gaps.
    ///
    /// **The cone: uniform in `√|v|`**, in `⌈2·(√v₁ − √v₀)·√v_w/hv⌉`
    /// gaps, `v_w = hv/(hu·sin α)` the slant the row step was laid at (the
    /// face's widest). Two rows at roots `s` and `s + Δs` then stand
    /// `Δs·(2s + Δs) ≤ hv·√(v_m/v_w)` apart, `v_m` the slant midway
    /// between them in `s`: `hv` at the widest radius, shorter toward the
    /// apex, and at most twice the uniform count over a window.
    ///
    /// *What is proved.* A chord of 3-D length `L` from radius `ρ₁` to a
    /// larger one, `Δu` round the axis, sags at most
    /// `cos α·L²/(4ρ₁·(1 + |cos(Δu/2)|))` (`cert::cone_chord_sag`, with
    /// `L² ≥ 4ρ₁ρ₂·sin²(Δu/2)`). At `L = hv·√(ρ₁/ρ_max)` that is
    /// `cos α·ρ_max·hu²/(4·(1 + |cos(Δu/2)|))`: the lattice's own sag at
    /// its widest radius, to second order in `hu`, for a chord along the
    /// surface (`Δu → 0`), and up to twice it for one half a turn round.
    ///
    /// *What is the rule's choice.* That gap, the length of the chord the
    /// target allows along the surface at the gap's own radius, is laid
    /// between rows. Nothing proves a triangle beside the boundary has no
    /// side longer than a gap, or none far round the axis: the rows shape
    /// the triangulation, the certificate is the guarantee, and the arm
    /// keeps `trimmed::UNPROVED_HEADROOM`. Rows uniform in `v` lay that
    /// gap at the widest radius alone; beside a small end the first row
    /// then stands many such chords from the boundary, the triangulation
    /// joins it to boundary points far round the axis, and a face ending
    /// near its apex holds triangles truly past δ.
    ///
    /// **No row within the radius every chord certifies at.** A chord
    /// with an end at radius `ρ` sags under `2·cos α·ρ`
    /// (`cert::cone_chord_sag`: `sin² ≤ 1`, and the other radius is under
    /// the sum), so where that is within the lattice's own sag at its
    /// widest radius, a row adds triangles and moves no bound.
    ///
    /// # Errors
    ///
    /// `sizing::ceil_count`'s, for a count past `sizing::MAX_COUNT`.
    pub(crate) fn lattice_rows(
        &self,
        (v0, v1): (f64, f64),
        (hu, hv): (f64, f64),
    ) -> Result<Vec<f64>, TessellateError> {
        #[allow(clippy::cast_precision_loss)]
        let lerp = |a: f64, b: f64, j: usize, n: usize| a + (b - a) * (j as f64 / n as f64);
        match self.kind() {
            Analytic::Cone { half_angle, .. } => {
                // `√|v|` signed: one formula for a window on either nappe
                // (a face lies on one, and its window may end at the apex).
                let root = |v: f64| v.signum() * v.abs().sqrt();
                let (s0, s1) = (root(v0), root(v1));
                let widest = hv / (hu * half_angle.sin());
                let n = ceil_count(2.0 * (s1 - s0) * widest.sqrt(), hv)?;
                // `2·cos α·ρ ≤ cos α·ρ_max·(1 − cos(hu/2))`, in slants.
                let certified = 0.5 * cert::cone_chord_sag(1.0, widest, widest, hu);
                Ok((1..n)
                    .map(|j| {
                        let s = lerp(s0, s1, j, n);
                        s * s.abs()
                    })
                    .filter(|v| v.abs() > certified)
                    .collect())
            }
            Analytic::Cylinder { .. } => Ok(Vec::new()),
            Analytic::Sphere { .. } | Analytic::Torus { .. } | Analytic::Spindle { .. } => {
                let n = ceil_count(v1 - v0, hv)?;
                Ok((1..n).map(|j| lerp(v0, v1, j, n)).collect())
            }
        }
    }

    /// **The lattice's own metric**: the lattice metric `k`
    /// (`nurbs_cert::lattice_metric`) of the realized cells of a uniform
    /// lattice of `nu × nv` cells over the chart box `(u0, u1) × (v0, v1)`,
    /// `(Δu/nu)/(Δv/nv)` to the nearest power of two, on every chart. A
    /// cone's lattice square in 3-D is a strip of aspect about `1/ρ_max` in
    /// the chart, a torus's is `√(r/(R + r))`, and a sphere's is within
    /// `2√2` of one (`sizing::sphere_grid_steps`). The curved lane's CDT
    /// runs in it. A cylinder lays no row, so the trimmed lane reads this
    /// at `nv = 1` and its aspect is the column step over the window's
    /// whole height ([`Self::trimmed_metric`] says what that plane is
    /// for).
    pub(crate) fn cell_metric(
        &self,
        (u0, u1): (f64, f64),
        (v0, v1): (f64, f64),
        (nu, nv): (usize, usize),
    ) -> f64 {
        #[allow(clippy::cast_precision_loss)]
        let aspect = ((u1 - u0) / nu as f64) / ((v1 - v0) / nv as f64);
        lattice_metric(std::iter::once(aspect))
    }

    /// **The metric the trimmed lane's triangulation runs in**: the
    /// lattice's own ([`Self::cell_metric`]) on every chart but the
    /// sphere, whose is the chart's own at the box's mid latitude,
    /// `(u·cos v, v)` to the nearest power of two: there a small face's
    /// boundary circle is near round, so a triangle on its points alone
    /// (an ear, or a side across several chord steps) usually has the
    /// face's interior points in its circumcircle and the CDT does not
    /// make it, which leaves the join fewer sides two faces both make
    /// (`crate::join`).
    ///
    /// **On a cylinder** the lane lays no row ([`Self::lattice_rows`]), so
    /// it reads this at `nv = 1`: the aspect is the column step over the
    /// window's whole height, points enter the triangulation at `(u/k, v)`,
    /// a column the height of the window reads as a square cell, and a
    /// triangle across one column from rim to rim is as well shaped there
    /// as any — so the triangulation prefers strips joining the two rims a
    /// column at a time, which is the shape whose span round the axis the
    /// target bounds. `k` is rounded to a power of two, so the plane
    /// changes in steps of two as the window's height grows.
    ///
    /// **Measured against the two alternatives** (debug, the correction
    /// on, the fuzz seed pinned; `mesh`, `sweep`'s meshing modules, and
    /// 900 two-hole cells over gap 1.0 to 3.0 columns): the cylinder's own
    /// development, `k = 1/r`, which is an isometry, and the metric this
    /// lane read when it laid rows, whose rows were square in 3-D and
    /// whose `k` is therefore `1/r` too up to the rounding. They are one
    /// alternative, and both lose. The cells are `r = 1` at δ/r 10⁻¹, 10⁻²
    /// and 10⁻³ and `r = 10⁻³` and `10` at δ/r 10⁻², 180 each, and this
    /// plane meshes all 900 with 0 or 4 points inserted. Under either
    /// alternative the 180 at δ 10⁻³ refuse `CertificateExceeded` with the
    /// correction's budget spent (2.7·δ to 27.6·δ); the 540 at δ/r 10⁻²
    /// mesh with 47 to 177 points more, and the 180 at δ/r 10⁻¹ with 0 to
    /// 33 more; 16 `mesh` rows and 16 `sweep` rows are red. There a
    /// column is `1/r` of the window's height in the CDT's own
    /// coordinates, the face reads as a tall strip, and the triangulation
    /// prefers triangles spanning several columns — the shape whose span
    /// round the axis nothing bounds.
    pub(crate) fn trimmed_metric(
        &self,
        (u0, u1): (f64, f64),
        (v0, v1): (f64, f64),
        (nu, nv): (usize, usize),
    ) -> f64 {
        match self.kind() {
            Analytic::Sphere { .. } => {
                lattice_metric(std::iter::once(1.0 / geom::mid_param(v0, v1).cos()))
            }
            Analytic::Cylinder { .. }
            | Analytic::Cone { .. }
            | Analytic::Torus { .. }
            | Analytic::Spindle { .. } => self.cell_metric((u0, u1), (v0, v1), (nu, nv)),
        }
    }

    /// The certificate of one triangle ([`ChartRow::cert`]).
    pub(crate) fn cert(
        &self,
        tri: [Point3<f64>; 3],
        uv: [[f64; 2]; 3],
        singular: [bool; 3],
    ) -> (f64, f64) {
        match *self.surface {
            Surface::Cylinder { frame, radius, .. } => {
                let (origin, axis) = (frame.origin(), frame.axis());
                (
                    cert::cert_cylinder(origin, axis, radius, tri),
                    cert::position_scale(origin, tri),
                )
            }
            Surface::Sphere { frame, radius, .. } => {
                let center = frame.origin();
                (
                    cert::cert_sphere(center, radius, tri),
                    cert::position_scale(center, tri),
                )
            }
            Surface::Cone { half_angle, .. } => (
                cert::cert_cone(half_angle, uv, singular),
                cert::cone_scale(half_angle, uv),
            ),
            Surface::Torus {
                major_radius,
                minor_radius,
                ..
            } => {
                let bound = cert::cert_torus(major_radius, minor_radius, uv);
                (bound, cert::uv_scale(bound, uv))
            }
            Surface::Spindle {
                major_radius,
                minor_radius,
                ..
            } => {
                let bound = cert::cert_spindle(major_radius, minor_radius, uv, singular);
                (bound, cert::uv_scale(bound, uv))
            }
            Surface::Plane { .. } | Surface::Nurbs(_) | Surface::Approx(_) => {
                unreachable!("AnalyticRow::new holds an analytic curved surface")
            }
        }
    }

    /// The chart's singular points (module docs), none for a cylinder or a
    /// torus.
    pub(crate) fn singular_points(&self) -> &[SingularPoint] {
        &self.singular
    }

    /// **Whether the point `p` is one of the chart's singular points**,
    /// decided where the kernel decides it
    /// (`geom_brep::chart_singularity::vertex_at_singular_point`, their
    /// 3-D distance at `band`): `true` where it decides zero to one,
    /// `false` where it decides every one clear.
    ///
    /// # Errors
    ///
    /// The escalation where no point decides zero and one is in the band:
    /// a vertex the lane cannot place on or off the singular point at
    /// this run's band. The lift was minted at the body's band and turns
    /// the face at a vertex it decided on, so reading an escalation as
    /// "off" would put the polygon's sides where the lift's are not;
    /// `trimmed`'s `singular_vertices` refuses the face typed instead
    /// (`SphereTrimCause::SingularUndecided`).
    pub(crate) fn is_singular(&self, p: Point3<f64>, band: Band) -> Result<bool, Indeterminate> {
        at_a_singular_point(self.singular.iter().map(|s| s.point), p, band)
    }

    /// The face's outward normal at the foot of `p` on its chart, from the
    /// chart's implicit form (`geom_brep::implicit_outward_normal`), folded
    /// through the face's `sense`; poison where the foot is not one point.
    pub(crate) fn outward(&self, sense: bool, p: Point3<f64>) -> OutwardNormal<f64> {
        geom_brep::implicit_outward_normal(self.surface, sense, p)
    }

    /// **Whether the trimmed lane's arm on this chart chords its boundary
    /// to its lattice** (`chords::tighten`): whether the chord pass bounds
    /// a boundary edge's chart steps by the arm's steps ([`Self::steps`]),
    /// so its points fall no farther apart than the lattice's. Every arm's
    /// but the cylinder's, whose boundary is chorded at its target alone
    /// (`work/tesslane/the-trimmed-cylinder-arm-chords-its-boundary-to-no-lattice.md`).
    /// This is not [`Self::lattice_rows`]' "no row": it decides the
    /// boundary's points, not the interior's, held while the cylinder still
    /// laid rows, and is read by the chord pass, which runs before any
    /// window is known.
    pub(crate) fn chorded_to_lattice(&self) -> bool {
        match self.kind() {
            Analytic::Cylinder { .. } => false,
            Analytic::Cone { .. }
            | Analytic::Sphere { .. }
            | Analytic::Torus { .. }
            | Analytic::Spindle { .. } => true,
        }
    }

    /// **The per-edge chord budget of the arm on this chart**, where it
    /// holds one: the sphere's ([`crate::trimmed::MAX_SPHERE_EDGE_CHORDS`],
    /// a projected circle passing near a pole), none elsewhere.
    pub(crate) fn chord_budget(&self) -> Option<usize> {
        match self.kind() {
            Analytic::Sphere { .. } => Some(crate::trimmed::MAX_SPHERE_EDGE_CHORDS),
            Analytic::Cylinder { .. }
            | Analytic::Cone { .. }
            | Analytic::Torus { .. }
            | Analytic::Spindle { .. } => None,
        }
    }

    /// **The plane the trimmed lane's triangulation runs in, where it is
    /// not the chart**: on the cone, its development, answered as
    /// `sin α`, the factor that turns the chart's azimuth `u` into the
    /// development's angle (`v` is already the slant length, the
    /// development's radius). The cone is the one row whose surface is
    /// developable with a singular point the chart spreads over a row: a
    /// ruling is a straight line of the surface, so the apex and two
    /// points of one ruling are one line in 3-D, while the chart puts
    /// the apex copy anywhere along `v = 0` and a Delaunay triangulation
    /// of the chart can join them into a triangle of zero area. In the
    /// development the apex is one point and the ruling a ray from it,
    /// so the triangulation never can (`crate::trimmed`, "The
    /// development"). The sphere's meridians are arcs, the cylinder has
    /// no singular point, and the torus and spindle are not developable:
    /// none of them, nor the spline row, answers.
    pub(crate) fn development(&self) -> Option<f64> {
        match self.kind() {
            Analytic::Cone { half_angle, .. } => Some(<f64 as geom_core::Real>::sin(half_angle)),
            Analytic::Cylinder { .. }
            | Analytic::Sphere { .. }
            | Analytic::Torus { .. }
            | Analytic::Spindle { .. } => None,
        }
    }
}

/// **Whether `p` is one of the singular `points`** at `band`, decided in
/// their order: `true` where one is decided zero distance from `p`,
/// `false` where every one is decided clear, the escalation where none
/// is decided zero and one is in the band. The one read of a vertex on a
/// singular point, which [`AnalyticRow::is_singular`], [`at_a_pole`] and
/// the trimmed lane's read of a spline chart's collapsed sides
/// (`trimmed::CollapsedSides`) make.
pub(crate) fn at_a_singular_point(
    points: impl Iterator<Item = Point3<f64>>,
    p: Point3<f64>,
    band: Band,
) -> Result<bool, Indeterminate> {
    let mut undecided = None;
    for point in points {
        match geom_brep::chart_singularity::vertex_at_singular_point(p, point, band) {
            Ok(true) => return Ok(true),
            Ok(false) => {}
            Err(e) => undecided = Some(e),
        }
    }
    undecided.map_or(Ok(false), Err)
}

/// **Whether `p` is a singular point of `surface`'s chart** (a cone's
/// apex, a sphere's poles: `topo::chart::Chart::poles`), read as the lane
/// reads it ([`AnalyticRow::is_singular`]) without a row: a reader with no
/// face window at hand (the route's `trimmed::walkable`) asks this.
/// `None` where the surface has no analytic chart.
pub(crate) fn at_a_pole(
    surface: &Surface<f64>,
    p: Point3<f64>,
    band: Band,
) -> Option<Result<bool, Indeterminate>> {
    let poles = topo::chart::Chart::of(surface)?.poles();
    Some(at_a_singular_point(
        poles.into_iter().map(|(point, _)| point),
        p,
        band,
    ))
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod tests {
    use super::{Analytic, AnalyticRow};
    use geom::Surface;
    use geom_core::{Point3, Vec3};

    /// **The cone's and the torus's certificates hold their triangles'
    /// true deviation.** On triangles laid in each chart (spans from a
    /// lattice cell's to a wide one's, at several places, on a cone with
    /// and without its apex as a corner, on a ring torus across its inner
    /// and outer sides), the true distance from the surface, sampled on a
    /// 300-point barycentric grid of each triangle and read in the
    /// surface's closed form, is at most the row's certificate
    /// ([`AnalyticRow::cert`]). A certificate that under-reports, scaled
    /// by `0.01` or zeroed on either arm, reds here; the mesh rows cannot
    /// see that, since a lattice sized at its own steps keeps the true
    /// deviation under δ whatever the certificate says.
    #[test]
    fn the_cone_and_torus_certificates_hold_the_true_deviation() {
        let origin = Point3::new(0.3, -0.2, 0.7);
        let axis = Vec3::new(1.0, 2.0, 2.0) * (1.0 / 3.0);
        let u_ref = Vec3::new(2.0, -1.0, 0.0) * (1.0 / 5f64.sqrt());
        let cone = Surface::Cone {
            frame: geom::Frame::new(origin, axis, u_ref),
            half_angle: 0.4,
        };
        let torus = Surface::Torus {
            frame: geom::Frame::new(origin, axis, u_ref),
            major_radius: 2.0,
            minor_radius: 0.6,
        };
        let distance = |surface: &Surface<f64>, p: Point3<f64>| -> f64 {
            let split = |w: Vec3<f64>| {
                let h = w.dot(axis);
                (h, (w - axis * h).norm())
            };
            match *surface {
                Surface::Cone { half_angle, .. } => {
                    let (h, rho) = split(p - origin);
                    let (s, c) = (half_angle.sin(), half_angle.cos());
                    [1.0, -1.0]
                        .map(|sign| {
                            let t = (rho * s + h * sign * c).max(0.0);
                            (rho - t * s).hypot(h - t * sign * c)
                        })
                        .into_iter()
                        .fold(f64::INFINITY, f64::min)
                }
                Surface::Torus {
                    major_radius,
                    minor_radius,
                    ..
                } => {
                    let (h, rho) = split(p - origin);
                    ((rho - major_radius).hypot(h) - minor_radius).abs()
                }
                _ => unreachable!("the row's two surfaces"),
            }
        };
        let mut checked = 0;
        for (surface, vs) in [(&cone, [0.0, 0.3, 2.5]), (&torus, [-2.5, 0.2, 2.9])] {
            let row = AnalyticRow::new(surface, 3.0).unwrap();
            for u0 in [0.0, 1.3, 4.1] {
                for v0 in vs {
                    for (du, dv) in [(0.05, 0.05), (0.3, 0.1), (0.8, 0.5), (0.2, 1.2)] {
                        for flip in [false, true] {
                            let uv = if flip {
                                [[u0, v0], [u0 + du, v0 + dv], [u0, v0 + dv]]
                            } else {
                                [[u0, v0], [u0 + du, v0], [u0 + du, v0 + dv]]
                            };
                            let apex = uv
                                .map(|[_, v]| matches!(surface, Surface::Cone { .. }) && v == 0.0);
                            let tri = uv.map(|[u, v]| surface.eval(u, v));
                            let (bound, _) = row.cert(tri, uv, apex);
                            const N: usize = 23;
                            let mut worst = 0.0f64;
                            for i in 0..=N {
                                for j in 0..=N - i {
                                    #[allow(clippy::cast_precision_loss)]
                                    let (x, y) = (i as f64 / N as f64, j as f64 / N as f64);
                                    let z = 1.0 - x - y;
                                    let p = Point3::new(
                                        x * tri[0].x + y * tri[1].x + z * tri[2].x,
                                        x * tri[0].y + y * tri[1].y + z * tri[2].y,
                                        x * tri[0].z + y * tri[1].z + z * tri[2].z,
                                    );
                                    worst = worst.max(distance(surface, p));
                                }
                            }
                            assert!(
                                worst <= bound * (1.0 + 1e-9) + 1e-14,
                                "{:?} triangle {uv:?}: true deviation {worst:e} past its \
                                 certificate {bound:e}",
                                surface.kind()
                            );
                            checked += usize::from(worst > 1e-6);
                        }
                    }
                }
            }
        }
        assert!(
            checked > 100,
            "the triangles deviate measurably ({checked}), so a scaled certificate reds"
        );
    }

    /// **What the cone arm's rows hold** ([`AnalyticRow::lattice_rows`]),
    /// read by geometry and not by the rule's own arithmetic. On cones
    /// from a needle to flat, over windows from a frustum's to one starting
    /// `10⁻⁶` of the way up, on either nappe, with `T` the lattice's sag at
    /// its widest radius:
    ///
    /// - **a chord no longer than a gap sags within its share of `T`.**
    ///   From a row (or the window's end) at radius `ρ_a` to any point at
    ///   a larger radius within the 3-D length of the gap above it, `Δu`
    ///   round the axis, the chord's largest distance from the cone
    ///   (searched along the chord) is at most
    ///   `T·(ρ_m/ρ_a)·2/(1 + |cos(Δu/2)|)`, `ρ_m` the gap's mid-root
    ///   radius, times `(hu/4)²/sin²(hu/4)` (`cos α·ρ_max·hu²/8` over `T`,
    ///   under 1.3 %). Rows uniform in `v`, or half as many, red here;
    /// - **no coarser row set does**: the widest gap in `√v` is within one
    ///   gap in the count of `hv/(2·√v_w)`, and the gap at the widest
    ///   radius is the lattice step. Twice the rows red here;
    /// - **the floor is where every chord certifies.** No row lies at a
    ///   radius `ρ` with `2·cos α·ρ ≤ T`, the first kept row is the first
    ///   past it, and a chord from under that radius to anywhere sags
    ///   within `T`. A floor twice as high, or none, reds here;
    /// - the rows ascend strictly inside the window, and the other
    ///   nappe's are their mirror image.
    ///
    /// Every other arm's rows are uniform in `v`.
    #[test]
    fn the_cone_arms_rows_stand_a_certified_chord_apart() {
        let frame = (
            Point3::new(0.0, 0.0, 0.0),
            Vec3::new(0.0, 0.0, 1.0),
            Vec3::new(1.0, 0.0, 0.0),
        );
        // The largest distance from the cone along the chord between two
        // of its points, each `(u, v)`: a ternary search of a concave
        // function of the chord's parameter.
        let sag = |alpha: f64, a: (f64, f64), b: (f64, f64)| -> f64 {
            let (sin, cos) = alpha.sin_cos();
            let at = |(u, v): (f64, f64)| (v * sin * u.cos(), v * sin * u.sin(), v * cos);
            let (p, q) = (at(a), at(b));
            let off = |t: f64| {
                let (x, y, z) = (
                    p.0 + (q.0 - p.0) * t,
                    p.1 + (q.1 - p.1) * t,
                    p.2 + (q.2 - p.2) * t,
                );
                sin * z - cos * x.hypot(y)
            };
            let (mut lo, mut hi) = (0.0_f64, 1.0_f64);
            for _ in 0..70 {
                let (m1, m2) = (lo + (hi - lo) / 3.0, hi - (hi - lo) / 3.0);
                if off(m1) < off(m2) {
                    lo = m1;
                } else {
                    hi = m2;
                }
            }
            off(0.5 * (lo + hi))
        };
        let mut rng = test_utils::fuzz::start("cone_rows");
        let (mut gaps_read, mut under_floor) = (0usize, 0usize);
        for half_angle in [0.002, 0.4, core::f64::consts::FRAC_PI_4, 1.4] {
            let cone = Surface::Cone {
                frame: geom::Frame::new(frame.0, frame.1, frame.2),
                half_angle,
            };
            for (widest, low, target) in [
                (2.0_f64, 1.0, 1e-3),
                (2.0, 0.0, 1e-3),
                (1.5, 1.5e-6, 2.5e-5),
                (300.0, 0.02, 1e-2),
            ] {
                let what = format!("α {half_angle}, window ({low}, {widest}), target {target}");
                let (sin, cos) = half_angle.sin_cos();
                let row = AnalyticRow::new(&cone, widest * sin).unwrap();
                let (hu, hv) = row.lattice_steps(row.steps(target));
                let rows = row.lattice_rows((low, widest), (hu, hv)).unwrap();
                // The lattice's own sag at its widest radius: the rim
                // chord across one column there.
                let lattice_sag = sag(half_angle, (0.0, widest), (hu, widest));
                assert!(
                    rows.windows(2).all(|w| w[0] < w[1])
                        && rows.iter().all(|&v| v > low && v < widest),
                    "{what}: the rows ascend inside the window"
                );
                // `cos α·ρ_max·hu²/8` over that sag: the lemma's bound is
                // the first, 1.3 % above the second at the angular cap.
                let second_order = (0.25 * hu / (0.25 * hu).sin()).powi(2);
                let floor = 0.5 * lattice_sag / (sin * cos);
                assert!(
                    rows.iter().all(|&v| v > floor * (1.0 - 1e-9)),
                    "{what}: a row within the slant {floor:e} every chord certifies at"
                );
                let stops: Vec<f64> = core::iter::once(low)
                    .chain(rows.iter().copied())
                    .chain(core::iter::once(widest))
                    .collect();
                let root_step = hv / (2.0 * widest.sqrt());
                let mut widest_root_gap = 0.0_f64;
                for pair in stops.windows(2) {
                    let (below, above) = (pair[0], pair[1]);
                    if below <= floor {
                        // The gap the floor leaves: every chord from under
                        // it certifies.
                        for _ in 0..20 {
                            let from = (0.0, floor * rng.unit());
                            let to = (rng.range(-3.0, 3.0), widest * rng.unit());
                            let found = sag(half_angle, from, to);
                            assert!(
                                found <= lattice_sag * (1.0 + 1e-9),
                                "{what}: a chord from {from:?} to {to:?} sags {found:e}, past                                  {lattice_sag:e}"
                            );
                            under_floor += 1;
                        }
                        continue;
                    }
                    widest_root_gap = widest_root_gap.max(above.sqrt() - below.sqrt());
                    let gap = above - below;
                    let mid = (0.5 * (above.sqrt() + below.sqrt())).powi(2);
                    for _ in 0..3 {
                        // A point at a larger slant within the gap's 3-D
                        // length of the lower row's point.
                        let up = gap * rng.unit();
                        let reach = ((gap * gap - up * up).max(0.0)).sqrt();
                        let to_v = below + up;
                        let half = (0.5 * reach / (sin * (below * to_v).sqrt())).min(1.0);
                        let du = 2.0 * half.asin() * rng.unit();
                        let found = sag(half_angle, (0.0, below), (du, to_v));
                        let share = (mid / below) * 2.0 / (1.0 + (0.5 * du).cos().abs());
                        assert!(
                            found <= lattice_sag * share * second_order * (1.0 + 1e-9),
                            "{what}: from the row at {below:e}, a chord {du:e} round to                              {to_v:e} sags {found:e}, past {share} of {lattice_sag:e}"
                        );
                    }
                    gaps_read += 1;
                }
                if rows.len() > 3 {
                    #[allow(clippy::cast_precision_loss)]
                    let count = stops.len() as f64;
                    assert!(
                        widest_root_gap <= root_step * (1.0 + 1e-12)
                            && widest_root_gap >= root_step * (1.0 - 1.0 / (count - 2.0)),
                        "{what}: the widest gap in √v, {widest_root_gap:e}, is the step                          {root_step:e} less at most one gap's share"
                    );
                    let top = widest - rows[rows.len() - 1];
                    assert!(
                        top <= hv && top >= hv * (1.0 - 2.0 / (count - 2.0)),
                        "{what}: the gap at the widest radius, {top:e}, is the lattice                          step {hv:e}"
                    );
                    let before = (rows[0].sqrt() - widest_root_gap).max(0.0).powi(2);
                    assert!(
                        before <= low * (1.0 + 1e-9) || before <= floor,
                        "{what}: the first kept row {:e} is the first past the floor                          {floor:e}",
                        rows[0]
                    );
                }
                let mirrored = row.lattice_rows((-widest, -low), (hu, hv)).unwrap();
                assert!(
                    mirrored.len() == rows.len()
                        && mirrored
                            .iter()
                            .rev()
                            .zip(&rows)
                            .all(|(m, v)| (m + v).abs() <= 1e-12 * widest),
                    "{what}: the v < 0 nappe's rows are the mirror image"
                );
            }
        }
        assert!(
            gaps_read > 500 && under_floor > 100,
            "gaps ({gaps_read}) and chords from under the floor ({under_floor}) were read"
        );
        let cylinder = Surface::Cylinder {
            frame: geom::Frame::new(frame.0, frame.1, frame.2),
            radius: 1.0,
        };
        let row = AnalyticRow::new(&cylinder, f64::NAN).unwrap();
        assert_eq!(
            row.lattice_rows((1.0, 2.0), (0.1, 0.3)).unwrap(),
            Vec::<f64>::new(),
            "a cylinder lays no row"
        );
    }

    /// **The fold check reads the face's side through the bit.** On each
    /// analytic row, at a point off the carrier on either side of it,
    /// `outward` points away from the point it is measured from exactly when
    /// `sense` is set: the foot on the axis for a cylinder and a cone, the
    /// centre for a sphere, the centre of the meridian circle through the
    /// point for a torus.
    #[test]
    fn outward_points_off_the_axis_exactly_when_sense_is_set() {
        let o = Point3::new(0.0, 0.0, 0.0);
        let z = Vec3::new(0.0, 0.0, 1.0);
        let x = Vec3::new(1.0, 0.0, 0.0);
        let foot = |p: Point3<f64>| Point3::new(0.0, 0.0, p.z);
        let core = |p: Point3<f64>| {
            let rho = (p.x * p.x + p.y * p.y).sqrt();
            Point3::new(p.x / rho, p.y / rho, 0.0)
        };
        type From = fn(Point3<f64>) -> Point3<f64>;
        type Case = (&'static str, Surface<f64>, Analytic, From, [Point3<f64>; 2]);
        let cases: [Case; 4] = [
            (
                "cylinder",
                Surface::Cylinder {
                    frame: geom::Frame::new(o, z, x),
                    radius: 1.0,
                },
                Analytic::Cylinder { radius: 1.0 },
                foot,
                [Point3::new(0.6, 0.3, 0.2), Point3::new(1.2, 0.6, 0.4)],
            ),
            (
                "sphere",
                Surface::Sphere {
                    frame: geom::Frame::new(o, z, x),
                    radius: 1.0,
                },
                Analytic::Sphere {
                    radius: 1.0,
                    widest: 1.0,
                },
                |_| Point3::new(0.0, 0.0, 0.0),
                [Point3::new(0.6, 0.3, 0.2), Point3::new(1.2, 0.6, 0.4)],
            ),
            (
                "cone",
                Surface::Cone {
                    frame: geom::Frame::new(o, z, x),
                    half_angle: core::f64::consts::FRAC_PI_4,
                },
                Analytic::Cone {
                    half_angle: core::f64::consts::FRAC_PI_4,
                    rho_max: 1.0,
                },
                foot,
                [Point3::new(0.2, 0.1, 0.5), Point3::new(0.6, 0.3, 0.4)],
            ),
            (
                "torus",
                Surface::Torus {
                    frame: geom::Frame::new(o, z, x),
                    major_radius: 1.0,
                    minor_radius: 0.5,
                },
                Analytic::Torus {
                    major: 1.0,
                    minor: 0.5,
                },
                core,
                [Point3::new(1.1, 0.0, 0.1), Point3::new(1.5, 0.4, 0.3)],
            ),
        ];
        for (name, surface, kind, from, points) in &cases {
            let row = AnalyticRow::new(surface, 1.0).unwrap();
            assert_eq!(
                std::mem::discriminant(&row.kind()),
                std::mem::discriminant(kind),
                "{name}: the row's kind is its surface's"
            );
            for sense in [true, false] {
                for (side, p) in ["inside", "outside"].into_iter().zip(*points) {
                    let n = row.outward(sense, p).vec();
                    assert_eq!(
                        n.dot(p - from(p)) > 0.0,
                        sense,
                        "{name}, sense {sense}, {side} the carrier: outward {n:?} must point \
                         away from the axis exactly when sense is set"
                    );
                }
            }
        }
    }
}
