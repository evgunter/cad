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
//!
//! **What "exact" claims, precisely** (review NOTE-1). It claims the
//! surface KIND: a stem wall is a `Surface::Torus`, not a spline fit of
//! one, and it exports as `TOROIDAL_SURFACE`. It does NOT claim that
//! every stored PARAMETER is the authored decimal — `revolve`
//! reconstructs a tube radius from the profile's bulge arcs rather than
//! carrying the authored number through, so `lily_stem`'s stored
//! `minor_radius` is 0.05999999999999961, some 3.9e-16 (56 ulps) below
//! the authored 0.060. That is float reconstruction of a derived
//! quantity, not approximation of a shape, and it is not chased here.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::f64::consts::PI;

use pncad::geom_brep::SurfaceKind;
use pncad::geom_core::{Affine3, Mat3, Point3, Vec2, Vec3};
use pncad::profile::{ArcSweep, LoopBuilder, ProfileLoop, ProfileVertex, SketchPlane};
use pncad::sweep::fillet::FilletError;
use pncad::sweep::{ExtrudeError, Extrusion, Revolution, RevolveAxis, extrude, revolve};
use pncad::topo::{Body, BooleanError, BooleanOp, Operand, TransformError};

use crate::scalar::Scalar;
use crate::{SceneBody, Stop, View};
use pncad::authoring::{p2, validated};

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
/// Stays raw under LIB-U2 PR-2: a closed carrier split at conventional
/// points (PQ4 mid-carrier seam, same-carrier joints) is refused by
/// the PATHS algebra by design.
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
    let profile =
        validated(plane, vec![circle_loop(spec.ring, 0.0, tube)]).expect("lily profile validates");
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
/// Stays raw under LIB-U2 PR-2: centre-first arcs have no PATHS
/// binding mode ({endpoints+bulge, tangent+endpoint, fillet} only).
fn lantern<S: Scalar>(
    attach: (f64, f64),
    dir: (f64, f64),
    globe: f64,
    top: f64,
    mouth: f64,
    lip_r: f64,
    lip_drop: f64,
) -> Body<S> {
    let r_top = (globe.powi(2) - top.powi(2)).sqrt();
    let r_mouth = (globe.powi(2) - mouth.powi(2)).sqrt();
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
    revolve(
        &validated(plane, vec![lp]).expect("lily profile validates"),
        sketch_axis(),
        Revolution::Full,
    )
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
/// Stays raw under LIB-U2 PR-2: via-point arcs (`arc_to_via` /
/// `close_arc_via`) have no PATHS binding mode ({endpoints+bulge,
/// tangent+endpoint, fillet} only).
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
        let l = (x.powi(2) + y.powi(2) + z.powi(2)).sqrt();
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
        &validated(plane, vec![lp]).expect("lily profile validates"),
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
         leaves. No wall is approximated — the surface KINDS are torus, \
         sphere, cone and plane exactly (stored parameters are float \
         reconstructions; see the module docs). Nothing is JOINED \
         either: see the wall probes.",
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
        // One chord budget for the whole scene is a poor fit here: at
        // 2e-3 the 0.44 m lantern is smooth and a 0.06 m stem tube
        // costs ~2e5 triangles, because the torus lane spends its
        // budget on the 5 m RING and not on the tube (findings 9).
        delta: 2e-3,
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

// ---------------------------------------------------------------
// The wall probes
// ---------------------------------------------------------------

/// A full sphere of radius `r` about `c` (in the world xz-plane at
/// y = 0), as a revolve of a half-disc whose diameter lies on the
/// axis — the shape a tepal seam would be carved with.
fn ball<S: Scalar>(c: (f64, f64), r: f64) -> Body<S> {
    let plane = SketchPlane::from_frame(pt3(c.0, 0.0, c.1), v3(1.0, 0.0, 0.0), v3(0.0, 0.0, 1.0));
    // Stays raw under LIB-U2 PR-2: authored centre-first
    // (`arc_to_center`), a binding mode the PATHS algebra does not
    // have — re-authoring as {endpoint+bulge} would re-derive the
    // bulge from the centre, re-typing a computed value.
    let lp = LoopBuilder::start(p2(0.0, -r))
        .arc_to_center(p2(0.0, r), p2(0.0, 0.0), ArcSweep::Ccw)
        .close();
    revolve(
        &validated(plane, vec![lp]).expect("lily profile validates"),
        sketch_axis(),
        Revolution::Full,
    )
    .expect("probe ball revolves")
    .body
}

/// Prints one probe line, asserting that the wall is still standing —
/// and that it is still THE SAME wall.
///
/// `pinned` names the exact refusal this probe claims: the variant and,
/// where the payload carries the geometric content of the claim, its
/// fields. Three outcomes, three meanings (the `skinned::narration`
/// shape):
///
/// - the pinned refusal → narrate it, the findings-list entry holds;
/// - a DIFFERENT refusal → panic. Err-ness alone is not the claim: a
///   probe that only pinned "some error" would stay green while the
///   frontier moved underneath it, and the findings list would quietly
///   become fiction (review MINOR-1);
/// - success → panic with instructions, the `curvedcut::pin_frontier`
///   retire-on-closure contract.
fn wall<T, E: core::fmt::Debug>(
    n: u32,
    what: &str,
    outcome: Result<T, E>,
    pinned: impl FnOnce(&E) -> bool,
    retire: &str,
) {
    match outcome {
        Err(e) if pinned(&e) => println!("   wall {n} — {what}: REFUSED TYPED, {e:?}"),
        Err(e) => panic!(
            "wall {n} ({what}) still refuses, but NOT with the refusal it pins \
             ({e:?}) — the wall MOVED. Re-derive this probe AND its findings-list \
             entry before trusting either."
        ),
        Ok(_) => panic!(
            "wall {n} ({what}) NO LONGER REFUSES — the lily can now say this. \
             Retire the probe and {retire}"
        ),
    }
}

/// The lily's frontier, run live: every shape the plant WANTED and the
/// kernel would not state, attempted for real and pinned by its own
/// typed refusal.
pub fn wall_probes<S: Scalar>() {
    println!("\n-- the lily's walls: what a plant asks for that the kernel will not say --");
    let pieces = plant::<S>();
    let by = |name: &str| -> &Body<S> {
        &pieces
            .iter()
            .find(|p| p.name == name)
            .expect("named lily piece")
            .body
    };
    let (stem, arch, lant) = (by("lily_stem"), by("lily_arch"), by("lily_lantern"));

    // 1. The stem is ONE stem. Its two arcs meet on a shared disk —
    //    an exact coincident planar contact, the crosslap mate — so
    //    the glue is the M5 S1 declared REST zip if it reaches it.
    wall(
        1,
        "glue the two stem arcs into one stem (declared coincident-planar mate)",
        crate::booleans::try_union_declared(stem, arch),
        // The KIND is the claim: the refusal names a TORUS face, i.e.
        // the tangent tube walls, not the coincident planar discs.
        |e| {
            matches!(
                e,
                BooleanError::CurvedBooleanUnsupported {
                    operand: Operand::A,
                    kind: SurfaceKind::Torus,
                    ..
                }
            )
        },
        "make the stem a single body",
    );

    // 2. The flower grows OUT OF the pedicel. That weld is a
    //    transverse curved boolean between a torus tube and a sphere
    //    zone: the SSI the kernel has no closed form for.
    wall(
        2,
        "weld the lantern onto the arch (torus tube x sphere zone)",
        pncad::topo::union(lant, arch),
        |e| {
            matches!(
                e,
                BooleanError::CurvedBooleanUnsupported {
                    operand: Operand::A,
                    kind: SurfaceKind::Cone,
                    ..
                }
            )
        },
        "join flower to stem and drop the set-back trick",
    );

    // 3. Real leaves sweep back out of their own plane. An extrusion
    //    along anything but the sketch normal is oblique, and oblique
    //    extrusion is deferred past M2.
    let leafp = {
        let plane =
            SketchPlane::from_frame(pt3(0.0, 0.0, 0.0), v3(1.0, 0.0, 0.0), v3(0.0, 1.0, 0.0));
        // Stays raw under LIB-U2 PR-2: via-point arcs + cusp tips
        // (see `leaf`).
        let lp = LoopBuilder::start(p2(0.0, 0.0))
            .arc_to_via(p2(0.5, 0.12), p2(1.0, 0.0))
            .close_arc_via(p2(0.5, 0.02));
        validated(plane, vec![lp]).expect("lily profile validates")
    };
    wall(
        3,
        "sweep a leaf back out of its own plane (oblique extrusion)",
        extrude(&leafp, Extrusion::Vector(v3::<S>(0.0, 0.3, 0.04))),
        |e| matches!(e, ExtrudeError::ObliqueExtrusion),
        "give the leaves a swept-back set",
    );

    // 4. A bud is an OVOID, not a ball: a sphere scaled along its own
    //    axis. There is no ellipsoid surface and no non-uniform map —
    //    `transform_rigid` is the only body map, and it decides
    //    rigidity rather than trusting it.
    let stretch = Affine3::from_parts(
        Mat3::from_cols(v3::<S>(1.0, 0.0, 0.0), v3(0.0, 1.0, 0.0), v3(0.0, 0.0, 1.6)),
        v3(0.0, 0.0, 0.0),
    );
    wall(
        4,
        "stretch a lantern into an ovoid bud (non-uniform scale)",
        pncad::topo::transform_rigid(lant, &stretch),
        // The NAMED predicate matters: a unit-norm failure on the
        // scaled column, not a determinant or orthogonality failure.
        |e| matches!(e, TransformError::NotRigid { check } if *check == "transform_rigid_col2_unit"),
        "model buds as spheroids",
    );

    // 5. The three leaves are hand-placed because there is no mirror:
    //    a reflection is improper, and the rigidity predicate decides
    //    the determinant, not just the column norms.
    let mirror = Affine3::from_parts(
        Mat3::from_cols(
            v3::<S>(1.0, 0.0, 0.0),
            v3(0.0, -1.0, 0.0),
            v3(0.0, 0.0, 1.0),
        ),
        v3(0.0, 0.0, 0.0),
    );
    wall(
        5,
        "mirror a leaf across the plant's plane (improper isometry)",
        pncad::topo::transform_rigid(by("lily_leaf_a"), &mirror),
        // A reflection's columns ARE unit and orthogonal; only the
        // determinant catches it, and that is the whole point.
        |e| matches!(e, TransformError::NotRigid { check } if *check == "transform_rigid_det_plus_one"),
        "author leaves once and mirror them",
    );

    // 6. The lantern's mouth is a hard circle where the sphere zone
    //    meets the conical pucker. A rolling ball would soften it —
    //    but `fillet_edges` is the whole-body door on a convex,
    //    planar-faced, trivalent polyhedron.
    let rim: Vec<pncad::topo::EdgeKey> = lant.edges().map(|(k, _)| k).collect();
    wall(
        6,
        "roll a ball along the lantern's mouth rim (fillet a curved body)",
        pncad::sweep::fillet::fillet_edges(
            lant,
            &rim,
            S::from_f64(0.02),
            pncad::geom_core::Band::linear().expect("band"),
        ),
        // margin EXACTLY zero is the finding: a co-surface seam
        // meridian, not a near-tangency that a tolerance could split.
        |e| matches!(e, FilletError::TangentialEdge { margin, .. } if *margin == 0.0),
        "soften the tepal-tip rim",
    );

    // 7. The lantern is THREE tepals fused, and their seams are
    //    longitudinal grooves. Carving one means a sphere-on-sphere
    //    subtract — the curved-on-curved boolean.
    wall(
        7,
        "carve a tepal seam into the lantern (sphere x sphere subtract)",
        pncad::topo::subtract(lant, &ball::<S>((-2.80, 0.90), 0.16)),
        |e| {
            matches!(
                e,
                BooleanError::CurvedOpUnsupported {
                    op: BooleanOp::Subtract,
                    operand: Operand::A,
                    ..
                }
            )
        },
        "give the lanterns their three tepal seams",
    );
    println!(
        "   (wall 9 — a TAPERING sweep — is the one remaining ABSENCE, not a \
         refusal, so it cannot be probed at runtime. Walls 8 and 10 CLOSED with \
         M6-3: `sweep::sweep_body` is the general-path sweep body and \
         `sweep::loft_body` the skin assembly — the loft stop builds one live, \
         and `skinned::narration`'s retire-on-closure pin fired as designed. \
         Wall 10's closure was only PARTIAL until #207: every curved path \
         refused at assembly on the skin fit's synthesized weight channel, \
         so the general-path sweep had no successful caller until that fix.)"
    );
}

// ---------------------------------------------------------------
// Review probes (PR #175 adversarial review, `review/lily`): the
// G1/placement claims checked against the STORED geometry, not the
// construction code, plus the finding-13 tessellation table
// re-measured. Kept as tests so a silent placement regression
// (finding 11: sign/handedness errors produce a valid solid in the
// wrong place) fails loud here — verified to catch a flipped
// `-spec.turn` in `tube_arc` during the review.
// ---------------------------------------------------------------

#[cfg(test)]
mod review_probes {
    use super::*;
    use pncad::topo::Surface;

    fn pieces() -> Vec<Piece<f64>> {
        plant::<f64>()
    }

    fn body<'a>(ps: &'a [Piece<f64>], name: &str) -> &'a Body<f64> {
        &ps.iter().find(|p| p.name == name).expect("piece").body
    }

    /// All stored cap planes of a body: (origin, unit normal).
    fn planes(b: &Body<f64>) -> Vec<(Point3<f64>, Vec3<f64>)> {
        b.faces()
            .filter_map(|(_, f)| match b.get_surface(f.surface) {
                Some(Surface::Plane { origin, normal, .. }) => Some((*origin, *normal)),
                _ => None,
            })
            .collect()
    }

    /// The body's single stored torus carrier: (center, axis, R, r, u_ref).
    fn torus(b: &Body<f64>) -> (Point3<f64>, Vec3<f64>, f64, f64, Vec3<f64>) {
        let mut found = None;
        for (_, f) in b.faces() {
            if let Some(Surface::Torus {
                center,
                axis,
                major_radius,
                minor_radius,
                u_ref,
            }) = b.get_surface(f.surface)
            {
                let t = (*center, *axis, *major_radius, *minor_radius, *u_ref);
                if let Some(prev) = &found {
                    // Both torus half-bands must share ONE carrier.
                    let (pc, pa, pr, pm, _): &(Point3<f64>, Vec3<f64>, f64, f64, Vec3<f64>) = prev;
                    assert!((pc.x - t.0.x).abs() < 1e-15 && (pc.z - t.0.z).abs() < 1e-15);
                    assert!((pa.y.abs() - t.1.y.abs()).abs() < 1e-15);
                    assert!((pr - t.2).abs() < 1e-15 && (pm - t.3).abs() < 1e-15);
                } else {
                    found = Some(t);
                }
            }
        }
        found.expect("a torus wall")
    }

    fn cross_norm(a: Vec3<f64>, b: Vec3<f64>) -> f64 {
        let c = (
            a.y * b.z - a.z * b.y,
            a.z * b.x - a.x * b.z,
            a.x * b.y - a.y * b.x,
        );
        (c.0 * c.0 + c.1 * c.1 + c.2 * c.2).sqrt()
    }

    /// Independently re-derived joint data (reviewer's own turtle
    /// algebra, computed outside this codebase — NOT lifted from
    /// [`Turtle`]): world (x, z) of the two stem joints and the unit
    /// tangents there.
    const P1: (f64, f64) = (-0.3640807271660629, 1.87303296707956);
    const T1: (f64, f64) = (-0.374606593415912, 0.9271838545667874);
    const P2: (f64, f64) = (-2.4599453279967154, 1.2322628544225218);
    const T2: (f64, f64) = (0.20791169081775934, -0.9781476007338056);
    const T3: (f64, f64) = (0.17364817766693053, -0.984807753012208);
    const SPHERE1_C: (f64, f64) = (-2.3934135869350324, 0.919255622187704);
    const SPHERE2_C: (f64, f64) = (-0.9512338661211347, 1.2295604347374214);

    /// A cap plane of `b` passes through world point `p` (xz-plane)
    /// with normal parallel to `t` — i.e. the tube's end tangent THERE
    /// is `t`, read off the stored cap, not the turtle.
    fn assert_cap(b: &Body<f64>, p: (f64, f64), t: (f64, f64), what: &str) {
        let tv = Vec3::new(t.0, 0.0, t.1);
        let hit = planes(b).into_iter().any(|(o, n)| {
            cross_norm(n, tv) < 1e-14
                && ((p.0 - o.x) * n.x + (0.0 - o.y) * n.y + (p.1 - o.z) * n.z).abs() < 1e-12
        });
        assert!(
            hit,
            "{what}: no stored cap plane through the joint with the joint tangent"
        );
    }

    /// Claim: G1-by-construction, read from STORED geometry. At each
    /// stem joint both bodies carry a cap plane through the SAME point
    /// with tangent-parallel normals, and the torus carriers live
    /// where the reviewer's independent derivation says they must.
    #[test]
    fn stem_joints_are_g1_in_the_stored_geometry() {
        let ps = pieces();
        let (stem, arch, pedicel) = (
            body(&ps, "lily_stem"),
            body(&ps, "lily_arch"),
            body(&ps, "lily_pedicel"),
        );
        let (c, a, big_r, r, u) = torus(stem);
        assert!((c.x - -5.0).abs() < 1e-12 && c.y.abs() < 1e-15 && c.z.abs() < 1e-12);
        assert!((a.x.abs() + a.z.abs()) < 1e-15 && (a.y.abs() - 1.0).abs() < 1e-15);
        // NOTE (review finding): the tube radius is RECONSTRUCTED by
        // revolve from the profile's bulge arcs, not carried exactly —
        // stored 0.05999999999999961 (4 ulps off the authored 0.060).
        assert!((big_r - 5.0).abs() < 1e-12 && (r - 0.060).abs() < 1e-12);
        // u_ref is the radial to the arc's start: +x for the stem.
        assert!(
            cross_norm(u, Vec3::new(1.0, 0.0, 0.0)) < 1e-15,
            "stem u_ref"
        );
        let (c2, _, big_r2, r2, _) = torus(arch);
        assert!((c2.x - -1.3839829671895292).abs() < 1e-12);
        assert!((c2.z - 1.460965714322057).abs() < 1e-12);
        assert!((big_r2 - 1.1).abs() < 1e-12 && (r2 - 0.052).abs() < 1e-12);
        let (_, _, big_r3, r3, _) = torus(pedicel);
        assert!((big_r3 - 0.42).abs() < 1e-12 && (r3 - 0.032).abs() < 1e-12);
        // Joint 1: stem end / arch start share point P1 and tangent T1.
        assert_cap(stem, P1, T1, "stem end");
        assert_cap(arch, P1, T1, "arch start");
        // Joint 2: arch end at P2 with tangent T2 (the flower hangs here).
        assert_cap(arch, P2, T2, "arch end");
        // The fork reuses P1; the pedicel's START tangent is the
        // authored (cos150, sin150), not T1 — branch, not continuation.
        assert_cap(
            pedicel,
            P1,
            (f64::cos(deg(150.0)), f64::sin(deg(150.0))),
            "pedicel start",
        );
    }

    /// Claim: the lantern axes ARE the stem tangents and the globes
    /// sit at attach + top·dir — read off the stored sphere/cone.
    #[test]
    fn lantern_axes_are_the_stored_stem_tangents() {
        let ps = pieces();
        for (name, t, cen, rad) in [
            ("lily_lantern", T2, SPHERE1_C, 0.44),
            ("lily_lantern2", T3, SPHERE2_C, 0.30),
        ] {
            let b = body(&ps, name);
            let tv = Vec3::new(t.0, 0.0, t.1);
            let mut saw_sphere = false;
            let mut saw_cone = false;
            for (_, f) in b.faces() {
                match b.get_surface(f.surface) {
                    Some(Surface::Sphere {
                        center,
                        radius,
                        axis,
                        ..
                    }) => {
                        saw_sphere = true;
                        assert!(
                            cross_norm(*axis, tv) < 1e-14,
                            "{name}: sphere axis || tangent"
                        );
                        assert!((center.x - cen.0).abs() < 1e-12, "{name} center.x");
                        assert!(center.y.abs() < 1e-15, "{name} center.y");
                        assert!((center.z - cen.1).abs() < 1e-12, "{name} center.z");
                        assert!((radius - rad).abs() < 1e-15, "{name} radius");
                    }
                    Some(Surface::Cone { axis, .. }) => {
                        saw_cone = true;
                        assert!(
                            cross_norm(*axis, tv) < 1e-14,
                            "{name}: cone axis || tangent"
                        );
                    }
                    _ => {}
                }
            }
            assert!(
                saw_sphere && saw_cone,
                "{name}: sphere zone + conical pucker"
            );
        }
    }

    /// Finding 13 re-measured: the tessellation table's numbers, plus
    /// the arch's 136,076, pinned as printed in the PR description.
    #[test]
    fn finding_13_tessellation_table_reproduces() {
        use pncad::mesh::validate::{signed_volume, triangle_count};
        let ps = pieces();
        let table = [
            ("lily_stem", 5e-3, 31_612usize),
            ("lily_stem", 2e-3, 76_436),
            ("lily_arch", 2e-3, 136_076),
            ("lily_lantern", 5e-3, 988),
            ("lily_lantern", 2e-3, 2_348),
        ];
        for (name, delta, want) in table {
            let m = pncad::mesh::tessellate(body(&ps, name), delta).expect("tessellate");
            assert_eq!(triangle_count(&m), want, "{name} @ {delta:e}");
        }
        // Lantern volume error at both deltas (1.25% / 0.53% claimed).
        let exact = 0.36225803729804673;
        for (delta, lo, hi) in [(5e-3, 0.0120, 0.0130), (2e-3, 0.0050, 0.0056)] {
            let m = pncad::mesh::tessellate(body(&ps, "lily_lantern"), delta).expect("tessellate");
            let rel = ((signed_volume(&m) - exact) / exact).abs();
            assert!(rel > lo && rel < hi, "lantern @ {delta:e}: rel {rel}");
        }
    }
}
