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

THE SURFACE-DEBT ID SPACE, WHICH THIS CENSUS OWNS
-------------------------------------------------
Every `gap:` entry names exactly ONE id, in the first position after
the colon: `gap: <ID> <free prose>`. Two id spaces meet there, and the
rule between them is the point of this section.

`docs/guide/north-star-audit.md` asks a SCENE question — can a user
reproduce this model? — so its `G##` ids are anchored to tour stops
and its stop tallies only mean something because every id is anchored
that way. This census asks a SURFACE question: can a user reach this
door? Debt no tour scene exercises therefore has no `G##` id, and
minting one would make the audit's tallies measure two different
things at once.

So: **where an audit gap id exists the census CITES it; where none
does, the census's family tag IS the id and the census owns it**
(`FAMILIES`, below). Both halves are checked mechanically —
`test_every_gap_entry_names_a_defined_id` reads the audit page's own
gap tables and fails on a citation the page does not define, and on a
family tag `FAMILIES` does not charter. `docs/LIB-LOG.md`'s residual
register, category B, points here for the enumeration rather than
carrying one in prose.

WHAT THIS DOES NOT CLAIM
------------------------
- Not that each `NOT_BOUND` entry is individually argued. They are
  argued BY FAMILY, in that constant's docstring, the way `NOT_CARRIED`
  argues its own.
- Not that the `gap:` families are decisions. They are OWED WORK, and
  each entry names the id that owns it.
- Not that a cited id is the RIGHT owner. The cross-doc check asks
  only whether the pointer RESOLVES — that `G18` is a gap the audit
  page defines, that `B-CHECKS` is a family this file charters. Which
  id owns which door is a judgement made by hand, at the entry.
- Not that the audit page's ids are all readable from here. The
  extraction reads TABLE ROWS whose first cell is `G` + digits, in the
  two sections that define ids; a gap the page names only in prose (the
  closed list's `G2's loft half` row, the `G1`/`G10` residue
  cross-references) is invisible to it. That is the intended reading:
  an id worth citing is one a reader can look UP.
- Not that the curated surface is all of `pncad`. `crate::workspace`,
  `crate::authoring`, `crate::guide`, `crate::export`, `crate::profile`
  and `crate::tolerance` are outside the census; the Rust guard reads
  all ten façade files, this one reads the three that curate the
  document layer and the common surface. `workspace::Workspace`,
  `random_document_id` and `update_to_store` are therefore NOT counted
  here — the audit page's `test_the_named_gaps_are_still_gaps` is what
  watches those, and it is where the first two landing and the third
  not is recorded.
- Not that a bound name is bound WELL. Coverage, not quality.
"""

import ast
import unittest
from pathlib import Path

REPO = Path(__file__).resolve().parents[3]
FACADE = REPO / "crates" / "pncad" / "src"
FACADE_FILES = ("document.rs", "select.rs", "prelude.rs")
STUB = REPO / "crates" / "pncad-py" / "pncad.pyi"
AUDIT = REPO / "docs" / "guide" / "north-star-audit.md"


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


# --- the audit page's gap ids -----------------------------------------

#: The `## `-level sections of `docs/guide/north-star-audit.md` that
#: DEFINE gap ids, one row per gap. Scoping the read to these two is
#: what keeps a row of the audit's scene table — whose first cell is a
#: row NUMBER, and whose fourth cell CITES a gap — from being read as
#: a definition of one.
GAP_SECTIONS = ("\n## The gap list\n", "\n## Closed gaps\n")


def markdown_section(page, heading):
    """A `## ` section's body: after the heading, up to the next one.

    The Rust guards that parse this same page
    (`crates/pncad/tests/all.rs::the_north_star_audit_has_a_row_for_
    every_tour_stop` and its tally sibling) carry a function of this
    name and this rule; this is it in Python, so the two read the page
    the same way. `\\n## ` needs the trailing space, so a `### `
    subheading stays INSIDE its section.
    """
    at = page.find(heading)
    if at < 0:
        raise AssertionError(f"the audit page has no `{heading.strip()}` heading")
    rest = page[at + len(heading) :]
    end = rest.find("\n## ")
    return rest if end < 0 else rest[:end]


def table_cells(line):
    """One Markdown row's cells, or `None` for a non-row.

    The Rust guards' rule again: a line that does not start with `|`
    is not a row, and the `|---|` separator is not one either.
    """
    t = line.strip()
    if not t.startswith("|"):
        return None
    cells = [c.strip() for c in t.strip("|").split("|")]
    if all(c and set(c) <= set("-:") for c in cells):
        return None
    return cells


def audit_gap_ids():
    """Every gap id `docs/guide/north-star-audit.md` DEFINES.

    A definition is a row of one of the two id-defining sections whose
    FIRST cell is `G` + digits — the shape the Rust tally guard uses to
    tell a gap row from the prose and headers around it. The open list
    and the closed list are read alike: a closed gap keeps its id, and
    an entry citing one (`G1`'s Expr residue, `G16`'s chamfer node) is
    citing a row that is still there to be read.

    What this cannot see is stated in the module docstring: a gap named
    only in prose, and the closed list's `G2's loft half` row, whose
    first cell is not a bare id.
    """
    page = AUDIT.read_text()
    ids = set()
    for heading in GAP_SECTIONS:
        for line in markdown_section(page, heading).splitlines():
            cells = table_cells(line)
            if not cells:
                continue
            first = cells[0]
            if first.startswith("G") and first[1:].isdigit():
                ids.add(first)
    return ids


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

# The family tags a NOT_BOUND entry may carry. A `gap:` entry names the
# id that owns it after the colon and is OWED WORK; the other two are
# the surface being a different shape in Python, not a debt.
SHAPE = "different-shape"
INTERIOR = "behind-a-door"
GAP = "gap"

#: The surface-debt ids this census OWNS, each with its charter.
#:
#: These are the families no audit gap id reaches, because no tour
#: scene exercises them — the census's own founding finding, and the
#: reason the module docstring's id-space section splits the two
#: spaces the way it does. Where the audit page DOES define an id, an
#: entry cites that instead and nothing is minted here: `G2` (sweep
#: and tube), `G8` (the memo), `G11` (tessellation and STL),
#: `G16` (chamfer's missing recipe node), `G18` (the
#: whole Python assembly series, whose row enumerates `assemble`,
#: `solve_document`, `product`, `split` and `inline` by name), and
#: `G1` for the Expr-in-a-profile-step residue its row records.
#:
#: **The spelling.** `B-` is the register category these entries used
#: to point at in prose — `docs/LIB-LOG.md`, "LIB residual register",
#: category B — which now points HERE for its enumeration, so the
#: lineage stays legible in the id itself and a reader arriving from
#: either document lands in the same place. Upper case with a hyphen
#: makes an id unmistakable for an audit `G##`, for a Python
#: identifier, or for the prose that follows it; no whitespace, so
#: `gap: <ID> <prose>` parses by splitting once.
#:
#: **What a charter is.** One line: the door that is missing, and what
#: a unit closing it would have to DELIVER. Not a plan, not a
#: schedule, and not a claim that the unit is small — sizing is the
#: brief's job, not the census's. What the charter buys is that a
#: dispatcher reading an id knows what closing it means, which is
#: exactly what "register B" as a prose paragraph did not give them.
FAMILIES = {
    "B-CHECKS": (
        "the advisory report-never-gate checks registry "
        "(DISCIPLINES-DESIGN DS6); closing it binds `run_checks` / "
        "`enforce_checks` and the findings they report, with `CheckId` "
        "and `Severity` as values a Python caller can dispatch on"
    ),
    "B-PICKING": (
        "picking, the fourth door onto a name — ray in, `StableName` "
        "out; closing it binds `pick_face` with its ray/target/hit "
        "vocabulary, answering in the same opaque-text alphabet "
        "`Evaluation.select` speaks"
    ),
    "B-RESOLVE": (
        "name resolution across re-evaluation; closing it binds "
        "`resolve` and its `Resolution` verdict — the question every "
        "consumer that STORES names must ask on the next run, which is "
        "every consumer the stub tells to store one"
    ),
    "B-READBACK": (
        "the geometry read-back doors; closing it binds `face_frame` / "
        "`edge_frame` / `vertex_position` / `denotation` and the `Pose` "
        "they answer in, giving `crate::select`'s third invariant — a "
        "name answers with VALUES, never keys — its first Python face"
    ),
    "B-EXPR-READ": (
        "the expression READ side; closing it binds `eval` / "
        "`eval_count` and their refusal, so a Python caller holding a "
        "`DocParam` environment can ask an expression for its value "
        "(the authoring half is G1's recorded residue, not this)"
    ),
    "B-CANCEL": (
        "cooperative cancellation; closing it puts a `CancelToken` on "
        "the `evaluate(doc)` door, which today takes none — so a Python "
        "caller cannot stop a long evaluation at all"
    ),
    "B-FORMAT": (
        "the D6 display formatter; closing it binds `fmt_length` / "
        "`fmt_angle` and their refusal, so choosing digits and a symbol "
        "stops being hand-work Python redoes beside `Length.in_unit`'s "
        "bare float"
    ),
    "B-VALIDATE4": (
        "the fourth validator rung; closing it binds "
        "`validate_pseudomanifold` beside the three `Body` already "
        "carries — the ladder is bound three-quarters and this is the "
        "missing quarter"
    ),
}

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
#: `Operand`, `Curve3`, `Surface`, `EdgeDescription`, `PropsQuadLane`):
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
#: decisions, they are debt, and the id after the colon says what owns
#: each. Six of the ids are the audit page's, cited (`G1`, `G2`, `G8`,
#: `G11`, `G16`, `G18`); the other eight are `FAMILIES` keys
#: this census owns, because the audit's SCENE-driven list does not
#: reach a door no tour scene exercises — which is exactly why those
#: accumulated unnoticed and why this census exists.
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
#: - **G18 — the pin-update door.** `update_references`,
#:   `UpdateError`, `mixed_pins`, `PinMultiplicity`, `PinSites`.
#:   This entry USED to read G15 and cover the whole content-pin
#:   family; LIB-G15 bound the rest of it (`ContentPin`,
#:   `content_pin`, `canonical_bytes`, `DocRef`,
#:   `header_document_id`, and `workspace::Workspace` /
#:   `random_document_id` beside them), so what remains is only the
#:   door that moves a pin AT ITS SITES. A site is an
#:   `InstantiatePart` node's `DocRef`, Python can author none, and
#:   `evaluate(doc)` takes no resolver — so every one of these doors
#:   would answer the "referenced nowhere" refusal on any document
#:   Python can build. That is G18's dependency, not a residue of the
#:   closed row, and the citation moved with it.
#: - **G18 — assembly, the at-rest gate (A5).** `assemble`,
#:   `Assembly`, `AssemblyError`, `AtRestFinding`, `Attribution`,
#:   `MintedDeclaration`, `RefusedRef`. The façade carried these
#:   BECAUSE a consumer could build an assembly and not check it; the
#:   same argument applies one layer out, and Python cannot check one
#:   at all. The audit's G18 row names this half by name, down to the
#:   list of types, and records that #938's fix pass curated it — so
#:   the id exists and is cited rather than minted.
#: - **G18 — mates and the solve.** `Alignment`, `MateFrame`,
#:   `MatePrimitive`, `MateRole`, `MateSide`, `AxisSense`,
#:   `SolvedPoses`, `Subgroup`, `MateFault`, `ClusterMaintenance`,
#:   `clusters`, `gauge_of`, `reading_edges`,
#:   `relative_freedom_components`, `solve_document` — plus the
#:   admission table a tool must read BEFORE committing
#:   (`ClassAdmission`, `class_admission`). Python can author no mate,
#:   so it cannot reach the assembly gate above even if that were
#:   bound. G18's row names `mate`, its four frame types and
#:   `solve_document`; the admission table it does not name, and this
#:   entry is where that reaches the record.
#: - **G18 — instantiated parts.** `PartResolver`, `PartFault`,
#:   `ResolveFailure`, `ResolveFault`, `PlacementRuleFault` — the
#:   document seam evaluation crosses to reach a referenced document.
#:   What is left of the sentence G15's row used to own: a Python
#:   author can now produce two documents a workspace will accept
#:   side by side — LIB-G15 bound that half — and still cannot
#:   assemble them. G18's row puts this FIRST in the series' stated
#:   order: `evaluate(doc)` takes no resolver, so an
#:   `InstantiatePart` node cannot evaluate from Python at all. Note
#:   which side of the seam that is: `Workspace` IS the `PartResolver`
#:   implementation in Rust, and Python can now build one — what is
#:   missing is the parameter `evaluate` would take it through.
#: - **G18 — split and inline, the recorded refactorings.** `split`,
#:   `inline`, `SplitOutcome`, `InlineOutcome`, `SplitError`,
#:   `InlineError`, `NodeMap`, `InterfaceRecord`, `InterfaceCrossing`.
#:   NOTE the collision: this `split` is the document refactoring, NOT
#:   the geometry `Node.split` Python binds. A looser mapping rule
#:   would have matched them and hidden the gap. G18's row names both
#:   verbs in its list of what is absent.
#: - **G18 — explicit product roots.** `product`, `product_named`,
#:   `ProductError`, `RootFault`. `Doc` has no `roots` reader and
#:   `DocEdit` no `SetRoots`, so Python cannot say what a document's
#:   product IS — which is `set_roots` and `product`, two more of the
#:   names G18's row lists.
#: - **B-CHECKS — the advisory checks (DISCIPLINES-DESIGN DS6).**
#:   `run_checks`, `enforce_checks`, `subject_body`, `ChecksReport`,
#:   `ChecksConfig`, `ChecksError`, `CheckFinding`, `CheckEvidence`,
#:   `CheckId`, `CheckKind`, `CheckRefusal`, `Severity`. The
#:   report-never-gate registry, and the largest census-owned family.
#: - **B-PICKING — picking.** `pick_face`, `PickTarget`, `PickHit`,
#:   `NodePick`, `NodePickError`, `HitTestError`, `Ray`. The fourth
#:   door onto a name — ray in, `StableName` out, the same alphabet
#:   `Evaluation.select` speaks.
#: - **B-RESOLVE — name resolution across re-evaluation.** `resolve`,
#:   `Resolution`, `RunCtx`. The question a consumer that STORES names
#:   must ask on every run — and Python's whole selection story is
#:   store-then-reuse (`Node.fillet` freezes a name set), so the
#:   absence bites exactly the consumers the stub tells to store.
#: - **The expression surface, split at the half the audit reaches.**
#:   `Expr`, `ParamEnv`, `parse_expr` and `ParseError` are **G1**: its
#:   row records the residue by name ("a profile step whose argument
#:   is an EXPRESSION rather than a literal" — the one door still
#:   blocking `plate_param` from scratch, and the same door
#:   `GeomPred.datum_distance`'s comparand waits on). The READ side
#:   `eval`, `eval_count` and `EvalError` is **B-EXPR-READ**, unnamed
#:   on that page or anywhere else: Python holds `DocParam` values and
#:   has no door from an expression to its value. One family, two ids,
#:   because the entries are what carry an id and only one half of
#:   this family has one.
#: - **B-READBACK — the geometry read-back doors.** `face_frame`,
#:   `edge_frame`, `vertex_position`, `denotation`, `Denotation`, and
#:   the `Pose` / `ReadbackError` they answer in. `crate::select`'s
#:   third invariant — "a name answers with values, never keys" — has
#:   no Python face: `Evaluation.all_faces` hands back names and
#:   nothing asks one where it is.
#: - **B-VALIDATE4 — the fourth validator rung.**
#:   `validate_pseudomanifold`. `Body` binds three of the ladder's
#:   four; this one is simply missing.
#: - **G16 — chamfer.** `chamfer_edges`, `Chamfered`. The fillet's
#:   ruled sibling, and the reason it cannot be bound the way
#:   `Node.fillet` was is one level down: `editor-core` has no
#:   `Chamfer` node, so this is a document-layer unit before it is a
#:   binding one — which IS G16, whose row says the same thing from
#:   the scene side ("**Not a bindings gap.** The day `Node::Chamfer`
#:   lands, binding it is the mechanical LIB-PYBUNDLE shape").
#: - **B-CANCEL — cooperative cancellation.** `CancelToken`.
#:   `evaluate(doc)` takes none, so a Python caller cannot stop a long
#:   evaluation.
#: - **B-FORMAT — the D6 display formatter.** `fmt_length`,
#:   `fmt_angle`, `FmtQuantityError`. `Length.in_unit` answers a bare
#:   float; choosing digits and a symbol is the formatter's job and
#:   Python redoes it by hand.
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
    "EdgeDescription": INTERIOR,
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
    # --- gap: the pin-UPDATE door (audit G18) ---------------------
    # G15's own doors are bound (`ContentPin`, `DocRef`,
    # `content_pin`, `canonical_bytes`, `header_document_id`, and
    # `Workspace` beside them). What is left of the pin family is the
    # door that moves a pin AT ITS SITES, and a site is an
    # `InstantiatePart` node's `DocRef` — the one thing Python cannot
    # author. So these are cited to the series that would bring the
    # node, not to the closed row.
    "PinMultiplicity": f"{GAP}: G18 the pin-update door",
    "PinSites": f"{GAP}: G18 the pin-update door",
    "UpdateError": f"{GAP}: G18 the pin-update door",
    "mixed_pins": f"{GAP}: G18 the pin-update door",
    "update_references": f"{GAP}: G18 the pin-update door",
    # --- gap: assembly at-rest gate (audit G18) -------------------
    "Assembly": f"{GAP}: G18 assembly at-rest gate",
    "AssemblyError": f"{GAP}: G18 assembly at-rest gate",
    "AtRestFinding": f"{GAP}: G18 assembly at-rest gate",
    "Attribution": f"{GAP}: G18 assembly at-rest gate",
    "MintedDeclaration": f"{GAP}: G18 assembly at-rest gate",
    "RefusedRef": f"{GAP}: G18 assembly at-rest gate",
    "assemble": f"{GAP}: G18 assembly at-rest gate",
    # --- gap: mates and the solve (audit G18) ---------------------
    "Alignment": f"{GAP}: G18 mates and the solve",
    "AxisSense": f"{GAP}: G18 mates and the solve",
    "ClassAdmission": f"{GAP}: G18 mates and the solve",
    "ClusterMaintenance": f"{GAP}: G18 mates and the solve",
    "MateFault": f"{GAP}: G18 mates and the solve",
    "MateFrame": f"{GAP}: G18 mates and the solve",
    "MatePrimitive": f"{GAP}: G18 mates and the solve",
    "MateRole": f"{GAP}: G18 mates and the solve",
    "MateSide": f"{GAP}: G18 mates and the solve",
    "SolvedPoses": f"{GAP}: G18 mates and the solve",
    "Subgroup": f"{GAP}: G18 mates and the solve",
    "class_admission": f"{GAP}: G18 mates and the solve",
    "clusters": f"{GAP}: G18 mates and the solve",
    "gauge_of": f"{GAP}: G18 mates and the solve",
    "reading_edges": f"{GAP}: G18 mates and the solve",
    "relative_freedom_components": f"{GAP}: G18 mates and the solve",
    "solve_document": f"{GAP}: G18 mates and the solve",
    # --- gap: instantiated parts (audit G18) ----------------------
    "PartFault": f"{GAP}: G18 instantiated parts",
    "PartResolver": f"{GAP}: G18 instantiated parts",
    "PlacementRuleFault": f"{GAP}: G18 instantiated parts",
    "ResolveFailure": f"{GAP}: G18 instantiated parts",
    "ResolveFault": f"{GAP}: G18 instantiated parts",
    # --- gap: split/inline refactorings (audit G18) ---------------
    "InlineError": f"{GAP}: G18 split/inline refactorings",
    "InlineOutcome": f"{GAP}: G18 split/inline refactorings",
    "InterfaceCrossing": f"{GAP}: G18 split/inline refactorings",
    "InterfaceRecord": f"{GAP}: G18 split/inline refactorings",
    "NodeMap": f"{GAP}: G18 split/inline refactorings",
    "SplitError": f"{GAP}: G18 split/inline refactorings",
    "SplitOutcome": f"{GAP}: G18 split/inline refactorings",
    "inline": f"{GAP}: G18 split/inline refactorings",
    "split": f"{GAP}: G18 split/inline refactorings",
    # --- gap: explicit product roots (audit G18) ------------------
    "ProductError": f"{GAP}: G18 explicit product roots",
    "RootFault": f"{GAP}: G18 explicit product roots",
    "product": f"{GAP}: G18 explicit product roots",
    "product_named": f"{GAP}: G18 explicit product roots",
    # --- gap: advisory checks DS6 (census-owned) ------------------
    "CheckEvidence": f"{GAP}: B-CHECKS advisory checks",
    "CheckFinding": f"{GAP}: B-CHECKS advisory checks",
    "CheckId": f"{GAP}: B-CHECKS advisory checks",
    "CheckKind": f"{GAP}: B-CHECKS advisory checks",
    "CheckRefusal": f"{GAP}: B-CHECKS advisory checks",
    "ChecksConfig": f"{GAP}: B-CHECKS advisory checks",
    "ChecksError": f"{GAP}: B-CHECKS advisory checks",
    "ChecksReport": f"{GAP}: B-CHECKS advisory checks",
    "Severity": f"{GAP}: B-CHECKS advisory checks",
    "enforce_checks": f"{GAP}: B-CHECKS advisory checks",
    "run_checks": f"{GAP}: B-CHECKS advisory checks",
    "subject_body": f"{GAP}: B-CHECKS advisory checks",
    # --- gap: picking (census-owned) ------------------------------
    "HitTestError": f"{GAP}: B-PICKING ray onto a name",
    "NodePick": f"{GAP}: B-PICKING ray onto a name",
    "NodePickError": f"{GAP}: B-PICKING ray onto a name",
    "PickHit": f"{GAP}: B-PICKING ray onto a name",
    "PickTarget": f"{GAP}: B-PICKING ray onto a name",
    "Ray": f"{GAP}: B-PICKING ray onto a name",
    "pick_face": f"{GAP}: B-PICKING ray onto a name",
    # --- gap: name resolution (census-owned) ----------------------
    "Resolution": f"{GAP}: B-RESOLVE names across runs",
    "RunCtx": f"{GAP}: B-RESOLVE names across runs",
    "resolve": f"{GAP}: B-RESOLVE names across runs",
    # --- gap: the expression surface (audit G1 + census-owned) ----
    "EvalError": f"{GAP}: B-EXPR-READ an expression's value",
    "Expr": f"{GAP}: G1 Expr-bearing authoring steps",
    "ParamEnv": f"{GAP}: G1 Expr-bearing authoring steps",
    "ParseError": f"{GAP}: G1 Expr-bearing authoring steps",
    "eval": f"{GAP}: B-EXPR-READ an expression's value",
    "eval_count": f"{GAP}: B-EXPR-READ an expression's value",
    "parse_expr": f"{GAP}: G1 Expr-bearing authoring steps",
    # --- gap: geometry read-back doors (census-owned) -------------
    "Denotation": f"{GAP}: B-READBACK a name answers with values",
    "Pose": f"{GAP}: B-READBACK a name answers with values",
    "ReadbackError": f"{GAP}: B-READBACK a name answers with values",
    "denotation": f"{GAP}: B-READBACK a name answers with values",
    "edge_frame": f"{GAP}: B-READBACK a name answers with values",
    "face_frame": f"{GAP}: B-READBACK a name answers with values",
    "vertex_position": f"{GAP}: B-READBACK a name answers with values",
    # --- gap: assorted single doors -------------------------------
    "CancelToken": f"{GAP}: B-CANCEL cooperative cancellation",
    "Chamfered": f"{GAP}: G16 chamfer has no recipe node",
    "FmtQuantityError": f"{GAP}: B-FORMAT the D6 display formatter",
    "chamfer_edges": f"{GAP}: G16 chamfer has no recipe node",
    "fmt_angle": f"{GAP}: B-FORMAT the D6 display formatter",
    "fmt_length": f"{GAP}: B-FORMAT the D6 display formatter",
    "validate_pseudomanifold": f"{GAP}: B-VALIDATE4 the fourth validator rung",
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
        nothing. Measured at LIB-G15: 324 curated names, 95 top-level
        stub names, 357 `Class.member` spellings. The
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
            f"or '{GAP}: <ID> <prose>'",
        )

    def test_every_gap_entry_names_a_defined_id(self):
        """**The pointers resolve — both id spaces, mechanically.**

        A `gap:` entry is OWED WORK, and the id after the colon is
        what a dispatcher works from. Before this guard those ids were
        free prose, and half of them read `register B <something>` —
        a pointer at a PARAGRAPH of `docs/LIB-LOG.md`, which is not an
        enumeration and cannot be dispatched against. So each entry
        now names exactly one id and each id must be defined:

        - an audit citation (`G` + digits) must be a gap
          `docs/guide/north-star-audit.md` actually defines, read off
          that page's own gap tables by [`audit_gap_ids`] — the
          cross-document half, which is what stops a citation from
          drifting when the page is re-cut;
        - anything else must be a `FAMILIES` key, chartered here.

        And it decays, like every other roster in this file: a
        `FAMILIES` entry no `gap:` entry cites is a charter for work
        nobody is tracking, which is the same failure as a stale
        exclusion — it fails here rather than sitting as decoration.

        **What this does NOT claim** (the module docstring says it
        too, and it matters most here): not that the cited id is the
        RIGHT owner for that door, only that it RESOLVES. `G18` being
        a defined gap is checkable; `assemble` being G18's work rather
        than the closed G15's is a reading, made by hand at the entry.
        Nor does
        it claim the prose after the id is accurate — only that there
        IS prose, because an entry reduced to a bare tag loses the one
        thing a human reader can use.
        """
        defined = audit_gap_ids()
        self.assertGreater(
            len(defined),
            12,
            "the audit page's gap tables parsed to almost nothing — its shape "
            "changed and this guard was about to pass vacuously",
        )
        bad = []
        cited = set()
        for name, family in sorted(NOT_BOUND.items()):
            if not family.startswith(f"{GAP}: "):
                continue
            words = family[len(f"{GAP}: ") :].split()
            if len(words) < 2:
                bad.append(f"{name}: {family!r} — an id and then no prose")
                continue
            gap_id = words[0]
            cited.add(gap_id)
            if gap_id.startswith("G") and gap_id[1:].isdigit():
                if gap_id not in defined:
                    bad.append(
                        f"{name}: cites {gap_id}, which north-star-audit.md's "
                        "gap tables do not define"
                    )
            elif gap_id not in FAMILIES:
                bad.append(
                    f"{name}: cites {gap_id}, which FAMILIES does not charter"
                )
        self.assertEqual(
            bad,
            [],
            "a 'gap:' entry must name an audit gap id the page defines, or a "
            "FAMILIES key, and then say something readable about it",
        )
        uncited = sorted(set(FAMILIES) - cited)
        self.assertEqual(
            uncited,
            [],
            "FAMILIES charters work no NOT_BOUND entry cites — either the "
            "entries moved off it (drop the charter) or the id is a placeholder "
            "for work nobody is tracking",
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
