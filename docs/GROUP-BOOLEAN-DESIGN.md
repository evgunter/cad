# Group boolean in the recipe layer — design conversation (D2 + F4)

STATUS: OPEN — design conversation, awaiting Evan's sign-off. Raised
by the die recipe conversion (M8-LOG "Demo-raised residuals",
2026-08-14, D2) and the heatsink's F4 note
(demos/tour/src/heatsink.rs:11–18). Substrate: fresh exploration
2026-08-15 (file:line refs below are from it).

## The problem, measured

`Node::Boolean` is strictly binary (node.rs:411). The die tour's
21-shell cutting tool costs twenty union nodes, and the fold's
accumulator side nests: the FIRST ball's cavity faces end up
**21 role segments deep** (FromA^20 then the subtraction's FromB —
the M8-LOG entry's "last ball" wording has the operand end
backwards; corrected here). Three distinct costs:

1. **Authoring**: twenty nodes to say one thing (D2 as filed).
2. **Naming/selection**: `NamePat` matches exact segment-for-segment
   paths (select.rs:416–427), so "any face of ball 7" is unwritable
   without spelling the whole 20-segment derivation. This is the
   sharpest cost and it is NOT fixed by sugar that desugars to a
   chain — the names still nest.
3. **F4 is the same mechanism**: `body_operand` (wire.rs:221–236)
   refuses a Pattern's `Instances` payload, which is why the
   heatsink's union-to-one-solid lives in demo code "honestly
   outside the document". A Boolean that accepted a LIST of bodies
   is exactly the widening that would let it accept `Instances`.

One fact makes a naïve flat union WRONG rather than merely new: the
21 pips are `Node::Transform`s of ONE shared ball node, and
Transform contributes no role segment (wire.rs:948–954) — the
operand bodies carry **bit-identical** StableNames. Today only the
FromA/FromB nesting shape distinguishes ball i from ball j. Any
group node must mint a per-operand discriminator or it aliases
immediately (the product gather's `ProductError::Naming` refusal is
the in-repo precedent for taking aliasing seriously).

## Options

**A — `Node::BooleanGroup`, disjoint-union scope (PROPOSED).**
A new node: union over an ordered operand list, each operand
denoting a Body OR a Pattern's `Instances` (flattened in list
order; index = flattened position). Contract: operands are
DECLARED pairwise disjoint — the same honest boundary
`product_named` and the graft door already state (and exactly the
boundary #382 half-1 just documented: nothing checks overlap;
declaring falsely is a false document). Lowering: ONE call through
the existing N-solid door (`topo::graft_disjoint_all_keyed`,
instance.rs:67–178 — the kernel needs NO new op and its binary
`BooleanNaming` record is never stretched). Naming: one NEW flat
segment `RoleSeg::Operand { i, of }` per constituent —
one segment deep regardless of group size; the index is
D8-structural recipe data exactly like `Loft.profiles` order
("Order is data", node.rs:305) and Pattern's ratified
`Instance { i }` precedent (NAMING-DESIGN N1 :80–84). Overlapping
unions keep the pairwise `Node::Boolean` — their nesting is honest
structure, and the S13 story is unchanged (the die becomes: tool =
BooleanGroup(21 pips) → one binary Subtract(cube, tool), which is
precisely the closed-group presentation S13 wants).

**B — edit-time sugar** (a builder that emits the pairwise chain).
No schema break, but costs 2 and 3 are untouched — the names still
nest and the selectors stay unwritable. Recorded as rejected
unless the naming cost is judged acceptable.

**C — a true n-ary kernel boolean.** Requires a new kernel naming
record (`BooleanNaming` is structurally two-operand,
ops.rs:167–227) and a general n-way intersection story. Nothing
measured needs it: the motivating scenes are disjoint groups.
Rejected as scope; option A leaves the door open (a later
overlapping-group node would be its own vocabulary change).

## What option A costs, eyes open

- **Schema v9** (new node variant + new role segment = ONE
  vocabulary change, one version, per persist/mod.rs:176–186's
  double-claim resolution rule). Costs the v8 pattern exactly:
  golden + SchemaTooOld/UnknownSchema rows both directions, the
  two version-pinning asserts, the Python mirror. The bump claim
  goes to main AT DISPATCH (the LBRET corollary), coordinated with
  LIB/ASM's version sequence.
- **New segment ripple**, enumerated: `RoleSeg` + `SegTag` +
  `sub_names` + the name hasher (next free segment tag verified at
  implementation time) + walk_names + the Python selector mirror.
  Closed list, all known sites.
- **Evaluation**: new content-key node tag (next free = 19),
  deterministic operand order = list order (D9), memo key covers
  the list.
- **Not included, named**: a `DocEdit` arm to edit an operand list
  (today no edit changes any node's inputs — "add a 22nd pip" is
  delete-and-reauthor, same as every other node; a structural-param
  edit arm is a separate future conversation). Hole/pattern verbs
  from KERNEL-VERBS stay where they are — this node is the
  substrate they were "blocked mainly on patterns" for, but verb
  sugar is not this change.

## The acceptance that keeps it honest

The die tour's tool collapses to one node; the FIRST ball's cavity
faces go from 21 segments to ≤3 (`Operand{0}` + the subtraction
wrap); a `select_where`/name row addresses "ball 7's cavity face"
directly; the heatsink's out-of-document union moves INTO the
document (F4's note retires at its origin site, both workarounds
deleted per the demo doctrine); `diefillet` exports stay
byte-identical (the geometry is untouched — same bodies, same
order through the graft door). ε/valence: no new geometry gates —
the disjoint contract is declared, not detected, stated in the
node's docs with #382's language.

## Questions for Evan

1. Option A as scoped (disjoint-union group, `Instances` accepted,
   overlap stays pairwise/binary) — sign off, or push back on the
   disjoint-only restriction?
2. The segment name and shape: `Operand { i, of: Box<StableName> }`
   — index-only, like `Instance`, or should it carry the operand's
   node id too?
3. Sequencing: this is editor-core/document-layer work,
   crate-disjoint from the M8 kernel slate — run it as an M8
   candidate unit after the current slate, or park it for the LIB
   program (whose LIBRARY-DESIGN treats node additions as
   coordination items)?
