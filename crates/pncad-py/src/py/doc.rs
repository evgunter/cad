//! The document surface: `Doc`, `DocEdit`, `Node`, `evaluate`.
//!
//! Python speaks Doc/DocEdit/evaluate/persist and **never an arena
//! key**. The only identifier that crosses is [`NodeId`], a
//! wrapper over `RecipeNodeId` — a recipe-level id, which is precisely
//! the document layer's own public vocabulary, not a slotmap key.

use pyo3::prelude::*;
use pyo3::types::{PyDict, PyString};

use crate::errors::{BoundaryEdit, ErrorClass};
use crate::py::typed_err;
use crate::tags::{edit_error_tag, edit_inner_variant_tag, persist_error_tag, workspace_error_tag};
use pncad::document as d;
use pncad::tolerance::Tol;

/// A point given as three lengths, as `Doc.corner_frame` takes each of
/// its corners.
type Lengths3 = (
    super::quantity::Length,
    super::quantity::Length,
    super::quantity::Length,
);

/// The refusing ARM's word, as an `EditError` carries it: the inner
/// refusal's own discriminant beside the carrier's, `None` where the
/// arm holds no inner refusal.
fn inner_variant(py: Python<'_>, tag: Option<&'static str>) -> Py<PyAny> {
    match tag {
        Some(tag) => PyString::new(py, tag).unbind().into_any(),
        None => py.None().into_any(),
    }
}

/// A number whose conversion into Python is INFALLIBLE — the integer
/// and float widths this door carries, whose `IntoPyObject` error type
/// is `Infallible`. The match is what says so: it has one arm because
/// there is one, and nothing here degrades to `None` for a reason that
/// cannot arise.
fn infallible<'py, T>(converted: Result<Bound<'py, T>, std::convert::Infallible>) -> Py<PyAny> {
    match converted {
        Ok(value) => value.into_any().unbind(),
    }
}

/// Every attribute an `EditError` carries, for one arm.
///
/// The names are the kernel's own field names where the kernel gives
/// one concept one name; where two arms name one concept differently
/// the concept's clearest word wins and `crate::edit_payload` states
/// the mapping at the arm. Every attribute is present at every raise
/// site of the class — the document layer's refusals and the three the boundary builds itself — so `getattr`
/// never raises and a caller need not branch on `variant` first.
fn edit_fields(
    py: Python<'_>,
    variant: &str,
    inner: Option<&'static str>,
    payload: &crate::edit_payload::EditPayload<'_>,
) -> [(&'static str, Py<PyAny>); 27] {
    let none = || py.None();
    // A field whose own construction failed degrades to `None` rather
    // than replacing the kernel's refusal with a boundary one: the
    // caller asked why the edit refused, and that answer must survive
    // a failure to build one of its attributes.
    let obj = |v: PyResult<Py<PyAny>>| v.unwrap_or_else(|_| py.None());
    let opt = |v: Option<PyResult<Py<PyAny>>>| v.map_or_else(none, obj);
    let text = |s: &str| PyString::new(py, s).unbind().into_any();
    let word = |w: Option<&str>| w.map_or_else(none, text);
    let node =
        |n: Option<d::RecipeNodeId>| opt(n.map(|n| Py::new(py, NodeId(n)).map(Py::into_any)));
    let kind = |k: Option<pncad::select::EntityKind>| {
        opt(k.map(|k| Py::new(py, super::select::entity_kind(k)).map(Py::into_any)))
    };
    let num = |v: Option<Py<PyAny>>| v.unwrap_or_else(none);

    [
        ("variant", text(variant)),
        ("inner_variant", inner_variant(py, inner)),
        ("node", node(payload.node)),
        ("input", node(payload.input)),
        ("slot", word(payload.slot)),
        ("param", word(payload.param.map(|p| p.as_str()))),
        (
            "name",
            opt(payload.name.map(|n| name_text(py, n).map(|s| text(&s)))),
        ),
        ("key", word(payload.key)),
        ("expected", word(payload.expected)),
        ("found", word(payload.found)),
        ("kind", word(payload.kind)),
        ("from_kind", kind(payload.from_kind)),
        ("to_kind", kind(payload.to_kind)),
        ("found_kind", kind(payload.found_kind)),
        ("expected_kind", kind(payload.expected_kind)),
        (
            "count",
            num(payload.count.map(|n| infallible(n.into_pyobject(py)))),
        ),
        (
            "given",
            num(payload.given.map(|n| infallible(n.into_pyobject(py)))),
        ),
        (
            "first",
            num(payload.first.map(|n| infallible(n.into_pyobject(py)))),
        ),
        (
            "again",
            num(payload.again.map(|n| infallible(n.into_pyobject(py)))),
        ),
        (
            "value",
            num(payload.value.map(|v| infallible(v.into_pyobject(py)))),
        ),
        (
            "offered",
            num(payload.offered.map(|v| match v {
                d::FreeValue::Continuous(v) => infallible(v.into_pyobject(py)),
                d::FreeValue::Count(n) => infallible(n.into_pyobject(py)),
            })),
        ),
        (
            "index",
            num(payload.index.map(|n| infallible(n.into_pyobject(py)))),
        ),
        (
            "mate",
            opt(payload.node.zip(payload.constraint).map(|(placement, id)| {
                Py::new(py, super::mate::MateRef(d::MateRef::new(placement, id))).map(Py::into_any)
            })),
        ),
        (
            "path",
            opt(payload.path.map(|p| {
                pyo3::types::PyTuple::new(py, p.iter().map(|i| u32::from(*i)))
                    .map(|t| t.into_any().unbind())
            })),
        ),
        ("value_path", word(payload.value_path)),
        (
            "pin",
            opt(payload
                .pin
                .map(|p| Py::new(py, super::store::ContentPin(p)).map(Py::into_any))),
        ),
        (
            "fault",
            opt(payload.fault.map(|(fault, held)| {
                let voice = super::mate::Voice::Held(held.clone());
                Py::new(py, super::mate::MateFault(fault.clone(), voice)).map(Py::into_any)
            })),
        ),
    ]
}

/// Raise `EditError` carrying the refusal's stable tag and the arm's
/// payload.
///
/// `variant` is the CARRIER's word — which edit refused —
/// `inner_variant` is the arm of the refusal that word holds, `None`
/// where it holds none, and the rest is the arm's payload
/// (`crate::edit_payload`). All of them are always present.
pub(crate) fn edit_err(py: Python<'_>, err: &d::EditError) -> PyErr {
    // `EditError` implements `Display`: the human message is real
    // prose; the machine payload is the `variant` tag (see
    // `crate::tags`) and the arm's fields.
    edit_err_saying(py, err, err.to_string())
}

/// [`edit_err`] with a message of the raising door's own: a door that
/// raises an `EditError` arm for a refusal that is not an edit states
/// the arm's problem (`EditError::problem`) and its own recourse,
/// since the edit door's recourse is about an edit nobody made. The
/// tag and payload are the arm's, as at every other site.
fn edit_err_saying(py: Python<'_>, err: &d::EditError, message: String) -> PyErr {
    typed_err(
        py,
        ErrorClass::Edit,
        message,
        &edit_fields(
            py,
            edit_error_tag(err),
            edit_inner_variant_tag(err),
            &crate::edit_payload::edit_payload(err),
        ),
    )
}

/// Raise `EditError` for a refusal the BOUNDARY built — a
/// [`BoundaryEdit`].
///
/// It has a `variant`, the `inner_variant` the same kernel arm
/// publishes at its own door, and nothing else to carry; the attributes
/// the document layer's arms fill are present and `None`: the class's
/// shape is one shape at every raise site, whichever side of the
/// boundary decided it.
///
/// **Where the `variant` comes from**, and it is not "always
/// `crate::tags`": the test is whether a kernel enum arm stands
/// behind the refusal. Where one does, the refusal carries the kernel
/// VALUE and the words are that enum's own maps', even though the raise
/// site is here — a `label=` text that is not a label
/// (`label_fault_tag`) and a body `Doc.corner_frame` reads no frame off
/// (`pin_refusal_tag`) are the cases, and forwarding the value is what
/// keeps one map the only speller of its words. Where
/// none does — a `serde_json` failure, a mate head or parameter name
/// whose constructor refuses with one struct — the word is minted in
/// `crate::tags`, where the tag inventory reads it.
///
/// **This door's set of refusals is closed** because it takes a
/// [`BoundaryEdit`] rather than a word: another boundary refusal is a
/// variant of that enum and an arm of
/// [`crate::tags::boundary_edit_tag`] before it can be raised. It is
/// the door that is closed, not the class — `EditError.variant`'s
/// other words are the document layer's, so nothing type-level stops a
/// site raising [`ErrorClass::Edit`] with a `variant` of its own.
fn boundary_edit_err(py: Python<'_>, refusal: BoundaryEdit<'_>, message: String) -> PyErr {
    typed_err(
        py,
        ErrorClass::Edit,
        message,
        &edit_fields(
            py,
            crate::tags::boundary_edit_tag(refusal),
            crate::tags::boundary_edit_inner_tag(refusal),
            &crate::edit_payload::EditPayload::NONE,
        ),
    )
}

/// Raise `PersistError` carrying the refusal's stable tag and the
/// arm's payload.
///
/// One door for the whole persistence vocabulary, the pin doors in
/// `crate::py::store` included: they run the same validator and the
/// same canonical serializer, so their refusals are the words
/// `Doc.save` and `load` already speak.
///
/// The machine payload is `variant` plus the fields, each present on
/// every arm and `None` where that arm does not carry it, so
/// `getattr` never raises and a caller reads the payload without
/// first branching on `variant`. The tuple is positional and the
/// match is exhaustive, so an arm added kernel-side arrives here as a
/// compile error rather than as a silently unprojected payload.
///
/// Two names are shared by arms that carry one concept under
/// different spellings, and the mapping is stated at the match:
/// `detail` is the underlying reporter's own words (the serializer's,
/// the JSON reader's, the deserializer's), and `document` is the
/// document's recorded ε, whichever arm reports it.
///
/// Four arms carry a NESTED refusal — a profile-program fault, a
/// distribution fault, a snapshot refusal, a replayed edit's own
/// `EditError`. Each crosses as its word on `inner_variant`; the
/// nested arm's own payload is the inner door's surface (a
/// `DistributionFault` has its own exception class, and an
/// `EditError` its own projection) and stays in the message here.
pub(crate) fn persist_err(py: Python<'_>, err: &d::PersistError) -> PyErr {
    use d::PersistError as E;

    let none = || py.None();
    // A field whose own construction failed degrades to `None` rather
    // than replacing the kernel's refusal with a boundary one: the
    // caller asked why the persistence door refused, and that answer
    // must survive a failure to build one of its attributes.
    let obj = |v: PyResult<Py<PyAny>>| v.unwrap_or_else(|_| py.None());
    let text = |s: &str| PyString::new(py, s).unbind().into_any();
    // `usize`'s and `f64`'s conversions are INFALLIBLE (their error
    // type is `Infallible`), so these two degrade nowhere: the match
    // is total.
    let int = |n: usize| -> Py<PyAny> {
        match n.into_pyobject(py) {
            Ok(value) => value.into_any().unbind(),
        }
    };
    let real = |x: f64| -> Py<PyAny> {
        match x.into_pyobject(py) {
            Ok(value) => value.into_any().unbind(),
        }
    };
    let node = |n: d::RecipeNodeId| obj(Py::new(py, NodeId(n)).map(|v| v.into_any()));
    let dim = |d: d::Dimension| text(crate::errors::dimension_tag(d));
    let word = |tag: &'static str| inner_variant(py, Some(tag));

    let (
        inner,
        site,
        which,
        name,
        unit,
        declared,
        detail,
        found,
        header,
        snapshot,
        line,
        column,
        index,
        process,
        document,
    ) = match err {
        // The site is a RECURSIVE descriptor (an edit's index wrapping
        // the site inside that edit's payload), so it crosses as the
        // kernel's own prose for where the float sits rather than as a
        // field per rung.
        E::NonFinite { site } => (
            none(),
            text(&site.to_string()),
            none(),
            none(),
            none(),
            none(),
            none(),
            none(),
            none(),
            none(),
            none(),
            none(),
            none(),
            none(),
            none(),
        ),
        E::ProfileProgram { node: n, fault } => (
            word(crate::tags::program_fault_tag(fault)),
            none(),
            node(n.id()),
            none(),
            none(),
            none(),
            none(),
            none(),
            none(),
            none(),
            none(),
            none(),
            none(),
            none(),
            none(),
        ),
        E::Distribution { var: p, fault } => (
            word(crate::tags::distribution_fault_tag(fault)),
            none(),
            none(),
            text(&p.to_string()),
            none(),
            none(),
            none(),
            none(),
            none(),
            none(),
            none(),
            none(),
            none(),
            none(),
            none(),
        ),
        E::DisplayUnit {
            var: p,
            unit: measures,
            declared: was,
        } => (
            none(),
            none(),
            none(),
            text(&p.to_string()),
            dim(*measures),
            dim(*was),
            none(),
            none(),
            none(),
            none(),
            none(),
            none(),
            none(),
            none(),
            none(),
        ),
        // `message` here, `message` at the parse arm and `detail` at
        // the unreadable one are one concept — the reporter's own
        // words — and cross on one attribute.
        E::Serialize { message } => (
            none(),
            none(),
            none(),
            none(),
            none(),
            none(),
            text(message),
            none(),
            none(),
            none(),
            none(),
            none(),
            none(),
            none(),
            none(),
        ),
        E::HeaderId { found: what } => (
            none(),
            none(),
            none(),
            none(),
            none(),
            none(),
            none(),
            text(what),
            none(),
            none(),
            none(),
            none(),
            none(),
            none(),
            none(),
        ),
        E::IdMismatch {
            header: from_header,
            snapshot: from_snapshot,
        } => (
            none(),
            none(),
            none(),
            none(),
            none(),
            none(),
            none(),
            none(),
            text(&from_header.hex()),
            text(&from_snapshot.hex()),
            none(),
            none(),
            none(),
            none(),
            none(),
        ),
        E::Parse {
            line: l,
            column: c,
            message,
        } => (
            none(),
            none(),
            none(),
            none(),
            none(),
            none(),
            text(message),
            none(),
            none(),
            none(),
            int(*l),
            int(*c),
            none(),
            none(),
            none(),
        ),
        E::Unreadable {
            line: l,
            column: c,
            detail: what,
        } => (
            none(),
            none(),
            none(),
            none(),
            none(),
            none(),
            text(what),
            none(),
            none(),
            none(),
            int(*l),
            int(*c),
            none(),
            none(),
            none(),
        ),
        // The refusal crosses as a WORD plus the reader's position, not
        // as prose: `inner_variant` is the dimension check that failed,
        // from the same map the expression text door's `kind` uses. No
        // `detail` — there is nothing here that needs a sentence to be
        // branchable, which is the whole point of the arm.
        E::Dimension {
            line: l,
            column: c,
            error,
        } => (
            word(crate::tags::expr_dimension_error_tag(error)),
            none(),
            none(),
            none(),
            none(),
            none(),
            none(),
            none(),
            none(),
            none(),
            int(*l),
            int(*c),
            none(),
            none(),
            none(),
        ),
        E::Snapshot(inner) => (
            word(crate::tags::snapshot_error_tag(inner)),
            none(),
            none(),
            none(),
            none(),
            none(),
            none(),
            none(),
            none(),
            none(),
            none(),
            none(),
            none(),
            none(),
            none(),
        ),
        E::EditReplay { index: at, error } => (
            word(edit_error_tag(error)),
            none(),
            none(),
            none(),
            none(),
            none(),
            none(),
            none(),
            none(),
            none(),
            none(),
            none(),
            int(*at),
            none(),
            none(),
        ),
        E::ToleranceConflict {
            process: committed,
            document: recorded,
        } => (
            none(),
            none(),
            none(),
            none(),
            none(),
            none(),
            none(),
            none(),
            none(),
            none(),
            none(),
            none(),
            none(),
            real(*committed),
            real(*recorded),
        ),
        // `value` IS the document's recorded ε — the same concept the
        // conflict arm reports as `document` — so it crosses there.
        E::ToleranceInvalid { value } => (
            none(),
            none(),
            none(),
            none(),
            none(),
            none(),
            none(),
            none(),
            none(),
            none(),
            none(),
            none(),
            none(),
            none(),
            real(*value),
        ),
    };
    typed_err(
        py,
        ErrorClass::Persist,
        // `PersistError` implements `Display`, so the human message is
        // real prose; the machine payload is the tag and the fields.
        err.to_string(),
        &[
            ("variant", text(persist_error_tag(err))),
            ("inner_variant", inner),
            ("site", site),
            ("node", which),
            ("name", name),
            ("unit", unit),
            ("declared", declared),
            ("detail", detail),
            ("found", found),
            ("header", header),
            ("snapshot", snapshot),
            ("line", line),
            ("column", column),
            ("index", index),
            ("process", process),
            ("document", document),
        ],
    )
}

/// The expression a node SLOT takes, refused at the door when its
/// dimension is not the slot's.
///
/// The dimension a slot requires is [`d::SlotId::expr_dimension`] — the
/// kernel's own table, read rather than restated — and the refusal is
/// the kernel's own `EditError`, the one `apply` raises for the same
/// expression in the same slot. So an angle handed to a length slot
/// says the same words whether the door caught it or the insert did,
/// and the boundary carries no second vocabulary for one fault.
pub(crate) fn slot_expr(
    py: Python<'_>,
    slot: d::SlotId,
    expr: &super::expr::SlotArg,
) -> PyResult<d::Formula> {
    let expected = slot.expr_dimension();
    let formula = expr.formula(py, expected)?;
    let found = formula.dim();
    if found == expected {
        return Ok(formula);
    }
    // A lone `0` is the constant zero of the slot's dimension (VR6).
    if formula.is_zero() {
        return Ok(d::Formula::zero(expected));
    }
    Err(edit_err(
        py,
        &d::EditError::SlotDimensionMismatch {
            slot,
            expected: slot.kind(),
            found,
        },
    ))
}

/// A stable name as Python holds it: the name's OWN serde text.
///
/// # The text is OPAQUE BY CONTRACT
///
/// It is a stable IDENTIFIER — carried from a materializer to a
/// selection unread, exactly as the Rust surface carries the value —
/// and its internal structure is **not API**. Parsing it is
/// representation-dependence: the encoding may change without notice,
/// and code that reads inside a name is code this crate will break.
/// The supported operations are equality, ordering, storage, and
/// handing it back to [`Node::fillet`]. Narrowing a set of names is a
/// SELECTOR's job — `Evaluation.select` / `select_where`,
/// which answer in this same alphabet; the binding is the one
/// licensed reader of the text.
///
/// # Why this encoding
///
/// `StableName` has exactly one serialization and the binding reuses
/// it rather than minting a second spelling, so a name is one
/// vocabulary across Rust, Python and the file format. The relation
/// to a saved document is VALUE equality, not byte equality: `save`
/// pretty-prints and this writes compact, so the two texts differ in
/// whitespace and parse to the same JSON value — and a name taken
/// from either round-trips through the other.
pub(crate) fn name_text(py: Python<'_>, name: &pncad::prelude::StableName) -> PyResult<String> {
    name.to_json().map_err(|err| {
        // Not a kernel arm: nothing in the document layer refuses a
        // name for failing to serialize — `StableName` has one
        // serialization and it does not fail — so there is no enum
        // whose word this could be, and the tag is minted here. That
        // is the exception `boundary_edit_err` names, and it is the
        // reason the surrounding rule is "from the arm's enum where
        // one exists" rather than "always through `crate::tags`".
        boundary_edit_err(
            py,
            BoundaryEdit::NameSerialize,
            format!("a stable name failed to serialize: {err}"),
        )
    })
}

/// The profile program a node holds, or a boundary `ValueError`.
fn profile_of<'d>(doc: &'d d::ProfileDoc, node: &NodeId) -> PyResult<&'d d::ProfileProgram> {
    match doc.node(node.0) {
        Some(d::Node::Profile(program)) => Ok(program),
        _ => Err(pyo3::exceptions::PyValueError::new_err(format!(
            "{} is not a profile",
            doc.spoken(node.0)
        ))),
    }
}

/// **A profile piece as opaque text** — the one serialization its
/// locator has, the same alphabet a name's text is written in.
pub(crate) fn piece_text(piece: &pncad::select::ProfileEdgeRef) -> PyResult<String> {
    serde_json::to_string(piece).map_err(|err| {
        pyo3::exceptions::PyValueError::new_err(format!(
            "a profile piece failed to serialize: {err}"
        ))
    })
}

/// A text as a label, or the label rule's refusal at the call that
/// offered it.
fn label_from_text(py: Python<'_>, text: &str) -> PyResult<d::Label> {
    d::Label::new(text)
        .map_err(|fault| boundary_edit_err(py, BoundaryEdit::Label(&fault), fault.to_string()))
}

/// Read a stable name back from [`name_text`]'s output.
///
/// Text that is not a name at all is a boundary `ValueError` — the
/// same class of refusal as a string where a `Frame` belongs,
/// with no kernel refusal to forward. A WELL-FORMED name that denotes
/// nothing in this document refuses at the kernel's own door
/// (`select_resolve`), which is where that belongs.
pub(crate) fn name_from_text(text: &str) -> PyResult<pncad::prelude::StableName> {
    pncad::prelude::StableName::from_json(text).map_err(|err| {
        pyo3::exceptions::PyValueError::new_err(format!(
            "not a stable name: {text:?} ({err}) — names come from \
             `Evaluation.all_edges` and its siblings"
        ))
    })
}

/// Read a named slot back from the word `EditError.slot` answers in.
///
/// A slot is a NAME, never an index (spec D5), and the name is the
/// same word in both directions: the word a refusal publishes is the
/// word a door takes, so a caller retries at the address it was
/// refused at without translating anything.
///
/// A word outside the alphabet is a boundary `ValueError` — the same
/// class of refusal as text that is not a stable name, with no kernel
/// refusal to forward. A well-formed word the TARGET NODE does not
/// carry is a different question and belongs to the kernel, which
/// answers it as `unknown_slot` naming the slot the node lacks.
///
/// `program`, a loft station's `station_slide` and `station_spin`, and
/// a constraint's `value_slide` and `value_spin` are words of the
/// alphabet with no slot to read back:
/// the rest of each address holds
/// an integer the word does not carry, so each refuses in its own
/// sentence rather than as a misspelling.
fn slot_from_text(word: &str) -> PyResult<d::SlotId> {
    if let Some(slot) = crate::slot_word::slot_from_word(word) {
        return Ok(slot);
    }
    Err(pyo3::exceptions::PyValueError::new_err(
        if word == "program" {
            "`program` addresses one expression inside a profile program, and the rest of \
         that address — a loop index, a step index and which argument — is not carried \
         by the word: a profile's numbers are re-authored, not edited at a slot"
                .to_owned()
        } else if word == "station_slide" || word == "station_spin" {
            format!(
                "`{word}` addresses one move of a loft section's station, and the rest of that \
             address — the section and the move's place in its station — is not carried by \
             the word: a parameter the number reads moves it, or the loft is re-authored"
            )
        } else if word == "section" || word == "member" {
            format!(
                "`{word}` addresses one entry of a list a node reads, and its position is not \
             carried by the word: `DocEdit.set_members` writes the list whole"
            )
        } else if word == "index" {
            "`index` addresses one index of a read of a family's member, and the rest of that \
         address — the seat the read sits at and which index — is not carried by the word: \
         the member read is re-authored"
                .to_owned()
        } else if matches!(word, "value_slide" | "value_spin") {
            format!(
                "`{word}` addresses one number of a placement's constraint, and the rest of \
             that address — the constraint — is not carried by the word, so no slot word \
             here writes at it: a parameter the number reads moves it, or the constraint is \
             re-authored"
            )
        } else {
            format!(
                "not a slot: {word:?} — a slot is named by its own word (`distance`, \
             `radius`, `count`, `origin_x`), the word `EditError.slot` answers in"
            )
        },
    ))
}

/// A recipe node's identity within a document.
/// The seam the document's doors resolve parts through: the same one
/// `evaluate(doc, resolver=)` crosses.
pub(crate) fn seam(
    resolver: Option<&super::store::Workspace>,
) -> Option<std::sync::Arc<dyn d::PartResolver>> {
    resolver.map(super::store::Workspace::resolver)
}

#[pyclass(frozen, module = "pncad", from_py_object)]
#[derive(Clone, Copy)]
pub(crate) struct NodeId(pub(crate) d::RecipeNodeId);

/// **What `DocEdit.set_param` writes**: a node, read through its
/// output, or anything a slot argument is — a variable, a formula, a
/// value.
#[derive(FromPyObject)]
pub(crate) enum SlotValueArg {
    /// A node, read through its one output.
    Node(NodeId),
    /// A variable, a formula or a value.
    Slot(super::expr::SlotArg),
}

/// **An operand as Python writes it**: a node, read at its one output
/// (a node with several, a revolve or a split, refuses
/// `ambiguous_output`: its port is read by `Doc.output`), a variable
/// — an output by `Doc.output`, or a name's — or a pose defined at the
/// seat (`PoseDef`).
#[derive(FromPyObject, Clone)]
pub(crate) enum OperandArg {
    /// A node, read at its one output.
    Node(NodeId),
    /// A variable, read as itself.
    Var(Var),
    /// A pose defined at the seat.
    Pose(super::pose::PoseDef),
    /// One piece of a copy, picked at the seat.
    Pick(PickRead),
}

impl OperandArg {
    /// The authored read.
    pub(crate) fn read(self) -> d::Operand {
        match self {
            Self::Node(node) => d::Operand::Node(node.0),
            Self::Var(var) => d::Operand::Var(var.0),
            Self::Pose(pose) => pose.inner,
            Self::Pick(pick) => pick.read(),
        }
    }
}

/// **A pick as Python writes it** (D10 `[placement.placement]`): one
/// piece of the copy `placement` reads, by the path to the piece's
/// variable in the source — each step an operand of the `⊔` the walk
/// reaches through the placements before it (`Doc.pick`). Read at a
/// seat, the door mints one pick variable there, as it mints a
/// selection, so a pick written at two seats is two variables.
#[pyclass(module = "pncad", frozen, from_py_object)]
#[derive(Clone)]
pub(crate) struct PickRead {
    placement: Box<OperandArg>,
    of: Vec<d::VarId>,
}

impl PickRead {
    /// The authored read.
    pub(crate) fn read(self) -> d::Operand {
        d::Operand::pick(self.placement.read(), self.of)
    }
}

#[pymethods]
impl PickRead {
    fn __repr__(&self) -> String {
        format!("PickRead(of={} steps)", self.of.len())
    }
}

/// **A placement's target space as Python writes it**: a node, read at
/// its space — the world (`Doc.world`), a `Node.space()`, or a
/// construction, whose body is born in a space of its own — or a
/// `Space` variable (`Doc.space`, or a construction's `space` output by
/// `Doc.output`).
#[derive(FromPyObject, Clone, Copy)]
pub(crate) enum SpaceArg {
    /// A node, read at its space.
    Node(NodeId),
    /// A `Space` variable.
    Var(Var),
}

impl SpaceArg {
    /// The authored read.
    pub(crate) fn read(self) -> d::Operand {
        match self {
            Self::Node(node) => d::Operand::Node(node.0),
            Self::Var(var) => d::Operand::Var(var.0),
        }
    }
}

/// **Where a name door reads a table, as Python writes it** (NAMES N1,
/// the scope): a node named alone at its one output, or a variable — a
/// split's half by its port's. A pose is an operand and holds no table,
/// so a name door does not take one.
#[derive(FromPyObject, Clone, Copy)]
pub(crate) enum AtArg {
    /// A node, read at its one output.
    Node(NodeId),
    /// A variable, read as itself.
    Var(Var),
}

impl AtArg {
    /// The read the door looks in.
    pub(crate) fn at(self) -> pncad::select::At {
        match self {
            Self::Node(node) => pncad::select::At::Node(node.0),
            Self::Var(var) => pncad::select::At::Read(var.0),
        }
    }
}

/// **One member of a family, read by index** (REFERENCES DM3, D10
/// `[repetition]`): `xs[i]`, or `xs[i, j]` for a family of two indices,
/// written on the family's operand — a node or a variable — with one
/// count per index. It is a read of the family, not a variable: an
/// operation reading it keys the member's names by the family's read.
/// Every body seat takes one; an index out of the family's range
/// refuses `member_out_of_range` at `evaluate`.
#[pyclass(frozen, module = "pncad", from_py_object)]
#[derive(Clone)]
pub(crate) struct IndexedRead {
    read: OperandArg,
    at: Vec<d::Formula>,
}

/// **The indices `[]` is handed**: one (`xs[i]`), or a tuple, one per
/// index of the family, the outermost first (`xs[i, j]`).
#[derive(FromPyObject)]
pub(crate) enum IndexArg {
    /// One index.
    One(super::expr::SlotArg),
    /// One per index of the family.
    Many(Vec<super::expr::SlotArg>),
}

impl IndexedRead {
    /// `read` at `index`, each read as a count.
    fn of(py: Python<'_>, read: OperandArg, index: &IndexArg) -> PyResult<Self> {
        let at = match index {
            IndexArg::One(one) => vec![one.formula(py, d::Dimension::Count)?],
            IndexArg::Many(many) => many
                .iter()
                .map(|i| i.formula(py, d::Dimension::Count))
                .collect::<PyResult<Vec<_>>>()?,
        };
        Ok(Self { read, at })
    }
}

#[pymethods]
impl IndexedRead {
    fn __repr__(&self) -> String {
        let read = match &self.read {
            OperandArg::Node(node) => format!("NodeId({})", node.0.full()),
            OperandArg::Var(var) => format!("Var({})", var.0.full()),
            OperandArg::Pose(_) | OperandArg::Pick(_) => {
                unreachable!(
                    "an indexed read is `[]` on a node or a variable, never a pose or a pick"
                )
            }
        };
        let at: Vec<String> = self.at.iter().map(ToString::to_string).collect();
        format!("{read}[{}]", at.join(", "))
    }
}

/// **A read at a body seat as Python writes it**: an operand read
/// plainly, or one member of a family (`op[i]`, [`IndexedRead`]).
#[derive(FromPyObject, Clone)]
pub(crate) enum BodyArg {
    /// One member of a family.
    Indexed(IndexedRead),
    /// An operand, read plainly.
    Read(OperandArg),
}

impl BodyArg {
    /// The authored read.
    pub(crate) fn read(self) -> d::BodyRead<d::Formula> {
        match self {
            Self::Indexed(IndexedRead { read, at }) => d::BodyRead::indexed(read.read(), at),
            Self::Read(read) => d::BodyRead::plain(read.read()),
        }
    }
}

/// **A union's or an intersect's members as Python writes them**: ONE
/// operand, a family read whole (a pattern's output), or a sequence of
/// reads, each on its own (an operand, or a member `op[i]`) — the
/// kernel's `Bodies`. Whether a read in a sequence is itself a family
/// is the kernel's question at the door (`slot_var_kind`), not this
/// one's.
#[derive(FromPyObject)]
pub(crate) enum MembersArg {
    /// One read: the family it reads, whole.
    Family(BodyArg),
    /// A sequence of reads, in the order written.
    Spelled(Vec<BodyArg>),
}

impl MembersArg {
    /// The authored argument.
    pub(crate) fn bodies(self) -> d::Bodies<d::BodyRead<d::Formula>> {
        match self {
            Self::Family(read) => d::Bodies::Family(read.read()),
            Self::Spelled(reads) => {
                d::Bodies::Spelled(reads.into_iter().map(BodyArg::read).collect())
            }
        }
    }
}

#[pymethods]
impl NodeId {
    fn __repr__(&self) -> String {
        format!("NodeId({})", self.0.full())
    }

    /// One member of the family this node's output is, at `index`: one
    /// count per index of the family ([`IndexedRead`]).
    fn __getitem__(&self, py: Python<'_>, index: IndexArg) -> PyResult<IndexedRead> {
        IndexedRead::of(py, OperandArg::Node(*self), &index)
    }

    fn __eq__(&self, other: &Self) -> bool {
        self.0 == other.0
    }

    fn __hash__(&self) -> u64 {
        let _tol = Tol::witness();
        self.0.0.digest()
    }
}

/// A parametric document: the recipe, not the geometry.
///
/// Rust's `Doc` is immutable — `apply` returns a NEW document. Python
/// object semantics make an in-place edit far more natural, so this
/// wrapper owns the current document and swaps it on a successful
/// apply. The Rust value semantics are still available: a refused edit
/// leaves the document untouched, exactly as the immutable API
/// guarantees, because the swap only happens on `Ok`.
#[pyclass(module = "pncad")]
pub(crate) struct Doc {
    pub(crate) inner: d::ProfileDoc,
    /// What the LAST accepted edit reported: the names it stranded.
    ///
    /// The Rust `apply` returns this beside the new document; the
    /// Python wrapper owns the document and swaps it, so the record
    /// is held here for the same span the document it describes is —
    /// an invariant [`Doc::take_up`] holds by being the only place
    /// either of the two is written.
    pub(crate) maintenance: Vec<d::Maintenance>,
}

/// The wrapper's own plumbing: the ONE place an accepted document is
/// taken up ([`Doc::take_up`]), the single-edit door onto it, and the
/// shared door bodies that land there. None of it is a
/// Python method.
impl Doc {
    /// **An authored formula, read against this document**: its
    /// names lowered to the variables they name ([`d::Doc::lowered`]).
    /// A name the document holds at another kind than the formula
    /// reads it at refuses `var_kind_mismatch`, naming both kinds; a
    /// name it does not hold refuses `unlowered_name`.
    /// What `eval` and `eval_count` evaluate: the formula, or the lone
    /// reader of the variable at the dimension it holds. A variable
    /// this document does not hold refuses `unresolved_var`.
    fn evaluand(&self, py: Python<'_>, evaluand: Evaluand) -> PyResult<d::Formula> {
        match evaluand {
            Evaluand::Formula(formula) => Ok(formula.0),
            Evaluand::Var(Var(var, _)) => {
                // An operation's output of a reference kind is no value an
                // expression reads; a scalar one has no binding outside an
                // evaluation, so its reader refuses `unresolved_var`.
                if let Some(held) = self.inner.var(var)
                    && held.kind().dimension().is_none()
                {
                    return Err(pyo3::exceptions::PyTypeError::new_err(format!(
                        "{} is an operation's output of kind {}, which no expression reads",
                        self.inner.spoken_var(var),
                        held.kind()
                    )));
                }
                let dim = self.inner.var(var).and_then(|held| match held.def() {
                    d::VarDef::Free(free) => Some(free.dim()),
                    d::VarDef::Defined(expr) => Some(expr.dim()),
                    d::VarDef::Index(_) => Some(d::Dimension::Count),
                    d::VarDef::Output { .. }
                    | d::VarDef::Select(_)
                    | d::VarDef::Pose(_)
                    | d::VarDef::Pick(_) => held.kind().dimension(),
                });
                let Some(dim) = dim else {
                    let unheld = d::EvalError::UnresolvedVar { var };
                    return Err(super::expr::eval_err(py, &unheld, Some(&self.inner)));
                };
                Ok(d::Formula::var(var, dim))
            }
        }
    }

    fn authored(&self, py: Python<'_>, formula: &d::Formula) -> PyResult<d::Formula> {
        self.inner
            .resolve(formula)
            .map_err(|fault| super::expr::lower_fault_err(py, &fault))
    }

    /// A single edit's door onto **the swap point**, [`Doc::take_up`],
    /// which replaces the held document and the maintenance record TOGETHER
    /// — which is what makes `last_maintenance` a fact about the
    /// document now held rather than about some earlier one. Returns
    /// the edit's record so each door can read the id it minted.
    ///
    /// A door that swapped the document by hand would leave the
    /// maintenance describing an edit two edits ago; that is the
    /// staleness this function exists to make unspellable.
    ///
    /// **The funnel is held by this doc and by
    /// `test_assembly_author.py`'s
    /// `test_last_maintenance_describes_the_last_accepted_edit_at_every_door`,
    /// not by the type system**: `inner` and `maintenance` are
    /// `pub(crate)` because the rest of the crate reads the document,
    /// so nothing stops a new door assigning either field directly.
    /// [`Doc::insert_node`] closes that hole for `insert` by taking its
    /// action up internally; a door reaching `d::apply` for any OTHER
    /// edit lands here, and a kernel door answering a whole action's paired document and
    /// maintenance lands in [`Doc::take_up`] directly; anything else is a
    /// bug the test names. The refactoring wrappers are the
    /// one family that does not pass through: they never `apply` a
    /// single edit, they project a kernel outcome whose document and
    /// maintenance were already paired below this wrapper, and they
    /// carry that pairing across whole.
    fn accept(&mut self, applied: d::Applied<d::ProfileProgram>) -> d::EditRecord {
        self.take_up(applied.doc, applied.maintenance);
        applied.record
    }

    /// [`Doc::accept`]'s swap, for a kernel door that applied a whole
    /// action and answers its document and maintenance already paired
    /// (a [`d::Recorded`]) rather than one edit's [`d::Applied`].
    fn take_up(&mut self, doc: d::ProfileDoc, maintenance: Vec<d::Maintenance>) {
        self.inner = doc;
        self.maintenance = maintenance;
    }

    /// Insert a node, label it when `label` is given, and take the
    /// action up: `insert`'s body, which takes it up internally rather
    /// than handing an un-accepted result back for a caller to
    /// remember to swap.
    ///
    /// A labelled insert is the kernel's two edits — the insert, which
    /// carries no label, then `SetLabel` on the id it minted — recorded
    /// as one action ([`d::Recording`]): both land or neither does, and
    /// the maintenance is the action's.
    fn insert_node(
        &mut self,
        node: d::AuthoredNode,
        label: Option<d::Label>,
    ) -> Result<NodeId, d::EditError> {
        self.minting(label, |action| action.insert(node))
    }

    /// `insert_node` for an insert spelled as its [`d::DocEdit`] — the
    /// kernel's own sugar for one, `DocEdit::place`'s among them.
    fn insert_edit(
        &mut self,
        edit: d::DocEdit<d::ProfileProgram>,
        label: Option<d::Label>,
    ) -> Result<NodeId, d::EditError> {
        self.minting(label, |action| {
            let minted = action.apply(edit)?;
            Ok(minted.unwrap_or_else(|| unreachable!("an accepted insert mints a node")))
        })
    }

    /// The shared tail: `insert` on a fresh action, then the label.
    fn minting(
        &mut self,
        label: Option<d::Label>,
        insert: impl FnOnce(
            &mut d::Recording<'_, d::ProfileProgram>,
        ) -> Result<d::RecipeNodeId, d::EditError>,
    ) -> Result<NodeId, d::EditError> {
        let tol = Tol::witness();
        let mut action = d::Recording::start(&self.inner, tol);
        let id = insert(&mut action)?;
        if let Some(label) = label {
            action.apply(d::DocEdit::SetLabel {
                node: id,
                label: Some(label),
            })?;
        }
        let done = action.finish()?;
        self.take_up(done.doc, done.maintenance);
        Ok(NodeId(id))
    }
}

#[pymethods]
impl Doc {
    /// An empty document.
    ///
    /// A document's id answers WHICH PART, and a workspace refuses to
    /// hold two files claiming one — so `Doc()` mints a FRESH random
    /// identity and two documents authored here are two parts.
    /// `Doc(seed)` derives the id from the seed text instead: same
    /// seed, same id, on every platform, which is the spelling a
    /// caller whose saves must reproduce byte for byte wants — and
    /// which therefore makes two same-seed documents the SAME part,
    /// deliberately. (The seed is not a label: a node's label is
    /// `Doc.label`.)
    ///
    /// Raises `IdentityError` if the OS entropy source refuses.
    #[new]
    #[pyo3(signature = (seed = None))]
    fn new(py: Python<'_>, seed: Option<&str>) -> PyResult<Self> {
        let tol = Tol::witness();
        let inner = match seed {
            Some(seed) => crate::identity::derived(seed, tol),
            // The tag is the store's own, through `crate::tags`, not a
            // literal chosen here: `interactive` refuses with the whole
            // `WorkspaceError` vocabulary, and which of its arms a
            // caller sees is that enum's fact to state, not this raise
            // site's to assume. Only `randomness_unavailable` is
            // reachable through this door today — `random_document_id`
            // has one failure arm — but that is a fact about ANOTHER
            // crate, and a literal here would go on being written
            // confidently over a second arm the day one appears.
            None => crate::identity::interactive(tol).map_err(|err| {
                typed_err(
                    py,
                    ErrorClass::Identity,
                    err.to_string(),
                    &[(
                        "variant",
                        PyString::new(py, workspace_error_tag(&err))
                            .unbind()
                            .into_any(),
                    )],
                )
            })?,
        };
        Ok(Self {
            inner,
            maintenance: Vec::new(),
        })
    }

    /// This document's identity as 32 lowercase hex digits — the
    /// canonical text form, the same one the save file's `id:` header
    /// carries and the workspace store keys on.
    ///
    /// Identity survives every edit; it is not a content hash.
    #[getter]
    fn id(&self) -> String {
        self.inner.id().hex()
    }

    /// Apply an edit, returning the id it minted (if any).
    ///
    /// On refusal the document is unchanged and a typed `EditError` is
    /// raised.
    ///
    /// An accepted edit may also have performed **maintenance** — the
    /// payload names a delete stranded. That rides the edit rather than
    /// being a second edit, so it is read off `last_maintenance`
    /// instead of returned here: the common case is an empty list, and
    /// widening every caller's return type for it would be paying for
    /// strands in documents that have none.
    fn apply(&mut self, py: Python<'_>, edit: &DocEdit) -> PyResult<Option<NodeId>> {
        let tol = Tol::witness();
        let applied = d::apply(&self.inner, &edit.inner, tol).map_err(|err| edit_err(py, &err))?;
        Ok(self.accept(applied).minted.map(NodeId))
    }

    /// The maintenance the LAST accepted edit performed, in the order
    /// it was performed: the payload names a delete or a reshaping
    /// stranded — so a caller reads an entry's `variant`, never its
    /// position.
    ///
    /// Empty after any edit that stranded no name, and empty on a fresh document — a document that has never applied an edit has
    /// no last edit to report about. A REFUSED edit leaves this
    /// untouched, exactly as it leaves the document untouched.
    ///
    /// **A document a refactoring minted reads that refactoring's own
    /// record.** `SplitOutcome.remainder`, `SplitOutcome.part` and
    /// `InlineOutcome.doc` are values produced by applying a whole
    /// edit LIST, so each reports what ITS list did, net of what a
    /// later edit in the same list took back: the names a departing
    /// node stranded and the remainder still carries. The document and
    /// that record cross together, so a caller reading here after either door reads the
    /// record the kernel has rather than an empty list.
    ///
    /// **The reading begins at the load boundary.** A `Doc` handed out
    /// by `Loaded.doc`, `Loaded.snapshot` or `Workspace.resolve` starts
    /// empty even though the replayed history's last edit may well have
    /// performed maintenance: the replay happens below this wrapper, so
    /// "the last accepted edit" means the last one accepted THROUGH one
    /// of these doors.
    ///
    /// Undo is keeping the prior document value, which restores every
    /// one of these exactly; what the record adds is VISIBILITY — a
    /// stranded name is reported at the edit that stranded it.
    #[getter]
    fn last_maintenance(&self) -> Vec<super::mate::Maintenance> {
        self.maintenance
            .iter()
            .cloned()
            .map(super::mate::Maintenance)
            .collect()
    }

    /// **The minted id of every step of the profile at `profile`**, one
    /// list per loop in program order.
    ///
    /// The positional reading, for a caller that holds no handle;
    /// `Doc.step` reaches one step's id from the handle its authoring
    /// call returned.
    ///
    /// Raises `ValueError` for a node that is not a profile.
    fn step_ids(&self, profile: &NodeId) -> PyResult<Vec<Vec<super::step::StepId>>> {
        Ok(profile_of(&self.inner, profile)?
            .ids
            .iter()
            .map(|loop_| loop_.iter().copied().map(super::step::StepId).collect())
            .collect())
    }

    /// **The id the profile at `profile` minted for the step `h`
    /// addresses** in its loop `loop`, which the author states: `h` is
    /// the `.step` of the chain state (or closed loop) the step's verb
    /// returned.
    ///
    /// Raises `StepHandleError` `handle_off_program` where the loop's
    /// program has no step at that index with that shape up to it — a
    /// handle is valid for the program it was authored for, and a
    /// value edit keeps it valid — and `ValueError` for a node that is
    /// not a profile.
    #[pyo3(signature = (profile, r#loop, h))]
    fn step(
        &self,
        py: Python<'_>,
        profile: &NodeId,
        r#loop: u32,
        h: &super::step::AuthoredStep,
    ) -> PyResult<super::step::StepId> {
        profile_of(&self.inner, profile)?
            .step(r#loop, &h.0)
            .map(super::step::StepId)
            .map_err(|refusal| super::step::handle_err(py, &refusal))
    }

    /// **The piece `role` of an authored step** of the profile at
    /// `profile`, in its loop `loop`: `role` is a handle's role
    /// accessor (`h.leg`, `h.run_out`, `h.piece(k)`).
    ///
    /// A role the step's verb draws is a piece whether or not the
    /// current values draw it; a name on one they do not resolves
    /// `Vanished` until they do.
    ///
    /// Raises what `Doc.step` raises.
    #[pyo3(signature = (profile, r#loop, role))]
    fn piece(
        &self,
        py: Python<'_>,
        profile: &NodeId,
        r#loop: u32,
        role: &super::step::StepRole,
    ) -> PyResult<super::step::Piece> {
        let edge = profile_of(&self.inner, profile)?
            .piece(r#loop, &role.step, role.role)
            .map_err(|refusal| super::step::handle_err(py, &refusal))?;
        super::step::Piece::of_edge(&edge)
    }

    /// **The piece every canonical segment of the profile at `profile`
    /// is**, one list per canonical loop (0 the outer loop, then the
    /// holes in description order), one piece per canonical segment in
    /// the loop's canonical traversal from its authored start — under
    /// the document's current parameter values.
    ///
    /// The positional reading, for a caller that holds no handle;
    /// `Doc.piece` spells a piece from the handle its authoring call
    /// returned. A piece stays the name of that piece whatever later
    /// moves the segment — a value edit, a hole becoming the outer
    /// loop, a `set_program` that keeps the step.
    ///
    /// Raises `ValueError` for a node that is not a profile, or whose
    /// program does not replay and validate under the current values.
    fn pieces(&self, profile: &NodeId) -> PyResult<Vec<Vec<super::step::Piece>>> {
        let program = profile_of(&self.inner, profile)?;
        let pieces = program
            .pieces(&self.inner.var_env::<f64>(), Tol::witness())
            .map_err(|refusal| {
                pyo3::exceptions::PyValueError::new_err(format!(
                    "{} has no pieces under the current values: {refusal}",
                    self.inner.spoken(profile.0)
                ))
            })?;
        pieces
            .edges
            .iter()
            .map(|loop_| loop_.iter().map(super::step::Piece::of_edge).collect())
            .collect()
    }

    /// **The placements**, in document order: every `Place` and
    /// `InstantiatePart` node — the nodes whose copies a bundle of
    /// constraints places, into the world or another space
    /// (`Doc.space`). Which body is the product is the product slot's
    /// (`Doc.set_product`), never derived from these.
    ///
    /// Written through `place` and `Node.instantiate_part`, and through
    /// nothing else: no edit places as a side effect.
    fn placements(&self) -> Vec<NodeId> {
        self.inner.placements().into_iter().map(NodeId).collect()
    }

    /// **The world**: the node every document holds from its creation,
    /// which no edit deletes or writes. A placement whose `target` is
    /// it places its copy into the world's space; it is not counted
    /// among the document's nodes (`order`).
    #[getter]
    fn world(&self) -> NodeId {
        NodeId(d::RecipeNodeId::WORLD)
    }

    /// **A placement's bundle**: each constraint by its address, with
    /// its mate or its value as written, in the bundle's order.
    ///
    /// Raises `EditError` (`constraint_on_non_placement`) for a node
    /// that holds no bundle, and (`unknown_node`) for one this
    /// document does not hold.
    fn bundle(
        &self,
        py: Python<'_>,
        placement: &NodeId,
    ) -> PyResult<Vec<(super::mate::MateRef, Py<PyAny>)>> {
        let Some(node) = self.inner.node(placement.0) else {
            return Err(edit_err(
                py,
                &d::EditError::UnknownNode {
                    id: d::SpokenNode::absent(placement.0),
                },
            ));
        };
        match node.authored(&self.inner) {
            d::Node::Place { constraints, .. } | d::Node::InstantiatePart { constraints, .. } => {
                constraints
                    .into_iter()
                    .map(|c| {
                        let at = super::mate::MateRef(d::MateRef::new(placement.0, c.id));
                        let held = match c.kind {
                            d::ConstraintKind::Mate(mate) => {
                                Py::new(py, super::mate::Mate(mate))?.into_any()
                            }
                            d::ConstraintKind::Value(value) => {
                                Py::new(py, super::mate::MateValue(value))?.into_any()
                            }
                        };
                        Ok((at, held))
                    })
                    .collect()
            }
            _ => Err(edit_err(
                py,
                &d::EditError::ConstraintOnNonPlacement {
                    node: self.inner.spoken(placement.0),
                },
            )),
        }
    }

    /// **A new space**, reading nothing, and return its node: the
    /// insert of `Node.space()`. A placement whose `target` is it
    /// places copies into it, beside the world's; a placement of a
    /// body of it into the world puts what it holds in the product.
    #[pyo3(signature = (*, label=None))]
    fn new_space(&mut self, py: Python<'_>, label: Option<&str>) -> PyResult<NodeId> {
        let label = label.map(|text| label_from_text(py, text)).transpose()?;
        self.insert_node(d::Node::Space, label)
            .map_err(|err| edit_err(py, &err))
    }

    /// **The space `node`'s body is in**, as its `Space` variable,
    /// derived from the recipe with nothing evaluated: a
    /// construction's body is in its own space (its `space` output), a
    /// placed copy in its placement's `target`, an operator's result in
    /// the space its operands share. The world's is `Doc.world`'s
    /// space. `None` for a node that builds no body — a profile, a
    /// measure, a space itself — or one this document does not hold.
    fn space(&self, node: &NodeId) -> Option<Var> {
        self.inner
            .node_space(node.0)
            .map(|space| Var(space, Some(d::VarKind::Space)))
    }

    /// **Write `constraints` as `placement`'s bundle**, in place of the
    /// one it holds, and return their addresses in order: the edit
    /// `DocEdit.set_constraints`. The bundle is written whole, as
    /// `Doc.place`'s is: a `Placed`, or `Mate`s and `MateValue`s, each
    /// id minted by the edit.
    ///
    /// Refuses typed on `EditError`: `constraint_on_non_placement`,
    /// `mate_refused` (`mate_unpinned` for a bundle that leaves a
    /// freedom unwritten, `[]` among them; `mate_overconstrained`),
    /// `space_mismatch` for a mate whose `to` is outside the
    /// placement's target, and the slot doors' refusals.
    fn set_constraints(
        &mut self,
        py: Python<'_>,
        placement: &NodeId,
        constraints: super::mate::BundleArg,
    ) -> PyResult<Vec<super::mate::MateRef>> {
        let edit = d::DocEdit::SetConstraints {
            placement: placement.0,
            constraints: super::mate::bundle(constraints),
            fresh: Vec::new(),
        };
        let applied =
            d::apply(&self.inner, &edit, Tol::witness()).map_err(|err| edit_err(py, &err))?;
        let record = self.accept(applied);
        Ok(record
            .minted_constraints
            .iter()
            .map(|&id| super::mate::MateRef(d::MateRef::new(placement.0, id)))
            .collect())
    }

    /// **Add `mate` to `placement`'s bundle**, and return the
    /// constraint's address: the gesture over `DocEdit.add_constraint`,
    /// which writes the bundle whole again with the mate: each value the
    /// mates then still leave is kept, and each freedom they leave that
    /// no value sets is written the constant `0`, so the copy stays
    /// pinned. The gesture reads the copy's geometry for which slides
    /// those are (the two a plane leaves, the one an axis leaves); the
    /// raw edit, which reads none, writes them in axis order. The
    /// constraint's id is minted by the edit.
    ///
    /// Refuses typed on `EditError`: `constraint_on_non_placement`,
    /// `mate_refused` (the solve's own admission of the datum, with
    /// the fault — `mate_overconstrained` where the other mates already
    /// fix what it fixes), and the slot doors' refusals.
    fn constrain(
        &mut self,
        py: Python<'_>,
        placement: &NodeId,
        mate: &super::mate::Mate,
    ) -> PyResult<super::mate::MateRef> {
        let tol = Tol::witness();
        let mut action = d::Recording::start(&self.inner, tol);
        let edit_failed = |err: d::EditError| edit_err(py, &err);
        let minted = action
            .constrain(placement.0, mate.0.clone(), &d::EvalOptions::default())
            .map_err(edit_failed)?;
        let done = action.finish().map_err(edit_failed)?;
        self.take_up(done.doc, done.maintenance);
        let id = *minted
            .first()
            .unwrap_or_else(|| unreachable!("an accepted constraint mints its id"));
        Ok(super::mate::MateRef(d::MateRef::new(placement.0, id)))
    }

    /// The `(id, pin)` reference an instantiate node carries, or
    /// `None` for any other node.
    ///
    /// The read side of `Node.instantiate_part`: what a document says
    /// about which part it references, at which version — the value
    /// `mixed_pins` reports over and `update_reference` moves.
    fn reference(&self, node: &NodeId) -> Option<super::store::DocRef> {
        match self.inner.node(node.0) {
            Some(d::Node::InstantiatePart { doc_ref, .. }) => Some(super::store::DocRef(*doc_ref)),
            _ => None,
        }
    }

    /// The **interface record** an instantiate node carries, or `None`
    /// for any other node.
    ///
    /// Empty for a directly-authored instance — an authored instance
    /// crosses nothing. A non-empty record is minted only by the
    /// `split` that observed declarations crossing its cut.
    fn interface(&self, node: &NodeId) -> Option<super::refactor::InterfaceRecord> {
        match self.inner.node(node.0) {
            Some(d::Node::InstantiatePart { interface, .. }) => {
                Some(super::refactor::InterfaceRecord(interface.clone()))
            }
            _ => None,
        }
    }

    /// **What KIND of node `node` is** — one stable word per recipe
    /// node, and the read half of the `Node::*` constructor family.
    ///
    /// The vocabulary is [`crate::node_kind::node_kind`]'s, drawn from
    /// one exhaustive `match` over the kernel's `Node` with no
    /// wildcard arm, so a node kind added there and given no Python
    /// word does not compile. The words are listed on that function
    /// and, for callers, in `pncad.pyi`.
    ///
    /// This is the NODE's kind, never its VALUE's: an extrude, a
    /// transform and a placed union all evaluate to a value whose
    /// `kind` is `"body"`, because that tag is the PAYLOAD's shape.
    /// Telling the recipes apart is the question this door exists to
    /// answer, and it answers it with no evaluation in hand at all.
    ///
    /// **An id this document does not hold REFUSES** — `EditError`
    /// carrying `unknown_node`, the word the document layer already
    /// speaks for that state. The sibling reads on this class
    /// ([`Self::reference`], [`Self::interface`]) answer `None`
    /// instead, and the difference is not an inconsistency: their
    /// `None` is a real answer about a real node — it carries no
    /// reference, it carries no interface — so there is a live third
    /// state for the refusal to be distinguished from. Every live
    /// node HAS a kind, so a `None` here could only ever mean "no
    /// such node", and answering that as a value rather than a
    /// refusal is the fail-quiet this repo forbids.
    fn node_kind(&self, py: Python<'_>, node: &NodeId) -> PyResult<&'static str> {
        self.inner
            .node(node.0)
            .map(crate::node_kind::node_kind)
            .ok_or_else(|| {
                let err = d::EditError::UnknownNode {
                    id: d::SpokenNode::absent(node.0),
                };
                let message = format!(
                    "{}. Recourse: ask for the kind of a node this document holds",
                    err.problem()
                );
                edit_err_saying(py, &err, message)
            })
    }

    /// Insert a node and return its minted id — the common case,
    /// spelled without the intermediate `DocEdit`.
    ///
    /// `label=` labels the new node in the same call: the insert and a
    /// `DocEdit.set_label`, both applied or neither. A text that is not
    /// a label refuses before anything is applied (`label_blank`,
    /// `label_line_break`, `label_control_character`,
    /// `label_direction_control`).
    #[pyo3(signature = (node, *, label=None))]
    fn insert(&mut self, py: Python<'_>, node: &Node, label: Option<&str>) -> PyResult<NodeId> {
        let label = label.map(|text| label_from_text(py, text)).transpose()?;
        self.insert_node(node.inner.clone(), label)
            .map_err(|err| edit_err(py, &err))
    }

    /// **Insert measures** (ERROR-DESIGN E3, D10): one `Measure` node
    /// per primitive, in order, as one action — all land or none does.
    /// Answers a `Measured`: the measures and their outputs, the
    /// observed values an assertion reads. Arithmetic over them is an
    /// ordinary `Formula`: name an output (`DocEdit.rename_var`) and
    /// write the arithmetic with `Doc.parse_formula`; an assertion reads
    /// the formula, or `DocEdit.declare_var` names it.
    ///
    /// Exactly `insert(Node.measure(p))` per primitive, recorded as one
    /// action and answered with the outputs.
    fn measure(
        &mut self,
        py: Python<'_>,
        primitives: Vec<super::measure::MeasurePrimitive>,
    ) -> PyResult<Measured> {
        let primitives: Vec<d::MeasurePrimitive<d::Operand>> =
            primitives.into_iter().map(|p| p.inner).collect();
        let tol = Tol::witness();
        let mut action = d::Recording::start(&self.inner, tol);
        let measured = action
            .measure(&primitives)
            .map_err(|err| edit_err(py, &err))?;
        let done = action.finish().map_err(|err| edit_err(py, &err))?;
        self.take_up(done.doc, done.maintenance);
        Ok(Measured {
            measures: measured.measures.into_iter().map(NodeId).collect(),
            outputs: measured
                .outputs
                .into_iter()
                .map(|out| Var::of(&self.inner, out))
                .collect(),
        })
    }

    /// **"Sketch on this face"**: `outline` (one loop or a list, as
    /// `Node.profile` takes it) drawn on its own plane and extruded
    /// `distance` up its own `z`, the extrude's copy placed against
    /// `face` of `body` — its start cap resting on the face's plane,
    /// looking out along the face's outward normal, its origin over the
    /// face body's own and turned `spin` about the normal — and the
    /// face's body and the copy fused. `face` is one of the opaque
    /// texts `Evaluation.all_faces` / `select` answered with. `orient=`
    /// is the mate's `Orientation`: where the profile's axes lie among
    /// the face body's space's. Unwritten, it is the one nearest where
    /// the copy stands, the profile's `z` along the face's outward
    /// normal, read off the face; one putting `z` opposite the normal is
    /// the orientation written backwards (`mate_backwards`).
    ///
    /// One action: the four nodes, the mate and its values all land or
    /// none does, and the first door that refuses raises its
    /// `EditError`. Answers the four ids as a `SketchedOnFace`;
    /// `label=` labels the union, the body the gesture leaves.
    #[pyo3(signature = (body, face, outline, distance, spin=None, *, orient=None, label=None))]
    #[allow(clippy::too_many_arguments)] // the gesture's own arguments, each a seat
    fn sketch_on_face(
        &mut self,
        py: Python<'_>,
        body: OperandArg,
        face: &str,
        outline: &Bound<'_, PyAny>,
        distance: super::expr::SlotArg,
        spin: Option<super::expr::SlotArg>,
        orient: Option<super::mate::Orientation>,
        label: Option<&str>,
    ) -> PyResult<SketchedOnFace> {
        let label = label.map(|text| label_from_text(py, text)).transpose()?;
        let face = name_from_text(face)?;
        let profile = d::ProfileProgram {
            loops: loops_from_outline(py, outline)?,
            ids: Vec::new(),
            on: None,
        };
        let distance = slot_expr(py, d::SlotId::Distance, &distance)?;
        let spin = match spin {
            Some(spin) => super::pose::scalar_at(py, d::Dimension::Angle, &spin, "spin")?,
            None => d::Formula::zero(d::Dimension::Angle),
        };
        let tol = Tol::witness();
        let mut action = d::Recording::start(&self.inner, tol);
        let edit_failed = |err: d::EditError| edit_err(py, &err);
        let sketched = action
            .sketch_on_face(
                body.read(),
                face,
                profile,
                distance,
                spin,
                orient.map(|o| o.0),
                &d::EvalOptions::default(),
            )
            .map_err(edit_failed)?;
        if let Some(label) = label {
            action
                .apply(d::DocEdit::SetLabel {
                    node: sketched.union,
                    label: Some(label),
                })
                .map_err(edit_failed)?;
        }
        let done = action.finish().map_err(edit_failed)?;
        self.take_up(done.doc, done.maintenance);
        Ok(SketchedOnFace(sketched))
    }

    /// **A takeover**: each of `copies` (placements) re-pointed to copy
    /// `result`, which replaces the body they copied, and every
    /// selection that read one of them rebound where the result carries
    /// its face uniquely; a face the result splits, ties or consumes
    /// stays stale. One action. Answers each name moved, as
    /// `(copy, old name, new name)`, for a name held outside the
    /// document.
    fn take_over(
        &mut self,
        py: Python<'_>,
        copies: Vec<NodeId>,
        result: OperandArg,
    ) -> PyResult<Vec<(NodeId, String, String)>> {
        let copies: Vec<d::RecipeNodeId> = copies.into_iter().map(|c| c.0).collect();
        let result = result.read();
        let done = d::take_over(
            &self.inner,
            &copies,
            &result,
            &d::EvalOptions::default(),
            Tol::witness(),
        )
        .map_err(|err| edit_err(py, &err))?;
        let mut moved = Vec::new();
        for (copy, names) in &done.taken.reseated {
            for (from, to) in names {
                moved.push((NodeId(*copy), name_text(py, from)?, name_text(py, to)?));
            }
        }
        self.take_up(done.doc, done.maintenance);
        Ok(moved)
    }

    /// **Place one copy of `source` into `target`**, placed by
    /// `constraints`, and return the placement's id: the insert of
    /// `Node.place(source, target, constraints)`.
    ///
    /// `source` is one body argument: several bodies are placed as one
    /// by placing their `Doc.assemble`. `target` is the space the copy is placed into, always written:
    /// the world (`Doc.world`), a `Doc.new_space()`, a construction's
    /// own space (the construction itself, or `Doc.space`'s `Var`).
    /// `constraints` is the bundle, written whole: a `Placed` (its one
    /// constructor, which writes the constant zero on each freedom
    /// left unnamed), or a sequence of `Mate`s and `MateValue`s, each
    /// constraint's id minted by the insert in order. Every bundle pins
    /// its copy: a sequence that leaves a freedom unwritten, `[]` among
    /// them, refuses (`mate_unpinned`). `None` is the identity bundle,
    /// `Placed.identity()`: the copy stands in `target` where its
    /// body's own coordinates put it — the "place part" gesture.
    ///
    /// Nothing places as a side effect, and nothing a placement does
    /// puts its copy in the product: the product is the body
    /// `Doc.set_product` names. A split's half is placed by its output,
    /// `place(doc.output(split, 1))`.
    ///
    /// Two placements of one body are two copies. `label=` labels the
    /// placement in the same call, as `insert`'s does.
    #[pyo3(signature = (source, target, constraints=None, *, label=None))]
    fn place(
        &mut self,
        py: Python<'_>,
        source: BodyArg,
        target: SpaceArg,
        constraints: Option<super::mate::BundleArg>,
        label: Option<&str>,
    ) -> PyResult<NodeId> {
        let label = label.map(|text| label_from_text(py, text)).transpose()?;
        let constraints = constraints.map_or_else(|| d::stand().bundle(), super::mate::bundle);
        let (read, target) = (source.read(), target.read());
        Node::placed(py, read.clone(), target.clone(), constraints.clone())?;
        self.insert_edit(d::DocEdit::place(read, target, constraints), label)
            .map_err(|err| edit_err(py, &err))
    }

    /// **The assemble** (`⊔`): `members` side by side as the pieces of
    /// one body, none decided against another, and return its id: the
    /// insert of `Node.assemble(members)`. Its door runs the at-rest
    /// census over the pieces, one arrival per member; its result is a
    /// body like any other, which a boolean cuts or joins piece by
    /// piece, and which `Doc.set_product` makes the document's product.
    #[pyo3(signature = (members, *, label=None))]
    fn assemble(
        &mut self,
        py: Python<'_>,
        members: MembersArg,
        label: Option<&str>,
    ) -> PyResult<NodeId> {
        let label = label.map(|text| label_from_text(py, text)).transpose()?;
        self.insert_edit(
            d::DocEdit::InsertNode {
                node: Box::new(d::Node::Assemble {
                    members: members.bodies(),
                }),
                fresh: Vec::new(),
            },
            label,
        )
        .map_err(|err| edit_err(py, &err))
    }

    /// **One piece of the copy `placement` reads** (D10
    /// `[placement.placement]`): `of` is the path to the piece's
    /// variable in the source, each step an operand of the `⊔` the walk
    /// reaches through the placements before it — `[bolt]` for the bolt
    /// of a copy of `bolt ⊔ plate`. Read at a seat; a path through any
    /// other operator raises `EditError` (`pick_path`) at the insert.
    fn pick(&self, placement: OperandArg, of: Vec<Var>) -> PickRead {
        PickRead {
            placement: Box::new(placement),
            of: of.into_iter().map(|var| var.0).collect(),
        }
    }

    /// **Set the product** (D10 `[operations]`): the one body the
    /// document delivers, read in the world, or `None` for an empty
    /// product. What export writes and an instance of the document
    /// copies. A body in another space raises `EditError`
    /// (`product_read`).
    fn set_product(&mut self, py: Python<'_>, body: Option<OperandArg>) -> PyResult<()> {
        let applied = d::apply(
            &self.inner,
            &d::DocEdit::SetProduct {
                product: body.map(OperandArg::read),
            },
            Tol::witness(),
        )
        .map_err(|err| edit_err(py, &err))?;
        self.accept(applied);
        Ok(())
    }

    /// **The product** (`Doc.set_product`): the variable the slot
    /// reads, `None` for an empty product.
    #[getter]
    fn product(&self) -> Option<Var> {
        self.inner
            .product()
            .map(|var| Var(var, Some(d::VarKind::Body)))
    }

    /// **`body`'s own frame at a corner it has**, as a pose to read: the
    /// frame through its straight edge from `corner` to `up` (its `z`,
    /// that way) and its vertex at `toward` (its `x` the way to it),
    /// each picked where the evaluation puts it exactly, in `body`'s own
    /// coordinates. Where a box's edges run along its own axes this is
    /// its own frame read with no number, so a frame moved from it by
    /// exact turns (`PoseDef.spin` by `Formula.turn_over(4)`) stands
    /// exactly where it is meant. A body with no such edge and vertex
    /// raises `EditError` under the pin refusal's word.
    #[pyo3(signature = (body, corner, up, toward, *, resolver=None))]
    fn corner_frame(
        &self,
        py: Python<'_>,
        body: OperandArg,
        corner: Lengths3,
        up: Lengths3,
        toward: Lengths3,
        resolver: Option<&super::store::Workspace>,
    ) -> PyResult<super::pose::PoseDef> {
        let at = |v: Lengths3| [v.0.0.meters(), v.1.0.meters(), v.2.0.meters()];
        let opts = d::EvalOptions {
            resolver: seam(resolver),
            ..d::EvalOptions::default()
        };
        d::corner_frame(
            &self.inner,
            &body.read(),
            at(corner),
            at(up),
            at(toward),
            &opts,
            Tol::witness(),
        )
        .map(|inner| super::pose::PoseDef { inner })
        .map_err(|refusal| {
            boundary_edit_err(
                py,
                BoundaryEdit::Pin(&refusal),
                format!("the body has no such corner: {refusal}"),
            )
        })
    }

    /// **Remove the constraint `mate` addresses**, the gesture over
    /// `DocEdit.remove_constraint`: the bundle written whole again
    /// without it, so the copy stays pinned, the slides it leaves read
    /// off the copy's geometry as `Doc.constrain` reads them. A mate taken away frees what it fixed, and a value taken
    /// away leaves its freedom; each value the remaining mates still
    /// leave is kept, and each freedom they leave that no value then
    /// sets is written the constant `0` (`Placed`'s rule). Answers the
    /// addresses of the values that rewrite wrote.
    ///
    /// Refuses typed on `EditError`: `unknown_node`,
    /// `constraint_on_non_placement`, `unknown_constraint` (the bundle
    /// holds no such id).
    fn remove_mate(
        &mut self,
        py: Python<'_>,
        mate: &super::mate::MateRef,
    ) -> PyResult<Vec<super::mate::MateRef>> {
        let at = mate.0;
        let mut action = d::Recording::start(&self.inner, Tol::witness());
        let edit_failed = |err: d::EditError| edit_err(py, &err);
        let minted = action
            .remove_mate(at.placement, at.id, &d::EvalOptions::default())
            .map_err(edit_failed)?;
        let done = action.finish().map_err(edit_failed)?;
        self.take_up(done.doc, done.maintenance);
        Ok(minted
            .iter()
            .map(|&id| super::mate::MateRef(d::MateRef::new(at.placement, id)))
            .collect())
    }

    /// The label a person gave `node`, or `None` when it has none.
    ///
    /// A label is document data beside the node: not unique, never
    /// identity, and set or cleared only by `DocEdit.set_label` (or
    /// `insert(..., label=...)`).
    ///
    /// Raises `EditError` (`unknown_node`) for a node this document
    /// does not hold, as `node_kind` does: `None` is the answer about
    /// a held node with no label, so it cannot also mean "no such
    /// node".
    fn label(&self, py: Python<'_>, node: &NodeId) -> PyResult<Option<String>> {
        if self.inner.node(node.0).is_none() {
            let err = d::EditError::UnknownNode {
                id: d::SpokenNode::absent(node.0),
            };
            let message = format!(
                "{}. Recourse: ask for the label of a node this document holds",
                err.problem()
            );
            return Err(edit_err_saying(py, &err, message));
        }
        Ok(self
            .inner
            .label(node.0)
            .map(|label| label.as_str().to_owned()))
    }

    /// How many nodes the document holds.
    #[getter]
    fn node_count(&self) -> usize {
        self.inner.len()
    }

    /// The document's nodes in id order, which is the order they were
    /// inserted in. The world (`Doc.world`) is no inserted node, so it
    /// is not among them, as `node_count` does not count it.
    fn order(&self) -> Vec<NodeId> {
        self.inner
            .ids()
            .into_iter()
            .filter(|&id| id != d::RecipeNodeId::WORLD)
            .map(NodeId)
            .collect()
    }

    /// **The document's named free parameters**, by name, in
    /// declaration order (`Doc::var_names`, each read through
    /// `Doc::free`). A defined variable is listed by
    /// [`Self::definitions`] instead: it holds no value, notation or
    /// distribution of its own.
    ///
    /// The read side of a free `DocEdit.declare_var`, and the only door
    /// that answers a whole parameter back: `Doc.eval` answers a
    /// parameter reference's NUMBER, with the dimension and the
    /// authored notation both erased, so a consumer showing a
    /// parameter — or checking that one it wrote is still written the
    /// way it wrote it — had nowhere to look.
    ///
    /// A snapshot, not a view: the map is built here and mutating it
    /// changes no document.
    #[getter]
    fn params<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyDict>> {
        let out = PyDict::new(py);
        // In declaration order: the dict's own order is the document's.
        for (id, param) in self.inner.free_vars() {
            if let Some(name) = self.inner.var_name(id) {
                out.set_item(VarName(name.clone()), FreeVar(param.clone()))?;
            }
        }
        Ok(out)
    }

    /// **The document's named defined variables**, by name, in
    /// declaration order: each one's definition, reading variables by
    /// id. The read side of a defined `DocEdit.declare_var` and of
    /// `DocEdit.define_var`; with [`Self::params`] it lists every named
    /// variable once.
    ///
    /// A snapshot, not a view: the map is built here and mutating it
    /// changes no document.
    #[getter]
    fn definitions<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyDict>> {
        let out = PyDict::new(py);
        for id in self.inner.var_ids() {
            let Some(expr) = self.inner.var(id).and_then(|v| v.def().defined()) else {
                continue;
            };
            if let Some(name) = self.inner.var_name(id) {
                out.set_item(VarName(name.clone()), super::expr::Expr(expr.clone()))?;
            }
        }
        Ok(out)
    }

    /// **The document's free variables**, by identity, in declaration
    /// order — the named ones and the anonymous ones (whose identity is
    /// all an edit can address them by). A defined variable's
    /// definition is [`Self::definition`].
    ///
    /// A snapshot, not a view: the map is built here and mutating it
    /// changes no document.
    #[getter]
    fn vars<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyDict>> {
        let out = PyDict::new(py);
        for (id, param) in self.inner.free_vars() {
            out.set_item(Var::of(&self.inner, id), FreeVar(param.clone()))?;
        }
        Ok(out)
    }

    /// **The definition of `var`**, when it is a defined variable: the
    /// expression it was defined by, reading variables by id. `None`
    /// for a free variable, or one the document does not hold.
    fn definition(&self, var: &Var) -> Option<super::expr::Expr> {
        self.inner
            .var(var.0)
            .and_then(|v| v.def().defined())
            .map(|expr| super::expr::Expr(expr.clone()))
    }

    /// The variable this document names `name`, or `None`.
    fn var(&self, name: &VarName) -> Option<Var> {
        self.inner
            .var_named(name.0.as_str())
            .map(|id| Var::of(&self.inner, id))
    }

    /// **The variable port `port` of `node` defines** (`Doc::output`):
    /// an operation's output, which lives exactly as long as its node.
    /// `None` for a node the document does not hold; a port the live
    /// node's signature does not have raises `ValueError`.
    #[pyo3(signature = (node, port = 0))]
    fn output(&self, node: &NodeId, port: u8) -> PyResult<Option<Var>> {
        let Some(signature) = self.inner.signature(node.0) else {
            return Ok(None);
        };
        match self.inner.output(node.0, port) {
            Some(var) => Ok(Some(Var::of(&self.inner, var))),
            None => Err(pyo3::exceptions::PyValueError::new_err(format!(
                "{} defines {} port(s), so it has no port {port}",
                self.inner.spoken(node.0),
                signature.len()
            ))),
        }
    }

    /// **The variable a node's slot reads** (`Doc::slot`): an
    /// expression slot's variable or an operand's read, or `None` for a
    /// node or a slot the document does not hold. Every slot reads one:
    /// a value written there is its own anonymous variable, and passing
    /// the handle to another slot is how two slots share it.
    fn slot(&self, node: &NodeId, slot: &str) -> PyResult<Option<Var>> {
        Ok(self
            .inner
            .slot(node.0, slot_from_text(slot)?)
            .map(|id| Var::of(&self.inner, id)))
    }

    /// The name this document holds for `var`, or `None` — for an
    /// anonymous variable, or one the document no longer holds.
    fn var_name(&self, var: &Var) -> Option<VarName> {
        self.inner.var_name(var.0).cloned().map(VarName)
    }

    /// **What pattern `placement` is**, for display
    /// (`pncad::pattern::recognise`): a row where it is a placement
    /// under exactly one index with no mate whose every non-zero value
    /// is a slide a multiple of the index, a grid where it is under two
    /// and slides along each, a ring where it has one mate and its spin
    /// is that multiple. `None` for any other node. It reads the recipe
    /// as written and decides nothing.
    fn recognise(&self, placement: &NodeId) -> Option<Recognised> {
        pncad::pattern::recognise(&self.inner, placement.0).map(|r| {
            let (index, within) = match &r {
                pncad::pattern::Recognised::Linear { index, .. }
                | pncad::pattern::Recognised::Circular { index, .. } => (*index, None),
                pncad::pattern::Recognised::Grid { outer, inner } => (outer.0, Some(inner.0)),
            };
            Recognised {
                index: Var::of(&self.inner, index),
                inner_index: within.map(|var| Var::of(&self.inner, var)),
                inner: r,
            }
        })
    }

    /// **The text of `expr`**, each variable it reads written by the
    /// name this document holds for it (`Doc::unparse`); one with no
    /// name here writes its full id, `#<ordinal>:<16 hex>`.
    fn unparse(&self, expr: super::expr::EitherForm) -> String {
        match expr {
            super::expr::EitherForm::Formula(formula) => self.inner.unparse(&formula.0),
            super::expr::EitherForm::Expr(expr) => self.inner.unparse(&expr.0),
        }
    }

    /// The document's tolerance.
    #[getter]
    fn epsilon(&self) -> f64 {
        self.inner.epsilon()
    }

    /// Bit-exact document equality (D9's replay currency).
    fn bit_eq(&self, other: &Self) -> bool {
        self.inner.bit_eq(&other.inner)
    }

    /// **Read `source` as an expression** against this document's
    /// declared parameters (`parse_formula`).
    ///
    /// The one door inward. The parser is CHECKING — every reduction
    /// runs the expression layer's smart constructors — so a text
    /// that survives it is a dimension-checked tree, and the whole
    /// algebra (operators, `sin`/`cos`/`tan`/`atan2`/`min`/`max`,
    /// unit suffixes, parameter references) is reachable through this
    /// one call.
    ///
    /// The DECLARATIONS come from the document, not from the caller:
    /// a bare identifier is a reference to a parameter this document
    /// declares, and it carries that parameter's dimension. So
    /// `"width / 2.0"` parses in a document that declares `width`
    /// and refuses `unknown_param` in one that does not — which is
    /// the same table `apply` re-checks a stored expression against,
    /// not a second one. Note the `2.0`: a bare integer literal is an
    /// exact `Count`, and dividing a length by one needs an explicit
    /// promotion, so the decimal point is what makes the divisor
    /// dimensionless.
    ///
    /// An expression nests at most 128 levels along its longest chain
    /// from the root to a leaf. The operators associate to the left, so
    /// a flat chain of more than 128 terms (`"a + b + ..."`) refuses
    /// `nested_too_deep`; the same terms grouped (`"(a + b) + (c + d)"`)
    /// nest less. Brackets alone nest nothing.
    ///
    /// Refuses typed on `ParseError`, carrying `variant` and the byte
    /// offset `pos`; a reduction the dimension checker refused
    /// arrives as `variant == "dimension"` with the constructor's own
    /// tag as `kind`.
    fn parse_formula(&self, py: Python<'_>, source: &str) -> PyResult<super::expr::Formula> {
        let declared = self.inner.var_scope();
        d::parse_formula(source, &declared)
            .map(super::expr::Formula)
            .map_err(|err| super::expr::parse_err(py, &err))
    }

    /// **This expression's value** under the document's current
    /// parameter values (`eval`).
    ///
    /// Answers a `Length` for a length expression, an `Angle` for an
    /// angle, and a bare `float` for a dimensionless one — the
    /// crossing rule the rest of this surface follows, rather than
    /// the kernel-unit `f64` the Rust door returns with its dimension
    /// erased.
    ///
    /// **Not `evaluate(doc)`.** That one runs the RECIPE and answers
    /// geometry; this one runs one expression's arithmetic and
    /// answers a number. Nothing about the document changes either
    /// way.
    ///
    /// A `Count` expression does not evaluate here: counts are exact,
    /// and promotion to a continuous value is explicit in the
    /// expression language or not at all, so this refuses
    /// `count_expr_in_continuous_eval` and `eval_count` is the door.
    /// The expression's names are read against this document by the
    /// edit door's lowering, whose refusals ride `EvalError` too:
    /// `unlowered_name` names a name no variable holds at the dimension
    /// it is read at, and `var_kind_mismatch` one held at another.
    /// Refuses typed on `EvalError` otherwise — `unresolved_var` a
    /// variable the document no longer holds, and
    /// `non_finite_result` is the arithmetic having overflowed or hit
    /// a pole.
    ///
    /// A `Var` evaluates as the lone reader of it at its own dimension
    /// (`eval_var`): how a slot's value is read off `Doc.slot`'s
    /// handle, a typed value's anonymous variable or a formula's
    /// anonymous definition alike.
    fn eval(&self, py: Python<'_>, expr: Evaluand) -> PyResult<Py<PyAny>> {
        let expr = self.evaluand(py, expr)?;
        let env = self.inner.var_env::<f64>();
        let value = d::eval(&self.authored(py, &expr)?, &env)
            .map_err(|err| super::expr::eval_err(py, &err, Some(&self.inner)))?;
        // Re-dimensioning what `eval` erased: the expression's own
        // dimension is what says which quantity the number is, and it
        // is correct by construction. `Count` cannot reach here — the
        // evaluator refused it above — and `Scalar` is dimensionless
        // by definition, so both fall to the bare float.
        match expr.dim() {
            d::Dimension::Length => Py::new(
                py,
                super::quantity::Length(pncad::quantity::Length::from_meters(value)),
            )
            .map(|v| v.into_any()),
            d::Dimension::Angle => Py::new(
                py,
                super::quantity::Angle(pncad::quantity::Angle::from_radians(value)),
            )
            .map(|v| v.into_any()),
            // `f64`'s conversion into Python is INFALLIBLE (its error
            // type is `Infallible`), so this arm cannot fail; the
            // match is what says so.
            d::Dimension::Count | d::Dimension::Scalar => match value.into_pyobject(py) {
                Ok(bound) => Ok(bound.unbind().into_any()),
            },
        }
    }

    /// **This count expression's exact value** under the document's
    /// current parameter values (`eval_count`).
    ///
    /// Exact integer arithmetic, never floating point: a count that
    /// overflows refuses `count_overflow` rather than wrapping, since
    /// a wrapped count is a fabricated one. A non-`Count` expression
    /// refuses `continuous_expr_in_count_eval` naming the dimension
    /// it actually has — a count is never inferred from a continuous
    /// value.
    /// A count `Var` evaluates as its lone reader (`eval_var_count`).
    fn eval_count(&self, py: Python<'_>, expr: Evaluand) -> PyResult<i64> {
        let expr = self.evaluand(py, expr)?;
        let env = self.inner.var_env::<f64>();
        d::eval_count(&self.authored(py, &expr)?, &env)
            .map_err(|err| super::expr::eval_err(py, &err, Some(&self.inner)))
    }

    /// Serialize this document to the persistence text format
    /// (the schema-v4 doors, via the curated façade).
    ///
    /// The Python wrapper holds only the CURRENT document — its edit
    /// history lives in the Rust values it discarded — so the file is
    /// a snapshot with an empty edit log. That is a complete, loadable
    /// document; the GUI's edit-log-bearing files load through the
    /// same `load` door.
    fn save(&self, py: Python<'_>) -> PyResult<String> {
        let tol = Tol::witness();
        d::save(&self.inner, &[], tol).map_err(|err| persist_err(py, &err))
    }

    fn __len__(&self) -> usize {
        self.inner.len()
    }

    fn __repr__(&self) -> String {
        format!("Doc({} nodes)", self.inner.len())
    }
}

/// **One move of a loft section's station**: a slide along, or a
/// spin about, one of the first section's own axes (`"x"`, `"y"` or
/// `"z"`) through its origin, by one length or one angle — the
/// kernel's `StationMove`, crossing whole.
#[pyclass(frozen, module = "pncad", from_py_object)]
#[derive(Clone)]
pub(crate) struct StationMove(pub(crate) d::StationMove<d::Formula>);

/// The first section's axis a station move names.
fn station_axis(axis: &str) -> PyResult<d::Axis3> {
    match axis {
        "x" => Ok(d::Axis3::X),
        "y" => Ok(d::Axis3::Y),
        "z" => Ok(d::Axis3::Z),
        other => Err(pyo3::exceptions::PyValueError::new_err(format!(
            "a station moves along or about \"x\", \"y\" or \"z\", not {other:?}"
        ))),
    }
}

/// The word [`station_axis`] reads `axis` from.
fn station_axis_word(axis: d::Axis3) -> &'static str {
    match axis {
        d::Axis3::X => "x",
        d::Axis3::Y => "y",
        d::Axis3::Z => "z",
    }
}

#[pymethods]
impl StationMove {
    /// Slid along the first section's `axis` by the length `by`.
    #[staticmethod]
    fn slide(py: Python<'_>, axis: &str, by: super::expr::SlotArg) -> PyResult<Self> {
        Ok(Self(d::StationMove::Slide {
            along: station_axis(axis)?,
            by: super::pose::scalar_at(py, d::Dimension::Length, &by, "station slide")?,
        }))
    }

    /// Spun about the first section's `axis` by the angle `by`,
    /// right-handed.
    #[staticmethod]
    fn spin(py: Python<'_>, axis: &str, by: super::expr::SlotArg) -> PyResult<Self> {
        Ok(Self(d::StationMove::Spin {
            about: station_axis(axis)?,
            by: super::pose::scalar_at(py, d::Dimension::Angle, &by, "station spin")?,
        }))
    }

    /// `"slide"` or `"spin"`.
    #[getter]
    fn kind(&self) -> &'static str {
        match self.0 {
            d::StationMove::Slide { .. } => "slide",
            d::StationMove::Spin { .. } => "spin",
        }
    }

    /// The first section's axis moved along or about.
    #[getter]
    fn axis(&self) -> &'static str {
        match self.0 {
            d::StationMove::Slide { along: axis, .. }
            | d::StationMove::Spin { about: axis, .. } => station_axis_word(axis),
        }
    }

    /// How far, as the formula it was written as.
    #[getter]
    fn by(&self) -> super::expr::Formula {
        match &self.0 {
            d::StationMove::Slide { by, .. } | d::StationMove::Spin { by, .. } => {
                super::expr::Formula(by.clone())
            }
        }
    }

    /// Bit-exact equality: the same move, its number equal.
    fn __eq__(&self, other: &Self) -> bool {
        self.0 == other.0
    }

    fn __repr__(&self) -> String {
        let super::expr::Formula(by) = self.by();
        format!("StationMove.{}({:?}, {by})", self.kind(), self.axis())
    }
}

/// One step of a row as Python writes it, `(axis, pitch)`: the axis by
/// its word, the pitch a length.
fn pattern_step(
    py: Python<'_>,
    axis: &str,
    pitch: &super::expr::SlotArg,
) -> PyResult<(d::TargetAxis, d::Formula)> {
    Ok((
        super::mate::target_axis(axis)?,
        super::pose::scalar_at(py, d::Dimension::Length, pitch, "row's pitch")?,
    ))
}

/// A ring's axis as Python writes it, `(on, to, along)`.
fn ring_axis(
    (on, to, along): (OperandArg, OperandArg, String),
) -> PyResult<(d::Operand, d::Operand, d::TargetAxis)> {
    Ok((on.read(), to.read(), super::mate::target_axis(&along)?))
}

/// The node a pattern writer authored. Every argument's dimension is
/// read at the boundary first, so a refusal here is a product the
/// writer cannot form.
fn pattern_node(authored: Result<d::AuthoredNode, d::DimensionError>) -> PyResult<Node> {
    authored
        .map(|inner| Node { inner })
        .map_err(|err| pyo3::exceptions::PyValueError::new_err(err.to_string()))
}

/// **A placement read back as the pattern it is** (`Doc.recognise`):
/// a row (`kind == "linear"`, its `steps`), a grid (`kind == "grid"`,
/// `steps` along `index` and `inner_steps` along `inner_index`) or a
/// ring (`kind == "circular"`, its `step`), under `index`.
#[pyclass(frozen, module = "pncad")]
pub(crate) struct Recognised {
    inner: pncad::pattern::Recognised,
    index: Var,
    inner_index: Option<Var>,
}

#[pymethods]
impl Recognised {
    /// `"linear"`, `"grid"` or `"circular"`.
    #[getter]
    fn kind(&self) -> &'static str {
        match self.inner {
            pncad::pattern::Recognised::Linear { .. } => "linear",
            pncad::pattern::Recognised::Grid { .. } => "grid",
            pncad::pattern::Recognised::Circular { .. } => "circular",
        }
    }

    /// The index: a grid's outer.
    #[getter]
    fn index(&self) -> Var {
        self.index
    }

    /// A grid's inner index; `None` for a row or a ring.
    #[getter]
    fn inner_index(&self) -> Option<Var> {
        self.inner_index
    }

    /// A row's slides, or a grid's along its outer index, each axis
    /// (`"x"`, `"y"`, `"z"`) and its pitch per member; empty for a ring.
    #[getter]
    fn steps(&self) -> Vec<(&'static str, super::expr::Formula)> {
        match &self.inner {
            pncad::pattern::Recognised::Linear { steps, .. }
            | pncad::pattern::Recognised::Grid {
                outer: (_, steps), ..
            } => slides(steps),
            pncad::pattern::Recognised::Circular { .. } => Vec::new(),
        }
    }

    /// A grid's slides along its inner index; empty for a row or a ring.
    #[getter]
    fn inner_steps(&self) -> Vec<(&'static str, super::expr::Formula)> {
        match &self.inner {
            pncad::pattern::Recognised::Grid {
                inner: (_, steps), ..
            } => slides(steps),
            _ => Vec::new(),
        }
    }

    /// A ring's turn per member; `None` for a row.
    #[getter]
    fn step(&self) -> Option<super::expr::Formula> {
        match &self.inner {
            pncad::pattern::Recognised::Circular { step, .. } => {
                Some(super::expr::Formula(step.clone()))
            }
            pncad::pattern::Recognised::Linear { .. } | pncad::pattern::Recognised::Grid { .. } => {
                None
            }
        }
    }

    fn __repr__(&self) -> String {
        format!("Recognised.{}({})", self.kind(), self.index.__repr__())
    }
}

/// Slides as Python reads them: each axis word and its pitch.
fn slides(
    steps: &[(pncad::document::TargetAxis, pncad::document::Formula)],
) -> Vec<(&'static str, super::expr::Formula)> {
    steps
        .iter()
        .map(|(axis, pitch)| {
            (
                super::mate::axis_word(*axis),
                super::expr::Formula(pitch.clone()),
            )
        })
        .collect()
}

/// **The loop programs an `outline` argument spells** — ONE
/// `ClosedLoop` or a list of them, in the order they were written
/// (`[outer, hole, hole]`) — the one reading both doors that take a
/// profile description share: `Node.profile`, which mints a profile
/// from it, and `DocEdit.set_program`, which writes it over a live
/// profile's program. One reading, so the two doors cannot disagree
/// about what a description is.
///
/// Nothing about the loop SET is pre-checked here: which loop is
/// outer, whether the holes nest, whether two loops cross, is the
/// kernel's own typed refusal at the door that consumes the loops.
/// A value that is neither a loop nor a sequence of them is
/// `extract`'s own `TypeError`, so a stringly-typed or numeric
/// argument still refuses at the boundary rather than being iterated
/// into nonsense.
fn loops_from_outline(
    py: Python<'_>,
    outline: &Bound<'_, PyAny>,
) -> PyResult<Vec<d::LoopProgram<d::Formula>>> {
    match outline.cast::<super::path::ClosedLoop>() {
        Ok(one) => Ok(vec![super::path::loop_program(py, &one.borrow())?]),
        Err(_) => {
            let many: Vec<PyRef<'_, super::path::ClosedLoop>> = outline.extract()?;
            many.iter()
                .map(|l| super::path::loop_program(py, l))
                .collect::<PyResult<Vec<_>>>()
        }
    }
}

/// A recipe node, before it is inserted into a document.
#[pyclass(frozen, module = "pncad", from_py_object)]
#[derive(Clone)]
pub(crate) struct Node {
    pub(crate) inner: d::AuthoredNode,
}

impl Node {
    /// Each expression slot of `node` checked against the slot it
    /// lands in, so a refusal names that slot.
    pub(crate) fn check_slots(py: Python<'_>, node: &d::AuthoredNode) -> PyResult<()> {
        for slot in node.slots() {
            if let Some(expr) = node.expr(slot) {
                slot_expr(
                    py,
                    slot,
                    &super::expr::SlotArg::Formula(super::expr::Formula(expr.clone())),
                )?;
            }
        }
        Ok(())
    }

    /// A `Place` of `source` into `target` holding `constraints`,
    /// each number checked against its slot. The ids are placeholders
    /// the insert door mints.
    pub(crate) fn placed(
        py: Python<'_>,
        source: d::BodyRead<d::Formula>,
        target: d::Operand,
        constraints: Vec<d::Constraint<d::Formula>>,
    ) -> PyResult<Self> {
        let inner = d::Node::place(source, target, constraints);
        Self::check_slots(py, &inner)?;
        Ok(Self { inner })
    }
}

#[pymethods]
impl Node {
    /// A closed polygonal sketch, drawn on its own xy.
    ///
    /// Coordinates arrive as length `Formula`s — the profile program's
    /// own slot type — so a vertex can be a written `25 mm`, a
    /// canonical literal or a parameter reference, and a bare number
    /// is a boundary refusal rather than an ambiguous unit. A profile
    /// is 2-D: it has no location, and where a body built from it
    /// stands is that body's placement's.
    #[staticmethod]
    fn polygon(
        py: Python<'_>,
        points: Vec<(super::expr::SlotArg, super::expr::SlotArg)>,
    ) -> PyResult<Self> {
        // The polygon as a loop PROGRAM (the post-switch v4 payload:
        // the program IS the profile's definition). The expansion
        // itself is `LoopProgram::polygon_expr`'s, not this door's:
        // the binding checks each vertex against the program slot it
        // lands in and hands the corners over. Too few points is not
        // pre-checked — the edit door's replay probe refuses it typed,
        // at `insert`.
        let point = |py2: Python<'_>,
                     step: usize,
                     p: &(super::expr::SlotArg, super::expr::SlotArg)|
         -> PyResult<[d::Formula; 2]> {
            let at = |arg| d::SlotId::Profile {
                loop_: 0,
                step: u32::try_from(step).unwrap_or(u32::MAX),
                arg,
            };
            Ok([
                slot_expr(py2, at(d::StepArg::PointX), &p.0)?,
                slot_expr(py2, at(d::StepArg::PointY), &p.1)?,
            ])
        };
        let corners = points
            .iter()
            .enumerate()
            .map(|(step, p)| point(py, step, p))
            .collect::<PyResult<Vec<_>>>()?;
        Ok(Self {
            inner: d::Node::Profile(d::ProfileProgram {
                loops: vec![d::LoopProgram::polygon_expr(corners)],
                ids: Vec::new(),
                on: None,
            }),
        })
    }

    /// The profile a PATHS loop authors, drawn on its own xy.
    ///
    /// The node is built from the loop's RECORDED program — the same
    /// verbs, with the same authored arguments, that the chain wrote
    /// — so what Python authored and what the document replays are
    /// one program, not two spellings of one shape.
    ///
    /// `outline` is ONE loop or a LIST of them — a plate with holes is
    /// `[outer, hole, hole]`, in that order (`ProfileProgram.loops` is
    /// a `Vec`, and this is the whole of it).
    ///
    /// Nothing about the loop SET is pre-checked. Which loop is outer,
    /// whether the holes nest, whether two loops cross — all of that
    /// is `Profile::validate`'s work, and it fires as the kernel's own
    /// typed refusal at `evaluate`. The binding's only job is that the
    /// loops arrive in the order they were written.
    #[staticmethod]
    fn profile(py: Python<'_>, outline: &Bound<'_, PyAny>) -> PyResult<Self> {
        let loops = loops_from_outline(py, outline)?;
        Ok(Self {
            inner: d::Node::Profile(d::ProfileProgram {
                loops,
                ids: Vec::new(),
                on: None,
            }),
        })
    }

    /// Extrude an upstream profile up its own +z.
    ///
    /// `distance` is the node's `distance` slot, a depth: a size,
    /// refused at `evaluate` unless definitely positive. The body lies
    /// in its own coordinates, between its profile's xy and `distance`
    /// up; a body that stands below a plane is placed so.
    /// `DocEdit.set_param(node, "distance", expr)` is what moves the
    /// depth, and what makes it a named, editable number: a literal is
    /// a new document per value, a parameter reference is one
    /// `set_var_value` per value.
    #[staticmethod]
    fn extrude(
        py: Python<'_>,
        profile: OperandArg,
        distance: super::expr::SlotArg,
    ) -> PyResult<Self> {
        let distance = slot_expr(py, d::SlotId::Distance, &distance)?;
        Ok(Self {
            inner: d::Node::Extrude {
                profile: profile.read(),
                distance,
            },
        })
    }

    /// Revolve an upstream profile about an axis line in its own plane.
    ///
    /// `axis_through` and `axis_heading` are the line in the profile's
    /// own 2-D coordinates: a point on it, two lengths, and its heading
    /// from the profile's x axis, an angle (`0` along x, `turn/4` along
    /// y). Two lengths and an angle in the plane cannot leave it, so no
    /// residual is checked. The revolve's second output
    /// (`Doc.output(revolve, 1)`) is the line lifted to 3-D, an axis
    /// other nodes read.
    ///
    /// `angle` is the node's `revolve_angle` slot —
    /// `DocEdit.set_param` is what drives it afterwards, as it is for
    /// an extrude's `distance`.
    #[staticmethod]
    fn revolve(
        py: Python<'_>,
        profile: OperandArg,
        axis_through: (super::expr::SlotArg, super::expr::SlotArg),
        axis_heading: super::expr::SlotArg,
        angle: super::expr::SlotArg,
    ) -> PyResult<Self> {
        let angle = slot_expr(py, d::SlotId::RevolveAngle, &angle)?;
        Ok(Self {
            inner: d::Node::Revolve {
                profile: profile.read(),
                axis_through: [
                    slot_expr(py, d::SlotId::Origin(d::Axis3::X), &axis_through.0)?,
                    slot_expr(py, d::SlotId::Origin(d::Axis3::Y), &axis_through.1)?,
                ],
                axis_heading: slot_expr(py, d::SlotId::AxisHeading, &axis_heading)?,
                angle,
            },
        })
    }

    /// **A solid tube** — a ring torus, or an elbow of one, from its
    /// INTENT parameters.
    ///
    /// The tube lies in its own coordinates: its spine is the circle
    /// about its own z through its own origin, and its own x is the
    /// reference direction the window's angles are measured from. Where
    /// it stands is its placement's.
    ///
    /// # This is not `Node.revolve` of a circle
    ///
    /// It would build a different body. A revolve reconstructs its
    /// minor radius through the profile's bulge arithmetic; this door
    /// stores the number you gave it, so `minor_radius` comes back
    /// out of the body bit for bit.
    ///
    /// # What refuses, and what is normalized instead
    ///
    /// A degenerate or reversed window, a window reaching one full
    /// period (say `TubeWindow.full()` instead), and the ring-torus
    /// convention `R > r > 0` — each is the kernel's own typed refusal
    /// at `evaluate`, tagged `tube`.
    ///
    /// There is no wall argument: a tube with a wall is
    /// `Node.hollow_tube`, a different node kind.
    #[staticmethod]
    fn tube(
        py: Python<'_>,
        major_radius: super::expr::SlotArg,
        window: &TubeWindow,
        minor_radius: super::expr::SlotArg,
    ) -> PyResult<Self> {
        Ok(Self {
            inner: d::Node::Tube {
                major_radius: slot_expr(py, d::SlotId::TubeMajorRadius, &major_radius)?,
                window: window.inner.clone(),
                minor_radius: slot_expr(py, d::SlotId::TubeMinorRadius, &minor_radius)?,
            },
        })
    }

    /// **A hollow tube** — `Node.tube`'s sibling with a WALL.
    ///
    /// `minor_radius` is the OUTER cross-sectional radius and `wall`
    /// the thickness; the inner wall stores `minor_radius - wall`,
    /// one subtraction of your own two numbers, so you recover it by
    /// writing the same subtraction.
    ///
    /// **`wall` is REQUIRED and has no default.** Hollowness is
    /// spelled by which door you call, not by whether you passed an
    /// argument: a solid tube and a hollow one are different
    /// artifacts — a full disc against an annulus — and there is no
    /// `wall=None` that quietly turns this into `Node.tube`.
    ///
    /// A `TubeWindow.full()` builds a torus SHELL, whose cavity is a
    /// void; an arc builds an open elbow of annular section.
    ///
    /// # Nothing is pre-checked, and the wall least of all
    ///
    /// Everything `Node.tube` refuses, this refuses identically, plus
    /// three verdicts only this door can raise: the thickness is not
    /// positive at tolerance, `minor_radius - wall` is not a bore,
    /// and — the one neither of the others can see — the gap between
    /// the two radii the body would STORE collapses, because at a
    /// large outer radius the subtraction rounds the inner radius
    /// back onto the outer. All three are the kernel's, decided
    /// before anything is minted, and all three arrive tagged `tube`.
    #[staticmethod]
    fn hollow_tube(
        py: Python<'_>,
        major_radius: super::expr::SlotArg,
        window: &TubeWindow,
        minor_radius: super::expr::SlotArg,
        wall: super::expr::SlotArg,
    ) -> PyResult<Self> {
        Ok(Self {
            inner: d::Node::HollowTube {
                major_radius: slot_expr(py, d::SlotId::TubeMajorRadius, &major_radius)?,
                window: window.inner.clone(),
                minor_radius: slot_expr(py, d::SlotId::TubeMinorRadius, &minor_radius)?,
                wall: slot_expr(py, d::SlotId::TubeWall, &wall)?,
            },
        })
    }

    /// A skinned solid through two or more section profiles.
    ///
    /// `first` is the first section's profile, and `then` each later
    /// section IN SKIN ORDER as `(profile, station)`: its profile and
    /// the `StationMove`s that carry the first section's plane to its
    /// own, slides along and spins about the first section's own axes,
    /// applied in the order written. Order is data, and reversing it
    /// reverses the produced surface's v-direction. The loft lies in
    /// its first section's coordinates. `v_degree` is the v-direction
    /// interpolation degree, a COUNT (structural material, D3), so it
    /// crosses as `Formula.count` and not as a continuous literal. It
    /// sets the faces too: at degree 1 through three sections or more
    /// the loft is one wall per profile segment per slab between two
    /// consecutive sections, each interior section a ring of edges; at 2
    /// or more one wall per segment spans every section.
    ///
    /// The degree is the node's `VDegree` slot —
    /// `DocEdit.bind_v_degree_param` is what makes it a named,
    /// editable number, `DocEdit.set_members` is what re-points the
    /// sections' profiles without re-authoring the node (one per
    /// section, else `loft_section_count`), and
    /// `DocEdit.set_loft_sections` writes the sections whole with their
    /// stations; a station's moves are its `station_slide` and
    /// `station_spin` slots.
    ///
    /// Nothing is pre-checked here. Too few sections, an empty station,
    /// a degree outside `1 ≤ d ≤ len − 1`, a section that is not a
    /// profile, sections whose loops do not correspond — every one of
    /// those is the kernel's own typed refusal, arriving from `insert`
    /// or from `evaluate` exactly where the Rust surface raises it.
    #[staticmethod]
    fn loft(
        py: Python<'_>,
        first: OperandArg,
        then: Vec<(OperandArg, Vec<StationMove>)>,
        v_degree: super::expr::SlotArg,
    ) -> PyResult<Self> {
        Ok(Self {
            inner: d::Node::Loft {
                first: first.read(),
                then: then
                    .into_iter()
                    .map(|(profile, station)| d::LoftSection {
                        profile: profile.read(),
                        station: station.into_iter().map(|m| m.0).collect(),
                    })
                    .collect(),
                v_degree: slot_expr(py, d::SlotId::VDegree, &v_degree)?,
            },
        })
    }

    /// Constant-radius rolling-ball blends on a SELECTION of
    /// `target`'s edges.
    ///
    /// `selection` is edge names as text: the strings
    /// `Evaluation.all_edges` answers with, or the ones a role-name
    /// door mints ([`super::select::band_rim`] and its five siblings)
    /// for a node no evaluation has reached yet. A name is CARRIED,
    /// never assembled and never read: the text is an opaque
    /// identifier whose internal structure is not API (see
    /// [`name_text`]), so a name is written by naming a ROLE and
    /// there is no supported way to filter a materialized set. There
    /// is deliberately no "every edge" spelling either.
    ///
    /// THE SELECTION FREEZES, exactly as in Rust: it is a commitment
    /// as of the evaluation you read it from, and an upstream edit
    /// that adds edges does NOT extend it. Materialize, store, and
    /// the recipe means what it said.
    ///
    /// Nothing is pre-checked beyond the text being a name at all. A
    /// name of the wrong kind refuses at the insert (`EditError`
    /// `selection_name_kind`), and so does an EMPTY selection
    /// (`selection_shape`, `name_count`): a set of edges holds at
    /// least one. A name that resolves to nothing (`select_resolve`)
    /// and a tangential edge the roller cannot enter (`fillet`) are
    /// the kernel's own typed refusals at `evaluate`.
    ///
    /// The node is built through Rust's `Node::fillet`, the one
    /// construction door, so the stored set is canonical (sorted,
    /// deduplicated) and two recipes that select the same edges are
    /// bit-identical whatever order Python listed them in.
    ///
    /// `radius` is the node's `radius` slot, moved afterwards by
    /// `DocEdit.set_param`; the SELECTION is repaired one name at a
    /// time by `DocEdit.rebind`.
    #[staticmethod]
    fn fillet(
        py: Python<'_>,
        target: OperandArg,
        radius: super::expr::SlotArg,
        selection: Vec<String>,
    ) -> PyResult<Self> {
        let radius = slot_expr(py, d::SlotId::Radius, &radius)?;
        let selection = selection
            .iter()
            .map(|text| name_from_text(text))
            .collect::<PyResult<Vec<_>>>()?;
        Ok(Self {
            inner: d::Node::fillet(target.read(), radius, selection),
        })
    }

    /// Equal-setback flat chamfers on a SELECTION of `target`'s
    /// edges — `Node.fillet`'s twin.
    ///
    /// `selection` is edge names as text, exactly as `Node.fillet`
    /// takes them: materialized or minted, CARRIED and never read,
    /// with no "every edge" spelling and no way to filter a
    /// materialized set. THE SELECTION FREEZES, in
    /// the same sense and for the same reason — read `Node.fillet`.
    ///
    /// `distance` is the SETBACK along each support from the edge,
    /// not a radius. That is the one thing this door does not share
    /// with its twin.
    ///
    /// Nothing is pre-checked beyond the text being a name at all. A
    /// name of the wrong kind refuses at the insert (`EditError`
    /// `selection_name_kind`), and so does an EMPTY selection
    /// (`selection_shape`, `name_count`): a set of edges holds at
    /// least one. A name that resolves to nothing (`select_resolve`)
    /// and an edge whose two supports are not both planes (`chamfer`)
    /// are the kernel's own typed refusals at `evaluate`.
    ///
    /// The node is built through Rust's `Node::chamfer`, the one
    /// construction door, so the stored set is canonical.
    ///
    /// `distance` is the node's `chamfer_distance` slot, moved
    /// afterwards by `DocEdit.set_param`; the selection is repaired
    /// by `DocEdit.rebind`, as a fillet's is.
    #[staticmethod]
    fn chamfer(
        py: Python<'_>,
        target: OperandArg,
        distance: super::expr::SlotArg,
        selection: Vec<String>,
    ) -> PyResult<Self> {
        let distance = slot_expr(py, d::SlotId::ChamferDistance, &distance)?;
        let selection = selection
            .iter()
            .map(|text| name_from_text(text))
            .collect::<PyResult<Vec<_>>>()?;
        Ok(Self {
            inner: d::Node::chamfer(target.read(), distance, selection),
        })
    }

    /// Hollow `target` into a thin solid of wall `thickness`, with the
    /// faces in `open` re-authored as annular RIMS.
    ///
    /// `open` is face names as TEXT — the strings
    /// `Evaluation.all_faces` or a selector answers with, or the ones
    /// [`super::select::band`] and its siblings mint — CARRIED and
    /// never read, and,
    /// unlike a blend's selection, IN THE ORDER GIVEN. The order is
    /// meaning: the kernel's record keeps a chart's designated faces
    /// in designation order, and the chart's rim is its FIRST
    /// designated face (a plane chart's members merge onto it, and the
    /// rim's name is that face's), so name first the face you want to
    /// carry the rim's identity. A curved chart that wraps round its
    /// axis — a dome's cap — keeps every face as a branch of a seamed
    /// band, each named for its own designation. A repeated name keeps its first
    /// occurrence. An EMPTY
    /// list is the SEALED hollow — every face offset inward, a cavity
    /// and no rim — which is legal and not a refusal.
    ///
    /// Every face of one solid on a chart must be named together:
    /// naming only some of the faces one solid has on one chart refuses
    /// (`shell`, the kernel's `OpenFaceChartPartial`).
    /// Another solid's faces on that chart are its own, and opening one
    /// solid's never names them. The
    /// designation FREEZES in the sense `Node.fillet` states.
    ///
    /// A name of the wrong kind refuses at the insert (`EditError`
    /// `selection_name_kind`). A name that resolves to nothing
    /// (`select_resolve`), a non-positive wall or a wall two facing
    /// faces cannot both afford, a rim the kernel cannot build or read
    /// (`shell`) — every one of those is the kernel's own typed refusal
    /// at `evaluate`.
    ///
    /// `thickness` is the node's `shell_thickness` slot, moved
    /// afterwards by `DocEdit.set_param`; a designated
    /// face that has come to denote the wrong face is repaired by
    /// `DocEdit.rebind`, which rewrites the list in place and
    /// re-canonicalizes it.
    #[staticmethod]
    fn shell(
        py: Python<'_>,
        target: OperandArg,
        thickness: super::expr::SlotArg,
        open: Vec<String>,
    ) -> PyResult<Self> {
        let thickness = slot_expr(py, d::SlotId::ShellThickness, &thickness)?;
        let open = open
            .iter()
            .map(|text| name_from_text(text))
            .collect::<PyResult<Vec<_>>>()?;
        Ok(Self {
            inner: d::Node::shell(target.read(), thickness, open),
        })
    }

    /// Split a target body by a tool: a plane, or a frame's coordinate
    /// plane read through its role (`RoleRead(frame, "xy")`).
    ///
    /// The value is a SPLIT, not a body: read it with `Value.split()`,
    /// which answers `(above, below)` with `None` for an empty side.
    /// A tool that is not a splitting surface, or a cut that produces
    /// nothing, refuses typed at `evaluate`.
    #[staticmethod]
    fn split(target: BodyArg, tool: super::pose::PoseArg) -> Self {
        Self {
            inner: d::Node::Split {
                target: target.read(),
                tool: tool.read(),
            },
        }
    }

    /// **A profile drawn on `frame`, tracing `faces`**: its chain (a
    /// `Chain`, or a list of them, outer first) reads the trace of each
    /// face, by its place in `faces`, in the frame's xy plane — a run
    /// onto a crossing (`TraceTarget.on_trace`), a heading along the
    /// trace (`Chain.along_trace`). `frame` is a frame pose of a body's
    /// space and each face a `(body, name)` of that space; the profile,
    /// and every sweep of it, is in that space. An argument whose
    /// dimension is not its slot's raises here, naming its loop, step
    /// and argument; a step whose trace refuses refuses at `evaluate`
    /// (`trace`), naming the step and the face.
    #[staticmethod]
    fn drawn_profile(
        py: Python<'_>,
        frame: OperandArg,
        faces: Vec<(OperandArg, String)>,
        outline: super::trace::Outline,
    ) -> PyResult<Self> {
        super::trace::drawn_profile(py, frame, faces, outline)
    }

    /// The mirror image of a body: `body` reflected across `plane` (a
    /// plane, or a frame's coordinate plane read through its role), a
    /// new body in `body`'s own space, its cells named as `body` names
    /// them. The plane reads that space alone, as a split's tool does.
    /// A symmetric part is `Node.union([body, mirror])` across one of
    /// `body`'s own faces.
    #[staticmethod]
    fn mirror(body: BodyArg, plane: super::pose::PoseArg) -> Self {
        Self {
            inner: d::Node::Mirror {
                body: body.read(),
                plane: plane.read(),
            },
        }
    }

    /// **One copy of `shapes` placed into `target` by `constraints`**:
    /// the operation whose output is a copy. Inserting it is
    /// `Doc.place`.
    ///
    /// `source` is one body argument: the copy has its kind (a body
    /// over a body). `target` is the space it places into
    /// (`Doc.place`'s). `constraints` is
    /// the bundle, written whole: a `Placed`, or `Mate`s and
    /// `MateValue`s. Every bundle pins its copy: the insert refuses one
    /// that leaves a freedom unwritten (`mate_unpinned`).
    #[staticmethod]
    fn place(
        py: Python<'_>,
        source: BodyArg,
        target: SpaceArg,
        constraints: super::mate::BundleArg,
    ) -> PyResult<Self> {
        Self::placed(
            py,
            source.read(),
            target.read(),
            super::mate::bundle(constraints),
        )
    }

    /// **A space of its own**, reading nothing: a target a placement
    /// places copies into beside the world's. Inserting it is
    /// `Doc.new_space`.
    #[staticmethod]
    fn space() -> Self {
        Self {
            inner: d::Node::Space,
        }
    }

    /// **The union**: the material in ANY member, folded into ONE
    /// body in the members' order (D9 — the fold order is the list's,
    /// and the list is data).
    ///
    /// `members` is a list of operands, each read on its own, or ONE
    /// operand standing for a whole family — a pattern's output — whose
    /// bodies are the members in index order. Any count is a union: one
    /// member is that body, none is the typed empty body, and a read
    /// listed twice is the same material twice, so `[a, a]` is `a`. A
    /// family read beside single reads in one list refuses at
    /// `Doc.insert` (`slot_var_kind`), as does a member id the document
    /// does not hold (`unresolved_input`). Whether a member is a BODY
    /// is the kernel's question at `evaluate`, as it is at every other
    /// operand seat.
    ///
    /// The membership is data, so `DocEdit.set_members` rewrites it on
    /// the live node.
    ///
    /// Members whose faces a margin decides on one surface glue there,
    /// and the coincidence is recorded for the `unproven_coincidence`
    /// check. A refusal only a fold step raises is
    /// `kind == "union_fold_step"`, naming the member it folds in.
    #[staticmethod]
    fn union(members: MembersArg) -> PyResult<Self> {
        Ok(Self {
            inner: d::Node::Union {
                members: members.bodies(),
            },
        })
    }

    /// **The assemble** (`⊔`): the members side by side as the pieces
    /// of one body, read as `Node.union` reads its members
    /// (`Doc.assemble`).
    #[staticmethod]
    fn assemble(members: MembersArg) -> PyResult<Self> {
        Ok(Self {
            inner: d::Node::Assemble {
                members: members.bodies(),
            },
        })
    }

    /// **The intersect**: the material in EVERY member, folded in the
    /// members' order.
    ///
    /// `members` is `Node.union`'s, read the same way:
    /// a list of operands or one family read, any count — one member
    /// is that body, none is the typed empty body, and `[a, a]` is
    /// `a`. A fold step whose members share no material is the typed
    /// empty body, not a refusal. `DocEdit.set_members` rewrites the
    /// list on the live node.
    #[staticmethod]
    fn intersect(members: MembersArg) -> PyResult<Self> {
        Ok(Self {
            inner: d::Node::Intersect {
                members: members.bodies(),
            },
        })
    }

    /// **The subtract**: `from_` with the material of `tool` cut away.
    ///
    /// The one pair node: difference neither commutes nor associates,
    /// so its two operands are named slots (`from`, spelled `from_`
    /// here because `from` is a Python keyword, and `tool`). Several
    /// tools are one subtract of their union,
    /// `Node.subtract(body, Node.union([t1, t2]))`. A body cut by
    /// itself, `Node.subtract(a, a)`, is the typed empty body.
    #[staticmethod]
    fn subtract(from_: BodyArg, tool: BodyArg) -> PyResult<Self> {
        Ok(Self {
            inner: d::Node::Subtract {
                from: from_.read(),
                tool: tool.read(),
            },
        })
    }

    /// **A row** (D10 `[repetition]`): `source` copied into `target`,
    /// member `k` slid `scalar(k)·pitch` along each `(axis, pitch)` of
    /// `steps` on the target's axes (`"x"`, `"y"`, `"z"`), with no mate
    /// — `pncad::pattern::linear_pattern`'s program, a `Node.place`
    /// like any other.
    ///
    /// `index` names an index the document declares
    /// (`DocEdit.declare_var(k, VarDecl.index(n))`), so the placement
    /// is a family of `n` members: `Node.union(row)` fuses them, and
    /// `row[i]` reads one. Each pitch is a length (`ValueError`
    /// otherwise, as `MateValue`'s); the insert refuses a name the
    /// document does not hold (`unlowered_name`) as any formula's.
    #[staticmethod]
    fn linear_pattern(
        py: Python<'_>,
        index: &VarName,
        source: BodyArg,
        target: SpaceArg,
        steps: Vec<(String, super::expr::SlotArg)>,
    ) -> PyResult<Self> {
        let steps = steps
            .iter()
            .map(|(axis, pitch)| pattern_step(py, axis, pitch))
            .collect::<PyResult<Vec<_>>>()?;
        pattern_node(pncad::pattern::linear_pattern(
            &index.0,
            source.read(),
            target.read(),
            &steps,
        ))
    }

    /// **A grid**: `Node.linear_pattern` over two indices, each
    /// `(index, axis, pitch)`, `inner`'s index declared within
    /// `outer`'s (`VarDecl.index(n, within=outer)`), member
    /// `(i, j)` slid along both — the family indexed `[outer, inner]`.
    #[staticmethod]
    fn grid(
        py: Python<'_>,
        outer: (VarName, String, super::expr::SlotArg),
        inner: (VarName, String, super::expr::SlotArg),
        source: BodyArg,
        target: SpaceArg,
    ) -> PyResult<Self> {
        let (outer_k, outer_axis, outer_pitch) = &outer;
        let (inner_k, inner_axis, inner_pitch) = &inner;
        pattern_node(pncad::pattern::grid(
            (&outer_k.0, pattern_step(py, outer_axis, outer_pitch)?),
            (&inner_k.0, pattern_step(py, inner_axis, inner_pitch)?),
            source.read(),
            target.read(),
        ))
    }

    /// **A ring**: `source` copied into `target` by one `Axis` mate
    /// equating `axis[0]` (an axis read off the copy) with `axis[1]`
    /// (an axis in the target), member `k` spun `scalar(k)·step` about
    /// it and slid zero along the target axis `axis[2]` it runs on.
    ///
    /// Exactly one of `count` and `step` says how far apart the
    /// members turn: `count` a full ring of that many (`turn / count`),
    /// `step` the angle as written. `doc` reads the mate's kinds, as
    /// `Placed.of`'s.
    #[staticmethod]
    #[pyo3(signature = (doc, index, source, target, axis, *, count=None, step=None))]
    #[allow(clippy::too_many_arguments)]
    fn circular_pattern(
        py: Python<'_>,
        doc: &Doc,
        index: &VarName,
        source: BodyArg,
        target: SpaceArg,
        axis: (OperandArg, OperandArg, String),
        count: Option<super::expr::SlotArg>,
        step: Option<super::expr::SlotArg>,
    ) -> PyResult<Self> {
        let step = match (count, step) {
            (Some(count), None) => pncad::pattern::RingStep::Full(super::pose::scalar_at(
                py,
                d::Dimension::Count,
                &count,
                "ring's count",
            )?),
            (None, Some(step)) => pncad::pattern::RingStep::By(super::pose::scalar_at(
                py,
                d::Dimension::Angle,
                &step,
                "ring's step",
            )?),
            _ => {
                return Err(pyo3::exceptions::PyTypeError::new_err(
                    "a ring takes exactly one of count= and step=",
                ));
            }
        };
        pattern_node(pncad::pattern::circular_pattern(
            &doc.inner,
            &index.0,
            source.read(),
            target.read(),
            ring_axis(axis)?,
            step,
        ))
    }

    /// **A bolt circle**: a full ring of `count` copies of `source`
    /// about `axis` (`Node.circular_pattern(..., count=count)`).
    #[staticmethod]
    fn bolt_circle(
        py: Python<'_>,
        doc: &Doc,
        index: &VarName,
        count: super::expr::SlotArg,
        source: BodyArg,
        target: SpaceArg,
        axis: (OperandArg, OperandArg, String),
    ) -> PyResult<Self> {
        let count = super::pose::scalar_at(py, d::Dimension::Count, &count, "ring's count")?;
        pattern_node(pncad::pattern::bolt_circle(
            &doc.inner,
            &index.0,
            count,
            source.read(),
            target.read(),
            ring_axis(axis)?,
        ))
    }

    /// An **instance of another document's product**: a leaf whose
    /// material crosses the document seam rather than arriving from an
    /// upstream node.
    ///
    /// `reference` is the `(id, pin)` pair — which part, at which
    /// version. Cargo.lock semantics: an edit to the referenced
    /// document never retargets this reference, and moving the pin is
    /// its own recorded edit (`DocEdit.update_reference`, or
    /// `update_references` for every site at once).
    ///
    /// **Placed into `target` by `constraints`.** An instance IS a
    /// placement: its part's world, placed into the space `target`
    /// (`Doc.place`'s) and moved by its bundle, written whole as
    /// `Doc.place`'s is. `None` stands it where the part's world stands,
    /// by the identity bundle (`Placed.identity()`), the "place part"
    /// gesture. A bundle that leaves a freedom unwritten, `[]` among
    /// them, refuses at the insert (`mate_unpinned`).
    ///
    /// The instance also carries no interface record: an AUTHORED
    /// instance crosses nothing, and a non-empty record is mintable
    /// only by the `split` that observed declarations crossing its
    /// cut. Read one back with `Doc.interface`.
    ///
    /// Evaluating this node needs a resolver — `evaluate(doc,
    /// resolver=workspace)`. Without one it refuses typed
    /// (`EvaluationError`, `kind == "part_no_resolver"`) rather than
    /// pretending the part is empty.
    #[staticmethod]
    #[pyo3(signature = (reference, target, constraints=None))]
    fn instantiate_part(
        py: Python<'_>,
        reference: &super::store::DocRef,
        target: SpaceArg,
        constraints: Option<super::mate::BundleArg>,
    ) -> PyResult<Self> {
        let constraints = constraints.map_or_else(|| d::stand().bundle(), super::mate::bundle);
        let inner = d::Node::instantiate_part(reference.0, target.read(), constraints);
        Self::check_slots(py, &inner)?;
        Ok(Self { inner })
    }

    /// **One measurement** (ERROR-DESIGN E3): a `Measure` node holds
    /// one closed-form primitive and defines one observed scalar, its
    /// output (`Doc.output(node)`), which only an assertion reads,
    /// directly or through a definition. Arithmetic over measured
    /// values is an ordinary `Formula` over the outputs.
    ///
    /// **The read site is the half that makes a measure report placed
    /// geometry.** A rigid transform is identity-preserving — the
    /// moved body keeps the upstream name — so resolving at the
    /// minting node measures the UNMOVED carrier. Selecting a face
    /// from a transform's own selection door and naming that transform
    /// gives the placed number; naming the minting node gives the
    /// authored one. Both are legal, and they are different questions.
    ///
    /// **The references ARE dag edges**: each is a selection of the named
    /// node's body (or, for a whole body, a read of it), so the
    /// referenced nodes are its data dependencies, and deleting one is
    /// accepted and reported on the measure (`stranded_read` and
    /// `stranded_selection`), like any other reader's.
    ///
    /// What each primitive reads is checked when the node is inserted:
    /// `distance` a face, an edge or a vertex, `angle` a face or an
    /// edge, `min_clearance` a body or a face, `gap` a face, an edge or
    /// a vertex, and any
    /// other refuses `slot_var_kind`. The rest are the kernel's own
    /// typed refusals at `evaluate`: a name that no longer resolves
    /// (`select_resolve`), a carrier pair with no v1 closed form
    /// (`measure_unsupported`) and a non-finite result
    /// (`measure_non_finite`).
    #[staticmethod]
    fn measure(primitive: &super::measure::MeasurePrimitive) -> Self {
        Self {
            inner: d::Node::measure(&primitive.inner),
        }
    }

    /// **A recorded tolerance requirement** (ERROR-DESIGN E10): design
    /// intent as document data, in the versioned recipe rather than in
    /// a script beside it.
    ///
    /// `value` is the scalar this bounds: a measure's output
    /// (`Doc.output(measure)`), `Doc.measure`'s `value`, or any formula
    /// or variable — a variable at its own kind's dimension, anything
    /// else at the bound's. A failed or poisoned
    /// measure under it poisons the assertion rather than producing a
    /// verdict about nothing. `relation` is how the value must relate to
    /// `bound` (`>=`, `<=` or `=`), and `bound` is a `Formula` from `Doc.parse_formula`.
    ///
    /// **The bound is an expression and not a quantity, because its
    /// DIMENSION is the value's.** It may be an angle, a count or a plain
    /// scalar as readily as a length. `Doc.parse_formula("0.5 mm")` is
    /// the one spelling, and it reaches document parameters
    /// (`"min_web"`) in the same call — which is what makes an assertion
    /// re-decidable by a parameter edit.
    ///
    /// That the value's dimension is the bound's is checked at
    /// `Doc.apply` (`assertion_dimension`), so a document never carries
    /// a comparison of radians with metres.
    ///
    /// **Report-only, structurally.** No op in the vocabulary accepts
    /// a verdict as an operand, the product gather skips an assertion
    /// as it skips a declaration, and nothing downstream changes shape
    /// because one is `Violated`. Read it with `Value.assertion`.
    #[staticmethod]
    fn assertion(
        py: Python<'_>,
        value: super::expr::SlotArg,
        relation: super::measure::AssertionRelation,
        bound: &super::expr::Formula,
    ) -> PyResult<Self> {
        Ok(Self {
            inner: d::Node::Assertion {
                // A variable is read at its own kind's dimension, so a
                // mismatch with the bound is the door's
                // `assertion_dimension`; anything else at the bound's.
                value: match &value {
                    super::expr::SlotArg::Var(Var(id, Some(kind))) => match kind.dimension() {
                        Some(dim) => d::Formula::var(*id, dim),
                        None => value.formula(py, bound.0.dim())?,
                    },
                    _ => value.formula(py, bound.0.dim())?,
                },
                bound: bound.0.clone(),
                relation: relation.to_kernel(),
            },
        })
    }
}

/// A document-level parameter name (guide §3.2): one identifier, the
/// same name the recipe's expressions reference. NOT an arena key:
/// recipe vocabulary, meaningful in any document.
#[pyclass(frozen, module = "pncad", from_py_object)]
#[derive(Clone)]
pub(crate) struct VarName(pub(crate) d::VarName);

#[pymethods]
impl VarName {
    /// Refuses typed (`EditError.variant == "param_name_not_an_identifier"`)
    /// a text no expression could read back as this parameter — blank,
    /// padded, not one identifier: the document layer's one rule for a
    /// name, asked at the boundary that turns text into one.
    #[new]
    fn new(py: Python<'_>, name: &str) -> PyResult<Self> {
        d::VarName::new(name).map(Self).map_err(|fault| {
            boundary_edit_err(py, BoundaryEdit::ParamName(&fault), fault.to_string())
        })
    }

    /// The name itself.
    #[getter]
    fn name(&self) -> String {
        self.0.as_str().to_owned()
    }

    fn __repr__(&self) -> String {
        format!("VarName({:?})", self.0.as_str())
    }

    fn __eq__(&self, other: &Self) -> bool {
        self.0 == other.0
    }

    fn __hash__(&self) -> u64 {
        use std::hash::{Hash, Hasher};
        let mut h = std::hash::DefaultHasher::new();
        self.0.hash(&mut h);
        h.finish()
    }
}

/// **A document variable's identity** (VARIABLES-DESIGN VR1): the id
/// the document minted it, which every expression reading it holds. A
/// rename moves the name and keeps this handle; a delete leaves it
/// naming nothing the document holds, and the id is never minted again.
///
/// An id is document-scoped: the same bits in another document name
/// another variable, or none. The handle carries the kind the document
/// held it at when it was read, `None` for one it no longer held: a
/// kind is fixed at minting, so it never goes stale.
#[pyclass(frozen, module = "pncad", from_py_object)]
#[derive(Clone, Copy)]
pub(crate) struct Var(pub(crate) d::VarId, pub(crate) Option<d::VarKind>);

impl Var {
    /// The handle of `id`, read in `doc`.
    pub(crate) fn of(doc: &d::ProfileDoc, id: d::VarId) -> Self {
        Self(id, doc.var(id).map(d::Var::kind))
    }
}

#[pymethods]
impl Var {
    /// **What the variable holds** (VR3, D10), fixed at minting: a
    /// scalar's dimension word (`"length"`, `"angle"`, `"scalar"`,
    /// `"count"`), a pose's (`"point"`, `"direction"`, `"axis"`,
    /// `"plane"`, `"frame"`) or a shape's (`"body"`, `"bodies"`,
    /// `"profile"`). `None` for a handle read where the document held
    /// no such variable.
    #[getter]
    fn kind(&self) -> Option<&'static str> {
        self.1.map(crate::errors::var_kind_tag)
    }

    /// The whole id: its mint ordinal, a colon, and its digest as sixteen
    /// lowercase hex digits — the key a saved file's variable table
    /// holds it under. (Named for when an id was its hex digest alone.)
    #[getter]
    fn hex(&self) -> String {
        self.0.full().to_string()
    }

    fn __repr__(&self) -> String {
        format!("Var({})", self.0.full())
    }

    /// One member of the family this variable holds, at `index`: one
    /// count per index of the family ([`IndexedRead`]).
    fn __getitem__(&self, py: Python<'_>, index: IndexArg) -> PyResult<IndexedRead> {
        IndexedRead::of(py, OperandArg::Var(*self), &index)
    }

    fn __eq__(&self, other: &Self) -> bool {
        self.0 == other.0
    }

    fn __hash__(&self) -> u64 {
        self.0.0.digest()
    }
}

/// **What `Doc.measure` inserted**: the measures, one per primitive in
/// order, and their outputs in the same order.
#[pyclass(frozen, module = "pncad", from_py_object)]
#[derive(Clone)]
pub(crate) struct Measured {
    measures: Vec<NodeId>,
    outputs: Vec<Var>,
}

#[pymethods]
impl Measured {
    /// The `Measure` nodes, one per primitive.
    #[getter]
    fn measures(&self) -> Vec<NodeId> {
        self.measures.clone()
    }

    /// Each measure's output, in the same order.
    #[getter]
    fn outputs(&self) -> Vec<Var> {
        self.outputs.clone()
    }

    fn __repr__(&self) -> String {
        format!("Measured({} measures)", self.measures.len())
    }
}

/// **What `Doc.sketch_on_face` inserted**: the profile, its extrude,
/// the extrude's copy placed on the face, and the union of the face's
/// body with the copy.
#[pyclass(frozen, module = "pncad", from_py_object)]
#[derive(Clone, Copy)]
pub(crate) struct SketchedOnFace(d::SketchedOnFace);

#[pymethods]
impl SketchedOnFace {
    /// The profile, drawn on its own plane.
    #[getter]
    fn profile(&self) -> NodeId {
        NodeId(self.0.profile)
    }

    /// The profile extruded up its own `z`.
    #[getter]
    fn extrude(&self) -> NodeId {
        NodeId(self.0.extrude)
    }

    /// The extrude's copy, placed on the face.
    #[getter]
    fn placement(&self) -> NodeId {
        NodeId(self.0.placement)
    }

    /// The face's body and the copy, fused.
    #[getter]
    fn union(&self) -> NodeId {
        NodeId(self.0.union)
    }

    fn __repr__(&self) -> String {
        format!(
            "SketchedOnFace(profile={}, extrude={}, placement={}, union={})",
            self.0.profile.full(),
            self.0.extrude.full(),
            self.0.placement.full(),
            self.0.union.full()
        )
    }
}

/// **What `Doc.eval` and `Doc.eval_count` evaluate**: a formula, or a
/// variable by its identity.
#[derive(FromPyObject)]
pub(crate) enum Evaluand {
    /// A formula.
    Formula(super::expr::Formula),
    /// A variable, read at the dimension it holds.
    Var(Var),
}

/// **A variable as an edit addresses it**: by its identity (`Var`), or
/// by the name the document holds for it (`VarName`).
#[derive(FromPyObject)]
pub(crate) enum VarArg {
    /// By identity.
    Id(Var),
    /// By name.
    Name(VarName),
}

impl VarArg {
    /// The kernel's address.
    pub(crate) fn var_ref(&self) -> d::VarRef {
        match self {
            Self::Id(var) => d::VarRef::Id(var.0),
            Self::Name(name) => d::VarRef::Name(name.0.clone()),
        }
    }
}

/// **A variable's definition as an edit carries it** — a free value, a
/// `Formula` over other variables that the edit door lowers (its names
/// resolved against the document's) and stores as the variable's
/// definition (VARIABLES-DESIGN VR3), or an index (D10 `[repetition]`).
#[pyclass(frozen, module = "pncad", from_py_object)]
#[derive(Clone)]
pub(crate) struct VarDecl(pub(crate) d::VarDecl);

#[pymethods]
impl VarDecl {
    /// A free variable holding `value`.
    #[staticmethod]
    fn free(value: &FreeVar) -> Self {
        Self(d::VarDecl::Free(value.0.clone()))
    }

    /// A variable defined by `expr`, of `expr`'s dimension: its value is
    /// `expr`'s, re-evaluated whenever a variable it reads moves, and it
    /// takes no value, unit or distribution of its own.
    #[staticmethod]
    fn defined(expr: &super::expr::Formula) -> Self {
        Self(d::VarDecl::defined(expr.0.clone()))
    }

    /// **An index** `k = index(count)` (D10 `[repetition]`): a `Count`
    /// variable taking each value `0 .. count`, so a placement whose
    /// values read it is evaluated once per value and its copy is a
    /// family of `count` members. `count` is a count (an `int`, a count
    /// `Formula`, or a variable read as one).
    ///
    /// `within` names another index this one is declared within, by
    /// its `Var` or its `VarName`: a reader of both is a family indexed
    /// by both, the outer first (a grid). Two indices neither of which
    /// is within the other meet in no reader, which refuses
    /// `indices_unrelated` where one node reads both.
    ///
    /// `declare_var` refuses a count of another dimension and a
    /// `within` that is no index of the document (`index`, with
    /// `inner_variant` `count_not_count` or `within_no_index`).
    #[staticmethod]
    #[pyo3(signature = (count, within=None))]
    fn index(
        py: Python<'_>,
        count: super::expr::SlotArg,
        within: Option<VarArg>,
    ) -> PyResult<Self> {
        Ok(Self(d::VarDecl::Index {
            count: count.formula(py, d::Dimension::Count)?,
            within: within.map(|w| match w {
                VarArg::Id(var) => d::Formula::var(var.0, d::Dimension::Count),
                VarArg::Name(name) => d::Formula::named(name.0, d::Dimension::Count),
            }),
        }))
    }

    /// The defining expression, or `None` for a free variable or an
    /// index.
    #[getter]
    fn expr(&self) -> Option<super::expr::Formula> {
        match &self.0 {
            d::VarDecl::Free(_) | d::VarDecl::Index { .. } => None,
            d::VarDecl::Defined(expr) => Some(super::expr::Formula(expr.clone())),
        }
    }

    /// The free value, or `None` for a defined variable or an index.
    #[getter]
    fn value(&self) -> Option<FreeVar> {
        match &self.0 {
            d::VarDecl::Free(free) => Some(FreeVar(free.clone())),
            d::VarDecl::Defined(_) | d::VarDecl::Index { .. } => None,
        }
    }

    /// An index's count, or `None` for any other declaration.
    #[getter]
    fn count(&self) -> Option<super::expr::Formula> {
        match &self.0 {
            d::VarDecl::Index { count, .. } => Some(super::expr::Formula(count.clone())),
            d::VarDecl::Free(_) | d::VarDecl::Defined(_) => None,
        }
    }

    /// The index an index is declared within, as the count read of it,
    /// or `None` for an index within none and any other declaration.
    #[getter]
    fn within(&self) -> Option<super::expr::Formula> {
        match &self.0 {
            d::VarDecl::Index { within, .. } => within.clone().map(super::expr::Formula),
            d::VarDecl::Free(_) | d::VarDecl::Defined(_) => None,
        }
    }

    fn __repr__(&self) -> String {
        match &self.0 {
            d::VarDecl::Free(free) => {
                format!("VarDecl.free({})", FreeVar(free.clone()).__repr__())
            }
            d::VarDecl::Defined(expr) => {
                format!("VarDecl.defined({})", d::unparse(expr, &|_| None))
            }
            d::VarDecl::Index { count, within } => {
                let count = d::unparse(count, &|_| None);
                match within {
                    None => format!("VarDecl.index({count})"),
                    Some(w) => format!(
                        "VarDecl.index({count}, within={})",
                        d::unparse(w, &|_| None)
                    ),
                }
            }
        }
    }
}

/// **A definition as a variable door takes it**: a free value, an
/// expression (a defined variable), or a [`VarDecl`] spelling either.
#[derive(FromPyObject)]
pub(crate) enum DeclArg {
    /// A free value.
    Free(FreeVar),
    /// A defining expression.
    Defined(super::expr::Formula),
    /// Either, spelled.
    Decl(VarDecl),
}

impl DeclArg {
    /// The kernel's definition.
    fn decl(&self) -> d::VarDecl {
        match self {
            Self::Free(value) => d::VarDecl::Free(value.0.clone()),
            Self::Defined(expr) => d::VarDecl::defined(expr.0.clone()),
            Self::Decl(decl) => decl.0.clone(),
        }
    }
}

/// `-0.0` folded to `0.0`, every other value untouched — the
/// normalization a hash must apply wherever the equality it mirrors is
/// IEEE (`-0.0 == 0.0`).
pub(crate) fn fold_zero(v: f64) -> f64 {
    if v == 0.0 { 0.0 } else { v }
}

/// The shared body of the three continuous `FreeVar` constructors:
/// declare the dimension, and hang an annotation on it if one was
/// offered.
///
/// The dimension check is the binding's, and it has to be: a
/// `Distribution` is dimension-free in the kernel, so there is no
/// kernel refusal to match here — E2 puts the dimension on the
/// parameter and nowhere else, and this door is the seam where the
/// Python-side offsets meet the parameter that owns their dimension.
/// `door` names the constructor in the raise, so a caller reads which
/// declaration disagreed with which annotation.
fn continuous(
    py: Python<'_>,
    door: &'static str,
    dim: d::Dimension,
    value: f64,
    distribution: Option<&super::analysis::Distribution>,
) -> PyResult<FreeVar> {
    let Some(dist) = distribution else {
        return Ok(FreeVar(d::FreeVar::continuous(dim, value)));
    };
    if dist.dim != dim {
        return Err(super::analysis::dimension_mismatch(py, door, dim, dist.dim));
    }
    Ok(FreeVar(d::FreeVar::continuous_with(dim, value, dist.inner)))
}

/// A named parameter's declared dimension and exact stored value
/// (guide §3.2): what `DocEdit.declare_var` writes.
///
/// Continuous values arrive as typed quantities, so the
/// dimension is carried by the constructor rather than guessed from a
/// bare float. A non-finite value is NOT pre-checked here — the edit
/// door refuses it typed (`non_finite_var`), fail-loud where
/// the kernel refuses.
///
/// The three continuous constructors take an OPTIONAL `distribution`
/// (ERROR-DESIGN E1/E2), because annotation is opt-in and a parameter
/// with none is FIXED under any analysis. The annotation's offsets
/// must be in the dimension the constructor declares — a `Length`
/// parameter takes a `Length` distribution — and a mismatch is a
/// `DimensionError` here rather than a plausible number later. `count`
/// takes none and cannot: a structural count is fixed under any error
/// analysis, and the kernel has no `Count` case to drop an annotation
/// on.
///
/// Equality, hashing and `repr` all see the annotation, and
/// [`Self::distribution`] reads it back.
#[pyclass(frozen, module = "pncad", from_py_object)]
#[derive(Clone)]
pub(crate) struct FreeVar(pub(crate) d::FreeVar);

#[pymethods]
impl FreeVar {
    /// A continuous Length parameter, with an optional Length
    /// distribution.
    #[staticmethod]
    #[pyo3(signature = (value, distribution = None))]
    fn length(
        py: Python<'_>,
        value: &super::quantity::Length,
        distribution: Option<&super::analysis::Distribution>,
    ) -> PyResult<Self> {
        continuous(
            py,
            "FreeVar.length",
            d::Dimension::Length,
            value.0.meters(),
            distribution,
        )
    }

    /// A continuous Angle parameter, with an optional Angle
    /// distribution.
    #[staticmethod]
    #[pyo3(signature = (value, distribution = None))]
    fn angle(
        py: Python<'_>,
        value: &super::quantity::Angle,
        distribution: Option<&super::analysis::Distribution>,
    ) -> PyResult<Self> {
        continuous(
            py,
            "FreeVar.angle",
            d::Dimension::Angle,
            value.0.radians(),
            distribution,
        )
    }

    /// A continuous Length parameter that REMEMBERS the notation it
    /// was authored in — `25 mm` stays `mm` in the document and in
    /// the file, where `length` records the canonical metre row.
    ///
    /// Total: a `WrittenLength` holds a length unit, so there is no
    /// dimension for the notation to disagree with and no refusal
    /// here. The mismatch the save/load validator watches for
    /// (`PersistError` `display_unit`) is unreachable through this
    /// door, which is the reason to author through it.
    ///
    /// No `distribution`: the kernel's own notation doors carry none
    /// (`FreeVar::written_length` writes `distribution: None`), and
    /// this binding does not reach past them to build the payload by
    /// hand. A parameter that wants both is declared here and then
    /// annotated through `DocEdit.set_var_distribution`,
    /// which carries the notation forward; [`Self::length`] takes
    /// both at once and records the canonical metre row.
    #[staticmethod]
    fn written_length(value: &super::quantity::WrittenLength) -> Self {
        Self(d::FreeVar::written_length(value.0))
    }

    /// A continuous Angle parameter that remembers its notation —
    /// [`Self::written_length`]'s mirror, total for its reason and
    /// carrying no annotation for its reason.
    #[staticmethod]
    fn written_angle(value: &super::quantity::WrittenAngle) -> Self {
        Self(d::FreeVar::written_angle(value.0))
    }

    /// A continuous dimensionless parameter, with an optional
    /// dimensionless distribution.
    #[staticmethod]
    #[pyo3(signature = (value, distribution = None))]
    fn scalar(
        py: Python<'_>,
        value: f64,
        distribution: Option<&super::analysis::Distribution>,
    ) -> PyResult<Self> {
        continuous(
            py,
            "FreeVar.scalar",
            d::Dimension::Scalar,
            value,
            distribution,
        )
    }

    /// An integer Count parameter (structural material, spec D3).
    ///
    /// No distribution, and there is no spelling for one: a
    /// structural count is fixed under any error analysis (E2), and
    /// the kernel's `Count` arm carries no annotation field to hang
    /// one on.
    #[staticmethod]
    fn count(value: i64) -> Self {
        Self(d::FreeVar::Count { value })
    }

    /// This parameter's distribution, or `None` if it declared none.
    ///
    /// The kernel's `FreeVar::distribution` reader, carrying the
    /// parameter's own dimension across with it — which is where an
    /// annotation's dimension lives, since the annotation itself has
    /// none (E2: no separate dimension field to disagree with the
    /// nominal).
    #[getter]
    fn distribution(&self) -> Option<super::analysis::Distribution> {
        self.0
            .distribution()
            .map(|dist| super::analysis::Distribution::borrowed(*dist, self.0.dim()))
    }

    /// The dimension this parameter declares.
    #[getter]
    fn dimension(&self) -> &'static str {
        crate::errors::dimension_tag(self.0.dim())
    }

    /// The notation this parameter was authored in, as the unit's own
    /// SYMBOL — `"mm"`, `"deg"`, `"m"`.
    ///
    /// A symbol rather than a unit object because the display unit a
    /// parameter carries is a one-byte code into the table
    /// (`UnitSym`), and a notation reaches Python as its symbol; a
    /// `Scalar` parameter names the dimensionless row, whose symbol is
    /// empty and reads as the absence it is. `None` is the different
    /// answer: a `Count` carries no notation at all, because a count
    /// is an integer and has none to carry.
    #[getter]
    fn unit(&self) -> Option<&'static str> {
        match &self.0 {
            d::FreeVar::Continuous { display_unit, .. } => Some(display_unit.def().symbol()),
            d::FreeVar::Count { .. } => None,
        }
    }

    /// Rust's `PartialEq`, mirrored — which is IEEE comparison of the
    /// stored value, NOT the bit comparison `FreeVar::bit_eq` makes.
    /// Two spellings of zero are therefore the same parameter here
    /// and different parameters to `bit_eq`, exactly as in Rust; a
    /// NaN value (which the edit door refuses, so it never reaches a
    /// document) equals nothing, itself included.
    ///
    /// Dimension is part of the value: a Length 1 and a Scalar 1 are
    /// different parameters.
    fn __eq__(&self, other: &Self) -> bool {
        self.0 == other.0
    }

    /// Consistent with [`Self::__eq__`]: `-0.0` is folded to `0.0`
    /// before hashing, because the equality that fold mirrors says
    /// they are the same parameter, and a hash that split them would
    /// break the invariant Python dicts rely on. NaN hashes like any
    /// other bit pattern and simply never compares equal.
    fn __hash__(&self) -> u64 {
        use std::hash::{Hash, Hasher};
        let mut h = std::hash::DefaultHasher::new();
        match &self.0 {
            d::FreeVar::Continuous {
                dim,
                value,
                display_unit,
                distribution,
            } => {
                0u8.hash(&mut h);
                format!("{dim:?}").hash(&mut h);
                fold_zero(*value).to_bits().hash(&mut h);
                // The authored notation is part of the DERIVED
                // `PartialEq` this hash mirrors, so it is part of the
                // hash: two parameters that `__eq__` calls different
                // may collide, but two it calls equal may never hash
                // apart, and leaving the unit out is the direction that
                // costs nothing to close. (`FreeVar::bit_eq` excludes
                // it — that comparator is D7 replay identity, where a
                // notation is not part of what a document IS.)
                display_unit.def().symbol().hash(&mut h);
                // The distribution is part of the parameter, so it is
                // part of the equality this hash mirrors — and it gets
                // the SAME `-0.0` fold the nominal gets, per field,
                // before its debug spelling is hashed. Without the
                // fold, `Band { lo: -0.0, .. }` and `Band { lo: 0.0,
                // .. }` compare EQUAL (Rust's `PartialEq` on `f64` is
                // IEEE) and hash APART, which is the one thing a
                // Python dict may not survive.
                format!("{:?}", distribution.map(d::Distribution::fold_signed_zeros)).hash(&mut h);
            }
            d::FreeVar::Count { value } => {
                1u8.hash(&mut h);
                value.hash(&mut h);
            }
        }
        h.finish()
    }

    fn __repr__(&self) -> String {
        match &self.0 {
            // The unit is shown because `__eq__` distinguishes it: a
            // repr that hid a field two values can differ on would
            // print them identically. The dimensionless row's symbol is
            // empty, which reads as the absence it is.
            d::FreeVar::Continuous {
                dim,
                value,
                display_unit,
                distribution: None,
            } => format!("FreeVar({dim:?} {value} {})", display_unit.def().symbol()),
            d::FreeVar::Continuous {
                dim,
                value,
                display_unit,
                distribution: Some(d),
            } => format!(
                "FreeVar({dim:?} {value} {} {d:?})",
                display_unit.def().symbol()
            ),
            d::FreeVar::Count { value } => format!("FreeVar(Count {value})"),
        }
    }
}

/// The VALUE half of a document parameter: what
/// `DocEdit.set_var_value` writes into an ALREADY-DECLARED
/// parameter.
///
/// This is the safe "just change the number" spelling. `define_var`
/// replaces a whole definition and takes a whole `FreeVar`, so using it to
/// move a value rebuilds the declaration from parts — and a parameter
/// read back from a file carrying a distribution (ERROR-DESIGN E1/E2)
/// loses it, silently, because Python has no way to spell the
/// annotation it would need to copy across. The value door carries the
/// declaration forward instead: the dimension and the annotation stay
/// exactly as the document has them.
///
/// Continuous values arrive as typed quantities for the same reason
/// `FreeVar`'s do — except that here the dimension is NOT being
/// declared, it is being matched: the quantity says which unit the
/// number is in, and the parameter's own declaration is what rules.
#[pyclass(frozen, module = "pncad", from_py_object)]
#[derive(Clone)]
pub(crate) struct FreeValue(pub(crate) d::FreeValue);

#[pymethods]
impl FreeValue {
    /// A continuous value in Length units.
    #[staticmethod]
    fn length(value: &super::quantity::Length) -> Self {
        Self(d::FreeValue::Continuous(value.0.meters()))
    }

    /// A continuous value in Angle units.
    #[staticmethod]
    fn angle(value: &super::quantity::Angle) -> Self {
        Self(d::FreeValue::Continuous(value.0.radians()))
    }

    /// A dimensionless continuous value.
    #[staticmethod]
    fn scalar(value: f64) -> Self {
        Self(d::FreeValue::Continuous(value))
    }

    /// An exact integer, for a `Count` parameter.
    #[staticmethod]
    fn count(value: i64) -> Self {
        Self(d::FreeValue::Count(value))
    }

    /// Rust's `PartialEq`, mirrored — IEEE on the stored number, so
    /// the two spellings of zero are the same value.
    fn __eq__(&self, other: &Self) -> bool {
        self.0 == other.0
    }

    fn __repr__(&self) -> String {
        format!("FreeValue({})", self.0)
    }
}

/// A single edit to a document — the ONE API surface shared by the
/// GUI, the bindings, macro recording and headless tests.
///
/// The exposed edits are `insert_node`, `delete_node`,
/// `set_members`, `set_param`, `set_tolerance`, the
/// variable doors (`declare_var` / `define_var` / `set_var_value`),
/// `add_constraint`, `remove_constraint`, `update_reference`, `rebind`, and
/// `bind_v_degree_param`, the structural-slot edit narrowed to one
/// named slot and a parameter reference.
///
/// **What is NOT here, and why each one is not.** The appearance and
/// metadata four are the FAÇADE's answer rather than this module's:
/// `Attr`, `AttrSet`, the record types and `MetaValue` are off
/// `pncad::document`'s curated list, so those arms have no payload a
/// consumer of that module can name in either language. The two
/// witness edits are the same sentence at `WitnessDatum` and
/// `BranchCertification`, which that list does not carry either. The
/// expression-path edit is a different reason and a repairable one:
/// its own refusal at a bad address renders the address through
/// `Debug`, which is what [`crate::py::typed_err`]'s prose gate
/// panics on, so the door would raise a binding panic exactly where
/// it is supposed to refuse.
#[pyclass(frozen, module = "pncad", from_py_object)]
#[derive(Clone)]
pub(crate) struct DocEdit {
    pub(crate) inner: d::DocEdit<d::ProfileProgram>,
}

/// The notations `DocEdit.set_var_unit` accepts: the two typed
/// unit objects a caller already writes quantities with.
///
/// A typed unit rather than a symbol STRING, for
/// `FreeVar.written_length`'s reason: a `LengthUnit` is an index into
/// a Length row of the table, so an off-table notation cannot be spelled
/// at all and the boundary extraction is the check. What remains for
/// the kernel to refuse is the pairing — `mm` on an angle — which is a
/// fact about the parameter rather than about the argument.
///
/// No `Scalar` arm: the dimensionless row is the only notation a
/// scalar parameter has, so a door to write it would be a door to
/// write what is already there.
#[derive(FromPyObject, Clone, Copy)]
enum DisplayUnitSpec {
    Length(super::quantity::LengthUnit),
    Angle(super::quantity::AngleUnit),
}

impl DisplayUnitSpec {
    /// The table code the kernel edit carries. Total both ways: every
    /// arm holds a table row (`UnitSym::from_def`).
    fn sym(self) -> d::UnitSym {
        match self {
            Self::Length(u) => d::UnitSym::from_def(&u.0.def()),
            Self::Angle(u) => d::UnitSym::from_def(&u.0.def()),
        }
    }
}

#[pymethods]
impl DocEdit {
    /// Insert a node.
    #[staticmethod]
    fn insert_node(node: &Node) -> Self {
        Self {
            inner: d::DocEdit::InsertNode {
                node: Box::new(node.inner.clone()),
                fresh: Vec::new(),
            },
        }
    }

    /// Delete a node. Its label, if any, goes with it.
    #[staticmethod]
    fn delete_node(id: &NodeId) -> Self {
        Self {
            inner: d::DocEdit::DeleteNode { id: id.0 },
        }
    }

    /// **Set or clear a node's label**: `label` replaces the label the
    /// node has, `None` clears it. A label is document data beside the
    /// node, so the edit recomputes nothing; it does move the content
    /// pin, as a recolour does.
    ///
    /// Refuses at this call a text that is not a label (`EditError`:
    /// `label_blank`, `label_line_break`, `label_control_character`,
    /// `label_direction_control`), and at `apply` a node the document
    /// does not hold (`unknown_node`) or an edit that would leave the
    /// label as it is (`label_unchanged`).
    #[staticmethod]
    #[pyo3(signature = (node, label))]
    fn set_label(py: Python<'_>, node: &NodeId, label: Option<&str>) -> PyResult<Self> {
        let label = label.map(|text| label_from_text(py, text)).transpose()?;
        Ok(Self {
            inner: d::DocEdit::SetLabel {
                node: node.0,
                label,
            },
        })
    }

    /// **Replace a node's whole LIST input** — a `Node.union`'s or a
    /// `Node.intersect`'s members, a `Node.loft`'s sections — with the
    /// list stated in full. `members` takes `Node.union`'s shape: a
    /// sequence of operands, or one operand reading a family whole.
    ///
    /// The one edit that changes a live node's inputs, and it can be
    /// that because it is unambiguous by construction: there is no
    /// positional spelling and no per-entry arm, so nothing is
    /// inferred about which of the old entries survived or moved.
    /// Dropping one member is this edit without it plus a
    /// `DocEdit.delete_node` of the orphan, one committed action.
    ///
    /// Every check `Doc.insert` makes of a node's inputs is remade
    /// here, of the REWRITTEN node, so this edit cannot reach a state
    /// an insert would have refused: `unresolved_input` for a member
    /// the document does not hold, `slot_var_kind` for a family read
    /// beside single reads, and `would_cycle` for a member downstream
    /// of the node itself — the one refusal an insert gets for free
    /// and this edit does not. Any count is a list: one member, or
    /// none, is a union or intersect still. A loft's sections are
    /// spelled one read each, so a family there refuses
    /// `loft_sections_spelled`, and are re-pointed in place, keeping
    /// each station, so a different count refuses `loft_section_count`
    /// (`DocEdit.set_loft_sections` adds or drops one); a node
    /// carrying no list at all refuses `set_members_on_non_list`.
    #[staticmethod]
    fn set_members(node: &NodeId, members: MembersArg) -> Self {
        Self {
            inner: d::DocEdit::SetMembers {
                node: node.0,
                members: members.bodies(),
            },
        }
    }

    /// **Write a `Node.loft`'s sections whole, each with its
    /// station**: `first`'s profile, then each later section as
    /// `(profile, station)`, `Node.loft`'s own shape. The edit that
    /// adds, drops or reorders a loft's sections, since each after the
    /// first stands where its station puts it. The degree is kept.
    ///
    /// It admits a loft exactly where `Doc.insert` would, through the
    /// same kernel checks: `slot_var_kind` for a section that is not a
    /// profile, `slot_dimension_mismatch` for a station move of the
    /// wrong dimension, `loft_shape` for too few sections, an empty
    /// station or a degree outside `1 ≤ d ≤ len − 1`. A node that is
    /// not a loft refuses `set_loft_sections_on_non_loft`.
    #[staticmethod]
    fn set_loft_sections(
        node: &NodeId,
        first: OperandArg,
        then: Vec<(OperandArg, Vec<StationMove>)>,
    ) -> Self {
        Self {
            inner: d::DocEdit::SetLoftSections {
                node: node.0,
                first: first.read(),
                then: then
                    .into_iter()
                    .map(|(profile, station)| d::LoftSection {
                        profile: profile.read(),
                        station: station.into_iter().map(|m| m.0).collect(),
                    })
                    .collect(),
                fresh: Vec::new(),
            },
        }
    }

    /// **Replace a CONTINUOUS slot's expression on a live node** — an
    /// extrude's `distance`, a fillet's `radius`, a revolve's
    /// `revolve_angle` — after the constructor that minted it.
    ///
    /// The constructors take numbers, so a node arrives with its
    /// slots holding literals. This is the door that moves one
    /// afterwards, and the door that puts an EXPRESSION there: hand
    /// it `Doc.parse_formula("plate_t * 2")` and the slot is driven by a
    /// document parameter from then on, exactly as
    /// `bind_v_degree_param` drives a structural one.
    ///
    /// **The slot is named by its WORD** — the same word
    /// `EditError.slot` answers in — so a refusal is an address a
    /// caller can retry at without translating anything. It is a
    /// NAME, never an index (spec D5), and a word outside the
    /// alphabet is a `ValueError` at the boundary rather than an edit
    /// the kernel gets to see.
    ///
    /// Structural slots are NOT this door's: `count`, `instance`,
    /// `v_degree` and `stations` are Count-typed, and aiming here at
    /// one refuses `structural_slot_needs_structural_edit` rather
    /// than quietly crossing the divide the edit vocabulary keeps
    /// unlosable. The `bind_*_param` trio is where they are edited.
    ///
    /// **An operand is a slot too** (D10, one door): at an operand's
    /// word — an extrude's `profile`, a boolean's `a`, a transform's
    /// `input` — the value is a read, a `NodeId` (its output in that
    /// seat) or a `Var`, and the node reads it from then on. At a
    /// scalar slot a node is read through its output, as a `Var` is.
    ///
    /// Refuses typed on `EditError`: `unknown_node`, `unknown_slot`
    /// for a slot this node does not carry (naming the slot it
    /// lacks), `slot_dimension_mismatch` for an expression of the
    /// wrong dimension, or any expression at an operand (carrying the
    /// required and offered pair), and `slot_unknown_var_name` /
    /// `slot_var_kind` for a read the slot does not take; at an operand
    /// also `operand_unresolved`, `ambiguous_output`,
    /// `defines_nothing` and `would_cycle`.
    #[staticmethod]
    fn set_param(py: Python<'_>, node: &NodeId, slot: &str, value: SlotValueArg) -> PyResult<Self> {
        let slot = slot_from_text(slot)?;
        let value = match (value, slot.dimension()) {
            (SlotValueArg::Node(node), _) => d::SlotValue::Read(d::Operand::Node(node.0)),
            (SlotValueArg::Slot(super::expr::SlotArg::Var(var)), None) => {
                d::SlotValue::Read(d::Operand::Var(var.0))
            }
            (SlotValueArg::Slot(arg), Some(dim)) => d::SlotValue::Formula(arg.formula(py, dim)?),
            // An expression at an operand is the kernel's refusal to
            // make, with the dimension it was offered: written at its
            // own dimension, a variable's read at its kind's.
            (SlotValueArg::Slot(arg), None) => {
                d::SlotValue::Formula(arg.formula(py, arg.own_dimension())?)
            }
        };
        Ok(Self {
            inner: d::DocEdit::SetParam {
                node: node.0,
                slot,
                value,
                fresh: Vec::new(),
            },
        })
    }

    /// Set the document tolerance.
    #[staticmethod]
    fn set_tolerance(eps: f64) -> Self {
        Self {
            inner: d::DocEdit::SetTolerance { eps },
        }
    }

    /// **Declare a variable** (guide §3.2): mint its id and hold `name`
    /// beside it. The edit applies cleanly even for a value the
    /// geometry will refuse — a program that refuses under the current
    /// binding is legal AT REST; the refusal belongs to replay.
    ///
    /// Refuses typed on a name the document already holds
    /// (`var_name_taken`), and on a broken annotation:
    /// `invalid_distribution` for an E2 invariant,
    /// `non_finite_var` for a NaN or infinite nominal or offset.
    /// Neither annotation fault is reachable through the
    /// `Distribution` constructors, which run the same check at the
    /// value; both are reachable through a file.
    ///
    /// A `Formula` (or `VarDecl.defined`) declares a DEFINED variable,
    /// whose value is the expression's over the variables it reads.
    /// It refuses `definition_unknown_var_name`,
    /// `definition_unresolved_var` and `definition_var_kind` for a read
    /// the document does not answer, `definition_cycle` for one that
    /// reads the variable back, and `definition_too_large` for an
    /// expansion past the bound.
    #[staticmethod]
    fn declare_var(name: &VarName, value: DeclArg) -> Self {
        Self {
            inner: d::DocEdit::DeclareVar {
                name: name.0.clone(),
                def: value.decl(),
            },
        }
    }

    /// **Replace a variable's definition**, keeping its identity, its
    /// name and its kind.
    ///
    /// **The whole definition is replaced.** The `FreeVar` handed over
    /// is what the variable ends up with, so one rebuilt from a
    /// dimension and a number has no distribution and the annotation
    /// the old one carried is gone. `set_var_value` is the door for
    /// moving a number, because it cannot drop what it never takes.
    ///
    /// A `Formula` (or `VarDecl.defined`) makes the variable a DEFINED
    /// one, keeping its identity; a `FreeVar` makes it free again.
    ///
    /// Refuses typed on a name the document does not hold
    /// (`unknown_var`), on a definition of another kind
    /// (`var_kind_fixed` — a kind is fixed when a variable is
    /// declared), and on `declare_var`'s annotation and definition
    /// faults.
    #[staticmethod]
    fn define_var(var: VarArg, value: DeclArg) -> Self {
        Self {
            inner: d::DocEdit::DefineVar {
                var: var.var_ref(),
                def: value.decl(),
                fresh: Vec::new(),
            },
        }
    }

    /// Write a new VALUE into an already-declared document parameter,
    /// keeping its declaration — its dimension and, if a file gave it
    /// one, its distribution (ERROR-DESIGN E1/E2).
    ///
    /// **Prefer this over `define_var` to move a number.** `define_var`
    /// replaces the whole definition: handed a `FreeVar` rebuilt from
    /// a dimension and a number — the natural spelling of a value
    /// change — any annotation the variable carried is gone with no
    /// refusal and no diagnostic. This door cannot do that, because it
    /// never names a definition at all.
    ///
    /// Refuses typed on a name the document does not declare
    /// (`unknown_var`) and on a kind mismatch — a count for
    /// a continuous parameter or the reverse
    /// (`var_value_kind_mismatch`) — a kind is fixed when a
    /// variable is declared.
    #[staticmethod]
    fn set_var_value(var: VarArg, value: &FreeValue) -> Self {
        Self {
            inner: d::DocEdit::SetVarValue {
                var: var.var_ref(),
                value: value.0,
            },
        }
    }

    /// Write a new NOTATION onto an already-declared document
    /// parameter, keeping its declaration — its dimension, its exact
    /// value and, if it has one, its distribution.
    ///
    /// `set_var_value`'s mirror over the other field of the same
    /// definition, and preferable over `define_var` for the same
    /// reason: a definition replaced whole makes the caller restate it
    /// to re-spell one unit, and whatever they leave out — the
    /// annotation, every time — is deleted with no refusal.
    ///
    /// A notation change is NOT a redeclaration — `FreeVar.bit_eq`
    /// already excludes the display unit as presentation metadata.
    ///
    /// The unit is a `LengthUnit` or an `AngleUnit` — the same objects
    /// `25 * mm` is written with — so an off-table notation is a
    /// `TypeError` at the boundary rather than a refusal from the
    /// kernel. `Scalar` parameters take no argument here: the
    /// dimensionless row is the only notation they have.
    ///
    /// Refuses typed on a name the document does not declare
    /// (`unknown_var`), on a `Count` parameter
    /// (`var_count_has_no_unit` — a count is an integer, not a
    /// quantity) and on a unit that does not measure the declared
    /// dimension (`var_unit_mismatch`).
    #[staticmethod]
    fn set_var_unit(var: VarArg, unit: DisplayUnitSpec) -> Self {
        Self {
            inner: d::DocEdit::SetVarUnit {
                var: var.var_ref(),
                unit: unit.sym(),
            },
        }
    }

    /// Write an E1/E2 ANNOTATION onto an already-declared document
    /// parameter, keeping its declaration — its dimension, its exact
    /// value and the notation it was authored in.
    ///
    /// The third of the carry-forward doors, one per field of the
    /// definition, and preferable over `define_var` for its siblings'
    /// reason: the authoring spelling for an annotated parameter writes
    /// the CANONICAL notation, so annotating through a whole definition
    /// re-spells a variable authored in millimetres, with no refusal
    /// and no diagnostic.
    ///
    /// **`None` CLEARS the annotation**, through this same door: the
    /// field is optional and "no annotation" is a value of the
    /// declaration, not a row removed from a map.
    ///
    /// Refuses typed on a name the document does not declare
    /// (`unknown_var`), on a `Count` parameter
    /// (`var_count_has_no_distribution` — a count takes no
    /// annotation, for the reason `FreeVar.count` gives) and on a
    /// distribution that breaks an E2 invariant
    /// (`non_finite_var`, `invalid_distribution`).
    ///
    /// The `Distribution`'s own dimension is NOT checked against the
    /// parameter's here: a kernel `Distribution` is dimension-free
    /// offsets, so this wrapper's `dim` is dropped building the
    /// payload and nothing survives for `apply` to compare. That is
    /// `set_var_value`'s position too — it takes a typed
    /// quantity and carries only the number — and the difference from
    /// the `FreeVar` constructors, which hold the declaration and its
    /// annotation at once and do check. Whether the binding should
    /// instead carry the dropped dimension and refuse at `apply` is
    /// LIB's `doc-param-edit-doors-drop-the-python-dimension`.
    #[staticmethod]
    #[pyo3(signature = (var, distribution))]
    fn set_var_distribution(
        var: VarArg,
        distribution: Option<&super::analysis::Distribution>,
    ) -> Self {
        Self {
            inner: d::DocEdit::SetVarDistribution {
                var: var.var_ref(),
                distribution: distribution.map(|d| d.inner),
            },
        }
    }

    /// **Name, rename or unname a variable** (VR2): writes the name and
    /// nothing else, so nothing recomputes and a `Var` handle keeps
    /// naming the same variable. `None` clears the name.
    ///
    /// Refuses typed on a variable the document does not hold
    /// (`unknown_var`), a name another variable holds
    /// (`var_name_taken`), the name the variable already has
    /// (`var_name_unchanged`), and clearing the name of a variable
    /// nothing reads (`anonymous_var_unread`) or more than one reader
    /// reads (`shared_var_needs_name`: an unnamed variable has one).
    #[staticmethod]
    #[pyo3(signature = (var, name))]
    fn rename_var(var: VarArg, name: Option<&VarName>) -> Self {
        Self {
            inner: d::DocEdit::RenameVar {
                var: var.var_ref(),
                name: name.map(|n| n.0.clone()),
            },
        }
    }

    /// **Delete a named variable** (VR7). Its readers stay, unresolved:
    /// evaluation refuses at each (`unresolved_var`), and the id is
    /// never minted again, so a later declare of the same name is a new
    /// variable the old readers do not read.
    ///
    /// Refuses typed on a variable the document does not hold
    /// (`unknown_var`) and on an anonymous one, whose lifecycle is its
    /// readers' (`delete_anonymous_var`).
    #[staticmethod]
    fn delete_var(var: VarArg) -> Self {
        Self {
            inner: d::DocEdit::DeleteVar { var: var.var_ref() },
        }
    }

    /// Bind `node`'s STRUCTURAL v-degree slot to the document
    /// parameter `name` — the edit that makes a loft's v-direction
    /// interpolation degree a named, editable number.
    ///
    /// A narrow door: the slot is named by the door rather than passed
    /// to it, and the expression is a parameter read at `Count`, so no
    /// expression algebra crosses and the edit cannot be aimed at a
    /// continuous slot. A degree says how the skin interpolates BETWEEN
    /// the sections, so a document whose degree is a literal is a
    /// re-authoring away from every other degree, and one bound here
    /// moves under a single `set_var_value` like any other named
    /// number.
    ///
    /// Refuses typed on a node with no v-degree slot — from Python
    /// that is every node but a `Node.loft` — and on an unknown or
    /// wrongly dimensioned parameter. The kernel's rule on the VALUE
    /// (`1 <= v_degree <= len(profiles) - 1`) is untouched by the
    /// binding: a bound degree is checked at `evaluate`, where a
    /// literal one is checked too.
    #[staticmethod]
    fn bind_v_degree_param(node: &NodeId, name: &VarName) -> Self {
        Self {
            inner: d::DocEdit::SetStructuralParam {
                node: node.0,
                slot: d::SlotId::VDegree,
                expr: d::Formula::named(name.0.clone(), d::Dimension::Count),
                fresh: Vec::new(),
            },
        }
    }

    /// **Write `constraints` as `placement`'s bundle**, in place of the
    /// one it holds, each id minted by the edit (`Doc.set_constraints`
    /// answers their addresses). Written whole, as `Doc.place`'s
    /// bundle is: a `Placed`, or `Mate`s and `MateValue`s.
    ///
    /// Refuses typed on `EditError`: `constraint_on_non_placement`,
    /// `mate_refused` with the door's fault (`mate_unpinned` for a
    /// bundle that leaves a freedom unwritten), and `space_mismatch`
    /// for a mate whose `to` is outside the placement's target.
    #[staticmethod]
    fn set_constraints(placement: &NodeId, constraints: super::mate::BundleArg) -> Self {
        Self {
            inner: d::DocEdit::SetConstraints {
                placement: placement.0,
                constraints: super::mate::bundle(constraints),
                fresh: Vec::new(),
            },
        }
    }

    /// **Add `mate` to `placement`'s bundle**, the bundle written whole
    /// again with it by `Placed`'s rule: the values the mates still
    /// leave kept, the rest of the freedoms they leave written `0`.
    /// The constraint's id is minted by the edit (`Doc.constrain`
    /// answers its address).
    ///
    /// Refuses typed on `EditError`: `constraint_on_non_placement`
    /// (the node holds no bundle), and `mate_refused` with the door's
    /// fault — `mate_not_on_the_copy`, `mate_kinds_differ`,
    /// `mate_overconstrained` where the other mates already fix what
    /// it fixes.
    #[staticmethod]
    fn add_constraint(placement: &NodeId, mate: &super::mate::Mate) -> Self {
        Self {
            inner: d::DocEdit::AddConstraint {
                placement: placement.0,
                mate: Box::new(mate.0.clone()),
                fresh: Vec::new(),
            },
        }
    }

    /// **Remove the constraint `mate` addresses** from its placement's
    /// bundle, the bundle written whole again without it by `Placed`'s
    /// rule: each value the remaining mates leave kept, each freedom
    /// they leave that no value sets written `0`, so the copy stays
    /// pinned (`Doc.remove_mate` answers the values it wrote).
    ///
    /// Refuses typed on `EditError`: `constraint_on_non_placement` and
    /// `unknown_constraint` (the bundle holds no such id).
    #[staticmethod]
    fn remove_constraint(mate: &super::mate::MateRef) -> Self {
        Self {
            inner: d::DocEdit::RemoveConstraint {
                placement: mate.0.placement,
                constraint: mate.0.id,
            },
        }
    }

    /// Move ONE instance's pin to a new version of the same document.
    ///
    /// The id does not move: identity and version stay distinct, and
    /// this edit touches only the version half. `update_references` is
    /// the whole-document elaboration over this primitive, and
    /// `Workspace.update_to_store` the one that computes the new pin
    /// from disk.
    ///
    /// **The new pin is RECIPE DATA, not a resolution.** `apply` does
    /// not reach across the document seam — it has no resolver and no
    /// store — so a pin naming content that does not exist is accepted
    /// HERE and refused at EVALUATION, through the seam vocabulary
    /// that names both pins (`part_pin_mismatch`, `part_unresolved`).
    /// Checking at the edit door would make the edit's meaning depend
    /// on which store was mounted when it was recorded, which is
    /// exactly what a recorded, replayable log must not carry.
    ///
    /// Refuses typed: `update_on_non_instance`, and `pin_unchanged`
    /// when the site already names that version — an update that
    /// records nothing is never reported as a success.
    #[staticmethod]
    fn update_reference(node: &NodeId, new_pin: &super::store::ContentPin) -> Self {
        Self {
            inner: d::DocEdit::UpdateReference {
                node: node.0,
                new_pin: new_pin.0,
            },
        }
    }

    /// **Repair a stored name**: rewrite every document site that
    /// references `from_name` EXACTLY to reference `to_name`.
    ///
    /// THE name repair, and the only one. A selection is stored as a
    /// stable name — a fillet's edges, a chamfer's, a shell's open
    /// faces — and a recipe edit upstream can
    /// leave one denoting something else or nothing at all. This edit
    /// says what it now denotes, once: no alias table persists and
    /// nothing follows automatically afterwards, so a second name
    /// that needs the same repair is a second edit.
    ///
    /// **A one-shot recorded intent, not a rename**, addressed by body
    /// and name: with `body` (the `Var` of a body, `Doc.output(node)`),
    /// the selections of that body naming `from_name` are rewritten,
    /// each re-canonicalizing as its kind does — an edge set re-sorts,
    /// a face set drops a repeat and keeps the earlier position; with
    /// no body, the names no selection holds (mate heads,
    /// appearance keys).
    ///
    /// Neither half keeps the kernel's bare word — `from` is a Python
    /// keyword — so both take the role suffix, exactly as
    /// `EditError.from_kind` / `to_kind` do rather than one of the
    /// pair reading oddly.
    ///
    /// Refuses typed on `EditError`: `rebind_identity` (a recorded
    /// no-op is noise), `rebind_kind_mismatch` (a face reference
    /// cannot come to denote an edge — the kind is part of the
    /// reference's type, and the refusal carries both kinds),
    /// `rebind_target_missing_node` (the selection must denote
    /// something the recipe still has), `rebind_unknown_name` (a
    /// source this document never minted is a typo; a
    /// deleted-but-once-lived node is the repair case and is
    /// allowed), and `rebind_no_references` (nothing references the
    /// source, so there is nothing to repair — a GUI's selection is
    /// not document state, and repairing one is re-selecting).
    #[staticmethod]
    #[pyo3(signature = (from_name, to_name, body=None))]
    fn rebind(from_name: &str, to_name: &str, body: Option<Var>) -> PyResult<Self> {
        Ok(Self {
            inner: d::DocEdit::Rebind {
                body: body.map(|var| var.0),
                from: name_from_text(from_name)?,
                to: name_from_text(to_name)?,
            },
        })
    }

    /// **Replace a live profile's PROGRAM whole** — its loops, their
    /// verbs, order and count, arc modes and targets — validated once,
    /// as one edit. The plane is not carried and does not move: it is
    /// the profile's one input, and no edit rewires a live node's
    /// inputs.
    ///
    /// `outline` is the profile description `Node.profile` takes —
    /// one closed loop, or `[outer, hole, hole]` in that order — read
    /// through the same door, so what this writes is what that mints.
    /// `keep` is one dict per new loop, in `outline`'s order, mapping
    /// the handle of a step of that loop's NEW program (the `.step` its
    /// verb returned) to the `StepId` of the old step it keeps
    /// (`Doc.step` reads them). A step no entry names is new, and the
    /// door mints it; a loop that keeps nothing is `{}`. Equal handles
    /// are one key, so two steps of one shaped prefix cannot both be
    /// named. The editor that
    /// reshaped the program is the one party that knows which leg it
    /// inserted, so the door is told rather than guessing.
    ///
    /// A name on a profile piece spells its step's id, so a name on a
    /// kept step keeps denoting its piece wherever the new program
    /// draws it and is not touched. A step the new program does not
    /// keep takes its id with it: every name on it — a fillet's
    /// selection, a shell's mouth, a derived frame's face, a paint —
    /// keeps its spelling, resolves to nothing, and is reported as a
    /// `strand` or a `stranded_appearance` until `DocEdit.rebind`
    /// repairs it. So is a name on a kept step's piece the new program
    /// stops drawing, as a fillet inserted before a leg takes the
    /// leg's segment.
    ///
    /// Raises `StepHandleError` `handle_off_program` for a handle that
    /// is not a step of its loop's new program. Refuses
    /// `step_ids_refused` before the program is replayed —
    /// `inner_variant` says which way the ids are wrong (`loop_count`,
    /// `shape`, `not_this_profiles`, or `repeated`; `not_minted`, an id
    /// the log lacks, is the load door's word for the same family) —
    /// `set_program_on_non_profile`
    /// for a node holding no program, and then everything an insert
    /// refuses of a profile: `slot_unknown_var_name` and its siblings
    /// over every argument, `profile_program_refused` for a program
    /// that does not close, replay or validate.
    #[staticmethod]
    fn set_program(
        py: Python<'_>,
        node: &NodeId,
        outline: &Bound<'_, PyAny>,
        keep: Vec<Bound<'_, PyDict>>,
    ) -> PyResult<Self> {
        let loops = loops_from_outline(py, outline)?;
        let keep = keep
            .iter()
            .map(|kept| {
                kept.iter()
                    .map(|(h, id)| {
                        Ok((
                            h.extract::<super::step::AuthoredStep>()?.0,
                            id.extract::<super::step::StepId>()?.0,
                        ))
                    })
                    .collect::<PyResult<Vec<_>>>()
            })
            .collect::<PyResult<Vec<_>>>()?;
        let ids =
            d::keep_grid(&loops, &keep).map_err(|refusal| super::step::handle_err(py, &refusal))?;
        Ok(Self {
            inner: d::DocEdit::SetProgram {
                node: node.0,
                loops,
                ids,
                fresh: Vec::new(),
            },
        })
    }
}

/// A loaded document: what the persistence door
/// answered — the snapshot as saved, the replayed current state, and
/// how many recorded edits the replay ran.
#[pyclass(frozen, module = "pncad")]
pub(crate) struct Loaded {
    snapshot: d::ProfileDoc,
    doc: d::ProfileDoc,
    edit_count: usize,
}

#[pymethods]
impl Loaded {
    /// The current document: the snapshot with every recorded edit
    /// replayed through the `apply` door.
    ///
    /// The replay runs below this wrapper, so the returned `Doc`
    /// reports no `last_maintenance` even where the replayed history's
    /// last edit performed some — that reading starts here.
    #[getter]
    fn doc(&self) -> Doc {
        Doc {
            inner: self.doc.clone(),
            maintenance: Vec::new(),
        }
    }

    /// The snapshot exactly as saved, before replay.
    #[getter]
    fn snapshot(&self) -> Doc {
        Doc {
            inner: self.snapshot.clone(),
            maintenance: Vec::new(),
        }
    }

    /// How many recorded edits the load replayed.
    #[getter]
    fn edit_count(&self) -> usize {
        self.edit_count
    }

    fn __repr__(&self) -> String {
        format!(
            "Loaded({} nodes, {} replayed edits)",
            self.doc.len(),
            self.edit_count
        )
    }
}

/// Parse, validate, and replay a saved document.
///
/// Every refusal is a typed `PersistError` carrying the arm's stable
/// `variant` tag — bad header, unknown schema, unparseable body, a
/// snapshot or edit log the shared validator rejects, an ε conflict.
#[pyfunction]
pub(crate) fn load(py: Python<'_>, text: &str) -> PyResult<Loaded> {
    let tol = Tol::witness();
    let loaded = d::load(text, tol).map_err(|err| persist_err(py, &err))?;
    Ok(Loaded {
        snapshot: loaded.snapshot,
        doc: loaded.doc,
        edit_count: loaded.edits.len(),
    })
}

/// **A tube's traversed window** — the kernel's `TubeWindow`, crossing
/// as a VALUE with two spellings and no third.
///
/// A class rather than an optional pair of angles, and that is the
/// design rather than ceremony: `window=None` would make "the full
/// ring" the shape you get by not saying anything, when it is one of
/// two things a caller must actually choose between. `Full` is a
/// spelling, not an omission, and the kernel refuses an arc that
/// reaches one full period precisely so the two never blur.
#[pyclass(module = "pncad", frozen)]
pub(crate) struct TubeWindow {
    inner: d::TubeWindow<d::Formula>,
}

#[pymethods]
impl TubeWindow {
    /// The full ring — the donut.
    #[staticmethod]
    fn full() -> Self {
        Self {
            inner: d::TubeWindow::Full,
        }
    }

    /// The arc from `t0` to `t1` about the spine axis, measured from
    /// the reference direction, right-handed. Wedge caps close the
    /// ends.
    ///
    /// Both angles cross as angle `Formula`s, the same seat
    /// `Node.revolve` takes its sweep angle at — a slot is never a
    /// bare float on this surface.
    ///
    /// Nothing is checked here. A reversed or degenerate span, and a
    /// span reaching one full period (which must say `full()`), are
    /// the kernel's own typed refusals at `evaluate`.
    #[staticmethod]
    fn arc(py: Python<'_>, t0: super::expr::SlotArg, t1: super::expr::SlotArg) -> PyResult<Self> {
        Ok(Self {
            inner: d::TubeWindow::Arc {
                t0: slot_expr(py, d::SlotId::TubeWindowStart, &t0)?,
                t1: slot_expr(py, d::SlotId::TubeWindowEnd, &t1)?,
            },
        })
    }

    fn __repr__(&self) -> String {
        match &self.inner {
            d::TubeWindow::Full => "TubeWindow.full()".to_owned(),
            d::TubeWindow::Arc { .. } => "TubeWindow.arc(...)".to_owned(),
        }
    }
}

/// Every kernel window spelling has a constructor on the python
/// mirror, enforced in the load-bearing direction: the match is over
/// the KERNEL enum, so a variant added there breaks this build rather
/// than leaving the python surface silently short of it. Never called.
const fn _binds_every_kernel_window(kernel: &d::TubeWindow<d::Formula>) -> &'static str {
    match kernel {
        d::TubeWindow::Full => "full",
        d::TubeWindow::Arc { .. } => "arc",
    }
}

/// Register the document surface on the module.
pub(crate) fn register(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<NodeId>()?;
    m.add_class::<IndexedRead>()?;
    m.add_class::<PickRead>()?;
    m.add_class::<Doc>()?;
    m.add_class::<DocEdit>()?;
    m.add_class::<VarName>()?;
    m.add_class::<Var>()?;
    m.add_class::<Measured>()?;
    m.add_class::<SketchedOnFace>()?;
    m.add_class::<FreeVar>()?;
    m.add_class::<VarDecl>()?;
    // The expansion bound `definition_too_large`'s `count` is measured
    // against.
    m.add("DEFINITION_NODE_BOUND", d::DEFINITION_NODE_BOUND)?;
    m.add_class::<FreeValue>()?;
    m.add_class::<Node>()?;
    m.add_class::<Recognised>()?;
    m.add_class::<TubeWindow>()?;
    m.add_class::<StationMove>()?;
    m.add_class::<Loaded>()?;
    m.add_function(wrap_pyfunction!(load, m)?)?;
    Ok(())
}
