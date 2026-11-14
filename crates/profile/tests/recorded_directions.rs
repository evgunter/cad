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

/// A box half-width small enough that unguided validation's pair
/// readings decide at the run's ε (a sixty-fourth of it): the boxed
/// polygon rows are about the directions' width, which is none at any
/// box, not about how wide a box validation reads.
fn small() -> f64 {
    tol().eps() / 64.0
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
        let centre = Point2::new(param("x", 1.0, small()), param("y", 0.0, small()));
        let lp = profile::rectangle(
            centre,
            param("w", 2.0, small()),
            param("h", 1.0, small()),
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
/// points share a variable it is stored with the width of the quotient
/// that mints it: never narrower than the direction over the box (its
/// enclosure holds the direction at both ends), and wider than that
/// hull by the quotient's dependency on `t` (measured: about three
/// times in `x`), which the row bounds at four.
#[test]
fn a_directed_leg_records_its_director_and_a_hop_its_true_width() {
    in_session(|| {
        let t = param("t", 1.0, small());
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
        // The hop from (t + 1, 0) to (2t, t + 1): direction (t − 1, t + 1)/|·|,
        // whose x runs from negative to positive across the box.
        let d1 = bounds(director(segs[1].directions));
        let truth = |tv: f64| {
            let (dx, dy) = (tv - 1.0, tv + 1.0);
            let n = dx.hypot(dy);
            (dx / n, dy / n)
        };
        let ends = [truth(1.0 - small()), truth(1.0 + small())];
        let encloses = |e: [(f64, f64); 2], (wx, wy): (f64, f64)| {
            e[0].0 <= wx && wx <= e[0].1 && e[1].0 <= wy && wy <= e[1].1
        };
        for w in ends {
            assert!(encloses(d1, w), "the hop's director {d1:?} holds {w:?}");
        }
        // The check can fail: a direction without the width, the hop's
        // own at the box's centre, holds neither end.
        let centre = [(0.0, 0.0), (1.0, 1.0)];
        assert!(ends.iter().all(|&w| !encloses(centre, w)));
        let hull = ends[1].0 - ends[0].0;
        assert!(
            d1[0].1 - d1[0].0 <= 4.0 * hull,
            "the hop's director {d1:?} against its hull {hull:e}"
        );
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
            ProfileError::DirectionNotUnit {
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

/// A square of side `l` whose first director is `d`, the others the axes.
fn square_of_side(l: f64, d: Vec2<f64>) -> ProfileLoop<f64> {
    let line = |x: f64, y: f64| Some(SegmentDirections::Line(Vec2::new(x, y)));
    RawLoop::new_directed([
        (
            Point2::new(0.0, 0.0),
            Segment::Line,
            Some(SegmentDirections::Line(d)),
        ),
        (Point2::new(l, 0.0), Segment::Line, line(0.0, 1.0)),
        (Point2::new(l, l), Segment::Line, line(-1.0, 0.0)),
        (Point2::new(0.0, l), Segment::Line, line(0.0, -1.0)),
    ])
}

/// **A recorded direction is a unit to rounding; its angle keeps the
/// band's slack.** The miss `‖o + ℓ·d − p‖` is read against the band, so
/// on a side of length ℓ a director may sit `ε/ℓ` off its chord in
/// angle and is stored as written; a director stretched by as much is
/// refused `DirectionNotUnit`, read on its own before the miss, which
/// would have held it to the same `ε/ℓ`. Each side length is one the
/// miss alone would have let the stretch through.
#[test]
fn a_recorded_direction_is_a_unit_to_rounding_and_its_angle_keeps_the_bands_slack() {
    let eps = tol().eps();
    for l in [(100.0 * eps).sqrt(), 1.0] {
        let off = 0.5 * eps / l;
        let (s, c) = Real::sin_cos(off);
        let turned = Vec2::new(c, s);
        let ok = table(square_of_side(l, turned))
            .unwrap_or_else(|e| panic!("side {l:e}: a director {off:e} off in angle validates: {e:?}"));
        assert_eq!(
            bits(director(ok.loops()[0].segments()[0].directions)),
            bits(turned),
            "side {l:e}: stored as written"
        );
        for stretch in [1.0 + off, 1.0 - off] {
            let err = table(square_of_side(l, Vec2::new(stretch, 0.0)))
                .expect_err("a stretched director refuses");
            assert!(
                matches!(
                    err,
                    ProfileError::DirectionNotUnit {
                        which: RecordedDirection::Director,
                        ..
                    }
                ),
                "side {l:e}, length {stretch}: {err:?}"
            );
        }
    }
}

/// **The lift carries a recorded direction verbatim.** A table director
/// that agrees with its chord within ε but not bit for bit (the chord's
/// unit turned by 1e-13) survives the lift to `Interval` as the point it
/// was; a lift that re-derived it from the chord would read `(0.6, 0.8)`.
#[test]
fn the_lift_carries_a_recorded_direction_and_never_rederives_it() {
    let d = Vec2::new(0.6 - 0.8e-13, 0.8 + 0.6e-13);
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

/// **A hop between two points that share a variable mints its axis with
/// no width**: the slab whose width `W` carries a box, drawn through its
/// four corners. The side from (W, 0) to (W, 1) is (0, 1) at every `W`,
/// and validation's mint reads `W − W` as the zero it is (a theorem at
/// `Sym`), where the quotient would carry the box's width twice.
#[test]
fn a_hop_along_an_axis_mints_its_director_with_no_width() {
    in_session(|| {
        let w = param("W", 1.0, small());
        let (z, one) = (Lane::from_f64(0.0), Lane::from_f64(1.0));
        let lp = Open
            .at(Point2::new(z, z))
            .line_to(Point2::new(w, z), tol())
            .unwrap()
            .line_to(Point2::new(w, one), tol())
            .unwrap()
            .line_to(Point2::new(z, one), tol())
            .unwrap()
            .line_to(Start, tol())
            .unwrap();
        let vp = validated(vec![lp.loop_]);
        let want = [(1.0, 0.0), (0.0, 1.0), (-1.0, 0.0), (0.0, -1.0)];
        for (k, (x, y)) in want.into_iter().enumerate() {
            let d = director(vp.loops()[0].segments()[k].directions);
            assert_eq!(bounds(d), [(x, x), (y, y)], "side {k}");
        }
    });
}

/// One recorded direction's bits, for an exact comparison.
fn recorded_bits(d: Option<SegmentDirections<f64>>) -> Option<Vec<u64>> {
    d.map(|d| match d {
        SegmentDirections::Line(v) => bits(v).to_vec(),
        SegmentDirections::Arc { start, end } => [bits(start), bits(end)].concat(),
    })
}

fn arc_rec(start: Vec2<f64>, end: Vec2<f64>) -> Option<SegmentDirections<f64>> {
    Some(SegmentDirections::Arc { start, end })
}

fn line_rec(x: f64, y: f64) -> Option<SegmentDirections<f64>> {
    Some(SegmentDirections::Line(Vec2::new(x, y)))
}

/// `lp`'s recorded directions are `want`, bit for bit, `None` included.
fn records(lp: &ProfileLoop<f64>, want: &[Option<SegmentDirections<f64>>], what: &str) {
    let got: Vec<_> = lp.directions().iter().map(|&d| recorded_bits(d)).collect();
    let want: Vec<_> = want.iter().map(|&d| recorded_bits(d)).collect();
    assert_eq!(got, want, "{what}: {:?}", lp.directions());
}

/// **What each verb records on its loop**, before validation reads it:
/// the exact direction where the verb holds one, and `None` where it
/// holds only points (validation then mints it). A verb that stopped
/// recording, or one that recorded where it holds no direction, goes
/// red here; the validated rows above cannot see either, since
/// validation mints a missing direction to the same value.
#[test]
fn each_verb_records_what_it_holds_on_the_loop() {
    let x = |x: f64, y: f64| Vec2::new(x, y);
    let circle = profile::circle(Point2::new(0.3, 0.2), 0.05, tol()).unwrap();
    records(
        &circle.loop_.into_loop(),
        &[
            arc_rec(x(1.0, 0.0), x(-1.0, 0.0)),
            arc_rec(x(-1.0, 0.0), x(1.0, 0.0)),
        ],
        "circle",
    );
    let split = profile::circle_split(Point2::new(1.0, 0.5), 0.4, 3, 0.25, tol()).unwrap();
    let unit = |k: usize| {
        let theta = 0.25 + 2.0 * core::f64::consts::PI * (k as f64) / 3.0;
        let (s, c) = Real::sin_cos(theta);
        x(c, s)
    };
    records(
        &split.loop_.into_loop(),
        &[
            arc_rec(unit(0), unit(1)),
            arc_rec(unit(1), unit(2)),
            arc_rec(unit(2), unit(0)),
        ],
        "split circle",
    );
    let rect = profile::rectangle(Point2::new(1.0, 0.0), 2.0, 1.0, tol()).unwrap();
    records(
        &rect.loop_.into_loop(),
        &[
            line_rec(1.0, 0.0),
            line_rec(0.0, 1.0),
            line_rec(-1.0, 0.0),
            line_rec(0.0, -1.0),
        ],
        "rectangle",
    );
    // A leg along a director records it; a leg to a point target
    // continuing that director, and hops, record nothing.
    let legs = Open
        .at(Point2::new(0.0, 0.0))
        .toward(1.0, 0.0, tol())
        .unwrap()
        .line(2.0, tol())
        .unwrap()
        .continue_to(Point2::new(3.0, 0.0), tol())
        .unwrap()
        .line_to(Point2::new(3.0, 1.0), tol())
        .unwrap()
        .line_to(Start, tol())
        .unwrap();
    records(
        &legs.loop_.into_loop(),
        &[line_rec(1.0, 0.0), None, None, None],
        "directed leg, point target, hops",
    );
    // Bulge and via: the lowering's own radials at the literal sign.
    let (a, b) = (Point2::new(4.0, 0.0), Point2::new(4.0, 4.0));
    let lowered = |bulge: f64| {
        let [s, e] = Arc2::from_chord_radials(a, b, bulge, bulge.signum()).1;
        arc_rec(s, e)
    };
    for bulge in [0.4, -0.3] {
        let lp = Open
            .at(Point2::new(0.0, 0.0))
            .line_to(a, tol())
            .unwrap()
            .arc_to(profile::Bulge { p: b, b: bulge }, tol())
            .unwrap()
            .line_to(Point2::new(0.0, 4.0), tol())
            .unwrap()
            .line_to(Start, tol())
            .unwrap();
        records(
            &lp.loop_.into_loop(),
            &[None, lowered(bulge), None, None],
            &format!("bulge {bulge}"),
        );
    }
    let q = Point2::new(5.0, 2.0);
    let via = Open
        .at(Point2::new(0.0, 0.0))
        .line_to(a, tol())
        .unwrap()
        .arc_to(profile::Via { q, p: b }, tol())
        .unwrap()
        .line_to(Point2::new(0.0, 4.0), tol())
        .unwrap()
        .line_to(Start, tol())
        .unwrap();
    records(
        &via.loop_.into_loop(),
        &[None, lowered(profile::bulge_from_via(a, q, b)), None, None],
        "via",
    );
    // A centred arc holds its points and its centre, not its radials.
    let center = Open
        .at(Point2::new(0.0, 0.0))
        .line_to(a, tol())
        .unwrap()
        .arc_to(
            profile::Center {
                c: Point2::new(5.0, 2.0),
                winding: ArcSweep::Ccw,
                p: b,
            },
            tol(),
        )
        .unwrap()
        .line_to(Point2::new(0.0, 4.0), tol())
        .unwrap()
        .line_to(Start, tol())
        .unwrap();
    records(&center.loop_.into_loop(), &[None; 4], "centred arc");
    // Tangent arcs: σ·(t.y, −t.x) at each end, the tangent turned away
    // from the turn's centre; left from east, the end tangent north.
    let tangent = Open
        .at(Point2::new(0.0, 0.0))
        .line_to(Point2::new(2.0, 0.0), tol())
        .unwrap()
        .tangent()
        .tangent_arc_to(Point2::new(3.0, 1.0), tol())
        .unwrap()
        .line_to(Point2::new(0.0, 2.0), tol())
        .unwrap()
        .line_to(Start, tol())
        .unwrap();
    records(
        &tangent.loop_.into_loop(),
        &[None, arc_rec(x(0.0, -1.0), x(1.0, -0.0)), None, None],
        "tangent arc",
    );
    // The B: a tangent arc then `arc_onto(Start)`, each recording its
    // radials (the onto arc's are the tangent construction's swapped);
    // checked against the vertices, not bitwise.
    let b_loop = Open
        .at(Point2::new(0.0, 0.0))
        .toward(0.0, 1.0, tol())
        .unwrap()
        .line(2.0, tol())
        .unwrap()
        .toward(1.0, 0.0, tol())
        .unwrap()
        .tangent_arc_to(Point2::new(0.25, 1.0), tol())
        .unwrap()
        .arc_onto(Start, tol())
        .unwrap()
        .toward(-1.0, 0.0, tol())
        .unwrap()
        .loop_
        .into_loop();
    assert_eq!(
        recorded_bits(b_loop.directions()[0]),
        recorded_bits(line_rec(0.0, 1.0))
    );
    let n = b_loop.vertices().len();
    for k in 1..n {
        let Segment::Arc(arc) = b_loop.segments()[k] else {
            panic!("the B's segment {k} is an arc");
        };
        let (rs, re) = radials(b_loop.directions()[k].expect("an onto/tangent arc records"));
        let ends = [b_loop.vertices()[k], b_loop.vertices()[(k + 1) % n]];
        for (v, r) in ends.into_iter().zip([rs, re]) {
            let want = (v - arc.centre) / arc.radius;
            assert!((r - want).norm() < 1e-15, "the B's segment {k}: {r:?} vs {want:?}");
        }
    }
    // Fillets and the arc_fillet's side arc hold their tangency points,
    // not a direction: nothing recorded.
    let (east, north) = (0.0_f64, core::f64::consts::FRAC_PI_2);
    let (west, south) = (core::f64::consts::PI, -north);
    let t = tol();
    let rounded = Open
        .at(Point2::new(0.0, -1.0))
        .angle(east, t)
        .unwrap()
        .fillet(0.25, t)
        .unwrap()
        .at(Point2::new(1.0, 0.0), t)
        .unwrap()
        .angle(north, t)
        .unwrap()
        .fillet(0.25, t)
        .unwrap()
        .at(Point2::new(0.0, 1.0), t)
        .unwrap()
        .angle(west, t)
        .unwrap()
        .fillet(0.25, t)
        .unwrap()
        .at(Point2::new(-1.0, 0.0), t)
        .unwrap()
        .angle(south, t)
        .unwrap()
        .fillet(0.25, t)
        .unwrap()
        .to(Start, t)
        .unwrap();
    records(&rounded.loop_.into_loop(), &[None; 8], "the rounded square");
    let side = Open
        .arc_fillet(
            profile::Center {
                c: Point2::new(0.0, 0.0),
                winding: ArcSweep::Cw,
                p: Point2::new(0.0, 2.0),
            },
            0.5,
            t,
        )
        .unwrap()
        .toward(1.0, 0.0, t)
        .unwrap()
        .to(Point2::new(4.0, 0.0), t)
        .unwrap()
        .line_to(Point2::new(4.0, 3.0), t)
        .unwrap()
        .line_to(Point2::new(-1.0, 3.0), t)
        .unwrap()
        .line_to(Start, t)
        .unwrap();
    records(&side.loop_.into_loop(), &[None; 6], "arc_fillet");
}
