//! **The reach meters a mixed corner by its own patch**
//! (`work/band/the-reach-meters-a-mixed-corner-as-a-uniform-one-and-refuses-short-edges.md`):
//! the fillet's torus piece between `E` and the odd band, over the turn
//! between the pair's rests, and the chamfer's mitred strip between its
//! two mitres (`reach::mixed_corner_reach`). Each holds the material the
//! corner changes, and with it some it leaves as it was: the air the odd
//! band would fill and the patch cuts back away (the flush corner's), or
//! the boss's own material the same way (the inset's). Short edges and
//! close faces the uniform corner's octant or plane refused now build,
//! at their closed forms. Where the patch's boundary is the material's
//! own (on the reflex side of the corner, beside the pair's strips), a
//! feature that enters the patch by `100ε` refuses and one that stands
//! `100ε` clear of it builds; a feature in the air or boss the patch
//! leaves as it was refuses as a bound.
//!
//! The odd edge with one pair edge is a turn, its pair strip cut off in
//! the odd strip's plane: the pair band's reach ends there, not in its
//! third face's plane, so a feature behind the mitre builds.
//!
//! The closed forms are `band_mixed_corner`'s: flush,
//! `ΔV = A(L − Y − Z + 2r) − (π/2)·A(r + d)` and
//! `(s²/2)(L − Y − Z) + K·s³`; inset, each negated.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::f64::consts::{FRAC_PI_2, PI};

use geom_core::{Band, Interval, Point2};
use sweep::blend::BlendError;
use sweep::blend::build::{Blended, fillet_edges};
use sweep::chamfer::chamfer_edges;
use sweep::test_support::{
    assert_naming_totality, at_rest, brick, corners, cylinder_of_arcs_at, prism_at, realized,
};
use topo::boolean::BooleanOp;
use topo::{Body, EdgeKey, validate_geometric};

use crate::band_closed_mixed_chain_shapes::{assert_touch_escalation, tightest_interval};
use crate::band_mixed_corner::{
    Scalar, boreless, chamfer_flush, edge, every_edge, fillet_flush, flush_at, inset, inset_at,
    l_corner, l_prism, realized_at, seams, tol, volume,
};

/// The size every single-corner row asks for, meters.
const S: f64 = 0.1;

#[derive(Clone, Copy, Debug)]
enum Verb {
    Fillet,
    Chamfer,
}

const VERBS: [Verb; 2] = [Verb::Fillet, Verb::Chamfer];

impl Verb {
    fn run<T: Scalar>(
        self,
        body: &Body<T>,
        edges: &[EdgeKey],
        size: f64,
    ) -> Result<Blended<T>, BlendError> {
        let operand = at_rest(body, tol());
        let size = T::from_f64(size);
        match self {
            Self::Fillet => fillet_edges(&operand, edges, size, tol()),
            Self::Chamfer => chamfer_edges(&operand, edges, size, tol()),
        }
        .map_err(|r| r.error)
    }

    /// The flush corner's `ΔV`, the odd edge `l` long and the pair `y`
    /// and `z`, each other end square.
    fn flush(self, size: f64, (l, y, z): (f64, f64, f64)) -> f64 {
        match self {
            Self::Fillet => {
                let a = size * size * (1.0 - PI / 4.0);
                let d = size * (10.0 - 3.0 * PI) / (12.0 - 3.0 * PI);
                a * (l - y - z + 2.0 * size) - FRAC_PI_2 * a * (size + d)
            }
            Self::Chamfer => {
                let k = 5.0 / 3.0 - 7.0 * 2f64.sqrt() / 6.0;
                size * size / 2.0 * (l - y - z) + k * size.powi(3)
            }
        }
    }
}

/// `ε`, the witness band's zero.
fn eps() -> f64 {
    Band::linear(tol()).expect("the witness band").zero()
}

/// One build: tier 3, `shells` closed shells, naming totality over the
/// request, and `ΔV` at `truth` within the certified enclosure and
/// `1e-12`.
fn builds<T: Scalar>(
    body: &Body<T>,
    edges: &[EdgeKey],
    (verb, size): (Verb, f64),
    (truth, shells): (f64, i64),
    what: &str,
) {
    builds_but(body, (edges, &[]), (verb, size), (truth, shells), what);
}

/// [`builds`], naming totality read over the request less `smooth`: a
/// bore's seams, which the blend leaves as they are.
fn builds_but<T: Scalar>(
    body: &Body<T>,
    (edges, smooth): (&[EdgeKey], &[EdgeKey]),
    (verb, size): (Verb, f64),
    (truth, shells): (f64, i64),
    what: &str,
) {
    let out = verb
        .run(body, edges, size)
        .unwrap_or_else(|e| panic!("{what} ({verb:?} {size}): builds, got {e:?}"));
    validate_geometric(&out.body, tol())
        .unwrap_or_else(|e| panic!("{what} ({verb:?} {size}): tier 3, got {e:?}"));
    let c = topo::readback::euler_counts(&out.body);
    assert_eq!(c.s, shells, "{what} ({verb:?} {size}): {shells} shells");
    let blended: Vec<EdgeKey> = edges
        .iter()
        .copied()
        .filter(|e| !smooth.contains(e))
        .collect();
    assert_naming_totality(body, &out, &blended, what);
    let ((v0, pad0), (v1, pad1)) = (volume(body), volume(&out.body));
    let (dv, pad) = (v1 - v0, pad0 + pad1);
    assert!(
        pad < 1e-9 && (dv - truth).abs() < 1e-12 + pad,
        "{what} ({verb:?} {size}): ΔV {dv} ± {pad} vs the closed form {truth}"
    );
}

/// A refusal of the reach: `FaceClearance`, a bound, its margin
/// negative.
fn refuses<T: Scalar>(body: &Body<T>, edges: &[EdgeKey], (verb, size): (Verb, f64), what: &str) {
    let got = verb.run(body, edges, size);
    assert!(
        matches!(
            got,
            Err(BlendError::FaceClearance { bounded: true, ref margin, .. })
                if margin.sign == geom_core::Sign::Negative
        ),
        "{what} ({verb:?} {size}): refuses FaceClearance, got {got:?}",
        got = got.as_ref().map(|_| ())
    );
}

/// A feature clear of the patch: it builds at `truth`, or the meter,
/// reading it through its box, refuses it as a bound.
fn builds_or_bound<T: Scalar>(
    body: &Body<T>,
    edges: &[EdgeKey],
    (verb, size): (Verb, f64),
    (truth, shells): (f64, i64),
    what: &str,
) {
    match verb.run(body, edges, size) {
        Err(BlendError::FaceClearance { bounded: true, .. }) => {}
        Err(e) => panic!("{what} ({verb:?} {size}): builds or refuses as a bound, got {e:?}"),
        Ok(_) => builds(body, edges, (verb, size), (truth, shells), what),
    }
}

/// The odd edge, then the pair edges along `+x` to `x` and `+y` to `y`,
/// of a flush corner at the origin whose odd edge is `h` tall.
fn three<T: Scalar>(body: &Body<T>, h: f64, x: f64, y: f64) -> [EdgeKey; 3] {
    let o = (0.0, 0.0, 0.0);
    [
        edge(body, o, (0.0, 0.0, h)),
        edge(body, o, (x, 0.0, 0.0)),
        edge(body, o, (0.0, y, 0.0)),
    ]
}

fn outlined<T: Scalar>(outline: &[(f64, f64)], h: f64) -> Body<T> {
    prism_at(corners(outline), T::zero(), T::from_f64(h), tol())
}

/// **The inset corner with its boss `h` tall**: `band_mixed_corner`'s
/// inset, the boss standing to `z = h`.
fn inset_boss<T: Scalar>(h: f64) -> (Body<T>, [EdgeKey; 3]) {
    let plate = prism_at(
        corners(&[(-1.0, -1.0), (1.0, -1.0), (1.0, 1.0), (-1.0, 1.0)]),
        T::from_f64(-1.0),
        T::one(),
        tol(),
    );
    let boss = prism_at(
        corners(&[(-1.0, -1.0), (0.0, -1.0), (0.0, 0.0), (-1.0, 0.0)]),
        T::from_f64(-0.5),
        T::from_f64(0.5 + h),
        tol(),
    );
    let body = realized_at(BooleanOp::Union, &plate, &boss);
    let o = (0.0, 0.0, 0.0);
    let edges = [
        edge(&body, o, (0.0, 0.0, h)),
        edge(&body, o, (0.0, -1.0, 0.0)),
        edge(&body, o, (-1.0, 0.0, 0.0)),
    ];
    (body, edges)
}

/// **The witnesses**, each built as it refused before: the flush L
/// with its odd edge `0.25` tall (and `0.26`), a pair edge `0.2` and
/// `0.25` long, an arm `0.15` thick, and the inset boss `0.25` tall,
/// both verbs at `0.1`. The uniform reading refused every chamfer here
/// `FaceClearance` at margin `0`, its side bounds read at a point on
/// the walls' planes.
fn the_short_and_close_witnesses_build<T: Scalar>() {
    for h in [0.25, 0.26] {
        let body = l_prism::<T>(h);
        let edges = l_corner(&body, h);
        for verb in VERBS {
            let truth = verb.flush(S, (h, 1.0, 1.0));
            builds(
                &body,
                &edges,
                (verb, S),
                (truth, 1),
                &format!("odd edge {h} tall"),
            );
        }
    }
    for y in [0.2, 0.25] {
        let outline = [
            (-1.0, -1.0),
            (y, -1.0),
            (y, 0.0),
            (0.0, 0.0),
            (0.0, 1.0),
            (-1.0, 1.0),
        ];
        let body = outlined::<T>(&outline, 2.0);
        let edges = three(&body, 2.0, y, 1.0);
        for verb in VERBS {
            let truth = verb.flush(S, (2.0, y, 1.0));
            builds(
                &body,
                &edges,
                (verb, S),
                (truth, 1),
                &format!("pair edge {y} long"),
            );
        }
    }
    let t = 0.15;
    let outline = [
        (-t, -t),
        (1.0, -t),
        (1.0, 0.0),
        (0.0, 0.0),
        (0.0, 1.0),
        (-t, 1.0),
    ];
    let body = outlined::<T>(&outline, 2.0);
    let edges = three(&body, 2.0, 1.0, 1.0);
    for verb in VERBS {
        let truth = verb.flush(S, (2.0, 1.0, 1.0));
        builds(&body, &edges, (verb, S), (truth, 1), "an arm 0.15 thick");
    }
    let (body, edges) = inset_boss::<T>(0.25);
    for verb in VERBS {
        let what = "an inset boss 0.25 tall";
        // The inset fillet's sections escalate their shared row's touch
        // at the tightest Interval row, as every mixed rim's does
        // (`work/band/every-mixed-rims-section-escalates-its-touch-reading-at-interval-at-eps-1e-12.md`),
        // past the reach.
        if tightest_interval::<T>() && matches!(verb, Verb::Fillet) {
            let got = verb.run(&body, &edges, S);
            let e = got
                .err()
                .unwrap_or_else(|| panic!("{what}: escalates at Interval at 1e-12"));
            assert_touch_escalation(what, &e);
            continue;
        }
        let truth = -verb.flush(S, (0.25, 1.0, 1.0));
        builds(&body, &edges, (verb, S), (truth, 1), what);
    }
}

#[test]
fn the_short_and_close_witnesses_build_at_their_closed_forms() {
    the_short_and_close_witnesses_build::<f64>();
}

#[test]
fn the_short_and_close_witnesses_build_at_their_closed_forms_at_interval() {
    the_short_and_close_witnesses_build::<Interval>();
}

/// The bracket's closed forms ([`band_mixed_corner`]'s rows): filleted
/// all round, bored; chamfered all round without its bore.
fn bracket_fillet(r: f64) -> f64 {
    let a = r * r * (1.0 - PI / 4.0);
    let d = r * (10.0 - 3.0 * PI) / (12.0 - 3.0 * PI);
    -a * (48.0 - 36.0 * r) - 12.0 * r.powi(3) * (1.0 - PI / 6.0) - 4.0 * PI * a * (0.3 + d)
}

fn bracket_chamfer(s: f64) -> f64 {
    -s * s / 2.0 * 48.0 + 8.0 * s.powi(3)
}

fn bored() -> Body<f64> {
    let bore = cylinder_of_arcs_at(2, 0.3, Point2::new(3.0, 2.0), -0.5, 1.5, tol());
    realized(BooleanOp::Subtract, &boreless(), &bore, tol())
}

/// **The bracket builds at the witness sizes and on to its real
/// limit**: chamfered all round without its bore at `0.14` and filleted
/// all round at `0.149`, which the uniform reading refused, and both up
/// to `0.249`, each at its closed form, which counts every band and
/// corner once and so holds only while no two overlap. At `0.251` the
/// plate's top and bottom bands, whose trimlines stand `r` up and down
/// its `0.5` sides, cross, and the support screen refuses there.
#[test]
fn the_bracket_builds_at_the_witness_sizes_and_up_to_its_real_limit() {
    let plain = boreless::<f64>();
    let bored = bored();
    for size in [0.14, 0.149, 0.249] {
        let what = format!("the bracket at {size}");
        builds(
            &plain,
            &every_edge(&plain),
            (Verb::Chamfer, size),
            (bracket_chamfer(size), 1),
            &what,
        );
        let all = every_edge(&bored);
        builds_but(
            &bored,
            (&all, &seams(&bored, &all)),
            (Verb::Fillet, size),
            (bracket_fillet(size), 1),
            &what,
        );
    }
    for (verb, body) in [(Verb::Chamfer, &plain), (Verb::Fillet, &bored)] {
        let got = verb.run(body, &every_edge(body), 0.251);
        assert!(
            matches!(got, Err(BlendError::FaceClearanceUncertified { .. })),
            "the bracket at 0.251 ({verb:?}): the plate's bands cross, got {got:?}",
            got = got.as_ref().map(|_| ())
        );
    }
}

/// **The real boundary of each corner patch**, as the corner point of
/// a box feature `w` across standing `z0` off `E` reaches it at the
/// size `S`. Moving that point by `−δ` puts it in the patch.
///
/// - Reflex, on the diagonal through the vertex away from the odd
///   band, `(x, x)`: the fillet's torus leaves the material
///   `ρ < ρ_t(z0) = 2r − √(r² − (z0 − r)²)` from the odd axis at
///   `(r, r)`, so `x = r − ρ_t/√2`; the chamfer's strip
///   `u − z ≥ s/√2 − s` in `u = (x + y)/√2`, so `x = z0/√2 + s/2 − s/√2`.
/// - Beside the `+x` pair strip, the corner `(X, y)` with `X` short of
///   that strip's rest (its mitre): `y = r − √(ρ_t² − (r − X)²)` for
///   the fillet and `y = √2·z0 + s(1 − √2) − X` for the chamfer.
const Z0: f64 = 0.005;
const W: f64 = 0.04;
const X: f64 = 0.05;

fn rho_t() -> f64 {
    2.0 * S - (S * S - (Z0 - S).powi(2)).sqrt()
}

fn reflex(verb: Verb) -> f64 {
    match verb {
        Verb::Fillet => S - rho_t() / 2f64.sqrt(),
        Verb::Chamfer => Z0 / 2f64.sqrt() + S / 2.0 - S / 2f64.sqrt(),
    }
}

fn beside(verb: Verb) -> f64 {
    match verb {
        Verb::Fillet => S - (rho_t().powi(2) - (S - X).powi(2)).sqrt(),
        Verb::Chamfer => 2f64.sqrt() * Z0 + S * (1.0 - 2f64.sqrt()) - X,
    }
}

/// The offsets each boundary is read at: into the patch by 3% of the
/// size and by `100ε`, and out of it by the same.
fn offsets() -> [f64; 4] {
    let (far, near) = (0.03 * S, 100.0 * eps());
    [-far, -near, near, far]
}

/// The flush L 2 tall less a box void.
fn voided<T: Scalar>(x: (f64, f64), y: (f64, f64)) -> Body<T> {
    let void = brick::<T>(x, y, (Z0, Z0 + W), tol());
    realized_at(BooleanOp::Subtract, &l_prism::<T>(2.0), &void)
}

/// **A feature entering the flush corner's patch refuses, and one
/// beside it builds** — a void on the reflex side of the corner, below
/// the odd band, and one beside the `+x` pair strip short of its rest,
/// which only the corner's region holds. The uniform reading cleared
/// the fillet's voids inside the patch (the body then failed to finish)
/// and refused every chamfer one, inside or out.
fn a_feature_entering_the_flush_patch_refuses<T: Scalar>(offsets: &[f64]) {
    for verb in VERBS {
        let truth = verb.flush(S, (2.0, 1.0, 1.0));
        for &d in offsets {
            let x0 = reflex(verb) - d;
            let body = voided::<T>((x0 - W, x0), (x0 - W, x0));
            let edges = l_corner(&body, 2.0);
            let what = format!("a reflex void off the patch by {d:e}");
            if d < 0.0 {
                refuses(&body, &edges, (verb, S), &what);
            } else {
                builds(&body, &edges, (verb, S), (truth, 2), &what);
            }
            let y1 = beside(verb) - d;
            let body = voided::<T>((X - W, X), (y1 - W, y1));
            let edges = l_corner(&body, 2.0);
            let what = format!("a void beside the pair strip off the patch by {d:e}");
            if d < 0.0 {
                refuses(&body, &edges, (verb, S), &what);
            } else {
                builds(&body, &edges, (verb, S), (truth, 2), &what);
            }
        }
    }
}

#[test]
fn a_feature_entering_the_flush_corners_patch_refuses_and_one_clear_of_it_builds() {
    a_feature_entering_the_flush_patch_refuses::<f64>(&offsets());
}

#[test]
fn a_feature_entering_the_flush_corners_patch_refuses_and_one_clear_of_it_builds_at_interval() {
    let [far_in, _, _, far_out] = offsets();
    a_feature_entering_the_flush_patch_refuses::<Interval>(&[far_in, far_out]);
}

/// **The inset corner is the flush one's complement turned a half turn
/// about the odd edge**, so its patch is the flush one's turned, and a
/// box island standing in the air there meets it where the flush void
/// meets the flush patch: an island entering it refuses, and one clear
/// of it builds, a second solid.
#[test]
fn an_island_entering_the_inset_corners_patch_refuses_and_one_clear_of_it_builds() {
    let (inset, _) = inset::<f64>();
    for verb in VERBS {
        let truth = -verb.flush(S, (1.0, 1.0, 1.0));
        for d in offsets() {
            for (what, (x, y)) in [
                ("a reflex island", (reflex(verb) - d, reflex(verb) - d)),
                ("an island beside the pair strip", (X, beside(verb) - d)),
            ] {
                let island = brick::<f64>((-x, W - x), (-y, W - y), (Z0, Z0 + W), tol());
                let body = realized(BooleanOp::Union, &inset, &island, tol());
                let o = (0.0, 0.0, 0.0);
                let edges = [
                    edge(&body, o, (0.0, 0.0, 1.0)),
                    edge(&body, o, (0.0, -1.0, 0.0)),
                    edge(&body, o, (-1.0, 0.0, 0.0)),
                ];
                let what = format!("{what} off the patch by {d:e}");
                if d < 0.0 {
                    refuses(&body, &edges, (verb, S), &what);
                } else {
                    builds(&body, &edges, (verb, S), (truth, 2), &what);
                }
            }
        }
    }
}

/// **The inset corner at 60° and 120°**: the flush void's mirror, a box
/// island on the boss's bisector's far side, refuses entering the patch
/// and builds clear of it. At 60° the fillet's torus reaches no further
/// than the vertex, as the flush one's.
#[test]
fn an_island_entering_the_inset_patch_at_a_non_right_plan_angle_refuses() {
    for phi in [PI / 3.0, 2.0 * PI / 3.0] {
        let (cos, sin) = ((phi / 2.0).cos(), (phi / 2.0).sin());
        // The boss's bisector, between `−y` and `φ` clockwise from it.
        let b = (-(phi / 2.0).sin(), -(phi / 2.0).cos());
        for verb in VERBS {
            let at = match verb {
                Verb::Fillet => S / sin - rho_t(),
                Verb::Chamfer => Z0 - S * (1.0 - cos),
            };
            if at >= 0.0 {
                continue;
            }
            let truth = match verb {
                Verb::Fillet => -fillet_flush(phi, S, (1.0, 1.0, 1.0)),
                Verb::Chamfer => -chamfer_flush(phi, S, (1.0, 1.0, 1.0)),
            };
            for d in offsets() {
                let t = at - d;
                let (x0, y0) = (t * b.0, t * b.1);
                let (body, _) = inset_at::<f64>(phi);
                let island = brick::<f64>((x0, x0 + W), (y0, y0 + W), (Z0, Z0 + W), tol());
                let body = realized(BooleanOp::Union, &body, &island, tol());
                let o = (0.0, 0.0, 0.0);
                let edges = [
                    edge(&body, o, (0.0, 0.0, 1.0)),
                    edge(&body, o, (0.0, -1.0, 0.0)),
                    edge(&body, o, (-phi.sin(), -phi.cos(), 0.0)),
                ];
                let what = format!(
                    "inset at {:.0}°, an island off the patch by {d:e}",
                    phi.to_degrees()
                );
                if d < 0.0 {
                    refuses(&body, &edges, (verb, S), &what);
                } else {
                    builds(&body, &edges, (verb, S), (truth, 2), &what);
                }
            }
        }
    }
}

/// **The region holds some material the patch leaves as it was**: in
/// the flush corner, the air below the torus (or the chamfer's strip)
/// that the odd band would fill and the patch cuts back; in the inset,
/// the boss's own material the same way. A feature standing there
/// refuses as a bound, though the result leaves it untouched
/// (`work/band/a-mixed-corners-reach-holds-the-air-its-patch-leaves.md`).
#[test]
fn a_feature_in_what_the_patch_leaves_as_it_was_refuses_as_a_bound() {
    let low = (0.003, 0.01);
    for (verb, xy) in [
        (Verb::Fillet, (0.002, 0.006)),
        (Verb::Chamfer, (0.01, 0.03)),
    ] {
        let island = brick::<f64>(xy, xy, low, tol());
        let body = realized(BooleanOp::Union, &l_prism::<f64>(2.0), &island, tol());
        refuses(
            &body,
            &l_corner(&body, 2.0),
            (verb, S),
            "an island in the flush air",
        );
        let void = brick::<f64>((-xy.1, -xy.0), (-xy.1, -xy.0), low, tol());
        let (inset, _) = inset::<f64>();
        let body = realized(BooleanOp::Subtract, &inset, &void, tol());
        let o = (0.0, 0.0, 0.0);
        let edges = [
            edge(&body, o, (0.0, 0.0, 1.0)),
            edge(&body, o, (0.0, -1.0, 0.0)),
            edge(&body, o, (-1.0, 0.0, 0.0)),
        ];
        refuses(&body, &edges, (verb, S), "a void in the inset boss");
    }
}

/// **The inset witness's fillet at every ε row**, fast enough for the
/// per-PR gate: the boss `0.25` tall fillets at `Interval` to tier 3
/// (its closed form is the slow rows'), but at the tightest row, where its sections escalate their
/// shared row's touch (the `_at_interval` witness row's branch).
#[test]
fn the_inset_witness_fillets_or_escalates_at_every_eps_row() {
    let (body, edges) = inset_boss::<Interval>(0.25);
    let what = "an inset boss 0.25 tall filleted at Interval";
    if tightest_interval::<Interval>() {
        let e = Verb::Fillet
            .run(&body, &edges, S)
            .err()
            .unwrap_or_else(|| panic!("{what}: escalates at 1e-12"));
        assert_touch_escalation(what, &e);
    } else {
        let out = Verb::Fillet
            .run(&body, &edges, S)
            .unwrap_or_else(|e| panic!("{what}: builds, got {e:?}"));
        validate_geometric(&out.body, tol())
            .unwrap_or_else(|e| panic!("{what}: tier 3, got {e:?}"));
    }
}

/// **At plan angles of 60° and 120°** a box void below the corner,
/// its corner `(x, y)` on the bisector's far side `t` from the vertex
/// (the fillet's `ρ_t` from the odd axis, which stands `r / sin(φ/2)` up
/// the bisector; the chamfer's strip `s(1 − cos(φ/2)) − z0` past the
/// vertex), refuses entering the patch and builds clear of it. At 60°
/// the fillet's torus reaches no further than the vertex
/// (`r / sin 30° = 2r`), so it has no reflex side to enter.
#[test]
fn a_feature_entering_the_patch_at_a_non_right_plan_angle_refuses() {
    for phi in [PI / 3.0, 2.0 * PI / 3.0] {
        let (cos, sin) = ((phi / 2.0).cos(), (phi / 2.0).sin());
        for verb in VERBS {
            // How far up the bisector the corner of the void stands.
            let at = match verb {
                Verb::Fillet => S / sin - rho_t(),
                Verb::Chamfer => Z0 - S * (1.0 - cos),
            };
            if at >= 0.0 {
                continue;
            }
            let truth = match verb {
                Verb::Fillet => fillet_flush(phi, S, (2.0, 1.0, 1.0)),
                Verb::Chamfer => chamfer_flush(phi, S, (2.0, 1.0, 1.0)),
            };
            for d in offsets() {
                let t = at - d;
                let (x0, y0) = (t * cos, t * sin);
                let (body, _) = flush_at::<f64>(phi);
                let void = brick::<f64>((x0 - W, x0), (y0 - W, y0), (Z0, Z0 + W), tol());
                let body = realized(BooleanOp::Subtract, &body, &void, tol());
                let o = (0.0, 0.0, 0.0);
                let edges = [
                    edge(&body, o, (0.0, 0.0, 2.0)),
                    edge(&body, o, (1.0, 0.0, 0.0)),
                    edge(&body, o, (phi.cos(), phi.sin(), 0.0)),
                ];
                let what = format!("at {:.0}°, a void off the patch by {d:e}", phi.to_degrees());
                if d < 0.0 {
                    refuses(&body, &edges, (verb, S), &what);
                } else {
                    builds(&body, &edges, (verb, S), (truth, 2), &what);
                }
            }
        }
    }
}

/// **The odd edge with one pair edge**: the pair strip is cut off in
/// the odd strip's plane, so its material starts at `x = s − y` across
/// its section's leg `y ∈ [−s, 0]`. A void whose corner `(x, −s/2)`
/// stands in that material refuses; one short of it, between the
/// odd strip's plane and the face the pair edge ends in, builds. The
/// uniform turn's reading capped the pair's reach at that face and
/// read every such void as crossing the band's material.
#[test]
fn the_odd_plus_one_turn_meters_its_pair_strip_from_the_odd_strips_plane() {
    let y1 = -S / 2.0;
    let truth = S * S / 2.0 + 2.0 * S.powi(3) / 3.0;
    let mut cases: Vec<(f64, bool)> = offsets().iter().map(|d| (S - y1 - d, *d < 0.0)).collect();
    cases.push((0.05, false));
    for (x1, inside) in cases {
        let body = voided::<f64>((x1 - W, x1), (y1 - W, y1));
        let o = (0.0, 0.0, 0.0);
        let edges = [
            edge(&body, o, (0.0, 0.0, 2.0)),
            edge(&body, o, (1.0, 0.0, 0.0)),
        ];
        let what = format!("a void at x = {x1} beside the turn");
        if inside {
            refuses(&body, &edges, (Verb::Chamfer, S), &what);
        } else {
            builds(&body, &edges, (Verb::Chamfer, S), (truth, 2), &what);
        }
    }
}

/// **Turned and off the origin** (a turn of 0.7 about `x` through
/// `(1/4, −1/2, 1/8)`), each case reads as it does in place: the odd
/// edge `0.25` tall chamfers and fillets, a reflex void entering either
/// patch by 3% refuses, and one 3% clear of it builds at its closed form
/// or refuses as a bound, never built wrong. The meter tiles a turned
/// face's world box, which reaches past the face toward the torus, so
/// today the fillet's refuses there and builds from 20%
/// (`work/band/a-turned-features-box-holds-the-reach-off-a-curved-patch.md`).
#[test]
fn a_turned_and_moved_mixed_corner_reads_as_it_does_in_place() {
    let pose = crate::common::poses::torax_pose();
    let motion = geom_core::test_support::rigid(&pose, tol());
    let moved = |b: &Body<f64>| topo::transform_rigid(b, &motion).expect("the body moves");
    let body = l_prism::<f64>(0.25);
    let edges = l_corner(&body, 0.25);
    let body = moved(&body);
    let far = 0.03 * S;
    for verb in VERBS {
        let truth = verb.flush(S, (0.25, 1.0, 1.0));
        builds(
            &body,
            &edges,
            (verb, S),
            (truth, 1),
            "turned, odd edge 0.25 tall",
        );
        for d in [-far, far, 0.2 * S] {
            let x0 = reflex(verb) - d;
            let body = voided::<f64>((x0 - W, x0), (x0 - W, x0));
            let edges = l_corner(&body, 2.0);
            let body = moved(&body);
            let what = format!("turned, a reflex void off the patch by {d:e}");
            let truth = verb.flush(S, (2.0, 1.0, 1.0));
            if d < 0.0 {
                refuses(&body, &edges, (verb, S), &what);
            } else if d < 0.2 * S {
                builds_or_bound(&body, &edges, (verb, S), (truth, 2), &what);
            } else {
                builds(&body, &edges, (verb, S), (truth, 2), &what);
            }
        }
    }
}
