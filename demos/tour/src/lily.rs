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
//! - the plant has its underground half now: a **corm** — the swollen
//!   stem-base a *Calochortus* rises from each spring — threaded on
//!   the straight basal internode, the **foot**. The corm is a
//!   sphere-zone swelling with a coaxial BORE at the stem's own
//!   diameter, so the two bodies are cosurface along the whole bore:
//!   the cleanest declared CYLINDRICAL contact this plant has, and
//!   the exact class M9-3 built. It still does not JOIN, and probes
//!   12 and 13 are why — measured, not assumed.
//! - the **stem** is a chain of circular tube arcs — each one a
//!   windowed TUBE ALONG AN ARC, i.e. a torus segment said in world
//!   coordinates: ring centre, spine axis, start radial, ring radius,
//!   angular window, tube radius. A turtle walks the arcs in the
//!   world xz-plane so consecutive arcs are G1 by construction
//!   (shared tangent), and the joint is a shared disk the eye does
//!   not see. They are separate BODIES: gluing them is a coincident-
//!   planar contact, which the kernel refuses (probe 1).
//! - the open **flower** is a lantern: a full revolve whose wall is a
//!   sphere zone truncated at both poles — a wide belly, a NECK cone
//!   narrowing to a throat disk the arch's tube exactly fills, and a
//!   puckered conical mouth closing to a small disk. Sphere zone +
//!   two cones + two planes, all exact. The neck is an AUTHORED cone
//!   (70 degrees; the globe's own tangent cone is 65.38 and the
//!   authoring algebra REFUSES it — see [`FLOWER_NECK_HALF_ANGLE`]).
//!   Its angle is not what makes the flower and the stem meet along
//!   one shared circle: what does that is the cone being CUT at the
//!   arch's radius, at the station whose spine tangent the flower
//!   axis is (probe 2).
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
//!   `review_probes::the_lofted_blade_tapers_and_rolls_in_the_stored_geometry`.
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
//! choice, and nothing in the kernel stands in its way any more: the
//! rational wall an arc-margined blade skins converges through the
//! interior knots its swept spine puts there, and a blade of this
//! stop's proportions prints an exact volume like every other body
//! here.
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
//! `review_probes::the_sepals_stand_outside_the_globe_they_are_tangent_to`.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::f64::consts::PI;

use pncad::geom_brep::SurfaceKind;
use pncad::geom_core::{Affine3, Mat3, Point2, Point3, Vec2, Vec3};
use pncad::prelude::{Open, Start};
use pncad::profile::{ArcSweep, Center, ProfileLoop, ProfileVertex, SketchPlane, Via};
// The named gap below (`section_loops`): the raw loop door is kernel
// vocabulary, off the façade, so the one scene that needs it names the
// kernel crate directly.
use pncad::sweep::blend::BlendError;
use pncad::sweep::{
    ExtrudeError, Extrusion, Revolution, RevolveAxis, TubeWindow, WedgeFrames, extrude, loft_body,
    revolve, revolved_caps, sweep_body, tube_along_arc,
};
use pncad::topo::{Body, BooleanError, Operand, TransformError};
use profile::RawLoop;

use crate::scalar::Scalar;
use crate::{SceneBody, Stop, View};
use pncad::authoring::{p2, validated};
use pncad::geom_core::Tol;

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
fn tube_arc<S: Scalar>(spec: ArcSpec, tube: f64, tol: Tol) -> (Body<S>, WedgeFrames<S>) {
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
        tol,
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
/// the centre and at `mouth` below it, then closed by a conical pucker
/// of drop `lip_drop` down to a disk of radius `lip_r` — the three
/// tepal tips meeting under the lantern. Above the zone it opens
/// through a NECK cone — `neck` is that cone's throat radius (the
/// stem tube's own) and its half-angle — to a throat disk sitting in
/// the attachment plane.
///
/// Faces: throat plane, neck cone, sphere zone, pucker cone, mouth
/// plane. Every one exact; the profile is authored centre-first
/// (`Center`) so the zone's carrier is the sphere itself and not a
/// fitted arc. That centre-intent is now sayable in the algebra
/// (LIB-G1 constructor 3): the globe centre is authored, the winding
/// is structural, and equidistance of the two endpoints from the
/// centre is CHECKED — this profile derives both radii from the
/// sphere, so it passes by construction and would refuse loudly if it
/// ever stopped doing so.
/// How far a **neck** cone of half-angle `half_angle` rises above the
/// globe's truncation circle to reach the throat radius `neck_r`.
///
/// The neck spans two radii the flower already has — the shoulder's
/// `r_top` and the throat's `neck_r`, which is the stem tube's — so a
/// half-angle is the whole of what is left to choose, and the drop
/// follows: `(r_top − neck_r) / tan α`.
///
/// One expression because two callers need the same number: the
/// [`meridian`] that draws the cone, and the placement that has to
/// know where the globe centre ended up with a neck in front of it.
fn neck_drop(globe: f64, top: f64, neck_r: f64, half_angle: f64) -> f64 {
    let r_top = (globe.powi(2) - top.powi(2)).sqrt();
    (r_top - neck_r) / half_angle.tan()
}

/// The lantern/bud **meridian**: the closed profile a flower or a
/// pre-tepal is revolved from, in sketch `(s, t)` with the axis along
/// `t`. Shared by [`lantern`] (revolved FULL, one body) and [`bud`]
/// (revolved PARTIAL, three bodies on three axes), so the bud's
/// segments are the same shape said three times and not a re-typed
/// near-copy.
///
/// `neck` — the throat radius and the neck cone's half-angle — is
/// what a flower welded to a TUBE needs and a bud on a tripod cannot
/// use. With it the profile opens at a throat disk of that radius and
/// rises to the shoulder along a [`neck_drop`] cone, so the revolved
/// body's topmost curved wall is a cone whose rim circle is a circle
/// of the throat radius about the flower axis, in the plane through
/// the attachment point — which is exactly a tube's meridian circle
/// at the station whose tangent that axis is. Without it the profile
/// opens flat at `r_top`: the attachment disk a bud's tepal is
/// truncated by, whose three tilted axes have no such station to
/// share.
fn meridian<S: Scalar>(
    globe: f64,
    top: f64,
    mouth: f64,
    lip_r: f64,
    lip_drop: f64,
    neck: Option<(f64, f64)>,
    tol: Tol,
) -> ProfileLoop<S> {
    let r_top = (globe.powi(2) - top.powi(2)).sqrt();
    let r_mouth = (globe.powi(2) - mouth.powi(2)).sqrt();
    let shoulder = neck.map_or(0.0, |(nr, a)| neck_drop(globe, top, nr, a));
    let t_mouth = shoulder + top + mouth;
    let t_end = t_mouth + lip_drop;
    let opening = match neck {
        Some((nr, _)) => Open
            .at(p2(0.0, 0.0))
            .line_to(p2(nr, 0.0), tol)
            .expect("lantern throat disk")
            .line_to(p2(r_top, shoulder), tol)
            .expect("lantern neck cone"),
        None => Open
            .at(p2(0.0, 0.0))
            .line_to(p2(r_top, 0.0), tol)
            .expect("lantern attachment disk"),
    };
    opening
        // The belly: the sphere's own arc about the globe centre,
        // swept the long way round the equator (Ccw in sketch (s, t)).
        .arc_to(
            Center {
                c: p2(0.0, shoulder + top),
                winding: ArcSweep::Ccw,
                p: p2(r_mouth, t_mouth),
            },
            tol,
        )
        .expect("lantern belly rides the globe")
        .line_to(p2(lip_r, t_end), tol)
        .expect("lantern pucker cone")
        .line_to(p2(0.0, t_end), tol)
        .expect("lantern lip disk")
        .line_to(Start, tol)
        .expect("lantern axis seam")
        .into()
}
#[allow(clippy::too_many_arguments)] // the 9th is the run-tolerance witness
fn lantern<S: Scalar>(
    attach: (f64, f64),
    dir: (f64, f64),
    globe: f64,
    top: f64,
    mouth: f64,
    lip_r: f64,
    lip_drop: f64,
    neck: (f64, f64),
    tol: Tol,
) -> Body<S> {
    // Sketch frame: origin at the attachment point, v along the
    // flower axis (into the flower), u the in-plane radial.
    let plane = SketchPlane::from_frame(
        pt3(attach.0, 0.0, attach.1),
        v3(-dir.1, 0.0, dir.0),
        v3(dir.0, 0.0, dir.1),
    );
    revolve(
        &validated(
            plane,
            vec![meridian(
                globe,
                top,
                mouth,
                lip_r,
                lip_drop,
                Some(neck),
                tol,
            )],
            tol,
        )
        .expect("lily profile validates"),
        sketch_axis(),
        Revolution::Full,
        tol,
    )
    .expect("lantern revolves")
    .body
}

/// The **corm**: the swollen underground stem-base a *Calochortus*
/// rises from each spring — and, since M9-3, the one place on this
/// plant where two authored bodies are made ONE.
///
/// A corm is not a thing the stem stands on; it is the stem's OWN base,
/// swollen, with the axis running through it. So it is authored as
/// exactly that: a sphere-zone swelling with a coaxial BORE at the
/// stem's diameter, threaded on the stem's foot. Its two planar caps
/// are ANNULI, which is load-bearing and not a styling choice: a full
/// revolve whose planar cap TOUCHES THE AXIS arrives as two half-faces
/// on one plane key, and that used to be the F7 maximal-faces defect
/// with no way out. It is repairable now: the cap's two seam edges are
/// the halves of the disc's diameter, so the pole is a vertex interior
/// to one straight carrier, and `merge_coplanar_faces` removes the
/// seam (`kef` then `kev`) leaving ONE face. An annular cap still
/// sidesteps the question rather than answering it — it revolves to
/// one whole face and has no such pair to repair.
///
/// Sketch frame: origin on the corm's top plane, `v` pointing DOWN
/// into the corm, so `t` is depth. Same axis convention as
/// [`lantern`] — placement is a frame choice, not an argument.
fn corm<S: Scalar>(
    top_z: f64,
    globe: f64,
    shoulder: f64,
    base: f64,
    bore_r: f64,
    tol: Tol,
) -> Body<S> {
    let r_top = (globe.powi(2) - shoulder.powi(2)).sqrt();
    let r_base = (globe.powi(2) - base.powi(2)).sqrt();
    let t_base = shoulder + base;
    let lp: ProfileLoop<S> = Open
        .at(p2(bore_r, 0.0))
        .line_to(p2(r_top, 0.0), tol)
        .expect("corm shoulder annulus")
        // The flank rides the corm's own sphere, about its centre —
        // authored centre-first, as the lantern's belly is.
        .arc_to(
            Center {
                c: p2(0.0, shoulder),
                winding: ArcSweep::Ccw,
                p: p2(r_base, t_base),
            },
            tol,
        )
        .expect("corm flank rides the sphere")
        .line_to(p2(bore_r, t_base), tol)
        .expect("corm base annulus")
        .line_to(Start, tol)
        .expect("corm bore wall")
        .into();
    let plane =
        SketchPlane::from_frame(pt3(0.0, 0.0, top_z), v3(1.0, 0.0, 0.0), v3(0.0, 0.0, -1.0));
    revolve(
        &validated(plane, vec![lp], tol).expect("corm profile validates"),
        sketch_axis(),
        Revolution::Full,
        tol,
    )
    .expect("corm revolves")
    .body
}

/// The stem's **foot**: the straight basal internode, a plain circular
/// cylinder standing in the corm's socket and rising to the world
/// origin, where the turtle's first arc begins.
///
/// Three 120° arcs of one carrier (`circle_split`), not a whole
/// circle: a boolean operand's curved wall must be maximal-faced, and
/// the split count is part of what the seam looks like.
fn foot<S: Scalar>(z0: f64, z1: f64, r: f64, tol: Tol) -> Body<S> {
    let rim = pncad::profile::circle_split(
        Point2::new(S::from_f64(0.0), S::from_f64(0.0)),
        S::from_f64(r),
        3,
        S::from_f64(0.0),
        tol,
    )
    .expect("the foot's three-arc rim authors");
    let plane = SketchPlane::new(Affine3::translation(v3::<S>(0.0, 0.0, z0)));
    let profile = validated(plane, vec![rim.into()], tol).expect("foot profile validates");
    extrude(&profile, Extrusion::Distance(S::from_f64(z1 - z0)), tol)
        .expect("the foot extrudes")
        .body
}

/// The corm's dimensions and the socket the stem stands in. The
/// socket's radius IS the stem tube's, because the two are the same
/// stem: one number, used twice.
const CORM_TOP_Z: f64 = -0.28;
/// See [`CORM_TOP_Z`].
const CORM_GLOBE: f64 = 0.30;
/// Depth of the corm's sphere centre below its top plane.
const CORM_SHOULDER: f64 = 0.22;
/// Depth of the base truncation below that centre.
const CORM_BASE: f64 = 0.22;
/// The stem tube's radius — the lower arc's, the bore's, and the
/// foot's: one number, because they are one stem.
const STEM_R: f64 = 0.060;
/// Where the foot's root end stops, below the corm.
const FOOT_BOTTOM_Z: f64 = -0.92;

/// Every cylindrical face of `body` carried by a cylinder of radius
/// `r` about the world z-axis.
///
/// **A library finding, recorded where it was met** (the demos'
/// purpose rule). The author knows exactly which contact he means —
/// "the socket wall against the foot's wall" — and there is no
/// selector on the plain `Body` API to say it with: the intent has to
/// be re-derived by walking every face in the arena and matching
/// stored surface parameters. It comes back as THREE faces on the
/// foot (the three-arc split) and TWO on the corm (a full revolve
/// halves every wall at its seam), so ONE contact in the author's head
/// is spelled as SIX `FacePairDeclaration`s. `crate::twopeg` meets the
/// same gap and mints its own matcher for it; the document layer has
/// selection (`GeoSelect`), the kernel-level `Body` does not, and a
/// declared contact is a kernel-level object.
fn axial_walls<S: Scalar>(body: &Body<S>, r: f64) -> Vec<pncad::topo::FaceKey> {
    body.faces()
        .filter(|(_, f)| {
            matches!(
                body.get_surface(f.surface),
                Some(pncad::geom::Surface::Cylinder { radius, .. })
                    if (radius.f() - r).abs() < 1e-12
            )
        })
        .map(|(k, _)| k)
        .collect()
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
    tol: Tol,
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
            &validated(
                plane,
                // No neck: a tepal's axis LEANS off the pedicel's
                // tangent, so there is no station whose meridian
                // circle a neck could be cut to. The bud keeps the
                // flat attachment disk and the set-back placement.
                vec![meridian(globe, top, mouth, lip_r, lip_drop, None, tol)],
                tol,
            )
            .expect("bud profile validates"),
            sketch_axis(),
            Revolution::Partial(S::from_f64(span)),
            tol,
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
/// `review_probes::the_lofted_blade_tapers_and_rolls_in_the_stored_geometry`
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
/// vocabulary — and the quadrature that used to stand in its way no
/// longer does. The blade this stop would draw has been built and
/// measured: a crescent section on this spine, at [`LEAF_STATIONS`]
/// stations and [`LEAF_V_DEGREE`], certifies an exact volume like
/// every other body here. That measurement is a standing row, not a
/// claim — `sweep`'s `cert5_offgrid_knot_rational::the_lily_crescent_
/// blade_certifies` rebuilds exactly this geometry and re-takes it.
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
    tol: Tol,
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
    let loops: Vec<ProfileLoop<f64>> = vec![crate::paths::path_polygon(
        &[
            (-0.5 * section.width, 0.0),
            (0.0, -section.keel),
            (0.5 * section.width, 0.0),
            (0.0, section.ridge),
        ],
        tol,
    )];
    sweep_body::<S>(&loops, place, &path, LEAF_STATIONS, LEAF_V_DEGREE, tol)
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
        // NAMED GAP — the one place in the tour the presented surface
        // cannot say what the demo means, recorded rather than worked
        // around (main.rs's purpose block). Both halves the ruling
        // named are closed; what is left is a third thing, and it is
        // about THIS SECTION FAMILY rather than about the lattice.
        //
        // This outline is FOUR corners said on EIGHT vertices, because a
        // loft matches segment j to segment j and the tip and attachment
        // sections must be spelled on one vertex budget. So one junction
        // per side is a straight run subdivided, at both ends of the
        // shoulder parameter: at `shoulder = 0` each shoulder is the
        // midpoint of two tips (the kite), and at `shoulder = 1` each tip
        // lies ON the rectangle edge its two neighbouring corners span —
        // collinear with them, though only the ridge and keel tips are
        // that edge's midpoint (the margins sit at y = 0 on an edge
        // spanning [-keel, ridge], and no section here has keel = ridge).
        // Only the eased sections in between turn at every vertex.
        //
        // CLOSED: those junctions are carrier IDENTITY, legal
        // undeclared, and the lattice spells them — `line(len)` off a
        // directed point for an interior subdivision, `continue_to(p)`
        // where the subdivision lands on a named point, and
        // `continue_to(Start)` for a run that crosses the seam. A
        // single section of this family authors end to end today.
        //
        // OPEN, and measured (`bool8_r1_probes`): the closer needs the
        // seam cut at a CORNER, and this family's corners MOVE. In the
        // kite the corners are the tips, so the sections whose seam is
        // a tip author — starts 0, 2, 4, 6 of the ring below. In the
        // rectangle the corners are the shoulders, so those sections
        // want starts 1, 3, 5, 7. The two sets are disjoint, and not by
        // accident: the kite's corner set IS its tips and the
        // rectangle's IS its shoulders, which are disjoint points of
        // the outline whatever budget is spent on it. A loft matches
        // segment j of every section to segment j of every other, so
        // every section here must be authored at ONE rotation — and
        // `leaf_a_plan` carries a `shoulder = 1` base AND a
        // `shoulder = 0` belly, so no rotation gives all of them a
        // corner at the seam. The section that misses out closes on a
        // subdivision vertex, which is a mid-carrier seam: PATHS §6
        // PQ4, deliberately left standing by the ruling.
        //
        // So this loop is still raw-authored: `ProfileLoop`'s fields
        // are sealed, and the only route left is the kernel's raw door,
        // `profile::RawLoop`, which `pncad::profile` deliberately
        // omits. That is why this crate carries a second kernel
        // dependency — the gap stays loud in the dependency graph
        // instead of hidden in a struct literal. What would close it is
        // a ruling on whether a DECLARED subdivision vertex is an
        // admissible seam; the question is put in PATHS §4.
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
#[allow(clippy::too_many_arguments)] // the 8th is the run-tolerance witness
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
    tol: Tol,
) -> Body<S> {
    try_lofted_blade(base, dir, up, len, curl, plan, stations, tol)
        .expect("the lofted blade skins its own sections")
        .body
}

/// [`lofted_blade`] with the refusal surfaced instead of expected, so
#[allow(clippy::too_many_arguments)] // the 8th is the run-tolerance witness
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
    tol: Tol,
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
    loft_body::<S>(&sections, &places, LEAF_V_DEGREE, tol)
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
/// nothing here is joined, and the flower's own throat disk and the
/// arch's end cap are exactly coincident where the two abut. But a
/// sepal that
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
    tol: Tol,
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
        lofted_blade::<S>(base, tau, n, len, curl, plan, SEPAL_STATIONS, tol)
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

/// The ARCH tube's radius — and the flower's throat radius, because
/// they are one circle. The arch's terminal meridian circle and the
/// lantern's neck-cone rim are the same locus, and that identity is
/// what makes the flower/arch weld a shared-circle contact; one
/// number is how the scene says so.
const ARCH_R: f64 = 0.052;

/// The flower's NECK cone, as its half-angle from the flower axis.
/// With the two radii the neck spans already fixed — the shoulder's
/// `sqrt(FLOWER_GLOBE² − FLOWER_TOP²)` and the throat's [`ARCH_R`] —
/// this one angle fixes the whole neck.
///
/// **Why an authored angle rather than the globe's own TANGENT cone**,
/// which is the derivation that suggests itself (α = atan(top/r_top)
/// = 65.38°, apex at the truncation plane's pole in the sphere, a G1
/// shoulder). Two reasons, and the first is a live refusal: the
/// authoring algebra will not take a leg that departs along its
/// predecessor's tangent when the departure is spelled in COORDINATES
/// — `PathError::JunctionTangent`, one recourse named, the structural
/// `.tangent()` verb (pinned by
/// `review_probes::the_globes_tangent_cone_neck_is_refused_by_the_junction_gate`).
/// The second is the shape: 70° sits INSIDE the tangent cone, so the
/// shoulder is a real convex crease where the globe meets its neck,
/// which is what a nodding *Calochortus* has and a G1 blend would
/// erase.
const FLOWER_NECK_HALF_ANGLE: f64 = 70.0 * PI / 180.0;

const GREEN_STEM: [f64; 3] = [0.36, 0.52, 0.30];
const GREEN_LEAF: [f64; 3] = [0.44, 0.62, 0.34];
/// *C. pulchellus* is the YELLOW fairy lantern — clear lemon, not the
/// white of *albus* this scene used to wear.
const YELLOW_TEPAL: [f64; 3] = [0.95, 0.84, 0.32];
/// The sepals are greener than the petals and stay so: on a live
/// pulchellus they read as the yellow-green sheath the globe hangs in.
const GREEN_SEPAL: [f64; 3] = [0.72, 0.76, 0.36];
/// The corm is underground and reads as such: a dull, papery brown.
const GREEN_CORM: [f64; 3] = [0.55, 0.44, 0.30];

/// Builds the whole plant: two stem arcs, one branching pedicel, two
/// nodding lanterns, three basal leaves — eight bodies, every one a
/// closed analytic solid.
///
/// The stem is walked by a [`Turtle`] so the arcs are G1 at their
/// joints by construction: each arc's start tangent IS the previous
/// arc's end tangent, and the lantern's axis IS the last tangent, so
/// the flower hangs along the stem's own direction rather than along
/// a hand-chosen vector.
pub fn plant<S: Scalar>(tol: Tol) -> Vec<Piece<S>> {
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

    let (stem, stem_caps) = tube_arc(lower, 0.060, tol);
    let (arch, arch_caps) = tube_arc(upper, ARCH_R, tol);
    let (pedicel_body, pedicel_caps) = tube_arc(pedicel, 0.032, tol);

    // The main flower's attachment point and axis — the same two the
    // lantern below is built on, named once so the sepals hang on the
    // flower's own axis rather than a hand-copied one.
    //
    // The attachment IS the arch's last spine point, with no set-back:
    // the flower's throat circle and the tube's terminal meridian
    // circle are then the SAME circle, which is what makes the weld a
    // shared-circle contact instead of a transverse one (see
    // [`weld_circle`], pinned by
    // `review_probes::the_flower_and_the_arch_share_one_circle`).
    let flower_attach = at_flower.p;
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
    // The globe's own centre, derived from the numbers the lantern is
    // built with rather than copied: the NECK's drop plus
    // `FLOWER_TOP`, along the flower axis from the attachment point.
    // The sepals then stand on the sphere the lantern actually has.
    let flower_globe_depth =
        neck_drop(FLOWER_GLOBE, FLOWER_TOP, ARCH_R, FLOWER_NECK_HALF_ANGLE) + FLOWER_TOP;
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
        deg(5.0),
        deg(90.0),
        deg(156.0),
        tol,
    );

    // The sepals' polar angle must CLEAR `acos(FLOWER_TOP /
    // FLOWER_GLOBE)`, the angle at which the sphere is truncated by
    // the neck's shoulder circle: nearer the pole than that the sphere
    // is not part of the body, so a sepal standing there would be
    // tangent to a surface that is not there. The margin below is the
    // clearance, and it puts them on the shoulder of the globe.
    let sepal_bodies: [Body<S>; 3] = sepals(
        (
            flower_attach.0 + flower_globe_depth * at_flower.t.0,
            0.0,
            flower_attach.1 + flower_globe_depth * at_flower.t.1,
        ),
        (at_flower.t.0, 0.0, at_flower.t.1),
        FLOWER_GLOBE,
        (FLOWER_TOP / FLOWER_GLOBE).acos() + deg(4.0),
        deg(180.0),
        1.05,
        0.40,
        sepal_plan,
        tol,
    );

    let mut pieces = vec![
        Piece {
            name: "lily_corm",
            color: GREEN_CORM,
            // The swollen stem-base, threaded on the foot below —
            // TOUCHING it along the whole bore and not joined to it,
            // like every other pair on this plant (probe 12).
            body: corm(
                CORM_TOP_Z,
                CORM_GLOBE,
                CORM_SHOULDER,
                CORM_BASE,
                STEM_R,
                tol,
            ),
            caps: None,
        },
        Piece {
            name: "lily_foot",
            color: GREEN_STEM,
            // The straight basal internode: runs up through the corm
            // to the world origin, where the turtle's first arc starts.
            body: foot(FOOT_BOTTOM_Z, 0.0, STEM_R, tol),
            caps: None,
        },
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
            // The flower sits ON the arch's last station, throat
            // circle to tube rim, and the neck cone is cut at the
            // arch's own radius — so the cone and the tube meet along
            // ONE shared circle rather than crossing. Gluing them is
            // still probe 2.
            body: lantern(
                flower_attach,
                at_flower.t,
                FLOWER_GLOBE,
                FLOWER_TOP,
                0.36,
                0.09,
                0.16,
                (ARCH_R, FLOWER_NECK_HALF_ANGLE),
                tol,
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
                tol,
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
                tol,
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
                tol,
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
pub fn stops(tol: Tol) -> Vec<Stop> {
    let pieces = plant::<f64>(tol);
    let note = format!(
        "{} closed solids: 3 torus-segment stem tubes said in WORLD \
         coordinates (centre/axis/u_ref/radii stored exactly as \
         given), 1 sphere-zone lantern with a NECK cone cut at the \
         arch tube's own radius — its rim IS that tube's terminal \
         meridian circle, so flower and stem meet on one shared \
         circle — and a conical mouth, 3 \
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
         Nothing is JOINED — the corm threaded on the stem's foot \
         least of all, and the leaf to its own sheath least of all \
         after that: see the wall probes.",
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
// The weld circle
// ---------------------------------------------------------------

/// One circle in space: centre, radius, unit normal — the only shape
/// the flower/arch junction is allowed to be.
#[derive(Clone, Copy, Debug)]
struct Circle {
    c: (f64, f64, f64),
    r: f64,
    n: (f64, f64, f64),
}

fn v_sub(a: (f64, f64, f64), b: (f64, f64, f64)) -> (f64, f64, f64) {
    (a.0 - b.0, a.1 - b.1, a.2 - b.2)
}

fn v_dot(a: (f64, f64, f64), b: (f64, f64, f64)) -> f64 {
    a.0 * b.0 + a.1 * b.1 + a.2 * b.2
}

fn v_cross(a: (f64, f64, f64), b: (f64, f64, f64)) -> (f64, f64, f64) {
    (
        a.1 * b.2 - a.2 * b.1,
        a.2 * b.0 - a.0 * b.2,
        a.0 * b.1 - a.1 * b.0,
    )
}

fn v_len(a: (f64, f64, f64)) -> f64 {
    v_dot(a, a).sqrt()
}

/// Every station circle of radius `rho` that `body`'s conical faces
/// carry — two per cone, one on each nappe.
///
/// Closed form off the stored `(apex, axis, half_angle)`: the
/// generator reaches radius `rho` at slant `v = rho / sin α`, whose
/// axial offset is `v·cos α = rho / tan α`. Nothing is sampled and
/// nothing is fitted; a cone's radius-`rho` locus IS a circle.
fn cone_station_circles<S: Scalar>(
    body: &Body<S>,
    rho: f64,
) -> Vec<(pncad::topo::FaceKey, Circle)> {
    let mut out = Vec::new();
    for (k, f) in body.faces() {
        let Some(&pncad::geom::Surface::Cone {
            apex,
            axis,
            half_angle,
            ..
        }) = body.get_surface(f.surface)
        else {
            continue;
        };
        let a = (apex.x.f(), apex.y.f(), apex.z.f());
        let d = (axis.x.f(), axis.y.f(), axis.z.f());
        let off = rho / half_angle.f().tan();
        for s in [1.0, -1.0] {
            out.push((
                k,
                Circle {
                    c: (
                        a.0 + s * off * d.0,
                        a.1 + s * off * d.1,
                        a.2 + s * off * d.2,
                    ),
                    r: rho,
                    n: d,
                },
            ));
        }
    }
    out
}

/// A stored torus carrier, reduced to the four numbers a meridian
/// circle is built from.
#[derive(Clone, Copy, Debug)]
struct TorusCarrier {
    centre: (f64, f64, f64),
    axis: (f64, f64, f64),
    big_r: f64,
    small_r: f64,
}

/// The body's single stored torus carrier.
///
/// **There are two readers of the same data**, deliberately: this one
/// (the weld path's, generic over the run scalar, first-match) and
/// `review_probes::torus`, which walks EVERY torus face and asserts
/// the half-bands share one carrier. Neither checks the other; what
/// ties them is that the review helper's half-band assertion is what
/// makes "first match" a complete answer here, so if the seam ever
/// split a torus wall onto two carriers the review row fails and this
/// one silently picks one. That row is the tie.
fn torus_carrier<S: Scalar>(body: &Body<S>) -> TorusCarrier {
    for (_, f) in body.faces() {
        if let Some(&pncad::geom::Surface::Torus {
            center,
            axis,
            major_radius,
            minor_radius,
            ..
        }) = body.get_surface(f.surface)
        {
            return TorusCarrier {
                centre: (center.x.f(), center.y.f(), center.z.f()),
                axis: (axis.x.f(), axis.y.f(), axis.z.f()),
                big_r: major_radius.f(),
                small_r: minor_radius.f(),
            };
        }
    }
    panic!("the arch stores a torus wall")
}

/// How far `circle` is from BEING a meridian circle of `torus`, as
/// the three residuals that say it is one: the centre's distance off
/// the spine circle, the radius mismatch, and the sine of the angle
/// between the circle's normal and the spine tangent under it.
///
/// A torus's meridian circle at azimuth `u` is
/// `(centre + radial(u)·R, r, tangential(u))`, so membership needs no
/// azimuth to be chosen: the azimuth is READ OFF the candidate centre
/// and the three residuals are what remains.
fn meridian_residuals(circle: Circle, torus: TorusCarrier) -> (f64, f64, f64) {
    let ta = torus.axis;
    let w = v_sub(circle.c, torus.centre);
    let h = v_dot(w, ta);
    let radial = (w.0 - h * ta.0, w.1 - h * ta.1, w.2 - h * ta.2);
    let rad_len = v_len(radial);
    // Distance from the spine circle: the meridian plane's own polar
    // coordinates, (in-plane radius − R, out-of-plane height).
    let off_spine = ((rad_len - torus.big_r).powi(2) + h * h).sqrt();
    let tangential = v_cross(ta, radial);
    let tl = v_len(tangential);
    let tangential = (tangential.0 / tl, tangential.1 / tl, tangential.2 / tl);
    (
        off_spine,
        (circle.r - torus.small_r).abs(),
        v_len(v_cross(circle.n, tangential)),
    )
}

/// **The weld circle, computed twice from stored carriers and
/// asserted equal.** This is the CONTENT of the flower/arch junction,
/// so it is a pin and not a sentence.
///
/// A torus's meridian circle at a station lies in the plane
/// perpendicular to the spine tangent there, and the lantern's axis
/// IS that tangent (`review_probes::
/// lantern_axes_are_the_stored_stem_tangents`) — so a cone coaxial
/// with it, cut at the tube's minor radius, meets that meridian
/// circle exactly. The two carriers are then analytically identical
/// along one circle, and the pair's contact is that circle rather
/// than a transverse SSI curve nobody has a closed form for.
///
/// Stated as a MEASUREMENT, not a search. The lantern's cones offer
/// EIGHT station circles at the tube's minor radius — two conical
/// WALLS, each halved at the full revolve's seam, each half-band
/// offering both nappes; the best of them must satisfy the torus's
/// meridian membership, the nearest DISTINCT one must miss by a
/// distance no tolerance could confuse with zero, and the winner must
/// sit on the tube's END — the frame `sweep::revolved_caps` reports —
/// rather than somewhere along it.
///
/// **Two of the three residuals are checks; the third is
/// bookkeeping.** The RADIUS residual cannot fail for any body:
/// [`cone_station_circles`] builds every candidate with `r` set to
/// the torus's own `minor_radius`, so its vanishing is definitional
/// and is carried only so the triple reads as one circle-equality.
/// The off-spine and normal residuals are the checks, and their
/// 1e-12 windows are what binds.
///
/// Returns the neck cone's face key, the circle, its residuals and
/// the nearest distinct station circle's score, for narration and for
/// the probes that need to talk about the PUCKER.
fn weld_circle<S: Scalar>(
    lantern: &Body<S>,
    arch: &Body<S>,
    arch_caps: &WedgeFrames<S>,
) -> (pncad::topo::FaceKey, Circle, (f64, f64, f64), f64) {
    let torus = torus_carrier(arch);
    let candidates = cone_station_circles(lantern, torus.small_r);
    // Two conical WALLS, each halved at the full revolve's seam, each
    // half-band offering its carrier's two nappes.
    assert!(
        candidates.len() == 8,
        "the lantern carries two cones — a neck and a pucker — halved at the \
         revolve seam, so eight station circles at the tube's minor radius; \
         got {}",
        candidates.len()
    );
    let score = |c: &Circle| {
        let (a, b, d) = meridian_residuals(*c, torus);
        a + b + d
    };
    let mut ranked = candidates;
    ranked.sort_by(|a, b| score(&a.1).total_cmp(&score(&b.1)));
    let (neck_face, best) = ranked[0];
    let res = meridian_residuals(best, torus);
    // The tube's END, not a station part-way along it: one of the two
    // joint frames the revolve recorded passes through this centre
    // with this normal.
    let on_end = [arch_caps.start, arch_caps.end].into_iter().any(|pose| {
        let o = (pose.origin.x.f(), pose.origin.y.f(), pose.origin.z.f());
        let n = (pose.axis.x.f(), pose.axis.y.f(), pose.axis.z.f());
        v_len(v_sub(best.c, o)) < 1e-12 && v_len(v_cross(best.n, n)) < 1e-12
    });
    assert!(
        res.0 < 1e-12 && res.1 == 0.0 && res.2 < 1e-12,
        "the flower's neck circle is not the arch's meridian circle: off-spine \
         {:e}, radius {:e}, normal {:e}",
        res.0,
        res.1,
        res.2
    );
    assert!(
        on_end,
        "the weld circle is a meridian circle of the arch's torus but not the \
         one at its END station — the flower is threaded on the tube, not \
         welded to its rim"
    );
    // The neck's two half-bands offer the SAME circle, so the miss
    // that matters is the nearest DISTINCT station circle — the neck's
    // other nappe, or either of the pucker's.
    let runner_up = ranked
        .iter()
        .filter(|(_, c)| v_len(v_sub(c.c, best.c)) > 1e-9)
        .map(|(_, c)| score(c))
        .fold(f64::INFINITY, f64::min);
    assert!(
        runner_up > 1e-3,
        "a second, distinct station circle coincides to {runner_up:e} too — the \
         match above is not the neck cone's alone"
    );
    (neck_face, best, res, runner_up)
}

// ---------------------------------------------------------------
// The wall probes
// ---------------------------------------------------------------

/// A full sphere of radius `r` about `c` (in the world xz-plane at
/// y = 0), as a revolve of a half-disc whose diameter lies on the
/// axis — the shape a tepal seam would be carved with.
fn ball<S: Scalar>(c: (f64, f64), r: f64, tol: Tol) -> Body<S> {
    let plane = SketchPlane::from_frame(pt3(c.0, 0.0, c.1), v3(1.0, 0.0, 0.0), v3(0.0, 0.0, 1.0));
    // Algebra-authored (LIB-G1): centre-first, with the sphere's own
    // centre authored and the bulge derived at lowering.
    let lp = Open
        .at(p2(0.0, -r))
        .arc_to(
            Center {
                c: p2(0.0, 0.0),
                winding: ArcSweep::Ccw,
                p: p2(0.0, r),
            },
            tol,
        )
        .expect("ball meridian rides its centre")
        .line_to(Start, tol)
        .expect("ball axis seam")
        .into();
    revolve(
        &validated(plane, vec![lp], tol).expect("lily profile validates"),
        sketch_axis(),
        Revolution::Full,
        tol,
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
pub fn wall_probes<S: Scalar>(tol: Tol) {
    println!("\n-- the lily's walls: what a plant asks for that the kernel will not say --");
    let pieces = plant::<S>(tol);
    let by = |name: &str| -> &Body<S> {
        &pieces
            .iter()
            .find(|p| p.name == name)
            .expect("named lily piece")
            .body
    };
    let (stem, arch, lant) = (by("lily_stem"), by("lily_arch"), by("lily_lantern"));
    // **The authored repair.** #1031's pole half made a full revolve's
    // axis-touching caps mergeable, so the scene now asks for that
    // repair rather than working around it: probe 7 subtracts from the
    // REPAIRED lantern, which is what a user would do the day the door
    // opened. The un-repaired body stays available above, and probe 13
    // is the row that pins the door open.
    let repaired_lantern = {
        let mut b = lant.clone();
        b.merge_coplanar_faces(tol)
            .expect("the lantern's pole-split caps repair (#1031's pole half)");
        b
    };
    let arch_caps = pieces
        .iter()
        .find(|p| p.name == "lily_arch")
        .and_then(|p| p.caps.as_ref())
        .expect("the arch is a partial revolve, so it has joint frames");

    // The junction wall 2 asks about, measured before it is asked for:
    // the flower's neck circle and the arch's terminal meridian circle
    // are ONE circle, in closed form off both stored carriers.
    let (_, weld, res, runner_up) = weld_circle(lant, arch, arch_caps);
    println!(
        "   the weld circle — centre ({:.6}, {:.6}, {:.6}), r = {:.3}, normal \
         ({:.6}, {:.6}, {:.6}): the arch's terminal meridian circle and the \
         lantern's neck-cone rim, off-spine {:.3e} / radius {:.3e} / normal \
         {:.3e}; nearest DISTINCT station circle — the neck cone's own \
         other nappe — misses by {:.3e}",
        weld.c.0,
        weld.c.1,
        weld.c.2,
        weld.r,
        weld.n.0,
        weld.n.1,
        weld.n.2,
        res.0,
        res.1,
        res.2,
        runner_up
    );

    // 1. The stem is ONE stem. Its two arcs meet on a shared disk —
    //    an exact coincident planar contact, the crosslap mate — so
    //    the glue is the M5 S1 declared REST zip if it reaches it.
    //
    //    M9-3 opened the declared-contact door to the
    //    plane/sphere/cylinder carrier inventory. The TORUS is not in
    //    that inventory, and this wall is the
    //    named residue of the ruling that decided so — banked as
    //    **#968**: the torus declared-Rest lane wants gate admission
    //    at the operand scan, a torus rung in `carrier_eq` so the
    //    declared descent has a verdict to consume, and a vocabulary
    //    for the torus × torus tangency at the shared rim circle,
    //    which the DEV-1 witness loci (plane × cylinder, parallel
    //    cylinders) do not cover.
    wall(
        1,
        "glue the two stem arcs into one stem (declared coincident-planar mate)",
        crate::booleans::try_union_declared(stem, arch, tol),
        // The KIND is the claim: the refusal names a TORUS face, i.e.
        // the tangent tube walls, not the coincident planar discs.
        |e| {
            // Reviewer pin (r1 probes): PR body claims (Torus, Plane).
            matches!(
                e,
                BooleanError::CurvedPairUnsupported {
                    op: None,
                    operand: Operand::A,
                    kind: SurfaceKind::Torus,
                    other_kind: SurfaceKind::Plane,
                    ..
                }
            )
        },
        "make the stem a single body — and close #968, whose whole content this is",
    );

    // 2. The flower grows OUT OF the pedicel, and it is now authored
    //    so that it CAN: the neck cone's rim and the arch tube's
    //    terminal meridian circle are one circle, asserted above off
    //    both stored carriers. There is no transverse curve here to
    //    have a closed form for.
    //
    //    What refuses FIRST is the OPERAND GATE, on KINDS: `op: None`
    //    is `gate_operand_pairs` (boolean/reduce.rs), which asks
    //    whether a boolean arm exists for the pair and lets boxes
    //    decide only whether the pair can matter. It reads kinds,
    //    never loci — so it cannot see the coincidence.
    //
    //    **What BINDS is further down, and it is not this pair's
    //    business at all.** The whole sequence is measured by
    //    `review_probes::the_declared_weld_refuses_exactly_as_the_
    //    undeclared_one_does` and its sibling. Widen the gate and the
    //    next refusal is `NonMaximalFaces` on this very body — and it
    //    still is, because THIS probe passes the UNREPAIRED lantern
    //    (probe 7 is the one that repairs first). That door is no
    //    longer a dead end for such a body: `merge_coplanar_faces`
    //    repairs the pole-split caps. A gate exemption was tried for
    //    this and WITHDRAWN — the fix is the repair op, not a
    //    narrowing of `gate_maximal_faces`. After F7 comes the curved
    //    PIERCE arm (wall 12's door), and only after that could a
    //    germ-pair question arise.
    //
    //    So wall 2's binding blocker is #1031, not #968's shape. The
    //    gate-admission reading was this unit's SPEC, and measuring it
    //    is what refuted it: declaring the weld changes nothing today,
    //    because the declared contact is the PLANAR pair the throat
    //    disk and the arch's cap already form, and a cone x torus Rest
    //    declaration would be Contradicted, correctly. #1059 is the
    //    derivation; the measurement is VERBS-LILYWELD PR-2's.
    wall(
        2,
        "weld the lantern onto the arch (cone x torus, meeting on one \
         shared circle)",
        pncad::topo::union(lant, arch, tol),
        |e| {
            // Reviewer pin (lilyweld r1 + r2 probes): PR body claims
            // (Cone, Torus), re-measured on the re-authored pair.
            matches!(
                e,
                BooleanError::CurvedPairUnsupported {
                    op: None,
                    operand: Operand::A,
                    kind: SurfaceKind::Cone,
                    other_kind: SurfaceKind::Torus,
                    ..
                }
            )
        },
        "join flower to stem — #1031's pole half is NECESSARY BUT NOT \
         SUFFICIENT: the measured chain runs gate -> F7 -> the curved \
         pierce arm, and only the first two are anyone's current unit",
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
            .arc_to(
                Via {
                    q: p2(0.5, 0.12),
                    p: p2(1.0, 0.0),
                },
                tol,
            )
            .expect("probe leaf outer arc")
            .arc_to(
                Via {
                    q: p2(0.5, 0.02),
                    p: Start,
                },
                tol,
            )
            .expect("probe leaf inner arc")
            .into();
        validated(plane, vec![lp], tol).expect("lily profile validates")
    };
    wall(
        3,
        "tilt a leaf out of its own plane the cheap way (oblique extrusion)",
        extrude(&leafp, Extrusion::Vector(v3::<S>(0.0, 0.3, 0.04)), tol),
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
        pncad::topo::transform_rigid(lant, &stretch, tol),
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
        pncad::topo::transform_rigid(by("lily_leaf_a"), &mirror, tol),
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
        pncad::sweep::blend::fillet_edges(lant, &rim, S::from_f64(0.02), tol),
        // margin EXACTLY zero is the finding: a co-surface seam
        // meridian, not a near-tangency that a tolerance could split.
        |e| matches!(&e.error, BlendError::TangentialEdge { margin, .. } if *margin == 0.0),
        "soften the tepal-tip rim",
    );

    // 7. The lantern is THREE tepals fused, and their seams are
    //    longitudinal grooves. Carving one is a sphere-on-sphere
    //    subtract — and the GEOMETRY agrees: the ball meets the
    //    spherical zone and clears the conical pucker's exact frustum
    //    by 0.4131 (measured in this module's `verbs_gate_r1_probes`;
    //    the figure moved with the flower when the weld was
    //    re-authored, and the probe is where it is read).
    //
    //    The pair-scoped gate ADMITS this cut: the pucker's box
    //    clears the ball's, so no unsupported KIND can enter the
    //    operation. The F7 door used to answer next; it no longer
    //    does, because the scene REPAIRS the operand first (below) and
    //    a repaired lantern is maximal-faced. What answers now is the
    //    reduction's curved PIERCE arm — wall 12's door — and the
    //    payload is quoted rather than described, the wall-7 lesson
    //    about reading a locus off a comment instead of a dump.
    //
    //    **#1031's POLE HALF has landed, and this is what it bought.**
    //    The lantern's two axis-touching caps were each two half-faces
    //    on one plane key; `merge_coplanar_faces` now repairs both —
    //    faces 10 to 8, vertices 10 to 8, edges 18 to 14, tier 3
    //    clean — because each cap's seam is the two halves of the
    //    disc's DIAMETER, so the pole is a vertex interior to one
    //    straight carrier and removing it changes no locus. The
    //    licence is collinearity, not poleness
    //    (`merge_faces::redundant_subdivision_vertex`). The teapot
    //    cup's coplanar pair is NOT repaired, and what the dump
    //    actually shows about it is its VALENCE — endpoints of
    //    valence 4, so there is no valence-2 junction to license
    //    anything. Its seam's straightness was never measured and no
    //    claim is made about it here.
    //
    //    #1031 stays open for its OTHER defect: an ordinary coplanar
    //    pair at a full-valence edge, measured on that cup's meridian
    //    plane (endpoints valence 4, no pole).
    //
    //    What is left after that is the breadth half, DEPENDENCY-STATED
    //    like probe 8's: it waits on the verbs/breadth slate,
    //    VERBS-PLAN Wave 2 items 6 (VERBS-GATE, the per-face-kind gate
    //    re-scope) and 9 (VERBS-SPHSPH, the sphere × sphere germ lane)
    //    — the ruling that put it there is M9-5's, and the demand
    //    signal is this probe. NOTE for those items: on this
    //    measurement a sphere × sphere germ arm alone does not flip
    //    this wall, because the F7 refusal happens first and is about
    //    the caps.
    wall(
        7,
        "carve a tepal seam into the lantern (sphere x sphere by geometry; the \
         operand's own shape answers first)",
        pncad::topo::subtract(&repaired_lantern, &ball::<S>((-2.80, 0.90), 0.16, tol), tol),
        |e| {
            matches!(
                e,
                BooleanError::CurvedPierceUnsupported {
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
            tol,
        )
    };
    wall(
        8,
        "graft the leaf's sheath onto its blade at their shared, DECLARED rectangle",
        crate::booleans::try_union_declared(by("lily_leaf_a"), &sheath, tol),
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

    // 12. The corm is the stem's OWN base, swollen: the two bodies are
    //     cosurface along the whole bore, at one radius stated once
    //     and used by both. That is a declared CYLINDRICAL `Rest` —
    //     precisely the contact class M9-3 built — and it is DECLARED
    //     here, face pair by face pair, because the author knows which
    //     wall meets which.
    //
    //     It refuses one door short of the zip, and the door is the
    //     reduction's curved-face arm rather than the declaration
    //     gate: an edge lying ON the shared carrier decides zero
    //     clearance and takes `CurvedPierceUnsupported` before any
    //     patch is discovered.
    //
    //     **Which edge is not the claim, and it has moved once.** The
    //     first measurement here was the corm's own annulus rim
    //     CIRCLE, on operand A. Since the curved pierce RING lane
    //     landed, the A-side pairs that used to raise first are
    //     resolved, and the sweep gets one pair further before the same
    //     wall stops it: the refusal is now operand B's seam RULING —
    //     `EdgeKey(4v1)`, a straight chart curve at azimuth 0 on the
    //     foot's own bore wall — against the corm's bore face
    //     `FaceKey(3v1)` (`r = 0.06`), both of its endpoints exactly on
    //     the shared carrier. It reaches the declared-cover rung's
    //     on-carrier `(Zero, Zero)` arm and refuses there because the
    //     curved containment door does not place both endpoints. So the
    //     operand below records which SIDE the sweep reaches first, not
    //     what the wall is about; a later crossing lane may move it
    //     again, and that is a measurement to re-take rather than a
    //     regression. What has not moved is the wall itself.
    //     M9-3 PR-A's rung teaches that arm to consult declarations —
    //     but the two-peg path it was measured on carries a PLANAR
    //     `Rest` at the rim plane as well, and the plant has none to
    //     offer: a stem passes THROUGH its corm, it does not sit on
    //     it. So the honest statement of this wall is narrow, testable
    //     and NOT about plants: a purely cylindrical mate, with no
    //     planar contact anywhere on it, does not reach the rest lane
    //     today. Filed as **#1032** with the measurement that isolates
    //     it — the refusal survives full engagement, partial
    //     engagement, and the two-peg fixture's own 3-arc face
    //     structure, so neither the minted rim nor the full-period
    //     face a revolve makes is the cause.
    let (corm_body, foot_body) = (by("lily_corm"), by("lily_foot"));
    let mut bore_decls = pncad::topo::BooleanDeclarations::none();
    for &fa in &axial_walls(corm_body, STEM_R) {
        for &fb in &axial_walls(foot_body, STEM_R) {
            bore_decls
                .coincident_faces
                .push(pncad::topo::FacePairDeclaration::new(
                    fa,
                    fb,
                    pncad::topo::ContactClass::Rest,
                ));
        }
    }
    wall(
        12,
        "thread the corm onto the stem's foot at their shared cylinder wall \
         (declared cylindrical Rest, no planar contact anywhere on the mate)",
        pncad::topo::union_with(corm_body, foot_body, &bore_decls, tol),
        // The KIND is the claim: the reduction's curved-face arm, at
        // an edge — NOT the declaration gate, which admitted the pair,
        // and not a carrier refusal. The operand is pinned too, as the
        // measurement of which side the sweep reaches first (see the
        // note above: it moved from A to B when the ring lane landed).
        |e| {
            matches!(
                e,
                BooleanError::CurvedPierceUnsupported {
                    operand: Operand::B,
                    ..
                }
            )
        },
        "give the plant a joined rootstock, and re-derive the two-peg cell's \
         claim about what a cylindrical mate needs beside it",
    );

    // 13 — RETIRED, and this is its retirement. It pinned the merge
    //      door SHUT on a full revolve's axis-touching caps: two
    //      half-faces on one plane key, which `merge_coplanar_faces`
    //      refused as `MergedFaceRoleAmbiguous` because its intra-face
    //      arm minted a ring from the surviving seam strut and the
    //      winding pass could then find no unique outline.
    //
    //      #1031's pole half opened it. The strut is not a ring: the
    //      cap's two seam edges are the two halves of the disc's
    //      DIAMETER, so the pole is interior to one straight carrier,
    //      `kev` removes it without changing any locus, and the cap
    //      comes back as ONE face. The wall's own retire note asked
    //      for exactly this — "make a revolve with an axis-touching
    //      flat cap usable as a boolean operand, and re-derive probe
    //      7's blocker sentence" — and both halves are done: probe 7
    //      now runs on the repaired body and names the door after.
    //
    //      What replaces the wall is the measurement it becomes: the
    //      repair, asserted on this scene's own lantern.
    {
        let mut before = lant.clone();
        let (f0, v0, e0) = (
            before.faces().count(),
            before.vertices().count(),
            before.edges().count(),
        );
        let outcome = before
            .merge_coplanar_faces(tol)
            .expect("probe 13 RETIRED: the lantern's caps now merge");
        println!(
            "   wall 13 — RETIRED: the lantern's two axis-touching caps MERGE \
             ({} group(s), {} skipped); faces {f0} -> {}, vertices {v0} -> {}, \
             edges {e0} -> {}",
            outcome.groups.len(),
            outcome.skipped.len(),
            before.faces().count(),
            before.vertices().count(),
            before.edges().count()
        );
        assert_eq!(outcome.groups.len(), 2, "both caps repair");
        assert_eq!(before.faces().count(), f0 - 2, "each cap became ONE face");
        assert_eq!(
            before.vertices().count(),
            v0 - 2,
            "each pole went with its seam"
        );
    }

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
        plant::<f64>(Tol::witness())
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
    ///
    /// The half-band assertion below is what licenses the weld path's
    /// [`super::torus_carrier`] to take the FIRST torus face it finds
    /// and call it the body's: this row is the check that there is
    /// only one carrier to find.
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
                    // Both torus half-bands must share ONE carrier, and
                    // that means EVERY component of it: a comparison
                    // that skips a component is a comparison two
                    // different tori can pass, which is exactly what
                    // this assertion exists to rule out.
                    let (pc, pa, pr, pm, pu): &(Point3<f64>, Vec3<f64>, f64, f64, Vec3<f64>) = prev;
                    assert!(
                        (*pc - t.0).norm() < 1e-15,
                        "two torus faces, two centers: {pc:?} vs {:?}",
                        t.0
                    );
                    assert!(
                        (*pa - t.1).norm() < 1e-15,
                        "two torus faces, two axes: {pa:?} vs {:?}",
                        t.1
                    );
                    assert!(
                        (*pu - t.4).norm() < 1e-15,
                        "two torus faces, two u_ref: {pu:?} vs {:?}",
                        t.4
                    );
                    assert!(
                        (pr - t.2).abs() < 1e-15 && (pm - t.3).abs() < 1e-15,
                        "two torus faces, two radius pairs: ({pr}, {pm}) vs ({}, {})",
                        t.2,
                        t.3
                    );
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
    /// The flower's globe centre: P2 (the arch's own last spine
    /// point, since the flower is welded there and not set back) plus
    /// the neck's drop plus `FLOWER_TOP` along T2.
    const SPHERE1_C: (f64, f64) = (-2.3668444700923885, 0.7942577551075498);

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

    /// Claim: the lantern axes ARE the stem tangents, and the globe
    /// sits at attach + (neck drop + top)·dir — read off the stored
    /// sphere and the two cones (neck and pucker), not off the
    /// construction code.
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
            let mut cones = 0usize;
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
                        cones += 1;
                        assert!(
                            cross_norm(*axis, tv) < 1e-14,
                            "{name}: cone axis || tangent"
                        );
                    }
                    _ => {}
                }
            }
            assert!(
                saw_sphere && cones == 4,
                "{name}: sphere zone between a neck cone and a conical pucker, \
                 both coaxial with the stem tangent and both halved at the \
                 revolve seam — saw {cones} cone faces"
            );
        }
    }

    /// **The declared weld's door sequence, measured** — VERBS-LILYWELD
    /// PR-2's opening measurement, kept as the record it is.
    ///
    /// PR-1 authored the flower/arch junction circle-coincident and
    /// left wall 2 pinned on the operand gate. The obvious next
    /// question is what a DECLARED union would do, and the answer
    /// today is: exactly what the undeclared one does. The scene's
    /// own `flush_declarations` DOES find the contact — the lantern's
    /// two throat-disk half-faces against the arch's end cap, an
    /// exact coincident planar Rest pair — and the union still
    /// refuses with the identical payload, because `gate_operand_pairs`
    /// runs on KINDS before any declaration is consulted.
    ///
    /// That is the pin: **declaring the weld changes nothing today**,
    /// and the differential between the declared and undeclared calls
    /// is empty. When the operand gate learns declared cone×torus,
    /// this row is what will show the two calls separating.
    #[test]
    fn the_declared_weld_refuses_exactly_as_the_undeclared_one_does() {
        let tol = Tol::witness();
        let ps = pieces();
        let (lant, arch) = (body(&ps, "lily_lantern"), body(&ps, "lily_arch"));
        let decls = crate::booleans::flush_declarations(lant, arch, tol);
        println!(
            "declared coincident face pairs: {:?}",
            decls.coincident_faces
        );
        assert_eq!(
            decls.coincident_faces.len(),
            2,
            "the throat disk arrives as two half-faces on one plane key, so the \
             flush contact against the arch's single end cap is two pairs"
        );
        let declared = pncad::topo::union_with(lant, arch, &decls, tol)
            .expect_err("the declared weld still refuses");
        let undeclared =
            pncad::topo::union(lant, arch, tol).expect_err("the undeclared weld still refuses");
        println!("declared:   {declared:?}\nundeclared: {undeclared:?}");
        assert_eq!(
            format!("{declared:?}"),
            format!("{undeclared:?}"),
            "the operand gate reads kinds before declarations, so these must be \
             the SAME refusal until the gate learns the declared pair"
        );
        assert!(
            matches!(
                declared,
                BooleanError::CurvedPairUnsupported {
                    op: None,
                    operand: Operand::A,
                    kind: SurfaceKind::Cone,
                    other_kind: SurfaceKind::Torus,
                    ..
                }
            ),
            "{declared:?}"
        );
    }

    /// **The lantern's two pole-split caps, named face by face** —
    /// the shape the F7 rule used to call a defect and no longer does.
    ///
    /// The lantern carries two PLANAR same-key adjacencies, the LIP
    /// disk and the THROAT disk, each a full revolve's cap that
    /// touches the axis and therefore arrives as two half-faces on one
    /// plane key. Its CURVED same-key adjacencies (two cones, one
    /// sphere zone) are the same structure with a different carrier.
    /// This row asserts the split — four planar half-faces, six curved
    /// — so a change that stopped telling the two apart fails here
    /// rather than silently.
    ///
    /// The MERGE door is now OPEN on exactly this shape: each cap's
    /// two seam edges are the halves of the disc's diameter, so the
    /// pole is interior to one straight carrier and
    /// `merge_coplanar_faces` repairs the pair to ONE face. This row
    /// pins both halves — the split as it arrives, and the repair.
    #[test]
    fn the_lanterns_two_pole_split_caps() {
        let tol = Tol::witness();
        let ps = pieces();
        let lant = body(&ps, "lily_lantern");
        let mut planar_pairs = 0usize;
        let mut curved_pairs = 0usize;
        for (_, e) in lant.edges() {
            let face_of = |he| {
                let p = lant.get_half_edge(he)?.parent_loop;
                Some(lant.get_loop(p)?.face)
            };
            let (Some(f1), Some(f2)) = (face_of(e.he_plus), face_of(e.he_minus)) else {
                continue;
            };
            if f1 == f2 {
                continue;
            }
            let (k1, k2) = (
                lant.get_face(f1).map(|f| f.surface),
                lant.get_face(f2).map(|f| f.surface),
            );
            if k1.is_none() || k1 != k2 {
                continue;
            }
            match k1.and_then(|k| lant.get_surface(k)) {
                Some(Surface::Plane { .. }) => planar_pairs += 1,
                Some(_) => curved_pairs += 1,
                None => {}
            }
        }
        // Each defect is counted from both of its two struts.
        assert_eq!(
            (planar_pairs, curved_pairs),
            (4, 6),
            "two planar caps (the lip disk and the throat disk) and three curved \
             walls (two cones, one sphere zone), each split at its seam"
        );
        let mut repaired = lant.clone();
        let outcome = repaired
            .merge_coplanar_faces(tol)
            .expect("the pole-split caps repair (#1031's pole half)");
        assert_eq!(outcome.groups.len(), 2, "both caps merged");
        assert_eq!(
            (repaired.faces().count(), repaired.vertices().count()),
            (8, 8),
            "each cap became one face and each pole went with its seam"
        );
        // `topo::validate` is the TIER 1 validator; the claim here is
        // about the repaired body's tier 2 and tier 3 standing, so it
        // runs those (delta review MIN-1: the check and the message
        // disagreed, and the message is what people read).
        assert_eq!(
            pncad::topo::validate_closed(&repaired),
            Ok(()),
            "tier 2 after repair"
        );
        assert_eq!(
            pncad::topo::validate_geometric(&repaired, tol),
            Ok(()),
            "tier 3 after repair"
        );
    }

    /// DELTA probe (ordinal-104 verification, `verbs/f7d-probes`),
    /// ADOPTED: the row above used to label a TIER 1 check "tier 3".
    /// That row is fixed, and this one stands beside it running both
    /// real validators, so the PR body's "tier 3 clean" claim is
    /// measured in-tree rather than inherited from a dev-run log.
    #[test]
    fn f7d_delta_repaired_lantern_actual_tiers() {
        let tol = Tol::witness();
        let ps = pieces();
        let lant = body(&ps, "lily_lantern");
        let mut repaired = lant.clone();
        repaired
            .merge_coplanar_faces(tol)
            .expect("the pole-split caps repair");
        assert_eq!(
            pncad::topo::validate_closed(&repaired),
            Ok(()),
            "tier 2 after repair (actual)"
        );
        assert_eq!(
            pncad::topo::validate_geometric(&repaired, tol),
            Ok(()),
            "tier 3 after repair (actual)"
        );
    }

    /// **The weld, as geometry.** The flower's neck circle and the
    /// arch tube's terminal meridian circle are ONE circle, each
    /// computed to closed form off its own body's stored carrier
    /// (`weld_circle`, which the scene also runs live). Checked here
    /// as well, for the reason the other boolean walls are: a scene
    /// claim must not cost a whole render pass to re-measure.
    ///
    /// The circle is then said a third way, against the reviewer's
    /// independent turtle algebra: it is centred on P2 with normal
    /// T2 and radius exactly the arch tube's. That is the whole
    /// content of #1059 — the lantern's axis is the stem's own
    /// tangent, so the cone cut at the tube's minor radius meets the
    /// meridian circle at that station exactly.
    #[test]
    fn the_flower_and_the_arch_share_one_circle() {
        let ps = pieces();
        let (_, circle, res, runner_up) = weld_circle(
            body(&ps, "lily_lantern"),
            body(&ps, "lily_arch"),
            caps(&ps, "lily_arch"),
        );
        println!(
            "weld circle: centre {:?}, r {}, normal {:?}; meridian residuals \
             (off-spine, radius — definitional — normal) {res:?}; nearest \
             DISTINCT station circle (the neck cone's own other nappe) misses \
             by {runner_up:e}",
            circle.c, circle.r, circle.n
        );
        assert_eq!(circle.r, ARCH_R, "the weld circle is the tube's own");
        assert!(
            (circle.c.0 - P2.0).abs() < 1e-12 && (circle.c.2 - P2.1).abs() < 1e-12,
            "the weld circle is centred on the arch's last spine point"
        );
        assert!(
            circle.c.1.abs() < 1e-15,
            "the weld circle's centre is in the plant's own plane"
        );
        assert!(
            cross_norm(
                Vec3::new(circle.n.0, circle.n.1, circle.n.2),
                Vec3::new(T2.0, 0.0, T2.1)
            ) < 1e-14,
            "the weld circle's normal is the stem tangent there"
        );
    }

    /// **A live refusal, banked where it was met.** The neck a
    /// derivation would pick is the globe's own TANGENT cone at the
    /// truncation circle — half-angle `atan(top / r_top)`, a G1
    /// shoulder, apex at the truncation plane's pole in the sphere.
    /// The authoring algebra will not take it: a leg whose departure
    /// is spelled in COORDINATES and lands within ε of the incoming
    /// tangent is `PathError::JunctionTangent`, and the refusal names
    /// its one recourse — say the tangency STRUCTURALLY, with the
    /// `.tangent()` verb, which makes it exact by construction rather
    /// than by arithmetic that happens to agree.
    ///
    /// So this is not a gap: it is the algebra declining to infer an
    /// intent it has a door for. The scene does not walk through that
    /// door because it does not want a G1 shoulder
    /// ([`FLOWER_NECK_HALF_ANGLE`]); the refusal is pinned here so
    /// that reasoning stays checkable and the margin stays visible.
    #[test]
    fn the_globes_tangent_cone_neck_is_refused_by_the_junction_gate() {
        let tol = Tol::witness();
        let (globe, top) = (FLOWER_GLOBE, FLOWER_TOP);
        let r_top = (globe.powi(2) - top.powi(2)).sqrt();
        // The tangent cone's own half-angle, and the drop that follows.
        let tangent_alpha = (top / r_top).atan();
        let drop = neck_drop(globe, top, ARCH_R, tangent_alpha);
        let out = p2::<f64>(0.0, 0.0);
        let refusal = Open
            .at(out)
            .line_to(p2(ARCH_R, 0.0), tol)
            .expect("throat disk")
            .line_to(p2(r_top, drop), tol)
            .expect("the tangent neck's own leg authors fine")
            .arc_to(
                Center {
                    c: p2(0.0, drop + top),
                    winding: ArcSweep::Ccw,
                    p: p2((globe.powi(2) - 0.36_f64.powi(2)).sqrt(), drop + top + 0.36),
                },
                tol,
            )
            .expect_err("a tangent belly departure is refused");
        println!("the tangent-cone neck answers {refusal:?}");
        assert!(
            matches!(
                refusal,
                pncad::profile::PathError::JunctionTangent { margin, arm }
                    if margin.abs() < 1e-15 && arm > 0.1
            ),
            "the tangent neck must refuse as JunctionTangent with a vanishing \
             turn margin on a real lever arm: {refusal:?}"
        );
    }

    /// The re-authored lantern's CENSUS and its exact mass, against
    /// the closed form of the solid of revolution it is.
    ///
    /// The census is what the re-authoring changed structurally: the
    /// meridian gained one segment (the neck cone) between the throat
    /// disk and the belly, and a FULL revolve emits every wall as two
    /// half-bands on one carrier, so the neck arrives as two faces
    /// with the seam struts and rim vertices that go with them. The
    /// exact volume is the same closed form
    /// `finding_13_tessellation_table_reproduces` measures the mesh
    /// against, taken here from the kernel's own `mass_properties`
    /// door rather than from a tessellation — so the two rows are
    /// independent readings of one number.
    #[test]
    fn the_lanterns_census_and_mass_are_the_re_authored_ones() {
        let ps = pieces();
        let lant = body(&ps, "lily_lantern");
        let census = (
            lant.shells().count(),
            lant.faces().count(),
            lant.edges().count(),
            lant.vertices().count(),
        );
        println!("lantern census (shells, faces, edges, vertices) = {census:?}");
        assert_eq!(census, (1, 10, 18, 10), "the re-authored lantern's census");
        let props = pncad::topo::mass_properties(lant, Tol::witness()).expect("mass properties");
        // The literal, DELIBERATELY: this row and
        // `finding_13_tessellation_table_reproduces` must not agree by
        // sharing an expression. That row builds the closed form from
        // the profile's own constants and compares a MESH against it;
        // this one compares the kernel's `mass_properties` door
        // against a transcribed number, so a change to the closed-form
        // expression cannot move both readings together.
        let exact = 0.36455193285177373;
        assert!(
            (props.volume - exact).abs() < 1e-12,
            "lantern volume {} vs the closed form {exact}",
            props.volume
        );
        assert_eq!(props.volume_pad, 0.0, "every lantern face is closed-form");
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
            ("lily_lantern", 5e-3, 1_084),
            ("lily_lantern", 2e-3, 2_560),
            ("lily_leaf_b", 2e-3, 468),
            ("lily_leaf_c", 2e-3, 414),
        ];
        // Measured first, compared once: a row-at-a-time assert stops
        // at the first move and hides the rest, and this table is read
        // as a whole.
        let got: Vec<(&str, f64, usize)> = table
            .iter()
            .map(|&(name, delta, _)| {
                let m = pncad::mesh::tessellate(body(&ps, name), delta, Tol::witness())
                    .expect("tessellate");
                (name, delta, triangle_count(&m))
            })
            .collect();
        println!("finding 13, measured: {got:?}");
        assert_eq!(got, table.to_vec(), "the finding-13 tessellation table");
        // Lantern volume error at both deltas. The exact figure is the
        // authored solid of revolution in closed form — the zone
        // integral between the two truncations, plus the two conical
        // frusta (the NECK above it and the pucker below) — so it is
        // derived from the same numbers the profile is drawn from
        // rather than transcribed:
        //   π[r²(a+b) − (a³+b³)/3] + π·h(R² + Rρ + ρ²)/3 per frustum.
        // It comes to 0.36455193285177373 m³.
        let (globe, top, mouth, lip_r, lip_drop): (f64, f64, f64, f64, f64) =
            (FLOWER_GLOBE, FLOWER_TOP, 0.36, 0.09, 0.16);
        let r_top = (globe.powi(2) - top.powi(2)).sqrt();
        let r_mouth = (globe.powi(2) - mouth.powi(2)).sqrt();
        let frustum = |h: f64, r0: f64, r1: f64| PI * h * r0.mul_add(r1, r0 * r0 + r1 * r1) / 3.0;
        let exact = PI * (globe.powi(2) * (top + mouth) - (top.powi(3) + mouth.powi(3)) / 3.0)
            + frustum(
                neck_drop(globe, top, ARCH_R, FLOWER_NECK_HALF_ANGLE),
                ARCH_R,
                r_top,
            )
            + frustum(lip_drop, r_mouth, lip_r);
        let rels: Vec<(f64, f64)> = [5e-3, 2e-3]
            .into_iter()
            .map(|delta| {
                let m = pncad::mesh::tessellate(body(&ps, "lily_lantern"), delta, Tol::witness())
                    .expect("tessellate");
                (delta, ((signed_volume(&m) - exact) / exact).abs())
            })
            .collect();
        println!("lantern volume error, measured: {rels:?}");
        for ((delta, rel), (lo, hi)) in rels.into_iter().zip([(0.0120, 0.0130), (0.0050, 0.0056)]) {
            assert!(rel > lo && rel < hi, "lantern @ {delta:e}: rel {rel}");
        }
        // A swept blade has no analytic wall to compare against, but it
        // has PAPPUS. A rigid section carried in the path's normal
        // frame sweeps A·(centroid arc length); the kite of chord `w`
        // with rises `ridge`/`keel` has area w(ridge+keel)/2 and its
        // centroid sits (ridge−keel)/3 above the chord, i.e. that far
        // OUTSIDE the spine's centre of curvature, so its arc is
        // len + |curl|·(ridge−keel)/3. Agreement to a couple of 1e-4
        // is the mesh's chord error at δ = 2e-3 under the aspect-capped
        // split schedule (TESS-SPLIT: the blades carry roughly a
        // quarter of their old triangles, all still certified inside
        // δ), and it is a two-sided band: exact agreement would mean
        // the volume was not measured off a real tessellation, and a
        // larger gap would mean the section rolled about the tangent
        // on its way down the path.
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
            let m =
                pncad::mesh::tessellate(body(&ps, name), 2e-3, Tol::witness()).expect("tessellate");
            let rel = ((signed_volume(&m) - pappus) / pappus).abs();
            assert!(rel > 5e-5 && rel < 4e-4, "{name}: rel {rel}");
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
            Tol::witness(),
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
            let (bc, br, _) = sphere_of(body(&ps, seg));
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
                let (center, _, axis) = sphere_of(body(&ps, n));
                // Stored axes may point either way along the line;
                // orient them all into the bud.
                let a = if axis.dot(bud_axis) < 0.0 {
                    -axis
                } else {
                    axis
                };
                (center, a)
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

    /// The lantern's PUCKER cone faces — every conical face whose
    /// carrier is not the NECK's.
    ///
    /// The lantern has two conical walls since the flower/arch weld
    /// was authored circle-coincident, and a full revolve halves each
    /// wall at its seam, so "the cone" is now four faces on two
    /// carriers. Wall 7's measurement is about the pucker alone (it
    /// is the wall the carving ball is near); the neck sits at the
    /// other end of the flower and folding it into the same box would
    /// measure a different thing. The neck is named by the weld pin,
    /// so the two walls are told apart by carrier and not by a
    /// threshold.
    pub(super) fn pucker_cone_faces(ps: &[Piece<f64>]) -> Vec<pncad::topo::FaceKey> {
        let lant = body(ps, "lily_lantern");
        let (neck, _, _, _) = weld_circle(lant, body(ps, "lily_arch"), caps(ps, "lily_arch"));
        let neck_apex = lant
            .faces()
            .find(|(k, _)| *k == neck)
            .and_then(|(_, f)| match lant.get_surface(f.surface) {
                Some(&Surface::Cone { apex, .. }) => Some(apex),
                _ => None,
            })
            .expect("the neck the weld pin names is a cone");
        lant.faces()
            .filter(|(_, f)| {
                matches!(
                    lant.get_surface(f.surface),
                    Some(&Surface::Cone { apex, .. }) if (apex - neck_apex).norm() > 1e-9
                )
            })
            .map(|(k, _)| k)
            .collect()
    }

    /// Everything a stored sphere holds: (centre, radius, axis,
    /// `u_ref`). Named so the agreement check below can compare the
    /// whole of one against the whole of another.
    type SphereCarrier = (Point3<f64>, f64, Vec3<f64>, Vec3<f64>);

    /// The body's single stored sphere: (centre, radius, axis).
    ///
    /// Single is CHECKED, not assumed. Every caller reads one sphere
    /// face and treats what it stores as the body's, so a second face
    /// carrying a different sphere would make that reading arbitrary —
    /// the same licence [`torus`] asserts for its own two half-bands,
    /// and asserted the same way: over the WHOLE carrier, `u_ref`
    /// included, because a partial comparison is one two different
    /// spheres can pass.
    fn sphere_of(b: &Body<f64>) -> (Point3<f64>, f64, Vec3<f64>) {
        let mut found: Option<SphereCarrier> = None;
        for (_, f) in b.faces() {
            if let Some(pncad::geom::Surface::Sphere {
                center,
                radius,
                axis,
                u_ref,
            }) = b.get_surface(f.surface)
            {
                let s = (*center, *radius, *axis, *u_ref);
                match found {
                    Some(p) => assert!(
                        (p.0 - s.0).norm() < 1e-15
                            && (p.1 - s.1).abs() < 1e-15
                            && (p.2 - s.2).norm() < 1e-15
                            && (p.3 - s.3).norm() < 1e-15,
                        "two sphere faces, two carriers: {p:?} vs {s:?}"
                    ),
                    None => found = Some(s),
                }
            }
        }
        let (center, radius, axis, _) = found.expect("body stores no sphere");
        (center, radius, axis)
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
                Tol::witness(),
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
                Tol::witness(),
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

#[cfg(test)]
mod verbs_gate_r1_probes {
    //! Reviewer probes (PR #1001 r1): the wall-7 steering correction,
    //! measured with the reviewer's own arithmetic rather than the
    //! kernel's boxes.

    use super::*;
    use pncad::topo::Surface;

    /// The wall-7 finding, re-derived with the reviewer's own
    /// arithmetic — and the measurement REVERSES the reading the wall
    /// text invites. The refusal names (Cone, Sphere): the pucker's
    /// SLAB BOX overlaps the carving ball's box, and that is real —
    /// but the pucker's EXACT frustum never comes within the ball's
    /// radius of it, while the sphere ZONE's carrier does meet the
    /// ball. So the pair the gate names is pure box looseness (the
    /// cone slab claims max-generator radius along its whole axial
    /// range); the geometry the model cares about is still
    /// sphere-on-sphere, and a tighter cone box would restore the
    /// original steering premise (waits on item 9) without any cone
    /// germ lane.
    #[test]
    fn wall7_the_cone_pair_is_box_looseness_the_ball_meets_the_zone() {
        let tol = Tol::witness();
        let pieces = plant::<f64>(tol);
        let lant = &pieces
            .iter()
            .find(|p| p.name == "lily_lantern")
            .expect("lantern piece")
            .body;
        // The carving ball of wall 7, in its own numbers.
        let (bc, br) = (Point3::new(-2.80, 0.0, 0.90), 0.16);
        let pucker = super::review_probes::pucker_cone_faces(&pieces);
        let mut min_frustum_gap = f64::INFINITY;
        let mut zone_hit = false;
        for (k, f) in lant.faces() {
            match lant.get_surface(f.surface) {
                Some(&Surface::Cone {
                    apex,
                    axis,
                    half_angle,
                    ..
                }) if pucker.contains(&k) => {
                    let mut v_lo = f64::INFINITY;
                    let mut v_hi = f64::NEG_INFINITY;
                    for lk in core::iter::once(f.outer).chain(f.rings.iter().copied()) {
                        let Some(l) = lant.get_loop(lk) else { continue };
                        let pncad::topo::LoopBoundary::Cycle { first } = l.boundary else {
                            continue;
                        };
                        for he in lant.loop_cycle(first).expect("walkable lantern loop") {
                            let v = lant.get_half_edge(he).expect("half-edge").start;
                            let p = *lant
                                .get_vertex(v)
                                .and_then(|vd| lant.get_point(vd.point))
                                .expect("vertex point");
                            let h = (p - apex).dot(axis);
                            v_lo = v_lo.min(h);
                            v_hi = v_hi.max(h);
                        }
                    }
                    let w = bc - apex;
                    let h = w.dot(axis);
                    let rad = (w - axis * h).norm();
                    let seg = |h0: f64, r0: f64, h1: f64, r1: f64| -> f64 {
                        let (dx, dy) = (h1 - h0, r1 - r0);
                        let t = (((h - h0) * dx + (rad - r0) * dy) / (dx * dx + dy * dy))
                            .clamp(0.0, 1.0);
                        let (px, py) = (h0 + t * dx, r0 + t * dy);
                        ((h - px).powi(2) + (rad - py).powi(2)).sqrt()
                    };
                    let ta = half_angle.tan();
                    let d = seg(v_lo, v_lo.abs() * ta, v_hi, v_hi.abs() * ta) - br;
                    min_frustum_gap = min_frustum_gap.min(d);
                }
                Some(&Surface::Sphere { center, radius, .. }) => {
                    let d = (center - bc).norm();
                    if d <= radius + br && d + br >= radius {
                        zone_hit = true;
                    }
                }
                _ => {}
            }
        }
        println!(
            "wall-7 probe: min frustum-to-ball surface gap = {min_frustum_gap:.4} \
             (positive = clear); sphere zone carrier meets ball = {zone_hit}"
        );
        assert!(
            min_frustum_gap > 0.0,
            "the pucker's exact frustum reaches the ball after all — the box-artifact \
             reading is wrong, re-derive"
        );
        assert!(
            zone_hit,
            "the carving ball no longer meets the sphere zone — the wall is not even \
             asking a sphere-on-sphere question any more"
        );

        // **The amendment (r1 fix pass), and it is a NEGATIVE result
        // stated as one.** The measurement above is the reviewer's,
        // unchanged: the pucker's exact frustum clears the carving
        // ball, and the sphere zone meets it. The cone arm now boxes
        // the FRUSTUM its axial window cuts rather than a slab pinned
        // at the window's widest radius — a real tightening, measured
        // below — and it is STILL not enough to separate this pair.
        //
        // What is left is not the constant-radius artifact the
        // reviewer measured. It is the AABB of a TILTED frustum: an
        // axis-aligned box around a slanted cone is bigger than the
        // cone, and no per-kind box construction can close that. So
        // the gate keeps naming (Cone, Sphere), honestly — "may
        // intersect" is exactly the claim it makes, and the two loci
        // do not.
        //
        // The residual is measured rather than asserted away: this
        // row prints the frustum's own AABB against the ball's and
        // the per-axis overlap, so the day an ORIENTED-box door or an
        // exact cone×sphere separation test lands, the number to beat
        // is written down.
        let (fa, fb) = frustum_aabb(lant, &pucker, (bc, br));
        let overlap = |lo_a: f64, hi_a: f64, lo_b: f64, hi_b: f64| hi_a.min(hi_b) - lo_a.max(lo_b);
        let per_axis = [
            overlap(fa.0.x, fa.1.x, fb.0.x, fb.1.x),
            overlap(fa.0.y, fa.1.y, fb.0.y, fb.1.y),
            overlap(fa.0.z, fa.1.z, fb.0.z, fb.1.z),
        ];
        let tightest = per_axis.iter().fold(f64::INFINITY, |m, v| m.min(*v));
        println!(
            "wall-7 probe: frustum AABB {fa:?} vs ball AABB {fb:?}; per-axis overlap \
             {per_axis:?} (tightest {tightest:.4}) — the exact loci clear by \
             {min_frustum_gap:.4}, so this overlap is AABB looseness on a tilted \
             frustum, not contact"
        );
        let ball_body = ball::<f64>((-2.80, 0.90), 0.16, tol);
        let mut repaired = lant.clone();
        repaired
            .merge_coplanar_faces(tol)
            .expect("the lantern's caps repair (#1031's pole half)");
        let refusal = pncad::topo::subtract(&repaired, &ball_body, tol)
            .expect_err("the tepal seam is still refused, somewhere");
        println!("wall-7 probe: the kernel answers {refusal:?}");
        // **The measured outcome, and it is neither branch the review
        // anticipated.** With the axial window taken from the
        // boundary's own locus, the pucker's box clears the ball by
        // {tightest} on its tightest axis, so the pair-scoped gate
        // ADMITS — and what answers is not a germ class at all but
        // the operand-shape precondition. WHICH faces, corrected at
        // M9-5 and re-measured here: NOT the zone's two half-bands —
        // same-key CURVED adjacency is the canonical maximal form —
        // but the lantern's two AXIS-TOUCHING PLANAR CAPS.
        //
        // So the sphere×sphere germ arm is not even reached. This
        // wall's dependency is #1031's pole half (the repair op), and
        // only THEN row 9.
        assert!(
            tightest < 0.0,
            "the pucker's box must clear the ball's for the gate to admit; it does \
             not, so this row's reading of the refusal below is wrong"
        );
        assert!(
            matches!(refusal, BooleanError::CurvedPierceUnsupported { .. }),
            "the gate admits and the REPAIRED lantern is maximal-faced, so what \
             refuses is the curved pierce arm — got {refusal:?}"
        );
    }

    /// The plant's other two boolean walls, as a TEST — same reason
    /// the bottle's are (`klein::verbs_gate_r1_probes`): the gate
    /// names a PAIR, so a change under the boxes moves them, and
    /// re-measuring must not cost a whole render pass.
    #[test]
    fn the_plants_other_boolean_walls_name_the_pairs_the_scene_claims() {
        let tol = Tol::witness();
        let pieces = plant::<f64>(tol);
        let by = |name: &str| {
            &pieces
                .iter()
                .find(|p| p.name == name)
                .expect("named lily piece")
                .body
        };
        let (stem, arch, lant) = (by("lily_stem"), by("lily_arch"), by("lily_lantern"));

        let glued = crate::booleans::try_union_declared(stem, arch, tol)
            .expect_err("the stem's two arcs still cannot be glued");
        println!("lily wall 1: {glued:?}");
        assert!(
            matches!(
                glued,
                BooleanError::CurvedPairUnsupported {
                    op: None,
                    operand: Operand::A,
                    kind: SurfaceKind::Torus,
                    other_kind: SurfaceKind::Plane,
                    ..
                }
            ),
            "wall 1 must name the stem's tube wall against a planar disc of the arch: \
             {glued:?}"
        );
        // **What this pair actually is, measured.** The gate names the
        // stem's tube wall against the arch's FAR cap — the disc at the
        // top of the arch, metres from anything the stem occupies. The
        // two exact loci never come near each other; what overlaps is
        // the stem wall's BOX, which for a torus is the whole tube
        // about the ring centre and reads nothing from the face's
        // boundary, so a 22° arc of a 5 m ring is boxed as the entire
        // 10 m ring.
        //
        // So wall 1 is not a germ-class wall and never was: no arm is
        // missing for a pair that does not meet. It is the box
        // artifact the cone arm already had fixed (its slab became the
        // frustum its window cuts) and the torus arm has not.
        //
        // The weld's own contact — the stem's end disc against the
        // arch's start disc — is plane×plane, declared and verified;
        // the tube walls take no part in it, because the arch's tube
        // is thinner than the stem's and the two walls share nothing
        // but the plane they both end on.
        let arch_far_cap = arch
            .faces()
            .filter_map(|(k, f)| match arch.get_surface(f.surface) {
                Some(&Surface::Plane { origin, .. }) => Some((k, origin)),
                _ => None,
            })
            .find(|&(_, o)| (o - pncad::geom_core::Point3::new(0.0, 0.0, 0.0)).norm() > 2.0)
            .expect("the arch carries a cap plane clear of the weld");
        // Unconditional on both halves. Under an `if let` this row
        // SELF-DISABLES the moment the refusal's shape changes — which
        // is exactly when the claim it makes needs re-reading, so the
        // one arrangement that must not be used is the one that goes
        // quiet then.
        let BooleanError::CurvedPairUnsupported { other_face, .. } = &glued else {
            panic!("wall 1's refusal is the operand gate's, or this reading is stale: {glued:?}");
        };
        assert_eq!(
            *other_face, arch_far_cap.0,
            "the pair the gate names is the stem's wall against the arch's FAR cap — \
             a box overlap, not a contact"
        );

        let welded = pncad::topo::union(lant, arch, tol)
            .expect_err("the lantern still cannot be welded to the arch");
        println!("lily wall 2: {welded:?}");
        assert!(
            matches!(
                welded,
                BooleanError::CurvedPairUnsupported {
                    op: None,
                    operand: Operand::A,
                    kind: SurfaceKind::Cone,
                    other_kind: SurfaceKind::Torus,
                    ..
                }
            ),
            "wall 2 must name a lantern CONE against the arch's tube — the pair the \
             gate has no arm for, with the two loci sharing one circle: {welded:?}"
        );
    }

    /// The axis-aligned box of the named cone frusta of the lantern,
    /// and the carving ball's — the kernel's own two constructions,
    /// re-derived here so the residual looseness is measured by an
    /// outside consumer rather than read out of the module under
    /// test. `faces` is which cone the caller means; the lantern
    /// carries two walls' worth.
    /// One axis-aligned box as (lo, hi).
    type Aabb = (Point3<f64>, Point3<f64>);

    fn frustum_aabb(
        lant: &Body<f64>,
        faces: &[pncad::topo::FaceKey],
        ball: (Point3<f64>, f64),
    ) -> (Aabb, Aabb) {
        let (bc, br) = ball;
        let mut lo = Point3::new(f64::INFINITY, f64::INFINITY, f64::INFINITY);
        let mut hi = Point3::new(f64::NEG_INFINITY, f64::NEG_INFINITY, f64::NEG_INFINITY);
        for (k, f) in lant.faces() {
            if !faces.contains(&k) {
                continue;
            }
            let Some(&Surface::Cone {
                apex,
                axis,
                half_angle,
                ..
            }) = lant.get_surface(f.surface)
            else {
                continue;
            };
            let (mut v_lo, mut v_hi) = (f64::INFINITY, f64::NEG_INFINITY);
            for lk in core::iter::once(f.outer).chain(f.rings.iter().copied()) {
                let Some(l) = lant.get_loop(lk) else { continue };
                let pncad::topo::LoopBoundary::Cycle { first } = l.boundary else {
                    continue;
                };
                for he in lant.loop_cycle(first).expect("walkable lantern loop") {
                    let v = lant.get_half_edge(he).expect("half-edge").start;
                    let p = *lant
                        .get_vertex(v)
                        .and_then(|vd| lant.get_point(vd.point))
                        .expect("vertex point");
                    let h = (p - apex).dot(axis);
                    v_lo = v_lo.min(h);
                    v_hi = v_hi.max(h);
                }
            }
            let ta = half_angle.tan();
            let h0 = v_lo.max(0.0).min(v_hi);
            let comp = |ai: f64, oi: f64| {
                let k = ta * (1.0 - ai * ai).max(0.0).sqrt();
                let g = |t: f64| (oi + t * ai - t.abs() * k, oi + t * ai + t.abs() * k);
                [g(v_lo), g(v_hi), g(h0)]
                    .into_iter()
                    .fold((f64::INFINITY, f64::NEG_INFINITY), |acc, (l, h)| {
                        (acc.0.min(l), acc.1.max(h))
                    })
            };
            let (x0, x1) = comp(axis.x, apex.x);
            let (y0, y1) = comp(axis.y, apex.y);
            let (z0, z1) = comp(axis.z, apex.z);
            lo = Point3::new(lo.x.min(x0), lo.y.min(y0), lo.z.min(z0));
            hi = Point3::new(hi.x.max(x1), hi.y.max(y1), hi.z.max(z1));
        }
        (
            (lo, hi),
            (
                Point3::new(bc.x - br, bc.y - br, bc.z - br),
                Point3::new(bc.x + br, bc.y + br, bc.z + br),
            ),
        )
    }
}
