//! **Where a prefix leaves the pen** ([`profile::pen`]): the tip's
//! position and the heading a leg from it departs along, read off the
//! replay driver — what a document step reading a face's trace runs
//! from (`editor_core::trace`).
#![allow(clippy::expect_used, clippy::float_cmp)]

use geom_core::{Point2, Tol};
use profile::{Step, Target};

fn tol() -> Tol {
    Tol::witness()
}

#[test]
fn a_director_is_the_heading_and_a_leg_end_its_incoming_tangent() {
    let at = Step::At(Point2::new(1.0, 2.0));
    let toward = Step::Toward { dx: 0.0, dy: 2.0 };
    let pen = profile::pen(&[at, toward], None, tol())
        .expect("the prefix replays")
        .expect("a positioned tip");
    assert_eq!((pen.at.x, pen.at.y), (1.0, 2.0));
    let h = pen.heading.expect("a director binds a heading");
    assert_eq!((h.x, h.y, pen.directed), (0.0, 1.0, true));

    let line = Step::Line(3.0);
    let pen = profile::pen(&[at, toward, line], None, tol())
        .expect("the prefix replays")
        .expect("a positioned tip");
    assert_eq!((pen.at.x, pen.at.y), (1.0, 5.0));
    let h = pen.heading.expect("a leg end carries its tangent");
    assert_eq!((h.x, h.y, pen.directed), (0.0, 1.0, false));
}

#[test]
fn a_tip_with_no_position_or_a_closed_loop_has_no_pen() {
    let angle = Step::Angle(0.5);
    assert!(
        profile::pen(&[angle], None, tol())
            .expect("the prefix replays")
            .is_none(),
        "a direction bound first has no position"
    );
    let square = [
        Step::At(Point2::new(0.0, 0.0)),
        Step::LineTo(Target::Point(Point2::new(1.0, 0.0))),
        Step::LineTo(Target::Point(Point2::new(1.0, 1.0))),
        Step::LineTo(Target::Start),
    ];
    assert!(
        profile::pen(&square, None, tol())
            .expect("the program replays")
            .is_none(),
        "a closed loop has no pen"
    );
    assert!(
        profile::pen(&[Step::Line(1.0)], None, tol()).is_err(),
        "a prefix the driver refuses refuses here"
    );
}
