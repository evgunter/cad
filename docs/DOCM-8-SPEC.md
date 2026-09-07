# DOCM-8 — A merged face's name is a flat constituent set, and a member-space declaration resolves through the fold's merges (spec)

**Program:** DOCM (`work/docm/plan.md`), unit `DOCM-8`
(`work/docm/DOCM-8.md`). **Ruling of record:** Ev, in chat 2026-09-06,
on `work/docm/member-space-declarations-are-order-shaped-across-a-chain.md`
(DOCM-7's review finding): a nested `Merged` name should never have
existed — **whatever mints a `Merged` mints it FLAT**; nothing
downstream flattens. The look-through half is the orchestrator's
reading of DM4's amended bullet, disclosed as such in the DM4 sentence
this unit makes true (`docs/DOCM-REFERENCES-DESIGN.md`).
**Track:** kernel change — the standard v6 unit.
**Pre-draw fields, logged before the draw:** difficulty **S/M**,
task-class **STRUCTURAL**.

- **S/M** — one loop in the pair emitter changes what it pushes; one
  arm of `collapse` becomes a refusal; one rewrite step before the
  union's resolver; three prose sites; whatever golden bits move are
  re-baselined and listed. No new node, no new segment.
- **STRUCTURAL** — naming and routing; every verdict is the kernel's
  as today; no numeric decision.

## The finding, in one paragraph

Three blocks in a chain, declared member-to-member, fuse in the orders
that fold the middle member last and refuse in the orders that fold it
earlier (`member_space_declarations_across_a_chain_are_order_shaped`,
`crates/editor-core/tests/docm7_union_declare.rs`): the middle member's
face is merged away at step 1, so at step 2 the pair names a face that
is no longer an operand-table row, and the only spelling that resolves
today is the accumulation's `Merged` row — which is minted NESTED
(`Merged([d.cap, Merged([a.cap, c.cap])])`), i.e. it records the fold
tree in the one place DM4 says no position is recorded. The names
README's N3 already defines a merged face as `Merged(sorted, deduped
constituents)`, a constituent SET: the nesting is the tree's own
contract violated, and the order-dependence follows from it.

## What the unit builds

**1. The flat mint** (`crates/editor-core/src/names/emit_topo.rs`, the
merge-group loop at ~531–565). A merge group's constituent descends to
an operand face whose name is read THROUGH its descent wrappers: peel
the `FromA`/`FromB` chain to the name's foot. If the foot is
`Merged(cs)`, the group's constituents are `cs`'s members, each
re-wrapped by the SAME chain (and then by this step's own side, as the
single name is wrapped today); otherwise the constituent is the name
as today. A published `Merged`'s constituent set therefore never
contains a `Merged` at any depth of wrapping; sort and dedup as today
(R8's loud collision unchanged). This is the ONE place the flatness is
decided — the mint site's rule for every consumer: the pair boolean
over a boolean, a merged face carried through untouched steps
(`[FromA([FromA([Merged(cs)])])]` is the shape the stop clause
measured), and the union's fold alike. *Amended 2026-09-06 at the stop
clause:* the first cut read only a foot-less `[Merged(cs)]`, so a
merged face that had passed through one untouched step nested again;
re-wrapping each constituent by the chain the whole name carried is
not a crossing of spaces — every constituent stays a face name of the
table the chain descends into, exactly as the wrapped `Merged` was.

**2. `collapse` refuses a nested `Merged`** (`names/emit_union.rs`,
the `Merged` arm). The arm keeps its sort-and-dedup over collapsed
constituents and REFUSES a constituent whose collapsed head is
`Merged` as an emission bug (`NamingError::Emission`, the same class
as `Fragment` at head), never flattens it. N3's sentence in
`names/README.md` gains the word "flat" and the rule that a
constituent is never itself a merged face.

**3. Look-through at the union's routing step** (`eval/wire.rs`,
between `route_declarations`'s bucket and `resolve_declarations`). At
a fold step, a declared name that is a member-space name
(`[FromMember { member, of }]`) and is NOT a row of either operand
table is rewritten to the accumulation's `Merged` row whose (flat)
constituent set contains it, and the rewritten pair is what the
resolver receives. `resolve_declarations` is unchanged (one definition,
two callers, exactly as DOCM-7 left it). Rules at the rewrite:
- Only member-space names look through; an accumulation-entity name
  (`Seam`, `Merged`, `Fragment`, `OutputBody`) is its own row or
  nothing, as today.
- Both names of one pair rewriting to ONE row refuse typed (the pair
  boolean's same-face refusal, whichever arm it already has), never
  fuse.
- A member-space name in no table and in no `Merged` row's set keeps
  today's refusal (`UnionDeclareStep` / the vanished rung, whichever
  DOCM-7 left for that case).
- The rewrite reads the step's operand tables and nothing else — no
  re-measurement, no search beyond a membership test over flat sets.

**4. Prose to the rule.** `route_declarations`'s doc, `Node::Union`'s
"why it records no position" paragraph, and `RoleSeg::FromMember`'s
identity paragraph (`names/role.rs`) — DOCM-7 rewrote all three to the
measured limit; rewrite them to the rule: a member-space declaration
resolves at its step through whatever merges the fold has performed,
and reordering the members changes nothing about whether it resolves.
Present tense, no history. `docs/DOCM-REFERENCES-DESIGN.md` DM4's
"Measured limit" paragraph is the orchestrator's to replace (done at
this spec's commit); do not edit it.

## Acceptance

- **A1 — the chain fuses in every order.** The DOCM-7 measurement row
  flips: all four member orders of the chain fixture fuse to one body
  of volume 2.2, and the merged cap row's constituent set is the same
  three member faces in every order (`{a.cap, c.cap, d.cap}`, flat).
  A four-member chain (two middles) fuses in every order likewise.
  Lift R1's red probe (`r1_red_member_space_declarations_survive_every_order`
  on `docm/7-review-r1`) as the row.
- **A2 — no nested `Merged` is ever published.** Over every fixture in
  `docm7_union_declare.rs`, `docm3_union.rs`, the pair-boolean naming
  suites and the die corpus, no published `Merged` has a constituent
  whose name, read through any `FromA`/`FromB` chain, is a `Merged`
  (one helper walks every table and peels wrappers — the head-only
  walk of the first cut missed the pass-through shape); and a
  synthetic nested name handed to `collapse` refuses
  `NamingError::Emission` (a row). A constituent that is a `Fragment`
  of a merged face is a fragment, not a merge: it is a legitimate
  constituent and no walker or arm treats its carrier as nesting (row
  it only if an existing suite reaches the shape; otherwise note it).
- **A3 — the pair boolean's own mint is flat.** A boolean over a
  boolean that declared a merge: the outer merged face's constituents
  are the inner merge's constituents each wrapped `FromA`/`FromB`, plus
  the partner — no `Merged` inside. Its edit-log replay is
  bit-identical.
- **A4 — the look-through is the union's alone.** `resolve_declarations`
  is byte-identical to the merge base; the pair boolean's evaluation
  rows are unchanged; a member-space pair whose two names rewrite to
  one row refuses typed; an accumulation-entity name does not look
  through (a row for each).
- **A5 — what moved, listed.** Every existing row passes unchanged
  except those whose stored names carried a nested `Merged` — list
  each in the PR with its old and new name; re-baseline the die corpus
  by the one regeneration command if any of its names move (say
  whether any did). DOCM-7's rows hold except the measurement row A1
  flips.
- **A6 — the prose says the rule** at the three sites and the README.

## Constraints, binding

- `docs/prompts/implementer-discipline.md` in full, by path. Hosted CI
  is the verification of record; poll it in the foreground.
- **Blinding: NO `Co-Authored-By` trailer in lane commits.**
- Merge-only; private `CARGO_TARGET_DIR` and scratch directory outside
  the worktree; `git status` before every `git add`; never `git add -A`.
- Comments state the invariant, not the history.
- Fence: `crates/editor-core/src/names/emit_topo.rs` (the merge-group
  loop only), `names/emit_union.rs` (the `Merged` arm of `collapse`
  and its doc), `names/README.md` N3, `eval/wire.rs` (the rewrite step
  and `route_declarations`'s doc; NOT `resolve_declarations`, NOT the
  fold order), `node.rs` and `names/role.rs` prose, tests, the corpus
  regeneration. Nothing in `crates/topo/*`, `resolve/`, `persist/`
  beyond the corpus.
- A golden or stored bit that changes is never a cost to weigh against
  the change that makes the names right: re-baseline and say what
  moved.
- **Stop clause.** FIRED 2026-09-06 on the pass-through shape and
  resolved by the amendment to item 1 above (the mint reads through
  descent wrappers; re-wrapping is not a space crossing). It fires
  again only if a constituent's foot, after peeling, is a `Merged`
  whose members are not face names of the table the chain descends
  into, or if the look-through cannot be expressed without changing
  `resolve_declarations`: STOP, write what you measured in the PR as a
  draft, end your turn.

## Out of scope

Any change to which faces the kernel merges; the two-pass authoring
hole (CHROME); `DeleteNode` and payload names; the pair verb's operand
asymmetry (its own issue).

## Review

v6 dual on the frozen head, claims to falsify:

- **C1** The chain fuses in every order (A1) on a chain the implementer
  did not choose (four members, or a chain that also has a disjoint
  member), with the merged row's flat set identical across orders.
- **C2** No nested `Merged` is minted anywhere (A2/A3): grep the diff
  for any flattening outside the mint site; hand `collapse` a nested
  name and see it refuse; build the boolean-over-boolean case.
- **C3** The look-through is one rewrite before one unchanged resolver
  (A4): `resolve_declarations` byte-identical; the same-row pair
  refuses; accumulation-entity names do not look through.
- **C4** The re-baseline list is complete (A5): every moved name named,
  the corpus regenerated by one command, nothing else moved.
- **C5** The three prose sites and N3 state the rule and no residue of
  the "measured limit" sentence survives (grep).
