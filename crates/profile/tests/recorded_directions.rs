//! **A validated segment records the unit directions its construction
//! made**: a line's director, an arc's radial at each end
//! (`crates/profile/README.md`, the profile format). A verb feeds what it
//! holds exactly; a hop between two points is given its director by the
//! one mint, in the verb; an arc that holds only its points has its
//! radials minted at validate; a table's recorded direction is checked
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
use profile::test_support::arc_verb_loop;
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
            // The centre plus the stored radius along each radial IS the
            // vertex through the tier: the radius is stored as authored,
            // so the difference is the zero polynomial.
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
/// points has its director minted by its verb, and where the two
/// points share a variable and no component is zero it is stored with
/// the width of the quotient
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
/// and the mint asks the chord's `W − W` whether it is zero as an
/// expression, where the quotient would carry the box's width twice.
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

/// What a point-to-point verb records at `f64` for the hop `a → b`: the
/// chord over its length.
fn hop(a: (f64, f64), b: (f64, f64)) -> Option<SegmentDirections<f64>> {
    Some(SegmentDirections::Line(
        (Point2::new(b.0, b.1) - Point2::new(a.0, a.1)).normalize(),
    ))
}

/// `lp`'s recorded directions are `want`, bit for bit, `None` included.
fn records(lp: &ProfileLoop<f64>, want: &[Option<SegmentDirections<f64>>], what: &str) {
    let got: Vec<_> = lp.directions().iter().map(|&d| recorded_bits(d)).collect();
    let want: Vec<_> = want.iter().map(|&d| recorded_bits(d)).collect();
    assert_eq!(got, want, "{what}: {:?}", lp.directions());
}

/// **What each verb records on its loop**, before validation reads it:
/// the exact direction where the verb holds one, a hop's minted
/// director (at `f64` its chord over its length, [`hop`]), and `None`
/// where a verb holds only points and no mint of its own (validation
/// then mints it). A verb that stopped recording, or one that recorded
/// where it holds no direction, goes red here; the validated rows above
/// cannot see either, since validation mints a missing direction to the
/// same value.
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
    // A leg along a director records it and a hop its minted director;
    // a leg to a point target continuing a director records nothing.
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
        &[
            line_rec(1.0, 0.0),
            None,
            hop((3.0, 0.0), (3.0, 1.0)),
            hop((3.0, 1.0), (0.0, 0.0)),
        ],
        "directed leg, point target, hops",
    );
    // Bulge and via: the lowering's own radials at the literal sign.
    let (a, b) = (Point2::new(4.0, 0.0), Point2::new(4.0, 4.0));
    let lowered = |bulge: f64| {
        let [s, e] =
            Arc2::from_chord_radials(a, b, bulge, geom_core::ArcSweep::of_constant(bulge)).1;
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
            &[
                hop((0.0, 0.0), (4.0, 0.0)),
                lowered(bulge),
                hop((4.0, 4.0), (0.0, 4.0)),
                hop((0.0, 4.0), (0.0, 0.0)),
            ],
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
        &[
            hop((0.0, 0.0), (4.0, 0.0)),
            lowered(profile::bulge_from_via(a, q, b)),
            hop((4.0, 4.0), (0.0, 4.0)),
            hop((0.0, 4.0), (0.0, 0.0)),
        ],
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
    records(
        &center.loop_.into_loop(),
        &[
            hop((0.0, 0.0), (4.0, 0.0)),
            None,
            hop((4.0, 4.0), (0.0, 4.0)),
            hop((0.0, 4.0), (0.0, 0.0)),
        ],
        "centred arc",
    );
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
        &[
            hop((0.0, 0.0), (2.0, 0.0)),
            arc_rec(x(0.0, -1.0), x(1.0, -0.0)),
            hop((3.0, 1.0), (0.0, 2.0)),
            hop((0.0, 2.0), (0.0, 0.0)),
        ],
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
    // not a direction: nothing recorded. The hops beside them record
    // their authored chord's director, whatever a fillet trims off the
    // segment.
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
    records(
        &side.loop_.into_loop(),
        &[
            None,
            None,
            None,
            hop((4.0, 0.0), (4.0, 3.0)),
            hop((4.0, 3.0), (-1.0, 3.0)),
            hop((-1.0, 3.0), (0.0, 2.0)),
        ],
        "arc_fillet",
    );
}

// ---- the direction mint: one document, one branch ---------------------

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

/// The quadrilateral (0, 0) → (x1, 0) → (x2, y) → (0, y), as its verbs
/// answer it.
fn quad<T: Decide>(x1: T, x2: T, y: T) -> Result<ConstructedLoop<T>, profile::PathError<T>> {
    let z = T::from_f64(0.0);
    Ok(Open
        .at(Point2::new(z, z))
        .line_to(Point2::new(x1, z), tol())?
        .line_to(Point2::new(x2, y), tol())?
        .line_to(Point2::new(z, y), tol())?
        .line_to(Start, tol())?
        .loop_)
}

/// The gate a verb's escalation names, or what the verb answered
/// instead, rendered.
fn escalated_gate<T: Real>(
    built: Result<ConstructedLoop<T>, profile::PathError<T>>,
) -> Result<Gate, String> {
    match built {
        Err(profile::PathError::Escalated { gate, .. }) => Ok(gate),
        Err(other) => Err(format!("{other:?}")),
        Ok(_) => Err("built".to_owned()),
    }
}

/// [`quad`]'s second side's stored director, the quad one unit tall.
fn quad_director<T: Decide>(x1: T, x2: T) -> Vec2<T> {
    let lp = quad(x1, x2, T::from_f64(1.0)).expect("the quad builds");
    line_director(validated(vec![lp]).loops()[0].segments()[1].kind)
}

/// [`quad_director`], as bounds.
fn quad_hop<T: Decide + Bounds>(x1: T, x2: T) -> [(f64, f64); 2] {
    bounds(quad_director(x1, x2))
}

const AXIS_UP: [(f64, f64); 2] = [(0.0, 0.0), (1.0, 1.0)];

/// The predicate and sign of each decision a build took.
fn answers(
    d: &[geom_core::sym::revalue::Decision],
) -> Vec<(&'static str, Option<geom_core::Sign>)> {
    d.iter().map(|d| (d.predicate, d.sign)).collect()
}

/// The signs of the `segment_direction_axis` readings a build took: one
/// per director stored as an axis, `None` for one that escalated.
fn axis_readings(d: &[geom_core::sym::revalue::Decision]) -> Vec<Option<geom_core::Sign>> {
    d.iter()
        .filter(|d| d.predicate == "segment_direction_axis")
        .map(|d| d.sign)
        .collect()
}

/// The three literal axis sides of [`quad`], each signed by one
/// reading of its other component: +x, then −x, then −y.
const LITERAL_SIDES: [Option<geom_core::Sign>; 3] = [
    Some(geom_core::Sign::Positive),
    Some(geom_core::Sign::Negative),
    Some(geom_core::Sign::Negative),
];

/// **A coincidence of values takes one branch at every valuation, in
/// both sessions** (ERROR-DESIGN E12): a hop between two corners whose
/// `x` are different parameters, equal at the nominal, has a chord
/// component that reads exactly `0.0` there and is no identity. Built at
/// `Sym<f64>` at the nominal and with one corner moved, every decision
/// the two builds take answers the same sign, and both store the
/// quotient and read no sign for this side. A mint that took a
/// component reading `0.0` for a zero stores a constant at the nominal
/// where the computation varies with the parameters, and is red on the
/// readings.
#[test]
fn a_value_coincident_hop_takes_one_branch_at_every_valuation() {
    use geom_core::sym::revalue::record;
    for kind in SESSIONS {
        let build = |x2: f64| record(|| run(kind, || quad_hop(pf("x1", 1.0), pf("x2", x2))));
        let ((d0, at0), (d1, at1)) = (build(1.0), build(1.0 + 1.0e-7));
        assert_eq!(answers(&at0), answers(&at1), "{kind:?}: one branch");
        assert_eq!(
            axis_readings(&at0),
            LITERAL_SIDES,
            "{kind:?}: no sign read here"
        );
        assert_eq!(d0[0], (0.0, 0.0), "{kind:?}: the nominal's quotient");
        assert!(d1[0].0 > 0.0, "{kind:?}: the moved build leans: {d1:?}");
    }
}

/// **One document, one branch: an identity mints its axis at every
/// valuation, in both sessions.** The hop `(x, 0) → ((x + w) − w, 1)` at
/// `Sym<f64>`, whose chord `x` is zero as an expression. At
/// `(x, w) = (1, 1)` `f64` computes it as `0.0`; at `(0.1, 0.3)` it
/// rounds to `2.8e-17`. Both store the axis, sign it by one reading of
/// the other component, and take the same decisions with the same
/// answers; so does the hop `(w, 0) → (w, 1)`.
///
/// A mint that asks the expression only where the value channel reads
/// zero (what a sign reading does: the tier is consulted only where the
/// value reads no definite sign) stores the quotient at `(0.1, 0.3)`,
/// one ulp off the axis, and is red there.
#[test]
fn an_identity_that_rounds_mints_its_axis_at_every_valuation() {
    use geom_core::sym::revalue::record;
    let rounds = |x: f64, w: f64| ((x + w) - w) - x;
    assert_eq!(rounds(1.0, 1.0), 0.0);
    assert_ne!(rounds(0.1, 0.3), 0.0);
    let up = [
        LITERAL_SIDES[0],
        LITERAL_SIDES[0],
        LITERAL_SIDES[1],
        LITERAL_SIDES[2],
    ];
    for kind in SESSIONS {
        let at = |x: f64, w: f64| {
            record(|| {
                run(kind, || {
                    let (x, w) = (pf("x", x), pf("w", w));
                    quad_hop(x, (x + w) - w)
                })
            })
        };
        let ((d0, at0), (d1, at1)) = (at(1.0, 1.0), at(0.1, 0.3));
        assert_eq!(d0, AXIS_UP, "{kind:?}: reads zero: the axis");
        assert_eq!(d1, AXIS_UP, "{kind:?}: rounds: the axis");
        assert_eq!(answers(&at0), answers(&at1), "{kind:?}: one branch");
        assert_eq!(axis_readings(&at1), up, "{kind:?}: one sign read per axis");
        let (d, same) = record(|| {
            run(kind, || {
                let w = pf("w", 1.0);
                quad_hop(w, w)
            })
        });
        assert_eq!(d, AXIS_UP, "{kind:?}: one variable twice");
        assert_eq!(axis_readings(&same), up, "{kind:?}: one variable twice");
    }
}

/// **The slab's side is proved in every symbolic session, over a box
/// and over a point, and is never read as a sign**: `W − W` over a box
/// straddles zero, so a sign reading of it escalates, and a deferring
/// session returns the escalation without asking the tier. The mint
/// asks the expression: the axis in all four, and every
/// `segment_direction_axis` reading the build takes is of an axis's
/// other component and is definite.
///
/// A mint that learned the zero from a sign reading stores the
/// quotient, straddling the axis, in the deferring session over a box,
/// and is red there. A plain `Interval` carries no expression: there
/// the side is the quotient, as wide as the box makes it.
#[test]
fn the_slabs_side_is_proved_in_every_session_and_never_escalates() {
    use geom_core::sym::revalue::record;
    let side = |kind: Session, h: f64| {
        record(|| {
            run(kind, || {
                let w = param("W", 1.0, h);
                quad_hop(w, w)
            })
        })
    };
    let cases = SESSIONS.map(|kind| [small(), 0.0].map(|h| (kind, h, side(kind, h))));
    for (kind, h, (d, _)) in cases.iter().flatten() {
        assert_eq!(*d, AXIS_UP, "{kind:?}, half-width {h:e}");
    }
    for (kind, h, (_, readings)) in cases.iter().flatten() {
        assert_eq!(
            axis_readings(readings),
            [
                LITERAL_SIDES[0],
                LITERAL_SIDES[0],
                LITERAL_SIDES[1],
                LITERAL_SIDES[2]
            ],
            "{kind:?}, half-width {h:e}: every reading definite"
        );
    }
    let w = Interval::from_bounds(1.0 - small(), 1.0 + small());
    let d = quad_hop(w, w);
    assert!(
        d[0].0 < 0.0 && 0.0 < d[0].1,
        "a plain interval: the quotient, straddling the axis: {d:?}"
    );
}

/// **A `max` in a corner is no identity, on either side of its
/// comparison**: the hop `(max(a, b), 0) → (b, 1)` at `Sym<Interval>`
/// over point boxes. Where `a < b` the chord's `x`, `max(a, b) − b`,
/// reads `[0, 0]` as a value; the question reads no value, so not the
/// arm the `max` takes either, and the mint stores the quotient, whose
/// value is the axis's. Where `a > b` the chord is `b − a`, the quotient
/// leaning back. No sign is read for this side on either.
///
/// A mint that took a component's Zero for proof is red where `a < b`
/// (a fourth reading).
#[test]
fn a_max_in_a_corner_is_no_identity_the_mint_proves() {
    let at = |a: f64| {
        geom_core::sym::revalue::record(|| {
            in_session(|| {
                let (a, b) = (param("a", a, 0.0), param("b", 1.0, 0.0));
                quad_hop(Real::max(a, b), b)
            })
        })
    };
    let (d, kept) = at(0.5);
    assert_eq!(d, AXIS_UP, "a < b: the quotient, whose value is the axis's");
    assert_eq!(
        axis_readings(&kept),
        LITERAL_SIDES,
        "a < b: no sign read here"
    );
    let (d, kept) = at(1.5);
    assert_eq!(
        axis_readings(&kept),
        LITERAL_SIDES,
        "a > b: no sign read here"
    );
    assert!(
        d[0].1 < 0.0,
        "a > b: the quotient, (b − a) leaning back: {d:?}"
    );
}

/// **The axis is signed by one reading of the other component, and
/// where that reading does not decide the verb refuses.** The hop
/// `(W, 0) → (W, H)` with `H` over a box:
///
/// - `H` definitely negative: the axis `(0, −1)`. A mint that signed
///   every axis `+1` is red here.
/// - `H` straddling zero: `line_to` refuses with the reading's
///   escalation under `segment_direction_axis`, in an eager and in a
///   deferring session, and does not store a quotient whose enclosure
///   is the whole circle. A mint that fell back to the quotient where
///   the reading escalates is red here.
///
/// At a plain `Interval` the same hop reads no sign (it carries no
/// expression): what refuses the chord that may have no length is the
/// junction's own gate, on the quotient.
#[test]
fn the_axis_is_signed_by_one_reading_and_an_undecided_sign_refuses() {
    for kind in SESSIONS {
        let d = run(kind, || {
            let w = param("W", 1.0, small());
            let lp = quad(w, w, param("H", -1.0, small())).expect("the quad builds");
            let vp = validated(vec![lp]);
            // Wound clockwise as drawn, so the outer loop is stored
            // reversed: the drawn hop is the stored one negated.
            vp.loops()[0]
                .segments()
                .iter()
                .map(|s| bounds(line_director(s.kind)))
                .find(|d| d[0] == (0.0, 0.0) && d[1].0 > 0.0)
        });
        assert_eq!(
            d,
            Some(AXIS_UP),
            "{kind:?}: the drawn (0, −1), stored reversed"
        );
        let refused = run(kind, || {
            let w = param("W", 1.0, small());
            escalated_gate(quad(w, w, param("H", 0.0, 0.5)))
        });
        assert_eq!(
            refused,
            Ok(Gate::SegmentDirectionAxis),
            "{kind:?}: the undecided sign refuses"
        );
    }
    let w = Interval::from_bounds(1.0 - small(), 1.0 + small());
    assert_eq!(
        escalated_gate(quad(w, w, Interval::from_bounds(-0.5, 0.5))),
        Ok(Gate::PathJunctionTurn),
        "a plain interval reads no sign"
    );
}

/// **A symbolic scalar over a dual mints the axis, with no tangent**:
/// `Sym<Dual64>` carries an expression, the hop `(w, 0) → (w, 1)` is
/// proved as at any symbolic scalar, and the stored director is the
/// literal `(0, 1)`, whose derivative along `w` is zero, as the
/// direction's is. A plain `Dual64` carries none and stores the
/// quotient, whose value and tangent are the same.
#[test]
fn a_symbolic_dual_mints_the_axis_and_a_plain_dual_the_quotient() {
    use geom_core::Dual64;
    let parts = |d: Vec2<Dual64>| [(d.x.value, d.x.deriv), (d.y.value, d.y.deriv)];
    let literal = [(0.0, 0.0), (1.0, 0.0)];
    let sym = in_session(|| {
        let w = Sym::param(
            ParamSymbol::new(test_utils::symbol_id("w")),
            Dual64::variable(1.0),
        );
        let d = quad_director(w, w);
        Vec2::new(d.x.value(), d.y.value())
    });
    assert_eq!(parts(sym), literal, "Sym<Dual64>: the axis");
    let w = Dual64::variable(1.0);
    assert_eq!(parts(quad_director(w, w)), literal, "Dual64: the quotient");
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
/// bits**: `f64` carries no expression, so no axis is ever stored. An
/// arc's radial is validation's `(v − c)/|·|`. A hop's director is its
/// verb's: the drawn chord over its length, and where validation stores
/// the loop reversed (drawn clockwise) that director negated. Where a
/// component is zero that is `±0`, the sign of the drawn difference
/// (`b − a` is `−0` exactly where `b` is `−0` and `a` is `+0`), flipped
/// by a reversal; the other component is `±1` exactly (`√(0 + y²)` is
/// `|y|`). So against the quotient of the stored ends, a hop's director
/// differs only in the sign of a zero, on a reversed loop: value-equal.
/// Rectangles of either winding and of every scale, corners on the
/// signed zeros among them, and centred arcs whose radials lie on the
/// axes.
#[test]
fn at_f64_a_minted_direction_is_the_quotient_with_its_zeros_sign() {
    let check = |vp: &ValidatedProfile<f64>, reversed: bool, what: &str| {
        for (k, s) in vp.loops()[0].segments().iter().enumerate() {
            let want = |v: Vec2<f64>| v / v.norm();
            match s.kind {
                profile::SegmentKind::Line { director: d } => {
                    let stored = s.end - s.start;
                    let drawn = if reversed { s.start - s.end } else { stored };
                    let minted = if reversed { -want(drawn) } else { want(drawn) };
                    assert_eq!(bits(d), bits(minted), "{what}: side {k}");
                    assert!(
                        d.equals_exactly(want(stored)),
                        "{what}: side {k}: value-equal to the stored ends' quotient"
                    );
                    let zero = |c: f64| if reversed { -c } else { c }.to_bits();
                    if drawn.x == 0.0 {
                        assert_eq!(d.x.to_bits(), zero(drawn.x), "{what}: side {k}'s zero");
                        assert_eq!(d.y.abs(), 1.0, "{what}: side {k}");
                    }
                    if drawn.y == 0.0 {
                        assert_eq!(d.y.to_bits(), zero(drawn.y), "{what}: side {k}'s zero");
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
            w * h < 0.0,
            &format!("rect {x0:e} {y0:e} {w:e} {h:e}"),
        );
    }
    // Corners on the signed zeros: a side drawn from `+0` to `−0` stores
    // `−0`.
    for (x0, y0) in [(0.0, 0.0), (-0.0, 0.0), (0.0, -0.0), (-0.0, -0.0)] {
        for (w, h) in [(1.0, 2.0), (-1.0, 2.0), (1.0, -2.0)] {
            check(
                &validated(vec![rect(x0, y0, w, h).loop_]),
                w * h < 0.0,
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
    check(&vp, false, "a side from +0 to −0");
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
            false,
            &format!("arc about ({cx:?}, 0)"),
        );
    }
}

/// **Each arc verb's stored arc is exact over a box on the quantity
/// that fixes its radius.** For the circle, the split circle, a bulge
/// leg of either sign, a via arc on either side of its chord, a tangent
/// arc to either side, the onto arc, a swept-angle arc to either side
/// and a line×line fillet, at
/// `Sym<Interval>` over a box about a tenth of the quantity wide, with
/// the registered-identity door shut (open for the fillet, whose own
/// door reads the tangency it registers):
///
/// - the stored radius is positive over the whole box;
/// - each vertex reads on its carrier ([`Arc2::off_carrier`] decides
///   Zero), though the read's enclosure is far wider than the band;
/// - where the verb records its radials, the centre plus the radius
///   along each is its vertex, in both components;
/// - the rim less the radius, `√(…) − radius`, does not decide: the
///   reading this row's readers no longer take;
/// - and each member of the box, built at `f64`, stores a centre, a
///   radius, a sweep and radials inside the boxed build's enclosures.
///
/// Red on a verb that stores its radius as a magnitude: with
/// `radius.abs()` in `circle_loop`, `rho.abs()` in `tangent_arc` or
/// `.abs()` on `Arc2::from_chord_radials`' radius, that verb's
/// along-the-radial readings come back undecided.
#[test]
fn each_arc_verbs_stored_arc_is_exact_over_a_box() {
    use geom_core::Sign;
    let band = geom_core::Band::linear(tol()).unwrap();
    let budget = SymBudget {
        max_terms: 4096,
        max_degree: 128,
    };
    let inside = |x: f64, e: Lane| e.lo() <= x && x <= e.hi();
    for (verb, c, h) in profile::test_support::ARC_VERBS {
        // The fillet's own door reads its joints through the tangency it
        // registers at its incoming foot, so it runs with the door open.
        let rules = if verb == "fillet" {
            SymRules::shipped()
        } else {
            SymRules::shipped_without_the_door()
        };
        with_session_rules(budget, rules, || {
            let lp = arc_verb_loop::<Lane>(verb, param("q", c, h), tol());
            let members: Vec<ProfileLoop<f64>> = [c - h, c, c + h]
                .into_iter()
                .map(|q| arc_verb_loop::<f64>(verb, q, tol()))
                .collect();
            let n = lp.vertices().len();
            let zero = |m: Lane| m.sign_within(band).map(|d| d.sign).ok() == Some(Sign::Zero);
            let mut arcs = 0;
            for k in 0..n {
                let Segment::Arc(arc) = lp.segments()[k] else {
                    continue;
                };
                arcs += 1;
                let ends = [lp.vertices()[k], lp.vertices()[(k + 1) % n]];
                assert!(
                    arc.radius.lo() > 0.0,
                    "{verb} segment {k}: a positive radius"
                );
                for (v, end) in ends.into_iter().zip(["start", "end"]) {
                    assert!(
                        zero(arc.off_carrier(v)),
                        "{verb} segment {k}: the {end} vertex on its carrier"
                    );
                }
                let recorded = lp.directions()[k].map(radials);
                if let Some((start, end)) = recorded {
                    let mut widest = 0.0_f64;
                    for (v, d, at) in [(ends[0], start, "start"), (ends[1], end, "end")] {
                        let miss = arc.on_radial(d) - v;
                        for (m, axis) in [(miss.x, "x"), (miss.y, "y")] {
                            widest = widest.max(m.hi() - m.lo());
                            assert!(
                                zero(m),
                                "{verb} segment {k}: the radius along the {at} radial is \
                                 its vertex, {axis}"
                            );
                        }
                    }
                    if verb != "onto" || k == 1 {
                        assert!(
                            widest > 1e3 * band.escalate(),
                            "{verb} segment {k}: no enclosure decided it ({widest:e})"
                        );
                        assert!(
                            !zero(arc.rim(ends[0]) - arc.radius),
                            "{verb} segment {k}: the rim less the radius does not decide"
                        );
                    }
                } else {
                    assert!(
                        matches!(verb, "fillet" | "sweep" | "sweep cw"),
                        "{verb} segment {k} records its radials"
                    );
                }
                for (q, member) in members.iter().enumerate() {
                    let Segment::Arc(m) = member.segments()[k] else {
                        panic!("{verb} member {q} segment {k} is an arc");
                    };
                    let mut pairs = vec![
                        (m.centre.x, arc.centre.x, "centre.x"),
                        (m.centre.y, arc.centre.y, "centre.y"),
                        (m.radius, arc.radius, "radius"),
                        (m.sweep, arc.sweep, "sweep"),
                    ];
                    if let (Some((s, e)), Some((bs, be))) =
                        (member.directions()[k].map(radials), recorded)
                    {
                        pairs.extend([
                            (s.x, bs.x, "start radial x"),
                            (s.y, bs.y, "start radial y"),
                            (e.x, be.x, "end radial x"),
                            (e.y, be.y, "end radial y"),
                        ]);
                    }
                    for (x, e, what) in pairs {
                        assert!(
                            inside(x, e),
                            "{verb} member {q} segment {k} {what}: {x:e} outside [{:e}, {:e}]",
                            e.lo(),
                            e.hi()
                        );
                    }
                }
            }
            assert!(arcs > 0, "{verb} stores an arc");
        });
    }
}

/// **The value channel keeps each arc's radial at the end it was
/// recorded at.** A quarter arc's two radials differ, so a channel that
/// handed the end radial back as the start one reads differently from
/// the `f64` form it was lifted from.
#[test]
fn the_value_channel_keeps_an_arcs_radials_at_their_own_ends() {
    use profile::SegmentKind;
    fn radials<T: geom_core::Real>(kind: SegmentKind<T>) -> [Vec2<T>; 2] {
        match kind {
            SegmentKind::Arc { radials, .. } => radials,
            SegmentKind::Line { .. } => panic!("an arc"),
        }
    }
    let q = (std::f64::consts::PI / 8.0).tan();
    let vp = Profile::new(
        SketchPlane::<f64>::xy(),
        vec![profile::test_support::bulge_loop(vec![
            (Point2::new(0.0, 0.0), 0.0),
            (Point2::new(2.0, 0.0), q),
            (Point2::new(1.0, 1.0), q),
        ])],
    )
    .validate(geom_core::Tol::witness())
    .expect("the D validates");
    let channel = vp
        .clone()
        .lift_onto(SketchPlane::<Interval>::xy())
        .expect("the lift")
        .value_channel()
        .expect("a point channel");
    let bits = |v: Vec2<f64>| (v.x.to_bits(), v.y.to_bits());
    let mut arcs = 0;
    for (s, c) in vp.loops()[0]
        .segments()
        .iter()
        .zip(channel.loops()[0].segments())
    {
        if matches!(s.kind, SegmentKind::Line { .. }) {
            continue;
        }
        arcs += 1;
        let ([rs, re], [cs, ce]) = (radials(s.kind), radials(c.kind));
        assert_ne!(bits(rs), bits(re), "a quarter arc's radials differ");
        assert_eq!(bits(cs), bits(rs), "the channel's start radial");
        assert_eq!(bits(ce), bits(re), "the channel's end radial");
    }
    assert_eq!(arcs, 2, "the D has two arcs");
}

/// What a build of [`quad`] came to: built and validated, or the head
/// of the verb's or validation's refusal.
fn outcome<T: Decide>(r: Result<ConstructedLoop<T>, profile::PathError<T>>) -> String {
    let head = |s: String| s.chars().take(60).collect::<String>();
    match r {
        Ok(lp) => match ConstructedProfile::new(SketchPlane::xy(), vec![lp]).validate(tol()) {
            Ok(_) => "built and validated".to_owned(),
            Err(e) => format!("validate: {}", head(format!("{e:?}"))),
        },
        Err(e) => format!("verb: {}", head(format!("{e:?}"))),
    }
}

/// **A chord with no length is refused in every lane, and the mint
/// gives it no axis.** The hop `(x1, 0) → (x2, y)` of [`quad`] with
/// both components zero: both proved, one proved and the other reading
/// Zero, and both merely reading zero. The mint stores the quotient,
/// which is no direction there, and the verb's junction gate answers on
/// it: by name at a value (`JunctionTurnWithinTolerance`), and as an
/// escalation of `path_junction_turn` over a box. Where the other
/// component straddles zero the sign reading itself escalates. No lane
/// builds; none names the zero chord as such
/// (`work/profile/a-hop-whose-chord-is-proved-zero-is-refused-by-the-junction-gate-not-by-name.md`).
#[test]
fn a_chord_with_no_length_is_refused_in_every_lane() {
    const NAMED: &str = "verb: JunctionTurnWithinTolerance";
    const JUNCTION: &str = "verb: Escalated { site: Door, gate: PathJunctionTurn";
    const SIGN: &str = "verb: Escalated { site: Door, gate: SegmentDirectionAxis";
    let i = |v: f64| Interval::from_bounds(v, v);
    let mut rows = vec![
        ("f64".to_owned(), NAMED, outcome(quad(1.0, 1.0, 0.0))),
        (
            "Interval".to_owned(),
            NAMED,
            outcome(quad(i(1.0), i(1.0), i(0.0))),
        ),
    ];
    for kind in SESSIONS {
        let boxed = || param("w", 1.0, small());
        let lanes: [(&str, &str, &dyn Fn() -> String); 6] = [
            ("both proved", NAMED, &|| {
                let w = pf("w", 1.0);
                outcome(quad(w, w, Sym::from_f64(0.0)))
            }),
            ("x proved, y reads 0", NAMED, &|| {
                let w = pf("w", 1.0);
                outcome(quad(w, w, pf("h", 0.0)))
            }),
            ("both read 0", NAMED, &|| {
                outcome(quad(pf("a", 1.0), pf("b", 1.0), pf("h", 0.0)))
            }),
            ("box, both proved", JUNCTION, &|| {
                outcome(quad(boxed(), boxed(), Sym::from_f64(0.0)))
            }),
            ("box, x proved, y the point 0", JUNCTION, &|| {
                outcome(quad(boxed(), boxed(), param("h", 0.0, 0.0)))
            }),
            ("box, x proved, y straddles 0", SIGN, &|| {
                outcome(quad(boxed(), boxed(), param("h", 0.0, small())))
            }),
        ];
        for (lane, want, build) in lanes {
            rows.push((format!("{kind:?}: {lane}"), want, run(kind, build)));
        }
    }
    for (lane, want, got) in rows {
        assert!(got.starts_with(want), "{lane}: {got}");
    }
}

/// **The sign is read at the least band.** A hop whose proved-zero
/// component leaves the other a sixty-fourth of ε long: its direction
/// is `+y` however short it is, the reading answers Positive, and the
/// axis is stored; the junction gate then refuses the short side by
/// name. A reading at the working band answers Zero here and is red on
/// the readings.
#[test]
fn the_axis_sign_is_read_at_the_least_band() {
    use geom_core::Sign::Positive;
    use geom_core::sym::revalue::record;
    for kind in SESSIONS {
        let (at_value, d) = record(|| {
            run(kind, || {
                let w = pf("w", 1.0);
                outcome(quad(w, w, pf("h", small())))
            })
        });
        assert_eq!(
            axis_readings(&d),
            [Some(Positive), Some(Positive)],
            "{kind:?}"
        );
        assert!(
            at_value.starts_with("verb: JunctionTurnWithinTolerance"),
            "{kind:?}: {at_value}"
        );
        let (_, d) = record(|| {
            run(kind, || {
                let w = param("w", 1.0, small());
                outcome(quad(w, w, param("h", small(), 0.0)))
            })
        });
        assert_eq!(
            axis_readings(&d),
            [Some(Positive), Some(Positive)],
            "{kind:?}: over a box"
        );
    }
}

/// **What was asked before does not move the branch.** The sum `x + w`
/// is minted before the session is installed, and the hop
/// `(x, 0) → (that − w, 1)` asks about a graph the session does not
/// hold. The same document built inside the session afterwards mints
/// the axis whether or not that hop was asked first: each ask starts
/// from the expression alone.
#[test]
fn an_earlier_ask_does_not_move_the_branch() {
    for kind in SESSIONS {
        let build = |ask_first: bool| {
            let minted_before = pf("x", 0.1) + pf("w", 0.3);
            run(kind, || {
                let (x, w) = (pf("x", 0.1), pf("w", 0.3));
                if ask_first {
                    let _ = quad(x, minted_before - w, Sym::from_f64(1.0));
                }
                quad_hop(x, (x + w) - w)
            })
        };
        assert_eq!(build(false), AXIS_UP, "{kind:?}");
        assert_eq!(build(true), AXIS_UP, "{kind:?}: asked first");
    }
}

/// **The one-branch statement is about a graph built inside its
/// session.** The identity `(x + w) − w` computed before the session is
/// installed is a graph the session's table does not hold; the question
/// proves nothing about it, and the hop stores the quotient: the axis
/// where the value channel computes `0.0`, one ulp off it at
/// `(0.1, 0.3)`. Every door builds its document inside the session it
/// decides in.
#[test]
fn a_graph_built_outside_its_session_stores_the_quotient() {
    for kind in SESSIONS {
        let outside = |x: f64, w: f64| {
            let (x, w) = (pf("x", x), pf("w", w));
            let x2 = (x + w) - w;
            run(kind, || quad_hop(x, x2))
        };
        assert_eq!(outside(1.0, 1.0), AXIS_UP, "{kind:?}");
        let leans = outside(0.1, 0.3);
        assert!(leans[0].0 > 0.0, "{kind:?}: {leans:?}");
    }
}

/// **An arc verb whose leg lowers to a line is a hop.** A bulge of the
/// literal zero has no arc; the leg is the line to its end, and its
/// director is minted and recorded as `line_to`'s is: at `f64` the
/// quotient with its zero's sign, and over a box the axis, with no
/// width, where the chord's `x` is proved zero.
#[test]
fn a_bulge_of_zero_records_the_hops_minted_director() {
    let (a, b) = (Point2::new(4.0, 0.0), Point2::new(4.0, 4.0));
    let lp = Open
        .at(Point2::new(0.0, 0.0))
        .line_to(a, tol())
        .unwrap()
        .arc_to(profile::Bulge { p: b, b: 0.0 }, tol())
        .unwrap()
        .line_to(Point2::new(0.0, 4.0), tol())
        .unwrap()
        .line_to(Start, tol())
        .unwrap();
    records(
        &lp.loop_.into_loop(),
        &[
            hop((0.0, 0.0), (4.0, 0.0)),
            hop((4.0, 0.0), (4.0, 4.0)),
            hop((4.0, 4.0), (0.0, 4.0)),
            hop((0.0, 4.0), (0.0, 0.0)),
        ],
        "bulge 0",
    );
    for kind in SESSIONS {
        let side = run(kind, || {
            let (w, z) = (param("W", 4.0, small()), Lane::from_f64(0.0));
            let h = Lane::from_f64(4.0);
            let lp = Open
                .at(Point2::new(z, z))
                .line_to(Point2::new(w, z), tol())
                .unwrap()
                .arc_to(
                    profile::Bulge {
                        p: Point2::new(w, h),
                        b: z,
                    },
                    tol(),
                )
                .unwrap()
                .line_to(Point2::new(z, h), tol())
                .unwrap()
                .line_to(Start, tol())
                .unwrap();
            bounds(line_director(
                validated(vec![lp.loop_]).loops()[0].segments()[1].kind,
            ))
        });
        assert_eq!(side, AXIS_UP, "{kind:?}");
    }
}
