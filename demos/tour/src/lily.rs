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
//! refused, in the [`crate::walls::wall`] style: each probe
//! ASSERTS its refusal and panics with instructions if the refusal
//! ever retires.
//!
//! What the lily IS, therefore:
//!
//! - the plant's underground half is its one JOINED body, the
//!   **rootstock** ([`rootstock`]): a **corm** — the swollen stem-base
//!   a *Calochortus* rises from each spring — threaded on the straight
//!   basal internode, the **foot**, and unioned with it. The corm is a
//!   sphere-zone swelling with a coaxial BORE at the stem's own
//!   diameter, so the two parts are cosurface along the whole bore:
//!   the plant's one CYLINDRICAL contact, with no planar contact
//!   anywhere on the mate. The union glues it into one shell whose
//!   volume is the two parts' sum.
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
//!   puckered conical mouth closing to a small disk, the rim where
//!   the globe meets the pucker rolled by `fillet_edges` into a torus
//!   band (probe 6, retired). Sphere zone + two cones + a torus band +
//!   two planes, all exact. The neck is an AUTHORED cone
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
//! - the two short **leaves** are keeled blades: a thin LANCEOLATE
//!   section — two circular arcs meeting at two sharp margins, an
//!   unequal ridge and keel across the chord — carried along a gently
//!   arching circular spine by the general-path SWEEP. The blade
//!   leaves the plane it was drawn in, which the extruded crescent
//!   could not do. Two things a sweep still cannot do: TAPER and ROLL.
//!   `sweep_body` takes one profile and derives its own frame, so
//!   there is no argument in which either could be asked for
//!   (findings entry 9).
//! - the long basal **leaf** and the three **sepals** are the same
//!   blade said as a LOFT, and they do both. The long leaf is one
//!   loft from sheath to tip: its spine curls up out of the stem, then
//!   arches over through the base section the other way. `loft_body` takes the
//!   sections and the placements as two lists, so the section may
//!   change station to station — the swept leaves' lens, narrow at
//!   the stem, wide and flat at the belly, small at the tip — and the frame
//!   may roll about the spine on the way. The long leaf turns 160
//!   degrees, eased hard toward the tip, which is what a real
//!   *Calochortus* leaf lying along the ground does. Both are pinned
//!   by measurement, not assertion: see
//!   `review_probes::the_lofted_blade_tapers_and_rolls_in_the_stored_geometry`.
//!
//!   What the loft still cannot do is close the tip to a POINT (a
//!   zero-width section has no chord — see [`Plan::tip`]) or be
//!   JOINED to the stem it grows from (probe 8). The tip lenses are
//!   therefore small but real, and left visibly so rather than shrunk
//!   until the blunt end stops showing.
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
//! sampled stations, so a blade's walls are B-spline surfaces —
//! rational on the swept leaves, whose sections are exact arcs —
//! interpolating exact points of an exact circular spine, not a
//! closed form of the swept or lofted solid. That is the price of
//! leaving the plane, and it is stated here rather than hidden. The
//! SEPALS' tangency to the globe is not fitted, though: the stand-off
//! is the section's own keel and the non-entry is a two-line argument
//! on [`sepals`], checked on the built solids by
//! `review_probes::the_sepals_stand_outside_the_globe_they_are_tangent_to`.
//!
//! # The layer the scene is composed at
//!
//! Every builder here is generic over the run scalar `S`, and every
//! number in the scene is an `f64` literal or an `f64` trig call. So
//! the scene is COMPOSED at `f64` — in `Vec3<f64>` and
//! `Point3<f64>`, through the kernel's own doors (`normalize`,
//! `dot`, `cross`, `reject_from`, `Mat3::rotation_about`, the
//! operators) — and each value is LIFTED to `S` at the door it is
//! handed to. `Real` carries no mixed-scalar arithmetic, so composing
//! at `S` would put an `S::from_f64` on every literal, and composing
//! in tuples would put a second vector algebra beside the kernel's
//! own.
//!
//! **The lift has two spellings and they divide on one line.** Where
//! the components are written at the door, it is the kernel's own
//! constructor — [`p2`]/`v2`/`p3`/`v3` from `pncad::authoring`. Where
//! an already-composed `f64` value crosses — a frame this file built,
//! a turtle's point, a stored carrier — it is `map(S::from_f64)`,
//! once, on the value. A sketch plane is a frame, and a frame crosses
//! no scalar upward: it is minted at `S`. Readback runs the other way by the same
//! rule: a stored `Point3<S>` becomes `f64` through `map(S::f)`, once,
//! where narration begins.
//!
//! **The swept and lofted blades are the exception**: their frames
//! and outlines are built from computed geometry (a spine, a curl), so
//! they enter `S` as the computed values they are — the frame's inputs
//! through `map(S::from_computed)` and minted at `S`, `lift_computed`
//! on each loop.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::f64::consts::PI;
use pncad::sweep::ExtrudeSide;

use pncad::geom_core::{
    Affine3, Mat3, Point2, Point3, RigidMotion, RigidMotionError, UnitVec3, Vec3,
};
use pncad::prelude::{CurveKind, Open, Start, SurfaceKind, SurfaceKindSet, query};
use pncad::profile::{ArcSweep, Center, ConstructedLoop, SketchPlane, Via};
use pncad::sweep::blend::fillet_edges;
use pncad::sweep::{
    ExtrudeError, Extrusion, Revolution, RevolveAxis, TubeWindow, WedgeFrames, extrude, loft_body,
    revolve, revolved_caps, sweep_body, tube_along_arc,
};
use pncad::topo::{
    AtRestBody, Body, BooleanBody, BooleanError, ContactRecords, EdgeCarrierSite, EdgeKey, Operand,
};

use crate::booleans::{check, expect_seamed, finished, try_union};
use crate::scalar::{Scalar, axis_frame, sketch_frame};
use crate::{SceneBody, TourStop, View};
use pncad::authoring::{p2, p3, v2, v3, validated};
use pncad::geom_core::{Tol, Turns};

// ---------------------------------------------------------------
// The turtle: a G1 chain of circular arcs in the world xz-plane.
// ---------------------------------------------------------------

/// A planar frame in the world xz-plane: a point and a unit tangent,
/// both with `y = 0`. The stem chain is a walk of these.
#[derive(Clone, Copy, Debug)]
struct Turtle {
    /// Position.
    p: Point3<f64>,
    /// Unit tangent.
    t: Vec3<f64>,
}

/// One arc of the walk, in the form [`tube_arc`] consumes: the ring
/// centre, the unit radial from that centre to the arc's START, the
/// ring radius, and the signed turn (positive = left, i.e. the
/// counterclockwise sense of the xz-plane drawn with +x right and
/// +z up).
#[derive(Clone, Copy, Debug)]
struct ArcSpec {
    center: Point3<f64>,
    radial: Vec3<f64>,
    ring: f64,
    turn: f64,
}

/// The turtle's own rotation: the walk's positive sense is
/// counterclockwise in the xz-plane drawn with +x right and +z up,
/// which is the right-handed rotation about **−ŷ** — the axis
/// [`ArcSpec`] states as the walk's convention and [`tube_arc`] hands
/// the kernel for a left turn. Said about +ŷ with the angle negated,
/// so the matrix is the kernel's own.
fn spin(turn: f64) -> Mat3<f64> {
    Mat3::rotation_about(Vec3::unit_y(), -turn)
}

impl Turtle {
    /// Turns through `turn` radians on a ring of radius `ring`,
    /// returning the arc and the advanced turtle. Positive `turn`
    /// curves left (centre on the left of travel).
    fn arc(self, ring: f64, turn: f64) -> (ArcSpec, Self) {
        // The left normal of the travel direction is `t x ŷ`; the
        // centre sits one ring radius along it, signed by the turn.
        let n = if turn >= 0.0 {
            self.t.cross(Vec3::unit_y())
        } else {
            Vec3::unit_y().cross(self.t)
        };
        let center = self.p + n * ring;
        let radial = (self.p - center) / ring;
        let rotate = spin(turn);
        let advanced = rotate * radial;
        (
            ArcSpec {
                center,
                radial,
                ring,
                turn,
            },
            Self {
                p: center + advanced * ring,
                t: rotate * self.t,
            },
        )
    }
}

// ---------------------------------------------------------------
// Builders
// ---------------------------------------------------------------

/// The revolve axis every lily piece uses: the sketch frame's own
/// origin, along +v. Each builder chooses the FRAME so that this one
/// axis lands where the piece needs it — the kernel's revolve takes
/// its axis in sketch coordinates, so placement is a frame choice,
/// not an argument (findings entry 6).
fn sketch_axis<S: Scalar>() -> RevolveAxis<S> {
    RevolveAxis {
        origin: p2(0.0, 0.0),
        dir: v2(0.0, 1.0),
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
        axis_frame(
            spec.center.map(S::from_f64),
            v3(0.0, sense, 0.0),
            spec.radial.map(S::from_f64),
            tol,
        ),
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
/// Faces: throat plane, neck cone, sphere zone, the torus band its
/// MOUTH is rolled into ([`roll_the_mouth`]), pucker cone, mouth
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
) -> ConstructedLoop<S> {
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
    attach: Point3<f64>,
    dir: Vec3<f64>,
    globe: f64,
    top: f64,
    mouth: f64,
    lip_r: f64,
    lip_drop: f64,
    neck: (f64, f64),
    mouth_roll: f64,
    tol: Tol,
) -> Body<S> {
    // Sketch frame: origin at the attachment point, v along the
    // flower axis (into the flower), u the in-plane radial — the
    // flower axis turned a quarter turn in the plant's own plane,
    // i.e. crossed with ŷ.
    let plane = sketch_frame::<S>(
        attach.map(S::from_computed),
        dir.cross(Vec3::unit_y()).map(S::from_computed),
        dir.map(S::from_computed),
        tol,
    );
    let sharp = revolve(
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
    .body;
    // The globe centre's depth along the flower axis: the neck, then
    // the globe's own truncation height.
    let centre = neck_drop(globe, top, neck.0, neck.1) + top;
    roll_the_mouth(sharp, attach, dir, centre, mouth_roll, tol)
}

/// The lantern with its **mouth rolled**: `fillet_edges` along the
/// circle where the sphere zone meets the conical pucker, at radius
/// `roll`. This is wall 6's retirement — the softened tepal-tip rim
/// the wall asked for, built in the scene.
///
/// The edges are selected BY DESCRIPTION, in two steps, and neither
/// reads a coordinate against a tolerance: the sphere × cone edges
/// (`query::edge_adjacent_matches`) are the lantern's two latitude
/// circles, the shoulder above the globe centre and the mouth below
/// it, and the mouth's are the ones deeper along the flower axis than
/// the globe centre (`centre`, measured from the attachment point),
/// which they are by `mouth` and the shoulder's are short of by `top`.
/// A full revolve leaves each circle as TWO edges, its halves either
/// side of the seam, and the roll takes both: one half alone ends its
/// chain at the seam vertex, which the battery refuses
/// (`UnsupportedCorner { SeamVertex }`), as a chain that stops partway
/// round a circle should.
fn roll_the_mouth<S: Scalar>(
    sharp: Body<S>,
    attach: Point3<f64>,
    dir: Vec3<f64>,
    centre: f64,
    roll: f64,
    tol: Tol,
) -> Body<S> {
    let at = attach.map(S::from_computed);
    let along = dir.map(S::from_computed);
    let depth = |e: EdgeKey| {
        let c = sharp
            .get_curve_geom(sharp.get_edge(e).expect("an edge of the lantern").curve)
            .and_then(|g| g.certified())
            .expect("a revolved rim carries a certified carrier");
        let (t0, t1) = c.params();
        (c.carrier().eval((t0 + t1) * S::from_f64(0.5)) - at)
            .dot(along)
            .f()
    };
    let mouth: Vec<EdgeKey> = query::all_edges(&sharp)
        .into_iter()
        .filter(|&e| {
            query::edge_adjacent_matches(
                &sharp,
                e,
                SurfaceKindSet::just(SurfaceKind::Sphere),
                SurfaceKindSet::just(SurfaceKind::Cone),
            )
        })
        .filter(|&e| depth(e) > centre)
        .collect();
    assert_eq!(
        mouth.len(),
        2,
        "the lantern's mouth is one circle, two half-arcs either side of the seam"
    );
    fillet_edges(
        &finished("the sharp lantern", sharp.clone(), tol),
        &mouth,
        S::from_f64(roll),
        tol,
    )
    .unwrap_or_else(|e| panic!("the lantern's mouth rolls at r = {roll}: {:?}", e.error))
    .body
}

/// The **corm**: the swollen underground stem-base a *Calochortus*
/// rises from each spring.
///
/// A corm is not a thing the stem stands on; it is the stem's OWN base,
/// swollen, with the axis running through it. So it is authored as
/// exactly that: a sphere-zone swelling with a coaxial BORE at the
/// stem's diameter, threaded on the stem's foot. Its two planar caps
/// are ANNULI because the corm is threaded on the stem: the bore is the
/// stem's own diameter. (A full revolve sweeps a planar cap whole
/// whether or not it touches the axis, so the annulus is not there to
/// sidestep a split; it is the shape.)
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
    let lp: ConstructedLoop<S> = Open
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
    // Drawn down from the corm's top: the xy plane turned a quarter
    // turn BACK about x̂, so sketch `v` runs down −ẑ.
    let plane = SketchPlane::from_frame(RigidMotion::turned(
        p3(0.0, 0.0, top_z),
        UnitVec3::exact_x(),
        Turns::quarters(-1),
    ));
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
    let rim = pncad::profile::circle_split(p2(0.0, 0.0), S::from_f64(r), 3, S::from_f64(0.0), tol)
        .expect("the foot's three-arc rim authors");
    let plane = SketchPlane::xy_at(p3(0.0, 0.0, z0));
    let profile = validated(plane, vec![rim.into()], tol).expect("foot profile validates");
    extrude(
        &profile,
        Extrusion::Distance {
            depth: S::from_f64(z1 - z0),
            side: ExtrudeSide::Along,
        },
        tol,
    )
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

/// A body's exact volume, read out at `f64`.
fn volume<S: Scalar>(b: &Body<S>, tol: Tol) -> f64 {
    pncad::topo::mass_properties(&crate::booleans::at_rest(b, tol), tol, crate::printed())
        .expect("the volume integrates")
        .enclosure
        .volume
        .f()
}

/// The rootstock's two parts as authored: the [`corm`] and the
/// [`foot`] threaded through its bore.
fn rootstock_parts<S: Scalar>(tol: Tol) -> (AtRestBody<S>, AtRestBody<S>) {
    let corm = corm(
        CORM_TOP_Z,
        CORM_GLOBE,
        CORM_SHOULDER,
        CORM_BASE,
        STEM_R,
        tol,
    );
    let foot = foot(FOOT_BOTTOM_Z, 0.0, STEM_R, tol);
    (
        finished("the corm", corm, tol),
        finished("the foot", foot, tol),
    )
}

/// The **rootstock**: the corm and the foot made ONE body, through the
/// contact they were authored to share — the corm's bore wall against
/// the foot's, which the union glues on its decided zero (D10). It
/// is the one place on this plant where two authored bodies are made
/// one.
///
/// The oracle is additivity: the bore is the foot's own cylinder, so
/// the parts' interiors are disjoint and the union's volume is their
/// sum, asked of the kernel's three answers.
fn rootstock<S: Scalar>(tol: Tol) -> BooleanBody<S> {
    let (corm, foot) = rootstock_parts::<S>(tol);
    let parts = volume(&corm, tol) + volume(&foot, tol);
    expect_seamed(
        "the rootstock (corm unioned with the foot at their socket)",
        check(try_union(&corm, &foot, tol), parts, tol),
        parts,
    )
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
/// curved-boolean wall the stem, flower and leaves are stopped by
/// (probes 2 and 7).
///
/// **Three, in the return type.** `plant` names the segments with a
/// `.zip(["lily_bud_a", …])`, which truncates silently against a
/// shorter `Vec` — a fourth segment would vanish from the manifest and
/// only show up as a smaller printed piece count. The arity is
/// therefore stated once, here, where a change to it is a compile
/// error at the call site.
#[allow(clippy::too_many_arguments)]
fn bud<S: Scalar>(
    attach: Point3<f64>,
    dir: Vec3<f64>,
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
    // The radial frame across the bud's axis: the in-plane radial
    // (the axis turned a quarter turn in the plant's own plane, i.e.
    // crossed with ŷ) and ŷ itself. The pairing is a SCENE
    // convention — which way round the axis `phi` is measured from —
    // so it is chosen here rather than asked of the kernel.
    let e1 = dir.cross(Vec3::unit_y());
    let e2 = Vec3::unit_y();
    let rad = |a: f64| {
        let (sa, ca) = (a.sin(), a.cos());
        e1 * ca + e2 * sa
    };
    core::array::from_fn(|i| {
        #[allow(clippy::cast_precision_loss)]
        let phi = 2.0 * PI * (i as f64) / 3.0;
        // The segment's own axis: the bud axis leaned `tilt` toward
        // the direction `lean` radians round from its own place.
        let l = rad(phi + lean);
        let (st, ct) = (tilt.sin(), tilt.cos());
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
        // The segment's own axis and the wedge's start radial,
        // DECIDED together: the axis is kept as the frame's `w` and
        // the radial yields its component along it, since that radial
        // is only perpendicular to the BUD's axis, not to this
        // segment's. All three share the ATTACHMENT: the tilt splays
        // their tips, not their bellies.
        let attach = attach.map(S::from_computed);
        let spine = axis_frame::<S>(
            attach,
            (dir * ct + l * st).map(S::from_computed),
            rad(phi - 0.5 * span).map(S::from_computed),
            tol,
        );
        // The meridian plane is the spine frame's own (u, w): the frame
        // turned a quarter turn about its u.
        let plane = SketchPlane::from_frame(
            spine
                .compose(RigidMotion::turned(
                    Point3::origin(),
                    UnitVec3::exact_x(),
                    Turns::quarters(1),
                ))
                .expect("a turn of a finite frame is finite"),
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

/// A swept leaf's cross-section: a LANCEOLATE lens — two circular arcs,
/// one per face, meeting at the two sharp margins at `±width/2` on a
/// chord. The upper arc rises `ridge` above the chord at the midrib
/// and the lower one drops `keel` below it. The two rises are
/// DIFFERENT, so the blade is asymmetric about its own chord.
///
/// Each arc is the one circle through its two margins and its apex,
/// turning through `4·atan(2·rise/width)` — under a quarter turn for
/// both leaves, so each lateral wall skins one rational Bézier span
/// across and carries no interior crease. The margins are the
/// section's only corners.
///
/// The lofted blades take the same lens, one per station, its three
/// numbers eased along the spine ([`Plan`]): every section is two arcs
/// on the same two margins, so a loft matches them segment to segment.
#[derive(Clone, Copy, Debug)]
struct Lance {
    /// Chord length, margin to margin — the blade's width.
    width: f64,
    /// Rise of the upper arc above the chord.
    ridge: f64,
    /// Drop of the lower arc below the chord.
    keel: f64,
}

impl Lance {
    /// The lens, wound counterclockwise in the sketch `(s, t)` frame
    /// from the `+s` margin: over the ridge to the `−s` margin, then
    /// under the keel back.
    fn outline(self, tol: Tol) -> Vec<ConstructedLoop<f64>> {
        let half = 0.5 * self.width;
        let closed = Open
            .at(Point2::new(half, 0.0))
            .arc_to(
                Via {
                    q: Point2::new(0.0, self.ridge),
                    p: Point2::new(-half, 0.0),
                },
                tol,
            )
            .expect("the blade's ridge arc")
            .arc_to(
                Via {
                    q: Point2::new(0.0, -self.keel),
                    p: Start,
                },
                tol,
            )
            .expect("the blade's keel arc");
        vec![closed.into()]
    }

    /// Linear blend, field by field.
    fn lerp(self, other: Self, s: f64) -> Self {
        let f = |a: f64, b: f64| a + (b - a) * s;
        Self {
            width: f(self.width, other.width),
            ridge: f(self.ridge, other.ridge),
            keel: f(self.keel, other.keel),
        }
    }

    /// The section's area: two circular segments on one chord, each
    /// `R²(θ − sin θ)/2` for its own radius `R` and turn `θ`.
    #[cfg(test)]
    fn area(self) -> f64 {
        self.segment_area(self.ridge) + self.segment_area(self.keel)
    }

    /// The section centroid's signed height above the chord (toward
    /// the ridge). A circular segment's centroid sits
    /// `4R·sin³(θ/2) / (3(θ − sin θ))` from its circle's centre, and
    /// that centre is `R·cos(θ/2)` on the far side of the chord.
    #[cfg(test)]
    fn centroid_rise(self) -> f64 {
        let moment = |h: f64| {
            let (r, th) = self.segment(h);
            let rise =
                4.0 * r * (0.5 * th).sin().powi(3) / (3.0 * (th - th.sin())) - r * (0.5 * th).cos();
            self.segment_area(h) * rise
        };
        (moment(self.ridge) - moment(self.keel)) / self.area()
    }

    /// The circular segment between the chord and the arc rising `h`:
    /// `R²(θ − sin θ)/2`.
    #[cfg(test)]
    fn segment_area(self, h: f64) -> f64 {
        let (r, th) = self.segment(h);
        0.5 * r * r * (th - th.sin())
    }

    /// The radius and turn of the arc through both margins rising `h`
    /// at the midrib.
    #[cfg(test)]
    fn segment(self, h: f64) -> (f64, f64) {
        let c = self.width;
        (
            (0.25 * c * c + h * h) / (2.0 * h),
            4.0 * (2.0 * h / c).atan(),
        )
    }
}

/// A swept leaf's numbers — [`leaf`]'s arguments bar `up`, which is
/// world `z` for both — named once because the scene and the Pappus
/// rows build the same blades.
#[derive(Clone, Copy, Debug)]
struct SweptLeaf {
    base: Point3<f64>,
    dir: Vec3<f64>,
    len: f64,
    section: Lance,
    /// Negative: the blade arches over, as a basal leaf does.
    curl: f64,
}

impl SweptLeaf {
    fn build<S: Scalar>(self, tol: Tol) -> Body<S> {
        leaf(
            self.base,
            self.dir,
            Vec3::unit_z(),
            self.len,
            self.section,
            self.curl,
            tol,
        )
    }
}

/// `lily_leaf_b`, the longer swept leaf.
const LEAF_B: SweptLeaf = SweptLeaf {
    base: Point3::new(-0.03, -0.06, 0.06),
    dir: Vec3::new(-0.68, -0.55, 0.44),
    len: 1.25,
    section: Lance {
        width: 0.170,
        ridge: 0.015,
        keel: 0.007,
    },
    curl: -0.40,
};

/// `lily_leaf_c`, the shorter swept leaf.
const LEAF_C: SweptLeaf = SweptLeaf {
    base: Point3::new(0.02, 0.01, 0.02),
    dir: Vec3::new(0.62, 0.10, 0.78),
    len: 0.95,
    section: Lance {
        width: 0.140,
        ridge: 0.013,
        keel: 0.006,
    },
    curl: -0.35,
};

/// The long basal leaf's placement and spine, named rather than
/// inlined because
/// `review_probes::the_lofted_blade_tapers_and_rolls_in_the_stored_geometry`
/// builds a SECOND blade from the very same numbers with the twist set
/// to zero, and measures the roll as the angle between the two. A
/// re-typed copy of these numbers would let the two drift and the
/// measurement would quietly stop meaning anything.
const LEAF_A_BASE: Point3<f64> = Point3::new(0.04, 0.05, 0.03);
/// See [`LEAF_A_BASE`].
const LEAF_A_DIR: Vec3<f64> = Vec3::new(-0.72, 0.52, 0.16);
/// See [`LEAF_A_BASE`].
const LEAF_A_UP: Vec3<f64> = Vec3::new(0.0, 0.0, 1.0);
/// Wall 7's carving ball, the one that cuts a tepal seam into the
/// lantern's zone sphere off its chart's polar axis: its centre, and
/// [`WALL7_BALL_RADIUS`]. The wall carves with it at the walls' scalar
/// and again in `f64` to draw, and a review row measures it, so it is
/// spelled once.
const WALL7_BALL_CENTER: Point3<f64> = Point3::new(-2.80, 0.0, 0.90);
/// See [`WALL7_BALL_CENTER`].
const WALL7_BALL_RADIUS: f64 = 0.16;
/// See [`LEAF_A_BASE`].
const LEAF_A_LEN: f64 = 5.10;
/// See [`LEAF_A_BASE`]. Negative: the blade arches OVER, which is what
/// a basal leaf lying along the ground does.
const LEAF_A_CURL: f64 = -0.62;
/// The long basal leaf's sheath ([`leaf_a`]): its length back out of
/// the base, and its curl, positive because it turns the other way
/// from the blade.
const LEAF_A_SHEATH_LEN: f64 = 0.34;
/// See [`LEAF_A_SHEATH_LEN`].
const LEAF_A_SHEATH_CURL: f64 = 0.85;

/// The long basal leaf's section plan: a narrow lens where it meets
/// the stem, a wide flat one at the belly a fifth of the way out, and
/// a small one — never a point — at the tip. The roll is 160
/// degrees eased hard toward the tip (exponent 2.6), so the blade lies
/// flat for most of its length and does its turning near the end, the
/// way a real *Calochortus* leaf does.
fn leaf_a_plan() -> Plan {
    Plan {
        base: Lance {
            width: 0.170,
            ridge: 0.028,
            keel: 0.020,
        },
        belly: Lance {
            width: 0.420,
            ridge: 0.034,
            keel: 0.016,
        },
        belly_at: 0.22,
        tip: Lance {
            width: 0.060,
            ridge: 0.010,
            keel: 0.006,
        },
        roll0: 0.0,
        twist: deg(160.0),
        twist_ease: 2.6,
    }
}

/// Stations along a leaf's swept spine (the swept-elbow corpus
/// fixture's count).
const LEAF_STATIONS: usize = 9;
/// Every blade's skin fit degree along the path, swept and lofted alike.
const BLADE_V_DEGREE: usize = 3;
/// Stations along the LOFTED long leaf. More than the swept blades
/// use, because a loft's stations carry the taper and the roll as well
/// as the spine: the skin interpolates the sections it is given and
/// chords between them, so a roll of half a turn wants stations dense
/// enough that no chord cuts a visible corner.
const LOFT_STATIONS: usize = 17;
/// Stations along the long leaf's sheath, its base section included.
const SHEATH_STATIONS: usize = 9;
/// Stations along a sepal — shorter blades, gentler roll.
const SEPAL_STATIONS: usize = 13;

/// A **keeled leaf blade**: a thin section carried along a gently
/// arching spine by [`sweep_body`] — the general-path sweep, not an
/// extrusion, so the blade leaves the plane it was drawn in.
///
/// The section is a [`Lance`], and the spine runs through its chord's
/// midpoint, i.e. through the midrib. Its arcs make every lateral wall
/// RATIONAL, so the walls' volume takes the quadrature lane; at the
/// tightest ε that reading is a certified bracket rather than a
/// number, which the tour narrates as one.
///
/// The spine leaves `base` along `dir` and turns through `curl`
/// radians toward `up` (rejected from `dir`; negative `curl`
/// arches the blade over, which is what a basal leaf does), staying a
/// circular arc of length `len` sampled at [`LEAF_STATIONS`] exact
/// points that a cubic `NurbsCurve3::interpolate` runs through. The
/// profile plane's normal IS the spine's start tangent, so the section
/// rides normal to its own path. The blade holds ONE width from base
/// to tip: there is no tapering sweep (findings entry 9).
fn leaf<S: Scalar>(
    base: Point3<f64>,
    dir: Vec3<f64>,
    up: Vec3<f64>,
    len: f64,
    section: Lance,
    curl: f64,
    tol: Tol,
) -> Body<S> {
    let (d, v, u) = blade_frame(dir, up, tol);
    // The spine: a circular arc of length `len` turning through `curl`
    // in the (d, v) plane, i.e. radius len/curl, sampled exactly.
    let r = len / curl;
    let pts: Vec<Point3<f64>> = (0..LEAF_STATIONS)
        .map(|k| {
            #[allow(clippy::cast_precision_loss)]
            let a = curl * (k as f64) / ((LEAF_STATIONS - 1) as f64);
            let (s, c) = (r * a.sin(), r * (1.0 - a.cos()));
            base + d * s + v * c
        })
        .collect();
    let path = pncad::geom::NurbsCurve3::interpolate(&pts, 3).expect("the leaf spine interpolates");
    // The outline is built in `f64`, so it enters `S` as the computed
    // values it is; the frame is minted at `S` from those values.
    let place = sketch_frame::<S>(
        base.map(S::from_computed),
        u.map(S::from_computed),
        v.map(S::from_computed),
        tol,
    );
    let outline: Vec<_> = section
        .outline(tol)
        .iter()
        .map(|lp| lp.lift_computed())
        .collect();
    sweep_body::<S>(&outline, place, &path, LEAF_STATIONS, BLADE_V_DEGREE, tol)
        .expect("the leaf sweeps along its spine")
        .body
}

// ---------------------------------------------------------------
// The LOFTED blade: the two things a sweep cannot say
// ---------------------------------------------------------------

/// How a lofted blade's section changes from attachment to tip, and
/// how it rolls on the way.
#[derive(Clone, Copy, Debug)]
struct Plan {
    /// The attachment section, where the blade meets the stem.
    base: Lance,
    /// The widest section — the blade's belly.
    belly: Lance,
    /// Where the belly sits, as a fraction of the spine.
    belly_at: f64,
    /// The tip section: a small lens, never a POINT, since a section
    /// of zero width has no chord for its arcs to stand on.
    tip: Lance,
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
    fn at(self, s: f64) -> Lance {
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
/// segment), and each SLAB — each adjacent pair of stations — may not
/// turn past π, because the loft's stacking statement is a fold over
/// those pairs and a slab that turns further stacks against its own
/// base section's normal and refuses `ReversedStacking` naming itself.
/// So the bound is on `curl / (stations − 1)`, not on `curl`: a blade
/// may coil as far as its sampling supports.
///
/// `review_probes::the_spine_curl_wall_re_measured` pins both sides of
/// that wall (13.0 rad coils at 17 stations; 10.0 rad refuses at 4).
/// Past a full turn the spine returns through its own body and the
/// kernel has no gate that says so — see the sweep crate's
/// `bool6_per_slab_stacking::a_curl_past_a_full_turn_builds_a_spine_that_revisits_itself`.
#[allow(clippy::too_many_arguments)] // the 8th is the run-tolerance witness
fn lofted_blade<S: Scalar>(
    base: Point3<f64>,
    dir: Vec3<f64>,
    up: Vec3<f64>,
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
/// the curl-wall probe (M8-14, #222 — `review_probes::
/// the_spine_curl_wall_re_measured`) can sweep the parameter and
/// state the measured frontier rather than a remembered one.
#[allow(clippy::too_many_arguments)] // the 8th is the run-tolerance witness
fn try_lofted_blade<S: Scalar>(
    base: Point3<f64>,
    dir: Vec3<f64>,
    up: Vec3<f64>,
    len: f64,
    curl: f64,
    plan: Plan,
    stations: usize,
    tol: Tol,
) -> Result<pncad::sweep::Lofted<S>, pncad::sweep::LoftError> {
    let (sections, places): (Vec<_>, Vec<_>) = blade_stations(
        base,
        blade_frame(dir, up, tol),
        len,
        curl,
        plan,
        stations,
        tol,
    )
    .unzip();
    loft_body::<S>(&sections, &places, BLADE_V_DEGREE, tol)
}

/// A lofted blade's stations, base to tip: section `k` is `plan.at(s_k)`
/// at fraction `s_k` of the arc, rolled `roll0 + twist * s_k^twist_ease`.
#[allow(clippy::too_many_arguments)] // the 7th is the run-tolerance witness
fn blade_stations<S: Scalar>(
    base: Point3<f64>,
    frame: BladeFrame,
    len: f64,
    curl: f64,
    plan: Plan,
    stations: usize,
    tol: Tol,
) -> impl Iterator<Item = (Vec<ConstructedLoop<S>>, SketchPlane<S>)> {
    (0..stations).map(move |k| {
        #[allow(clippy::cast_precision_loss)]
        let s = (k as f64) / ((stations - 1) as f64);
        let roll = plan.roll0 + plan.twist * s.powf(plan.twist_ease);
        blade_station(base, frame, len / curl, curl * s, roll, plan.at(s), tol)
    })
}

/// One station of a lofted blade whose spine is a circular arc of
/// signed radius `r` leaving `base` along the frame's `d` and turning
/// toward its `v`: the point `a` radians round that arc, the section
/// placed normal to the arc there and rolled `roll` about its tangent.
/// The arc turns in the `(d, v)` plane about `u`, so `u` itself is
/// fixed and the roll is the only other motion. A negative `a` runs
/// back out of `base` along `-d`.
fn blade_station<S: Scalar>(
    base: Point3<f64>,
    (d, v, u): BladeFrame,
    r: f64,
    a: f64,
    roll: f64,
    section: Lance,
    tol: Tol,
) -> (Vec<ConstructedLoop<S>>, SketchPlane<S>) {
    let (sa, ca) = (a.sin(), a.cos());
    let p = base + d * (r * sa) + v * (r * (1.0 - ca));
    let vk = v * ca - d * sa;
    let (st, ct) = (roll.sin(), roll.cos());
    let uu = u * ct + vk * st;
    let vv = vk * ct - u * st;
    // Built in `f64`, so they enter `S` as the computed values they are.
    (
        section
            .outline(tol)
            .iter()
            .map(|lp| lp.lift_computed())
            .collect(),
        sketch_frame::<S>(
            p.map(S::from_computed),
            uu.map(S::from_computed),
            vv.map(S::from_computed),
            tol,
        ),
    )
}

/// The long basal leaf, sheath and blade in ONE loft. The sheath is
/// the base section run back out of [`LEAF_A_BASE`] along `-LEAF_A_DIR`
/// for [`LEAF_A_SHEATH_LEN`], curling through [`LEAF_A_SHEATH_CURL`]
/// toward `up` — the other way from the blade, which arches over — so
/// the spine is two circular arcs meeting G1 at the base, and the base
/// section is an interior station. Both arcs turn in the same `(d, v)`
/// plane, so one frame and one angle along it place every station.
fn leaf_a<S: Scalar>(plan: Plan, tol: Tol) -> Body<S> {
    let frame = blade_frame(LEAF_A_DIR, LEAF_A_UP, tol);
    let sheath = (0..SHEATH_STATIONS - 1).map(|k| {
        #[allow(clippy::cast_precision_loss)]
        let s = (k as f64) / ((SHEATH_STATIONS - 1) as f64);
        blade_station(
            LEAF_A_BASE,
            frame,
            LEAF_A_SHEATH_LEN / LEAF_A_SHEATH_CURL,
            -LEAF_A_SHEATH_CURL * (1.0 - s),
            plan.roll0,
            plan.base,
            tol,
        )
    });
    let blade = blade_stations(
        LEAF_A_BASE,
        frame,
        LEAF_A_LEN,
        LEAF_A_CURL,
        plan,
        LOFT_STATIONS,
        tol,
    );
    let (sections, places): (Vec<_>, Vec<_>) = sheath.chain(blade).unzip();
    loft_body::<S>(&sections, &places, BLADE_V_DEGREE, tol)
        .expect("the long leaf skins its sheath and blade as one loft")
        .body
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
/// nothing above ground is joined, and the flower's own throat disk and the
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
    globe_center: Point3<f64>,
    axis: Vec3<f64>,
    globe: f64,
    theta: f64,
    phase: f64,
    len: f64,
    curl: f64,
    plan: Plan,
    tol: Tol,
) -> [Body<S>; 3] {
    // The two radials spanning the plane perpendicular to the flower
    // axis: the in-xz-plane one (the axis crossed with ŷ) and ŷ.
    let e1 = axis.cross(Vec3::unit_y());
    let e2 = Vec3::unit_y();
    let (st, ct) = (theta.sin(), theta.cos());
    // The offset that makes the keel graze rather than pierce.
    let stand = globe + plan.base.keel;
    core::array::from_fn(|i| {
        #[allow(clippy::cast_precision_loss)]
        let phi = phase + 2.0 * PI * (i as f64) / 3.0;
        let (sp, cp) = (phi.sin(), phi.cos());
        let rad = e1 * cp + e2 * sp;
        // n: the outward normal at (theta, phi). `-axis` is the
        // flower's upper pole, the axis pointing INTO the flower.
        let n = -axis * ct + rad * st;
        // tau: the tangent there, running outward and down.
        let tau = axis * st + rad * ct;
        let base = globe_center + n * stand;
        lofted_blade::<S>(base, tau, n, len, curl, plan, SEPAL_STATIONS, tol)
    })
}

/// A blade's local frame as three world vectors: the spine's start
/// tangent, the direction it curls toward, and the section's width
/// axis. See [`blade_frame`].
type BladeFrame = (Vec3<f64>, Vec3<f64>, Vec3<f64>);

/// The right-handed `(d, v, u)` blade frame: `d` the spine's start
/// tangent, `v` the direction the blade curls toward, and `u = v x d`,
/// so a sketch plane built on `(u, v)` has `d` for its normal. Shared
/// by [`leaf`] and [`lofted_blade`] so the swept and lofted blades sit
/// in the SAME frame — the difference between them is the verb, not
/// the placement.
///
/// `d` and `v` are the MINT's, not this file's: [`axis_frame`] keeps
/// the tangent and yields `up` to it at the run's band, which is the
/// same decision every other frame in the tour goes through. `u` is
/// then an exact cross of two witnesses — no length left to decide.
fn blade_frame(dir: Vec3<f64>, up: Vec3<f64>, tol: Tol) -> BladeFrame {
    let f = axis_frame::<f64>(Point3::origin(), dir, up, tol);
    let (d, v) = (f.w().get(), f.u().get());
    (d, v, v.cross(d))
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
    /// For a piece that is a boolean RESULT (the rootstock), the
    /// contact records its op returned, which its tier-3′ gate consumes.
    pub contacts: Option<ContactRecords>,
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

/// The rolling ball's radius along the lantern's MOUTH, the circle
/// where the sphere zone meets the conical pucker (wall 6's request).
/// The crease there is shallow and CONCAVE: the pucker turns 9.37°
/// off the globe's tangent, outward, so a ball rolled along it adds
/// material, and its band spans only about `0.164·r` of meridian. At
/// r = 0.25 that is 3.3 cm, the tangency circles at carrier radii
/// 0.2647 (on the globe) and 0.2400 (on the pucker) either side of the
/// 0.2534 mouth, wide enough to read at the render's scale. Measured
/// on the scene's own lantern, the roll builds tier-3 valid from
/// r = 0.005 to r = 0.6 and refuses `SpineIrregular` at r = 1.0, where
/// the ball's spine would turn inside out. See [`roll_the_mouth`].
const FLOWER_MOUTH_ROLL: f64 = 0.25;

const GREEN_STEM: [f64; 3] = [0.36, 0.52, 0.30];
const GREEN_LEAF: [f64; 3] = [0.44, 0.62, 0.34];
/// *C. pulchellus* is the YELLOW fairy lantern — clear lemon, not the
/// white of *albus* this scene used to wear.
const YELLOW_TEPAL: [f64; 3] = [0.95, 0.84, 0.32];
/// The sepals are greener than the petals and stay so: on a live
/// pulchellus they read as the yellow-green sheath the globe hangs in.
const GREEN_SEPAL: [f64; 3] = [0.72, 0.76, 0.36];
/// The rootstock is underground and reads as such: a dull, papery
/// brown, foot and corm alike, because they are one body.
const BROWN_ROOTSTOCK: [f64; 3] = [0.55, 0.44, 0.30];

/// Builds the whole plant: the rootstock, two stem arcs, one branching
/// pedicel, the lantern, the bud's three pre-tepals, three basal
/// leaves and three sepals — fourteen bodies, every one closed.
///
/// The stem is walked by a [`Turtle`] so the arcs are G1 at their
/// joints by construction: each arc's start tangent IS the previous
/// arc's end tangent, and the lantern's axis IS the last tangent, so
/// the flower hangs along the stem's own direction rather than along
/// a hand-chosen vector.
pub fn plant<S: Scalar>(tol: Tol) -> Vec<Piece<S>> {
    let root = Turtle {
        p: Point3::origin(),
        t: Vec3::unit_z(),
    };
    // The long, nearly straight rise, then the tight turn-over that
    // makes the arch. 22 degrees on a 5 m ring reads as "leaning";
    // 170 degrees on a 1.1 m ring turns the stem right over.
    let (lower, at_fork) = root.arc(STEM_RING, deg(22.0));
    let (upper, at_flower) = at_fork.arc(1.1, deg(170.0));
    // The side pedicel leaves the fork heading up and out, then
    // curls 130 degrees over so its lantern hangs nearly plumb.
    let fork = Turtle {
        p: at_fork.p,
        t: Vec3::new(deg(150.0).cos(), 0.0, deg(150.0).sin()),
    };
    let (pedicel, at_bud) = fork.arc(0.42, deg(130.0));

    let (stem, stem_caps) = tube_arc(lower, STEM_TUBE, tol);
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
        base: Lance {
            width: 0.105,
            ridge: 0.014,
            keel: 0.008,
        },
        belly: Lance {
            width: 0.265,
            ridge: 0.017,
            keel: 0.008,
        },
        belly_at: 0.30,
        tip: Lance {
            width: 0.036,
            ridge: 0.006,
            keel: 0.004,
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
    let bud_attach = at_bud.p - at_bud.t * 0.06;
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
        flower_attach + at_flower.t * flower_globe_depth,
        at_flower.t,
        FLOWER_GLOBE,
        (FLOWER_TOP / FLOWER_GLOBE).acos() + deg(4.0),
        deg(180.0),
        1.05,
        0.40,
        sepal_plan,
        tol,
    );
    let rootstock = rootstock::<S>(tol);

    let mut pieces = vec![
        Piece {
            name: "lily_rootstock",
            color: BROWN_ROOTSTOCK,
            body: rootstock.body.into_body(),
            caps: None,
            contacts: Some(rootstock.contacts),
        },
        Piece {
            name: "lily_stem",
            color: GREEN_STEM,
            body: stem,
            caps: Some(stem_caps),
            contacts: None,
        },
        Piece {
            name: "lily_arch",
            color: GREEN_STEM,
            body: arch,
            caps: Some(arch_caps),
            contacts: None,
        },
        Piece {
            name: "lily_pedicel",
            color: GREEN_STEM,
            body: pedicel_body,
            caps: Some(pedicel_caps),
            contacts: None,
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
                FLOWER_MOUTH_ROLL,
                tol,
            ),
            caps: None,
            contacts: None,
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
            body: leaf_a(leaf_a_plan(), tol),
            caps: None,
            contacts: None,
        },
        Piece {
            name: "lily_leaf_b",
            color: GREEN_LEAF,
            body: LEAF_B.build(tol),
            caps: None,
            contacts: None,
        },
        Piece {
            name: "lily_leaf_c",
            color: GREEN_LEAF,
            body: LEAF_C.build(tol),
            caps: None,
            contacts: None,
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
            contacts: None,
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
            contacts: None,
        });
    }
    pieces
}

/// The lower stem's tube: a 5 m ring at the authored 0.06 m pipe, so its
/// ring is about 83 times its pipe.
const STEM_RING: f64 = 5.0;
const STEM_TUBE: f64 = 0.060;

/// **The lily's declared frontier** (the tour's #795 posture: a scene
/// may declare a frontier up front, and must fail loudly when it stops
/// refusing). At ε = 1e-12 the lower stem's tube — a torus whose ring is
/// about 83 times its pipe — refuses as limb 2's kernel approximation
/// limit: its rim's certified distance bound from the torus lies far
/// above the rim's own rounding, the thin-torus class fork row 203 rules
/// a kernel limit at that row (R/r of 20 or more;
/// `work/offset/the-torus-composite-bounds-a-thin-torus-far-above-its-carriers-rounding.md`).
/// The stem is the plant's spine, so the lily's stop and its wall probes
/// stand down at that row and say so. `Some(narration)` where the
/// frontier holds; `None` where the plant builds.
///
/// # Panics
///
/// At ε = 1e-12, when the stem builds (the frontier retired: drop this
/// declaration) or refuses other than as the kernel limit; at any other
/// row, when it refuses at all.
pub fn stem_frontier(tol: Tol) -> Option<String> {
    let at_row = (tol.eps() / 1.0e-12 - 1.0).abs() < 1.0e-3;
    let root = Turtle {
        p: Point3::origin(),
        t: Vec3::unit_z(),
    };
    let (lower, _) = root.arc(STEM_RING, deg(22.0));
    let sense = if lower.turn >= 0.0 { -1.0 } else { 1.0 };
    let got = tube_along_arc(
        axis_frame(lower.center, v3(0.0, sense, 0.0), lower.radial, tol),
        lower.ring,
        TubeWindow::Arc {
            t0: 0.0,
            t1: lower.turn.abs(),
        },
        STEM_TUBE,
        tol,
    );
    match (at_row, got) {
        (false, Ok(_)) => None,
        (true, Err(e)) if format!("{e:?}").contains("AnalyticRung3(KernelLimit {") => {
            Some(format!(
                "DECLARED FRONTIER at eps = {:e}: the lower stem (a {STEM_RING} m ring at a \
             {STEM_TUBE} m pipe, ring/pipe ~ 83) refuses as limb 2's kernel approximation \
             limit — a thin torus, the class ruled a kernel limit at this row — so the \
             plant, its stop and its wall probes stand down here: {e}",
                tol.eps()
            ))
        }
        (true, Ok(_)) => panic!(
            "the lily's declared stem frontier RETIRED: the thin stem tube builds at eps = \
             1e-12; drop `stem_frontier` and run the plant at every row"
        ),
        (_, Err(e)) => panic!("the lily's stem tube refused outside its declared frontier: {e:?}"),
    }
}

/// The tour stop.
pub fn stops(tol: Tol) -> Vec<TourStop> {
    let pieces = plant::<f64>(tol);
    let note = format!(
        "{} closed solids: 1 ROOTSTOCK — a sphere-zone corm whose \
         bore is the stem's own cylinder, UNIONED with the foot threaded \
         through it on their one cylindrical Rest contact (the \
         bore wall against the foot's three arcs; no planar pair on the \
         mate), one shell whose volume is the two parts' sum — 3 \
         torus-segment stem tubes said in WORLD \
         coordinates (centre/axis/u_ref/radii stored exactly as \
         given), 1 sphere-zone lantern with a NECK cone cut at the \
         arch tube's own radius — its rim IS that tube's terminal \
         meridian circle, so flower and stem meet on one shared \
         circle — and a conical mouth, 3 \
         PARTIAL revolves of that same meridian forming the bud's \
         tripod of pre-tepals, 2 SWEPT \
         keeled blades (one lanceolate section — two arcs meeting \
         at the margins — carried along an arching NURBS spine), and \
         4 LOFTED ones — the long basal leaf and the three sepals — which taper AND roll, the two things a \
         sweep cannot be asked for, on the same lanceolate section. \
         The long leaf is one loft from its sheath, curling up out of \
         the stem, through the base section to wide to small, turning 160 \
         degrees about its own spine on the way, eased toward the tip. \
         The sepals stand TANGENT to the globe: the stand-off is the \
         section's own keel, and no point of any sepal's section rims \
         is inside the sphere. The eight analytic bodies approximate nothing — torus, \
         sphere, cylinder, cone and plane exactly, parameters included; \
         the blades are fitted skins, the price of leaving the plane. \
         Nothing above ground is JOINED — the long leaf to the stem it \
         grows from least of all: see the wall probes.",
        pieces.len()
    );
    vec![TourStop {
        name: "lily",
        caption: "fairy lantern (Calochortus pulchellus)".to_string(),
        montage: true,
        story: "a nodding yellow fairy lantern — a brown rootstock (the \
                corm unioned with the stem's foot through its socket), \
                arching stem, a closed globular lantern with three spreading sepals tangent to the \
                globe, a bud of three nested pre-tepals on a tripod of axes, \
                one long tapering twisted basal leaf and two shorter \
                untapered ones; torus/sphere/cylinder/cone/plane exact to the stored \
                parameter, blades skinned out of plane",
        ops: "revolve(Full) bored corm + extrude three-arc foot -> union (the \
              rootstock); Turtle-walked G1 arc chain -> tube_along_arc(world centre/axis/ \
              u_ref/radii, windowed) tubes; revolve(Full) sphere-zone \
              lantern and revolve(Partial) x3 on a tilted tripod for the \
              bud; sweep_body(lanceolate arc section, arched NURBS \
              spine) for the two short leaves; loft_body(tapering lanceolate \
              sections on rolled placements) for the long leaf and the sepals",
        delta: 5e-3,
        note: Some(note),
        view: View {
            elev: 12.0,
            azim: -78.0,
            up: 'z',
        },
        bodies: pieces
            .into_iter()
            .map(|p| {
                let sb = match p.contacts {
                    Some(contacts) => SceneBody::seamed(p.name, p.color, p.body, contacts),
                    None => SceneBody::plain(p.name, p.color, p.body),
                };
                // The split is by SECTION. The lofted blades (`leaf_a`
                // and the sepals) are lofted through straight-sided
                // sections, and at 5e-3 their meshes are within 0.1% of
                // exact volume. Every other body has a curved section
                // (tubes, revolves, the lens-section swept leaves) and
                // takes 2e-3. The names below are the lofted blades; a
                // piece added, renamed or rebuilt with another section
                // is placed by this rule, not by its name.
                if p.name == "lily_leaf_a" || p.name.starts_with("lily_sepal") {
                    sb
                } else {
                    sb.finer(2e-3)
                }
            })
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
    c: Point3<f64>,
    r: f64,
    n: Vec3<f64>,
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
        // Read back at `f64` once, where the narration starts.
        let a = apex.map(S::f);
        let d = axis.map(S::f);
        let off = rho / half_angle.f().tan();
        for s in [1.0, -1.0] {
            out.push((
                k,
                Circle {
                    c: a + d * (s * off),
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
    centre: Point3<f64>,
    axis: Vec3<f64>,
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
                centre: center.map(S::f),
                axis: axis.map(S::f),
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
    let w = circle.c - torus.centre;
    let h = w.dot(ta);
    let radial = w.reject_from(ta);
    let rad_len = radial.norm();
    // Distance from the spine circle: the meridian plane's own polar
    // coordinates, (in-plane radius − R, out-of-plane height).
    let off_spine = ((rad_len - torus.big_r).powi(2) + h * h).sqrt();
    let tangential = ta.cross(radial).normalize();
    (
        off_spine,
        (circle.r - torus.small_r).abs(),
        circle.n.cross(tangential).norm(),
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
        let o = pose.origin.map(S::f);
        let n = pose.axis.map(S::f);
        (best.c - o).norm() < 1e-12 && best.n.cross(n).norm() < 1e-12
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
        .filter(|(_, c)| (c.c - best.c).norm() > 1e-9)
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
fn ball<S: Scalar>(c: Point3<f64>, r: f64, tol: Tol) -> Body<S> {
    // The xy plane turned a quarter turn about x̂: sketch `v` up ẑ.
    let plane = SketchPlane::from_frame(RigidMotion::turned(
        c.map(S::from_f64),
        UnitVec3::exact_x(),
        Turns::quarters(1),
    ));
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
/// - success → panic with instructions, the [`crate::walls::wall`]
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

/// The lantern carved with its three tepal seams: wall 7's ball, and
/// the same ball turned a third and two thirds of a turn about the
/// lantern's axis through its zone sphere's centre, subtracted in turn.
/// Each seam's section is a circle tilted against the zone's chart.
fn seamed_lantern(lantern: Body<f64>, tol: Tol) -> Result<Body<f64>, BooleanError> {
    let (zc, axis) = lantern
        .faces()
        .find_map(|(_, f)| match lantern.get_surface(f.surface) {
            Some(&pncad::topo::Surface::Sphere { center, axis, .. }) => Some((center, axis)),
            _ => None,
        })
        .expect("the lantern's zone sphere");
    let mut body = finished("lily_lantern", lantern, tol);
    for k in 0..3 {
        let turn = Affine3::rotation_about_axis(zc, axis, f64::from(k) * 2.0 * PI / 3.0);
        let ball = ball::<f64>(
            turn.transform_point(WALL7_BALL_CENTER),
            WALL7_BALL_RADIUS,
            tol,
        );
        body = pncad::topo::subtract(&body, &finished("a tepal seam's ball", ball, tol), tol)?
            .body()
            .expect("the carve leaves the lantern")
            .body
            .clone();
    }
    Ok(body.into_body())
}

/// Where wall 7 stops: the carve itself, or drawing what it built.
#[derive(Debug)]
#[allow(dead_code)] // the payloads are read through `Debug` in the wall's report
enum Wall7 {
    Carve(BooleanError),
    Draw(pncad::mesh::TessellateError),
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
    // The pieces the walls below hand to the boolean doors, finished
    // once.
    let operand = |name: &str| finished(name, by(name).clone(), tol);
    let (stem, arch, lant) = (
        &operand("lily_stem"),
        &operand("lily_arch"),
        &operand("lily_lantern"),
    );
    let leaf_a = &operand("lily_leaf_a");
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
        weld.c.x,
        weld.c.y,
        weld.c.z,
        weld.r,
        weld.n.x,
        weld.n.y,
        weld.n.z,
        res.0,
        res.1,
        res.2,
        runner_up
    );

    // Wall 1 — RETIRED, kept live. The stem's two arcs meet on a shared
    //    disc, an exact coincident planar contact the union glues on its
    //    decided zero, and the glue builds: the section certificate
    //    reads the two tube walls (tori with parallel axes apart) as two
    //    loops, mirror images across the weld plane, each outside one
    //    face's window. The scene still ships the stem as two pieces;
    //    gluing them is the scene's own change to make.
    let glued = crate::booleans::try_union(stem, arch, tol)
        .expect("the stem's two arcs glue into one stem");
    assert_eq!(
        glued.body().map(|b| b.body.shells().count()),
        Some(1),
        "the glued stem is one shell"
    );
    println!("   wall 1 — RETIRED: the stem's two arcs glue into one stem");

    // 2. The flower grows OUT OF the pedicel, and it is now authored
    //    so that it CAN: the neck cone's rim and the arch tube's
    //    terminal meridian circle are one circle, asserted above off
    //    both stored carriers. There is no transverse curve here to
    //    have a closed form for.
    //
    //    What refuses is the crossing layer, on the weld circle
    //    itself: the lantern's neck rim lies on the arch's torus wall
    //    as its terminal meridian circle, and a circle ON a torus is a
    //    coincidence the circle × torus lane does not decide. This is
    //    the chain VERBS-LILYWELD PR-2 measured — gate, then F7, then
    //    the curved PIERCE arm — reached now that the cone is on the
    //    operand roster and the full revolve sweeps its caps whole.
    //    The planar pair the throat disk and the arch's cap form is
    //    decided on its zero; the weld circle is what stops it.
    wall(
        2,
        "weld the lantern onto the arch (cone x the arch's end cap, \
         meeting on one shared circle)",
        pncad::topo::union(lant, arch, tol),
        |e| {
            // The lantern's edge refused is the weld circle, lying on
            // the arch's torus wall
            // (`review_probes::the_weld_refuses_on_the_weld_circle`).
            matches!(
                e,
                BooleanError::CurvedPierceUnsupported {
                    operand: Operand::A,
                    ..
                }
            )
        },
        "join flower to stem — the weld circle lying on the arch's torus \
         wall is a coincidence the crossing layer refuses",
    );

    // 3. The lily's leaves DO leave their own plane now — each blade
    //    is a lanceolate section swept along an arching spine. What is
    //    still refused is the cheap way to ask for it: an EXTRUSION
    //    along anything but the sketch normal is oblique, and oblique
    //    extrusion is deferred past M2. The probe pins that door, not
    //    the out-of-plane blade, which the scene above builds live.
    let leafp = {
        let plane = SketchPlane::from_frame(RigidMotion::axes_xy(p3(0.0, 0.0, 0.0)));
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
    //    a body moves only by a `RigidMotion`, and a matrix becomes one
    //    only through the door that decides its rigidity.
    let band = pncad::geom_core::Band::linear(tol).expect("the run's band");
    let stretch = Affine3::from_parts(
        Mat3::from_cols(v3(1.0, 0.0, 0.0), v3(0.0, 1.0, 0.0), v3(0.0, 0.0, 1.6)),
        v3(0.0, 0.0, 0.0),
    );
    wall(
        4,
        "stretch a lantern into an ovoid bud (non-uniform scale)",
        RigidMotion::check_rigid(&stretch, band)
            .map(|stretch| pncad::topo::transform_rigid(lant, &stretch, tol)),
        // The NAMED predicate matters: a unit-norm failure on the
        // scaled column, not a determinant or orthogonality failure.
        |e| matches!(e, RigidMotionError::NotRigid { check } if *check == "transform_rigid_col2_unit"),
        "model buds as spheroids",
    );

    // Wall 5 — RETIRED. A mirror is a construction now
    //    (`topo::mirror_body`): a leaf mirrored across the plant's plane
    //    builds, and is a body at rest. The scene's three leaves are
    //    still placed by hand; authoring one and mirroring it is the
    //    scene's own change to make.
    let across = pncad::topo::Reflection::across(p3(0.0, 0.0, 0.0), v3(0.0, 1.0, 0.0), tol)
        .expect("the plant's plane has a normal");
    let mirrored = pncad::topo::mirror_body(leaf_a, &across, tol)
        .expect("a leaf mirrors across the plant's plane");
    finished("lily_leaf_a mirrored", mirrored, tol);
    println!("   wall 5 — RETIRED: a leaf mirrored across the plant's plane builds at rest");

    // Wall 6 — RETIRED, kept live. The lantern's mouth was a hard
    //    circle where the sphere zone meets the conical pucker, and the
    //    wall asked a rolling ball to soften it by requesting EVERY edge
    //    of the lantern at once. What that refused (`TangentialEdge` at
    //    margin exactly zero) was a seam meridian, an edge with the same
    //    sphere on both sides, which no fillet should take; the rim
    //    itself, asked for by both of its half-arcs, rolls. The scene
    //    now rolls it ([`roll_the_mouth`], r = [`FLOWER_MOUTH_ROLL`]),
    //    and what stays live is that the lantern the scene ships carries
    //    the band: one torus face, coaxial with the flower.
    let bands = lant
        .faces()
        .filter(|(_, f)| lant.get_surface(f.surface).map(|g| g.kind()) == Some(SurfaceKind::Torus))
        .count();
    assert_eq!(bands, 1, "the lantern's rolled mouth is one torus band");
    println!(
        "   wall 6 — RETIRED: the lantern's mouth is rolled at r = {FLOWER_MOUTH_ROLL} in \
         the scene, one torus band, at rest"
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
    //    does, because a full revolve sweeps each planar cap whole and
    //    the lantern arrives maximal-faced. The crossing layer has
    //    circle × sphere roots, the sphere pair's chord rides the two
    //    spheres' radical plane, and although this ball sits off the
    //    lantern's axis — the plane is tilted against the zone's polar
    //    axis — the join selects the section's arcs by the side of the
    //    run they leave on, and the carved zone measures by
    //    Gauss–Bonnet. So the CARVE builds, and one seam draws: the
    //    trimmed lane's sphere arm meshes a sphere face bounded by a
    //    circle tilted against its chart (this module's
    //    `verbs_gate_r1_probes`). The three seams are the same ball
    //    turned a third of a turn about the lantern's axis
    //    ([`seamed_lantern`]), and the seam a third of a turn round meets
    //    neither of the zone's seam meridians: its section lies inside
    //    one half-band, which keeps it as a ring, and the mesher has no
    //    lane for a curved face with a ring
    //    (`work/tess/valid-ringed-torus-faces-build-but-cannot-be-meshed.md`).
    //    The payload is quoted rather than described, the wall-7
    //    lesson about reading a locus off a comment instead of a dump.
    //
    //    The lantern's two axis-touching caps used to arrive as two
    //    half-faces each on one plane key, and the scene repaired them
    //    with `merge_coplanar_faces` before cutting. A full revolve
    //    now sweeps a planar wall whole (BAND, "one wall per run"), so
    //    the operand is cut as built.
    //
    //    What is left after that is the breadth half, DEPENDENCY-STATED
    //    like probe 8's: it waits on the verbs/breadth slate,
    //    VERBS-PLAN Wave 2 items 6 (VERBS-GATE, the per-face-kind gate
    //    re-scope) and 9 (VERBS-SPHSPH, the sphere × sphere germ lane)
    //    — the ruling that put it there is M9-5's, and the demand
    //    signal is this probe.
    // The carves build; drawing the three is what refuses. One carve
    // runs at the walls' own scalar, so its decisions record under the
    // probe scalar like every other wall's; the mesher is an f64 door,
    // so what it draws is the f64 carve, as a user's would be.
    let carve = |lantern: &AtRestBody<S>| {
        pncad::topo::subtract(
            lantern,
            &finished(
                "the wall-7 ball",
                ball::<S>(WALL7_BALL_CENTER, WALL7_BALL_RADIUS, tol),
                tol,
            ),
            tol,
        )
    };
    wall(
        7,
        "carve the lantern's three tepal seams and draw it (sphere x sphere by \
         geometry; each seam's section is tilted against the zone's chart)",
        carve(lant).map_err(Wall7::Carve).and_then(|_| {
            let lantern = plant::<f64>(tol)
                .into_iter()
                .find(|p| p.name == "lily_lantern")
                .expect("named lily piece")
                .body;
            let seamed = seamed_lantern(lantern, tol).map_err(Wall7::Carve)?;
            pncad::mesh::tessellate(&seamed, 2e-3, tol).map_err(Wall7::Draw)
        }),
        |e| {
            matches!(
                e,
                Wall7::Draw(pncad::mesh::TessellateError::RingOnCurvedFace { .. })
            )
        },
        "give the lanterns their three tepal seams",
    );
    // 8. The leaf GROWS OUT OF the stem. The scene already stands the
    //    long leaf's base in the lower stem: the base lens lies across
    //    the tube, its inner margin 0.021 from the tube's axis, and the
    //    sheath and blade run on out of it either side. So the join is
    //    the two pieces as authored, unioned, with no placement of its
    //    own. The tube's wall crosses the lens's ridge and keel walls in
    //    one loop about the base section, through sheath and blade
    //    alike, and cuts the inner margin's two tip seams; the walls
    //    cross steeply all round it (the sine of their angle reads 0.95
    //    or more off the authored numbers), so nothing here is a
    //    tangency.
    //
    //    What refuses is the first thing the union reads: a spline seam
    //    of the leaf, operand B, against the tube's torus wall. The
    //    curved crossing lane has rows for a line, a circle and an
    //    ellipse and none for a spline span, so it cannot say whether or
    //    where the seam crosses the wall, at 1e-9 and 1e-6 alike
    //    (`work/spliceface/a-spline-span-against-a-curved-face-is-not-dispatched-to-the-composite`).
    //    Behind it is the section of the leaf's NURBS walls against the
    //    torus, C5's rung 3, which is SECT's. At 1e-12 the lower stem's
    //    declared frontier stands the lily walls down before this one.
    wall(
        8,
        "grow the long leaf out of the stem (its NURBS walls through the \
         lower stem's torus wall)",
        crate::booleans::try_union(stem, leaf_a, tol),
        |e| {
            matches!(
                e,
                BooleanError::EdgeCarrierUnsupported {
                    operand: Operand::B,
                    edge,
                    face,
                    face_of: Operand::A,
                    site: EdgeCarrierSite::CurvedCrossing,
                } if query::edge_carrier_kind(leaf_a, *edge) == Some(CurveKind::Nurbs)
                    && query::face_surface_kind(stem, *face) == Some(SurfaceKind::Torus)
            )
        },
        "grow the leaves out of the stem instead of standing them beside it",
    );

    // 13 — RETIRED. It pinned the merge door SHUT on a full revolve's
    //      axis-touching caps: two half-faces on one plane key. #1031's
    //      pole half opened that door, and then the sweep stopped
    //      needing it: a full revolve sweeps a planar wall whole, so the
    //      lantern's caps arrive as ONE face each and there is nothing
    //      left to merge. What replaces the wall is that measurement,
    //      asserted on this scene's own lantern.
    {
        let mut before = Body::clone(lant);
        let outcome = before
            .merge_coplanar_faces(tol)
            .expect("probe 13 RETIRED: the merge door stays open");
        println!(
            "   wall 13 — RETIRED: the lantern's two axis-touching caps arrive \
             as one face each ({} merge group(s))",
            outcome.groups.len(),
        );
        assert_eq!(outcome.groups.len(), 0, "no cap arrives split");
    }

    // 15 — RETIRED. The swept leaves fit at `BLADE_V_DEGREE`, and the
    //      tour's own gate (`gated` in `main.rs`) holds every body to
    //      tier 3 at each ε row the tour runs, so no probe repeats it.

    // 16 — RETIRED. The lofted blades take the swept leaves' lens,
    //      and the tour's gate holds them to tier 3 at each ε row as it
    //      does every body, so no probe repeats it.

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
    use pncad::prelude::SurfaceKind;
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

    /// The degeneracy floor for [`assert_two_cap_planes`], and the
    /// only number in this probe that is a THRESHOLD rather than a
    /// measurement.
    ///
    /// It sits between the two windows in play. Below it is
    /// `assert_cap`'s own arithmetic: 1e-12 on the point-in-plane
    /// residual and 1e-14 on the parallelism, so two frames closer
    /// together than those are literally one plane as far as that
    /// check can see, and its disjunction over the two would be a
    /// single test wearing a disjunction's clothes. Above it is the
    /// scene: the closest the lily's own cap pairs come is the arch's
    /// `sin(10°) ≈ 0.17` of normal angle (a 170° wedge), five orders
    /// clear. A floor here is therefore unreachable by float noise
    /// and nowhere near any real value — it fails only on an actual
    /// collapse, and it is not a second, weaker copy of the placement
    /// claim.
    const CAP_DISTINCT: f64 = 1e-6;

    /// A tube's two joint frames are two DISTINCT planes.
    ///
    /// This is the floor under [`assert_cap`]'s disjunction, and it is
    /// what the disjunction cannot supply for itself: a body whose two
    /// frames had collapsed onto one plane satisfies every
    /// single-target call trivially, because the existential can no
    /// longer tell "both ends are where they should be" from "the ends
    /// are the same plane and one of them happens to be right". The
    /// arch is incidentally covered — its two calls carry different
    /// targets, so a collapse would fail one of them — but the stem
    /// and the pedicel each get ONE call per free end and nothing else
    /// is watching.
    ///
    /// Two planes are the same plane when their normals are parallel
    /// AND they carry the same offset, so distinctness is the
    /// disjunction of the two ways out: a normal angle, or a
    /// separation along the normal. Either alone would be wrong — a
    /// 180° wedge has parallel caps at different offsets, and a
    /// closing wedge has near-coincident caps at a real angle.
    fn assert_two_cap_planes(caps: &WedgeFrames<f64>, what: &str) {
        let (a, b) = (caps.start, caps.end);
        let angle = a.axis.cross(b.axis).norm();
        let offset = (b.origin - a.origin).dot(a.axis).abs();
        assert!(
            angle > CAP_DISTINCT || offset > CAP_DISTINCT,
            "{what}: the two joint frames are ONE plane (normal angle {angle:.3e}, \
             offset {offset:.3e}) — every single-target cap assertion on this body \
             is vacuous until they separate"
        );
    }

    /// One of the tube's two JOINT FRAMES passes through world point
    /// `p` (xz-plane) with normal parallel to `t` — i.e. the tube's
    /// end tangent THERE is `t`.
    ///
    /// The frames come from `sweep::revolved_caps` (LIB-U5). Which of
    /// the two ends answers is the revolve's business, so both are
    /// offered — that
    /// is a two-element check against NAMED caps, not a search of the
    /// whole boundary. What the disjunction is NOT free to do is get
    /// easier as the caps converge, so
    /// [`assert_two_cap_planes`] runs first on every call.
    ///
    /// **The sign is part of the claim.** A cap frame's `axis` is the
    /// stored plane normal in CHART sense (`topo::readback::face_pose`
    /// is explicit that it is not corrected by the face's orientation
    /// sense), and `revolve`'s partial path builds the start cap's
    /// plane from the profile chain REVERSED and the end cap's from
    /// the same chain forward (`sweep::revolve::partial`, phases 1 and
    /// 4). The two therefore disagree by construction, in the one way
    /// that makes both of them point OUT of the solid: the start cap's
    /// normal is minus the tube's tangent there, the end cap's is plus
    /// it. That is a per-cap rule, not a per-call one — it says what
    /// each frame must be IF it is the one at this joint — so it costs
    /// the disjunction nothing, and it closes the hole where an
    /// anti-parallel normal read as "parallel to the tangent".
    fn assert_cap(caps: &WedgeFrames<f64>, p: (f64, f64), t: (f64, f64), what: &str) {
        assert_two_cap_planes(caps, what);
        let tv = Vec3::new(t.0, 0.0, t.1);
        let hit =
            [(caps.start, -1.0), (caps.end, 1.0)]
                .into_iter()
                .any(|(pose, outward): (_, f64)| {
                    let (o, n) = (pose.origin, pose.axis);
                    n.cross(tv).norm() < 1e-14
                        && outward * n.dot(tv) > 0.0
                        && (Point3::new(p.0, 0.0, p.1) - o).dot(n).abs() < 1e-12
                });
        assert!(
            hit,
            "{what}: neither joint frame passes through the joint with the joint \
             tangent, in the cap's own outward sense"
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
        // Joint 0: the stem's FREE end. The turtle stands at the
        // origin facing +z (`plant`'s `root`), so unlike every joint
        // below this frame is authored and exact rather than walked —
        // no literal to derive, and no reason for the stem's start to
        // be the one frame in the chain nothing looks at.
        assert_cap(caps(&ps, "lily_stem"), (0.0, 0.0), (0.0, 1.0), "stem start");
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
                            axis.cross(tv).norm() < 1e-14,
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
                            axis.cross(tv).norm() < 1e-14,
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

    /// **The weld's door sequence, measured** — VERBS-LILYWELD PR-2's
    /// opening measurement, carried past the cone's admission to the
    /// boolean.
    ///
    /// The lantern's throat disk against the arch's end cap is an exact
    /// coincident planar Rest pair, which the union decides on its
    /// zero. Past the operand gate (the cone is on its roster) and F7,
    /// the union stops at the crossing layer on the WELD CIRCLE itself:
    /// the lantern's neck rim, which lies on the arch's torus wall as
    /// its terminal meridian circle — a coincidence the circle × torus
    /// lane does not decide. This row is what shows it moving when a
    /// coincidence door learns the circle.
    #[test]
    fn the_weld_refuses_on_the_weld_circle() {
        let tol = Tol::witness();
        let ps = pieces();
        let (lant, arch) = (body(&ps, "lily_lantern"), body(&ps, "lily_arch"));
        let (lant, arch) = (
            &finished("lily_lantern", lant.clone(), tol),
            &finished("lily_arch", arch.clone(), tol),
        );
        let refused = pncad::topo::union(lant, arch, tol).expect_err("the weld still refuses");
        println!("weld: {refused:?}");
        let BooleanError::CurvedPierceUnsupported {
            operand: Operand::A,
            face,
            edge,
            ..
        } = refused
        else {
            panic!("the weld stops at the curved pierce: {refused:?}");
        };
        let carrier = lant
            .get_edge(edge)
            .and_then(|e| lant.get_curve_geom(e.curve))
            .and_then(|g| g.certified())
            .map(|c| c.carrier().clone());
        let kind = arch
            .get_face(face)
            .and_then(|f| arch.get_surface(f.surface))
            .map(|s| s.kind());
        assert!(
            matches!(
                carrier,
                Some(pncad::geom::Curve3::Circle { radius, .. }) if (radius - ARCH_R).abs() < 1e-12
            ) && kind == Some(SurfaceKind::Torus),
            "the refused edge is the weld circle, on the arch's torus wall: \
             {carrier:?} on {kind:?}"
        );
    }

    /// **The lantern's caps arrive whole** — the shape the F7 rule
    /// used to call a defect, gone at its source.
    ///
    /// The lantern has two PLANAR caps that touch the axis, the LIP
    /// disk and the THROAT disk. A full revolve used to cut each at its
    /// two seam meridians into two half-faces on one plane key, which
    /// `merge_coplanar_faces` then repaired; it now sweeps a planar
    /// wall whole (BAND, "one wall per run"), so neither cap carries a
    /// same-key adjacency. Its CURVED walls (two cones, one sphere
    /// zone) are still cut at the seam, and the row tells the two
    /// apart — no planar pair, six curved — so a change that stopped
    /// doing so fails here rather than silently. The body as built is
    /// tier 2 and tier 3 clean (the delta review's MIN-1 asked for
    /// both real validators, not the tier 1 one).
    #[test]
    fn the_lanterns_caps_arrive_whole() {
        let tol = Tol::witness();
        let ps = pieces();
        let lant = body(&ps, "lily_lantern");
        let mut planar_pairs = 0usize;
        let mut curved_pairs = 0usize;
        for (k, _) in lant.edges() {
            let Ok(sides) = pncad::topo::readback::edge_sides(lant, k) else {
                continue;
            };
            if sides.plus.face == sides.minus.face || sides.plus.surface != sides.minus.surface {
                continue;
            }
            match lant.get_surface(sides.plus.surface) {
                Some(Surface::Plane { .. }) => planar_pairs += 1,
                Some(_) => curved_pairs += 1,
                None => {}
            }
        }
        // Each pair is counted from both of its two struts.
        assert_eq!(
            (planar_pairs, curved_pairs),
            (0, 6),
            "two whole planar caps (the lip disk and the throat disk) and three \
             curved walls (two cones, one sphere zone), each split at its seam"
        );
        assert_eq!(
            pncad::topo::validate_closed(lant),
            Ok(()),
            "tier 2 as built"
        );
        assert_eq!(
            pncad::topo::validate_geometric(lant, tol),
            Ok(()),
            "tier 3 as built"
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
            (circle.c.x - P2.0).abs() < 1e-12 && (circle.c.z - P2.1).abs() < 1e-12,
            "the weld circle is centred on the arch's last spine point"
        );
        assert!(
            circle.c.y.abs() < 1e-15,
            "the weld circle's centre is in the plant's own plane"
        );
        assert!(
            circle.n.cross(Vec3::new(T2.0, 0.0, T2.1)).norm() < 1e-14,
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
    /// disk and the belly, and a FULL revolve emits every CURVED wall as
    /// two half-bands on one carrier, so the neck arrives as two faces
    /// with the seam struts and rim vertices that go with them. Its two
    /// planar caps it sweeps whole (BAND, one wall per run: 10 faces,
    /// 18 edges, 10 vertices became 8, 14, 8). The
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
        // RE-BASELINED (wall 6 retired: the mouth rolled into one
        // torus band): (1, 8, 14, 8) -> (1, 9, 17, 10), one band face
        // between the zone and the pucker, its two tangency circles
        // each two half-arcs, and their seam vertices.
        assert_eq!(census, (1, 9, 17, 10), "the re-authored lantern's census");
        let props = pncad::topo::mass_properties(
            &crate::booleans::at_rest(lant, Tol::witness()),
            Tol::witness(),
            crate::printed(),
        )
        .expect("mass properties");
        // The literal, DELIBERATELY: this row and
        // `finding_13_tessellation_table_reproduces` must not agree by
        // sharing an expression. That row builds the closed form from
        // the profile's own constants and compares a MESH against it;
        // this one compares the kernel's `mass_properties` door
        // against a transcribed number, so a change to the closed-form
        // expression cannot move both readings together.
        // RE-BASELINED (wall 6 retired): 0.36455193285177373 for the
        // sharp lantern, plus the 1.14119807294e-5 the concave mouth
        // roll adds at r = 0.25 (`mouth_roll_delta_v`, by Pappus).
        let exact = 0.36456334483250313;
        assert!(
            (props.enclosure.volume - exact).abs() < 1e-12,
            "lantern volume {} vs the closed form {exact}",
            props.enclosure.volume
        );
        assert_eq!(
            props.enclosure.volume_pad, 0.0,
            "every lantern face is closed-form"
        );
    }

    /// The volume the mouth's roll ADDS ([`roll_the_mouth`]): the
    /// meridian region between the globe's arc, the pucker's line and
    /// the rolling ball's arc, revolved. The crease is concave, so the
    /// ball rides OUTSIDE both supports: its centre is `globe + roll`
    /// from the globe centre and `roll` off the pucker's line on the
    /// side away from the material. By Green's theorem Pappus's
    /// `2π∬ρ dA` is `π∮ρ² dy` round that region's three sides, the
    /// snowman's waist band ([`crate::snowman::arc_rho2_dy`]) with one
    /// arc a line. Coordinates are the sketch's `(ρ, t)` about the
    /// globe centre; volume does not care where along the axis that is.
    fn mouth_roll_delta_v(globe: f64, mouth: f64, lip_r: f64, lip_drop: f64, roll: f64) -> f64 {
        let m = ((globe * globe - mouth * mouth).sqrt(), mouth);
        let l = (lip_r, mouth + lip_drop);
        let len = (l.0 - m.0).hypot(l.1 - m.1);
        let u = ((l.0 - m.0) / len, (l.1 - m.1) / len);
        // The side away from the axis, the material's outside.
        let n = (u.1, -u.0);
        let a = (m.0 + n.0 * roll, m.1 + n.1 * roll);
        // The ball's centre is `a + s·u` at `globe + roll` from the
        // globe centre: the larger root of a quadratic in s.
        let b = 2.0 * (a.0 * u.0 + a.1 * u.1);
        let c = a.0 * a.0 + a.1 * a.1 - (globe + roll).powi(2);
        let s = (-b + (b * b - 4.0 * c).sqrt()) / 2.0;
        let centre = (a.0 + u.0 * s, a.1 + u.1 * s);
        let on_globe = (
            centre.0 * globe / (globe + roll),
            centre.1 * globe / (globe + roll),
        );
        let on_pucker = (centre.0 - n.0 * roll, centre.1 - n.1 * roll);
        let line =
            |p: (f64, f64), q: (f64, f64)| (q.1 - p.1) * (p.0 * p.0 + p.0 * q.0 + q.0 * q.0) / 3.0;
        (PI * (line(m, on_pucker)
            + crate::snowman::arc_rho2_dy(centre, roll, on_pucker, on_globe)
            + crate::snowman::arc_rho2_dy((0.0, 0.0), globe, on_globe, m)))
        .abs()
    }

    /// Finding 13 re-measured: one chord budget spends wildly
    /// differently per body, and these are the numbers.
    ///
    /// The five analytic rows are the SAME counts the sketch-frame
    /// revolve produced — the tube door changed which parameters are
    /// stored, not which torus they describe, so the tessellator sees
    /// the same surface and splits it the same way. The two SWEPT
    /// blade rows are the other half of the finding, and it runs the
    /// other way: a swept lens skin costs about six and a half times the
    /// stem tube at the same δ (5_374 against 828 at 2e-3). Part of that
    /// is geometry — the lens is curved across, where a straight-sided
    /// section is flat — and part is the sizing lane: a rational wall
    /// is sized per cell of the 16 × 16 refinement its certified bound
    /// is assembled on, and that per-cell grid builds more cells than
    /// the whole-patch schedule would
    /// (`work/tess/rational-wall-cell-grid-costs-more-than-the-whole-patch-schedule.md`).
    /// A torus spends the chord budget per curvature, through
    /// `mesh::sizing::torus_grid_steps`; a rational skin, per cell.
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
        // RE-BASELINED (PRED, `[mesh.sizing]`): each mesher lane sizes
        // its worst triangle at δ by its own theorem rather than at δ/2,
        // so the stem's and arch's tori and the lantern's sphere zone
        // and cones and its mouth's torus band coarsen — stem
        // 392 -> 188 and 828 -> 476, arch 2_960 -> 1_472, lantern
        // 1_220 -> 832 and 3_038 -> 1_900 (its cones take no rows, its
        // sphere zone sizes its azimuth at its widest latitude). The
        // leaves are NURBS, whose lattice did not move.
        let table = [
            ("lily_stem", 5e-3, 188usize),
            ("lily_stem", 2e-3, 476),
            ("lily_arch", 2e-3, 1_472),
            // RE-BASELINED (BAND, one wall per run): a full revolve
            // sweeps the lantern's two planar caps whole, and each cap
            // as one disc meshes two triangles fewer than as two
            // half-discs — 1_084 -> 1_080 and 2_560 -> 2_556.
            // RE-BASELINED (wall 6 retired): the mouth's torus band,
            // sized per curvature like any torus, adds its own cells —
            // 1_080 -> 1_220 and 2_556 -> 3_038.
            ("lily_lantern", 5e-3, 832),
            ("lily_lantern", 2e-3, 1_900),
            // RE-BASELINED: the swept leaves are fitted at the cubic
            // degree along the path (wall 15 retired) — 6_468 -> 5_576
            // and 5_992 -> 5_164. RE-BASELINED again (PR 4984, one
            // second-order quotient for rational bounds): the blade
            // face's certified second-derivative sups tighten (`muu`
            // 0.138 -> 0.126, `muv` 0.043 -> 0.023 on leaf b), so its
            // per-cell grid drops 96 cells — 5_576 -> 5_374 and
            // 5_164 -> 4_962. The sampled |S − Π| stays under 8.5e-5
            // against the face's 1e-3. RE-BASELINED (#5531, the
            // trimmed lane's CDT in the lattice metric): leaf b's blade
            // face no longer snaps its bands to the patch column count
            // (71 snapped bands -> 0: their aspects are malign in raw
            // UV and benign over the face's metric), so each band keeps
            // its own columns and the grid drops 1_152 -> 1_081 cells —
            // 5_374 -> 5_254. Leaf c measures unchanged.
            ("lily_leaf_b", 2e-3, 5_254),
            ("lily_leaf_c", 2e-3, 4_962),
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
        //   π[r²(a+b) − (a³+b³)/3] + π·h(R² + Rρ + ρ²)/3 per frustum,
        // plus the material the mouth's roll adds
        // ([`mouth_roll_delta_v`]). It comes to 0.36456334483250313 m³.
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
            + frustum(lip_drop, r_mouth, lip_r)
            + mouth_roll_delta_v(globe, mouth, lip_r, lip_drop, FLOWER_MOUTH_ROLL);
        let rels: Vec<(f64, f64)> = [5e-3, 2e-3]
            .into_iter()
            .map(|delta| {
                let m = pncad::mesh::tessellate(body(&ps, "lily_lantern"), delta, Tol::witness())
                    .expect("tessellate");
                (delta, ((signed_volume(&m) - exact) / exact).abs())
            })
            .collect();
        println!("lantern volume error, measured: {rels:?}");
        // RE-BASELINED (PRED, `[mesh.sizing]`): the rolled lantern
        // measured 0.012769 and 0.0050019 at the δ/2 sizing and measures
        // 0.015945 and 0.0062491 now — the chords' inscribed deficit
        // grows with the coarser grid, inside the `area·δ` a mesh
        // within δ allows.
        for ((delta, rel), (lo, hi)) in rels.into_iter().zip([(0.0153, 0.0167), (0.0060, 0.0067)]) {
            assert!(rel > lo && rel < hi, "lantern @ {delta:e}: rel {rel}");
        }
        // A swept blade has no analytic wall to compare against, but it
        // has PAPPUS. A rigid section carried in the path's normal
        // frame sweeps A·(centroid arc length). The lens's area and
        // centroid are its two circular segments' closed forms
        // ([`Lance::area`], [`Lance::centroid_rise`]), and the centroid
        // sits that far above the chord, i.e. OUTSIDE the spine's
        // centre of curvature, so its arc is len + |curl|·rise.
        //
        // Pappus is the IDEAL sweep's volume, and the blade is a skin
        // through [`LEAF_STATIONS`] sections on a cubic interpolant of
        // the arc, so the two differ by the fit. The kernel's certified
        // volume must therefore contain the skin's OWN volume,
        // integrated over its surfaces
        // ([`crate::patch_quadrature`]), at every width: a
        // sound enclosure misses the true volume at none. Pappus is
        // held to the skin at the fit's scale: a cubic spline through
        // stations `Δ` apart in the arc's angle misses it by
        // `(5/384)·Δ⁴` of its own scale, and a section's area is
        // quadratic in its scale, so the volume misses by about twice
        // that. And the mesh must fall SHORT of the skin by between
        // 3e-3 and 6e-3: every chord across the convex lens cuts
        // inside it, and the inscribed deficit measured 4.4e-3 and
        // 5.0e-3 at δ = 2e-3. A mesh that met the skin would not have
        // been measured off a real tessellation; one further off would
        // mean the section rolled about the tangent on its way down
        // the path.
        //
        // `lily_leaf_a` is NOT in this list, and its absence is the
        // point: it is the lofted blade, and it both tapers and rolls.
        // Pappus wants a rigid section carried in the normal frame,
        // which is exactly what a loft stops being. What pins the
        // lofted blade instead is
        // `the_lofted_blade_tapers_and_rolls_in_the_stored_geometry`.
        for (name, leaf) in [("lily_leaf_b", LEAF_B), ("lily_leaf_c", LEAF_C)] {
            let pappus = leaf.section.area()
                * leaf
                    .curl
                    .abs()
                    .mul_add(leaf.section.centroid_rise(), leaf.len);
            let b = body(&ps, name);
            // The caps are the authored lens placed rigidly, and each
            // reading is taken about two points: a shell that does not
            // close reads two volumes.
            let cap = |_o: [f64; 3], _n: [f64; 3]| leaf.section.area();
            let about = [[0.0; 3], [leaf.base.x, leaf.base.y, leaf.base.z]];
            let coarse = crate::patch_quadrature::measure(b, 24, &about, cap);
            let fine = crate::patch_quadrature::measure(b, 32, &about, cap);
            let skin_v = fine.volume[0];
            let skin_err = (skin_v - coarse.volume[0])
                .abs()
                .max((skin_v - fine.volume[1]).abs());
            #[allow(clippy::cast_precision_loss)]
            let fit_step = leaf.curl.abs() / (LEAF_STATIONS - 1) as f64;
            let fit_scale = 2.0 * (5.0 / 384.0) * fit_step.powi(4);
            let fit = ((skin_v - pappus) / pappus).abs();
            assert!(
                fit <= fit_scale,
                "{name}: the skin's V = {skin_v} misses Pappus {pappus} by {fit:e} of it, past \
                 the cubic fit's scale {fit_scale:e}"
            );
            let e = pncad::topo::validate_geometric_certificate(b, Tol::witness())
                .expect("the swept leaf is tier 3 clean")
                .measure(crate::printed())
                .unwrap_or_else(|refusal| panic!("{name}: no certified volume: {refusal}"))
                .enclosure
                .enclosure();
            let (lo, hi) = (e.volume_lo, e.volume_hi);
            assert!(
                lo <= skin_v - skin_err && skin_v + skin_err <= hi,
                "{name}: the skin's own V = {skin_v} (± {skin_err:e}) outside the certified \
                 [{lo}, {hi}]"
            );
            let m = pncad::mesh::tessellate(b, 2e-3, Tol::witness()).expect("tessellate");
            let short = (skin_v - signed_volume(&m)) / skin_v;
            assert!(
                short > 3e-3 && short < 6e-3,
                "{name}: mesh short by {short}"
            );
        }
    }

    /// The lofted blade's two claims — TAPER and ROLL — read off the
    /// STORED body, not off the construction code.
    ///
    /// Both are read through the body's two planar cap faces, which
    /// are the loft's end sections placed in the world. For each cap:
    /// its two vertices, the MARGINS, whose chord is the width and
    /// whose direction is the width axis `u`, and `v = n x u`.
    ///
    /// The section's ridge/keel ASYMMETRY is what makes the roll
    /// measurable at all: `v`'s sign is otherwise a coin flip, because
    /// a `u` read off two margins is only defined up to sign. The
    /// ridge is authored strictly deeper than the keel, so the side of
    /// the chord the deeper rim's apex lies on picks the orientation
    /// out uniquely, and the angle from the base frame to the tip frame
    /// is then a full signed turn rather than a turn mod pi.
    #[test]
    fn the_lofted_blade_tapers_and_rolls_in_the_stored_geometry() {
        let ps = pieces();
        let b = body(&ps, "lily_leaf_a");
        let caps = cap_frames(b);
        // The BASE cap is the wide one; the TIP cap the narrow one.
        let (base, tip) = if caps[0].width > caps[1].width {
            (&caps[0], &caps[1])
        } else {
            (&caps[1], &caps[0])
        };
        // TAPER. The authored base lens is 0.170 margin to margin and
        // the tip lens 0.060. A sweep could produce neither number from
        // the other — one profile goes down the path.
        assert!(
            (base.width - 0.170).abs() < 1e-9,
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
        // across them mixes the roll with the spine's own turn. That
        // vanishes if the comparison is made
        // against the SAME blade built with the twist set to zero:
        // identical spine, identical stations, identical sections, so
        // the two tip caps are coplanar and their frames differ by the
        // roll and nothing else.
        let untwisted = leaf_a::<f64>(
            Plan {
                twist: 0.0,
                ..leaf_a_plan()
            },
            Tol::witness(),
        );
        let flat = cap_frames(&untwisted);
        let flat_tip = if flat[0].width > flat[1].width {
            &flat[1]
        } else {
            &flat[0]
        };
        assert!(
            flat_tip.n.cross(tip.n).norm() < 1e-12,
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
    /// lantern actually stores: every margin and every cap rim apex of
    /// every sepal is at least `FLOWER_GLOBE` from the globe centre, so
    /// no sepal enters
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
            // The margins, and the apex of each cap's two rims: the
            // keel's apex is the section's deepest point.
            let mut points: Vec<Point3<f64>> = s.vertex_points().map(|(_, p)| p).collect();
            for (_, f) in s.faces() {
                if let Some(&pncad::geom::Surface::Plane { origin, normal, .. }) =
                    s.get_surface(f.surface)
                {
                    let margins: Vec<Point3<f64>> = s
                        .vertex_points()
                        .map(|(_, p)| p)
                        .filter(|p| (*p - origin).dot(normal).abs() < 1e-9)
                        .collect();
                    let mid = margins[0] + (margins[1] - margins[0]) * 0.5;
                    points.extend(rim_apices(s, origin, normal, mid));
                }
            }
            for p in points {
                let d = p.distance(g);
                // 1e-12, not 0: the base keel apex is placed AT the
                // sphere by construction, and it gets there through a
                // rotation composed into the section placement, so it
                // lands within an ulp or two of R rather than on it.
                // Anything that actually entered the flower would be
                // inside by a section thickness, six orders of
                // magnitude past this.
                assert!(
                    d >= FLOWER_GLOBE - 1e-12,
                    "{name}: a rim point is {d} from the globe centre, inside R = {FLOWER_GLOBE}"
                );
                closest = closest.min(d);
            }
        }
        // Two-sided: the sepals must GRAZE, not merely miss. The base
        // section's keel is authored 0.008 and the stand-off is
        // R + keel, so the nearest rim point sits at R + 0 (the keel's
        // apex, on the sphere) — within the float noise of a rotation
        // composed through the placement.
        //
        // 1e-12, the SAME window as the inside bound above, so the
        // pair is a symmetric ±1e-12 collar on R and the claim "the
        // nearest sepal rim point is within 1e-12 of the globe" is the
        // literal assertion rather than a summary of it.
        println!(
            "sepal tangency: nearest rim point {:e} from R",
            closest - FLOWER_GLOBE
        );
        assert!(
            closest < FLOWER_GLOBE + 1e-12,
            "nearest sepal rim point {closest} — tangency claimed, {} of clearance found",
            closest - FLOWER_GLOBE
        );

        // And the OTHER flower. Tangency to the globe a sepal stands
        // on says nothing about the bud 1.44 m up the stem, and the
        // kernel will not say anything either: these are separate
        // bodies, this scene joins none of them (its one union is the
        // rootstock, underground), so one solid passing
        // through another is not a condition any operation here could
        // refuse. It is the SCENE's invariant, so the scene tests it.
        // Sepal 0's radial, unphased, points almost exactly at the bud
        // and a 1.05 m sepal reaches 0.027 m inside it — this assert
        // is what caught that, and reds if the phase is dropped.
        for seg in ["lily_bud_a", "lily_bud_b", "lily_bud_c"] {
            let (bc, br, _) = sphere_of(body(&ps, seg));
            for name in ["lily_sepal_a", "lily_sepal_b", "lily_sepal_c"] {
                let sb = body(&ps, name);
                for (_, p) in sb.vertex_points() {
                    let d = p.distance(bc);
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
        let across = |a: Vec3<f64>| a.reject_from(bud_axis).normalize();
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
                let h = normal.cross(a).normalize();
                let s: f64 = b
                    .vertex_points()
                    .map(|(_, p)| p)
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
            let place = (halves[0] + halves[1]).normalize();
            // Read both across the BUD's axis and take the signed
            // angle from the place to the lean. Sign and magnitude
            // are both the claim: 0 would be the star, and the
            // opposite sign the mirror-image pinwheel.
            let across = |v: Vec3<f64>| v.reject_from(bud_axis).normalize();
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
        /// Distance between its two margins.
        width: f64,
        /// The in-plane axis perpendicular to the width, signed so it
        /// points from the chord toward the RIDGE (the deeper of the
        /// section's two rises).
        v: Vec3<f64>,
    }

    /// Both caps of a lofted blade, each reduced to a [`Cap`].
    ///
    /// **Two, and the arity is checked here rather than at the
    /// callers.** Every caller reads the result positionally — the
    /// wide cap against the narrow one — so a body that handed back
    /// one plane, or three, would be indexed as if it were a blade
    /// and answer with a frame belonging to some other face. The
    /// per-face `on.len() == 8` below cannot catch that: it fires on
    /// the vertex set of whatever plane it is looking at, one plane
    /// too late to say the SET of planes is wrong.
    /// The apices of a lofted blade's section rims on the cap plane
    /// `(origin, normal)` whose chord midpoint is `mid`: each rim is a
    /// stored circle arc through both margins, under a half turn, so
    /// its apex lies on the chord's bisector one radius from its centre,
    /// on the side away from it.
    fn rim_apices(
        b: &Body<f64>,
        origin: Point3<f64>,
        normal: Vec3<f64>,
        mid: Point3<f64>,
    ) -> Vec<Point3<f64>> {
        b.edges()
            .filter_map(|(_, e)| {
                let geom = b.get_curve_geom(e.curve)?.certified()?;
                let pncad::geom::Curve3::Circle { center, radius, .. } = *geom.carrier() else {
                    return None;
                };
                ((center - origin).dot(normal).abs() < 1e-9).then(|| {
                    let out = mid - center;
                    center + out * (radius / out.norm())
                })
            })
            .collect()
    }

    fn cap_frames(b: &Body<f64>) -> Vec<Cap> {
        let mut out = Vec::new();
        for (_, f) in b.faces() {
            let Some(&pncad::geom::Surface::Plane { origin, normal, .. }) =
                b.get_surface(f.surface)
            else {
                continue;
            };
            let on_plane = |p: Point3<f64>| (p - origin).dot(normal).abs() < 1e-9;
            let on: Vec<Point3<f64>> = b
                .vertex_points()
                .map(|(_, p)| p)
                .filter(|p| on_plane(*p))
                .collect();
            assert_eq!(on.len(), 2, "a lens section's vertices are its two margins");
            let width = (on[1] - on[0]).norm();
            let axis = (on[1] - on[0]) / width;
            let mid = on[0] + (on[1] - on[0]) * 0.5;
            // v completes the frame; its SIGN is fixed by asking which
            // side of the chord the deeper rim (the ridge) rises on.
            let mut v = normal.cross(axis);
            let rises: Vec<f64> = rim_apices(b, origin, normal, mid)
                .into_iter()
                .map(|p| (p - mid).dot(v))
                .collect();
            assert_eq!(rises.len(), 2, "a lens section has two rims: {rises:?}");
            let hi = rises.iter().copied().fold(f64::NEG_INFINITY, f64::max);
            let lo = rises.iter().copied().fold(f64::INFINITY, f64::min);
            if hi < -lo {
                v = -v;
            }
            out.push(Cap {
                n: normal,
                width,
                v,
            });
        }
        assert_eq!(
            out.len(),
            2,
            "a lofted blade has exactly two planar cap faces"
        );
        out
    }

    /// The signed angle from `a` to `b` about `axis` (all unit, `a`
    /// and `b` perpendicular to `axis` up to float noise).
    fn signed_angle(a: Vec3<f64>, b: Vec3<f64>, axis: Vec3<f64>) -> f64 {
        let s = a.cross(b).dot(axis);
        let c = a.dot(b);
        s.atan2(c)
    }

    /// **The spine curl wall, re-measured against the per-slab
    /// stacking fold.** The wall is no longer at spine turn π and is
    /// no longer about how far the spine goes. The loft's stacking
    /// statement is a fold over adjacent section pairs, each decided
    /// against both its sections' normals, so a blade whose spine
    /// turns a full circle and more builds as long as each of its
    /// slabs advances — and at [`LOFT_STATIONS`] stations each slab
    /// carries 1/16 of the turn.
    ///
    /// What remains is a SAMPLING wall, at per-slab turn π, i.e. total
    /// curl `(stations − 1)·π`. This probe pins it from both sides in
    /// the scene's own vocabulary by lofting the same blade at four
    /// stations, where the wall is at `3π ≈ 9.42` rad, and the
    /// refusal NAMES the slab that walls. At [`LOFT_STATIONS`] the
    /// blade coils past two full turns and still builds, so the
    /// refusal side is pinned at four stations instead.
    #[test]
    fn the_spine_curl_wall_re_measured() {
        let blade = |curl: f64, stations: usize| {
            try_lofted_blade::<f64>(
                LEAF_A_BASE,
                LEAF_A_DIR,
                LEAF_A_UP,
                LEAF_A_LEN,
                -curl,
                leaf_a_plan(),
                stations,
                Tol::witness(),
            )
        };
        // Through π and far past it: every one of these refused
        // `ReversedStacking` above 3.0 rad under the end-to-end
        // statement, on a summary none of their slabs disagreed with.
        for curl in [0.45, 1.0, 2.0, 2.5, 2.8, 3.0, 3.5, 4.7, 6.0, 9.0, 13.0] {
            let out = blade(curl, LOFT_STATIONS);
            assert!(
                out.is_ok(),
                "curl {curl} rad over {LOFT_STATIONS} stations refused ({:?}) — \
                 each slab turns {:.4} rad, nowhere near the per-slab wall at π; \
                 re-derive this probe and the lofted_blade prose",
                out.err(),
                curl / ((LOFT_STATIONS - 1) as f64),
            );
        }
        // The wall that remains is per-slab, and at THIS scene's
        // sampling it is out of reach: 13.0 rad over 17 stations is
        // 0.81 rad per slab and still builds. Coarsen the sampling and
        // it comes into range — 10.0 rad over FOUR stations is 3.33
        // rad per slab, past π, and the refusal names the slab.
        //
        // Measured while re-deriving this probe, and worth knowing
        // before anyone reads the four-station row as the whole story:
        // at four, five, six and eight stations the blade meets a
        // DIFFERENT wall first — `nurbs_span_meter` escalating inside
        // the Euler assembly (#222's meter) — from 3.0 rad upward, so
        // the build side of the per-slab wall is not reachable at the
        // same station count as its refusal side in this scene. Both
        // sides ARE executed together, on a bare arc spine, in
        // `crates/sweep/tests/bool6_per_slab_stacking.rs`.
        let out = blade(10.0, 4);
        assert!(
            matches!(
                out,
                Err(pncad::sweep::LoftError::ReversedStacking { slab: 0 })
            ),
            "10.0 rad over 4 stations is 3.33 rad per slab: expected the per-slab \
             wall, naming slab 0, got {out:?} — the stacking wall moved; re-derive \
             this probe, the lofted_blade prose, and KERNEL-VERBS together"
        );
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
    /// arithmetic. A slab box of the pucker (max-generator radius along
    /// its whole axial range) overlaps the carving ball's box, but the
    /// pucker's EXACT frustum never comes within the ball's radius of
    /// it, while the sphere ZONE's carrier does meet the ball. So a
    /// (Cone, Sphere) pair named on boxes is pure box looseness; the
    /// geometry the model cares about is sphere-on-sphere.
    #[test]
    fn wall7_the_cone_pair_is_box_looseness_the_ball_meets_the_zone() {
        let tol = Tol::witness();
        let pieces = plant::<f64>(tol);
        let lant = &pieces
            .iter()
            .find(|p| p.name == "lily_lantern")
            .expect("lantern piece")
            .body;
        // The carving ball of wall 7.
        let (bc, br) = (super::WALL7_BALL_CENTER, super::WALL7_BALL_RADIUS);
        let pucker = super::review_probes::pucker_cone_faces(&pieces);
        let mut min_frustum_gap = f64::INFINITY;
        let mut zone_hit = false;
        let mut zone = None;
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
                    let rad = w.reject_from(axis).norm();
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
                        zone = Some((center, radius));
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

        // The cone arm boxes the FRUSTUM its axial window cuts rather
        // than a slab pinned at the window's widest radius. This row
        // prints that AABB against the ball's and the per-axis overlap,
        // and holds the box clear of the ball below.
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
        let ball_body = finished("the wall-7 ball", ball::<f64>(bc, br, tol), tol);
        let lant = &finished("lily_lantern", lant.clone(), tol);
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
        // With the caps swept whole, the crossing layer's circle × sphere
        // roots, the radical-plane join and its chords' arcs, taken from
        // the germs, for a section tilted against the zone's polar axis, the carve
        // builds under every op, and each answer meets the lens the
        // ball and the zone's sphere share, from the radii and the
        // centre distance alone. What stops the scene is drawing it.
        assert!(
            tightest < 0.0,
            "the pucker's box must clear the ball's for the gate to admit; it does \
             not, so this row's reading of the carve below is wrong"
        );
        let (zc, zr) = zone.expect("the zone's sphere");
        let d = (zc - bc).norm();
        let x = (d * d + zr * zr - br * br) / (2.0 * d);
        let cap = |r: f64, h: f64| PI * h * h * (3.0 * r - h) / 3.0;
        let lens = cap(zr, zr - x) + cap(br, br - (d - x));
        let (va, vb) = (volume(lant, tol), 4.0 / 3.0 * PI * br.powi(3));
        let built = |label: &str, r: Result<pncad::topo::BooleanResult<f64>, BooleanError>| {
            let r = r.unwrap_or_else(|e| panic!("{label}: the carve builds, got {e:?}"));
            let body = r.body().expect("a body").body.clone();
            assert_eq!(
                pncad::topo::validate_geometric(&body, tol),
                Ok(()),
                "{label}"
            );
            body
        };
        let carved = built("A ∖ B", pncad::topo::subtract(lant, &ball_body, tol));
        for (label, body, want) in [
            ("A ∖ B", carved.clone(), va - lens),
            (
                "A ∪ B",
                built("A ∪ B", pncad::topo::union(lant, &ball_body, tol)),
                va + vb - lens,
            ),
            (
                "A ∩ B",
                built("A ∩ B", pncad::topo::intersect(lant, &ball_body, tol)),
                lens,
            ),
            (
                "B ∖ A",
                built("B ∖ A", pncad::topo::subtract(&ball_body, lant, tol)),
                vb - lens,
            ),
        ] {
            let got = volume(&body, tol);
            println!("wall-7 probe: {label} volume {got}, lens oracle {want}");
            assert!(
                (got - want).abs() <= 1e-9 * want.max(1.0),
                "{label}: volume {got} against the lens oracle {want}"
            );
        }
        // One seam draws: its section crosses the zone's seam meridians,
        // and the trimmed lane's sphere arm meshes the faces it bounds.
        let draw = pncad::mesh::tessellate(&carved, 2e-3, tol)
            .unwrap_or_else(|e| panic!("the carve draws, got {e:?}"));
        pncad::mesh::validate::check_mesh(&draw)
            .unwrap_or_else(|e| panic!("the carve's mesh checks, got {e:?}"));
        let carved_area = pncad::topo::mass_properties(
            &crate::booleans::at_rest(&carved, tol),
            tol,
            crate::printed(),
        )
        .expect("the carve's area integrates")
        .enclosure
        .surface_area
        .f();
        pncad::mesh::validate::check_volume(&draw, va - lens, carved_area, 2e-3)
            .unwrap_or_else(|e| panic!("the carve's mesh encloses the carve, got {e:?}"));
        // All three do not: the seam a third of a turn round lies inside
        // one half-band of the zone, a ring of that face.
        let seamed = super::seamed_lantern(lant.clone().into_body(), tol)
            .unwrap_or_else(|e| panic!("the three seams carve, got {e:?}"));
        let drawn = pncad::mesh::tessellate(&seamed, 2e-3, tol);
        assert!(
            matches!(
                drawn,
                Err(pncad::mesh::TessellateError::RingOnCurvedFace { .. })
            ),
            "the seamed lantern's ringed zone face has no tessellation lane — got {:?}",
            drawn.map(|m| m.positions.len())
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
        let operand = |name: &str| finished(name, by(name).clone(), tol);
        let (stem, arch, lant) = (
            &operand("lily_stem"),
            &operand("lily_arch"),
            &operand("lily_lantern"),
        );

        crate::booleans::try_union(stem, arch, tol).expect("the stem's two arcs glue");

        // **The weld itself has no torus contact**, measured
        // off the two loci: the stem tube's end circle has radius
        // `STEM_R` = 0.060 and the arch's start disc radius `ARCH_R` =
        // 0.052, concentric and coplanar on the weld plane, so the two
        // stand `0.060 − 0.052 = 0.008 m` apart.
        //
        // The arch's cap planes, named by position: the weld disc is
        // the one within 2 m of the world origin (where the stem
        // starts).
        let weld = arch
            .faces()
            .filter_map(|(_, f)| match arch.get_surface(f.surface) {
                Some(&Surface::Plane { origin, .. }) => Some(origin),
                _ => None,
            })
            .find(|&o| (o - pncad::geom_core::Point3::origin()).norm() <= 2.0)
            .expect("the arch carries a weld cap");
        let ring_c = pncad::geom_core::Point3::new(-5.0, 0.0, 0.0);
        let to_stem_wall = |p: pncad::geom_core::Point3<f64>| {
            let rel = p - ring_c;
            let theta = rel.z.atan2(rel.x).clamp(0.0, deg(22.0));
            let on = pncad::geom_core::Point3::new(
                ring_c.x + 5.0 * theta.cos(),
                0.0,
                ring_c.z + 5.0 * theta.sin(),
            );
            // The UNSIGNED distance to the tube SURFACE: a point of
            // the weld disc sits inside the tube's radius, so the
            // signed form would report how far in, not how far off.
            ((p - on).norm() - STEM_R).abs()
        };
        // The weld disc, sampled: its own plane's two in-plane
        // directions are ±y and the perpendicular to the end tangent.
        let e2 = {
            let n = pncad::geom_core::Vec3::new(deg(22.0).sin(), 0.0, -deg(22.0).cos());
            n.normalize()
        };
        let e1 = pncad::geom_core::Vec3::new(0.0, 1.0, 0.0);
        let mut sep = f64::INFINITY;
        for ir in 0..=24 {
            let r = ARCH_R * f64::from(ir) / 24.0;
            for ip in 0..96 {
                let psi = 2.0 * PI * f64::from(ip) / 96.0;
                sep = sep.min(to_stem_wall(weld + (e1 * psi.cos() + e2 * psi.sin()) * r));
            }
        }
        assert!(
            (sep - (STEM_R - ARCH_R)).abs() < 1e-6,
            "the weld's true separation is its annular gap {}, measured {sep}",
            STEM_R - ARCH_R
        );

        let welded = pncad::topo::union(lant, arch, tol)
            .expect_err("the lantern still cannot be welded to the arch");
        println!("lily wall 2: {welded:?}");
        assert!(
            matches!(
                welded,
                BooleanError::CurvedPierceUnsupported {
                    operand: Operand::A,
                    ..
                }
            ),
            "wall 2 must stop at the crossing layer on the lantern's weld circle, \
             which lies on the arch's torus wall: {welded:?}"
        );
    }

    /// **The rootstock, measured**: the corm/foot socket is the
    /// corm's one bore wall against the foot's three arcs, all
    /// cylindrical, no planar pair, and the union glues it — one shell,
    /// valid at tier 3, its volume the two parts' sum (the interiors
    /// are disjoint) and the closed form of a sphere slab plus the
    /// foot standing out of it.
    #[test]
    fn the_rootstock_is_one_shell_at_the_parts_sum() {
        let tol = Tol::witness();
        let pieces = plant::<f64>(tol);
        let by = |name: &str| {
            &pieces
                .iter()
                .find(|p| p.name == name)
                .expect("named lily piece")
                .body
        };
        let (corm, foot) = rootstock_parts::<f64>(tol);
        let (corm, foot) = (&corm, &foot);
        let rootstock = by("lily_rootstock");
        assert_eq!(rootstock.shells().count(), 1, "one rootstock shell");
        assert_eq!(
            pncad::topo::validate_geometric(rootstock, tol),
            Ok(()),
            "the rootstock is valid at tier 3"
        );
        let (got, parts) = (
            volume(rootstock, tol),
            volume(corm, tol) + volume(foot, tol),
        );
        assert!(
            (got - parts).abs() <= 1e-12 * parts,
            "the rootstock's volume is the parts' sum: {got} vs {parts}"
        );
        // The slab of the corm's sphere between its two cut planes,
        // π[R²h − h³/3] from −base to +shoulder about the centre, plus
        // the foot's cylinder where it stands out of that slab.
        let (r, sh, ba) = (CORM_GLOBE, CORM_SHOULDER, CORM_BASE);
        let slab = PI * (r * r * (sh + ba) - (sh.powi(3) + ba.powi(3)) / 3.0);
        let stands_out = PI * STEM_R * STEM_R * (-FOOT_BOTTOM_Z - (sh + ba));
        let closed = slab + stands_out;
        assert!(
            (got - closed).abs() <= 1e-12 * closed,
            "the rootstock's volume is the slab-plus-foot closed form: {got} vs {closed}"
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
