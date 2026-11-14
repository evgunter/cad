//! **Which way a face's boundary winds in the face's chart**: the one
//! statement of the rule, for a reader that holds a face's `sense` bit
//! and a polygon in its chart's `(u, v)` plane.
//!
//! # The rule
//!
//! The polygon a face's boundary draws round the face's region in its
//! chart winds COUNTER-CLOCKWISE exactly when `Face::sense` is set
//! ([`ChartWinding::of_face`]). Three facts make it, and none of them
//! is a kind of surface:
//!
//! 1. A loop runs with its face's interior on its left, seen with the
//!    face's outward normal toward the viewer (DESIGN, "Orientation is
//!    one rule").
//! 2. The outward normal is the chart normal where `sense` is set and
//!    its negation where it is not
//!    ([`crate::OutwardNormal::from_chart`]).
//! 3. The chart normal is `∂u × ∂v`, so a chart carries the `(u, v)`
//!    plane's counter-clockwise turn onto the turn that is
//!    counter-clockwise seen with the chart normal toward the viewer.
//!    `geom::Surface` states this of each analytic chart, and a spline
//!    chart's normal is `S_u × S_v` by definition
//!    (`geom::Surface::normal`).
//!
//! So it holds on every chart, analytic or spline, wherever the chart
//! is regular; `the_rule_holds_on_every_chart` reads it off each kind.
//! A ring bounds its face from inside and winds the other way.
//!
//! # What it is not
//!
//! The winding is a fact about a polygon in the chart. A VECTOR on the
//! face's outward side — a normal, a loop's 3-D vector area — is turned
//! by the bit through [`crate::OutwardNormal::from_chart`], not here.

/// The way a closed polygon winds in a chart's `(u, v)` plane, `u` to
/// the right and `v` up.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ChartWinding {
    /// Its region on the left of every side.
    CounterClockwise,
    /// Its region on the right of every side.
    Clockwise,
}

impl ChartWinding {
    /// **The rule** (module docs): the winding of the polygon a face of
    /// sense `sense` draws round its region in its chart.
    ///
    /// `sense` is a `Face::sense` the caller resolved, never a decided
    /// sign.
    #[must_use]
    pub fn of_face(sense: bool) -> Self {
        if sense {
            Self::CounterClockwise
        } else {
            Self::Clockwise
        }
    }

    /// The winding a polygon HAS, read off its signed area in the chart
    /// (the shoelace sum, or any positive multiple of it). `None` for a
    /// polygon of no area, or of a poisoned one: it winds neither way.
    #[must_use]
    pub fn of_signed_area(area: f64) -> Option<Self> {
        if area > 0.0 {
            Some(Self::CounterClockwise)
        } else if area < 0.0 {
            Some(Self::Clockwise)
        } else {
            None
        }
    }

    /// Whether the polygon's region lies to the LEFT of each of its
    /// sides, walked in the polygon's order.
    #[must_use]
    pub fn interior_is_left(self) -> bool {
        self == Self::CounterClockwise
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use geom::{NurbsSurface, Surface};
    use geom_core::spline::KnotVector;
    use geom_core::{Point3, Vec3};

    use super::ChartWinding;
    use crate::OutwardNormal;

    const Z: Vec3<f64> = Vec3::new(0.0, 0.0, 1.0);
    const X: Vec3<f64> = Vec3::new(1.0, 0.0, 0.0);
    const O: Point3<f64> = Point3::new(0.3, -0.2, 0.1);

    /// One chart of each kind, with a regular point of it. The cone's
    /// two rows are its two nappes and the torus's its outer and inner
    /// halves, where the chart normal leans off and toward the axis.
    fn charts() -> Vec<(&'static str, Surface<f64>, (f64, f64))> {
        let saddle = {
            let k = || KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1).unwrap();
            let control = vec![
                Point3::new(0.0, 0.0, 0.0),
                Point3::new(0.0, 1.0, 0.4),
                Point3::new(1.0, 0.0, 0.3),
                Point3::new(1.0, 1.0, -0.2),
            ];
            NurbsSurface::new(k(), k(), control, vec![1.0; 4]).unwrap()
        };
        let cone = || Surface::Cone {
            apex: O,
            axis: Z,
            half_angle: 0.4,
            u_ref: X,
        };
        let torus = || Surface::Torus {
            center: O,
            axis: Z,
            major_radius: 2.0,
            minor_radius: 0.5,
            u_ref: X,
        };
        vec![
            (
                "plane",
                Surface::Plane {
                    origin: O,
                    normal: Z,
                    u_ref: X,
                },
                (0.3, -0.4),
            ),
            (
                "cylinder",
                Surface::Cylinder {
                    origin: O,
                    axis: Z,
                    radius: 0.7,
                    u_ref: X,
                },
                (1.1, 0.4),
            ),
            ("cone, positive nappe", cone(), (2.0, 0.8)),
            ("cone, negative nappe", cone(), (2.0, -0.8)),
            (
                "sphere",
                Surface::Sphere {
                    center: O,
                    radius: 0.9,
                    axis: Z,
                    u_ref: X,
                },
                (4.0, -0.6),
            ),
            ("torus, outer half", torus(), (0.5, 0.3)),
            ("torus, inner half", torus(), (0.5, 2.9)),
            (
                "spindle",
                Surface::Spindle {
                    center: O,
                    axis: Z,
                    major_radius: -0.4,
                    minor_radius: 1.0,
                    u_ref: X,
                },
                (5.0, 0.2),
            ),
            ("spline", Surface::Nurbs(saddle.into()), (0.4, 0.6)),
        ]
    }

    /// The 3-D vector area of the closed cycle `pts`, about its first
    /// point: on the side its turn is counter-clockwise seen from.
    fn vector_area(pts: &[Point3<f64>]) -> Vec3<f64> {
        let o = pts[0];
        (0..pts.len()).fold(Vec3::new(0.0, 0.0, 0.0), |sum, i| {
            sum + (pts[i] - o).cross(pts[(i + 1) % pts.len()] - o)
        })
    }

    fn shoelace(poly: &[(f64, f64)]) -> f64 {
        (0..poly.len())
            .map(|i| {
                let (p, q) = (poly[i], poly[(i + 1) % poly.len()]);
                p.0 * q.1 - q.0 * p.1
            })
            .sum()
    }

    /// The rule, read off every chart kind at both senses and both
    /// windings: a small square about a regular chart point is walked in
    /// 3-D with the face's interior on its left about the face's outward
    /// normal exactly when it winds in the chart as [`ChartWinding::of_face`]
    /// says. A chart whose `∂u × ∂v` ran against the normal `sense` signs
    /// would red its row at both senses.
    #[test]
    fn the_rule_holds_on_every_chart() {
        const H: f64 = 1e-3;
        for (name, surface, (u, v)) in charts() {
            let ccw = [
                (u - H, v - H),
                (u + H, v - H),
                (u + H, v + H),
                (u - H, v + H),
            ];
            let cw = [ccw[3], ccw[2], ccw[1], ccw[0]];
            for sense in [true, false] {
                let outward = OutwardNormal::from_chart(surface.normal(u, v), sense).vec();
                for (polygon, drawn) in [
                    (ccw, ChartWinding::CounterClockwise),
                    (cw, ChartWinding::Clockwise),
                ] {
                    assert_eq!(
                        ChartWinding::of_signed_area(shoelace(&polygon)),
                        Some(drawn),
                        "{name}: the fixture's own polygon"
                    );
                    let walked: Vec<Point3<f64>> =
                        polygon.iter().map(|&(u, v)| surface.eval(u, v)).collect();
                    let interior_left = vector_area(&walked).dot(outward);
                    assert!(
                        interior_left.abs() > 1e-9,
                        "{name}: the square has no area in 3-D ({interior_left:e})"
                    );
                    assert_eq!(
                        interior_left > 0.0,
                        drawn == ChartWinding::of_face(sense),
                        "{name}, sense {sense}: a {drawn:?} polygon bounds the face \
                         exactly when the rule names that winding ({interior_left:e})"
                    );
                }
            }
        }
    }

    /// On the analytic charts the outward normal a holder of a carrier
    /// and the bit reads is the implicit gradient's
    /// ([`crate::implicit_outward_normal`]), not `∂u × ∂v` folded. The
    /// rule is stated against the second, so the two must name one side
    /// wherever a reader pairs a winding with the first.
    #[test]
    fn the_implicit_outward_normal_is_the_chart_normals_side() {
        for (name, surface, (u, v)) in charts() {
            if surface.spline_chart().is_some() {
                continue;
            }
            for sense in [true, false] {
                let chart = OutwardNormal::from_chart(surface.normal(u, v), sense).vec();
                let implicit =
                    crate::implicit_outward_normal(&surface, sense, surface.eval(u, v)).vec();
                assert!(
                    chart.dot(implicit) > 0.99,
                    "{name}, sense {sense}: chart {chart:?} against implicit {implicit:?}"
                );
            }
        }
    }

    /// A polygon of no area winds neither way, so no reader takes a
    /// winding off one; nor off a poisoned area.
    #[test]
    fn no_area_is_no_winding() {
        for area in [0.0, -0.0, f64::NAN] {
            assert_eq!(ChartWinding::of_signed_area(area), None, "{area}");
        }
    }
}
