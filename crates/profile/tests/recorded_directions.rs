//! **A validated segment records the unit directions its construction
//! made**: a line's director, an arc's radial at each end
//! (`crates/profile/README.md`, the profile format). A verb feeds what it
//! holds exactly; a segment that holds only its points has its direction
//! decided once at validate; a table's recorded direction is checked
//! there; the lift carries them; reversal swaps and negates.
#![allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::float_cmp
)]

use geom_core::sym::with_session_rules;
use geom_core::{
    Arc2, Bounds, Decide, Interval, ParamSymbol, Point2, Real, Sym, SymBudget, SymRules, Tol, Vec2,
};
use profile::{
    ArcSweep, ConstructedLoop, ConstructedProfile, Open, Profile, ProfileError, ProfileLoop,
    RawLoop, RecordedDirection, Segment, SegmentDirections, SketchPlane, Start, ValidatedProfile,
};

fn tol() -> Tol {
    Tol::witness()
}

fn validated<T: Decide>(loops: Vec<ConstructedLoop<T>>) -> ValidatedProfile<T> {
    ConstructedProfile::new(SketchPlane::xy(), loops)
        .validate(tol())
        .expect("the profile validates")
}

fn bits(v: Vec2<f64>) -> [u64; 2] {
    [v.x.to_bits(), v.y.to_bits()]
}

fn bounds<T: Bounds>(v: Vec2<T>) -> [(f64, f64); 2] {
    [(v.x.lo(), v.x.hi()), (v.y.lo(), v.y.hi())]
}

/// The lane a box replays in: `Sym<Interval>`, each named value a
/// parameter over its box.
type Lane = Sym<Interval>;

/// `f` run inside a symbolic session, as the driver runs a leaf.
fn in_session<R>(f: impl FnOnce() -> R) -> R {
    let budget = SymBudget {
        max_terms: 4096,
        max_degree: 128,
    };
    with_session_rules(budget, SymRules::shipped(), f).0
}

/// The parameter `name` over `c ± h`.
fn param(name: &str, c: f64, h: f64) -> Lane {
    Sym::param(
        ParamSymbol::new(test_utils::symbol_id(name)),
        Interval::from_bounds(c - h, c + h),
    )
}

fn radials<T: Real>(d: SegmentDirections<T>) -> (Vec2<T>, Vec2<T>) {
    match d {
        SegmentDirections::Arc { start, end } => (start, end),
        SegmentDirections::Line(_) => panic!("an arc's directions are radials"),
    }
}

fn director<T: Real>(d: SegmentDirections<T>) -> Vec2<T> {
    match d {
        SegmentDirections::Line(d) => d,
        SegmentDirections::Arc { .. } => panic!("a line's direction is its director"),
    }
}

/// **The circle verb's radials are exactly ±x**, at `f64` and over a
/// box on its centre and radius: the verb places its vertices at
/// `centre ± r·x̂`, so it holds the radial without a quotient, and the
/// validated segment stores it with no width.
#[test]
fn a_circles_radials_are_exactly_plus_and_minus_x_over_a_box() {
    let vp = validated(vec![
        profile::circle(Point2::new(0.3, 0.2), 0.05, tol())
            .unwrap()
            .into(),
    ]);
    let segs = vp.loops()[0].segments();
    let (one, zero) = (1.0f64, 0.0f64);
    let want = [((one, zero), (-one, zero)), ((-one, zero), (one, zero))];
    for (k, ((sx, sy), (ex, ey))) in want.into_iter().enumerate() {
        let (start, end) = radials(segs[k].directions);
        assert_eq!(
            bits(start),
            bits(Vec2::new(sx, sy)),
            "f64 segment {k}'s start radial"
        );
        assert_eq!(
            bits(end),
            bits(Vec2::new(ex, ey)),
            "f64 segment {k}'s end radial"
        );
    }
    in_session(|| {
        let centre = Point2::new(param("cx", 0.3, 1e-3), param("cy", 0.2, 1e-3));
        let vp = validated(vec![
            profile::circle(centre, param("r", 0.05, 1e-4), tol())
                .unwrap()
                .into(),
        ]);
        let band = geom_core::Band::linear(tol()).unwrap();
        let segs = vp.loops()[0].segments();
        for (k, ((sx, sy), (ex, ey))) in want.into_iter().enumerate() {
            let (start, end) = radials(segs[k].directions);
            assert_eq!(
                bounds(start),
                [(sx, sx), (sy, sy)],
                "boxed segment {k}'s start radial"
            );
            assert_eq!(
                bounds(end),
                [(ex, ex), (ey, ey)],
                "boxed segment {k}'s end radial"
            );
            // Certified against the vertices by the registered radial
            // facts: the centre plus the radius along each radial IS the
            // vertex through the tier, though `|r| − r` is no polynomial.
            let profile::SegmentKind::Arc { arc, .. } = segs[k].kind else {
                panic!("a circle's segments are arcs");
            };
            for (v, d, end) in [(segs[k].start, start, "start"), (segs[k].end, end, "end")] {
                let miss = arc.on_radial(d) - v;
                for (c, axis) in [(miss.x, "x"), (miss.y, "y")] {
                    let sign = c.sign_within(band).map(|d| d.sign);
                    assert_eq!(
                        sign.ok(),
                        Some(geom_core::Sign::Zero),
                        "segment {k}'s {end} radial, {axis}"
                    );
                }
            }
        }
    });
}

/// **A hole's reversal swaps its radials and rotates nothing.** The
/// same circle as a hole is canonicalized clockwise, which retraces each
/// arc: its radials are the outer reading's swapped, bit for bit.
#[test]
fn a_reversed_loop_swaps_its_radials_and_negates_its_directors() {
    let circle = || profile::circle(Point2::new(0.0, 0.0), 1.0, tol()).unwrap();
    let outer = validated(vec![circle().into()]);
    let square = profile::rectangle(Point2::new(0.0, 0.0), 4.0, 4.0, tol()).unwrap();
    let holed = validated(vec![square.into(), circle().into()]);
    let hole = &holed.loops()[1];
    assert_eq!(hole.role(), profile::LoopRole::Hole);
    let n = hole.segments().len();
    for k in 0..n {
        let (s, e) = radials(hole.segments()[k].directions);
        let (os, oe) = radials(outer.loops()[0].segments()[(n - k - 1) % n].directions);
        assert_eq!((bits(s), bits(e)), (bits(oe), bits(os)), "hole segment {k}");
    }
    // A table read both ways: reversing negates a director exactly.
    let lp = square_with_directors(1.0);
    let rev = lp.reversed();
    for (k, d) in rev.directions().iter().enumerate() {
        let d = director(d.expect("recorded"));
        let o = director(lp.directions()[(4 - k - 1) % 4].expect("recorded"));
        assert_eq!(bits(d), bits(-o), "reversed director {k}");
    }
    let back = rev.reversed();
    for (k, d) in back.directions().iter().enumerate() {
        let (d, o) = (director(d.unwrap()), director(lp.directions()[k].unwrap()));
        assert_eq!(bits(d), bits(o), "reversed twice is the identity: {k}");
    }
}

/// **The rectangle's directors are the axes**, with no width over a box.
#[test]
fn a_rectangles_directors_are_the_axes_over_a_box() {
    in_session(|| {
        let centre = Point2::new(param("x", 1.0, 1e-11), param("y", 0.0, 1e-11));
        let lp = profile::rectangle(
            centre,
            param("w", 2.0, 1e-11),
            param("h", 1.0, 1e-11),
            tol(),
        )
        .unwrap();
        let vp = validated(vec![lp.into()]);
        let want = [(1.0, 0.0), (0.0, 1.0), (-1.0, 0.0), (0.0, -1.0)];
        for (k, (x, y)) in want.into_iter().enumerate() {
            let d = director(vp.loops()[0].segments()[k].directions);
            assert_eq!(bounds(d), [(x, x), (y, y)], "side {k}");
        }
    });
}

/// **A leg drawn along a director records that director with no
/// width**, though its start carries a box; a leg between two authored
/// points has its director decided at validate, and where the two
/// points share a variable it is stored with its TRUE width, never
/// narrower: its enclosure holds the direction at both ends of the box.
#[test]
fn a_directed_leg_records_its_director_and_a_hop_its_true_width() {
    in_session(|| {
        let t = param("t", 1.0, 1e-11);
        let (z, one) = (Lane::from_f64(0.0), Lane::from_f64(1.0));
        let lp = Open
            .at(Point2::new(t, z))
            .toward(Lane::from_f64(1.0), z, tol())
            .unwrap()
            .line(one, tol())
            .unwrap()
            .line_to(Point2::new(t + t, t + one), tol())
            .unwrap()
            .line_to(Start, tol())
            .unwrap();
        let vp = validated(vec![lp.loop_]);
        let segs = vp.loops()[0].segments();
        let d0 = director(segs[0].directions);
        assert_eq!(
            bounds(d0),
            [(1.0, 1.0), (0.0, 0.0)],
            "the directed leg's director"
        );
        // The hop from (t + 1, 0) to (2t, t + 1): direction (t − 1, t + 1)/|·|.
        let d1 = director(segs[1].directions);
        for tv in [1.0f64 - 1e-11, 1.0 + 1e-11] {
            let (dx, dy) = (tv - 1.0, tv + 1.0);
            let n = dx.hypot(dy);
            let (wx, wy) = (dx / n, dy / n);
            assert!(
                d1.x.lo() <= wx && wx <= d1.x.hi() && d1.y.lo() <= wy && wy <= d1.y.hi(),
                "the hop's director {d1:?} holds its direction at t = {tv}: ({wx}, {wy})"
            );
        }
    });
}

/// **The bulge and tangent lowerings record their radials**: each is the
/// unit from the centre to its vertex, at `f64`, for both turn senses.
#[test]
fn bulge_and_tangent_arcs_record_their_centre_to_vertex_radials() {
    let check = |vp: &ValidatedProfile<f64>, what: &str| {
        let lp = &vp.loops()[0];
        for (k, s) in lp.segments().iter().enumerate() {
            let profile::SegmentKind::Arc { arc, .. } = s.kind else {
                continue;
            };
            let (rs, re) = radials(s.directions);
            for (v, r, end) in [(s.start, rs, "start"), (s.end, re, "end")] {
                let want = (v - arc.centre) / arc.radius;
                assert!(
                    (r - want).norm() < 1e-12,
                    "{what} segment {k} {end}: {r:?} vs {want:?}"
                );
            }
        }
    };
    for b in [0.4, -0.3] {
        let lp = Open
            .at(Point2::new(0.0, 0.0))
            .line_to(Point2::new(4.0, 0.0), tol())
            .unwrap()
            .arc_to(
                profile::Bulge {
                    p: Point2::new(4.0, 4.0),
                    b,
                },
                tol(),
            )
            .unwrap()
            .line_to(Point2::new(0.0, 4.0), tol())
            .unwrap()
            .line_to(Start, tol())
            .unwrap();
        check(&validated(vec![lp.loop_]), &format!("bulge {b}"));
    }
    for y in [1.0, -1.0] {
        let lp = Open
            .at(Point2::new(0.0, 0.0))
            .line_to(Point2::new(2.0, 0.0), tol())
            .unwrap()
            .tangent()
            .tangent_arc_to(Point2::new(3.0, y), tol())
            .unwrap()
            .line_to(Point2::new(0.0, 2.0 * y), tol())
            .unwrap()
            .line_to(Start, tol())
            .unwrap();
        check(
            &validated(vec![lp.loop_]),
            &format!("tangent arc to y = {y}"),
        );
    }
    let _ = ArcSweep::Ccw;
}

/// A unit square through the fixture door with each side's director
/// recorded, the first one scaled by `stretch`.
fn square_with_directors(stretch: f64) -> ProfileLoop<f64> {
    let line = |x: f64, y: f64| Some(SegmentDirections::Line(Vec2::new(x, y)));
    RawLoop::new_directed([
        (Point2::new(0.0, 0.0), Segment::Line, line(stretch, 0.0)),
        (Point2::new(1.0, 0.0), Segment::Line, line(0.0, 1.0)),
        (Point2::new(1.0, 1.0), Segment::Line, line(-1.0, 0.0)),
        (Point2::new(0.0, 1.0), Segment::Line, line(0.0, -1.0)),
    ])
}

fn table(lp: ProfileLoop<f64>) -> Result<ValidatedProfile<f64>, ProfileError> {
    Profile::new(SketchPlane::xy(), vec![lp]).validate(tol())
}

/// **A table's recorded direction is checked at validate, and carried
/// verbatim when it agrees**: a director within ε of its chord is stored
/// as written, not re-derived; one off its chord, one of the wrong
/// length, an arc's radial off its vertex, and directions of the other
/// kind are each refused typed and never stored.
#[test]
fn a_tables_recorded_direction_is_checked_and_refused_typed() {
    let ok = table(square_with_directors(1.0)).expect("an agreeing table validates");
    assert_eq!(
        bits(director(ok.loops()[0].segments()[0].directions)),
        bits(Vec2::new(1.0, 0.0))
    );
    let refuse = |lp: ProfileLoop<f64>| table(lp).expect_err("the table refuses");
    assert!(
        matches!(
            refuse(square_with_directors(1.5)),
            ProfileError::InconsistentDirection {
                which: RecordedDirection::Director,
                ..
            }
        ),
        "a director of length 1.5"
    );
    assert!(
        matches!(
            refuse(square_with_directors(-1.0)),
            ProfileError::InconsistentDirection {
                which: RecordedDirection::Director,
                ..
            }
        ),
        "a director pointing back"
    );
    let half = Arc2 {
        centre: Point2::new(0.0, 0.0),
        radius: 1.0,
        sweep: core::f64::consts::PI,
    };
    let arcs = |start: Vec2<f64>| {
        RawLoop::new_directed([
            (
                Point2::new(1.0, 0.0),
                Segment::Arc(half),
                Some(SegmentDirections::Arc {
                    start,
                    end: Vec2::new(-1.0, 0.0),
                }),
            ),
            (Point2::new(-1.0, 0.0), Segment::Arc(half), None),
        ])
    };
    let good = table(arcs(Vec2::new(1.0, 0.0))).expect("agreeing radials validate");
    let (s, e) = radials(good.loops()[0].segments()[0].directions);
    assert_eq!(
        (bits(s), bits(e)),
        (bits(Vec2::new(1.0, 0.0)), bits(Vec2::new(-1.0, 0.0)))
    );
    // The unrecorded arc's radials are decided from its vertices.
    let (s, e) = radials(good.loops()[0].segments()[1].directions);
    assert!((s - Vec2::new(-1.0, 0.0)).norm() < 1e-15 && (e - Vec2::new(1.0, 0.0)).norm() < 1e-15);
    assert!(
        matches!(
            refuse(arcs(Vec2::new(0.0, 1.0))),
            ProfileError::InconsistentDirection {
                which: RecordedDirection::StartRadial,
                ..
            }
        ),
        "a start radial a quarter turn off"
    );
    let kind: ProfileLoop<f64> = RawLoop::new_directed([
        (
            Point2::new(1.0, 0.0),
            Segment::Arc(half),
            Some(SegmentDirections::Line(Vec2::new(1.0, 0.0))),
        ),
        (Point2::new(-1.0, 0.0), Segment::Arc(half), None),
    ]);
    assert!(
        matches!(
            refuse(kind.clone()),
            ProfileError::InconsistentDirection {
                which: RecordedDirection::Kind,
                ..
            }
        ),
        "a director on an arc"
    );
    // A constructed loop decides nothing about its directions, but owes
    // their kind.
    let constructed = ConstructedProfile::new(
        SketchPlane::xy(),
        vec![ConstructedLoop::fixture(kind, vec![0, 1])],
    )
    .validate(tol())
    .expect_err("a constructed loop with a director on an arc refuses");
    assert!(
        matches!(
            constructed,
            ProfileError::InconsistentDirection {
                which: RecordedDirection::Kind,
                ..
            }
        ),
        "{constructed:?}"
    );
    // A table whose points give no direction: the decided mint's own
    // guard, `vertex_separation`, refuses it before anything is stored.
    let flat = RawLoop::new_directed([
        (Point2::new(0.0, 0.0), Segment::Line, None),
        (Point2::new(0.0, 0.0), Segment::Line, None),
        (Point2::new(1.0, 1.0), Segment::Line, None),
    ]);
    assert!(matches!(refuse(flat), ProfileError::DegenerateSegment(_)));
}

/// **The lift carries a recorded direction verbatim.** A table director
/// that agrees with its chord within ε but not bit for bit survives the
/// lift to `Interval` as the point it was; a lift that re-derived it from
/// the chord would read `(0.6, 0.8)`.
#[test]
fn the_lift_carries_a_recorded_direction_and_never_rederives_it() {
    let d = Vec2::new(0.6, 0.8 + 1e-13);
    let lp = RawLoop::new_directed([
        (
            Point2::new(0.0, 0.0),
            Segment::Line,
            Some(SegmentDirections::Line(d)),
        ),
        (Point2::new(3.0, 4.0), Segment::Line, None),
        (Point2::new(-1.0, 3.0), Segment::Line, None),
    ]);
    let vp = table(lp).expect("the table validates");
    let stored = director(vp.loops()[0].segments()[0].directions);
    assert_eq!(
        bits(stored),
        bits(d),
        "validation stores the table's director as written"
    );
    let lifted = vp
        .clone()
        .lift_onto(SketchPlane::<Interval>::xy())
        .expect("the lift");
    let l = director(lifted.loops()[0].segments()[0].directions);
    assert_eq!(bounds(l), [(d.x, d.x), (d.y, d.y)], "the lifted director");
    let channel = lifted.value_channel().expect("a point channel");
    assert_eq!(
        bits(director(channel.loops()[0].segments()[0].directions)),
        bits(d),
        "the value channel"
    );
}
