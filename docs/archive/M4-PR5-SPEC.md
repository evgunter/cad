# M4 PR 5 binding spec — GeomSource + bit-identity retirement + Declare threading

Status: **LANDED — PR #102, merged 2026-07-25** (historical record;
review outcomes and rulings in docs/M4-LOG.md). Originally:
BINDING for the PR 5 implementer (M4-PLAN item 5; F5/F7
ratified; NAMING-DESIGN N6 normative; DESIGN.md M4 roadmap entry the
retirement contract). Deviations via the REPORT mechanism only.
Written by the orchestrator 2026-07-25, post-PR-4 merge (#96).

## D1 — GeomSource carried on every description

`GeomSource { node: RecipeNodeId, expr: ExprPath, orient: Or }`,
`Or ∈ {Id, Rev}`, exactly as N6 writes it. Every surface/curve/point
description minted by evaluation carries the recipe expression that
produced its parameters. Composition rules are N6's: a Transform node
composes into `expr`; `revert` flips `orient` (`rev ∘ rev = id`).
Same-source is SYNTACTIC identity of `(GeomSource, orient)` — a
provenance lookup, zero numerics. Binding caveat from the PR 1
review (recorded ruling): ExprPath same-slot ancestor replacement
silently re-points stale paths — GeomSource must NOT assume
re-point detectability; identity claims hold per evaluation against
the current doc, never across unaudited doc mutations.

## D2 — Consumer migration: the declared rung becomes source lookup

`merge_faces.rs` (declared rung of `merge_coplanar_faces`) and
`plane_eq.rs` (`oriented_plane_eq`) migrate to `(GeomSource, orient)`
comparison. The M3-era implicit path (records minted by the op from
structural/declared coincidence) REMAINS; only the identity test
changes. No behavioral widening: equal bits WITHOUT shared source
stay unglued (rung (b) is ratified; the converse of N6's theorem is
deliberately unclaimed).

## D3 — bit_identity leaves production

`geom_core::bit_identity` becomes debug/test-only, surviving as
`debug_assert!(same_source ⇒ eq_bits)` at the migrated call sites —
the "records agree with bits" assertion DESIGN.md's M4 entry
promises. CI bit-identity tripwires update to an EMPTY
production-consumer allowlist; `interval.rs`'s entry is renamed to
its true justification (scalar plumbing, not coincidence) and stays.
The tripwires themselves stay armed (a new production consumer must
fail CI). DESIGN.md ratification text waits for the PR 8 exit sweep;
this PR updates only the tripwire allowlist + doc notes.

## D4 — Declare threading (F5, the A7 fix)

The `Declare` node (exists since PR 1; edit-time existence-validated
per the PR 1 D3 carve-out) is THREADED at evaluation: coincidence
intents as StableName pairs resolve through the operands' name
tables (PR 3/4 resolution machinery) into the contact records the
boolean already consumes. A reused 3′ body's declarations re-enter
downstream ops by NAME, never by arena key. Resolution failures are
the N5 typed errors — a Declare naming a vanished/ambiguous name
refuses loudly with that ResolveError; no silent drop, no
best-effort gluing. `UndeclaredContact` on 3′ reuse becomes a
certified pass exactly when the recipe declares the surviving
intent, and stays a loud refusal otherwise.

## D5 — Merged names go live

Declare-glued merges are the FIRST eval-level minting of N3
`Merged` rows. Obligations banked on this moment, all landing here:
PR 3 R4 (Merged lane end-to-end fixture — retire the synthetic
merge_groups unit REPORT line), PR 3 R8 (Merged-collision
discriminator, resolve the code comment), PR 4 review Finding 10
(pin that PR 7's coarse `vanished_candidates` reappear among PR 4's
`offers` on a REAL Merged row — the claim currently rests on
reading, not a pin).

## D6 — Acceptance showcases (all pre-built, all counted)

1. **M3 closure corpus**: the rows that 3′-refused with
   `UndeclaredContact` now certify with recipe-declared intent; the
   envelope entry update rides the PR (text queued for the PR 8
   DESIGN.md sweep).
2. **Corner-aligned table** (`crates/topo/tests/demo_tripwires.rs`):
   the PRIMARY tripwire (second leg unions once flush faces glue via
   the declared rung) must FIRE — follow its baked demo-upgrade
   instructions in this PR. The secondary tripwire (tier-3
   `DescriptionNotAdjacent` on in-plane seam edges) fires only if
   source-identity gives those edges an honest description — REPORT
   whichever way it lands; do not force it.
3. **#91 flush-plane pair**: the H×T coincident-plane variant (the
   pinned refusal) glues when authored WITH a Declare; the
   decoupled variant is untouched. If the demo-refresh branch has
   landed its lap-joint tripwire, that wire must fire too (check at
   start; REPORT if the demo branch is unmerged and bank the wire).
4. **R13**: seed the naming/resolution corpus with the
   boolean-of-boolean document shapes from the #90 fixture
   (`issue86_double_subtract` promoted to recipe documents).

## D7 — #95 disposition (CONDITIONAL — check before implementing)

If Evan's 👍 has landed on the REVISED #95 ruling (comment
5077393718 — ask the orchestrator; do not poll GitHub yourself):
implement **disposition 2 with the recursive key**:
`naming_key(N) = H(content_key(N), [(input_id_i,
naming_key(input_i)), ...])` — computed like content keys but
including input node ids, composing through the DAG (the one-level
context check of disposition 1 fails the grandparent re-point case;
see the #95 thread). Memo hit still requires content-key match; a
naming-key mismatch reuses the geometry half and re-derives the
naming half — or re-runs the whole op if emission is not cleanly
separable (correctness identical; REPORT which one lands).
Regression pins: the PR 4 fix-pass fixture re-run WITH memo
transfer (`FromB(cap-of-c)` exists, `FromB(cap-of-b)` vanishes with
an honest diagnosis) AND a grandparent-case pin (re-point X's input
to a twin; N's table must embed X's re-derived names). If the 👍
has not landed, SKIP entirely (it forks to its own PR) and put one
REPORT line saying so.

## D8 — Out of scope (do not touch)

Issue #93 (A×Z join-stage vertex-only-probing gap) is a kernel fix
with its own fixture, scheduled separately — do not fix it here even
though Declare work is nearby. No persistence (PR 6), no schema
work, no new feature nodes, no GUI. Boolean-of-boolean EXCLUSION is
lifted (#90 merged) — R13 seeding above is in scope; new join-stage
surgery is not.

## D9 — Process (standing, unchanged)

One implementer + one adversarial e2e reviewer + one fix pass.
OUTPUT DISCIPLINE per the standing header (≤~150 lines per tool
call; skeleton first; chunked reads; report ≤150 dense lines).
Persistent clone under ~/.local/share/cad-work/, never /tmp; commit
AND push after every coherent unit; RAM discipline on the 5G box
(pgrep for other cargo runs; sequential batteries). Fail loud, no
escape hatches; every refusal typed. Merge gates on green hosted
Actions checks. CI additions land as ci.yml rows (ci-local.sh kept
in sync as the billing-outage fallback).
