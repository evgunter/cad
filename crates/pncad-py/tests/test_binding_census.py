"""The Python-side census of the façade's curated surface, as a TEST.

The Rust façade has one of these already:
`crates/pncad/tests/all.rs::every_document_layer_root_export_is_carried_or_listed`
makes the document layer's growth fail LOUDLY at the façade — every
`editor-core` root export is either carried by a `pub use` list or
named in `NOT_CARRIED` with the family it belongs to. Python had no
equivalent. `tests/test_stubs.py` compares the `.pyi` against the
compiled module name for name, which catches stub DRIFT and nothing
else: a door curated into the Rust façade and never spelled in Python
is invisible to it. Doors accumulated that way — the assembly gate,
the advisory checks, picking, the expression read side, the
workspace/content-pin family — and nothing noticed.

This is the mechanism. Every name the façade's three curated lists
introduce is either bound in Python or listed below with the family it
belongs to, and a name that is neither fails here, naming itself.

STDLIB ONLY, AND NO COMPILED MODULE. Every input is source TEXT — the
three façade `.rs` files and `pncad.pyi` — so this runs wherever
`python3` runs, in the same degraded environment `run-python-tests.sh`
exists for (that box has no pip and no ensurepip). It deliberately
does NOT import `pncad`: `test_stubs.py` already pins the stub to the
module, so the stub is a faithful stand-in for the module, and reading
it instead means the census is runnable even when nothing has been
built.

THE SIDES IT COMPARES
---------------------
The Rust side is the `pub use` lists in `crates/pncad/src/document.rs`,
`select.rs` and `prelude.rs` — the three files that ARE the curated
surface. They are read with the Rust guard's own technique: comments
stripped first, then the leaf name of every `pub use` item, so prose
naming a type is not read as an export. `prelude.rs` re-exports through
`crate::document` and `crate::select`, so a prelude entry has an origin
in the same census and nothing is double-counted; the geometry crates
(`geom_core`, `profile`, `sweep`, `topo`, `mesh`, `stl`, `step_export`,
`step_import`, `quantity`) enter through the prelude — plus the two
`topo::readback` names `select.rs` lifts — and that is the whole point:
the prelude is what a `use pncad::prelude::*` consumer gets, so it is
the surface Python is measured against.

The Python side is the top-level names `pncad.pyi` declares, plus the
`Class.member` spellings it declares, extracted with `ast`.

THE MAPPING RULE, STATED HONESTLY
---------------------------------
Name-for-name equality does not work in either direction. Python spells
much of this surface as METHODS — `Node.fillet` for the kernel's
`fillet_edges`, `Evaluation.select` for the free `select`, `Doc.apply`
for `apply` — and renames where a Python keyword or convention demands
it (`IN` is `inch`; `RecipeNodeId` is `NodeId`). So a curated name is
accounted for in exactly one of three ways:

1. `pncad.pyi` declares a top-level name spelled identically. Sixty
   names land here — `Doc`, `Node`, `Selector`, `SegTag`, `circle`.
2. `BOUND_AS` maps it to the Python spelling that answers the same
   question, and THAT SPELLING IS VERIFIED to exist in the stub — a
   mapping naming a spelling the stub does not declare fails. Without
   that check the roster would be decorative: any curated name could be
   waved off by writing a plausible method name beside it.
3. `NOT_BOUND` lists it with the family it belongs to.

What `BOUND_AS` claims: a Python caller can do the thing the curated
Rust name does, at that spelling. What it does NOT claim: the same
signature, the same receiver, or the same layer. Python's surface is
document-layer-first, so the kernel's direct body operations arrive as
recipe-node constructors (`union` and its two siblings are one
`Node.boolean` taking a `BooleanOp`), and a mapping is a pointer to the
door, not an assertion that the two are interchangeable. Semantics are
not checked at all: nothing here verifies that Python's `Frame` is the
`Frame` the façade curates. That is `ty`'s and the corpus tests' job.

BOTH DIRECTIONS DECAY, exactly as the Rust guard's do. An entry in
either roster that is no longer a curated façade name is stale, and so
is a `NOT_BOUND` entry Python has since started binding, or a
`BOUND_AS` entry whose curated name Python now spells identically. A
roster that only ever grows is a roster nobody is reading.

WHAT THIS DOES NOT CLAIM
------------------------
- Not that each `NOT_BOUND` entry is individually argued. They are
  argued BY FAMILY, in that constant's docstring, the way `NOT_CARRIED`
  argues its own.
- Not that the `gap:` families are decisions. They are OWED WORK, and
  each entry carries the pointer that owns it.
- Not that the curated surface is all of `pncad`. `crate::workspace`,
  `crate::authoring`, `crate::guide`, `crate::export`, `crate::profile`
  and `crate::tolerance` are outside the census; the Rust guard reads
  all ten façade files, this one reads the three that curate the
  document layer and the common surface. `workspace::Workspace`,
  `random_document_id` and `update_to_store` are therefore NOT counted
  here even though they are part of gap G15 — the audit page's
  `test_the_named_gaps_are_still_gaps` is what watches those.
- Not that a bound name is bound WELL. Coverage, not quality.
"""

import ast
import unittest
from pathlib import Path

REPO = Path(__file__).resolve().parents[3]
FACADE = REPO / "crates" / "pncad" / "src"
FACADE_FILES = ("document.rs", "select.rs", "prelude.rs")
STUB = REPO / "crates" / "pncad-py" / "pncad.pyi"


# --- the Rust side ----------------------------------------------------


def code_without_comments(src):
    """`src` with `//` comments cut, tracking string literals.

    A transliteration of the Rust guard's function of the same name,
    and for the same reason: the façade files argue at length about
    types they deliberately do NOT export, and a scan that read prose
    would count `MeshPick` as carried because a comment says it is not.
    """
    out = []
    for line in src.splitlines():
        in_str = False
        cut = len(line)
        i = 0
        while i < len(line):
            ch = line[i]
            if ch == "\\" and in_str:
                i += 1
            elif ch == '"':
                in_str = not in_str
            elif ch == "/" and not in_str and line[i + 1 : i + 2] == "/":
                cut = i
                break
            i += 1
        out.append(line[:cut])
    return "\n".join(out)


def pub_use_names(src):
    """Every name a `pub use` statement of `src` introduces.

    No root restriction, unlike the Rust guard's `pub_use_names`: that
    one asks "which of `editor_core`'s names does this file carry",
    while this one asks "what can a consumer of these three modules
    name", and the answer includes the geometry crates the prelude
    lifts. The leaf of each path item is the name it introduces
    (`editor_core::persist::MigrationError` introduces
    `MigrationError`); a braced group introduces each of its items.
    """
    code = code_without_comments(src)
    names = set()
    rest = code
    while True:
        at = rest.find("pub use ")
        if at < 0:
            break
        rest = rest[at + len("pub use ") :]
        end = rest.find(";")
        if end < 0:
            break
        stmt = rest[:end].strip()
        rest = rest[end + 1 :]
        open_, close = stmt.find("{"), stmt.rfind("}")
        if 0 <= open_ < close:
            items = stmt[open_ + 1 : close]
        else:
            items = stmt.rsplit("::", 1)[-1]
        for item in items.split(","):
            item = item.strip()
            if item:
                names.add(item.rsplit("::", 1)[-1])
    return names


def curated_names():
    """The façade's curated surface, as the three lists spell it."""
    names = set()
    for stem in FACADE_FILES:
        names |= pub_use_names((FACADE / stem).read_text())
    return names


# --- the Python side --------------------------------------------------


def stub_surface():
    """`(top_level, members)` from `pncad.pyi`.

    `top_level` is what `test_stubs.py`'s `stub_names` collects, on the
    same rule (underscore names are the stub's private spelling
    machinery). `members` is every `Class.attribute` the stub declares
    — methods, properties and annotated attributes alike — which is the
    alphabet `BOUND_AS` spells its right-hand sides in.
    """
    tree = ast.parse(STUB.read_text())
    top, members = set(), set()
    for node in tree.body:
        if isinstance(node, (ast.ClassDef, ast.FunctionDef, ast.AsyncFunctionDef)):
            top.add(node.name)
        elif isinstance(node, ast.AnnAssign) and isinstance(node.target, ast.Name):
            top.add(node.target.id)
        elif isinstance(node, ast.Assign):
            for target in node.targets:
                if isinstance(target, ast.Name):
                    top.add(target.id)
        if isinstance(node, ast.ClassDef):
            for item in node.body:
                if isinstance(item, (ast.FunctionDef, ast.AsyncFunctionDef)):
                    members.add(f"{node.name}.{item.name}")
                elif isinstance(item, ast.AnnAssign) and isinstance(
                    item.target, ast.Name
                ):
                    members.add(f"{node.name}.{item.target.id}")
                elif isinstance(item, ast.Assign):
                    for target in item.targets:
                        if isinstance(target, ast.Name):
                            members.add(f"{node.name}.{target.id}")
    top = {n for n in top if not n.startswith("_") or n == "__build_info__"}
    return top, members


# --- the rosters ------------------------------------------------------

#: Curated names Python binds under a DIFFERENT spelling. Each value is
#: verified to exist in the stub, so an entry cannot wave a name away
#: with a method that is not there.
#:
#: Four spelling shifts account for all of it:
#:
#: - **A free function became a method on the value it takes.** The
#:   kernel's `validate*` / `mass_properties` are `Body`'s; the
#:   selector and materializer doors are `Evaluation`'s, which is the
#:   evaluation they answer "as of"; `apply` and `save` are `Doc`'s.
#: - **A kernel body operation became a recipe-node constructor.**
#:   Python speaks the document layer (the stub says so in its first
#:   paragraph), so `extrude`, `revolve`, `loft_body`, `fillet_edges`
#:   and `transform_rigid` arrive as `Node.*`, and the three Boolean
#:   verbs plus their `_with` siblings arrive as ONE `Node.boolean`
#:   taking a `BooleanOp` — the arm split moved from the verb to an
#:   argument.
#: - **A type became the door that reads it.** `DatumValue` is what
#:   `Value.datum` answers; `NodeValue` IS `Value` and
#:   `ValuePayload`'s discriminant is `Value.kind`; `NodeErrorKind`'s
#:   tag is `EvaluationError.kind`;
#:   `DocumentId` is the 32 hex digits `Doc.id` answers.
#: - **A rename.** `RecipeNodeId` is `NodeId` (the stub says what it is
#:   NOT: an arena key). The six unit constants are lower-cased —
#:   `IN` is `inch` because `in` is a Python keyword, a shift the stub
#:   comments on at the declaration.
BOUND_AS = {
    "CM": "cm",
    "DEG": "deg",
    "DatumValue": "Value.datum",
    "DocumentId": "Doc.id",
    "IN": "inch",
    "M": "m",
    "MM": "mm",
    "NodeErrorKind": "EvaluationError.kind",
    "NodeValue": "Value",
    "RAD": "rad",
    "RecipeNodeId": "NodeId",
    "ValuePayload": "Value.kind",
    "all_bodies": "Evaluation.all_bodies",
    "all_edges": "Evaluation.all_edges",
    "all_faces": "Evaluation.all_faces",
    "all_vertices": "Evaluation.all_vertices",
    "apply": "Doc.apply",
    "declare": "Doc.declare",
    "declare_all": "Doc.declare_all",
    "declare_node": "Node.declare",
    "extrude": "Node.extrude",
    "fillet_edges": "Node.fillet",
    "find_flush_candidates": "Evaluation.find_flush_candidates",
    "intersect": "Node.boolean",
    "intersect_with": "Node.boolean",
    "loft_body": "Node.loft",
    "mass_properties": "Body.mass_properties",
    "revolve": "Node.revolve",
    "save": "Doc.save",
    "select": "Evaluation.select",
    "select_where": "Evaluation.select_where",
    "step_string": "Evaluation.step_string",
    "subtract": "Node.boolean",
    "subtract_with": "Node.boolean",
    "transform_rigid": "Node.transform",
    "union": "Node.boolean",
    "union_with": "Node.boolean",
    "validate": "Body.validate",
    "validate_closed": "Body.validate_closed",
    "validate_geometric": "Body.validate_geometric",
}

# The family tags a NOT_BOUND entry may carry. A `gap:` entry names its
# pointer after the colon and is OWED WORK; the other two are the
# surface being a different shape in Python, not a debt.
SHAPE = "different-shape"
INTERIOR = "behind-a-door"
GAP = "gap"

#: Curated names with no Python spelling at all, by family.
#:
#: **`different-shape` — the curated face is a different shape in
#: Python, and binding the name would ADD ceremony rather than reach.**
#:
#: - *Values that cross as builtins.* `Point2`/`Point3`/`Vec2`/`Vec3`
#:   are `tuple[float, ...]` throughout the stub (`SketchPlane.u`,
#:   `Node.datum_axis`, `Node.transform`), `Real` is `float`, `Mat3`
#:   and `Affine3` never cross at all, and the `crate::authoring`
#:   constructors that exist so a Rust literal needs no `from_f64`
#:   (`p2`, `p3`, `v2`, `v3`, `real`) have nothing to construct.
#:   `Tol`/`Tolerance` cross as the `float` epsilon
#:   `DocEdit.set_tolerance` takes and `Doc.epsilon` reads.
#: - *Names, which are opaque text.* `StableName` is `str` by the
#:   ordinal-28 contract the stub restates on every materializer: the
#:   supported operations are equality, ordering, storage and handing
#:   it back. `RolePath` and `RoleSeg` are the INSIDE of that text,
#:   which nothing user-side may read (`Selector` and `NamePat`, both
#:   bound, are how a name is classified without reading one);
#:   `EdgeKey`/`FaceKey`/`VertexKey` are the arena keys the whole
#:   curation exists to keep unnameable, and they reached the census
#:   only because the prelude lifts `topo` wholesale.
#: - *Selector plumbing the audit already ruled on.* `TagPat` and
#:   `Side` are Rust constructor plumbing covered by `SegPat.tag` /
#:   `SegPat.group` / `SegPat.side`, and `CurveKindSet` /
#:   `SurfaceKindSet` / `ALL_SURFACE_KINDS` cross as
#:   `kind | list[kind]` arguments to `GeomPred.curve_kind` and
#:   `GeomPred.surface_kind`. That is the "deliberately NOT bound,
#:   stated" clause of the audit's G13 row, restated here so the
#:   census does not re-open it.
#: - *Refusal payloads flattened to a tag.* Python's exceptions carry
#:   their refusal as ATTRIBUTES, but the arm is a `variant`/`kind`
#:   string rather than a bound payload class, so the Rust arm types
#:   have no Python name: `PersistError.variant` stands for
#:   `SnapshotError`, `NonFiniteSite`, `ProgramFault` and
#:   `MigrationError`; `EvaluationError` for `NodeError`,
#:   `BooleanError`, `TransformError` and the sweep/loft/fillet/
#:   revolve refusals; `PathError` for `ProfileError` and
#:   `RecordedProgramError`; `ValidationError.door` for
#:   `MassPropsError`; `ExportError` for `StepExportError`;
#:   `SelectRefusal` for `DeclareError` and `InterrogateError`.
#: - *An option struct that became keyword arguments.* `StepOptions` is
#:   `Evaluation.step_string`'s `product_name=`; `ImportOptions` is
#:   `import_step`'s absent second argument; `EvalOptions` has no
#:   Python spelling because `evaluate(doc)` takes none (its one
#:   consequence that IS owed work — the memo — is filed under `gap`
#:   below, not here).
#: - *Recourse and deferral sentences.* `CONTACT_RECOURSE`,
#:   `FIT_DEFERRAL`, `SEL_DATUM_DISTANCE`, `REGENERATE_RECOURSE`,
#:   `UNDER_RECOURSE` and `CLASS_DEFERRAL` are the prose a Rust refusal
#:   cites; Python's refusals carry theirs in the exception's message.
#:   `SCHEMA_VERSION` is the same shape of constant on the persistence
#:   door, which Python reaches only through `load`.
#: - *Structures Python's authoring surface replaces with its own.*
#:   `Applied` and `EditRecord` are `apply`'s pair, and `Doc.apply`
#:   mutates in place and answers `Optional[NodeId]`, so there is no
#:   pair to name. `PartialPath` is the Rust lattice's one type where
#:   Python has one CLASS PER STATE (`PathOpen`, `PathPoint`,
#:   `PathDirectedPoint`, …), which is §L4's typestate translation.
#:   `LineTarget` and `TangentArcTarget` are absorbed into the verbs
#:   that take them, and `bulge_from_center`/`bulge_from_via` into the
#:   `Center`/`Via` spec modes that are bound. `Dimension` is what
#:   `DocParam.length`/`angle`/`count`/`scalar` choose between;
#:   `SlotId` is what `DocEdit.bind_count_param` names implicitly;
#:   `ProfileDoc` is the alias `Node.profile` builds from loops;
#:   `SplitSide` is the position in `Value.split`'s tuple.
#: - *A write sink Python does not need.* `write_step` takes a Rust
#:   `Write`; `Evaluation.step_string` answers the text and Python
#:   writes it. (`write_ascii`/`write_binary` are NOT here — STL is a
#:   gap, below, and the sink is not what is missing.)
#:
#: **`behind-a-door` — kernel machinery a bound door uses and never
#: hands to Python.** The operation results and their geometry
#: (`Extruded`, `Extrusion`, `Revolved`, `Revolution`, `Lofted`,
#: `Filleted`, `BooleanBody`, `BooleanResult`, `BooleanResultKind`,
#: `Operand`, `Curve3`, `Surface`, `EdgeGeometry`, `PropsQuadLane`):
#: the document layer consumes them and Python receives a `Value`. The
#: profile ladder's rungs (`Profile`, `ProfileLoop`, `ProfileVertex`,
#: `ValidatedLoop`, `ValidatedProfile`, `SegmentKind`,
#: `FilletLegShape`, `validated`) and the recorded program the node
#: stores (`ProfileProgram`, `LoopProgram`, `ProgramStep`,
#: `ProgramTarget`, `ProgramArcData`, `StepArg`): the PATHS lattice
#: hands `Node.profile` a `ClosedLoop` and the program is built inside
#: Rust, so no Python value is ever one of these. The naming table and
#: its provenance payloads (`NameTable`, `DuplicateName`,
#: `ProfileEdgeRef`, `ProfileVertexRef`, `edge_name`, `face_name` —
#: key-to-name inversions with no key to invert), the declared-contact
#: interior behind `FlushFinding` (`DeclaredContact`, `ContactVerdict`,
#: `ContactRefusal`, `FlushEvidence`, `ContactRecords`,
#: `BooleanDeclarations`), the fillet's coincidence band
#: (`Band`, `BandError`, built from the run's epsilon), and
#: `ContentBits` — a TRAIT, and Python has no way to spell one.
#:
#: **`gap` — genuinely unbound doors, and each is OWED WORK.** This is
#: the family that makes the census worth having: these are not
#: decisions, they are debt, and the pointer after the colon says who
#: owns each. Four have gap ids on
#: `docs/guide/north-star-audit.md`; the rest are register category B
#: work (`docs/LIB-LOG.md`, "bindings-parity residuals") that the
#: audit's SCENE-driven gap list does not reach, because no tour scene
#: exercises them — which is exactly why they accumulated unnoticed and
#: why this census exists.
#:
#: - **G2 — sweep and tube.** `sweep_body`, `tube_along_arc`,
#:   `tube_along_arc_hollow`, `TubeError`, `TubeWindow`. Banked, not
#:   merely unbound: `wire_sweep` refuses unconditionally and
#:   `Node::Tube` does not exist (a schema-version break).
#: - **G8 — the memo.** `EvalOptions`' consequence: `evaluate(doc)`
#:   takes no prior evaluation, so memoized recompute is unobservable
#:   from Python. The audit's G8 row measures this residue by name.
#: - **G11 — tessellation and STL.** `Mesh`, `tessellate`,
#:   `TessellateError`, and the STL writers with their options and
#:   header vocabulary. Python loses steps 4 and 5 of the guide's
#:   ladder, so there is no mesh-vs-exact cross-check.
#: - **G15 — content pins and cross-document references.**
#:   `ContentPin`, `content_pin`, `canonical_bytes`, `DocRef`,
#:   `header_document_id`, and the pin-update door
#:   (`update_references`, `UpdateError`, `mixed_pins`,
#:   `PinMultiplicity`, `PinSites`). The audit's G15 row already
#:   watches the `workspace::` half; this is the `document::` half, in
#:   the census the audit's absence test cannot see.
#: - **Assembly, the at-rest gate (A5).** `assemble`, `Assembly`,
#:   `AssemblyError`, `AtRestFinding`, `Attribution`,
#:   `MintedDeclaration`, `RefusedRef`. The façade carried these
#:   BECAUSE a consumer could build an assembly and not check it; the
#:   same argument applies one layer out, and Python cannot check one
#:   at all. No gap id owns it today.
#: - **Mates and the solve.** `Alignment`, `MateFrame`,
#:   `MatePrimitive`, `MateRole`, `MateSide`, `AxisSense`,
#:   `SolvedPoses`, `Subgroup`, `MateFault`, `ClusterMaintenance`,
#:   `clusters`, `gauge_of`, `reading_edges`,
#:   `relative_freedom_components`, `solve_document` — plus the
#:   admission table a tool must read BEFORE committing
#:   (`ClassAdmission`, `class_admission`). Python can author no mate,
#:   so it cannot reach the assembly gate above even if that were
#:   bound. No gap id.
#: - **Instantiated parts.** `PartResolver`, `PartFault`,
#:   `ResolveFailure`, `ResolveFault`, `PlacementRuleFault` — the
#:   document seam evaluation crosses to reach a referenced document.
#:   G15's neighbour and the reason its row says a Python author "can
#:   produce two documents a workspace will accept side by side, and
#:   cannot then assemble them". No gap id of its own.
#: - **Split and inline, the recorded refactorings.** `split`,
#:   `inline`, `SplitOutcome`, `InlineOutcome`, `SplitError`,
#:   `InlineError`, `NodeMap`, `InterfaceRecord`, `InterfaceCrossing`.
#:   NOTE the collision: this `split` is the document refactoring, NOT
#:   the geometry `Node.split` Python binds. A looser mapping rule
#:   would have matched them and hidden the gap. No gap id.
#: - **Explicit product roots.** `product`, `product_named`,
#:   `ProductError`, `RootFault`. `Doc` has no `roots` reader and
#:   `DocEdit` no `SetRoots`, so Python cannot say what a document's
#:   product IS. No gap id.
#: - **The advisory checks (DISCIPLINES-DESIGN DS6).** `run_checks`,
#:   `enforce_checks`, `subject_body`, `ChecksReport`, `ChecksConfig`,
#:   `ChecksError`, `CheckFinding`, `CheckEvidence`, `CheckId`,
#:   `CheckKind`, `CheckRefusal`, `Severity`. The report-never-gate
#:   registry. No gap id.
#: - **Picking.** `pick_face`, `PickTarget`, `PickHit`, `NodePick`,
#:   `NodePickError`, `HitTestError`, `Ray`. The fourth door onto a
#:   name — ray in, `StableName` out, the same alphabet
#:   `Evaluation.select` speaks. No gap id.
#: - **Name resolution across re-evaluation.** `resolve`, `Resolution`,
#:   `RunCtx`. The question a consumer that STORES names must ask on
#:   every run — and Python's whole selection story is store-then-reuse
#:   (`Node.fillet` freezes a name set), so the absence bites exactly
#:   the consumers the stub tells to store. No gap id.
#: - **The expression surface.** `Expr`, `ParamEnv`, `parse_expr`,
#:   `ParseError`, and its read side `eval`, `eval_count`, `EvalError`.
#:   The audit's G1/G10 residue names the authoring half ("a profile
#:   step whose argument is an EXPRESSION rather than a literal" — the
#:   one door still blocking `plate_param` from scratch, and the same
#:   door `GeomPred.datum_distance`'s comparand waits on). The READ
#:   half is unnamed anywhere: Python holds `DocParam` values and has
#:   no door from an expression to its value.
#: - **The geometry read-back doors.** `face_frame`, `edge_frame`,
#:   `vertex_position`, `denotation`, `Denotation`, and the `Pose` /
#:   `ReadbackError` they answer in. `crate::select`'s third invariant
#:   — "a name answers with values, never keys" — has no Python face:
#:   `Evaluation.all_faces` hands back names and nothing asks one where
#:   it is. No gap id.
#: - **The fourth validator rung.** `validate_pseudomanifold`. `Body`
#:   binds three of the ladder's four; this one is simply missing. No
#:   gap id.
#: - **Chamfer.** `chamfer_edges`, `Chamfered`. The fillet's ruled
#:   sibling, and the reason it cannot be bound the way `Node.fillet`
#:   was is one level down: `editor-core` has no `Chamfer` node, so
#:   this is a document-layer unit before it is a binding one.
#: - **Cooperative cancellation.** `CancelToken`. `evaluate(doc)` takes
#:   none, so a Python caller cannot stop a long evaluation.
#: - **The D6 display formatter.** `fmt_length`, `fmt_angle`,
#:   `FmtQuantityError`. `Length.in_unit` answers a bare float;
#:   choosing digits and a symbol is the formatter's job and Python
#:   redoes it by hand. No gap id.
NOT_BOUND = {
    # --- different-shape ------------------------------------------
    "ALL_SURFACE_KINDS": SHAPE,
    "Affine3": SHAPE,
    "Applied": SHAPE,
    "Axis3": SHAPE,
    "BooleanError": SHAPE,
    "CLASS_DEFERRAL": SHAPE,
    "CONTACT_RECOURSE": SHAPE,
    "CurveKindSet": SHAPE,
    "DeclareError": SHAPE,
    "Dimension": SHAPE,
    "EdgeKey": SHAPE,
    "EditRecord": SHAPE,
    "EvalOutcome": SHAPE,
    "FIT_DEFERRAL": SHAPE,
    "FaceKey": SHAPE,
    "ExtrudeError": SHAPE,
    "FilletError": SHAPE,
    "ImportOptions": SHAPE,
    "InterrogateError": SHAPE,
    "LineTarget": SHAPE,
    "LoftError": SHAPE,
    "Mat3": SHAPE,
    "MassPropsError": SHAPE,
    "MigrationError": SHAPE,
    "NodeError": SHAPE,
    "NodeResult": SHAPE,
    "NonFiniteSite": SHAPE,
    "PartialPath": SHAPE,
    "PathNoCornerReason": SHAPE,
    "Point2": SHAPE,
    "Point3": SHAPE,
    "ProfileDoc": SHAPE,
    "ProfileError": SHAPE,
    "ProgramFault": SHAPE,
    "REGENERATE_RECOURSE": SHAPE,
    "Real": SHAPE,
    "RecordedProgramError": SHAPE,
    "RevolveAxis": SHAPE,
    "RevolveError": SHAPE,
    "RolePath": SHAPE,
    "RoleSeg": SHAPE,
    "SCHEMA_VERSION": SHAPE,
    "SEL_DATUM_DISTANCE": SHAPE,
    "Side": SHAPE,
    "SlotId": SHAPE,
    "SnapshotError": SHAPE,
    "SplitSide": SHAPE,
    "StableName": SHAPE,
    "StepExportError": SHAPE,
    "StepOptions": SHAPE,
    "SurfaceKindSet": SHAPE,
    "TagPat": SHAPE,
    "TangentArcTarget": SHAPE,
    "Tol": SHAPE,
    "Tolerance": SHAPE,
    "TransformError": SHAPE,
    "UNDER_RECOURSE": SHAPE,
    "Vec2": SHAPE,
    "Vec3": SHAPE,
    "VertexKey": SHAPE,
    "bulge_from_center": SHAPE,
    "bulge_from_via": SHAPE,
    "p2": SHAPE,
    "p3": SHAPE,
    "real": SHAPE,
    "v2": SHAPE,
    "v3": SHAPE,
    "write_step": SHAPE,
    # --- behind-a-door --------------------------------------------
    "Band": INTERIOR,
    "BandError": INTERIOR,
    "BooleanBody": INTERIOR,
    "BooleanDeclarations": INTERIOR,
    "BooleanResult": INTERIOR,
    "BooleanResultKind": INTERIOR,
    "BooleanValue": INTERIOR,
    "ContactRecords": INTERIOR,
    "ContactRefusal": INTERIOR,
    "ContactVerdict": INTERIOR,
    "ContentBits": INTERIOR,
    "Curve3": INTERIOR,
    "DeclaredContact": INTERIOR,
    "DuplicateName": INTERIOR,
    "EdgeGeometry": INTERIOR,
    "Extruded": INTERIOR,
    "Extrusion": INTERIOR,
    "FilletLegShape": INTERIOR,
    "Filleted": INTERIOR,
    "FlushEvidence": INTERIOR,
    "Lofted": INTERIOR,
    "LoopProgram": INTERIOR,
    "NameTable": INTERIOR,
    "Operand": INTERIOR,
    "Profile": INTERIOR,
    "ProfileEdgeRef": INTERIOR,
    "ProfileLoop": INTERIOR,
    "ProfileProgram": INTERIOR,
    "ProfileVertex": INTERIOR,
    "ProfileVertexRef": INTERIOR,
    "ProgramArcData": INTERIOR,
    "ProgramStep": INTERIOR,
    "ProgramTarget": INTERIOR,
    "PropsQuadLane": INTERIOR,
    "Revolution": INTERIOR,
    "Revolved": INTERIOR,
    "SegmentKind": INTERIOR,
    "StepArg": INTERIOR,
    "Surface": INTERIOR,
    "ValidatedLoop": INTERIOR,
    "ValidatedProfile": INTERIOR,
    "edge_name": INTERIOR,
    "face_name": INTERIOR,
    "validated": INTERIOR,
    # --- gap: sweep and tube (audit G2) ---------------------------
    "TubeError": f"{GAP}: G2 sweep/tube",
    "TubeWindow": f"{GAP}: G2 sweep/tube",
    "sweep_body": f"{GAP}: G2 sweep/tube",
    "tube_along_arc": f"{GAP}: G2 sweep/tube",
    "tube_along_arc_hollow": f"{GAP}: G2 sweep/tube",
    # --- gap: the memo (audit G8's measured residue) --------------
    "EvalOptions": f"{GAP}: G8 memoized recompute",
    # --- gap: tessellation and STL (audit G11) --------------------
    "AsciiOptions": f"{GAP}: G11 tessellation/STL",
    "BinaryHeader": f"{GAP}: G11 tessellation/STL",
    "BinaryHeaderError": f"{GAP}: G11 tessellation/STL",
    "BinaryOptions": f"{GAP}: G11 tessellation/STL",
    "Mesh": f"{GAP}: G11 tessellation/STL",
    "SolidName": f"{GAP}: G11 tessellation/STL",
    "SolidNameError": f"{GAP}: G11 tessellation/STL",
    "TessellateError": f"{GAP}: G11 tessellation/STL",
    "tessellate": f"{GAP}: G11 tessellation/STL",
    "write_ascii": f"{GAP}: G11 tessellation/STL",
    "write_binary": f"{GAP}: G11 tessellation/STL",
    # --- gap: content pins and cross-document refs (audit G15) ----
    "ContentPin": f"{GAP}: G15 content pins",
    "DocRef": f"{GAP}: G15 content pins",
    "PinMultiplicity": f"{GAP}: G15 content pins",
    "PinSites": f"{GAP}: G15 content pins",
    "UpdateError": f"{GAP}: G15 content pins",
    "canonical_bytes": f"{GAP}: G15 content pins",
    "content_pin": f"{GAP}: G15 content pins",
    "header_document_id": f"{GAP}: G15 content pins",
    "mixed_pins": f"{GAP}: G15 content pins",
    "update_references": f"{GAP}: G15 content pins",
    # --- gap: assembly at-rest gate (register B, no gap id) -------
    "Assembly": f"{GAP}: register B assembly gate",
    "AssemblyError": f"{GAP}: register B assembly gate",
    "AtRestFinding": f"{GAP}: register B assembly gate",
    "Attribution": f"{GAP}: register B assembly gate",
    "MintedDeclaration": f"{GAP}: register B assembly gate",
    "RefusedRef": f"{GAP}: register B assembly gate",
    "assemble": f"{GAP}: register B assembly gate",
    # --- gap: mates and the solve (register B, no gap id) ---------
    "Alignment": f"{GAP}: register B mates",
    "AxisSense": f"{GAP}: register B mates",
    "ClassAdmission": f"{GAP}: register B mates",
    "ClusterMaintenance": f"{GAP}: register B mates",
    "MateFault": f"{GAP}: register B mates",
    "MateFrame": f"{GAP}: register B mates",
    "MatePrimitive": f"{GAP}: register B mates",
    "MateRole": f"{GAP}: register B mates",
    "MateSide": f"{GAP}: register B mates",
    "SolvedPoses": f"{GAP}: register B mates",
    "Subgroup": f"{GAP}: register B mates",
    "class_admission": f"{GAP}: register B mates",
    "clusters": f"{GAP}: register B mates",
    "gauge_of": f"{GAP}: register B mates",
    "reading_edges": f"{GAP}: register B mates",
    "relative_freedom_components": f"{GAP}: register B mates",
    "solve_document": f"{GAP}: register B mates",
    # --- gap: instantiated parts (register B, G15's neighbour) ----
    "PartFault": f"{GAP}: register B instantiate-part",
    "PartResolver": f"{GAP}: register B instantiate-part",
    "PlacementRuleFault": f"{GAP}: register B instantiate-part",
    "ResolveFailure": f"{GAP}: register B instantiate-part",
    "ResolveFault": f"{GAP}: register B instantiate-part",
    # --- gap: split/inline refactorings (register B, no gap id) ---
    "InlineError": f"{GAP}: register B split/inline",
    "InlineOutcome": f"{GAP}: register B split/inline",
    "InterfaceCrossing": f"{GAP}: register B split/inline",
    "InterfaceRecord": f"{GAP}: register B split/inline",
    "NodeMap": f"{GAP}: register B split/inline",
    "SplitError": f"{GAP}: register B split/inline",
    "SplitOutcome": f"{GAP}: register B split/inline",
    "inline": f"{GAP}: register B split/inline",
    "split": f"{GAP}: register B split/inline",
    # --- gap: explicit product roots (register B, no gap id) ------
    "ProductError": f"{GAP}: register B product roots",
    "RootFault": f"{GAP}: register B product roots",
    "product": f"{GAP}: register B product roots",
    "product_named": f"{GAP}: register B product roots",
    # --- gap: advisory checks DS6 (register B, no gap id) ---------
    "CheckEvidence": f"{GAP}: register B checks",
    "CheckFinding": f"{GAP}: register B checks",
    "CheckId": f"{GAP}: register B checks",
    "CheckKind": f"{GAP}: register B checks",
    "CheckRefusal": f"{GAP}: register B checks",
    "ChecksConfig": f"{GAP}: register B checks",
    "ChecksError": f"{GAP}: register B checks",
    "ChecksReport": f"{GAP}: register B checks",
    "Severity": f"{GAP}: register B checks",
    "enforce_checks": f"{GAP}: register B checks",
    "run_checks": f"{GAP}: register B checks",
    "subject_body": f"{GAP}: register B checks",
    # --- gap: picking (register B, no gap id) ---------------------
    "HitTestError": f"{GAP}: register B picking",
    "NodePick": f"{GAP}: register B picking",
    "NodePickError": f"{GAP}: register B picking",
    "PickHit": f"{GAP}: register B picking",
    "PickTarget": f"{GAP}: register B picking",
    "Ray": f"{GAP}: register B picking",
    "pick_face": f"{GAP}: register B picking",
    # --- gap: name resolution (register B, no gap id) -------------
    "Resolution": f"{GAP}: register B name resolution",
    "RunCtx": f"{GAP}: register B name resolution",
    "resolve": f"{GAP}: register B name resolution",
    # --- gap: the expression surface (audit G1/G10 residue) -------
    "EvalError": f"{GAP}: G1 residue expressions",
    "Expr": f"{GAP}: G1 residue expressions",
    "ParamEnv": f"{GAP}: G1 residue expressions",
    "ParseError": f"{GAP}: G1 residue expressions",
    "eval": f"{GAP}: G1 residue expressions",
    "eval_count": f"{GAP}: G1 residue expressions",
    "parse_expr": f"{GAP}: G1 residue expressions",
    # --- gap: geometry read-back doors (register B, no gap id) ----
    "Denotation": f"{GAP}: register B read-back doors",
    "Pose": f"{GAP}: register B read-back doors",
    "ReadbackError": f"{GAP}: register B read-back doors",
    "denotation": f"{GAP}: register B read-back doors",
    "edge_frame": f"{GAP}: register B read-back doors",
    "face_frame": f"{GAP}: register B read-back doors",
    "vertex_position": f"{GAP}: register B read-back doors",
    # --- gap: assorted single doors (register B, no gap id) -------
    "CancelToken": f"{GAP}: register B cancellation",
    "Chamfered": f"{GAP}: register B chamfer",
    "FmtQuantityError": f"{GAP}: register B quantity formatter",
    "chamfer_edges": f"{GAP}: register B chamfer",
    "fmt_angle": f"{GAP}: register B quantity formatter",
    "fmt_length": f"{GAP}: register B quantity formatter",
    "validate_pseudomanifold": f"{GAP}: register B fourth validator",
}


class TestBindingCensus(unittest.TestCase):
    def setUp(self):
        self.curated = curated_names()
        self.top, self.members = stub_surface()

    def test_the_census_is_not_vacuous(self):
        """Floors on both scanners, picked by measurement.

        The Rust guard asserts `exported.len() > 150` for the same
        reason: a scanner that returned nothing would satisfy every
        set difference below and the guard would pass having read
        nothing. Measured at the time of writing: 323 curated names,
        86 top-level stub names, 332 `Class.member` spellings. The
        floors sit below those with room for ordinary shrinkage and
        far above zero.
        """
        self.assertGreater(
            len(self.curated), 300, "the façade's three lists shrank drastically"
        )
        self.assertGreater(
            len(self.top), 75, "the stub scanner found almost nothing"
        )
        self.assertGreater(
            len(self.members), 250, "the member scanner found almost nothing"
        )

    def test_every_bound_as_spelling_exists_in_the_stub(self):
        """A mapping to a spelling the stub does not declare is a
        claim nobody is checking — the failure mode that would make
        this whole roster decorative."""
        absent = sorted(
            f"{name} -> {spelling}"
            for name, spelling in BOUND_AS.items()
            if spelling not in self.top and spelling not in self.members
        )
        self.assertEqual(
            absent,
            [],
            "BOUND_AS names Python spellings pncad.pyi does not declare",
        )

    def test_the_two_rosters_are_disjoint(self):
        overlap = sorted(set(BOUND_AS) & set(NOT_BOUND))
        self.assertEqual(overlap, [], "a name cannot be both bound and unbound")

    def test_every_not_bound_family_is_one_of_the_three(self):
        bad = sorted(
            f"{name}: {family}"
            for name, family in NOT_BOUND.items()
            if family not in (SHAPE, INTERIOR)
            and not family.startswith(f"{GAP}: ")
        )
        self.assertEqual(
            bad,
            [],
            f"a NOT_BOUND family must be {SHAPE!r}, {INTERIOR!r}, "
            f"or '{GAP}: <pointer>'",
        )

    def test_every_curated_name_is_bound_or_listed(self):
        """**The obligation, mechanical.**

        A door curated into the Rust façade and never spelled in
        Python fails here, at the moment it is curated, naming itself
        — which is what nothing was doing when the assembly, checks,
        picking, expression-read and content-pin families accumulated.
        """
        unaccounted = sorted(
            n
            for n in self.curated
            if n not in self.top and n not in BOUND_AS and n not in NOT_BOUND
        )
        self.assertEqual(
            unaccounted,
            [],
            f"{len(unaccounted)} curated façade name(s) are neither bound in "
            "Python nor listed: bind each, or add it to BOUND_AS with the "
            "Python spelling that answers the same question, or to NOT_BOUND "
            "with the family it belongs to (a 'gap:' entry must name the "
            "pointer that owns the work).",
        )

    def test_the_rosters_decay(self):
        """Both directions, exactly as the Rust guard's stale check.

        A roster entry that is no longer a curated façade name, a
        `NOT_BOUND` entry Python has since started binding, or a
        `BOUND_AS` entry whose curated name Python now spells
        identically — each is a stale exclusion claiming a decision
        nobody is making.
        """
        stale = sorted(
            f"{n} (not a curated façade name)"
            for n in set(BOUND_AS) | set(NOT_BOUND)
            if n not in self.curated
        )
        stale += sorted(
            f"{n} (Python binds it top-level now)"
            for n in NOT_BOUND
            if n in self.top
        )
        stale += sorted(
            f"{n} (Python spells it identically now; drop the mapping)"
            for n in BOUND_AS
            if n in self.top
        )
        self.assertEqual(stale, [], "stale roster entries — remove them")


if __name__ == "__main__":
    unittest.main()
