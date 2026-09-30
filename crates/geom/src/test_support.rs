//! **One fixture of every analytic kind**, behind the `test-support`
//! feature (on only through dev-dependency edges), for the rows that
//! hold a field-by-field reader to the walks ([`Surface::data`],
//! [`Curve3::data`]).
//!
//! Each kind is a builder over a flat scalar list, laid out in its
//! variant's field order, beside the number of scalars it reads. The
//! kinds are read off the variant rosters ([`SurfaceVariant`],
//! [`Curve3Variant`]), so a variant added to [`Surface`] or [`Curve3`]
//! fails this module until it is given a builder or declared as not
//! read field by field.

use geom_core::{Point3, Vec3};
use strum::IntoEnumIterator;

pub use crate::curves::Curve3Variant;
use crate::datum::ANALYTIC_DATA_MAX;
pub use crate::surfaces::SurfaceVariant;
use crate::{Curve3, Surface};

/// One analytic kind: a builder over a flat scalar list, and how many
/// of the list's leading scalars it reads.
#[derive(Clone, Copy, Debug)]
pub struct AnalyticKind<K> {
    /// Builds the kind from `x`, reading `x[..scalars]` in field order.
    pub build: fn(&[f64]) -> K,
    /// How many scalars `build` reads.
    pub scalars: usize,
}

/// A list of distinct, finite, nonzero scalars long enough for any
/// kind's builder: `1.0, 2.0, …`.
#[must_use]
pub fn scalar_base() -> Vec<f64> {
    (1..=3 * ANALYTIC_DATA_MAX).map(|i| i as f64).collect()
}

fn pt(x: &[f64]) -> Point3<f64> {
    Point3::new(x[0], x[1], x[2])
}

fn dir(x: &[f64]) -> Vec3<f64> {
    Vec3::new(x[0], x[1], x[2])
}

/// Every analytic surface kind, in declaration order.
#[must_use]
pub fn analytic_surfaces() -> Vec<AnalyticKind<Surface<f64>>> {
    SurfaceVariant::iter()
        .filter_map(|kind| -> Option<AnalyticKind<Surface<f64>>> {
            Some(match kind {
                SurfaceVariant::Plane => AnalyticKind {
                    build: |x| Surface::Plane {
                        origin: pt(&x[0..3]),
                        normal: dir(&x[3..6]),
                        u_ref: dir(&x[6..9]),
                    },
                    scalars: 9,
                },
                SurfaceVariant::Cylinder => AnalyticKind {
                    build: |x| Surface::Cylinder {
                        origin: pt(&x[0..3]),
                        axis: dir(&x[3..6]),
                        radius: x[6],
                        u_ref: dir(&x[7..10]),
                    },
                    scalars: 10,
                },
                SurfaceVariant::Cone => AnalyticKind {
                    build: |x| Surface::Cone {
                        apex: pt(&x[0..3]),
                        axis: dir(&x[3..6]),
                        half_angle: x[6],
                        u_ref: dir(&x[7..10]),
                    },
                    scalars: 10,
                },
                SurfaceVariant::Sphere => AnalyticKind {
                    build: |x| Surface::Sphere {
                        center: pt(&x[0..3]),
                        radius: x[3],
                        axis: dir(&x[4..7]),
                        u_ref: dir(&x[7..10]),
                    },
                    scalars: 10,
                },
                SurfaceVariant::Torus => AnalyticKind {
                    build: |x| Surface::Torus {
                        center: pt(&x[0..3]),
                        axis: dir(&x[3..6]),
                        major_radius: x[6],
                        minor_radius: x[7],
                        u_ref: dir(&x[8..11]),
                    },
                    scalars: 11,
                },
                SurfaceVariant::Nurbs | SurfaceVariant::Approx => return None,
            })
        })
        .collect()
}

/// Every analytic carrier kind, in declaration order.
#[must_use]
pub fn analytic_curves() -> Vec<AnalyticKind<Curve3<f64>>> {
    Curve3Variant::iter()
        .filter_map(|kind| -> Option<AnalyticKind<Curve3<f64>>> {
            Some(match kind {
                Curve3Variant::Line => AnalyticKind {
                    build: |x| Curve3::Line {
                        origin: pt(&x[0..3]),
                        dir: dir(&x[3..6]),
                    },
                    scalars: 6,
                },
                Curve3Variant::Circle => AnalyticKind {
                    build: |x| Curve3::Circle {
                        center: pt(&x[0..3]),
                        axis: dir(&x[3..6]),
                        radius: x[6],
                        u_ref: dir(&x[7..10]),
                    },
                    scalars: 10,
                },
                Curve3Variant::Ellipse => AnalyticKind {
                    build: |x| Curve3::Ellipse {
                        center: pt(&x[0..3]),
                        axis: dir(&x[3..6]),
                        major: x[6],
                        minor: x[7],
                        u_ref: dir(&x[8..11]),
                    },
                    scalars: 11,
                },
                Curve3Variant::Spiric => AnalyticKind {
                    build: |x| Curve3::Spiric {
                        center: pt(&x[0..3]),
                        axis: dir(&x[3..6]),
                        u_ref: dir(&x[6..9]),
                        major_radius: x[9],
                        minor_radius: x[10],
                        offset: x[11],
                    },
                    scalars: 12,
                },
                Curve3Variant::Nurbs => return None,
            })
        })
        .collect()
}
