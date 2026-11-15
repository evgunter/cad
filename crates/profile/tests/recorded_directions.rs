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
    ArcSweep, ConstructedLoop, ConstructedProfile, Gate, Open, Profile, ProfileError, ProfileLoop,
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

/// A validated arc's radials.
fn arc_radials<T: Real>(k: profile::SegmentKind<T>) -> (Vec2<T>, Vec2<T>) {
    match k {
        profile::SegmentKind::Arc {
            radials: [start, end],
            ..
        } => (start, end),
        profile::SegmentKind::Line { .. } => panic!("an arc's directions are radials"),
    }
}

/// A validated line's director.
fn line_director<T: Real>(k: profile::SegmentKind<T>) -> Vec2<T> {
    match k {
        profile::SegmentKind::Line { director } => director,
        profile::SegmentKind::Arc { .. } => panic!("a line's direction is its director"),
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
        let (start, end) = arc_radials(segs[k].kind);
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
            let (start, end) = arc_radials(segs[k].kind);
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
        let (s, e) = arc_radials(hole.segments()[k].kind);
        let (os, oe) = arc_radials(outer.loops()[0].segments()[(n - k - 1) % n].kind);
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
            let d = line_director(vp.loops()[0].segments()[k].kind);
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
        let d0 = line_director(segs[0].kind);
        assert_eq!(
            bounds(d0),
            [(1.0, 1.0), (0.0, 0.0)],
            "the directed leg's director"
        );
        // The hop from (t + 1, 0) to (2t, t + 1): direction (t − 1, t + 1)/|·|,
        // whose x runs from negative to positive across the box.
        let d1 = bounds(line_director(segs[1].kind));
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
            let (rs, re) = arc_radials(s.kind);
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
        bits(line_director(ok.loops()[0].segments()[0].kind)),
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
    let (s, e) = arc_radials(good.loops()[0].segments()[0].kind);
    assert_eq!(
        (bits(s), bits(e)),
        (bits(Vec2::new(1.0, 0.0)), bits(Vec2::new(-1.0, 0.0)))
    );
    // The unrecorded arc's radials are decided from its vertices.
    let (s, e) = arc_radials(good.loops()[0].segments()[1].kind);
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

/// **What "unit" means for a table's direction, at each ε**: the one
/// reading is the miss `‖o + ℓ·d − p‖` at the band, which bounds both the
/// turn and the stretch, `(|d| − 1)·ℓ` by the triangle inequality, check
/// 1's measure for a unit datum at arm ℓ. So on a side of length ℓ a
/// director turned or stretched by `ε/(2ℓ)`, as the review's slack probe
/// found, is ACCEPTED and stored as written; one turned or stretched by
/// `20ε/ℓ`, past the band's far end at that lever, refuses
/// `InconsistentDirection`; and one by `2ε/ℓ`, between the band's ends,
/// escalates `segment_direction`.
#[test]
fn a_tables_direction_is_a_unit_by_check_ones_measure_at_its_lever() {
    let eps = tol().eps();
    let turned = |a: f64| {
        let (s, c) = Real::sin_cos(a);
        Vec2::new(c, s)
    };
    for l in [(100.0 * eps).sqrt(), 1.0] {
        let off = 0.5 * eps / l;
        for (d, what) in [
            (turned(off), "turned"),
            (Vec2::new(1.0 + off, 0.0), "stretched"),
            (Vec2::new(1.0 - off, 0.0), "shrunk"),
        ] {
            let ok = table(square_of_side(l, d)).unwrap_or_else(|e| {
                panic!("side {l:e}: a director {what} by {off:e} validates: {e:?}")
            });
            let stored = line_director(ok.loops()[0].segments()[0].kind);
            assert_eq!(
                bits(stored),
                bits(d),
                "side {l:e}, {what}: stored as written"
            );
            // Within the band at the lever: the turn and the stretch.
            assert!(
                stored.y.atan2(stored.x).abs() * l <= eps,
                "side {l:e}, {what}: the turn"
            );
            assert!(
                (stored.norm() - 1.0).abs() * l <= eps,
                "side {l:e}, {what}: the stretch"
            );
        }
        let far = 20.0 * eps / l;
        for (d, what) in [
            (turned(far), "turned"),
            (Vec2::new(1.0 + far, 0.0), "stretched"),
            (Vec2::new(1.0 - far, 0.0), "shrunk"),
        ] {
            let err = table(square_of_side(l, d)).expect_err("past the band's far end");
            assert!(
                matches!(
                    err,
                    ProfileError::InconsistentDirection {
                        which: RecordedDirection::Director,
                        ..
                    }
                ),
                "side {l:e}, {what} by {far:e}: {err:?}"
            );
        }
        let between = 2.0 * eps / l;
        for (d, what) in [
            (turned(between), "turned"),
            (Vec2::new(1.0 + between, 0.0), "stretched"),
        ] {
            let err = table(square_of_side(l, d)).expect_err("between the band's ends");
            assert!(
                matches!(
                    err,
                    ProfileError::Escalated {
                        gate: Gate::SegmentDirection,
                        ..
                    }
                ),
                "side {l:e}, {what} by {between:e}: {err:?}"
            );
        }
    }
}

/// **Every verb's own directions pass the check a table gets**: each
/// verb's loop read back as a table (`into_loop`) validates whole, at
/// `f64` and at `Sym<Interval>` over a box.
#[test]
fn every_verbs_directions_pass_the_check_a_table_gets() {
    let t = tol();
    let p = |x: f64, y: f64| Point2::new(x, y);
    let f64_loops: Vec<(&str, ProfileLoop<f64>)> = vec![
        (
            "circle",
            profile::circle(p(0.3, 0.2), 0.05, t)
                .unwrap()
                .loop_
                .into_loop(),
        ),
        (
            "split circle",
            profile::circle_split(p(1.0, 0.5), 0.4, 3, 0.25, t)
                .unwrap()
                .loop_
                .into_loop(),
        ),
        (
            "rectangle",
            profile::rectangle(p(1.0, 0.0), 2.0, 1.0, t)
                .unwrap()
                .loop_
                .into_loop(),
        ),
        (
            "directed leg",
            Open.at(p(0.0, 0.0))
                .toward(1.0, 0.0, t)
                .unwrap()
                .line(2.0, t)
                .unwrap()
                .line_to(p(2.0, 1.0), t)
                .unwrap()
                .line_to(Start, t)
                .unwrap()
                .loop_
                .into_loop(),
        ),
        (
            "bulge",
            Open.at(p(0.0, 0.0))
                .line_to(p(4.0, 0.0), t)
                .unwrap()
                .arc_to(
                    profile::Bulge {
                        p: p(4.0, 4.0),
                        b: 0.4,
                    },
                    t,
                )
                .unwrap()
                .line_to(p(0.0, 4.0), t)
                .unwrap()
                .line_to(Start, t)
                .unwrap()
                .loop_
                .into_loop(),
        ),
        (
            "via",
            Open.at(p(0.0, 0.0))
                .line_to(p(4.0, 0.0), t)
                .unwrap()
                .arc_to(
                    profile::Via {
                        q: p(5.0, 2.0),
                        p: p(4.0, 4.0),
                    },
                    t,
                )
                .unwrap()
                .line_to(p(0.0, 4.0), t)
                .unwrap()
                .line_to(Start, t)
                .unwrap()
                .loop_
                .into_loop(),
        ),
        (
            "tangent arc",
            Open.at(p(0.0, 0.0))
                .line_to(p(2.0, 0.0), t)
                .unwrap()
                .tangent()
                .tangent_arc_to(p(3.0, 1.0), t)
                .unwrap()
                .line_to(p(0.0, 2.0), t)
                .unwrap()
                .line_to(Start, t)
                .unwrap()
                .loop_
                .into_loop(),
        ),
        (
            "the B (onto)",
            Open.at(p(0.0, 0.0))
                .toward(0.0, 1.0, t)
                .unwrap()
                .line(2.0, t)
                .unwrap()
                .toward(1.0, 0.0, t)
                .unwrap()
                .tangent_arc_to(p(0.25, 1.0), t)
                .unwrap()
                .arc_onto(Start, t)
                .unwrap()
                .toward(-1.0, 0.0, t)
                .unwrap()
                .loop_
                .into_loop(),
        ),
    ];
    for (what, lp) in f64_loops {
        if let Err(e) = table(lp) {
            panic!("{what} read back as a table: {e:?}");
        }
    }
    in_session(|| {
        let h = small();
        let centre = Point2::new(param("cx", 1.0, h), param("cy", 0.5, h));
        let split = profile::circle_split(centre, param("r", 0.4, h), 3, param("ph", 0.25, h), t)
            .unwrap()
            .loop_
            .into_loop();
        let (z, four) = (Lane::from_f64(0.0), Lane::from_f64(4.0));
        let bulge = Open
            .at(Point2::new(z, z))
            .line_to(Point2::new(four, z), t)
            .unwrap()
            .arc_to(
                profile::Bulge {
                    p: Point2::new(four, four),
                    b: param("b", 0.4, h),
                },
                t,
            )
            .unwrap()
            .line_to(Point2::new(z, four), t)
            .unwrap()
            .line_to(Start, t)
            .unwrap()
            .loop_
            .into_loop();
        let tangent = Open
            .at(Point2::new(z, z))
            .line_to(Point2::new(param("L", 2.0, h), z), t)
            .unwrap()
            .tangent()
            .tangent_arc_to(Point2::new(Lane::from_f64(3.0), param("Y", 1.0, h)), t)
            .unwrap()
            .line_to(Point2::new(z, Lane::from_f64(2.0)), t)
            .unwrap()
            .line_to(Start, t)
            .unwrap()
            .loop_
            .into_loop();
        for (what, lp) in [
            ("split circle", split),
            ("bulge", bulge),
            ("tangent arc", tangent),
        ] {
            if let Err(e) = Profile::new(SketchPlane::xy(), vec![lp]).validate(t) {
                panic!("{what} over ±{h:e} at Sym, read back as a table: {e:?}");
            }
        }
    });
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
    let stored = line_director(vp.loops()[0].segments()[0].kind);
    assert_eq!(
        bits(stored),
        bits(d),
        "validation stores the table's director as written"
    );
    let lifted = vp
        .clone()
        .lift_onto(SketchPlane::<Interval>::xy())
        .expect("the lift");
    let l = line_director(lifted.loops()[0].segments()[0].kind);
    assert_eq!(bounds(l), [(d.x, d.x), (d.y, d.y)], "the lifted director");
    let channel = lifted.value_channel().expect("a point channel");
    assert_eq!(
        bits(line_director(channel.loops()[0].segments()[0].kind)),
        bits(d),
        "the value channel"
    );
}

/// **A hop between two points that share a variable mints its axis with
/// no width**: the slab whose width `W` carries a box, drawn through its
/// four corners. The side from (W, 0) to (W, 1) is (0, 1) at every `W`,
/// and validation's mint reads `W − W` as the zero the tier proves, in
/// an eager session over a box, where the quotient would carry the box's
/// width twice. (In a deferring session over a box it does not:
/// `the_slabs_side_takes_the_quotient_in_a_deferring_session_over_a_box`.)
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
            let d = line_director(vp.loops()[0].segments()[k].kind);
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
            assert!(
                (r - want).norm() < 1e-15,
                "the B's segment {k}: {r:?} vs {want:?}"
            );
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

// ---- the axis mint at two valuations and in each session --------------

/// Where the axis mint's residual is filed: it asks for proof only where
/// the value reads Zero.
const RESIDUAL: &str =
    "work/boxfinish/the-axis-mint-asks-for-proof-only-where-the-value-reads-zero.md";

/// The symbolic sessions a build runs in: eager (each Zero offered to
/// the tier as it is decided) and deferring (the coincidence door's
/// replay, which proves nothing as it decides).
#[derive(Clone, Copy, Debug)]
enum Session {
    Eager,
    Deferring,
}

const SESSIONS: [Session; 2] = [Session::Eager, Session::Deferring];

fn run<R>(kind: Session, f: impl FnOnce() -> R) -> R {
    let budget = SymBudget {
        max_terms: 4096,
        max_degree: 128,
    };
    match kind {
        Session::Eager => with_session_rules(budget, SymRules::shipped(), f).0,
        Session::Deferring => {
            geom_core::sym::deferred::with_deferred_session(
                budget,
                SymRules::shipped(),
                geom_core::SymRetry::none(),
                f,
            )
            .0
        }
    }
}

/// A parameter at `Sym<f64>`.
fn pf(name: &str, v: f64) -> Sym<f64> {
    Sym::param(ParamSymbol::new(test_utils::symbol_id(name)), v)
}

/// The quadrilateral (0, 0) → (x1, 0) → (x2, 1) → (0, 1), its second
/// side's stored director as bounds.
fn quad_hop<T: Decide + Bounds>(x1: T, x2: T) -> [(f64, f64); 2] {
    let (z, one) = (T::from_f64(0.0), T::from_f64(1.0));
    let lp = Open
        .at(Point2::new(z, z))
        .line_to(Point2::new(x1, z), tol())
        .unwrap()
        .line_to(Point2::new(x2, one), tol())
        .unwrap()
        .line_to(Point2::new(z, one), tol())
        .unwrap()
        .line_to(Start, tol())
        .unwrap();
    bounds(line_director(
        validated(vec![lp.loop_]).loops()[0].segments()[1].kind,
    ))
}

const AXIS_UP: [(f64, f64); 2] = [(0.0, 0.0), (1.0, 1.0)];

/// The predicate and sign of each decision a build took.
fn answers(
    d: &[geom_core::sym::revalue::Decision],
) -> Vec<(&'static str, Option<geom_core::Sign>)> {
    d.iter().map(|d| (d.predicate, d.sign)).collect()
}

/// How many `segment_direction_axis` readings a build kept.
fn axis_readings(d: &[geom_core::sym::revalue::Decision]) -> usize {
    d.iter()
        .filter(|d| d.predicate == "segment_direction_axis")
        .count()
}

/// **A coincidence of values takes one branch at every valuation, in
/// both sessions** (ERROR-DESIGN E12): a hop between two corners whose
/// `x` are different parameters, equal at the nominal, has a chord
/// component that reads exactly `0.0` there and is no identity. Built at
/// `Sym<f64>` at the nominal and with one corner moved, every decision
/// the two builds take answers the same sign, and both store the
/// quotient: a mint that branched on the component's value would store a
/// constant at the nominal where the computation varies with the
/// parameters (what the five `revalue_corpus` rows caught).
#[test]
fn a_value_coincident_hop_takes_one_branch_at_every_valuation() {
    use geom_core::sym::revalue::record;
    for kind in SESSIONS {
        let build = |x2: f64| record(|| run(kind, || quad_hop(pf("x1", 1.0), pf("x2", x2))));
        let ((d0, at0), (d1, at1)) = (build(1.0), build(1.0 + 1.0e-7));
        assert_eq!(answers(&at0), answers(&at1), "{kind:?}: one branch");
        // The quad's three other sides are literal axes, two readings
        // kept each; this side keeps none.
        assert_eq!(axis_readings(&at0), 6, "{kind:?}: none kept on this side");
        assert_eq!(d0[0], (0.0, 0.0), "{kind:?}: the nominal's quotient");
        assert!(d1[0].0 > 0.0, "{kind:?}: the moved build leans: {d1:?}");
    }
}

/// **An identity that reads Zero mints its axis in both sessions, the
/// deferring one by discharge, and keeps its two readings**: the hop
/// `(w, 0) → (w, 1)` at `Sym<f64>`. In a deferring session the Zero is
/// stamped with its place on the list and proven only when the mint asks
/// (`geom_core::sym::deferred::discharge`); a mint that did not ask
/// there would store the quotient. The two readings the axis stands on
/// are kept in the decision record, beside the quad's three literal
/// axis sides' two each; a mint that discarded them would leave none.
#[test]
fn an_identity_that_reads_zero_mints_its_axis_in_both_sessions_and_keeps_its_readings() {
    use geom_core::sym::revalue::record;
    for kind in SESSIONS {
        let (d, at) = record(|| {
            run(kind, || {
                let w = pf("w", 1.0);
                quad_hop(w, w)
            })
        });
        assert_eq!(d, AXIS_UP, "{kind:?}: the axis");
        // Two readings kept on every side, this one's among them.
        assert_eq!(axis_readings(&at), 8, "{kind:?}: the readings kept");
    }
}

/// **Pinned: an identity whose `f64` value rounds takes the quotient**,
/// at `Sym<f64>` in both sessions (the replay's lane): the hop
/// `(x, 0) → ((x + w) − w, 1)`, whose chord `x` is zero as a theorem.
/// At `(x, w) = (1, 1)` `f64` computes it as `0.0`, the tier proves the
/// Zero and the mint stores the axis; at `(0.1, 0.3)` it rounds to
/// `2.8e-17`, a definite sign the tier is never asked about, and the
/// mint stores the quotient. Today's truth, not the goal: [`RESIDUAL`].
#[test]
fn an_identity_that_rounds_takes_the_axis_only_where_its_value_reads_zero() {
    let rounds = |x: f64, w: f64| ((x + w) - w) - x;
    assert_eq!(rounds(1.0, 1.0), 0.0);
    assert_ne!(rounds(0.1, 0.3), 0.0);
    for kind in SESSIONS {
        let at = |x: f64, w: f64| {
            run(kind, || {
                let (x, w) = (pf("x", x), pf("w", w));
                quad_hop(x, (x + w) - w)
            })
        };
        assert_eq!(at(1.0, 1.0), AXIS_UP, "{kind:?}: reads zero: the axis");
        let d = at(0.1, 0.3);
        assert!(
            d[0].0 > 0.0 && d[0].0 == d[0].1,
            "{kind:?}: rounds: the quotient, one ulp off the axis ({RESIDUAL}): {d:?}"
        );
    }
}

/// **Pinned: the slab's side takes the quotient in a deferring session
/// over a box**: `W − W` over a box straddles zero, the least band
/// escalates, a deferring session returns the escalation without asking
/// the tier, and the mint reads no Zero to discharge. An eager session
/// hands the escalation to the tier, which proves it; over a point box
/// both read `[0, 0]`, a Zero the deferring session discharges. Latent
/// (the one production deferring session is `Sym<f64>`); today's truth,
/// not the goal: [`RESIDUAL`].
#[test]
fn the_slabs_side_takes_the_quotient_in_a_deferring_session_over_a_box() {
    let side = |kind: Session, h: f64| {
        run(kind, || {
            let w = param("W", 1.0, h);
            quad_hop(w, w)
        })
    };
    assert_eq!(side(Session::Eager, small()), AXIS_UP, "eager, a box");
    assert_eq!(side(Session::Eager, 0.0), AXIS_UP, "eager, a point");
    assert_eq!(side(Session::Deferring, 0.0), AXIS_UP, "deferring, a point");
    let d = side(Session::Deferring, small());
    assert!(
        d[0].0 < 0.0 && 0.0 < d[0].1,
        "deferring, a box: the quotient, straddling the axis ({RESIDUAL}): {d:?}"
    );
}

/// **Pinned: a `max` in a corner takes the branch its point box
/// certifies**: the hop `(max(a, b), 0) → (b, 1)` at `Sym<Interval>` over
/// point boxes mints the axis where `a < b` (the tier reads the `max`'s
/// comparison off the box and proves `max(a, b) − b` zero) and the
/// quotient where `a > b` (where the chord is `b − a`, no zero). That is
/// the tier's own reading of a comparison, which DECIDE's row on asking
/// for an identity weighs
/// (`work/decide/a-construction-can-ask-whether-a-value-is-an-identity-only-by-taking-a-sign-reading.md`).
#[test]
fn a_max_in_a_corner_takes_the_branch_its_point_box_certifies() {
    let at = |a: f64| {
        in_session(|| {
            let (a, b) = (param("a", a, 0.0), param("b", 1.0, 0.0));
            quad_hop(Real::max(a, b), b)
        })
    };
    assert_eq!(at(0.5), AXIS_UP, "a < b: the axis");
    let d = at(1.5);
    assert!(
        d[0].1 < 0.0,
        "a > b: the quotient, (b − a) leaning back: {d:?}"
    );
}

/// **A radial takes one branch at every valuation, in both sessions**:
/// an arc about `(cx, 2)` from `(ax, 0)`, where `ax` and `cx` are
/// different parameters equal at the nominal, so the start radial's `x`
/// reads exactly `0.0` there and is no identity. The centred arc records
/// no direction, so validation mints both radials; the end radial
/// `(cx + 2, 2) − (cx, 2)` has its `y` an identity. Moved apart, every
/// decision answers alike, the start radial takes the quotient at both
/// valuations and the end radial the axis at both.
#[test]
fn a_radial_takes_one_branch_at_every_valuation() {
    use geom_core::sym::revalue::record;
    for kind in SESSIONS {
        let build = |ax: f64| {
            record(|| {
                run(kind, || {
                    let (ax, cx) = (pf("ax", ax), pf("cx", 5.0));
                    let (z, two) = (Sym::from_f64(0.0), Sym::from_f64(2.0));
                    let lp = Open
                        .at(Point2::new(ax, z))
                        .arc_to(
                            profile::Center {
                                c: Point2::new(cx, two),
                                winding: ArcSweep::Ccw,
                                p: Point2::new(cx + two, two),
                            },
                            tol(),
                        )
                        .unwrap()
                        .line_to(Start, tol())
                        .unwrap();
                    let vp: ValidatedProfile<Sym<f64>> = validated(vec![lp.loop_]);
                    let arc = vp.loops()[0]
                        .segments()
                        .iter()
                        .find(|s| matches!(s.kind, profile::SegmentKind::Arc { .. }))
                        .expect("the arc")
                        .kind;
                    let (s, e) = arc_radials(arc);
                    (bounds(s), bounds(e))
                })
            })
        };
        let (((s0, e0), at0), ((s1, e1), at1)) = (build(5.0), build(5.0 + 1.0e-7));
        assert_eq!(answers(&at0), answers(&at1), "{kind:?}: one branch");
        assert_eq!(
            s0[0],
            (0.0, 0.0),
            "{kind:?}: the nominal start radial's quotient"
        );
        assert!(s1[0].0 != 0.0, "{kind:?}: moved, it leans: {s1:?}");
        for e in [e0, e1] {
            assert_eq!(
                e,
                [(1.0, 1.0), (0.0, 0.0)],
                "{kind:?}: the end radial's axis"
            );
        }
    }
}

/// **At `f64` a minted direction is the quotient, and these are its
/// bits**: nothing is proven at `f64`, so every direction validation
/// mints is `(end − start)/|·|` for a side, `(v − c)/|·|` for a radial.
/// Where a component is zero that is `±0`, its sign the sign of the
/// difference (`b − a` is `−0` exactly where `b` is `−0` and `a` is
/// `+0`), and the other component is `±1` exactly (`√(0 + y²)` is
/// `|y|`). The axis mint PR 6963 shipped stored `+0` there in every case,
/// so the two differ only where one end holds `−0.0` and the other
/// `+0.0`: value-equal, and nothing outside `profile` reads a direction.
/// Rectangles of either winding and of every scale, corners on the
/// signed zeros among them, and centred arcs whose radials lie on the
/// axes.
#[test]
fn at_f64_a_minted_direction_is_the_quotient_with_its_zeros_sign() {
    let check = |vp: &ValidatedProfile<f64>, what: &str| {
        for (k, s) in vp.loops()[0].segments().iter().enumerate() {
            let want = |v: Vec2<f64>| v / v.norm();
            match s.kind {
                profile::SegmentKind::Line { director: d } => {
                    let c = s.end - s.start;
                    assert_eq!(bits(d), bits(want(c)), "{what}: side {k}");
                    if c.x == 0.0 {
                        assert_eq!(d.x.to_bits(), c.x.to_bits(), "{what}: side {k}'s zero");
                        assert_eq!(d.y.abs(), 1.0, "{what}: side {k}");
                    }
                    if c.y == 0.0 {
                        assert_eq!(d.y.to_bits(), c.y.to_bits(), "{what}: side {k}'s zero");
                        assert_eq!(d.x.abs(), 1.0, "{what}: side {k}");
                    }
                }
                profile::SegmentKind::Arc {
                    arc,
                    radials: [start, end],
                    ..
                } => {
                    for (v, r) in [(s.start, start), (s.end, end)] {
                        assert_eq!(bits(r), bits(want(v - arc.centre)), "{what}: arc {k}");
                    }
                }
            }
        }
    };
    let rect = |x0: f64, y0: f64, w: f64, h: f64| {
        Open.at(Point2::new(x0, y0))
            .line_to(Point2::new(x0 + w, y0), tol())
            .unwrap()
            .line_to(Point2::new(x0 + w, y0 + h), tol())
            .unwrap()
            .line_to(Point2::new(x0, y0 + h), tol())
            .unwrap()
            .line_to(Start, tol())
            .unwrap()
    };
    let mut seed = 0x9e37_79b9_7f4a_7c15u64;
    let mut next = || {
        seed ^= seed << 13;
        seed ^= seed >> 7;
        seed ^= seed << 17;
        (seed >> 11) as f64 / (1u64 << 53) as f64
    };
    for _ in 0..200 {
        let scale = 10f64.powf(6.0 * next() - 3.0);
        let (x0, y0) = ((next() - 0.5) * 8.0 * scale, (next() - 0.5) * 8.0 * scale);
        let (w, h) = ((0.1 + next()) * scale, (0.1 + next()) * scale);
        let (w, h) = (
            if next() < 0.5 { w } else { -w },
            if next() < 0.5 { h } else { -h },
        );
        check(
            &validated(vec![rect(x0, y0, w, h).loop_]),
            &format!("rect {x0:e} {y0:e} {w:e} {h:e}"),
        );
    }
    // Corners on the signed zeros: a side from `+0` to `−0` stores `−0`,
    // where the axis mint stored `+0`.
    for (x0, y0) in [(0.0, 0.0), (-0.0, 0.0), (0.0, -0.0), (-0.0, -0.0)] {
        for (w, h) in [(1.0, 2.0), (-1.0, 2.0), (1.0, -2.0)] {
            check(
                &validated(vec![rect(x0, y0, w, h).loop_]),
                &format!("rect at ({x0:?}, {y0:?})"),
            );
        }
    }
    let quad = Open
        .at(Point2::new(-0.0, 0.0))
        .line_to(Point2::new(1.0, 0.0), tol())
        .unwrap()
        .line_to(Point2::new(1.0, 1.0), tol())
        .unwrap()
        .line_to(Point2::new(0.0, 1.0), tol())
        .unwrap()
        .line_to(Start, tol())
        .unwrap();
    let vp = validated(vec![quad.loop_]);
    check(&vp, "a side from +0 to −0");
    let last = line_director(vp.loops()[0].segments().last().unwrap().kind);
    assert_eq!(
        bits(last),
        bits(Vec2::new(-0.0, -1.0)),
        "the side (0, 1) → (−0, 0) stores −0 in its zero component"
    );
    // Centred arcs whose radials lie on the axes, the centre on a signed
    // zero.
    for cx in [0.0, -0.0] {
        let lp = Open
            .at(Point2::new(cx, -2.0))
            .arc_to(
                profile::Center {
                    c: Point2::new(cx, 0.0),
                    winding: ArcSweep::Ccw,
                    p: Point2::new(2.0, 0.0),
                },
                tol(),
            )
            .unwrap()
            .line_to(Start, tol())
            .unwrap();
        check(
            &validated(vec![lp.loop_]),
            &format!("arc about ({cx:?}, 0)"),
        );
    }
}

/// One arc verb's loop as its door stores it, `q` the quantity that
/// fixes the arc's radius: a radius, a bulge, a through-point's or a
/// target's coordinate, a leg's length.
fn arc_verb_loop<T: Decide>(verb: &str, q: T) -> ProfileLoop<T> {
    let t = tol();
    let l = T::from_f64;
    let p = |x: f64, y: f64| Point2::new(l(x), l(y));
    match verb {
        "circle" => profile::circle(p(0.3, 0.2), q, t).unwrap().loop_.into_loop(),
        "split circle" => profile::circle_split(p(1.0, 0.5), q, 3, l(0.25), t)
            .unwrap()
            .loop_
            .into_loop(),
        "bulge" | "bulge cw" => Open
            .at(p(0.0, 0.0))
            .line_to(p(4.0, 0.0), t)
            .unwrap()
            .arc_to(
                profile::Bulge {
                    p: p(4.0, 4.0),
                    b: q,
                },
                t,
            )
            .unwrap()
            .line_to(p(0.0, 4.0), t)
            .unwrap()
            .line_to(Start, t)
            .unwrap()
            .loop_
            .into_loop(),
        "via" | "via cw" => Open
            .at(p(0.0, 0.0))
            .line_to(p(4.0, 0.0), t)
            .unwrap()
            .arc_to(
                profile::Via {
                    q: Point2::new(q, l(2.0)),
                    p: p(4.0, 4.0),
                },
                t,
            )
            .unwrap()
            .line_to(p(0.0, 4.0), t)
            .unwrap()
            .line_to(Start, t)
            .unwrap()
            .loop_
            .into_loop(),
        "tangent" | "tangent cw" => {
            let far = if verb == "tangent" { 2.0 } else { -2.0 };
            Open.at(p(0.0, 0.0))
                .line_to(p(2.0, 0.0), t)
                .unwrap()
                .tangent()
                .tangent_arc_to(Point2::new(l(3.0), q), t)
                .unwrap()
                .line_to(p(0.0, far), t)
                .unwrap()
                .line_to(Start, t)
                .unwrap()
                .loop_
                .into_loop()
        }
        "onto" => Open
            .at(p(0.0, 0.0))
            .toward(l(0.0), l(1.0), t)
            .unwrap()
            .line(q, t)
            .unwrap()
            .toward(l(1.0), l(0.0), t)
            .unwrap()
            .tangent_arc_to(p(0.25, 1.0), t)
            .unwrap()
            .arc_onto(Start, t)
            .unwrap()
            .toward(l(-1.0), l(0.0), t)
            .unwrap()
            .loop_
            .into_loop(),
        "fillet" => {
            let (east, north) = (0.0_f64, core::f64::consts::FRAC_PI_2);
            let (west, south) = (core::f64::consts::PI, -north);
            Open.at(p(0.0, -1.0))
                .angle(l(east), t)
                .unwrap()
                .fillet(q, t)
                .unwrap()
                .at(p(1.0, 0.0), t)
                .unwrap()
                .angle(l(north), t)
                .unwrap()
                .fillet(q, t)
                .unwrap()
                .at(p(0.0, 1.0), t)
                .unwrap()
                .angle(l(west), t)
                .unwrap()
                .fillet(q, t)
                .unwrap()
                .at(p(-1.0, 0.0), t)
                .unwrap()
                .angle(l(south), t)
                .unwrap()
                .fillet(q, t)
                .unwrap()
                .to(Start, t)
                .unwrap()
                .loop_
                .into_loop()
        }
        other => panic!("no such verb: {other}"),
    }
}

/// Each arc verb with the box its defining quantity ranges over.
const ARC_VERBS: [(&str, f64, f64); 10] = [
    ("circle", 0.05, 0.005),
    ("split circle", 0.4, 0.04),
    ("bulge", 0.4, 0.05),
    ("bulge cw", -0.3, 0.05),
    ("via", 5.0, 0.1),
    ("via cw", 3.5, 0.1),
    ("tangent", 1.0, 0.1),
    ("tangent cw", -1.0, 0.1),
    ("onto", 2.0, 0.05),
    ("fillet", 0.25, 0.02),
];

/// PROBE: what each verb's stored arc reads over a box.
#[test]
#[ignore = "probe"]
fn probe_each_arc_verbs_radius_along_its_radial() {
    let band = geom_core::Band::linear(tol()).unwrap();
    for (verb, c, h) in ARC_VERBS {
        let rows = std::panic::catch_unwind(|| {
            in_session(|| {
                let lp = arc_verb_loop::<Lane>(verb, param("q", c, h));
                let n = lp.vertices().len();
                let mut rows = Vec::new();
                for k in 0..n {
                    let Segment::Arc(arc) = lp.segments()[k] else {
                        continue;
                    };
                    let (a, b) = (lp.vertices()[k], lp.vertices()[(k + 1) % n]);
                    let sign = |m: Lane| {
                        let w = m.hi() - m.lo();
                        (m.sign_within(band).map(|d| d.sign).ok(), w)
                    };
                    let radius = (arc.radius.lo(), arc.radius.hi());
                    let gaps = [sign(arc.off_carrier(a)), sign(arc.off_carrier(b))];
                    let rims = [sign(arc.rim(a) - arc.radius), sign(arc.rim(b) - arc.radius)];
                    let along = lp.directions()[k].map(|d| {
                        let (s, e) = radials(d);
                        let (ma, mb) = (arc.on_radial(s) - a, arc.on_radial(e) - b);
                        [sign(ma.x), sign(ma.y), sign(mb.x), sign(mb.y)]
                    });
                    rows.push(format!(
                        "   seg {k}: radius {radius:?}\n      gap {gaps:?}\n      rim-radius {rims:?}\n      along {along:?}"
                    ));
                }
                rows
            })
        });
        println!("== {verb} over {c} ± {h}");
        match rows {
            Ok(rows) => rows.iter().for_each(|r| println!("{r}")),
            Err(_) => println!("   THE DOOR REFUSED OR PANICKED OVER THE BOX"),
        }
    }
}
