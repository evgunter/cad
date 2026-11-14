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
//!   ([`AnalyticRow::lattice_steps`]: rows every `lever·h_u` on a ruled
//!   chart, square in 3-D at the face's widest radius; periodic steps
//!   capped at `sizing::MAX_ANGULAR_STEP`) and the plane the trimmed
//!   lane's triangulation runs in: the chart in its **metric**
//!   ([`AnalyticRow::metric`]), or on the cone its **development**
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
//! row sits in the chart's `v` domain ([`RowEnd`]). A cylinder, a torus
//! and a spline chart have none.
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
use geom_core::Point3;
use geom_core::predicate::Band;
use topo::{Body, FaceKey};

use crate::cert;
use crate::nurbs_cert::{FaceBounds, NurbsCellGrid, face_cells, lattice_budget, lattice_metric};
use crate::sizing::{
    cap_angular, cone_grid_step, sagitta_step, sphere_grid_steps, torus_grid_steps,
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
    /// ([`Self::steps`]): on a ruled chart, rows every `lever·hu` metres,
    /// square in 3-D at the face's widest radius (the cylinder's radius,
    /// the cone's largest), which shape the triangulation and move no
    /// bound; on a torus and a spindle sheet both periodic steps capped at
    /// `sizing::MAX_ANGULAR_STEP`; on a sphere the steps as they are, both
    /// capped already.
    pub(crate) fn lattice_steps(&self, (hu, hv): (f64, f64)) -> (f64, f64) {
        match self.kind() {
            Analytic::Cylinder { radius: lever } | Analytic::Cone { rho_max: lever, .. } => {
                (hu, lever * hu)
            }
            Analytic::Torus { .. } | Analytic::Spindle { .. } => (cap_angular(hu), cap_angular(hv)),
            Analytic::Sphere { .. } => (hu, hv),
        }
    }

    /// **The metric the trimmed lane's triangulation runs in**: the
    /// lattice metric `k` (`nurbs_cert::lattice_metric`) of a uniform
    /// lattice of `nu × nv` cells over the chart box `(u0, u1) × (v0, v1)`.
    /// The sphere's is the chart's own at the box's mid latitude,
    /// `(u·cos v, v)` to the nearest power of two: there a small face's
    /// boundary circle is near round, so an ear on it usually has the
    /// face's interior points in its circumcircle and the CDT does not cut
    /// it. Every other chart's is its lattice's realized spacing: a ruled
    /// lattice square in 3-D is a strip of aspect about `1/lever` in the
    /// chart, and a torus's is `√(r/(R + r))`.
    pub(crate) fn metric(
        &self,
        (u0, u1): (f64, f64),
        (v0, v1): (f64, f64),
        (nu, nv): (usize, usize),
    ) -> f64 {
        #[allow(clippy::cast_precision_loss)]
        let aspect = match self.kind() {
            Analytic::Sphere { .. } => 1.0 / geom::mid_param(v0, v1).cos(),
            Analytic::Cylinder { .. }
            | Analytic::Cone { .. }
            | Analytic::Torus { .. }
            | Analytic::Spindle { .. } => ((u1 - u0) / nu as f64) / ((v1 - v0) / nv as f64),
        };
        lattice_metric(std::iter::once(aspect))
    }

    /// The certificate of one triangle ([`ChartRow::cert`]).
    pub(crate) fn cert(
        &self,
        tri: [Point3<f64>; 3],
        uv: [[f64; 2]; 3],
        singular: [bool; 3],
    ) -> (f64, f64) {
        match *self.surface {
            Surface::Cylinder {
                origin,
                axis,
                radius,
                ..
            } => (
                cert::cert_cylinder(origin, axis, radius, tri),
                cert::position_scale(origin, tri),
            ),
            Surface::Sphere { center, radius, .. } => (
                cert::cert_sphere(center, radius, tri),
                cert::position_scale(center, tri),
            ),
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
    /// 3-D distance at `band`). An escalation reads off, as the curved
    /// lane's walk reads it (`walk`'s `singular_point_at`): a vertex the
    /// loop's lift could not decide leaves the loop with no lift, and the
    /// face refuses before this is read.
    pub(crate) fn is_singular(&self, p: Point3<f64>, band: Band) -> bool {
        self.singular.iter().any(|s| {
            geom_brep::chart_singularity::vertex_at_singular_point(p, s.point, band) == Ok(true)
        })
    }

    /// The face's outward normal at the foot of `p` on its chart, from the
    /// chart's implicit form (`geom_brep::implicit_outward_normal`), folded
    /// through the face's `sense`; poison where the foot is not one point.
    pub(crate) fn outward(&self, sense: bool, p: Point3<f64>) -> OutwardNormal<f64> {
        geom_brep::implicit_outward_normal(self.surface, sense, p)
    }

    /// **Whether the trimmed lane's arm on this chart chords its boundary
    /// to its lattice** (`chords::tighten`): every arm's but the
    /// cylinder's, which chords at its target alone
    /// (`work/tesslane/the-trimmed-cylinder-arm-chords-its-boundary-to-no-lattice.md`).
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
            apex: origin,
            axis,
            half_angle: 0.4,
            u_ref,
        };
        let torus = Surface::Torus {
            center: origin,
            axis,
            major_radius: 2.0,
            minor_radius: 0.6,
            u_ref,
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
                    origin: o,
                    axis: z,
                    radius: 1.0,
                    u_ref: x,
                },
                Analytic::Cylinder { radius: 1.0 },
                foot,
                [Point3::new(0.6, 0.3, 0.2), Point3::new(1.2, 0.6, 0.4)],
            ),
            (
                "sphere",
                Surface::Sphere {
                    center: o,
                    radius: 1.0,
                    axis: z,
                    u_ref: x,
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
                    apex: o,
                    axis: z,
                    half_angle: core::f64::consts::FRAC_PI_4,
                    u_ref: x,
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
                    center: o,
                    axis: z,
                    major_radius: 1.0,
                    minor_radius: 0.5,
                    u_ref: x,
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
