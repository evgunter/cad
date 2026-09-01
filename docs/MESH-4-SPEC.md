# MESH-4 — issue 881's remaining half: named ε operations

**Binding at dispatch** (S-MESH program, `docs/S-MESH-PLAN.md`;
difficulty logged pre-draw: **M**). Read
`docs/prompts/implementer-discipline.md` in full before starting.
Issue 881's reopen comment is the primary specification — its scope
verbatim; its NON-scope is this unit's binding gate. Read MESH-3's
two comments on the issue first (the halfcap band-shape witness and
the premise falsification): they are this unit's substrate, and the
`pole_index` find MESH-3 minted is the fifth consumer you port.

## Situation

`crates/mesh`'s ε inventory is a census of bare-f64 comparisons:
`Tol::eps()` / `SizingTols::eps` hand out naked scalars and every
terminal read spells its own band arithmetic. The reopen comment's
shape: name the operations — `separates`, `coincident`,
`dominates`, `pad` — over the terminal reads, so the inventory
BECOMES the methods and a future read cannot be spelled without
naming its kind.

## The decided open question

The comment left "ops on `Tol` itself vs a mesh-local newtype"
open. **Decided here: the mesh-local newtype.** Grounds: `Tol` is
`geom-core` and #741's configuration surface (LIB's plan; LIB holds
that half) may reshape it — growing cross-crate API on contended
ground mid-#741 is coordination this unit does not need, while the
inventory being named is a mesh-local fact about mesh's terminal
reads. The newtype wraps the band at the `Tol` seam, carries the
four named ops, and records at its definition that if #741's
surface later grows the same operations, the newtype collapses onto
it (the seam stated, the collapse a future unit's one-liner).
**Before implementing, read LIB's #741 plan doc; if their half is
already moving on this ground, STOP and report instead of racing
them.** Coordinate via a comment on issue 741 announcing the
mesh-local newtype and its collapse seam (keyword hygiene as
always).

## Deliverables

1. **The newtype and the four ops**, with semantics written at the
   definition: `separates(a, b)` / `coincident(a, b)` (the
   band-membership pair — state their negation relationship
   explicitly or keep them independent with the reason),
   `dominates` (band-relative magnitude), `pad` (band widening for
   conservative bounds). Each op's doc names the D2-addendum row
   its callers' decisions live on.
2. **Every terminal read in `crates/mesh` ports onto the ops** —
   the census is MESH-3's corrected inventory (walk.rs 14: 4
   params + 5 hand-offs + 5 terminal reads incl. `pole_index`'s
   find, `coincident_declared`, the pole guard's own read;
   `gap_is_noise`; `iso_side_starts`; plus the crate-wide terminal
   reads the inventory pin counts). The eps-inventory pin row
   updates to count METHOD CALLS, not bare comparisons — the pin
   is the mechanical form of "the inventory becomes the methods."
   A read that genuinely cannot port (a raw f64 needed for an
   external seam) is recorded at the site with the reason.
3. **`Tol::eps()` / `SizingTols::eps` stop handing out bare f64s
   to mesh code**: mesh-internal callers go through the newtype;
   the raw accessors' remaining mesh callers (if any must remain)
   are enumerated and pinned so a new one reds the inventory.
4. **THE BINDING GATE — no mesh byte moves.** The issue's
   non-scope verbatim. Ship a byte-identity pin over the tour
   corpus as the unit's own gate (the two-build FNV instruments
   are in-tree: `r2_bytes`, `r1_probe_hash`, plus MESH-5's
   branch-reaching wedge hashes — run them across your refactor
   and pin the result). Any op whose introduction would change a
   decision is a defect in the port, not a tolerance to adjust.
5. **Rows**: the inventory pin in its new method-counting form
   (red-first against a bare comparison reintroduced); each op's
   semantics pinned once at the newtype (band edges, the
   `pad` widening direction); the ported guard/classification
   rows stay green untouched.
6. **ε posture** (issue-1356): this unit is ABOUT the ε reads —
   run the three-ε battery and state that the port is
   band-transparent (the ops compute the same booleans the bare
   spellings did, pinned by item 4).
7. **Class sweep** (discipline §5): bare `<= eps` / `< eps` /
   `eps *` arithmetic in `crates/mesh` after the port — the
   residue is the item-2 exception list, nothing else.

## Acceptance

- Byte-identity pin green across the two builds (item 4); the
  inventory pin counting methods; every terminal read ported or
  excepted-with-reason; hosted CI green on the final head; gate
  record per head in the PR.

## Hard rules

- NO `Co-Authored-By`, no model names. "issue 881" spelled out, no
  closing keywords (the orchestrator closes at merge if this
  completes the issue's remaining half — say in the PR whether it
  does).
- Scope fence: `crates/mesh` (the newtype's home, the terminal
  reads, suites). NOT: `geom-core`'s `Tol` (#741's/LIB's ground —
  comment, don't edit), other crates' ε reads, `walk.rs`
  classification DECISIONS (the reads port, the decisions must not
  move — item 4 is the proof), `docs/MODEL-AB-LOG.md` /
  `docs/S-MESH-*.md` / SMELL edits.
- Re-merge main before opening the PR.
