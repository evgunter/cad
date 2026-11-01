//! **Which side of a face one bound of a vertex's sector leaves on**:
//! the one reading of a bound's side, taken by the boolean's sector
//! passes (`boolean::sectors`) and the split's neighbourhood walk
//! ([`crate::splitting::entry_side`]) alike.
//!
//! A bound is a ray out of a vertex with what stands behind it
//! ([`Reach`]): a line edge's far vertex, a curved edge's departure
//! tangent and bend, a probe's straight ray, or a subdivision bisector,
//! which bends as its sector's face does. The face is read through its
//! datum at the vertex ([`FaceDatum`]): the normal that names its sides,
//! how it bends ([`FaceBend`]), and the lever its curvature is charged
//! at.
//!
//! The reading is first order, then one order down where first order
//! ties ([`read`], [`Tie::descend`]):
//!
//! - **First order** (`d̂·n̂` at the bound's reach, a line edge's far
//!   vertex in metres), charged for the face's curvature where the
//!   datum carries a lever: decided off the band, the bound is
//!   [`FirstOrder::Off`] the face on that side.
//! - **A first-order tie** is a [`Tie`], which [`Tie::descend`] reads
//!   one order down, the bound's bend past the face's normal section
//!   along it: decided, the bound leaves the face tangent to it,
//!   [`BoundSide::Leaves`]; tied there too, it rides the face,
//!   [`BoundSide::Rides`]; where a bend is not certified (a spline face,
//!   a spiric or spline edge against a curved face), it refuses.
//!
//! A consumer that keys on the first-order tie reads [`read`] and keeps
//! the [`Tie`] whole; one that wants the side reads
//! [`Tie::descend`] too.

use geom_brep::recourse::Refused;
use geom_brep::{ConicBend, EntersMaterial, LeverEscalation, OutwardNormal, ReferenceNormal};
use geom_core::{Band, Decide, Decided, Indeterminate, Margin, Mat3, Point3, Sign, Vec3};

/// What stands behind a sector bound, which decides how its side of a
/// face is read ([`read`]). A bound is its edge's departure from the
/// base vertex: a line's chord, a curved edge's tangent there.
#[derive(Clone, Copy, Debug)]
pub(crate) enum Reach<T: geom_core::Real> {
    /// A line edge: its far vertex, read in metres off the base vertex.
    Chord {
        /// The sector's base vertex.
        base: Point3<T>,
        /// The edge's other end.
        far: Point3<T>,
    },
    /// A probe direction with no edge behind it, levered at a stated
    /// arm (`vtxfac`'s interior probes): a straight ray.
    Extent(T),
    /// A curved edge — a conic, a spiric or a spline: its departure
    /// tangent at the base vertex, levered at the edge's own extent,
    /// and how it bends off that tangent ([`ConicBend`]), which
    /// [`Tie::descend`] reads where the tangent ties with a face.
    Conic {
        /// The edge's extent from below ([`geom_brep::edge_extent_inf`]):
        /// no longer than its arc, so a point at that arc length lies on
        /// the edge.
        extent: T,
        /// The edge's bend at the base vertex.
        bend: ConicBend<T>,
    },
    /// A subdivision bisector: a direction with no point behind it,
    /// levered at its sector's arm, and how its sector's face bends
    /// along it: the face's normal section's curvature vector
    /// ([`FaceBend::section`] along the face's normal), `None` on a
    /// spline face, whose bend is not certified.
    Bisector(T, Option<Vec3<T>>),
}

impl<T: geom_core::Real> Reach<T> {
    /// The length the bound reaches from its base vertex, in metres:
    /// a line edge's chord, a curved edge's extent, a bisector's arm.
    pub(crate) fn length(self) -> T {
        match self {
            Self::Chord { base, far } => (far - base).norm(),
            Self::Extent(l) | Self::Conic { extent: l, .. } | Self::Bisector(l, _) => l,
        }
    }
}

impl<T: Decide> Reach<T> {
    /// A bisector of a sector on `face` (outward unit normal `n`) along
    /// the unit `dir`, levered at `arm`: it bends as the face's normal
    /// section along it.
    pub(crate) fn bisector(arm: T, dir: Vec3<T>, n: Vec3<T>, face: FaceBend<T>) -> Self {
        Self::Bisector(arm, face.section(dir, n).map(|a| n * a))
    }

    /// A bisector of a sector on a plane, levered at `arm`.
    #[cfg(test)]
    pub(crate) fn straight_bisector(arm: T) -> Self {
        Self::Bisector(arm, Some(Vec3::zero()))
    }

    /// The bound's signed first-order departure from the plane through
    /// its base vertex with unit normal `n`, as [`read`] reads it: a
    /// line edge's far vertex, in metres; any other bound's direction
    /// `dir` levered at its reach ([`rise`]). The vertex-on-face pass
    /// quotes it for a sector whose bounds tie but whose tilt it cannot
    /// decide (`bool_sector_coplanar`): the steeper bound's first-order
    /// departure is what a smaller tolerance reads off the plane. A
    /// conic's bend is [`Tie::descend`]'s.
    pub(crate) fn departure(self, dir: Vec3<T>, n: Vec3<T>) -> T {
        match self {
            Self::Chord { base, far } => crate::sector_shape::plane_offset(base, n, far),
            Self::Extent(l) | Self::Conic { extent: l, .. } | Self::Bisector(l, _) => {
                rise(dir, n, l)
            }
        }
    }
}

/// A direction's first-order rise off the plane with unit normal `n`,
/// levered at `arm`: `d̂·n̂·arm`, in metres.
pub(crate) fn rise<T: Decide>(dir: Vec3<T>, n: Vec3<T>, arm: T) -> T {
    geom_brep::rise(dir, n, arm).value()
}

/// **Whether a face is a plane** ([`FaceBend::shape`]): on a plane a
/// bound in it lies in the face, while a bound in a curved face's
/// tangent plane rides it to first order only.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum FaceShape {
    /// The face lies on a plane.
    Plane,
    /// The face lies on a curved surface.
    Curved,
}

impl FaceShape {
    /// The shape of a face on a surface of `kind`.
    pub(crate) fn of_kind(kind: geom::SurfaceKind) -> Self {
        match kind {
            geom::SurfaceKind::Plane => Self::Plane,
            _ => Self::Curved,
        }
    }
}

/// **How a face bends at the vertex**, as a bound tied with its tangent
/// plane is read against it one order down ([`Tie::descend`]).
#[derive(Clone, Copy, Debug)]
pub(crate) enum FaceBend<T: geom_core::Real> {
    /// A plane: it does not bend.
    Plane,
    /// A carrier with an implicit form `F`: its gradient and Hessian at
    /// the vertex, which give its normal section along any tangent.
    Implicit {
        /// `∇F` at the vertex.
        gradient: Vec3<T>,
        /// `∇²F` at the vertex.
        hessian: Mat3<T>,
    },
    /// A spline face: no implicit form, so no certified bend.
    Fitted,
}

impl<T: geom_core::Real> FaceBend<T> {
    /// How `surface` bends at its point `p`.
    pub(crate) fn at(surface: &geom::Surface<T>, p: Point3<T>) -> Self {
        match surface {
            geom::Surface::Plane { .. } => Self::Plane,
            geom::Surface::Nurbs(_) | geom::Surface::Approx(_) => Self::Fitted,
            _ => {
                // `dᵀ∇²F d` polarized on the axes: the form's matrix.
                let form = |d| geom_brep::implicit_hessian_form(surface, p, d);
                let axes = [Vec3::unit_x(), Vec3::unit_y(), Vec3::unit_z()];
                let diag = axes.map(form);
                let half = T::from_f64(0.5);
                let off = |i: usize, j: usize| (form(axes[i] + axes[j]) - diag[i] - diag[j]) * half;
                let (xy, xz, yz) = (off(0, 1), off(0, 2), off(1, 2));
                Self::Implicit {
                    gradient: geom_brep::implicit_gradient(surface, p),
                    hessian: Mat3::from_cols(
                        Vec3::new(diag[0], xy, xz),
                        Vec3::new(xy, diag[1], yz),
                        Vec3::new(xz, yz, diag[2]),
                    ),
                }
            }
        }
    }

    /// Whether the face is a plane.
    pub(crate) fn shape(self) -> FaceShape {
        match self {
            Self::Plane => FaceShape::Plane,
            Self::Implicit { .. } | Self::Fitted => FaceShape::Curved,
        }
    }

    /// The face's normal section's acceleration along the unit tangent
    /// `d`, toward the unit normal `n`, per arc length squared:
    /// `−dᵀ(∇²F)d / (∇F·n)` (differentiate `F(c(s)) = 0` twice), zero
    /// on a plane, `None` on a spline face.
    pub(crate) fn section(self, d: Vec3<T>, n: Vec3<T>) -> Option<T> {
        match self {
            Self::Plane => Some(T::zero()),
            Self::Implicit { gradient, hessian } => {
                Some(T::zero() - d.dot(hessian * d) / gradient.dot(n))
            }
            Self::Fitted => None,
        }
    }
}

#[cfg(test)]
impl FaceBend<f64> {
    /// A sphere of `radius` whose implicit gradient at the vertex is the
    /// unit `gradient`: read against an outward normal along the
    /// gradient, a ball, which bends off its tangent plane against the
    /// normal; against it, a hollow, which bends toward it.
    pub(crate) fn ball(gradient: Vec3<f64>, radius: f64) -> Self {
        let k = 1.0 / radius;
        Self::Implicit {
            gradient,
            hessian: Mat3::from_cols(
                Vec3::new(k, 0.0, 0.0),
                Vec3::new(0.0, k, 0.0),
                Vec3::new(0.0, 0.0, k),
            ),
        }
    }
}

/// **A face at the vertex, as a bound's side is read against it**: the
/// normal naming its sides, how it bends, and the lever its curvature
/// is charged at (`None`: a splitting plane, which has no curvature and
/// takes no charge).
#[derive(Clone, Copy, Debug)]
pub(crate) struct FaceDatum<T: geom_core::Real> {
    /// The normal: against it is [`Side::Below`] (into a face's
    /// material), along it [`Side::Above`].
    pub reference: ReferenceNormal<T>,
    /// How the face bends at the vertex.
    pub bend: FaceBend<T>,
    /// The face's smallest radius of curvature within the bound's reach
    /// (`boolean::sectors::PierceLever`), where a definite first-order
    /// verdict is charged for it.
    pub lever: Option<T>,
}

impl<T: geom_core::Real> FaceDatum<T> {
    /// A face's datum: its outward normal at the vertex, its bend, and
    /// its curvature lever.
    pub(crate) fn of_face(normal: OutwardNormal<T>, bend: FaceBend<T>, lever: T) -> Self {
        Self {
            reference: ReferenceNormal::of_face_outward(normal),
            bend,
            lever: Some(lever),
        }
    }

    /// A plane's datum with no curvature charge: a splitting plane, or a
    /// face's plane a section is read against.
    pub(crate) fn of_plane(reference: ReferenceNormal<T>) -> Self {
        Self {
            reference,
            bend: FaceBend::Plane,
            lever: None,
        }
    }
}

/// A side of a face: against its datum's normal, or along it.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Side {
    /// Against the normal: into a face's material.
    Below,
    /// Along the normal: out of a face's material.
    Above,
}

impl Side {
    fn of(verdict: EntersMaterial) -> Option<Self> {
        match verdict {
            EntersMaterial::Enters => Some(Self::Below),
            EntersMaterial::Exits => Some(Self::Above),
            EntersMaterial::Tangent => None,
        }
    }
}

/// **A bound's side of a face, to the order it is read.**
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum BoundSide {
    /// Decided at first order.
    Off(Side),
    /// Tied at first order, decided one order down: the bound leaves the
    /// face tangent to it, toward this side. (A first-order tie, which
    /// [`EntersMaterial::Tangent`] names, is undecided; this is decided.)
    Leaves(Side),
    /// Tied at first order and at the order below: the bound rides the
    /// face to the orders read (a line edge in a plane, a conic whose
    /// whole edge lies within the band of a plane).
    Rides,
}

/// The first order of [`read`]: a side, or a tie [`Tie::descend`] reads
/// one order down.
#[derive(Clone, Copy, Debug)]
pub(crate) enum FirstOrder<T: geom_core::Real> {
    /// Decided at first order, and charged for the face's curvature.
    Off(Side),
    /// Tied at first order.
    Tied(Tie<T>),
}

/// **A bound tied at first order with a face**, and what the tie was
/// decided on.
#[derive(Clone, Copy, Debug)]
pub(crate) struct Tie<T: geom_core::Real> {
    dir: Vec3<T>,
    reach: Reach<T>,
    face: FaceDatum<T>,
    /// The predicate that decided the Zero.
    pub predicate: &'static str,
    /// The Zero, with the margin it was decided on.
    pub decided: Decided,
}

/// Why a reading with no curvature charge read no side: the first
/// order ([`first_order`]) or the descent ([`Tie::descend`]). Every arm
/// carries the indeterminate it was read on.
#[derive(Clone, Copy, Debug)]
pub(crate) enum ReadFault<T: geom_core::Real> {
    /// A line edge's far vertex lies in the band of the plane
    /// (`bool_chord_side`).
    Chord(Indeterminate),
    /// The direction's arm or its first-order departure lies in the
    /// band, with the arm and the departure it read.
    Departure {
        /// The arm gate's or the reading's escalation.
        escalation: LeverEscalation,
        /// The bound's reach.
        arm: T,
        /// Its first-order departure over that reach, in metres.
        departure: T,
    },
    /// A conic's lift off a plane lies in the band of its first-order
    /// residue, or is not certified (a spiric or a spline).
    Bend(Indeterminate),
    /// The bound's parting from the face it is tied with lies in the
    /// band.
    Parting(LeverEscalation),
    /// A tie where a bend is not certified (a spline face, a bisector
    /// of one, or a spiric or spline edge against a curved face): the
    /// gate's rejection of the first-order Zero it stands on.
    Unbent(Indeterminate),
}

impl<T: geom_core::Real> ReadFault<T> {
    /// The indeterminate the fault was read on.
    pub(crate) fn diag(self) -> Indeterminate {
        match self {
            Self::Chord(diag) | Self::Bend(diag) | Self::Unbent(diag) => diag,
            Self::Departure { escalation, .. } | Self::Parting(escalation) => escalation.diag(),
        }
    }
}

/// Why [`read`] or [`read_side`] read no side.
#[derive(Clone, Copy, Debug)]
pub(crate) enum BoundSideFault<T: geom_core::Real> {
    /// The reading before any charge.
    Read(ReadFault<T>),
    /// The curvature charge did not clear the band: the side is second
    /// order there.
    Charge(Refused),
    /// The curvature charge lies in the band.
    ChargeEscalated(Indeterminate),
}

/// The first order of [`read`] before its charge: a side with the
/// bound's displacement off the face's tangent plane in metres, or a tie.
fn first_order<T: Decide>(
    dir: Vec3<T>,
    reach: Reach<T>,
    face: FaceDatum<T>,
    band: Band,
) -> Result<Result<(Side, T), Tie<T>>, ReadFault<T>> {
    let n = face.reference.vec();
    let length = reach.length();
    let tie = |predicate, decided| {
        Ok(Err(Tie {
            dir,
            reach,
            face,
            predicate,
            decided,
        }))
    };
    Ok(Ok(match reach {
        Reach::Chord { base, far } => {
            let offset = crate::sector_shape::plane_offset(base, n, far);
            match crate::validate::decide_reported("bool_chord_side", Margin::of(offset), band) {
                Ok(Decided {
                    sign: Sign::Negative,
                    ..
                }) => (Side::Below, offset.abs()),
                Ok(Decided {
                    sign: Sign::Positive,
                    ..
                }) => (Side::Above, offset.abs()),
                Ok(zero) => return tie("bool_chord_side", zero),
                Err(diag) => return Err(ReadFault::Chord(diag)),
            }
        }
        Reach::Extent(_) | Reach::Conic { .. } | Reach::Bisector(..) => {
            match geom_brep::departure_side_reported(dir, face.reference, length, band) {
                Ok((verdict, margin)) => match Side::of(verdict) {
                    Some(side) => (side, rise(dir, n, length).abs()),
                    None => {
                        return tie(
                            "enters_material",
                            Decided {
                                sign: Sign::Zero,
                                margin,
                            },
                        );
                    }
                },
                Err(escalation) => {
                    return Err(ReadFault::Departure {
                        escalation,
                        arm: length,
                        departure: rise(dir, n, length),
                    });
                }
            }
        }
    }))
}

/// **A bound's side of a face at first order**, read as its [`Reach`]
/// allows, and charged for the face's curvature where the datum carries
/// a lever.
///
/// # The reading, per reach
///
/// A `Zero` here is a [`Tie`], which a consumer may key on as "this
/// bound lies in the plane". So a Zero has to be a distance, not a
/// direction times a lever that is not the bound's own length.
/// - [`Reach::Chord`] (a line edge): its far vertex's signed distance
///   from the plane through the base vertex, in metres
///   ([`crate::sector_shape::plane_offset`]). The edge is its chord, so
///   every point of it lies between the base's distance (`≈ 0`, the
///   vertex is On) and the far vertex's: a Zero puts the whole edge
///   within the band of the plane.
/// - [`Reach::Conic`] (a curved edge): the departure tangent levered at
///   the edge's own extent. A definite sign is exact (the sign of
///   `t̂·n̂`); a Zero is a first-order tangency.
/// - [`Reach::Extent`] (a probe): the direction levered at its arm.
/// - [`Reach::Bisector`]: a subdivision direction has no point behind
///   it, so it is levered at its sector's arm.
///
/// # Why a definite first-order verdict is charged on a curved face
///
/// The verdict is the bound's side NEAR THE VERTEX: a bound whose
/// first-order departure `s = |d̂·n̂|` from the face's tangent plane
/// is nonzero lies on that side of the face for every small enough
/// distance, whatever the face's curvature and whatever the bound's
/// own. What the charge asks is whether that side is RESOLVED at the
/// band.
///
/// It reads the bound's tangent ray, not points of the bound. At
/// distance `l` along the ray, the face departs its tangent plane by
/// at most the sagitta `l²/lever`, so the ray sits at least
/// `g(l) = s·l − l²/lever` off the face on the verdict's side. That
/// lower bound grows faster than the first-order term, and far enough
/// out it flips sign. The witness is a hole wall (`r = 1`, material
/// outside, so the outward normal points at the axis): a direction
/// with `s ≈ 0.0995` is a definite `Exits`, and it does leave the
/// material, but by `l = 0.5` it has crossed back, and the point there
/// sits at `ρ = 1.0726`, INSIDE the material. So the charge is read
/// where `g` peaks, `l* = s·lever/2`, giving `s²·lever/4`, and capped
/// at the reach's own length: a separation witnessed only beyond the
/// bound's own extent is a direction levered at a lever that is not
/// the bound's length. The verdict stands iff that separation clears
/// the band. What refuses is a bound leaving the face within about
/// `2·sqrt(band/lever)` radians of tangent (or a reach too short to
/// witness its slope), whose side is second order.
///
/// The charge is direction-agnostic on purpose, which is sound for
/// either bend. A plane's lever is infinite, the sagitta underflows to
/// zero, and the charge reduces to the reading just decided definite.
/// A tie takes no charge: it is not a side.
pub(crate) fn read<T: Decide>(
    dir: Vec3<T>,
    reach: Reach<T>,
    face: FaceDatum<T>,
    band: Band,
) -> Result<FirstOrder<T>, BoundSideFault<T>> {
    let length = reach.length();
    let (side, displacement) = match first_order(dir, reach, face, band) {
        Ok(Ok(off)) => off,
        Ok(Err(tie)) => return Ok(FirstOrder::Tied(tie)),
        Err(fault) => return Err(BoundSideFault::Read(fault)),
    };
    let Some(lever) = face.lever else {
        return Ok(FirstOrder::Off(side));
    };
    // **The sagitta bound, and why the textbook ½ is not in it.** At
    // lateral offset `l` a circle of radius `R` departs its tangent
    // line by exactly `R − sqrt(R² − l²) = l²/(R + sqrt(R² − l²))`. The
    // familiar `l²/2R` is that expression's small-`l` LIMIT and is a
    // LOWER bound on it, so charging `l²/2R` would under-charge exactly
    // where the distance is a large fraction of the radius. Dropping
    // the ½ gives `l²/R`, an upper bound unconditionally (`l* ≤ R/2`
    // here, inside the circle's reach).
    let slope = displacement / length;
    match crate::validate::decide_reported(
        "bool_pierce_sector_side_curved",
        Margin::of(sagitta_charge(slope, lever, length)),
        band,
    ) {
        Ok(decided) => match Refused::of(decided, band) {
            None => Ok(FirstOrder::Off(side)),
            Some(refused) => Err(BoundSideFault::Charge(refused)),
        },
        Err(diag) => Err(BoundSideFault::ChargeEscalated(diag)),
    }
}

/// [`read`], descended where first order ties ([`Tie::descend`]).
#[cfg(test)]
pub(crate) fn read_side<T: Decide>(
    dir: Vec3<T>,
    reach: Reach<T>,
    face: FaceDatum<T>,
    band: Band,
) -> Result<BoundSide, BoundSideFault<T>> {
    match read(dir, reach, face, band)? {
        FirstOrder::Off(side) => Ok(BoundSide::Off(side)),
        FirstOrder::Tied(tie) => tie.descend(band).map_err(BoundSideFault::Read),
    }
}

/// **A bound's side of a plane with no curvature charge** (a splitting
/// plane's): [`read_side`] with no charge to refuse, where a bound whose
/// reach the band does not read as a length is tied rather than refused
/// ([`Tie::of_unread_reach`]): such an edge lies within the band of the
/// plane wherever its bend leaves it, and the descent reads it so.
pub(crate) fn read_plane_side<T: Decide>(
    dir: Vec3<T>,
    reach: Reach<T>,
    reference: ReferenceNormal<T>,
    band: Band,
) -> Result<BoundSide, ReadFault<T>> {
    let face = FaceDatum::of_plane(reference);
    let tie = match first_order(dir, reach, face, band) {
        Ok(Ok((side, _))) => return Ok(BoundSide::Off(side)),
        Ok(Err(tie)) => tie,
        Err(fault) => Tie::of_unread_reach(dir, reach, face, fault).ok_or(fault)?,
    };
    tie.descend(band)
}

/// **The curvature charge**: the least separation from a face that a
/// ray leaving it at `slope` (a sine) keeps past a bend of radius
/// `lever`, `slope·l − l²/lever`, read at `l = min(slope·lever/2,
/// reach)`, where it peaks within the reach. [`read`] carries the
/// argument and the witness; `vtxfac::faces_holding_pierce` charges a
/// held pierce's run with it.
pub(crate) fn sagitta_charge<T: Decide>(slope: T, lever: T, reach: T) -> T {
    let l = (slope * T::from_f64(0.5) * lever).min(reach);
    slope * l - l.powi(2) / lever
}

impl<T: Decide> Tie<T> {
    /// **A bound whose reach the band does not read as a length**, tied:
    /// `fault` is [`read`]'s, on the arm gate's rung (the reach is within
    /// the band, or decided zero there). A consumer that reads such a
    /// bound as lying within the band of the plane (the split walk: a
    /// conic edge whose whole extent is in the band) ties it here and
    /// descends it as any tie; the Zero quoted is the arm's. `None` for
    /// any other fault.
    pub(crate) fn of_unread_reach(
        dir: Vec3<T>,
        reach: Reach<T>,
        face: FaceDatum<T>,
        fault: ReadFault<T>,
    ) -> Option<Self> {
        match fault {
            ReadFault::Departure { escalation, .. }
                if escalation.rung() == geom_brep::LeverRung::Arm =>
            {
                Some(Self {
                    dir,
                    reach,
                    face,
                    predicate: "enters_material_arm",
                    decided: Decided {
                        sign: Sign::Zero,
                        margin: escalation.diag().margin,
                    },
                })
            }
            _ => None,
        }
    }

    /// **The tied bound's side one order down**: the sign of
    /// `(κ_b·n̂ − II_F(d̂, d̂))·l²/2` ([`geom_brep::parting_side`]), the
    /// bound's bend past the face's normal section along it, at the
    /// shorter of the bound's reach and the face's lever: a side the
    /// bound leaves to, [`BoundSide::Leaves`], or tied there too,
    /// [`BoundSide::Rides`]. The bound's bend is a conic's curvature, a
    /// bisector's sector face's normal section along it, and nothing for
    /// a line edge or a probe, which are straight; a plane's section is
    /// nothing.
    ///
    /// A conic against a plane is read by its certified lift
    /// ([`geom_brep::conic_bend_side`]), whose `Tangent` is the whole
    /// edge within the band of the plane; a line edge or a probe against
    /// a plane rides it.
    ///
    /// # Errors
    ///
    /// [`BoundSideFault::Bend`]: a conic's lift against a plane in the
    /// band of its first-order residue, or a spiric's or a spline's,
    /// which certify no bend. [`BoundSideFault::Parting`]: the parting in
    /// the band. [`BoundSideFault::Unbent`]: a spline face, a bisector
    /// of one, or a spiric or spline edge against a curved face, whose
    /// bend is not certified.
    pub(crate) fn descend(self, band: Band) -> Result<BoundSide, ReadFault<T>> {
        let (face, reach) = (self.face, self.reach);
        let own = match (face.bend, reach) {
            (FaceBend::Fitted, _) | (_, Reach::Bisector(_, None)) => return Err(self.unbent(band)),
            (FaceBend::Plane, Reach::Conic { extent, bend }) => {
                return match geom_brep::conic_bend_side(
                    self.dir,
                    extent,
                    bend,
                    face.reference,
                    band,
                ) {
                    Ok(verdict) => {
                        Ok(Side::of(verdict).map_or(BoundSide::Rides, BoundSide::Leaves))
                    }
                    Err(diag) => Err(ReadFault::Bend(diag)),
                };
            }
            (FaceBend::Plane, Reach::Chord { .. } | Reach::Extent(_)) => {
                return Ok(BoundSide::Rides);
            }
            (_, Reach::Bisector(_, Some(bend))) => bend,
            (FaceBend::Implicit { .. }, Reach::Chord { .. } | Reach::Extent(_)) => Vec3::zero(),
            (FaceBend::Implicit { .. }, Reach::Conic { bend, .. }) => match bend.bounds {
                geom_brep::BendBounds::Planar { .. } => bend.curvature,
                geom_brep::BendBounds::Unbounded => return Err(self.unbent(band)),
            },
        };
        let n = face.reference.vec();
        let Some(section) = face.bend.section(self.dir.normalize(), n) else {
            return Err(self.unbent(band));
        };
        let arm = face
            .lever
            .map_or(reach.length(), |lever| reach.length().min(lever));
        match geom_brep::parting_side(own - n * section, face.reference, arm, band) {
            Ok(verdict) => Ok(Side::of(verdict).map_or(BoundSide::Rides, BoundSide::Leaves)),
            Err(escalation) => Err(ReadFault::Parting(escalation)),
        }
    }

    fn unbent(&self, band: Band) -> ReadFault<T> {
        ReadFault::Unbent(geom_core::k_stats::gate_rejection(
            self.predicate,
            self.decided,
            band,
            geom_core::SizedPass::NonZero,
        ))
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod tests {
    use super::*;
    use geom_brep::ConicBend;
    use geom_core::Tol;

    fn band() -> Band {
        Band::linear(Tol::witness()).unwrap()
    }

    /// A quarter of the unit circle leaving the origin along `t` (in
    /// the `xz` plane, `tilt` off `+x` toward `+z`), bending toward
    /// `bend` turned by the same `tilt` about `y` (so square to `t`).
    fn quarter_arc(bend: Vec3<f64>, tilt: f64) -> (Vec3<f64>, Reach<f64>) {
        let span = core::f64::consts::FRAC_PI_2;
        let (c, s) = (tilt.cos(), tilt.sin());
        let t = Vec3::new(c, 0.0, s);
        let bend = Vec3::new(c * bend.x - s * bend.z, bend.y, s * bend.x + c * bend.z);
        assert!(
            t.dot(bend).abs() < 1e-15,
            "the bend is square to the tangent"
        );
        let carrier = geom::Curve3::Circle {
            center: Point3::origin() + bend,
            axis: t.cross(bend),
            radius: 1.0,
            u_ref: -bend,
        };
        let extent = geom_brep::edge_extent_inf(&carrier, 0.0, span, 2.0 * (span / 2.0).sin());
        let tangent = carrier.deriv(0.0);
        let bend = ConicBend::of(&carrier, tangent, carrier.deriv2(0.0), span);
        (tangent.normalize() * extent, Reach::Conic { extent, bend })
    }

    /// **A tie is kept whole at first order and descended on asking**: a
    /// line in a plane rides it, and against a curved face a bound reads
    /// its bend past the face's normal section along it: a line leaves a
    /// ball outward and a hollow inward, an arc bending down leaves a
    /// plane and a ball inward and rides a ball of its own radius.
    #[test]
    fn a_tie_descends_by_the_face_and_the_bound() {
        let b = band();
        let z = Vec3::new(0.0, 0.0, 1.0);
        let line = Reach::Chord {
            base: Point3::origin(),
            far: Point3::new(1.0, 0.0, 0.0),
        };
        let x = Vec3::new(1.0, 0.0, 0.0);
        let datum = |bend| FaceDatum::of_face(OutwardNormal::from_chart(z, true), bend, 2.0);
        for (bend, want) in [
            (FaceBend::Plane, BoundSide::Rides),
            (FaceBend::ball(z, 2.0), BoundSide::Leaves(Side::Above)),
            (FaceBend::ball(-z, 2.0), BoundSide::Leaves(Side::Below)),
        ] {
            let FirstOrder::Tied(tie) = read(x, line, datum(bend), b).unwrap() else {
                panic!("a line in the tangent plane ties at first order");
            };
            assert_eq!(tie.predicate, "bool_chord_side", "the chord's tie");
            assert_eq!(tie.descend(b).unwrap(), want, "{bend:?}");
        }
        let (dir, arc) = quarter_arc(Vec3::new(0.0, 0.0, -1.0), 0.0);
        for (bend, want) in [
            (FaceBend::Plane, BoundSide::Leaves(Side::Below)),
            (FaceBend::ball(z, 2.0), BoundSide::Leaves(Side::Below)),
            (FaceBend::ball(z, 1.0), BoundSide::Rides),
            (FaceBend::ball(z, 0.5), BoundSide::Leaves(Side::Above)),
        ] {
            assert_eq!(
                read_side(dir, arc, datum(bend), b).unwrap(),
                want,
                "a unit arc bending down against {bend:?}"
            );
        }
        let FirstOrder::Tied(tie) = read(x, line, datum(FaceBend::Fitted), b).unwrap() else {
            panic!("a line in the tangent plane ties at first order");
        };
        assert!(
            matches!(tie.descend(b), Err(ReadFault::Unbent(..))),
            "a spline face's bend is not read"
        );
    }
}
