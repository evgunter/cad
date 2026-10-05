//! **`carrier_eq`** — the oriented-carrier-equality ladder,
//! kind-generalized (CONTACT-DESIGN C4's `Rest` table).
//!
//! [`super::oriented_plane_eq`] answers "same plane?" through four
//! rungs: same recipe source, declared intent, definite geometric
//! difference, else a typed undeclared-coincidence refusal. `Rest`
//! generalizes S1's planar vocabulary to every carrier kind, so the
//! ladder generalizes with it — SAME four rungs, SAME three verdicts
//! ([`CarrierRelation`]), SAME typed refusals ([`CarrierEqError`]),
//! with only the rung-3 margin list varying by kind:
//!
//! | kind | defining data | margins, each at its named lever arm |
//! |------|---------------|--------------------------------------|
//! | plane | origin, outward normal | `bool_plane_parallel` (·arm), `bool_plane_offset` |
//! | sphere | centre, radius | `carrier_sphere_center`, `carrier_sphere_radius` |
//! | cylinder | axis line, radius | `carrier_cyl_axis_parallel` (·arm), `carrier_cyl_axis_offset`, `carrier_cyl_radius` |
//! | torus | centre, axis line, both radii | `carrier_torus_axis_parallel` (·arm), `carrier_torus_center`, `carrier_torus_major_radius`, `carrier_torus_minor_radius` |
//!
//! The lever arms are the honest ones: an ANGULAR margin (two
//! normalized directions crossed) is dimensionless, so it is metered
//! at the extent over which the verdict is consumed
//! ([`ConsumedExtent`]), turning it into the displacement the
//! misalignment induces there. A LENGTH margin (a centre separation, a
//! radius difference, a point-to-axis distance) is already in metres
//! and is metered at unit arm — multiplying it by the extent would
//! price the same error twice.
//!
//! **A declared pair is read as one displacement**, not datum by
//! datum: each datum just inside the band would let the carriers stand
//! nearly twice the band apart where their errors add up. The declared
//! posture bounds the whole displacement over the consumed extent from
//! above and below ([`declared_reading`]): the upper bound bridges, a
//! lower bound past the band contradicts, and between them the
//! declaration is unsettled. The undeclared posture still reads each
//! datum on its own, since there a definite datum only says the
//! carriers differ.
//!
//! **Orientation across kinds.** The plane arm's Same± verdict is a
//! statement about MATERIAL SIDES, and so is every other arm's: a
//! sphere's or cylinder's description carries the `outward` bit
//! saying whether the face's outward normal agrees with the surface's
//! outward radial direction (S10's sense fold, one dimension curved).
//! `Rest` contact is precisely [`CarrierRelation::SameOpposite`] — a
//! peg's convex wall against a bore's concave wall is the curved
//! spelling of two boxes' opposed faces. Aligned coincidence is
//! CONTAINMENT, not contact (the C1 lemma); the carrier ladder
//! reports it honestly as `SameOriented` and the CONTACT doors
//! ([`super::contact_verify::contact_pair_verdict`]) are what refuse it.
//!
//! **Value-equality still never glues** (AQ6). Two independently
//! authored spheres with bit-equal radii reach rung 4 and refuse
//! `Undeclared`, exactly as two bit-equal planes do — the declaration
//! is what makes them one carrier, and nothing else is.

use geom_brep::recourse::Classified;
use geom_core::{Band, Decide, Decided, Indeterminate, Margin, Point3, Sign, Vec3};

use super::refusal_routes::Contradiction;
use crate::contact::ContactVerdict;
use crate::validate::{decide, decide_reported};

use super::plane_eq::{
    PlaneDesc, PlaneIdentity, PlaneRung, orientation_zero, oriented_plane_eq_verdict,
    plane_source_rung,
};

/// The relation between two oriented carriers: the three outcomes
/// every kind's ladder produces.
///
/// This is ONE type across kinds — `plane_eq` re-exports it as
/// `PlaneRelation`, the spelling its planar callers use. A parallel
/// per-kind verdict enum would let a caller handle "same carrier" for
/// planes and forget it for cylinders.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum CarrierRelation {
    /// Same carrier, same material side (the ⁺ case of Eq. 15.3 and
    /// its curved analogues) — flush walls, and the merge stage's
    /// pair.
    SameOriented,
    /// Same carrier, opposite material sides (the ⁻ case) — this, and
    /// only this, is `Rest` contact.
    SameOpposite,
    /// Definitely different carriers.
    Distinct,
}

/// Typed refusal of [`carrier_eq`]; re-exported by `plane_eq` as
/// `PlaneEqError`.
#[derive(Debug)]
pub enum CarrierEqError {
    /// A rung of the plane ladder could not decide: the only rungs that
    /// escalate here (the curved data rungs refuse `Undeclared` or
    /// `Contradicted` instead).
    Escalated {
        /// The rung that could not decide.
        rung: PlaneRung,
        /// Its diagnostics: the margin in band, or the one the rung
        /// decided at zero where zero does not pass.
        diag: Indeterminate,
    },
    /// Geometrically coincident-or-near without shared source or
    /// declared intent: an undeclared coincidence (F6). Carries the
    /// orientation the data rungs had ALREADY decided before the
    /// refusal (alignment is settled before the coincidence margins
    /// are taken), so a refusal can name the relation a declaration
    /// would assert — the refusal-menu payload (SELECT-DESIGN §3d,
    /// LIB-PYG5 R3) — without re-running any decide on the error
    /// path.
    Undeclared {
        /// What the coincidence measure read.
        coincidence: CoincidenceMeasure,
        /// The decided orientation: [`CarrierRelation::SameOriented`]
        /// or [`CarrierRelation::SameOpposite`], never `Distinct`.
        relation: CarrierRelation,
    },
    /// A declared pair whose displacement over the consumed extent may
    /// stand past the band — its upper bound does — while no point
    /// known to be consumed does: neither bridged nor contradicted
    /// ([`declared_reading`]). The diagnostics carry the upper bound.
    Unsettled {
        /// The upper bound's diagnostics.
        diag: Indeterminate,
    },
    /// A declared pair whose carriers are DEFINITELY distinct — the
    /// recipe's declaration contradicts the geometry; refused loudly,
    /// never glued.
    Contradicted {
        /// The fact that contradicted the declaration, set by the rung
        /// that decided it.
        fact: Contradiction,
        /// The deciding predicate, with an `INVALID` margin: the
        /// verdict is definite, and the rung keeps no measure.
        diag: Indeterminate,
    },
}

/// What an undeclared coincidence's measure read
/// ([`CarrierEqError::Undeclared`]).
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CoincidenceMeasure {
    /// Every datum decided zero: the pair would verify if declared.
    /// The margin is the one the band decided for the named datum
    /// (the plane's offset; a curved kind's first datum).
    Zero {
        /// The datum's predicate.
        predicate: &'static str,
        /// Its decided margin, with the band that decided it.
        decided: Classified,
    },
    /// A datum, or the declared reading of the pair, did not decide
    /// zero: in band, or past it where the declared reading stands off.
    Undecided(Indeterminate),
    /// A datum is not finite: poisoned input, which no declaration
    /// and no move of the faces reads.
    Unreadable(Indeterminate),
}

impl CoincidenceMeasure {
    /// The margin the measure read, with its band, as the payload a
    /// refusal quotes.
    #[must_use]
    pub const fn reported(self) -> Indeterminate {
        match self {
            Self::Zero {
                predicate,
                decided: Classified { margin, band },
            } => Indeterminate {
                margin,
                band,
                predicate: Some(predicate),
                terminal_sliver: false,
            },
            Self::Undecided(diag) | Self::Unreadable(diag) => diag,
        }
    }

    /// One coincidence datum decided: its sign and the margin the band
    /// decided, or the measure it leaves where it decides nothing. A
    /// datum that is not finite (NaN or ±∞) is unreadable whatever sign
    /// it reads: an infinite offset is no more a locus than a NaN one.
    pub(crate) fn decide<T: Decide>(
        name: &'static str,
        margin: Margin<T>,
        band: Band,
    ) -> Result<Decided, Self> {
        let finite = geom_core::is_finite_length(margin.value());
        match decide_reported(name, margin, band) {
            Ok(decided) if finite => Ok(decided),
            Ok(_) => Err(Self::Unreadable(Indeterminate {
                margin: geom_core::MarginDiag::INVALID,
                band,
                predicate: Some(name),
                terminal_sliver: false,
            })),
            Err(diag) => Err(Self::not_zero(diag)),
        }
    }

    /// A measure that did not decide zero, as what it is: unreadable
    /// where its margin is poisoned, undecided otherwise.
    pub(crate) fn not_zero(diag: Indeterminate) -> Self {
        if diag.margin.is_invalid() {
            Self::Unreadable(diag)
        } else {
            Self::Undecided(diag)
        }
    }
}

/// One carrier's conventional oriented description.
///
/// Every variant carries the MATERIAL side, not the chart's: the
/// plane arm folds the face sense into `normal` (S10), and the curved
/// arms carry it as `outward`. A description built from a raw chart
/// normal would make the Same± verdict a statement about nothing.
#[derive(Clone, Copy, Debug)]
pub enum CarrierDesc<T: geom_core::Real> {
    /// A plane, by a point on it and its unit outward normal.
    Plane {
        /// A point on the plane.
        origin: Point3<T>,
        /// The unit outward normal (of the face, not of the chart).
        normal: Vec3<T>,
    },
    /// A sphere, by centre and radius.
    Sphere {
        /// The centre.
        center: Point3<T>,
        /// The radius (positive).
        radius: T,
        /// Whether the face's outward normal points AWAY from the
        /// centre (a convex wall) rather than toward it (a cavity).
        outward: bool,
    },
    /// A cylinder, by a point on its axis, the unit axis direction,
    /// and the radius. The axis is a LINE: its direction sign carries
    /// no material information, so it never enters the Same± verdict.
    Cylinder {
        /// A point on the axis.
        origin: Point3<T>,
        /// The unit axis direction.
        axis: Vec3<T>,
        /// The radius (positive).
        radius: T,
        /// Whether the face's outward normal points AWAY from the
        /// axis (a shaft) rather than toward it (a bore).
        outward: bool,
    },
    /// A torus, by centre, the unit axis of revolution, and the two
    /// radii. Unlike a cylinder's, the centre is a POINT of the
    /// carrier's own data (it is where the tube's midplane meets the
    /// axis), so the two carriers' centres are compared directly and
    /// no perpendicular projection is taken. The axis is a LINE: a
    /// torus is carried onto itself by reversing it, so its direction
    /// sign holds no material information.
    Torus {
        /// The centre.
        center: Point3<T>,
        /// The unit axis of revolution.
        axis: Vec3<T>,
        /// The major radius: centre to tube centre (positive).
        major_radius: T,
        /// The minor radius: the tube radius (positive).
        minor_radius: T,
        /// Whether the face's outward normal points OUT of the tube (a
        /// pipe wall) rather than into it (a toroidal cavity).
        outward: bool,
    },
}

impl<T: geom_core::Real> CarrierDesc<T> {
    /// The kind's name, for messages and the kind-mismatch rung: the
    /// surface kind's own spelling ([`geom::SurfaceKind::name`]).
    pub fn kind(&self) -> &'static str {
        match self {
            Self::Plane { .. } => geom::SurfaceKind::Plane,
            Self::Sphere { .. } => geom::SurfaceKind::Sphere,
            Self::Cylinder { .. } => geom::SurfaceKind::Cylinder,
            Self::Torus { .. } => geom::SurfaceKind::Torus,
        }
        .name()
    }
}

/// **The region a carrier verdict is consumed on**: a ball enclosing
/// both faces ([`geom_brep::ExtentBall`]), and points known to lie on
/// each face (their boundary vertices). The ball bounds the
/// displacement between the carriers from ABOVE at every consumed
/// point; only a point known to be consumed can bound it from below,
/// so a declared verdict contradicts on the witnesses or on a reading
/// that holds across the whole ball, never on the ball's far side
/// alone.
#[derive(Clone, Copy, Debug)]
pub struct ConsumedExtent<'w, T: geom_core::Real> {
    /// A ball enclosing both faces.
    pub reach: geom_brep::ExtentBall<T>,
    /// Points on the first description's face, then on the second's.
    pub on: [&'w [Point3<T>]; 2],
}

impl<T: geom_core::Real> ConsumedExtent<'static, T> {
    /// An extent known only by a ball enclosing it: no point of either
    /// face is known, so only a reading that holds across the whole
    /// ball can contradict a declaration.
    #[must_use]
    pub fn unwitnessed(reach: geom_brep::ExtentBall<T>) -> Self {
        Self {
            reach,
            on: [&[], &[]],
        }
    }

    /// The unwitnessed ball of radius `arm` about the origin: the
    /// extent a bare arm names, for the rows that meter at one.
    #[cfg(test)]
    pub(crate) fn arm(arm: T) -> Self {
        Self::unwitnessed(geom_brep::ExtentBall::new(Point3::origin(), arm))
    }
}

/// **`carrier_eq`** — module docs for the ladder. `id` is the
/// comparison's identity evidence (recipe sources + declared intent);
/// `extent` the region the verdict is consumed on, which levers the
/// ANGULAR margins and bounds a declared pair's displacement; `band`
/// the run's linear band.
///
/// # Errors
///
/// [`CarrierEqError`] — sliver escalation, undeclared coincidence, an
/// unsettled or a contradicted declaration.
pub fn carrier_eq<T: Decide>(
    c1: &CarrierDesc<T>,
    c2: &CarrierDesc<T>,
    id: PlaneIdentity<'_>,
    extent: &ConsumedExtent<'_, T>,
    band: Band,
) -> Result<CarrierRelation, CarrierEqError> {
    carrier_eq_verdict(c1, c2, id, extent, band).map(|(rel, _)| rel)
}

/// [`carrier_eq`] plus the TRILEAN: whether the verdict stands on the
/// geometry's own definite evidence or on the declaration bridging an
/// in-band residue. One implementation, two projections — see
/// [`super::plane_eq::oriented_plane_eq_verdict`], whose contract this
/// widens to every kind.
///
/// Declared, the pair is read as ONE displacement over the consumed
/// extent ([`declared_reading`]); undeclared, each datum is read on its
/// own, the angular ones levered at the extent
/// ([`at_consumed_extent`]).
///
/// # Errors
///
/// [`CarrierEqError`] — as [`carrier_eq`].
pub fn carrier_eq_verdict<T: Decide>(
    c1: &CarrierDesc<T>,
    c2: &CarrierDesc<T>,
    id: PlaneIdentity<'_>,
    extent: &ConsumedExtent<'_, T>,
    band: Band,
) -> Result<(CarrierRelation, ContactVerdict), CarrierEqError> {
    if id.declared {
        declared_verdict(c1, c2, id, extent, band)
    } else {
        undeclared_ladder(c1, c2, id, extent, band)
    }
}

/// [`carrier_eq_verdict`] at the FACE-PAIR door, whose undeclared
/// coincidence is an offer of the declaration the declared door then
/// reads over the same extent: the coincidence stands only where the
/// declared reading would ([`coincident_as_declared`]). The corner
/// sites read their own arm, not a pair's extent, and take
/// [`carrier_eq_verdict`] as it is.
///
/// # Errors
///
/// As [`carrier_eq_verdict`].
pub(super) fn pair_door_verdict<T: Decide>(
    c1: &CarrierDesc<T>,
    c2: &CarrierDesc<T>,
    id: PlaneIdentity<'_>,
    extent: &ConsumedExtent<'_, T>,
    band: Band,
) -> Result<(CarrierRelation, ContactVerdict), CarrierEqError> {
    match carrier_eq_verdict(c1, c2, id, extent, band) {
        Err(CarrierEqError::Undeclared {
            coincidence: coincidence @ CoincidenceMeasure::Zero { .. },
            relation,
        }) => {
            coincident_as_declared(c1, c2, extent, relation, band).map_err(|diag| {
                CarrierEqError::Undeclared {
                    coincidence: CoincidenceMeasure::not_zero(diag),
                    relation,
                }
            })?;
            Err(CarrierEqError::Undeclared {
                coincidence,
                relation,
            })
        }
        verdict => verdict,
    }
}

/// [`carrier_eq_verdict`]'s undeclared posture: each datum read on its
/// own ([`at_consumed_extent`]).
fn undeclared_ladder<T: Decide>(
    c1: &CarrierDesc<T>,
    c2: &CarrierDesc<T>,
    id: PlaneIdentity<'_>,
    extent: &ConsumedExtent<'_, T>,
    band: Band,
) -> Result<(CarrierRelation, ContactVerdict), CarrierEqError> {
    let (c1, c2, arm) = at_consumed_extent(c1, c2, extent.reach);
    match (&c1, &c2) {
        (
            CarrierDesc::Plane {
                origin: o1,
                normal: n1,
            },
            CarrierDesc::Plane {
                origin: o2,
                normal: n2,
            },
        ) => oriented_plane_eq_verdict(
            &PlaneDesc {
                origin: *o1,
                normal: *n1,
            },
            &PlaneDesc {
                origin: *o2,
                normal: *n2,
            },
            id,
            extent,
            band,
        ),
        (
            CarrierDesc::Sphere {
                center: p1,
                radius: r1,
                outward: w1,
            },
            CarrierDesc::Sphere {
                center: p2,
                radius: r2,
                outward: w2,
            },
        ) => {
            if let Some(v) = source_rung(id, *w1 != *w2) {
                return Ok((v, ContactVerdict::Definite));
            }
            data_rungs(&sphere_data(*p1, *r1, *p2, *r2), *w1 == *w2, band)
        }
        (
            CarrierDesc::Cylinder {
                origin: p1,
                axis: a1,
                radius: r1,
                outward: w1,
            },
            CarrierDesc::Cylinder {
                origin: p2,
                axis: a2,
                radius: r2,
                outward: w2,
            },
        ) => {
            if let Some(v) = source_rung(id, *w1 != *w2) {
                return Ok((v, ContactVerdict::Definite));
            }
            data_rungs(
                &cylinder_data((*p1, *a1, *r1), (*p2, *a2, *r2), arm),
                *w1 == *w2,
                band,
            )
        }
        (
            CarrierDesc::Torus {
                center: p1,
                axis: a1,
                major_radius: r1,
                minor_radius: t1,
                outward: w1,
            },
            CarrierDesc::Torus {
                center: p2,
                axis: a2,
                major_radius: r2,
                minor_radius: t2,
                outward: w2,
            },
        ) => {
            if let Some(v) = source_rung(id, *w1 != *w2) {
                return Ok((v, ContactVerdict::Definite));
            }
            data_rungs(
                &torus_data((*p1, *a1, *r1, *t1), (*p2, *a2, *r2, *t2), arm),
                *w1 == *w2,
                band,
            )
        }
        // Different kinds: definitely different carriers. A plane is
        // not a cylinder at any radius, so this needs no numerics.
        _ => Ok((CarrierRelation::Distinct, ContactVerdict::Definite)),
    }
}

/// A margin the ladder reads, with the fact it contradicts.
type Datum<T> = (&'static str, Contradiction, Margin<T>);

/// The sphere's data: centre separation, radius difference.
fn sphere_data<T: Decide>(p1: Point3<T>, r1: T, p2: Point3<T>, r2: T) -> Vec<Datum<T>> {
    vec![
        (
            "carrier_sphere_center",
            Contradiction::SphereCentresDiffer,
            Margin::norm3(p1 - p2),
        ),
        (
            "carrier_sphere_radius",
            Contradiction::SphereRadiiDiffer,
            Margin::of(r1 - r2),
        ),
    ]
}

/// The cylinder's data. The axis LINE, not the axis ray: parallelism
/// is metered on the cross product (sign-free by construction) at
/// `arm`, and the offset is the perpendicular distance from the second
/// axis point to the first axis — the two data a cylinder's axis
/// actually has.
fn cylinder_data<T: Decide>(
    (p1, a1, r1): (Point3<T>, Vec3<T>, T),
    (p2, a2, r2): (Point3<T>, Vec3<T>, T),
    arm: T,
) -> Vec<Datum<T>> {
    vec![
        (
            "carrier_cyl_axis_parallel",
            Contradiction::CylinderAxesNotParallel,
            Margin::levered(a1.cross(a2).norm(), arm),
        ),
        (
            "carrier_cyl_axis_offset",
            Contradiction::CylinderAxesApart,
            Margin::norm3(perpendicular(p2 - p1, a1)),
        ),
        (
            "carrier_cyl_radius",
            Contradiction::CylinderRadiiDiffer,
            Margin::of(r1 - r2),
        ),
    ]
}

/// The torus's data. The axis LINE, as for a cylinder; the offset
/// datum, however, is NOT a cylinder's: a torus pins a point ON its
/// axis (the tube midplane's centre), so the whole centre separation is
/// the datum and projecting out its axial part would discard a real
/// difference — two coaxial tori slid along the axis are different
/// carriers.
fn torus_data<T: Decide>(
    (p1, a1, r1, t1): (Point3<T>, Vec3<T>, T, T),
    (p2, a2, r2, t2): (Point3<T>, Vec3<T>, T, T),
    arm: T,
) -> Vec<Datum<T>> {
    vec![
        (
            "carrier_torus_axis_parallel",
            Contradiction::TorusAxesNotParallel,
            Margin::levered(a1.cross(a2).norm(), arm),
        ),
        (
            "carrier_torus_center",
            Contradiction::TorusCentresDiffer,
            Margin::norm3(p1 - p2),
        ),
        (
            "carrier_torus_major_radius",
            Contradiction::TorusMajorRadiiDiffer,
            Margin::of(r1 - r2),
        ),
        (
            "carrier_torus_minor_radius",
            Contradiction::TorusTubeRadiiDiffer,
            Margin::of(t1 - t2),
        ),
    ]
}

/// `v`'s part perpendicular to the unit `axis`.
fn perpendicular<T: geom_core::Real>(v: Vec3<T>, axis: Vec3<T>) -> Vec3<T> {
    v - axis * v.dot(axis)
}

/// The angle between two axis LINES as a chord, `|a₁ − σ·a₂|` for the
/// sign σ that brings them nearest: `2·sin(θ/2)`, which bounds how far
/// a unit offset turns between them. Comparison-free (`min`).
fn line_tilt<T: geom_core::Real>(a1: Vec3<T>, a2: Vec3<T>) -> T {
    (a1 - a2).norm().min((a1 + a2).norm())
}

/// **The pair as the undeclared ladder reads it when its verdict is
/// consumed over `reach`**: each description anchored where its
/// position datum is read nearest the extent's centre, and the lever
/// arm for its angular data, the extent's farthest reach from that
/// pivot ([`geom_brep::ExtentBall`]'s module docs). Each datum is
/// decided on its own: a definite one says the carriers differ, and
/// all in band is the coincidence a declaration would have to bridge,
/// which [`declared_reading`] then reads as one sum.
///
/// - plane — the plane ladder reads the extent itself (offsets at its
///   centre, the arm its radius) and the pair passes through;
/// - cylinder — the axis offset is read at the second description's
///   axis point, which moves along its axis to the foot of the
///   extent's centre;
/// - torus — the datum is the centre: the arm is the extent's reach
///   from it.
///
/// Sphere pairs and mixed kinds carry no angular datum; they pass
/// through with the arm read from the extent's centre.
fn at_consumed_extent<T: geom_core::Real>(
    c1: &CarrierDesc<T>,
    c2: &CarrierDesc<T>,
    reach: geom_brep::ExtentBall<T>,
) -> (CarrierDesc<T>, CarrierDesc<T>, T) {
    let centre = reach.center();
    let (c1, c2, pivot) = match (*c1, *c2) {
        (
            c1,
            CarrierDesc::Cylinder {
                origin,
                axis,
                radius,
                outward,
            },
        ) if matches!(c1, CarrierDesc::Cylinder { .. }) => {
            let foot = reach.foot_on(origin, axis);
            (
                c1,
                CarrierDesc::Cylinder {
                    origin: foot,
                    axis,
                    radius,
                    outward,
                },
                foot,
            )
        }
        (c1, c2 @ CarrierDesc::Torus { center, .. }) if matches!(c1, CarrierDesc::Torus { .. }) => {
            (c1, c2, center)
        }
        (c1, c2) => (c1, c2, centre),
    };
    (c1, c2, reach.lever_from(pivot))
}

/// The declared posture: rung 1 (same source), the kind rung, then the
/// pair read as one displacement ([`declared_reading`]).
fn declared_verdict<T: Decide>(
    c1: &CarrierDesc<T>,
    c2: &CarrierDesc<T>,
    id: PlaneIdentity<'_>,
    extent: &ConsumedExtent<'_, T>,
    band: Band,
) -> Result<(CarrierRelation, ContactVerdict), CarrierEqError> {
    let same_source = match (c1, c2) {
        (
            CarrierDesc::Plane {
                origin: o1,
                normal: n1,
            },
            CarrierDesc::Plane {
                origin: o2,
                normal: n2,
            },
        ) => plane_source_rung(
            &PlaneDesc {
                origin: *o1,
                normal: *n1,
            },
            &PlaneDesc {
                origin: *o2,
                normal: *n2,
            },
            id,
        ),
        (CarrierDesc::Sphere { outward: w1, .. }, CarrierDesc::Sphere { outward: w2, .. })
        | (CarrierDesc::Cylinder { outward: w1, .. }, CarrierDesc::Cylinder { outward: w2, .. })
        | (CarrierDesc::Torus { outward: w1, .. }, CarrierDesc::Torus { outward: w2, .. }) => {
            source_rung(id, *w1 != *w2)
        }
        // Two kinds share no source; the reading refuses the pair.
        _ => None,
    };
    match same_source {
        Some(relation) => Ok((relation, ContactVerdict::Definite)),
        None => declared_reading(c1, c2, extent, band),
    }
}

/// A definite verdict's diagnostics: the rung keeps no measure.
fn definite(predicate: &'static str, band: Band) -> Indeterminate {
    Indeterminate {
        margin: geom_core::MarginDiag::INVALID,
        band,
        predicate: Some(predicate),
        terminal_sliver: false,
    }
}

/// **A declared pair, read as one displacement over the consumed
/// extent** (C4's declared rung, every kind). The displacement between
/// the two carriers at a consumed point is read as an interval:
///
/// - its UPPER bound holds at every point of the extent's ball — the
///   position data read at a pivot, plus the tilt levered from it, plus
///   the radius differences, summed. At or under the zero band the
///   geometry stands on its own ([`ContactVerdict::Definite`]); inside
///   the band the declaration bridges it ([`ContactVerdict::Bridged`]),
///   which then holds at every consumed point, not datum by datum;
/// - its LOWER bound is the larger of a reading that holds across the
///   whole ball (the radius difference less everything else) and the
///   displacement at each witness point. Only a lower bound past the
///   band is evidence that a consumed point stands off, so only it
///   contradicts, naming the datum that reads farthest
///   ([`attribution`]);
/// - an upper bound past the band beside a lower bound that is not is
///   [`CarrierEqError::Unsettled`]: the ball over-states the faces, and
///   no point known to be consumed settles the question.
///
/// Per kind (`c` the ball's centre, `R` its radius, `t` the tilt as
/// [`line_tilt`]'s chord, which bounds how far a unit offset turns):
///
/// - plane — the offset `n̂₁·(o₁ − c) − σ·n̂₂·(o₂ − c)` read at `c`,
///   plus `|n̂₁ − σ·n̂₂|·R`; σ is the decided orientation;
/// - sphere — the centre separation plus the radius difference;
/// - cylinder — the axis offset at `p` (the foot of `c` on the second
///   axis) plus `t` times the reach from `p` (widened by that offset,
///   so it holds from either axis), plus the radius difference;
/// - torus — the centre separation, the major-radius difference and
///   `t·max(R₁, R₂)` move the tube's core circle, plus the
///   minor-radius difference; this holds at every point of the torus,
///   whatever the ball.
pub(super) fn declared_reading<T: Decide>(
    c1: &CarrierDesc<T>,
    c2: &CarrierDesc<T>,
    extent: &ConsumedExtent<'_, T>,
    band: Band,
) -> Result<(CarrierRelation, ContactVerdict), CarrierEqError> {
    let witnessed = witnessed(c1, c2, extent);
    let (sigma, relation) = match (*c1, *c2) {
        (CarrierDesc::Plane { normal: n1, .. }, CarrierDesc::Plane { normal: n2, .. }) => {
            match decide_reported(
                "bool_plane_orient",
                Margin::levered(n1.dot(n2), extent.reach.radius()),
                band,
            ) {
                Ok(Decided {
                    sign: Sign::Positive,
                    ..
                }) => (T::one(), CarrierRelation::SameOriented),
                Ok(Decided {
                    sign: Sign::Negative,
                    ..
                }) => (-T::one(), CarrierRelation::SameOpposite),
                // The orientation reads nothing where the planes stand
                // near square; a witness standing off says why.
                refused => {
                    if matches!(
                        decide("bool_plane_reach_floor", Margin::of(witnessed), band),
                        Ok(Sign::Positive)
                    ) {
                        return Err(CarrierEqError::Contradicted {
                            fact: Contradiction::PlanesNotParallel,
                            diag: definite("bool_plane_parallel", band),
                        });
                    }
                    return Err(match refused {
                        Ok(Decided { margin, .. }) => orientation_zero(margin, band),
                        Err(diag) => CarrierEqError::Escalated {
                            rung: PlaneRung::Orientation,
                            diag,
                        },
                    });
                }
            }
        }
        (CarrierDesc::Sphere { outward: w1, .. }, CarrierDesc::Sphere { outward: w2, .. })
        | (CarrierDesc::Cylinder { outward: w1, .. }, CarrierDesc::Cylinder { outward: w2, .. })
        | (CarrierDesc::Torus { outward: w1, .. }, CarrierDesc::Torus { outward: w2, .. }) => (
            T::one(),
            if w1 == w2 {
                CarrierRelation::SameOriented
            } else {
                CarrierRelation::SameOpposite
            },
        ),
        // Two kinds: no orientation to read, and no reading below.
        _ => (T::one(), CarrierRelation::Distinct),
    };
    // A declaration that a plane is a cylinder is contradicted by the
    // structural fact that no radius makes it one.
    let Some(Reading {
        names: [upper_name, floor_name],
        upper,
        across,
        data,
    }) = reading(c1, c2, extent.reach, sigma)
    else {
        return Err(CarrierEqError::Contradicted {
            fact: Contradiction::KindsDiffer,
            diag: definite("carrier_kind", band),
        });
    };
    // A witness is a consumed point, so the displacement there bounds
    // the upper bound from below whatever ball the caller passed: a
    // ball that does not enclose a witness never bridges past it.
    let floor = across.max(witnessed).max(T::zero());
    let upper = upper.max(floor);
    let unsettled = |margin| CarrierEqError::Unsettled {
        diag: Indeterminate {
            margin,
            band,
            predicate: Some(upper_name),
            terminal_sliver: false,
        },
    };
    match decide_reported(upper_name, Margin::of(upper), band) {
        Ok(Decided {
            sign: Sign::Zero, ..
        }) => Ok((relation, ContactVerdict::Definite)),
        Err(diag) if !diag.margin.is_invalid() => Ok((relation, ContactVerdict::Bridged)),
        // A poisoned reading bridges nothing.
        Err(diag) => Err(CarrierEqError::Unsettled { diag }),
        // A sum of magnitudes reads negative only on broken input.
        Ok(Decided {
            sign: Sign::Negative,
            margin,
        }) => Err(unsettled(margin)),
        Ok(Decided {
            sign: Sign::Positive,
            margin,
        }) => match decide(floor_name, Margin::of(floor), band) {
            Ok(Sign::Positive) => {
                let (name, fact) = attribution(&data, band);
                Err(CarrierEqError::Contradicted {
                    fact,
                    diag: definite(name, band),
                })
            }
            _ => Err(unsettled(margin)),
        },
    }
}

/// **The undeclared posture's affirmative, read as the declared rung
/// would read it.** Rung 4's coincidence (every datum decided zero)
/// says "a declaration would verify"; the declared rung reads the data's
/// SUM, so the coincidence stands only where that sum decides zero too
/// — a declared pair the detector called coincident then verifies
/// [`ContactVerdict::Definite`], whatever K. Where the sum does not,
/// the refusal carries its reading, in band or past it, as the
/// coincidence the detector cannot call.
fn coincident_as_declared<T: Decide>(
    c1: &CarrierDesc<T>,
    c2: &CarrierDesc<T>,
    extent: &ConsumedExtent<'_, T>,
    relation: CarrierRelation,
    band: Band,
) -> Result<(), Indeterminate> {
    let sigma = match relation {
        CarrierRelation::SameOpposite => -T::one(),
        CarrierRelation::SameOriented | CarrierRelation::Distinct => T::one(),
    };
    let Some(Reading {
        names: [upper_name, _],
        upper,
        ..
    }) = reading(c1, c2, extent.reach, sigma)
    else {
        return Ok(());
    };
    match decide_reported(upper_name, Margin::of(upper), band) {
        Ok(Decided {
            sign: Sign::Zero, ..
        }) => Ok(()),
        Ok(Decided { margin, .. }) => Err(Indeterminate {
            margin,
            band,
            predicate: Some(upper_name),
            terminal_sliver: false,
        }),
        Err(diag) => Err(diag),
    }
}

/// The displacement at each witness: its distance from the other
/// carrier, less its own (a vertex stands within rounding of its own
/// carrier, not on it). Zero where no point is known.
fn witnessed<T: Decide>(
    c1: &CarrierDesc<T>,
    c2: &CarrierDesc<T>,
    extent: &ConsumedExtent<'_, T>,
) -> T {
    extent.on[0]
        .iter()
        .map(|&v| distance_to(c2, v) - distance_to(c1, v))
        .chain(
            extent.on[1]
                .iter()
                .map(|&v| distance_to(c1, v) - distance_to(c2, v)),
        )
        .fold(T::zero(), T::max)
}

/// A pair's displacement over the consumed extent ([`declared_reading`]).
struct Reading<T: Decide> {
    /// The upper and the lower bound's predicate names.
    names: [&'static str; 2],
    /// The upper bound, at every point of the ball.
    upper: T,
    /// The lower bound that holds across the whole ball (may read
    /// negative: no bound).
    across: T,
    /// The kind's data, for [`attribution`].
    data: Vec<Datum<T>>,
}

/// The [`Reading`] of a same-kind pair over `reach`, the planes'
/// orientation sign `sigma` decided; `None` for a kind mismatch.
fn reading<T: Decide>(
    c1: &CarrierDesc<T>,
    c2: &CarrierDesc<T>,
    reach: geom_brep::ExtentBall<T>,
    sigma: T,
) -> Option<Reading<T>> {
    let (names, upper, across, data) = match (*c1, *c2) {
        (
            CarrierDesc::Plane {
                origin: o1,
                normal: n1,
            },
            CarrierDesc::Plane {
                origin: o2,
                normal: n2,
            },
        ) => {
            let (centre, radius) = (reach.center(), reach.radius());
            let offset = n1.dot(o1 - centre) - sigma * n2.dot(o2 - centre);
            let swing = Margin::levered((n1 - n2 * sigma).norm(), radius).value();
            (
                ["bool_plane_reach", "bool_plane_reach_floor"],
                offset.abs() + swing,
                offset.abs() - swing,
                vec![
                    (
                        "bool_plane_parallel",
                        Contradiction::PlanesNotParallel,
                        Margin::levered(n1.cross(n2).norm(), radius),
                    ),
                    (
                        "bool_plane_offset",
                        Contradiction::PlanesApart,
                        Margin::of(offset),
                    ),
                ],
            )
        }
        (
            CarrierDesc::Sphere {
                center: p1,
                radius: r1,
                ..
            },
            CarrierDesc::Sphere {
                center: p2,
                radius: r2,
                ..
            },
        ) => {
            let apart = (p1 - p2).norm();
            (
                ["carrier_sphere_reach", "carrier_sphere_reach_floor"],
                (r1 - r2).abs() + apart,
                (r1 - r2).abs() - apart,
                sphere_data(p1, r1, p2, r2),
            )
        }
        (
            CarrierDesc::Cylinder {
                origin: o1,
                axis: a1,
                radius: r1,
                ..
            },
            CarrierDesc::Cylinder {
                origin: o2,
                axis: a2,
                radius: r2,
                ..
            },
        ) => {
            let pivot = reach.foot_on(o2, a2);
            let apart = perpendicular(pivot - o1, a1).norm();
            let arm = reach.lever_from(pivot) + apart;
            let core = apart + Margin::levered(line_tilt(a1, a2), arm).value();
            (
                ["carrier_cyl_reach", "carrier_cyl_reach_floor"],
                (r1 - r2).abs() + core,
                (r1 - r2).abs() - core,
                cylinder_data((o1, a1, r1), (pivot, a2, r2), arm),
            )
        }
        (
            CarrierDesc::Torus {
                center: p1,
                axis: a1,
                major_radius: r1,
                minor_radius: t1,
                ..
            },
            CarrierDesc::Torus {
                center: p2,
                axis: a2,
                major_radius: r2,
                minor_radius: t2,
                ..
            },
        ) => {
            let arm = r1.max(r2);
            let core = (p1 - p2).norm()
                + (r1 - r2).abs()
                + Margin::levered(line_tilt(a1, a2), arm).value();
            (
                ["carrier_torus_reach", "carrier_torus_reach_floor"],
                (t1 - t2).abs() + core,
                (t1 - t2).abs() - core,
                torus_data((p1, a1, r1, t1), (p2, a2, r2, t2), arm),
            )
        }
        _ => return None,
    };
    Some(Reading {
        names,
        upper,
        across,
        data,
    })
}

/// The datum a contradicted declaration is named by: the first that
/// reads definitely nonzero, else the first in band, else the first.
/// It decides nothing — the floor already did — and names the
/// counter-evidence for the refusal and its steer.
fn attribution<T: Decide>(data: &[Datum<T>], band: Band) -> (&'static str, Contradiction) {
    let reads: Vec<_> = data
        .iter()
        .map(|&(name, fact, margin)| (name, fact, decide(name, margin, band)))
        .collect();
    reads
        .iter()
        .find(|(.., read)| matches!(read, Ok(Sign::Positive | Sign::Negative)))
        .or_else(|| reads.iter().find(|(.., read)| read.is_err()))
        .or(reads.first())
        .map_or(
            ("carrier_kind", Contradiction::KindsDiffer),
            |&(name, fact, _)| (name, fact),
        )
}

/// The distance from `v` to the carrier `c`.
fn distance_to<T: geom_core::Real>(c: &CarrierDesc<T>, v: Point3<T>) -> T {
    match *c {
        CarrierDesc::Plane { origin, normal } => normal.dot(v - origin).abs(),
        CarrierDesc::Sphere { center, radius, .. } => ((v - center).norm() - radius).abs(),
        CarrierDesc::Cylinder {
            origin,
            axis,
            radius,
            ..
        } => (perpendicular(v - origin, axis).norm() - radius).abs(),
        CarrierDesc::Torus {
            center,
            axis,
            major_radius,
            minor_radius,
            ..
        } => {
            let d = v - center;
            let height = d.dot(axis);
            let ring = perpendicular(d, axis).norm() - major_radius;
            ((ring.powi(2) + height.powi(2)).sqrt() - minor_radius).abs()
        }
    }
}

/// Rung 1 for the curved arms: both descriptions carry the same
/// recipe source ⇒ same carrier by the N6 theorem, with the material
/// side read off the descriptions' own `outward` bits.
///
/// **Not [`crate::source::source_declaration`]'s ladder**, whose
/// `orient` (with the face's sense composed in) is the plane rung's
/// material side: a curved description cannot be reversed, so `revert`
/// records a curved face's reversal on its `sense` AND its source's
/// `orient`, and the composition cancels — a face against its reverted
/// twin reads `SameSource` there and opposed here.
///
/// The plane arm's version additionally debug-asserts that the bits
/// agree; the curved arms have no canonicalized bit form to assert
/// against, and inventing one would be a second source of truth.
fn source_rung(id: PlaneIdentity<'_>, opposed: bool) -> Option<CarrierRelation> {
    let (s1, s2) = (id.s1?, id.s2?);
    s1.same_base(s2).then_some(if opposed {
        CarrierRelation::SameOpposite
    } else {
        CarrierRelation::SameOriented
    })
}

/// Rungs 3–4 for the curved arms in the undeclared posture, driven by
/// the kind's margin list: a definitely-nonzero margin means
/// `Distinct`, and an all-zero-or-in-band list is the typed
/// `Undeclared` refusal (value equality never glues).
fn data_rungs<T: Decide>(
    margins: &[Datum<T>],
    aligned: bool,
    band: Band,
) -> Result<(CarrierRelation, ContactVerdict), CarrierEqError> {
    let same = if aligned {
        CarrierRelation::SameOriented
    } else {
        CarrierRelation::SameOpposite
    };
    let mut first_zero: Option<CoincidenceMeasure> = None;
    let mut first_unread: Option<CoincidenceMeasure> = None;
    let mut first_in_band: Option<CoincidenceMeasure> = None;
    for &(name, _, margin) in margins {
        match CoincidenceMeasure::decide(name, margin, band) {
            Ok(Decided {
                sign: Sign::Positive | Sign::Negative,
                ..
            }) => {
                return Ok((CarrierRelation::Distinct, ContactVerdict::Definite));
            }
            Ok(Decided {
                sign: Sign::Zero,
                margin,
            }) => {
                first_zero = first_zero.or(Some(CoincidenceMeasure::Zero {
                    predicate: name,
                    decided: Classified { margin, band },
                }));
            }
            Err(unread @ CoincidenceMeasure::Unreadable(_)) => {
                first_unread = first_unread.or(Some(unread));
            }
            Err(in_band) => first_in_band = first_in_band.or(Some(in_band)),
        }
    }
    // Rung 4: coincident-or-near with no identity rung — near
    // coincidence NEVER silently becomes contact, and bit-equal data
    // without a shared source stays unglued. A datum that cannot be
    // read is reported first, then the first one in band; when every
    // datum decided zero, the first one's decided margin rides.
    let coincidence = match first_unread.or(first_in_band).or(first_zero) {
        Some(coincidence) => coincidence,
        None => unreachable!(
            "every curved kind reads at least two data, and each datum decides zero, decides \
             nonzero (returned above) or does not decide"
        ),
    };
    Err(CarrierEqError::Undeclared {
        coincidence,
        // The alignment this traversal was run under: the relation a
        // declaration of this pair would verify with (R3).
        relation: same,
    })
}

/// Points on `c`'s carrier around its datum, the witnesses a face of
/// it would offer: the points a unit or a radius off its origin
/// along two directions square to its normal or axis.
#[cfg(test)]
pub(crate) fn points_on(c: &CarrierDesc<f64>) -> Vec<Point3<f64>> {
    let square = |n: Vec3<f64>| {
        let seed = if n.x.abs() < 0.9 {
            Vec3::new(1.0, 0.0, 0.0)
        } else {
            Vec3::new(0.0, 1.0, 0.0)
        };
        let u = n.cross(seed).normalize();
        [u, -u, n.cross(u), -n.cross(u)]
    };
    match *c {
        CarrierDesc::Plane { origin, normal } => square(normal).map(|d| origin + d).to_vec(),
        CarrierDesc::Sphere { center, radius, .. } => square(Vec3::new(0.0, 0.0, 1.0))
            .into_iter()
            .chain([Vec3::new(0.0, 0.0, 1.0), Vec3::new(0.0, 0.0, -1.0)])
            .map(|d| center + d * radius)
            .collect(),
        CarrierDesc::Cylinder {
            origin,
            axis,
            radius,
            ..
        } => square(axis)
            .into_iter()
            .flat_map(|d| [0.0, 1.0].map(|h| origin + d * radius + axis * h))
            .collect(),
        CarrierDesc::Torus {
            center,
            axis,
            major_radius,
            minor_radius,
            ..
        } => square(axis)
            .into_iter()
            .flat_map(|d| {
                [
                    center + d * (major_radius + minor_radius),
                    center + d * major_radius + axis * minor_radius,
                ]
            })
            .collect(),
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod tests {
    use super::*;
    use geom_core::Tol;

    fn band() -> Band {
        Band::linear(Tol::witness()).unwrap()
    }

    fn sphere(c: [f64; 3], r: f64, outward: bool) -> CarrierDesc<f64> {
        CarrierDesc::Sphere {
            center: Point3::from_array(c),
            radius: r,
            outward,
        }
    }

    fn cyl(o: [f64; 3], a: [f64; 3], r: f64, outward: bool) -> CarrierDesc<f64> {
        CarrierDesc::Cylinder {
            origin: Point3::from_array(o),
            axis: Vec3::from_array(a),
            radius: r,
            outward,
        }
    }

    fn torus(c: [f64; 3], a: [f64; 3], major: f64, minor: f64, outward: bool) -> CarrierDesc<f64> {
        CarrierDesc::Torus {
            center: Point3::from_array(c),
            axis: Vec3::from_array(a),
            major_radius: major,
            minor_radius: minor,
            outward,
        }
    }

    /// A ball of radius `arm` about the origin, no point of either face
    /// known.
    fn at(arm: f64) -> ConsumedExtent<'static, f64> {
        ConsumedExtent::unwitnessed(geom_brep::ExtentBall::new(Point3::origin(), arm))
    }

    /// The same ball, with points known on each face.
    fn witnessed<'w>(arm: f64, on: [&'w [Point3<f64>]; 2]) -> ConsumedExtent<'w, f64> {
        ConsumedExtent { on, ..at(arm) }
    }

    fn declared() -> PlaneIdentity<'static> {
        PlaneIdentity {
            s1: None,
            s2: None,
            declared: true,
        }
    }

    /// **The declared sum reads at the face-pair door only.** Two
    /// spheres whose centres stand `0.6·ε` apart and whose radii differ
    /// by `0.6·ε`: each datum decides zero, their sum (`1.2·ε`) does not.
    /// The ladder as the corner sites read it calls the pair coincident
    /// (a margin decided at zero); the face-pair door, whose coincidence
    /// offers the declaration the declared door reads over the same
    /// extent, refuses it with the sum's in-band reading instead.
    #[test]
    fn the_declared_sum_reads_at_the_pair_door_and_not_at_the_corners() {
        let e = band().zero();
        let a = sphere([0.0, 0.0, 0.0], 2.0, true);
        let b = sphere([0.6 * e, 0.0, 0.0], 2.0 + 0.6 * e, true);
        match carrier_eq_verdict(&a, &b, PlaneIdentity::NONE, &at(1.0), band()) {
            Err(CarrierEqError::Undeclared {
                coincidence: CoincidenceMeasure::Zero { predicate, .. },
                ..
            }) => {
                assert_eq!(predicate, "carrier_sphere_center", "every datum zero");
            }
            other => panic!("the corner sites' ladder: {other:?}"),
        }
        match pair_door_verdict(&a, &b, PlaneIdentity::NONE, &at(1.0), band()) {
            Err(CarrierEqError::Undeclared {
                coincidence: CoincidenceMeasure::Undecided(diag),
                ..
            }) => {
                assert!(
                    !diag.margin.is_invalid() && diag.predicate == Some("carrier_sphere_reach"),
                    "the sum in band: {diag:?}"
                );
            }
            other => panic!("the pair door: {other:?}"),
        }
    }

    /// **A witness bounds the reading from below whatever ball it is
    /// read over.** Two planes through the origin, one tilted `90·Kε`
    /// per metre about the x-axis, read over a 1 cm ball about the
    /// origin: the tilt there reads under the band. A point on the flat
    /// face 10 m off the hinge stands `900·Kε` from the tilted plane, past
    /// the ball, so the ball does not enclose the faces; the witness
    /// still contradicts the declaration rather than the ball bridging
    /// past it.
    #[test]
    fn a_witness_past_a_ball_that_misses_it_contradicts() {
        let theta = 90.0 * band().escalate();
        let flat = CarrierDesc::Plane {
            origin: Point3::origin(),
            normal: Vec3::new(0.0, 0.0, 1.0),
        };
        let tilted = CarrierDesc::Plane {
            origin: Point3::origin(),
            normal: Vec3::new(0.0, -theta, 1.0).normalize(),
        };
        assert!(matches!(
            declared_reading(&flat, &tilted, &at(0.01), band()),
            Ok((_, ContactVerdict::Definite | ContactVerdict::Bridged))
        ));
        let on = [Point3::new(10.0, -10.0, 0.0)];
        assert!(matches!(
            declared_reading(&flat, &tilted, &witnessed(0.01, [&on, &[]]), band()),
            Err(CarrierEqError::Contradicted { .. })
        ));
    }

    /// **A curved datum that is not finite is unreadable, and is
    /// reported ahead of a datum in band.** Two spheres whose centres
    /// stand apart inside the ambiguity band, the second's radius NaN
    /// or `+∞`: the radius decides no sign, so the pair is neither
    /// apart nor in band, and the refusal names the radius.
    #[test]
    fn an_unreadable_curved_datum_is_reported_ahead_of_one_in_band() {
        let b = band();
        let apart = (b.zero() + b.escalate()) / 2.0;
        let a = sphere([0.0, 0.0, 0.0], 2.0, true);
        for radius in [f64::NAN, f64::INFINITY] {
            let poisoned = sphere([apart, 0.0, 0.0], radius, true);
            match carrier_eq_verdict(&a, &poisoned, PlaneIdentity::NONE, &at(1.0), b) {
                Err(CarrierEqError::Undeclared {
                    coincidence: CoincidenceMeasure::Unreadable(diag),
                    ..
                }) => {
                    assert!(diag.margin.is_invalid(), "{radius}: {diag:?}");
                    assert_eq!(diag.predicate, Some("carrier_sphere_radius"), "{radius}");
                }
                other => panic!("a radius of {radius} is unreadable: {other:?}"),
            }
        }
    }

    /// The peg-in-bore row: value-equal radii, opposed material
    /// sides. UNDECLARED it refuses (value equality never glues);
    /// DECLARED it is the `Rest` verdict.
    #[test]
    fn sphere_value_equal_needs_the_declaration() {
        let a = sphere([0.0, 0.0, 0.0], 2.0, true);
        let b = sphere([0.0, 0.0, 0.0], 2.0, false);
        assert!(matches!(
            carrier_eq(&a, &b, PlaneIdentity::NONE, &at(1.0), band()),
            Err(CarrierEqError::Undeclared { .. })
        ));
        assert_eq!(
            carrier_eq(&a, &b, declared(), &at(1.0), band()).unwrap(),
            CarrierRelation::SameOpposite
        );
    }

    /// Aligned coincidence is reported honestly as `SameOriented` —
    /// the carrier ladder's job is the carrier; refusing containment
    /// is the contact door's job (module docs, C1 lemma).
    #[test]
    fn sphere_aligned_is_same_oriented_not_a_carrier_error() {
        let a = sphere([1.0, 0.0, 0.0], 2.0, true);
        let b = sphere([1.0, 0.0, 0.0], 2.0, true);
        assert_eq!(
            carrier_eq(&a, &b, declared(), &at(1.0), band()).unwrap(),
            CarrierRelation::SameOriented
        );
    }

    /// Every definite verdict wins over every declaration: a
    /// definitely different radius contradicts, naming the margin
    /// that decided.
    #[test]
    fn definite_radius_difference_contradicts_the_declaration() {
        let a = sphere([0.0, 0.0, 0.0], 2.0, true);
        let b = sphere([0.0, 0.0, 0.0], 2.5, false);
        let err = carrier_eq(&a, &b, declared(), &at(1.0), band()).unwrap_err();
        match err {
            CarrierEqError::Contradicted { diag: d, .. } => {
                assert_eq!(d.predicate, Some("carrier_sphere_radius"));
            }
            other => panic!("expected Contradicted, got {other:?}"),
        }
        // Undeclared, the same pair is simply two different carriers.
        assert_eq!(
            carrier_eq(&a, &b, PlaneIdentity::NONE, &at(1.0), band()).unwrap(),
            CarrierRelation::Distinct
        );
    }

    /// ε-row, three outcomes at one geometry: a sub-band radius
    /// difference. UNDECLARED it refuses typed (in-band is never a
    /// silent pass); DECLARED it is the bridged residue and stands;
    /// a definite difference at the same site contradicts.
    #[test]
    fn sphere_radius_epsilon_row_three_outcomes() {
        let a = sphere([0.0, 0.0, 0.0], 2.0, true);
        let in_band = sphere([0.0, 0.0, 0.0], 2.0 + 1e-12, false);
        assert!(
            matches!(
                carrier_eq(&a, &in_band, PlaneIdentity::NONE, &at(1.0), band()),
                Err(CarrierEqError::Undeclared { .. })
            ),
            "in-band, undeclared: refuses"
        );
        assert_eq!(
            carrier_eq(&a, &in_band, declared(), &at(1.0), band()).unwrap(),
            CarrierRelation::SameOpposite,
            "in-band, declared: the bridged residue"
        );
        let definite = sphere([0.0, 0.0, 0.0], 2.001, false);
        assert!(
            matches!(
                carrier_eq(&a, &definite, declared(), &at(1.0), band()),
                Err(CarrierEqError::Contradicted { .. })
            ),
            "definite, declared: contradicted"
        );
    }

    /// A cylinder's axis is a LINE: reversing the stored direction is
    /// the same carrier, and the material side comes from `outward`
    /// alone.
    #[test]
    fn cylinder_axis_direction_sign_is_not_material() {
        let a = cyl([0.0, 0.0, 0.0], [0.0, 0.0, 1.0], 3.0, true);
        let b = cyl([0.0, 0.0, 5.0], [0.0, 0.0, -1.0], 3.0, false);
        assert_eq!(
            carrier_eq(&a, &b, declared(), &at(1.0), band()).unwrap(),
            CarrierRelation::SameOpposite
        );
    }

    /// A parallel-but-offset axis is a definitely different cylinder;
    /// declaring it contradicts at the axis-offset margin.
    #[test]
    fn cylinder_offset_axis_is_distinct() {
        let a = cyl([0.0, 0.0, 0.0], [0.0, 0.0, 1.0], 3.0, true);
        let b = cyl([0.5, 0.0, 0.0], [0.0, 0.0, 1.0], 3.0, false);
        assert_eq!(
            carrier_eq(&a, &b, PlaneIdentity::NONE, &at(1.0), band()).unwrap(),
            CarrierRelation::Distinct
        );
        let on = points_on(&a);
        match carrier_eq(&a, &b, declared(), &witnessed(1.0, [&on, &[]]), band()).unwrap_err() {
            CarrierEqError::Contradicted { diag: d, .. } => {
                assert_eq!(d.predicate, Some("carrier_cyl_axis_offset"));
            }
            other => panic!("expected Contradicted, got {other:?}"),
        }
    }

    /// ε-row on the cylinder's own margins, three outcomes at one
    /// geometry — the row the sphere already had, owed to every new
    /// margin. The radius datum carries it: sub-band, the declaration
    /// bridges and the undeclared pair refuses; definite, it
    /// contradicts.
    #[test]
    fn cylinder_radius_epsilon_row_three_outcomes() {
        // Derived from the RUN's band, never a literal: the same
        // number is a gap at one ε and nothing at another, and a row
        // that only means what it says at one ε is not an ε row.
        let b = band();
        let a = cyl([0.0, 0.0, 0.0], [0.0, 0.0, 1.0], 3.0, true);
        let in_band = cyl(
            [0.0, 0.0, 0.0],
            [0.0, 0.0, 1.0],
            3.0 + (b.zero() + b.escalate()) * 0.5,
            false,
        );
        assert!(
            matches!(
                carrier_eq(&a, &in_band, PlaneIdentity::NONE, &at(1.0), band()),
                Err(CarrierEqError::Undeclared { .. })
            ),
            "in-band, undeclared: refuses"
        );
        assert_eq!(
            carrier_eq(&a, &in_band, declared(), &at(1.0), band()).unwrap(),
            CarrierRelation::SameOpposite,
            "in-band, declared: the bridged residue"
        );
        let definite = cyl(
            [0.0, 0.0, 0.0],
            [0.0, 0.0, 1.0],
            3.0 + b.escalate() * 1000.0,
            false,
        );
        match carrier_eq(&a, &definite, declared(), &at(1.0), band()).unwrap_err() {
            CarrierEqError::Contradicted { diag: d, .. } => {
                assert_eq!(d.predicate, Some("carrier_cyl_radius"));
            }
            other => panic!("expected Contradicted, got {other:?}"),
        }
    }

    /// The cylinder's ANGULAR margin is metered at its named lever
    /// arm, which is the arm the caller passes: a tilt that is
    /// indecisive over a 1 m consumption extent is definite over a
    /// 1000 km one. Same geometry, two arms, two honest answers.
    #[test]
    fn cylinder_axis_tilt_is_decided_at_the_lever_arm() {
        let b = band();
        let a = cyl([0.0, 0.0, 0.0], [0.0, 0.0, 1.0], 3.0, true);
        // A tilt whose displacement is sub-band over 1 m and decisive
        // over 1000 km — stated in band units so the row survives
        // every ε the matrix runs.
        let tilt = b.zero() * 0.1;
        let tilted = cyl([0.0, 0.0, 0.0], [tilt, 0.0, 1.0], 3.0, false);
        assert_eq!(
            carrier_eq(&a, &tilted, declared(), &at(1.0), band()).unwrap(),
            CarrierRelation::SameOpposite,
            "at a 1 m arm the tilt is below the band: the declaration stands"
        );
        // A point of the face half the arm along the axis, where the
        // tilt has carried the other carrier far past the band.
        let on = [Point3::new(3.0, 0.0, 5e5)];
        match carrier_eq(&a, &tilted, declared(), &witnessed(1e6, [&on, &[]]), band()).unwrap_err()
        {
            CarrierEqError::Contradicted { diag: d, .. } => {
                assert_eq!(d.predicate, Some("carrier_cyl_axis_parallel"));
            }
            other => panic!("expected Contradicted at the long arm, got {other:?}"),
        }
    }

    /// The toroidal peg-in-bore row: one carrier, opposed material
    /// sides. UNDECLARED it refuses (value equality never glues);
    /// DECLARED it is the `Rest` verdict — the torus arm's version of
    /// the sphere and cylinder rows above.
    #[test]
    fn torus_value_equal_needs_the_declaration() {
        let a = torus([0.0, 0.0, 0.0], [0.0, 0.0, 1.0], 5.0, 0.06, true);
        let b = torus([0.0, 0.0, 0.0], [0.0, 0.0, 1.0], 5.0, 0.06, false);
        assert!(matches!(
            carrier_eq(&a, &b, PlaneIdentity::NONE, &at(1.0), band()),
            Err(CarrierEqError::Undeclared { .. })
        ));
        assert_eq!(
            carrier_eq(&a, &b, declared(), &at(1.0), band()).unwrap(),
            CarrierRelation::SameOpposite
        );
    }

    /// A torus's axis is a LINE: reversing the stored direction leaves
    /// the same point set, and the material side comes from `outward`
    /// alone.
    #[test]
    fn torus_axis_direction_sign_is_not_material() {
        let a = torus([1.0, 2.0, 3.0], [0.0, 0.0, 1.0], 5.0, 0.06, true);
        let b = torus([1.0, 2.0, 3.0], [0.0, 0.0, -1.0], 5.0, 0.06, false);
        assert_eq!(
            carrier_eq(&a, &b, declared(), &at(1.0), band()).unwrap(),
            CarrierRelation::SameOpposite
        );
    }

    /// **The centre is a POINT, not an axis anchor.** Sliding a torus
    /// ALONG its own axis moves the carrier — the datum a cylinder's
    /// perpendicular-projection offset would have thrown away, which
    /// is why the torus arm compares the whole centre separation.
    #[test]
    fn torus_slid_along_its_own_axis_is_a_different_carrier() {
        let a = torus([0.0, 0.0, 0.0], [0.0, 0.0, 1.0], 5.0, 0.06, true);
        let b = torus([0.0, 0.0, 0.5], [0.0, 0.0, 1.0], 5.0, 0.06, false);
        assert_eq!(
            carrier_eq(&a, &b, PlaneIdentity::NONE, &at(1.0), band()).unwrap(),
            CarrierRelation::Distinct
        );
        let on = points_on(&a);
        match carrier_eq(&a, &b, declared(), &witnessed(1.0, [&on, &[]]), band()).unwrap_err() {
            CarrierEqError::Contradicted { diag: d, .. } => {
                assert_eq!(d.predicate, Some("carrier_torus_center"));
            }
            other => panic!("expected Contradicted, got {other:?}"),
        }
    }

    /// The two radii are INDEPENDENT data, and each contradicts on its
    /// own margin: a bore of the right ring but the wrong tube is not
    /// the same carrier, and neither is the reverse.
    #[test]
    fn each_torus_radius_contradicts_at_its_own_margin() {
        let a = torus([0.0, 0.0, 0.0], [0.0, 0.0, 1.0], 5.0, 0.06, true);
        for (b, expected) in [
            (
                torus([0.0, 0.0, 0.0], [0.0, 0.0, 1.0], 5.5, 0.06, false),
                "carrier_torus_major_radius",
            ),
            (
                torus([0.0, 0.0, 0.0], [0.0, 0.0, 1.0], 5.0, 0.09, false),
                "carrier_torus_minor_radius",
            ),
        ] {
            let on = points_on(&a);
            match carrier_eq(&a, &b, declared(), &witnessed(1.0, [&on, &[]]), band()).unwrap_err() {
                CarrierEqError::Contradicted { diag: d, .. } => {
                    assert_eq!(d.predicate, Some(expected))
                }
                other => panic!("expected Contradicted at {expected}, got {other:?}"),
            }
            assert_eq!(
                carrier_eq(&a, &b, PlaneIdentity::NONE, &at(1.0), band()).unwrap(),
                CarrierRelation::Distinct
            );
        }
    }

    /// ε-row on the torus's own margins, three outcomes at one
    /// geometry — owed to every new margin, and stated in BAND UNITS
    /// so it means the same thing at every ε the matrix runs. The
    /// minor radius carries it.
    #[test]
    fn torus_minor_radius_epsilon_row_three_outcomes() {
        let b = band();
        let a = torus([0.0, 0.0, 0.0], [0.0, 0.0, 1.0], 5.0, 0.06, true);
        let in_band = torus(
            [0.0, 0.0, 0.0],
            [0.0, 0.0, 1.0],
            5.0,
            0.06 + (b.zero() + b.escalate()) * 0.5,
            false,
        );
        assert!(
            matches!(
                carrier_eq(&a, &in_band, PlaneIdentity::NONE, &at(1.0), band()),
                Err(CarrierEqError::Undeclared { .. })
            ),
            "in-band, undeclared: refuses"
        );
        assert_eq!(
            carrier_eq(&a, &in_band, declared(), &at(1.0), band()).unwrap(),
            CarrierRelation::SameOpposite,
            "in-band, declared: the bridged residue"
        );
        let definite = torus(
            [0.0, 0.0, 0.0],
            [0.0, 0.0, 1.0],
            5.0,
            0.06 + b.escalate() * 1000.0,
            false,
        );
        match carrier_eq(&a, &definite, declared(), &at(1.0), band()).unwrap_err() {
            CarrierEqError::Contradicted { diag: d, .. } => {
                assert_eq!(d.predicate, Some("carrier_torus_minor_radius"));
            }
            other => panic!("expected Contradicted, got {other:?}"),
        }
    }

    /// The torus's ANGULAR margin is levered at its own ring, whatever
    /// extent the caller names: a tilt about a diameter swings the far
    /// side of the ring by the tilt times `R`. A tilt of `0.3·Kε`, in
    /// band at a metre, stands `1.5·Kε` off at the 5 m ring: unsettled
    /// where no point of the face is known, contradicted by the point on
    /// the tube's crown there.
    #[test]
    fn torus_axis_tilt_is_levered_at_the_ring() {
        let b = band();
        let a = torus([0.0, 0.0, 0.0], [0.0, 0.0, 1.0], 5.0, 0.06, true);
        let tilt = b.escalate() * 0.3;
        let tilted = torus([0.0, 0.0, 0.0], [tilt, 0.0, 1.0], 5.0, 0.06, false);
        assert!(
            matches!(
                carrier_eq(&a, &tilted, declared(), &at(1.0), band()),
                Err(CarrierEqError::Unsettled { .. })
            ),
            "unwitnessed, the swing at the ring is unsettled"
        );
        let on = [Point3::new(5.0, 0.0, 0.06)];
        match carrier_eq(&a, &tilted, declared(), &witnessed(1.0, [&on, &[]]), band()).unwrap_err()
        {
            CarrierEqError::Contradicted { diag: d, .. } => {
                assert_eq!(d.predicate, Some("carrier_torus_axis_parallel"));
            }
            other => panic!("expected Contradicted at the crown, got {other:?}"),
        }
    }

    /// Aligned coincidence is the carrier ladder's honest
    /// `SameOriented`, on the torus arm as on every other: two
    /// same-carrier walls facing the SAME way are declarable as a
    /// continuation, and refusing a `Rest` claim on them is the
    /// declaration door's job.
    #[test]
    fn torus_aligned_is_same_oriented() {
        let a = torus([0.0, 0.0, 0.0], [0.0, 0.0, 1.0], 5.0, 0.06, true);
        let b = torus([0.0, 0.0, 0.0], [0.0, 0.0, 1.0], 5.0, 0.06, true);
        assert_eq!(
            carrier_eq(&a, &b, declared(), &at(1.0), band()).unwrap(),
            CarrierRelation::SameOriented
        );
    }

    /// A torus is not a cylinder at any pair of radii: the kind rung
    /// answers structurally, in both directions.
    #[test]
    fn torus_against_cylinder_is_a_kind_mismatch() {
        let t = torus([0.0, 0.0, 0.0], [0.0, 0.0, 1.0], 5.0, 0.06, true);
        let c = cyl([0.0, 0.0, 0.0], [0.0, 0.0, 1.0], 5.0, false);
        assert_eq!(
            carrier_eq(&t, &c, PlaneIdentity::NONE, &at(1.0), band()).unwrap(),
            CarrierRelation::Distinct
        );
        assert!(matches!(
            carrier_eq(&c, &t, declared(), &at(1.0), band()),
            Err(CarrierEqError::Contradicted { .. })
        ));
    }

    /// Kinds do not compare: a plane is not a cylinder at any radius,
    /// and a declaration saying so is contradicted structurally.
    #[test]
    fn kind_mismatch_is_distinct_and_contradicts_when_declared() {
        let p = CarrierDesc::Plane {
            origin: Point3::origin(),
            normal: Vec3::new(0.0, 0.0, 1.0),
        };
        let c = cyl([0.0, 0.0, 0.0], [0.0, 0.0, 1.0], 3.0, true);
        assert_eq!(
            carrier_eq(&p, &c, PlaneIdentity::NONE, &at(1.0), band()).unwrap(),
            CarrierRelation::Distinct
        );
        assert!(matches!(
            carrier_eq(&p, &c, declared(), &at(1.0), band()),
            Err(CarrierEqError::Contradicted { .. })
        ));
    }

    /// The plane arm is the plane ladder's: `carrier_eq` on two plane
    /// descriptions agrees with `oriented_plane_eq` called directly,
    /// verdict for verdict.
    #[test]
    fn plane_arm_delegates_unchanged() {
        let p1 = PlaneDesc {
            origin: Point3::origin(),
            normal: Vec3::new(0.0, 0.0, 1.0),
        };
        let p2 = PlaneDesc {
            origin: Point3::new(0.0, 0.0, 0.0),
            normal: Vec3::new(0.0, 0.0, -1.0),
        };
        let via_carrier = carrier_eq(
            &CarrierDesc::Plane {
                origin: p1.origin,
                normal: p1.normal,
            },
            &CarrierDesc::Plane {
                origin: p2.origin,
                normal: p2.normal,
            },
            declared(),
            &at(1.0),
            band(),
        )
        .unwrap();
        let direct =
            crate::boolean::plane_eq::oriented_plane_eq(&p1, &p2, declared(), &at(1.0), band())
                .unwrap();
        assert_eq!(via_carrier, direct);
        assert_eq!(direct, CarrierRelation::SameOpposite);
    }

    /// **The curved rung's material side is the faces' `outward` bits,
    /// which the composed `orient` does not track** (`source_rung`'s
    /// docs). One sourced cylinder face against itself, against its
    /// reverted body, and against a twin with its sense flipped, each
    /// pair both ways: the rung reads the reverted pair opposed, where
    /// the declaration ladder over the same composed sources reads it
    /// `SameSource`.
    ///
    /// The ladder column is a measurement of today's composition, not
    /// a contract: its reverted row is the reading
    /// [`face_oriented_source`](super::super::reduce::face_oriented_source)'s
    /// docs call wrong for a curved face. A composition that learns
    /// curved faces and moves that row to an opposed reading is the
    /// fix, to be re-pinned here.
    #[test]
    fn the_curved_source_rung_reads_a_reverted_face_as_opposed() {
        use super::super::reduce::face_oriented_source;
        use crate::source::{GeomSource, SurfaceDeclaration as D, source_declaration};
        use CarrierRelation::{SameOpposite, SameOriented};
        let mut body = crate::Body::<f64>::new();
        let (face, key) = crate::test_support_fixtures::unit_cyl_sheet(
            &mut body,
            None,
            (0.0, 1.0),
            (0.0, 1.0),
            true,
            Tol::witness(),
        );
        body.set_surface_source(key, GeomSource::minted(7, 0))
            .unwrap();
        let reverted = body.revert().unwrap();
        let mut flipped = body.clone();
        flipped.set_face_sense(face, false).unwrap();
        for (name, other, rung, composed_today) in [
            ("itself", &body, SameOriented, D::SameSource),
            ("its reverted body", &reverted, SameOpposite, D::SameSource),
            ("its sense flipped", &flipped, SameOpposite, D::Mirrored),
        ] {
            for (x, y) in [(&body, other), (other, &body)] {
                assert_eq!(
                    crate::boolean::rest::carrier_pair_relation(x, face, y, face, false, band())
                        .unwrap()
                        .unwrap(),
                    rung,
                    "the curved rung, a face against {name}"
                );
                assert_eq!(
                    source_declaration(
                        face_oriented_source(x, face).as_ref(),
                        face_oriented_source(y, face).as_ref()
                    ),
                    composed_today,
                    "the composed sources' reading of a face against {name} moved; \
                     an opposed reading of the reverted pair is the curved-aware \
                     composition to re-pin, not a regression"
                );
            }
        }
    }
}
