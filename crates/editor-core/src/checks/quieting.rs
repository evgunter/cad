//! **What a finding's assertion recourse determines, as data**
//! (DESIGN `[one-way.places]`, "A refusal's recourse"): the assertion
//! that would quiet an unproven coincidence ([`Quieting`]), and the two
//! faces a `Gap` that would quiet an interference is written over
//! ([`OpposedPair`]). Each is minted by the quieting rule that later
//! reads the authored assertion: the first by `at_rest::quieting`, which
//! keeps only what `at_rest::quiet_row` takes, the second by
//! `at_rest::could_quiet`, which asks `at_rest::gap_assertions` and
//! `at_rest::quiet_verdicts`. The finding's recourse is worded from it.

use crate::Formula;
use crate::doc::Doc;
use crate::edit::{DocEdit, EditError, Recorded, Recording};
use crate::measure::{AssertionRelation, Claim, MeasurePrimitive, MeasureVerb};
use crate::names::StableName;
use crate::node::Node;
use crate::operand::{BodyRead, Operand};
use crate::var::{VarId, VarKind};

use super::at_rest::FaceSite;

/// **One reference of a [`Quieting`]'s measure, as the measure reads
/// it** — a value only the kernel mints, by the rule that reads it back
/// as the row's cell. Its reading is opaque: the parts are read through
/// [`Self::read`], [`Self::name`] and [`Self::pose`], and an author's
/// seat is [`Self::operand`]. So a reference cannot be built, or
/// rewritten, from a finding's raw cell names, which are keyed by the
/// `⊔` that gathered them and would be admitted and then fail to
/// resolve.
///
/// ```compile_fail,E0599
/// // No constructor is public: a variable cannot be written as one.
/// let _ = editor_core::QuietingRef::Read(editor_core::VarId::WORLD);
/// ```
///
/// ```compile_fail,E0616
/// // Nor can a minted reference's reading be rewritten.
/// fn rewrite(r: &mut editor_core::QuietingRef) {
///     let _ = &mut r.0;
/// }
/// ```
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct QuietingRef(pub(crate) Reference);

/// **What a [`QuietingRef`] reads**, crate-internal so the mint is its
/// one writer.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) enum Reference {
    /// The entity `name` names in `read`'s table, read as that cell, or
    /// as the pose read directly off it: one read directly off a cell,
    /// an axis, a point or a plane.
    Entity {
        /// The body read.
        read: VarId,
        /// The entity's name in that read's table.
        name: StableName,
        /// The kind of the pose read off the cell, `None` for the cell
        /// itself.
        pose: Option<VarKind>,
    },
    /// A variable read as it is: the plane, or the frame read as its xy
    /// plane, a split cuts with.
    Read(VarId),
}

impl QuietingRef {
    /// The entity `name` names in `read`'s table, as itself or as the
    /// pose `pose` read directly off it.
    pub(crate) fn entity(read: VarId, name: StableName, pose: Option<VarKind>) -> Self {
        Self(Reference::Entity { read, name, pose })
    }

    /// The variable `var`, read as it is.
    pub(crate) fn variable(var: VarId) -> Self {
        Self(Reference::Read(var))
    }

    /// **The variable the reference reads**: the body read its cell is
    /// named in, or the variable read as it is.
    #[must_use]
    pub fn read(&self) -> VarId {
        match &self.0 {
            Reference::Entity { read, .. } | Reference::Read(read) => *read,
        }
    }

    /// The cell's name in [`Self::read`]'s table, `None` for a variable
    /// read as it is.
    #[must_use]
    pub fn name(&self) -> Option<&StableName> {
        match &self.0 {
            Reference::Entity { name, .. } => Some(name),
            Reference::Read(_) => None,
        }
    }

    /// The kind of the pose read off the cell, `None` for the cell
    /// itself and for a variable read as it is.
    #[must_use]
    pub fn pose(&self) -> Option<VarKind> {
        match &self.0 {
            Reference::Entity { pose, .. } => *pose,
            Reference::Read(_) => None,
        }
    }

    /// The reference as an author writes it at a measure's seat: a
    /// selection of the one name in the body read plainly, a pose
    /// defined off that selection, or the variable.
    #[must_use]
    pub fn operand(&self) -> Operand {
        use crate::pose::PoseDef;
        match &self.0 {
            Reference::Read(var) => Operand::Var(*var),
            Reference::Entity { read, name, pose } => {
                let of = Operand::Select {
                    body: Box::new(BodyRead::plain(Operand::Var(*read))),
                    names: vec![name.clone()],
                };
                let pose = match pose {
                    None => return of,
                    Some(VarKind::Axis) => PoseDef::Axis { of },
                    Some(VarKind::Point) => PoseDef::Point { of },
                    Some(VarKind::Plane) => PoseDef::Plane { face: of },
                    // Only this crate builds an entity reference: the mint
                    // and the rule's decode, over `eval::read_directly_off`,
                    // which reads no other kind off a cell.
                    Some(kind) => unreachable!("a pose read directly off a cell: {kind:?}"),
                };
                Operand::Pose(Box::new(pose))
            }
        }
    }
}

/// **The assertion that would quiet an unproven coincidence** (D10
/// `[assertions.quiet]`): `measure(references) = 0`. The kernel mints it
/// only where the rule that decides which holding assertion quiets a row
/// (`at_rest::quiet_row`) reads it as the row's cells, so authoring
/// [`Self::edits`] quiets the row it was minted for.
///
/// Its parts are read, never written: no other crate can build one, so
/// a value of it is the one the rule minted.
///
/// ```compile_fail,E0451
/// fn forge(r: [editor_core::QuietingRef; 2]) -> editor_core::Quieting {
///     editor_core::Quieting {
///         measure: editor_core::MeasureVerb::Gap,
///         references: r,
///     }
/// }
/// ```
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Quieting {
    measure: MeasureVerb,
    references: [QuietingRef; 2],
}

impl Quieting {
    /// The assertion `measure(references) = 0`, as the mint writes it.
    pub(crate) fn minted(measure: MeasureVerb, references: [QuietingRef; 2]) -> Self {
        Self {
            measure,
            references,
        }
    }

    /// The measure related to zero: a `Gap`, its references outer then
    /// inner, or a `Distance`.
    #[must_use]
    pub fn measure(&self) -> MeasureVerb {
        self.measure
    }

    /// Its two references, in the measure's order.
    #[must_use]
    pub fn references(&self) -> &[QuietingRef; 2] {
        &self.references
    }

    /// The relation the claim holds its measure to its bound by.
    pub const RELATION: AssertionRelation = AssertionRelation::Equal;

    /// **The assertion in `doc` that already says this claim**, its
    /// verdict aside: the measure of this verb over the two references,
    /// in order, related to zero. `None` where no assertion does, which
    /// is when authoring [`Self::edits`] adds one.
    #[must_use]
    pub fn held_in<P: crate::ProfilePayload>(&self, doc: &Doc<P>) -> Option<crate::RecipeNodeId> {
        super::at_rest::asserted(doc, self)
    }

    /// The measure over its references, each as an author writes it
    /// ([`QuietingRef::operand`]).
    #[must_use]
    pub fn primitive(&self) -> MeasurePrimitive<Operand> {
        let [a, b] = &self.references;
        let (a, b) = (a.operand(), b.operand());
        match self.measure {
            MeasureVerb::Gap => MeasurePrimitive::Gap { outer: a, inner: b },
            MeasureVerb::Distance => MeasurePrimitive::Distance { a, b },
            MeasureVerb::Angle => MeasurePrimitive::Angle { a, b },
        }
    }

    /// The claim over the measure's output `value`: `value = 0`, at the
    /// measure's dimension.
    #[must_use]
    pub fn claim(&self, value: VarId) -> Claim<Formula> {
        let dimension = match self.measure {
            MeasureVerb::Angle => crate::Dimension::Angle,
            MeasureVerb::Gap | MeasureVerb::Distance => crate::Dimension::Length,
        };
        Claim::Value {
            value: Formula::var(value, dimension),
            bound: Formula::zero(dimension),
            relation: Self::RELATION,
        }
    }

    /// **The edits that author it in `doc`, recorded as one action**: a
    /// [`Node::Measure`] over [`Self::primitive`], whose insert mints each
    /// reference's selection and pose, then a [`Node::Assertion`] of
    /// [`Self::claim`] over its output, the last node minted. A variable
    /// the measure reads as it is (a reference [`QuietingRef::name`] answers `None` for) gains a reader,
    /// and an unnamed variable has one (VARIABLES VR2), so an unnamed one
    /// is named first ([`DocEdit::RenameVar`]) by [`tool_name`], which
    /// the recourse says it by.
    ///
    /// # Errors
    ///
    /// The insert door's refusals; `doc` is untouched.
    pub fn edits<P: Clone + crate::ProfilePayload>(
        &self,
        doc: &Doc<P>,
        tol: geom_core::Tol,
    ) -> Result<Recorded<P>, EditError> {
        let mut action = Recording::start(doc, tol);
        self.record(&mut action)?;
        action.finish()
    }

    /// **[`Self::edits`], recorded into `action`**, which the caller
    /// finishes: the same edits, each name read off the document
    /// `action` holds as it is reached. Answers the assertion, the last
    /// node it mints.
    ///
    /// # Errors
    ///
    /// The insert door's refusals; `action` answers them from then on.
    pub fn record<P: Clone + crate::ProfilePayload>(
        &self,
        action: &mut Recording<'_, P>,
    ) -> Result<crate::RecipeNodeId, EditError> {
        for reference in &self.references {
            let Reference::Read(var) = &reference.0 else {
                continue;
            };
            // Each name is read off the document as the action holds it
            // so far. The mint reads at most one variable as it is (a
            // split's one tool), and that is the name `said` gives it; a
            // variable read at both references is named once, at the
            // first.
            if let Some(name) = tool_name(action.doc(), *var) {
                action.apply(DocEdit::RenameVar {
                    var: (*var).into(),
                    name: Some(name),
                })?;
            }
        }
        let measured = action.measure(&[self.primitive()])?;
        action.insert(Node::Assertion(self.claim(measured.outputs[0])))
    }

    /// The claim in words, each reference as `by` speaks it, a cell by
    /// its name and a split's unnamed tool by the name [`Self::edits`]
    /// gives it: `Gap(the end cap of …, the start cap of …) = 0`.
    #[must_use]
    pub fn said(&self, by: crate::spoken::Speaker<'_>) -> crate::Sentence {
        let reference = |r: &QuietingRef| match &r.0 {
            Reference::Read(var) => by.tool_name(*var).map_or_else(
                || by.read(*var),
                |name| crate::Sentence::from(name.as_str()),
            ),
            Reference::Entity { name, pose, .. } => {
                let pose = match pose {
                    None => "",
                    Some(VarKind::Axis) => "the axis of ",
                    Some(VarKind::Point) => "the centre of ",
                    Some(_) => "the plane of ",
                };
                crate::sentence!(pose, by.name(name))
            }
        };
        // The measure as a claim spells it: the verb, capitalized.
        let measure = match self.measure {
            MeasureVerb::Gap => "Gap",
            MeasureVerb::Distance => "Distance",
            MeasureVerb::Angle => "Angle",
        };
        let [a, b] = &self.references;
        crate::sentence!(measure, "(", reference(a), ", ", reference(b), ") = 0")
    }
}

/// **The name a split's tool is given when a measure reads it too**,
/// where `doc` holds `var` unnamed and as no output: `tool`, else the
/// first of `tool_2`, `tool_3`, … `doc` does not hold. `None` for a
/// variable that needs none.
pub(crate) fn tool_name<P: crate::ProfilePayload>(
    doc: &Doc<P>,
    var: VarId,
) -> Option<crate::VarName> {
    let unnamed = doc.var_name(var).is_none() && doc.var(var)?.def().output().is_none();
    if !unnamed {
        return None;
    }
    let free = |name: &str| doc.var_named(name).is_none();
    let text = std::iter::once("tool".to_owned())
        .chain((2u32..).map(|n| format!("tool_{n}")))
        .find(|name| free(name))?;
    crate::VarName::new(text).ok()
}

/// **The two faces a `Gap` that would quiet an interference is written
/// over** (D10 `[assertions.quiet]`): one face of each copy, opposed,
/// their gap decided negative, the overlap lying between their
/// carriers, and the two references an author writes the `Gap` over.
/// The bound is the author's: any `Gap(references) ≤ −δ` that holds at
/// every member of the assertion quiets the finding.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct OpposedPair {
    /// The `Gap`'s outer face, as the finding's copy names it.
    pub outer: FaceSite,
    /// The `Gap`'s inner face, likewise.
    pub inner: FaceSite,
    /// The two references as an author writes them, outer then inner:
    /// each a selection of one name in its copy's read
    /// ([`QuietingRef::operand`]), a family member's face by the name
    /// its member wraps, so that the assertion reaches the family's index
    /// and each member reads its own face. The kernel mints them only
    /// where the rule reads them back as [`Self::outer`] and
    /// [`Self::inner`].
    pub references: [QuietingRef; 2],
}

impl OpposedPair {
    /// The pair in words, each reference's face as `by` speaks its name,
    /// outer then inner.
    #[must_use]
    pub fn said(&self, by: crate::spoken::Speaker<'_>) -> crate::Sentence {
        let face = |r: &QuietingRef| match &r.0 {
            Reference::Entity { name, .. } => crate::sentence!(by.name(name)),
            Reference::Read(var) => by.read(*var),
        };
        let [outer, inner] = &self.references;
        crate::sentence!(face(outer), " and ", face(inner))
    }
}
