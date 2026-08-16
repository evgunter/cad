# ASM-R2a — the mate solve (binding spec)

Binds A3 (Mate node), A11 (constructive-solve boundary, all five
rules), **A12** (reading edges + mates-as-non-body-roots, ratified
#522), CONTACT-DESIGN C1–C4 (declaration STRUCTURE only — no
geometry verification in this unit; that is R2-b), and ASM-4's
D-2 amendment rider (ii) (the hoist predicate re-keys on cluster).
Design record: docs/ASM-R2-SPEC-DRAFT.md (recon addendum, coset
table, M9-1 seam). Pre-logged **L / NUMERIC** (the coset case
splits are decided numeric predicates; the draft's structural
pre-log amended per #409's mixed rule BEFORE the draw).
**DISPATCH GATE: M9-1 PR-2 merged** (editor-core carries the
kernel `ContactClass` re-export — the #524 seam agreement).
Deviations reported, never absorbed.

## D-1: the node

`Node::Mate { a, b, class, alignment }` per A3: `a`/`b` are
instance-qualified stable references (`InPart`-composed); `class`
is the editor-core re-export of the KERNEL `ContactClass` (M9-1)
— v1 admits `Rest`/`Tangent`; `Fit { .. }` refuses typed naming
the deferral and AQ6's recourse (the #524 cross-program
agreement: no wire reservation — Fit arrives as its own schema
version with its first consumer). `alignment` carries which
frames coincide, axis senses, and the clocking rider. A mate is a
leaf: `inputs()` stays empty (name refs are not consuming edges —
the shipped D3 carve-out).

## D-2: A12 mechanics

Reading edges are RECOMPUTED from the name heads (the instantiate
node each of `a`/`b` resolves through), never stored: one
function, recipe-data-only, no derived graph kept beside the DAG.
A9's relative-freedom partition and A11's cluster partition run
over consuming ∪ reading edges; A10's coverage/ancestor-freedom/
maintenance/gather run over CONSUMING edges only. Consequences,
each its own test: mates couple A9 components; instances KEEP
their roots on mate insert (no tip-transfer); the Mate node is an
ordinary NON-BODY root (auto-maintained, gather ignores it); a
dangling head (N5) contributes no edge until `Rebind`.

## D-3: cluster-record keying migration (the delicate step)

`Doc.placements` re-keys from per-instance-node to
per-cluster-REPRESENTATIVE (the gauge = document-order-first
instance of the mate-connected component), the generalization
placement.rs's module docs promise. Recorded maintenance edits:
mate-insert JOIN consumes the absorbed cluster's frame into the
edit; mate-delete SPLIT re-mints the orphaned cluster's frame
from its solved pose; gauge deletion rewrites the key to the next
representative — every one an ordinary recorded `DocEdit`, undo
restoring exactly. ASM-4's hoist predicate re-keys on "exactly
one CLUSTER" in the same pass (amendment rider ii). Singleton
clusters must be BIT-COMPATIBLE with today's shape: a document
with no mates round-trips byte-identically (schema disposition:
if the key representation forces a bump, take M9-1 PR-2's break —
coordinate on the #524 thread; do NOT mint a separate bump for
this unit without reporting).

## D-4: the per-pair coset solve

The draft's coset table (ASM-R2-SPEC-DRAFT "R2-a coset table") is
BINDING: primitives {frame-coincidence → trivial; coaxial →
cylindrical; planar-rest(+offset) → planar; clocking as rider},
closure set {SE(3), planar, cylindrical, prismatic, revolute,
trivial, empty}, every case split a decided predicate through the
`k_stats` funnel with a NAMED predicate (Indeterminate = typed
escalation, never silent). Fold verdicts: residual trivial →
DETERMINED; positive-dim → UNDER, refusal naming the residual
subgroup and its parameters (A11 rule 4 recourse); empty →
CONTRADICTORY, refusal naming the two mates, the failed
predicate, and the measured clash. Standalone clocking (no
carrying mate) refuses typed — the table lacks it by design.
Spanning tree from the gauge, document-order tie-breaks; tree
edges must solve DETERMINED; non-tree mates record as DECLARING
(solved nothing — R2-b verifies them). ReferenceCycle-style
diagnosis discipline throughout.

## D-5: evaluation

Compose outward from the gauge; the solved placement satisfies
its mates' coincidences BY CONSTRUCTION (Δc ≡ 0 — A3's derived
property). D9: two fresh processes, bit-identical evaluation and
save bytes. The A4 pin covers mates and cluster records through
the ordinary canonical bytes (verify: a mate edit moves the pin;
a re-solve that changes nothing does not).

## Acceptance rows

1. Two-instance coaxial+planar(⊥) chain → DETERMINED; evaluates;
   D9 two-fresh-process bit identity (evaluation + save bytes).
2. V-block (two planar, dihedral matched) → UNDER refusing typed,
   naming prismatic + its direction parameters.
3. Gap-mismatched parallel planar pair → CONTRADICTORY naming
   both mates, the predicate, and the measured clash value.
4. Cluster migration: join, split, gauge-deletion each produce
   the expected recorded edits and key rewrites, each its own
   assertion; undo of each restores exactly; no-mates document
   round-trips byte-identically (singleton compatibility).
5. The closure enumeration: a unit test per table entry
   (subgroup-type pair × case split), asserting the result type —
   the table's closedness is the proof obligation, executed.
6. A12 partition rows: mated instances share an A9 component;
   instances remain roots across mate insert/delete; Mate is a
   non-body root the gather ignores; dangling-head mate
   contributes no edge and the solve refuses typed naming it.
7. Refusals: Fit class (names AQ6 recourse), standalone clocking,
   UNDER (rule-4 text), CONTRADICTORY, Indeterminate escalation
   (construct a within-band case at an authored ε) — every
   message names its subject; every arm tested.
8. Cold clippy: CI scope + interval + pncad-py python lanes.
   k-lint fires → report, never silence.

## Standing brief lines

As ASM-4-SPEC's, verbatim (OUTPUT DISCIPLINE; foreground rows;
poll harness-backgrounded output files; kill by recorded PID only;
local-scripts/ tooling; merge-before-open + re-merge on movement +
confirm checks START; invariant comments; commit+push per unit;
PR bodies from lane-private paths, never the shared scratchpad).

## Amendments (adjudicated at review, ordinal 50, 2026-08-16)

1. **Acceptance row 1 re-worded** (implementer deviation 1 +
   reviewer concurrence: the original row was UNSATISFIABLE under
   the binding table — planar∩cylindrical leaves a 1-dim residual
   in every arm): row 1's DETERMINED chain is A11 rule 1's own
   example, coaxial + clocking (→ prismatic) ∩ planar rest with
   normal along the axis (→ trivial). The pin-in-hole and slot
   entries remain asserted in row 5.
2. **D-3 maintenance wording** (implementer deviation 2 +
   reviewer concurrence; ratified A11 rule 2 itself says the join
   is ONE recorded edit with the absorbed frame consumed into the
   edit record): cluster maintenance is CARRIED ON THE ACCEPTED
   EDIT'S RECORD (the A10 automatic-maintenance pattern),
   deterministic from the edit, replay-reproduced, undo = prior
   document value — not minted as additional DocEdits, which
   would double-log the fact and make replay order-dependent.
