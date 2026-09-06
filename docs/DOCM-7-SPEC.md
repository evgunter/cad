# DOCM-7 — `Node::Union`'s declaration channel, in member space (spec)

**Program:** DOCM (`work/docm/plan.md`), unit `DOCM-7`
(`work/docm/DOCM-7.md`). **Ratified design:**
`docs/DOCM-REFERENCES-DESIGN.md` DM4 as amended 2026-09-06 (the
"declaration channel, in member space" bullet — Ev's ruling) — read
DM4 and DM5 first; this spec binds the build and does not re-open
them. The finding it answers is
`work/docm/n-ary-union-has-no-declaration-channel.md` (closed,
pointing here): two flush placements of one prototype under a union
refuse `UndeclaredContact` with no recourse, so a flat union can fuse
only members that do not touch.
**Track:** kernel change — the standard v6 unit (binding spec, drawn
implementer arm, cross-model dual review, union fix pass,
record-at-merge; §Review below).
**Pre-draw fields, logged before the draw:** difficulty **M**, task-class
**STRUCTURAL**.

- **M** — one new optional input on an existing node (every exhaustive
  `Node` match that reads inputs or the `declare` slot; the wire
  mirror; `refactor.rs`; `persist/check.rs`'s declare validation), the
  fold's step-routing of declared pairs, the resolver shared with the
  pair boolean, the `Merged` naming arm made reachable, rows. No new
  node kind, no new naming segment.
- **STRUCTURAL** — a declaration is data routed to the step where it
  applies; every verdict is the kernel's pair verb's as today. No
  numeric decision.

## What the unit builds

**1. The edge** (`node.rs`). `Node::Union { members, declare:
Option<RecipeNodeId> }`, the same shape `Boolean`'s `declare` has: an
optional `Declare` node. `Node::inputs` returns it after the members
(the pair boolean's precedent; `Node::inputs` order is the memo's and
the key's). The variant doc's "# No `declare` field" section is
rewritten to what is true now: a declaration names a pair of entities
in the UNION's member-keyed name space and records no fold position.
`DocEdit::SetMembers` leaves `declare` as it was. The edit door admits
a `declare` input only if the node it names is a `Declare` (the pair
boolean's existing check — ONE definition, both callers). `persist/
check.rs` validates the edge as it validates `Boolean`'s. The wire
mirror and the content key gain the input (the key through
`upstream_keys` as every input does — no hand feed; the DOCM-1 review's
tag-42 lesson).

**2. Member-space names** (`names/`). A `Declare` pair wired to a union
names its entities as `StableName`s in the union's OWN name space:
`FromMember { member, of }` (a member's entity), and after this unit
also a `Seam`/`Merged`/`Fragment` row the union minted at an earlier
step (an entity of the accumulation). Nothing new is minted for this;
the space already exists (`member_view`, `collapse_name`).

**3. Routing the pairs to their fold step** (`eval/wire.rs`,
`wire_union`). Before the fold, every pair is classified by ONE
function beside `wire_union` — `route_declarations(id, members,
pairs) -> Result<Vec<Vec<pair>>, NodeErrorKind>`, one bucket per fold
step — from the `member` ids its two names carry:

- both names in member `m`: the pair is `m`'s CARRIED contact, fed as
  operand B's carried record at `m`'s step (or operand A's if `m` is
  the first member, whose step is step 1 as operand A);
- names in members `m` (earlier in list order) and `n` (later): fed
  at `n`'s step, `m`'s side as operand A (the accumulation) and `n`'s
  as operand B;
- a name that is an accumulation entity (a `Seam`/`Merged`/`Fragment`
  the union minted at step `k`) pairs with member `n` at `n`'s step
  if `n > k`, else refuses typed;
- a name whose `member` is not in the list, or that names no member
  at all, refuses typed `DeclareResolve` through the N5 ladder as a
  vanished name does — the case `SetMembers` creates by removing a
  declared member — never silently dropped.

At each step the bucket is resolved against the two tables the step
actually joins (the accumulation's table, the member's `member_view`)
by the pair boolean's own `resolve_declarations` — moved to a home
both callers name, its side-picking untouched: a name in neither
table or in both still refuses typed there. So the union reuses the
resolver and the verb (`verb.build(Union, decls).run_pair`), and adds
only the routing.

**4. The `Merged` arm is reachable now** (`names/emit_union.rs`,
`collapse`). With a declared coincident pair the pair verb merges the
two faces and the pair emitter mints `Merged`; the union's collapse
arm for it — written unreachable and saying so — executes. Rewrite
its comment to the present and pin it (A3).

**5. The refusal's recourse** (`union_refusal`). The doc sentence "a
union has no `declare` edge" and the pointer at the filed issue go;
the `UndeclaredContact` refusal from a union names the two members
(as it does since DOCM-3's fix) and its recourse says to declare the
pair on the union's `declare` input. `demos/tour`: if the die or any
tour document can now spell a touching union it could not before,
say so in the PR; do not rewrite a demo in this unit.

## Acceptance

- **A1 — the motivating case fuses.** DOCM-3's reviewer probe (two
  flush placements of one prototype under a union) with a `Declare`
  naming the two flush faces in member space evaluates `Ok`, and the
  fused body is `bit_eq` to the pair `Boolean(Union)` with the same
  declaration spelled in operand space; without the declaration it
  still refuses `UndeclaredContact` naming both members.
- **A2 — routing.** A three-member union where members 1 and 3 touch
  and 2 is disjoint: the pair declared on (1, 3) is fed at step 3 and
  step 2 receives none (a row reads the buckets); the same document
  with the members listed (3, 1, 2) fuses to a `bit_eq` body (D9: the
  order is the list's, the result is not — for a union). A same-member
  carried pair reaches its member's step as a carried record.
- **A3 — naming.** The merged face carries `Merged` under the union
  and resolves through a selector spelled against it; a member's
  untouched faces keep their `FromMember` names bit-identical to the
  undeclared evaluation's (the declaration renames nothing it does not
  merge).
- **A4 — refusals, typed, one row each:** a declared name in neither
  table; in both; naming a member not in the list (after
  `SetMembers` removed it); `declare` naming a non-`Declare` node at
  the edit door and at load (`persist/check.rs`); an accumulation
  entity paired with an earlier member.
- **A5 — the key and the wire.** Adding, changing or removing the
  `declare` edge recomputes the union and nothing upstream; the wire
  round-trip replays a union with a `declare` bit-identical; the
  corpus document that carries a union (DOCM-3's die) is unchanged
  and its digest holds.
- **A6 — nothing else moved.** Every DOCM-3 row passes unchanged
  (A1's pip removal included); the pair boolean's declaration rows
  pass unchanged after the resolver moves; `denotes_body` and the
  seat rows are untouched.

## Constraints, binding

- `docs/prompts/implementer-discipline.md` in full, by path. Hosted CI
  is the verification of record; poll it in the foreground; never end
  a turn with background work active.
- **Blinding: NO `Co-Authored-By` trailer in lane commits** (the A/B
  experiment's rule overrides the harness convention; if one lands in
  a pushed commit, note it in the PR body and carry on — never rewrite
  history).
- Merge-only: no rebase, no force-push, no squash. Push early and
  often.
- Private `CARGO_TARGET_DIR` and private scratch directory, both
  outside the worktree. Read `git status` before every `git add`;
  never `git add -A`.
- Comments state the invariant, not the history. The PR description
  carries the argument.
- Nothing here touches `crates/topo/*`, `crates/profile/*`, the
  analysis lane, `resolve/vdiff.rs`, or `crates/pncad-py` beyond the
  rows its exhaustive mirrors force (file the `.pyi`/`Node.union`
  consequences in the PR body).
- Do not add a `declare` field to `PlacedUnion`, a per-step
  declaration list, or any spelling that records a fold position; do
  not change the pair verb or the pair boolean's behaviour.
- **Stop clause.** If a declared pair cannot be routed to one step
  without reading a position (the member ids do not determine the
  step), or the pair verb's declaration door cannot take an
  accumulation-side entity, STOP: write what you measured (file:line,
  the shape) in the PR as a draft and end your turn — the orchestrator
  rules.

## Out of scope

The union seat's declaration chrome (CHROME); the Python surface
(LIB); splice (DM6, parked); `PlacedUnion`.

## Review

v6 dual on the frozen head, claims to falsify (the reviewers get these
verbatim plus `docs/prompts/reviewer-style-lane.md` by path):

- **C1** A declared touching union fuses `bit_eq` to the pair spelling
  and refuses without the declaration (A1) — on a prototype and a
  placement the implementer did not choose.
- **C2** Routing reads member ids only — grep the diff for any use of
  a list index or step number in a declaration's identity; the
  reordered list fuses `bit_eq` (A2); a same-member pair is a carried
  record at the right step.
- **C3** ONE resolver serves the pair boolean and the union (grep for
  a second copy of the side-picking); the pair boolean's rows are
  unchanged.
- **C4** Every refusal in A4 is typed and reached; nothing is dropped
  silently after `SetMembers`.
- **C5** The `Merged` arm executes on a real document (A3) and the
  untouched names are bit-identical; the key recomputes the union
  alone; the wire replays bit-identical; DOCM-3's die is unchanged
  (A5, A6).
- **C6** Every exhaustive `Node` match that reads inputs or `declare`
  gained its arm with no wildcard; the load validator checks the edge;
  the variant and refusal docs state the present.
