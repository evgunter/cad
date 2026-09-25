# Persistent naming (NAMING-DESIGN N1–N7)

Recipe steps and GUI picks refer to boundary entities of intermediate bodies.
Arena keys cannot be those references: they are body-lineage-scoped and diverge
after any edit that changes kill history. A stable name is a *derivation path*,
the minting recipe node plus the combinatorial role the entity plays there; it
denotes a construction role, not a point set. Because replay is a specified
function of the recipe (D9) and every birth is recorded (D5), evaluation emits
the name↔entity table and re-resolution is a lookup, never a match.

## Where in the code

| Decisions | Module |
|---|---|
| N1 `StableName`, `RolePath`, `RoleSeg`, `EntityKind`; N2 `Qualifier` | `role.rs`; `RecipeNodeId` in `crates/editor-core/src/node.rs` |
| N4 `NameTable`, `Entry::{Unique,Tied}`, `EntityRef` | `table.rs` |
| N4 emission, `NamingError` | `emit.rs` (helpers, totality check), `emit_sweep.rs` (extrude/revolve/loft), `emit_topo.rs` (boolean, split, N3 merge), `emit_union.rs` (the n-ary union: member-keying in, collapse out), `emit_blend.rs` behind `emit_fillet.rs`/`emit_chamfer.rs`, `emit_shell.rs` (the shell: survivors `FromTarget`, cavity twins `Inner`, a chart's rim `Rim` of its first designated face, a hole's promoted annulus `HoleRim`) |
| N2 discriminators; tie propagation | `discriminate.rs`; `defer.rs` |
| A path's canonical form: its name-ordered positions (N3 sets, `SideOf` partners, a junction's lines, a union seam's sides), and what ordering a union seam does to its ranks | `canonical.rs`, which the mint, the union's collapse and every rewrite of a published name end in; `seam_pair.rs` (which seam line a rank lies on) |
| N5 `ResolveError`, `Diagnosis`, tombstones, offers; diff engine; hit-testing; `Rebind` | `crates/editor-core/src/resolve/mod.rs`; `resolve/vdiff.rs`; `resolve/hit.rs`, `resolve/pick.rs`; `edit.rs` |
| N6 `GeomSource` | `crates/topo/src/source.rs`; consumers `crates/topo/src/merge_faces.rs`, `crates/topo/src/boolean/plane_eq.rs` |
| Which node minted a named entity (`NameOrigin`); name → geometry (`denotation`, `face_frame`, ...) | `attribute.rs`; `interrogate.rs` |
| Selectors, geometric filters, detect/declare | `select.rs`, `geompred.rs`, `flush.rs`; design in `docs/SELECT-DESIGN.md`, usage in `docs/guide/selecting.md` |

## Names

**N1 — A stable name is a derivation path.** `StableName { kind, node, path }`:
a runtime `EntityKind` (Body, Face, Edge, Vertex — bodies are first-class), the
minting `RecipeNodeId` (from the document's monotone counter at insertion; never
positional, never reused), and `RolePath = Vec<RoleSeg>`. `RoleSeg` is one closed
enum grouped by op: extrude (`Cap`, `Lateral`, ...), revolve (`Band`, `Pole`,
...), boolean (`FromA`, `FromB`, `FromMember { member, of }`, `Seam`, `Merged`,
`Fragment`), split
(`SectionFace`, `SectionEdge`, `SplitFragment`, ...), blend (shared by fillet and
chamfer, told apart by the minting node), `InPart`, pattern `Instance { i, of }`
with `i` recipe-structural. Role arguments are themselves names; profile locators
(`ProfileEdgeRef`, `ProfileVertexRef`) name a profile piece by the id its step
was minted with, never by its position (below).
Names contain no floats and no arena keys; a pass-through op (Transform,
split-intact entity, a `Part`'s projection of one half or one instance) adds no
segment, so `node` stays the original minter. Names
are document-local; assembly wrapping is `ASSEMBLY.md`'s.

**N1, the revolve poles.** `Pole(v)` names the ONE body vertex an on-axis
profile vertex revolves to, looked up in the sweep's `poles` export. A PARTIAL
revolve keeps the axis run: the rotation fixes every point of it and both
meridian chains meet at each of its vertices, so an INTERIOR vertex of a
subdivided axis run — an on-axis side carried by several collinear legs, which
the continuation verbs author — is structurally a pole and takes `Pole(v)` like
the run's tips. A FULL revolve deletes the axis run outright, so an interior
vertex of it has no body entity and nothing to name: the export's `None` is the
answer there, and the run's tips are the only named on-axis vertices. Totality
is the check on that silence — `check_total` refuses a table leaving a LIVE body
vertex unnamed, so a `None` standing over surviving geometry cannot pass.

**N1, the profile pieces: authored things are named by minted ids.** A name
spells what the author made by the id it was minted with, and what the kernel
made by the verdicts that decided it. Nodes already follow this rule, and so do
union members (`FromMember`, DM4). Profile pieces follow it as well:

- **The id.** Every step of a profile program carries a `StepId` in the recipe
  (`ProfileProgram::ids`). It is minted from the document's monotone step
  counter when the step is authored, by `InsertNode` or `SetProgram`. Like a
  `RecipeNodeId`, it is never positional and never reused, and it is unique
  across the whole document; the load door checks all three. A name may
  spell only a step the document has minted: the doors that write a name
  (`InsertNode`, `Rebind`, `SetAppearance`, `SetAppearanceMeta`) refuse one
  at or beyond the counter, and so does the load door.
- **The role.** A step draws its pieces from a fixed list of roles, one list
  per verb:
  - every verb that draws one segment has one role, `Leg`: `line`,
    `line_to`, `continue_to`, `arc_to`, `tangent_arc_to`, `to` (the far
    end), and the close to `Start`, whose leg is the seam;
  - a verb that only binds a point, a direction or a junction (`at`,
    `angle`, `toward`, `tangent`, `cusp`, `turn`) draws nothing and has no
    role;
  - a fillet has three roles, `RunIn`, `Arc` and `RunOut`: the run into it,
    its arc, and the run out of it. The fused verbs have the same three,
    and their authored arc carriers are their runs: `arc_fillet`'s incoming
    arc is its `RunIn`, `fillet_arc`'s arrival arc its `RunOut`, and
    `arc_fillet_arc` has both;
  - a `circle_split` has `Piece(k)` for each `k < n` (the `n` is
    structural);
  - a `circle` has `Piece(0)` and `Piece(1)`. It is drawn as the
    `circle_split` with `n = 2` and phase 0: two semicircles, split at the
    circle's `±x` points.

  A fillet's pieces belong to the step that authors its radius (the replay's
  `RadiusEmission.step`), not to the later step whose arrival resolves the
  corner.
- **The locator.** `ProfileEdgeRef::Piece { step, role }` names the piece that
  role of that step drew. `ProfileVertexRef::Piece { step, role }` names the
  vertex where that piece starts, in authored order: a loop's vertex `k` is
  where its piece `k` starts. A locator holds no loop index and no segment
  index.
- **A kernel-built section.** A section a kernel door builds rather than an
  author — a tube's circle and a hollow tube's bore — has no authored step. It
  is named structurally under the node that builds it:
  `ProfileEdgeRef::Section { circle, role }`, the circle `Outer` or `Bore` and
  the role `Piece(k)`. The node kind fixes the section's shape, as it fixes an
  extrude's two caps, so nothing can renumber it.
- **What cannot move it.** None of these changes what a locator denotes:
  - a value edit;
  - a change in which loop is the outer one, or in which way a loop runs;
  - a document parameter passing through a state that does not replay;
  - a `SetProgram` that keeps the step;
  - a pin update over a child document that kept the step.
- **Undrawn pieces vanish rather than alias.**
  - A role the current values do not draw has no piece. An example is a run
    that a `Zero` fit suppresses. A locator on it resolves `Vanished` until
    the values draw it again.
  - Two pieces on one carrier can be drawn as a single segment. That segment
    answers to the earlier piece in authored order, and the later piece's
    locator resolves `Vanished`. A fillet's run and an authored leg on the
    same carrier are such a pair. A leg into a fillet is authored before
    it, so the leg keeps the segment and the fillet's `RunIn` vanishes. The
    fillet is authored before the leg that follows it, so its `RunOut`
    keeps the segment and that leg's `Leg` vanishes. In the chain `at`,
    `toward(+x)`, `fillet(r)`, `toward(+y)`, `to (2, 2)`, `line_to(0, 2)`,
    `line_to(Start)`, the right wall is `{fillet, RunOut}`, and the far-end
    step's `Leg` denotes nothing. At a radius whose run out fits `Zero`
    the right wall is not drawn, and `{fillet, RunOut}` resolves `Vanished`
    until a radius draws it again.
  - A step that `SetProgram` drops takes its id with it. A name on that step
    keeps its spelling and resolves `Vanished`, and DM7's report names it.
    Because the id is never minted again, no later program can draw it.
- **The canonical numbering is not a name.** It is still the order in which
  the emitters, the loft's correspondence and the viewer's per-segment marks
  iterate (V3, DM8). When a profile's value is built, the naming anchor
  (`eval/anchor.rs`, `ProfilePieces`) pairs each canonical segment and vertex
  with the step and role that drew it, read off the replay record's
  per-segment piece (`profile::ReplayStructure::pieces`), and the sweep
  emitters name what they iterate by it.
- **Loft walls.** A loft pairs canonical segment `k` of every section into one
  wall (DM8). The wall's role-path segment (`LoftWall`) holds one locator per
  section, naming the pieces that pairing joined, and so does a seam edge
  between two walls (`LoftSeam`): one vertex locator per section. A cap and its rims and vertices
  belong to one section, and are named by that section's own locators. If a
  value edit changes which pieces pair, the old wall's name vanishes. It
  does not follow `k` to the new pairing.

**N2 — Split discriminators are covariant margined predicates.** When one source
yields n fragments, `Fragment(Qualifier)` follows the parent-bearing segment:
`Qualifier::SideOf`, a sign vector of `name_frag_side_of` verdicts against the
cutting partners' outward-oriented carrier planes, or `Qualifier::OrderAlong {
rank, of }`, the `name_frag_order_along` rank along the parent's oriented line
— for pieces on a seam line whose pair's two sides carry distinguishable names,
the seam pair's `n_a × n_b` (the pair's `a` face first, one orientation
whichever step cut the line; a union reading the pair in name order reads a
swapped pair's rank from the other end), and otherwise — any other edge, or a
seam between two same-named faces (two placements of one prototype) — the
parent's own oriented carrier. One case counts CELLS rather than fragments:
a union's piece of a member edge, `FromMember(m, e)` + `OrderAlong { rank, of }`.
The finished body's vertices on `e`'s segment (`name_frag_on_member_edge`)
cut it into cells numbered along `e`'s oriented carrier in `m`'s body; `of`
counts cells, not pieces — cells held by another member or by none count too,
so some ranks below `of` index a cell no piece of `m` holds — and `rank` is
the first cell the piece covers. That is an ordinal along the parent's
oriented carrier, and it moves with neither member order nor which member
keeps a flush stretch, as far as the boolean's output is itself order-free
(`emit_union::rank_member_edges`). An edge that lies along several
members' edges, where they run flush, is a piece of the least of them in
name order, and an edge lying along a member edge is a piece of it
whatever the fold named it (`emit_union::Flush`). A seam vertex cites
such an edge whole, `FromMember(m, e)`, never a piece; a vertex at a
member vertex is that vertex, and one where a single face crosses a
member edge is `Seam` of that edge and that face.

A union's face is named for its PARENT, read off the finished body.
Each merged face links the member faces it lists, and linking is
transitive; the member faces so linked are one parent, named
`Merged` of all of them, and a member face no merge links is its own
parent. A parent the finished body holds as one face is that face's
name. A parent it holds as several faces qualifies each with one
`Fragment(SideOf)`: the partners are the parents of the faces across
the group's seam edges, each cited by its parent name. So a face
merged and then cut, and a face cut and then merged, are both
`Merged(set)` + `SideOf`; a face cut by two members is one `SideOf`
over both members' walls, whether one fold step cut it or two. The
fold's spellings are replaced, not refined: which step cut a face,
and whether it met the cut or the merge first, depend on member
order, and the parent and its seam partners depend only on the
finished body.

The verdicts run through `k_stats`,
so fragment identity changes only at a recorded flip; an in-band margin refuses (`NamingError::Escalated`), never a
silent pick, and an ambient tolerance that forms no classification band at all
refuses (`NamingError::Band`) carrying the band constructor's own diagnostic —
the overflow and the collapse want opposite repairs, so the refusal says which
one it caught. Where nothing covariant discriminates (congruent candidates,
overlapping extents, a section line crossing one operand face twice) the table
records one `Entry::Tied` row: naming a tie succeeds, referencing it is
`ResolveError::Ambiguous`, and the only repair is a recorded user choice. Ties
propagate downstream as tied (`defer.rs`); `select_where` filters a tied name
all-or-nothing (`SelectRefusal::TiedDisagrees`), no per-candidate narrowing.

**N3 — Merge policy: names retire into the merge, loudly.** Coplanar-face
merging (F7) merges only structural or declared-coincident faces, which share a
recipe source; the merged face is `Merged(sorted, deduped, flat constituents)`,
and a constituent is never itself a bare merged face: whatever mints a `Merged`
mints it flat — a merge of a merged face lists the faces, never the merge — and
a nested `Merged` is an emission bug, refused at the mint and again at the
union's collapse rather than flattened (the fragment carve-out is stated once,
at `RoleSeg::Merged`). A merged row COVERS a name when the name is a constituent
or is a merged face all of whose faces are (`names/merged.rs`), which is how the
offers and the union's look-through read a flat set. The
constituents retire. In a union no face publishes under a constituent's
name: every piece of a member face a merge links is a piece of its parent
(N2), including a piece that never itself merged. A seam cites, as each
side, the parent of the face beside it, and a `SideOf` partner cites its
parent. Two edges of one seam between the same two faces are two pieces of
that seam, ranked along its line (N2), never told apart by citing a
retired constituent. Referencing a constituent fails with the merged name offered, and
when an edit removes the coincidence the merged name vanishes with its
constituents offered. Numeric coplanarity never merges, so merges change only at
recipe edits, structural parameters, or recorded flips; nothing rebinds itself.

## The name table

**N4 — Eager, per-node, cache-transferable.** Every op names every boundary
entity of every output body from kernel birth data alone (`Extruded`/`Revolved`
maps, `SplitNaming`, `BooleanNaming`, `BlendNaming`, D5 provenance); an unnamed
live entity is `NamingError::Unnamed`, and `NameTable::insert` refuses aliasing
and kind disagreement (`DuplicateName`). The table is bidirectional (`lookup`,
`name_of`), lives in `NodeValue::name_table`, and rides memo hits with the
geometry; consumed entities have no row. The invariant CI pins: the table is a
function of (recipe structure, structural parameters, predicate verdict vector)
only — same recipe and same verdicts give an identical table at f64 and at
Interval (`tests/m4_pr3_names_ci.rs`, `tests/m4_pr3_names_interval.rs`,
`tests/lib_g16_corpus_name_digests.rs`). The kernel never sees a `StableName`;
hit-testing (`resolve/hit.rs`) reads the table backwards, so the GUI never sees
an arena key.

**The row is a shared handle.** A table keys on `NameRef` — one `Arc<StableName>`
per row, held by both directions — and a role segment holds its argument name by
the same type, so a downstream name EMBEDS its operand's row rather than copying
the descent below it. Cloning a name costs its own path and nothing deeper. The
public doors (`insert`, `insert_tied`, `lookup`, `name_of`, `iter`) take and
return bare names and share on the way in; their `_ref` twins, which they
delegate to, take the handle a caller already holds, and an emitter reading an
operand uses those.

**The seal is an order cache.** `NameRef` carries one word beside the name.
`NameTable::seal_order` walks a finished table in its own key order and stamps
each row with its POSITION under one fresh epoch; two names of one epoch compare
by position in O(1), and any other pair — two epochs, or either name unstamped —
compares structurally. The answer is identical either way, because the walk that
minted the positions enumerated a structurally-ordered map: the stamp short-cuts
a comparison whose result it reproduces, so no output depends on whether a name
was stamped (D9). Epochs come from a process-wide counter that SATURATES rather
than wrapping — an epoch is never reused, and a seal past that point simply
leaves the table unstamped. A table is sealed the first time it is read as an
operand, by `defer::upstream_name` for a table read one entity at a time and by
`emit_union::member_view`, `emit::name_pattern`, `emit::name_placed_union` and
`emit::name_in_part` for one read whole; a row inserted after a seal is
unstamped and compares structurally, and a clone starts unsealed so what it
gains is stamped on its next operand use.

**The guard the lint exception rests on.** The stamp is interior mutability
inside a map key, which `clippy::mutable_key_type` flags at every map keyed by a
`StableName`; the root `clippy.toml` excepts `NameRef` and states why. What makes
that a receipt rather than a promise is two-sided: `seal_order` asserts under
`debug_assertions` that its walk really is the structural order, so a lying
position cannot be minted silently, and `table.rs`'s unit tests compare the
handle's order against the name's own on every pair of a sealed table, a
post-seal insert, a bare-name probe through `Borrow`, two epochs, and a
twenty-four-deep chain. The check is at the minting site and not in the compare
because `[profile.release]` keeps debug assertions on, and a check inside the
compare would restore the cost the cache exists to remove.

## Resolution

**N5 — Typed resolution failure.** `ResolveError` is `Vanished { name, diagnosis,
last_good: Option<Tombstone> }`, `Ambiguous { name, candidates, tie: TieWitness }`
or `NodeGone { name, edit }`. `Diagnosis` is `PredicateFlip { predicate, from,
to }`, `StructuralParam { node, param }`, `RecipeEdit { edit }`, `Cascade
{ through }` (an embedded operand name vanished first), `GroupResized { node,
was, now, cutters }` (the fragment's group changed in number, and `cutters`
names the seams on its parent only one run spells), `Upstream
{ node, cause }` (evidence upstream of the minting node, off the derivation
path) or `WitnessBifurcation` (SOLVER-DESIGN W3). Diagnosis is computable
because every node evaluation records its verdict log (`k_stats`);
`resolve/vdiff.rs` diffs two runs per predicate by sign population
(permutation-invariant) and is shared with `SetTolerance`'s ε-audit. The
with-history lanes — recorded flips, structural parameters, recipe edits — read
two scopes (`resolve::upstream_nodes` states the rule): first the name's
derivation path (N1: the nodes the name mentions), answering `PredicateFlip`,
`StructuralParam` or `RecipeEdit`; then, below the qualifier-delta rung, the
nodes that were strict ancestors of the minting node in the last-good document
or in the current one, each walked within its own document, minus the path,
answering `Upstream { node, cause }` — a candidate cause that fed the name
without deciding it. A node in neither set is never read. When the path is
silent the ladder is `Cascade`, then the qualifier-delta rung (a
`PredicateFlip` recovered from `SideOf` verdicts stored in the names), then
the upstream scope, then the GROUP-SIZE rung, then
`Diagnosis::cause_not_in_evidence` = `RecipeEdit {
NodeChanged(minting node) }`, a site rather than a claim that an edit happened —
reached in particular when the evidence lived on a pair the boolean's BVH sweep
pruned; results are unaffected, only diagnosis richness degrades. Between the
flip diff and the qualifier-delta rung sits the SHADOW-EXECUTION rung
(`resolve::shadow_exec_flip`): when a run recorded no `name_frag_side_of`
verdict at the minting node at all, it re-runs the vanished name's own
discriminator pairs against both contexts — the partner read at the boolean's
operand, the per-vertex stream aggregated through this module's own
`aggregate_side`, and the answer calibrated against the verdict the qualifier
records — and reports the first partner whose side changed, marked
`FlipSource::ShadowExec` so no reader mistakes it for a line of a log. The
GROUP-SIZE rung (`resolve::group_resized`, whose docs say why a fragment name
can vanish with no flip) needs a prior run: when the last-good table at the
minting node carried the name, and the group its emitter divided the
fragment's parent into held `was` entities there and holds `now ≠ was` in the
current run, the diagnosis is `GroupResized { node, was, now, cutters }`. The group is
the one the emitter formed, read from the record it keeps beside the table
(`names::FragmentGroups`, not persisted), not re-derived from the names: it
counts the distinct entities of the node's output descended from the parent
within the group by the emitter's own descent, however each is spelled — an
undivided pass-through, an N3 `Merged` survivor, and at a union the published
entities a fold step's group descends to, followed by entity through every
later step. A piece a later step re-mints under a seam name of its own is not
a descendant by that descent and is not counted. Two tied parents that
share a base are two groups, each counted on its own, where the emitter groups
by parent entity; where it groups by parent names (the seam lanes) their
pieces share one group and the rung declines. That is a statement about two
recorded groups, not a claimed flip. `cutters` (`GroupCutters`,
`resolve::group_cutters`) reads the minting node's two tables for the `Seam`
rows on the group's parent — a face group's seam edges, an edge group's seam
vertices, matched on the `Seam { a, b }` pair with any `Fragment` tail and never
on the row — and names every cutter whose seam with the parent only the
last-good table spells (`gone`) or only the current one does (`new`); both empty
says the two tables spell seams on the parent with the same cutters. A seam is a
crossing, not a division: a contact that only trims or notches the parent is
listed alike, so the field states which seams one run spells, not which cutters
stopped or started dividing the group. At a union a cutter is compared without
the fold's `Fragment` tail, which is the fold's and not the member's; a renamed
cutter reads as one gone and one new; a cutter vertex fused onto the parent
edge is spelled as a pass-down vertex, not a seam, so a crossing that became a
touch reads as gone. It names none, and says why, where the tables cannot say:
a group no seam rows on one parent name bound (a split's, a pair boolean's own
seam chain or merged face), tied parents whose shared name the seams are
spelled on, a prior table that spells no seam on the parent, and a seam on the
parent spelled in a shape the reading does not follow. The ladder orders cause before effect: the flips, the qualifier delta,
the doc-diff lanes and `Upstream` name a cause, the group-size change is an
effect whose cause the evidence does not hold, so it runs after every cause-naming rung
and before the fallback. A collapsed fragment's undivided base, when it
resolves, rides in the offers for either qualifier kind. `Tombstone`
carries the last-good entry for ghost rendering; selection tools hold name plus
tombstone, never a key. N3's offers ride beside the verbatim error in
`ResolutionFailure::offers`. The automatic rebinding menu is empty: the only
repair is `DocEdit::Rebind { from, to }`, recorded once, no alias table.

**N6 — Recipe-source identity retires bit identity.** Every surface, curve and
point description carries `GeomSource { node, expr, orient }` beside the arena; a
transform composes into `expr` (`SourceExpr::Placed`), `revert` flips `orient`
(`rev ∘ rev = id`). Same source is syntactic identity of the triple. Theorem:
same `GeomSource` ⇒ bit-identical descriptions (D9); the converse is not
claimed, so equal bits without a shared source stay unglued. The declared
coincidence rung is this lookup (`merge_faces.rs`, `oriented_plane_eq`); the bit
comparison survives only as the debug assertions behind `plane_bits_witness`, and the gate
`scripts/gates/bit-identity-consumer.sh` keeps the production allowlist empty.
Identity holds per evaluation against the current document only.

**N7 — The topology-change sites, exhaustively.** (i) structural parameter
change, (ii) reified predicate flip, N2 discriminators included, (iii) recipe
edit, which stable node ids localize to derivation paths through the edited
node. Within a flip-free, edit-free replay arena-key identity remains the proof
device; everywhere else the name table carries resolution.

## Open

- Out-of-family detection: a failure says the name broke, not that the edit
  left the design family; no membership predicate exists.

