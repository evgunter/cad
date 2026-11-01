//! **A closed conic lying in the partner's face.** A one-segment circle
//! is one edge with one vertex, both its ends. Where such an edge of one
//! operand lies in a face of the other, the section loop is that edge,
//! and its one site is the edge's vertex: one pair record carries both
//! of the loop's germs, one each way round it. The join takes it as one
//! segment (`boolean::join`'s `one_site_segments`): the edge's own side
//! copies it whole, and the partner's face is pierced at the site and
//! joined there as a ring of its own, or, where that face's boundary
//! runs through the site (two rims tangent at their vertices), walled
//! off in the face from its boundary vertex. Where the partner holds the circle
//! along a closed edge of its own through the same vertex (two coaxial
//! one-segment circles phased alike), both sides copy their edges.
//!
//! The rows, each a one-segment circle built through the raw loop door:
//!
//! - a boss standing on a plate's top face, its vertex at four azimuths,
//!   and one hanging under its bottom face off the axis, with the
//!   census; the same boss at `Interval`;
//! - a second boss on a plate whose top face already holds a boss's
//!   ring, and a boss over a hole whose ring lies inside its rim;
//! - a boss over a hole or a pocket, by pose: tangent to its rim inside
//!   or outside, apart, concentric, each read alike in the two-arc form,
//!   and the tangent poses turned; and tangent inside with short arcs
//!   either side of the tip, whose union is one shell pinched along them,
//!   and the rims swapped, a boss inside a hole touching it along them,
//!   whose union is two solids;
//! - the boss and the hole tilted, their rims ellipses tangent outside,
//!   in both forms;
//! - a plug in the bore of an annulus standing on a plate;
//! - rods tangent inside one another along a ruling, a rim lying flush
//!   in the partner's top or floor, at rest and turned;
//! - two equal coaxial cylinders stacked end to end;
//! - a tube whose rim lies inside one face of a ball;
//! - a puck and the quarter-disc ring revolved round it.
//!
//! Every row runs ∪, ∩ and both differences in both operand orders, and
//! each run is read through `differential::outcome` (tiers 2 and 3′, the
//! certificate, the legal operand, the closed-form volume) and meshed;
//! a mesh along the short arcs' pinch holds four triangles at an edge.
//! The ∩ of two solids meeting only in a face is empty. One row pins
//! what is not built yet, refused typed: the union of a boss over a hole
//! whose rim is tangent inside the boss's (a crescent). The puck runs
//! its unions alone: its ∩ and differences refuse
//! `FallbackExtentUnsupported` on the ring's torus, tangent to the puck's
//! top along its rim
//! (`work/reachhold/a-puck-and-its-rounding-ring-refuse-intersect-and-difference-on-the-tangent-top.md`).

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::f64::consts::{FRAC_PI_2, PI, SQRT_2, TAU};

use geom_core::{Affine3, Arc2, Bounds, Decide, Interval, Point2, Point3, Real, Tol, Vec3};
use profile::{Profile, ProfileLoop, RawLoop, Segment, SketchPlane};
use sweep::test_support::{ball_poled_z, brick, revolved_about_y};
use sweep::{ExtrudeSide, Extrusion, Revolution, extrude};
use topo::{
    AtRestBody, AtRestPolicy, Body, BooleanError, BooleanResult, CensusContact, SplitJoinError,
    ValidationError,
};

use crate::common::differential::{VOLUME_CLAIM, VOLUME_PAD_BUDGET, every_op_both_orders, outcome};
use crate::common::oracles::{ball_volume, cap_volume};
use crate::common::poses::torax_pose;

fn tol() -> Tol {
    Tol::witness()
}

/// A one-segment circle of radius `r` about `(cx, cy)`, its one vertex
/// at azimuth `az`.
fn circle<T: Real>(cx: f64, cy: f64, r: f64, az: f64) -> ProfileLoop<T> {
    let f = T::from_f64;
    RawLoop::new([(
        Point2::new(f(cx + r * az.cos()), f(cy + r * az.sin())),
        Segment::Arc(Arc2 {
            centre: Point2::new(f(cx), f(cy)),
            radius: f(r),
            sweep: f(TAU),
        }),
    )])
}

/// `loops` extruded from `z = z0` up by `h`.
fn extruded<T: Decide + AtRestPolicy>(loops: Vec<ProfileLoop<T>>, z0: f64, h: f64) -> Body<T> {
    let f = T::from_f64;
    let profile = Profile::new(SketchPlane::<T>::xy(), loops)
        .validate(tol())
        .unwrap();
    let depth = Extrusion::Distance {
        depth: f(h),
        side: ExtrudeSide::Along,
    };
    let body = extrude(&profile, depth, tol()).unwrap().body;
    let up = Affine3::translation(Vec3::new(f(0.0), f(0.0), f(z0)));
    topo::transform_rigid(&body, &geom_core::test_support::rigid(&up, tol()), tol()).unwrap()
}

fn fin<T: AtRestPolicy>(what: &str, body: Body<T>) -> AtRestBody<T> {
    topo::test_support::finished(what, body, tol())
}

/// The plate `[−1, 1]² × [−0.5, 0]`: 2.
fn plate<T: Decide + AtRestPolicy>() -> AtRestBody<T> {
    fin(
        "the plate",
        brick((-1.0, 1.0), (-1.0, 1.0), (-0.5, 0.0), tol()),
    )
}

/// A body's shells, faces, rings, edges and vertices.
fn census<T: Real>(body: &Body<T>) -> [usize; 5] {
    [
        body.shells().count(),
        body.faces().count(),
        body.faces().map(|(_, f)| f.rings.len()).sum(),
        body.edges().count(),
        body.vertices().count(),
    ]
}

/// One run's verdict: `SOUND` at its closed form, or an empty result
/// where the closed form is empty; a body is meshed, unless it holds a
/// window in a curved face, which the mesher refuses typed
/// (`TessellateError::RingOnCurvedFace`). The body's [`census`], if
/// there is one.
fn holds(row: &str, r: Result<BooleanResult<f64>, BooleanError>, want: f64) -> Option<[usize; 5]> {
    let counted = match &r {
        Ok(BooleanResult::Body(b)) => {
            match mesh::tessellate(&b.body, 5e-3, tol()) {
                Ok(m) => {
                    mesh::validate::check_mesh(&m).unwrap_or_else(|e| panic!("{row}: mesh {e:?}"))
                }
                Err(mesh::TessellateError::RingOnCurvedFace { .. }) => {}
                Err(e) => panic!("{row}: {e:?}"),
            }
            Some(census(&b.body))
        }
        _ => None,
    };
    let line = outcome(r, want, tol());
    let ok = if want == 0.0 {
        line.starts_with("EMPTY ok")
    } else {
        line.starts_with("OK SOUND")
    };
    assert!(ok, "{row}: {line}");
    counted
}

/// Every op in both orders between `a` and `b` (volumes `va`, `vb`,
/// overlapping in `vab`), each held to its closed form ([`holds`]);
/// their censuses, by op.
fn every_op(
    what: &str,
    a: &AtRestBody<f64>,
    b: &AtRestBody<f64>,
    v: (f64, f64, f64),
) -> Vec<(&'static str, Option<[usize; 5]>)> {
    every_op_both_orders(a, b, v, tol())
        .into_iter()
        .map(|(op, r, want)| (op, holds(&format!("{what}: {op}"), r, want)))
        .collect()
}

/// A boss on the plate's census, by op, counted by hand: the union is
/// the plate's six faces, the boss's wall and top, and its rim a ring
/// of the plate's top (one shell, 8 faces, 1 ring, 15 edges, 10
/// vertices); the boss less the plate is the boss (3, 3, 2); the plate
/// less the boss is the plate (6, 12, 8).
const BOSS_ON_PLATE: [(&str, Option<[usize; 5]>); 6] = [
    ("A ∪ B", Some([1, 8, 1, 15, 10])),
    ("B ∪ A", Some([1, 8, 1, 15, 10])),
    ("A ∩ B", None),
    ("B ∩ A", None),
    ("A ∖ B", Some([1, 3, 0, 3, 2])),
    ("B ∖ A", Some([1, 6, 0, 12, 8])),
];

/// Both unions of `a` and `b`, which meet in no volume, held to `want`.
fn both_unions(what: &str, a: &AtRestBody<f64>, b: &AtRestBody<f64>, want: f64) {
    for (op, r) in [
        ("A ∪ B", topo::union(a, b, tol())),
        ("B ∪ A", topo::union(b, a, tol())),
    ] {
        holds(&format!("{what}: {op}"), r, want);
    }
}

/// The union of `a` and `b`, which must build.
fn united(a: &AtRestBody<f64>, b: &AtRestBody<f64>) -> AtRestBody<f64> {
    topo::union(a, b, tol())
        .unwrap()
        .body()
        .unwrap()
        .body
        .clone()
}

/// **A boss on a plate**: the cylinder `r = 0.3`, `z ∈ [0, 1]`, standing
/// on the top face, its rim's vertex at four azimuths; and the same
/// cylinder hung under the bottom face about `(0.2, 0.1)`. Boss `0.09π`,
/// plate 2, overlap 0.
#[test]
fn a_boss_on_a_plate() {
    let plate = plate();
    for az in [0.0, 1.0, PI, -2.0] {
        let boss = fin(
            "the boss",
            extruded(vec![circle(0.0, 0.0, 0.3, az)], 0.0, 1.0),
        );
        let what = format!("a boss, vertex at {az}");
        let counted = every_op(&what, &boss, &plate, (0.09 * PI, 2.0, 0.0));
        assert_eq!(counted, BOSS_ON_PLATE, "{what}: census");
    }
    let under = fin(
        "the boss",
        extruded(vec![circle(0.2, 0.1, 0.3, 0.5)], -1.5, 1.0),
    );
    let counted = every_op("a boss under", &under, &plate, (0.09 * PI, 2.0, 0.0));
    assert_eq!(counted, BOSS_ON_PLATE, "a boss under: census");
}

/// **The boss on the plate at `Interval`**, its vertex at two
/// azimuths: every op builds at tier 3 with the f64 run's census (∩
/// empty), and its volume enclosure brackets the closed form.
#[test]
fn a_boss_on_a_plate_at_interval() {
    let plate = plate::<Interval>();
    for az in [0.0, 1.0] {
        let boss = fin(
            "the boss",
            extruded::<Interval>(vec![circle(0.0, 0.0, 0.3, az)], 0.0, 1.0),
        );
        let (vb, vp) = (0.09 * PI, 2.0);
        for ((op, r), (_, counted)) in [
            ("A ∪ B", topo::union(&boss, &plate, tol())),
            ("B ∪ A", topo::union(&plate, &boss, tol())),
            ("A ∩ B", topo::intersect(&boss, &plate, tol())),
            ("B ∩ A", topo::intersect(&plate, &boss, tol())),
            ("A ∖ B", topo::subtract(&boss, &plate, tol())),
            ("B ∖ A", topo::subtract(&plate, &boss, tol())),
        ]
        .into_iter()
        .zip(BOSS_ON_PLATE)
        {
            let what = format!("vertex at {az}, {op}");
            let want = match op {
                "A ∪ B" | "B ∪ A" => vb + vp,
                "A ∖ B" => vb,
                "B ∖ A" => vp,
                _ => 0.0,
            };
            let r = r.unwrap_or_else(|e| panic!("{what}: {e:?}"));
            let Some(b) = r.body() else {
                assert_eq!(want, 0.0, "{what}: empty");
                assert_eq!(counted, None, "{what}: empty");
                continue;
            };
            topo::validate_geometric(&b.body, tol())
                .unwrap_or_else(|e| panic!("{what}: tier 3: {e:?}"));
            assert_eq!(Some(census(&b.body)), counted, "{what}: census");
            let v =
                topo::mass_properties(&b.body, tol(), topo::test_support::pinned_resolution(tol()))
                    .expect("the measured body")
                    .met()
                    .expect("the measured body")
                    .volume;
            assert!(
                v.lo() <= want && want <= v.hi() && v.hi() - v.lo() < 1e-9,
                "{what}: [{}, {}] against {want}",
                v.lo(),
                v.hi()
            );
        }
    }
}

/// **The pierced face's other rings**: a second boss on a plate whose
/// top face holds the first boss's ring, outside the new rim; and a boss
/// over a hole `r = 0.1`, whose ring lies inside the rim and goes to the
/// disc the rim cuts out.
#[test]
fn a_boss_beside_a_ring_and_over_a_hole() {
    let first = fin(
        "the first boss",
        extruded(vec![circle(-0.5, 0.0, 0.3, 0.0)], 0.0, 1.0),
    );
    let second = fin(
        "the second boss",
        extruded(vec![circle(0.5, 0.0, 0.3, 1.0)], 0.0, 1.0),
    );
    let one = topo::union(&plate(), &first, tol())
        .unwrap()
        .body()
        .unwrap()
        .body
        .clone();
    every_op(
        "the second boss",
        &second,
        &one,
        (0.09 * PI, 2.0 + 0.09 * PI, 0.0),
    );
    let square = RawLoop::polygon([
        Point2::new(-1.0, -1.0),
        Point2::new(1.0, -1.0),
        Point2::new(1.0, 1.0),
        Point2::new(-1.0, 1.0),
    ]);
    let holed = fin(
        "the holed plate",
        extruded(vec![square, circle(0.0, 0.0, 0.1, 0.0)], -0.5, 0.5),
    );
    let boss = fin(
        "the boss",
        extruded(vec![circle(0.0, 0.0, 0.3, 0.0)], 0.0, 1.0),
    );
    every_op(
        "a boss over a hole",
        &boss,
        &holed,
        (0.09 * PI, 2.0 - 0.005 * PI, 0.0),
    );
}

/// **The containment fallback cuts a closed rim, beside a ring too.** The
/// boss `r = 0.3` less the plate is the boss, by the fallback, which
/// divides the plate's top along the boss's rim, one closed edge whose
/// vertex touches the top with no surgery, with a circular edge from a
/// lone vertex: the disc is held by the boss's floor, and the rest of the
/// top is a discard.
///
/// Over the hole `r = 0.1` the cut is the same: the rim's vertex reads in
/// the corner of the top it leaves into, beside the hole's ring, because
/// the corner's lever is each bound's extent from above
/// (`geom_brep::edge_extent_sup`), the closed ring's diameter, where its
/// collapsed chord read every side Zero and the vertex in no corner
/// (`work/flush/a-fallback-cut-refuses-beside-a-ring-a-seam-or-a-pinch.md`,
/// its ring pose). With a lid standing on the boss as well, the lid's
/// bottom is cut along the boss's top rim and held there too: each face
/// is cut on its own. The body is the boss throughout. Red before the
/// fallback cut: the plain top was one whole row; at the chord lever the
/// holed top was uncut, "lies in no corner".
#[test]
fn the_fallback_cuts_a_closed_rim_beside_a_ring() {
    let boss = fin(
        "the boss",
        extruded(vec![circle(0.0, 0.0, 0.3, 0.0)], 0.0, 1.0),
    );
    let square = || {
        RawLoop::polygon([
            Point2::new(-1.0, -1.0),
            Point2::new(1.0, -1.0),
            Point2::new(1.0, 1.0),
            Point2::new(-1.0, 1.0),
        ])
    };
    let holed = || {
        fin(
            "the holed plate",
            extruded(vec![square(), circle(0.0, 0.0, 0.1, 0.0)], -0.5, 0.5),
        )
    };
    let lid = fin("the lid", extruded(vec![square()], 1.0, 0.5));
    let lidded = united(&holed(), &lid);
    // Each case, and whether a lid's bottom is cut beside the top.
    let cases = [
        ("the plate", plate(), false),
        ("the holed plate", holed(), false),
        ("the holed plate and a lid", lidded, true),
    ];
    for (label, other, lid_held) in cases {
        let Ok(BooleanResult::Body(out)) = topo::subtract(&boss, &other, tol()) else {
            panic!("{label}: the boss less it builds");
        };
        assert_eq!(
            out.kind,
            topo::BooleanResultKind::OperandA,
            "{label}: the fallback"
        );
        let n = &out.naming;
        let root = |f| n.face_fragments_b.root(f);
        // The rows by the plane of the B face they name: the top at z 0,
        // the lid's bottom at z 1.
        let level = |f: topo::FaceKey| match topo::face_carrier(&other, root(f)) {
            Some(topo::CarrierDesc::Plane { origin, .. }) => (origin.z * 2.0).round() as i8,
            c => panic!("{label}: a plane face, {c:?}"),
        };
        let held: Vec<i8> = n.substituted.iter().map(|s| level(s.face)).collect();
        let uncut: Vec<(i8, &str)> = n
            .uncut
            .iter()
            .map(|u| (level(u.face), u.what.as_str()))
            .collect();
        let want: Vec<i8> = if lid_held { vec![0, 2] } else { vec![0] };
        assert_eq!(held, want, "{label}: the faces held in part, by level");
        for s in &n.substituted {
            assert!(
                n.discards.iter().any(|r| r.operand == s.operand
                    && r.face != s.face
                    && root(r.face) == root(s.face)),
                "{label}: the rest of a cut face is a discard"
            );
        }
        assert_eq!(uncut, vec![], "{label}: nothing left uncut");
    }
}

/// **A plug in an annulus's bore**: the plate united with the tube
/// `r ∈ [0.3, 0.5]`, `z ∈ [0, 1]`, and the cylinder `r = 0.3` of the
/// same height filling its bore, every vertex at one azimuth, at two.
/// The plug's floor rim lies in the plate's top along the bore's floor
/// rim, a closed edge of each through one site, in a ring of the
/// plate's top: its join is a lone ring holding the null edge and the
/// closed edge (`chord_join`'s `LoneRing::Closed`), whose chord runs
/// against the edge's half beside it (`closed_chord_runs_forward`) and
/// is stated in a chart. Plug `0.09π`, the plate and tube `2 + 0.16π`,
/// overlap 0.
#[test]
fn a_plug_in_an_annulus_bore() {
    for az in [0.0, 1.0] {
        let tube = fin(
            "the tube",
            extruded(
                vec![circle(0.0, 0.0, 0.5, az), circle(0.0, 0.0, 0.3, az)],
                0.0,
                1.0,
            ),
        );
        let annulus = united(&plate(), &tube);
        let plug = fin(
            "the plug",
            extruded(vec![circle(0.0, 0.0, 0.3, az)], 0.0, 1.0),
        );
        every_op(
            &format!("a plug, vertices at {az}"),
            &plug,
            &annulus,
            (0.09 * PI, 2.0 + 0.16 * PI, 0.0),
        );
    }
}

/// What one op of [`a_boss_over_a_hole_by_pose`] reads.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Reads {
    /// Empty, as its closed form is.
    Empty,
    /// `SOUND` at its closed form, in this many shells.
    Sound(usize),
    /// Two shells at the closed form, the boss and the plate touching
    /// along the rim circle, whose tier 3′ finds that edge–edge overlap
    /// unrecorded, and nothing else but curved pairs it cannot decide.
    Unrecorded,
    /// Two shells at the closed form, the boss and the plate touching at
    /// a point, whose tier 3′ fails only on curved pairs it cannot
    /// decide
    /// (`work/restread/census-cross-solid-curved-pairs-undecidable-on-shell-results.md`).
    Undecided,
    /// `TangentCrescentUnsupported`.
    Crescent,
    /// `Join(SectionLoopUndecided)`.
    LoopsUndecided,
    /// `Join(UnpairedLooseEnds { count })`.
    Loose(usize),
}

/// A pose's unions, over the through hole and over the pocket; its
/// intersections are empty and its differences `SOUND` in one shell.
type Cells = [Reads; 2];

/// One op's run held to what it `reads` ([`Reads`]), its closed form
/// `want`.
fn reads_as(row: &str, r: Result<BooleanResult<f64>, BooleanError>, want: f64, reads: Reads) {
    let refused = |r: &Result<BooleanResult<f64>, BooleanError>| match r {
        Err(BooleanError::TangentCrescentUnsupported { .. }) => Some(Reads::Crescent),
        Err(BooleanError::Join(j)) => match j.error() {
            SplitJoinError::SectionLoopUndecided { .. } => Some(Reads::LoopsUndecided),
            SplitJoinError::UnpairedLooseEnds { count } => Some(Reads::Loose(*count)),
            _ => None,
        },
        _ => None,
    };
    match reads {
        Reads::Empty => {
            assert_eq!(want, 0.0, "{row}: the closed form");
            holds(row, r, want);
        }
        Reads::Sound(shells) => {
            let counted = holds(row, r, want);
            assert_eq!(counted.map(|c| c[0]), Some(shells), "{row}: shells");
        }
        Reads::Unrecorded | Reads::Undecided => {
            let r = r.unwrap_or_else(|e| panic!("{row}: {e:?}"));
            let b = r.body().unwrap_or_else(|| panic!("{row}: empty"));
            assert_eq!(census(&b.body)[0], 2, "{row}: shells");
            let t3p = topo::validate_pseudomanifold(&b.body, &b.contacts, tol());
            let rim = |e: &ValidationError| {
                matches!(
                    e,
                    ValidationError::UnrecordedContact {
                        contact: CensusContact::EdgeEdgeOverlap { .. },
                        ..
                    }
                )
            };
            let curved =
                |e: &ValidationError| matches!(e, ValidationError::CensusUndecidable { .. });
            assert!(
                t3p.as_ref()
                    .is_err_and(|es| es.iter().any(rim) == (reads == Reads::Unrecorded)
                        && es.iter().all(|e| rim(e) || curved(e))),
                "{row}: tier 3′ {t3p:?}"
            );
            let v =
                topo::mass_properties(&b.body, tol(), topo::test_support::pinned_resolution(tol()))
                    .expect("the measured body")
                    .met()
                    .expect("the measured body")
                    .volume;
            assert!((v - want).abs() < 1e-7, "{row}: volume {v} against {want}");
            let line = outcome(Ok(r), want, tol());
            assert!(
                line.starts_with("OK BAD t2=true t3p=false cert=true operand=true"),
                "{row}: {line}"
            );
        }
        refusal => assert_eq!(
            refused(&r),
            Some(refusal),
            "{row}: {:?}",
            r.map(|_| "a body")
        ),
    }
}

/// The outward normal's `z` of `face` of `body` where it is a plane
/// through `z = 0` facing straight up or down.
fn floor_or_top(body: &Body<f64>, face: topo::FaceKey) -> Option<f64> {
    let pose = topo::readback::face_pose(body, face).ok()?;
    let z = if pose.sense {
        pose.axis.z
    } else {
        -pose.axis.z
    };
    (topo::readback::face_carrier_kind(body, face).ok()? == geom::SurfaceKind::Plane
        && pose.origin.z.abs() < 1e-12
        && (z.abs() - 1.0).abs() < 1e-12)
        .then_some(z)
}

/// A rim of radius `r` about `(cx, 0)`: a one-segment circle with its
/// vertex at azimuth `az` (`one`), or two arcs with vertices there and
/// opposite.
fn rim(one: bool, cx: f64, r: f64, az: f64) -> ProfileLoop<f64> {
    if one {
        return circle(cx, 0.0, r, az);
    }
    let arc = |a: f64| {
        (
            Point2::new(cx + r * a.cos(), r * a.sin()),
            Segment::Arc(Arc2 {
                centre: Point2::new(cx, 0.0),
                radius: r,
                sweep: PI,
            }),
        )
    };
    RawLoop::new([arc(az), arc(az + PI)])
}

/// The plate's outline, `[−1, 1]²`.
fn square() -> ProfileLoop<f64> {
    RawLoop::polygon([
        Point2::new(-1.0, -1.0),
        Point2::new(1.0, -1.0),
        Point2::new(1.0, 1.0),
        Point2::new(-1.0, 1.0),
    ])
}

/// A pose of [`a_boss_over_a_hole_by_pose`]: its name, the hole's
/// centre `x`, radius and vertex azimuth, the boss's vertex azimuth, and
/// what its ops read, with one-segment rims and with two-arc rims alike.
type Pose = (&'static str, f64, f64, f64, f64, Cells);

/// The poses of [`a_boss_over_a_hole_by_pose`], the tangent ones first.
fn poses() -> [Pose; 16] {
    use Reads::{Crescent, Sound, Undecided, Unrecorded};
    let crescent = [Crescent; 2];
    let built = [Sound(1); 2];
    [
        (
            "tangent inside, vertices there",
            0.2,
            0.1,
            0.0,
            0.0,
            crescent,
        ),
        (
            "tangent inside, the boss's at 1",
            0.2,
            0.1,
            0.0,
            1.0,
            crescent,
        ),
        (
            "tangent inside, the boss's at π",
            0.2,
            0.1,
            0.0,
            PI,
            crescent,
        ),
        (
            "tangent inside, the hole's at 1",
            0.2,
            0.1,
            1.0,
            0.0,
            crescent,
        ),
        (
            "tangent inside, the hole's at π",
            0.2,
            0.1,
            PI,
            0.0,
            crescent,
        ),
        (
            "tangent inside, neither there",
            0.2,
            0.1,
            2.0,
            1.0,
            crescent,
        ),
        (
            "the boss inside the hole, tangent",
            -0.1,
            0.4,
            0.0,
            0.0,
            [Undecided; 2],
        ),
        ("tangent outside, vertices there", 0.4, 0.1, PI, 0.0, built),
        (
            "tangent outside, the boss's there",
            0.4,
            0.1,
            0.0,
            0.0,
            built,
        ),
        (
            "tangent outside, the hole's there",
            0.4,
            0.1,
            PI,
            1.0,
            built,
        ),
        (
            "tangent outside, the boss's at 1",
            0.4,
            0.1,
            0.0,
            1.0,
            built,
        ),
        ("tangent outside, neither there", 0.4, 0.1, 2.0, 1.0, built),
        ("inside, apart", 0.1, 0.1, 0.0, 0.0, [Sound(1), Sound(2)]),
        ("concentric, equal", 0.0, 0.3, 0.0, 0.0, [Unrecorded; 2]),
        (
            "concentric, equal, the hole's at 1",
            0.0,
            0.3,
            1.0,
            0.0,
            [Unrecorded; 2],
        ),
        ("concentric, larger", 0.0, 0.4, 0.0, 0.0, [Sound(2); 2]),
    ]
}

/// One pose's ops in both forms, through and pocketed, every op in both
/// orders, both operands turned by `turn` where one is given.
fn by_pose((pose, hx, hr, hole_az, boss_az, cells): Pose, turn: Option<&Affine3<f64>>) {
    let placed = |what: &str, body: Body<f64>| match turn {
        None => fin(what, body),
        Some(turn) => fin(
            what,
            topo::transform_rigid(&body, &geom_core::test_support::rigid(turn, tol()), tol())
                .unwrap(),
        ),
    };
    let vb = 0.09 * PI;
    for form in [true, false] {
        for (pocket, depth) in [(false, 0.5), (true, 0.25)] {
            let what = format!(
                "{pose}, {}, {}{}",
                if form { "one segment" } else { "two arcs" },
                if pocket { "pocket" } else { "through" },
                if turn.is_some() { ", turned" } else { "" }
            );
            let boss = placed(
                "the boss",
                extruded(vec![rim(form, 0.0, 0.3, boss_az)], 0.0, 1.0),
            );
            let holed = if pocket {
                let cutter = fin(
                    "the cutter",
                    extruded(vec![rim(form, hx, hr, hole_az)], -0.25, 0.5),
                );
                let cut = topo::subtract(&plate(), &cutter, tol()).unwrap();
                placed(
                    "the pocketed plate",
                    cut.body().unwrap().body.clone().into_body(),
                )
            } else {
                placed(
                    "the holed plate",
                    extruded(vec![square(), rim(form, hx, hr, hole_az)], -0.5, 0.5),
                )
            };
            let vh = 2.0 - PI * hr * hr * depth;
            for (op, r, want) in every_op_both_orders(&boss, &holed, (vb, vh, 0.0), tol()) {
                let reads = match op {
                    "A ∪ B" | "B ∪ A" => cells[usize::from(pocket)],
                    "A ∩ B" | "B ∩ A" => Reads::Empty,
                    _ => Reads::Sound(1),
                };
                if let (None, Err(BooleanError::TangentCrescentUnsupported { a_face, b_face })) =
                    (turn, &r)
                {
                    let (a, b) = if op == "A ∪ B" {
                        (&boss, &holed)
                    } else {
                        (&holed, &boss)
                    };
                    for (face, body) in [(*a_face, a), (*b_face, b)] {
                        assert_eq!(
                            floor_or_top(body, face),
                            Some(if std::ptr::eq(body, &boss) { -1.0 } else { 1.0 }),
                            "{what}: {op}: the boss's floor and the plate's top"
                        );
                    }
                }
                reads_as(&format!("{what}: {op}"), r, want, reads);
            }
        }
    }
}

/// **A boss over a hole, by pose**: the boss `r = 0.3`, `z ∈ [0, 1]`,
/// over the plate holed through (`z ∈ [−0.5, 0]`) or pocketed (cut by
/// `subtract` to `z = −0.25`) by a circle about `(x, 0)`, each rim a
/// one-segment circle with its vertex at the azimuth given, or two arcs
/// with vertices there and opposite ([`poses`]). Every op in both
/// orders.
///
/// Where the hole lies inside the boss's rim, tangent to it at
/// `(0.3, 0, 0)`, the boss's floor and the plate's top face overlap in
/// a crescent that narrows to the tangent point. A vertex of either rim
/// there puts a vertex pair at the point (the other rim is split
/// there), and so does a touch with no vertex there, where the sweep
/// splits both rims; the union refuses `TangentCrescentUnsupported`,
/// naming the floor and the top
/// (`work/cleave/a-union-across-a-tangent-crescent-refuses-unbuilt.md`).
/// Built, the union would be one shell (through) or two (pocket, a
/// cavity touching the outer shell at the point). The intersection is
/// empty and the differences keep the operands.
///
/// The boss inside a hole tangent to its rim (`r = 0.4` about
/// `(−0.1, 0)`) curves the other way: the floor and the top only touch,
/// and the union is two solids. Outside the rim, tangent, the faces'
/// sectors coincide at first order and the union builds in one shell,
/// in both forms, wherever the vertices lie: a one-segment rim's
/// midpoint is read where its two-arc form's far vertex would be
/// (`boolean::sectors::closed_reach`), and the boss's rim walled off in
/// the plate's top at the shared vertex (`boolean::join`'s
/// `closes_at_its_site`). The pose `tangent outside, the boss's there`
/// has both rims at `circle()`'s phase, each vertex at azimuth 0. A
/// concentric hole of the
/// boss's radius leaves two solids touching along the rim, and the
/// union's records name only the vertex pair
/// (`work/cleave/a-union-touching-along-a-coincident-rim-records-only-its-vertex-pair.md`);
/// a larger one leaves them apart.
///
/// The same crescent between faces aligned (a holed plate standing on
/// `z = 0` beside the boss, its hole tangent inside the boss's rim)
/// overlaps the operands, and every op refuses at the join. In the
/// one-segment form the plate's top cuts the boss's wall at the hole's
/// vertex, and the piece of wall between the two floors has every edge
/// on the plate's boundary: its rims in the plate's top and floor, its
/// wrap edge along the hole's wall. No witness reads that curved face's
/// interior, and the boss's floor is flush with the plate's, so neither
/// loop of the floor rim's section reads a side.
#[test]
fn a_boss_over_a_hole_by_pose() {
    use Reads::{LoopsUndecided, Loose};
    for pose in poses() {
        by_pose(pose, None);
    }
    // Pinned red: the one-segment form's refusal is the witness
    // ladder's,
    // `work/honesplit/the-uncut-shell-witness-reads-no-curved-face-interior.md`.
    for (form, refusal) in [(true, LoopsUndecided), (false, Loose(4))] {
        let what = if form { "one segment" } else { "two arcs" };
        let boss = fin(
            "the boss",
            extruded(vec![rim(form, 0.0, 0.3, 0.0)], 0.0, 1.0),
        );
        let beside = fin(
            "the plate beside",
            extruded(vec![square(), rim(form, 0.2, 0.1, 0.0)], 0.0, 0.5),
        );
        for (op, r, _) in every_op_both_orders(&boss, &beside, (0.0, 0.0, 0.0), tol()) {
            reads_as(&format!("aligned, {what}: {op}"), r, 0.0, refusal);
        }
    }
}

/// **The tangent poses turned**: each tangent pose of
/// [`a_boss_over_a_hole_by_pose`], both operands turned rigidly off the
/// axes ([`torax_pose`]), reads the cells it reads at rest. The crescent
/// refusal's two faces are named at rest only ([`floor_or_top`] reads
/// them in the rest frame).
#[test]
fn a_boss_over_a_hole_tangent_to_it_turned() {
    let turn = torax_pose();
    for pose in poses().into_iter().filter(|p| p.0.contains("tangent")) {
        by_pose(pose, Some(&turn));
    }
}

/// **A boss over two holes, each tangent outside its rim, no vertex at
/// either touch**: the boss `r = 0.3` with its vertex at azimuth 1, the
/// plate holed through by `r = 0.1` about `(±0.4, 0)` with their vertices
/// at azimuth 2. The sweep splits the boss's rim at both touches, so one
/// fragment of it runs from touch to touch with no vertex of its own in
/// the plate's top: the vertex pairs there read it running into the top
/// one order down (`boolean::sectors::runs_into`), and the difference
/// cuts the top along the whole rim. Every op in both orders, both
/// forms, at rest and turned, at the closed form.
#[test]
fn a_boss_over_two_holes_tangent_outside_with_no_vertex_at_either_touch() {
    let vb = 0.09 * PI;
    let vh = 2.0 - 2.0 * PI * 0.01 * 0.5;
    for (turn, placed) in [("at rest", None), ("turned", Some(torax_pose()))] {
        for one in [true, false] {
            let place = |what: &str, body: Body<f64>| match &placed {
                None => fin(what, body),
                Some(m) => fin(
                    what,
                    topo::transform_rigid(&body, &geom_core::test_support::rigid(m, tol()), tol())
                        .unwrap(),
                ),
            };
            let boss = place(
                "the boss",
                extruded(vec![rim(one, 0.0, 0.3, 1.0)], 0.0, 1.0),
            );
            let holed = place(
                "the plate holed twice",
                extruded(
                    vec![square(), rim(one, 0.4, 0.1, 2.0), rim(one, -0.4, 0.1, 2.0)],
                    -0.5,
                    0.5,
                ),
            );
            let what = format!(
                "two holes, {}, {turn}",
                if one { "one segment" } else { "two arcs" }
            );
            every_op(&what, &boss, &holed, (vb, vh, 0.0));
        }
    }
}

/// **A rim flush in the partner's top, tangent inside its rim**: `A` the
/// one-segment cylinder `r = 0.3`, `z ∈ [0, 1]`, inside the one-segment
/// cylinder `B`, `r = 0.5` about `(−0.2, 0)`, `z ∈ [−0.5, 1]`; the walls
/// are tangent along the ruling through `(0.3, 0)` and the tops
/// coplanar, `A`'s rim tangent inside `B`'s at `(0.3, 0, 1)`. `A`'s
/// vertex lies at azimuth 0, 1 or π, `B`'s at 0 or 1; every op in both
/// orders, at rest and turned. Each cell builds its closed form in one
/// shell (read through `differential::outcome`, unmeshed: the mesher's
/// certificate on `B`'s wall is its own question), or meets one of
/// two pinned limits:
///
/// - With `A`'s vertex at the touch, where the op reads the two tops
///   facing one way and keeps `A`'s, the join does not take a rim lying
///   flush in a face at its one vertex (`boolean::join`'s `stands_on`),
///   and `A ∪ B` and `B ∩ A` refuse `SingleSiteSectionLoop`
///   (`work/tangrim/a-flush-closed-rim-at-its-vertex-is-joined-reversed.md`).
/// - Where a seam ruling lies on the tangent line, `A`'s with its
///   vertex at the touch (any other op) or `B`'s with its vertex there
///   (`B ∖ A`), the op may refuse `EdgeInFaceGap::CurvedFace` rather
///   than carry the ruling as one edge lying in the partner's wall where
///   the crescent's two cusps owe two ([`one_shell_or_gap_at`]).
///
/// With `A`'s vertex elsewhere the sweep splits its rim at `B`'s
/// vertex, and the cut along it in `B`'s dropped top ends at that
/// vertex, a corner of `B`'s closed rim, read at the rim's reach
/// (`boolean::fate`'s `leaving`).
#[test]
fn a_rim_flush_in_its_partners_top_tangent_inside_builds_or_meets_a_pinned_limit() {
    let va = 0.09 * PI;
    let vb = 0.25 * PI * 1.5;
    for (turn, placed) in [("at rest", None), ("turned", Some(torax_pose()))] {
        for (a_az, b_az) in [(0.0, 0.0), (0.0, 1.0), (1.0, 0.0), (PI, 0.0)] {
            let place = |what: &str, body: Body<f64>| match &placed {
                None => fin(what, body),
                Some(m) => fin(
                    what,
                    topo::transform_rigid(&body, &geom_core::test_support::rigid(m, tol()), tol())
                        .unwrap(),
                ),
            };
            let a = place(
                "the inner rod",
                extruded(vec![circle(0.0, 0.0, 0.3, a_az)], 0.0, 1.0),
            );
            let b = place(
                "the outer rod",
                extruded(vec![circle(-0.2, 0.0, 0.5, b_az)], -0.5, 1.5),
            );
            for (op, r, want) in every_op_both_orders(&a, &b, (va, vb, va), tol()) {
                let row = format!("A's vertex at {a_az}, B's at {b_az}, {turn}: {op}");
                if a_az == 0.0 && matches!(op, "A ∪ B" | "B ∩ A") {
                    assert!(
                        matches!(&r, Err(BooleanError::Join(j))
                            if matches!(j.error(), SplitJoinError::SingleSiteSectionLoop { count: 1 })),
                        "{row}: {:?}",
                        r.map(|_| ())
                    );
                } else {
                    let seam = a_az == 0.0 || (b_az == 0.0 && op == "B ∖ A");
                    one_shell_or_gap_at(&row, r, want, seam);
                }
            }
        }
    }
}

/// `r` holds its closed form `want` (`differential::outcome`) in one
/// shell (an empty result where it is empty), or with `gap` refuses
/// `EdgeInFaceGap::CurvedFace`, the parting declining to build a
/// crescent's two cusps on a curved face until
/// `work/splitdoor/the-boolean-builds-the-slit-on-a-curved-face-once-it-is-meshed-and-measured.md`
/// builds them.
fn one_shell_or_gap_at(
    row: &str,
    r: Result<BooleanResult<f64>, BooleanError>,
    want: f64,
    gap: bool,
) {
    let shells = match &r {
        Ok(BooleanResult::Body(b)) => b.body.shells().count(),
        _ => 0,
    };
    let line = outcome(r, want, tol());
    if gap && line.contains("gap: CurvedFace") {
        return;
    }
    if want == 0.0 {
        assert!(line.starts_with("EMPTY ok"), "{row}: {line}");
    } else {
        assert!(line.starts_with("OK SOUND"), "{row}: {line}");
        assert_eq!(shells, 1, "{row}: shells");
    }
}

/// **A fate cut whose kept rim leaves along the dropped face's boundary
/// and bends into it**: the kept copy's rim, tangent inside the dropped
/// copy's, leaves their touch along the dropped face's boundary edge at
/// first order, and the cut places it in the corner it bends into one
/// order down (`boolean::fate`'s `corner_holds`). `A ∩ B` of the
/// two-arc rods of
/// [`a_rim_flush_in_its_partners_top_tangent_inside_builds_or_meets_a_pinned_limit`],
/// `B` over `z ∈ [0, 0.5]`, keeps `A`'s floor and cuts `B`'s along `A`'s
/// rim; and the hole's rod `r = 0.1` about `(0.2, 0)` inside the boss's,
/// both one-segment over `z ∈ [0, 1]`, tangent at `(0.3, 0)` where the
/// boss's vertex lies. Every op in both orders, at its closed form in
/// one shell, at rest and turned, but that a difference of the second
/// pose may refuse `EdgeInFaceGap::CurvedFace`, the boss's seam ruling
/// lying in the hole's wall on the tangent line, until
/// `work/splitdoor/the-boolean-builds-the-slit-on-a-curved-face-once-it-is-meshed-and-measured.md`
/// builds it ([`one_shell_or_gap_at`]).
///
/// The first pose's `A ∩ B` is the one that turns on the bend. The
/// second is the one-segment form of the same contact, the cut's end
/// the boss's one vertex: it reaches the reading at both of a corner's
/// bounds, and its cells are read at their closed forms or the
/// `CurvedFace` limit.
#[test]
fn a_kept_rim_leaving_along_the_dropped_faces_boundary_cuts_where_it_bends() {
    for (turn, placed) in [("at rest", None), ("turned", Some(torax_pose()))] {
        let place = |what: &str, body: Body<f64>| match &placed {
            None => fin(what, body),
            Some(m) => fin(
                what,
                topo::transform_rigid(&body, &geom_core::test_support::rigid(m, tol()), tol())
                    .unwrap(),
            ),
        };
        let poses = [
            (
                "the two-arc rods, their floors flush",
                place(
                    "the inner rod",
                    extruded(vec![rim(false, 0.0, 0.3, 0.0)], 0.0, 1.0),
                ),
                place(
                    "the outer rod",
                    extruded(vec![rim(false, -0.2, 0.5, 0.0)], 0.0, 0.5),
                ),
                (0.09 * PI, 0.125 * PI, 0.045 * PI),
            ),
            (
                "the hole's rod inside the boss's",
                place(
                    "the boss's rod",
                    extruded(vec![rim(true, 0.0, 0.3, 0.0)], 0.0, 1.0),
                ),
                place(
                    "the hole's rod",
                    extruded(vec![rim(true, 0.2, 0.1, PI)], 0.0, 1.0),
                ),
                (0.09 * PI, 0.01 * PI, 0.01 * PI),
            ),
        ];
        for (k, (pose, a, b, v)) in poses.into_iter().enumerate() {
            for (op, r, want) in every_op_both_orders(&a, &b, v, tol()) {
                let gap = k == 1 && op.contains('∖');
                one_shell_or_gap_at(&format!("{pose}, {turn}: {op}"), r, want, gap);
            }
        }
    }
}

/// The widest certified volume enclosure (`volume_pad`, m³) the ellipse
/// rims' bodies are judged through, in place of
/// `differential::VOLUME_PAD_BUDGET`, which the tilted boss alone does
/// not meet above ε = 1e-12 (`2.9e-5` at 1e-6 and `4.4e-7` at 1e-9 in the
/// one-segment form, measured 2026-11-01). The results measure at most
/// `3.0e-5`, `5.8e-7` and `1.0e-9` at 1e-6, 1e-9 and 1e-12; at 1e-12 they
/// are held to the budget itself.
const ELLIPSE_PAD_CEILING: f64 = 1e-4;

/// **Ellipse rims tangent outside**: both cylinders of
/// [`a_boss_over_a_hole_by_pose`]'s tangent-outside pose tilted `0.3`
/// rad about `x` and cut by `z = 0` (the boss by a slab, the plate's hole
/// through it), so both rims are ellipses tangent at `(0.3, 0, 0)`. One
/// segment with both vertices at the touch, the boss's there alone or
/// the hole's there alone, and two arcs with a vertex of each there. In
/// the one-segment form a closed rim is copied whole on its own carrier
/// (`chord_join`'s `closed_edge_spec`) and the loop closes at its site
/// (`boolean::join`'s `closes_at_its_site`), as for a circle.
///
/// Every op in both orders, at rest and turned, builds at its closed
/// form within [`ELLIPSE_PAD_CEILING`], tiers 1–3 and the certificate.
/// Each passes tier 3′ but the unions, whose census reads the two rims,
/// coplanar and tangent, through the curve-pair touch, and escalates on
/// `curve_pair_coplanar_sub_side` (pinned red:
/// `work/cpair/two-coplanar-ellipses-have-no-meeting-method.md`).
#[test]
fn ellipse_rims_tangent_outside_build_every_op() {
    let tilt = Affine3::rotation_about_axis(Point3::origin(), Vec3::unit_x(), 0.3);
    let tilted = |body: Body<f64>| {
        topo::transform_rigid(&body, &geom_core::test_support::rigid(&tilt, tol()), tol()).unwrap()
    };
    let slab = fin(
        "the slab",
        brick((-2.0, 2.0), (-2.0, 2.0), (0.0, 1.0), tol()),
    );
    // A section by `z = const` is the rim's ellipse: its circle's area
    // over the tilt's cosine.
    let stretch = 1.0 / 0.3f64.cos();
    let (va, vb) = (PI * 0.09 * stretch, 2.0 - PI * 0.01 * 0.5 * stretch);
    for (pose, one, hole_az, boss_az) in [
        ("vertices there", true, PI, 0.0),
        ("the boss's there", true, 0.0, 0.0),
        ("the hole's there", true, PI, PI),
        ("two arcs", false, PI, 0.0),
    ] {
        let tall = fin(
            "the tall boss",
            tilted(extruded(vec![rim(one, 0.0, 0.3, boss_az)], -3.0, 6.0)),
        );
        let boss = topo::intersect(&tall, &slab, tol())
            .unwrap()
            .body()
            .unwrap()
            .body
            .clone();
        let bore = fin(
            "the tilted bore",
            tilted(extruded(vec![rim(one, 0.4, 0.1, hole_az)], -3.0, 6.0)),
        );
        let holed = topo::subtract(&plate(), &bore, tol())
            .unwrap()
            .body()
            .unwrap()
            .body
            .clone();
        for (turn, m) in [("at rest", None), ("turned", Some(torax_pose()))] {
            let place = |what: &str, b: &AtRestBody<f64>| match &m {
                None => b.clone(),
                Some(m) => fin(
                    what,
                    topo::transform_rigid(b, &geom_core::test_support::rigid(m, tol()), tol())
                        .unwrap(),
                ),
            };
            let (a, b) = (place("the boss", &boss), place("the plate", &holed));
            for (op, r, want) in every_op_both_orders(&a, &b, (va, vb, 0.0), tol()) {
                ellipse_rims_hold(&format!("{pose}, {turn}: {op}"), r, want, op.contains('∪'));
            }
        }
    }
}

/// One of [`ellipse_rims_tangent_outside_build_every_op`]'s runs: empty
/// where `want` is, else a body at `want` within its pad, the pad within
/// [`ELLIPSE_PAD_CEILING`] (the budget at ε ≤ 1e-12), tiers 1–3 and the
/// certificate; tier 3′ passes, or, on a `union`, escalates on the
/// coplanar touch.
fn ellipse_rims_hold(
    row: &str,
    r: Result<BooleanResult<f64>, BooleanError>,
    want: f64,
    union: bool,
) {
    let res = match r {
        Ok(BooleanResult::Body(res)) => res,
        Ok(_) if want == 0.0 => return,
        other => panic!("{row}: {:?}", other.map(|_| ())),
    };
    let body = &res.body;
    topo::validate(body).unwrap_or_else(|e| panic!("{row}: tier 1: {e:?}"));
    topo::validate_closed(body).unwrap_or_else(|e| panic!("{row}: tier 2: {e:?}"));
    topo::validate_geometric(body, tol()).unwrap_or_else(|e| panic!("{row}: tier 3: {e:?}"));
    topo::validate_geometric_certificate(body, tol())
        .unwrap_or_else(|e| panic!("{row}: certificate: {e:?}"));
    let m = topo::test_support::pinned_volume(row, body, tol());
    let ceiling = if tol().eps() <= 1e-12 {
        VOLUME_PAD_BUDGET
    } else {
        ELLIPSE_PAD_CEILING
    };
    assert!(
        m.volume_pad <= ceiling && (m.volume - want).abs() <= m.volume_pad + VOLUME_CLAIM,
        "{row}: volume {} ± {} against the closed form {want}, ceiling {ceiling}",
        m.volume,
        m.volume_pad
    );
    let three_prime = topo::validate_pseudomanifold(body, &res.contacts, tol());
    if union {
        let e = format!("{:?}", three_prime.expect_err(row));
        assert!(
            e.contains("CensusEscalated") && e.contains("curve_pair_coplanar_sub_side"),
            "{row}: tier 3′ moved: {e}"
        );
    } else {
        three_prime.unwrap_or_else(|e| panic!("{row}: tier 3′: {e:?}"));
    }
}

/// A rim of radius `r` about `(cx, 0)` with vertices at the azimuths
/// `az`, ascending within one turn, its arcs running between them.
fn arcs_at(cx: f64, r: f64, az: &[f64]) -> ProfileLoop<f64> {
    rim_about((cx, 0.0), r, az)
}

/// How a short-arc pose's union reads ([`short_arcs_at_a_crescents_tip`]).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Tip {
    /// `SOUND` in one shell at the closed form; ∩ empty and both
    /// differences `SOUND` alike.
    Built,
    /// `TangentCrescentUnsupported`.
    Crescent,
    /// The profile escalates on its short arcs.
    Unprofiled,
    /// `Escalated { decision: Crossing(OnEdge) }`.
    OnEdge,
    /// `CurvedPierceEscalated { decision: BoundaryMeeting }`: the rim's
    /// meeting with the other rim, read in band, off the other's wall.
    Meeting,
    /// `Escalated { decision: Coincidence(Sectors, _) }`.
    Sectors,
    /// `CurvedPierceUnsupported`.
    Pierce,
}

/// What `r` reads as a [`Tip`] other than [`Tip::Built`].
fn refused_as(r: &Result<BooleanResult<f64>, BooleanError>) -> Option<Tip> {
    use topo::BooleanDecision::{Coincidence, Crossing};
    Some(match r.as_ref().err()? {
        BooleanError::TangentCrescentUnsupported { .. } => Tip::Crescent,
        BooleanError::Escalated {
            decision: Crossing(topo::CrossingDecision::OnEdge),
            ..
        } => Tip::OnEdge,
        BooleanError::CurvedPierceEscalated {
            decision: topo::BooleanDecision::BoundaryMeeting,
            ..
        } => Tip::Meeting,
        BooleanError::Escalated {
            decision: Coincidence(topo::Coincide::Sectors, _),
            ..
        } => Tip::Sectors,
        BooleanError::CurvedPierceUnsupported { .. } => Tip::Pierce,
        _ => return None,
    })
}

/// A short-arc pose's union held to its closed form `want`: `SOUND`
/// (tiers 2 and 3′, the certificate, the closed-form volume) in one
/// shell, the pinned volume at the closed form. The boss's wall and the
/// hole's meet along the run of short arcs, as do the floor over the
/// hole and the plate's top, so its mesh holds four triangles at an
/// edge only there: within `run.1` of `run.0`, the tip, on the plane
/// `z = 0`. Every other edge is used twice, in opposite directions.
fn united_at_the_run(
    row: &str,
    r: Result<BooleanResult<f64>, BooleanError>,
    want: f64,
    run: (Point3<f64>, f64),
) {
    let b = r
        .as_ref()
        .ok()
        .and_then(BooleanResult::body)
        .unwrap_or_else(|| panic!("{row}: {:?}", r.as_ref().map(|_| "empty")));
    assert_eq!(census(&b.body)[0], 1, "{row}: shells");
    let v = topo::test_support::pinned_volume(row, &b.body, tol()).volume;
    assert!((v - want).abs() < 1e-7, "{row}: volume {v} against {want}");
    let m = mesh::tessellate(&b.body, 5e-3, tol()).unwrap_or_else(|e| panic!("{row}: {e:?}"));
    match mesh::validate::check_mesh(&m) {
        Ok(()) | Err(mesh::validate::MeshError::NonManifoldEdge { .. }) => {}
        Err(e) => panic!("{row}: mesh {e:?}"),
    }
    let mut uses = std::collections::BTreeMap::<(u32, u32), (u32, i64)>::new();
    for t in m.patches.iter().flat_map(|p| &p.triangles) {
        for k in 0..3 {
            let (a, b) = (t[k], t[(k + 1) % 3]);
            let e = uses.entry((a.min(b), a.max(b))).or_default();
            e.0 += 1;
            e.1 += if a < b { 1 } else { -1 };
        }
    }
    let on_run = |i: u32| {
        let p = m.positions[i as usize];
        (p - run.0).norm() <= run.1
    };
    let mut pinched = 0;
    for ((a, b), (count, balance)) in uses {
        assert_eq!(balance, 0, "{row}: mesh edge ({a}, {b}) winds one way");
        match count {
            2 => {}
            4 if on_run(a) && on_run(b) => pinched += 1,
            _ => panic!(
                "{row}: mesh edge {:?}–{:?} used {count} times",
                m.positions[a as usize], m.positions[b as usize]
            ),
        }
    }
    assert!(pinched > 0, "{row}: no mesh edge along the run");
    let line = outcome(r, want, tol());
    assert!(line.starts_with("OK SOUND"), "{row}: {line}");
}

/// **Short arcs at a crescent's tip**: the boss `r = 0.3` over the
/// through hole `r = 0.1` about `(0.2, 0)` of
/// [`a_boss_over_a_hole_by_pose`], tangent inside at `(0.3, 0)`, each
/// rim cut by vertices at azimuth `±s` on the boss and `±3s` on the hole
/// (or one side of the tip only, or with no vertex at the tip), and at
/// π. The cuts either side of the tip are one point within the band, so
/// the arcs between them lie on each other: the section runs along
/// them, and the long arcs leave their far ends at an angle of about
/// `2s` that the short arcs' length cannot resolve but the long arcs'
/// can. The union is one shell (`boolean::sectors` `coincident_run`).
/// Where only one side of the tip is cut, the other side is a true
/// crescent at the tip and the union refuses
/// `TangentCrescentUnsupported`; where only one rim is cut, the cut rim
/// meets the other rim in band, off that rim's wall, and the crossing
/// layer escalates that meeting at the cut rim and the wall
/// (`BoundaryMeeting`, [`Tip::Meeting`]).
///
/// Each rim cut at `±s` alone has a reflex long arc between its two
/// cuts; its profile escalates at every ε, so a reflex arc never reaches
/// the reading as a run here (`coincident_run` admits arcs of at most a
/// half-turn).
///
/// The unions by ε (default, 1e-6, 1e-12), `A ∪ B` then `B ∪ A`.
#[test]
fn short_arcs_at_a_crescents_tip() {
    use Tip::{Built, Crescent, Meeting, Pierce, Sectors, Unprofiled};
    let p = PI;
    type Row = (&'static str, [f64; 4], [f64; 4], [[Tip; 2]; 3]);
    let tip = |s: f64| [-s, 0.0, s, PI];
    let span = |s: f64| [-s, s, PI, f64::NAN];
    let rows: [Row; 11] = [
        (
            "both, s = 1e-5",
            tip(1e-5),
            tip(3e-5),
            [[Built; 2], [Unprofiled; 2], [Unprofiled; 2]],
        ),
        (
            "both, s = 1e-6",
            tip(1e-6),
            tip(3e-6),
            [[Built; 2], [Unprofiled; 2], [Sectors; 2]],
        ),
        (
            "both, s = 1e-4",
            tip(1e-4),
            tip(3e-4),
            [[Unprofiled; 2], [Built; 2], [Pierce; 2]],
        ),
        (
            "no vertex at the tip",
            span(1e-5),
            span(3e-5),
            [[Built; 2], [Unprofiled; 2], [Pierce; 2]],
        ),
        (
            "no vertex at the tip, s = 1e-4",
            span(1e-4),
            span(3e-4),
            [[Unprofiled; 2], [Built; 2], [Pierce; 2]],
        ),
        (
            "the boss's spans the tip",
            span(1e-5),
            tip(3e-5),
            [[Built; 2], [Unprofiled; 2], [Pierce; 2]],
        ),
        (
            "the hole's spans the tip",
            tip(1e-5),
            span(3e-5),
            [[Built; 2], [Unprofiled; 2], [Unprofiled; 2]],
        ),
        (
            "two cuts, each rim's long arc reflex",
            [-1e-5, 1e-5, f64::NAN, f64::NAN],
            [-3e-5, 3e-5, f64::NAN, f64::NAN],
            [[Unprofiled; 2], [Unprofiled; 2], [Unprofiled; 2]],
        ),
        (
            "one side",
            [0.0, 1e-5, p, f64::NAN],
            [0.0, 3e-5, p, f64::NAN],
            [[Crescent; 2], [Unprofiled; 2], [Unprofiled; 2]],
        ),
        (
            "the boss's only",
            tip(1e-5),
            [0.0, p, f64::NAN, f64::NAN],
            [[Meeting; 2], [Unprofiled; 2], [Unprofiled; 2]],
        ),
        (
            "the hole's only",
            [0.0, p, f64::NAN, f64::NAN],
            tip(3e-5),
            [[Meeting; 2], [Unprofiled; 2], [Pierce; 2]],
        ),
    ];
    let row_at = match tol().eps() {
        1e-6 => 1,
        1e-12 => 2,
        _ => 0,
    };
    let (vb, vh) = (0.09 * PI, 2.0 - 0.005 * PI);
    for (pose, boss_az, hole_az, by_eps) in rows {
        let [want_ab, want_ba] = by_eps[row_at];
        let cut = |az: [f64; 4]| az.into_iter().filter(|a| !a.is_nan()).collect::<Vec<_>>();
        let (boss_rim, hole_rim) = (
            arcs_at(0.0, 0.3, &cut(boss_az)),
            arcs_at(0.2, 0.1, &cut(hole_az)),
        );
        let profiles = [vec![boss_rim.clone()], vec![square(), hole_rim.clone()]];
        if profiles.into_iter().any(|loops| {
            Profile::new(SketchPlane::<f64>::xy(), loops)
                .validate(tol())
                .is_err()
        }) {
            assert_eq!(
                [want_ab, want_ba],
                [Unprofiled; 2],
                "{pose}: the profile escalates"
            );
            continue;
        }
        assert_ne!(want_ab, Unprofiled, "{pose}: the profile validates");
        let cut = boss_az
            .iter()
            .filter(|a| a.abs() < 1.0)
            .fold(0.0, |m: f64, a| m.max(a.abs()));
        let run = (Point3::new(0.3, 0.0, 0.0), 2.0 * 0.3 * cut);
        let boss = fin("the boss", extruded(vec![boss_rim], 0.0, 1.0));
        let holed = fin(
            "the holed plate",
            extruded(vec![square(), hole_rim], -0.5, 0.5),
        );
        for (op, r, want) in every_op_both_orders(&boss, &holed, (vb, vh, 0.0), tol()) {
            let row = format!("{pose}: {op}");
            let reads = match op {
                "A ∪ B" => want_ab,
                "B ∪ A" => want_ba,
                _ if [want_ab, want_ba].contains(&Built) => Built,
                _ => continue,
            };
            if reads != Built {
                assert_eq!(
                    refused_as(&r),
                    Some(reads),
                    "{row}: {:?}",
                    r.map(|_| "a body")
                );
                continue;
            }
            match op {
                "A ∩ B" | "B ∩ A" => reads_as(&row, r, want, Reads::Empty),
                "A ∖ B" | "B ∖ A" => reads_as(&row, r, want, Reads::Sound(1)),
                _ => united_at_the_run(&row, r, want, run),
            }
        }
    }
}

/// **A closed edge lying in a cylinder or a cone face**: a triangle
/// revolved about the axis straddling the wall, its vertex on the wall
/// sweeping a circle in the wall's face. The carrier ladder decides the
/// straddle's cones distinct from the wall, so the crossing layer reads
/// the circle as an edge lying on the wall's carrier, and every op holds
/// its closed form. The straddle is `π/2`; the cylinder (`r = 1` over
/// `y ∈ [−1, 1]`) is `2π` and holds `5π/24` of it, the cone frustum
/// (radius `1.5` to `0.5` over the same height) `13π/6` and holds
/// `3π/32`.
#[test]
fn a_closed_edge_in_a_cylinder_or_cone_face_holds_every_op() {
    let revolved = |what: &str, verts: &[(f64, f64)]| {
        fin(
            what,
            revolved_about_y(
                verts
                    .iter()
                    .map(|&(x, y)| (Point2::new(x, y), 0.0))
                    .collect(),
                Revolution::Full,
                tol(),
            ),
        )
    };
    let straddle = revolved("the straddle", &[(1.0, 0.0), (1.5, 0.5), (0.5, 0.5)]);
    let cylinder = revolved(
        "the cylinder",
        &[(0.0, -1.0), (1.0, -1.0), (1.0, 1.0), (0.0, 1.0)],
    );
    let cone = revolved(
        "the cone",
        &[(0.0, -1.0), (1.5, -1.0), (0.5, 1.0), (0.0, 1.0)],
    );
    for (what, wall, v_wall, v_in) in [
        ("cylinder", &cylinder, 2.0 * PI, 5.0 * PI / 24.0),
        ("cone", &cone, 13.0 * PI / 6.0, 3.0 * PI / 32.0),
    ] {
        every_op(what, wall, &straddle, (v_wall, PI / 2.0, v_in));
    }
}

/// **Equal coaxial cylinders stacked end to end**: `r = 1` over
/// `z ∈ [0, 1]` and `[1, 2]`, both vertices at azimuth 0, so the rims
/// are one closed edge of each through one site; and with the upper
/// vertex turned 2 rad, where each vertex lies inside the other's rim
/// and the loop has two sites. `∪` is `2π`, `∩` empty, and each
/// difference its minuend, `π`.
#[test]
fn coaxial_equal_cylinders_build_every_op() {
    let lo = fin(
        "the lower",
        extruded(vec![circle(0.0, 0.0, 1.0, 0.0)], 0.0, 1.0),
    );
    for az in [0.0, 2.0] {
        let hi = fin(
            "the upper",
            extruded(vec![circle(0.0, 0.0, 1.0, az)], 1.0, 1.0),
        );
        every_op(
            &format!("the stack, upper vertex at {az}"),
            &lo,
            &hi,
            (PI, PI, 0.0),
        );
    }
}

/// **A tube's rim inside one face of a ball**: the tube `r = 1` rising
/// two from where its rim lies on the ball `√2` about the origin, tilted
/// 1.2 rad about `x` so its rim crosses neither of the ball's seam
/// meridians, and turned besides. The ball keeps of the tube the cap
/// above the rim's plane. Read without a mesh: a sphere face bounded by
/// a circle off its chart's latitudes has no tessellation lane
/// (`work/tess/sphere-face-bounded-by-a-tilted-circle-has-no-tessellation-lane.md`).
#[test]
fn a_rim_inside_a_ball_face() {
    let z0 = 1.0f64;
    let ball = fin("the ball", ball_poled_z(SQRT_2, Vec3::zero(), tol()));
    let o = Point3::origin();
    for turn in [0.0, 0.4] {
        let place = Affine3::rotation_about_axis(o, Vec3::unit_z(), turn)
            * Affine3::rotation_about_axis(o, Vec3::unit_x(), 1.2);
        let tube = topo::transform_rigid(
            &extruded(vec![circle(0.0, 0.0, 1.0, 0.0)], z0, 2.0),
            &geom_core::test_support::rigid(&place, tol()),
            tol(),
        )
        .unwrap();
        let tube = fin("the tube", tube);
        let v = (
            2.0 * PI,
            ball_volume(SQRT_2),
            cap_volume(SQRT_2, SQRT_2 - z0),
        );
        for (op, r, want) in every_op_both_orders(&tube, &ball, v, tol()) {
            let line = outcome(r, want, tol());
            assert!(line.starts_with("OK SOUND"), "turned {turn}, {op}: {line}");
        }
    }
}

/// **A puck and its rounding ring unite**: the puck `r = 1`, `z ∈ [0,
/// 0.25]`, and the quarter disc of radius 0.25 revolved round it, whose
/// torus meets the puck's top along its top rim, whose inner wall is the
/// puck's and whose floor continues the puck's. Both of the puck's rims
/// are edges of the ring through the puck's vertices: the puck's
/// volume and the ring's, by Pappus.
#[test]
fn a_puck_and_its_rounding_ring_unite() {
    let (a, r) = (1.0_f64, 0.25_f64);
    let q = (FRAC_PI_2 / 4.0).tan();
    let ring = revolved_about_y(
        vec![
            (Point2::new(a, 0.0), 0.0),
            (Point2::new(a + r, 0.0), q),
            (Point2::new(a, r), 0.0),
        ],
        Revolution::Full,
        tol(),
    );
    let turn = Affine3::rotation_about_axis(Point3::origin(), Vec3::unit_x(), FRAC_PI_2);
    let mut ring =
        topo::transform_rigid(&ring, &geom_core::test_support::rigid(&turn, tol()), tol()).unwrap();
    ring.merge_coplanar_faces(tol()).unwrap();
    let ring = fin("the ring", ring);
    let puck = fin("the puck", extruded(vec![circle(0.0, 0.0, a, 0.0)], 0.0, r));
    let quarter = PI * r * r / 4.0 * TAU * (a + 4.0 * r / (3.0 * PI));
    both_unions("the rounded puck", &puck, &ring, PI * a * a * r + quarter);
}

/// A rim of radius `r` about `c` with vertices at the azimuths `az`,
/// ascending within one turn, its arcs running between them.
fn rim_about(c: (f64, f64), r: f64, az: &[f64]) -> ProfileLoop<f64> {
    RawLoop::new(az.iter().enumerate().map(|(k, &a)| {
        let next = az.get(k + 1).copied().unwrap_or(az[0] + TAU);
        (
            Point2::new(c.0 + r * a.cos(), c.1 + r * a.sin()),
            Segment::Arc(Arc2 {
                centre: Point2::new(c.0, c.1),
                radius: r,
                sweep: next - a,
            }),
        )
    }))
}

/// **Short arcs at a crescent's tip, turned and at the finest band**:
/// the pose of [`short_arcs_at_a_crescents_tip`] at `s = 3e-7`, whose
/// cuts either side of the tip lie within the band at ε 1e-12 too, so
/// the skip in `boolean::sectors` `crescent` is read there (that row's
/// table builds nothing at 1e-12), with the tip turned to azimuth `φ`.
/// Every union is one shell at the closed form, `SOUND`; with the skip
/// off, the unturned ones refuse `TangentCrescentUnsupported`. At ε 1e-6
/// the profile escalates on the short arcs.
///
/// The rims swapped, the boss `r = 0.1` inside the hole `r = 0.3`,
/// tangent there, the boss and the plate only touch, along the run, and
/// their union is two solids at the closed form, passing tier 3, whose
/// tier 3′ reads as [`Reads::Undecided`]'s. At the rims' shared vertex
/// either side of the tip the long arcs part at about `2s`, which the
/// short arcs' arm leaves in band and the long arcs' reach decides: no
/// sector pair of the boss's floor and the hole's wall meets there
/// (`boolean::sectors` `pair_search`).
#[test]
fn short_arcs_at_a_turned_tip_at_the_finest_band() {
    let s = 3e-7;
    let coarse = tol().eps() == 1e-6;
    for phi in [0.0, 0.7, 2.0] {
        for (pose, rb, rh, cb, ch, kb, kh, tip) in [
            ("inside", 0.3, 0.1, 0.0, 0.2, 1.0, 3.0, true),
            (
                "inside, no vertex at the tip",
                0.3,
                0.1,
                0.0,
                0.2,
                1.0,
                3.0,
                false,
            ),
            ("swapped", 0.1, 0.3, 0.2, 0.0, 3.0, 1.0, true),
        ] {
            let az = |k: f64| {
                let mut az = vec![phi - k * s, phi + k * s, phi + PI];
                if tip {
                    az.insert(1, phi);
                }
                az
            };
            let at = |d: f64| (d * phi.cos(), d * phi.sin());
            let boss_rim = rim_about(at(cb), rb, &az(kb));
            let hole_rim = rim_about(at(ch), rh, &az(kh));
            let profiled = [vec![boss_rim.clone()], vec![square(), hole_rim.clone()]]
                .into_iter()
                .all(|loops| {
                    Profile::new(SketchPlane::<f64>::xy(), loops)
                        .validate(tol())
                        .is_ok()
                });
            assert_eq!(profiled, !coarse, "{pose}, φ = {phi}: the profile");
            if coarse {
                continue;
            }
            let (vb, vh) = (PI * rb * rb, 2.0 - 0.5 * PI * rh * rh);
            let tip = Point3::new(at(cb).0 + rb * phi.cos(), at(cb).1 + rb * phi.sin(), 0.0);
            let run = (tip, 2.0 * rb * kb * s);
            let boss = fin("the boss", extruded(vec![boss_rim], 0.0, 1.0));
            let holed = fin(
                "the holed plate",
                extruded(vec![square(), hole_rim], -0.5, 0.5),
            );
            for (op, r, want) in every_op_both_orders(&boss, &holed, (vb, vh, 0.0), tol()) {
                let row = format!("{pose}, φ = {phi}: {op}");
                match op {
                    "A ∩ B" | "B ∩ A" => reads_as(&row, r, want, Reads::Empty),
                    "A ∖ B" | "B ∖ A" => reads_as(&row, r, want, Reads::Sound(1)),
                    _ if pose == "swapped" => {
                        let b = r
                            .as_ref()
                            .ok()
                            .and_then(BooleanResult::body)
                            .unwrap_or_else(|| panic!("{row}: {:?}", r.as_ref().map(|_| "empty")));
                        let t3 = topo::validate_geometric(&b.body, tol());
                        assert!(t3.is_ok(), "{row}: tier 3 {t3:?}");
                        reads_as(&row, r, want, Reads::Undecided);
                    }
                    _ => united_at_the_run(&row, r, want, run),
                }
            }
        }
    }
}

/// **The boss inside the hole, the rims sharing a vertex beside the
/// tip, over the cut spacings and azimuths**
/// (`work/splitdoor/a-boss-inside-a-hole-touching-along-short-arcs-breaks-a-germ-invariant.md`):
/// [`short_arcs_at_a_turned_tip_at_the_finest_band`]'s `swapped` pose
/// at `s = 1e-6` and `1e-5`, with and without a vertex at the tip. No
/// union refuses `ClassificationInvariant` at any ε; at ε 1e-9 each
/// reads as [`Reads::Undecided`]. Other ratios of the two rims' cuts
/// leave no shared vertex and escalate `Crossing(OnEdge)` on either
/// head, which this row does not read.
#[test]
fn a_boss_inside_a_hole_sharing_a_vertex_beside_the_tip() {
    let fine = tol().eps() == 1e-9;
    for s in [1e-6, 1e-5] {
        for phi in [0.0, 2.0] {
            for tip in [true, false] {
                let az = |k: f64| {
                    let mut az = vec![phi - k * s, phi + k * s, phi + PI];
                    if tip {
                        az.insert(1, phi);
                    }
                    az
                };
                let at = |d: f64| (d * phi.cos(), d * phi.sin());
                // The boss's cuts at `±3s` on `r = 0.1` and the hole's at
                // `±s` on `r = 0.3` land on one point each side.
                let boss_rim = rim_about(at(0.2), 0.1, &az(3.0));
                let hole_rim = rim_about(at(0.0), 0.3, &az(1.0));
                let profiled = [vec![boss_rim.clone()], vec![square(), hole_rim.clone()]]
                    .into_iter()
                    .all(|loops| {
                        Profile::new(SketchPlane::<f64>::xy(), loops)
                            .validate(tol())
                            .is_ok()
                    });
                assert!(profiled || !fine, "s = {s}, φ = {phi}: the profile");
                if !profiled {
                    continue;
                }
                let (vb, vh) = (PI * 0.01, 2.0 - 0.5 * PI * 0.09);
                let boss = fin("the boss", extruded(vec![boss_rim], 0.0, 1.0));
                let holed = fin("the plate", extruded(vec![square(), hole_rim], -0.5, 0.5));
                for (op, r, want) in every_op_both_orders(&boss, &holed, (vb, vh, 0.0), tol()) {
                    if !op.contains('∪') {
                        continue;
                    }
                    let row = format!("s = {s}, φ = {phi}, tip {tip}: {op}");
                    assert!(
                        !matches!(r, Err(BooleanError::ClassificationInvariant { .. })),
                        "{row}: {:?}",
                        r.map(|_| "a body")
                    );
                    if fine {
                        reads_as(&row, r, want, Reads::Undecided);
                    }
                }
            }
        }
    }
}

/// **Review 3 (PR 5993): the meeting's text does not put the edge on
/// the face's carrier when it is not.** [`short_arcs_at_a_crescents_tip`]'s
/// "the boss's only" pose at the default ε escalates
/// `CurvedPierceEscalated { CurveMeeting }` at the boss's `r = 0.3` rim
/// against the hole's `r = 0.1` wall about `(0.2, 0)`: the rim touches
/// that cylinder at one point and does not lie on it, yet the text's
/// subject reads "where an edge on a curved face's carrier meets a
/// curve of that face's boundary", and its first lever, "make the two
/// cuts of the curved face one construction", names no cut this pose has.
#[test]
fn review3_a_meeting_off_the_carrier_says_so() {
    // The pose's profile validates at the default ε only.
    if tol().eps() != 1e-9 {
        return;
    }
    let boss_rim = arcs_at(0.0, 0.3, &[-1e-5, 0.0, 1e-5, PI]);
    let hole_rim = arcs_at(0.2, 0.1, &[0.0, PI]);
    let boss = fin("the boss", extruded(vec![boss_rim], 0.0, 1.0));
    let holed = fin(
        "the holed plate",
        extruded(vec![square(), hole_rim], -0.5, 0.5),
    );
    let got = topo::union(&boss, &holed, tol());
    let Err(e @ BooleanError::CurvedPierceEscalated { .. }) = &got else {
        panic!(
            "the meeting escalates, located: {:?}",
            got.map(|_| "a body")
        );
    };
    let text = e.to_string();
    assert!(
        !text.contains("an edge on a curved face's carrier"),
        "the rim is not on the hole's wall: {text}"
    );
}
