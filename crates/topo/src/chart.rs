//! **The analytic chart of a curved surface** — a point's `(u, v)`,
//! and the lever arms that convert a chart-coordinate discrepancy into
//! metres.
//!
//! Three consumers read it: `mesh`'s boundary walk, which inverts
//! every junction and every chord point of a curved face's loop to
//! build its UV polygon; [`crate::coherence`], which measures how far a
//! body's own carriers and vertices disagree about those coordinates;
//! and `shell`'s rim nesting, generic over its scalar. The
//! CLASSIFICATION built on top of it — which boundary edge is a rim and
//! which a meridian, which of them carry one iso side — is next door in
//! [`crate::chart_iso`].
//!
//! **The coordinates are geom's reading** ([`geom::ChartInverse`]):
//! [`Chart::u_of`], [`Chart::v_of`] and [`Chart::rim_v`] read it and
//! spell no inversion of their own. So a point's angles are named in the
//! windows every placed pcurve image is named in (C4): the azimuth and a
//! torus's tube angle in `(−1, τ − 1]`, a sphere's latitude in its own.
//! They read it at [`Band::least`]: every decision the reading takes
//! for them is a choice between names of one point — the side of a
//! window's cut, which formula names the angle, and the nappe a point
//! on a cone's axis is read on, where both name it — and none of them
//! reads [`geom::ChartPoint::singular`].
//!
//! What is NOT here is each consumer's DISPOSITION of an answer: the
//! typed refusal `mesh` turns an unclassifiable carrier into, the band
//! spelling it reads a separation at, its rotation, its pole fan and
//! its emission; and this crate's own band and its report. A
//! disposition belongs to the crate that disposes. `chart_iso`'s items
//! say which half is theirs, and
//! `mesh::walk::tests::the_two_spellings_of_the_band_agree` is the
//! executed reconciliation of the band (`Eps` is `mesh`-local by
//! MESH-4's ruling and has no accessor).

use geom::{ChartArm, ChartInverse, Surface};
use geom_core::predicate::Band;
use geom_core::{Decide, Point2, Point3, Real, Vec3};

/// A curved surface's chart (everything but the plane, which takes the
/// planar path): geom's inversion, and the kind data its levers and
/// poles read.
pub struct Chart<T: Real = f64> {
    /// Kind-specific data.
    pub kind: ChartKind<T>,
    inverse: ChartInverse<T>,
}

/// The kind payload of a [`Chart`].
pub enum ChartKind<T: Real = f64> {
    /// Cylinder of radius `r`; v = axial meters.
    Cylinder {
        /// The radius.
        r: T,
    },
    /// Cone; v = slant meters from the apex.
    Cone {
        /// The half-angle α.
        half_angle: T,
    },
    /// Sphere of radius `r`; v = latitude.
    Sphere {
        /// The radius.
        r: T,
    },
    /// Torus; v = minor angle (periodic).
    Torus {
        /// The major radius R.
        major: T,
        /// The minor radius r.
        minor: T,
    },
}

impl<T: Real> Chart<T> {
    /// The chart of a curved surface. `None` for the plane, and for a
    /// kind with no closed inversion ([`geom::NoClosedInverse`]): the
    /// tessellator meshes a spline through its own net, not through an
    /// analytic chart.
    pub fn of(surface: &Surface<T>) -> Option<Self> {
        let inverse = ChartInverse::of(surface).ok()?;
        let kind = match inverse.arm() {
            ChartArm::Plane => return None,
            ChartArm::Cylinder { radius } => ChartKind::Cylinder { r: radius },
            ChartArm::Cone { half_angle } => ChartKind::Cone { half_angle },
            ChartArm::Sphere { radius } => ChartKind::Sphere { r: radius },
            ChartArm::Torus { major, minor } => ChartKind::Torus { major, minor },
        };
        Some(Self { kind, inverse })
    }

    /// The revolution/extrusion axis.
    pub fn axis(&self) -> Vec3<T> {
        self.inverse.axis()
    }

    /// The seam direction (u = 0).
    pub fn u_ref(&self) -> Vec3<T> {
        self.inverse.u_ref()
    }

    /// `axis × u_ref` (the frame convention).
    pub fn v_ref(&self) -> Vec3<T> {
        self.inverse.v_ref()
    }

    /// A point on the axis (apex/center/origin).
    pub fn anchor(&self) -> Point3<T> {
        self.inverse.anchor()
    }

    /// Distance of a point from the chart axis — the lever arm that
    /// converts an angular u-discrepancy into a spatial one (arc
    /// length `r·δu`). `(u_ref, v_ref, axis)` is orthonormal by
    /// construction, so this is the in-plane component's length.
    ///
    /// Kind-free on purpose: it reads the POINT, not the surface
    /// parameters, so the cone — whose radius varies along the loop as
    /// `v·sin α` rather than sitting in `ChartKind` — needs no special
    /// case, and a point on the axis correctly reports 0.
    pub fn radial(&self, p: Point3<T>) -> T {
        let w = p - self.anchor();
        (w.dot(self.u_ref()).powi(2) + w.dot(self.v_ref()).powi(2)).sqrt()
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
    /// metres, which is the only honest unit to compare against ε.
    pub fn v_lever(&self) -> T {
        match self.kind {
            ChartKind::Cylinder { .. } | ChartKind::Cone { .. } => T::one(),
            ChartKind::Sphere { r } => r,
            ChartKind::Torus { minor, .. } => minor,
        }
    }

    /// Whether v is a periodic coordinate (torus minor angle).
    pub fn v_periodic(&self) -> bool {
        matches!(self.kind, ChartKind::Torus { .. })
    }
}

impl<T: Decide> Chart<T> {
    /// The chart coordinates of a point ([`geom::ChartInverse::point`]).
    pub fn uv_of(&self, p: Point3<T>) -> Point2<T> {
        self.inverse.point(p, Band::least()).uv
    }

    /// The chart u of a point: [`Self::uv_of`]'s first coordinate.
    pub fn u_of(&self, p: Point3<T>) -> T {
        self.uv_of(p).x
    }

    /// The chart v of a point: [`Self::uv_of`]'s second coordinate.
    pub fn v_of(&self, p: Point3<T>) -> T {
        self.uv_of(p).y
    }

    /// The v of a rim from its carrier circle's center and radius: the
    /// v of the point at that radius from the axis, along `u_ref`, at
    /// the center's height — the circle read as the coaxial one its
    /// height and radius state, wherever its center lies. One value per
    /// EDGE, which two sub-edges of ONE carrier circle already agreed on
    /// bitwise (same center, same radius) but two DISTINCT `Circle`
    /// carriers stating the same analytic circle (what an exporter emits
    /// for a split rim) did not (issue #653); the iso-side run rule
    /// closes it by giving the whole run the first edge's value, which
    /// is what makes the row bitwise straight whatever carries it —
    /// [`crate::coherence`] measures the value that discards, and
    /// `mesh`'s walk is what discards it.
    pub fn rim_v(&self, center: Point3<T>, radius: T) -> T {
        let h = (center - self.anchor()).dot(self.axis());
        self.v_of(self.anchor() + self.axis() * h + self.u_ref() * radius)
    }
}

impl Chart<f64> {
    /// The chart's singular points with their v values: the list
    /// [`geom_brep::chart_singularity::singular_points`] states (sphere
    /// poles, cone apex; empty otherwise).
    pub fn poles(&self) -> Vec<(Point3<f64>, f64)> {
        use geom_brep::chart_singularity::Singularity;
        geom_brep::chart_singularity::singular_points(&self.inverse)
            .into_iter()
            .map(|(which, p)| {
                let v = match which {
                    Singularity::North => core::f64::consts::FRAC_PI_2,
                    Singularity::South => -core::f64::consts::FRAC_PI_2,
                    Singularity::Apex => 0.0,
                };
                (p, v)
            })
            .collect()
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod tests {
    use super::*;

    /// A chart about +z, anchored at the origin.
    fn z_chart(surface: impl FnOnce(Point3<f64>, Vec3<f64>, Vec3<f64>) -> Surface<f64>) -> Chart {
        let surface = surface(
            Point3::new(0.0, 0.0, 0.0),
            Vec3::new(0.0, 0.0, 1.0),
            Vec3::new(1.0, 0.0, 0.0),
        );
        Chart::of(&surface).unwrap()
    }

    #[test]
    fn cone_mirror_nappe_u_of() {
        let chart = Chart::of(&Surface::Cone {
            apex: Point3::new(0.0, 1.0, 0.0),
            axis: Vec3::new(0.0, 1.0, 0.0),
            half_angle: core::f64::consts::FRAC_PI_4,
            u_ref: Vec3::new(1.0, 0.0, 0.0),
        })
        .unwrap();
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
        let c = z_chart(|origin, axis, u_ref| Surface::Cylinder {
            origin,
            axis,
            radius: 2.0,
            u_ref,
        });
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
        let c = z_chart(|apex, axis, u_ref| Surface::Cone {
            apex,
            axis,
            half_angle: core::f64::consts::FRAC_PI_4,
            u_ref,
        });
        for h in [3.0, 9.0] {
            let d = c.radial(Point3::new(0.0, h, h));
            assert!((d - h).abs() < 1e-15, "expected {h}, got {d}");
        }
    }

    /// A rim's v is the v of its circle's points: on a sphere and a
    /// torus, a circle about the axis reads the latitude (tube angle) its
    /// points read.
    #[test]
    fn a_rim_reads_the_v_of_its_points() {
        let sphere = z_chart(|center, axis, u_ref| Surface::Sphere {
            center,
            radius: 2.0,
            axis,
            u_ref,
        });
        let torus = z_chart(|center, axis, u_ref| Surface::Torus {
            center,
            axis,
            major_radius: 3.0,
            minor_radius: 1.0,
            u_ref,
        });
        for (chart, center, radius, on) in [
            (
                &sphere,
                Point3::new(0.0, 0.0, 1.2),
                1.6,
                Point3::new(0.0, -1.6, 1.2),
            ),
            (
                &torus,
                Point3::new(0.0, 0.0, 0.6),
                2.2,
                Point3::new(-2.2, 0.0, 0.6),
            ),
        ] {
            let (rim, point) = (chart.rim_v(center, radius), chart.v_of(on));
            assert!(
                (rim - point).abs() < 1e-15,
                "rim v {rim} against its point's {point}"
            );
        }
    }
}
