# LIB-G17 — `Node::Shell`, the shell recipe door

**Status: binding at dispatch (LIB orchestrator, 2026-09-06), as the
faithful elaboration of `docs/RECIPE-DOORS-DESIGN.md` D5 (ratified
2026-08-29). Deleted at merge per `docs/DOC-LEDGER.md`; the item
`work/lib/LIB-G17.md` is the record that survives.** Every mechanism
below was measured in-tree at main `76cf4ec8` (file:line); the
implementer re-measures at its merge base and says what moved.

## 1. The unit in one paragraph

RECIPE-DOORS unit 3 of 3. `topo::shell` / `shell_open` ship, return
`Shelled<T> { body, naming: ShellNaming }` (`crates/topo/src/shell.rs:514`,
`:539`), and are on the verb seat as `Verb::Shell { thickness, open }`
with `Verb::run_shell` and `VerbRecord::Shell(ShellNaming)`
(`crates/verbs/src/verb.rs:137`, `crates/verbs/src/run.rs:469`). There is
no recipe node, so a shelled part cannot rebuild, mints no
`StableName`s, and is invisible to Python; the teapot's mouth is
designated by a numeric plane scan (`demos/tour/src/teapot.rs`,
`plane_chart_at`) because nothing else can name it. This unit adds
`Node::Shell`, its lowering through the seat, its naming emitter, its
Python spelling, and the corpus rows that prove each.

## 2. The node (D5, payload decided at ratification)

```rust
Node::Shell {
    /// The body hollowed.
    target: RecipeNodeId,
    /// The wall thickness — a magnitude ([`SlotId::ShellThickness`], Length).
    thickness: Expr,
    /// The faces opened into rims, by stable name, IN DESIGNATION ORDER.
    open: Vec<StableName>,
}
```

- **`open` is ORDERED, not canonical.** This is the one place the
  blend precedent does not transfer, and the reason is in the kernel's
  record: a designated chart's rim IS its first designated face
  (`RimNaming::rim` is `sources[0]`, `shell.rs`), "so a caller that
  wants a particular face to carry the rim's identity names it
  first". Sorting would silently change which face the rim inherits.
  So `Node::shell(target, thickness, open)` (the one construction
  door) keeps order and DEDUPLICATES keeping the first occurrence; a
  wire list with a repeated name is a corrupt file, refused at load
  the way a non-canonical blend selection is
  (`crates/editor-core/src/persist/check.rs:692`) — a new
  `SnapshotError` arm, never a quiet repair. State this at the field.
- **Empty `open` is the sealed hollow**, legal, no refusal: D5 gives
  the sealed form no node of its own and the seat's own doc says
  "Empty is the sealed hollow". Unlike `BlendSelectionEmpty`. Say why
  at the site.
- `SlotId::ShellThickness` is its own slot (Length): a wall thickness
  is neither a radius nor a setback, the same argument
  `ChamferDistance` makes at `node.rs:276`. Every exhaustive `SlotId`
  match grows an arm (`dimension`, `label`, the dimension census).
- Every exhaustive `Node` match grows an arm, with no wildcard
  anywhere: `node.rs` (inputs, slots, `expr`/`expr_mut`, selections
  at `:2430`/`:2474`, `input_fault`, the kind census), `eval/mod.rs`
  (content-key tag, verb tag, the selection walk at `:3099`),
  `eval/wire.rs` (the dispatch), `refactor.rs`, `persist/check.rs`,
  `crates/viewer/src/tree.rs:182` (kind label) and
  `crates/viewer/src/combine.rs:515`, `crates/pncad-py` (below). The
  tube pair's file list is the sweep: `grep -rln 'Node::Tube'` at your
  merge base is the roster of sites, and the PR lists each with its
  disposition.
- **Content key**: `Node::Shell { .. } => document_verb_tag(verbs::VerbKind::Shell)`,
  the chamfer's shape (`eval/mod.rs:2785`); the verb's content tag is
  the seat's, read not invented. Tags append, never reuse — re-read
  main's roster at every re-merge (the LIB-TUBE lesson at `:2797`).

## 3. The lowering (`eval/wire.rs`)

`wire_shell(id, target, open, doc, results, vals, env, tol)`, the
`wire_blend` shape (`wire.rs:1717`) with a face selection:

1. `body_operand(results, target)`; `thickness = need_scalar(vals, SlotId::ShellThickness)`.
2. **Resolve `open` through the N5 ladder** — `ladder::resolve_in`
   against the target's table, exactly as `resolve_selection`
   (`wire.rs:1939`) does for edges, with the kind check demanding
   `EntityKey::Face`. Keep designation order (D9's arena-order
   rule is for DERIVED lists; here the order is authored data the
   kernel reads). Refusals are new `NodeErrorKind` arms,
   `ShellOpenResolve { error: Box<ResolveError> }` and
   `ShellOpenKind { name, found }`; do NOT widen the `BlendSelection*`
   trio's `verb: BlendKind` label to cover a face verb — their tag
   values are a pinned Python contract (`TAG_INVENTORY`).
3. `Verb::Shell { thickness, open }.run_shell(&body, tol)` — the seat
   is the door; nothing calls `topo::shell_open` from the document
   layer. The refusal routes through `verb_refused`'s existing
   `Shell` arm (`wire.rs:1612`), which today drops the payload and
   says so at length: **replace that arm** with a carriage (§4).
4. `verbs::read_record(out.record, shell_record, FOREIGN)` with an
   exhaustive projection (`blend_record`'s shape, `verbs/blend.rs`);
   a `None`-shaped record is impossible here (`ShellNaming` is not an
   `Option`) so there is no `no_records` sentence — do not invent one.
5. `names::name_shell(id, target, &target_table, &out.body, &naming)` (§5).
6. `stamp_minted(&mut body, id)`; **attach the parameter-identity
   channel** (VERB-SEAT-DESIGN P2) for `ScalarParam::ShellThickness`,
   which the seat already declares with its flow
   (`crates/verbs/src/flow.rs:73`, `:361`) — the blends' `attach_blend`
   shape; if `param_source` has no attach door for a shell's carriers,
   that is a STOP (§8), not a silent omission.

A `crate::verbs::shell` correspondence module beside `blend.rs` is
the natural home for the build/record/emitter/slot data; with one
instance a struct may be more than the seat needs, and that is the
implementer's call — what is not optional is that no match in it has a
wildcard and that `blend.rs` is not opened to add a shell (its module
docs say why a shell's author never has to).

## 4. Refusals

`NodeErrorKind` is scalar-free by construction (`eval/mod.rs:487`,
and `verb_refused`'s comment at `wire.rs:1622`), and `ShellError<T>`
is generic — the first kernel refusal reaching the document layer that
carries lane scalars (`Thickness { thickness: T }`,
`WallClearance { gap, needed }`, `Face(Box<ReplaceFaceError<T>>)`,
`shell.rs:210`). The carriage: **`NodeErrorKind::Shell(Box<ShellError<f64>>)`**,
reached by a TOTAL fold of the lane error to its f64 witness, arm by
arm, no wildcard — a new arm in `ShellError` is a compile error here.
The kernel's `Display` text is preserved by construction (it is the
same type at f64). If any nested payload cannot be folded without
losing an arm or a number, STOP per §8: making the kernel type
scalar-free is SHELL's decision, not this unit's, and a truncating
fold would be the silent substitution D5 forbids. Both `Display` and
the Python tag read the family: `node_error_tag` gains `"shell"`, and
the two open-resolution arms gain `"shell_open_resolve"` /
`"shell_open_kind"` (the `chamfer_selection_*` spelling).

## 5. Naming: `names::emit_shell` — a translation of `ShellNaming`, no geometry read

The record is written by the doors themselves as they act (§1), so
this emitter is the mechanical row-to-role translation the blend
emitter is (`names/emit_blend.rs` module docs: covariance, the
survivor/mint totality, tie deferral through `TieRows`). Roles, all
additive vocabulary (pre-release: no version, per the D1 factual
update), each carrying the SOURCE entity's own name from the target's
table so a rebuild moves them with it:

| record row | result entity | role |
|---|---|---|
| `outer` (survivors keep operand keys; every non-retired source face/edge/vertex present in the result) | outer wall | `RoleSeg::FromTarget(name)` — the blend's pass-through, reused |
| `inner`, `inner_edges`, `inner_vertices` | cavity twin | **`RoleSeg::Inner(Box<StableName>)`** — "the cavity twin of ⟨source⟩" |
| `rims[i].rim` | the chart's annular rim face | **`RoleSeg::Rim(Box<StableName>)`** of `sources[0]`'s name — "the rim of ⟨the first designated face⟩" |
| `rims[i].ring_edges` / `ring_vertices` | ring | already rows of `inner_edges`/`inner_vertices` verbatim (the record says so) — `Inner(⟨boundary edge⟩)`, no second role |
| `rims[i].holes[j].face` | promoted hole annulus | **`RoleSeg::HoleRim { of: Box<StableName>, hole: u32 }`**, `j` in pairing order |
| `dead` | nothing | nothing — a designated face's own name VANISHES; `Rim(that name)` is what a selector for "the mouth's rim" says |

Anything that is neither a survivor nor a recorded mint is
`NamingError::MissingUpstream`, loudly; `check_total` closes the other
direction. A tie in any upstream name defers the row (B1). The role
docs state each invariant, not this unit's history (implementer
discipline §4). The `RoleSeg` exhaustive matches (`role.rs:687`'s
census and its siblings) grow arms.

**What a selector can now say, and the row that proves it (§7):**
after a rebuild that changes `thickness` and the target's size, the
name `Rim(mouth)` still resolves to the rim, `Inner(bottom)` to the
cavity floor, and `FromTarget(side)` to the outer wall.

## 6. Python (`crates/pncad-py`)

`Node.shell(target: NodeId, thickness: Length, open: list[str]) -> Node`
beside `Node.chamfer` (`py/doc.rs:1373`, `pncad.pyi:1242`): the
selection is names as TEXT, order kept, docstring stating the
first-named-carries-the-rim rule and that an empty list seals. Tags
(§4) with their `TAG_INVENTORY` rows and a construction pin in
`src/tests.rs`'s `*_tags_are_stable` family for every arm that is
constructible without geometry. The census (`test_binding_census.py`):
if `shell`/`shell_open`/`Shelled`/`ShellNaming`/`ShellError` are
curated façade names, their rows move honestly (`Node.shell` is the
`BOUND_AS` spelling of the two doors; the record and refusal types
follow the carrier rule the file states at its `BlendError` entry);
if they are not curated (measure: `crates/pncad/src/prelude.rs:40`
mentions `ShellError` as reachable through `pncad::topo`), say so in
the PR and touch no roster. Stub test, ty legal fixture, and a
Python test module that mirrors §7's rows through public doors only.

## 7. Acceptance — the corpus documents and their rows

Two corpus documents in `crates/editor-core/tests/corpus/`, the
`die_chamfer.rs` / `tube_ring.rs` shape (a document builder, the
saved-text pin so the wire form of `open` is pinned, registered in
`corpus/mod.rs`):

- **`cup.rs`** — a box of side `L` (dyadic), `Node::Shell` with
  `open = [top]`, thickness `t` (dyadic). Closed forms, DERIVED in the
  test's module docs rather than copied: the cavity is
  `(L−2t) × (L−2t) × (L−t)`, so `V = L³ − (L−2t)²(L−t)`;
  `A = 6L² + 4(L−2t)(L−t)` (five outer faces, the rim annulus
  `L² − (L−2t)²`, the cavity floor `(L−2t)²` and four cavity walls
  `(L−2t)(L−t)`). Both exact in f64 at dyadic inputs, so they are
  `MassPin`s compared with `==`, not metered. Also the sealed form
  (`open = []`): two shells, one solid, genus 0, `V = L³ − (L−2t)³`.
- **`vessel.rs`** — a full revolve of a meridian (a foot, a belly, a
  mouth disc: the teapot's class, chosen because that is where
  `shell_open` was wrong twice, `teapot.rs` note 2), `open` naming the
  mouth. On a full revolve the mouth is a chart of TWO half-disc faces
  and the kernel refuses a partial designation
  (`ShellError::OpenFaceChartPartial`); this unit does NOT auto-complete
  charts (that is a kernel rule and the seat's), so the document names
  both halves. **Record in the PR whether naming both halves is a
  natural spelling** — if it is the friction the teapot note recorded
  in a new coat, file it on LIB's slate as its own issue in this PR
  rather than smoothing it here.

Rows, in `crates/editor-core/tests/lib_g17_shell_node.rs` (and the
Python mirror): evaluates green in both lanes and the eps rows CI
runs; the closed forms; the rim/inner/outer names resolve; **the
rebuild row** — bump `L` and `t` through `DocEdit`, re-evaluate, the
same three names resolve to the same roles and the closed forms move
accordingly; the sealed row; the refusal rows — an unknown name in
`open` (typed `ShellOpenResolve`), an edge name (`ShellOpenKind`), a
non-positive thickness (`ShellError::Thickness` carried with its
number), a half-chart designation on the vessel
(`OpenFaceChartPartial` carried verbatim), and the load-door refusal
of a repeated `open` entry; the content key moves when `open`'s ORDER
changes between two designated faces of one chart (the rim's identity
moved, so the key must); the refusal texts pinned the way
`lib_g16_blend_messages.rs` pins the blends'.

`docs/GUIDE.md`: the recipe-doors ladder gains the shell step beside
chamfer and tube, present tense, spelled through `Node.shell`.

## 8. Stop clauses

Stop and report (the PR stays open, the item says `blocked_on` what)
rather than working around: (a) `ShellError<T>` cannot be folded
totally to f64 (§4); (b) `param_source` has no attach door for the
shell's minted carriers (§3.6); (c) the seat's `run_shell` refuses on
an operand the kernel door accepts, or vice versa (a seat defect,
SEAT's/SHELL's). Each names the file:line and the arm.

## 9. Not this unit

The teapot scene's conversion to the document door (`demos/tour` is a
render-lane change with tess-budget rows; file it as an issue on LIB's
slate with a pointer to this unit's `vessel.rs` as the spelling). A
viewer tool (VIEW's; only the exhaustive arms that must compile).
Chart auto-completion or any change to the kernel's designation rules
(SHELL's). Shelling a hollow operand, cone nappes, `Approx` walls
(SHELL's open rows). Curating `ShellNaming` into the façade (the
record's arena keys never cross, per the standing division).

## 10. A/B protocol fields

Full protocol, block **LIB-13 slot 1** (draw record in
`docs/MODEL-AB-LOG.md`). Pre-draw fields: difficulty **M-L**, class
**STRUCTURAL** — a new node kind across every enumerated site, a
seat-driven lowering, an emitter with three new roles, no new numeric
decision (every predicate is the kernel's). The G16 precedent logged
the same. Dual review under v6 at the frozen head; ordinal claimed at
review dispatch.
