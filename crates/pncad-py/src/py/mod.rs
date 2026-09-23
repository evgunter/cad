//! The PyO3 surface. Compiled only under the `python` feature.

mod analysis;
mod assembly;
pub(crate) mod checks;
pub(crate) mod doc;
mod expr;
mod flush;
mod mate;
mod measure;
mod mesh;
mod path;
mod pick;
mod place;
mod quantity;
mod readback;
mod refactor;
mod resolve;
mod select;
mod store;
mod value;

use pyo3::prelude::*;
use pyo3::types::PyDict;

use crate::errors::ErrorClass;

pyo3::create_exception!(
    pncad,
    PncadError,
    pyo3::exceptions::PyException,
    "Base class for every refusal this module raises.\n\n\
     Every subclass carries its refusal as ATTRIBUTES (LIBRARY-DESIGN \
     typed exceptions carrying the structured error, never \
     strings). The message is for humans; the attributes are the \
     contract."
);
pyo3::create_exception!(
    pncad,
    EditError,
    PncadError,
    "The document layer refused an edit (unknown node, cycle, slot \
     dimension mismatch, ...). Carries `variant`, which edit refused, \
     and `inner_variant`, the arm of the refusal that edit carries — \
     `None` where it carries none. `EvaluationError` states why the \
     second word is a second attribute.\n\n\
     The rest is the refusing arm's PAYLOAD, present on every arm and \
     `None` where that arm does not carry it: `node`, `input` and \
     `referenced_by` (the node the refusal is about, a node it names, \
     a node downstream that references it), `slot`, `param`, `name`, \
     `key`, `expected` and `found` (the dimension the door required \
     and the one it was offered), `kind`, `from_kind`, `to_kind`, \
     `count`, `first`, `again`, `value`, `offered`, `determinant`, \
     `path`, `value_path` and `pin`.\n\n\
     ONE ATTRIBUTE PER CONCEPT. Where two arms name one concept \
     differently the concept's clearest word wins — `expected`/ \
     `found` carry `declared`/`referenced` and `measured`/`bound` \
     too. Where two arms name two concepts the same they are spelled \
     apart: a short list's `found` is a COUNT and rides `count`, \
     because one attribute carries one type."
);
pyo3::create_exception!(
    pncad,
    EvaluationError,
    PncadError,
    "A node failed to evaluate, or was poisoned by an upstream \
     failure. Carries `node` and, for a poisoning, `through` — plus \
     the two words that say what refused: `kind` and \
     `inner_kind`.\n\n\
     TWO WORDS BECAUSE THERE ARE TWO ENUMS. `kind` is the CARRIER's \
     discriminant — `revolve`, `tube`, `shell` — fixed by the node's \
     kind before any payload is read, and what a caller branching on \
     the op ladder holds. `inner_kind` is the kernel refusal's OWN \
     arm, which exists only once the carrier has said which refusal \
     it holds: `wall_exceeds_radius` under `tube`, `sliver_rim` \
     under `revolve`. Neither is a coarser spelling of the other, so \
     each is projected where it lives; folding them into one \
     vocabulary would move every shipped `kind` value and leave a \
     caller splitting words by prefix.\n\n\
     `inner_kind` is `None` where the refusal has no arms, and where \
     `kind` is already the payload's own word (a mate, part or \
     placement-rule fault reads its discriminant straight through)."
);
pyo3::create_exception!(
    pncad,
    ValidationError,
    PncadError,
    "A body failed a topological or geometric validator. From the \
     validate doors it carries `door`, the gate that refused, and \
     `failure_count`, how many refusals it collected; from \
     `mass_properties` it carries `reason` instead.\n\n\
     `Body.validate_geometric_measured` raises through both halves: \
     the gate's shape when tier 3 refuses, and the \
     `mass_properties` shape when the gate passed and the \
     measurement could not reach its target. That second refusal \
     carries `volume_lo`, `volume_hi` and `surface_area` beside \
     `reason` — the sign-level bracket the gate DID certify, which \
     is the whole of what the quadrature is entitled to say about \
     such a body. They are `None` on every other refusal."
);
pyo3::create_exception!(
    pncad,
    QuantityOpMismatch,
    PncadError,
    "An operator applied to two QUANTITIES whose dimensions do not \
     admit it — `1 * m + 1 * rad`. Carries `op`, `left`, `right`: \
     the operator and the two dimension tags.\n\n\
     The class is the Rust type's own name. The quantity boundary's \
     operator check is not the library's only dimension check, and \
     the document layer's own refusal type reaches Python under \
     DOOR names rather than one type name: `LiteralError` from \
     literal construction, from the measurement constructors and from \
     the recorded-program lift; `ParseError` with `variant == \
     \"dimension\"` from `Doc.parse_expr`; `EditError` from \
     `Doc.apply`; and `PersistError` with `variant == \"dimension\"` \
     from `load`. Six doors, four classes — the roster with each \
     one's attribute is on `ErrorClass::DIMENSION_DOORS` in \
     `crate::errors`. Each carries the failing check's own tag, so \
     which check refused is branchable at every one."
);
pyo3::create_exception!(
    pncad,
    FmtQuantityError,
    PncadError,
    "`Length.format` or `Angle.format` refused a value: it is NaN or \
     ±∞, and a non-finite quantity has no display form. Carries \
     `variant` (`\"non_finite\"`) and `value`, the refused float.\n\n\
     Reachable, and that is why it is typed rather than asserted \
     away. `quantity`'s newtypes are plain value wrappers that \
     refuse no float, so `float(\"inf\") * mm` is an ordinary \
     `Length`; poison is stopped at the doors values LEAVE through, \
     and rendering one for a human is one of those. The recourse is \
     upstream by construction — nothing this formatter could be \
     asked differently turns poison into text — so the message names \
     the operation that produced the value."
);
pyo3::create_exception!(
    pncad,
    LiteralError,
    PncadError,
    "A value the expression layer refused: non-finite, or a count \
     written as a continuous literal. Carries `kind`, the stable tag \
     of the refusing arm.\n\n\
     Not `QuantityOpMismatch`: that one is the quantity boundary's \
     operator check, a different type. The expression layer's refusal \
     type has dimension-mismatch arms too, and it reaches Python at \
     six doors under four class names — the roster is on \
     `ErrorClass::DIMENSION_DOORS` in `crate::errors`. THIS class is \
     three of those six: literal construction, the measurement \
     arithmetic constructors, and the recorded-program lift (which \
     spells its tag `variant` rather than `kind` — filed, not \
     decided)."
);
pyo3::create_exception!(
    pncad,
    ParseError,
    PncadError,
    "`Doc.parse_expr` could not read the source as an expression. \
     Carries `variant`, the stable tag of the refusing arm, and \
     `pos`, the byte offset in the source — which for a parser is \
     the recourse, since it says WHERE to edit.\n\n\
     The arm's own payload rides beside them and is `None` on the \
     arms that do not carry it: `char`, `expected`, `found`, `text`, \
     `symbol`, `name`, `arity`/`given` (a function's declared arity \
     and the count supplied), and `kind`. That last one is the \
     dimension checker's tag, on `variant == \"dimension\"` — the \
     text door runs every smart constructor, so a reduction the \
     checker refuses arrives here rather than as `LiteralError`, \
     with the position a `LiteralError` has nowhere to put."
);
pyo3::create_exception!(
    pncad,
    EvalError,
    PncadError,
    "`Doc.eval` or `Doc.eval_count` refused an expression. Carries \
     `variant`, the stable tag of the refusing arm, plus `name` (the \
     parameter at fault), `expected` and `found` (dimension tags) \
     and `count`, each `None` where the arm does not carry it.\n\n\
     Numeric domain is NOT here: division by zero and out-of-domain \
     trig are not refusals in the expression layer — the evaluator \
     has no branches to hide them behind — and reach a caller as \
     `non_finite_result` on the finished value instead."
);
pyo3::create_exception!(
    pncad,
    PersistError,
    PncadError,
    "A save or load the persistence doors refused (bad header, \
     unknown schema, unparseable body, a snapshot or edit log that \
     fails the shared validator, ...). Carries `variant`, the stable \
     tag of the refusing arm, plus every arm's payload as \
     attributes — `None` where the arm does not carry one.\n\n\
     Four arms wrap a refusal of their own (a profile-program fault, \
     a distribution fault, a snapshot invariant, a replayed edit's \
     `EditError`), and its word rides beside the carrier's on \
     `inner_variant`; the nested refusal's own payload is the inner \
     door's surface. `detail` is the underlying reporter's own words \
     wherever an arm has one, `document` the document's recorded ε \
     wherever an arm reports it, and `site` the kernel's prose for \
     where a non-finite float sits."
);
pyo3::create_exception!(
    pncad,
    ExportError,
    PncadError,
    "The document-layer export door refused. Carries `variant` and \
     `node`; a poisoning adds `through`, a wrong-kind value adds \
     `kind`."
);
pyo3::create_exception!(
    pncad,
    TessellateError,
    PncadError,
    "The tessellator refused a body. Carries `variant`, the stable tag \
     of the refusing arm, plus the arm's numbers as attributes \
     (`value`, `bound`, `requested`, `note`; `None` where \
     inapplicable).\n\n\
     The offending face or edge is an arena KEY and does not cross — \
     the curation exists to keep those unnameable — so a refusal names \
     WHICH arm fired and, where the arm carries one, the number that \
     makes it actionable. The message is the tessellator's own prose; \
     the tag is the branchable part."
);
pyo3::create_exception!(
    pncad,
    StlError,
    PncadError,
    "An STL export refused. Carries `variant`, the stable tag of the \
     refusing arm, plus the arm's payload as attributes — `None` \
     where the arm does not carry one.\n\n\
     Three Rust refusals share this class because they refuse the same \
     CALL: the writers' own `StlError` (`degenerate_triangle`, \
     `index_out_of_range`, `too_many_triangles`, `io`), and the two \
     validated option newtypes, which are keyword arguments here — \
     `solid_name_unrepresentable`, `binary_header_too_long`, \
     `binary_header_sniffs_ascii`. The tags share one namespace, so \
     which of the three refused is readable off `variant`.\n\n\
     The payload: `triangle`, `index`, `count`, `character`, `len`, \
     and `detail` — the underlying reporter's own words, whether \
     that reporter is the output sink or the UTF-8 decoder."
);
pyo3::create_exception!(
    pncad,
    StepImportError,
    PncadError,
    "A STEP text the importer refused, or one that parsed to a \
     non-solid. Carries `variant` — the importer's own refusal tag, \
     one word per arm — or `wireframe`, which is not a refusal at \
     all: the file parsed, to something this door does not adopt. \
     Carries `promoted_kind` beside it, present on every arm and \
     `None` where that arm does not carry it.\n\n\
     `recognition_ambiguous` neither forwards nor withholds. The word \
     names the CONDITION — a face that cannot import without \
     promotion sits on a surface whose recognition estimator is \
     ill-conditioned at the file's own tolerance — and \
     `promoted_kind` says which analytic kind's estimator declined, \
     `plane` or `cylinder`. The two lead different places: a plane \
     that will not certify is a flatness question at the import \
     tolerance, a cylinder that will not is an ill-conditioned axis \
     and wants more of the patch. The entity ids and the conditioning \
     margin are in the message."
);
pyo3::create_exception!(
    pncad,
    PathError,
    PncadError,
    "The PATHS authoring algebra refused the geometry AT THE CALL \
     SITE (junction check, `NoCornerForFillet`, the tangent-line \
     close, a nonpositive radius, ...) — the same refusal the Rust \
     surface returns, raised where the verb was written. Carries \
     `variant`, the stable tag of the refusing arm."
);
pyo3::create_exception!(
    pncad,
    SelectRefusal,
    PncadError,
    "A selection query `select_where` could not answer — the same \
     typed refusal the Rust door returns (an in-band decided margin, \
     a tied name whose candidates disagree, a non-datum reference, \
     ...). Carries `reason`, the stable tag of the refusing arm, plus \
     the arm's payload as attributes (`None` where inapplicable)."
);
pyo3::create_exception!(
    pncad,
    IdentityError,
    PncadError,
    "A document identity could not be minted: the OS entropy source \
     refused. Identity is never defaulted — two documents sharing an \
     id are the same part, and a workspace refuses to hold both — so \
     the refusal is surfaced. Carries `variant`."
);
pyo3::create_exception!(
    pncad,
    WorkspaceError,
    PncadError,
    "The workspace store refused. Carries `variant`, the stable tag \
     of the refusing arm, and the arm's payload as attributes — \
     `path`, `id`, `first`, `second`, `wanted`, `found` — each \
     present on every arm and `None` where that arm does not carry \
     it.\n\n\
     The arm the store exists to make loud is `pin_mismatch`: a \
     reference names a VERSION, so a document that changed under one \
     refuses with `wanted` and `found` rather than resolving to the \
     new content. `pncad.PIN_MISMATCH_RECOURSE` is the recourse \
     sentence its message ends on."
);
pyo3::create_exception!(
    pncad,
    MateError,
    PncadError,
    "The mate solve could not place an instance. Carries `variant`, \
     the stable tag of the refusing arm, and `fault` — the \
     `MateFault` VALUE, which carries the arm's payload.\n\n\
     The solve itself is TOTAL and never raises: a refusing cluster \
     must not fail an unrelated one, so `solve_document` records the \
     fault per node and `SolvedPoses.fault` hands back the same value \
     this exception carries. This class is raised only where an \
     answer is a pose or nothing — `SolvedPoses.placement`. One \
     payload vocabulary, so the value and the exception cannot \
     disagree."
);
pyo3::create_exception!(
    pncad,
    AssemblyError,
    PncadError,
    "The at-rest assembly gate refused. Carries `variant`, the stable \
     tag of the refusing arm, plus the arm's payload as attributes \
     (`mate`, `side`, `name`, `why`, `class_`, `findings`), each \
     present on every arm and `None` where that arm does not carry \
     it.\n\n\
     **The two verdict arms are NOT interchangeable.** `at_rest` is a \
     finding AGAINST the document — a refuted declaration or an \
     undeclared contact. `uncertified` is the declared direction's \
     FRONTIER: nothing was refuted and nothing was undeclared, the \
     census simply has no certifier lane for the faces a declaration \
     names, so nothing was decided about the geometry either way. A \
     caller who catches this class must say which of the two they \
     mean.\n\n\
     A gather refusal arrives here under the gather's OWN tag \
     (`no_body_roots`, `root_failed`, ...), not a wrapper tag: which \
     invariant broke is what a caller branches on."
);
pyo3::create_exception!(
    pncad,
    ProductError,
    PncadError,
    "The whole-document gather refused. Carries `variant`, the stable \
     tag of the refusing arm, plus `node`, `through` and `name` \
     (`None` where the arm does not carry them).\n\n\
     A product is all of the roots or none of them — there are no \
     partial products."
);
pyo3::create_exception!(
    pncad,
    SplitError,
    PncadError,
    "The `split` refactoring refused. Carries `variant`, the stable \
     tag of the refusing arm, plus its payload as attributes \
     (`node`, `consumer`, `input`, `gauge`, `instance`, `param`, \
     `name`, `id`), `None` where inapplicable."
);
pyo3::create_exception!(
    pncad,
    InlineError,
    PncadError,
    "The `inline` refactoring refused. Carries `variant`, the stable \
     tag of the refusing arm, plus its payload as attributes \
     (`node`, `by`, `name`, `param`, `key`, `root`, `host_epsilon`, \
     `part_epsilon`), `None` where inapplicable.\n\n\
     Inline crosses the SAME document seam evaluation does, so a \
     reference that will not resolve refuses under the seam's own \
     tags — `part_pin_mismatch`, `part_epsilon_seam`, \
     `part_unresolved` — the ones `EvaluationError.kind` already \
     speaks. A stale pin is refused here, never silently retargeted."
);
pyo3::create_exception!(
    pncad,
    UpdateError,
    PncadError,
    "A whole-document pin update produced no edit list. Carries \
     `variant` (`no_such_reference` or `already_pinned`), `id` — the \
     document id, which both arms name because \"which part did you \
     mean\" is the only question an author can act on here — and \
     `pin`, the pin every site already names, on `already_pinned` \
     alone."
);
pyo3::create_exception!(
    pncad,
    ReadbackError,
    PncadError,
    "A read-back door could not say what a name denotes or where it \
     sits. Carries `variant`, the stable tag of the refusing arm, \
     plus the arm's payload as attributes (`node`, `through`, \
     `candidates`, `wanted`, `found`, `index`, `payload`, \
     `carrier`), each present on every arm and `None` where that arm \
     does not carry it.\n\n\
     Two refusals share this class because they refuse the same \
     CALL — \"where is the entity this name denotes\". The name \
     half resolves the name against the evaluation \
     (`no_such_name`, `ambiguous`, `wrong_kind`, `whole_body`, the \
     node ladder); the GEOMETRY half reads the carrier and arrives \
     under its own tags, not a wrapper tag (`dangling_entity`, \
     `dangling_geometry`, `no_canonical_frame`, `no_carrier`).\n\n\
     The two dangling tags stay apart because they are different \
     facts about the model: `dangling_entity` is a stale or foreign \
     handle, `dangling_geometry` is a live entity naming geometry \
     the body itself no longer has.\n\n\
     `ambiguous` is the one to read twice: a tie is a naming success \
     and a referencing failure, and the door refuses rather than \
     picking a candidate. `Evaluation.denotation` is how a caller \
     asks BEFORE reading a frame."
);
pyo3::create_exception!(
    pncad,
    HitTestError,
    PncadError,
    "A hit test could not answer. Carries `variant`, the stable tag of \
     the refusing arm, plus `node`, `through`, `kind`, `body` and \
     `hits`, each present on every arm and `None` where that arm does \
     not carry it.\n\n\
     A MISS is not this. The ray hitting no offered triangle is \
     `None`, typed, and an error is never flattened into it — so \
     catching this class never means \"nothing was there\".\n\n\
     Three arms are the standing ladder, spelled exactly as \
     `ReadbackError` spells it (`node_not_evaluated`, `node_failed`, \
     `node_poisoned`): a mesh displayed for a node this evaluation did \
     not produce cannot belong to it. `unnamed` is a KERNEL BUG \
     report — the node evaluated and the entity has no name in its \
     table — and it carries the entity's `kind` and `body`, never its \
     arena key.\n\n\
     `ambiguous` is the certified tie BETWEEN FACES, and it is the \
     one to read twice: the ray met several faces the arithmetic \
     cannot order — a cube's shared edge, a corner, a face met \
     edge-on in front of a transversal one — and the door names them \
     all rather than choosing on a rule you did not ask for. `hits` \
     is the list of `PickHit`, one per tied face, each of them TRUE \
     and complete. Several triangles of ONE face are not this: they \
     are one answer, with the hull of their intervals.\n\n\
     `NodePick.patch_names` answers with instances of this class IN A \
     SLOT rather than raising: one naming-emission bug must not cost a \
     consumer the names of every other patch it is drawing."
);
pyo3::create_exception!(
    pncad,
    NodePickError,
    PncadError,
    "A pick index could not be built. Carries `variant`, the stable \
     tag of the refusing arm, plus `node`, `through`, `kind`, `body`, \
     `hits`, `index_variant`, `patch`, `triangle` and `index`, each \
     present on every arm and `None` where that arm does not carry \
     it.\n\n\
     `not_a_body` and `no_such_body` are different states and stay \
     apart: a datum, profile, declaration or mate NEVER draws, while a \
     node that draws nothing today (an annihilated boolean, an empty \
     split side) draws again after an edit.\n\n\
     Two arms FORWARD rather than wrap. The standing ladder arrives \
     under `HitTestError`'s own tags, because it IS that refusal; a \
     tessellation refusal arrives under the tessellator's own tag and \
     prose. What a forwarded arm does not bring is the inner refusal's \
     extra ATTRIBUTES — a tessellation refusal's `value`, `bound`, \
     `requested` and `note` stay on `TessellateError`, where \
     `Body.tessellate` raises them. `mesh_index` neither forwards nor \
     withholds: the word names the door whose invariant broke — the \
     pick INDEX's — and `index_variant` carries the payload's own \
     discriminant beside it, `position_out_of_range` today, with the \
     three numbers that arm carries: `patch` and `triangle` locate the \
     offending triangle in the mesh value, `index` is the position it \
     referenced outside the buffer. The payload's own type is not \
     raisable, so this is the only door those numbers cross."
);
pyo3::create_exception!(
    pncad,
    ChecksError,
    PncadError,
    "The advisory-check registry could not RUN. Carries `variant`, the \
     stable tag of the refusing arm (`root_without_value`, `band`, \
     `product_unavailable`), and `node` — the root without a value, \
     `None` on the other arms.\n\n\
     NOT a finding. A check that ran and disagreed is a value in the \
     report; this class means nothing was checked."
);
pyo3::create_exception!(
    pncad,
    CheckRefusal,
    PncadError,
    "`enforce_checks` refused: the report carries findings whose check \
     the CALLER configured at `Severity.Error`. Carries `findings`, \
     every refusing `CheckFinding` in report order.\n\n\
     The registry's one refusing path, and it refuses on nothing the \
     caller did not ask to be refused on — no default severity is \
     `Error`, and the separation resident cannot be set to it at all."
);
pyo3::create_exception!(
    pncad,
    FrameError,
    PncadError,
    "A frame constructor refused its inputs — the same typed refusal \
     the Rust door returns: a direction that was not DEFINITELY \
     usable (coincident eye and target, a roll reference along the \
     aim, a zero mirror normal), or a tolerance yielding no usable \
     band. Carries `variant`, the stable tag of the refusing arm — \
     which names the offending INPUT, so nothing else spells that \
     fact — plus the arm's payload as attributes, `None` where the \
     arm does not carry one.\n\n\
     A margin that landed in the ambiguity band carries the \
     classifier's diagnostic: `margin` (or `margin_low` / \
     `margin_high` for an enclosure), the band's `zero` and \
     `escalate`, and the deciding `predicate`. A definite zero \
     carries none of it. The band arm carries its own word on \
     `inner_variant`, with `field` and `value` beside it."
);

pyo3::create_exception!(
    pncad,
    DistributionFault,
    PncadError,
    "A `Distribution` constructor was handed offsets that break an \
     E2 invariant. Carries `variant` (the stable tag), and `field`, \
     `sigma`, `lo`, `hi` — the arms' payloads, present on every arm \
     and `None` where that arm does not carry one.\n\n\
     The kernel's own `Distribution::check` decides this, so a \
     distribution Python accepts is one the edit door and the \
     persistence validator accept too. Raised EARLY, at the value \
     rather than at the edit: the same fault reaches `EditError` as \
     `invalid_distribution` when a document is loaded or edited \
     another way."
);
pyo3::create_exception!(
    pncad,
    MeasureUnavailable,
    PncadError,
    "A mass could not be priced: the parameter carries a BAND, which \
     states limits without a shape. Carries `variant` and `param`, \
     the parameter that blocked the pricing.\n\n\
     A refusal, not an absence. `band` is the author saying they know \
     the extremes and not the distribution, and promoting it to a \
     uniform would be a strictly stronger claim than they made — so \
     the door refuses anything whose answer would depend on the \
     shape, and answers only the two cases every measure on the band \
     agrees about."
);
pyo3::create_exception!(
    pncad,
    MeasureNodeFault,
    PncadError,
    "`Node.measure` was handed an expression that reads a reference \
     the node does not carry. Carries `variant` (the stable tag), \
     `verb` (which primitive reads it), `index` (the out-of-range \
     one) and `refs` (how many the node carries).\n\n\
     The kernel's own `Node::measure` decides this — the one \
     construction door, running the check the edit door and the load \
     door's re-check both run — so a measure Python accepts is one a \
     document accepts. Raised EARLY, at the node rather than at the \
     edit: the same fault reaches `EditError` as `measure_malformed` \
     when a document is loaded or edited another way."
);
pyo3::create_exception!(
    pncad,
    MeasureUnavailableAt,
    PncadError,
    "A measure whose answer is an ENCLOSURE, read at a build whose \
     scalar is a point. Carries `variant`, `verb`, `scalar` and \
     `door` — the primitive, the scalar this build ran at, and the \
     door that CAN answer.\n\n\
     Not `MeasureUnavailable`, which is the analysis lane refusing to \
     price a mass over a band. This is the measurement lane, and it \
     is a typed ABSENCE rather than a failure: the measure node \
     evaluated fine and has no value, which is why an assertion over \
     it reports `Unevaluated` carrying this same reason instead of \
     being poisoned. A `min_clearance` at `f64` is the whole of it \
     today — a station pair found by a point-scalar search is an \
     upper bound on the minimum rather than the minimum, and \
     reporting one would be a degradation ERROR-DESIGN E7 forbids by \
     name."
);
pyo3::create_exception!(
    pncad,
    McRefusal,
    PncadError,
    "A Monte-Carlo run produced nothing (ERROR-DESIGN E11.1). Carries \
     `variant` (the stable tag), and `param`, `node` and `cause` — the \
     arms' payloads, present on every arm and `None` where that arm \
     does not carry one.\n\n\
     Three ways a run has no estimate: a varying parameter carries a \
     BAND, which states limits without a shape and cannot be drawn \
     from (`band_has_no_measure`, `param`); the request asked for zero \
     samples, and an estimator over no draws has no estimate \
     (`no_samples`); or the document does not build at its nominal, so \
     there is nothing to replay (`nominal_does_not_build`, `node` and \
     `cause`).\n\n\
     The band arm's `variant` is `MeasureUnavailable`'s own word, \
     because it carries that refusal: the lane refuses the WHOLE run \
     naming the parameter rather than sampling the rest, since a mean \
     over a subset of the parameters is an estimate of a different \
     document."
);
pyo3::create_exception!(
    pncad,
    AnalysisPolicyError,
    PncadError,
    "An `AnalysisPolicy` that cannot be honoured: `quantile_mass` is \
     not a finite number strictly inside `(0, 1)`. Carries `variant` \
     and `mass`, the requested share.\n\n\
     Mass 1 asks for an infinite box and mass 0 for an empty one, and \
     neither is a box."
);

/// Raise the exception class [`ErrorClass`] names, with `fields`
/// attached as instance attributes.
///
/// This is the single construction site for every typed refusal, so
/// "the payload is attributes, not prose" is enforced in one place
/// rather than repeated at each raise — and so is its twin, "the
/// message is the kernel's own `Display`, never a `Debug` dump":
/// [`crate::errors::reads_as_prose`] runs here on every raise, which
/// makes the rule hold for doors written after it rather than only
/// for the ones it was written for.
///
/// **One attribute name is unusable: `args`.** It is
/// `BaseException`'s own, and CPython requires it to be a tuple, so
/// setting it to anything else — `None` included, which is what the
/// "every field on every arm" shape puts on the arms that do not
/// carry it — raises `TypeError: 'NoneType' object is not iterable`
/// from inside the raise itself. Found at LIB-B-EXPR-READ, where
/// `WrongArity`'s argument count wanted the name; it is `given`
/// there instead. The rest of `BaseException`'s surface
/// (`with_traceback`, `add_note`, `__notes__`) is method- or
/// dunder-shaped and no payload has wanted one.
///
/// It is a `debug_assert`, which in this workspace is live in every
/// profile — the root manifest keeps them on under release too — so it
/// runs over every door the Python suite exercises and stays on in a
/// built wheel. A Debug dump reaching a user is a binding bug, and
/// D9's converse says a detectable bug state panics; what the check
/// cannot see is a door no test reaches.
///
/// **The second thing enforced in one place: a class that carries a
/// discriminant mints it here.** [`ErrorClass::Evaluation`] holds a
/// [`crate::errors::EvalReason`] and [`ErrorClass::Validation`] a
/// [`crate::errors::ValidationRefusal`]; [`class_discriminant`] says
/// which attribute each writes and what word, and [`raise_typed`]
/// writes it — not the raise site, which cannot name either class
/// without naming a variant and therefore cannot spell a word of its
/// own. A site that passes one of those attributes anyway is
/// overwritten by the minted word, for the attribute the refusal in
/// hand writes, and named by the assertion below for EITHER attribute
/// the class mints onto; that is the one gap the type cannot close,
/// since the payload is a list of `(&str, Py<PyAny>)` pairs and any
/// name is spellable in it. The assertion reads
/// [`ClassDiscriminant::attributes`], which is why a `reason` passed
/// beside a `ValidationError` whose refusal writes `door` is caught
/// rather than reaching Python untouched.
pub(crate) fn typed_err(
    py: Python<'_>,
    class: ErrorClass,
    message: impl Into<String>,
    fields: &[(&str, Py<PyAny>)],
) -> PyErr {
    let message: String = message.into();
    debug_assert!(
        crate::errors::reads_as_prose(&message),
        "{} was raised with a `Debug` rendering where its human \
         message belongs: {message}",
        class.class_name()
    );
    // Over the class's WHOLE attribute set, not the one word this
    // value writes: `ValidationError` mints onto two attributes and a
    // site spelling the other one would otherwise survive the raise.
    let minted = class_discriminant(class);
    debug_assert!(
        minted.is_none_or(|d| !fields.iter().any(|(name, _)| d.attributes.contains(name))),
        "{}'s `{}` is minted from the discriminant its class carries; a \
         raise site that passes one too is spelling a Python-visible \
         word where no inventory reads it",
        class.class_name(),
        minted.map_or("", |d| spelled_discriminant(&d, fields).unwrap_or(""))
    );
    raise_typed(py, class, message, fields)
}

/// The class table and the attribute loop.
///
/// Split out from [`typed_err`] when a second raising door existed. It
/// has one caller now, so the split buys no sharing; it stays because
/// the two halves answer different questions — this one maps a class to
/// a Python exception type and hangs the payload on it, while
/// [`typed_err`] decides whether a message is fit to raise at all. A
/// reader looking for either finds it without the other.
fn raise_typed(
    py: Python<'_>,
    class: ErrorClass,
    message: String,
    fields: &[(&str, Py<PyAny>)],
) -> PyErr {
    let err = match class {
        ErrorClass::Edit => EditError::new_err(message),
        ErrorClass::Evaluation(_) => EvaluationError::new_err(message),
        ErrorClass::Validation(_) => ValidationError::new_err(message),
        ErrorClass::QuantityOp => QuantityOpMismatch::new_err(message),
        ErrorClass::FmtQuantity => FmtQuantityError::new_err(message),
        ErrorClass::Literal => LiteralError::new_err(message),
        ErrorClass::Parse => ParseError::new_err(message),
        ErrorClass::Eval => EvalError::new_err(message),
        ErrorClass::Persist => PersistError::new_err(message),
        ErrorClass::Export => ExportError::new_err(message),
        ErrorClass::Tessellate => TessellateError::new_err(message),
        ErrorClass::StlExport => StlError::new_err(message),
        ErrorClass::StepImport => StepImportError::new_err(message),
        ErrorClass::Path => PathError::new_err(message),
        ErrorClass::Select => SelectRefusal::new_err(message),
        ErrorClass::Frame => FrameError::new_err(message),
        ErrorClass::Identity => IdentityError::new_err(message),
        ErrorClass::Workspace => WorkspaceError::new_err(message),
        ErrorClass::Mate => MateError::new_err(message),
        ErrorClass::Assembly => AssemblyError::new_err(message),
        ErrorClass::Product => ProductError::new_err(message),
        ErrorClass::Split => SplitError::new_err(message),
        ErrorClass::Inline => InlineError::new_err(message),
        ErrorClass::Update => UpdateError::new_err(message),
        ErrorClass::Readback => ReadbackError::new_err(message),
        ErrorClass::HitTest => HitTestError::new_err(message),
        ErrorClass::NodePick => NodePickError::new_err(message),
        ErrorClass::Checks => ChecksError::new_err(message),
        ErrorClass::Enforce => CheckRefusal::new_err(message),
        ErrorClass::Distribution => DistributionFault::new_err(message),
        ErrorClass::Measure => MeasureUnavailable::new_err(message),
        ErrorClass::MeasureNode => MeasureNodeFault::new_err(message),
        ErrorClass::MeasureUnavailableAt => MeasureUnavailableAt::new_err(message),
        ErrorClass::AnalysisPolicy => AnalysisPolicyError::new_err(message),
        ErrorClass::Mc => McRefusal::new_err(message),
    };
    // Attaching attributes needs the instance, which materialises the
    // exception value; a failure here would itself be a Python error,
    // so it replaces the original rather than being swallowed.
    let value = err.value(py);
    for (name, field) in fields {
        if let Err(set_failed) = value.setattr(*name, field.bind(py)) {
            return set_failed;
        }
    }
    // The class's OWN discriminant, after the raise site's fields so
    // that the word the class carries is the word Python reads even
    // where a site spelled one beside it (`typed_err` asserts that it
    // did not).
    if let Some(minted) = class_discriminant(class)
        && let Err(set_failed) = value.setattr(
            minted.attribute,
            pyo3::types::PyString::new(py, minted.word),
        )
    {
        return set_failed;
    }
    PyErr::from_value(value.clone().into_any())
}

/// What a class mints for itself: the attributes its own discriminant
/// can write, and the one this value writes with the word it writes
/// there.
#[derive(Clone, Copy)]
struct ClassDiscriminant {
    /// **Every** attribute this class's discriminant writes, over all
    /// of that discriminant's variants — one word for
    /// [`crate::errors::EvalReason`], two for
    /// [`crate::errors::ValidationRefusal`], whose four door refusals
    /// and one measurement refusal do not write the same one. It is
    /// the set a raise site of this class may not spell, and it is
    /// wider than `attribute` on purpose.
    attributes: &'static [&'static str],
    /// The attribute THIS value writes, which is one of `attributes`.
    attribute: &'static str,
    /// The word written there, from the discriminant's exhaustive map.
    word: &'static str,
}

/// The attribute a class's own carried discriminant is written to, and
/// the word written there — `None` for a class that carries none.
///
/// The two classes that carry one are the two whose discriminant is
/// this crate's decision rather than a kernel refusal's tag, and
/// carrying it is what takes the choice of word away from the raise
/// site: a site cannot name the class without naming a variant, and the
/// word is minted here from the exhaustive map rather than read off the
/// field list. Every other class's `variant` or `reason` is a kernel
/// enum's word, taken from that enum's own map at the raise.
///
/// **Exhaustive, with no wildcard arm.** A class that carries a
/// discriminant and is not named here would fall into a `None` the
/// assertion in [`typed_err`] reads as "nothing to check", so the
/// generalised assertion above is only as general as this table: the
/// next carrying class has to be written in, and the compiler is what
/// says so. Every class-keyed match in this crate is exhaustive for the
/// same reason.
fn class_discriminant(class: ErrorClass) -> Option<ClassDiscriminant> {
    match class {
        ErrorClass::Evaluation(reason) => Some(ClassDiscriminant {
            attributes: crate::errors::EvalReason::ATTRIBUTES,
            attribute: crate::errors::EvalReason::ATTRIBUTE,
            word: crate::tags::eval_reason_tag(reason),
        }),
        ErrorClass::Validation(refusal) => Some(ClassDiscriminant {
            attributes: crate::errors::ValidationRefusal::ATTRIBUTES,
            attribute: refusal.attribute(),
            word: crate::tags::validation_refusal_tag(refusal),
        }),
        ErrorClass::Edit
        | ErrorClass::QuantityOp
        | ErrorClass::FmtQuantity
        | ErrorClass::Literal
        | ErrorClass::Parse
        | ErrorClass::Eval
        | ErrorClass::Persist
        | ErrorClass::Export
        | ErrorClass::Tessellate
        | ErrorClass::StlExport
        | ErrorClass::StepImport
        | ErrorClass::Path
        | ErrorClass::Select
        | ErrorClass::Frame
        | ErrorClass::Identity
        | ErrorClass::Workspace
        | ErrorClass::Mate
        | ErrorClass::Assembly
        | ErrorClass::Product
        | ErrorClass::Split
        | ErrorClass::Inline
        | ErrorClass::Update
        | ErrorClass::Readback
        | ErrorClass::HitTest
        | ErrorClass::NodePick
        | ErrorClass::Checks
        | ErrorClass::Enforce
        | ErrorClass::Distribution
        | ErrorClass::Measure
        | ErrorClass::MeasureNode
        | ErrorClass::MeasureUnavailableAt
        | ErrorClass::AnalysisPolicy
        | ErrorClass::Mc => None,
    }
}

/// Which of a class's own attributes a raise site spelled, if any —
/// the name the assertion in [`typed_err`] reports.
fn spelled_discriminant(
    minted: &ClassDiscriminant,
    fields: &[(&str, Py<PyAny>)],
) -> Option<&'static str> {
    minted
        .attributes
        .iter()
        .copied()
        .find(|attribute| fields.iter().any(|(name, _)| name == attribute))
}

/// Python bindings for the pncad B-rep CAD kernel.
///
/// Author exact solids as a document, evaluate it, measure and export
/// the result. Start with `docs/GUIDE.md` §2.8 or the worked script
/// `crates/pncad-py/examples/bracket.py`; `crates/pncad-py/README.md`
/// covers installation. Refusals are typed exceptions carrying
/// attributes — all of them subclass `PncadError`.
///
/// The name `pncad` is a placeholder until the project is named.
#[pymodule]
#[pyo3(name = "pncad")]
fn pncad_module(m: &Bound<'_, PyModule>) -> PyResult<()> {
    let py = m.py();

    m.add("PncadError", py.get_type::<PncadError>())?;
    m.add("EditError", py.get_type::<EditError>())?;
    m.add("EvaluationError", py.get_type::<EvaluationError>())?;
    m.add("ValidationError", py.get_type::<ValidationError>())?;
    m.add("QuantityOpMismatch", py.get_type::<QuantityOpMismatch>())?;
    m.add("FmtQuantityError", py.get_type::<FmtQuantityError>())?;
    m.add("LiteralError", py.get_type::<LiteralError>())?;
    m.add("ParseError", py.get_type::<ParseError>())?;
    m.add("EvalError", py.get_type::<EvalError>())?;
    m.add("PersistError", py.get_type::<PersistError>())?;
    m.add("ExportError", py.get_type::<ExportError>())?;
    m.add("TessellateError", py.get_type::<TessellateError>())?;
    m.add("StlError", py.get_type::<StlError>())?;
    m.add("StepImportError", py.get_type::<StepImportError>())?;
    m.add("PathError", py.get_type::<PathError>())?;
    m.add("SelectRefusal", py.get_type::<SelectRefusal>())?;
    m.add("FrameError", py.get_type::<FrameError>())?;
    m.add("IdentityError", py.get_type::<IdentityError>())?;
    m.add("WorkspaceError", py.get_type::<WorkspaceError>())?;
    m.add("MateError", py.get_type::<MateError>())?;
    m.add("AssemblyError", py.get_type::<AssemblyError>())?;
    m.add("ProductError", py.get_type::<ProductError>())?;
    m.add("SplitError", py.get_type::<SplitError>())?;
    m.add("InlineError", py.get_type::<InlineError>())?;
    m.add("UpdateError", py.get_type::<UpdateError>())?;
    m.add("ReadbackError", py.get_type::<ReadbackError>())?;
    m.add("HitTestError", py.get_type::<HitTestError>())?;
    m.add("NodePickError", py.get_type::<NodePickError>())?;
    m.add("ChecksError", py.get_type::<ChecksError>())?;
    m.add("CheckRefusal", py.get_type::<CheckRefusal>())?;
    m.add("DistributionFault", py.get_type::<DistributionFault>())?;
    m.add("MeasureUnavailable", py.get_type::<MeasureUnavailable>())?;
    m.add("MeasureNodeFault", py.get_type::<MeasureNodeFault>())?;
    m.add(
        "MeasureUnavailableAt",
        py.get_type::<MeasureUnavailableAt>(),
    )?;
    m.add("AnalysisPolicyError", py.get_type::<AnalysisPolicyError>())?;
    m.add("McRefusal", py.get_type::<McRefusal>())?;

    quantity::register(m)?;
    path::register(m)?;
    place::register(m)?;
    doc::register(m)?;
    expr::register(m)?;
    select::register(m)?;
    readback::register(m)?;
    pick::register(m)?;
    resolve::register(m)?;
    store::register(m)?;
    mate::register(m)?;
    assembly::register(m)?;
    refactor::register(m)?;
    flush::register(m)?;
    checks::register(m)?;
    mesh::register(m)?;
    value::register(m)?;
    analysis::register(m)?;
    measure::register(m)?;

    // Build-provenance surface. The persistence format carries no
    // schema version to publish here (the persist module docs say
    // why): a file this build cannot read refuses `unreadable`.
    let meta = PyDict::new(py);
    meta.set_item("f64_only", true)?;
    meta.set_item("abi3", "py38")?;
    m.add("__build_info__", meta)?;

    Ok(())
}
