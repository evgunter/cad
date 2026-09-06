# DOCM-6 — The instantiation seam carries mate identity and mint health (spec, DRAFT until §Fork is ruled)

**Program:** DOCM (`work/docm/plan.md`), unit `DOCM-6`
(`work/docm/DOCM-6.md`). **Ruling of record:** the plan's "questions
still open" item 1 — carry `MintedDeclaration` (and `unminted`) across
`PartValue` so a carried refutation names its mate and the outermost
gate sees inner mint health. The finding it answers is
`work/docm/instantiation-seam-drops-mate-identity.md` (from MATE-6's
dual review) — read it in full. **The one open decision is §Fork**,
Ev's; everything else here is settled by the ruling of record and
`crates/editor-core/ASSEMBLY.md`.
**Track:** kernel change — the standard v6 unit.
**Pre-draw fields, logged before the draw:** difficulty **M**, task-class
**STRUCTURAL**.

- **M** — one value type widens at a seam (`eval/parts.rs`'s
  `PartValue`), the gather re-keys two more row kinds across the
  graft's descendant map (the shape `carry_contacts` already has),
  `attribute` gains an arm, and the outermost gate reads what it
  carried; the `pncad-py` mirror of `Attribution`/`AssemblyError` is
  exhaustive and forces rows. No new node, no new naming.
- **STRUCTURAL** — data movement across the seam; every verdict is the
  kernel's as today. No numeric decision.

## What the unit builds

**1. The channel** (`eval/parts.rs`). `PartValue` gains
`minted: Arc<Vec<MintedDeclaration>>` and
`unminted: Arc<Vec<MintRefusal>>`, both read off the inner document's
`Product` at the seam (`parts.rs:~345`, where today they are dropped),
each `MintedDeclaration`'s `faces` still in the INNER product's keys
(the graft re-keys them, item 2). The channel's `Arc`s are the cache's:
every instance of one part shares one row set.

**2. Carried across the graft** (`product.rs`). The gather re-keys a
grafted source's `minted` rows' `faces` through the graft's descendant
map exactly as `carry_contacts` re-keys the records (total over a
grafted source; a missing image is the same defensive
`ContactLineage`-class refusal, never a dropped row), and collects
them into a new `Product::carried: Vec<CarriedDeclaration>` with
`CarriedDeclaration { through: RecipeNodeId /* the instantiating node */,
of: DocumentId /* the inner document */, declaration: MintedDeclaration }`.
A sub-assembly's own `carried` rows carry up again with `through`
naming the OUTER instantiating node and the inner path kept
(`via: Vec<RecipeNodeId>`), so a refutation three documents down names
its whole route. `unminted` rows carry the same way into
`Product::carried_unminted: Vec<CarriedRefusal { through, of, via, refusal }>`.
`Product::minted` and `Product::unminted` (this document's own) are
unchanged.

**3. Attribution names carried mates** (`assembly.rs`). `attribute`'s
`by_pair` lookup also searches `carried` rows; a hit yields a new
`Attribution::Carried { through, of, via, declaration, relation }`
where `relation` is `Refuted` or `Declined` as today's two arms (one
definition of the pair lookup, three callers: own-minted refuted,
own-minted declined, carried). `Unattributed` is then what it says: a
finding no declaration of ANY document in the tree answers for. The
Q1 sentence the finding quotes ("a refuted carried declaration lands
`Refuted` naming its mate") becomes true; state it at the arm and
strike the "cannot name its mate" prose wherever it stands
(`ASSEMBLY.md`'s D-1 paragraph, `assembly.rs:~58`).

**4. Mint health at the outermost gate** (`assembly.rs`,
`assemble_gathered`). Today the gate raises the head of THIS
document's `unminted`. After this unit it also reads
`carried_unminted` — and what it does with a non-empty list is §Fork.

**5. The Python mirror.** `Attribution`'s exhaustive mirror in
`pncad-py` forces the `Carried` arm's tag and projection rows; no new
Python door. Disclose the rows.

## Fork — Ev's ruling, recorded here before dispatch

**Question.** An inner document whose only mate cannot be minted
refuses its own `assemble`. Instantiated into an outer document, is
that refusal the OUTER document's error?

- **(A) Yes — refuse at the outermost gate** (the orchestrator's
  recommendation; the fail-loud reading of Q1 and of A2's "what a
  document means is its product"). `assemble_gathered` raises a new
  `AssemblyError::CarriedMintRefusal { through, of, via, refusal }`
  for the first carried refusal in gather order, BEFORE its own
  `unminted` head and before the at-rest gate, naming the inner
  document and mate so the author knows which file to open. The
  landing's badge renders it. Cost: an outer assembly is unusable while
  an inner part is broken — which is what a broken part means.
- **(B) Advisory** — the gate certifies over the records it has;
  `carried_unminted` rides `Assembly` as data and the registry gets a
  resident (or the badge a secondary line) that reports "N declarations
  of instantiated parts could not be minted", attributed. Cost: an
  assembly reads as at rest while a part inside it has an unverified
  contact, which the Q1 ruling's letter called the bound on "verification
  runs once at the outermost gate".

Whichever is ruled, the rows in A4 assert it and the other option's
sentence is not written anywhere.

## Acceptance

- **A1 — the channel is total.** For every assembly fixture in the mate
  suites and `asm_r2b_assembly.rs`, the outer product's `carried` rows
  equal, one for one, the union of the inner documents' own `minted`
  rows (by `(of, mate)`), with every carried `faces` pair an image
  under the graft's map of the inner pair; `carried_unminted` likewise.
  A three-level assembly (part in sub-assembly in assembly) carries
  with `via` naming the route.
- **A2 — a carried refutation names its mate.** MATE-6's P-class
  fixture (a part with a declared contact instantiated under a
  placement that contradicts it): the outer gate's finding attributes
  `Carried { … relation: Refuted }` naming the inner mate and the
  instantiating node; the merge base attributes `Unattributed` on the
  same fixture (the row shows both).
- **A3 — `Unattributed` is only the undeclared contact.** Over every
  fixture whose outer gate refuses, each `Unattributed` finding is an
  `UndeclaredContact` (or an escalation/census arm the doc names) —
  never a carried declaration's refutation or decline.
- **A4 — the fork's rows.** (A): an inner document with an unmintable
  mate instantiated into an outer document refuses the outer gate
  `CarriedMintRefusal` naming `(through, of, mate)`; the inner
  document alone refuses as today; two levels down names the route.
  (B): the outer gate certifies and the report/badge carries the
  attributed advisory. Only the ruled option's rows exist.
- **A5 — nothing else moved.** Every existing assembly, mate and
  registry row passes unchanged; DOCM-5's landing rows (one gather)
  pass unchanged; the wire round-trip is untouched (`PartValue` is
  never persisted).
- **A6 — the Python mirror** carries the forced rows and nothing
  else.

## Constraints, binding

- `docs/prompts/implementer-discipline.md` in full, by path. Hosted
  CI is the verification of record; poll it in the foreground.
- **Blinding: NO `Co-Authored-By` trailer in lane commits.**
- Merge-only; private `CARGO_TARGET_DIR` and scratch directory outside
  the worktree; `git status` before every `git add`; never `git add -A`.
- Comments state the invariant, not the history.
- Fence: `crates/editor-core/src/eval/parts.rs`, `product.rs` (the
  carry, beside `carry_contacts`/`carry_names`; not the gather order,
  not the at-rest gate's verdicts), `assembly.rs` (`attribute`,
  `assemble_gathered`'s new read, the new arms), `ASSEMBLY.md`'s D-1
  paragraph, `crates/pncad-py` for forced rows only, tests. Nothing in
  `crates/topo/*`, `mate/`, `resolve/`, the viewer beyond rendering a
  new badge arm through the existing `product_refusal`-style door.
- Do not re-verify inner records at the outer gate (D-1: records cross
  the seam, verification runs once at the outermost gate — unchanged);
  do not mint inner mates again; do not add a second attribution
  lookup.
- **Stop clause.** If the graft's descendant map has no image for a
  minted row's face on any fixture (a bridge gap, not a dropped row),
  or if carrying `unminted` needs the inner evaluation to outlive the
  seam, STOP: write what you measured in the PR as a draft and end
  your turn.

## Out of scope

Widening `assemble`'s single-refusal raise to every `unminted` row
(its own follow-up); the Python surface beyond forced rows; the
viewer's presentation beyond the badge arm.

## Review

v6 dual on the frozen head, claims to falsify:

- **C1** The channel is total (A1) on an assembly the implementer did
  not choose, three levels deep; every carried face pair is an image
  under the graft's map (grep for any re-measurement or positional
  lookup on that path).
- **C2** A carried refutation names its mate and route (A2); the merge
  base did not; `Unattributed` is only the undeclared contact (A3).
- **C3** The fork's ruled option holds (A4) and the other option's
  behaviour is nowhere in the tree.
- **C4** Nothing re-verifies or re-mints across the seam; the at-rest
  gate's verdicts are unchanged on every fixture; DOCM-5's one-gather
  landing rows hold.
- **C5** The pair lookup has one definition and three callers; the
  Python mirror carries exactly the forced rows.
