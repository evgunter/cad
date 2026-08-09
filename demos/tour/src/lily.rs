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
//!   windowed TUBE ALONG AN ARC, i.e. a torus segment said in world
//!   coordinates: ring centre, spine axis, start radial, ring radius,
//!   angular window, tube radius. A turtle walks the arcs in the
//!   world xz-plane so consecutive arcs are G1 by construction
//!   (shared tangent), and the joint is a shared disk the eye does
//!   not see. They are separate BODIES: gluing them is a coincident-
//!   planar contact, which the kernel refuses (probe 1).
//! - the **flowers** are lanterns: a full revolve whose wall is a
//!   sphere zone truncated at both poles — a wide belly, a small
//!   attachment disk where the pedicel enters, and a puckered conical
//!   mouth closing to a small disk. Sphere zone + cone + two planes,
//!   all exact — a `revolve` still, unchanged by this refresh.
//! - the **leaves** are keeled blades: a thin four-line KITE section —
//!   two sharp margins on a chord, an unequal ridge and keel across
//!   it — carried along a gently arching circular spine by the
//!   general-path sweep. The blade now leaves the plane it was drawn
//!   in, which the extruded crescent could not do. Two things it
//!   still cannot do: TAPER (findings entry 9, so one width base to
//!   tip) and carry an ARC in its section (the skin lane refuses a
//!   rational wall — see [`leaf`]).
//!
//! Proportions are chosen, not measured: a stylized lily that the
//! kernel can state exactly beats a literal one it must approximate.
//!
//! **What "exact" claims, precisely** (review NOTE-1). For the
//! analytic pieces it claims the surface KIND *and* the stored
//! PARAMETERS: a stem wall is a `Surface::Torus`, not a spline fit of
//! one, it exports as `TOROIDAL_SURFACE`, and its centre, axis,
//! `u_ref`, major radius and minor radius are the world-coordinate
//! numbers this module passed in — `lily_stem`'s stored
//! `minor_radius` is the authored 0.060, not a reconstruction
//! 3.9e-16 (56 ulps) below it, because nothing on the tube path goes
//! profile → bulge → radius, and placement is an argument rather than
//! a silent sketch-frame landing.
//!
//! The leaf blades are the one place the lily is FITTED rather than
//! stated. A swept skin is a NURBS surface through sampled stations,
//! so a blade's walls are B-spline surfaces interpolating nine exact
//! points of an exact circular spine, not a closed form of the swept
//! kite. That is the price of leaving the plane, and it is stated here
//! rather than hidden.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::f64::consts::PI;

use pncad::geom_brep::SurfaceKind;
use pncad::geom_core::{Affine3, Mat3, Point3, Vec2, Vec3};
use pncad::prelude::{Open, Start};
use pncad::profile::{ArcSweep, ProfileLoop, SketchPlane};
use pncad::sweep::fillet::FilletError;
use pncad::sweep::readback::{WedgeFrames, revolved_caps};
use pncad::sweep::{
    ExtrudeError, Extrusion, Revolution, RevolveAxis, TubeWindow, extrude, revolve, sweep_body,
    tube_along_arc,
};
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

/// One stem segment: a circular tube of radius `tube` running along
/// the arc — a torus segment with two disk caps, said in WORLD
/// coordinates through [`tube_along_arc`].
///
/// Every argument is the intent itself and is stored verbatim: the
/// ring centre is the world point in the xz-plane, `u_ref` is the
/// radial to the arc's START, `major_radius` is the ring, and
/// `minor_radius` is the authored tube radius — no profile, no bulge,
/// no reconstruction. The axis is `-ŷ` for a left turn and `+ŷ` for a
/// right one, because a right-handed rotation about `-ŷ` runs
/// COUNTERCLOCKWISE in the xz-plane drawn with +x right and +z up,
/// which is the turtle's positive sense; with that choice the
/// traversed window is always `[0, |turn|]` from `u_ref`.
fn tube_arc<S: Scalar>(spec: ArcSpec, tube: f64) -> (Body<S>, WedgeFrames<S>) {
    let sense = if spec.turn >= 0.0 { -1.0 } else { 1.0 };
    let revolved = tube_along_arc(
        pt3(spec.center.0, 0.0, spec.center.1),
        v3(0.0, sense, 0.0),
        v3(spec.radial.0, 0.0, spec.radial.1),
        S::from_f64(spec.ring),
        TubeWindow::Arc {
            t0: S::from_f64(0.0),
            t1: S::from_f64(spec.turn.abs()),
        },
        S::from_f64(tube),
    )
    .expect("stem tube arc builds");
    // The joint frames, ASKED of the operation that made them (LIB-U5
    // deliverable 3): a windowed tube's two wedge caps ARE the tube's
    // ends, and each cap plane's normal is the tube's tangent there.
    // Before this door the only way to see them was to scan every face
    // of the finished body for planar carriers.
    let caps = revolved_caps(&revolved).expect("a windowed tube has caps");
    (revolved.body, caps)
}

/// A **lantern**: the closed globular flower. A full revolve whose
/// wall is a sphere ZONE of radius `globe`, truncated at `top` above
/// the centre (the attachment disk the pedicel enters through) and at
/// `mouth` below it, then closed by a conical pucker of drop
/// `lip_drop` down to a disk of radius `lip_r` — the three tepal tips
/// meeting under the lantern.
///
/// Faces: attachment plane, sphere zone, cone, mouth plane. Every one
/// exact; the profile is authored centre-first (`arc_center`) so the
/// zone's carrier is the sphere itself and not a fitted arc. That
/// centre-intent is now sayable in the algebra (LIB-G1 constructor 3):
/// the globe centre is authored, the winding is structural, and
/// equidistance of the two endpoints from the centre is CHECKED — this
/// profile derives both radii from the sphere, so it passes by
/// construction and would refuse loudly if it ever stopped doing so.
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
    let lp = Open
        .at(p2(0.0, 0.0))
        .line_to(p2(r_top, 0.0))
        .expect("lantern attachment disk")
        // The belly: the sphere's own arc about the globe centre,
        // swept the long way round the equator (Ccw in sketch (s, t)).
        .arc_center(p2(0.0, top), p2(r_mouth, t_mouth), ArcSweep::Ccw)
        .expect("lantern belly rides the globe")
        .line_to(p2(lip_r, t_end))
        .expect("lantern pucker cone")
        .line_to(p2(0.0, t_end))
        .expect("lantern lip disk")
        .line_to(Start)
        .expect("lantern axis seam")
        .into();
    revolve(
        &validated(plane, vec![lp]).expect("lily profile validates"),
        sketch_axis(),
        Revolution::Full,
    )
    .expect("lantern revolves")
    .body
}

/// Stations along a leaf's swept spine, and the v-degree its skin is
/// fitted at (the swept-elbow corpus fixture's numbers).
const LEAF_STATIONS: usize = 9;
/// The leaf skin's fit degree along the path.
const LEAF_V_DEGREE: usize = 3;

/// A **keeled leaf blade**: a thin section carried along a gently
/// arching spine by [`sweep_body`] — the general-path sweep, not an
/// extrusion, so the blade leaves the plane it was drawn in.
///
/// The section is a KITE of four straight lines: the two sharp
/// margins at `±width/2` on the chord, a ridge `ridge` above it and a
/// keel `keel` below, the two rises DIFFERENT so the blade is
/// asymmetric about its own chord exactly as the extruded crescent
/// was. The spine runs through the chord's midpoint, i.e. through the
/// midrib.
///
/// **Why straight lines and not the crescent's arcs.** The skin lane
/// only carries INTEGRAL sections. An arc is a rational NURBS, a
/// rational section skins to a rational wall, and a rational carrier
/// has no `speed_lower_bound` — `nurbs_span_meter` comes back
/// `Invalid` and the body refuses at assembly (`geom-brep`'s rung-3
/// span meter; the same poison #207 removed for INTEGRAL inputs it
/// never claimed to remove for rational ones). So the swept blade is
/// the honest shape the sweep vocabulary can state today, and the
/// arcs stay where they still work — the lanterns' meridian and the
/// stem's tube. Nothing here approximates a curve with a chord: a
/// kite is exactly a kite.
///
/// The spine leaves `base` along `dir` and turns through `curl`
/// radians toward `up` (Gram–Schmidt'd against `dir`; negative `curl`
/// arches the blade over, which is what a basal leaf does), staying a
/// circular arc of length `len` sampled at [`LEAF_STATIONS`] exact
/// points that a cubic `NurbsCurve3::interpolate` runs through. The
/// profile plane's normal IS the spine's start tangent, so the section
/// rides normal to its own path. The blade holds ONE width from base
/// to tip: there is no tapering sweep (findings entry 9).
fn leaf<S: Scalar>(
    base: (f64, f64, f64),
    dir: (f64, f64, f64),
    up: (f64, f64, f64),
    len: f64,
    width: f64,
    ridge: f64,
    keel: f64,
    curl: f64,
) -> Body<S> {
    let nrm = |(x, y, z): (f64, f64, f64)| {
        let l = (x.powi(2) + y.powi(2) + z.powi(2)).sqrt();
        (x / l, y / l, z / l)
    };
    let d = nrm(dir);
    let dot = up.0 * d.0 + up.1 * d.1 + up.2 * d.2;
    let v = nrm((up.0 - dot * d.0, up.1 - dot * d.1, up.2 - dot * d.2));
    // u = v x d completes a right-handed (u, v, d) frame, so the
    // sketch plane's normal u x v is the spine's start tangent d.
    let u = (
        v.1 * d.2 - v.2 * d.1,
        v.2 * d.0 - v.0 * d.2,
        v.0 * d.1 - v.1 * d.0,
    );
    // The spine: a circular arc of length `len` turning through `curl`
    // in the (d, v) plane, i.e. radius len/curl, sampled exactly.
    let r = len / curl;
    let pts: Vec<Point3<f64>> = (0..LEAF_STATIONS)
        .map(|k| {
            #[allow(clippy::cast_precision_loss)]
            let a = curl * (k as f64) / ((LEAF_STATIONS - 1) as f64);
            let (s, c) = (r * a.sin(), r * (1.0 - a.cos()));
            Point3::new(
                base.0 + s * d.0 + c * v.0,
                base.1 + s * d.1 + c * v.1,
                base.2 + s * d.2 + c * v.2,
            )
        })
        .collect();
    let path =
        pncad::geom_curves::NurbsCurve3::interpolate(&pts, 3).expect("the leaf spine interpolates");
    let place = SketchPlane::from_frame(
        pt3(base.0, base.1, base.2),
        v3(u.0, u.1, u.2),
        v3(v.0, v.1, v.2),
    )
    .placement;
    // The kite, wound counterclockwise in the sketch (s, t) frame:
    // margin, keel, margin, ridge.
    let section: Vec<ProfileLoop<f64>> = vec![pncad::authoring::polygon(&[
        (-0.5 * width, 0.0),
        (0.0, -keel),
        (0.5 * width, 0.0),
        (0.0, ridge),
    ])];
    sweep_body::<S>(&section, place, &path, LEAF_STATIONS, LEAF_V_DEGREE)
        .expect("the leaf sweeps along its spine")
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
    /// For a piece built as a PARTIAL revolve (the stem tubes), the
    /// two joint frames its revolve recorded — read back from the
    /// operation rather than rediscovered in the finished body.
    /// `None` for the full revolves and extrusions, which have no
    /// wedge caps.
    // Read by the joint-frame test below, which is the point of
    // carrying it: the render path wants the body alone.
    #[allow(dead_code)]
    pub caps: Option<WedgeFrames<S>>,
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

    let (stem, stem_caps) = tube_arc(lower, 0.060);
    let (arch, arch_caps) = tube_arc(upper, 0.052);
    let (pedicel_body, pedicel_caps) = tube_arc(pedicel, 0.032);

    vec![
        Piece {
            name: "lily_stem",
            color: GREEN_STEM,
            body: stem,
            caps: Some(stem_caps),
        },
        Piece {
            name: "lily_arch",
            color: GREEN_STEM,
            body: arch,
            caps: Some(arch_caps),
        },
        Piece {
            name: "lily_pedicel",
            color: GREEN_STEM,
            body: pedicel_body,
            caps: Some(pedicel_caps),
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
            caps: None,
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
            caps: None,
        },
        Piece {
            name: "lily_leaf_a",
            color: GREEN_LEAF,
            body: leaf(
                (0.04, 0.05, 0.03),
                (-0.60, 0.66, 0.52),
                (0.0, 0.0, 1.0),
                1.45,
                0.195,
                0.016,
                0.008,
                -0.45,
            ),
            caps: None,
        },
        Piece {
            name: "lily_leaf_b",
            color: GREEN_LEAF,
            body: leaf(
                (-0.03, -0.06, 0.06),
                (-0.68, -0.55, 0.44),
                (0.0, 0.0, 1.0),
                1.25,
                0.170,
                0.015,
                0.007,
                -0.40,
            ),
            caps: None,
        },
        Piece {
            name: "lily_leaf_c",
            color: GREEN_LEAF,
            body: leaf(
                (0.02, 0.01, 0.02),
                (0.62, 0.10, 0.78),
                (0.0, 0.0, 1.0),
                0.95,
                0.140,
                0.013,
                0.006,
                -0.35,
            ),
            caps: None,
        },
    ]
}

/// The tour stop.
pub fn stops() -> Vec<Stop> {
    let pieces = plant::<f64>();
    let note = format!(
        "{} closed solids: 3 torus-segment stem tubes said in WORLD \
         coordinates (centre/axis/u_ref/radii stored exactly as \
         given), 2 sphere-zone lanterns with conical mouths, and 3 \
         keeled leaf blades — a four-line kite section swept along an \
         arching NURBS spine, out of the plane it was drawn in. The \
         five analytic bodies approximate nothing — torus, sphere, \
         cone and plane exactly, parameters included; the blades are \
         fitted skins, the price of leaving the plane. Nothing is \
         JOINED: see the wall probes.",
        pieces.len()
    );
    vec![Stop {
        name: "lily",
        caption: "globe lily (Calochortus albus)".to_string(),
        montage: true,
        story: "a nodding globe lily — arching stem, two closed globular \
                lanterns, arching keeled basal leaves; torus/sphere/cone/plane \
                exact to the stored parameter, blades swept out of plane",
        ops: "Turtle-walked G1 arc chain -> tube_along_arc(world centre/axis/ \
              u_ref/radii, windowed) tubes; revolve(Full) sphere-zone \
              lanterns; sweep_body(kite section, arched NURBS spine) leaves",
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
    // Algebra-authored (LIB-G1): centre-first, with the sphere's own
    // centre authored and the bulge derived at lowering.
    let lp = Open
        .at(p2(0.0, -r))
        .arc_center(p2(0.0, 0.0), p2(0.0, r), ArcSweep::Ccw)
        .expect("ball meridian rides its centre")
        .line_to(Start)
        .expect("ball axis seam")
        .into();
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

    // 3. The lily's leaves DO leave their own plane now — each blade
    //    is a crescent section swept along an arching spine. What is
    //    still refused is the cheap way to ask for it: an EXTRUSION
    //    along anything but the sketch normal is oblique, and oblique
    //    extrusion is deferred past M2. The probe pins that door, not
    //    the out-of-plane blade, which the scene above builds live.
    let leafp = {
        let plane =
            SketchPlane::from_frame(pt3(0.0, 0.0, 0.0), v3(1.0, 0.0, 0.0), v3(0.0, 1.0, 0.0));
        // Algebra-authored (LIB-G1): via-point arcs (see `leaf`).
        let lp = Open
            .at(p2(0.0, 0.0))
            .arc_via(p2(0.5, 0.12), p2(1.0, 0.0))
            .expect("probe leaf outer arc")
            .arc_via(p2(0.5, 0.02), Start)
            .expect("probe leaf inner arc")
            .into();
        validated(plane, vec![lp]).expect("lily profile validates")
    };
    wall(
        3,
        "tilt a leaf out of its own plane the cheap way (oblique extrusion)",
        extrude(&leafp, Extrusion::Vector(v3::<S>(0.0, 0.3, 0.04))),
        |e| matches!(e, ExtrudeError::ObliqueExtrusion),
        "let a sketch profile lean out of its plane in one step",
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
         refusal, so it cannot be probed at runtime; it is why the swept \
         blades above hold one width from base to tip. Walls 8 and 10 CLOSED \
         with M6-3: `sweep::sweep_body` is the general-path sweep body and \
         `sweep::loft_body` the skin assembly — the leaves here build three \
         live, and `skinned::narration`'s retire-on-closure pin fired as \
         designed. Wall 10's closure was only PARTIAL until #207: every \
         curved path refused at assembly on the skin fit's synthesized weight \
         channel, so the general-path sweep had no successful caller until \
         that fix.)"
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

    /// A tube piece's two joint frames, as its revolve recorded them.
    fn caps<'a>(ps: &'a [Piece<f64>], name: &str) -> &'a WedgeFrames<f64> {
        ps.iter()
            .find(|p| p.name == name)
            .expect("piece")
            .caps
            .as_ref()
            .expect("a stem tube is a partial revolve, so it has caps")
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
    ///
    /// **GAP (LIB-U5): these stay literals, and the door that would
    /// retire them does not exist yet.** U5's `revolved_caps` answers
    /// where each tube's cap plane IS, which is what `assert_cap`
    /// below now asks — but "where does the PATH put its next joint,
    /// and with what tangent frame?" is a path-frame read-back, U4's
    /// deliverable. The producer here (`Turtle`) is demo-local, so
    /// there is no kernel choice to interrogate yet: these are an
    /// independent oracle for the frames the doors report, and under
    /// the U5 discriminator an independent derivation is a TEST, not
    /// a restatement to migrate. When U4's path-frame door lands,
    /// P1/T1/P2/T2/T3 become queries against it.
    const P1: (f64, f64) = (-0.3640807271660629, 1.87303296707956);
    const T1: (f64, f64) = (-0.374606593415912, 0.9271838545667874);
    const P2: (f64, f64) = (-2.4599453279967154, 1.2322628544225218);
    const T2: (f64, f64) = (0.20791169081775934, -0.9781476007338056);
    const T3: (f64, f64) = (0.17364817766693053, -0.984807753012208);
    const SPHERE1_C: (f64, f64) = (-2.3934135869350324, 0.919255622187704);
    const SPHERE2_C: (f64, f64) = (-0.9512338661211347, 1.2295604347374214);

    /// One of the tube's two JOINT FRAMES passes through world point
    /// `p` (xz-plane) with normal parallel to `t` — i.e. the tube's
    /// end tangent THERE is `t`.
    ///
    /// The frames come from `sweep::readback::revolved_caps` (LIB-U5):
    /// this used to scan every face of the body for a planar carrier
    /// and hope the right one turned up. Which of the two ends
    /// answers is the revolve's business, so both are offered — that
    /// is a two-element check against NAMED caps, not a search of the
    /// whole boundary.
    fn assert_cap(caps: &WedgeFrames<f64>, p: (f64, f64), t: (f64, f64), what: &str) {
        let tv = Vec3::new(t.0, 0.0, t.1);
        let hit = [caps.start, caps.end].into_iter().any(|pose| {
            let (o, n) = (pose.origin, pose.axis);
            cross_norm(n, tv) < 1e-14
                && ((p.0 - o.x) * n.x + (0.0 - o.y) * n.y + (p.1 - o.z) * n.z).abs() < 1e-12
        });
        assert!(
            hit,
            "{what}: neither joint frame passes through the joint with the joint tangent"
        );
    }

    /// Claim: G1-by-construction, read from STORED geometry — and the
    /// stored geometry IS the world-coordinate intent, not a
    /// reconstruction of it. Nothing on the tube door's path goes
    /// profile → bulge → radius, so every quantity the caller handed
    /// in comes back bit-for-bit and is asserted with `==`: the
    /// turtle's ring centres and radials, the ring radii, and the tube
    /// radii. Only the DERIVED joint data (cap plane through a joint
    /// point, with the joint tangent as its normal) is windowed, and
    /// only because a cap frame is trigonometry away from the stored
    /// carrier.
    #[test]
    fn stem_joints_are_g1_in_the_stored_geometry() {
        let ps = pieces();
        let (stem, arch, pedicel) = (
            body(&ps, "lily_stem"),
            body(&ps, "lily_arch"),
            body(&ps, "lily_pedicel"),
        );
        // The stem: the turtle stands at the origin facing +z and
        // turns LEFT on a 5 m ring, so the centre is one ring radius
        // along the left normal −x̂ and the start radial is +x̂ — both
        // exact decimals, and both stored as such. The spine axis is
        // −ŷ because a left turn in the xz-plane is a right-handed
        // rotation about −ŷ.
        let (c, a, big_r, r, u) = torus(stem);
        assert_eq!((c.x, c.y, c.z), (-5.0, 0.0, 0.0), "stem ring centre");
        assert_eq!((a.x, a.y, a.z), (0.0, -1.0, 0.0), "stem spine axis");
        assert_eq!((u.x, u.y, u.z), (1.0, 0.0, 0.0), "stem u_ref");
        assert_eq!((big_r, r), (5.0, 0.060), "stem radii");
        // The arch and pedicel ring centres are turtle-walked, so the
        // literals are the reviewer's independent derivation (see the
        // GAP note) — but they are stored EXACTLY as the walk produced
        // them, so the radii, which are authored decimals, are `==`.
        let (c2, a2, big_r2, r2, _) = torus(arch);
        assert!((c2.x - -1.3839829671895292).abs() < 1e-12);
        assert!((c2.z - 1.460965714322057).abs() < 1e-12);
        assert_eq!((a2.x, a2.y, a2.z), (0.0, -1.0, 0.0), "arch spine axis");
        assert_eq!((big_r2, r2), (1.1, 0.052), "arch radii");
        let (_, a3, big_r3, r3, _) = torus(pedicel);
        assert_eq!((a3.x, a3.y, a3.z), (0.0, -1.0, 0.0), "pedicel spine axis");
        assert_eq!((big_r3, r3), (0.42, 0.032), "pedicel radii");
        // Joint 1: stem end / arch start share point P1 and tangent T1.
        assert_cap(caps(&ps, "lily_stem"), P1, T1, "stem end");
        assert_cap(caps(&ps, "lily_arch"), P1, T1, "arch start");
        // Joint 2: arch end at P2 with tangent T2 (the flower hangs here).
        assert_cap(caps(&ps, "lily_arch"), P2, T2, "arch end");
        // The fork reuses P1; the pedicel's START tangent is the
        // authored (cos150, sin150), not T1 — branch, not continuation.
        assert_cap(
            caps(&ps, "lily_pedicel"),
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

    /// Finding 13 re-measured: one chord budget for the whole scene
    /// spends wildly differently per body, and these are the numbers.
    ///
    /// The five analytic rows are the SAME counts the sketch-frame
    /// revolve produced — the tube door changed which parameters are
    /// stored, not which torus they describe, so the tessellator sees
    /// the same surface and splits it the same way. The three blade
    /// rows are new, and they are the other half of the finding: a
    /// swept skin over a 4-vertex section costs three orders of
    /// magnitude less than a torus tube at the same δ, because the
    /// torus lane spends its budget on the RING and not on the tube.
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
            ("lily_leaf_a", 2e-3, 1_276),
            ("lily_leaf_b", 2e-3, 976),
            ("lily_leaf_c", 2e-3, 826),
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
        // A swept blade has no analytic wall to compare against, but it
        // has PAPPUS. A rigid section carried in the path's normal
        // frame sweeps A·(centroid arc length); the kite of chord `w`
        // with rises `ridge`/`keel` has area w(ridge+keel)/2 and its
        // centroid sits (ridge−keel)/3 above the chord, i.e. that far
        // OUTSIDE the spine's centre of curvature, so its arc is
        // len + |curl|·(ridge−keel)/3. Agreement to a few 1e-5 is the
        // mesh's chord error at δ = 2e-3, and it is a two-sided band:
        // exact agreement would mean the volume was not measured off a
        // real tessellation, and a larger gap would mean the section
        // rolled about the tangent on its way down the path.
        for (name, w, ridge, keel, len, curl) in [
            ("lily_leaf_a", 0.195, 0.016, 0.008, 1.45, 0.45),
            ("lily_leaf_b", 0.170, 0.015, 0.007, 1.25, 0.40),
            ("lily_leaf_c", 0.140, 0.013, 0.006, 0.95, 0.35),
        ] {
            let area = 0.5 * w * (ridge + keel);
            let pappus = area * curl.mul_add((ridge - keel) / 3.0, len);
            let m = pncad::mesh::tessellate(body(&ps, name), 2e-3).expect("tessellate");
            let rel = ((signed_volume(&m) - pappus) / pappus).abs();
            assert!(rel > 1e-5 && rel < 5e-5, "{name}: rel {rel}");
        }
    }
}
