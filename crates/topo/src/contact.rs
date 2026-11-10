//! **The contact vocabulary** (CONTACT-DESIGN C3/C4) — the ONE place
//! the kernel names what a glued or recorded contact is, what a
//! verification decided, and what recourse a refusal offers.
//!
//! The vocabulary lives HERE, at the lowest crate that can hold it,
//! because the census's `UnrecordedContact`/`ContactContradicted`
//! findings carry the class the pair was
//! read as (SELECT-DESIGN §3d, "one vocabulary end-to-end"), and the
//! recipe layer's flush detector mints findings in it.
//!
//! Upward layers RE-EXPORT these types; a parallel enum anywhere is
//! the bug this module exists to make impossible.

use geom_core::Indeterminate;

use crate::entity::FaceKey;

/// **What a boolean glues a cross-operand face pair as** (C4's
/// continuation clause and its claim-free `Tangent`): the class its
/// decided relation makes it ([`crate::Relation::glued_class`]).
/// Public as what a refusal's read of a glued pair names
/// ([`crate::DeclarationRead`]); nothing takes it as an input.
///
/// - [`Rest`](Self::Rest) — the pair lies on ONE carrier with its senses
///   OPPOSED: a conformal contact.
/// - [`Continuation`](Self::Continuation) — the pair lies on ONE
///   carrier with its senses ALIGNED: one surface carried on, whether
///   the two abut along a boundary curve or overlap on a patch; a union
///   merges it.
/// - [`Tangent`](Self::Tangent) — the pair lies on DISTINCT carriers
///   tangent along a locus the witness lane certifies. It says nothing
///   of the materials: whether the pair is a contact, an overlap, a seam
///   or a nested touch is read where it is consumed (C7).
///
/// It is also the class a contact pair names ([`ContactPair`]'s
/// `class`), which the façade's curated lists carry.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum GlueClass {
    /// One carrier, senses opposed.
    Rest,
    /// One carrier, senses aligned.
    Continuation,
    /// Distinct carriers tangent along a certified locus.
    Tangent,
}

impl GlueClass {
    /// Every class.
    pub(crate) const ALL: &'static [GlueClass] = &[Self::Rest, Self::Continuation, Self::Tangent];

    /// The class's name, for messages.
    pub fn name(self) -> &'static str {
        match self {
            Self::Rest => "Rest",
            Self::Continuation => "Continuation",
            Self::Tangent => "Tangent",
        }
    }

    /// Whether the class is ONE carrier (`Rest` or a continuation) —
    /// what the classification stages' same-carrier treatment and the
    /// merge stage consume. A `Tangent` pair's carriers are distinct.
    pub fn is_one_carrier(self) -> bool {
        matches!(self, Self::Rest | Self::Continuation)
    }

    /// **The relation that glues a face pair under this class**: the one
    /// table between classes and relations. [`crate::Relation::glued_class`]
    /// and [`Self::carrier_relation`] read it.
    #[must_use]
    pub const fn relation(self) -> crate::Relation {
        match self {
            Self::Rest => crate::Relation::SameOpposite,
            Self::Continuation => crate::Relation::SameOriented,
            Self::Tangent => crate::Relation::SurfaceTangent,
        }
    }

    /// The carrier relation a one-carrier class glues under: its
    /// [`Self::relation`] as the carrier ladder spells it. `None` for a
    /// tangency, whose carriers are distinct.
    pub(crate) const fn carrier_relation(self) -> Option<crate::boolean::CarrierRelation> {
        match self.relation() {
            crate::Relation::SameOriented => Some(crate::boolean::CarrierRelation::SameOriented),
            crate::Relation::SameOpposite => Some(crate::boolean::CarrierRelation::SameOpposite),
            _ => None,
        }
    }
}

/// **The recourse for a contact refusal**: move the geometry so the
/// faces clearly touch or clearly part.
///
/// This is deliberately NOT `geom_core::COINCIDENCE_RECOURSE`, whose
/// "lower the tolerance" arm does not reach a contact: the glue door
/// glues a pair whose margin decides Zero and refuses one in band under
/// every setting (D10), so a contact that is too close to call is
/// settled only by geometry that is not.
pub const CONTACT_RECOURSE: &str = "move the geometry so the faces clearly touch or clearly part";

/// [`CONTACT_RECOURSE`] with the recourse marker, as a finding the
/// viewer shows ends on it (`contact_recourse_is_one_sentence` holds
/// the two spellings to one).
pub(crate) const CONTACT_RECOURSE_MARKED: &str =
    "Recourse: move the geometry so the faces clearly touch or clearly part";

/// **A contact verification's typed refusal**.
///
/// `Clone` and `PartialEq` because the refusal is CARRIED: the census
/// wraps it in
/// [`ValidationError::CensusUnsupported`](crate::ValidationError::CensusUnsupported)'s
/// cause, and that error is `Clone + PartialEq` so a consumer can hold
/// and compare a whole report. No `Eq` — the diagnostics carry `f64`
/// margins, which is why `ValidationError` has none either.
#[derive(Clone, Debug, PartialEq)]
// The variant roster the sample-coverage row reads (test builds only).
#[cfg_attr(
    test,
    derive(strum::EnumDiscriminants),
    strum_discriminants(name(ContactRefusalKind), vis(pub(crate)), derive(strum::EnumIter))
)]
pub enum ContactRefusal {
    /// In-band geometry, or an escalation inside a definite condition:
    /// terminal, priced, honest.
    Escalated {
        /// The escalating predicate's diagnostics.
        diag: Indeterminate,
    },
    /// The configuration is outside the class's certifiable set — the
    /// demanded set IS the certifiable set (C3's order-k boundary),
    /// so this refuses typed rather than sampling. Carries what was
    /// asked for, never a fallback verdict.
    NotCertifiable {
        /// Why the configuration is outside the lane.
        what: &'static str,
    },
}

// The human-readable rendering (LIB-DOORS F6 shape): each arm states
// the PROBLEM in the contact vocabulary — in-band residue, the
// certifiable set — plus the
// recourse where an author has one.
//
// The diagnostics render through `Indeterminate::payload`, never the
// bare `Indeterminate` Display: the bare one ends in
// `COINCIDENCE_RECOURSE`, whose "lower the tolerance" arm is wrong at
// a contact site, and these arms supply [`CONTACT_RECOURSE`]
// themselves. That is the same composition the tier-3′ census and the
// boolean's own refusals make, so one contact story is told in one
// sentence shape wherever it surfaces.
//
// [`Self::NotCertifiable`] carries no recourse: moving the geometry
// within the band does not move a configuration inside the certifiable
// set, and `what` is the only honest steering there is.
impl core::fmt::Display for ContactRefusal {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Escalated { diag } => write!(
                f,
                "whether the faces touch escalated ({}); {CONTACT_RECOURSE}",
                diag.payload()
            ),
            Self::NotCertifiable { what } => {
                write!(f, "this contact cannot be certified ({what})")
            }
        }
    }
}

impl std::error::Error for ContactRefusal {}

/// **The row a `Tangent` reading ruled a pair out on**: one sample of
/// the C4 `Tangent` table read a definite "no" there. A closed type, so a
/// reader matches the row, never its name ([`Self::predicate`]).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
// The variant roster the sample-coverage row reads (test builds only).
#[cfg_attr(
    test,
    derive(strum::EnumDiscriminants),
    strum_discriminants(name(TangentRowKind), vis(pub(crate)), derive(strum::EnumIter))
)]
pub enum TangentRow {
    /// The outward normals agree at a tangent sample: under a contact's
    /// claim, containment (the C1 lemma).
    SensesAligned,
    /// The normals are definitely perpendicular.
    Independent,
    /// The normals are definitely not parallel at the derived angle.
    Parallel,
    /// The two materials both hold the crescent between the sheets.
    Overlap,
    /// The surfaces stand definitely off each other along their shared
    /// normal; the reading's sign says which side, in the orientation the
    /// sample's senses decided.
    Gap,
    /// The witness lies definitely off this surface where the pair
    /// touches.
    Off(geom_brep::PairSide),
}

impl TangentRow {
    /// The row's name, as its reading was logged and as `Debug` shows it.
    #[must_use]
    pub const fn predicate(self) -> &'static str {
        match self {
            Self::SensesAligned => "contact_tangent_opposed",
            Self::Independent => "contact_tangent_independent",
            Self::Parallel => "contact_tangent_parallel",
            Self::Overlap => "contact_tangent_overlap",
            Self::Gap => "contact_tangent_gap",
            Self::Off(geom_brep::PairSide::First) => "contact_tangent_off_1",
            Self::Off(geom_brep::PairSide::Second) => "contact_tangent_off_2",
        }
    }
}

/// **A definite "no" of the C4 `Tangent` table**: a sample's row, or two
/// tangent samples whose senses disagree.
#[derive(Clone, Copy, Debug, PartialEq)]
// The variant roster the sample-coverage row reads (test builds only).
#[cfg_attr(
    test,
    derive(strum::EnumDiscriminants),
    strum_discriminants(name(TangentRulingKind), vis(pub(crate)), derive(strum::EnumIter))
)]
pub enum TangentRuling {
    /// Ruled out at a sample, on `row`.
    Row {
        /// The row.
        row: TangentRow,
        /// The reading that decided it.
        by: geom_core::Decided,
    },
    /// Two samples each read tangent, the outward normals agreeing at one
    /// and opposing at the other: a definite crossing between them.
    ///
    /// Along a curve that is a line of curvature of both surfaces (a
    /// ruling of the lane's straight kinds, a latitude circle of two
    /// coaxial surfaces of revolution) `n₁·n₂` is constant (Rodrigues),
    /// so no such locus reaches it. A Villarceau circle two tori share is
    /// a line of curvature of neither: on a torus and its mirror image in
    /// the bitangent plane they share one in, the pair is tangent at four
    /// isolated points, two aligned and two opposed, and a nine-station
    /// schedule over the whole circle ends at its first station off them,
    /// not parallel. A span whose step runs from an opposed point to an
    /// aligned one reads both tangent, and is this
    /// (`contact_verify::tests::two_tangent_samples_with_opposite_senses_are_mixed_sides`).
    SidesMixed {
        /// The two readings of `n₁·n₂`, the earlier sample's first.
        at: [geom_core::Decided; 2],
    },
}

impl TangentRuling {
    /// What the ruling found, as a clause.
    #[must_use]
    pub const fn fact(self) -> &'static str {
        match self {
            Self::Row { row, .. } => match row {
                TangentRow::SensesAligned => "the two faces' outward normals agree",
                TangentRow::Independent => "the two faces' normals are perpendicular",
                TangentRow::Parallel => "the two faces' normals are not parallel",
                TangentRow::Overlap => "both faces' materials hold the crescent between them",
                TangentRow::Gap => "the two faces stand off each other along their shared normal",
                TangentRow::Off(_) => "the line runs off a face's carrier",
            },
            Self::SidesMixed { .. } => {
                "the two faces' outward normals agree at one point of the line and oppose at \
                 another"
            }
        }
    }
}

/// Why a `Tangent` reading is outside what the jet certificate reaches.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Uncertified {
    /// The (carrier kind, surface-kind pair) triple is outside the
    /// certificate's span-bound lane.
    OutsideLane,
    /// The schedule's span bounds do not exist for the configuration.
    NoSpanBounds,
    /// The pair is in the lane and every sample decided, and a span bound
    /// between samples stands beyond the band on the finest schedule.
    SagUnreached,
}

impl Uncertified {
    /// Why, as a clause.
    #[must_use]
    pub const fn what(self) -> &'static str {
        match self {
            Self::OutsideLane => {
                "the (carrier kind, surface-kind pair) triple is outside the jet \
                 certificate's span-bound lane (the order-k boundary)"
            }
            Self::NoSpanBounds => {
                "the jet schedule's span bounds are unavailable for this configuration"
            }
            Self::SagUnreached => {
                "the jet schedule's span bounds do not reach: every sample is decided, and a \
                 bound between them (the witness's or the touch's sag, or the drift of the \
                 relative curvature) stands beyond the band on the finest schedule"
            }
        }
    }
}

/// **What the C4 `Tangent` table read of a pair along a locus.**
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TangentVerdict {
    /// Tangent along the whole locus, on the margin the touch was
    /// decided on.
    Touch(geom_core::MarginDiag),
    /// Ruled out.
    Ruled(TangentRuling),
    /// A reading in band.
    InBand(Indeterminate),
    /// Outside what the certificate reaches.
    NotCertifiable(Uncertified),
}

/// **The row the `Rest` door ruled a pair out on.**
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RestRow {
    /// One carrier, its senses aligned: one surface carried on.
    SensesAligned,
    /// Two carriers.
    TwoCarriers,
}

/// **The `Rest` door's definite "no"**: its row, and the datum that
/// decided the carriers apart where one did (a sense and a kind decide
/// no datum).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RestRuling {
    /// The row.
    pub row: RestRow,
    /// The datum's reading.
    pub by: Option<geom_core::Decided>,
}

impl RestRuling {
    /// What the ruling found, as a clause.
    #[must_use]
    pub const fn fact(self) -> &'static str {
        match self.row {
            RestRow::SensesAligned => "the two faces lie on one carrier with their senses aligned",
            RestRow::TwoCarriers => "the two faces lie on two carriers",
        }
    }
}

/// **What the `Rest` door read of a face pair**: one carrier with the
/// senses opposed, or ruled out.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RestVerdict {
    /// One carrier, senses opposed, on the margin it was decided one on.
    Rest(geom_core::MarginDiag),
    /// Ruled out.
    Ruled(RestRuling),
}

/// **A reading that rules out what a face–face record claims** (C4 at
/// rest): the ruling of the door that read the record, as that door
/// returned it.
///
/// The façade's curated lists do not carry it, nor [`TangentRuling`],
/// which `ValidationError::MaterialOnBothSides` carries too: every
/// verdict ends the same way, the defect ending with no lever, and the
/// faces ride beside it in a type they do carry. The falsifier is a caller
/// that must act on which reading ruled the record out (a viewer drawing
/// the tangent line for a tangent ruling and both faces for a `Rest`
/// one); the carry is
/// `work/bind/contact-verdict-is-a-rung-under-contact-contradicted.md`.
#[derive(Clone, Copy, Debug, PartialEq)]
// The variant roster the sample-coverage row reads (test builds only).
#[cfg_attr(
    test,
    derive(strum::EnumDiscriminants),
    strum_discriminants(name(ContactVerdictKind), vis(pub(crate)), derive(strum::EnumIter))
)]
pub enum ContactVerdict {
    /// The `Rest` door's ruling.
    Rest(RestRuling),
    /// The two carriers derive no tangent line: they stand definitely
    /// apart (`apart`), or definitely cross, on the reading `by`.
    NotTangent {
        /// Apart, not crossing.
        apart: bool,
        /// The reading that decided it.
        by: geom_core::Decided,
    },
    /// The `Tangent` table's ruling along the line.
    Tangent(TangentRuling),
}

impl ContactVerdict {
    /// Where the verdict holds, read after "contradicted": the `Rest`
    /// door compares the two carriers whole, the others read the
    /// tangent line.
    #[must_use]
    pub const fn witness(self) -> &'static str {
        match self {
            Self::Rest(_) => "across the whole of both faces",
            Self::NotTangent { .. } | Self::Tangent(_) => "along their tangent line",
        }
    }

    /// What the verdict found, as a clause.
    #[must_use]
    pub const fn fact(self) -> &'static str {
        match self {
            Self::Rest(ruling) => ruling.fact(),
            Self::NotTangent { apart: true, .. } => "their carriers stand definitely apart",
            Self::NotTangent { apart: false, .. } => "their carriers definitely cross",
            Self::Tangent(ruling) => ruling.fact(),
        }
    }
}

/// **A face pair and the contact class it is read as**: what a record
/// names, what a finding reports, and what a refusal quotes back.
///
/// Key-based because it is the kernel's currency; the recipe layer's
/// name-based form carries the same class (G1: names cross the
/// boundary, keys never do).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ContactPair {
    /// The A-side face.
    pub a: FaceKey,
    /// The B-side face.
    pub b: FaceKey,
    /// The class the pair is read as.
    pub class: GlueClass,
}

/// **The kernel contact finding** — "this face pair would verify as
/// this class" (SELECT-DESIGN §3a: a VALUE, never itself a record). A
/// finding is only ever minted on a definite reading.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ContactFinding {
    /// What was found.
    pub pair: ContactPair,
}

/// **Whether a face–face record of `x` and `y` holds two pieces apart
/// over an area**: both faces planar. Planar faces a record holds apart
/// lie on one plane and meet over an area, since two planes meet along a
/// line only by crossing; a record of curved faces may be a tangency
/// along a line, which this does not tell from an area, and reads
/// `false`.
pub(crate) fn area_record<T: geom_core::Decide>(
    body: &crate::body::Body<T>,
    x: FaceKey,
    y: FaceKey,
) -> bool {
    let planar = |f| crate::face_normal::face_plane(body, f).is_some();
    planar(x) && planar(y)
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod tests {
    use super::*;

    /// Each class's relation glues back as that class.
    #[test]
    fn a_glue_class_is_its_relations_class() {
        for &class in GlueClass::ALL {
            assert_eq!(class.relation().glued_class(), Some(class), "{class:?}");
        }
    }

    /// The contact recourse moves the geometry and nothing else: the
    /// three-arm coincidence sentence must not leak in, since no
    /// tolerance moves a glue (D10), and nothing is declared.
    #[test]
    fn recourse_moves_the_geometry_and_has_no_tolerance_lever() {
        assert_eq!(
            CONTACT_RECOURSE_MARKED,
            format!("Recourse: {CONTACT_RECOURSE}")
        );
        assert!(CONTACT_RECOURSE.starts_with("move the geometry"));
        assert!(
            !CONTACT_RECOURSE.contains("tolerance") && !CONTACT_RECOURSE.contains("declare"),
            "a contact refusal has no tolerance arm and no declaration to make"
        );
        assert!(
            !geom_core::COINCIDENCE_RECOURSE.contains("faces clearly touch"),
            "the three-arm sentence stays the DECIDABILITY sentence"
        );
    }

    /// **Each verdict states its own fact, and where it holds.** A
    /// table: the `Rest` door's two rows, the tangent line read apart and
    /// crossing, and every tangent ruling, each to its sentence. The gap
    /// row's reading may mean apart, crossing or which sheet stands over
    /// the other (its senses' orientation), so its sentence reads no
    /// sign, and both signs say it.
    #[test]
    fn each_verdict_states_its_own_fact_and_witness() {
        let by = |sign| geom_core::Decided {
            sign,
            margin: geom_core::MarginDiag::value(3e-6),
        };
        let (pos, neg) = (by(geom_core::Sign::Positive), by(geom_core::Sign::Negative));
        let row = |row, by| ContactVerdict::Tangent(TangentRuling::Row { row, by });
        let line = "along their tangent line";
        let cases = [
            (
                ContactVerdict::Rest(RestRuling {
                    row: RestRow::SensesAligned,
                    by: None,
                }),
                "across the whole of both faces",
                "the two faces lie on one carrier with their senses aligned",
            ),
            (
                ContactVerdict::Rest(RestRuling {
                    row: RestRow::TwoCarriers,
                    by: Some(pos),
                }),
                "across the whole of both faces",
                "the two faces lie on two carriers",
            ),
            (
                ContactVerdict::NotTangent {
                    apart: true,
                    by: neg,
                },
                line,
                "their carriers stand definitely apart",
            ),
            (
                ContactVerdict::NotTangent {
                    apart: false,
                    by: pos,
                },
                line,
                "their carriers definitely cross",
            ),
            (
                row(TangentRow::SensesAligned, pos),
                line,
                "the two faces' outward normals agree",
            ),
            (
                row(TangentRow::Independent, by(geom_core::Sign::Zero)),
                line,
                "the two faces' normals are perpendicular",
            ),
            (
                row(TangentRow::Parallel, pos),
                line,
                "the two faces' normals are not parallel",
            ),
            (
                row(TangentRow::Overlap, pos),
                line,
                "both faces' materials hold the crescent between them",
            ),
            (
                row(TangentRow::Gap, pos),
                line,
                "the two faces stand off each other along their shared normal",
            ),
            (
                row(TangentRow::Gap, neg),
                line,
                "the two faces stand off each other along their shared normal",
            ),
            (
                row(TangentRow::Off(geom_brep::PairSide::Second), neg),
                line,
                "the line runs off a face's carrier",
            ),
            (
                ContactVerdict::Tangent(TangentRuling::SidesMixed { at: [neg, pos] }),
                line,
                "the two faces' outward normals agree at one point of the line and oppose at \
                 another",
            ),
        ];
        for (verdict, witness, fact) in cases {
            assert_eq!(verdict.witness(), witness, "{verdict:?}");
            assert_eq!(verdict.fact(), fact, "{verdict:?}");
        }
        let text = crate::ValidationError::ContactContradicted {
            record: ContactPair {
                a: FaceKey::default(),
                b: FaceKey::default(),
                class: GlueClass::Rest,
            },
            verdict: cases[0].0,
        }
        .to_string();
        assert_eq!(
            text,
            format!(
                "the recorded Rest contact is contradicted across the whole of both faces: the \
                 two faces lie on one carrier with their senses aligned. {}",
                geom_core::KERNEL_OR_FILE_DEFECT_ENDING
            )
        );
    }

    #[test]
    fn class_names_are_the_records_spelling() {
        assert_eq!(GlueClass::Continuation.name(), "Continuation");
        assert_eq!(GlueClass::Rest.name(), "Rest");
        assert_eq!(GlueClass::Tangent.name(), "Tangent");
    }
}
