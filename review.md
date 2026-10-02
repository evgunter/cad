# Review of PR #3844, frozen head 7d72667dd1

Lane `reach-dual3844-r1`. Wall clock 14:00–14:55 UTC, 2026-10-02. Glimpses: none. I read the PR body via `get` only, no comments or reviews, and fetched no other `analysis/reach-dual/*` branch. CI on 7d72667: `test` and `lint` are green (check runs read).
**Verdict: NOT-MERGEABLE-AS-IS.** MAJOR 2 · MINOR 3 · NOTE 3. Probes are in `probes/`: `probe_r1_backstop.rs` goes into `topo/tests/all.rs`, the snippet is appended to `sweep/tests/reach_continuation.rs`, and `mutants.sh`/`mutants.log` hold the mutant runs. Oracles are box and closed-form arithmetic plus a parametric point-in-solid test, never the kernel.

## MAJOR
**M1. The declared-pair allowance lets out MAJ-1's own wrong component, which main refuses.** `crates/topo/src/boolean/ops.rs:1662-1680` (sure; DEMONSTRATED BY EXECUTION, `probe_declared_large_face_lets_a_wrong_component_out`).
- **Fixture.** MAJ-1's body: a 2 m plate carrying a 3 mm wrong cube as extra height (ops.rs:3625). I planted it through `AtRestPolicy::gate_volume_backstop` with the two plates' tops declared once.
- **What passes.** With nothing declared, every row refuses (that is main's verdict). With the one declaration:
  - at ε 1e-9: ∩ ≤ A and ∪ ≥ A both PASS the 3 mm cube (ΔV 2.7e-8 m³ against an allowance of 4e-8);
  - at ε 1e-6: a 3 cm cube passes (2.7e-5 m³ against 4e-5);
  - at ε 1e-6 and ×1e3 scale: a 3 m cube passes (27 m³);
  - at ×1e-3 scale and ε 1e-9: a 30 µm cube passes.
- **The ∪ ≤ A+B and ∖ ≥ A−B arms.** Under one declaration they pass planted excesses of 1e-9 to 3e-8 m³ (`probe_joint_arms_under_declarations`).
- **What is honest and what is not.** The `escalate` factor is the door's real ceiling: carrier_eq bridges a declared displacement only while it is below `escalate` (carrier_eq.rs:744-760). The rest of the allowance is not:
  - it ignores the verdict, so a pair the door reads `Definite` (displacement in the zero band, carrier_eq.rs:751) still gets K·ε·A, even though the glue cannot move such a pair past the zero band (by inspection). My probe's tops are identical, the textbook Definite case;
  - it uses the whole smaller face, not the consumed extent the door already holds (`DeclaredPairs::extent`, mod.rs:596);
  - it widens every arm, whichever bound the pair could actually move.
- **Against the PR's own witness.** The live residue the allowance exists for is 1.74e-12 m³. The allowance there is 8.7e-10, 512× larger (my closed form).
- **The PR's claim fails.** "It is the door's own error bound, not a constant" is true of the factor `escalate`, not of the product. The ∪ ≥ A arm the probe defeats is an existing bound, not a new one.

**M2. Duplicate declarations multiply the allowance; the door treats them as one.** ops.rs:1662 sums the raw `decls.coincident_faces`; `DeclaredPairs` collapses it into a map (mod.rs:786-790) (sure; DEMONSTRATED BY EXECUTION).
- `union_with` with the wedge-on-block pair declared 1, 2 and 100 times builds the same body each time (13.587155742749397), so the door accepts the duplicates.
- Fed the same 100× list, the backstop's allowance is 100× larger: at ε 1e-9 a 1 cm wrong cube on the 2 m plate (1e-6 m³) passes ∩ and ∪; at 1e-6 a 3 cm cube passes.
- The caller controls the allowance without limit, and nothing in the PR mentions it.

## MINOR
**m1. The unit's headline row does not exercise the interval re-derivation.** `crates/sweep/tests/reach_continuation.rs:706-710` (sure; DEMONSTRATED BY EXECUTION, mutants).
- The docstring says the backstop "re-derives that tie in interval arithmetic and builds it".
- With the interval step replaced by a refusal on the f64 margin (`nointerval`), the flush-top intersect stays green at 1e-9 and 1e-12: the operands carry declarations, and the allowance alone covers the 2-ulp tie.
- `allow0` is green too, so either mechanism alone suffices and the row cannot say which one built it.
- The only row that kills `nointerval` is CONTACT's `contact9_side_codes::a_vertex_pair_reads_a_dipping_chord_at_its_far_vertex`, which is not this unit's row. The unit's subject has no row of its own (review policy: untested-subject).

**m2. `encloses_material` is stricter than the DESIGN rule it cites.** ops.rs:1904-1955 against DESIGN.md:274-277 (likely; by inspection).
- DESIGN's +V invariant reads V/A against the linear band with "zero and escalated exempt", so an in-band negative is exempt.
- The door instead refuses any negative the interval certifies at the exact band, however small.
- The PR calls this "its ratified exemption". It reads "escalated" as "the interval straddles", not as the ambiguity band.
- I could not build a sub-ε inverted slab to show it (`brick`/`mapped_cube` refuse thin geometry). The reachable case is a door sliver whose stored geometry is inverted below ε.

**m3. The B ∖ A row passes A,B-keyed declarations to `subtract_with(&b, &a, …)`.** reach_continuation.rs:759 (sure; DEMONSTRATED BY EXECUTION).
- Re-keyed for (B, A), the verdict is the same, `FallbackExtentUnsupported`, so the pinned refusal stands.
- The row still asserts on a mis-declared call.
- In the same probe, B∪A with (B, A) keys BUILDS at the oracle (23.785398163397442 against 23.78539816339745), while A∪B refuses. That is evidence for the `rounded-stack` item's operand-order story.

## NOTE
- **n1. Mutants.** All five (allowance ×0, allowance ×100, no interval, `encloses_material` dropped, ∪ arm dropped) turn at least one row red at 1e-9 and 1e-12 (`probes/mutants.log`).
  - ×100, the ∪ drop and the `encloses_material` drop are each caught only by the planted unit tests in `ops.rs`.
  - `nointerval` is caught only by CONTACT's row (m1).
- **n2. Every row in the PR's tables matches my closed form.** Rounded half (24−(4−π)/4)/2 = 11.892699081698725; block+wedge 13.587155742747658; ±1.74e-12; B∖A within 3e-17.
  - Full `ci`-profile topo+sweep at 1e-12 and 1e-6: 3904/3904 pass, my 5 probes included. Unlike the PR body's 1e-12 run, the rigid_map row did not fail here (not this PR's).
  - Widened wedge door (`probe_wedge_door_widened`): tilts ±0.5…2.9ε, scales ×1e-3/1/1e3, ε at all three values, 200 point-in-solid samples per union. No wrong volume, no containment mismatch, no `ResultVolumeImplausible`.
  - Refusals outside the PR's tables: standing ±2ε/±2.9ε ∪ → `RestZipUnsupported`, B∪A → `JoinDesync`; at 1e-6, ×1e-3, sunk −2ε/−2.9ε B∪A → `Escalated`. Reusing (A∪B)∩A undeclared refuses `UndeclaredCoincidence`.
- **n3. The design stop is honest.** `gate`'s doc (ops.rs:2524) names the gap. But the tier-3 item's measurement table has no committed instrument (the probe was "temporary") and cannot be re-run from the tree.
  - Bounds pin 14→15: "never formed at a dual" holds, since `QuadLane::certified` is the only constructor (props.rs:2565).
  - "Reads no bracket to decide anything" (bounds-allowlist.sh:558-563) is doubtful (unsure): `closed_form` lifts T's certified bracket, and arm 1's refusal is decided on it, which matters at Probe/Interval scalars.

## Style (questions exercised: Q1, Q2, Q3, Q4, Q6, Q8 partial — `volume_backstop`/`bound_holds`/`encloses_material` and `props.rs` `face_flux` read end to end, not all of `ops.rs`)
- **Q1, likely.** `props.rs:1805` `closed_form_inputs` re-spells `face_flux`'s resolve/flatten prologue (props.rs:1839-1866), with no disclosure.
- **Q1, sure.** `ops.rs:1767` `interval_volume` and `props.rs:666` `PastTarget::interval_volume` are two functions of the same name.
- **Q1, likely.** `encloses_material` re-implements `bound_holds`' arm-1, interval and open logic for a one-body margin.
- **Q2, likely.** The fn-doc claims "a trim displaced inside the band moves a closed body's volume only to second order", and "the only one" (ops.rs:1505-1510). Nothing checks either claim.
- **Q3, sure.** `door_backstop_settled_residue.rs:82` tolerates `escalate·sin φ`, 512× the measured residue, so the row cannot go red until the door's residue degrades by more than 500×.
- **Q4, sure.** MAJ-1's row says "must refuse however much area" (ops.rs:3647). Since this PR, that holds only with nothing declared, and the row's doc does not say so.
- **Q6, likely.** The whole-face allowance and its every-arm application are disclosed (the TANG patch note) but no work item schedules tightening them.
