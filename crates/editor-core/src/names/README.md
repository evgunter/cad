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
| N4 emission, `NamingError` | `emit.rs` (helpers, totality check), `emit_sweep.rs` (extrude/revolve/loft), `emit_topo.rs` (boolean, split, N3 merge), `emit_union.rs` (the n-ary union: member-keying in, collapse out), `emit_blend.rs` behind `emit_fillet.rs`/`emit_chamfer.rs` |
| N2 discriminators; tie propagation | `discriminate.rs`; `defer.rs` |
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
(`ProfileEdgeRef`, `ProfileVertexRef`) are the profile crate's canonical
combinatorial identities, never enumeration indices.
Names contain no floats and no arena keys; a pass-through op (Transform,
split-intact entity) adds no segment, so `node` stays the original minter. Names
are document-local; assembly wrapping is `ASSEMBLY.md`'s.

**N2 — Split discriminators are covariant margined predicates.** When one source
yields n fragments, `Fragment(Qualifier)` follows the parent-bearing segment:
`Qualifier::SideOf`, a sign vector of `name_frag_side_of` verdicts against the
cutting partners' outward-oriented carrier planes, or `Qualifier::OrderAlong {
rank, of }`, the `name_frag_order_along` rank along the parent's oriented
carrier. Both run through `k_stats`, so fragment identity changes only at a
recorded flip; an in-band margin refuses (`NamingError::Escalated`), never a
silent pick. Where nothing covariant discriminates (congruent candidates,
overlapping extents, a section line crossing one operand face twice) the table
records one `Entry::Tied` row: naming a tie succeeds, referencing it is
`ResolveError::Ambiguous`, and the only repair is a recorded user choice. Ties
propagate downstream as tied (`defer.rs`); `select_where` filters a tied name
all-or-nothing (`SelectRefusal::TiedDisagrees`), no per-candidate narrowing.

**N3 — Merge policy: names retire into the merge, loudly.** Coplanar-face
merging (F7) merges only structural or declared-coincident faces, which share a
recipe source; the merged face is `Merged(sorted, deduped constituents)`. The
constituents retire: referencing one fails with the merged name offered, and
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

## Resolution

**N5 — Typed resolution failure.** `ResolveError` is `Vanished { name, diagnosis,
last_good: Option<Tombstone> }`, `Ambiguous { name, candidates, tie: TieWitness }`
or `NodeGone { name, edit }`. `Diagnosis` is `PredicateFlip { predicate, from,
to }`, `StructuralParam { node, param }`, `RecipeEdit { edit }`, `Cascade
{ through }` (an embedded operand name vanished first) or `WitnessBifurcation`
(SOLVER-DESIGN W3). Diagnosis is computable because every node evaluation
records its verdict log (`k_stats`); `resolve/vdiff.rs` diffs two runs per
predicate by sign population (permutation-invariant) and is shared with
`SetTolerance`'s ε-audit. When the diff is silent the ladder is `Cascade`, then
the qualifier-delta rung (a `PredicateFlip` recovered from `SideOf` verdicts
stored in the names), then `Diagnosis::cause_not_in_evidence` = `RecipeEdit {
NodeChanged(minting node) }`, a site rather than a claim that an edit happened —
reached in particular when the evidence lived on a pair the boolean's BVH sweep
pruned; results are unaffected, only diagnosis richness degrades. `Tombstone`
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
comparison survives only as the debug assertion `plane_bits_agree`, and the gate
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
- Shadow re-execution of a pruned pair to mint missing verdicts at diagnosis
  time (`work/` item `vdiff-pruned-pair-shadow-exec-rung`).
