//! **The analytic chart of a curved surface** — the closed forms that
//! invert a point into `(u, v)` and convert a chart-coordinate
//! discrepancy into metres.
//!
//! Two consumers, and the type is here so that they run the SAME
//! expressions rather than two copies of them:
//!
//! - `mesh`'s boundary walk, which inverts every junction and every
//!   chord point of a curved face's loop to build its UV polygon;
//! - [`crate::coherence`], which measures how far a body's own
//!   carriers and vertices disagree about those coordinates.
//!
//! **The expressions are the walk's, moved and not rewritten.** Every
//! method below is the one `mesh::walk` has always run, and there is
//! one copy of each. The CLASSIFICATION built on top of them — which
//! boundary edge is a rim and which a meridian, which of them carry
//! one iso side — is one copy too, next door in [`crate::chart_iso`].
//!
//! What is NOT here is each consumer's DISPOSITION of an answer: the
//! typed refusal `mesh` turns an unclassifiable carrier into, the band
//! spelling it reads a separation at, its rotation, its pole fan and
//! its emission; and this crate's own band and its report. A
//! disposition belongs to the crate that disposes. Those two seams are
//! named where they are crossed rather than left for a reader to
//! notice — `chart_iso`'s items say which half is theirs, and
//! `mesh::walk::tests::the_two_spellings_of_the_band_agree` is the
//! executed reconciliation of the one expression that genuinely could
//! not move (the band: `Eps` is `mesh`-local by MESH-4's ruling and
//! has no accessor).
//!
//! `f64` alone, deliberately: these are float-path facts about `f64`
//! coordinates — `atan2`, `hypot`, `asin` on a clamped ratio — and the
//! question a discrepancy between two of them asks does not survive
//! translation to an interval scalar, where the two values would
//! overlap by construction.

use geom::Surface;
use geom_core::{Point3, Vec3};

/// A curved surface's chart data for inversion (everything but the
/// plane, which takes the planar path).
pub struct Chart {
    /// The revolution/extrusion axis.
    pub axis: Vec3<f64>,
    /// The seam direction (u = 0).
    pub u_ref: Vec3<f64>,
    /// `axis × u_ref` (the frame convention).
    pub v_ref: Vec3<f64>,
    /// A point on the axis (apex/center/origin).
    pub anchor: Point3<f64>,
    /// Kind-specific inversion data.
    pub kind: ChartKind,
}

/// The kind payload of a [`Chart`].
pub enum ChartKind {
    /// Cylinder of radius `r`; v = axial meters.
    Cylinder {
        /// The radius.
        r: f64,
    },
    /// Cone; v = slant meters from the apex.
    Cone {
        /// The half-angle α.
        half_angle: f64,
    },
    /// Sphere of radius `r`; v = latitude.
    Sphere {
        /// The radius.
        r: f64,
    },
    /// Torus; v = minor angle (periodic).
    Torus {
        /// The major radius R.
        major: f64,
        /// The minor radius r.
        minor: f64,
    },
}

impl Chart {
    /// Builds the chart for a curved surface (`None` for planes;
    /// `Nurbs` is refused upstream).
    pub fn of(surface: &Surface<f64>) -> Option<Chart> {
        match *surface {
            // Both spline kinds are refused upstream: the tessellator
            // meshes them through their own net, not through an
            // analytic chart.
            Surface::Plane { .. } | Surface::Nurbs(_) | Surface::Approx(_) => None,
            Surface::Cylinder {
                origin,
                axis,
                radius,
                u_ref,
            } => Some(Chart {
                axis,
                u_ref,
                v_ref: axis.cross(u_ref),
                anchor: origin,
                kind: ChartKind::Cylinder { r: radius },
            }),
            Surface::Cone {
                apex,
                axis,
                half_angle,
                u_ref,
            } => Some(Chart {
                axis,
                u_ref,
                v_ref: axis.cross(u_ref),
                anchor: apex,
                kind: ChartKind::Cone { half_angle },
            }),
            Surface::Sphere {
                center,
                radius,
                axis,
                u_ref,
            } => Some(Chart {
                axis,
                u_ref,
                v_ref: axis.cross(u_ref),
                anchor: center,
                kind: ChartKind::Sphere { r: radius },
            }),
            Surface::Torus {
                center,
                axis,
                major_radius,
                minor_radius,
                u_ref,
            } => Some(Chart {
                axis,
                u_ref,
                v_ref: axis.cross(u_ref),
                anchor: center,
                kind: ChartKind::Torus {
                    major: major_radius,
                    minor: minor_radius,
                },
            }),
        }
    }

    /// Raw azimuth of a point, in (−π, π].
    pub fn azimuth(&self, p: Point3<f64>) -> f64 {
        let w = p - self.anchor;
        w.dot(self.v_ref).atan2(w.dot(self.u_ref))
    }

    /// Distance of a point from the chart axis — the lever arm that
    /// converts an angular u-discrepancy into a spatial one (arc
    /// length `r·δu`). `(u_ref, v_ref, axis)` is orthonormal by
    /// construction (`v_ref = axis × u_ref`), so this is just the
    /// in-plane component's length.
    ///
    /// Kind-free on purpose: it reads the POINT, not the surface
    /// parameters, so the cone — whose radius varies along the loop as
    /// `v·sin α` rather than sitting in `ChartKind` — needs no special
    /// case, and a point on the axis correctly reports 0.
    pub fn radial(&self, p: Point3<f64>) -> f64 {
        let w = p - self.anchor;
        w.dot(self.u_ref).hypot(w.dot(self.v_ref))
    }

    /// Raw chart u of a point, in (−π, π] — the azimuth, **except on a
    /// cone's mirror nappe** (v < 0, i.e. below the apex along the
    /// axis), where `S(u, v) = apex + axis·(v cos α) + radial(u)·(v sin α)`
    /// has a *negative* radial coefficient: the point at chart u sits
    /// at spatial azimuth u + π, so inversion subtracts the π back.
    /// (Revolve places every surface with the shared +a₃ axis, so
    /// downward-opening cone walls live on the mirror nappe — PR 5.)
    pub fn u_of(&self, p: Point3<f64>) -> f64 {
        let az = self.azimuth(p);
        if matches!(self.kind, ChartKind::Cone { .. }) && (p - self.anchor).dot(self.axis) < 0.0 {
            if az > 0.0 {
                az - core::f64::consts::PI
            } else {
                az + core::f64::consts::PI
            }
        } else {
            az
        }
    }

    /// The non-periodic-or-raw v coordinate of a point.
    pub fn v_of(&self, p: Point3<f64>) -> f64 {
        let w = p - self.anchor;
        let h = w.dot(self.axis);
        match self.kind {
            ChartKind::Cylinder { .. } => h,
            ChartKind::Cone { half_angle } => h / half_angle.cos(),
            ChartKind::Sphere { r } => (h / r).clamp(-1.0, 1.0).asin(),
            ChartKind::Torus { major, .. } => {
                let rho = (w - self.axis * h).norm();
                h.atan2(rho - major)
            }
        }
    }

    /// The v counterpart of [`Self::radial`]: `|∂S/∂v|`, the length
    /// one unit of the chart's v coordinate displaces a point by.
    ///
    /// Constant per kind because v is either already a length
    /// (cylinder — axial metres; cone — slant metres, both
    /// `|∂S/∂v| = 1`) or an angle turning on a fixed radius (sphere —
    /// latitude on `r`; torus — minor angle on `r`). u needs the point
    /// because its lever arm is the *distance from the axis*, which
    /// varies over a cone and a sphere; v's does not, so this takes
    /// none.
    ///
    /// Together `(radial(p), v_lever())` convert a UV discrepancy into
    /// metres, which is the only honest unit to compare against ε — the
    /// argument `mesh`'s swept-rectangle domain guard makes, and the
    /// one [`crate::coherence`]'s band makes for the three coherence
    /// conditions — the gap is in chart units either way, and only the
    /// lever arm turns it into a length.
    pub fn v_lever(&self) -> f64 {
        match self.kind {
            ChartKind::Cylinder { .. } | ChartKind::Cone { .. } => 1.0,
            ChartKind::Sphere { r } => r,
            ChartKind::Torus { minor, .. } => minor,
        }
    }

    /// Whether v is a periodic coordinate (torus minor angle).
    pub fn v_periodic(&self) -> bool {
        matches!(self.kind, ChartKind::Torus { .. })
    }

    /// The chart's pole points with their v values (sphere poles, cone
    /// apex; empty otherwise).
    pub fn poles(&self) -> Vec<(Point3<f64>, f64)> {
        match self.kind {
            ChartKind::Sphere { r } => vec![
                (self.anchor + self.axis * r, core::f64::consts::FRAC_PI_2),
                (self.anchor - self.axis * r, -core::f64::consts::FRAC_PI_2),
            ],
            ChartKind::Cone { .. } => vec![(self.anchor, 0.0)],
            ChartKind::Cylinder { .. } | ChartKind::Torus { .. } => Vec::new(),
        }
    }

    /// The (raw) v of a rim from its carrier circle's center and
    /// radius — one value per EDGE, which two sub-edges of ONE carrier
    /// circle already agreed on bitwise (same center, same radius) but
    /// two DISTINCT `Circle` carriers stating the same analytic circle
    /// (what an exporter emits for a split rim) did not. That was the
    /// module header's "once per edge is not once per side" caveat,
    /// issue #653, on the v axis; the iso-side run rule closes it by
    /// giving the whole run the first edge's value, which is what makes
    /// the row bitwise straight whatever carries it —
    /// [`crate::coherence`] measures the value that discards, and
    /// `mesh`'s walk is what discards it.
    pub fn rim_v(&self, center: Point3<f64>, radius: f64) -> f64 {
        let h = (center - self.anchor).dot(self.axis);
        match self.kind {
            ChartKind::Cylinder { .. } => h,
            ChartKind::Cone { half_angle } => h / half_angle.cos(),
            ChartKind::Sphere { .. } => h.atan2(radius),
            ChartKind::Torus { major, .. } => h.atan2(radius - major),
        }
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod tests {
    use super::*;

    /// A chart about +z, anchored at the origin. `mesh`'s walk keeps
    /// its own copy of this three-line constructor for the rows that
    /// exercise the classification built on top of these forms; there
    /// is nothing to share but the `Chart` literal itself.
    fn z_chart(kind: ChartKind) -> Chart {
        let axis = Vec3::new(0.0, 0.0, 1.0);
        let u_ref = Vec3::new(1.0, 0.0, 0.0);
        Chart {
            axis,
            u_ref,
            v_ref: axis.cross(u_ref),
            anchor: Point3::new(0.0, 0.0, 0.0),
            kind,
        }
    }

    #[test]
    fn cone_mirror_nappe_u_of() {
        let chart = Chart {
            axis: Vec3::new(0.0, 1.0, 0.0),
            u_ref: Vec3::new(1.0, 0.0, 0.0),
            v_ref: Vec3::new(0.0, 1.0, 0.0).cross(Vec3::new(1.0, 0.0, 0.0)),
            anchor: Point3::new(0.0, 1.0, 0.0),
            kind: ChartKind::Cone {
                half_angle: core::f64::consts::FRAC_PI_4,
            },
        };
        let u = chart.u_of(Point3::new(0.5, 0.5, 0.0));
        assert!(
            (u - core::f64::consts::PI).abs() < 1e-12,
            "expected pi, got {u}"
        );
    }

    /// The lever arm is the distance from the axis, and sliding a point
    /// ALONG the axis does not change it.
    #[test]
    fn radial_is_the_distance_from_the_axis() {
        let c = z_chart(ChartKind::Cylinder { r: 2.0 });
        for z in [-10.0, 0.0, 7.5] {
            let d = c.radial(Point3::new(3.0, 4.0, z));
            assert!((d - 5.0).abs() < 1e-15, "expected 5, got {d} at z = {z}");
        }
    }

    /// `radial` reads the POINT, not `ChartKind` — which is what makes
    /// the cone work without a special case. A cone's radius is not in
    /// its kind payload at all (only the half-angle is); it varies along
    /// the loop, so one chart must yield different lever arms at
    /// different points. On a 45° cone from the origin, radius = height.
    #[test]
    fn radial_varies_along_a_cone_whose_kind_carries_no_radius() {
        let c = z_chart(ChartKind::Cone {
            half_angle: core::f64::consts::FRAC_PI_4,
        });
        for h in [3.0, 9.0] {
            let d = c.radial(Point3::new(0.0, h, h));
            assert!((d - h).abs() < 1e-15, "expected {h}, got {d}");
        }
    }
}
