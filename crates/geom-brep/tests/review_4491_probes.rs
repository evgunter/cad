//! Review probes for PR 4491: `edge_join`-shaped extensions of a
//! restricted description compose in the range.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_brep::{MappedCurve, MappedSource, SketchSegment};
use geom_core::{Affine3, Arc2, Point2};

fn sources() -> [MappedSource<f64>; 2] {
    let place = Affine3::identity();
    [
        MappedSource::PlacedSegment {
            segment: SketchSegment::Line {
                a: Point2::new(1.0, 0.0),
                b: Point2::new(-1.0, 0.5),
            },
            place,
        },
        MappedSource::PlacedSegment {
            segment: SketchSegment::Arc {
                a: Point2::new(1.0, 0.0),
                b: Point2::new(-1.0, 0.0),
                arc: Arc2 {
                    centre: Point2::new(0.0, 0.0),
                    radius: 1.0,
                    sweep: core::f64::consts::PI,
                },
            },
            place,
        },
    ]
}

/// A split child extended forward (`s1 > 1`) or backward (`s0 < 0`), as
/// `edge_join` restricts a kept edge over the joined span, evaluates as
/// the parent at the composed parameter.
#[test]
fn extensions_compose_in_the_range() {
    for source in sources() {
        let whole = MappedCurve::whole(source);
        let child = whole.restrict(0.2, 0.6);
        // forward: kept child [0.2, 0.6] joined with [0.6, 0.8] → s1 = 1.5
        let fwd = child.restrict(0.0, 1.5);
        // backward: joined with [0.0, 0.2] → s0 = -0.5
        let bwd = child.restrict(-0.5, 1.0);
        for i in 0..=8 {
            let s = f64::from(i) / 8.0;
            let d = fwd.eval(s).distance(whole.eval(0.2 + 0.6 * s));
            assert!(d < 1e-15, "forward s = {s}: {d:e}");
            let d = bwd.eval(s).distance(whole.eval(0.6 * s));
            assert!(d < 1e-15, "backward s = {s}: {d:e}");
        }
        assert_eq!(fwd.range.start(), Some(0.2));
        assert!(bwd.range.start().unwrap().abs() < 1e-16);
    }
}
