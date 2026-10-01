//! MSOLVE-12 acceptance — **a pair the coset table separates solves or
//! refuses in its own words** (the MSOLVE-12 spec's A1–A2 and review
//! claims C1–C2).
//!
//! The translation stage divides only by the number the table decided
//! away from zero to reach it: the sine of a non-parallel verdict, or
//! the cosine of a non-perpendicular one. So a pair the table
//! separates at the very edge of its band has a finite candidate,
//! which membership then decides on a measured margin; and a pair whose
//! meeting point is past the format refuses under that cause, not
//! under a membership predicate that measured nothing.
//!
//! `c1_every_separated_pair_measures_what_it_refuses` walks every
//! table entry that turns on such a decision, at the ulps around the
//! edge, both orders. `c2_the_pose_is_the_independent_intersection`
//! checks the solved pose against closed forms the stage does not
//! use. The `out_of_range` rows reach the one cause left, at the coset
//! and through the doors. Every row runs at `Tol::witness()`.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use std::collections::BTreeMap;

use crate::fixture;

use editor_core::mate::coset::{Arm, Coset, FoldStop, Subgroup, intersect};
use editor_core::{
    Alignment, AxisSense, CapEnd, ContactClass, DocEdit, DocumentId, LeverRefusal, MateFault,
    MateFrame, MatePrimitive, Node, NodeErrorClass, ProfileDoc, RecipeNodeId,
};
use fixture::resolver::{PartStore, in_part, with_resolver};
use fixture::{FIXTURE_MATE_AXIS, insert, len, on_frame, solve, square, step};
use geom_core::linalg::{Affine3, Mat3, Point3, UnitVec3, Vec3};
use geom_core::predicate::Band;
use geom_core::{RANGE_RECOURSE, Tol};

fn band() -> Band {
    Band::linear(Tol::witness()).unwrap()
}

fn unit(v: Vec3<f64>) -> UnitVec3<f64> {
    UnitVec3::new(v, FIXTURE_MATE_AXIS, band()).unwrap()
}

fn lever(arm: f64) -> Arm {
    Arm::of(0.0, arm).expect("a finite arm the format can decide over")
}

fn coset(subgroup: Subgroup, translation: Vec3<f64>) -> Coset {
    Coset {
        subgroup,
        representative: Affine3::from_parts(Mat3::identity(), translation),
    }
}

// ---- C1: every separated pair, at the edge ----

/// The table entries whose verdict turns on a sine or a cosine, by the
/// decision that separates the pair and the subgroups it pairs.
#[derive(Clone, Copy, Debug)]
enum Entry {
    PlanePlane,
    PlaneLinePerpendicular,
    PlanePrismaticPerpendicular,
    LineLine,
    LinePrismatic,
    PrismaticPrismatic,
    PlaneLineParallel,
    PlaneRevoluteParallel,
    LineRevoluteParallel,
    RevoluteRevoluteParallel,
}

const ENTRIES: [Entry; 10] = [
    Entry::PlanePlane,
    Entry::PlaneLinePerpendicular,
    Entry::PlanePrismaticPerpendicular,
    Entry::LineLine,
    Entry::LinePrismatic,
    Entry::PrismaticPrismatic,
    Entry::PlaneLineParallel,
    Entry::PlaneRevoluteParallel,
    Entry::LineRevoluteParallel,
    Entry::RevoluteRevoluteParallel,
];

impl Entry {
    /// Whether the edge is the perpendicular decision's (the cosine)
    /// rather than the parallel one's (the levered sine).
    fn perpendicular(self) -> bool {
        matches!(
            self,
            Self::PlanePrismaticPerpendicular | Self::PlaneLinePerpendicular
        )
    }

    /// The direction `s` past the edge from `n`: `n` tilted by `s`
    /// along `shape`, or an in-plane direction of `n` lifted by `s`.
    fn second(self, n: Vec3<f64>, shape: (f64, f64), s: f64) -> Option<UnitVec3<f64>> {
        let (b1, b2) = n.orthonormal_basis();
        let v = if self.perpendicular() {
            b1 * shape.0 + b2 * shape.1 + n * s
        } else {
            n + b1 * (shape.0 * s) + b2 * (shape.1 * s)
        };
        UnitVec3::new(v, FIXTURE_MATE_AXIS, band()).ok()
    }

    /// The margin the table decides for `n` and `m` at `arm`.
    fn margin(self, n: Vec3<f64>, m: Vec3<f64>, arm: f64) -> f64 {
        if self.perpendicular() {
            (m.dot(n) * arm).abs()
        } else {
            (n.cross(m) * arm).norm()
        }
    }

    fn pair(self, a: UnitVec3<f64>, b: UnitVec3<f64>) -> (Subgroup, Subgroup) {
        use Subgroup::{Cylindrical, Planar, Prismatic, Revolute};
        let point = Point3::new(0.25, -0.5, 0.125);
        let line = |direction| Cylindrical { point, direction };
        let pin = |direction| Revolute { point, direction };
        match self {
            Self::PlanePlane => (Planar { normal: a }, Planar { normal: b }),
            Self::PlaneLinePerpendicular | Self::PlaneLineParallel => {
                (Planar { normal: a }, line(b))
            }
            Self::PlanePrismaticPerpendicular => (Planar { normal: a }, Prismatic { direction: b }),
            Self::LineLine => (line(a), line(b)),
            Self::LinePrismatic => (line(a), Prismatic { direction: b }),
            Self::PrismaticPrismatic => (Prismatic { direction: a }, Prismatic { direction: b }),
            Self::PlaneRevoluteParallel => (Planar { normal: a }, pin(b)),
            Self::LineRevoluteParallel => (line(a), pin(b)),
            Self::RevoluteRevoluteParallel => (pin(a), pin(b)),
        }
    }
}

/// The `s` at which `entry`'s margin crosses the band's escalate
/// threshold, by bisection, so a walk of the ulps around it straddles
/// the edge.
fn edge(entry: Entry, n: Vec3<f64>, shape: (f64, f64), arm: f64, band: Band) -> f64 {
    let escalate = band.escalate();
    let (mut lo, mut hi) = (escalate / arm * 0.25, escalate / arm * 4.0);
    for _ in 0..200 {
        let mid = 0.5 * (lo + hi);
        if mid <= lo || mid >= hi {
            break;
        }
        match entry.second(n, shape, mid) {
            Some(m) if entry.margin(n, m.get(), arm) >= escalate => hi = mid,
            _ => lo = mid,
        }
    }
    hi
}

/// **No pair the table separates refuses under a margin nobody
/// measured.** Every entry whose verdict turns on a sine or a cosine,
/// over three normals, four arms, three tilt shapes and the 80 ulps
/// around the band's escalate edge, held-then-added and the reverse:
/// the representatives coincide (the pair meets at the origin) or the
/// added one is offset by a few centimetres. An escalation carries a
/// margin, and with coincident representatives it is the table's own
/// split; a contradiction quotes a finite measurement and never falls
/// on a pair that meets at the origin; every pair the table separated
/// with coincident representatives SOLVES, to a finite pose. The arms
/// reach a megametre, so the meeting point of an offset pair can lie
/// past the session's range, where membership decides on the rounding
/// of that pose
/// (`work/msolve/a-far-meeting-point-fails-membership-by-its-own-rounding.md`).
#[test]
fn c1_every_separated_pair_measures_what_it_refuses() {
    let band = band();
    let table_splits = ["mate_axes_parallel", "mate_axis_normal_perpendicular"];
    let mut census: BTreeMap<String, usize> = BTreeMap::new();
    for entry in ENTRIES {
        for n in [
            Vec3::new(0.0, 0.0, 1.0),
            Vec3::new(0.3, 0.2, 1.0),
            Vec3::new(1.0, 1.0, 1.0),
        ] {
            let n = unit(n).get();
            for arm in [1.0_f64, 3.7, 123.456, 1e6] {
                for shape in [(1.0, 0.0), (0.6, 0.8), (0.3, -0.9)] {
                    let base = edge(entry, n, shape, arm, band).to_bits() as i64;
                    for k in -20_i64..60 {
                        let s = f64::from_bits((base + k) as u64);
                        let Some(m) = entry.second(n, shape, s) else {
                            continue;
                        };
                        let (g1, g2) = entry.pair(unit(n), m);
                        for (held, added) in [(g1, g2), (g2, g1)] {
                            for offset in [Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.01, 0.02, 0.03)] {
                                let at = format!(
                                    "{entry:?} n={n:?} arm={arm} shape={shape:?} s={s:e} \
                                     held={} offset={offset:?}",
                                    held.name()
                                );
                                let coincident = offset.norm() == 0.0;
                                let held = coset(held, Vec3::new(0.0, 0.0, 0.0));
                                let added = coset(added, offset);
                                let word = match intersect(held, added, band, lever(arm)) {
                                    Ok(c) => {
                                        let t = c.representative.translation;
                                        assert!(
                                            t.x.is_finite() && t.y.is_finite() && t.z.is_finite(),
                                            "a solved pose is finite: {at}: {t:?}"
                                        );
                                        "solves".to_string()
                                    }
                                    Err(FoldStop::Indeterminate(d)) => {
                                        assert!(
                                            !d.margin.is_invalid(),
                                            "an escalation carries the margin it measured: \
                                             {at}: {d:?}"
                                        );
                                        let name = d.predicate.expect("a named decision");
                                        assert!(
                                            !coincident || table_splits.contains(&name),
                                            "coincident representatives escalate only at the \
                                             table's own split: {at}: {d:?}"
                                        );
                                        format!("escalates {name}")
                                    }
                                    Err(FoldStop::Clash { predicate, clash }) => {
                                        assert!(
                                            !coincident,
                                            "a pair meeting at the origin is no contradiction: \
                                             {at}: {predicate} {clash:?}"
                                        );
                                        assert!(
                                            clash.deviation().is_some_and(f64::is_finite),
                                            "a clash quotes a finite measurement: {at}: {clash:?}"
                                        );
                                        format!("clash {predicate}")
                                    }
                                    Err(
                                        stop @ (FoldStop::OutOfRange | FoldStop::Unleverable(_)),
                                    ) => {
                                        panic!(
                                            "every arm here decides angles and every meeting \
                                             point is measurable: {at}: {stop:?}"
                                        )
                                    }
                                };
                                *census.entry(word).or_default() += 1;
                            }
                        }
                    }
                }
            }
        }
    }
    let solved = census.get("solves").copied().unwrap_or(0);
    let split: usize = table_splits
        .iter()
        .map(|name| {
            census
                .get(&format!("escalates {name}"))
                .copied()
                .unwrap_or(0)
        })
        .sum();
    assert!(
        solved > 0 && split > 0,
        "the walk straddles every edge: {census:?}"
    );
}

// ---- C2: the pose is the intersection, by an independent closed form ----

fn close(got: Vec3<f64>, want: Vec3<f64>, rel: f64) -> bool {
    (got - want).norm() <= rel * want.norm().max(1.0)
}

/// **Where it solves, the pose is the true intersection; where it
/// refuses two skew lines, the clash is their gap.** Every pair is
/// BUILT around a known point `X` rather than solved for one, so no
/// reference here shares a spelling with the stage: a line `d` in the
/// held plane and an in-plane direction `k` across it frame a second
/// plane `m = n·cos s + k·sin s`, which meets the first in the line
/// through `X` along `d`; a line along `u`, lifted `s` out of the
/// plane, crosses it at `X`; two lines through `X` at an angle `s` to
/// each other cross there, and lifted apart by `h` along `n` miss each
/// other by exactly `h`. Each pair runs at tilts from 0.7 down to the
/// band's edge, both orders where both are meaningful, prismatic and
/// cylindrical lines alike, to the agreement two spellings can offer:
/// the rounding over the sine.
#[test]
fn c2_the_pose_is_the_independent_intersection() {
    let band = band();
    let arm = 3.7;
    let x = Vec3::new(0.3, -0.2, 0.5);
    let edge_tilt = band.escalate() / arm * 1.5;
    let at = |direction: UnitVec3<f64>, cylinder: bool| {
        if cylinder {
            Subgroup::Cylindrical {
                point: Point3::new(-0.4, 0.1, 0.2),
                direction,
            }
        } else {
            Subgroup::Prismatic { direction }
        }
    };
    let solved = |held: Coset, added: Coset| {
        intersect(held, added, band, lever(arm))
            .unwrap_or_else(|stop| panic!("the pair meets: {stop:?}"))
            .representative
            .translation
    };
    let mut checked = 0_usize;
    for (n, tilts) in [
        (Vec3::new(0.0, 0.0, 1.0), vec![0.7, 0.1, 1e-3, edge_tilt]),
        (Vec3::new(0.3, 0.2, 1.0), vec![0.7, 0.1, 1e-3]),
    ] {
        let normal = unit(n);
        let n = normal.get();
        let (d, k) = n.orthonormal_basis();
        let line = unit(d);
        let plane = |r| coset(Subgroup::Planar { normal }, r);
        let r1 = x + k * 0.4 + d * 0.25;

        // A line lying in the plane, and a pin standing on it: the
        // added anchor, which is on the plane, is the pose.
        let on_plane = x + d * 0.6 - k * 0.3;
        for added in [
            coset(at(line, false), on_plane),
            coset(at(line, true), on_plane),
            coset(
                Subgroup::Revolute {
                    point: Point3::new(0.1, 0.2, 0.3),
                    direction: normal,
                },
                on_plane,
            ),
        ] {
            let got = solved(plane(r1), added);
            assert!(close(got, on_plane, 1e-12), "{got:?} vs {on_plane:?}");
            checked += 1;
        }

        for s in tilts {
            let rel = (64.0 * f64::EPSILON / s).max(1e-12);
            let (sin, cos) = s.sin_cos();

            // Two planes: the held anchor's foot on their line.
            let m = unit(n * cos + k * sin);
            let r2 = x + m.get().cross(d) * 0.7 - d * 0.3;
            let got = solved(plane(r1), coset(Subgroup::Planar { normal: m }, r2));
            let want = x + d * 0.25;
            assert!(
                close(got, want, rel),
                "planes at s={s:e}: {got:?} vs {want:?}"
            );
            checked += 1;

            // A line crossing the plane at X, prismatic or cylindrical,
            // either side held.
            let u = unit(d * cos + n * sin);
            for cylinder in [false, true] {
                let crossing = coset(at(u, cylinder), x + u.get() * 0.9);
                for (held, added) in [(plane(r1), crossing), (crossing, plane(r1))] {
                    let got = solved(held, added);
                    assert!(
                        close(got, x, rel),
                        "line (cylinder: {cylinder}) and plane at s={s:e}: {got:?} vs {x:?}"
                    );
                    checked += 1;
                }
            }

            // Two lines through X at the angle s: they cross at X. Lifted
            // apart by h along n they are skew, the held line's nearest
            // point is still X, and the cylinder's clash is their gap.
            let v = unit(d * cos + k * sin);
            let h = 0.05;
            for cylinder in [false, true] {
                let held = coset(at(line, cylinder), x + d * 0.3);
                let crossing = coset(at(v, cylinder), x - v.get() * 0.6);
                let got = solved(held, crossing);
                assert!(
                    close(got, x, rel),
                    "lines (cylinder: {cylinder}) at s={s:e}: {got:?} vs {x:?}"
                );
                checked += 1;
            }
            let skew = coset(at(v, true), x + n * h - v.get() * 0.6);
            match intersect(coset(at(line, true), x + d * 0.3), skew, band, lever(arm)) {
                Err(FoldStop::Clash {
                    predicate: "mate_member_point_on_axis",
                    clash,
                }) => {
                    let gap = clash.deviation().expect("a measured gap");
                    assert!(
                        (gap - h).abs() <= rel,
                        "skew lines at s={s:e}: the clash {gap} is the gap {h}"
                    );
                }
                other => panic!("skew lines at s={s:e} contradict: {other:?}"),
            }
            checked += 1;
        }
    }
    assert_eq!(checked, 2 * 3 + 7 * 8, "every shape at every tilt");
}

// ---- The lever's own floor ----

/// **No angle is decided over a lever at or below the band's zero
/// threshold.** A plane and a cylinder whose axis lies in it, both
/// through the origin, meet in the slide along the axis at any arm the
/// angles are decidable at. At an arm of zero, a tenth, a half and the
/// whole of the zero threshold every levered margin reads zero, so
/// the axis would be called parallel to the normal and the table would
/// answer a pin; the door refuses first, in the lever's words, carrying
/// the arm and the threshold — both orders, and before any division.
#[test]
fn a_lever_inside_the_zero_band_decides_no_angle() {
    let band = band();
    let plane = coset(
        Subgroup::Planar {
            normal: unit(Vec3::new(0.0, 0.0, 1.0)),
        },
        Vec3::new(0.0, 0.0, 0.0),
    );
    let axis = coset(
        Subgroup::Cylindrical {
            point: Point3::new(0.0, 0.0, 0.0),
            direction: unit(Vec3::new(1.0, 0.0, 0.0)),
        },
        Vec3::new(0.0, 0.0, 0.0),
    );
    let zero = band.zero();
    for arm in [0.0, 0.1 * zero, 0.5 * zero, zero] {
        for (held, added) in [(plane, axis), (axis, plane)] {
            match intersect(held, added, band, lever(arm)) {
                Err(FoldStop::Unleverable(LeverRefusal::BelowZeroBand { arm: a, zero: z })) => {
                    assert_eq!((a.to_bits(), z.to_bits()), (arm.to_bits(), zero.to_bits()));
                }
                other => panic!("arm {arm:e}: {other:?}"),
            }
        }
    }
    for (held, added) in [(plane, axis), (axis, plane)] {
        let met = intersect(held, added, band, lever(2.0 * band.escalate()))
            .expect("past the escalate threshold the pair meets");
        assert_eq!(met.subgroup.name(), "prismatic", "the slide along the axis");
    }
    let text = LeverRefusal::BelowZeroBand { arm: 0.0, zero }.to_string();
    assert!(text.contains("finer than the parts"), "{text}");
}

// ---- The one cause left: a meeting point past the format ----

/// **A meeting point the format cannot hold refuses as such.** Two
/// planes the table separates by a levered sine of a hundred metres,
/// offset by a datum-scale distance: they meet some 1e302 m out,
/// further than a length can be measured to. The refusal is the
/// range's, at the coset. So is a length membership measures that the
/// format cannot hold: two determined poses whose candidate holds a
/// finite length but sit further apart than any length can say.
#[test]
fn out_of_range_at_the_coset() {
    let far = |x: f64| coset(Subgroup::Trivial, Vec3::new(x, 0.0, 0.0));
    let (held, added) = (far(1.2e154), far(-1.2e154));
    assert!(held.representative.translation.norm().is_finite());
    assert!(matches!(
        intersect(held, added, band(), lever(1.0)),
        Err(FoldStop::OutOfRange)
    ));
    let held = coset(
        Subgroup::Planar {
            normal: unit(Vec3::new(0.0, 0.0, 1.0)),
        },
        Vec3::new(0.0, 0.0, 1e152),
    );
    let added = coset(
        Subgroup::Planar {
            normal: unit(Vec3::new(1e-150, 0.0, 1.0)),
        },
        Vec3::new(0.0, 0.0, 0.0),
    );
    assert!(matches!(
        intersect(held, added, band(), lever(1e152)),
        Err(FoldStop::OutOfRange)
    ));
}

fn part(label: &str) -> (ProfileDoc, RecipeNodeId) {
    let doc = ProfileDoc::empty(DocumentId::derive(label), Tol::witness());
    let (doc, profile) = on_frame(
        doc,
        [0.0, 0.0, 0.0],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![square(0.0, 0.0, 0.5)],
    );
    insert(
        doc,
        Node::Extrude {
            profile,
            distance: len(1.0),
        },
    )
}

/// A planar rest between the two instances' start caps, both sides
/// framed along `axis`, `a`'s at `origin`.
fn rest(
    body: RecipeNodeId,
    ids: [RecipeNodeId; 2],
    origin: [f64; 3],
    axis: [f64; 3],
) -> Node<editor_core::ProfileProgram> {
    let side = |o| MateFrame::authored(o, axis, [0.0, 1e152, 0.0]);
    Node::Mate {
        a: fixture::head(in_part(ids[0], body, CapEnd::Start)),
        b: fixture::head(in_part(ids[1], body, CapEnd::Start)),
        class: ContactClass::Rest,
        alignment: Alignment {
            a: side(origin),
            b: side([0.0, 0.0, 0.0]),
            primitive: MatePrimitive::PlanarRest { offset: 0.0 },
            sense: AxisSense::Aligned,
            clocking: None,
        },
    }
}

/// **Through the doors.** Two planar rests between one pair of
/// instances: the first through a frame 1e152 m up its own normal, the
/// second tilted 1e-150 from it, each frame's vectors authored at its
/// origin's scale so the frame ladder reads them. The insert door
/// admits both (every datum is finite, and so is the lever's square),
/// the solve separates the planes, and the pair refuses
/// `PoseOutOfRange`, naming both mates — its own arm, its own class,
/// the range's recourse.
#[test]
fn out_of_range_through_the_doors() {
    let mut store = PartStore::new();
    let (doc_ref, body) = store.insert_part(part("msolve12-range-part"), Tol::witness());
    let doc = ProfileDoc::empty(DocumentId::derive("msolve12-range"), Tol::witness());
    let (doc, i0) = insert(doc, Node::instantiate_part(doc_ref));
    let (doc, i1) = insert(doc, Node::instantiate_part(doc_ref));
    let o = with_resolver(store);
    let add = |doc, node| {
        let (doc, id) = step(doc, DocEdit::InsertNode { node });
        (doc, id.expect("the insert minted an id"))
    };
    let (doc, first) = add(
        doc,
        rest(body, [i0, i1], [0.0, 0.0, 1e152], [0.0, 0.0, 1e152]),
    );
    let (doc, second) = add(
        doc,
        rest(body, [i0, i1], [0.0, 0.0, 0.0], [100.0, 0.0, 1e152]),
    );
    let poses = solve(&doc, &o, Tol::witness());
    let fault = poses.fault(second).expect("the second mate refuses");
    assert_eq!(
        fault,
        &MateFault::PoseOutOfRange {
            held: first,
            added: second,
        }
    );
    assert_eq!(
        NodeErrorClass::of_mate(fault),
        NodeErrorClass::MatePoseOutOfRange
    );
    let text = fault.to_string();
    assert!(text.contains(RANGE_RECOURSE), "{text}");
    assert!(
        text.contains(&format!("mates {first} and {second} meet")),
        "{text}"
    );
}
