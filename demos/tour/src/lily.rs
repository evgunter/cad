//! **The fairy lantern** (*Calochortus pulchellus*, the Mount Diablo
//! globe lily): a nodding, closed globular YELLOW flower hanging from
//! a slender arching stem, with three long spreading sepals and
//! lance-shaped basal leaves. The scene wore *C. albus* — the white
//! one — until this refresh; the sepals are what tell the two apart,
//! and the sepals are what the loft made buildable.
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
//! - the open **flower** is a lantern: a full revolve whose wall is a
//!   sphere zone truncated at both poles — a wide belly, a small
//!   attachment disk where the pedicel enters, and a puckered conical
//!   mouth closing to a small disk. Sphere zone + cone + two planes,
//!   all exact — a `revolve` still, unchanged by this refresh.
//! - the **bud** is the same meridian said THREE times, PARTIALLY:
//!   three pre-tepals of 156 degrees each, on three axes forming a
//!   narrow tripod about the bud's own, and rolled so they nest like a
//!   pinwheel rather than merely abutting. All three share the
//!   attachment point, so the tilt splays their tips the way a bud on
//!   the turn parts at the point while still held at the neck. It
//!   needs nothing the kernel did not already have: `revolve` takes
//!   its axis in SKETCH coordinates, so a tilted axis is spelled by
//!   tilting the sketch frame, and a `Partial` sweep of a meridian
//!   whose ends sit on that axis closes fine.
//! - the two short **leaves** are keeled blades: a thin four-line
//!   KITE section — two sharp margins on a chord, an unequal ridge
//!   and keel across it — carried along a gently arching circular
//!   spine by the general-path SWEEP. The blade leaves the plane it
//!   was drawn in, which the extruded crescent could not do. Two
//!   things a sweep still cannot do: TAPER and ROLL. `sweep_body`
//!   takes one profile and derives its own frame, so there is no
//!   argument in which either could be asked for (findings entry 9).
//! - the long basal **leaf** and the three **sepals** are the same
//!   blade said as a LOFT, and they do both. `loft_body` takes the
//!   sections and the placements as two lists, so the section may
//!   change station to station — rectangle at the stem, wide flat
//!   diamond at the belly, small diamond at the tip — and the frame
//!   may roll about the spine on the way. The long leaf turns 160
//!   degrees, eased hard toward the tip, which is what a real
//!   *Calochortus* leaf lying along the ground does. Both are pinned
//!   by measurement, not assertion: see
//!   [`review_probes::the_lofted_blade_tapers_and_rolls_in_the_stored_geometry`].
//!
//!   What the loft still cannot do is close the tip to a POINT (a
//!   zero-width section refuses, though as a NON-SIMPLE profile
//!   rather than the degenerate segment it looks like — see
//!   [`Plan::tip`]) or be
//!   JOINED to the stem it grows from (probe 8). The tip diamonds are
//!   therefore small but real, and left visibly so rather than shrunk
//!   until the blunt end stops showing.
//!
//! **Every blade section here is straight lines, and that is
//! OUTSTANDING WORK rather than a constraint.** Giving the blades
//! their lanceolate arcs back is a real follow-up, not a settled
//! choice — and it wants checking against the QUADRATURE half of the
//! rational bank, which is not retired (`QuadratureUnsupported` keeps
//! its own pins, and this stop prints an exact volume for every body).
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
//! The blades — swept and lofted alike — are the one place the plant
//! is FITTED rather than stated. A skin is a NURBS surface through
//! sampled stations, so a blade's walls are B-spline surfaces
//! interpolating exact points of an exact circular spine, not a
//! closed form of the swept or lofted solid. That is the price of
//! leaving the plane, and it is stated here rather than hidden. The
//! SEPALS' tangency to the globe is not fitted, though: the stand-off
//! is the section's own keel and the non-entry is a two-line argument
//! on [`sepals`], checked on the built solids by
//! [`review_probes::the_sepals_stand_outside_the_globe_they_are_tangent_to`].

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::f64::consts::PI;

use pncad::geom_brep::SurfaceKind;
use pncad::geom_core::{Affine3, Mat3, Point2, Point3, Vec2, Vec3};
use pncad::prelude::{Open, Start};
use pncad::profile::{ArcSweep, Center, ProfileLoop, ProfileVertex, SketchPlane, Via};
// The named gap below (`section_loops`): the raw loop door is kernel
// vocabulary, off the façade, so the one scene that needs it names the
// kernel crate directly.
use pncad::sweep::fillet::FilletError;
use pncad::sweep::{
    ExtrudeError, Extrusion, Revolution, RevolveAxis, TubeWindow, WedgeFrames, extrude, loft_body,
    revolve, revolved_caps, sweep_body, tube_along_arc,
};
use pncad::topo::{Body, BooleanError, BooleanOp, Operand, TransformError};
use profile::RawLoop;

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
/// exact; the profile is authored centre-first (`Center`) so the
/// zone's carrier is the sphere itself and not a fitted arc. That
/// centre-intent is now sayable in the algebra (LIB-G1 constructor 3):
/// the globe centre is authored, the winding is structural, and
/// equidistance of the two endpoints from the centre is CHECKED — this
/// profile derives both radii from the sphere, so it passes by
/// construction and would refuse loudly if it ever stopped doing so.
/// The lantern/bud **meridian**: the closed profile a flower or a
/// pre-tepal is revolved from, in sketch `(s, t)` with the axis along
/// `t`. Shared by [`lantern`] (revolved FULL, one body) and [`bud`]
/// (revolved PARTIAL, three bodies on three axes), so the bud's
/// segments are the same shape said three times and not a re-typed
/// near-copy.
fn meridian<S: Scalar>(
    globe: f64,
    top: f64,
    mouth: f64,
    lip_r: f64,
    lip_drop: f64,
) -> ProfileLoop<S> {
    let r_top = (globe.powi(2) - top.powi(2)).sqrt();
    let r_mouth = (globe.powi(2) - mouth.powi(2)).sqrt();
    let t_mouth = top + mouth;
    let t_end = t_mouth + lip_drop;
    Open.at(p2(0.0, 0.0))
        .line_to(p2(r_top, 0.0))
        .expect("lantern attachment disk")
        // The belly: the sphere's own arc about the globe centre,
        // swept the long way round the equator (Ccw in sketch (s, t)).
        .arc_to(Center {
            c: p2(0.0, top),
            winding: ArcSweep::Ccw,
            p: p2(r_mouth, t_mouth),
        })
        .expect("lantern belly rides the globe")
        .line_to(p2(lip_r, t_end))
        .expect("lantern pucker cone")
        .line_to(p2(0.0, t_end))
        .expect("lantern lip disk")
        .line_to(Start)
        .expect("lantern axis seam")
        .into()
}

fn lantern<S: Scalar>(
    attach: (f64, f64),
    dir: (f64, f64),
    globe: f64,
    top: f64,
    mouth: f64,
    lip_r: f64,
    lip_drop: f64,
) -> Body<S> {
    // Sketch frame: origin at the attachment point, v along the
    // flower axis (into the flower), u the in-plane radial.
    let plane = SketchPlane::from_frame(
        pt3(attach.0, 0.0, attach.1),
        v3(-dir.1, 0.0, dir.0),
        v3(dir.0, 0.0, dir.1),
    );
    revolve(
        &validated(plane, vec![meridian(globe, top, mouth, lip_r, lip_drop)])
            .expect("lily profile validates"),
        sketch_axis(),
        Revolution::Full,
    )
    .expect("lantern revolves")
    .body
}

/// A **bud**: three pre-tepals, each a PARTIAL revolve of the same
/// [`meridian`], on three axes that form a narrow TRIPOD about the
/// bud's own axis and are rolled so the wedges nest like a pinwheel.
///
/// This is what an unopened *Calochortus* actually is — not a small
/// version of the open flower, but three tepals not yet fused, each
/// wrapping past its neighbour. A full revolve can only say the
/// finished globe; three partial ones can say the tepals it is made
/// of, and the kernel has had this since the wedge door.
///
/// The three knobs and what they do:
///
/// - `tilt` — how far each segment's axis leans off the bud's own.
///   Zero puts all three on one axis and the bud is a globe with three
///   seams. The tripod is pivoted at the **attachment**: all three
///   segments share the point the pedicel enters, and the tilt splays
///   their TIPS apart — a bud on the turn, its three tepals starting
///   to part at the point while still held together at the neck. A
///   few degrees is plenty, because the splay is amplified by the
///   whole length of the pucker: what is a narrow tripod at the neck
///   is a visible parting at the tip.
/// - `lean` — WHICH WAY it leans, as an angle relative to that
///   segment's own position round the bud. At 0 each segment leans
///   outward along its own radius, which is a symmetric tripod and
///   reads as a three-pointed star. At a quarter turn it leans
///   SIDEWAYS, across its neighbour, and the result is chiral: that is
///   the pinwheel.
/// - `span` — the angular width of a segment. Above a third of a turn
///   the three overlap, which is what makes them nest rather than
///   merely abut.
///
/// Placement is a FRAME choice, not an argument: `revolve` takes its
/// axis in sketch coordinates ([`sketch_axis`]), so a tilted axis is
/// spelled by tilting the sketch plane. The segments overlap each
/// other on purpose and are not joined — gluing them is the same
/// curved-boolean wall the rest of the plant is stopped by (probes 2
/// and 7).
///
/// **Three, in the return type.** `plant` names the segments with a
/// `.zip(["lily_bud_a", …])`, which truncates silently against a
/// shorter `Vec` — a fourth segment would vanish from the manifest and
/// only show up as a smaller printed piece count. The arity is
/// therefore stated once, here, where a change to it is a compile
/// error at the call site.
#[allow(clippy::too_many_arguments)]
fn bud<S: Scalar>(
    attach: (f64, f64),
    dir: (f64, f64),
    globe: f64,
    top: f64,
    mouth: f64,
    lip_r: f64,
    lip_drop: f64,
    tilt: f64,
    lean: f64,
    span: f64,
) -> [Body<S>; 3] {
    let ax = (dir.0, 0.0, dir.1);
    let e1 = (-dir.1, 0.0, dir.0);
    let e2 = (0.0, 1.0, 0.0);
    let rad = |a: f64| {
        let (sa, ca) = (a.sin(), a.cos());
        (
            ca * e1.0 + sa * e2.0,
            ca * e1.1 + sa * e2.1,
            ca * e1.2 + sa * e2.2,
        )
    };
    let nrm = |(x, y, z): (f64, f64, f64)| {
        let l = (x.powi(2) + y.powi(2) + z.powi(2)).sqrt();
        (x / l, y / l, z / l)
    };
    core::array::from_fn(|i| {
        #[allow(clippy::cast_precision_loss)]
        let phi = 2.0 * PI * (i as f64) / 3.0;
        // The segment's own axis: the bud axis leaned `tilt` toward
        // the direction `lean` radians round from its own place.
        let l = rad(phi + lean);
        let (st, ct) = (tilt.sin(), tilt.cos());
        let a = nrm((
            ct * ax.0 + st * l.0,
            ct * ax.1 + st * l.1,
            ct * ax.2 + st * l.2,
        ));
        // The wedge STARTS half a span before the segment's
        // place — and then sweeps AWAY from it, not across it:
        // `revolve` turns right-handed about the sketch axis,
        // which in this frame (`e1 x e2` is MINUS the bud axis)
        // is the direction of decreasing `phi`. So the wedge
        // actually lands centred a full `span` short of `phi`,
        // measured and pinned in
        // `review_probes::the_buds_three_axes_form_the_authored_tripod`.
        // Nothing above depends on where it lands — the three
        // stay 120 degrees apart and the overlap is still
        // 3*span - 360 — but the CHIRALITY does: the lean read
        // off the realized centre is `lean + span`, still nowhere
        // near the achiral star, and still a pinwheel.
        //
        // Gram-Schmidt against the tilted axis, since that radial
        // is only perpendicular to the BUD's axis, not to this
        // segment's.
        let start = rad(phi - 0.5 * span);
        let d = start.0 * a.0 + start.1 * a.1 + start.2 * a.2;
        let u = nrm((start.0 - d * a.0, start.1 - d * a.1, start.2 - d * a.2));
        // All three share the ATTACHMENT: the tilt splays their
        // tips, not their bellies.
        let plane = SketchPlane::from_frame(
            pt3(attach.0, 0.0, attach.1),
            v3(u.0, u.1, u.2),
            v3(a.0, a.1, a.2),
        );
        revolve(
            &validated(plane, vec![meridian(globe, top, mouth, lip_r, lip_drop)])
                .expect("bud profile validates"),
            sketch_axis(),
            Revolution::Partial(S::from_f64(span)),
        )
        .expect("bud segment revolves")
        .body
    })
}

/// A leaf blade's cross-section: a KITE, i.e. the two sharp margins
/// at `±width/2` on a chord with a `ridge` above it and a `keel`
/// below. The two rises are DIFFERENT, so the blade is asymmetric
/// about its own chord exactly as the extruded crescent was.
#[derive(Clone, Copy, Debug)]
struct Kite {
    /// Chord length, margin to margin — the blade's width.
    width: f64,
    /// Rise above the chord.
    ridge: f64,
    /// Drop below the chord.
    keel: f64,
}

/// The long basal leaf's placement and spine, named rather than
/// inlined because
/// [`review_probes::the_lofted_blade_tapers_and_rolls_in_the_stored_geometry`]
/// builds a SECOND blade from the very same numbers with the twist set
/// to zero, and measures the roll as the angle between the two. A
/// re-typed copy of these numbers would let the two drift and the
/// measurement would quietly stop meaning anything.
const LEAF_A_BASE: (f64, f64, f64) = (0.04, 0.05, 0.03);
/// See [`LEAF_A_BASE`].
const LEAF_A_DIR: (f64, f64, f64) = (-0.72, 0.52, 0.16);
/// See [`LEAF_A_BASE`].
const LEAF_A_UP: (f64, f64, f64) = (0.0, 0.0, 1.0);
/// See [`LEAF_A_BASE`].
const LEAF_A_LEN: f64 = 5.10;
/// See [`LEAF_A_BASE`]. Negative: the blade arches OVER, which is what
/// a basal leaf lying along the ground does.
const LEAF_A_CURL: f64 = -0.62;

/// The long basal leaf's section plan: a rectangle where it meets the
/// stem, a wide flat diamond at the belly a fifth of the way out, and
/// a small diamond — never a point — at the tip. The roll is 160
/// degrees eased hard toward the tip (exponent 2.6), so the blade lies
/// flat for most of its length and does its turning near the end, the
/// way a real *Calochortus* leaf does.
fn leaf_a_plan() -> Plan {
    Plan {
        base: Section {
            width: 0.170,
            ridge: 0.028,
            keel: 0.020,
            shoulder: 1.0,
        },
        belly: Section {
            width: 0.420,
            ridge: 0.034,
            keel: 0.016,
            shoulder: 0.0,
        },
        belly_at: 0.22,
        tip: Section {
            width: 0.060,
            ridge: 0.010,
            keel: 0.006,
            shoulder: 0.0,
        },
        roll0: 0.0,
        twist: deg(160.0),
        twist_ease: 2.6,
    }
}

/// Stations along a leaf's swept spine, and the v-degree its skin is
/// fitted at (the swept-elbow corpus fixture's numbers).
const LEAF_STATIONS: usize = 9;
/// The leaf skin's fit degree along the path.
const LEAF_V_DEGREE: usize = 3;
/// Stations along the LOFTED long leaf. More than the swept blades
/// use, because a loft's stations carry the taper and the roll as well
/// as the spine: the skin interpolates the sections it is given and
/// chords between them, so a roll of half a turn wants stations dense
/// enough that no chord cuts a visible corner.
const LOFT_STATIONS: usize = 17;
/// Stations along a sepal — shorter blades, gentler roll.
const SEPAL_STATIONS: usize = 13;

/// A **keeled leaf blade**: a thin section carried along a gently
/// arching spine by [`sweep_body`] — the general-path sweep, not an
/// extrusion, so the blade leaves the plane it was drawn in.
///
/// The section is a [`Kite`] of four straight lines, and the spine
/// runs through its chord's midpoint, i.e. through the midrib.
///
/// **Restoring the lanceolate arcs is outstanding work on this
/// stop** — the kite is what the blade was given, not a limit of the
/// vocabulary — with one thing to check first: the span meter's half
/// of the rational bank is retired and the QUADRATURE half is not,
/// and every body in this stop prints an exact volume, so an
/// arc-walled blade may meet `QuadratureUnsupported` where the kite
/// does not.
///
/// Nothing here approximates a curve with a chord, meanwhile: a kite
/// is exactly a kite.
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
    section: Kite,
    curl: f64,
) -> Body<S> {
    let (d, v, u) = blade_frame(dir, up);
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
    let path = pncad::geom::NurbsCurve3::interpolate(&pts, 3).expect("the leaf spine interpolates");
    let place = SketchPlane::from_frame(
        pt3(base.0, base.1, base.2),
        v3(u.0, u.1, u.2),
        v3(v.0, v.1, v.2),
    )
    .placement;
    // The kite, wound counterclockwise in the sketch (s, t) frame:
    // margin, keel, margin, ridge.
    let loops: Vec<ProfileLoop<f64>> = vec![crate::paths::path_polygon(&[
        (-0.5 * section.width, 0.0),
        (0.0, -section.keel),
        (0.5 * section.width, 0.0),
        (0.0, section.ridge),
    ])];
    sweep_body::<S>(&loops, place, &path, LEAF_STATIONS, LEAF_V_DEGREE)
        .expect("the leaf sweeps along its spine")
        .body
}

// ---------------------------------------------------------------
// The LOFTED blade: the two things a sweep cannot say
// ---------------------------------------------------------------

/// A lofted blade's cross-section: the [`Kite`] with two extra pairs
/// of vertices — SHOULDERS — on the way from each margin to the ridge
/// and the keel.
///
/// The shoulder parameter is what lets one section list carry a
/// rectangle and a diamond at once. At `shoulder = 0` each shoulder
/// sits exactly on the straight line between its two neighbours, so
/// the outline IS the kite, said with eight segments instead of four.
/// At `shoulder = 1` it sits at the corner of the bounding rectangle,
/// so the outline IS the rectangle. Everything between is the eased
/// morph the leaf's base needs — a rectangle where it meets the stem,
/// a diamond a fifth of the way out.
///
/// Eight segments and not four because a loft matches segment `j` of
/// every section to segment `j` of every other
/// ([`pncad::sweep::LoftError`]'s `SectionShapeMismatch` arm): there
/// is no correspondence between a 4-gon's corners and a 4-gon's tips,
/// so the two shapes must be spelled on a common vertex budget. The
/// collinear vertices at `shoulder = 0` are exact, not approximate —
/// a midpoint of two authored points.
#[derive(Clone, Copy, Debug)]
struct Section {
    /// Chord length, margin to margin.
    width: f64,
    /// Rise above the chord.
    ridge: f64,
    /// Drop below the chord.
    keel: f64,
    /// 0 = kite, 1 = the bounding rectangle.
    shoulder: f64,
}

impl Section {
    /// The eight-vertex outline, wound counterclockwise in the sketch
    /// `(s, t)` frame from the `+s` margin.
    fn outline(self) -> Vec<ProfileLoop<f64>> {
        // The shoulder between tips `a` and `b`: their midpoint at
        // `shoulder = 0`, their vector sum (the rectangle corner) at 1.
        let shoulder = |a: (f64, f64), b: (f64, f64)| {
            let m = (0.5 * (a.0 + b.0), 0.5 * (a.1 + b.1));
            (m.0 + self.shoulder * m.0, m.1 + self.shoulder * m.1)
        };
        let right = (0.5 * self.width, 0.0);
        let ridge = (0.0, self.ridge);
        let left = (-0.5 * self.width, 0.0);
        let keel = (0.0, -self.keel);
        // NAMED GAP (LIB-RETTAIL, 2026-08-12) — the one place in the tour
        // the presented surface cannot say what the demo means, recorded
        // rather than worked around (main.rs's purpose block).
        //
        // At `shoulder = 0` the shoulder IS the midpoint of two tips, so
        // three consecutive vertices are EXACTLY collinear — deliberately,
        // because a loft matches segment j to segment j and the 4-tip and
        // 8-corner sections must be spelled on one vertex budget. The PATHS
        // lattice refuses that junction at authoring
        // (`JunctionTangent { margin: 0.0 }`), while `Profile::validate`
        // ACCEPTS it: collinear line/line is carrier IDENTITY, legal
        // undeclared (`ProfileLoop::tangent_joints`' normative semantics).
        // So the two junction rules disagree on same-carrier continuation,
        // and the lattice is the stricter one — a library finding, not a
        // demo defect. Until the lattice gains a same-carrier continuation
        // verb (vocabulary, out of this unit's fence), the PRESENTED
        // surface has no spelling for this loop at all: `ProfileLoop`'s
        // fields are sealed, so the only route left is the kernel's raw
        // door, `profile::RawLoop`, which `pncad::profile` deliberately
        // omits. That is why this crate carries a second kernel
        // dependency — the tour reaches around its own façade here, and
        // the gap is loud in the dependency graph instead of hidden in a
        // struct literal.
        let v = |(x, y): (f64, f64)| ProfileVertex::new(Point2::new(x, y), 0.0);
        vec![RawLoop::new(vec![
            v(right),
            v(shoulder(right, ridge)),
            v(ridge),
            v(shoulder(ridge, left)),
            v(left),
            v(shoulder(left, keel)),
            v(keel),
            v(shoulder(keel, right)),
        ])]
    }

    /// Linear blend, field by field.
    fn lerp(self, other: Self, s: f64) -> Self {
        let f = |a: f64, b: f64| a + (b - a) * s;
        Self {
            width: f(self.width, other.width),
            ridge: f(self.ridge, other.ridge),
            keel: f(self.keel, other.keel),
            shoulder: f(self.shoulder, other.shoulder),
        }
    }
}

/// How a lofted blade's section changes from attachment to tip, and
/// how it rolls on the way.
#[derive(Clone, Copy, Debug)]
struct Plan {
    /// The attachment section: a rectangle where the blade meets the
    /// stem (`shoulder = 1`).
    base: Section,
    /// The widest section — the blade's belly.
    belly: Section,
    /// Where the belly sits, as a fraction of the spine.
    belly_at: f64,
    /// The tip section. A smaller diamond, never a POINT — but the
    /// refusal is not the one the shape suggests. Setting `width` to
    /// 0 while the tip keeps its rises leaves the two margins on top
    /// of each other and the four-line section stops being SIMPLE:
    /// the loft refuses `Skin(SectionProfile { NonSimple { kind:
    /// Touch, .. } })`, naming the two segments that meet.
    /// `DegenerateSegment` is the OTHER collapse — `width`, `ridge`
    /// and `keel` all 0, the whole section gone to a point — and
    /// arrives only then. Both measured on this plan; neither is
    /// pinned by a test, because a wall probe here would have to
    /// build a whole second blade to reach it.
    tip: Section,
    /// Roll about the spine AT THE BASE, radians. The spine's bend
    /// and the blade's facing are independent choices, and this is
    /// what separates them: `curl` decides which way the spine arcs
    /// (toward the frame's `v`), `roll0` decides which way the blade
    /// faces once it gets there. A sepal wants to arc outward from
    /// the flower and still show its face to a viewer standing to one
    /// side, which is `curl` in the radial plane and `roll0` a quarter
    /// turn off it.
    roll0: f64,
    /// FURTHER roll from base to tip, radians.
    twist: f64,
    /// Roll easing exponent: the roll at fraction `s` of the spine is
    /// `twist * s^ease`. A real *Calochortus* leaf lies flat for most
    /// of its length and does its twisting near the tip, which is
    /// `ease` above 1.
    twist_ease: f64,
}

impl Plan {
    /// The section at fraction `s` of the spine.
    fn at(self, s: f64) -> Section {
        if s < self.belly_at {
            self.base.lerp(self.belly, s / self.belly_at)
        } else {
            self.belly
                .lerp(self.tip, (s - self.belly_at) / (1.0 - self.belly_at))
        }
    }
}

/// A **lofted blade**: the same arching circular spine the swept
/// [`leaf`] rides, but with a section that CHANGES station to station
/// and a frame that ROLLS about the spine — the two things
/// [`sweep_body`] cannot say, because it carries one rigid profile
/// along a frame it derives itself.
///
/// [`loft_body`] takes the sections and the placements as separate
/// lists, so both are ours to author: section `k` is `plan.at(s_k)`
/// and placement `k` is the spine frame at `s_k` rolled by
/// `plan.twist * s_k^twist_ease` about the spine's own tangent. The
/// walls are the same fitted skins the sweep produces — this is the
/// same verb underneath, asked a question with more room in it.
///
/// The spine is set up exactly as [`leaf`]'s: a circular arc of length
/// `len` turning through `curl` toward `up`, sampled at `stations`
/// exact points. Two live walls bound what this can be asked for: the
/// tip may not close to a point (a zero-width section is a degenerate
/// segment), and the spine may not turn past π — the loft's stacking
/// trilean is an END-TO-END statement, `cos(curl/2)` for a planar arc
/// spine, so past a half turn of total position stacking it refuses
/// `ReversedStacking` (its own filed frontier, #368).
/// `review_probes::the_spine_curl_wall_re_measured` pins both sides
/// of the curl wall (3.0 builds, 3.5 refuses typed).
fn lofted_blade<S: Scalar>(
    base: (f64, f64, f64),
    dir: (f64, f64, f64),
    up: (f64, f64, f64),
    len: f64,
    curl: f64,
    plan: Plan,
    stations: usize,
) -> Body<S> {
    try_lofted_blade(base, dir, up, len, curl, plan, stations)
        .expect("the lofted blade skins its own sections")
        .body
}

/// [`lofted_blade`] with the refusal surfaced instead of expected, so
/// the curl-wall probe (M8-14, #222 — `review_probes::
/// the_spine_curl_wall_re_measured`) can sweep the parameter and
/// state the measured frontier rather than a remembered one.
fn try_lofted_blade<S: Scalar>(
    base: (f64, f64, f64),
    dir: (f64, f64, f64),
    up: (f64, f64, f64),
    len: f64,
    curl: f64,
    plan: Plan,
    stations: usize,
) -> Result<pncad::sweep::Lofted<S>, pncad::sweep::LoftError> {
    let (d, v, u) = blade_frame(dir, up);
    let r = len / curl;
    let mut sections: Vec<Vec<ProfileLoop<f64>>> = Vec::with_capacity(stations);
    let mut places: Vec<Affine3<f64>> = Vec::with_capacity(stations);
    for k in 0..stations {
        #[allow(clippy::cast_precision_loss)]
        let s = (k as f64) / ((stations - 1) as f64);
        let a = curl * s;
        let (sa, ca) = (a.sin(), a.cos());
        // The spine point, and the (tangent, up) pair carried round
        // with it — the arc turns in the (d, v) plane about u, so u
        // itself is fixed and the roll below is the only other motion.
        let p = (
            base.0 + r * sa * d.0 + r * (1.0 - ca) * v.0,
            base.1 + r * sa * d.1 + r * (1.0 - ca) * v.1,
            base.2 + r * sa * d.2 + r * (1.0 - ca) * v.2,
        );
        let vk = (
            ca * v.0 - sa * d.0,
            ca * v.1 - sa * d.1,
            ca * v.2 - sa * d.2,
        );
        // The roll: turn (u, vk) about the tangent by the eased angle.
        let th = plan.roll0 + plan.twist * s.powf(plan.twist_ease);
        let (st, ct) = (th.sin(), th.cos());
        let uu = (
            ct * u.0 + st * vk.0,
            ct * u.1 + st * vk.1,
            ct * u.2 + st * vk.2,
        );
        let vv = (
            ct * vk.0 - st * u.0,
            ct * vk.1 - st * u.1,
            ct * vk.2 - st * u.2,
        );
        sections.push(plan.at(s).outline());
        places.push(
            SketchPlane::from_frame(
                pt3(p.0, p.1, p.2),
                v3(uu.0, uu.1, uu.2),
                v3(vv.0, vv.1, vv.2),
            )
            .placement,
        );
    }
    loft_body::<S>(&sections, &places, LEAF_V_DEGREE)
}

/// The three **sepals**, the feature that reads as *pulchellus*
/// rather than *albus*: long pointed blades that spread from high on
/// the globe and project past it instead of being hidden by it. They
/// are [`lofted_blade`]s for the reason the long leaf is — a sepal
/// TAPERS, and a sepal that did not taper would read as a strap.
///
/// # They meet the globe TANGENTIALLY, and provably never re-enter it
///
/// Two bodies that overlap are not a modelling error in this scene —
/// nothing here is joined, and the pedicel is deliberately set back
/// INSIDE the lantern so its end cap cannot z-fight. But a sepal that
/// merely *starts near* the flower and hopes to miss it is a fudge,
/// and this one does not have to be: the tangency is exact
/// arithmetic, and the staying-out is a two-line proof.
///
/// Write `G` for the globe centre, `R` for its radius, and take the
/// sepal's own outward normal
///
/// ```text
/// n(θ, φ) = cos θ · (−axis) + sin θ · rad(φ)
/// ```
///
/// at polar angle `theta` from the flower's upper pole. Then:
///
/// - the base sits at `G + (R + keel) · n`, so the blade's KEEL — its
///   deepest point below its own chord, and the only part of the
///   section on the globe's side — grazes the sphere exactly. Not
///   near it: the offset IS the keel the section declares;
/// - the spine leaves along `τ = sin θ · axis + cos θ · rad`, which is
///   perpendicular to `n` and therefore TANGENT to the sphere;
/// - `up` is `n`, so a POSITIVE `curl` bends the spine along `+n`,
///   away from the globe. The spine point at arc angle `a` is then
///   `G + ((R + keel) + r(1 − cos a))·n + r sin a·τ` with `n ⊥ τ`, so
///   its distance from `G` is
///   `√[((R + keel) + r(1 − cos a))² + (r sin a)²] ≥ R + keel`
///   for every `a`, the two summands being non-negative. The sepal is
///   outside the globe at its base and gets monotonically no closer.
///
/// The blade starts APPRESSED — at `roll0 = 0` its broad face lies
/// against the sphere, which is how a sepal sits — and its `twist`
/// then rolls the face outward toward the tip. That is a real
/// *Calochortus* habit and, not by accident, the one motion a swept
/// blade could not have been given.
///
/// **Three, in the return type**, for the reason [`bud`] states: the
/// naming `.zip` at the call site truncates silently against a shorter
/// `Vec`, so the arity is stated once and checked by the compiler.
#[allow(clippy::too_many_arguments)]
fn sepals<S: Scalar>(
    globe_center: (f64, f64, f64),
    axis: (f64, f64, f64),
    globe: f64,
    theta: f64,
    phase: f64,
    len: f64,
    curl: f64,
    plan: Plan,
) -> [Body<S>; 3] {
    // The two radials spanning the plane perpendicular to the flower
    // axis: the in-xz-plane one and ŷ.
    let e1 = (-axis.2, 0.0, axis.0);
    let e2 = (0.0, 1.0, 0.0);
    let (st, ct) = (theta.sin(), theta.cos());
    // The offset that makes the keel graze rather than pierce.
    let stand = globe + plan.base.keel;
    core::array::from_fn(|i| {
        #[allow(clippy::cast_precision_loss)]
        let phi = phase + 2.0 * PI * (i as f64) / 3.0;
        let (sp, cp) = (phi.sin(), phi.cos());
        let rad = (
            cp * e1.0 + sp * e2.0,
            cp * e1.1 + sp * e2.1,
            cp * e1.2 + sp * e2.2,
        );
        // n: the outward normal at (theta, phi). `-axis` is the
        // flower's upper pole, the axis pointing INTO the flower.
        let n = (
            ct * -axis.0 + st * rad.0,
            ct * -axis.1 + st * rad.1,
            ct * -axis.2 + st * rad.2,
        );
        // tau: the tangent there, running outward and down.
        let tau = (
            st * axis.0 + ct * rad.0,
            st * axis.1 + ct * rad.1,
            st * axis.2 + ct * rad.2,
        );
        let base = (
            globe_center.0 + stand * n.0,
            globe_center.1 + stand * n.1,
            globe_center.2 + stand * n.2,
        );
        lofted_blade::<S>(base, tau, n, len, curl, plan, SEPAL_STATIONS)
    })
}

/// A blade's local frame as three world vectors: the spine's start
/// tangent, the direction it curls toward, and the section's width
/// axis. See [`blade_frame`].
type BladeFrame = ((f64, f64, f64), (f64, f64, f64), (f64, f64, f64));

/// The right-handed `(d, v, u)` blade frame: `d` the spine's start
/// tangent, `v` the `up` vector Gram–Schmidt'd against it, and
/// `u = v x d`, so a sketch plane built on `(u, v)` has `d` for its
/// normal. Shared by [`leaf`] and [`lofted_blade`] so the swept and
/// lofted blades sit in the SAME frame — the difference between them
/// is the verb, not the placement.
fn blade_frame(dir: (f64, f64, f64), up: (f64, f64, f64)) -> BladeFrame {
    let nrm = |(x, y, z): (f64, f64, f64)| {
        let l = (x.powi(2) + y.powi(2) + z.powi(2)).sqrt();
        (x / l, y / l, z / l)
    };
    let d = nrm(dir);
    let dot = up.0 * d.0 + up.1 * d.1 + up.2 * d.2;
    let v = nrm((up.0 - dot * d.0, up.1 - dot * d.1, up.2 - dot * d.2));
    let u = (
        v.1 * d.2 - v.2 * d.1,
        v.2 * d.0 - v.0 * d.2,
        v.0 * d.1 - v.1 * d.0,
    );
    (d, v, u)
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

/// The BUD's globe radius and truncation height — much smaller than
/// the open flower's, because a bud is mostly taper.
const BUD_GLOBE: f64 = 0.125;
/// See [`BUD_GLOBE`].
const BUD_TOP: f64 = 0.113;

/// The main lantern's globe radius and the height above the globe
/// centre at which the attachment disk truncates it. Named because
/// the SEPALS stand on this sphere and must stand on the same one the
/// lantern is revolved from — see [`sepals`].
const FLOWER_GLOBE: f64 = 0.44;
/// See [`FLOWER_GLOBE`].
const FLOWER_TOP: f64 = 0.40;

const GREEN_STEM: [f64; 3] = [0.36, 0.52, 0.30];
const GREEN_LEAF: [f64; 3] = [0.44, 0.62, 0.34];
/// *C. pulchellus* is the YELLOW fairy lantern — clear lemon, not the
/// white of *albus* this scene used to wear.
const YELLOW_TEPAL: [f64; 3] = [0.95, 0.84, 0.32];
/// The sepals are greener than the petals and stay so: on a live
/// pulchellus they read as the yellow-green sheath the globe hangs in.
const GREEN_SEPAL: [f64; 3] = [0.72, 0.76, 0.36];

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

    // The main flower's attachment point and axis — the same two the
    // lantern below is built on, named once so the sepals hang on the
    // flower's own axis rather than a hand-copied one.
    let flower_attach = (
        at_flower.p.0 - 0.08 * at_flower.t.0,
        at_flower.p.1 - 0.08 * at_flower.t.1,
    );
    // The sepal plan: a narrow strap at the neck easing to a lance,
    // then tapering hard to a near-point. The roll is gentle and
    // spread evenly (ease 1.0) — a sepal curls, it does not corkscrew.
    let sepal_plan = Plan {
        base: Section {
            width: 0.105,
            ridge: 0.014,
            keel: 0.008,
            shoulder: 1.0,
        },
        belly: Section {
            width: 0.265,
            ridge: 0.017,
            keel: 0.008,
            shoulder: 0.0,
        },
        belly_at: 0.30,
        tip: Section {
            width: 0.036,
            ridge: 0.006,
            keel: 0.004,
            shoulder: 0.0,
        },
        // Appressed at the base (the face lies on the globe), rolling
        // its face outward toward the tip.
        roll0: 0.0,
        twist: deg(75.0),
        twist_ease: 1.3,
    };
    // The globe's own centre, derived from the two numbers the lantern
    // is built with rather than copied: `FLOWER_TOP` along the flower
    // axis from the attachment point. The sepals then stand on the
    // sphere the lantern actually has.
    //
    // `theta` must clear `acos(FLOWER_TOP / FLOWER_GLOBE)` = 24.6
    // degrees, the polar angle where the sphere is TRUNCATED by the
    // attachment disk: above that the sphere is not part of the body,
    // so a sepal standing there would be tangent to a surface that
    // is not there. 38 degrees puts them on the shoulder of the globe.
    // The BUD: three pre-tepals, not a small flower. A much smaller
    // globe and a much skinnier, longer pucker than the open lantern's
    // — an unopened Calochortus is mostly taper.
    let bud_attach = (
        at_bud.p.0 - 0.06 * at_bud.t.0,
        at_bud.p.1 - 0.06 * at_bud.t.1,
    );
    let bud_bodies: [Body<S>; 3] = bud(
        bud_attach,
        at_bud.t,
        BUD_GLOBE,
        BUD_TOP,
        0.095,
        0.014,
        0.22,
        // 5 degrees of splay, leaning a quarter turn round from each
        // segment's own place (the chiral choice — see `bud`), and a
        // 156-degree span so the three overlap by 108 degrees in total
        // and genuinely nest rather than merely abut.
        deg(5.0),
        deg(90.0),
        deg(156.0),
    );

    let sepal_bodies: [Body<S>; 3] = sepals(
        (
            flower_attach.0 + FLOWER_TOP * at_flower.t.0,
            0.0,
            flower_attach.1 + FLOWER_TOP * at_flower.t.1,
        ),
        (at_flower.t.0, 0.0, at_flower.t.1),
        FLOWER_GLOBE,
        // Stand them as high on the globe as a tangency can go: the
        // attachment disk TRUNCATES the sphere at
        // acos(FLOWER_TOP / FLOWER_GLOBE) = 24.6 degrees, and above
        // that there is no sphere to be tangent to. Four degrees below
        // the truncation puts the sepal bases on the shoulder right
        // beside the rim the pedicel enters through — as close to
        // meeting the stem as a blade standing on this sphere can be.
        (FLOWER_TOP / FLOWER_GLOBE).acos() + deg(4.0),
        // Half a turn of phase. Without it sepal 0's radial is `e1`,
        // which for this flower's axis points almost straight at the
        // second lantern 1.44 m away — and a 1.05 m sepal reaches it,
        // ending up 0.027 m INSIDE that globe with 7530 mesh points
        // to spare. Nothing in the kernel objects: these are separate
        // bodies and this scene joins none of them, so a body passing
        // through another is not an error the kernel could raise —
        // which is exactly why the scene has to check. Turning the
        // triple by pi sends sepal 0 the other way.
        deg(180.0),
        1.05,
        0.40,
        sepal_plan,
    );

    let mut pieces = vec![
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
            color: YELLOW_TEPAL,
            // Set back 0.08 along the stem's own tangent so the
            // pedicel tip is INSIDE the flower: two bodies sharing a
            // plane would z-fight, and gluing them is probe 1.
            body: lantern(
                flower_attach,
                at_flower.t,
                FLOWER_GLOBE,
                FLOWER_TOP,
                0.36,
                0.09,
                0.16,
            ),
            caps: None,
        },
        Piece {
            name: "lily_leaf_a",
            color: GREEN_LEAF,
            // THE LOFTED LEAF: the one blade that tapers and twists.
            // It is the long basal leaf a real Calochortus lays along
            // the ground, and it does what those do near the tip —
            // rolls most of a half turn, so the blade you were looking
            // at edge-on you end up looking at face-on. Neither motion
            // is expressible as a sweep (probe 9): the sweep carries
            // ONE profile along a frame it derives itself.
            body: lofted_blade(
                LEAF_A_BASE,
                LEAF_A_DIR,
                LEAF_A_UP,
                LEAF_A_LEN,
                LEAF_A_CURL,
                leaf_a_plan(),
                LOFT_STATIONS,
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
                Kite {
                    width: 0.170,
                    ridge: 0.015,
                    keel: 0.007,
                },
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
                Kite {
                    width: 0.140,
                    ridge: 0.013,
                    keel: 0.006,
                },
                -0.35,
            ),
            caps: None,
        },
    ];
    // The bud's three pre-tepals, then the three sepals, each named in
    // order round its own axis so the manifest and the export stems
    // stay stable.
    for (name, body) in ["lily_bud_a", "lily_bud_b", "lily_bud_c"]
        .into_iter()
        .zip(bud_bodies)
    {
        pieces.push(Piece {
            name,
            color: GREEN_SEPAL,
            body,
            caps: None,
        });
    }
    // The three sepals, named in order round the flower axis so the
    // manifest and the export stems stay stable.
    for (name, body) in ["lily_sepal_a", "lily_sepal_b", "lily_sepal_c"]
        .into_iter()
        .zip(sepal_bodies)
    {
        pieces.push(Piece {
            name,
            color: GREEN_SEPAL,
            body,
            caps: None,
        });
    }
    pieces
}

/// The tour stop.
pub fn stops() -> Vec<Stop> {
    let pieces = plant::<f64>();
    let note = format!(
        "{} closed solids: 3 torus-segment stem tubes said in WORLD \
         coordinates (centre/axis/u_ref/radii stored exactly as \
         given), 1 sphere-zone lantern with a conical mouth, 3 \
         PARTIAL revolves of that same meridian forming the bud's \
         tripod of pre-tepals, 2 SWEPT \
         keeled blades (one kite section carried along an arching \
         NURBS spine), and 4 LOFTED ones — the long basal leaf and \
         the three sepals — which taper AND roll, the two things a \
         sweep cannot be asked for. The long leaf runs rectangle at \
         the stem to wide diamond to small diamond, turning 160 \
         degrees about its own spine on the way, eased toward the tip. \
         The sepals stand TANGENT to the globe: the stand-off is the \
         section's own keel, and no vertex of any sepal is inside the \
         sphere. The five analytic bodies approximate nothing — torus, \
         sphere, cone and plane exactly, parameters included; the \
         blades are fitted skins, the price of leaving the plane. \
         Nothing is JOINED, the leaf to its own sheath least of all: \
         see the wall probes.",
        pieces.len()
    );
    vec![Stop {
        name: "lily",
        caption: "fairy lantern (Calochortus pulchellus)".to_string(),
        montage: true,
        story: "a nodding yellow fairy lantern — arching stem, two closed \
                globular lantern with three spreading sepals tangent to the \
                globe, a bud of three nested pre-tepals on a tripod of axes, \
                one long tapering twisted basal leaf and two shorter \
                untapered ones; torus/sphere/cone/plane exact to the stored \
                parameter, blades skinned out of plane",
        ops: "Turtle-walked G1 arc chain -> tube_along_arc(world centre/axis/ \
              u_ref/radii, windowed) tubes; revolve(Full) sphere-zone \
              lantern and revolve(Partial) x3 on a tilted tripod for the \
              bud; sweep_body(kite section, arched NURBS spine) for the \
              two short leaves; loft_body(rectangle -> diamond -> diamond \
              sections on rolled placements) for the long leaf and the sepals",
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
        .arc_to(Center {
            c: p2(0.0, 0.0),
            winding: ArcSweep::Ccw,
            p: p2(0.0, r),
        })
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
    crate::walls::wall("lily", n, what, outcome, pinned, retire);
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
            .arc_to(Via {
                q: p2(0.5, 0.12),
                p: p2(1.0, 0.0),
            })
            .expect("probe leaf outer arc")
            .arc_to(Via {
                q: p2(0.5, 0.02),
                p: Start,
            })
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
    //    but the battery refuses the seam before any door is
    //    reached: its two supports meet tangentially.
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
    // 8. The leaf wants to GROW OUT OF the stem, not merely start
    //    beside it: a sheath of rectangular section leaving the stem
    //    and turning into the blade's own base angle, then joined
    //    there. That contact is as clean as a contact gets — the two
    //    bodies are authored on the SAME rectangle in the same plane,
    //    so the mate is exact rather than tolerated, and
    //    `flush_declarations` finds and declares it.
    //
    //    It still refuses, and the refusal is the interesting part: it
    //    names a CURVED EDGE, not the contact. Both operands are
    //    skinned bodies whose wall-wall seams are NURBS iso-curves,
    //    and the boolean lane is planar-complete with curved work
    //    wired per germ class. The declared conformal join is C7
    //    (CONTACT-DESIGN, ratified #178) and is M8's one row in the
    //    modeling-verb register, with this rebuild named as its
    //    consumer — so this wall is not a gap in the plan, it is the
    //    plan, probed.
    let sheath = {
        let base_section = leaf_a_plan().base;
        // Runs back out of the leaf's base along -dir, so its bottom
        // cap IS the leaf's bottom cap: same plane, same rectangle,
        // opposite outward normals.
        lofted_blade::<S>(
            LEAF_A_BASE,
            (-LEAF_A_DIR.0, -LEAF_A_DIR.1, -LEAF_A_DIR.2),
            LEAF_A_UP,
            0.34,
            0.85,
            Plan {
                base: base_section,
                belly: base_section,
                belly_at: 0.5,
                tip: base_section,
                roll0: 0.0,
                twist: 0.0,
                twist_ease: 1.0,
            },
            9,
        )
    };
    wall(
        8,
        "graft the leaf's sheath onto its blade at their shared, DECLARED rectangle",
        crate::booleans::try_union_declared(by("lily_leaf_a"), &sheath),
        // The KIND is the claim, as in wall 1: a curved EDGE stops
        // this, not a curved face and not the planar contact. If this
        // ever starts refusing on the contact instead, the sentence
        // above is wrong and must be re-derived before it is believed.
        |e| {
            matches!(
                e,
                BooleanError::CurvedEdgeUnsupported {
                    operand: Operand::A,
                    ..
                }
            )
        },
        "grow the leaves out of the stem instead of standing them beside it",
    );

    println!(
        "   (wall 9 — a TAPERING SWEEP — is still an ABSENCE rather than a \
         refusal, so it cannot be probed at runtime: `sweep_body` takes ONE \
         profile and derives its own frame, so there is no argument in which \
         a taper or a roll could be asked for and refused. What changed is \
         that the shapes it named are no longer out of reach — `lily_leaf_a` \
         and the three sepals are `loft_body` calls that taper AND roll, \
         because a loft takes the sections and the placements as separate \
         lists and both are the author's. The absence is now exactly the \
         one-op convenience: taper along a path-following frame, without \
         hand-placing every station. Walls 10 and 11 CLOSED with M6-3 — \
         `sweep::sweep_body` is the general-path sweep body and \
         `sweep::loft_body` the skin assembly, and this scene builds two \
         swept blades and four lofted ones live. Wall 10's closure was only \
         PARTIAL until #207: every curved path refused at assembly on the \
         skin fit's synthesized weight channel, so the general-path sweep \
         had no successful caller until that fix.)"
    );
}

// ---------------------------------------------------------------
// Review probes (PR #175 adversarial review, `review/lily`): the
// G1/placement claims checked against the STORED geometry, not the
// construction code, plus the finding-13 tessellation table
// re-measured. Kept as tests so a silent placement regression
// (finding 11: sign/handedness errors produce a valid solid in the
// wrong place) fails loud here.
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

    /// One of the tube's two JOINT FRAMES passes through world point
    /// `p` (xz-plane) with normal parallel to `t` — i.e. the tube's
    /// end tangent THERE is `t`.
    ///
    /// The frames come from `sweep::revolved_caps` (LIB-U5). Which of
    /// the two ends answers is the revolve's business, so both are
    /// offered — that
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
        // The bud is three partial revolves on three tilted axes, so
        // its claim is a different one and lives in
        // `the_buds_three_axes_form_the_authored_tripod`.
        for (name, t, cen, rad) in [("lily_lantern", T2, SPHERE1_C, FLOWER_GLOBE)] {
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
    /// the same surface and splits it the same way. The two SWEPT
    /// blade rows are the other half of the finding: a swept skin over
    /// a 4-vertex section costs three orders of magnitude less than a
    /// torus tube at the same δ, because the torus lane spends its
    /// budget on the RING and not on the tube.
    ///
    /// The LOFTED bodies are deliberately absent from this table. A
    /// loft's wall count and knot structure follow the section list
    /// and the station count rather than one profile, so pinning their
    /// triangle counts here would pin the demo's proportions, not a
    /// tessellation finding — and those proportions are chosen to look
    /// like a plant and will be re-chosen. What IS pinned about them
    /// is geometric: see the taper/roll and sepal-tangency tests.
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
        //
        // `lily_leaf_a` is NOT in this list any more, and its absence
        // is the point: it is the lofted blade, and it both tapers and
        // rolls. Pappus wants a rigid section carried in the normal
        // frame, which is exactly what a loft stops being. What pins
        // the lofted blade instead is
        // `the_lofted_blade_tapers_and_rolls_in_the_stored_geometry`.
        let blades: [(&str, f64, f64, f64, f64, f64); 2] = [
            ("lily_leaf_b", 0.170, 0.015, 0.007, 1.25, 0.40),
            ("lily_leaf_c", 0.140, 0.013, 0.006, 0.95, 0.35),
        ];
        for (name, w, ridge, keel, len, curl) in blades {
            let area = 0.5 * w * (ridge + keel);
            let pappus = area * curl.mul_add((ridge - keel) / 3.0, len);
            let m = pncad::mesh::tessellate(body(&ps, name), 2e-3).expect("tessellate");
            let rel = ((signed_volume(&m) - pappus) / pappus).abs();
            assert!(rel > 1e-5 && rel < 5e-5, "{name}: rel {rel}");
        }
    }

    /// The lofted blade's two claims — TAPER and ROLL — read off the
    /// STORED body, not off the construction code.
    ///
    /// Both are read through the body's two planar cap faces, which
    /// are the loft's end sections placed in the world. For each cap:
    /// the vertices lying on it, their centroid, the width axis `u`
    /// (the direction of the two farthest-apart vertices, which for
    /// both a rectangle-with-midpoints and a kite-with-midpoints are
    /// the two MARGINS, the section being far wider than it is thick),
    /// and `v = n x u`.
    ///
    /// The section's ridge/keel ASYMMETRY is what makes the roll
    /// measurable at all: `v`'s sign is otherwise a coin flip, because
    /// a `u` read off a farthest-pair is only defined up to sign. The
    /// ridge is authored strictly deeper than the keel, so the signed
    /// extent along `v` picks the orientation out uniquely, and the
    /// angle from the base frame to the tip frame is then a full
    /// signed turn rather than a turn mod pi.
    #[test]
    fn the_lofted_blade_tapers_and_rolls_in_the_stored_geometry() {
        let ps = pieces();
        let b = body(&ps, "lily_leaf_a");
        let caps = cap_frames(b);
        assert_eq!(caps.len(), 2, "a lofted blade has exactly two caps");
        // The BASE cap is the wide one (the rectangle at the stem);
        // the TIP cap the narrow one.
        let (base, tip) = if caps[0].width > caps[1].width {
            (&caps[0], &caps[1])
        } else {
            (&caps[1], &caps[0])
        };
        // TAPER. The authored base rectangle is 0.170 margin to margin
        // with rises 0.028/0.020, so its farthest pair is the diagonal
        // hypot(0.170, 0.048) = 0.17665; the authored tip diamond is
        // 0.060 margin to margin, and its farthest pair IS that chord
        // (0.060 > hypot(0.010, 0.006)). A sweep could produce neither
        // number from the other — one profile goes down the path.
        assert!(
            (base.width - 0.176_646_5).abs() < 1e-6,
            "base cap width {}",
            base.width
        );
        assert!(
            (tip.width - 0.060).abs() < 1e-9,
            "tip cap width {}",
            tip.width
        );
        // ROLL, isolated. Comparing the blade's own two ends does NOT
        // measure the roll: the caps live in different planes (the
        // spine turns through `curl` between them), so any angle read
        // across them mixes the roll with the spine's own turn, and
        // the base cap's farthest pair is a rectangle DIAGONAL rather
        // than its width axis, tilting the frame a further 15.8
        // degrees. Both effects vanish if the comparison is made
        // against the SAME blade built with the twist set to zero:
        // identical spine, identical stations, identical sections, so
        // the two tip caps are coplanar and their frames differ by the
        // roll and nothing else.
        let untwisted = lofted_blade::<f64>(
            LEAF_A_BASE,
            LEAF_A_DIR,
            LEAF_A_UP,
            LEAF_A_LEN,
            LEAF_A_CURL,
            Plan {
                twist: 0.0,
                ..leaf_a_plan()
            },
            LOFT_STATIONS,
        );
        let flat = cap_frames(&untwisted);
        let flat_tip = if flat[0].width > flat[1].width {
            &flat[1]
        } else {
            &flat[0]
        };
        assert!(
            cross_norm(flat_tip.n, tip.n) < 1e-12,
            "the twin's tip cap must be coplanar with the blade's"
        );
        // The SIGN is part of the claim, not an accident: the
        // ridge/keel asymmetry above fixes each `v` uniquely, so this
        // is a full signed turn and a MIRROR-rolled blade (twist
        // -160) must not pass. Asserting `turn.abs()` would let one
        // through; the authored twist is +160, and that is what the
        // stored geometry has to say.
        let turn = signed_angle(flat_tip.v, tip.v, tip.n);
        let want = deg(160.0);
        assert!(
            (turn - want).abs() < 1e-9,
            "blade roll {turn} rad, wanted {want} (the authored twist, SIGN included)"
        );
    }

    /// The sepals' TANGENCY claim, checked against the globe the
    /// lantern actually stores: every vertex of every sepal is at
    /// least `FLOWER_GLOBE` from the globe centre, so no sepal enters
    /// the flower. The doc comment on [`sepals`] argues this for the
    /// SPINE; this measures it on the built solid, section thickness
    /// and all.
    #[test]
    fn the_sepals_stand_outside_the_globe_they_are_tangent_to() {
        let ps = pieces();
        // The globe centre, re-derived from the lantern's own stored
        // sphere rather than from the plant's construction.
        let lant = body(&ps, "lily_lantern");
        let mut centre = None;
        for (_, f) in lant.faces() {
            if let Some(pncad::geom::Surface::Sphere { center, radius, .. }) =
                lant.get_surface(f.surface)
            {
                assert!((radius - FLOWER_GLOBE).abs() < 1e-12);
                centre = Some(*center);
            }
        }
        let g = centre.expect("the lantern stores its globe");
        let mut closest = f64::INFINITY;
        for name in ["lily_sepal_a", "lily_sepal_b", "lily_sepal_c"] {
            let s = body(&ps, name);
            for (_, v) in s.vertices() {
                let p = s.get_point(v.point).expect("vertex point");
                let d = ((p.x - g.x).powi(2) + (p.y - g.y).powi(2) + (p.z - g.z).powi(2)).sqrt();
                // 1e-12, not 0: the base keel vertex is placed AT the
                // sphere by construction, and it gets there through a
                // rotation composed into the section placement, so it
                // lands within an ulp or two of R rather than on it.
                // MEASURED: the nearest of the three sepals' vertices
                // sits 5.551e-17 inside R — one ulp of 0.44 exactly —
                // so this is a bound with four orders of headroom,
                // not a number fitted to the result. Anything that
                // actually entered the flower would be inside by a
                // section thickness, six orders of magnitude past
                // this.
                assert!(
                    d >= FLOWER_GLOBE - 1e-12,
                    "{name}: a vertex is {d} from the globe centre, inside R = {FLOWER_GLOBE}"
                );
                closest = closest.min(d);
            }
        }
        // Two-sided: the sepals must GRAZE, not merely miss. The base
        // section's keel is authored 0.008 and the stand-off is
        // R + keel, so the nearest vertex sits at R + 0 (the keel
        // vertex itself, on the sphere) — within the float noise of a
        // rotation composed through the placement.
        //
        // 1e-12, the SAME window as the inside bound above, so the
        // pair is a symmetric ±1e-12 collar on R and the claim "the
        // nearest sepal vertex is within 1e-12 of the globe" is the
        // literal assertion rather than a summary of it. This was
        // 1e-9 and reading as a floor; it is not one. The measured
        // margin is a single ulp (see above), so the collar could be
        // four orders tighter still — 1e-12 is where it stops being
        // a claim about the plant and starts being a claim about
        // float arithmetic.
        assert!(
            closest < FLOWER_GLOBE + 1e-12,
            "nearest sepal vertex {closest} — tangency claimed, {} of clearance found",
            closest - FLOWER_GLOBE
        );

        // And the OTHER flower. Tangency to the globe a sepal stands
        // on says nothing about the bud 1.44 m up the stem, and the
        // kernel will not say anything either: these are separate
        // bodies, this scene joins none of them, so one solid passing
        // through another is not a condition any operation here could
        // refuse. It is the SCENE's invariant, so the scene tests it.
        // Sepal 0's radial, unphased, points almost exactly at the bud
        // and a 1.05 m sepal reaches 0.027 m inside it — this assert
        // is what caught that, and reds if the phase is dropped.
        for seg in ["lily_bud_a", "lily_bud_b", "lily_bud_c"] {
            let (bc, br) = sphere_of(body(&ps, seg));
            for name in ["lily_sepal_a", "lily_sepal_b", "lily_sepal_c"] {
                let sb = body(&ps, name);
                for (_, v) in sb.vertices() {
                    let p = sb.get_point(v.point).expect("vertex point");
                    let d =
                        ((p.x - bc.x).powi(2) + (p.y - bc.y).powi(2) + (p.z - bc.z).powi(2)).sqrt();
                    assert!(
                        d > br,
                        "{name}: a vertex is {d} inside {seg}'s globe R = {br}"
                    );
                }
            }
        }
    }

    /// The bud's TRIPOD, read off the three segments' stored spheres.
    ///
    /// Three claims, none re-derived from the construction code: each
    /// segment's axis makes the authored `tilt` with the bud's own;
    /// the three are spaced a third of a turn apart around it; and all
    /// three pass through the one attachment point, which is what
    /// makes the tips splay rather than the bellies. The attachment is
    /// derived from stored data alone — the sphere centre sits `top`
    /// along the segment's own axis from it.
    #[test]
    fn the_buds_three_axes_form_the_authored_tripod() {
        let ps = pieces();
        let bud_axis = Vec3::new(T3.0, 0.0, T3.1);
        let segs: Vec<(Point3<f64>, Vec3<f64>)> = ["lily_bud_a", "lily_bud_b", "lily_bud_c"]
            .into_iter()
            .map(|n| {
                let b = body(&ps, n);
                let mut found = None;
                for (_, f) in b.faces() {
                    if let Some(pncad::geom::Surface::Sphere { center, axis, .. }) =
                        b.get_surface(f.surface)
                    {
                        // Stored axes may point either way along the
                        // line; orient them all into the bud.
                        let a = if axis.dot(bud_axis) < 0.0 {
                            -*axis
                        } else {
                            *axis
                        };
                        found = Some((*center, a));
                    }
                }
                found.expect("a bud segment stores its globe")
            })
            .collect();
        for (i, (_, a)) in segs.iter().enumerate() {
            let c = a.dot(bud_axis);
            assert!(
                (c - deg(5.0).cos()).abs() < 1e-12,
                "bud segment {i}: axis leans {} rad, wanted {}",
                c.acos(),
                deg(5.0)
            );
        }
        // A third of a turn apart, measured across the bud axis.
        let across = |a: Vec3<f64>| {
            let p = a - bud_axis * a.dot(bud_axis);
            p / p.norm()
        };
        for (i, j) in [(0, 1), (1, 2), (2, 0)] {
            let c = across(segs[i].1).dot(across(segs[j].1));
            assert!(
                (c - deg(120.0).cos()).abs() < 1e-12,
                "bud segments {i},{j}: {} rad apart round the axis",
                c.acos()
            );
        }
        // And all three axes pass through ONE attachment point.
        let neck = |(c, a): (Point3<f64>, Vec3<f64>)| c - a * BUD_TOP;
        let n0 = neck(segs[0]);
        for (i, seg) in segs.iter().enumerate().skip(1) {
            let d = (neck(*seg) - n0).norm();
            assert!(d < 1e-12, "bud segment {i}'s neck is {d} from segment 0's");
        }
        // CHIRALITY, which none of the above pins. Tilt, spacing and
        // the shared neck are all LEAN-INVARIANT: the achiral star
        // (`lean` 0, every segment leaning outward along its own
        // radius) satisfies all three exactly as the pinwheel does,
        // because three axes 120 degrees apart look the same however
        // far round they have been carried from the places they
        // belong to. The lean is the angle BETWEEN those two, so it
        // needs the segment's own place — which is stored, as the
        // centre of its wedge: a partial revolve leaves two cap
        // half-planes, and the span is centred on the place.
        //
        // The caps are the planar faces CONTAINING the segment's axis
        // (the mouth's annulus is planar too, but its normal lies
        // ALONG the axis). Each cap's half-plane direction is
        // `n x axis` up to sign, and the sign is settled by asking
        // which way the cap's own vertices lie.
        for (i, name) in ["lily_bud_a", "lily_bud_b", "lily_bud_c"]
            .into_iter()
            .enumerate()
        {
            let b = body(&ps, name);
            let a = segs[i].1;
            let mut halves: Vec<Vec3<f64>> = Vec::new();
            for (_, f) in b.faces() {
                let Some(&pncad::geom::Surface::Plane { origin, normal, .. }) =
                    b.get_surface(f.surface)
                else {
                    continue;
                };
                if normal.dot(a).abs() > 0.5 {
                    continue;
                }
                let h = normal.cross(a);
                let h = h / h.norm();
                let s: f64 = b
                    .vertices()
                    .filter_map(|(_, v)| b.get_point(v.point).copied())
                    .filter(|p| (*p - origin).dot(normal).abs() < 1e-9)
                    .map(|p| (p - n0).dot(h))
                    .sum();
                halves.push(if s < 0.0 { -h } else { h });
            }
            assert_eq!(halves.len(), 2, "bud segment {i}: two wedge caps");
            // The two half-planes ARE the authored span apart, which
            // is what makes their bisector the segment's own place.
            let sp = halves[0].dot(halves[1]).acos();
            assert!(
                (sp - deg(156.0)).abs() < 1e-9,
                "bud segment {i}: wedge spans {sp} rad"
            );
            let place = halves[0] + halves[1];
            let place = place / place.norm();
            // Read both across the BUD's axis and take the signed
            // angle from the place to the lean. Sign and magnitude
            // are both the claim: 0 would be the star, and the
            // opposite sign the mirror-image pinwheel.
            let across = |v: Vec3<f64>| {
                let p = v - bud_axis * v.dot(bud_axis);
                p / p.norm()
            };
            let lean = signed_angle(across(place), across(a), bud_axis);
            // NEGATIVE ninety, for the authored quarter turn: `bud`
            // measures its lean in the sketch frame (e1, e2), whose
            // e1 x e2 is MINUS the bud axis, so a positive quarter
            // turn there reads as a negative one about the axis
            // itself. The handedness is the point; the frame it is
            // spelled in is not.
            // What that lean HAS to be, from the authored numbers and
            // nothing else. Two sign reversals sit between the
            // authored quarter turn and this reading, and both are
            // real properties of the stored bud rather than
            // bookkeeping:
            //
            //   * the wedge sweeps AWAY from the place it starts half
            //     a span before (see `bud`), landing centred a full
            //     `span` short of it, so the lean read off the
            //     realized centre is `lean + span` = 246 degrees;
            //   * that angle is measured here about the BUD's axis,
            //     and the frame `bud` spells its angles in is
            //     left-handed about it, so 246 reads as -246, i.e.
            //     +114.
            //
            // The window is 0.05 degrees around it because the wedge
            // lives in the segment's OWN tilted plane and is being
            // read across the bud's: at 5 degrees of tilt that
            // distortion is 0.037 degrees (measured), and it is the
            // only slack here. The three rival arrangements are
            // nowhere near it — the achiral star reads -156, the
            // mirror-image pinwheel -66 — so this is the handedness,
            // pinned.
            let want = deg(360.0) - deg(90.0) - deg(156.0);
            assert!(
                (lean - want).abs() < deg(0.05),
                "bud segment {i}: leans {} deg off its own place, wanted {} \
                 (-156 would be the achiral star, -66 the mirror)",
                lean.to_degrees(),
                want.to_degrees()
            );
        }
    }

    /// The single stored sphere of a body: (centre, radius).
    fn sphere_of(b: &Body<f64>) -> (Point3<f64>, f64) {
        for (_, f) in b.faces() {
            if let Some(pncad::geom::Surface::Sphere { center, radius, .. }) =
                b.get_surface(f.surface)
            {
                return (*center, *radius);
            }
        }
        panic!("body stores no sphere")
    }

    /// One planar cap of a lofted blade, reduced to the numbers the
    /// taper/roll test reads. See that test for why each is taken.
    struct Cap {
        /// The cap plane's stored unit normal.
        n: Vec3<f64>,
        /// Distance between the two farthest-apart vertices on it.
        width: f64,
        /// The in-plane axis perpendicular to the width, signed so it
        /// points from the chord toward the RIDGE (the deeper of the
        /// section's two rises).
        v: Vec3<f64>,
    }

    /// Both caps of a lofted blade, each reduced to a [`Cap`].
    fn cap_frames(b: &Body<f64>) -> Vec<Cap> {
        let mut out = Vec::new();
        for (_, f) in b.faces() {
            let Some(&pncad::geom::Surface::Plane { origin, normal, .. }) =
                b.get_surface(f.surface)
            else {
                continue;
            };
            // The vertices ON this plane.
            let on: Vec<Point3<f64>> = b
                .vertices()
                .filter_map(|(_, v)| b.get_point(v.point).copied())
                .filter(|p| (*p - origin).dot(normal).abs() < 1e-9)
                .collect();
            assert_eq!(on.len(), 8, "a blade section has eight vertices");
            // Farthest pair -> the width and its axis.
            let mut width = 0.0;
            let mut axis = Vec3::new(0.0, 0.0, 0.0);
            for (i, a) in on.iter().enumerate() {
                for bp in &on[i + 1..] {
                    let d = (*bp - *a).norm();
                    if d > width {
                        width = d;
                        axis = (*bp - *a) / d;
                    }
                }
            }
            // v completes the frame; its SIGN is fixed by asking which
            // side the deeper rise (the ridge) is on.
            let mut v = normal.cross(axis);
            let c = on.iter().fold(Vec3::new(0.0, 0.0, 0.0), |acc, p| {
                acc + (*p - Point3::new(0.0, 0.0, 0.0))
            }) / 8.0;
            let centroid = Point3::new(c.x, c.y, c.z);
            let hi = on
                .iter()
                .map(|p| (*p - centroid).dot(v))
                .fold(f64::NEG_INFINITY, f64::max);
            let lo = on
                .iter()
                .map(|p| (*p - centroid).dot(v))
                .fold(f64::INFINITY, f64::min);
            if hi < -lo {
                v = -v;
            }
            out.push(Cap {
                n: normal,
                width,
                v,
            });
        }
        out
    }

    /// The signed angle from `a` to `b` about `axis` (all unit, `a`
    /// and `b` perpendicular to `axis` up to float noise).
    fn signed_angle(a: Vec3<f64>, b: Vec3<f64>, axis: Vec3<f64>) -> f64 {
        let s = a.cross(b).dot(axis);
        let c = a.dot(b);
        s.atan2(c)
    }

    /// **The spine curl wall, pinned from both sides.** Through π the
    /// loft builds; past spine turn π it refuses `ReversedStacking`,
    /// because the stacking trilean is an END-TO-END statement (mean
    /// last-section displacement against the first section's normal —
    /// for a planar arc spine that is `cos(curl/2)`, negative past π),
    /// not a per-slab one. That wall is filed as its own frontier
    /// (#368); if either side of this pin moves, re-derive the
    /// `lofted_blade` prose with it.
    #[test]
    fn the_spine_curl_wall_re_measured() {
        // Through π the blade builds.
        for curl in [0.45, 1.0, 2.0, 2.5, 2.8, 3.0] {
            let out = try_lofted_blade::<f64>(
                LEAF_A_BASE,
                LEAF_A_DIR,
                LEAF_A_UP,
                LEAF_A_LEN,
                -curl,
                leaf_a_plan(),
                LOFT_STATIONS,
            );
            assert!(
                out.is_ok(),
                "curl {curl} rad refused ({:?}) — the span-meter wall is BACK; \
                 re-derive this probe and the lofted_blade prose",
                out.err()
            );
        }
        // The standing wall: past π, the end-to-end stacking trilean
        // reverses — TYPED, and pinned by variant so it fails loud if
        // the loft's stacking statement ever changes shape.
        for curl in [3.5, 4.7, 6.0] {
            let out = try_lofted_blade::<f64>(
                LEAF_A_BASE,
                LEAF_A_DIR,
                LEAF_A_UP,
                LEAF_A_LEN,
                -curl,
                leaf_a_plan(),
                LOFT_STATIONS,
            );
            assert!(
                matches!(out, Err(pncad::sweep::LoftError::ReversedStacking)),
                "curl {curl} rad: expected the end-to-end ReversedStacking wall, \
                 got {out:?} — the stacking wall moved; re-derive this probe, \
                 the lofted_blade prose, and the filed frontier together"
            );
        }
    }
}
