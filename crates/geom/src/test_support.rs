//! **Test fixtures**, behind the `test-support` feature (on only
//! through dev-dependency edges): one fixture of every analytic kind,
//! for the rows that hold a field-by-field reader to the walks
//! ([`Surface::data`], [`Curve3::data`]); the NURBS segments a row hands
//! a spline door ([`nurbs_segment`], [`poisoned_nurbs_segment`]); and
//! the chart frames of [`reframed`].
//!
//! Each analytic kind is a builder over a flat scalar list, laid out in its
//! variant's field order, beside the number of scalars it reads. The
//! kinds are read off the kind rosters ([`SurfaceKind::ALL`],
//! [`CurveKind::ALL`]), so a variant added to [`Surface`] or [`Curve3`]
//! fails this module until it is given a builder or declared as not
//! read field by field.

pub mod collapsed;
pub mod held_arc;

use crate::datum::ANALYTIC_DATA_MAX;
use crate::{Curve3, CurveKind, Surface, SurfaceKind};
use core::num::NonZeroUsize;
use geom_core::spline::{KnotVector, Knots, SpanLocate};
use geom_core::{Point3, UnitVec3, Vec3};
use std::sync::Arc;

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
    SurfaceKind::ALL
        .into_iter()
        .filter_map(|kind| -> Option<AnalyticKind<Surface<f64>>> {
            Some(match kind {
                SurfaceKind::Plane => AnalyticKind {
                    build: |x| Surface::Plane {
                        frame: crate::Frame::new(pt(&x[0..3]), dir(&x[3..6]), dir(&x[6..9])),
                    },
                    scalars: 9,
                },
                SurfaceKind::Cylinder => AnalyticKind {
                    build: |x| Surface::Cylinder {
                        frame: crate::Frame::new(pt(&x[0..3]), dir(&x[3..6]), dir(&x[7..10])),
                        radius: x[6],
                    },
                    scalars: 10,
                },
                SurfaceKind::Cone => AnalyticKind {
                    build: |x| Surface::Cone {
                        frame: crate::Frame::new(pt(&x[0..3]), dir(&x[3..6]), dir(&x[7..10])),
                        half_angle: x[6],
                    },
                    scalars: 10,
                },
                SurfaceKind::Sphere => AnalyticKind {
                    build: |x| Surface::Sphere {
                        frame: crate::Frame::new(pt(&x[0..3]), dir(&x[4..7]), dir(&x[7..10])),
                        radius: x[3],
                    },
                    scalars: 10,
                },
                SurfaceKind::Torus => AnalyticKind {
                    build: |x| Surface::Torus {
                        frame: crate::Frame::new(pt(&x[0..3]), dir(&x[3..6]), dir(&x[8..11])),
                        major_radius: x[6],
                        minor_radius: x[7],
                    },
                    scalars: 11,
                },
                SurfaceKind::Spindle => AnalyticKind {
                    build: |x| Surface::Spindle {
                        frame: crate::Frame::new(pt(&x[0..3]), dir(&x[3..6]), dir(&x[8..11])),
                        major_radius: x[6],
                        minor_radius: x[7],
                    },
                    scalars: 11,
                },
                SurfaceKind::Nurbs | SurfaceKind::Approx => return None,
            })
        })
        .collect()
}

/// Every analytic carrier kind, in declaration order.
#[must_use]
pub fn analytic_curves() -> Vec<AnalyticKind<Curve3<f64>>> {
    CurveKind::ALL
        .into_iter()
        .filter_map(|kind| -> Option<AnalyticKind<Curve3<f64>>> {
            Some(match kind {
                CurveKind::Line => AnalyticKind {
                    build: |x| Curve3::line(pt(&x[0..3]), UnitVec3::squared(dir(&x[3..6]))),
                    scalars: 6,
                },
                CurveKind::Circle => AnalyticKind {
                    build: |x| Curve3::circle(pt(&x[0..3]), dir(&x[3..6]), x[6], dir(&x[7..10])),
                    scalars: 10,
                },
                CurveKind::Ellipse => AnalyticKind {
                    build: |x| {
                        Curve3::Ellipse(crate::Ellipse::new(
                            crate::Frame::new(pt(&x[0..3]), dir(&x[3..6]), dir(&x[8..11])),
                            x[6],
                            x[7],
                        ))
                    },
                    scalars: 11,
                },
                CurveKind::Spiric => AnalyticKind {
                    build: |x| Curve3::Spiric {
                        frame: crate::Frame::new(pt(&x[0..3]), dir(&x[3..6]), dir(&x[6..9])),
                        major_radius: x[9],
                        minor_radius: x[10],
                        offset: x[11],
                    },
                    scalars: 12,
                },
                CurveKind::Nurbs | CurveKind::Periodic => return None,
            })
        })
        .collect()
}

/// A degree-1 NURBS segment on `[0, 1]` from `a` to `b`, unit weights.
#[allow(clippy::expect_used)]
fn segment<T: SpanLocate>(a: Point3<T>, b: Point3<T>) -> Curve3<T> {
    let knots = Knots::from(KnotVector::unit_segment(NonZeroUsize::MIN)).lift(T::from_f64);
    Curve3::Nurbs(Arc::new(
        crate::NurbsCurve3::new(knots, vec![a, b], vec![T::one(), T::one()])
            .expect("a degree-1 segment with unit weights validates"),
    ))
}

/// **A described NURBS carrier**: the degree-1 segment from `a` to `b`
/// on `[0, 1]`, unit weights — the smallest spline a row can hand a
/// door that refuses or reads splines as such.
#[must_use]
pub fn nurbs_segment(a: Point3<f64>, b: Point3<f64>) -> Curve3<f64> {
    segment(a, b)
}

/// **A NURBS carrier poisoned in every channel**: the degree-1 segment
/// whose two control points are the scalar's poison
/// (`net::poison_point`). It validates, so every evaluator answers it,
/// and every answer is poison.
#[must_use]
pub fn poisoned_nurbs_segment<T: SpanLocate>() -> Curve3<T> {
    let p = crate::net::poison_point::<T, Point3<T>>();
    segment(p, p)
}

/// **The frames a frame pin charts one surface on**, as `(t, flip)`
/// for [`reframed`]: the stored frame, the seam turned about the axis
/// to four other azimuths, and the axis flipped with the seam turned.
pub const CHART_FRAMES: [(f64, bool); 6] = [
    (0.0, false),
    (0.3, false),
    (1.0, false),
    (2.5, false),
    (-2.0, false),
    (0.7, true),
];

/// **`surface` on another chart**: its frame's seam turned by `t` about
/// its axis (or a plane's normal), and the axis flipped when `flip`.
/// The locus is the same, and only its chart moves. A spline or a
/// spindle sheet has no frame to turn, and panics.
#[must_use]
pub fn reframed(surface: &Surface<f64>, t: f64, flip: bool) -> Surface<f64> {
    let turn = |axis: Vec3<f64>, u_ref: Vec3<f64>| {
        let u = u_ref * t.cos() + axis.cross(u_ref) * t.sin();
        if flip { (-axis, u) } else { (axis, u) }
    };
    match *surface {
        Surface::Plane {
            frame: carrier_frame,
        } => {
            let (origin, normal, u_ref) = (
                carrier_frame.origin(),
                carrier_frame.axis(),
                carrier_frame.u_ref(),
            );
            let (normal, u_ref) = turn(normal, u_ref);
            Surface::Plane {
                frame: crate::Frame::new(origin, normal, u_ref),
            }
        }
        Surface::Cylinder {
            frame: carrier_frame,
            radius,
        } => {
            let (origin, axis, u_ref) = (
                carrier_frame.origin(),
                carrier_frame.axis(),
                carrier_frame.u_ref(),
            );
            let (axis, u_ref) = turn(axis, u_ref);
            Surface::Cylinder {
                frame: crate::Frame::new(origin, axis, u_ref),
                radius,
            }
        }
        Surface::Cone {
            frame: carrier_frame,
            half_angle,
        } => {
            let (apex, axis, u_ref) = (
                carrier_frame.origin(),
                carrier_frame.axis(),
                carrier_frame.u_ref(),
            );
            let (axis, u_ref) = turn(axis, u_ref);
            Surface::Cone {
                frame: crate::Frame::new(apex, axis, u_ref),
                half_angle,
            }
        }
        Surface::Sphere {
            frame: carrier_frame,
            radius,
        } => {
            let (center, axis, u_ref) = (
                carrier_frame.origin(),
                carrier_frame.axis(),
                carrier_frame.u_ref(),
            );
            let (axis, u_ref) = turn(axis, u_ref);
            Surface::Sphere {
                frame: crate::Frame::new(center, axis, u_ref),
                radius,
            }
        }
        Surface::Torus {
            frame: carrier_frame,
            major_radius,
            minor_radius,
        } => {
            let (center, axis, u_ref) = (
                carrier_frame.origin(),
                carrier_frame.axis(),
                carrier_frame.u_ref(),
            );
            let (axis, u_ref) = turn(axis, u_ref);
            Surface::Torus {
                frame: crate::Frame::new(center, axis, u_ref),
                major_radius,
                minor_radius,
            }
        }
        _ => unreachable!("only a framed analytic surface is reframed"),
    }
}
