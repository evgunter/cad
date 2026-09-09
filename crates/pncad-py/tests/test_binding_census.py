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

   **A NAME MATCH ACCOUNTS MEMBERS, NOT THE TYPE** (Ev, 2026-09-09, on
   `work/lib/datum-crosses-name-for-name-as-two-types.md`). Where the
   curated name resolves to a `pub enum` or `pub struct`, the match
   accounts only the members the Python namesake actually SPELLS — an
   arm as a class attribute of the same name (`SurfaceKind.Plane`) or
   as a snake-cased method, property or static constructor
   (`Node::Extrude` -> `Node.extrude`); a struct's bare-`pub` field as
   a same-named attribute. Every other member owes a row of its own,
   in `MEMBERS_BOUND_AS` or `MEMBERS_NOT_BOUND`, and a member with
   neither a spelling nor a row fails here naming itself.

   The reason is that rule 1 is exactly as strong as the coincidence
   that the two sides picked the same word, and twice that coincidence
   has hidden a whole door. Rust's `Datum` is the AUTHORING enum whose
   arms a recipe holds; Python's is the READ-side value `Value.datum()`
   answers with, and it spells none of the six — so `Datum::FaceFrame`
   was invisible for the life of the family chartered to bind it.
   `Node::Union` and `DocEdit::SetMembers` were the same shape behind
   two names this file accounts whole, and only a hand-kept roster in
   `tests/test_north_star.py` could see them. The member rule would
   have demanded a row for each on the day the name matched.

   WHAT IT STILL CANNOT SEE, because a rule with an unstated blind
   spot is an unverified claim:

   - a member whose snake-cased name COINCIDENTALLY matches an
     unrelated attribute of the namesake is accounted, and nothing
     here asks whether the two are about the same thing. Four of
     `editor_core::Evaluation`'s ten fields are accounted that way,
     against a Python `Evaluation` that is a different type;
   - a member reachable only through a type ALIAS or an associated
     type, and a member whose type is a generic parameter — the
     resolver's blind spots (b) and (d), which it states at
     `payload_identifiers` and `sweep`;
   - the resolver is CRATE-aware and not MODULE-aware (its blind spot
     (h)), so two types with one name inside one crate are one
     declaration here, and the first in path order supplies both
     member lists;
   - a TUPLE struct's fields have no names, so there is nothing for a
     namesake to spell and nothing to owe a row.
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
family tag `FAMILIES` does not charter. `work/lib/log.md`'s "LIB
residual register", category B, points here for the enumeration
rather than carrying one in prose.

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
import functools
import importlib.util
import re
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
    (`editor_core::ident::DocumentId` introduces `DocumentId`); a
    braced group introduces each of its items.
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


# --- the curated declarations, through the sweep's resolver ------------

#: The payload-rung sweep, which is the resolver this file SHARES rather
#: than re-implements. It already answers the two questions the member
#: rule needs — which crates the façade's path closure reaches, and which
#: `pub enum`/`pub struct` each curated name resolves to — and it answers
#: them crate-aware, which a spelling comparison cannot. Re-deriving that
#: here is the defect that script exists to close: the pattern was prose,
#: three runs re-implemented it, and no two agreed on a number.
SWEEP = REPO / "scripts" / "payload-rung-sweep.py"


@functools.cache
def resolver():
    """`scripts/payload-rung-sweep.py`, loaded as a module.

    By PATH rather than by `import`, because the file is a script with a
    hyphen in its name and is not an importable module — and because the
    path is the honest statement of what is shared. Nothing runs at import:
    the script's work is behind `main`, so this costs a parse.

    STDLIB ONLY still holds, and so does NO COMPILED MODULE. What arrives
    is more Rust SOURCE TEXT — the same reading, over the façade's whole
    path-dependency closure instead of its three curated files.
    """
    spec = importlib.util.spec_from_file_location("payload_rung_sweep", SWEEP)
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


_LEADING_SNAKE = re.compile(r"(.)([A-Z][a-z]+)")
_INNER_SNAKE = re.compile(r"([a-z0-9])([A-Z])")


def snake_case(name):
    """`FaceFrame` -> `face_frame`: the shift a Rust arm makes to become a
    Python constructor, property or method."""
    return _INNER_SNAKE.sub(r"\1_\2", _LEADING_SNAKE.sub(r"\1_\2", name)).lower()


@functools.cache
def curated_declarations():
    """`{name: (declaration, [member, ...])}` for the names rule 1 matches.

    A curated name enters only if `pncad.pyi` declares it top-level — a
    name in `BOUND_AS` or `NOT_BOUND` is argued WHOLE, with its reason at
    its entry, and the member rule is about what a name match accounts for.
    The resolution is the sweep's: the declaration in the crate the `pub
    use` statement named, falling back to the declaration set for that
    spelling where the carrying crate does not declare it.
    """
    sweep = resolver()
    index = sweep.declarations(REPO, sweep.dependency_set(REPO))
    lists = tuple(stem.removesuffix(".rs") for stem in FACADE_FILES)
    top, _ = stub_surface()
    found = {}
    for name, where in sweep.curated(REPO, lists).items():
        if name not in top:
            continue
        roots = {root for statements in where.values() for root in statements}
        decls = [d for d in index.get(name, []) if d.crate in roots]
        decls = decls or index.get(name, [])
        if not decls:
            continue
        members = sweep.declared_members(decls[0])
        if members:
            found[name] = (decls[0], members)
    return found


def unspelled_members(spelled):
    """`[(type, member), ...]` — every member of a matched declaration the
    Python namesake does not spell, in declaration order."""
    out = []
    for name, (_decl, members) in sorted(curated_declarations().items()):
        for member in members:
            if f"{name}.{member}" in spelled:
                continue
            if f"{name}.{snake_case(member)}" in spelled:
                continue
            out.append((name, member))
    return out


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
#:   `Value.datum` answers, and `UnitVec3` — the type that makes a
#:   datum's normal unit, so that an unnormalized one has no spelling
#:   in Rust either — is what `Datum.direction` answers, as the plain
#:   triple it always was; its constructor's refusals cross the way
#:   every other typed refusal does, as tags on `EvaluationError.kind`
#:   (Python builds datums through
#:   `Node.datum_plane`/`Node.datum_axis`, never by naming the type).
#:   Three tags carry them, and all three are Python-visible:
#:   `degenerate_direction` for a zero-length direction,
#:   `non_finite_direction` for one whose length overflows the norm or
#:   is not a number, and `escalated` — whose `predicate` payload reads
#:   `datum_unit_norm` for a datum, the kernel constructor's funnel
#:   name, where the same field reads `eval_direction_norm` for the
#:   directions the evaluation layer owns.
#:   `NodeValue` IS `Value` and
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
    # `VerbKind`/`Arity` are `NodeErrorKind::VerbArity`'s payload — an
    # internal wiring-bug refusal — and cross exactly as their carrier
    # does: flattened to the `verb_arity` tag `EvaluationError.kind`
    # answers (the `UnitVec3Error` row's precedent; the typed payload
    # stays a Rust-side diagnosis surface).
    "Arity": "EvaluationError.kind",
    "VerbKind": "EvaluationError.kind",
    "UnevaluatedReason": "Verdict.reason",
    "UnitVec3": "Datum.direction",
    "UnitVec3Error": "EvaluationError.kind",
    "PI": "pi_rad",
    # `DistributionField` names WHICH offset of a distribution a fault
    # is about, and it crosses as that word on the fault rather than as
    # a class: a three-member closed set naming struct fields is what a
    # caller compares against, and a class would add an import for no
    # question it answers. The `NodeErrorKind` row's shape — a curated
    # enum flattened onto an exception attribute — one level in, on an
    # exception that already carries `variant`.
    "DistributionField": "DistributionFault.field",
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
    # The FIFTH read-back door, and the one that is not a frame.
    # `face_carrier_kind` answers a face's stored `SurfaceKind` tag
    # through the same node ladder the three above walk — a VALUE the
    # way a `Pose` is, and the one "is this face planar" is a
    # comparison of. It is where `face_frame` declines: a NURBS
    # carrier has no canonical frame and refuses there, and its kind
    # is still readable here. It left the `gap` roster at
    # LIB-B-FACE-FRAME, which closed the family that chartered it.
    #
    # `SurfaceKind` itself is spelled identically and is accounted by
    # rule 1; it already crossed INTO the kernel for
    # `GeomPred.surface_kind`, and this door is the first to cross it
    # back out.
    "face_carrier_kind": "Evaluation.face_carrier_kind",
    # PICKING, the fourth door onto a name. `pick_face` is a free
    # function taking the evaluation, so it arrives as an `Evaluation`
    # method beside the three read-back doors above — the same shift,
    # for the same reason. `Ray`, `PickHit`, `NodePick`,
    # `NodePickError` and `HitTestError` are spelled identically and
    # are accounted by rule 1, not here. All seven left the `gap`
    # roster at LIB-B-PICKING, which closed the family.
    #
    # `PickTarget` is the entry that carries the argument, and it is
    # NOT a narrowing. Rust's `pick_face` takes a slice of them;
    # Python's takes a list of `NodePick`s and makes each target
    # itself — because CUR3 recorded `MeshPick` DECIDED absent from the
    # façade and `PickTarget::pick` is a `&MeshPick`, so through
    # `pncad` a raw target has no constructor in EITHER language. The
    # value that plays the target's role is the `NodePick`, whose
    # pairing cannot be mis-asserted. The carrier-projection rule reads
    # out the same way it did for `DanglingRef` above: a payload's
    # category follows what its CARRIER does at the crossing, the
    # carrier here is the door's `targets` argument, and that argument
    # crosses holding `NodePick`s.
    "PickTarget": "NodePick",
    "pick_face": "Evaluation.pick_face",
    # `MeshPickError` is `NodePickError::Index`'s payload, and it
    # crosses by the same rule and at a different spelling from
    # `DanglingRef`'s. `ReadbackError`'s arm had no word of its own, so
    # its payload's two arms BECAME the carrier's two tags. This
    # carrier's arm does: `mesh_index` says which door's invariant
    # broke, and a caller branching on the standing ladder needs it to
    # stay put. So the payload's discriminant arrives BESIDE the
    # carrier's rather than in place of it, at `index_variant`, `None`
    # on every other arm.
    #
    # One value today (`position_out_of_range`), and the attribute
    # exists for what the type does NOT have yet: the match that mints
    # it is exhaustive, so a second indexing invariant stops the
    # bindings compiling instead of joining the first under one word.
    # Python has no class for the payload and needs none, but its
    # three numbers cross beside the discriminant — `patch`,
    # `triangle` and `index` on the same exception, `None` on every
    # other arm. They describe a mesh that violates its own invariant,
    # which is a bug report and not something to branch on; a bug
    # report is assembled from numbers, and the payload's type is not
    # raisable, so this door is the only crossing they get.
    "MeshPickError": "NodePickError.index_variant",
    # `StepImportError::RecognitionAmbiguous`'s `kind` field — which
    # analytic kind's stage-1 estimator declined on a face that could
    # not import without promotion. It crosses by the carrier rule at
    # `MeshPickError`'s spelling and for that entry's reason: the
    # carrier's arm HAS a word of its own, `recognition_ambiguous`,
    # which names the condition and which a caller branching on the
    # import ladder needs to stay put. So the payload's discriminant
    # arrives BESIDE it rather than in place of it, `None` on every
    # other arm.
    #
    # Two values, and they are two different next moves: `plane`
    # declining is a flatness question at the file's tolerance,
    # `cylinder` declining is an ill-conditioned axis and wants more
    # of the patch. The match that mints them is exhaustive, so a
    # third promotable kind stops the bindings compiling instead of
    # joining one of these.
    #
    # No Python test row: firing the arm needs a file with a
    # multi-bound curved face on an ill-conditioned NURBS surface,
    # which is a `step-import` fixture and not something an authoring
    # door reaches. Both words are pinned in Rust, where the refusal
    # constructs.
    "PromotedKind": "StepImportError.promoted_kind",
    # THE SUCCESS HALF OF THE SAME DOOR, which crosses as a VALUE and
    # so puts the carrier rule to work in the other direction.
    #
    # `StepImport` is what `import_step` answers. Its `Solid` arm is
    # the whole of what Python's door can adopt, so the carrier crosses
    # AS that arm — `ImportReport`, a frozen value with `body`,
    # `enclosure`, `eps_in` and the three record lists — rather than as
    # a two-arm sum with a `variant` a Python caller could never see
    # the other value of. The `Wireframe` arm is not dropped: it is
    # already this door's typed refusal (`StepImportError.variant ==
    # "wireframe"`), because a wireframe is not a body and this door
    # adopts bodies.
    #
    # THE MEASUREMENT, because a shape claim is a claim. Before this
    # the door answered a bare `Body` and the arm's other five fields
    # had no Python spelling at all; the natural journey — import a
    # file, ask what it encloses — ran the certified quadrature TWICE
    # over one body at one band, once inside the import gate and once
    # in `Body.mass_properties`. `enclosure` is the gate's own result
    # handed back, so that journey now measures once, and
    # `test_document.py` pins the two BIT for bit rather than within a
    # tolerance: the claim is that they are the same computation, and
    # an almost-equality would pass for two different ones that agreed.
    "StepImport": "ImportReport",
    # The record vocabulary the `Solid` arm names —
    # `StructureNormalization`, `CurvePromotion`, `PlacedInstance` and
    # the `FaceCensus` pair a normalization maps between — is NOT
    # mapped here and needs no entry: each crosses as a frozen row
    # class Python spells identically, so all four are accounted for by
    # rule 1. What a row projects is its own fields, which is the
    # census's voice for a value the caller READS, where a refusal's
    # payload gets a word on the exception. Only the two discriminants
    # below need a mapping, because a Python caller holds a word rather
    # than the enum.
    #
    # `StructureNormalization::kind`, by the carrier rule and at the
    # value side's spelling: the carrier projects every field as an
    # attribute, so its discriminant is one of those fields — a word at
    # `kind`, with the one arm's payload beside it at `promoted_to`
    # (which analytic kind certified, `PromotedKind`'s own two words)
    # and `residual`, `None` on the other four arms. The word does NOT
    # fold the kind in: a caller reading "a patch was promoted" reads
    # `surface_promotion` whichever kind it was, and branches on the
    # second attribute if it cares. Five words from an exhaustive
    # match, so a sixth normalization stops the bindings compiling.
    "NormalizationKind": "StructureNormalization.kind",
    # `CurvePromotion::kind`, the same rule one row over. One word
    # today (`circle`) and a map rather than a literal because the
    # recognizer's named exclusions — line-as-degree-1, ellipse, helix,
    # open arcs — each land here when their follow-up does.
    "PromotedCurveKind": "CurvePromotion.kind",
    # THE OP FAMILIES' REFUSALS, at the same spelling and under the
    # same rule, applied at the carrier that holds the most of them.
    # `EvaluationError.kind` says WHICH DOOR refused (`revolve`,
    # `tube`, `boolean`, `fillet`); each of these is the kernel
    # refusal that door raised, and its arm now crosses as
    # `inner_kind` beside the carrier's word rather than in place of
    # it — `MeshPickError`'s reading, one carrier over, for a carrier
    # whose word every caller branching on the op ladder already
    # holds.
    #
    # THE MEASUREMENT, because a category is a claim. Each of these
    # was `different-shape` or `behind-a-door`, and both dispositions
    # were true when written: the arms differed only in PROSE, so
    # there was no Python shape to point at. There is one now, and it
    # is a word per arm — twenty-one for `RevolveError`, forty-one for
    # `BooleanError`, twenty-two for `ShellError` — minted by an
    # exhaustive match, so a kernel arm added without a word stops the
    # bindings compiling. What still has no Python spelling is the
    # arm's FIELDS, and that is the payload question, tracked
    # separately: a mapping here claims the discriminant crosses, and
    # nothing more.
    #
    # `ShellError`, `SkinError`, `ParamAttachError` and the kernel's
    # `topo::splitting::SplitError` cross the same way and are NOT
    # rows here: none of them is a leaf name of the three curated
    # lists this census reads (they arrive through the whole-crate
    # re-exports), so this file never accounted for them and does not
    # start now.
    "BandError": "EvaluationError.inner_kind",
    "BlendError": "EvaluationError.inner_kind",
    "BooleanError": "EvaluationError.inner_kind",
    "ExtrudeError": "EvaluationError.inner_kind",
    "LoftError": "EvaluationError.inner_kind",
    "ProfileError": "EvaluationError.inner_kind",
    "RevolveError": "EvaluationError.inner_kind",
    "TransformError": "EvaluationError.inner_kind",
    "TubeError": "EvaluationError.inner_kind",
    # `NamingError` and `ProgramRefusal` are new to the curated lists
    # and arrive already bound. Both were `NOT_CARRIED` at the façade
    # under "the curated face is a different shape", and the reading
    # did not hold: neither has a curated door of its own, both are
    # payloads of curated refusals (`NodeErrorKind::Naming`,
    # `EditError::ProfileProgramRefused`), and a consumer could match
    # either arm and not name what it caught. Carried, and their arms
    # cross at the two carriers' second words.
    "NamingError": "EvaluationError.inner_kind",
    "ProgramRefusal": "EditError.inner_variant",
    # `MetaVersionError` is the same row one arm over, and it arrives
    # by the same reading failing. It was `NOT_CARRIED` under "the
    # curated face is a different shape", qualified: it is a nested
    # REFUSAL rather than a leaf value, and the arm holding it carried
    # no inner word to name it by, so naming it was the per-arm-tag
    # question. That question is answered — every carrier that
    # projects a word now projects its arm's word beside it — and
    # `EditError::MetaUnversioned` answers WHICH of the three ways a
    # stored metadata value breaks the D7 producer convention
    # (`not_a_map`, `missing_version`, `version_not_int`) rather than
    # only that it did.
    #
    # THE MEASUREMENT: there is no Python door that mints a
    # `SetAppearanceMeta`, so no Python row can provoke this arm and
    # the reach is a Rust construction pin — `src/tests.rs`'s
    # `every_edit_arm_projects_the_payload_it_carries`, which the
    # carriage moves from 57 of 58 arms to 58 of 58, because the arm
    # it could not build was the one whose third field this façade did
    # not carry.
    "MetaVersionError": "EditError.inner_variant",
    # THE PERSISTENCE DOOR'S PAYLOAD, under the same rule at the
    # carrier that wraps the most refusals of other layers.
    # `PersistError.variant` says WHICH stage refused; three of these
    # are what that stage was refusing ABOUT, and they now cross
    # rather than staying in the message.
    #
    # THE MEASUREMENT, because a category is a claim. All three were
    # `different-shape`, and that reading was true when written: the
    # persistence door carried one attribute, so there was no Python
    # shape to point at. There is one now. `SnapshotError`'s
    # nineteen arms and `ProgramFault`'s two mint a word apiece from
    # an exhaustive match, so a kernel arm added without one stops the
    # bindings compiling, and the word rides `inner_variant` beside
    # the stage's own. `NonFiniteSite` is the one that does NOT cross
    # as a word: it is a RECURSIVE descriptor (an edit's index
    # wrapping the site inside that edit's payload), so it crosses as
    # the kernel's own prose for where the float sits — one sentence
    # on `site`, not a field per rung.
    #
    # What still has no Python spelling is the arms' FIELDS: a
    # snapshot refusal's node ids and counts are the snapshot door's
    # surface, not the persistence door's. A mapping here claims the
    # discriminant crosses, and nothing more.
    "ProgramFault": "PersistError.inner_variant",
    "SnapshotError": "PersistError.inner_variant",
    "NonFiniteSite": "PersistError.site",
    # THE STL DOOR'S TWO OPTION REFUSALS. Both were `different-shape`
    # under "their refusals ride `StlError.variant`", and the reading
    # was true when written: only the tag crossed, so the arms had no
    # Python shape. They have one now — the character the
    # `solid <name>` grammar does not admit, and the header's byte
    # length — projected from the same exhaustive match as the
    # writers' own arms, so an arm added to either enum stops the
    # bindings compiling.
    "SolidNameError": "StlError.character",
    "BinaryHeaderError": "StlError.len",
    # THE TWO-TOLERANCE ESCALATION PAYLOAD, curated at the prelude
    # because THIRTEEN prelude refusals carry it and a Rust caller
    # holding an `Escalated` arm could not name what it held.
    #
    # THE MEASUREMENT, and it MOVED. This entry was `INTERIOR` by the
    # carrier rule, on the count that not one of the thirteen
    # projected the escalation's own shape — each crossing as a single
    # tag plus the kernel's prose, with no bound exception carrying a
    # margin, an enclosure bound or a band. `FrameError` now does: the
    # frame constructors' degenerate arm carries `Option<Indeterminate>`
    # and projects it as `margin` / `margin_low` / `margin_high`,
    # `zero` / `escalate` and `predicate`. `FrameError` is not one of
    # the thirteen — it is a `geom_core` refusal the frame
    # constructors return, not a prelude-curated one — so that count
    # is unchanged; what has changed is the sentence about bound
    # exceptions, and this row moves with it.
    #
    # `MarginDiag` — the payload's own field type — is a curated name
    # too, on the same list and for the same crossing: its own row is
    # below, `different-shape`, because its discriminant arrives as
    # which of these attributes is set rather than as a word.
    "Indeterminate": "FrameError.margin",
    # THE SHELL DOOR'S OWN REFUSAL, curated at `pncad::document`
    # beside the two `CheckEvidence` arms that carry it, and its
    # discriminant is the word those arms publish: `band`, `props`,
    # `escalated` or `zero_volume`. `CheckEvidence.reason` is the same
    # refusal's sentence — the kernel's own prose — and this is the
    # branchable half beside it, so a caller stops substring-matching
    # the sentence to learn which shell refusal escalated the count.
    #
    # Its own fields do not cross: which shell, and the mass-properties
    # failure under `props`, are the shell door's vocabulary and this
    # entry claims the discriminant and nothing more.
    "ShellClassifyError": "CheckEvidence.inner_variant",
    # THE TIER-3′ CENSUS VOCABULARY, at the one door on this surface
    # whose discriminant crosses in a SEQUENCE.
    #
    # THE MEASUREMENT, because both entries were `INTERIOR` and both
    # arguments were true when written. The validate doors crossed
    # their failures as joined `Display` prose with a `door` and a
    # `failure_count` and no per-arm tag whatsoever, so which
    # coincidence the census found was not a thing Python could read
    # at all — the finding reached a caller only inside that text,
    # through the kernel's own rendering. Those rows also said what
    # would flip them: "a validate projection with per-arm tags — and
    # both would move together". That door exists now, and they do.
    #
    # `ValidationError.findings` is a list of `ValidationFinding`s, one
    # per failure, `len(findings) == failure_count`. `CensusContact` is
    # `UndeclaredContact`'s payload and its arm is `contact_kind`;
    # `CensusSubject` is what `CensusUnsupported` and
    # `CensusLaneUnsupported` are ABOUT and its arm is `subject_kind`,
    # with `entity_kind` beside it for the `Entity` arm's carrier.
    # Each is minted by an exhaustive match in `src/tags.rs`, so a
    # kernel arm added without a word stops the bindings compiling.
    #
    # THE SEQUENCE IS THE EXCEPTION AND IS ARGUED AS ONE. Every other
    # mapping on this list is a scalar attribute because every other
    # refusal reports ONE fault; `validate*` is the one door that
    # reports many at once, which `failure_count` has said since it was
    # bound. `ValidationFinding`'s docstring, `pncad.pyi` and the
    # README all state that at the door, so it does not read as a
    # second convention.
    #
    # What still has no Python spelling is the arm's FIELDS — which
    # vertex, which face — and that is the payload question rather
    # than this one: no arena key crosses to a surface that holds
    # names.
    #
    # EVERY payload DISCRIMINANT of this door now crosses, and the
    # four entries here are that rule rather than a pair. The two
    # below joined the two above under the ruling that generalised
    # them: a discriminant a caller can act on crosses as an attribute
    # of its own, named per type, `None` on every other arm — one word
    # per concept, never one word meaning different things under
    # different `variant`s. `StaleDeclaration` is
    # `stale_contact_declaration`'s and says WHICH record lost its
    # witness, which is which record to withdraw; `RingContact` is
    # `ring_meets_outer`'s and says HOW the ring meets the outer loop,
    # which is where the ring has to move. Neither payload's FIELDS
    # cross — both are arena keys — so the projection stops at the
    # discriminant, as it does at the two above.
    #
    # THE MEASUREMENT for the second pair. Both were `INTERIOR` with
    # the argument that the ruling had named two types and these were
    # not them. `ValidationFinding.stale_kind` and
    # `.ring_contact_kind` are those two rows moved: each is minted by
    # an exhaustive match in `src/tags.rs` beside the other two, so a
    # kernel arm added to either enum stops the bindings compiling.
    # No Python scene reaches either arm — a stale record needs a
    # declaration parted from its witness and no door hands one out, a
    # ring on its outer loop needs raw Euler surgery — so the words
    # are pinned per arm in `src/tests.rs` and named as unreachable in
    # `tests/test_validate.py`. Crossing them is still the move: the
    # attribute is the contract, and a caller reads it the day a door
    # produces one.
    "CensusContact": "ValidationFinding.contact_kind",
    "CensusSubject": "ValidationFinding.subject_kind",
    "RingContact": "ValidationFinding.ring_contact_kind",
    "StaleDeclaration": "ValidationFinding.stale_kind",
    # NAME RESOLUTION across re-evaluation, the verdict a stored name
    # gets on the next run. `Resolution` is spelled identically and is
    # accounted by rule 1; these two are the family's shape entries,
    # and both left the `gap` roster at LIB-B-RESOLVE, which closed it.
    #
    # `resolve` is a free function taking the run, so it arrives as an
    # `Evaluation` method beside the read-back and picking doors above
    # — the same shift, for the same reason.
    #
    # `RunCtx` is the entry that carries the argument. Rust's is a
    # `(doc, eval)` PAIR because neither half answers alone: the
    # evaluation holds the name tables, and the document decides
    # whether a stored name's minting node is still in the recipe at
    # all. Python's `Evaluation` IS that pair — it captures the
    # document at `evaluate`, beside the `ParamEnv` it already captured
    # for `select_where`, and for the identical reason stated there:
    # the answer must be as of the document the evaluation is OF, and
    # threading a doc back in per query would let the two drift (the
    # sharper form of that hazard here, since Python's `Doc` is
    # mutable and `accept` swaps it under the handle). The
    # carrier-projection rule reads out as it did for `PickTarget`: a
    # payload's category follows what its CARRIER does at the crossing,
    # the carrier is the door's run argument, and that argument crosses
    # as the `Evaluation`.
    "RunCtx": "Evaluation",
    "resolve": "Evaluation.resolve",
    # The verdict's three PAYLOADS, curated so the arms cross. The
    # carrier-projection rule places them: `Resolution` projects a
    # discriminant (`resolution_status_tag`), so its payloads project
    # theirs — the `DanglingRef` reading, on a carrier that is a VALUE
    # rather than a refusal.
    #
    # The two enums are one attribute between them, and the merge is
    # deliberate rather than a shortcut: `Resolution.variant` is
    # WHICH arm, and which vocabulary it is drawn from is already said
    # by `status`. `vanished` / `ambiguous` / `node_gone` under a
    # failure, `target_failed` / `target_poisoned` /
    # `target_not_evaluated` under an indeterminate, `None` when
    # resolved. Two attributes would have made a caller read `status`
    # first to know which one to look at, which is exactly the shape
    # this file's every-attribute-always-present rule exists to avoid.
    "ResolveError": "Resolution.variant",
    "ResolveIndeterminate": "Resolution.variant",
    # `ResolutionFailure` is the struct that pairs one of those arms
    # with the repair candidates, and `offers` is what it ADDS — the
    # error half is `variant` plus `detail` above. So it crosses at the
    # attribute that is its own contribution, on the reading that a
    # mapping points at the door rather than asserting a shape.
    "ResolutionFailure": "Resolution.offers",
    # The expression surface, which hangs off the DOCUMENT for the
    # read-back doors' reason one layer over: all three free
    # functions take a per-document table — `parse_expr` the declared
    # DIMENSIONS, the two evaluators the bound VALUES — so all three
    # arrive as `Doc` methods and the table is never threaded in
    # separately, where it could drift from the document it describes.
    # `Expr`, `ParseError` and `EvalError` are spelled identically and
    # are accounted by rule 1, not here.
    #
    # `unparse` is the odd one: it is the text door OUTWARD and takes
    # only the expression, so it is a property of the `Expr` rather
    # than a document method — which is exactly the carrier-projection
    # reading, the receiver following what the door actually needs.
    #
    # They left the `gap` roster at LIB-B-EXPR-READ, which closed
    # B-EXPR-READ; the four G1-cited ones left as ordinary decay, and
    # G1 stays open on its authoring half.
    "eval": "Doc.eval",
    "eval_count": "Doc.eval_count",
    "parse_expr": "Doc.parse_expr",
    "unparse": "Expr.text",
    # The display formatter, on the receiver the carrier-projection
    # rule picks — the same reading that put `unparse` on `Expr` two
    # entries up, applied to a door whose Rust signature does NOT
    # name its carrier. `fmt_length` takes canonical metres as a bare
    # `f64` because Rust reaches this module from BELOW, where the
    # newtype is already unwrapped; Python has no such caller, and
    # what a Python consumer holds is a `Length` precisely so that a
    # length and an angle cannot be interchanged. Binding the free
    # functions as free functions would hand that interchange back —
    # `fmt_length((90 * deg).radians, mm)` type-checks and prints
    # plausible nonsense — so the door lands where the value it needs
    # already lives, and `Length.format(deg)` is refused the way
    # every other dimension confusion at this boundary is.
    #
    # `FmtQuantityError` is spelled identically in the stub and is
    # accounted by rule 1, not here.
    #
    # They left the `gap` roster at LIB-B-FORMAT, which closed
    # B-FORMAT.
    "fmt_angle": "Angle.format",
    "fmt_length": "Length.format",
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
    # `InputFault` (DM5) is the same shape one door further: the rule
    # is stated once on the node and rendered by three doors, and the
    # two a Python caller can reach are edit doors, so it crosses as
    # `EditError.variant` — `duplicate_input` and `too_few_members`.
    # The third renderer is the load validator's `SnapshotError`, which
    # this façade does not carry at all.
    # The edit door's PAYLOAD, projected at LIB-DOORS-1. `SlotId` is
    # `EditError.slot`, one stable word per named slot — the
    # measurement is that a caller who could read WHICH edit refused
    # could not read which SLOT it refused at, and six arms carry one.
    # `AttrKind` is `EditError.kind` at the two appearance arms;
    # `ExprPath` is `EditError.path`, whose two halves ride the `node`
    # and `slot` attributes that already name them, so the address
    # crosses whole across three attributes rather than as a fourth
    # type. All three were `NOT_CARRIED` at the façade under "the
    # curated face is a different shape", and for `AttrKind` and
    # `ExprPath` the reading did not hold: they are `EditError`
    # payloads with no curated door of their own, which is the payload
    # rule `ProgramRefusal` and `NamingError` moved under.
    "AttrKind": "EditError.kind",
    "ExprPath": "EditError.path",
    "SlotId": "EditError.slot",
    "InputFault": "EditError.variant",
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
    "tube_along_arc": "Node.tube",
    "tube_along_arc_hollow": "Node.hollow_tube",
    "fillet_edges": "Node.fillet",
    "find_flush_candidates": "Evaluation.find_flush_candidates",
    "intersect": "Node.boolean",
    "intersect_with": "Node.boolean",
    "loft_body": "Node.loft",
    "mass_properties": "Body.mass_properties",
    # The façade's lattice-backed loop door and the document layer's
    # node constructor answer the same question — a closed outline from
    # a coordinate table — one rung apart, which is the mapping rule's
    # ordinary case (`extrude` -> `Node.extrude`). Python has no
    # loops-in-hand seat to spell the Rust door at: `Node.polygon`
    # names a frame and mints a node, and the refusals it can raise are
    # the same PATHS refusals, tagged (`polygon_too_few_vertices`,
    # `junction_tangent`).
    "polygon": "Node.polygon",
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
    # The fourth rung takes a SECOND argument in Rust, and it crosses
    # by being captured rather than passed: a `Body` carries the
    # declared contacts its producer minted for it, so the Python door
    # is a bare method like the three above it. `ContactRecords` stays
    # `INTERIOR` for the reason it always was — nothing hands one out
    # — and now for a second: there is no Python constructor, so a
    # door taking one would be uncallable, and a door taking ANOTHER
    # body's records would spell exactly the mis-pairing tier 3′
    # exists to refuse.
    "validate_pseudomanifold": "Body.validate_pseudomanifold",
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
#: to point at in prose — the "LIB residual register", category B,
#: spelled `docs/LIB-LOG.md` when these entries were written and now
#: living at `work/lib/log.md` — and that register points HERE for
#: its enumeration, so the lineage stays legible in the id itself and
#: a reader arriving from either document lands in the same place.
#: Upper case with a hyphen makes an id unmistakable for an audit
#: `G##`, for a Python identifier, or for the prose that follows it;
#: no whitespace, so `gap: <ID> <prose>` parses by splitting once.
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
FAMILIES: dict[str, str] = {
    # THE MAP WAS EMPTY ONCE, and it could be again: every family the
    # LIB residual register's category B enumerated is closed
    # (B-READBACK, B-CHECKS, B-CANCEL, B-FACE-FRAME, B-PART,
    # B-NOTATION, B-DISTRIBUTIONS, B-MEASURES), and the one below does
    # not come from that register either.
    #
    # THE ONE LEFT IS A MEMBER-RULE FINDING: a member of a curated type
    # Python spells identically, so the name match accounted it and no
    # roster here could report it until members were counted. The
    # rule's other two findings closed at LIB-GAPS-1, which bound
    # `Node.datum_point`, `Node.datum_frame` and `Mesh.boundaries`.
    # `test_every_gap_entry_names_a_defined_id` reads this map in both
    # directions: no entry may cite a key that is not here, and no key
    # here may go uncited.
    "B-DOC-EDITS": (
        "the three `DocEdit` arms no Python constructor builds — an "
        "expression at a path, and the two witness edits. Closing it "
        "needs a curated payload for the witness pair and a prose "
        "rendering for the path refusal, then one constructor per arm "
        "and one test row per tag each can raise."
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
#:   `EdgeKey`/`FaceKey`/`LoopKey`/`VertexKey` are the arena keys the
#:   whole curation exists to keep unnameable in Python, and they
#:   reached the census only because the prelude lifts `topo`
#:   wholesale. `LoopKey` joined its three siblings on the Rust list
#:   because a validation refusal names all four and a Rust caller
#:   could spell three of them; nothing about that reaches Python,
#:   which holds names and never keys.
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
#:   have no Python name:
#:   `EvaluationError` for `NodeError`,
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
#:   that is what it is.
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
#:   ride `StlError` under `solid_name_*` / `binary_header_*` tags,
#:   with their arms' payloads projected beside the tag, which is why
#:   `SolidNameError` and `BinaryHeaderError` are in `BOUND_AS` and
#:   not in the flattened-payload bullet above; `ImportOptions` is
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
#:   in the exception's message.
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
#:   `ProfileDoc` is the alias `Node.profile` builds from loops;
#:   `SplitSide` is the position in `Value.split`'s tuple.
#:
#:   **`SlotId` left this bullet at LIB-DOORS-1** and is in `BOUND_AS`
#:   below. Its sentence stayed true and stopped being the whole
#:   answer: Python still ADDRESSES a slot through a door that names
#:   it, and it now READS one off a refusal — `EditError.slot`, a
#:   stable word per slot. Addressing and reading are two questions,
#:   and a roster that compares names has to answer the second.
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
#: `BooleanDeclarations` — `ContactRecords` is behind a SECOND door
#: since LIB-B-VALIDATE4, `Body.validate_pseudomanifold`, which takes
#: one in Rust and captures it here rather than asking a caller for a
#: value they cannot build), the fillet's coincidence band
#: (`Band`, `BandError`, built from the run's epsilon), and
#: `ContentBits` — a TRAIT, and Python has no way to spell one.
#:
#: **`gap` — genuinely unbound doors, and each is OWED WORK.** This is
#: the family that makes the census worth having: these are not
#: decisions, they are debt, and the id after the colon says what owns
#: each. Two of the ids are the audit page's, cited (`G1`, `G2`); the
#: rest are `FAMILIES` keys
#: this census owns, because the audit's SCENE-driven list does not
#: reach a door no tour scene exercises — which is exactly why those
#: accumulated unnoticed and why this census exists.
#:
#: **No count of the families is written here, and that is a
#: correction.** The sentence used to carry one, it was stale, and it
#: was stale in BOTH halves at once — "two … the other seven" while
#: nine families were cited. Worse, it went stale twice inside one
#: afternoon: LIB-B-EXPR-READ re-cut it to "one … the other eight",
#: and LIB-B-PICKING closing in the same window made that wrong
#: before either landed. A number that two concurrent units can
#: falsify is not a measurement, it is a merge conflict with a
#: plausible face. What is checked always is
#: [`TestBindingCensus.test_every_gap_entry_names_a_defined_id`],
#: which fails on a charter nobody cites and on a citation nothing
#: defines, and says nothing about how many there are — the module
#: docstring's own argument against prose counts, arriving a second
#: time at the paragraph that describes the rosters rather than at
#: the one that describes the mapping rule.
#:
#: - **G2 — the SWEEP half only.** `sweep_body`. The family used to
#:   hold five names; the tube half closed at LIB-TUBE (see the
#:   paragraph below) and `sweep_body` is what is left. Banked, not
#:   merely unbound: `wire_sweep` refuses unconditionally (U4/LQ3,
#:   kernel-owned), so a binding would add a door that cannot
#:   succeed.
#: **G2's TUBE half is CLOSED** (LIB-TUBE). It held
#: `tube_along_arc`, `tube_along_arc_hollow`, `TubeError` and
#: `TubeWindow`, and its own row said what would close it: a
#: `Node::Tube` that does not exist, behind a schema break. That
#: break is not a thing any more (#1553 demolished the version
#: machinery); both node kinds landed as additive vocabulary —
#: `Node::Tube` and `Node::HollowTube` per the #1205 split ruling —
#: and the four names moved to the
#: dispositions their revolve twins already carry: the two kernel
#: doors to `BOUND_AS` (`Node.tube` and `Node.hollow_tube` are the
#: Python spellings of the questions they answer, one door each),
#: `TubeError` to `INTERIOR` beside `RevolveError`, since its
#: refusals cross as the `tube` error tag rather than as a type, and
#: `TubeWindow` to a TOP-LEVEL python name — it is the one of the
#: four a caller must be able to spell, because a window is an
#: argument and `full()` is a choice, not an omission.
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
#: **B-PICKING is CLOSED and no longer a `gap` id here**
#: (LIB-B-PICKING). It held seven names — `pick_face`, `PickTarget`,
#: `PickHit`, `NodePick`, `NodePickError`, `HitTestError` and `Ray` —
#: and the fourth door onto a name now answers in Python in the same
#: opaque-text alphabet `Evaluation.select` speaks: a `PickHit.name`
#: goes to `Node.fillet` unread.
#:
#: The closing measured one thing worth recording, and it is a
#: SECOND-ORDER effect of a curation decision rather than anything the
#: charter foresaw. CUR3 kept `MeshPick` and `MeshPickError` interior,
#: which makes a raw `PickTarget` unconstructible through the façade —
#: the argument is in `crates/pncad/src/select.rs`, and it is about
#: CONSTRUCTION. Two consequences fall out of it downstream. The first
#: is good and was intended: with no raw target to assemble, the door
#: that cannot be mis-paired is not merely preferred but the only one,
#: so the Python signature takes `NodePick`s and the
#: confidently-wrong-name lane (#1098) has no spelling here at all.
#: The second was not foreseen: `NodePickError::Index` CARRIES a
#: `MeshPickError`, so a refusal whose payload type was unnameable
#: crossed as one tag plus prose, where every other arm of that enum
#: is matchable. That was not a relitigation of CUR3 and the closing
#: did not treat it as one — the construction argument is about a
#: `MeshPick`, and a refusal is received rather than built. The
#: payload alone is curated now and its discriminant crosses at
#: `NodePickError.index_variant` (`BOUND_AS`, above), which is where
#: the reasoning for the split spelling lives. Its three numbers
#: crossed later, as `patch`, `triangle` and `index` on the same
#: exception: the closing left them in the prose, and the standing
#: rule is that an arm's payload is attributes.
#: **B-RESOLVE is CLOSED and no longer a `gap` id here**
#: (LIB-B-RESOLVE). It held three names — `resolve`, `Resolution` and
#: `RunCtx` — and the question a consumer that STORES names must ask
#: on every run is now askable in the language whose whole selection
#: story is store-then-reuse: `Evaluation.resolve(name)` answers a
#: three-state verdict for any name `select`, `all_faces` or a
#: `PickHit` handed back on an earlier run.
#:
#: What the closing measured was a LIMIT, and it was the family's own
#: version of a shape this roster recorded three times: the verdict's
#: three arms crossed and their PAYLOAD types did not, so a Python
#: caller learned THAT a name failed and read prose about which. The
#: closing did not relitigate it and the curation since has: the three
#: payload types are carried, and `Resolution.variant` is the
#: discriminant beside `status` (`BOUND_AS`, above). What stays absent
#: is the ladder's telemetry underneath them — the diagnosis, the
#: tombstone and the tie witness — which no consumer on either side of
#: the boundary reads.
#:
#: Two things the binding gained that the charter did not name.
#: `RunCtx` is a PAIR in Rust and Python's `Evaluation` became that
#: pair — it now captures the document at `evaluate` beside the
#: `ParamEnv` it already captured, so a caller cannot ask an
#: evaluation about a document it is not of. And the door is
#: EVALUATION-WIDE where `denotation` is node-scoped: `resolve`
#: answers which node carries a name, so it resolves names
#: `denotation` refuses `no_such_name` for at the node a caller
#: happened to ask.
#: **B-FACE-FRAME is CLOSED and no longer a `gap` id here**
#: (LIB-B-FACE-FRAME). It cited exactly ONE name on this roster —
#: `face_carrier_kind`, now `Evaluation.face_carrier_kind` in
#: `BOUND_AS` — and its charter named THREE doors. The gap between
#: those two numbers is a BLIND SPOT of this census rather than an
#: accounting error, and it is worth writing down at the closure
#: because nothing here would ever have reported it.
#:
#: `Pose.sense` is a FIELD of a curated type. This file's alphabet is
#: top-level names plus the `Class.member` spellings `BOUND_AS` points
#: AT, so a curated type Python spells identically is accounted WHOLE,
#: by rule 1, and a field it fails to project is invisible. That is
#: the same blind spot `py/select.rs`'s growth tripwire records for
#: enum VARIANTS ("the tripwire sees the KERNEL enum and this file's
#: mirror, not `pncad.pyi`", issue #1309), one level out: this census
#: compares NAMES, and a name's insides are nobody's roster. `sense`
#: was real missing surface — the read-back carried the bool and
#: Python could not read it, so the outward normal was unformable.
#:
#: `Datum.face_frame` is the other half, and it is stranger. The
#: curated authoring name is `Datum` (the `editor_core` enum), Python
#: declares a top-level `Datum`, and rule 1 accounts it on SPELLING
#: ALONE — which is exactly what rule 1 claims to do and no more. The
#: two are not the same type: Rust's is the AUTHORING enum whose arms
#: Python spells as `Node.datum_*` constructors, Python's is the
#: READ-side value `Value.datum()` answers with. So a whole authoring
#: arm went unbound behind a name-for-name match. The module docstring
#: says outright that semantics are not checked ("nothing here
#: verifies that Python's `Frame` is the `Frame` the façade
#: curates"); this is that sentence with a bill attached, banked as
#: `work/lib/datum-crosses-name-for-name-as-two-types.md`.
#:
#: What closing it bound: `Node.datum_face_frame(at, face, spin)` — a
#: DAG input, an opaque face name and an angle with no default —
#: `Evaluation.face_carrier_kind`, and `Pose.sense`. The positive form
#: is `tests/test_face_frame.py`, where the three meet: the frame's
#: origin is the pose's, its normal is `sense * axis`, its +x is
#: `u_ref` turned by the spin about that normal, and the face whose
#: kind reads `Torus` is the face the frame refuses.
#: **B-PART is CLOSED and no longer a `gap` id here**
#: (LIB-B-PART), and it is B-FACE-FRAME's lesson arriving twice more,
#: once at each level this file cannot see into. It cited ONE name —
#: `PartSelect`, which is now spelled identically in `pncad.pyi` and
#: so leaves this roster entirely under rule 1 — and its charter named
#: THREE things.
#:
#: `Node.part` is an ARM of a curated enum. `Node` is curated and the
#: stub declares a top-level `Node`, so rule 1 accounts it WHOLE, and
#: which of its two dozen arms Python spells as a constructor is
#: invisible here. That is the same blind spot as `Pose.sense` one
#: level over: a FIELD there, a VARIANT here, and the census compares
#: NAMES, whose insides are nobody's roster.
#:
#: `SlotId.Instance` is a field of a `different-shape` row, and the
#: shape argument is what hid it: "`SlotId` is what
#: `DocEdit.bind_count_param` names implicitly" is true and remains
#: true, and it means a slot with NO door is unreportable. `Instance`
#: had none — `bind_count_param` hardcodes `SlotId::Count`, and its
#: own prose claimed that was "the only structural slot there is",
#: which four Count-dimensioned slots contradict. Closing the family
#: added `DocEdit.bind_instance_param` beside it, one door per slot,
#: keeping the shape decision rather than crossing the enum.
#:
#: That row has since moved into `BOUND_AS` (LIB-DOORS-1), and the
#: paragraph above is unchanged by the move: the door-per-slot
#: decision it records is about ADDRESSING a slot, and what moved is
#: reading one off a refusal.
#:
#: The third thing is not a name at all, and is the measurement worth
#: keeping here: closing this family REQUIRED binding a door the
#: charter never mentioned. `PartSelect.instance` selects out of a
#: plural `Instances` payload, exactly one node emits one
#: (`Node::Pattern`), and that node was deliberately unbound because
#: no downstream door consumed its value. `Node.part` IS that door, so
#: binding the selector without the source would have shipped an
#: unreachable half and an unconstructible refusal tag. Nothing in
#: this file could have reported that: both names are arms behind
#: `Node`. What found it was reading the kernel for the family's
#: INPUTS, and that is a step this census cannot prompt.
#:
#: What closing it bound: `Node.part(of, select)`,
#: `PartSelect.split_half` / `PartSelect.instance`, `Node.pattern`,
#: and `DocEdit.bind_instance_param`. The positive form is
#: `tests/test_part_select.py`.
#: **B-NOTATION is CLOSED and no longer a `gap` id here**
#: (LIB-B-NOTATION), and it is the first family whose roster was
#: HONEST about its own entries: it cited two names, `WrittenLength`
#: and `WrittenAngle`, the family needed exactly those two bound, and
#: both now leave the roster under rule 1. What it could not see was
#: everything ELSE closing them required, which is B-FACE-FRAME's
#: lesson arriving on a family that had no accounting error at all.
#:
#: The measurement first, because the charter's claim was executed
#: rather than repeated: `DocParam.length(25 * mm)` saved
#: `"display_unit": "m"`. The `mm` erases at the `Length` door,
#: because a Python `Length` wraps `quantity::Length` and is canonical
#: metres and nothing else — and it CANNOT be taught the unit, since
#: it carries the arithmetic `quantity::written` refuses to define a
#: notation for. So the notation crosses as the second type, which is
#: what these two entries always named.
#:
#: `Doc.params` is a METHOD of a curated type. `Doc` is curated and
#: the stub declares a top-level `Doc`, so rule 1 accounts it WHOLE —
#: and Python had NO door that answered a document parameter back, not
#: the map and not one by name. `Doc.eval` answers a parameter
#: reference's number with the dimension and the notation both erased,
#: so a document could remember `mm` and no caller could ask. Binding
#: the authoring half alone would have shipped a memory nothing could
#: read except by saving to text and parsing the JSON by hand.
#:
#: `LengthUnit.__eq__` is a MISSING DUNDER, one level further in than
#: a method and invisible to BOTH rosters here: `test_stubs.py` checks
#: stub-declared operators against the compiled class, so an operator
#: neither side declares is unreportable. The unit classes carried no
#: comparison at all, so `mm == mm` was true only by identity, and a
#: unit read back off a value compared unequal to the constant it was
#: written in. A read door for a notation is unusable without it.
#:
#: What closing it bound: `WrittenLength` / `WrittenAngle`,
#: `DocParam.written_length` / `written_angle`, `DocParam.unit`,
#: `Doc.params`, and equality and hashing on `LengthUnit` /
#: `AngleUnit`. `Expr::written_length` needed nothing:
#: `Doc.parse_expr("25 mm")` already reaches `literal_with_unit` and
#: `Expr.text` reads the notation back. The positive form is
#: `tests/test_notation.py`.
#: **B-EXPR-READ is CLOSED and no longer a `gap` id here**
#: (LIB-B-EXPR-READ). It held three names — `eval`, `eval_count` and
#: `EvalError` — and closing it moved NINE, because the three could
#: not be reached without the four the roster filed under `G1`
#: (`Expr`, `ParseError`, `parse_expr`, `unparse`) plus the
#: environment (`ParamEnv`) and the second refusal class. That is the
#: measurement the closing paid for and the one worth keeping: **the
#: entries an id owns are not always the entries a unit must move.**
#: A read door needs the value it reads, and the census had split
#: this family across two ids on WHICH HALF THE AUDIT REACHED rather
#: than on what a unit would have to build — a split the module
#: docstring predicted ("not that a cited id is the RIGHT owner") and
#: this is the first case that executed it.
#:
#: `Expr`, `ParseError` and `EvalError` are top-level names in
#: `pncad.pyi`; `parse_expr`, `eval` and `eval_count` are `BOUND_AS`
#: `Doc` methods and `unparse` is `Expr.text`; `ParamEnv` is
#: `INTERIOR`, corrected from a `gap` it should never have been (see
#: its entry). The positive form is `tests/test_expressions.py`.
#:
#: **G1 stays open**, and the part of its row this unit touched is
#: unchanged in what it measures: "a profile step whose argument is
#: an EXPRESSION rather than a literal" — the door still blocking
#: `plate_param` from scratch, and the one
#: `GeomPred.datum_distance`'s comparand waits on. What changed is
#: that THAT residue is now a SIGNATURE rather than a missing name,
#: so this census cannot see it and does not pretend to;
#: `tests/test_north_star.py` executes an `Expr` against the arc and
#: parameter doors that refuse it, which is the shape every other
#: signature gap on that page is watched in. G1 keeps a citation
#: here regardless, on a different residue of the same row:
#: `ArrivesTangent`, the seam's declared tangent joint.
#:
#: **B-VALIDATE4 is CLOSED and no longer a `gap` id here**
#: (LIB-B-VALIDATE4). It held ONE name, `validate_pseudomanifold`,
#: and closing it moved exactly that one — B-FORMAT's cheap case
#: rather than B-EXPR-READ's expensive one, and the audit-reach check
#: that separates them came back negative again: the door needs a
#: body and a `ContactRecords`, Python has had `Body` since §L4, and
#: no other id owned an entry this unit had to build.
#:
#: **What was NOT already crossing is the second argument, and how it
#: crosses is the family's whole content.** `ContactRecords` has no
#: Python constructor and never will — it is minted by the ops that
#: certify geometry — so a `validate_pseudomanifold(contacts)` door
#: would be a door nobody can call, and one taking ANOTHER body's
#: records would spell precisely the mis-pairing F1 refuses. The
#: records are therefore CAPTURED: a `Body` carries the declarations
#: its producer minted for it, and the Python door is a bare method
#: like the three rungs below it. That is #1668's carrier-projection
#: rule (`fmt_length` landing on the `Length` it needs) at a door
#: whose Rust signature takes two things, and #1664's pairing
#: argument (`RunCtx` becoming `Evaluation`) at a door that would
#: otherwise let a caller ask one body about another's intent.
#: `ContactRecords` accordingly stays `INTERIOR` — the same
#: disposition, now with a second reason under it.
#:
#: **The capture reconciles the kernel's two homes for a record set,
#: and does it where the kernel does.** `NodeValue::contacts` is what
#: `instantiate` carried in (D-1); a boolean's records ride
#: `BooleanValue::Body` instead; `editor_core::product::sources_of`
#: is where the two meet, and `Value.body` / `Value.bodies` now make
#: the same reconciliation so a body read off a value and the same
#: body read by the gather cannot disagree about what was declared
#: over it. `assemble` is the second source (D-1 plus the mates'
#: minted D-2), and `product` is deliberately the third case: it
#: gathers and declares NOTHING, so its body is plain. Those two are
#: the same geometry through two doors, and the suite pins that they
#: answer differently — which is the only way to show from Python
#: that the capture is load-bearing rather than decorative.
#:
#: **One thing the closing measured, and it is a defect the other
#: three rungs could not reach.** Only tier 3′ produces the census
#: arms, and the kernel words three of them out of `Debug` — so the
#: first honest call of this door PANICKED inside the crate's own
#: "never a `Debug` dump" assertion. Filed, not fixed
#: (`work/lib/tier-3-prime-findings-render-through-debug.md`); the
#: raise takes a narrow, single-caller exemption that says why, and
#: the text is pinned so the kernel fix goes red.
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
#: **B-CANCEL is CLOSED and no longer a `gap` id here**
#: (LIB-B-CANCEL) — the last of the queued mechanical sweep, and the
#: one whose accounting says the most about what a charter can and
#: cannot see. It held ONE name, `CancelToken`, and
#: closing it moved that one; by the B-FORMAT arithmetic that is the
#: cheap case. It was not, and the reason is the family's own shape.
#:
#: **A door's ARGUMENT crossing is not the same as its ANSWER
#: crossing, and this charter counted only the first.** B-FORMAT was
#: cheap because `fmt_length`'s arguments already had Python
#: spellings; B-CANCEL's argument is a token nobody had to build
#: either — `CancelToken` is an `Arc<AtomicBool>` with two methods.
#: What was missing was the OTHER end: a canceled run answers by
#: being a PARTIAL `Evaluation`, and Python could not ask an
#: `Evaluation` whether it completed. So the family bound its one
#: chartered name and then had to bind the answer that name makes
#: reachable — `Evaluation.canceled` — before the door meant
#: anything. **A charter that names the ARGUMENT of a missing door
#: undercounts by whatever the door ANSWERS with**, which is the
#: general form of the thing B-EXPR-READ found by splitting on
#: audit reach and B-FORMAT found by not needing to.
#:
#: `EvalOutcome` therefore stays `different-shape` and is the ONE
#: roster entry this family re-cut in place rather than moved: see
#: its entry, which now records a shape it did not have when the
#: disposition was written.
#:
#: The audit-reach check ran first, as it now does on every family,
#: and came back NEGATIVE in the strongest available form:
#: `docs/guide/north-star-audit.md` does not contain the string
#: "cancel" at all. No tour scene interrupts an evaluation, so no
#: audit gap id reaches this door and nothing was filed under another
#: id to reclaim — the census's founding finding in its purest case,
#: and the reason this family was census-owned rather than cited.
#:
#: The positive form is `tests/test_cancellation.py`. Its oracle is
#: the kernel's own contract on a canceled run — full `order()`, the
#: completed prefix, node granularity — read back through the doors
#: that report it, with the DETERMINISTIC arm (a token canceled
#: before the run starts, so the check before the first node fires)
#: as the pin, and the interruption of a run already under way
#: measured rather than threshold-guarded. What that arm needed was
#: not a kernel change: it was releasing the GIL across the kernel
#: call, because a Python thread that cannot run cannot set a flag.
#: **B-FORMAT is CLOSED and no longer a `gap` id here**
#: (LIB-B-FORMAT). It held three names — `fmt_length`, `fmt_angle`
#: and `FmtQuantityError` — and closing it moved exactly those three,
#: which is worth recording only because the family before it did not:
#: B-EXPR-READ chartered three and moved nine. The difference is not
#: luck, it is that this family's pins were ALREADY BOUND.
#: `fmt_length` needs a canonical value and a `LengthUnit`; Python has
#: had `Length` and the seven unit constants since §L4, so there was
#: nothing to build first and no entry filed under another id to
#: reclaim. A family is cheap when its arguments already cross, and
#: that is the only general thing the two closings say together.
#:
#: `FmtQuantityError` is a top-level name in `pncad.pyi` and needed no
#: `BOUND_AS`; `fmt_length` and `fmt_angle` are `BOUND_AS` METHODS on
#: the quantity they format, for the reason recorded at their entry —
#: the receiver is what makes `Length.format(deg)` unspellable. The
#: positive form is `tests/test_quantities.py`, whose oracle is the
#: EXPRESSION PARSER: `Doc.parse_expr(x.format(u))` evaluates back to
#: `x`'s exact bits, which is the formatter's own headline pin
#: checked against the door it names — something the Rust side cannot
#: do, since `quantity` sits below `editor-core` and its fixture has
#: to transliterate the parser's literal rule by hand.
NOT_BOUND = {
    # --- different-shape ------------------------------------------
    "ALL_SURFACE_KINDS": SHAPE,
    "Affine3": SHAPE,
    "Applied": SHAPE,
    "Axis3": SHAPE,
    "AsciiOptions": SHAPE,
    "BinaryHeader": SHAPE,
    "BinaryOptions": SHAPE,
    "BlendRefusal": SHAPE,
    "CONTACT_RECOURSE": SHAPE,
    "CurveKindSet": SHAPE,
    "DeclareError": SHAPE,
    "Dimension": SHAPE,
    "EdgeKey": SHAPE,
    "EditRecord": SHAPE,
    "EvalOptions": SHAPE,
    # A two-variant enum flattened to the boolean that answers it:
    # `Evaluation.canceled`, bound at LIB-B-CANCEL.
    #
    # RE-CUT rather than moved, and the re-cut is the point. This
    # disposition was written when Python's `evaluate` minted a token
    # nobody could reach, so the outcome of a Python evaluation was a
    # CONSTANT — `Completed`, on every run, unconditionally. A
    # constant needs no accessor, so `different-shape` was true
    # vacuously: there was no Python shape, and none was owed.
    # B-CANCEL's `cancel=` made the second variant reachable and the
    # entry therefore owed a real shape, which is the general hazard
    # this file should be read for — a disposition can be honest about
    # a surface and stop being honest when a NEIGHBOURING door opens,
    # with nothing mechanical to say so. Nothing here fails when that
    # happens; only reading the entry beside the door does.
    "EvalOutcome": SHAPE,
    "FIT_DEFERRAL": SHAPE,
    "FaceKey": SHAPE,
    "ImportOptions": SHAPE,
    # The element type of `ImportOptions::declared_contacts`, curated
    # at the prelude because filling a public field means spelling its
    # element type and a Rust caller could not: the import-side
    # declaration channel was callable and not FILLABLE. That defect
    # is a Rust one and it does not reproduce here, which is why this
    # entry is `different-shape` rather than a gap. It follows its
    # carrier one bullet above — `ImportOptions` is `import_step`'s
    # absent second argument — and an element type of an absent
    # argument has strictly less to cross than the argument does.
    #
    # Read this entry beside `import_step` if that second argument is
    # ever bound: at that moment `ImportContact` needs a Python
    # spelling of its own (a constructor for the position anchor), and
    # this row stops being honest in exactly the shape the
    # `EvalOutcome` entry above records.
    "ImportContact": SHAPE,
    "InterrogateError": SHAPE,
    "LineTarget": SHAPE,
    # `continue_to`'s target trait, absorbed into the verb exactly as
    # `LineTarget` and `TangentArcTarget` are — and the verb itself is
    # not bound in Python yet either (the Rust-side roster records
    # that, `surface_census.rs`).
    "ContinueTarget": SHAPE,
    # The fourth arena key, joining the three above it in the same
    # bullet and for their reason: a validation refusal names a ring
    # beside its face, and Python holds neither.
    "LoopKey": SHAPE,
    # The two type-erased sums OVER those keys — "any entity" and "any
    # geometry", the form a refusal reports a site in. Same family as
    # the keys themselves and for exactly their reason: a Python
    # caller holds opaque NAME text and never a key, so a sum over
    # keys has nothing to project either. What the sums' arms say
    # DOES reach Python, at the two doors where the arm is the answer
    # rather than the site: `ReadbackError.variant` is
    # `dangling_entity` or `dangling_geometry`, which is which of the
    # two came back empty (`DanglingRef` is the `BOUND_AS` entry that
    # records it), and `ValidationFinding.entity_kind` is which KIND of
    # carrier a census refusal's entity subject is. Both project the
    # discriminant and neither projects the key, which is why this row
    # does not move: the sum is still a sum over things Python cannot
    # hold.
    "EntityId": SHAPE,
    "GeomRef": SHAPE,
    "Mat3": SHAPE,
    "MassPropsError": SHAPE,
    # WHAT THE CLASSIFIER SAW, curated at the prelude beside the
    # `Indeterminate` that holds it — and a discriminant that crosses
    # as WHICH ATTRIBUTE IS SET rather than as a word.
    #
    # THE MEASUREMENT. The frame constructors' degenerate arm carries
    # an `Option<Indeterminate>` and the binding forks on this type's
    # three arms to publish it: `margin` for a value,
    # `margin_low`/`margin_high` for an enclosure, and neither for a
    # poisoned margin, in every case beside the band's `zero` and
    # `escalate`. So all three arms reach a Python caller and each is
    # distinguishable from the other two — which is what makes the
    # carriage a carriage — but the shape they arrive in is the
    # attribute set of the refusal, not a tag on it. A `margin_diag`
    # word beside them would publish one fact twice, the
    # `frame_error_tag` rule at the arm one rung up.
    "MarginDiag": SHAPE,
    # The attribution walk's verdict, and the door that answers it.
    # Same family as `RolePath`/`RoleSeg` and for their reason: it
    # reads the INSIDE of a name, which nothing user-side may read.
    "NameOrigin": SHAPE,
    "NodeError": SHAPE,
    "NodeResult": SHAPE,
    # The display-unit CODE a `DocParam` carries. A one-byte index into
    # the unit table has no Python spelling and should not get one: a
    # notation reaches Python as its SYMBOL, which is what
    # `DocParam.__repr__` prints.
    "UnitSym": SHAPE,
    "PartialPath": SHAPE,
    # The fillet refusal envelope's entry types. A Python caller reads
    # the same content off `PathError.corners` — one `(x, y, reason)`
    # row per refusing corner, the reason its stable tag — so the Rust
    # enums have no Python spelling of their own.
    #
    # `NoCornerReason` is new to the curated lists and joins them
    # rather than arriving as a gap, because its two arms are already
    # IN those rows: `corner_reason_tag` matches the no-tangent-circle
    # arm one level in, so a corner refuses as `offset_carriers_
    # disjoint` or `no_corner_side_candidate` and never as the arm
    # name alone. It is curated at the prelude for the pair it closes
    # there — the fillet constructor's no-corner reason beside the
    # lattice door's `PathNoCornerReason`, which was carried on its
    # own — and its Python half was never the gap.
    "CornerReason": SHAPE,
    "CornerRefusal": SHAPE,
    "CornerWindow": SHAPE,
    "NoCornerReason": SHAPE,
    "PathNoCornerReason": SHAPE,
    "Point2": SHAPE,
    "Point3": SHAPE,
    "ProfileDoc": SHAPE,
    "ProfileLift": SHAPE,
    "REGENERATE_RECOURSE": SHAPE,
    "Real": SHAPE,
    "RecordedProgramError": SHAPE,
    "ResolveFailure": SHAPE,
    "RevolveAxis": SHAPE,
    "RolePath": SHAPE,
    "RoleSeg": SHAPE,
    "ASSERT_BOUND": SHAPE,
    "SEL_DATUM_DISTANCE": SHAPE,
    "Side": SHAPE,
    "SolidName": SHAPE,
    "SplitSide": SHAPE,
    "StableName": SHAPE,
    "StepExportError": SHAPE,
    "StepOptions": SHAPE,
    "SurfaceKindSet": SHAPE,
    "TagPat": SHAPE,
    "TangentArcTarget": SHAPE,
    "Tol": SHAPE,
    "Tolerance": SHAPE,
    "Vec2": SHAPE,
    "Vec3": SHAPE,
    # The slot vocabulary's 3-vector families, beside `Axis3`: Python
    # addresses a slot through a door that names it, so the Rust enum
    # grouping three of them is not a name a Python caller needs. The
    # COMPONENT is not lost with it — `EditError.slot` spells the axis
    # into the word (`origin_x`), so a family and its axis read off
    # one string rather than off a type Python would have to hold.
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
    # The kernel flush seat (`topo::flush`): the detect/declare
    # protocol over two `Body`s and their arena keys — the same
    # vocabulary Python is deliberately kept from naming. The Python
    # spelling of these questions is the document door, already bound:
    # `Evaluation.find_flush_candidates` and `Doc.declare`.
    "flush": SHAPE,
    "real": SHAPE,
    # The loops-only resolution door the sketch frame created: a caller
    # with loops in hand and no document — a form previewing what it is
    # about to author — asks it rather than inventing a plane node. No
    # Python door has that shape: `Node.profile` always has a document
    # to name a frame in, so there is nothing here Python cannot say.
    "resolve_loops": INTERIOR,
    "v2": SHAPE,
    "v3": SHAPE,
    "write_step": SHAPE,
    # --- behind-a-door --------------------------------------------
    "Band": INTERIOR,
    "BlendKind": INTERIOR,
    # The blend refusal's payload vocabulary, curated at LIB-CUR4 so a
    # prelude-carried `BlendError` is matchable THROUGH the prelude.
    # `INTERIOR` by the rule the two CUR3/CUR4 cases together settle:
    # **a payload's category follows what its CARRIER does at the
    # crossing.** `ReadbackError` projects its arms as tags, so CUR3's
    # `DanglingRef` is in `BOUND_AS` at `ReadbackError.variant` and its
    # arms ARE two tags. `BlendError` projects no arms at all —
    # `node_error_tag` reads the VERB, so the whole refusal arrives as
    # one `fillet`/`chamfer` tag plus the kernel's `Display` prose — so
    # there is no tag to split, none to pin, and nothing for a Python
    # caller to branch on. Not a `gap:` either: the debt is the blend
    # door being unprojected, which is #1479's row and not a missing
    # binding for these four types.
    "BlendSite": INTERIOR,
    "BooleanBody": INTERIOR,
    "BooleanDeclarations": INTERIOR,
    "BooleanResult": INTERIOR,
    "BooleanResultKind": INTERIOR,
    "BooleanValue": INTERIOR,
    # The instantiation seam's declaration bookkeeping. `Relation` and
    # `Route` are the two halves of what a carried finding says, and
    # Python reads both without holding either type:
    # `Attribution.relation` is `carried_refuted` / `carried_declined`
    # for a declaration a document below authored, and
    # `Attribution.of` / `.via` are its route. The same pair rides
    # `AssemblyError` with `variant == "carried_mint_refusal"` and
    # `Assembly.carried`'s rows. `CarriedDeclarations` is the
    # evaluation VALUE channel's bundle, behind `Product` and
    # `NodeValue`, both interior.
    "CarriedDeclarations": INTERIOR,
    "Relation": INTERIOR,
    "Route": INTERIOR,
    "Chamfered": INTERIOR,
    "ContactRecords": INTERIOR,
    # The contact vocabulary's fourth quarter, curated beside the
    # three that were already here. `INTERIOR` by the carrier rule,
    # measured one rung DOWN from where its carrier now crosses: it is
    # `CensusContact::ConformalPatch`'s payload, and `CensusContact`
    # itself reaches Python as `ValidationFinding.contact_kind` —
    # `conformal_patch`, the word — while what that arm CARRIES stays
    # behind it. The finding is a declared pair and a verdict, and
    # projecting it would be projecting a payload's payload; the
    # discriminant is what a curated list owes and it crosses.
    # (`FlushFinding`, which Python DOES hold, is the detector's own
    # type and not this one — a pair, a class and the evidence — so it
    # settles nothing here either way.) `DeclaredContact` and
    # `ContactVerdict` — this type's two fields — sit at `INTERIOR`
    # beside it, which is the sibling test the `CensusContact` entry
    # names.
    "ContactFinding": INTERIOR,
    "ContactRefusal": INTERIOR,
    "ContactVerdict": INTERIOR,
    # THE PROFILE REFUSALS' PAYLOAD VOCABULARY, curated at the prelude
    # so a prelude-carried `ProfileError`, `CornerReason` or
    # `PathError` is matchable THROUGH the prelude rather than only
    # nameable one module hop away.
    #
    # `INTERIOR` by the carrier rule, and the measurement is that both
    # carriers cross as words with their FIELDS left behind.
    # `ProfileError` reaches Python at `EvaluationError.inner_kind` —
    # `non_simple`, `escalated`, one word per arm — so which contact
    # two segments made, which stage escalated and which segment it
    # was are in the kernel's prose and nowhere else. `PathError`
    # reaches it with a `corners` list, and that list carries the
    # corner's point and its reason word: the anchor arm flattens to
    # `anchor_outside_trimmed_extent`, so the side and its carrier
    # kind — and with the carrier kind, whether the setback beside it
    # is a distance or an arc length — do not cross either.
    #
    # Not a `gap:`, for the reason the blend four are not: the debt is
    # a door projecting its arms' FIELDS, which is
    # `work/lib/pncad-py-seven-doors-lack-field-projection.md`'s
    # (the `path` door is one of the six it names), and not a missing
    # binding for these five types.
    "ContactKind": INTERIOR,
    "EscalationSite": INTERIOR,
    "FilletLeg": INTERIOR,
    "FilletLegCarrier": INTERIOR,
    "SegmentRef": INTERIOR,
    "ContentBits": INTERIOR,
    # `BlendError::ConvexitySignFlip`'s payload and
    # `UnsupportedCorner`'s, the other two of the blend four.
    "Convexity": INTERIOR,
    "CornerConfig": INTERIOR,
    "Curve3": INTERIOR,
    "DeclaredContact": INTERIOR,
    "DuplicateName": INTERIOR,
    # What an edge's locus IS, read back off a certified carrier.
    # Python holds an opaque `Body` handle and no door on it answers a
    # description, so nothing here is a thing Python can read.
    #
    # `MappedCurve` — the `Scaffold` arm's payload — is not a curated
    # name and so not an entry here; the argument for stopping at that
    # rung is written where this type is carried, in `prelude.rs`
    # group 4, and it is an argument about the ARM rather than about
    # the crossing: the scaffolding door is fenced to construction and
    # tier 3 refuses it at rest.
    "EdgeDescription": INTERIOR,
    "Extruded": INTERIOR,
    "Extrusion": INTERIOR,
    "FilletLegShape": INTERIOR,
    "Filleted": INTERIOR,
    "FlushEvidence": INTERIOR,
    "Lofted": INTERIOR,
    "LoopProgram": INTERIOR,
    # A11's member vocabulary, and the structural answer it gives.
    # `member_of` is the admission rule an authoring door gates on:
    # which instance a mate head names, and which pattern copy of it.
    # A Python caller reaches that answer through the door it was
    # going to call anyway — every mate door refuses a head outside
    # the vocabulary with `MateFault::DanglingHead`, which is bound and
    # dispatchable. What needs the PREDICATE rather than the refusal is
    # an interactive picker, which must refuse before it proposes; that
    # is the Rust caller this door was made public for.
    #
    # So `Member` is behind a door in the plainest sense — nothing in
    # Python hands one out, and no bound door takes one. Read this
    # entry beside a Python picker if one is ever written: that is the
    # neighbouring door whose opening would make this disposition stop
    # being honest, in exactly the shape `EvalOutcome`'s entry records.
    "Member": INTERIOR,
    # The one thing `Frame::rotate_then_translate` refuses, and the
    # only thing `EditError::PlacementAxis` converts from — a type so
    # that no other node refusal can reach a caller wearing the axis's
    # words. Python never holds one: the constructor raises EditError
    # with variant `placement_axis` and the axis's own prose, which is
    # the whole of what the type carries.
    "AxisRefusal": INTERIOR,
    # The wrapper `MateFault::PlacerRefused` and
    # `EditError::PlacementAxis` carry an evaluation refusal in, so the
    # document layer's two error enums can hold one unaltered. Python
    # never holds the wrapper: both doors project the refusal as the
    # SAME tag word a node failure crosses with — `MateFault.error`
    # and `EvaluationError.kind` are one vocabulary — and the prose is
    # the fault's own `str()`. Nothing in Python hands one out and no
    # bound door takes one.
    "NodeRefusal": INTERIOR,
    # DI3's pairing payload: the two document ids behind a refused
    # pair — a prior the memo dropped (`Evaluation.prior_refused` on
    # the Rust side), a gather handed the wrong evaluation, a solve
    # handed the wrong document. Nothing in Python hands one out. The
    # two ERRORING doors project their arm as a tag word
    # (`evaluation_of_another_document`,
    # `mate_poses_of_another_document`), which is what a Python caller
    # branches on; the memo's arm is not an error at all, and the fact
    # it records reaches Python where it always did, as
    # `Evaluation.reused` being 0 with every node recomputed. Not a
    # `gap:`: the debt, if there is one, is the `evaluate` door's
    # PROSE about a foreign prior, which `pncad.pyi`'s own paragraph
    # owns, not a missing binding for this payload.
    "Mispaired": INTERIOR,
    "NameTable": INTERIOR,
    "Operand": INTERIOR,
    # The evaluation environment, and the entry that had been on the
    # WRONG side of the line: it was listed as a `G1` gap on the
    # reasoning that "`select_where` takes one, so a caller who cannot
    # spell the type cannot call the door" — which is the RUST door's
    # shape. Python's `select_where` never took one; `Evaluation`
    # captures the environment at `evaluate` so the answer is as of
    # the same document the evaluation is of. LIB-B-EXPR-READ put a
    # second door on that footing rather than changing it: `Doc.eval`
    # builds the environment from the document it is a method on. Two
    # doors, both of them holding one internally, neither handing it
    # to Python — which is what `behind-a-door` means.
    "ParamEnv": INTERIOR,
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
    # `Revolved::kind` — the ratified case split, curated at the
    # prelude because the two arms are two disjoint sets of handles
    # and a Rust caller reading the wedge caps off a partial revolve
    # has to branch. `INTERIOR` by the carrier rule and by the
    # plainest measurement of it in this file: the carrier does not
    # cross AT ALL. Python speaks the document layer, so a revolve is
    # a `Node.revolve` whose answer is a body — no `Revolved` value
    # reaches Python, and `pncad-py` names neither type anywhere.
    #
    # Not a `gap:` either: what a Python caller wants out of those
    # handles is the FACES, and it asks for those by name through
    # `Evaluation.select` rather than by key. A key bundle has nothing
    # to project to a surface that holds names and never keys.
    "RevolvedKind": INTERIOR,
    # `BlendError::UnsupportedCorner`'s second field, the policy
    # `CornerConfig::policy` assigns (LIB-CUR4).
    "RunOutPolicy": INTERIOR,
    "SegmentKind": INTERIOR,
    "StepArg": INTERIOR,
    "Surface": INTERIOR,
    "ValidatedLoop": INTERIOR,
    "ValidatedProfile": INTERIOR,
    "edge_name": INTERIOR,
    "face_name": INTERIOR,
    # The predicate; `Member` above carries the argument for both.
    "member_of": INTERIOR,
    "validated": INTERIOR,
    # **The gathered-product doors, one family, and they are what the
    # binding CALLS.** `Product` is the document's product with
    # everything the gather knows about it, `product_recorded` is the
    # gather that builds one, `Subject` is what the check registry runs
    # over, and `run_checks_on` / `assemble_gathered` are the two doors
    # that take a product a caller already holds.
    #
    # A Python `Evaluation` is the immutable (document, evaluation)
    # pair captured at `evaluate`, and a product is a pure function of
    # that pair and the run's tolerance — so the evaluation gathers
    # ONCE and memoizes, and every bound door that wants a product
    # (`run_checks`, `assemble`, `product`, `product_named`) reads that
    # one gather through exactly these five names
    # (`crates/pncad-py/src/product_memo.rs`). They are `INTERIOR`
    # because the memo removes the question a Python caller would have
    # held a `Product` to ask: there is nothing to hand between doors,
    # so there is nothing to name.
    #
    # That is what changed. The cost this entry used to record — both
    # questions on one evaluation gathering twice — is gone, and so is
    # the reason it stood: `assemble_gathered` CONSUMES its product, so
    # the memo hands it a COPY and keeps the original, measured at
    # about a fiftieth of the gather it saves at the heat sink's
    # 160-fin point. An ownership order still does not cross this
    # boundary; nothing has to cross it any more.
    "Product": INTERIOR,
    "Subject": INTERIOR,
    "assemble_gathered": INTERIOR,
    "product_recorded": INTERIOR,
    "run_checks_on": INTERIOR,
    # The gather's debug-only witness: how many products this thread
    # has gathered, for a consumer asserting it gathers once per
    # operation. Not a question about a document at all.
    #
    # The binding is now such a consumer — one gather per evaluation,
    # whatever a caller asks — and it asserts it on this counter, from
    # RUST: `crate::tests::product_memo_rows` drives the very functions
    # the four doors call, on the default build path where every
    # code-tier run executes them. Binding a read for the python suite
    # instead would put a profile-dependent number on the public
    # surface (the counter leaves with the `debug-assertions` stanza at
    # publish), for a question no user of the library has.
    #
    # `cfg(debug_assertions)` gates the counter, the increment and this
    # reader alike — which is NOT the same as "absent from a release
    # build" in this workspace, whose `[profile.release]` keeps
    # `debug-assertions` on until publish. Every binary this repo
    # produces carries it; cargo's own release defaults are what strip
    # it. Either way it is not a Python door.
    "gathers_on_this_thread": INTERIOR,
    # --- gap: the SWEEP half of G2, still banked -------------------
    # The tube half closed at LIB-TUBE; `sweep_body` did not, and the
    # reason is not "no binding was written" — `wire_sweep` refuses
    # unconditionally (U4/LQ3, kernel-owned). Binding a door that
    # cannot succeed would move a name out of this list without
    # moving anything a caller can do.
    "sweep_body": f"{GAP}: G2 sweep (wire_sweep banked on U4/LQ3)",
    # --- gap: parameter distributions and the analysis lane -------
    # --- the measurement authoring vocabulary, closed --------------
    # `SitedRef` is bound where a reference is AUTHORED — `Node.mate`
    # takes each of its two sides as a node and a name, and
    # `Node.measure` takes a LIST of the same pair, because a mate has
    # exactly two sides and a measure's arity is its reference list.
    # The type itself is therefore never handed across, and a class
    # wrapping the two halves would add ceremony and no reach: a name
    # is opaque text by the ordinal-28 contract and a read site is a
    # node id. This row was a `gap:` until LIB-B-MEASURES, on the
    # reading that the measure half would hand one over; it does not,
    # and this is the same sentence the entry already carried, now
    # true of both doors.
    "SitedRef": SHAPE,
    # **The clearance engine's refusal, flattened to a tag — and
    # unreachable at the lane Python evaluates on.** It reaches Python
    # as `EvaluationError.kind == "measure_clearance_refused"`
    # (`src/tags.rs`), which is this bullet's ordinary shape. What is
    # NOT ordinary is that no Python evaluation can produce one: the
    # refusal's only producer is
    # `impl MinClearanceLane for geom_core::Interval`, and the binding
    # evaluates at `f64` alone (`src/py/value.rs`), so the FEATURE is
    # not what gates it — the SCALAR is, and `pncad-py --features
    # interval` reaches it no better. That is `profile_lift`'s
    # sentence above arriving on the refusal side: the door starts
    # answering differently exactly when Python gains a non-`f64`
    # evaluation, and it should gain its spelling in the unit that
    # brings one. Binding an exception class nothing raises would move
    # a name off this roster without moving anything a caller can do,
    # which is `sweep_body`'s argument two bullets up.
    #
    # Its SIBLING went the other way and the pair is the measurement:
    # `MeasureUnavailableAt` is what the `f64` lane answers a
    # `min_clearance` WITH, so it is reachable today and is bound
    # top-level under rule 1. One kernel file, one verb, two refusals,
    # and the lane decides which of them a Python caller can ever see.
    "MinClearanceRefusal": SHAPE,
    # B-FACE-FRAME IS GONE FROM THIS ROSTER, closed at
    # LIB-B-FACE-FRAME, and the id left `FAMILIES` with it. It cited
    # exactly ONE name here — `face_carrier_kind` — which is now in
    # `BOUND_AS` as `Evaluation.face_carrier_kind`. The family's other
    # two charter names were never rows and could not be; the closure
    # paragraph in this constant's docstring says why, because that
    # gap between a three-door charter and a one-row roster is the
    # measurement worth keeping.
    # B-PART IS GONE FROM THIS ROSTER, closed at LIB-B-PART, and the
    # id left `FAMILIES` with it. It cited exactly ONE name here —
    # `PartSelect` — which now leaves the roster ENTIRELY rather than
    # moving to `BOUND_AS`: `pncad.pyi` declares a top-level
    # `PartSelect` with the same two arms, so rule 1 accounts it, the
    # way it accounts the sibling selector vocabulary `PatternKind`.
    # The closure paragraph in this constant's docstring says what the
    # family's other two charter names were, and why neither was ever
    # a row.
    # B-NOTATION IS GONE FROM THIS ROSTER, closed at LIB-B-NOTATION,
    # and the id left `FAMILIES` with it. It cited TWO names here —
    # `WrittenLength` and `WrittenAngle` — and both leave the roster
    # ENTIRELY rather than moving to `BOUND_AS`: `pncad.pyi` declares
    # both at the same spelling, so rule 1 accounts them. The closure
    # paragraph in this constant's docstring records what a roster
    # honest about its own two entries still could not see.
    # B-DISTRIBUTIONS IS GONE FROM THIS ROSTER, closed at
    # LIB-B-DISTRIBUTIONS, and the id left `FAMILIES` with it. It cited
    # THREE names and they left three different ways, which is the
    # rule demonstrated on one family: `Distribution` and
    # `DistributionFault` are top-level in `pncad.pyi` at those exact
    # spellings, so rule 1 accounts them and they leave the roster
    # entirely; `DistributionField` is in `BOUND_AS` as
    # `DistributionFault.field`, because a three-member set naming
    # struct fields crosses as the word on the fault rather than as a
    # class.
    #
    # THE CHARTER'S OTHER HALF WAS NEVER A ROW HERE AND COULD NOT
    # BECOME ONE. It named the three analysis doors — the analyzed
    # box, tail mass, leaf mass — and every one of them is curated in
    # `crates/pncad/src/analysis.rs`, which is not one of the three
    # files this census reads (the module docstring says so: it reads
    # the three that curate the DOCUMENT layer and the common
    # surface). So `analyzed_box`, `tail_mass`, `box_mass`,
    # `AnalysisPolicy`, `AnalyzedBox`, `AnalyzedParam`,
    # `MeasureUnavailable`, `OffsetInterval`, `DEFAULT_QUANTILE_MASS`
    # and `sample_offset` are invisible to this file in BOTH
    # directions: unbound they flipped no row, and bound they flip none
    # either. The `Distribution` row is what made the family
    # dispatchable at all, and the doors that closed it are reported in
    # the unit rather than counted here. That is the B-FACE-FRAME gap
    # between a charter and a roster, in its widest form so far: three
    # of the charter's four things outside the alphabet.
    #
    # THE GATE, MEASURED, because a reader of the roster cannot see it
    # either: the analysis façade splits at
    # `crates/pncad/src/analysis.rs:49` (ungated) and `:55`/`:66`/`:83`
    # /`:89`/`:104` (`#[cfg(feature = "interval")]`). All three
    # chartered doors are on the ungated line, so the family closes on
    # the DEFAULT build the wheel is made from; what is gated is the E6
    # driver with its `ParamBox`, the E4/E5 stackup, `assertion_at` and
    # the E10 reporting layer, none of which this family chartered.
    # The positive form is `tests/test_distributions.py`.
    # B-MEASURES IS GONE FROM THIS ROSTER, closed at LIB-B-MEASURES,
    # and the id left `FAMILIES` with it — which emptied that map, the
    # last census-owned charter closing. It cited SEVEN names and they
    # left three different ways, which is one more way than
    # B-DISTRIBUTIONS demonstrated:
    #
    #   - FIVE leave the roster ENTIRELY under rule 1, because
    #     `pncad.pyi` declares each top-level at the same spelling:
    #     `MeasureExpr`, `MeasurePrimitive` and `AssertionDir` as the
    #     authoring vocabulary, and `MeasureNodeFault` and
    #     `MeasureUnavailableAt` as exception classes keeping their
    #     Rust types' own names.
    #   - TWO stay here and are RETAGGED `SHAPE` — `SitedRef` and
    #     `MinClearanceRefusal`, each argued at its own entry above.
    #     Neither is a debt any more and neither is reach: one is a
    #     type whose two halves are what the doors take, the other a
    #     refusal flattened to a tag no `f64` evaluation can raise.
    #
    # THE CHARTER NAMED A SPELLING THIS FILE HAD ALREADY TAKEN. The
    # analysis lane's `MeasureUnavailable` bound at LIB-B-DISTRIBUTIONS
    # one unit earlier, and this family's `MeasureUnavailableAt` is a
    # DIFFERENT kernel type answering a different question — the band
    # that states limits without a shape, against the point scalar
    # with nowhere to put an enclosure. Both keep their own Rust
    # names, neither subclasses the other, and the two tag functions
    # are deliberately not one function.
    #
    # THE GATE, MEASURED, and it is not the one B-DISTRIBUTIONS found:
    # every name this family owns is curated in
    # `crates/pncad/src/document.rs`, which carries NO `cfg`, so
    # nothing here is behind `interval` on the façade. The limit is a
    # LANE — see the `MinClearanceRefusal` entry — and it bites one
    # refusal out of the family's whole surface.
    #
    # WHAT THIS FILE COULD NOT SEE, in both directions. `Node::Measure`
    # and `Node::Assertion` are ARMS of `Node`, which rule 1 accounts
    # whole, so two of the kernel's twenty-five recipe node kinds were
    # unconstructible from Python for the life of the binding and no
    # roster here said so. `Doc.node_kind` answered `"measure"` and
    # `"assertion"` the whole time — a committed vocabulary exhaustive
    # over the kernel enum cannot tell a word a caller can reach from
    # one it cannot. `MeasureNodeFault`'s one arm sat behind the edit
    # tag `measure_malformed`, and nine more measurement arms sit
    # behind `EvaluationError.kind`, which is a `BOUND_AS` mapping
    # this file accounts whole. The positive form is
    # `tests/test_measures.py`.
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
    # B-PICKING IS GONE FROM THIS ROSTER, closed at LIB-B-PICKING, and
    # the id is gone from `FAMILIES` with it. Five of its seven names
    # are top-level in `pncad.pyi` name for name (`Ray`, `PickHit`,
    # `NodePick`, `NodePickError`, `HitTestError`); two are in
    # `BOUND_AS` — `pick_face` as `Evaluation.pick_face`, and
    # `PickTarget` as `NodePick`, the value that plays its role in a
    # language where a raw target has no constructor. The positive
    # form is `tests/test_picking.py`.
    # B-RESOLVE IS GONE FROM THIS ROSTER, closed at LIB-B-RESOLVE, and
    # the id is gone from `FAMILIES` with it. One of its three names is
    # top-level in `pncad.pyi` name for name (`Resolution`); two are in
    # `BOUND_AS` — `resolve` as `Evaluation.resolve`, and `RunCtx` as
    # `Evaluation`, which is the (document, evaluation) pair the Rust
    # type is. The positive form is `tests/test_resolve.py`.
    # --- gap: the seam's declared arrival (audit G1) ---------------
    # The token that DECLARES the seam's tangent joint (PATHS-DESIGN §6;
    # ruled 2026-09-02 — every zero-turn joint is a declared tangent
    # joint, so there is one token rather than two). Not absorbed into
    # the verbs the way `LineTarget`/`ContinueTarget`/`TangentArcTarget`
    # are: those are traits naming what a parameter accepts, while this
    # is a VALUE a caller writes — `line_to(Start.arrives_tangent())` —
    # so a Python author who cannot name it cannot author a loop whose
    # seam is a declared tangent joint. Same family as the profile
    # lattice itself, and it rides beside `continue_to`, which the
    # Rust-side surface census records as unbound for the same reason
    # (`surface_census.rs`'s `Spelling::NotBound`).
    "ArrivesTangent": f"{GAP}: G1 the seam's declared tangent joint",
    # B-EXPR-READ IS GONE FROM THIS ROSTER, closed at
    # LIB-B-EXPR-READ, and the id is gone from `FAMILIES` with it.
    # `EvalError` is a top-level name in `pncad.pyi` and needed no
    # `BOUND_AS`; `eval` and `eval_count` are in `BOUND_AS` as `Doc`
    # methods, because the environment they read is the document's.
    #
    # The FIVE G1 entries left with them, and that is the decay rule
    # rather than a re-assignment: `Expr`, `ParseError`, `parse_expr`
    # and `unparse` are names Python now spells, and a `gap:` entry
    # Python binds is stale whatever id it cites. `ParamEnv` moved
    # for the OTHER reason — it is `INTERIOR` now, below, because
    # both doors that take one build it from the document in hand.
    # **G1 is not closed by any of that**, and it did not stop being
    # cited here either: `ArrivesTangent` above carries the id now,
    # for a residue of the SAME row that has nothing to do with
    # expressions. What the expression half's residue became is a
    # SIGNATURE rather than a missing name — no door takes an `Expr`
    # INTO a document — so this census structurally cannot watch that
    # half and does not pretend to; `tests/test_north_star.py` does,
    # by executing an `Expr` against the arc and parameter doors that
    # still refuse it. The positive form is
    # `tests/test_expressions.py`.
    # --- gap: geometry read-back doors (census-owned) -------------
    # --- gap: assorted single doors -------------------------------
    # B-CANCEL IS GONE FROM THIS ROSTER, closed at LIB-B-CANCEL, and
    # the id is gone from `FAMILIES` with it. `CancelToken` is a
    # top-level name in `pncad.pyi` at that exact spelling and needed
    # no `BOUND_AS`; it was the family's only entry and it left alone.
    # What it did NOT leave alone is `EvalOutcome`, three screens up,
    # whose `different-shape` was vacuous until this door opened — the
    # paragraph above says why a charter naming a door's ARGUMENT
    # undercounts by whatever that door ANSWERS with.
    # B-FORMAT IS GONE FROM THIS ROSTER, closed at LIB-B-FORMAT, and
    # the id is gone from `FAMILIES` with it. All three of its names
    # left and nothing else moved with them — the paragraph above says
    # why that is the boring case and B-EXPR-READ's was not.
    # B-VALIDATE4 IS GONE FROM THIS ROSTER TOO, closed at
    # LIB-B-VALIDATE4, with its id gone from `FAMILIES`. It held ONE
    # name and moved exactly that one — B-FORMAT's case, for
    # B-FORMAT's reason (its arguments already crossed), except that
    # the second argument crossed by being CAPTURED rather than by
    # already having a spelling; `BOUND_AS` says how, and
    # `tests/test_validate.py` is the positive form.
    # B-NAME-BUILDERS IS GONE FROM THIS ROSTER, closed at LIB-PYNAMES,
    # and the id is gone from `FAMILIES` with it. All five names left
    # and none needed a `BOUND_AS`: `pncad.pyi` declares `band`,
    # `band_pi`, `band_rim`, `meridian_vertex` and `carried` at those
    # exact spellings, so rule 1 accounts them. It is B-FORMAT's case
    # with one difference worth the line: the family was chartered
    # over a door's ANSWER rather than its argument — the name TEXT
    # `Node.fillet` and `Node.shell` already took — so nothing else
    # moved with them and nothing behind them was undercounted.
    # `tests/test_role_names.py` is the positive form.
}


#: Members a matched type's Python namesake does not spell, and the Python
#: spelling that answers the same question. Keyed `Type::Member`, and each
#: value is VERIFIED to exist in the stub exactly as `BOUND_AS`'s is — a row
#: naming a spelling `pncad.pyi` does not declare fails.
#:
#: WHY A SECOND TABLE AND NOT `Type::Member` KEYS IN `BOUND_AS`. Every check
#: over that roster reads its keys as CURATED NAMES: `test_the_rosters_decay`
#: fails a key the façade no longer exports, and
#: `test_every_curated_name_is_bound_or_listed` reads one as the accounting
#: of a name. `Node::Union` is not a curated name and never will be — it is a
#: member of one. Two questions, two tables, each decaying against the set it
#: is actually about, which is the same reason `BOUND_AS` and `NOT_BOUND` are
#: two tables rather than one column.
#:
#: THREE SHAPES ACCOUNT FOR ALL OF IT.
#:
#: - **An arm that crosses as a TAG WORD**, which is most of this table.
#:   Python's exceptions carry their refusal as ATTRIBUTES, and the arm is a
#:   `variant`/`kind` STRING rather than a bound payload class — the
#:   flattened-payload bullet of `NOT_BOUND`, one level in. Each word is
#:   minted by an exhaustive match in `crates/pncad-py/src/tags.rs`, so a
#:   kernel arm added without one stops the bindings compiling; what the row
#:   adds is the other direction, that a reader of the ARM can find the word.
#:   A few VALUE enums cross the same way and are noted where their attribute
#:   is not called `variant`.
#: - **An arm that crosses as a CONSTRUCTOR** — the `Node::Extrude` ->
#:   `Node.extrude` shift, at the arms rule 1 could not see: `Datum`'s are
#:   `Node.datum_*`, and `DocEdit::SetStructuralParam` is one door per slot.
#: - **A field that crosses as a READER**, the struct half: `Ray::dir` is
#:   `Ray.direction`, `Mesh::patches` is `Mesh.patch`.
#:
#: A row claims what a `BOUND_AS` row claims and no more: a Python caller can
#: reach what that member is about, at that spelling. Not the same shape, not
#: the same receiver, and nothing about semantics.
MEMBERS_BOUND_AS = {
    # --- an arm that crosses as a TAG WORD -------------------------
    "AssemblyError::Product": "AssemblyError.variant",
    "AssemblyError::Reference": "AssemblyError.variant",
    "AssemblyError::NoAtRestRecord": "AssemblyError.variant",
    "AssemblyError::CarriedMintRefusal": "AssemblyError.variant",
    "AssemblyError::AtRest": "AssemblyError.variant",
    "AssemblyError::Uncertified": "AssemblyError.variant",
    # A VALUE's arms, not a refusal's, and the word is `relation` because
    # what the walk answers is how a declaration stands to the document
    # it was gathered from.
    "Attribution::Refuted": "Attribution.relation",
    "Attribution::Declined": "Attribution.relation",
    "Attribution::Carried": "Attribution.relation",
    "Attribution::Unattributed": "Attribution.relation",
    # The registry's evidence, a value: which resident found what.
    "CheckEvidence::Connectedness": "CheckEvidence.variant",
    "CheckEvidence::Escalated": "CheckEvidence.variant",
    "CheckEvidence::Unsupported": "CheckEvidence.variant",
    "CheckEvidence::StaleExpectation": "CheckEvidence.variant",
    "CheckEvidence::NotSeparated": "CheckEvidence.variant",
    "CheckEvidence::SeparationUnavailable": "CheckEvidence.variant",
    "ChecksError::Root": "ChecksError.variant",
    "ChecksError::Band": "ChecksError.variant",
    "ChecksError::EvaluationOfAnotherDocument": "ChecksError.variant",
    "ChecksError::Product": "ChecksError.variant",
    # `Mints` is the arm the namesake spells (`ClassAdmission.mints`);
    # the other two are the word.
    "ClassAdmission::NoAtRestRecord": "ClassAdmission.variant",
    "ClassAdmission::NotAdmitted": "ClassAdmission.variant",
    # What an accepted edit did to the placement registry, read off
    # `Doc.last_maintenance`.
    "ClusterMaintenance::Join": "ClusterMaintenance.variant",
    "ClusterMaintenance::Split": "ClusterMaintenance.variant",
    "ClusterMaintenance::GaugeRewrite": "ClusterMaintenance.variant",
    "ClusterMaintenance::Drop": "ClusterMaintenance.variant",
    # THE SECOND SAME-SPELLED PAIR, and this rule is what found it.
    # `pncad.pyi`'s `DimensionError` is the QUANTITY boundary's refusal —
    # `1 * m + 1 * rad`, with `op`/`left`/`right` — while the curated
    # name is `editor_core`'s document-layer refusal, a different type
    # answering a different question. Rule 1 matched them on spelling
    # alone, exactly as it matched the two `Datum`s. The ten arms cross
    # where the class's own docstring says they do: `Doc.parse_expr`
    # raises `ParseError` with `variant == "dimension"` and the
    # mismatch's own tag as `kind`, which is the one of the three
    # crossings that keeps it branchable (literal construction is
    # `LiteralError.kind`; a save file's arrives as `PersistError`
    # `variant == "parse"`, issue #694).
    "DimensionError::Mismatch": "ParseError.kind",
    "DimensionError::MulNeedsScalar": "ParseError.kind",
    "DimensionError::DivNeedsScalarDivisor": "ParseError.kind",
    "DimensionError::TrigNeedsAngle": "ParseError.kind",
    "DimensionError::CountNeedsExplicitPromotion": "ParseError.kind",
    "DimensionError::NotCount": "ParseError.kind",
    "DimensionError::LiteralCountIsInteger": "ParseError.kind",
    "DimensionError::NonFiniteLiteral": "ParseError.kind",
    "DimensionError::DisplayUnitMismatch": "ParseError.kind",
    "DimensionError::UnknownDisplayUnit": "ParseError.kind",
    "DistributionFault::NonFinite": "DistributionFault.variant",
    "DistributionFault::SigmaNotPositive": "DistributionFault.variant",
    "DistributionFault::NominalOutsideSupport": "DistributionFault.variant",
    "EditError::UnknownNode": "EditError.variant",
    "EditError::ProfileProgramRefused": "EditError.variant",
    "EditError::UnresolvedInput": "EditError.variant",
    "EditError::WouldCycle": "EditError.variant",
    "EditError::DuplicateInput": "EditError.variant",
    "EditError::RepeatedDesignation": "EditError.variant",
    "EditError::SetMembersOnNonList": "EditError.variant",
    "EditError::TooFewMembers": "EditError.variant",
    "EditError::DeleteWouldDangle": "EditError.variant",
    "EditError::UnknownSlot": "EditError.variant",
    "EditError::SlotDimensionMismatch": "EditError.variant",
    "EditError::StructuralSlotNeedsStructuralEdit": "EditError.variant",
    "EditError::NotStructuralSlot": "EditError.variant",
    "EditError::UnknownPayloadParam": "EditError.variant",
    "EditError::PayloadParamDimensionMismatch": "EditError.variant",
    "EditError::MeasureMalformed": "EditError.variant",
    "EditError::AssertionTarget": "EditError.variant",
    "EditError::DeclareInputNotDeclare": "EditError.variant",
    "EditError::AssertionDimension": "EditError.variant",
    "EditError::UnknownDocParam": "EditError.variant",
    "EditError::DocParamDimensionMismatch": "EditError.variant",
    "EditError::ContinuousParamCannotBeCount": "EditError.variant",
    "EditError::DocParamNotDeclared": "EditError.variant",
    "EditError::DocParamValueKindMismatch": "EditError.variant",
    "EditError::PathOffTree": "EditError.variant",
    "EditError::Dimension": "EditError.variant",
    "EditError::DeclareNamesMissingNode": "EditError.variant",
    "EditError::ReadSiteMissingNode": "EditError.variant",
    "EditError::NonFiniteDocParam": "EditError.variant",
    "EditError::InvalidDistribution": "EditError.variant",
    "EditError::RebindTargetMissingNode": "EditError.variant",
    "EditError::RebindUnknownName": "EditError.variant",
    "EditError::RebindKindMismatch": "EditError.variant",
    "EditError::RebindIdentity": "EditError.variant",
    "EditError::RebindNoReferences": "EditError.variant",
    "EditError::WitnessOnNonSketch": "EditError.variant",
    "EditError::DuplicateWitnessEntry": "EditError.variant",
    "EditError::EmptyWitnessBulk": "EditError.variant",
    "EditError::NameUnresolvedInEvaluation": "EditError.variant",
    "EditError::RebindAppearanceCollision": "EditError.variant",
    "EditError::AppearanceWrongKind": "EditError.variant",
    "EditError::AppearanceNamesMissingNode": "EditError.variant",
    "EditError::AppearanceNotSet": "EditError.variant",
    "EditError::InvalidTolerance": "EditError.variant",
    "EditError::MetaUnversioned": "EditError.variant",
    "EditError::MetaNonFinite": "EditError.variant",
    "EditError::MetaNotSet": "EditError.variant",
    "EditError::RebindMetadataCollision": "EditError.variant",
    "EditError::Roots": "EditError.variant",
    "EditError::PlacementOnNonInstance": "EditError.variant",
    "EditError::PlacementRuleMismatch": "EditError.variant",
    "EditError::EmptyPlacementList": "EditError.variant",
    "EditError::ImproperPlacement": "EditError.variant",
    "EditError::NonFinitePlacement": "EditError.variant",
    "EditError::PlacementAxis": "EditError.variant",
    "EditError::NonFiniteAlignment": "EditError.variant",
    "EditError::UpdateOnNonInstance": "EditError.variant",
    "EditError::PinUnchanged": "EditError.variant",
    "EvalError::UnknownParam": "EvalError.variant",
    "EvalError::ParamDimensionMismatch": "EvalError.variant",
    "EvalError::CountExprInContinuousEval": "EvalError.variant",
    "EvalError::ContinuousExprInCountEval": "EvalError.variant",
    "EvalError::CountOverflow": "EvalError.variant",
    "EvalError::CountToScalarOutOfRange": "EvalError.variant",
    "EvalError::NonFiniteResult": "EvalError.variant",
    "FmtQuantityError::NonFinite": "FmtQuantityError.variant",
    "HitTestError::NodeNotEvaluated": "HitTestError.variant",
    "HitTestError::NodeFailed": "HitTestError.variant",
    "HitTestError::NodePoisoned": "HitTestError.variant",
    "HitTestError::Unnamed": "HitTestError.variant",
    "InlineError::UnknownNode": "InlineError.variant",
    "InlineError::NotAnInstance": "InlineError.variant",
    "InlineError::InstanceConsumed": "InlineError.variant",
    "InlineError::Unresolved": "InlineError.variant",
    "InlineError::EpsilonSeam": "InlineError.variant",
    "InlineError::PartCarriesMetadata": "InlineError.variant",
    "InlineError::ParamConflict": "InlineError.variant",
    "InlineError::UnplaceableFrame": "InlineError.variant",
    "InlineError::InstanceBodyNameReferenced": "InlineError.variant",
    "InlineError::ForeignInstanceName": "InlineError.variant",
    "InlineError::StrandedPartName": "InlineError.variant",
    "InlineError::Edit": "InlineError.variant",
    "MateFault::PosesOfAnotherDocument": "MateFault.variant",
    "MateFault::Frame": "MateFault.variant",
    "MateFault::ClassNotAdmitted": "MateFault.variant",
    "MateFault::TableLacks": "MateFault.variant",
    "MateFault::Indeterminate": "MateFault.variant",
    "MateFault::Band": "MateFault.variant",
    "MateFault::Contradictory": "MateFault.variant",
    "MateFault::Under": "MateFault.variant",
    "MateFault::DanglingHead": "MateFault.variant",
    "MateFault::PlacerRefused": "MateFault.variant",
    "MateFault::PartSelectsAnotherCopy": "MateFault.variant",
    "MateFault::SelfMate": "MateFault.variant",
    "MateFault::Unleverable": "MateFault.variant",
    "MeasureNodeFault::RefIndexOutOfRange": "MeasureNodeFault.variant",
    "MeasureUnavailableAt::NeedsEnclosure": "MeasureUnavailableAt.variant",
    "NodePickError::Standing": "NodePickError.variant",
    "NodePickError::NotABody": "NodePickError.variant",
    "NodePickError::NoSuchBody": "NodePickError.variant",
    "NodePickError::Tessellate": "NodePickError.variant",
    "ParseError::UnexpectedChar": "ParseError.variant",
    "ParseError::UnexpectedEnd": "ParseError.variant",
    "ParseError::UnexpectedToken": "ParseError.variant",
    "ParseError::TrailingInput": "ParseError.variant",
    "ParseError::MalformedNumber": "ParseError.variant",
    "ParseError::IntegerOverflow": "ParseError.variant",
    "ParseError::UnknownUnit": "ParseError.variant",
    "ParseError::UnknownFunction": "ParseError.variant",
    "ParseError::WrongArity": "ParseError.variant",
    "ParseError::UnknownParam": "ParseError.variant",
    "ParseError::Dimension": "ParseError.variant",
    "PathError::JunctionTangent": "PathError.variant",
    "PathError::JunctionCusp": "PathError.variant",
    "PathError::SeamTangent": "PathError.variant",
    "PathError::SeamArrivalOffDirection": "PathError.variant",
    "PathError::SeamArrivalLeverTooShort": "PathError.variant",
    "PathError::ContinuationTargetOffRay": "PathError.variant",
    "PathError::NoCornerForFillet": "PathError.variant",
    "PathError::NoCornerOfPair": "PathError.variant",
    "PathError::FilletOffsetLeverTooShort": "PathError.variant",
    "PathError::ArcLegOnOpenFillet": "PathError.variant",
    "PathError::SeamRetrimsArcFirstSide": "PathError.variant",
    "PathError::NonpositiveLeg": "PathError.variant",
    "PathError::NonpositiveFilletRadius": "PathError.variant",
    "PathError::NonpositiveCircleRadius": "PathError.variant",
    "PathError::DegenerateArcSpec": "PathError.variant",
    "PathError::CircleSplitCount": "PathError.variant",
    "PathError::PolygonTooFewVertices": "PathError.variant",
    "PathError::ArcContinueNeedsArcCarrier": "PathError.variant",
    "PathError::ArcContinueOffCarrier": "PathError.variant",
    "PathError::ZeroDirection": "PathError.variant",
    "PathError::ArcViaCollinear": "PathError.variant",
    "PathError::DegenerateArcChord": "PathError.variant",
    "PathError::ArcCenterNotEquidistant": "PathError.variant",
    "PathError::DegenerateArcCenter": "PathError.variant",
    "PathError::FarEndAnchorWithoutFillet": "PathError.variant",
    "PathError::Escalated": "PathError.variant",
    "PathError::Band": "PathError.variant",
    "PathError::Structure": "PathError.variant",
    "PathError::UnderdeterminedLeg": "PathError.variant",
    "PathError::OverdeterminedJunction": "PathError.variant",
    "PersistError::NonFinite": "PersistError.variant",
    "PersistError::ProfileProgram": "PersistError.variant",
    "PersistError::Distribution": "PersistError.variant",
    "PersistError::DisplayUnit": "PersistError.variant",
    "PersistError::Serialize": "PersistError.variant",
    "PersistError::HeaderId": "PersistError.variant",
    "PersistError::IdMismatch": "PersistError.variant",
    "PersistError::Parse": "PersistError.variant",
    "PersistError::Unreadable": "PersistError.variant",
    "PersistError::EditReplay": "PersistError.variant",
    "PersistError::ToleranceConflict": "PersistError.variant",
    "PersistError::ToleranceInvalid": "PersistError.variant",
    "ProductError::EvaluationOfAnotherDocument": "ProductError.variant",
    "ProductError::UnknownNode": "ProductError.variant",
    "ProductError::Naming": "ProductError.variant",
    "ProductError::RootFailed": "ProductError.variant",
    "ProductError::RootPoisoned": "ProductError.variant",
    "ProductError::NoBodyRoots": "ProductError.variant",
    "ProductError::Graft": "ProductError.variant",
    "ProductError::SolidInvalid": "ProductError.variant",
    "ProductError::ProductInvalid": "ProductError.variant",
    "ProductError::ContactLineage": "ProductError.variant",
    "ReadbackError::Dangling": "ReadbackError.variant",
    "ReadbackError::NoCanonicalFrame": "ReadbackError.variant",
    "ReadbackError::NoCarrier": "ReadbackError.variant",
    # A value the at-rest gate hands back, not a raised refusal.
    "RefusedRef::Vanished": "RefusedRef.variant",
    "RefusedRef::ReadBelowARoot": "RefusedRef.variant",
    "RefusedRef::Ambiguous": "RefusedRef.variant",
    "RefusedRef::NotAFace": "RefusedRef.variant",
    # The VERDICT's three arms are `status`, not `variant`: `variant`
    # beside it is the failure's own arm, which is why the two words
    # are separate here (`ResolveError`/`ResolveIndeterminate` in
    # `BOUND_AS`).
    "Resolution::Resolved": "Resolution.status",
    "Resolution::Failed": "Resolution.status",
    "Resolution::Indeterminate": "Resolution.status",
    # `reason` rather than `variant`, the word this door has always
    # carried.
    "SelectRefusal::InBand": "SelectRefusal.reason",
    "SelectRefusal::TiedDisagrees": "SelectRefusal.reason",
    "SelectRefusal::Unreadable": "SelectRefusal.reason",
    "SelectRefusal::NotADatum": "SelectRefusal.reason",
    "SelectRefusal::NotALength": "SelectRefusal.reason",
    "SelectRefusal::PairInBand": "SelectRefusal.reason",
    "SelectRefusal::BadValue": "SelectRefusal.reason",
    "SelectRefusal::Band": "SelectRefusal.reason",
    "SplitError::EmptyCut": "SplitError.variant",
    "SplitError::UnknownCutNode": "SplitError.variant",
    "SplitError::PartIdCollides": "SplitError.variant",
    "SplitError::SeveredEdge": "SplitError.variant",
    "SplitError::OperandSeveredFromMate": "SplitError.variant",
    "SplitError::TornCluster": "SplitError.variant",
    "SplitError::UncutParamReference": "SplitError.variant",
    "SplitError::PartNameReachesRemainder": "SplitError.variant",
    "SplitError::NameStraddlesCut": "SplitError.variant",
    "SplitError::BodyNameCrossesCut": "SplitError.variant",
    "SplitError::Pin": "SplitError.variant",
    "SplitError::PartEdit": "SplitError.variant",
    "SplitError::RemainderEdit": "SplitError.variant",
    "StepImportError::Syntax": "StepImportError.variant",
    "StepImportError::DanglingReference": "StepImportError.variant",
    "StepImportError::WrongEntityType": "StepImportError.variant",
    "StepImportError::MalformedRecord": "StepImportError.variant",
    "StepImportError::UnsupportedEntity": "StepImportError.variant",
    "StepImportError::UnsupportedUnit": "StepImportError.variant",
    "StepImportError::NothingToImport": "StepImportError.variant",
    "StepImportError::Structure": "StepImportError.variant",
    "StepImportError::MissingUncertainty": "StepImportError.variant",
    "StepImportError::InvalidEpsOverride": "StepImportError.variant",
    "StepImportError::DeclarationUnresolved": "StepImportError.variant",
    "StepImportError::VertexWithoutPoint": "StepImportError.variant",
    "StepImportError::MalformedReal": "StepImportError.variant",
    "StepImportError::Topology": "StepImportError.variant",
    "StepImportError::Assembly": "StepImportError.variant",
    "StepImportError::Adoption": "StepImportError.variant",
    "StepImportError::RimOffWallBoundary": "StepImportError.variant",
    "StepImportError::RecognitionAmbiguous": "StepImportError.variant",
    "StepImportError::Pcurves": "StepImportError.variant",
    "StepImportError::Placement": "StepImportError.variant",
    "StepImportError::Instance": "StepImportError.variant",
    "StepImportError::TierInvalid": "StepImportError.variant",
    "StlError::DegenerateTriangle": "StlError.variant",
    "StlError::IndexOutOfRange": "StlError.variant",
    "StlError::TooManyTriangles": "StlError.variant",
    "StlError::Io": "StlError.variant",
    # The solve's freedom class, a value: `Subgroup.variant` plus the
    # axis attributes each arm carries.
    "Subgroup::Se3": "Subgroup.variant",
    "Subgroup::Planar": "Subgroup.variant",
    "Subgroup::Cylindrical": "Subgroup.variant",
    "Subgroup::Prismatic": "Subgroup.variant",
    "Subgroup::Revolute": "Subgroup.variant",
    "Subgroup::Trivial": "Subgroup.variant",
    "Subgroup::Empty": "Subgroup.variant",
    "TessellateError::InvalidChordalTolerance": "TessellateError.variant",
    "TessellateError::UnsupportedSurface": "TessellateError.variant",
    "TessellateError::UnsupportedNurbsFace": "TessellateError.variant",
    "TessellateError::UnsupportedCurve": "TessellateError.variant",
    "TessellateError::NullScaffoldEdge": "TessellateError.variant",
    "TessellateError::RingOnCurvedFace": "TessellateError.variant",
    "TessellateError::EmptyLoop": "TessellateError.variant",
    "TessellateError::MissingEntity": "TessellateError.variant",
    "TessellateError::ResolutionOverflow": "TessellateError.variant",
    "TessellateError::CertificateExceeded": "TessellateError.variant",
    "TessellateError::Triangulation": "TessellateError.variant",
    "TessellateError::SelfTouchingTrimLoop": "TessellateError.variant",
    "TessellateError::UnsupportedCurvedDomain": "TessellateError.variant",
    "TessellateError::UnsupportedCurvedShape": "TessellateError.variant",
    "TessellateError::Band": "TessellateError.variant",
    "UpdateError::NoSuchReference": "UpdateError.variant",
    "UpdateError::AlreadyPinned": "UpdateError.variant",
    # The one door that reports MANY faults at once, so the word rides
    # the FINDING rather than the exception: `len(findings) ==
    # failure_count`, one `variant` each (`crates/pncad-py/src/
    # validation.rs`).
    "ValidationError::Band": "ValidationFinding.variant",
    "ValidationError::DanglingDescription": "ValidationFinding.variant",
    "ValidationError::UncertifiableSurface": "ValidationFinding.variant",
    "ValidationError::PoisonedSurfaceDescription": "ValidationFinding.variant",
    "ValidationError::ApproxCertification": "ValidationFinding.variant",
    "ValidationError::ApproxLaneUnsupported": "ValidationFinding.variant",
    "ValidationError::DegenerateTorus": "ValidationFinding.variant",
    "ValidationError::DegenerateTorusEscalated": "ValidationFinding.variant",
    "ValidationError::NonpositiveTorusTube": "ValidationFinding.variant",
    "ValidationError::EdgeCertification": "ValidationFinding.variant",
    "ValidationError::DescriptionNotAdjacent": "ValidationFinding.variant",
    "ValidationError::PlanarFaceResidual": "ValidationFinding.variant",
    "ValidationError::PlanarFaceEscalated": "ValidationFinding.variant",
    "ValidationError::PlanarBoundaryResidual": "ValidationFinding.variant",
    "ValidationError::PlanarBoundaryEscalated": "ValidationFinding.variant",
    "ValidationError::SliverDihedral": "ValidationFinding.variant",
    "ValidationError::TransverseNotIntrinsic": "ValidationFinding.variant",
    "ValidationError::ScaffoldAtRest": "ValidationFinding.variant",
    "ValidationError::TangentNotIntrinsic": "ValidationFinding.variant",
    "ValidationError::UndeclaredCusp": "ValidationFinding.variant",
    "ValidationError::LaminaWedge": "ValidationFinding.variant",
    "ValidationError::LoopRoleInverted": "ValidationFinding.variant",
    "ValidationError::CurvedSenseInverted": "ValidationFinding.variant",
    "ValidationError::NegativeVolume": "ValidationFinding.variant",
    "ValidationError::VolumeUncomputable": "ValidationFinding.variant",
    "ValidationError::Pcurve": "ValidationFinding.variant",
    "ValidationError::RingMeetsOuter": "ValidationFinding.variant",
    "ValidationError::RingContactEscalated": "ValidationFinding.variant",
    "ValidationError::UndeclaredContact": "ValidationFinding.variant",
    "ValidationError::StaleContactDeclaration": "ValidationFinding.variant",
    "ValidationError::ContactContradicted": "ValidationFinding.variant",
    "ValidationError::CensusEscalated": "ValidationFinding.variant",
    "ValidationError::CensusUnsupported": "ValidationFinding.variant",
    "ValidationError::CensusLaneUnsupported": "ValidationFinding.variant",
    "ValidationError::CensusUndecidable": "ValidationFinding.variant",
    "ValidationError::DanglingTopology": "ValidationFinding.variant",
    "ValidationError::DanglingGeometry": "ValidationFinding.variant",
    "ValidationError::NextPrevMismatch": "ValidationFinding.variant",
    "ValidationError::LoopCycleOverrun": "ValidationFinding.variant",
    "ValidationError::ParentLoopMismatch": "ValidationFinding.variant",
    "ValidationError::UnreachableHalfEdge": "ValidationFinding.variant",
    "ValidationError::EdgeHalvesIdentical": "ValidationFinding.variant",
    "ValidationError::EdgeSlotBackpointerMismatch": "ValidationFinding.variant",
    "ValidationError::HalfEdgeUnclaimed": "ValidationFinding.variant",
    "ValidationError::HalfEdgeMultiplyClaimed": "ValidationFinding.variant",
    "ValidationError::EdgeNotAntiparallel": "ValidationFinding.variant",
    "ValidationError::EmanatingStartMismatch": "ValidationFinding.variant",
    "ValidationError::EmptyLoopVertexWithEmanating": "ValidationFinding.variant",
    "ValidationError::LoneVertexWithIncidence": "ValidationFinding.variant",
    "ValidationError::VertexOrbitOverrun": "ValidationFinding.variant",
    "ValidationError::OrbitForeignMember": "ValidationFinding.variant",
    "ValidationError::SplitVertexOrbit": "ValidationFinding.variant",
    "ValidationError::OuterListedAsRing": "ValidationFinding.variant",
    "ValidationError::BackPointerMismatch": "ValidationFinding.variant",
    "ValidationError::OrphanEntity": "ValidationFinding.variant",
    "ValidationError::MultiplyOwned": "ValidationFinding.variant",
    "ValidationError::OrphanGeometry": "ValidationFinding.variant",
    "ValidationError::SolidWithoutShells": "ValidationFinding.variant",
    "ValidationError::ShellWithoutFaces": "ValidationFinding.variant",
    "ValidationError::EdgeAcrossShells": "ValidationFinding.variant",
    "ValidationError::ComponentEulerViolation": "ValidationFinding.variant",
    "ValidationError::MissingProvenance": "ValidationFinding.variant",
    "ValidationError::LeakedProvenance": "ValidationFinding.variant",
    "ValidationError::ScaffoldingEmptyLoop": "ValidationFinding.variant",
    "ValidationError::ScaffoldingStrutVertex": "ValidationFinding.variant",
    "ValidationError::ShellDisconnected": "ValidationFinding.variant",
    "ValidationError::NullScaffoldShared": "ValidationFinding.variant",
    "ValidationError::LeakedNullFaceRecord": "ValidationFinding.variant",
    "ValidationError::StaleNullFaceLoop": "ValidationFinding.variant",
    "ValidationError::NullEdgeAtRest": "ValidationFinding.variant",
    "ValidationError::NullFaceAtRest": "ValidationFinding.variant",
    # --- an arm or a field that crosses as a DOOR ------------------
    # `Route` is `INTERIOR`; what it SAYS crosses as the two attributes
    # that name it, `of` and `via`.
    "CarriedDeclaration::route": "CarriedDeclaration.of",
    # The lattice's closed loop is handed to `Node.profile` opaquely and
    # read back only as two counts.
    "ClosedLoop::loop_": "ClosedLoop.vertex_count",
    "ClosedLoop::program": "ClosedLoop.step_count",
    # THE ARM THIS RULE WAS RULED FOR. `Datum` is the `editor_core`
    # AUTHORING enum; `pncad.pyi`'s `Datum` is the READ-side value
    # `Value.datum()` answers with, and it spells none of these six. So
    # rule 1 accounted the whole enum on a spelling coincidence and
    # `FaceFrame` was invisible for the life of its family
    # (`work/lib/datum-crosses-name-for-name-as-two-types.md`). All six
    # arms cross as `Node.datum_*` constructors, one per arm. `Point`
    # and `Frame` were the family B-DATUM-DOORS chartered, closed at
    # LIB-GAPS-1: a Python author now builds six of six datum kinds,
    # and each reads back through `Value.datum()`.
    "Datum::Plane": "Node.datum_plane",
    "Datum::Axis": "Node.datum_axis",
    "Datum::AxisInPlane": "Node.datum_axis_in_plane",
    "Datum::FaceFrame": "Node.datum_face_frame",
    "Datum::Point": "Node.datum_point",
    "Datum::Frame": "Node.datum_frame",
    # `Tied` is the arm the namesake spells; `Unique` is that attribute
    # being false.
    "Denotation::Unique": "Denotation.tied",
    # The structural-slot edit, reached through one door per slot rather
    # than by naming the arm — `bind_count_param`, `bind_instance_param`
    # and `bind_v_degree_param` all build this arm.
    "DocEdit::SetStructuralParam": "DocEdit.bind_count_param",
    # The continuous arm is what the three dimensioned constructors
    # mint; `Count` is the arm the namesake spells.
    "DocParam::Continuous": "DocParam.length",
    # As `DocParam` above, one rung down at the value.
    "DocParamValue::Continuous": "DocParamValue.length",
    # THE RUST RUN'S OWN FIELDS, under a Python class that is a different
    # type: `pncad.pyi`'s `Evaluation` is the binding's captured
    # (document, evaluation) pair. Four of its ten fields carry names
    # the pair spells anyway (`order`, `recomputed`, `reused`,
    # `part_evaluations`); these are the rest. `prior_refused` is the
    # `Mispaired` entry's sentence in `NOT_BOUND` — the fact reaches
    # Python as `reused` being 0 with every node recomputed.
    "Evaluation::document": "Doc.id",
    "Evaluation::prior_refused": "Evaluation.reused",
    "Evaluation::nodes": "Evaluation.value",
    "Evaluation::outcome": "Evaluation.canceled",
    # The detector's finding: its pair crosses as two NAMES, its class
    # as the word, its evidence as the rung that carried it.
    "FlushFinding::pair": "FlushFinding.a",
    "FlushFinding::class": "FlushFinding.class_",
    "FlushFinding::evidence": "FlushFinding.rung",
    # The maintenance travels with the document handed back, so it is
    # read off that `Doc` rather than off the outcome
    # (`crates/pncad-py/src/py/refactor.rs`).
    "InlineOutcome::maintenance": "Doc.last_maintenance",
    # The replayed edit LIST crosses as its length; `Applied`/
    # `EditRecord` are `different-shape` in `NOT_BOUND` and the records
    # field is below with them.
    "Loaded::edits": "Loaded.edit_count",
    # One patch at a time, by index, beside `patch_count`.
    "Mesh::patches": "Mesh.patch",
    # `class_` because `class` is a Python keyword — the `IN`/`inch`
    # shift, one level in.
    "MintedDeclaration::class": "MintedDeclaration.class_",
    # The entity-kind filter, named for what it does at the door.
    "NamePat::kind": "NamePat.of_kind",
    # The datum arm crosses as one constructor per `Datum` arm (see the
    # `Datum` rows above); this points at the first of the six.
    "Node::Datum": "Node.datum_plane",
    # Spelled out, as the stub spells every direction.
    "Ray::dir": "Ray.direction",
    # The nested patterns a segment pattern matches its arguments with.
    "SegPat::args": "SegPat.of",
    # The alternatives, named for the door that takes them.
    "Selector::alts": "Selector.any_of",
    # The frame crosses as its four readers rather than as an `Affine3`,
    # which is `different-shape` in `NOT_BOUND`; this points at the
    # first of them.
    "SketchPlane::placement": "SketchPlane.origin",
    # As `InlineOutcome` below: the maintenance rides the document each
    # half is handed back on.
    "SplitOutcome::remainder_maintenance": "Doc.last_maintenance",
    "SplitOutcome::part_maintenance": "Doc.last_maintenance",
}

#: Members with no Python spelling at all, by family — `NOT_BOUND`'s three
#: families, for their reasons, one level in.
#:
#: `different-shape` and `behind-a-door` read as they do above. A `gap:`
#: entry is OWED WORK and names the id that owns it. One of the ids here
#: is chartered in `FAMILIES` by this rule's first run — the five
#: `DocEdit` arms no Python constructor builds; the rule's other two
#: findings, the two `Datum` arms and the `Mesh` field, are bound and
#: gone from this table. The remaining entry cites `G2`, the audit's,
#: beside `sweep_body` above.
MEMBERS_NOT_BOUND = {
    # THE PATH VERBS' ARC SPECS, one family. A spec's fields are its
    # CONSTRUCTOR's arguments — `Bulge(p, b)`, `Center(c, winding, p)` —
    # and nothing reads one back: the spec is consumed by the verb it is
    # passed to. `SplitHalf`/`Side` and the radius sign are the same
    # `different-shape` argument the selector plumbing carries.
    "ArcLen::r": SHAPE,
    "ArcLen::side": SHAPE,
    "ArcLen::len": SHAPE,
    "Bulge::p": SHAPE,
    "Bulge::b": SHAPE,
    "Center::c": SHAPE,
    "Center::winding": SHAPE,
    "Center::p": SHAPE,
    "Radius::r": SHAPE,
    "Radius::side": SHAPE,
    "Sweep::r": SHAPE,
    "Sweep::side": SHAPE,
    "Sweep::angle": SHAPE,
    "Via::q": SHAPE,
    "Via::p": SHAPE,
    # `ContactRecords` is `INTERIOR` and this is that entry one level
    # in: nothing hands one out, so the field has nothing to project.
    "Assembly::contacts": INTERIOR,
    # The kernel's own story about the refused declaration, composed
    # into the finding's `str()` — the flattened-payload bullet at the
    # door that renders rather than raises.
    "AtRestFinding::error": SHAPE,
    # The replay structure is the lattice's own bookkeeping; no Python
    # value is ever one.
    "ClosedLoop::structure": INTERIOR,
    # THE THREE EDITS STILL WITH NO PYTHON DOOR, and each is a
    # different sentence now that LIB-EDITS has built the two that
    # were only missing. Those two left this roster ENTIRELY rather
    # than moving into `MEMBERS_BOUND_AS`: `DocEdit.set_param` and
    # `DocEdit.rebind` spell `SetParam` and `Rebind` namesake for
    # namesake, and rule 1 accounts a member the stub spells. Their
    # doors are the CONTINUOUS slot edit — one door for every slot,
    # the opposite decision from the three `bind_*_param` doors,
    # because the continuous slots are the whole named alphabet
    # `EditError.slot` publishes and this door reads it in the other
    # direction — and the one name repair, whose halves take the role
    # suffix `EditError.from_kind` / `to_kind` take, for the reason
    # those two do: `from` is a Python keyword.
    #
    # The WITNESS PAIR is the appearance four's argument at a second
    # pair of types: `crates/pncad/src/document.rs` does not carry
    # `WitnessDatum` or `BranchCertification` at all, so the arms have
    # no payload a consumer of that module can name — in Rust or in
    # Python. Filed as
    # `work/lib/the-witness-edits-need-a-facade-type.md`.
    #
    # The EXPRESSION-PATH edit is neither: its payload is curated
    # (`ExprPath` is `EditError.path`) and the constructor is
    # mechanical. What blocks it is its own refusal — `path_off_tree`
    # renders the address through `Debug`, and the binding's prose
    # gate panics on that, so the door would panic exactly where it is
    # supposed to refuse. Filed as
    # `work/lib/the-expression-path-edit-cannot-refuse-as-prose.md`.
    "DocEdit::SetExpression": f"{GAP}: B-DOC-EDITS no `DocEdit` constructor builds this arm",
    "DocEdit::ReWitness": f"{GAP}: B-DOC-EDITS no `DocEdit` constructor builds this arm",
    "DocEdit::ReWitnessBulk": f"{GAP}: B-DOC-EDITS no `DocEdit` constructor builds this arm",
    # THE APPEARANCE FOUR, and the reason is the FAÇADE's rather than
    # this file's: `crates/pncad/src/document.rs` carries `AttrKind` and
    # leaves `Attr`, `AttrSet` and the record types out, because nothing
    # a consumer of that module holds answers in them. An arm whose
    # payload the façade does not carry has nothing for a Python
    # constructor to take, and the metadata pair is the same sentence at
    # `MetaValue`.
    "DocEdit::SetAppearance": SHAPE,
    "DocEdit::ClearAppearance": SHAPE,
    "DocEdit::SetAppearanceMeta": SHAPE,
    "DocEdit::ClearAppearanceMeta": SHAPE,
    # The epoch is minted per run and is not a caller's choice, which is
    # `EvalOptions`'s own sentence one rung in. The appearance
    # resolution is the appearance family the façade leaves out (see the
    # `DocEdit` rows).
    "Evaluation::epoch": INTERIOR,
    "Evaluation::appearance": SHAPE,
    # `EditRecord` is `different-shape`: `Doc.apply` mutates in place and
    # answers `Optional[NodeId]`, so there is no record list to hand
    # back.
    "Loaded::records": SHAPE,
    # The declared FACES are arena keys, which the curation exists to
    # keep unnameable in Python; the pair reaches a caller as the two
    # names `a` and `b`.
    "MintedDeclaration::faces": SHAPE,
    # The one recipe node kind with no Python constructor, and it is not
    # an omission: `wire_sweep` refuses unconditionally (U4/LQ3,
    # kernel-owned), so the door could not succeed. `sweep_body` carries
    # the same citation in `NOT_BOUND`.
    "Node::Sweep": f"{GAP}: G2 sweep — no `Node.sweep`, `wire_sweep` refusing",
}


class TestBindingCensus(unittest.TestCase):
    def setUp(self):
        self.curated = curated_names()
        self.top, self.members = stub_surface()
        self.declarations = curated_declarations()

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
        # And the same floor on the RUST side of the member rule, whose
        # scanner is a whole second reader: 104 matched declarations
        # carrying 616 members at this unit's merge base. A resolver that
        # answered nothing would satisfy the member obligation vacuously,
        # exactly as an empty curated set would satisfy the name one.
        self.assertGreater(
            len(self.declarations),
            60,
            "no curated name resolved to a declaration with members",
        )
        self.assertGreater(
            sum(len(members) for _decl, members in self.declarations.values()),
            400,
            "the declaration scanner found almost no members",
        )

    def test_every_bound_as_spelling_exists_in_the_stub(self):
        """A mapping to a spelling the stub does not declare is a
        claim nobody is checking — the failure mode that would make
        this whole roster decorative."""
        absent = sorted(
            f"{name} -> {spelling}"
            for table in (BOUND_AS, MEMBERS_BOUND_AS)
            for name, spelling in table.items()
            if spelling not in self.top and spelling not in self.members
        )
        self.assertEqual(
            absent,
            [],
            "a bound-as roster names Python spellings pncad.pyi does not "
            "declare",
        )

    def test_the_two_rosters_are_disjoint(self):
        overlap = sorted(set(BOUND_AS) & set(NOT_BOUND))
        overlap += sorted(set(MEMBERS_BOUND_AS) & set(MEMBERS_NOT_BOUND))
        self.assertEqual(overlap, [], "a name cannot be both bound and unbound")

    def test_every_not_bound_family_is_one_of_the_three(self):
        bad = sorted(
            f"{name}: {family}"
            for table in (NOT_BOUND, MEMBERS_NOT_BOUND)
            for name, family in table.items()
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
        a pointer at a PARAGRAPH of `work/lib/log.md`'s "LIB residual
        register", which is not an enumeration and cannot be
        dispatched against. So each entry now names exactly one id
        and each id must be defined:

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
        entries = sorted(NOT_BOUND.items()) + sorted(MEMBERS_NOT_BOUND.items())
        for name, family in entries:
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

    def test_every_member_of_a_matched_type_is_spelled_or_listed(self):
        """**The member obligation, mechanical.**

        Rule 1 accounts a curated name by SPELLING, which is exactly as
        strong as the coincidence that the two sides picked the same
        word. Where that name resolves to an enum or a struct, this is
        what the match actually buys: the members the Python namesake
        spells, and nothing else. A member it does not spell is a door
        that may or may not exist, and it fails here until someone says
        which.

        `Datum::FaceFrame` is why. The curated `Datum` is `editor_core`'s
        AUTHORING enum and `pncad.pyi`'s is the read-side value — two
        types, one word — so a whole authoring arm was unbound behind a
        name-for-name match for the life of the family chartered to bind
        it. `Node::Union` and `DocEdit::SetMembers` were the same shape
        behind two more names this file accounts whole.
        """
        unaccounted = sorted(
            f"{name}::{member}"
            for name, member in unspelled_members(self.members)
            if f"{name}::{member}" not in MEMBERS_BOUND_AS
            and f"{name}::{member}" not in MEMBERS_NOT_BOUND
        )
        self.assertEqual(
            unaccounted,
            [],
            f"{len(unaccounted)} member(s) of a curated type Python spells "
            "identically are neither spelled by the namesake nor listed: "
            "spell each on the namesake (an arm as a same-named class "
            "attribute or a snake-cased constructor, a field as a same-named "
            "attribute), or add it to MEMBERS_BOUND_AS with the Python "
            "spelling that answers the same question, or to MEMBERS_NOT_BOUND "
            "with the family it belongs to.",
        )

    def test_the_member_rule_catches_a_spelling_that_moves(self):
        """**The falsifier, run rather than argued.**

        A guard that has never been seen to fail is a guard nobody has
        checked, and the member rule's whole claim is that a door
        disappearing from the Python side stops being invisible. So the
        rule is run once against a stub surface with `Node.extrude`
        taken out of it: the arm must come back unaccounted, naming
        itself, and must NOT be unaccounted against the real surface.

        `Node::Extrude` is the subject because it is the shift the rule
        is named for — an arm crossing as a snake-cased constructor —
        and because `Node` is exactly the type whose arms rule 1
        accounted whole while `Union` sat unbound behind it.
        """
        moved = {member for member in self.members if member != "Node.extrude"}
        self.assertIn(
            ("Node", "Extrude"),
            unspelled_members(moved),
            "the member rule did not notice a constructor leaving the stub",
        )
        self.assertNotIn(
            ("Node", "Extrude"),
            unspelled_members(self.members),
            "`Node.extrude` is in the stub and the rule should account it",
        )

    def test_the_member_rosters_decay(self):
        """Both directions again, over the set the member rule is about.

        A row for a member the declaration no longer has — renamed,
        deleted, or its type no longer curated or no longer matched by
        rule 1 — is a decision about nothing. So is a row for a member
        the Python namesake has SINCE started spelling, which is how a
        `gap:` row is meant to leave when the door is built.
        """
        declared = {
            f"{name}::{member}"
            for name, (_decl, members) in self.declarations.items()
            for member in members
        }
        unspelled = {
            f"{name}::{member}"
            for name, member in unspelled_members(self.members)
        }
        rows = set(MEMBERS_BOUND_AS) | set(MEMBERS_NOT_BOUND)
        stale = sorted(
            f"{row} (not a member of a curated type rule 1 matches)"
            for row in rows
            if row not in declared
        )
        stale += sorted(
            f"{row} (the Python namesake spells it now; drop the row)"
            for row in rows
            if row in declared and row not in unspelled
        )
        self.assertEqual(stale, [], "stale member rows — remove them")

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
