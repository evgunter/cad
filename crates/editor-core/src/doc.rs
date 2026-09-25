//! `Doc` — the document as a PLAIN VALUE (spec D2, DESIGN.md D8: the
//! recipe is data): the recipe DAG plus document metadata. Cheap-clone
//! plain Rust (`Vec`/`BTreeMap`; no persistent-structure dependency —
//! document scale does not justify one; revisit only with corpus
//! latency data). All mutation goes through the pure
//! [`crate::edit::apply`]; undo/redo is keeping prior values.

use std::collections::BTreeMap;

use geom_core::Real;

use crate::appearance::{AppearanceMap, AppearanceRecord};
use crate::distribution::{Distribution, DistributionFault, DistributionField};
use crate::expr::{Dimension, Expr, ExprPath, ParamEnv, ParamValue};
use crate::ident::DocumentId;
use crate::names::StableName;
use crate::node::{Node, RecipeNodeId};
use geom_core::Tol;

/// A document-level parameter name (spec D4's "parameter refs").
#[derive(
    Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, serde::Serialize, serde::Deserialize,
)]
pub struct ParamName(pub String);

impl ParamName {
    /// Convenience constructor.
    pub fn new(name: impl Into<String>) -> Self {
        Self(name.into())
    }
}

/// The name, bare — one home for the spelling every refusal that FRAMES
/// a parameter name in a sentence of its own uses ("parameter width is
/// declared length").
///
/// A `{:?}` over this newtype renders the name plus `Debug`'s quotes,
/// which is prose carrying a delimiter the sentence did not ask for,
/// and it is `Debug`'s prose only for as long as the payload stays a
/// `String`: the day it grows a field the sentence starts dumping
/// braces with no edit to the wording. Rendering through here is what
/// makes that a compile-time question rather than a wording accident.
///
/// **One door quotes, deliberately.**
/// [`crate::ParseError::UnknownParam`] echoes the bytes an author
/// typed, which may be a typo, so its quotes delimit what was read
/// rather than decorating a name the document holds. Nothing else
/// decides this per call site; the row is
/// `display_contract::a_parameter_name_renders_unquoted_at_every_door_but_parse`.
impl core::fmt::Display for ParamName {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(&self.0)
    }
}

/// A document-level named parameter's declared dimension and exact
/// stored value (spec D2/D4: `f64` bit-exact for continuous, `i64`
/// for Count — bit-identical replay is trivial by representation).
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub enum DocParam {
    /// A continuous parameter in canonical kernel units.
    Continuous {
        /// Declared dimension (never `Count`; `apply` refuses).
        dim: Dimension,
        /// The value, exact `f64`.
        value: f64,
        /// The display unit this parameter was AUTHORED in — the same
        /// presentation metadata a literal carries
        /// (`Expr::display_unit`), for the same reason: a parameter
        /// authored in millimetres should read back in millimetres.
        ///
        /// Not optional, for [`crate::expr::Lit`]'s reason: a
        /// dimensionless parameter names the dimensionless row rather
        /// than declining to name one, so no reader has to invent a
        /// notation and no two readers can invent different ones.
        ///
        /// It rides with the DECLARATION, beside `dim` and
        /// `distribution`, and not with the value — which is exactly
        /// why [`crate::DocEdit::SetDocParamValue`] leaves it alone
        /// (see [`DocParamValue`]): how a parameter is written is a
        /// fact about the parameter, not about the number being typed
        /// into it.
        ///
        /// The unit must MEASURE `dim`. Nothing here can enforce that
        /// — the payload is `pub` and the dimension is data — so the
        /// pairing is a document invariant checked by the shared
        /// save/load validator (`persist::check`), like every other
        /// invariant this `pub` payload can be corrupted past. The
        /// authoring doors ([`DocParam::written_length`],
        /// [`DocParam::written_angle`]) cannot produce a mismatched
        /// one at all.
        display_unit: crate::expr::UnitSym,
        /// Optional uncertainty about this parameter (ERROR-DESIGN
        /// E1/E2), as offsets from `value` in the parameter's own
        /// `dim`. Document metadata read ONLY by
        /// [`crate::analysis`]: it enters no evaluation, no content
        /// key and no predicate, and `None` — the default — means the
        /// parameter is FIXED, not that its uncertainty is unknown.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        distribution: Option<Distribution>,
    },
    /// An integer Count parameter (structural material, spec D3).
    ///
    /// Carries NO distribution, and cannot — the argument is
    /// [`Self::with_distribution`]'s rustdoc (E11.3). It comes out
    /// UNREPRESENTABLE here rather than as a refusal: there is no
    /// spelling to refuse.
    Count {
        /// The exact value.
        value: i64,
    },
}

/// The VALUE half of a document parameter, with no declaration
/// attached: what a value-only edit
/// ([`crate::DocEdit::SetDocParamValue`]) writes.
///
/// A parameter's declaration — its dimension, and its optional
/// [`Distribution`] — belongs to the parameter, not to the number
/// being typed into it. Carrying only the number is what lets the
/// value door leave both alone; a caller that rebuilds a whole
/// [`DocParam`] from `(dim, value)` deletes the annotation, silently,
/// because [`crate::DocEdit::SetDocParam`] is create-or-replace.
#[derive(Debug, Clone, Copy, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum DocParamValue {
    /// A continuous parameter's nominal, in its ALREADY-DECLARED
    /// dimension (canonical kernel units).
    Continuous(f64),
    /// A `Count` parameter's exact integer.
    Count(i64),
}

impl DocParamValue {
    /// Whether this is the `Count` arm — the kind a value edit must
    /// match against the existing declaration.
    pub fn is_count(&self) -> bool {
        matches!(self, Self::Count(_))
    }
}

impl core::fmt::Display for DocParamValue {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Continuous(v) => write!(f, "continuous {v}"),
            Self::Count(v) => write!(f, "count {v}"),
        }
    }
}

/// Why a notation cannot be written onto a declaration
/// ([`DocParam::with_display_unit`]).
///
/// The two reasons a notation edit is refused, decided in ONE place —
/// the door — so that its callers only route them. The edit vocabulary
/// maps these to [`crate::EditError::DocParamCountHasNoUnit`] and
/// [`crate::EditError::DocParamUnitMismatch`]; nothing re-derives which
/// of the two applies.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DisplayUnitRefusal {
    /// The parameter is a [`DocParam::Count`]. A count is an exact
    /// integer, not a quantity: it names no notation, and the arm
    /// carries no field to write one into.
    CountHasNoNotation,
    /// The offered unit measures a different quantity than the
    /// parameter was declared with — millimetres for an angle, degrees
    /// for a length.
    Mismatch {
        /// What the offered unit measures.
        unit: Dimension,
        /// What the parameter declares.
        declared: Dimension,
    },
}

// The refusal's own prose, for a caller holding the door's `Err`
// without an `EditError` around it. The edit vocabulary renders its
// two arms in the parameter's own terms and names it; this says the
// fault alone.
impl core::fmt::Display for DisplayUnitRefusal {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::CountHasNoNotation => {
                f.write_str("a count is an integer rather than a quantity, so it has no notation")
            }
            Self::Mismatch { unit, declared } => write!(
                f,
                "the display unit measures {unit} but the parameter is declared {declared}"
            ),
        }
    }
}

impl core::error::Error for DisplayUnitRefusal {}

/// Why an E1/E2 annotation cannot be written onto a declaration
/// ([`DocParam::with_distribution`]).
///
/// [`DisplayUnitRefusal`]'s shape at the third field, and for its
/// reason: the two ways the annotation door can refuse, decided in ONE
/// place — the door — so that its callers only route them. The edit
/// vocabulary maps these to
/// [`crate::EditError::DocParamCountHasNoDistribution`] and to the
/// fault's own refusals ([`crate::EditError::NonFiniteDocParam`],
/// [`crate::EditError::InvalidDistribution`]); nothing re-derives
/// which applies.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DistributionRefusal {
    /// The parameter is a [`DocParam::Count`], which takes no
    /// annotation and carries no field to write one into — the
    /// argument is [`DocParam::with_distribution`]'s rustdoc (E11.3).
    CountHasNoAnnotation,
    /// The offered distribution breaks an E2 invariant — the same
    /// [`Distribution::check`] the persistence doors run, so an
    /// annotation a file could not carry cannot be written by an edit
    /// either.
    Invalid {
        /// The invariant that failed.
        fault: DistributionFault,
    },
}

// The refusal's own prose, for a caller holding the door's `Err`
// without an `EditError` around it — [`DisplayUnitRefusal`]'s
// convention. The fault half defers to `DistributionFault`'s sentence
// rather than minting a second spelling of the same invariant.
impl core::fmt::Display for DistributionRefusal {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::CountHasNoAnnotation => f.write_str(
                "a count is a structural parameter, fixed under any error analysis, so it takes no distribution",
            ),
            Self::Invalid { fault } => write!(f, "{fault}"),
        }
    }
}

impl core::error::Error for DistributionRefusal {}

/// WHICH float of a continuous document parameter a refusal is about
/// ([`DocParam::first_non_finite`]).
///
/// One name for the answer at both doors: the nominal, or the
/// distribution offset [`DistributionField`] names. An `Option<
/// DistributionField>` would say the same thing with absence standing
/// for the nominal, and a reader would have to know which absence it
/// was.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DocParamField {
    /// The parameter's own value.
    Nominal,
    /// An offset of the E1/E2 annotation beside it.
    Offset(DistributionField),
}

// The field's prose, forwarded by every door that renders it. The
// offset arm defers to `DistributionField`'s own word rather than
// minting a second spelling of it.
impl core::fmt::Display for DocParamField {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Nominal => f.write_str("the nominal"),
            Self::Offset(field) => write!(f, "distribution field {field}"),
        }
    }
}

/// **What makes an expression's document-parameter references
/// unusable** ([`Doc::param_ref_fault`], spec D6) — one vocabulary for
/// the edit doors and the load door.
///
/// The rule is the param TABLE's: a reference names a declared
/// parameter, and reads it at the dimension it was declared with. An
/// expression carries the dimension it read at, so a (re)declaration
/// that moves a parameter's dimension breaks every expression
/// referencing it — which is why the edit door re-asks this of every
/// slot after a declaration lands, and why a file can carry a pairing
/// no edit door would have written.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum ParamRefFault {
    /// The expression names a parameter the document does not declare.
    Unknown {
        /// The name it reads.
        name: ParamName,
    },
    /// The parameter is declared, at another dimension than the
    /// expression reads it at.
    Dimension {
        /// The name it reads.
        name: ParamName,
        /// The dimension the declaration carries.
        declared: Dimension,
        /// The dimension the expression reads it at.
        referenced: Dimension,
    },
}

impl DocParam {
    /// **The parameter's first float that is not a number** (the ruled
    /// non-finite policy, D2), or `None` — the nominal first, then the
    /// annotation's offsets in [`Distribution::first_non_finite`]'s
    /// order.
    ///
    /// One predicate with one home, asked by the create-or-replace
    /// edit door ([`crate::DocEdit::SetDocParam`]) and by the
    /// save/load validator's float walk, each naming the answer in
    /// its own vocabulary. It answers WHICH field rather than a bare
    /// yes: the walk has to identify one to decide there is a defect
    /// at all, and a diagnostic that names `sigma` beats one that
    /// names only the parameter.
    ///
    /// A [`DocParam::Count`] carries no float and no annotation, so it
    /// has nothing this can find.
    pub(crate) fn first_non_finite(&self) -> Option<DocParamField> {
        let Self::Continuous {
            value,
            distribution,
            ..
        } = self
        else {
            return None;
        };
        if !value.is_finite() {
            return Some(DocParamField::Nominal);
        }
        distribution
            .as_ref()?
            .first_non_finite()
            .map(DocParamField::Offset)
    }

    /// **A declaration the document cannot hold, stated once**: the
    /// `Continuous` arm carries a continuous dimension, so declaring it
    /// with `Count` is the structural/continuous divide spelled two
    /// ways at once (spec D3).
    ///
    /// The create-or-replace edit door ([`crate::DocEdit::SetDocParam`])
    /// is the one that asks it, and the one where it can fire: the
    /// `pub` payload is what makes the state reachable at all. The
    /// save/load validator does not, because the same declaration
    /// refuses one walk earlier there — `UnitSym::measures` answers
    /// Length, Angle or Scalar and never Count, so a continuous
    /// parameter declared `Count` fails the notation walk whatever
    /// notation it carries.
    pub(crate) fn is_continuous_count(&self) -> bool {
        matches!(
            self,
            Self::Continuous {
                dim: Dimension::Count,
                ..
            }
        )
    }

    /// The parameter's dimension.
    pub fn dim(&self) -> Dimension {
        match self {
            Self::Continuous { dim, .. } => *dim,
            Self::Count { .. } => Dimension::Count,
        }
    }

    /// A continuous LENGTH parameter that remembers the notation it
    /// was authored in ([`quantity::WrittenLength`]).
    ///
    /// **Total** — there is no dimension for the unit to disagree
    /// with: a `WrittenLength` holds a `LengthUnit`, which is an index
    /// into a Length row of the table (#669), and this door supplies
    /// `Dimension::Length` itself. The mismatch the validator watches
    /// for is unreachable through here, which is the whole reason to
    /// author through it rather than through the `pub` payload.
    ///
    /// No distribution: the E1/E2 annotation belongs to the parameter
    /// and is added through its own door, exactly as
    /// [`DocParam::continuous`] leaves it alone.
    pub fn written_length(written: quantity::WrittenLength) -> Self {
        Self::Continuous {
            dim: Dimension::Length,
            value: written.meters(),
            display_unit: crate::expr::UnitSym::from_def(&written.unit().def()),
            distribution: None,
        }
    }

    /// A continuous ANGLE parameter that remembers its authored
    /// notation — [`DocParam::written_length`]'s mirror, total for the
    /// same reason.
    pub fn written_angle(written: quantity::WrittenAngle) -> Self {
        Self::Continuous {
            dim: Dimension::Angle,
            value: written.radians(),
            display_unit: crate::expr::UnitSym::from_def(&written.unit().def()),
            distribution: None,
        }
    }

    /// A continuous parameter with no distribution, written in the
    /// canonical unit for its dimension — the plain authoring
    /// spelling.
    pub fn continuous(dim: Dimension, value: f64) -> Self {
        Self::Continuous {
            dim,
            value,
            display_unit: crate::expr::UnitSym::canonical_for(dim),
            distribution: None,
        }
    }

    /// A continuous parameter carrying `distribution` — the annotated
    /// authoring spelling. Only continuous parameters can be
    /// annotated, so this is a constructor rather than a method: there
    /// is no `Count` case to silently drop the annotation.
    pub fn continuous_with(dim: Dimension, value: f64, distribution: Distribution) -> Self {
        Self::Continuous {
            dim,
            value,
            display_unit: crate::expr::UnitSym::canonical_for(dim),
            distribution: Some(distribution),
        }
    }

    /// This parameter's distribution, if it is continuous and carries
    /// one.
    pub fn distribution(&self) -> Option<&Distribution> {
        match self {
            Self::Continuous { distribution, .. } => distribution.as_ref(),
            Self::Count { .. } => None,
        }
    }

    /// This parameter with `value` written into it, keeping the whole
    /// DECLARATION — the dimension, the authored display unit and the
    /// optional distribution — untouched. **The VALUE carry-forward, in
    /// one place**: every value door goes through here rather than
    /// rebuilding a parameter from parts, so no door can drop an
    /// annotation it never mentioned. The notation has its own, in
    /// [`Self::with_display_unit`].
    ///
    /// `None` when the value's arm does not match the declaration's.
    /// Changing a parameter's kind is a REDECLARATION — the
    /// create-or-replace door, where the dimension and the annotation
    /// are stated afresh — and a value edit that quietly performed one
    /// would be the same silent deletion in a different disguise.
    pub fn with_value(&self, value: DocParamValue) -> Option<Self> {
        match (self, value) {
            (
                Self::Continuous {
                    dim,
                    display_unit,
                    distribution,
                    ..
                },
                DocParamValue::Continuous(value),
            ) => Some(Self::Continuous {
                dim: *dim,
                value,
                display_unit: *display_unit,
                distribution: *distribution,
            }),
            (Self::Count { .. }, DocParamValue::Count(value)) => Some(Self::Count { value }),
            // EXHAUSTIVE on purpose, both sides spelled: a new
            // `DocParam` arm or a new value arm must say how a value
            // edit reaches it, or the compile breaks.
            (Self::Continuous { .. }, DocParamValue::Count(_))
            | (Self::Count { .. }, DocParamValue::Continuous(_)) => None,
        }
    }

    /// This parameter written in `unit`, keeping the whole rest of the
    /// DECLARATION — the dimension, the exact value and the optional
    /// distribution — untouched. [`Self::with_value`]'s mirror over the
    /// other field, and the notation carry-forward in one place: every
    /// door that writes a notation goes through here rather than
    /// rebuilding a parameter from parts, so no door can drop an
    /// annotation it never mentioned.
    ///
    /// # Changing a NOTATION is not a redeclaration
    ///
    /// **This is the home of that argument**; everywhere else that
    /// needs it cites this paragraph rather than restating it.
    ///
    /// Changing a parameter's KIND is a redeclaration — that is
    /// [`Self::with_value`]'s argument, and why a value edit refuses a
    /// kind change rather than performing one. Changing its NOTATION is
    /// a different class of thing, and the document already ruled so:
    /// [`Self::bit_eq`] EXCLUDES `display_unit` as presentation
    /// metadata, the same ruling `Expr::bit_eq` makes about a literal's.
    /// A notation edit therefore changes nothing bit-semantic equality
    /// sees — it enters the history and it persists, and replay
    /// identity and `diff.rs` are blind to it exactly as they are to a
    /// literal's notation. So there is nothing about the parameter for
    /// a caller to restate; the create-or-replace door would make them
    /// restate it all, and silently delete whatever they forgot.
    ///
    /// # Errors
    ///
    /// A TYPED reason rather than a bare `None`, because there are two
    /// of them and the edit door reports them as two different
    /// refusals: a caller that had to re-derive which one applied would
    /// be the second home of a rule that lives here.
    /// [`DisplayUnitRefusal::CountHasNoNotation`] for a [`Self::Count`],
    /// [`DisplayUnitRefusal::Mismatch`] for a unit that does not MEASURE
    /// the declared dimension — the pairing the save/load validator
    /// refuses a document for (`persist::check`) and the one
    /// [`Self::written_length`]/[`Self::written_angle`] make unreachable
    /// by construction. The predicate is [`crate::UnitSym::measures`],
    /// asked rather than restated.
    ///
    /// EXHAUSTIVE on both arms as [`Self::with_value`] is: a new
    /// `DocParam` variant must say how a notation edit reaches it, or
    /// the compile breaks.
    pub fn with_display_unit(
        &self,
        unit: crate::expr::UnitSym,
    ) -> Result<Self, DisplayUnitRefusal> {
        match self {
            Self::Continuous {
                dim,
                value,
                display_unit: _,
                distribution,
            } => {
                let measured = unit.measures();
                if measured != *dim {
                    return Err(DisplayUnitRefusal::Mismatch {
                        unit: measured,
                        declared: *dim,
                    });
                }
                Ok(Self::Continuous {
                    dim: *dim,
                    value: *value,
                    display_unit: unit,
                    distribution: *distribution,
                })
            }
            Self::Count { .. } => Err(DisplayUnitRefusal::CountHasNoNotation),
        }
    }

    /// This parameter carrying `distribution`, keeping the whole rest
    /// of the DECLARATION — the dimension, the exact value and the
    /// authored display unit — untouched. [`Self::with_value`]'s and
    /// [`Self::with_display_unit`]'s mirror over the third field, and
    /// the ANNOTATION carry-forward in one place: every door that
    /// writes an E1/E2 annotation goes through here rather than
    /// rebuilding a parameter from parts, so no door can drop a field
    /// it never mentioned. [`Self::continuous_with`], the authoring
    /// spelling, writes the CANONICAL notation, so annotating through
    /// create-or-replace re-spells a parameter authored in
    /// millimetres; there is nothing to restate here.
    ///
    /// # `None` clears, and clearing is this door
    ///
    /// There is ONE annotation door, not a set/clear pair. The field
    /// is an `Option<Distribution>` and "no annotation" is a VALUE of
    /// the declaration — E1/E2's reading that an absent distribution
    /// means no error analysis applies to the parameter — so writing
    /// `None` is the same carry-forward edit as writing `Some`.
    /// [`crate::DocEdit::SetAppearanceMeta`]/`ClearAppearanceMeta` are
    /// two arms because a meta entry is a ROW IN A MAP, where clearing
    /// removes the row rather than writing a value; that is a
    /// different shape, and this is the one sentence that says so.
    ///
    /// # A COUNT takes no annotation (E11.3)
    ///
    /// **This is the home of that argument**; everywhere else that
    /// needs it cites this paragraph rather than restating it, the
    /// convention [`Self::with_display_unit`] follows for its own.
    ///
    /// A count is a STRUCTURAL parameter — it says how many of a
    /// thing there are — and an error analysis prices the spread of a
    /// continuous quantity, so a count is fixed under any of them.
    /// That is why [`Self::Count`] carries no field to hang a
    /// distribution on, which makes the rule UNREPRESENTABLE in the
    /// declaration rather than refused at it; and it is why every door
    /// that can be handed a count and an annotation TOGETHER refuses
    /// instead of ignoring one of them.
    ///
    /// # Errors
    ///
    /// A TYPED reason rather than a bare `None`, for
    /// [`Self::with_display_unit`]'s reason: there are two of them and
    /// the edit door reports them as different refusals.
    /// [`DistributionRefusal::CountHasNoAnnotation`] for a
    /// [`Self::Count`] (the section above) and
    /// [`DistributionRefusal::Invalid`] for an
    /// offered distribution [`Distribution::check`] refuses, the SAME
    /// check the persistence doors run.
    ///
    /// [`crate::DocEdit::SetDocParam`] reaches that same
    /// [`Distribution::check`] without this door, so the shared write
    /// path runs it again rather than trusting this one; neither copy
    /// is the other's fallback, and a door that leaned on the tail
    /// would hand a caller OUTSIDE `apply` a parameter no file could
    /// carry. [`Self::with_display_unit`] and the pairing check are
    /// doubled the same way, for the same reason.
    ///
    /// Clearing a `Count`'s annotation is refused too, rather than
    /// accepted as a no-op: a caller aiming an annotation edit at a
    /// count has the wrong parameter, and a door that answered `Ok`
    /// because the field happened to be absent would hide that.
    ///
    /// EXHAUSTIVE on both arms as its two siblings are: a new
    /// `DocParam` variant must say how an annotation edit reaches it,
    /// or the compile breaks.
    pub fn with_distribution(
        &self,
        distribution: Option<Distribution>,
    ) -> Result<Self, DistributionRefusal> {
        match self {
            Self::Continuous {
                dim,
                value,
                display_unit,
                distribution: _,
            } => {
                if let Some(d) = &distribution
                    && let Err(fault) = d.check()
                {
                    return Err(DistributionRefusal::Invalid { fault });
                }
                Ok(Self::Continuous {
                    dim: *dim,
                    value: *value,
                    display_unit: *display_unit,
                    distribution,
                })
            }
            Self::Count { .. } => Err(DistributionRefusal::CountHasNoAnnotation),
        }
    }

    /// Bit-semantic equality (spec D7): continuous values compare by
    /// BITS (`0.0` ≠ `-0.0` here), everything else structurally.
    ///
    /// **The display unit is EXCLUDED**, exactly as it is from
    /// `Expr::bit_eq` and for the same reason: it is presentation
    /// metadata, so two parameters differing only in how they are
    /// written are the same parameter. The consequence is the one the
    /// literal already has — a change of notation alone is a document
    /// edit that enters the history and persists, and is invisible to
    /// replay identity and to `diff.rs`. Spelled out rather than
    /// omitted, because the field is named in the pattern below and a
    /// reader is owed the reason it is not in the comparison.
    ///
    /// EXHAUSTIVE on purpose, on BOTH sides of the pair: the mismatched
    /// pairs are spelled out rather than swept up, so a future
    /// `DocParam` variant must say how it compares here or the compile
    /// breaks. A wildcard would have answered `false` for a new variant
    /// against ITSELF — two equal parameters reported as differing,
    /// through [`Doc::bit_eq`] and `diff.rs`, which is D7's replay
    /// identity and the document diff reading the same wrong answer.
    pub fn bit_eq(&self, other: &DocParam) -> bool {
        match (self, other) {
            (
                Self::Continuous {
                    dim: da,
                    value: va,
                    display_unit: _,
                    distribution: ha,
                },
                Self::Continuous {
                    dim: db,
                    value: vb,
                    display_unit: _,
                    distribution: hb,
                },
            ) => {
                da == db
                    && va.to_bits() == vb.to_bits()
                    // Present-vs-present compares BIT-exact on the
                    // offsets; present-vs-absent differs.
                    && match (ha, hb) {
                        (None, None) => true,
                        (Some(a), Some(b)) => a.bit_eq(b),
                        (None, Some(_)) | (Some(_), None) => false,
                    }
            }
            (Self::Count { value: a }, Self::Count { value: b }) => a == b,
            (Self::Continuous { .. }, Self::Count { .. })
            | (Self::Count { .. }, Self::Continuous { .. }) => false,
        }
    }
}

/// The document: recipe DAG (node map + insertion-ordered list) +
/// document metadata (spec D2; ratified F2's substrate). `P` is the
/// opaque profile payload (spec D1/D3 — see [`Node`]).
///
/// **A field added here that holds a [`StableName`] is placed in
/// `Carrier` below**, which is the one enumeration of the document's
/// name carriers: the delete door's DM7 report, the split door's
/// containment check, `inline_part`'s classification and the snapshot
/// validator all read it, so the carrier arrives at one edit rather
/// than at four sites that each spell the list by hand.
///
/// That first step is this sentence and nothing else — no check sees
/// a new field of this struct, so a field added and not placed
/// compiles and walks nowhere. What is compiler-forced begins one
/// step later, at the variant.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
// The `with`-routed nodes field hides `P` from serde's bound
// inference; state the bounds explicitly.
#[serde(bound(
    serialize = "P: serde::Serialize",
    deserialize = "P: serde::Deserialize<'de>"
))]
pub struct Doc<P> {
    /// The document's stable identity (ASM-1 D-1): authored data
    /// supplied at construction, never minted from ambient randomness
    /// in this crate. Survives every edit; excluded from the content
    /// pin (the pin answers "which version", the id "which part").
    pub(crate) id: DocumentId,
    /// The monotone id counter: the next [`RecipeNodeId`] to mint.
    /// Never decremented — deletion does not free ids (spec D3).
    pub(crate) next_id: u64,
    /// The monotone STEP counter: the next [`crate::StepId`] to mint
    /// for an authored profile step (`names/README.md`, "N1, the
    /// profile pieces"). Never decremented — a step a `SetProgram`
    /// drops takes its id with it, and the id is never minted again.
    /// A counter of its own rather than the node counter's, so an
    /// authored step moves no node id.
    pub(crate) next_step: u64,
    /// The nodes, by stable id.
    #[serde(with = "crate::persist::strict::nodes")]
    pub(crate) nodes: BTreeMap<RecipeNodeId, Node<P>>,
    /// Insertion order of the live nodes (the recipe's presentation
    /// order; the DAG's edges are the nodes' input refs, spec D3).
    pub(crate) order: Vec<RecipeNodeId>,
    /// The document's ordered product roots (ASSEMBLY-DESIGN A10,
    /// ASM-ROOTS D-1): document data, never a DAG node. Two invariants
    /// hold at rest and after every edit — *coverage* (every node is
    /// an ancestor of, or is, some root) and *ancestor-freedom* (no
    /// root is a strict ancestor of another) — which together say the
    /// root SET is exactly the DAG's sink set; the list adds the
    /// product's solid ORDER, which is therefore semantic. No
    /// duplicates; every entry is live.
    pub(crate) roots: Vec<RecipeNodeId>,
    /// Cluster placement frames (ASSEMBLY-DESIGN A11, ASM-2A D-2):
    /// document data keyed by the instantiate node whose singleton
    /// cluster the frame places. A MISSING entry is the identity frame
    /// — a legal, complete state — so the registry never needs a row
    /// per instance, and zero-/multi-anchor states cannot be spelled.
    /// Written only by the recorded `SetPlacement` edit; every key
    /// names a live `InstantiatePart` node.
    #[serde(with = "crate::persist::strict::placements")]
    pub(crate) placements: BTreeMap<RecipeNodeId, crate::placement::Frame>,
    /// Document-level named parameters.
    #[serde(with = "crate::persist::strict::params")]
    pub(crate) params: BTreeMap<ParamName, DocParam>,
    /// The recorded modeling tolerance ε (M4 PR 6 spec D4): new
    /// documents record the process's committed ambient ε; loading
    /// reconciles the recorded value against the process (one process
    /// = one ε); `SetTolerance` edits it.
    pub(crate) epsilon: f64,
    /// Per-node witness data (M4 PR 4, SOLVER-DESIGN W1/W4): the
    /// opaque branch-selection datum of each sketch-bearing node,
    /// written ONLY by the recorded `ReWitness` edits (and, at M6, by
    /// committed sketch edits). Document state under GQ3 — undo/redo
    /// and replay need no special cases.
    #[serde(with = "crate::persist::strict::witnesses")]
    pub(crate) witnesses: BTreeMap<RecipeNodeId, crate::witness::WitnessDatum>,
    /// Free-form document metadata (display units etc. — presentation
    /// only, GQ5). Empty in v1 (spec D2).
    #[serde(with = "crate::persist::strict::doc_metadata")]
    pub(crate) metadata: BTreeMap<String, String>,
    /// Appearance attributes keyed by stable name (M4 PR 7;
    /// DESIGN.md's ratified attachment contract). Presentation
    /// metadata: NEVER enters evaluation content keys — see
    /// [`crate::appearance`] for the loss (N3/N5) and wrapper (B11)
    /// semantics.
    #[serde(with = "crate::persist::pairs")]
    pub(crate) appearance: AppearanceMap,
}

/// **Which of the document's own fields hold a [`StableName`]** — one
/// variant per field, iterated by [`Doc::name_carriers`].
///
/// [`Node::payload_names`] is the exhaustive answer to "which
/// PAYLOADS carry a name". This is the wider question — which of
/// [`Doc`]'s fields hold a name at all — and it has one answer here
/// rather than a hand-written list at each walk that needs it.
///
/// **Exhaustive the way `persist::check`'s walk roster is.** A
/// VARIANT added here does not compile: [`Doc::names_in`] maps a
/// carrier to the names behind it with no wildcard arm, and the
/// roster row's `f6_variants!` weld writes a second such match, so
/// the new variant is an E0004 at both until it says which field it
/// reads. [`Carrier::ALL`] is what [`Doc::name_carriers`] iterates,
/// and nothing about the array is compiler-forced, so a variant
/// missing from it never walks —
/// `tests::the_carrier_roster_is_what_the_walk_iterates` is what reds
/// on that.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Carrier {
    /// The nodes' name-carrying payloads, read through
    /// [`Node::payload_names`] — which stays the one list of NODE
    /// carriers (DM7), so this variant names the field and not its
    /// contents.
    Payloads,
    /// The appearance store's keys: a `StableName` under Declare's N5
    /// semantics (`DocEdit::SetAppearance`), held by the document
    /// itself rather than by any node.
    Appearance,
}

impl Carrier {
    /// Every carrier, in the order [`Doc::name_carriers`] walks them
    /// — which it walks them BY, so this is the order rather than a
    /// description of one.
    pub(crate) const ALL: [Carrier; 2] = [Carrier::Payloads, Carrier::Appearance];
}

/// **One [`StableName`] the document holds, and what holds it** — the
/// element of [`Doc::name_carriers`].
///
/// The two carriers are not the same shape and this does not flatten
/// them: a payload name has a carrying node, which is the node a
/// report names and a containment check tests; a store key has none,
/// because the store holds the attachment itself. That difference is
/// why DM7's report has two arms (`Maintenance::Strand` and
/// `Maintenance::StrandedAppearance`), and they map onto these two
/// variants one to one.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum NameCarrier<'a> {
    /// A name in a live node's payload; that node carries it.
    Payload {
        /// The node whose payload holds the name.
        node: RecipeNodeId,
        /// The name.
        name: &'a StableName,
    },
    /// A key of the appearance store. No node carries it.
    Store {
        /// The name.
        name: &'a StableName,
    },
}

impl<'a> NameCarrier<'a> {
    /// The name, for the callers that ask one question of both
    /// carriers — `inline_part`'s classification, the snapshot
    /// validator's id check, DM7's filter on the deleted node.
    ///
    /// Flattening here is the caller's choice, not the walk's: the
    /// variant is still there to match on. The public API flattens
    /// one layer up in the same way — `pncad`'s `Maintenance.name`
    /// answers for both `Strand` and `StrandedAppearance`, and a
    /// caller that needs the carrying node reads the class instead of
    /// the accessor.
    pub(crate) fn name(self) -> &'a StableName {
        match self {
            Self::Payload { name, .. } | Self::Store { name } => name,
        }
    }
}

impl<P> Doc<P> {
    /// The empty document under the given identity: no nodes, no
    /// params, recorded ε = the process's ambient tolerance (M4 PR 6
    /// spec D4: a new document adopts the process ε, so in-process
    /// documents NEVER disagree with the committed tolerance; the
    /// OnceLock bootstrap commits here on first touch if nothing
    /// committed earlier) — replay's origin (spec D7). The id is
    /// authored data (ASM-1 D-1): there is no id-less document and no
    /// ambient-randomness default.
    pub fn empty(id: DocumentId, tol: Tol) -> Self {
        Self {
            id,
            next_id: 0,
            next_step: 0,
            nodes: BTreeMap::new(),
            order: Vec::new(),
            roots: Vec::new(),
            placements: BTreeMap::new(),
            params: BTreeMap::new(),
            epsilon: tol.eps(),
            witnesses: BTreeMap::new(),
            metadata: BTreeMap::new(),
            appearance: AppearanceMap::new(),
        }
    }

    /// The empty document under a label-derived identity —
    /// [`Self::empty`] ∘ [`DocumentId::derive`], the deterministic
    /// spelling corpus/demos/tests use.
    pub fn empty_derived(label: &str, tol: Tol) -> Self {
        Self::empty(DocumentId::derive(label), tol)
    }

    /// The document's stable identity.
    pub fn id(&self) -> DocumentId {
        self.id
    }

    /// The same document under a different identity: `id` replaces
    /// this one's and NOTHING else moves.
    ///
    /// This is the fork constructor (A4). Identity answers "which
    /// part", so a document that is to become a SECOND part gets a
    /// second id, and every reference pinning the first keeps naming
    /// the first. Content is untouched, so the content pin is
    /// unchanged — the pin's preimage is the document's serde form
    /// with the `id` key removed
    /// ([`crate::persist::canonical_bytes`]), which is what makes a
    /// copy under a fresh id detectably the same content.
    ///
    /// Minting the fresh id is the CALLER's: this crate is
    /// deterministic by construction and touches no ambient
    /// randomness.
    pub fn under_identity(mut self, id: DocumentId) -> Self {
        self.id = id;
        self
    }

    /// The node with the given id, if live.
    pub fn node(&self, id: RecipeNodeId) -> Option<&Node<P>> {
        self.nodes.get(&id)
    }

    /// **Could this document have minted `id`** — the one reading of
    /// the mint counter that leaves this crate, and the one DI1's
    /// minting-entry walk asks of a history entry
    /// (`crates/editor-core/IDENTITY.md`).
    ///
    /// True exactly when `id` is below the counter. The counter is
    /// monotone — never decremented, because deletion does not free
    /// ids (spec D3) — so along any forward path of [`Doc::apply`]s
    /// this answer goes false to true and never back. That is what
    /// makes DI1's walk — up the history until the counter drops
    /// below the held id — land on the entry that minted it: the
    /// predicate is false above the mint and true from the mint on.
    ///
    /// **What a `true` does NOT mean**: not that the node is there.
    /// A minted id may since have been deleted, and its id is not
    /// reused, so the predicate keeps answering true for it forever.
    /// Liveness is [`Doc::node`]'s question, and DI1's rule asks both
    /// — descent first, then liveness.
    ///
    /// The counter itself stays private: a caller can ask whether a
    /// particular id is behind it and cannot read where it stands, so
    /// the monotonicity argument stays on the side that owns it.
    pub fn has_minted(&self, id: RecipeNodeId) -> bool {
        id.0 < self.next_id
    }

    /// Live node ids in insertion order.
    pub fn order(&self) -> &[RecipeNodeId] {
        &self.order
    }

    /// The ordered product roots (A10): the gather order of the
    /// document's product solids.
    pub fn roots(&self) -> &[RecipeNodeId] {
        &self.roots
    }

    /// The placement frame of `node`'s CLUSTER (A11): the frame
    /// recorded against the cluster's gauge, or the identity when
    /// nothing is recorded — the missing entry IS the identity, so
    /// this is total.
    ///
    /// This is the CLUSTER's frame, which places its gauge; an
    /// instance's own world placement is this composed with its solved
    /// relative pose ([`crate::mate::SolvedPoses::placement`]). The
    /// two coincide exactly for a singleton cluster, which is every
    /// cluster in a mate-less document.
    pub fn placement(&self, node: RecipeNodeId) -> crate::placement::Frame {
        self.placements
            .get(&crate::mate::gauge_of(self, node))
            .copied()
            .unwrap_or(crate::placement::Frame::IDENTITY)
    }

    /// Replaces the whole placement registry — the ONE door A11's
    /// cluster-record maintenance writes through
    /// ([`crate::mate::solve::maintain`], deriving the rows or
    /// re-applying recorded ones), so re-keying is a single observable
    /// act rather than a scatter of per-row edits.
    pub(crate) fn set_placements(&mut self, rows: BTreeMap<RecipeNodeId, crate::placement::Frame>) {
        self.placements = rows;
    }

    /// The recorded placement rows, in node order. Rows absent here
    /// are identity placements, not missing state.
    pub fn placements(&self) -> &BTreeMap<RecipeNodeId, crate::placement::Frame> {
        &self.placements
    }

    /// Number of live nodes.
    pub fn len(&self) -> usize {
        self.order.len()
    }

    /// Whether the document has no live nodes.
    pub fn is_empty(&self) -> bool {
        self.order.is_empty()
    }

    /// **The param-table rule, asked of one expression** — the first
    /// reference it makes that the table cannot answer, or `None`.
    ///
    /// One home for the question (spec D6), with FOUR callers — the
    /// two doors' two walks each, one over a node's SLOT expressions
    /// and one over the PAYLOAD expressions no slot addresses
    /// (`crate::node::payload_exprs`):
    ///
    /// - the edit door's `edit::check_param_refs` (slots) and the
    ///   payload arm of `edit::check_node_slots`;
    /// - the load door's `persist::check::first_slot_param_ref_fault`
    ///   and `first_payload_param_ref_fault`.
    ///
    /// Each names this one answer in its own vocabulary; none of them
    /// re-states the rule.
    pub(crate) fn param_ref_fault(&self, expr: &Expr) -> Option<ParamRefFault> {
        let mut refs = Vec::new();
        expr.param_refs(&mut refs);
        refs.into_iter()
            .find_map(|(name, referenced)| match self.params.get(&name) {
                None => Some(ParamRefFault::Unknown { name }),
                Some(p) if p.dim() != referenced => Some(ParamRefFault::Dimension {
                    declared: p.dim(),
                    name,
                    referenced,
                }),
                Some(_) => None,
            })
    }

    /// The document-level named parameters.
    pub fn params(&self) -> &BTreeMap<ParamName, DocParam> {
        &self.params
    }

    /// The recorded modeling tolerance ε (spec D2; edited by PR 6's
    /// `SetTolerance`).
    pub fn epsilon(&self) -> f64 {
        self.epsilon
    }

    /// The document metadata map (empty in v1, spec D2).
    pub fn metadata(&self) -> &BTreeMap<String, String> {
        &self.metadata
    }

    /// The recorded witness datum of a sketch-bearing node, if any
    /// (SOLVER-DESIGN W1; written only by `ReWitness` edits).
    pub fn witness(&self, id: RecipeNodeId) -> Option<&crate::witness::WitnessDatum> {
        self.witnesses.get(&id)
    }

    /// Every recorded witness, by node.
    pub fn witnesses(&self) -> &BTreeMap<RecipeNodeId, crate::witness::WitnessDatum> {
        &self.witnesses
    }

    /// The appearance store: attributes by stable name (M4 PR 7;
    /// edited through `SetAppearance`/`ClearAppearance`; `Rebind`
    /// rewrites keys — the attribute rides the name).
    pub fn appearance(&self) -> &AppearanceMap {
        &self.appearance
    }

    /// One name's appearance record (attrs + D7 metadata), if any.
    pub fn appearance_of(&self, name: &StableName) -> Option<&AppearanceRecord> {
        self.appearance.get(name)
    }

    /// **Every [`StableName`] this document holds, with what holds
    /// it** — the one answer to "which carriers hold a name",
    /// enumerated rather than spelled out again at each walk that
    /// needs it.
    ///
    /// Four walks used to spell this list by hand, each in its own
    /// order: DM7's delete report, the split door's containment
    /// check, `inline_part`'s foreign-name classification and the
    /// snapshot validator. A field of this struct that began holding
    /// a name reached none of them.
    ///
    /// **The order is a CONTRACT**, and it is [`Carrier::ALL`]'s:
    /// every payload name first, in document order and within one
    /// node in [`Node::payload_names`]' order, then every appearance
    /// key, in the store's own `BTreeMap` order — which is
    /// [`StableName`]'s, so nothing is sorted here. DM7's report is
    /// this walk filtered, so the clause's payload-strands-then-store
    /// -strands order is this order.
    ///
    /// Three rows hold that order, and they are the three that red
    /// when [`Carrier::ALL`] is reversed — measured, nothing else
    /// does: `tests::name_carriers_reads_the_payloads_then_the_store`
    /// here, and at the delete door
    /// `dm7_delete_strands::an_appearance_strand_follows_the_payload_strands_of_the_same_delete`
    /// and
    /// `dm7_delete_strands::an_appearance_strand_precedes_the_cluster_acts_of_the_same_delete`.
    ///
    /// **Cost.** Lazy: the carriers are yielded as they are found, so
    /// a reader that refuses at the first bad name stops there
    /// instead of paying for the rest of the document. What is
    /// allocated is one boxed iterator per carrier and the `Vec` of
    /// borrows [`Node::payload_names`] returns per node REACHED —
    /// that per-node vector is the node's own and is unchanged by
    /// this walk; the walk adds no vector of its own.
    pub(crate) fn name_carriers(&self) -> impl Iterator<Item = NameCarrier<'_>> {
        Carrier::ALL
            .into_iter()
            .flat_map(move |carrier| self.names_in(carrier))
    }

    /// The names ONE carrier holds — the map from a [`Carrier`] to
    /// the field it reads.
    ///
    /// Exhaustive with no wildcard arm, which is what makes
    /// [`Carrier`] the enumeration rather than a comment beside one:
    /// a variant added there does not compile until it says what it
    /// reads here. (A FIELD added to [`Doc`] is not compile-forced —
    /// becoming a variant in the first place is the struct doc's
    /// convention, and it is the one step of this that a person has
    /// to take.)
    ///
    /// Boxed because the arms are different iterators and the caller
    /// is one `flat_map` over [`Carrier::ALL`]: two allocations per
    /// walk, against the whole enumeration's worth of names a
    /// collecting version built eagerly.
    fn names_in(&self, carrier: Carrier) -> Box<dyn Iterator<Item = NameCarrier<'_>> + '_> {
        match carrier {
            Carrier::Payloads => Box::new(
                self.order
                    .iter()
                    .filter_map(|&id| self.nodes.get(&id).map(|node| (id, node)))
                    .flat_map(|(id, node)| {
                        node.payload_names()
                            .into_iter()
                            .map(move |name| NameCarrier::Payload { node: id, name })
                    }),
            ),
            Carrier::Appearance => Box::new(
                self.appearance
                    .keys()
                    .map(|name| NameCarrier::Store { name }),
            ),
        }
    }

    /// The expression subtree an [`ExprPath`] addresses, or `None` if
    /// the node is gone, the slot absent, or the path off the tree
    /// (spec D5).
    pub fn expr_at(&self, path: &ExprPath) -> Option<&Expr>
    where
        P: crate::ProfilePayload,
    {
        self.nodes
            .get(&path.node)?
            .expr(path.slot)?
            .descend(&path.path)
    }

    /// The evaluation environment for this document's parameters,
    /// embedding stored exact values into any [`Real`] `T` (spec D4:
    /// the evaluator is scalar-generic; units erase here, GQ5).
    pub fn param_env<T: Real>(&self) -> ParamEnv<T> {
        let bindings = self
            .params
            .iter()
            .map(|(name, p)| {
                // The nominal alone crosses into evaluation: a
                // distribution is document metadata the scalar channel
                // never sees (E1).
                let v = match *p {
                    DocParam::Continuous { dim, value, .. } => ParamValue::Continuous {
                        dim,
                        value: T::from_f64(value),
                    },
                    DocParam::Count { value } => ParamValue::Count(value),
                };
                (name.clone(), v)
            })
            .collect();
        ParamEnv { bindings }
    }
}

impl<P: PartialEq + crate::ProfilePayload> Doc<P> {
    /// Bit-semantic document equality (spec D7's replay-identity
    /// comparator; M4 PR 1 review non-blocker): every float field —
    /// expression literals, continuous doc params, recorded ε —
    /// compares by BITS; ids, order, structure, metadata compare
    /// structurally. `PartialEq` on `Doc` remains IEEE-semantic
    /// (conflates `±0.0`); use THIS for replay pins and audits.
    pub fn bit_eq(&self, other: &Doc<P>) -> bool {
        self.id == other.id
            && self.next_id == other.next_id
            && self.next_step == other.next_step
            && self.order == other.order
            && self.roots == other.roots
            && self.epsilon.to_bits() == other.epsilon.to_bits()
            // Placement coordinates are floats: compare BY BITS, like
            // every other float field here.
            && self.placements.len() == other.placements.len()
            && self.placements.iter().all(|(id, frame)| {
                other
                    .placements
                    .get(id)
                    .is_some_and(|theirs| frame.bit_eq(theirs))
            })
            // Witness bytes are exact data (no float semantics to
            // conflate) — structural equality IS bit equality here.
            && self.witnesses == other.witnesses
            && self.metadata == other.metadata
            // Appearance attrs are float-free by construction
            // (integers/bools/strings), and D7 metadata floats compare
            // BY BITS through `MetaValue`'s own `PartialEq` — so
            // structural equality IS bit equality here.
            && self.appearance == other.appearance
            && self.nodes.len() == other.nodes.len()
            && self.nodes.iter().all(|(id, node)| {
                other
                    .nodes
                    .get(id)
                    .is_some_and(|theirs| node.bit_eq(theirs))
            })
            && self.params.len() == other.params.len()
            && self.params.iter().all(|(name, p)| {
                other
                    .params
                    .get(name)
                    .is_some_and(|theirs| p.bit_eq(theirs))
            })
    }
}

// ---------------------------------------------------------------
// The document's REGISTRY KEYS, and what a key may name.
//
// `Doc` carries two maps keyed by node id — the A11 placement registry
// and the witness store — and each holds its key to a NODE KIND: a
// placement names an instance, a witness names a sketch. Both rules are
// asked twice, by the edit door that writes the row and by the
// save/load validator that re-reads it, so both live here, beside the
// maps they are about and where a `Doc` is in scope. A predicate that
// needs only the node is a `Node` method instead (`Node::input_fault`
// and its siblings); these need the document to resolve the key at all.
// ---------------------------------------------------------------

/// What makes an A11 placement row inadmissible
/// ([`placement_fault`]) — one vocabulary for the edit door and the
/// load door's re-check of the registry.
#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) enum PlacementFault {
    /// The key names no live [`Node::InstantiatePart`]. A11 puts the
    /// frame on an instance's cluster, so nothing else has one.
    NotAnInstance,
    /// The frame carries a non-finite coordinate: no predicate can
    /// decide anything about where it puts the material.
    NonFiniteFrame,
    /// The frame is IMPROPER — determinant ≤ 0, i.e. a mirror (A6).
    /// Admitting one is gated on the equivariance audit R4 owns.
    ImproperFrame {
        /// The linear part's determinant.
        determinant: f64,
    },
}

/// **A11's admission rule for one placement row, stated once**: the
/// key instantiates a part, and the frame is finite and proper.
///
/// One predicate with one home, asked by every door that admits a row
/// — [`crate::DocEdit::SetPlacement`] and the load door's walk over the
/// registry — each naming the answer in its own vocabulary. The
/// question is asked in one place, so the two doors cannot disagree
/// about which rows exist.
///
/// What is NOT here is the GAUGE rule (`SnapshotError::PlacementNotGauge`),
/// and that asymmetry is the invariant rather than an omission:
/// `SetPlacement` does not refuse a non-gauge key, it KEYS THE ROW ON
/// THE GAUGE, and the cluster maintenance re-keys the whole registry
/// whenever the mate graph moves ([`crate::mate::solve::reconcile`]).
/// A non-gauge row is therefore unrepresentable through the edit doors
/// and needs no refusal there; it is reachable only in a file, which is
/// the door that asks.
pub(crate) fn placement_fault<P>(
    doc: &Doc<P>,
    node: RecipeNodeId,
    frame: &crate::placement::Frame,
) -> Option<PlacementFault> {
    if !matches!(doc.nodes.get(&node), Some(Node::InstantiatePart { .. })) {
        return Some(PlacementFault::NotAnInstance);
    }
    // The frame half is the frame's own rule
    // ([`crate::placement::Frame::admission_fault`]), so a cluster
    // frame and a placement rule's listed frames are held to one
    // standard rather than to two spellings of one.
    Some(match frame.admission_fault()? {
        crate::placement::FrameFault::NonFinite => PlacementFault::NonFiniteFrame,
        crate::placement::FrameFault::Improper { determinant } => {
            PlacementFault::ImproperFrame { determinant }
        }
    })
}

/// What makes a witness row's KEY inadmissible ([`witness_site_fault`])
/// — one vocabulary for the witness edit doors and the load door's
/// re-check of the store.
#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) enum WitnessSiteFault {
    /// The key names no live node at all.
    NoSuchNode,
    /// The key names a live node that bears no sketch, so there is no
    /// branch for a witness to record a choice about.
    NotSketchBearing,
}

/// **The witness store's key rule, stated once**: a witness is
/// attached to a live node that bears a sketch ([`Node::Profile`] —
/// the v1 sketch node kind; mates extend this at their milestone).
///
/// One predicate with one home, asked by every door that writes a row
/// — [`crate::DocEdit::ReWitness`] and
/// [`crate::DocEdit::ReWitnessBulk`] — and by the load door's walk over
/// the store, each naming the answer in its own vocabulary. It is the
/// same shape as [`placement_fault`]'s first arm, and for the same
/// reason: a registry key names a node of a required kind, and the two
/// doors must agree on which keys exist.
pub(crate) fn witness_site_fault<P>(doc: &Doc<P>, node: RecipeNodeId) -> Option<WitnessSiteFault> {
    match doc.nodes.get(&node) {
        None => Some(WitnessSiteFault::NoSuchNode),
        Some(Node::Profile(_)) => None,
        Some(_) => Some(WitnessSiteFault::NotSketchBearing),
    }
}

/// **The recorded ε's admission rule, stated once**: finite and
/// strictly positive.
///
/// One predicate with one home, asked by the edit door that records an
/// ε ([`crate::DocEdit::SetTolerance`]) and by the save/load
/// validator's snapshot walk, each naming the answer in its own
/// vocabulary. ε parameterizes every predicate band in the document, so
/// a value the two doors disagreed about would be a document whose
/// every geometric answer depends on which door it came through.
///
/// A free function beside the field rather than a `Doc` method: the
/// edit door decides the value BEFORE it is a document's ε, and a
/// method would have nothing to be called on there.
pub(crate) fn epsilon_admissible(eps: f64) -> bool {
    eps.is_finite() && eps > 0.0
}

#[cfg(test)]
mod tests {
    #![allow(clippy::panic, clippy::expect_used)]

    use super::{Carrier, Doc, NameCarrier};
    use crate::appearance::AppearanceRecord;
    use crate::ident::{ContentPin, DocRef, DocumentId};
    use crate::mate::ContactClass;
    use crate::names::{EntityKind, FaceName, StableName};
    use crate::node::{InterfaceCrossing, InterfaceRecord, Node, RecipeNodeId, SitedRef};
    use crate::program::ProfileDoc;
    use geom_core::Tol;

    test_utils::f6_variants! {
        /// **The document's name carriers**, welded to [`Carrier`] by
        /// the match the macro writes: a carrier added to the enum
        /// leaves it non-exhaustive, and the census below compares
        /// this roster against [`Carrier::ALL`] in both directions.
        const CARRIER: Carrier = [Payloads, Appearance];
    }

    fn name(node: u64, kind: EntityKind) -> StableName {
        StableName {
            kind,
            node: RecipeNodeId(node),
            path: Vec::new(),
        }
    }

    /// **A carrier [`Carrier::ALL`] does not name never walks**:
    /// [`Doc::name_carriers`] is driven by the array.
    ///
    /// One half of the weld is the compiler's: a variant added to
    /// [`Carrier`] does not compile until `Doc::names_in` places it.
    /// The compiler does NOT say the new variant reached the array —
    /// adding one leaves the array's length alone, so a carrier can
    /// be placed, be readable, and be walked by nobody. That is the
    /// case this row holds, and it is the likely one: the author who
    /// adds a field to [`Doc`] is pushed to the match by a broken
    /// build and to the array by nothing.
    ///
    /// A carrier REMOVED from the array is the compiler's again, but
    /// only halfway: dropping an entry alone is a type error, since
    /// the array's length is its type; dropping the length with it
    /// leaves a variant nothing constructs, which is a `dead_code`
    /// warning and an error under the gate's `-D warnings` — for as
    /// long as nothing else constructs it. This row is the answer
    /// that does not depend on that.
    ///
    /// **What it does NOT say.** It says every carrier is WALKED, not
    /// that the arm walking it reads its own field: an arm that read
    /// the wrong field, or yielded nothing, passes this row. Held
    /// elsewhere, and measured rather than assumed — an `Appearance`
    /// arm that yields nothing reds thirteen rows across the crate's
    /// two test binaries, the order row below among them, because
    /// every reader of the enumeration is a reader of that arm. A
    /// per-carrier non-empty row would say nothing those thirteen do
    /// not, so there is none.
    ///
    /// Nor does it say a third carrier gets a `NameCarrier` shape
    /// that suits it. `Payload`/`Store` is a ruled design decision
    /// about the two that exist — a carrying node or none — and a
    /// third that is neither is a design question this row cannot
    /// pose, only the build it breaks can.
    #[test]
    fn the_carrier_roster_is_what_the_walk_iterates() {
        let walked: Vec<String> = Carrier::ALL
            .iter()
            .map(test_utils::f6::variant_identifier)
            .collect();
        let walked: Vec<&str> = walked.iter().map(String::as_str).collect();
        if let Some(report) = test_utils::census::set_difference(
            CARRIER.identifiers(),
            &walked,
            "the `Carrier` roster and `Carrier::ALL` disagree",
            "in `Carrier::ALL` and absent from the roster",
            "in the roster and absent from `Carrier::ALL`, which is what `Doc::name_carriers` \
             iterates — so this carrier is never walked",
        ) {
            panic!("{report}");
        }
    }

    /// **Both carriers, in the contracted order**: every payload name
    /// in document order, then every store key in the store's own
    /// order.
    ///
    /// The fixture makes both halves of that falsifiable. The two
    /// nodes sit in the document in the REVERSE of their id order, so
    /// a walk over the node map instead of `Doc::order` swaps the
    /// first two rows; and both store keys are minted by a node that
    /// sorts BEFORE either payload name, so a walk that merged the
    /// two carriers into one sorted list — or ran the store first —
    /// puts them at the front instead of the back.
    ///
    /// DM7's report is this walk filtered on the deleted node, so
    /// this order is the clause's payload-strands-before-store-strands
    /// order; `dm7_delete_strands` holds that end of it at the door.
    /// The fixture is built by poking `Doc`'s fields, not through
    /// the edit doors, because the reversal it has to be able to
    /// exhibit is one no edit door can mint. The doors touch
    /// `Doc::order` twice — `InsertNode` pushes the id it has just
    /// minted, `DeleteNode` retains — and ids are minted
    /// monotonically, so through the doors the order is ascending by
    /// id always and the reversal is unreachable. A loaded document
    /// can hold any order. In-crate reach spells that state in five
    /// lines and keeps the row's subject the walk rather than the
    /// door.
    #[test]
    fn name_carriers_reads_the_payloads_then_the_store() {
        let mut doc: ProfileDoc = Doc::empty_derived("carriers", Tol::witness());
        let first = name(9, EntityKind::Face);
        let second = name(9, EntityKind::Edge);
        let third = name(10, EntityKind::Face);
        let painted_a = name(0, EntityKind::Face);
        let painted_b = name(1, EntityKind::Face);

        // Declared in id order, ordered in the document backwards.
        doc.nodes.insert(
            RecipeNodeId(0),
            Node::Declare {
                pairs: vec![(
                    (
                        SitedRef::at_mint(first.clone()),
                        SitedRef::at_mint(second.clone()),
                    ),
                    ContactClass::Rest,
                )],
            },
        );
        doc.nodes.insert(
            RecipeNodeId(1),
            Node::Declare {
                pairs: vec![(
                    (
                        SitedRef::at_mint(third.clone()),
                        SitedRef::at_mint(third.clone()),
                    ),
                    ContactClass::Tangent,
                )],
            },
        );
        // The third payload shape this walk reaches: an instance's
        // interface record. Its crossing's `outer` is a name in THIS
        // document and is carried by the INSTANCE — which is what a
        // DM7 strand over it names — while its `inner` is no name of
        // this document at all (`Node::payload_names`' arm says why),
        // so its absence below is asserted by the same equality.
        let crossed = name(11, EntityKind::Face);
        doc.nodes.insert(
            RecipeNodeId(2),
            Node::InstantiatePart {
                doc_ref: DocRef {
                    id: DocumentId::derive("carriers-part"),
                    pin: ContentPin([7u8; 32]),
                },
                interface: InterfaceRecord {
                    crossings: vec![InterfaceCrossing::Mate {
                        class: ContactClass::Rest,
                        outer: FaceName::new(crossed.clone()).expect("the fixture spells a face"),
                        inner: FaceName::new(name(12, EntityKind::Face))
                            .expect("the fixture spells a face"),
                    }],
                },
            },
        );
        doc.order = vec![RecipeNodeId(2), RecipeNodeId(1), RecipeNodeId(0)];
        for key in [&painted_b, &painted_a] {
            doc.appearance
                .insert(key.clone(), AppearanceRecord::default());
        }

        assert_eq!(
            doc.name_carriers().collect::<Vec<_>>(),
            vec![
                NameCarrier::Payload {
                    node: RecipeNodeId(2),
                    name: &crossed,
                },
                NameCarrier::Payload {
                    node: RecipeNodeId(1),
                    name: &third,
                },
                NameCarrier::Payload {
                    node: RecipeNodeId(1),
                    name: &third,
                },
                NameCarrier::Payload {
                    node: RecipeNodeId(0),
                    name: &first,
                },
                NameCarrier::Payload {
                    node: RecipeNodeId(0),
                    name: &second,
                },
                NameCarrier::Store { name: &painted_a },
                NameCarrier::Store { name: &painted_b },
            ],
            "the walk is `Carrier::ALL`'s order: document order over the payloads, then the \
             store's own key order"
        );
    }
}
