# ASM — the assemblies implementation program (plan)

Band 3's implementation program: execute `docs/ASSEMBLY-DESIGN.md`'s
ladder (ratified via #333, design conversation #328) at its v1 scope —
**rungs R1–R2** (A1's (a)+(b)). Runs concurrent with M8 (kernel
residuals) and the bindings-parity program, per the LIB precedent.
Narrative record: `docs/ASM-LOG.md`.

## Scope

- **R1 — the body graph** (first): `InstantiatePart` (pin + explicit
  frame) and `Pattern` nodes; materialized evaluation through
  `transform_rigid` with full re-certification (A2); instance-qualified
  naming (GQ4 wrapper × N1–N7); document identity + content-hash pins
  (A4, D9); the split/inline recorded refactorings; disjoint-assembly
  validity (A5, per-solid checks). Substrate correction (R1 recon,
  2026-08-10): the ladder's "no kernel prerequisites" is false for
  multi-solid referenced documents — `graft_disjoint` is
  single-solid-source and `name_pattern` refuses multi-body masters;
  a small kernel-door extension rides in ASM-2b. A2's "memoized"
  referenced-document evaluation is also NEW machinery (today's memo
  is per-document, in-process, ε-keyed) — and its content keys are
  deliberately a DIFFERENT vocabulary from A4 pins (memo keys include
  ambient ε and resolved bits; pins hash authored canonical bytes).
- **R2 — mates, constructively**: `Mate` nodes carrying
  CONTACT-DESIGN's declaration vocabulary + alignment data (A3);
  frame-chain solving; declaration minting into the contact record
  set; planar contact verification (the existing census inventory);
  typed refusal on simultaneous systems. **Gated on the AQ3 working
  session** (the constructive-solve boundary), ratified with Evan
  before dispatch.
- **Out of this program**: R3 rides M9/C7 (the at-rest door is bound
  to that spec — CONTACT-DESIGN C7's sibling-deliverable paragraph,
  #337); R4 stays banked (instanced evaluation, mirror + its
  equivariance-audit prerequisite, import-as-assembly post-AQ1);
  rungs (c)/(d) are their own eras.

## Unit cut (refined per the R1 substrate report,
cad-work/asm-r1-substrate/report.md, 2026-08-10)

1. **ASM-1 — identity + pins (M, decision-heavy/code-light)**:
   DocumentId; the CANONICAL-bytes definition (today's save bytes
   include the edit log — the naive pin moves on undo-history;
   canonical form must be defined explicitly, with the
   metadata/appearance/witness inclusion ruled); crypto-hash dep
   choice (none in tree); the (id × pin) wrapper; the minimal
   workspace store (none exists — save/load are String↔Doc); the
   v4-additive-vs-v5 schema posture ruling.
2. **ASM-2a — `InstantiatePart`, single-solid parts (L)**: the node
   + cross-document load/evaluate + per-pin memo + the
   `transform_rigid`/graft loop (mirroring step-import's #325 door)
   + ε-seam evidence + instance-qualified naming +
   disjoint-validity evidence.
3. **ASM-2b — multi-solid referenced documents (M)**: sub-assembly
   instantiation — the `graft_disjoint` multi-solid-source
   extension (the kernel touch) + multi-body instance naming (the
   `emit.rs` refusal wall). Required before ASM-4 (a split-off
   subtree is generally multi-solid).
4. **ASM-3 — `Pattern` (S/M, gated on the C1 ruling)**: assembly
   patterns vs the SHIPPED `Node::Pattern` whose product is
   `ValuePayload::Instances` (N bodies as data, export-refused) —
   the semantics ruling precedes the spec; provenance indices
   already ride (`GeomSource::placed`).
5. **ASM-4 — split/inline (L, last; depends on ASM-1's store)**:
   new `DocEdit` arms; the codebase's first multi-document
   operation; the acceptance harness (structural +
   name-resolution identity).
6. **ASM-5+ — R2** (post-AQ3): `Mate` nodes, frame-chain solve,
   declaration minting, planar verification (census is
   planar-corpus-only today — exactly R2's need), the typed
   simultaneous-system refusal.

## Process (the standing rules, verbatim)

Substrate exploration → binding spec → one implementer + one blinded
adversarial reviewer + one fix pass. A/B v3 triples with rows at
merge (block series and dual-review ordinals SHARED across the
concurrent orchestrators — compute at dispatch from merged blinded
rows on main; next dual = 21 per the LIB close-out). Hosted CI the
only gate; merge-before-open + CONFLICTING-is-an-outage discipline;
reviews run real e2e demos. Evan touchpoints: the AQ3 session's
design PR (before R2), plus any genuine forks — including the
pending A9 candidate (the relative-freedom/component-partition
definition, in chat 2026-08-10).

## Exit shape

An assembly document authors, evaluates, validates, and round-trips
end-to-end at v1 scope: N instances + patterns evaluate to one
multi-solid body with instance-qualified names; pins move only by
recorded update-edits with mate re-verification; split/inline hold
their acceptance; constructively-solvable mate chains place and
verify; everything outside v1 refuses typed with recourse text
naming its rung. Demos demonstrate real usage per the standing
demo-purpose rule.
