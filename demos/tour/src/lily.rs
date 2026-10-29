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
//!   blade said as a LOFT, and they do both. `loft_body` takes the
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

use pncad::geom_brep::{CertCheck, CertifyError};
use pncad::geom_core::{
    Affine3, Mat3, Point2, Point3, RigidMotion, RigidMotionError, UnitVec3, Vec3,
};
use pncad::prelude::{Open, Start, SurfaceKind, SurfaceKindSet, query};
use pncad::profile::{ArcSweep, Center, ConstructedLoop, SketchPlane, Via};
use pncad::sweep::blend::fillet_edges;
use pncad::sweep::{
    ExtrudeError, Extrusion, Revolution, RevolveAxis, TubeWindow, WedgeFrames, extrude, loft_body,
    revolve, revolved_caps, sweep_body, tube_along_arc,
};
use pncad::topo::{
    AtRestBody, Body, BooleanBody, BooleanError, ContactRecords, EdgeDescribeFailure, EdgeKey,
    EulerOpError, Operand, PairRefusalSite,
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
pub fn neck_drop(globe: f64, top: f64, neck_r: f64, half_angle: f64) -> f64 {
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
pub const CORM_GLOBE: f64 = 0.30;
/// Depth of the corm's sphere centre below its top plane.
pub const CORM_SHOULDER: f64 = 0.22;
/// Depth of the base truncation below that centre.
pub const CORM_BASE: f64 = 0.22;
/// The stem tube's radius — the lower arc's, the bore's, and the
/// foot's: one number, because they are one stem.
pub const STEM_R: f64 = 0.060;
/// Where the foot's root end stops, below the corm.
pub const FOOT_BOTTOM_Z: f64 = -0.92;

/// A body's exact volume, read out at `f64`.
pub fn volume<S: Scalar>(b: &Body<S>, tol: Tol) -> f64 {
    pncad::topo::mass_properties(&crate::booleans::at_rest(b, tol), tol, crate::printed())
        .expect("the volume integrates")
        .enclosure
        .volume
        .f()
}

/// The rootstock's two parts as authored: the [`corm`] and the
/// [`foot`] threaded through its bore.
pub fn rootstock_parts<S: Scalar>(tol: Tol) -> (AtRestBody<S>, AtRestBody<S>) {
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
pub struct Lance {
    /// Chord length, margin to margin — the blade's width.
    pub width: f64,
    /// Rise of the upper arc above the chord.
    pub ridge: f64,
    /// Drop of the lower arc below the chord.
    pub keel: f64,
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
}

/// A swept leaf's numbers — [`leaf`]'s arguments bar `up`, which is
/// world `z` for both — named once because the scene and the Pappus
/// rows build the same blades.
#[derive(Clone, Copy, Debug)]
pub struct SweptLeaf {
    pub base: Point3<f64>,
    dir: Vec3<f64>,
    pub len: f64,
    pub section: Lance,
    /// Negative: the blade arches over, as a basal leaf does.
    pub curl: f64,
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
pub const LEAF_B: SweptLeaf = SweptLeaf {
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
pub const LEAF_C: SweptLeaf = SweptLeaf {
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
pub const LEAF_A_BASE: Point3<f64> = Point3::new(0.04, 0.05, 0.03);
/// See [`LEAF_A_BASE`].
pub const LEAF_A_DIR: Vec3<f64> = Vec3::new(-0.72, 0.52, 0.16);
/// See [`LEAF_A_BASE`].
pub const LEAF_A_UP: Vec3<f64> = Vec3::new(0.0, 0.0, 1.0);
/// Wall 7's carving ball, the one that cuts a tepal seam into the
/// lantern's zone sphere off its chart's polar axis: its centre, and
/// [`WALL7_BALL_RADIUS`]. The wall carves with it at the walls' scalar
/// and again in `f64` to draw, and a review row measures it, so it is
/// spelled once.
pub const WALL7_BALL_CENTER: Point3<f64> = Point3::new(-2.80, 0.0, 0.90);
/// See [`WALL7_BALL_CENTER`].
pub const WALL7_BALL_RADIUS: f64 = 0.16;
/// See [`LEAF_A_BASE`].
pub const LEAF_A_LEN: f64 = 5.10;
/// See [`LEAF_A_BASE`]. Negative: the blade arches OVER, which is what
/// a basal leaf lying along the ground does.
pub const LEAF_A_CURL: f64 = -0.62;

/// The long basal leaf's section plan: a narrow lens where it meets
/// the stem, a wide flat one at the belly a fifth of the way out, and
/// a small one — never a point — at the tip. The roll is 160
/// degrees eased hard toward the tip (exponent 2.6), so the blade lies
/// flat for most of its length and does its turning near the end, the
/// way a real *Calochortus* leaf does.
pub fn leaf_a_plan() -> Plan {
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
pub const LEAF_STATIONS: usize = 9;
/// Every blade's skin fit degree along the path, swept and lofted alike.
const BLADE_V_DEGREE: usize = 3;
/// Stations along the LOFTED long leaf. More than the swept blades
/// use, because a loft's stations carry the taper and the roll as well
/// as the spine: the skin interpolates the sections it is given and
/// chords between them, so a roll of half a turn wants stations dense
/// enough that no chord cuts a visible corner.
pub const LOFT_STATIONS: usize = 17;
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
pub struct Plan {
    /// The attachment section, where the blade meets the stem.
    pub base: Lance,
    /// The widest section — the blade's belly.
    pub belly: Lance,
    /// Where the belly sits, as a fraction of the spine.
    pub belly_at: f64,
    /// The tip section: a small lens, never a POINT, since a section
    /// of zero width has no chord for its arcs to stand on.
    pub tip: Lance,
    /// Roll about the spine AT THE BASE, radians. The spine's bend
    /// and the blade's facing are independent choices, and this is
    /// what separates them: `curl` decides which way the spine arcs
    /// (toward the frame's `v`), `roll0` decides which way the blade
    /// faces once it gets there. A sepal wants to arc outward from
    /// the flower and still show its face to a viewer standing to one
    /// side, which is `curl` in the radial plane and `roll0` a quarter
    /// turn off it.
    pub roll0: f64,
    /// FURTHER roll from base to tip, radians.
    pub twist: f64,
    /// Roll easing exponent: the roll at fraction `s` of the spine is
    /// `twist * s^ease`. A real *Calochortus* leaf lies flat for most
    /// of its length and does its twisting near the tip, which is
    /// `ease` above 1.
    pub twist_ease: f64,
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
pub fn lofted_blade<S: Scalar>(
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
pub fn try_lofted_blade<S: Scalar>(
    base: Point3<f64>,
    dir: Vec3<f64>,
    up: Vec3<f64>,
    len: f64,
    curl: f64,
    plan: Plan,
    stations: usize,
    tol: Tol,
) -> Result<pncad::sweep::Lofted<S>, pncad::sweep::LoftError> {
    let (d, v, u) = blade_frame(dir, up, tol);
    let r = len / curl;
    let mut sections: Vec<Vec<ConstructedLoop<f64>>> = Vec::with_capacity(stations);
    let mut places: Vec<SketchPlane<S>> = Vec::with_capacity(stations);
    for k in 0..stations {
        #[allow(clippy::cast_precision_loss)]
        let s = (k as f64) / ((stations - 1) as f64);
        let a = curl * s;
        let (sa, ca) = (a.sin(), a.cos());
        // The spine point, and the (tangent, up) pair carried round
        // with it — the arc turns in the (d, v) plane about u, so u
        // itself is fixed and the roll below is the only other motion.
        let p = base + d * (r * sa) + v * (r * (1.0 - ca));
        let vk = v * ca - d * sa;
        // The roll: turn (u, vk) about the tangent by the eased angle.
        let th = plan.roll0 + plan.twist * s.powf(plan.twist_ease);
        let (st, ct) = (th.sin(), th.cos());
        let uu = u * ct + vk * st;
        let vv = vk * ct - u * st;
        sections.push(plan.at(s).outline(tol));
        places.push(sketch_frame::<S>(
            p.map(S::from_computed),
            uu.map(S::from_computed),
            vv.map(S::from_computed),
            tol,
        ));
    }
    // Built in `f64`, so they enter `S` as the computed values they are.
    let sections: Vec<Vec<_>> = sections
        .iter()
        .map(|s| s.iter().map(|lp| lp.lift_computed()).collect())
        .collect();
    loft_body::<S>(&sections, &places, BLADE_V_DEGREE, tol)
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

pub fn deg(d: f64) -> f64 {
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
    /// wedge caps. The weld wall reads the arch's ([`wall_probes`]).
    pub caps: Option<WedgeFrames<S>>,
    /// For a piece that is a boolean RESULT (the rootstock), the
    /// contact records its op returned, which its tier-3′ gate consumes.
    pub contacts: Option<ContactRecords>,
}

/// The BUD's globe radius and truncation height — much smaller than
/// the open flower's, because a bud is mostly taper.
const BUD_GLOBE: f64 = 0.125;
/// See [`BUD_GLOBE`].
pub const BUD_TOP: f64 = 0.113;

/// The main lantern's globe radius and the height above the globe
/// centre at which the attachment disk truncates it. Named because
/// the SEPALS stand on this sphere and must stand on the same one the
/// lantern is revolved from — see [`sepals`].
pub const FLOWER_GLOBE: f64 = 0.44;
/// See [`FLOWER_GLOBE`].
pub const FLOWER_TOP: f64 = 0.40;

/// The ARCH tube's radius — and the flower's throat radius, because
/// they are one circle. The arch's terminal meridian circle and the
/// lantern's neck-cone rim are the same locus, and that identity is
/// what makes the flower/arch weld a shared-circle contact; one
/// number is how the scene says so.
pub const ARCH_R: f64 = 0.052;

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
pub const FLOWER_NECK_HALF_ANGLE: f64 = 70.0 * PI / 180.0;

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
pub const FLOWER_MOUTH_ROLL: f64 = 0.25;

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
         The long leaf runs narrow at the stem to wide to small, turning 160 \
         degrees about its own spine on the way, eased toward the tip. \
         The sepals stand TANGENT to the globe: the stand-off is the \
         section's own keel, and no point of any sepal's section rims \
         is inside the sphere. The eight analytic bodies approximate nothing — torus, \
         sphere, cylinder, cone and plane exactly, parameters included; \
         the blades are fitted skins, the price of leaving the plane. \
         Nothing above ground is JOINED — the leaf to its own sheath \
         least of all: see the wall probes.",
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
pub struct Circle {
    pub c: Point3<f64>,
    pub r: f64,
    pub n: Vec3<f64>,
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
pub fn weld_circle<S: Scalar>(
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
pub fn ball<S: Scalar>(c: Point3<f64>, r: f64, tol: Tol) -> Body<S> {
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
pub fn seamed_lantern(lantern: Body<f64>, tol: Tol) -> Result<Body<f64>, BooleanError> {
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
    //    The crossing layer passes the weld circle: the lantern's neck
    //    rim lies on the arch's torus wall as its terminal meridian
    //    circle, and the carrier ladder decides the neck cone distinct
    //    from the torus, so the circle is an edge lying on the wall.
    //    The planar pair the throat disk and the arch's cap form is
    //    decided on its zero. What refuses is the interior-loop guard,
    //    on the neck cone against the arch's torus wall: the section
    //    certificate has no arm for a cone against a torus that is not
    //    coaxial with it, so it cannot certify that the two walls meet
    //    in no loop the join did not cut.
    wall(
        2,
        "weld the lantern onto the arch (cone x the arch's end cap, \
         meeting on one shared circle)",
        pncad::topo::union(lant, arch, tol),
        |e| {
            // The pair refused is the neck cone against the arch's
            // torus wall (`review_probes::the_weld_refuses_at_the_cone_torus_pair`).
            matches!(
                e,
                BooleanError::CurvedPairUnsupported {
                    site: PairRefusalSite::InteriorLoopGuard,
                    operand: Operand::A,
                    kind: SurfaceKind::Cone,
                    other_kind: SurfaceKind::Torus,
                    ..
                }
            )
        },
        "join flower to stem — the section certificate has no arm for the \
         neck cone against the arch's torus wall",
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
    // 8. The leaf wants to GROW OUT OF the stem, not merely start
    //    beside it: a sheath of the blade's base section leaving the stem
    //    and turning into the blade's own base angle, then joined
    //    there. That contact is as clean as a contact gets — the two
    //    bodies are authored on the SAME section in the same plane,
    //    so the mate is exact rather than tolerated, and the union
    //    decides it on its zero.
    //
    //    The contact itself reads: the blade's base rim lies along the
    //    sheath's rim, a run of one circle the curve-pair reading
    //    decides, and against the sheath's other wall it is that rim
    //    edge of the sheath, meeting the wall at the lens's two tips
    //    only. The blade's NURBS seams rise from the tips, and the sweep
    //    reads each against the sheath's base cap, in the blade's own cap
    //    plane, through the ring composite of the cap's plane along the
    //    seam: no crossing inside it, its start on the cap being its end
    //    vertex's meeting. Each NURBS seam of one body is read against the
    //    other's NURBS walls by the spline-face arm, a seam's end at a tip
    //    being its end vertex's meeting. At the result, the shared rim,
    //    a seam between the two bodies' NURBS walls, reads transverse off
    //    each wall's chart. The rim is the boundary row of both walls, a
    //    curve known in advance, so it is described as their shared row
    //    and certified through each wall's row certificate. The union
    //    then stops at the interior-loop guard, which has no arm for a
    //    spline wall's pair. The flush reading along the two bodies'
    //    NURBS edges is behind that
    //    (`work/splice/a-unions-flush-reading-cannot-follow-a-nurbs-carrier`).
    let sheath = {
        let base_section = leaf_a_plan().base;
        // Runs back out of the leaf's base along -dir, so its bottom
        // cap IS the leaf's bottom cap: same plane, same section,
        // opposite outward normals.
        let sheath = lofted_blade::<S>(
            LEAF_A_BASE,
            -LEAF_A_DIR,
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
        );
        finished("the leaf's sheath", sheath, tol)
    };
    wall(
        8,
        "graft the leaf's sheath onto its blade at their shared section",
        crate::booleans::try_union(leaf_a, &sheath, tol),
        // The rim's crossing refuses at every ε that reaches it; the
        // reading that refuses is keyed by ε (measured 2026-10-29). At
        // 1e-9 the rim's certificate reads the walls' crossing over the
        // whole row, and on the rim where they are tangent at two points,
        // symmetric about its middle, the cover reads the crossing change
        // sign across each: a definite exact zero, at no sample. At 1e-6 a
        // sample's crossing reads in band first, an escalation there
        // (`work/splice/lily-wall-8-at-1e-6-reads-the-rims-crossing-angle-in-band`).
        // At 1e-12 the lower stem's declared frontier stands the lily walls
        // down before this one. If this ever refuses elsewhere, the
        // sentence above is wrong and must be re-derived before it is
        // believed.
        |e| {
            let BooleanError::EdgeUndescribed {
                failure: EdgeDescribeFailure::Intersection,
                cause: Some(cause),
                ..
            } = e
            else {
                return false;
            };
            let EulerOpError::Certification { error } = &**cause else {
                return false;
            };
            if tol.eps() == 1e-6 {
                matches!(
                    error,
                    CertifyError::Escalated {
                        check: CertCheck::Transversality,
                        sample,
                        ..
                    } if *sample != pncad::geom_brep::certify::NOT_A_SAMPLE
                )
            } else {
                matches!(
                    error,
                    CertifyError::NotTransverse {
                        sample: pncad::geom_brep::certify::NOT_A_SAMPLE,
                        lever: None,
                        verdict: pncad::geom_brep::recourse::Refused::Zero(_),
                    }
                )
            }
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
    //      tour's own gate (`gated` in `lib.rs`) holds every body to
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
