//! **The globe lily** (*Calochortus albus*, the fairy lantern): a
//! nodding, closed globular flower hanging from a slender arching
//! stem, with lance-shaped basal leaves.
//!
//! # What the kernel can and cannot say about a plant
//!
//! A plant is a *tapering, G1-continuous, branching tube system with
//! doubly-curved membranes on the ends*. The kernel today speaks
//! **analytic solids of revolution and extrusion, joined by
//! booleans**. This stop is the honest intersection of the two, and
//! every place the intersection is empty is pinned by
//! [`wall_probes`] — a live, fail-loud record of what the kernel
//! refused, in the `curvedcut::pin_frontier` style: each probe
//! ASSERTS its refusal and panics with instructions if the refusal
//! ever retires.
//!
//! What the lily IS, therefore:
//!
//! - the **stem** is a chain of circular tube arcs — each one a
//!   PARTIAL REVOLVE of a circle profile about an axis one ring
//!   radius away, i.e. a torus segment. A turtle walks the arcs in
//!   the world xz-plane so consecutive arcs are G1 by construction
//!   (shared tangent), and the joint is a shared disk the eye does
//!   not see. They are separate BODIES: gluing them is a coincident-
//!   planar contact, which the kernel refuses (probe 1).
//! - the **flowers** are lanterns: a full revolve whose wall is a
//!   sphere zone truncated at both poles — a wide belly, a small
//!   attachment disk where the pedicel enters, and a puckered conical
//!   mouth closing to a small disk. Sphere zone + cone + two planes,
//!   all exact.
//! - the **leaves** are lanceolate crescents: two circular arcs of
//!   DIFFERENT radii spanning the same chord, extruded thin. The
//!   asymmetry between the two radii is the blade's curve.
//!
//! Proportions are chosen, not measured: a stylized lily that the
//! kernel can state exactly beats a literal one it must approximate.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::f64::consts::PI;

use geom_core::{Point2, Point3, Tolerance, Vec2, Vec3};
use profile::{
    ArcSweep, LoopBuilder, Profile, ProfileLoop, ProfileVertex, SketchPlane, ValidatedProfile,
};
use sweep::{Extrusion, Revolution, RevolveAxis, extrude, revolve};
use topo::Body;

use crate::scalar::Scalar;
use crate::{SceneBody, Stop, View};

// ---------------------------------------------------------------
// The turtle: a G1 chain of circular arcs in the world xz-plane.
// ---------------------------------------------------------------

/// A planar frame in the world xz-plane: a point and a unit tangent,
/// both with `y = 0`. The stem chain is a walk of these.
#[derive(Clone, Copy, Debug)]
struct Turtle {
    /// Position `(x, z)`.
    p: (f64, f64),
    /// Unit tangent `(x, z)`.
    t: (f64, f64),
}

/// One arc of the walk, in the form [`tube_arc`] consumes: the ring
/// centre, the unit radial from that centre to the arc's START, the
/// ring radius, and the signed turn (positive = left, i.e. the
/// counterclockwise sense of the xz-plane drawn with +x right and
/// +z up).
#[derive(Clone, Copy, Debug)]
struct ArcSpec {
    center: (f64, f64),
    radial: (f64, f64),
    ring: f64,
    turn: f64,
}

fn rot((x, z): (f64, f64), a: f64) -> (f64, f64) {
    (x * a.cos() - z * a.sin(), x * a.sin() + z * a.cos())
}

impl Turtle {
    /// Turns through `turn` radians on a ring of radius `ring`,
    /// returning the arc and the advanced turtle. Positive `turn`
    /// curves left (centre on the left of travel).
    fn arc(self, ring: f64, turn: f64) -> (ArcSpec, Self) {
        // Left normal of (tx, tz) is (-tz, tx); the centre sits one
        // ring radius along it, signed by the turn.
        let n = if turn >= 0.0 {
            (-self.t.1, self.t.0)
        } else {
            (self.t.1, -self.t.0)
        };
        let center = (self.p.0 + ring * n.0, self.p.1 + ring * n.1);
        let radial = ((self.p.0 - center.0) / ring, (self.p.1 - center.1) / ring);
        let advanced = rot(radial, turn);
        (
            ArcSpec {
                center,
                radial,
                ring,
                turn,
            },
            Self {
                p: (center.0 + ring * advanced.0, center.1 + ring * advanced.1),
                t: rot(self.t, turn),
            },
        )
    }
}

// ---------------------------------------------------------------
// Builders
// ---------------------------------------------------------------

fn v3<S: Scalar>(x: f64, y: f64, z: f64) -> Vec3<S> {
    Vec3::new(S::from_f64(x), S::from_f64(y), S::from_f64(z))
}

fn pt3<S: Scalar>(x: f64, y: f64, z: f64) -> Point3<S> {
    Point3::new(S::from_f64(x), S::from_f64(y), S::from_f64(z))
}

fn p2<S: Scalar>(x: f64, y: f64) -> Point2<S> {
    Point2::new(S::from_f64(x), S::from_f64(y))
}

fn validated<S: Scalar>(plane: SketchPlane<S>, loops: Vec<ProfileLoop<S>>) -> ValidatedProfile<S> {
    Profile::new(plane, loops)
        .validate(Tolerance::get())
        .expect("lily profile validates")
}

/// The revolve axis every lily piece uses: the sketch frame's own
/// origin, along +v. Each builder chooses the FRAME so that this one
/// axis lands where the piece needs it — the kernel's revolve takes
/// its axis in sketch coordinates, so placement is a frame choice,
/// not an argument (findings entry 6).
fn sketch_axis<S: Scalar>() -> RevolveAxis<S> {
    RevolveAxis {
        origin: p2(0.0, 0.0),
        dir: Vec2::new(S::from_f64(0.0), S::from_f64(1.0)),
    }
}

/// A circle as a two-vertex closed arc carrier (bulge 1 = semicircle).
fn circle_loop<S: Scalar>(cx: f64, cy: f64, r: f64) -> ProfileLoop<S> {
    ProfileLoop::new(vec![
        ProfileVertex {
            pos: p2(cx + r, cy),
            bulge: S::from_f64(1.0),
        },
        ProfileVertex {
            pos: p2(cx - r, cy),
            bulge: S::from_f64(1.0),
        },
    ])
}

/// One stem segment: a circular tube of radius `tube` swept along the
/// arc, i.e. a PARTIAL REVOLVE of a circle profile about an axis one
/// ring radius away — a torus segment with two disk caps.
///
/// The sketch frame is pinned at the ring centre with `u` the radial
/// to the arc's start and `v = ŷ`; the revolve is then `Partial(-turn)`
/// because a right-hand rotation about +ŷ runs CLOCKWISE in the
/// xz-plane as drawn.
fn tube_arc<S: Scalar>(spec: ArcSpec, tube: f64) -> Body<S> {
    let plane = SketchPlane::from_frame(
        pt3(spec.center.0, 0.0, spec.center.1),
        v3(spec.radial.0, 0.0, spec.radial.1),
        v3(0.0, 1.0, 0.0),
    );
    let profile = validated(plane, vec![circle_loop(spec.ring, 0.0, tube)]);
    revolve(
        &profile,
        sketch_axis(),
        Revolution::Partial(S::from_f64(-spec.turn)),
    )
    .expect("stem tube arc revolves")
    .body
}

/// A **lantern**: the closed globular flower. A full revolve whose
/// wall is a sphere ZONE of radius `globe`, truncated at `top` above
/// the centre (the attachment disk the pedicel enters through) and at
/// `mouth` below it, then closed by a conical pucker of drop
/// `lip_drop` down to a disk of radius `lip_r` — the three tepal tips
/// meeting under the lantern.
///
/// Faces: attachment plane, sphere zone, cone, mouth plane. Every one
/// exact; the profile is authored centre-first (`arc_to_center`) so
/// the zone's carrier is the sphere itself and not a fitted arc.
fn lantern<S: Scalar>(
    attach: (f64, f64),
    dir: (f64, f64),
    globe: f64,
    top: f64,
    mouth: f64,
    lip_r: f64,
    lip_drop: f64,
) -> Body<S> {
    let r_top = (globe * globe - top * top).sqrt();
    let r_mouth = (globe * globe - mouth * mouth).sqrt();
    let t_mouth = top + mouth;
    let t_end = t_mouth + lip_drop;
    // Sketch frame: origin at the attachment point, v along the
    // flower axis (into the flower), u the in-plane radial.
    let plane = SketchPlane::from_frame(
        pt3(attach.0, 0.0, attach.1),
        v3(-dir.1, 0.0, dir.0),
        v3(dir.0, 0.0, dir.1),
    );
    let lp = LoopBuilder::start(p2(0.0, 0.0))
        .line_to(p2(r_top, 0.0))
        // The belly: the sphere's own arc about the globe centre,
        // swept the long way round the equator (Ccw in sketch (s, t)).
        .arc_to_center(p2(r_mouth, t_mouth), p2(0.0, top), ArcSweep::Ccw)
        .line_to(p2(lip_r, t_end))
        .line_to(p2(0.0, t_end))
        .close();
    revolve(&validated(plane, vec![lp]), sketch_axis(), Revolution::Full)
        .expect("lantern revolves")
        .body
}

/// A **lanceolate leaf**: two circular arcs of DIFFERENT radii on the
/// same chord (`w_out` the outer sagitta, `w_in` the inner one), giving
/// a crescent blade with two sharp tips, extruded `thick` thin.
///
/// The blade runs from `base` along `dir` for `len`; `up` orients the
/// blade plane (Gram–Schmidt'd against `dir`). The kernel has no
/// tapering sweep and no non-uniform scale, so the blade's shape is
/// entirely the two radii's difference (findings entries 3, 8).
fn leaf<S: Scalar>(
    base: (f64, f64, f64),
    dir: (f64, f64, f64),
    up: (f64, f64, f64),
    len: f64,
    w_out: f64,
    w_in: f64,
    thick: f64,
) -> Body<S> {
    let nrm = |(x, y, z): (f64, f64, f64)| {
        let l = (x * x + y * y + z * z).sqrt();
        (x / l, y / l, z / l)
    };
    let d = nrm(dir);
    let dot = up.0 * d.0 + up.1 * d.1 + up.2 * d.2;
    let v = nrm((up.0 - dot * d.0, up.1 - dot * d.1, up.2 - dot * d.2));
    // n = d x v, the extrusion direction; the frame origin steps back
    // half a thickness so the blade straddles its own mid-surface.
    let n = (
        d.1 * v.2 - d.2 * v.1,
        d.2 * v.0 - d.0 * v.2,
        d.0 * v.1 - d.1 * v.0,
    );
    let o = (
        base.0 - 0.5 * thick * n.0,
        base.1 - 0.5 * thick * n.1,
        base.2 - 0.5 * thick * n.2,
    );
    let plane = SketchPlane::from_frame(pt3(o.0, o.1, o.2), v3(d.0, d.1, d.2), v3(v.0, v.1, v.2));
    let lp = LoopBuilder::start(p2(0.0, 0.0))
        .arc_to_via(p2(0.5 * len, w_out), p2(len, 0.0))
        .close_arc_via(p2(0.5 * len, w_in));
    extrude(
        &validated(plane, vec![lp]),
        Extrusion::Distance(S::from_f64(thick)),
    )
    .expect("leaf extrudes")
    .body
}

// ---------------------------------------------------------------
// The plant
// ---------------------------------------------------------------

fn deg(d: f64) -> f64 {
    d * PI / 180.0
}

/// One named piece of the plant: render colour + body.
pub struct Piece<S: Scalar> {
    /// Export/manifest stem (`lily_<part>`).
    pub name: &'static str,
    /// Base RGB.
    pub color: [f64; 3],
    /// The body.
    pub body: Body<S>,
}

const GREEN_STEM: [f64; 3] = [0.36, 0.52, 0.30];
const GREEN_LEAF: [f64; 3] = [0.44, 0.62, 0.34];
const WHITE_TEPAL: [f64; 3] = [0.95, 0.94, 0.89];

/// Builds the whole plant: two stem arcs, one branching pedicel, two
/// nodding lanterns, three basal leaves — eight bodies, every one a
/// closed analytic solid.
///
/// The stem is walked by a [`Turtle`] so the arcs are G1 at their
/// joints by construction: each arc's start tangent IS the previous
/// arc's end tangent, and the lantern's axis IS the last tangent, so
/// the flower hangs along the stem's own direction rather than along
/// a hand-chosen vector.
pub fn plant<S: Scalar>() -> Vec<Piece<S>> {
    let root = Turtle {
        p: (0.0, 0.0),
        t: (0.0, 1.0),
    };
    // The long, nearly straight rise, then the tight turn-over that
    // makes the arch. 22 degrees on a 5 m ring reads as "leaning";
    // 170 degrees on a 1.1 m ring turns the stem right over.
    let (lower, at_fork) = root.arc(5.0, deg(22.0));
    let (upper, at_flower) = at_fork.arc(1.1, deg(170.0));
    // The side pedicel leaves the fork heading up and out, then
    // curls 130 degrees over so its lantern hangs nearly plumb.
    let fork = Turtle {
        p: at_fork.p,
        t: (deg(150.0).cos(), deg(150.0).sin()),
    };
    let (pedicel, at_bud) = fork.arc(0.42, deg(130.0));

    vec![
        Piece {
            name: "lily_stem",
            color: GREEN_STEM,
            body: tube_arc(lower, 0.060),
        },
        Piece {
            name: "lily_arch",
            color: GREEN_STEM,
            body: tube_arc(upper, 0.052),
        },
        Piece {
            name: "lily_pedicel",
            color: GREEN_STEM,
            body: tube_arc(pedicel, 0.032),
        },
        Piece {
            name: "lily_lantern",
            color: WHITE_TEPAL,
            // Set back 0.08 along the stem's own tangent so the
            // pedicel tip is INSIDE the flower: two bodies sharing a
            // plane would z-fight, and gluing them is probe 1.
            body: lantern(
                (
                    at_flower.p.0 - 0.08 * at_flower.t.0,
                    at_flower.p.1 - 0.08 * at_flower.t.1,
                ),
                at_flower.t,
                0.44,
                0.40,
                0.36,
                0.09,
                0.16,
            ),
        },
        Piece {
            name: "lily_lantern2",
            color: WHITE_TEPAL,
            body: lantern(
                (
                    at_bud.p.0 - 0.06 * at_bud.t.0,
                    at_bud.p.1 - 0.06 * at_bud.t.1,
                ),
                at_bud.t,
                0.30,
                0.27,
                0.24,
                0.05,
                0.15,
            ),
        },
        Piece {
            name: "lily_leaf_a",
            color: GREEN_LEAF,
            body: leaf(
                (0.04, 0.05, 0.03),
                (-0.60, 0.66, 0.52),
                (0.0, 0.0, 1.0),
                1.45,
                0.16,
                0.035,
                0.026,
            ),
        },
        Piece {
            name: "lily_leaf_b",
            color: GREEN_LEAF,
            body: leaf(
                (-0.03, -0.06, 0.06),
                (-0.68, -0.55, 0.44),
                (0.0, 0.0, 1.0),
                1.25,
                0.14,
                0.030,
                0.024,
            ),
        },
        Piece {
            name: "lily_leaf_c",
            color: GREEN_LEAF,
            body: leaf(
                (0.02, 0.01, 0.02),
                (0.62, 0.10, 0.78),
                (0.0, 0.0, 1.0),
                0.95,
                0.115,
                0.025,
                0.022,
            ),
        },
    ]
}

/// The tour stop.
pub fn stops() -> Vec<Stop> {
    let pieces = plant::<f64>();
    let note = format!(
        "{} bodies, each a closed analytic solid: 3 torus-segment stem \
         tubes (partial revolves of a circle about a distant axis), 2 \
         sphere-zone lanterns with conical mouths, 3 extruded crescent \
         leaves. Nothing is approximated — the walls are torus, sphere, \
         cone and plane exactly. Nothing is JOINED either: see the \
         wall probes.",
        pieces.len()
    );
    vec![Stop {
        name: "lily",
        caption: "globe lily (Calochortus albus)".to_string(),
        montage: true,
        story: "a nodding globe lily — arching stem, two closed globular \
                lanterns, lanceolate basal leaves; torus/sphere/cone/plane \
                only, every surface exact",
        ops: "Turtle-walked G1 arc chain -> revolve(Partial) tubes; \
              revolve(Full) sphere-zone lanterns; extrude(two-arc crescent) leaves",
        delta: 5e-3,
        note: Some(note),
        view: View {
            elev: 12.0,
            azim: -78.0,
            up: 'z',
        },
        bodies: pieces
            .into_iter()
            .map(|p| SceneBody::plain(p.name, p.color, p.body))
            .collect(),
    }]
}
