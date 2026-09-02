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
`step_import`, `quantity`) enter through the prelude — plus the three
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

1. `pncad.pyi` declares a top-level name spelled identically —
   `Doc`, `Node`, `Selector`, `SegTag`, `circle`, `Pose`. This is
   where MOST curated names land, and NO COUNT IS WRITTEN DOWN: the
   number moves whenever either side grows, and one written here
   would be a stale claim rather than a checked one. The count this
   bullet used to carry had been caught stale once and corrected once
   (it read "sixty-two" while standing at 111, and was rewritten to "a
   hundred and twenty-four"), which is the argument: a prose count is
   checked only when someone happens to look. What stops a scanner
   passing vacuously is the FLOORS
   asserted in `test_the_census_is_not_vacuous`, and those are
   assertions rather than prose.
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
  page defines, that `B-PICKING` is a family this file charters. Which
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
  watches those, and it is where all three landing is recorded
  (`update_to_store` is a `Workspace` METHOD, so what watches it is
  `tests/test_assembly_author.py`, which walks the door).
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
    an entry citing one (`G1`'s Expr residue, `G2`'s tube node) is
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
#:   evaluation they answer "as of"; `apply` and `save` are `Doc`'s;
#:   `tessellate` is `Body`'s, beside them.
#: - **A writer into a sink became a door that answers the bytes.**
#:   `write_ascii` and `write_binary` take a Rust `Write`, which is not
#:   a value Python holds; `Mesh.to_stl_ascii` answers the text and
#:   `Mesh.to_stl_binary` the bytes, and Python writes the file. Their
#:   two option structs are keyword arguments, listed under
#:   `different-shape` with `StepOptions`.
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
#:   NOT: an arena key). The unit constants are lower-cased, and two
#:   whose symbols are not Python identifiers shift further: `IN` is
#:   `inch` because `in` is a keyword, and `PI` is `pi_rad` because its
#:   symbol `pi rad` is two words. The stub comments on both at the
#:   declaration. (Not counted here: the table grows a row whenever
#:   `quantity` does, and a number written beside it would date at the
#:   next one.)
BOUND_AS = {
    "CM": "cm",
    "DEG": "deg",
    "AssertionVerdict": "Verdict",
    "DatumValue": "Value.datum",
    "DocumentId": "Doc.id",
    "IN": "inch",
    "M": "m",
    "MM": "mm",
    "NodeErrorKind": "EvaluationError.kind",
    "NodeValue": "Value",
    "UnevaluatedReason": "Verdict.reason",
    "PI": "pi_rad",
    # The document seam, and the two enums that say why it did not
    # open. `Workspace` IS a `PartResolver` (the document layer's own
    # impl) and is passed as itself to `evaluate(doc, resolver=...)`;
    # `PartFault`'s arms and the `ResolveFault` classification inside
    # them cross as `EvaluationError.kind` tags — `part_no_resolver`,
    # `part_pin_mismatch`, `part_epsilon_seam`, `part_unresolved`,
    # `part_root_failed`, `part_product`, `part_reference_cycle`,
    # `part_depth_exceeded` — the same flattening `NodeErrorKind` gets
    # above. They left the `gap` roster at LIB-G18a, when the resolver
    # parameter made them reachable: the tags existed before it, and
    # `part_no_resolver` was the only one an evaluation could produce.
    "PartFault": "EvaluationError.kind",
    "PartResolver": "Workspace",
    # The read-back doors, which hang off the evaluation because a
    # name is only meaningful against the run that minted it — so the
    # free functions arrive as `Evaluation` methods, beside the
    # materializers that answer the names they take. `Pose`,
    # `Denotation` and `ReadbackError` are spelled identically and are
    # accounted by rule 1, not here. They left the `gap` roster at
    # LIB-B-READBACK, which closed the family that chartered them.
    #
    # `DanglingRef` is `ReadbackError::Dangling`'s payload and crosses
    # as `ReadbackError.variant`, the way `RootFault` crosses as
    # `EditError.variant`: its two arms ARE the two tags —
    # `dangling_entity` for a topological key that does not resolve,
    # `dangling_geometry` for a geometry key reached from a live
    # entity that does not — because which lookup came back empty is
    # what a caller branches on. Python has no class for the payload
    # and needs none; the tag carries the whole of it.
    "DanglingRef": "ReadbackError.variant",
    "denotation": "Evaluation.denotation",
    "edge_frame": "Evaluation.edge_frame",
    "face_frame": "Evaluation.face_frame",
    "vertex_position": "Evaluation.vertex_position",
    # The four different-shape entries LIB-G18b left behind after
    # binding the rest of the assembly vocabulary name-for-name.
    #
    # `NodeMap` is a type ALIAS for a map, and Python spells the two
    # maps that ride it as ordered PAIR LISTS
    # (`SplitOutcome.node_map`, `InlineOutcome.node_map`) — a
    # `dict[NodeId, NodeId]` would need `NodeId` hashable-and-ordered
    # as a key type for no gain, since both are read in order and
    # never looked up by one id.
    #
    # `RootFault` crosses as `EditError.variant`, and the tags are the
    # FAULT's, not a wrapper's — `root_not_live`, `root_duplicate`,
    # `root_ancestor`, `root_uncovered` — because which invariant
    # broke is what a caller branches on. Shared by the edit door and
    # the persistence validator, exactly as the one Rust type is.
    #
    # `PlacementRuleFault` likewise crosses as `EditError.variant`
    # (`placement_rule_mismatch`, `empty_placement_list`,
    # `non_finite_placement`, `improper_placement`). It sat in the
    # `gap` roster carrying the reason "a placement is set by an edit
    # Python cannot author" — MEASURED WRONG, and corrected here: the
    # fault is the GROUP BOOLEAN's placement-rule fault, not the
    # assembly registry's, and `Node.placed_union_at` has reached it
    # since LIB-PYPU (an improper frame raises `improper_placement`
    # today). `DocEdit.set_placement`'s own refusals are separate
    # `EditError` arms that share the tag namespace, so binding it
    # changed nothing about this entry except who noticed.
    #
    "NodeMap": "SplitOutcome.node_map",
    "PlacementRuleFault": "EditError.variant",
    "RootFault": "EditError.variant",
    "RAD": "rad",
    "RecipeNodeId": "NodeId",
    "ResolveFault": "EvaluationError.kind",
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
    "chamfer_edges": "Node.chamfer",
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
    "tessellate": "Body.tessellate",
    "subtract_with": "Node.boolean",
    "transform_rigid": "Node.transform",
    "union": "Node.boolean",
    "union_with": "Node.boolean",
    "validate": "Body.validate",
    "validate_closed": "Body.validate_closed",
    "validate_geometric": "Body.validate_geometric",
    "write_ascii": "Mesh.to_stl_ascii",
    "write_binary": "Mesh.to_stl_binary",
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
#: and tube), `G18` (the
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
#:
#: **How a family CLOSES.** When the unit that owns an id binds its
#: doors, every `gap:` entry citing that id moves off the roster —
#: into `BOUND_AS`, or off it entirely where Python now spells the
#: name identically — and the charter goes with them. It has to:
#: [`TestBindingCensus.test_every_gap_entry_names_a_defined_id`] fails
#: on a `FAMILIES` key no entry cites, because a charter nobody is
#: working from is a decoration, and this file's whole argument is
#: that a roster which only grows is a roster nobody reads. So a
#: closed family leaves NO stub here, and the guard keeps passing in
#: both directions. What records the closure is the ENTRIES, each
#: carrying the unit that moved it — `B-READBACK` closed at
#: LIB-B-READBACK, the first family to close, and the four verbs it
#: chartered say so where they now sit in `BOUND_AS`.
FAMILIES = {
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
    "B-NOTATION": (
        "authored notation, the D6 boundary's other half; closing it "
        "binds `WrittenLength` / `WrittenAngle` onto the `DocParam` and "
        "expression constructors, so a Python caller who writes "
        "`25 * mm` gets a parameter that REMEMBERS the millimetres — "
        "today the unit erases at the `Length` door and the document "
        "records the canonical row, which is what Rust authoring "
        "stopped doing at schema v20"
    ),
    "B-FORMAT": (
        "the D6 display formatter; closing it binds `fmt_length` / "
        "`fmt_angle` and their refusal, so choosing digits and a symbol "
        "stops being hand-work Python redoes beside `Length.in_unit`'s "
        "bare float"
    ),
    "B-DISTRIBUTIONS": (
        "parameter uncertainty and the analysis lane (ERROR-DESIGN "
        "E1/E2); closing it binds `Distribution`'s four forms onto a "
        "`DocParam` constructor and the analysis doors that read them "
        "back — the analyzed box, tail mass and leaf mass, with "
        "`DistributionFault` and the band's typed measure refusal as "
        "arms a Python caller can dispatch on. Today Python authors "
        "unannotated parameters only: a document read back from a file "
        "carries an annotation Python can READ ONLY THROUGH equality, "
        "hashing and `repr`, and cannot spell. The sharp edge is the "
        "WRITE door, not the read one — `set_doc_param` is "
        "create-or-replace, so passing it a `DocParam` rebuilt from a "
        "dimension and a number DELETES the annotation silently. "
        "`set_doc_param_value` is the value-only door that carries the "
        "declaration forward and is what a Python caller moving a "
        "number must use until this family closes"
    ),
    "B-MEASURES": (
        "AUTHORING a measurement (ERROR-DESIGN E3/E10); closing it "
        "binds `MeasureExpr`'s constructors, `MeasurePrimitive`'s "
        "three verbs and `AssertionDir` onto `Node.measure` / "
        "`Node.assertion` constructors, with `MeasureNodeFault` as the "
        "refusal a caller dispatches on. The READ half already ships "
        "and is deliberately not in this gap: `Value.measure` answers "
        "with a `Measurement` (value plus the F1 dimension it rides) "
        "and `Value.assertion` with a `Verdict` (three states kept "
        "three, both numbers on a decided one). That split is the "
        "unit's own disposition: the friction the R-series reviews "
        "keep finding is unreadable RESULTS, and a Python caller can "
        "now read a web and its verdict off any evaluation, including "
        "one loaded from a file authored elsewhere. What a Python "
        "caller cannot yet do is WRITE one — the same asymmetry "
        "B-DISTRIBUTIONS records, and without B-DISTRIBUTIONS's sharp "
        "edge, because no existing write door silently drops a measure"
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
#:   `SelectRefusal` for `DeclareError` and, where a selection wraps
#:   one, `InterrogateError`; `ReadbackError.variant` for
#:   `InterrogateError` at the read-back doors themselves, where the
#:   kernel's own `ReadbackError` arms arrive under their own tags
#:   rather than a wrapper's — one Rust type, two Python classes,
#:   because the two doors refuse different CALLS — and for
#:   `DanglingRef`, the `Dangling` arm's payload, whose two arms are
#:   the two `dangling_*` tags;
#:   `EvaluationError.kind` for `ResolveFailure`, whose classified
#:   fault IS the `part_*` tag (`ResolveFault` and `PartFault` are in
#:   `BOUND_AS` at that spelling) and whose `message` is the
#:   exception's message — the resolver's own diagnosis, prose because
#:   that is what it is;
#:   `StlError.variant` for `SolidNameError` and `BinaryHeaderError`,
#:   which refuse the same CALL the writers do because the options
#:   they validate are that call's keyword arguments.
#: - *An option struct that became keyword arguments.* `StepOptions` is
#:   `Evaluation.step_string`'s six keywords, one per field and each
#:   defaulting to the Rust default — a correspondence the crate's own
#:   `surface_census` holds to the struct, so this entry cannot go
#:   back to naming a subset while the record grows; `AsciiOptions` and
#:   `BinaryOptions` are `Mesh.to_stl_ascii`'s `solid_name=` and
#:   `Mesh.to_stl_binary`'s `header=`, and their two VALIDATED
#:   newtypes cross as the `str` those arguments take — `SolidName`
#:   and `BinaryHeader` protect an invariant, not a vocabulary, and
#:   the invariant is checked at the call rather than at a
#:   constructor Python would otherwise have to name. Their refusals
#:   ride `StlError.variant` under `solid_name_*` / `binary_header_*`
#:   tags, which is why `SolidNameError` and `BinaryHeaderError` are
#:   in the flattened-payload bullet above too; `ImportOptions` is
#:   `import_step`'s absent second argument; `EvalOptions` is
#:   `evaluate`'s `resolver=`, the one field of it that changes an
#:   ANSWER, bound at LIB-G18a — which is also when its memo residue
#:   left the `gap` roster, and note the memo was never a field of it
#:   (`prior` is `evaluate`'s own second argument, bound as `prior=`).
#:   The three fields with no Python spelling are stated rather than
#:   waved: `epoch` is minted per run and is not a caller's choice;
#:   `parallel` and `boolean_sweep` are runtime switches the kernel
#:   documents as ANSWER-PRESERVING and test-facing (`parallel` exists
#:   so D9's determinism cross-check can compare both schedules in one
#:   run, `boolean_sweep`'s two paths are bit-identical by the BVH
#:   differential suite's own pin), so no ANSWER is unreachable
#:   through them. `profile_lift` (M10-P) is a FOURTH such field and
#:   its argument is a different one, because it is not
#:   answer-preserving in general: it decides whether profile geometry
#:   is elaborated at the evaluation's own scalar. What makes it
#:   unreachable-without-loss here is that Python evaluates at `f64`
#:   ALONE, and at `f64` the lift is a no-op by construction — guided
#:   elaboration reproduces the pinned one bitwise, which
#:   `editor-core`'s `m10_p_lift` suite pins over the whole corpus. The
#:   field starts changing answers exactly when Python gains a non-f64
#:   evaluation, and it should gain a spelling in the same unit that
#:   brings one. A PERFORMANCE door — "evaluate this in parallel" —
#:   would be a new unit and a new entry, not this one.
#: - *Recourse and deferral sentences.* `CONTACT_RECOURSE`,
#:   `FIT_DEFERRAL`, `SEL_DATUM_DISTANCE` and `REGENERATE_RECOURSE`
#:   are the prose a Rust refusal cites; Python's refusals carry theirs
#:   in the exception's message. `SCHEMA_VERSION` is the same shape of
#:   constant on the persistence door, which Python reaches only
#:   through `load`.
#:
#:   `UNDER_RECOURSE` and `CLASS_DEFERRAL` left this bullet at
#:   LIB-G18b and are bound top-level, on `PIN_MISMATCH_RECOURSE`'s
#:   precedent: an assembly author's two most-hit refusals are an
#:   under-determined mate and a class outside v1, and a test that
#:   wants to say "the refusal ends on its recourse" must not do it by
#:   re-typing the sentence. `CLASS_DEFERRAL` is also what
#:   `ClassAdmission.why` answers for the `not_admitted` arm, from the
#:   table rather than restated — so the constant and the door agree
#:   by construction.
#: - *Structures Python's authoring surface replaces with its own.*
#:   `Applied` and `EditRecord` are `apply`'s pair, and `Doc.apply`
#:   mutates in place and answers `Optional[NodeId]`, so there is no
#:   pair to name. `PartialPath` is the Rust lattice's one type where
#:   Python has one CLASS PER STATE (`PathOpen`, `PathPoint`,
#:   `PathDirectedPoint`, …), which is §L4's typestate translation.
#:   `LineTarget`, `ContinueTarget` and `TangentArcTarget` are absorbed
#:   into the verbs that take them, and
#:   `bulge_from_center`/`bulge_from_via` into the
#:   `Center`/`Via` spec modes that are bound. `Dimension` is what
#:   `DocParam.length`/`angle`/`count`/`scalar` choose between;
#:   `SlotId` is what `DocEdit.bind_count_param` names implicitly;
#:   `ProfileDoc` is the alias `Node.profile` builds from loops;
#:   `SplitSide` is the position in `Value.split`'s tuple.
#: - *A write sink Python does not need.* `write_step` takes a Rust
#:   `Write`; `Evaluation.step_string` answers the text and Python
#:   writes it. (`write_ascii`/`write_binary` are not here either, and
#:   for the same reason — they are in `BOUND_AS`, as the two `Mesh`
#:   doors that answer the bytes. The sink was never what was
#:   missing.)
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
#: each. TWO of the ids are the audit page's, cited (`G1`, `G2`); the
#: other seven are `FAMILIES` keys
#: this census owns, because the audit's SCENE-driven list does not
#: reach a door no tour scene exercises — which is exactly why those
#: accumulated unnoticed and why this census exists.
#:
#: - **G2 — sweep and tube.** `sweep_body`, `tube_along_arc`,
#:   `tube_along_arc_hollow`, `TubeError`, `TubeWindow`. Banked, not
#:   merely unbound: `wire_sweep` refuses unconditionally and
#:   `Node::Tube` does not exist (a schema-version break).
#: **G18 is CLOSED and no longer a `gap` id here** (LIB-G18b). It
#: held six families and 43 names: the pin-update door
#: (`update_references`, `UpdateError`, `mixed_pins`,
#: `PinMultiplicity`, `PinSites`), the at-rest gate (`assemble`,
#: `Assembly`, `AssemblyError`, `AtRestFinding`, `Attribution`,
#: `MintedDeclaration`, `RefusedRef`), mates and the solve
#: (`Alignment`, `MateFrame`, `MatePrimitive`, `MateRole`, `MateSide`,
#: `AxisSense`, `SolvedPoses`, `Subgroup`, `MateFault`,
#: `ClusterMaintenance`, `clusters`, `gauge_of`, `reading_edges`,
#: `relative_freedom_components`, `solve_document`, `ClassAdmission`,
#: `class_admission`), instantiated parts (`PlacementRuleFault`),
#: split and inline (`split`, `inline`, `SplitOutcome`,
#: `InlineOutcome`, `SplitError`, `InlineError`, `NodeMap`,
#: `InterfaceRecord`, `InterfaceCrossing`) and explicit product roots
#: (`product`, `product_named`, `ProductError`, `RootFault`).
#:
#: Thirty-nine are top-level names in `pncad.pyi`; four are in
#: `BOUND_AS` and the comment there says why each has a different
#: Python shape. NOTE the collision the mapping rule kept honest:
#: top-level `split` is the document REFACTORING, and `Node.split` is
#: the geometry verb — two different doors that a looser rule would
#: have matched to each other, which is exactly how this family stayed
#: invisible for as long as it did.
#:
#: One correction the closing measured, recorded rather than quietly
#: fixed: `PlacementRuleFault` carried the reason "no document Python
#: can produce reaches one — a placement is set by an edit Python
#: cannot author". That was measured against the wrong door. The fault
#: is the GROUP BOOLEAN's placement-rule fault, and
#: `Node.placed_union_at` has reached it since LIB-PYPU; binding
#: `DocEdit.set_placement` changed nothing about it, because that
#: edit's own refusals are separate `EditError` arms sharing the tag
#: namespace.
#: **B-CHECKS is CLOSED and no longer a `gap` id here**
#: (LIB-B-CHECKS). It held thirteen names, the largest census-owned
#: family: `run_checks`, `enforce_checks`, `subject_body`,
#: `ChecksReport`, `ChecksConfig`, `ChecksError`, `CheckFinding`,
#: `CheckEvidence`, `CheckId`, `CheckKind`, `CheckRefusal`,
#: `Severity` and `Advisory`. All thirteen are top-level names in
#: `pncad.pyi` and none needed `BOUND_AS` — the report/gate split
#: crossed with the same shape it has in Rust, a value out of
#: `run_checks` and a typed refusal out of `enforce_checks`, and the
#: two knob TYPES crossed as two types because their difference is
#: DS6's waiver rule (`Advisory` is `Severity` minus `Error`, so a
#: resident shipping no acknowledgment record cannot be set to refuse
#: — unspellable in Python as in Rust).
#:
#: The closing measured one thing worth recording: the charter named
#: "the connectedness check" as the family's resident, and by the
#: time it was closed the registry had TWO — the product-separation
#: resident shipped 2026-08-29, and it is the one that carries the
#: `Advisory` knob. A charter is written when a family is named, not
#: when it is closed, and this is what that gap looks like in
#: practice: the id and the door list stayed right, the resident
#: count did not.
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
#: - **B-VALIDATE4 — the fourth validator rung.**
#:   `validate_pseudomanifold`. `Body` binds three of the ladder's
#:   four; this one is simply missing.
#: **G16 is CLOSED and no longer a `gap` id here** (LIB-G16). It held
#: `chamfer_edges` and `Chamfered`, and its own row said what would
#: close it: "the day `Node::Chamfer` lands, binding it is the
#: mechanical LIB-PYBUNDLE shape". `Node::Chamfer` landed at schema
#: v16, `Node.chamfer` binds it, and the two names moved to the
#: dispositions their fillet twins already carry — the kernel verb to
#: `BOUND_AS` (`Node.chamfer` is the Python spelling of the question
#: `chamfer_edges` answers) and the record to `INTERIOR`, where
#: `Filleted` already sits. `BlendKind` joins them as `INTERIOR`:
#: which blend a shared refusal came from IS visible in Python, as the
#: error `kind` tag (`fillet`/`chamfer` and the three
#: `*_selection_*` tags), so the discriminant crosses — just not as a
#: type.
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
    "AsciiOptions": SHAPE,
    "BinaryHeader": SHAPE,
    "BinaryHeaderError": SHAPE,
    "BinaryOptions": SHAPE,
    "BlendError": SHAPE,
    "BlendRefusal": SHAPE,
    "BooleanError": SHAPE,
    "CONTACT_RECOURSE": SHAPE,
    "CurveKindSet": SHAPE,
    "DeclareError": SHAPE,
    "Dimension": SHAPE,
    "EdgeKey": SHAPE,
    "EditRecord": SHAPE,
    "EvalOptions": SHAPE,
    "EvalOutcome": SHAPE,
    "FIT_DEFERRAL": SHAPE,
    "FaceKey": SHAPE,
    "ExtrudeError": SHAPE,
    "ImportOptions": SHAPE,
    "InterrogateError": SHAPE,
    "LineTarget": SHAPE,
    # `continue_to`'s target trait, absorbed into the verb exactly as
    # `LineTarget` and `TangentArcTarget` are — and the verb itself is
    # not bound in Python yet either (the Rust-side roster records
    # that, `surface_census.rs`).
    "ContinueTarget": SHAPE,
    "LoftError": SHAPE,
    "Mat3": SHAPE,
    "MassPropsError": SHAPE,
    "MigrationError": SHAPE,
    # The attribution walk's verdict, and the door that answers it.
    # Same family as `RolePath`/`RoleSeg` and for their reason: it
    # reads the INSIDE of a name, which nothing user-side may read.
    "NameOrigin": SHAPE,
    "NodeError": SHAPE,
    "NodeResult": SHAPE,
    "NonFiniteSite": SHAPE,
    # The display-unit CODE a `DocParam` carries. A one-byte index into
    # the unit table has no Python spelling and should not get one: a
    # notation reaches Python as its SYMBOL, which is what
    # `DocParam.__repr__` prints.
    "UnitSym": SHAPE,
    "PartialPath": SHAPE,
    "PathNoCornerReason": SHAPE,
    "Point2": SHAPE,
    "Point3": SHAPE,
    "ProfileDoc": SHAPE,
    "ProfileError": SHAPE,
    "ProfileLift": SHAPE,
    "ProgramFault": SHAPE,
    "REGENERATE_RECOURSE": SHAPE,
    "Real": SHAPE,
    "RecordedProgramError": SHAPE,
    "ResolveFailure": SHAPE,
    "RevolveAxis": SHAPE,
    "RevolveError": SHAPE,
    "RolePath": SHAPE,
    "RoleSeg": SHAPE,
    "SCHEMA_VERSION": SHAPE,
    "ASSERT_BOUND": SHAPE,
    "SEL_DATUM_DISTANCE": SHAPE,
    "Side": SHAPE,
    "SlotId": SHAPE,
    "SnapshotError": SHAPE,
    "SolidName": SHAPE,
    "SolidNameError": SHAPE,
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
    "Vec2": SHAPE,
    "Vec3": SHAPE,
    # The slot vocabulary's 3-vector families, beside `Axis3` and
    # `SlotId` and for their reason: Python addresses a slot through
    # its own spelling, so the Rust enum that groups three of them is
    # not a name a Python caller needs.
    "VectorSlot": SHAPE,
    "VertexKey": SHAPE,
    "attribute": SHAPE,
    "bulge_from_center": SHAPE,
    "bulge_from_via": SHAPE,
    # A cone-delete is composed caller-side in Python: the bound door
    # is `Doc.apply` over one `DocEdit.delete_node` at a time, and the
    # order this answers is what a chrome needs to state a cost before
    # the click.
    "cascade_delete_order": SHAPE,
    "p2": SHAPE,
    "p3": SHAPE,
    # The kernel query seat (`topo::query`): its doors answer over a
    # `Body` and arena keys — the vocabulary the curation keeps
    # unnameable in Python (see `EdgeKey`/`FaceKey` above). The Python
    # spelling of the same questions is the document door:
    # `Evaluation.select_where` with `GeomPred.curve_kind` /
    # `GeomPred.surface_kind` / `GeomPred.adjacent_kinds`, and the
    # `Evaluation.all_edges`/`all_faces` materializers.
    "query": SHAPE,
    "real": SHAPE,
    "v2": SHAPE,
    "v3": SHAPE,
    "write_step": SHAPE,
    # --- behind-a-door --------------------------------------------
    "Band": INTERIOR,
    "BandError": INTERIOR,
    "BlendKind": INTERIOR,
    "BooleanBody": INTERIOR,
    "BooleanDeclarations": INTERIOR,
    "BooleanResult": INTERIOR,
    "BooleanResultKind": INTERIOR,
    "BooleanValue": INTERIOR,
    "Chamfered": INTERIOR,
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
    # --- gap: parameter distributions and the analysis lane -------
    # --- gap: authoring a measurement (census-owned) --------------
    # The READING half ships (`Value.measure`, `Value.assertion`); what
    # is listed here is the authoring vocabulary alone.
    "AssertionDir": f"{GAP}: B-MEASURES measurement authoring",
    "MeasureExpr": f"{GAP}: B-MEASURES measurement authoring",
    "MeasureRef": f"{GAP}: B-MEASURES measurement authoring",
    "MeasureNodeFault": f"{GAP}: B-MEASURES measurement authoring",
    "MeasurePrimitive": f"{GAP}: B-MEASURES measurement authoring",
    "Distribution": f"{GAP}: B-DISTRIBUTIONS parameter uncertainty",
    "WrittenAngle": f"{GAP}: B-NOTATION authored notation",
    "WrittenLength": f"{GAP}: B-NOTATION authored notation",
    "DistributionFault": f"{GAP}: B-DISTRIBUTIONS parameter uncertainty",
    "DistributionField": f"{GAP}: B-DISTRIBUTIONS parameter uncertainty",
    # G18 IS GONE FROM THIS ROSTER, closed at LIB-G18b. Its six
    # families held 43 names — the pin-update door, the at-rest gate,
    # mates and the solve, instantiated parts, split/inline, explicit
    # product roots — and every one of them is now accounted for:
    # thirty-nine name-for-name in `pncad.pyi`, and four in `BOUND_AS`
    # because their Python shape differs (`NodeMap`, `RootFault`,
    # `PlacementRuleFault`, and `MateSide`, which is both). The
    # positive form is `tests/test_assembly_author.py`.
    # B-CHECKS IS GONE FROM THIS ROSTER, closed at LIB-B-CHECKS, and
    # the id is gone from `FAMILIES` with it — a charter no entry
    # cites is what `test_every_gap_entry_names_a_defined_id`'s
    # decay half fails on. Its thirteen names are all top-level in
    # `pncad.pyi`, name for name, so none of them needed `BOUND_AS`:
    # the registry's shape crossed unchanged, including the two knob
    # TYPES whose difference is DS6's waiver rule. The positive form
    # is `tests/test_checks.py`.
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
    "unparse": f"{GAP}: G1 Expr-bearing authoring steps",
    # --- gap: geometry read-back doors (census-owned) -------------
    # --- gap: assorted single doors -------------------------------
    "CancelToken": f"{GAP}: B-CANCEL cooperative cancellation",
    "FmtQuantityError": f"{GAP}: B-FORMAT the D6 display formatter",
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
        nothing. Measured at LIB-G15's merge base: 328 curated names,
        96 top-level stub names, 364 `Class.member` spellings — a
        SNAPSHOT, and the reason the floors below are the assertion
        and these numbers are not. The
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
