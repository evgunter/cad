//! **One row per chart**: what the mesh knows about the chart a curved
//! face is meshed on. A row is built once per face ([`ChartRow::of`])
//! and every lane and the chord pass read it:
//!
//! - its **steps** for a deviation target ([`ChartRow::steps`]): on each
//!   chart axis, the step the chart's cell bound needs, `+∞` along a
//!   ruling (a cylinder's or a cone's `v`), where no cell bound reads the
//!   extent. The chord pass's one rule (`chords::tighten`) holds every
//!   boundary chord to one step; the curved and trimmed lanes lay their
//!   lattices from the same steps;
//! - the per-triangle **certificate** ([`ChartRow::cert`], `crate::cert`
//!   and the spline cell table), with the magnitude it rounds at;
//! - its **singular rows** ([`ChartRow::singular_rows`]): the rows of the
//!   chart where a whole `u` line is one point (below);
//! - its **outward normal** at a point ([`ChartRow::outward`]).
//!
//! A row carries the face's own window where a chart's steps read one: a
//! cone's largest radius and a sphere's latitude nearest the equator
//! ([`ChartRow::of`]). The torus's steps read no window
//! (`sizing::torus_grid_steps` says why), and the cylinder's and the
//! spline's need none.
//!
//! # The singular-row rule
//!
//! A singular row is a value `v = v_s` of the chart where the whole line
//! `u ↦ S(u, v_s)` is one point: a sphere's poles at `v = ±π/2`, a cone's
//! apex at `v = 0`, a spindle sheet's apexes at `v = ±v*`. A cylinder, a
//! torus and a spline chart have none. A boundary loop that passes the
//! point arrives along one `u` and leaves along another, and its lift
//! (`topo::Body::loop_lift`) runs along the row between them through the
//! face's turn there, so the trimmed lane's polygon keeps the point twice,
//! once at each `u`, and the row is a side of the polygon
//! (`trimmed::flat_points`). Every lane reads the row the same way:
//!
//! - a triangle with two corners on the point is degenerate in 3-D and
//!   dropped, so the triangles left at the point are a fan about it;
//! - a corner on a singular row has no azimuth, and its certificate reads
//!   the other corners' ([`ChartRow::cert`]'s `singular` flags);
//! - the lattice keeps off the one shape where that fan is not manifold,
//!   exactly one column strictly between the side's ends
//!   (`trimmed::pole_side_columns`, `curved::pole_columns`);
//! - a point read past the row, a side off the row turning more than
//!   `3π/2` in `u`, and a mesh vertex at two chart places off the row
//!   refuse typed (`trimmed::singular_row_shape`).
//!
//! A point is on a row when it is within `64ε` of it in the chart's own
//! `v` unit (radians on a sphere and a spindle sheet, slant metres on a
//! cone), or past it where the row ends the chart's `v` domain: a pole's
//! latitude is read from an `atan2` a few ulps off `±π/2`, and the cone's
//! apex row is read at `v == 0.0` exactly
//! (`lift_probes::a_lift_runs_along_the_apex_row_through_the_faces_turn`).
//! `64ε` is far inside the nearest a chord point off the row can sit at
//! any count `sizing::MAX_COUNT` admits.

use geom::Surface;
use geom_brep::OutwardNormal;
use geom_core::{Point3, Vec3};
use topo::{Body, FaceKey};

use crate::cert;
use crate::nurbs_cert::{FaceBounds, NurbsCellGrid, face_cells, lattice_budget};
use crate::sizing::{cone_grid_step, sagitta_step, sphere_grid_steps, torus_grid_steps};
use crate::types::TessellateError;

/// What the mesh knows about one face's chart (module docs). The spline
/// row borrows the face's certified cell table from the chord pass's
/// memo ([`FaceBounds`]).
pub(crate) enum ChartRow<'a> {
    /// A cylinder: `v` axial metres, ruled in `v`.
    Cylinder {
        origin: Point3<f64>,
        axis: Vec3<f64>,
        radius: f64,
    },
    /// A cone: `v` slant metres from the apex, ruled in `v`, with the
    /// face's largest radius.
    Cone { half_angle: f64, rho_max: f64 },
    /// A sphere: `v` latitude, with the cosine of the face's latitude
    /// nearest the equator.
    Sphere {
        center: Point3<f64>,
        radius: f64,
        widest: f64,
    },
    /// A ring torus: `v` the minor angle.
    Torus { major: f64, minor: f64 },
    /// A spindle sheet: the torus's chart with a signed major radius,
    /// closed in `v` between its apexes at `±apex`.
    Spindle { major: f64, minor: f64, apex: f64 },
    /// A described NURBS face, or an approximating surface meshed on its
    /// fit: the certified cell table.
    Spline { grid: &'a NurbsCellGrid },
}

/// Where a singular row sits in the chart's `v` domain: at its top end,
/// its bottom end, or inside it (a cone's apex, between its two nappes).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum RowEnd {
    Top,
    Bottom,
    Inside,
}

/// One singular row of a chart (module docs, the singular-row rule).
#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct SingularRow {
    /// The row's `v`.
    pub v: f64,
    /// Where it sits in the chart's `v` domain.
    pub end: RowEnd,
}

/// A point within this of a singular row, in the chart's `v` unit, is on
/// it (module docs).
const ON_ROW: f64 = 64.0 * f64::EPSILON;

impl SingularRow {
    /// Whether `v` is on this row: within [`ON_ROW`] of it, or past it
    /// where it ends the domain.
    pub(crate) fn holds(self, v: f64) -> bool {
        match self.end {
            RowEnd::Top => self.v - v <= ON_ROW,
            RowEnd::Bottom => v - self.v <= ON_ROW,
            RowEnd::Inside => (v - self.v).abs() <= ON_ROW,
        }
    }

    /// Whether `v` lies past this row's end of the domain by more than
    /// the rounding of the chart's own evaluation there, `4ε` relative:
    /// a point no image the lane reads can reach but through the row.
    /// A row inside the domain has no past.
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
    /// The row of face `fk`, on `surface`, reading its window from its
    /// outer loop ([`crate::sizing::cone_face_rho_max`],
    /// [`crate::sizing::sphere_face_widest_cos`]) and a spline face's cell
    /// table from `bounds`, which the chord pass has filled.
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
        surface: &Surface<f64>,
        bounds: &'a FaceBounds,
    ) -> Result<Self, TessellateError> {
        Ok(match *surface {
            Surface::Plane { .. } => {
                return Err(TessellateError::MissingEntity {
                    what: "a chart row for a plane",
                });
            }
            // The mvfs "no description yet" state (types docs). An
            // approximating surface has no such state: its fit is
            // certified by construction.
            Surface::Nurbs(ref payload) if payload.is_placeholder() => {
                return Err(TessellateError::UnsupportedSurface { face: fk });
            }
            Surface::Nurbs(_) | Surface::Approx(_) => Self::Spline {
                grid: face_cells(bounds, fk)?,
            },
            Surface::Cylinder {
                origin,
                axis,
                radius,
                ..
            } => Self::Cylinder {
                origin,
                axis,
                radius,
            },
            Surface::Cone { half_angle, .. } => Self::Cone {
                half_angle,
                rho_max: crate::sizing::cone_face_rho_max(body, fk, half_angle)?,
            },
            Surface::Sphere { center, radius, .. } => Self::Sphere {
                center,
                radius,
                widest: crate::sizing::sphere_face_widest_cos(body, fk, surface)?,
            },
            Surface::Torus {
                major_radius,
                minor_radius,
                ..
            } => Self::Torus {
                major: major_radius,
                minor: minor_radius,
            },
            Surface::Spindle {
                major_radius,
                minor_radius,
                ..
            } => Self::Spindle {
                major: major_radius,
                minor: minor_radius,
                apex: geom::sheet_apex(major_radius, minor_radius).v,
            },
        })
    }

    /// The steps `(h_u, h_v)` the chart's cell bound needs at deviation
    /// `target` (module docs): `+∞` along a ruling; uncapped, so a reader
    /// stepping a periodic coordinate caps them itself (why the chord pass
    /// does not is at [`torus_grid_steps`]). A spline face's are its
    /// whole-patch certificate's at the lane's lattice budget
    /// ([`lattice_budget`]); its lattice is laid per cell.
    pub(crate) fn steps(&self, target: f64) -> (f64, f64) {
        match *self {
            Self::Cylinder { radius, .. } => (sagitta_step(target, radius), f64::INFINITY),
            Self::Cone {
                half_angle,
                rho_max,
            } => (cone_grid_step(target, half_angle, rho_max), f64::INFINITY),
            Self::Sphere { radius, widest, .. } => sphere_grid_steps(target, radius, widest),
            Self::Torus { major, minor } | Self::Spindle { major, minor, .. } => {
                torus_grid_steps(target, major, minor)
            }
            Self::Spline { grid } => grid.patch().grid_steps(lattice_budget(target)),
        }
    }

    /// The certificate of one triangle, with its 3-D corners `tri`, its
    /// chart corners `uv` and which corners lie on a singular row, and
    /// the magnitude the bound rounds at (`cert::verdict`'s `scale`).
    pub(crate) fn cert(
        &self,
        tri: [Point3<f64>; 3],
        uv: [[f64; 2]; 3],
        singular: [bool; 3],
    ) -> (f64, f64) {
        match *self {
            Self::Cylinder {
                origin,
                axis,
                radius,
            } => (
                cert::cert_cylinder(origin, axis, radius, tri),
                cert::position_scale(origin, tri),
            ),
            Self::Sphere { center, radius, .. } => (
                cert::cert_sphere(center, radius, tri),
                cert::position_scale(center, tri),
            ),
            Self::Cone { half_angle, .. } => (
                cert::cert_cone(half_angle, uv, singular),
                cert::cone_scale(half_angle, uv),
            ),
            Self::Torus { major, minor } => {
                let bound = cert::cert_torus(major, minor, uv);
                (bound, cert::uv_scale(bound, uv))
            }
            Self::Spindle { major, minor, .. } => {
                let bound = cert::cert_spindle(major, minor, uv, singular);
                (bound, cert::uv_scale(bound, uv))
            }
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

    /// The chart's singular rows (module docs, the singular-row rule).
    pub(crate) fn singular_rows(&self) -> Vec<SingularRow> {
        use core::f64::consts::FRAC_PI_2;
        let row = |v, end| SingularRow { v, end };
        match *self {
            Self::Sphere { .. } => {
                vec![row(FRAC_PI_2, RowEnd::Top), row(-FRAC_PI_2, RowEnd::Bottom)]
            }
            Self::Cone { .. } => vec![row(0.0, RowEnd::Inside)],
            Self::Spindle { apex, .. } => vec![row(apex, RowEnd::Top), row(-apex, RowEnd::Bottom)],
            Self::Cylinder { .. } | Self::Torus { .. } | Self::Spline { .. } => Vec::new(),
        }
    }

    /// The face's outward normal at the foot of `p` on its chart, from the
    /// chart's implicit form (`geom_brep::implicit_outward_normal`), folded
    /// through the face's `sense`; poison where the foot is not one point.
    /// `None` on a spline chart, which has no implicit form.
    pub(crate) fn outward(
        &self,
        surface: &Surface<f64>,
        sense: bool,
        p: Point3<f64>,
    ) -> Option<OutwardNormal<f64>> {
        match *self {
            Self::Cylinder { .. }
            | Self::Cone { .. }
            | Self::Sphere { .. }
            | Self::Torus { .. }
            | Self::Spindle { .. } => Some(geom_brep::implicit_outward_normal(surface, sense, p)),
            Self::Spline { .. } => None,
        }
    }

    /// Whether the row is a spline chart's.
    pub(crate) fn is_spline(&self) -> bool {
        match *self {
            Self::Spline { .. } => true,
            Self::Cylinder { .. }
            | Self::Cone { .. }
            | Self::Sphere { .. }
            | Self::Torus { .. }
            | Self::Spindle { .. } => false,
        }
    }
}

/// Whether chart `v` is on any of `rows` ([`SingularRow::holds`]).
pub(crate) fn on_singular_row(rows: &[SingularRow], v: f64) -> bool {
    rows.iter().any(|r| r.holds(v))
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod tests {
    use super::ChartRow;
    use geom::Surface;
    use geom_core::{Point3, Vec3};

    /// **The row's singular rows are the chart's singular points.** For
    /// each analytic kind, the `v` of every singular row is one of the
    /// singular points `topo::chart::Chart::poles` reads off
    /// `geom_brep::chart_singularity` for the same surface, and every one
    /// of those is a row: a sphere's two poles, a cone's apex, a spindle
    /// sheet's two apexes, a horn's one cusp (listed once, at `π`, by the
    /// chart, and as both ends `±π` by the row), none on a cylinder or a
    /// ring torus. The trimmed lane asserts the empty half of this per
    /// face; this row holds both halves on every kind.
    #[test]
    fn the_rows_singular_rows_are_the_charts_poles() {
        let o = Point3::new(0.0, 0.0, 0.0);
        let (z, x) = (Vec3::new(0.0, 0.0, 1.0), Vec3::new(1.0, 0.0, 0.0));
        let spindle = |major: f64, minor: f64| Surface::Spindle {
            center: o,
            axis: z,
            major_radius: major,
            minor_radius: minor,
            u_ref: x,
        };
        let surfaces = [
            Surface::Cylinder {
                origin: o,
                axis: z,
                radius: 1.0,
                u_ref: x,
            },
            Surface::Cone {
                apex: o,
                axis: z,
                half_angle: 0.5,
                u_ref: x,
            },
            Surface::Sphere {
                center: o,
                radius: 1.0,
                axis: z,
                u_ref: x,
            },
            Surface::Torus {
                center: o,
                axis: z,
                major_radius: 2.0,
                minor_radius: 0.5,
                u_ref: x,
            },
            spindle(-0.5, 1.0),
            spindle(0.4, 1.0),
            spindle(1.0, 1.0),
        ];
        for surface in &surfaces {
            let row = match *surface {
                Surface::Cylinder {
                    origin,
                    axis,
                    radius,
                    ..
                } => ChartRow::Cylinder {
                    origin,
                    axis,
                    radius,
                },
                Surface::Cone { half_angle, .. } => ChartRow::Cone {
                    half_angle,
                    rho_max: 1.0,
                },
                Surface::Sphere { center, radius, .. } => ChartRow::Sphere {
                    center,
                    radius,
                    widest: 1.0,
                },
                Surface::Torus {
                    major_radius,
                    minor_radius,
                    ..
                } => ChartRow::Torus {
                    major: major_radius,
                    minor: minor_radius,
                },
                Surface::Spindle {
                    major_radius,
                    minor_radius,
                    ..
                } => ChartRow::Spindle {
                    major: major_radius,
                    minor: minor_radius,
                    apex: geom::sheet_apex(major_radius, minor_radius).v,
                },
                Surface::Plane { .. } | Surface::Nurbs(_) | Surface::Approx(_) => {
                    unreachable!("the rows above are analytic surfaces")
                }
            };
            let rows: Vec<f64> = row.singular_rows().iter().map(|r| r.v).collect();
            let poles: Vec<f64> = topo::chart::Chart::of(surface)
                .expect("an analytic chart")
                .poles()
                .iter()
                .map(|&(_, v)| v)
                .collect();
            #[allow(clippy::float_cmp)]
            let holds = |of: &[f64], v: f64| {
                of.iter()
                    .any(|&w| w == v || w == -v && w.abs() == core::f64::consts::PI)
            };
            assert!(
                rows.iter().all(|&v| holds(&poles, v)) && poles.iter().all(|&v| holds(&rows, v)),
                "{:?}: the row's singular rows {rows:?} against the chart's poles {poles:?}",
                surface.kind()
            );
        }
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
        type Case<'a> = (&'a str, Surface<f64>, ChartRow<'a>, From, [Point3<f64>; 2]);
        let rows: [Case<'_>; 4] = [
            (
                "cylinder",
                Surface::Cylinder {
                    origin: o,
                    axis: z,
                    radius: 1.0,
                    u_ref: x,
                },
                ChartRow::Cylinder {
                    origin: o,
                    axis: z,
                    radius: 1.0,
                },
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
                ChartRow::Sphere {
                    center: o,
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
                ChartRow::Cone {
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
                ChartRow::Torus {
                    major: 1.0,
                    minor: 0.5,
                },
                core,
                [Point3::new(1.1, 0.0, 0.1), Point3::new(1.5, 0.4, 0.3)],
            ),
        ];
        for (name, surface, row, from, points) in &rows {
            for sense in [true, false] {
                for (side, p) in ["inside", "outside"].into_iter().zip(*points) {
                    let n = row
                        .outward(surface, sense, p)
                        .expect("an analytic row reads outward")
                        .vec();
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
