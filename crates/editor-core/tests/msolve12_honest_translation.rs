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
    Alignment, AxisSense, CapEnd, ContactClass, DocEdit, DocumentId, MateFault, MateFrame,
    MatePrimitive, Node, NodeErrorClass, ProfileDoc, RecipeNodeId,
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
                                    Err(FoldStop::OutOfRange) => {
                                        panic!("no meeting point here is past the format: {at}")
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

/// The solution of `rows · x = rhs` by Cramer's rule — a solve the
/// translation stage does not use.
fn cramer(rows: [Vec3<f64>; 3], rhs: [f64; 3]) -> Vec3<f64> {
    let [r0, r1, r2] = rows;
    let det = r0.dot(r1.cross(r2));
    let col = |i: usize| {
        let pick = |r: Vec3<f64>, b: f64| match i {
            0 => Vec3::new(b, r.y, r.z),
            1 => Vec3::new(r.x, b, r.z),
            _ => Vec3::new(r.x, r.y, b),
        };
        let (a0, a1, a2) = (pick(r0, rhs[0]), pick(r1, rhs[1]), pick(r2, rhs[2]));
        a0.dot(a1.cross(a2)) / det
    };
    Vec3::new(col(0), col(1), col(2))
}

fn close(got: Vec3<f64>, want: Vec3<f64>, rel: f64) -> bool {
    (got - want).norm() <= rel * want.norm().max(1.0)
}

/// **Where it solves, the pose is the true intersection.** Each shape
/// the translation stage solves in closed form is checked against a
/// formula it does not use, at tilts from 0.7 down to the band's edge
/// (where the planes meet some 1e8 m out, and the agreement
/// two spellings can offer is the rounding over the sine). The planes'
/// line is found through its nearest point to the origin,
/// `(h₁ n₂ − h₂ n₁) × w / |w|²` with `w = n₁ × n₂`, and the held
/// representative projected onto it; a plane and a line meet by
/// Cramer's rule over the plane and two normals of the line, either
/// side held; two lines built to cross at a known point meet there.
/// A line lying in the plane leaves the slide along itself free, and
/// the pose is the foot of the held point on it; a pin on the plane is
/// where it stands.
#[test]
fn c2_the_pose_is_the_independent_intersection() {
    let band = band();
    let r1 = Vec3::new(0.4, -0.3, 0.2);
    let r2 = Vec3::new(-0.1, 0.7, 0.05);
    let arm = 3.7;
    let edge_tilt = band.escalate() / arm * 1.5;
    let mut checked = 0_usize;
    for (n, tilts) in [
        (Vec3::new(0.0, 0.0, 1.0), vec![0.7, 0.1, 1e-3, edge_tilt]),
        (Vec3::new(0.3, 0.2, 1.0), vec![0.7, 0.1, 1e-3]),
    ] {
        let normal = unit(n);
        let n = normal.get();
        let (b1, b2) = n.orthonormal_basis();
        let on_plane = r2 - n * n.dot(r2 - r1);
        let along = unit(b1 * 0.6 + b2 * 0.8);
        let foot = on_plane + along.get() * along.get().dot(r1 - on_plane);
        let slide = intersect(
            coset(Subgroup::Planar { normal }, r1),
            coset(Subgroup::Prismatic { direction: along }, on_plane),
            band,
            lever(arm),
        )
        .expect("a line in the plane meets it");
        assert_eq!(slide.subgroup.name(), "prismatic");
        let got = slide.representative.translation;
        assert!(
            close(got, foot, 1e-12),
            "line in the plane: {got:?} vs {foot:?}"
        );
        let got = intersect(
            coset(Subgroup::Planar { normal }, r1),
            coset(
                Subgroup::Revolute {
                    point: Point3::new(0.1, 0.2, 0.3),
                    direction: normal,
                },
                on_plane,
            ),
            band,
            lever(arm),
        )
        .expect("a pin on the plane stands on it")
        .representative
        .translation;
        assert!(close(got, on_plane, 1e-12), "pin: {got:?} vs {on_plane:?}");
        checked += 2;
        for s in tilts {
            let at_edge = s < 1e-6;
            let rel = if at_edge { 1e-6 } else { 1e-12 };
            let tilted = unit(n + b1 * (0.6 * s) + b2 * (0.8 * s));
            let lifted = unit(b1 * 0.6 + b2 * 0.8 + n * s);

            let m = tilted.get();
            let c = intersect(
                coset(Subgroup::Planar { normal }, r1),
                coset(Subgroup::Planar { normal: tilted }, r2),
                band,
                lever(arm),
            )
            .expect("two planes the table separates meet");
            let w = n.cross(m);
            let (h1, h2) = (n.dot(r1), m.dot(r2));
            let p = (m * h1 - n * h2).cross(w) / w.norm_squared();
            let d = w / w.norm();
            let want = p + d * d.dot(r1 - p);
            let got = c.representative.translation;
            assert!(
                close(got, want, rel),
                "planes at s={s:e}: {got:?} vs {want:?}"
            );
            checked += 1;

            let u = lifted.get();
            let (e1, e2) = u.orthonormal_basis();
            let want = cramer([n, e1, e2], [n.dot(r1), e1.dot(r2), e2.dot(r2)]);
            let plane = coset(Subgroup::Planar { normal }, r1);
            let line = coset(Subgroup::Prismatic { direction: lifted }, r2);
            for (held, added, order) in [(plane, line, "plane held"), (line, plane, "line held")] {
                let got = intersect(held, added, band, lever(arm))
                    .expect("a line crossing a plane meets it")
                    .representative
                    .translation;
                assert!(
                    close(got, want, rel),
                    "{order} at s={s:e}: {got:?} vs {want:?}"
                );
                checked += 1;
            }

            if !at_edge {
                let q2 = r1 + n * 0.3 + m * 0.2;
                let want = r1 + n * 0.3;
                let got = intersect(
                    coset(Subgroup::Prismatic { direction: normal }, r1),
                    coset(Subgroup::Prismatic { direction: tilted }, q2),
                    band,
                    lever(arm),
                )
                .expect("two coplanar lines meet")
                .representative
                .translation;
                assert!(
                    close(got, want, rel),
                    "lines at s={s:e}: {got:?} vs {want:?}"
                );
                checked += 1;
            }
        }
    }
    assert_eq!(checked, 2 * 2 + 6 * 4 + 3, "every shape at every tilt");
}

// ---- The one cause left: a meeting point past the format ----

/// **A meeting point the format cannot hold refuses as such.** Two
/// planes the table separates by a levered sine of a hundred metres,
/// offset by a datum-scale distance: they meet some 1e302 m out, and
/// the translation's length overflows the format. The refusal is the
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
/// the solve separates the planes, and the second mate refuses
/// `PoseOutOfRange` — its own arm, its own class, the range's recourse.
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
    let (doc, _) = add(
        doc,
        rest(body, [i0, i1], [0.0, 0.0, 1e152], [0.0, 0.0, 1e152]),
    );
    let (doc, second) = add(
        doc,
        rest(body, [i0, i1], [0.0, 0.0, 0.0], [100.0, 0.0, 1e152]),
    );
    let poses = solve(&doc, &o, Tol::witness());
    let fault = poses.fault(second).expect("the second mate refuses");
    assert_eq!(fault, &MateFault::PoseOutOfRange { mate: second });
    assert_eq!(
        NodeErrorClass::of_mate(fault),
        NodeErrorClass::MatePoseOutOfRange
    );
    let text = fault.to_string();
    assert!(text.contains(RANGE_RECOURSE), "{text}");
    assert!(text.contains(&second.to_string()), "{text}");
}
