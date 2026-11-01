//! **The closed-form tangent locus** — the DEV-1 witness lane: the
//! contact line of a tangent carrier pair, for exactly the
//! configurations whose locus is closed-form. Every numeric decision is
//! a named row through `dihedral::decide`.

use geom::Surface;
use geom_core::{
    Band, Decide, Decided, Indeterminate, Margin, MarginDiag, Point3, Real, Sign, UnitVec3, Vec3,
};

use crate::dihedral::decide;
use crate::extent::{ExtentBall, Reach};
use crate::intersect::{
    ParallelAxes, PlaneCylinder, RuledFrame, RuledSection, cylinder_axes_parallel_reported,
    decide_across_reported, parallel_axes_at, parallel_cylinder_gap_reported, parallel_swing,
    ruled_frame, ruled_gap, ruled_parallel, ruled_section,
};

/// **The certified-lane tangent LOCUS** (M9-2, the M9-1 PR-2 DEV-1
/// ruling): the closed-form contact line of a tangent carrier pair,
/// for exactly the configurations whose locus IS closed-form — a
/// plane and a cylinder tangent along a ruling, and two PARALLEL
/// cylinders tangent along the line between closest generators.
///
/// Its consumers are `topo`'s: the flush detector's `Tangent` arm (a
/// tangency finding without a locus is one the verifier cannot check)
/// and every at-rest verification that mints the witness a `Tangent`
/// declaration is verified along.
#[derive(Clone, Copy, Debug)]
pub enum TangentLocus<T: Real> {
    /// The tangent line: `origin + t·dir`, `dir` unit (both certified
    /// carriers are ruled along it).
    Line {
        /// A point on the locus.
        origin: Point3<T>,
        /// The margin that decided the two surfaces tangent: the gap
        /// row's Zero.
        gap: MarginDiag,
        /// The locus direction (the shared ruling / axis direction).
        dir: Vec3<T>,
    },
}

/// Typed refusal of [`tangent_locus`] (closed enum, D3 style).
#[derive(Debug)]
pub enum TangentLocusError {
    /// A margin landed in the sliver band.
    Escalated(Indeterminate),
    /// The pair is definitely NOT tangent: `apart` distinguishes the
    /// definite-clearance side from the definite-crossing side. On
    /// `tangent_locus_face_clear` ([`tangent_locus_of_faces`]) it is the
    /// two faces that are definitely apart, one standing clear of the
    /// other's carrier across an in-band locus; the carriers may touch
    /// elsewhere.
    NotTangent {
        /// `true`: definite clearance; `false`: definite crossing.
        apart: bool,
        /// The row whose definite verdict refused the tangency.
        predicate: &'static str,
    },
    /// The configuration is outside the closed-form lane (kinds other
    /// than plane×cylinder / parallel cylinders, or a non-parallel
    /// axis relation): the demanded set IS the certifiable set — no
    /// sampled locus, ever.
    Unsupported {
        /// Why the configuration has no closed-form locus.
        what: &'static str,
    },
}

/// The closed-form tangent locus of two carriers (see
/// [`TangentLocus`]). Kind dispatch is structural; every numeric
/// decision is a named three-outcome row, and the tangency itself is
/// the section classifiers' verdict, read through their own rows:
///
/// - **plane×cylinder** is [`crate::plane_cylinder_section`]'s
///   axis-in-plane lane at `reach`, read at the foot of its centre on
///   the axis and levered from there: `pc_axis_plane_parallel` (a
///   definite tilt is `Unsupported`) then `pc_parallel_gap`
///   (`TangentLine` mints its ruling, `ParallelLines` crosses, `Empty`
///   clears). A tilt moves
///   the ruling by its angle times the distance from the foot, so the
///   axis row reads the displacement it induces across the faces the
///   locus is consumed on, and the gap row decides the gap and that
///   displacement as one sum: the ruling it mints stands within one
///   zero band of the carriers across the faces, and the `Tangent`
///   table then verifies it sample by sample.
/// - **parallel cylinders** read [`crate::cylinder_cylinder_section`]'s
///   rows at `reach` as it does, in either order:
///   `cc_axes_parallel`, then `cc_parallel_gap`, the external margin
///   `r1 + r2 − d` with the tilt beside it (Zero mints the ruling,
///   Negative clears). Where the walls cross that margin's way,
///   `tangent_locus_internal_gap` reads `|r1 − r2| − d` the same way —
///   an internal tangency, which no section classifier holds — and
///   `tangent_locus_side` places its generator.
///
/// Either arm's tangency needs both rows Zero, so where the axis row is
/// in band and the gap row still decides a crossing or a clearance, the
/// pair is `NotTangent` on that row: a definite verdict answers before
/// an in-band one ([`definite_over_in_band`]).
///
/// `reach` is the declared pair's consumed extent, the one `topo`'s
/// carrier-pair doors lever their ladder at.
///
/// **CONTRACT — the separation invariant** (consumed by the reduce
/// sweep's declared-cover rung): every configuration this lane mints
/// a locus for has each carrier wholly in ONE closed residual
/// half-space of the other — a plane tangent to a cylinder has the
/// whole cylinder on one side; each of two externally (or
/// internally) tangent parallel cylinders is one-signed against the
/// other — so an on-carrier edge under a verified declaration never
/// crosses the partner surface. A new arm may NOT land here without
/// restating its own residual-sign story.
///
/// **The coaxial cylinder×sphere pair satisfies this contract and still
/// has no arm.** At the only coaxial tangency, `R = r`, the sphere lies
/// wholly in the cylinder's non-positive residual half-space and the
/// cylinder wholly in the sphere's non-negative one — opposite
/// orientations per direction, which the contract allows (the
/// internally tangent parallel cylinder pair the `|r1 − r2|` fallback
/// admits has the same structure); both halves are pinned by
/// `crates/topo/tests/verbs_cylsph_tangent_residuals.rs`. Its circle
/// locus serves the coaxial KISS alone (material wedge 0 or 2π), and two
/// things keep it out. No consumer builds the kiss edge from a locus of
/// any shape, so a circle arm would complete no boolean. And answering
/// that pair here would take it past `Unsupported`, which is where
/// `topo` routes a declared pair meeting along one circle by material
/// wedge — the routing that tells a smooth seam (wedge π, never a
/// `Tangent` contact) from a kiss — so the arm lands only behind that
/// routing, run on the locus.
///
/// # Errors
///
/// [`TangentLocusError`] — escalation, definite non-tangency, or a
/// configuration outside the closed-form lane.
pub fn tangent_locus<T: Decide>(
    a: &geom::Surface<T>,
    b: &geom::Surface<T>,
    reach: ExtentBall<T>,
    band: Band,
) -> Result<TangentLocus<T>, TangentLocusError> {
    locus_of(
        a,
        b,
        reach,
        band,
        None::<fn(UnitVec3<T>) -> [Option<(T, T)>; 2]>,
    )
}

/// Whether carriers of kinds `a` and `b` are in the closed-form lane
/// at all (plane × cylinder, cylinder × cylinder): every other pair is
/// [`TangentLocusError::Unsupported`] before any extent is read, so a
/// caller can leave it unasked.
#[must_use]
pub fn tangent_locus_kinds(a: geom::SurfaceKind, b: geom::SurfaceKind) -> bool {
    use geom::SurfaceKind::{Cylinder, Plane};
    matches!((a, b), (Plane | Cylinder, Cylinder) | (Cylinder, Plane))
}

/// [`tangent_locus`] of two faces on `a` and `b`, whose extents settle
/// an in-band reading: `extents` gives each face's least and greatest
/// coordinate along a unit direction about the world origin, padded,
/// `a`'s first (`None` where a face's extent does not read).
///
/// That contract is the caller's to keep: an extent narrower than its
/// face reads a reaching face clear, and nothing here can tell, since
/// the lane holds the carriers and not the faces. `topo` keeps it by
/// the kernel's one face box rule read in the direction's frame.
///
/// The carriers are read across `reach`, which encloses both faces,
/// so an in-band reading is the carriers'. Where it escalates, the
/// lane asks whether the faces reach the locus at all: a face whose
/// extent across the locus stands far enough off it lies definitely
/// clear of the other carrier, and the pair is `NotTangent { apart }`
/// on `tangent_locus_face_clear`, the escalation superseded
/// ([`Section::faces_clear`]).
///
/// # Errors
///
/// As [`tangent_locus`].
pub fn tangent_locus_of_faces<T: Decide>(
    a: &geom::Surface<T>,
    b: &geom::Surface<T>,
    reach: ExtentBall<T>,
    band: Band,
    extents: impl FnOnce(UnitVec3<T>) -> [Option<(T, T)>; 2],
) -> Result<TangentLocus<T>, TangentLocusError> {
    locus_of(a, b, reach, band, Some(extents))
}

/// [`tangent_locus_of_faces`], or [`tangent_locus`] with no `extents`:
/// then no cross-section is read.
fn locus_of<T: Decide>(
    a: &geom::Surface<T>,
    b: &geom::Surface<T>,
    reach: ExtentBall<T>,
    band: Band,
    extents: Option<impl FnOnce(UnitVec3<T>) -> [Option<(T, T)>; 2]>,
) -> Result<TangentLocus<T>, TangentLocusError> {
    // The witness reads the ball its callers hand it
    // (`crate::extent`'s module docs).
    let reach = &Reach::Ball(reach);
    let ((pending, read), log) =
        geom_core::k_stats::detached(|| closed_form_locus(a, b, reach, band));
    match (read, pending) {
        (Err(TangentLocusError::Escalated(diag)), Some(pending)) if extents.is_some() => {
            // The section is read only behind an escalation. Its own
            // readings decide the answer only through a face decided
            // clear, which is a verdict, so their escalations are never
            // what the outcome rests on.
            let (clear, clear_log) = geom_core::k_stats::detached(|| {
                extents
                    .zip(pending.section(reach, band))
                    .is_some_and(|(extents, section)| section.faces_clear(extents, band))
            });
            geom_core::k_stats::splice_superseded(clear_log);
            if clear {
                geom_core::k_stats::splice_superseded(log);
                Err(TangentLocusError::NotTangent {
                    apart: true,
                    predicate: FACE_CLEAR,
                })
            } else {
                geom_core::k_stats::splice(log);
                Err(TangentLocusError::Escalated(diag))
            }
        }
        (read, _) => {
            geom_core::k_stats::splice(log);
            read
        }
    }
}

/// The row a face clear of the other carrier refuses the tangency on.
const FACE_CLEAR: &str = "tangent_locus_face_clear";

/// The two arms' readings, with what the pair's cross-section is read
/// from where it has one.
#[allow(clippy::type_complexity)]
fn closed_form_locus<T: Decide>(
    a: &geom::Surface<T>,
    b: &geom::Surface<T>,
    reach: &Reach<T>,
    band: Band,
) -> (
    Option<Pending<T>>,
    Result<TangentLocus<T>, TangentLocusError>,
) {
    match (a, b) {
        (
            &Surface::Plane {
                origin: q,
                normal: n,
                ..
            },
            &Surface::Cylinder {
                origin,
                axis,
                radius,
                ..
            },
        )
        | (
            &Surface::Cylinder {
                origin,
                axis,
                radius,
                ..
            },
            &Surface::Plane {
                origin: q,
                normal: n,
                ..
            },
        ) => {
            let pc = PlaneCylinder {
                q,
                n,
                o: origin,
                a: axis,
                r: radius,
            };
            let frame = ruled_frame(&pc, reach);
            let plane_first = matches!(a, Surface::Plane { .. });
            let read = plane_cylinder_locus(&pc, &frame, band);
            let pending = Pending::PlaneCylinder {
                pc,
                frame,
                plane_first,
            };
            (Some(pending), read)
        }
        (
            &Surface::Cylinder {
                origin: o1,
                axis: a1,
                radius: r1,
                ..
            },
            &Surface::Cylinder {
                origin: o2,
                axis: a2,
                radius: r2,
                ..
            },
        ) => {
            let axes = parallel_axes_at(reach, (o1, a1), (o2, a2));
            let swing = parallel_swing(reach, (o1, a1), (o2, a2), axes.d);
            let read = cylinders_locus(((o1, a1), (o2, a2)), (r1, r2), &axes, swing, reach, band);
            let pending = Pending::Cylinders {
                radii: (r1, r2),
                dirs: (a1, a2),
                axes,
            };
            (Some(pending), read)
        }
        _ => (
            None,
            Err(TangentLocusError::Unsupported {
                what: "the closed-form tangent-locus lane holds plane×cylinder and parallel \
                       cylinder pairs only (the DEV-1 certified set)",
            }),
        ),
    }
}

/// The plane×cylinder arm on `frame`: the tilt row, then the gap row.
fn plane_cylinder_locus<T: Decide>(
    pc: &PlaneCylinder<T>,
    frame: &RuledFrame<T>,
    band: Band,
) -> Result<TangentLocus<T>, TangentLocusError> {
    let gap = "pc_parallel_gap";
    let (parallel, axis_log) = geom_core::k_stats::detached(|| ruled_parallel(frame, band));
    let parallel = match parallel {
        Ok(parallel) => {
            geom_core::k_stats::splice(axis_log);
            parallel
        }
        Err(diag) => {
            return Err(definite_over_in_band(diag, axis_log, || {
                match ruled_gap(pc, frame, band) {
                    Ok((Sign::Positive, _)) => Some((false, gap)),
                    Ok((Sign::Negative, _)) => Some((true, gap)),
                    Ok((Sign::Zero, _)) | Err(_) => None,
                }
            }));
        }
    };
    match ruled_section(pc, frame, parallel, band).map_err(TangentLocusError::Escalated)? {
        Some(RuledSection::TangentLine { origin, dir, gap }) => {
            Ok(TangentLocus::Line { origin, gap, dir })
        }
        Some(RuledSection::ParallelLines { .. }) => Err(TangentLocusError::NotTangent {
            apart: false,
            predicate: gap,
        }),
        Some(RuledSection::Empty) => Err(TangentLocusError::NotTangent {
            apart: true,
            predicate: gap,
        }),
        None => Err(TangentLocusError::Unsupported {
            what: "plane×cylinder tangency is closed-form only along a ruling — \
                   the axis must lie in the plane's direction space",
        }),
    }
}

/// The parallel-cylinder arm, its axes read once at the feet (`axes`,
/// their `swing` across the reach): the axes row, then the gap rows.
#[allow(clippy::type_complexity)]
fn cylinders_locus<T: Decide>(
    lines: ((Point3<T>, Vec3<T>), (Point3<T>, Vec3<T>)),
    (r1, r2): (T, T),
    axes: &ParallelAxes<T>,
    swing: T,
    reach: &Reach<T>,
    band: Band,
) -> Result<TangentLocus<T>, TangentLocusError> {
    let escalate = TangentLocusError::Escalated;
    let (line1, line2) = lines;
    let a1 = line1.1;
    // Each tangency below is one only with the axes parallel, so its
    // margin stands for both.
    let (parallel, axis_log) =
        geom_core::k_stats::detached(|| cylinder_axes_parallel_reported(reach, line1, line2, band));
    let parallel = match parallel {
        Ok(Decided {
            sign: Sign::Zero,
            margin,
        }) => {
            geom_core::k_stats::splice(axis_log);
            margin
        }
        Ok(Decided { .. }) => {
            geom_core::k_stats::splice(axis_log);
            return Err(TangentLocusError::Unsupported {
                what: "cylinder×cylinder tangency is closed-form only for PARALLEL \
                       axes (the generator line); skew/crossing axes are outside \
                       the lane",
            });
        }
        Err(diag) => {
            return Err(definite_over_in_band(
                diag,
                axis_log,
                || match parallel_cylinder_gaps((r1, r2), axes.d, swing, band) {
                    Ok(CylinderGap::NotTangent { apart, predicate }) => Some((apart, predicate)),
                    Ok(CylinderGap::External(_) | CylinderGap::Internal(_)) | Err(_) => None,
                },
            ));
        }
    };
    let &ParallelAxes {
        foot1,
        d_vec: w,
        d: dist,
    } = axes;
    match parallel_cylinder_gaps((r1, r2), dist, swing, band).map_err(escalate)? {
        CylinderGap::External(gap) => Ok(TangentLocus::Line {
            origin: foot1 + w.normalize() * r1,
            gap: gap.jointly(parallel),
            dir: a1,
        }),
        CylinderGap::NotTangent { apart, predicate } => {
            Err(TangentLocusError::NotTangent { apart, predicate })
        }
        CylinderGap::Internal(gap) => {
            // Internal tangency: the smaller cylinder rests inside the
            // larger; the generator sits on the offset direction at the
            // LARGER radius from the larger axis. With coaxial axes
            // (dist in the zero band AND radii in the zero band) the
            // locus direction is ill-posed — refuse typed.
            match decide("tangent_locus_side", Margin::of(dist), band).map_err(escalate)? {
                Sign::Positive => {}
                _ => {
                    return Err(TangentLocusError::Unsupported {
                        what: "coaxial equal-radius cylinders have no isolated tangent \
                               generator (conformal contact is Rest, not Tangent)",
                    });
                }
            }
            // Which cylinder contains which decides the generator's
            // side (derived: with ŵ = o1→o2 and |w| = |r1 − r2|, the
            // touch point is P = o1 + ŵ·r1 when r1 > r2 — c2 inside
            // c1 — and P = o1 − ŵ·r1 when r1 < r2, both from
            // collinearity of P, o1, o2 with |P−o1| = r1, |P−o2| = r2).
            // The side is DECIDED, never an evaluation-lane comparison.
            let sign =
                match decide("tangent_locus_side", Margin::of(r1 - r2), band).map_err(escalate)? {
                    Sign::Positive => T::one(),
                    Sign::Negative => T::zero() - T::one(),
                    Sign::Zero => {
                        return Err(TangentLocusError::Unsupported {
                            what: "equal-radius internal tangency contradicts the definite \
                               axis offset — no closed-form generator",
                        });
                    }
                };
            Ok(TangentLocus::Line {
                origin: foot1 + w.normalize() * (sign * r1),
                gap: gap.jointly(parallel),
                dir: a1,
            })
        }
    }
}

/// What a closed-form pair's cross-section is read from, kept until a
/// read escalates ([`Pending::section`]).
enum Pending<T: Real> {
    /// The plane×cylinder arm on its frame.
    PlaneCylinder {
        pc: PlaneCylinder<T>,
        frame: RuledFrame<T>,
        plane_first: bool,
    },
    /// The parallel-cylinder arm at its feet.
    Cylinders {
        radii: (T, T),
        dirs: (Vec3<T>, Vec3<T>),
        axes: ParallelAxes<T>,
    },
}

impl<T: Decide> Pending<T> {
    /// The cross-section ([`Section`]), `None` where its direction does
    /// not decide.
    fn section(&self, reach: &Reach<T>, band: Band) -> Option<Section<T>> {
        match self {
            Self::PlaneCylinder {
                pc,
                frame,
                plane_first,
            } => Section::plane_cylinder(pc, frame, *plane_first, reach, band),
            Self::Cylinders { radii, dirs, axes } => {
                Section::cylinders(*radii, *dirs, axes, reach, band)
            }
        }
    }
}

/// **A closed-form pair's cross-section across its locus**, where a
/// face that does not reach the locus is read against the other
/// carrier. `across` is the unit direction perpendicular to the locus
/// in the plane both carriers are tangent to, and `at` the locus's
/// coordinate along it about the world origin.
///
/// Each carrier is a line (a plane) or a circle (a cylinder) in every
/// section perpendicular to the first cylinder's axis, the plane's
/// axis in the plane×cylinder arm; `slack` bounds how far the second
/// carrier stands, across the reach, from its section at the feet (the
/// tilt the axis row allows, levered across the reach). A point of a
/// face whose coordinate along `across` stands `s` or more off `at` is
/// then at least [`Section::clearance`] off the other carrier, so a
/// face whose padded extent stands there lies definitely clear of the
/// other face.
struct Section<T: Real> {
    across: UnitVec3<T>,
    at: T,
    walls: Walls<T>,
}

/// The two carriers in the section, in the pair's order.
enum Walls<T: Real> {
    /// A plane and a cylinder of radius `r` whose axis stands at least
    /// `h` off the plane across the reach; `plane_first` where the
    /// plane is the pair's first.
    PlaneCylinder { plane_first: bool, h: T, r: T },
    /// Two cylinders of radii `radii` whose axes stand `d` apart at the
    /// feet, the second within `slack` of its section there.
    Cylinders { radii: (T, T), d: T, slack: T },
}

impl<T: Decide> Section<T> {
    /// The plane×cylinder section on `frame`: `across` is the axis
    /// crossed with the normal, which the plane holds and which is
    /// perpendicular to the axis, and the plane's height off the axis
    /// moves by the axis' normal component times the farthest the
    /// reach stands from the foot. `None` where `across` does not
    /// decide a direction.
    fn plane_cylinder(
        pc: &PlaneCylinder<T>,
        frame: &RuledFrame<T>,
        plane_first: bool,
        reach: &Reach<T>,
        band: Band,
    ) -> Option<Self> {
        // `r·|a × n|`: the wall's reach across the locus, a length.
        let across = UnitVec3::new(pc.a.cross(pc.n) * pc.r, ACROSS, band).ok()?;
        let tilt = frame.c.abs() * reach.lever_from(frame.o);
        Some(Self {
            at: across.get().dot(frame.o - Point3::origin()),
            across,
            walls: Walls::PlaneCylinder {
                plane_first,
                h: frame.gap_signed.abs() - tilt,
                r: pc.r,
            },
        })
    }

    /// The parallel-cylinder section at the feet (`axes`): `across` is
    /// the first axis crossed with the offset between the feet, and the
    /// second axis, read in the first's direction through its foot,
    /// moves by the two directions' difference times the farthest the
    /// reach stands from that foot. `None` where `across` does not
    /// decide a direction (the axes in band of coaxial).
    fn cylinders(
        radii: (T, T),
        (a1, a2): (Vec3<T>, Vec3<T>),
        axes: &ParallelAxes<T>,
        reach: &Reach<T>,
        band: Band,
    ) -> Option<Self> {
        // `|a1 × d_vec| = |d_vec|`, the axes' offset, a length.
        let across = UnitVec3::new(a1.cross(axes.d_vec), ACROSS, band).ok()?;
        let foot2 = axes.foot1 + axes.d_vec;
        let turn = (a1 - a2).norm().min((a1 + a2).norm());
        Some(Self {
            at: across.get().dot(axes.foot1 - Point3::origin()),
            across,
            walls: Walls::Cylinders {
                radii,
                d: axes.d_vec.norm(),
                slack: turn * reach.lever_from(foot2),
            },
        })
    }

    /// The least distance from a point of face `i`'s carrier that
    /// stands `s` or more off the locus across it to the other carrier.
    /// A point of a wall of radius `ri` at angle `φ` from the locus
    /// stands `ri·|sin φ|` across, so `|cos φ| ≤ k`; its distance to
    /// the other circle's centre, `D` from its own, then lies between
    /// `√(ri² + D² ∓ 2·ri·D·k)`.
    fn clearance(&self, i: usize, s: T) -> T {
        let zero = T::zero();
        let s = s.max(zero);
        let room = |s: T, r: T| (T::one() - (s / r).powi(2)).max(zero).sqrt();
        match self.walls {
            Walls::PlaneCylinder { plane_first, h, r } => {
                if (i == 0) == plane_first {
                    // A plane point `s` across stands `√(s² + h²)` or
                    // more off the axis.
                    (s.powi(2) + h.max(zero).powi(2)).sqrt() - r
                } else {
                    h - r * room(s, r)
                }
            }
            Walls::Cylinders { radii, d, slack } => {
                let (ri, rj) = if i == 0 { radii } else { (radii.1, radii.0) };
                let k = room((s - slack).max(zero), ri);
                let base = ri.powi(2) + d.powi(2);
                let cross = (ri + ri) * d * k;
                let near = (base - cross).max(zero).sqrt();
                let far = (base + cross).sqrt();
                (near - rj).max(rj - far) - slack
            }
        }
    }

    /// Whether some face's extent across the locus (`extents`, read
    /// along `across`) stands far enough off it to lie definitely clear
    /// of the other carrier, decided on `tangent_locus_face_clear`.
    fn faces_clear(
        &self,
        extents: impl FnOnce(UnitVec3<T>) -> [Option<(T, T)>; 2],
        band: Band,
    ) -> bool {
        extents(self.across)
            .into_iter()
            .enumerate()
            .any(|(i, extent)| {
                extent.is_some_and(|(lo, hi)| {
                    let s = (lo - self.at).max(self.at - hi);
                    matches!(
                        decide(FACE_CLEAR, Margin::of(self.clearance(i, s)), band),
                        Ok(Sign::Positive)
                    )
                })
            })
    }
}

/// The site the cross-section's direction is decided under.
const ACROSS: &str = "tangent_locus_across";

/// **Definite counter-evidence answers before an in-band parallelism.**
/// Each closed-form tangency needs its axis row Zero AND its gap row
/// Zero, so where the axis row is in band (`diag`, its run `axis_log`)
/// and the gap row, read across the reach with the tilt beside it,
/// still decides a crossing or a clearance (`refute`'s `(apart,
/// predicate)`), the pair is definitely not tangent and the axis row's
/// escalation is superseded. Otherwise the escalation stands, and the
/// gap reading it rests on nothing is dropped.
fn definite_over_in_band(
    diag: Indeterminate,
    axis_log: geom_core::k_stats::Detached,
    refute: impl FnOnce() -> Option<(bool, &'static str)>,
) -> TangentLocusError {
    let (refuted, gap_log) = geom_core::k_stats::detached(refute);
    match refuted {
        Some((apart, predicate)) => {
            geom_core::k_stats::splice_superseded(axis_log);
            geom_core::k_stats::splice(gap_log);
            TangentLocusError::NotTangent { apart, predicate }
        }
        None => {
            geom_core::k_stats::splice(axis_log);
            drop(gap_log);
            TangentLocusError::Escalated(diag)
        }
    }
}

/// What a parallel cylinder pair's gap rows read.
enum CylinderGap {
    /// `r1 + r2 − d` Zero: external tangency, with its margin.
    External(MarginDiag),
    /// `|r1 − r2| − d` Zero: internal tangency, with its margin.
    Internal(MarginDiag),
    /// A definite clearance or crossing, with the row that decided it.
    NotTangent {
        apart: bool,
        predicate: &'static str,
    },
}

/// **The gap rows of two cylinders whose axes stand `d` apart**, each
/// read across the reach with the axes' `swing` beside it: external
/// tangency first (`|w| = r1 + r2`, the common case and the flush
/// detector's), internal (`|w| = |r1 − r2|`) second. Fixed probe order
/// (D9).
fn parallel_cylinder_gaps<T: Decide>(
    radii: (T, T),
    d: T,
    swing: T,
    band: Band,
) -> Result<CylinderGap, Indeterminate> {
    match parallel_cylinder_gap_reported(radii, d, swing, band)? {
        (Sign::Zero, gap) => return Ok(CylinderGap::External(gap)),
        (Sign::Negative, _) => {
            return Ok(CylinderGap::NotTangent {
                apart: true,
                predicate: "cc_parallel_gap",
            });
        }
        (Sign::Positive, _) => {}
    }
    let internal = "tangent_locus_internal_gap";
    Ok(
        match decide_across_reported(
            [internal, "tangent_locus_internal_gap_floor"],
            (radii.0 - radii.1).abs() - d,
            swing,
            band,
        )? {
            (Sign::Zero, gap) => CylinderGap::Internal(gap),
            // d < |r1 − r2|: one cylinder nested strictly inside the
            // other, their least distance definitely positive.
            (Sign::Positive, _) => CylinderGap::NotTangent {
                apart: true,
                predicate: internal,
            },
            // |r1 − r2| < d < r1 + r2: the surfaces cross.
            (Sign::Negative, _) => CylinderGap::NotTangent {
                apart: false,
                predicate: internal,
            },
        },
    )
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod tests {
    use super::*;
    use geom_core::Tol;

    /// **The kinds the lane holds are the kinds its dispatch reads.** A
    /// pair outside [`tangent_locus_kinds`] is `Unsupported`, and a pair
    /// inside it is not. Red when either side moves alone.
    #[test]
    fn the_lane_kinds_are_the_dispatchs() {
        let band = Band::linear(Tol::witness()).unwrap();
        let plane = Surface::Plane {
            origin: Point3::new(0.0, 0.0, -1.0),
            normal: Vec3::unit_z(),
            u_ref: Vec3::unit_x(),
        };
        let cyl = Surface::Cylinder {
            origin: Point3::origin(),
            axis: Vec3::unit_x(),
            radius: 1.0,
            u_ref: Vec3::unit_z(),
        };
        let beside = Surface::Cylinder {
            origin: Point3::new(0.0, 0.0, 2.0),
            axis: Vec3::unit_x(),
            radius: 1.0,
            u_ref: Vec3::unit_z(),
        };
        let sphere = Surface::Sphere {
            center: Point3::origin(),
            radius: 1.0,
            axis: Vec3::unit_z(),
            u_ref: Vec3::unit_x(),
        };
        let metre = ExtentBall::new(Point3::origin(), 1.0);
        for (a, b) in [
            (&plane, &cyl),
            (&cyl, &plane),
            (&cyl, &beside),
            (&plane, &plane),
            (&plane, &sphere),
            (&sphere, &cyl),
        ] {
            let unsupported = matches!(
                tangent_locus(a, b, metre, band),
                Err(TangentLocusError::Unsupported { .. })
            );
            assert_eq!(
                tangent_locus_kinds(a.kind(), b.kind()),
                !unsupported,
                "{:?} x {:?}",
                a.kind(),
                b.kind()
            );
        }
    }

    /// **The axis row is levered over the consumed extent.** A unit
    /// cylinder resting on `z = 0` at the origin, its axis rising
    /// `0.3·ε` per metre along `x`: over a 1 m patch about the origin
    /// (a 2 m lever from the axis) the tilt reads zero and the ruling
    /// is minted; over a 10 m patch along `x` the lever is 6 m, the tilt
    /// reads in band, and the row escalates rather than mint a ruling
    /// the far end stands `3·ε` off.
    #[test]
    fn the_axis_row_reads_the_tilt_across_the_extent() {
        let band = Band::linear(Tol::witness()).unwrap();
        let theta: f64 = 0.3 * band.zero();
        let plane = Surface::Plane {
            origin: Point3::origin(),
            normal: Vec3::unit_z(),
            u_ref: Vec3::unit_x(),
        };
        let cyl = Surface::Cylinder {
            origin: Point3::new(0.0, 0.0, 1.0),
            axis: Vec3::new(theta.cos(), 0.0, theta.sin()),
            radius: 1.0,
            u_ref: Vec3::unit_z(),
        };
        let metre = ExtentBall::new(Point3::origin(), 1.0);
        match tangent_locus(&plane, &cyl, metre, band) {
            Ok(TangentLocus::Line { .. }) => {}
            other => panic!("over a metre the tilt reads zero and the ruling is minted: {other:?}"),
        }
        let ten = ExtentBall::new(Point3::new(5.0, 0.0, 0.0), 5.0);
        match tangent_locus(&plane, &cyl, ten, band) {
            Err(TangentLocusError::Escalated(d)) => assert_eq!(
                d.predicate,
                Some("pc_axis_plane_parallel"),
                "the axis row escalates"
            ),
            other => panic!("over 10 m the tilt reads in band and escalates: {other:?}"),
        }
    }

    fn x_cylinder(y: f64, z: f64, radius: f64) -> Surface<f64> {
        Surface::Cylinder {
            origin: Point3::new(0.0, y, z),
            axis: Vec3::unit_x(),
            radius,
            u_ref: Vec3::unit_z(),
        }
    }

    /// **A plane through a cylinder's axis, the axis in band of it,
    /// crosses.** A unit cylinder along `x` whose axis stands
    /// `√(zero·escalate)` above `z = 0`: the plane cuts it along two
    /// rulings a metre from the axis, whichever side the axis lies on,
    /// so the section's `pc_parallel_gap` decides the crossing. The
    /// side of the axis is not a question the tangency asks.
    #[test]
    fn an_axis_in_band_of_the_plane_crosses_rather_than_escalates() {
        let band = Band::linear(Tol::witness()).unwrap();
        let plane = Surface::Plane {
            origin: Point3::origin(),
            normal: Vec3::unit_z(),
            u_ref: Vec3::unit_x(),
        };
        let cyl = x_cylinder(0.0, (band.zero() * band.escalate()).sqrt(), 1.0);
        let metre = ExtentBall::new(Point3::origin(), 1.0);
        match tangent_locus(&plane, &cyl, metre, band) {
            Err(TangentLocusError::NotTangent {
                apart: false,
                predicate: "pc_parallel_gap",
            }) => {}
            other => panic!("the plane definitely cuts the cylinder: {other:?}"),
        }
    }

    /// **On exactly parallel axes the external gap reads both radii, in
    /// either order.** Radii `1` and `1 − 0.9·zero` (equal within the
    /// band, so the declaration verifies), axes `r1 + r2 − 0.6·zero`
    /// apart: the walls overlap by `0.6·zero`, inside the zero band. The
    /// gap is `r1 + r2 − d` whichever cylinder comes first; one radius
    /// doubled would put `1.5·zero` on one order (in band, escalating)
    /// and `−0.3·zero` on the other (tangent). The axes are parallel
    /// exactly, so no tilt pivot enters the reading.
    #[test]
    fn on_exact_parallels_the_external_gap_reads_both_radii_in_either_order() {
        use crate::intersect::{EqualCylinderSection, cylinder_cylinder_section};
        let band = Band::linear(Tol::witness()).unwrap();
        let zero = band.zero();
        let (r1, r2) = (1.0, 1.0 - 0.9 * zero);
        let c1 = x_cylinder(0.0, 0.0, r1);
        let c2 = x_cylinder(r1 + r2 - 0.6 * zero, 0.0, r2);
        let metre = ExtentBall::new(Point3::origin(), 1.0);
        for (label, a, b) in [("c1, c2", &c1, &c2), ("c2, c1", &c2, &c1)] {
            match tangent_locus(a, b, metre, band) {
                Ok(TangentLocus::Line { .. }) => {}
                other => panic!("{label}: the witness mints the ruling: {other:?}"),
            }
            match cylinder_cylinder_section(a, b, &metre, band) {
                Ok(EqualCylinderSection::TangentLine(_)) => {}
                other => panic!("{label}: the section classifies the same tangency: {other:?}"),
            }
        }
    }

    /// A unit cylinder whose axis stands at height 1 above `z = 0` at
    /// `x = 0`, rising `0.3·zero` per metre along `x`, its stored origin
    /// `back` metres behind that point along the axis.
    fn tilted_resting_cylinder(back: f64, zero: f64) -> Surface<f64> {
        let theta = 0.3 * zero;
        let axis = Vec3::new(theta.cos(), 0.0, theta.sin());
        Surface::Cylinder {
            origin: Point3::new(0.0, 0.0, 1.0) - axis * back,
            axis,
            radius: 1.0,
            u_ref: Vec3::unit_y(),
        }
    }

    /// **The gap is read where the extent is, not at the stored
    /// origin.** The resting, barely tilted cylinder of
    /// [`tilted_resting_cylinder`] stored 1000 m back along its axis:
    /// over a metre about the origin the tilt reads zero and the axis
    /// stands one radius off the plane, so the ruling mints. Read at the
    /// stored origin the axis would stand `300·zero` lower, a definite
    /// crossing.
    #[test]
    fn a_far_stored_origin_does_not_move_the_gap() {
        let band = Band::linear(Tol::witness()).unwrap();
        let plane = Surface::Plane {
            origin: Point3::origin(),
            normal: Vec3::unit_z(),
            u_ref: Vec3::unit_x(),
        };
        let cyl = tilted_resting_cylinder(1000.0, band.zero());
        let metre = ExtentBall::new(Point3::origin(), 1.0);
        match tangent_locus(&plane, &cyl, metre, band) {
            Ok(TangentLocus::Line { origin, .. }) => assert!(
                origin.x.abs() < 1.0 && origin.z.abs() < band.zero(),
                "the ruling is read beside the extent, on the plane: {origin:?}"
            ),
            other => panic!("the resting cylinder is tangent across the extent: {other:?}"),
        }
    }

    /// **The tilt is levered from the foot to the far side of the
    /// extent, not over its radius alone.** The same cylinder stored at
    /// its foot, with a 1 m extent centred 5 m off its axis along `y`:
    /// the extent reaches `√26 + 1 ≈ 6.1` m from the foot, so the
    /// `0.3·zero` tilt reads `1.83·zero`, in band. Over the extent's
    /// radius alone it would read `0.3·zero` and mint a ruling that
    /// stands nearly twice the zero band off across the faces.
    #[test]
    fn the_tilt_is_levered_from_the_foot_across_the_extent() {
        let band = Band::linear(Tol::witness()).unwrap();
        let plane = Surface::Plane {
            origin: Point3::origin(),
            normal: Vec3::unit_z(),
            u_ref: Vec3::unit_x(),
        };
        let cyl = tilted_resting_cylinder(0.0, band.zero());
        let off_axis = ExtentBall::new(Point3::new(0.0, 5.0, 0.0), 1.0);
        match tangent_locus(&plane, &cyl, off_axis, band) {
            Err(TangentLocusError::Escalated(d)) => assert_eq!(
                d.predicate,
                Some("pc_axis_plane_parallel"),
                "the axis row escalates"
            ),
            other => panic!("the tilt levered across the extent reads in band: {other:?}"),
        }
    }

    /// **A definite gap answers before an in-band tilt.** The plane
    /// pair above over the 10 m patch, its axis tilt in band, with the
    /// axis lowered or raised half a metre: no tilt in band closes a
    /// half-metre crossing or clearance, so the pair is definitely not
    /// tangent, on the gap row. At the tangent height the gap reads
    /// zero and the axis row's escalation stands.
    #[test]
    fn a_definite_gap_refuses_a_plane_pair_whose_tilt_is_in_band() {
        let band = Band::linear(Tol::witness()).unwrap();
        let theta: f64 = 0.3 * band.zero();
        let plane = Surface::Plane {
            origin: Point3::origin(),
            normal: Vec3::unit_z(),
            u_ref: Vec3::unit_x(),
        };
        let cyl = |height: f64| Surface::Cylinder {
            origin: Point3::new(0.0, 0.0, height),
            axis: Vec3::new(theta.cos(), 0.0, theta.sin()),
            radius: 1.0,
            u_ref: Vec3::unit_z(),
        };
        let ten = ExtentBall::new(Point3::new(5.0, 0.0, 0.0), 5.0);
        for (height, want_apart, why) in [(0.5, false, "crossing"), (1.5, true, "clear")] {
            match tangent_locus(&plane, &cyl(height), ten, band) {
                Err(TangentLocusError::NotTangent { apart, predicate }) => {
                    assert_eq!(apart, want_apart, "{why}");
                    assert_eq!(predicate, "pc_parallel_gap", "{why}");
                }
                other => panic!("{why}: the gap refutes the tangency: {other:?}"),
            }
        }
        match tangent_locus(&plane, &cyl(1.0), ten, band) {
            Err(TangentLocusError::Escalated(d)) => {
                assert_eq!(d.predicate, Some("pc_axis_plane_parallel"));
            }
            other => panic!("at the tangent height the tilt escalates: {other:?}"),
        }
    }

    /// The same for two cylinders: unit cylinders along `x` over the
    /// 10 m patch, the second's axis tilted in band, their axes 3 m
    /// apart (clear of each other) or 1 m apart (crossing). At 2 m the
    /// external gap reads zero and the axis row's escalation stands.
    #[test]
    fn a_definite_gap_refuses_a_cylinder_pair_whose_tilt_is_in_band() {
        let band = Band::linear(Tol::witness()).unwrap();
        let theta: f64 = 0.3 * band.zero();
        let along = |y: f64, tilt: f64| Surface::Cylinder {
            origin: Point3::new(0.0, y, 0.0),
            axis: Vec3::new(tilt.cos(), 0.0, tilt.sin()),
            radius: 1.0,
            u_ref: Vec3::unit_z(),
        };
        let ten = ExtentBall::new(Point3::new(5.0, 0.0, 0.0), 5.0);
        for (y, want, why) in [
            (3.0, (true, "cc_parallel_gap"), "clear"),
            (1.0, (false, "tangent_locus_internal_gap"), "crossing"),
        ] {
            match tangent_locus(&along(0.0, 0.0), &along(y, theta), ten, band) {
                Err(TangentLocusError::NotTangent { apart, predicate }) => {
                    assert_eq!((apart, predicate), want, "{why}");
                }
                other => panic!("{why}: the gap refutes the tangency: {other:?}"),
            }
        }
        match tangent_locus(&along(0.0, 0.0), &along(2.0, theta), ten, band) {
            Err(TangentLocusError::Escalated(d)) => {
                assert_eq!(d.predicate, Some("cc_axes_parallel"));
            }
            other => panic!("at the tangent distance the tilt escalates: {other:?}"),
        }
    }

    /// **A face that does not reach the locus settles an in-band
    /// reading.** A unit cylinder along `x` over the plane `z = 0`, its
    /// axis `5.5·ε` above the tangent height, so the gap row is in band.
    /// The across direction is `±ŷ` and the ruling stands at `y = 0`. A
    /// plane face over `y ∈ [0.01, 0.5]` stands about `5·10⁻⁵` off the
    /// wall at its nearest edge: no tangency, on the face row. A face
    /// over `y ∈ [−0.25, 0.25]` reaches the ruling and the gap row's
    /// escalation stands; so does a pair whose extents do not read.
    #[test]
    fn a_face_clear_of_the_ruling_settles_an_in_band_gap() {
        let band = Band::linear(Tol::witness()).unwrap();
        let plane = Surface::Plane {
            origin: Point3::origin(),
            normal: Vec3::unit_z(),
            u_ref: Vec3::unit_x(),
        };
        let cyl = Surface::Cylinder {
            origin: Point3::new(0.0, 0.0, 1.0 + 5.5 * band.zero()),
            axis: Vec3::unit_x(),
            radius: 1.0,
            u_ref: Vec3::unit_z(),
        };
        let reach = ExtentBall::new(Point3::new(0.5, 0.0, 0.0), 1.5);
        let wall = (-1.0, 1.0);
        let across = |plane_face: (f64, f64)| {
            move |dir: UnitVec3<f64>| {
                // The across direction is ±ŷ; read each face's span on it.
                let flip = |(lo, hi): (f64, f64)| {
                    if dir.get().y > 0.0 {
                        (lo, hi)
                    } else {
                        (-hi, -lo)
                    }
                };
                [Some(flip(plane_face)), Some(flip(wall))]
            }
        };
        match tangent_locus_of_faces(&plane, &cyl, reach, band, across((0.01, 0.5))) {
            Err(TangentLocusError::NotTangent { apart, predicate }) => {
                assert!(apart, "clear");
                assert_eq!(predicate, "tangent_locus_face_clear");
            }
            other => panic!("the face off the ruling is no tangency: {other:?}"),
        }
        match tangent_locus_of_faces(&cyl, &plane, reach, band, |d| {
            let [p, w] = across((0.01, 0.5))(d);
            [w, p]
        }) {
            Err(TangentLocusError::NotTangent { predicate, .. }) => {
                assert_eq!(predicate, "tangent_locus_face_clear", "either order");
            }
            other => panic!("the face off the ruling, cylinder first: {other:?}"),
        }
        for (why, read) in [
            (
                "over the ruling",
                tangent_locus_of_faces(&plane, &cyl, reach, band, across((-0.25, 0.25))),
            ),
            (
                "unread",
                tangent_locus_of_faces(&plane, &cyl, reach, band, |_| [None, None]),
            ),
        ] {
            match read {
                Err(TangentLocusError::Escalated(d)) => {
                    assert_eq!(d.predicate, Some("pc_parallel_gap"), "{why}");
                }
                other => panic!("{why}: the gap row escalates: {other:?}"),
            }
        }
    }

    /// The same for parallel cylinders: unit cylinders along `x`, the
    /// second's axis at `y = 2 + 5.5·ε` (outside) or the second a rod of
    /// radius 0.2 at `y = 0.8 − 5.5·ε` (inside). Across is `±ẑ`, the
    /// ruling at `z = 0`; the second's face over `z ∈ [0.1, 0.2]`
    /// stands off it, and one over `z ∈ [−0.1, 0.1]` reaches it.
    #[test]
    fn a_cylinder_face_clear_of_the_ruling_settles_an_in_band_gap() {
        let band = Band::linear(Tol::witness()).unwrap();
        let g = 5.5 * band.zero();
        let along = |y: f64, r: f64| Surface::Cylinder {
            origin: Point3::new(0.0, y, 0.0),
            axis: Vec3::unit_x(),
            radius: r,
            u_ref: Vec3::unit_z(),
        };
        let reach = ExtentBall::new(Point3::new(0.5, 1.0, 0.0), 2.0);
        let across = |second: (f64, f64)| {
            move |dir: UnitVec3<f64>| {
                let flip = |(lo, hi): (f64, f64)| {
                    if dir.get().z > 0.0 {
                        (lo, hi)
                    } else {
                        (-hi, -lo)
                    }
                };
                [Some(flip((-1.0, 1.0))), Some(flip(second))]
            }
        };
        for (other, gap_row, why) in [
            (along(2.0 + g, 1.0), "cc_parallel_gap", "outside"),
            (along(0.8 - g, 0.2), "tangent_locus_internal_gap", "inside"),
        ] {
            let first = along(0.0, 1.0);
            match tangent_locus_of_faces(&first, &other, reach, band, across((0.1, 0.2))) {
                Err(TangentLocusError::NotTangent { apart, predicate }) => {
                    assert!(apart, "{why}");
                    assert_eq!(predicate, "tangent_locus_face_clear", "{why}");
                }
                other => panic!("{why}: the face off the ruling is no tangency: {other:?}"),
            }
            match tangent_locus_of_faces(&first, &other, reach, band, across((-0.1, 0.1))) {
                Err(TangentLocusError::Escalated(d)) => {
                    assert_eq!(d.predicate, Some(gap_row), "{why}");
                }
                other => panic!("{why}: over the ruling the gap row escalates: {other:?}"),
            }
        }
    }

    /// **The face-clear bound at the band's edge.** Each pair's gap
    /// row is in band (`5.5·ε`), and one face's extent across the locus
    /// starts `s` off it, where the true clearance is the gap plus the
    /// two walls' bend, `s²/2` per unit curvature summed. At an `s`
    /// whose clearance stands inside the band the escalation stands; at
    /// one a few band widths clear, the face is clear. A bound loose on
    /// the unsound side by as little as `s` reads the first clear.
    #[test]
    fn the_face_clear_bound_holds_at_the_band_edge() {
        let band = Band::linear(Tol::witness()).unwrap();
        let (eps, g) = (band.zero(), 5.5 * band.zero());
        let plane = Surface::Plane {
            origin: Point3::origin(),
            normal: Vec3::unit_z(),
            u_ref: Vec3::unit_x(),
        };
        let along = |origin: Point3<f64>, r: f64| Surface::Cylinder {
            origin,
            axis: Vec3::unit_x(),
            radius: r,
            u_ref: Vec3::unit_z(),
        };
        let reach = ExtentBall::new(Point3::new(0.5, 1.0, 0.0), 2.0);
        // `across` is ±ŷ for the plane pair and ±ẑ for the cylinders;
        // `which` is the face whose extent starts `s` off the ruling, the
        // other spanning it.
        let read = |a: &Surface<f64>, b: &Surface<f64>, z_across: bool, which: usize, s: f64| {
            tangent_locus_of_faces(a, b, reach, band, move |dir: UnitVec3<f64>| {
                let positive = if z_across { dir.get().z } else { dir.get().y } > 0.0;
                let span = |(lo, hi): (f64, f64)| if positive { (lo, hi) } else { (-hi, -lo) };
                let mut out = [Some(span((-1.0, 1.0))), Some(span((-1.0, 1.0)))];
                out[which] = Some(span((s, 0.5)));
                out
            })
        };
        let lifted = along(Point3::new(0.0, 0.0, 1.0 + g), 1.0);
        let unit = along(Point3::origin(), 1.0);
        let outside = along(Point3::new(0.0, 2.0 + g, 0.0), 1.0);
        let inside = along(Point3::new(0.0, 0.8 - g, 0.0), 0.2);
        // (pair, across ẑ, face, curvature sum): the clearance at `s` is
        // `g + κ·s²/2`.
        let rows = [
            ("plane face", &plane, &lifted, false, 0, 1.0),
            ("wall face over a plane", &plane, &lifted, false, 1, 1.0),
            ("outside wall", &unit, &outside, true, 1, 2.0),
            ("inside rod", &unit, &inside, true, 1, 4.0),
        ];
        for (why, a, b, z_across, which, kappa) in rows {
            let s_at = |clearance: f64| (2.0 * (clearance - g) / kappa).sqrt();
            match read(a, b, z_across, which, s_at(7.5 * eps)) {
                Err(TangentLocusError::Escalated(_)) => {}
                other => panic!("{why}: a face in band of the wall escalates: {other:?}"),
            }
            match read(a, b, z_across, which, s_at(25.5 * eps)) {
                Err(TangentLocusError::NotTangent { predicate, .. }) => {
                    assert_eq!(predicate, "tangent_locus_face_clear", "{why}");
                }
                other => panic!("{why}: a face clear of the wall is no tangency: {other:?}"),
            }
        }
    }

    /// **An escalation the answer does not rest on stays off the log.**
    /// Near-coaxial cylinders (radii 1 and 0.5, axes `3·ε` apart) are
    /// definitely nested, so no cross-section is read; and where one
    /// face reads clear, the other face's in-band clearance is spliced
    /// superseded.
    #[test]
    fn a_definite_answer_leaves_no_escalation_on_the_log() {
        let band = Band::linear(Tol::witness()).unwrap();
        let reach = ExtentBall::new(Point3::new(0.5, 0.0, 0.0), 1.5);
        let along = |origin: Point3<f64>, r: f64| Surface::Cylinder {
            origin,
            axis: Vec3::unit_x(),
            radius: r,
            u_ref: Vec3::unit_z(),
        };
        let bracket = geom_core::k_stats::Bracket::open();
        let out = tangent_locus(
            &along(Point3::origin(), 1.0),
            &along(Point3::new(0.0, 3.0 * band.zero(), 0.0), 0.5),
            reach,
            band,
        );
        let log = bracket.finish();
        assert!(
            matches!(out, Err(TangentLocusError::NotTangent { .. })),
            "{out:?}"
        );
        assert!(log.escalations.is_empty(), "nested: {:?}", log.escalations);

        let plane = Surface::Plane {
            origin: Point3::origin(),
            normal: Vec3::unit_z(),
            u_ref: Vec3::unit_x(),
        };
        let lifted = along(Point3::new(0.0, 0.0, 1.0 + 5.5 * band.zero()), 1.0);
        let flip = |dir: UnitVec3<f64>, (lo, hi): (f64, f64)| {
            if dir.get().y > 0.0 {
                (lo, hi)
            } else {
                (-hi, -lo)
            }
        };
        let bracket = geom_core::k_stats::Bracket::open();
        let out = tangent_locus_of_faces(&plane, &lifted, reach, band, |d| {
            [Some(flip(d, (1e-6, 0.5))), Some(flip(d, (0.5, 1.0)))]
        });
        let log = bracket.finish();
        assert!(
            matches!(out, Err(TangentLocusError::NotTangent { .. })),
            "{out:?}"
        );
        assert!(
            log.escalations.is_empty(),
            "one face clear: {:?}",
            log.escalations
        );
    }
}
