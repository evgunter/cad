//! **What a finding's assertion recourse determines, as data**
//! (DESIGN `[one-way.places]`, "A refusal's recourse"): the assertion
//! that would quiet an unproven coincidence ([`Quieting`]), and the two
//! faces a `Gap` that would quiet an interference is written over
//! ([`OpposedPair`]). Each is minted by the quieting rule that later
//! reads the authored assertion (`at_rest::quieting`,
//! `at_rest::localize`), and the finding's recourse is worded from it.

use crate::Formula;
use crate::doc::Doc;
use crate::edit::{DocEdit, EditError, Maintenance, Recorded, Recording};
use crate::measure::{AssertionRelation, Claim, MeasurePrimitive, MeasureVerb};
use crate::names::StableName;
use crate::node::{Node, RecipeNodeId};
use crate::operand::{BodyRead, Operand};
use crate::var::{VarId, VarKind};

use super::at_rest::FaceSite;

/// **The measure a [`Quieting`] relates to zero.**
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum QuietingMeasure {
    /// [`MeasurePrimitive::Gap`], its references `outer` then `inner`.
    Gap,
    /// [`MeasurePrimitive::Distance`], its references `a` then `b`.
    Distance,
}

impl QuietingMeasure {
    /// The measure's verb.
    #[must_use]
    pub const fn verb(self) -> MeasureVerb {
        match self {
            Self::Gap => MeasureVerb::Gap,
            Self::Distance => MeasureVerb::Distance,
        }
    }

    /// The measure as a claim spells it.
    #[must_use]
    pub const fn word(self) -> &'static str {
        match self {
            Self::Gap => "Gap",
            Self::Distance => "Distance",
        }
    }

    pub(crate) const fn of(verb: MeasureVerb) -> Option<Self> {
        match verb {
            MeasureVerb::Gap => Some(Self::Gap),
            MeasureVerb::Distance => Some(Self::Distance),
            MeasureVerb::Angle => None,
        }
    }
}

/// **The pose a [`QuietingRef`] reads off its cell.**
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum CellPose {
    /// The axis of a cylinder, a cone or a torus
    /// ([`crate::pose::PoseDef::Axis`]).
    Axis,
    /// The centre of a sphere ([`crate::pose::PoseDef::Point`]).
    Point,
}

impl CellPose {
    /// The kind of the pose variable the reference reads.
    #[must_use]
    pub const fn kind(self) -> VarKind {
        match self {
            Self::Axis => VarKind::Axis,
            Self::Point => VarKind::Point,
        }
    }

    pub(crate) const fn of(kind: VarKind) -> Option<Self> {
        match kind {
            VarKind::Axis => Some(Self::Axis),
            VarKind::Point => Some(Self::Point),
            _ => None,
        }
    }
}

/// **One reference of a [`Quieting`]'s measure, as the measure reads
/// it.**
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum QuietingRef {
    /// The entity `name` names in `read`'s table, read as that cell, or
    /// as the pose read directly off it.
    Entity {
        /// The body read.
        read: VarId,
        /// The entity's name in that read's table.
        name: StableName,
        /// The pose read off the cell, `None` for the cell itself.
        pose: Option<CellPose>,
    },
    /// A variable read as it is: the plane, or the frame read as its xy
    /// plane, a split cuts with.
    Read(VarId),
}

impl QuietingRef {
    /// The reference as an author writes it at a measure's seat: a
    /// selection of the one name in the body read plainly, a pose
    /// defined off that selection, or the variable.
    #[must_use]
    pub fn operand(&self) -> Operand {
        match self {
            Self::Read(var) => Operand::Var(*var),
            Self::Entity { read, name, pose } => {
                let cell = Operand::Select {
                    body: Box::new(BodyRead::plain(Operand::Var(*read))),
                    names: vec![name.clone()],
                };
                match pose {
                    None => cell,
                    Some(CellPose::Axis) => {
                        Operand::Pose(Box::new(crate::pose::PoseDef::Axis { of: cell }))
                    }
                    Some(CellPose::Point) => {
                        Operand::Pose(Box::new(crate::pose::PoseDef::Point { of: cell }))
                    }
                }
            }
        }
    }
}

/// **The assertion that would quiet an unproven coincidence** (D10
/// `[assertions.quiet]`): `measure(references) = 0`. The rule that
/// decides which holding assertion quiets a row
/// (`at_rest::quiet_row`) mints it from the same reading of the row, so
/// authoring [`Self::edits`] quiets the row it was minted for.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Quieting {
    /// The measure related to zero.
    pub measure: QuietingMeasure,
    /// Its two references, in the measure's order.
    pub references: [QuietingRef; 2],
}

/// **A [`Quieting`] authored** ([`Quieting::edits`]).
#[derive(Debug)]
pub struct QuietingEdits<P: crate::ProfilePayload> {
    /// The document the edits produce.
    pub doc: Doc<P>,
    /// The edits, in order: the naming of a split's unnamed tool, the
    /// measure's insert, which mints each reference's selection and
    /// pose, then the assertion's.
    pub edits: Vec<DocEdit<P>>,
    /// The maintenance they reported.
    pub maintenance: Vec<Maintenance>,
    /// The measure node.
    pub measure: RecipeNodeId,
    /// The assertion node, which the quieted finding names.
    pub assertion: RecipeNodeId,
}

impl Quieting {
    /// The relation the claim holds its measure to its bound by.
    pub const RELATION: AssertionRelation = AssertionRelation::Equal;

    /// The measure over its references, each as an author writes it
    /// ([`QuietingRef::operand`]).
    #[must_use]
    pub fn primitive(&self) -> MeasurePrimitive<Operand> {
        let [a, b] = &self.references;
        let (a, b) = (a.operand(), b.operand());
        match self.measure {
            QuietingMeasure::Gap => MeasurePrimitive::Gap { outer: a, inner: b },
            QuietingMeasure::Distance => MeasurePrimitive::Distance { a, b },
        }
    }

    /// The claim over the measure's output `value`: `value = 0`.
    #[must_use]
    pub fn claim(value: VarId) -> Claim<Formula> {
        Claim::Value {
            value: Formula::var(value, crate::Dimension::Length),
            bound: Formula::zero(crate::Dimension::Length),
            relation: Self::RELATION,
        }
    }

    /// **The edits that author it in `doc`**: a [`Node::Measure`] over
    /// [`Self::primitive`], then a [`Node::Assertion`] of
    /// [`Self::claim`] over its output. A variable the measure reads as
    /// it is ([`QuietingRef::Read`]) gains a reader, and an unnamed
    /// variable has one (VARIABLES VR2), so an unnamed one is named
    /// first ([`DocEdit::RenameVar`]): `tool`, or `tool_2` and on where
    /// the document holds that name.
    ///
    /// # Errors
    ///
    /// The insert door's refusals; `doc` is untouched.
    pub fn edits<P: Clone + crate::ProfilePayload>(
        &self,
        doc: &Doc<P>,
        tol: geom_core::Tol,
    ) -> Result<QuietingEdits<P>, EditError> {
        let mut action = Recording::start(doc, tol);
        for reference in &self.references {
            let QuietingRef::Read(var) = reference else {
                continue;
            };
            let shared_unnamed = doc.var_name(*var).is_none()
                && doc.var(*var).is_some_and(|v| v.def().output().is_none());
            if shared_unnamed {
                action.apply(DocEdit::RenameVar {
                    var: (*var).into(),
                    name: Some(fresh_name(doc)),
                })?;
            }
        }
        let measured = action.measure(&[self.primitive()])?;
        let (measure, value) = (measured.measures[0], measured.outputs[0]);
        let assertion = action.insert(Node::Assertion(Self::claim(value)))?;
        let Recorded {
            doc,
            edits,
            maintenance,
            ..
        } = action.finish()?;
        Ok(QuietingEdits {
            doc,
            edits,
            maintenance,
            measure,
            assertion,
        })
    }

    /// The claim in words, each reference as `by` speaks it, a cell by
    /// its name: `Gap(the end cap of …, the start cap of …) = 0`.
    #[must_use]
    pub fn said(&self, by: crate::spoken::Speaker<'_>) -> String {
        let reference = |r: &QuietingRef| match r {
            QuietingRef::Read(var) => by.read(*var),
            QuietingRef::Entity { name, pose, .. } => {
                let cell = by.name(name).to_string();
                match pose {
                    None => cell,
                    Some(CellPose::Axis) => format!("the axis of {cell}"),
                    Some(CellPose::Point) => format!("the centre of {cell}"),
                }
            }
        };
        let [a, b] = &self.references;
        format!(
            "{}({}, {}) = 0",
            self.measure.word(),
            reference(a),
            reference(b)
        )
    }
}

/// A name `doc` does not hold, for a split's tool a second reader
/// reads: `tool`, else the first of `tool_2`, `tool_3`, … that is free.
fn fresh_name<P: crate::ProfilePayload>(doc: &Doc<P>) -> crate::VarName {
    let free = |name: &str| doc.var_named(name).is_none();
    let text = if free("tool") {
        "tool".to_owned()
    } else {
        (2u32..)
            .map(|n| format!("tool_{n}"))
            .find(|name| free(name))
            .unwrap_or_else(|| unreachable!("a document holds finitely many names"))
    };
    crate::VarName::new(text).unwrap_or_else(|fault| unreachable!("a written name: {fault}"))
}

/// **The two faces a `Gap` that would quiet an interference is written
/// over** (D10 `[assertions.quiet]`): one face of each copy, opposed,
/// their gap decided negative, the overlap lying between their
/// carriers. The bound is the author's: any `Gap(outer, inner) ≤ −δ`
/// that holds quiets the finding.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct OpposedPair {
    /// The `Gap`'s outer face.
    pub outer: FaceSite,
    /// The `Gap`'s inner face.
    pub inner: FaceSite,
}

impl OpposedPair {
    /// The pair in words, each face as `by` speaks its name, outer then
    /// inner.
    #[must_use]
    pub fn said(&self, by: crate::spoken::Speaker<'_>) -> String {
        format!(
            "{} and {}",
            by.name(&self.outer.face),
            by.name(&self.inner.face)
        )
    }
}
