---
id: a-glued-pair-is-re-read-through-the-declared-rung-which-bridges-in-band
kind: issue
title: A glued pair is re-read at its consuming site through C4's declared rung, which bridges an in-band displacement D10 says refuses
status: closed
opened: 2026-10-29
closed: 2026-10-30
priority: P0
cost: H
refs: [d10-exit-walk]
pr: 6079
branch: contacthold/glued-pair-reads-its-record
---


Found by the D10 exit walk (`work/intent/d10-exit-walk-report.md`, clause 1).

## What

Stage 4 F (#4641) removed every public declaration of a face pair, and
the glue door (#4496) decides each pair from its margins. The glue
table is then read at the consuming sites through the ladder rung that
was C4's declaration: `vtxfac.rs:978` and `recl.rs:140` build
`PlaneIdentity { declared: declared_one_carrier }` from the glue table
(`declared.class_of`), and `merge_faces.rs:2669` passes
`PlaneIdentity::DECLARED` for its `declared` argument.
`PlaneIdentity::declared` (`plane_eq.rs:116`) routes
`carrier_eq_reading` (`carrier_eq.rs:365`) and `plane_ladder`
(`plane_eq.rs:202`) to `carrier_eq::declared_reading`
(`carrier_eq.rs:700`). That reading bridges a displacement whose upper
bound is in band at every consumed point (`ContactVerdict::Bridged`,
`carrier_eq.rs:790`). The undeclared path refuses the same in-band
offset (`PlaneEqError::Undecided`, `plane_eq.rs:279`).

DESIGN `[coincidence]`: "a margin decided Zero glues them, a definite
one keeps them apart, and one in the sliver band between refuses." The
topo README's C4 is the clause that says a declaration bridges the
residue, and DESIGN's state-of-the-build list retires it.

## The question

Is the second read a decided coincidence being read again, which has
to agree with the door's decision? Or is it a fresh decision over a
different extent, which D10 says refuses in band? If it is the first,
the rung's names (`declared`, `DECLARED`, `declared_reading`, "F5") and
its docs (`plane_eq.rs:9-17`) are the only residue. If it is the
second, `Bridged` admits an in-band sliver at that site, and
`reachhold/a-settled-declared-coincidence-crosses-a-tight-volume-bound`
is one of its consequences.

Related: `the-at-rest-face-face-confirm-bridges-an-in-band-relative-curvature`
(the at-rest half), `boolean-refusals-for-door-glued-pairs-keep-declaration-names`
(the refusal names).

## The answer (designer pair, 2026-10-30; no [ev], since ratified text is unchanged)

Both designers recommended the same final state independently. The blinding and the first reports are on `analysis/design-fork/glued-pair-reread`. `[coincidence.booleans]` "Nothing is decided twice" and `[verdicts.closeness]` "from then on they are one cell" already state it.

The premise was not quite right. The consuming read is written as a fresh decision, but for a decided pair every outcome except the relation is unreachable, and the relation is already stored (`GlueClass::relation`). Places that do measure again, and can bridge:
- `merge_faces` measures over the result fragments' extent;
- the n-ary fold step's `Given` pairs are read declared over the step's extent;
- `vtxfac`'s glued-sector tilt.

**The final state (this unit):**
1. **The door decides once and records once.** A `GluedPair` keeps the margin the door decided on. The one-carrier arm of `verify_declared_contacts` / `verify_one_carrier_declaration` writes the coincidence row from that reading rather than re-running a ladder. Check that the recorded margins `unproven-coincidence` replays are unchanged.
2. **A consumer reads the certificate.** `recl::require_same`, `vtxfac`'s conformal lump and sector tilt, and `merge_faces`' declared path take `class_of` → `relation()` for a glued pair and run no ladder. A glued pair's refusal arms (contradicted, unsettled, declared-opposite, "declared rung returned Distinct") go. If one of them is still somehow reached, it is an invariant breach, not a contact refusal. Unglued pairs keep today's corner readings.
3. **One ladder posture.**
   - `carrier_eq` / `plane_ladder` / `carrier_pair_relation` take no identity flag; `PlaneIdentity` goes.
   - The one reading: a definitely-apart datum is `Distinct`; the summed displacement bound over the consumed extent decided Zero is `SameOriented`/`SameOpposite`; anything else is `Undecided`.
   - `declared_reading`'s lower-bound and witness-floor claim machinery, `ContactVerdict::Bridged`, and `CarrierEqError::{Unsettled, Contradicted}` all retire.
   - `DoorReading::of`'s silent `None` mapping goes with them, and `GluedPairs::settled` derives from `GlueClass::relation` rather than duplicating it.
4. **The at-rest half, folded in** (`the-at-rest-face-face-confirm-bridges-an-in-band-relative-curvature`). The tangency lane's `declared` flag (`census.rs` `tangent_table_at_rest(.., true, ..)`, `contact_verify::rest_pair_reading`) goes too. The census's use of `Bridged` as a "tighten the zero band" signal for Door 2 (`census::in_band_reading`, `chart_region`) becomes an explicit tightened `Band`. `ContactVerdict` is then deleted. `section_cert/ruling.rs`'s `reading(.., true, ..)` is a different "declared" (a chart reading): check it, and rename only what is this posture.
5. **The public declaration surface goes.** `merge_coplanar_faces_declared(&[(SurfaceKey, SurfaceKey)])` and the public `PlaneIdentity` let a caller state a coincidence. The boolean passes its glue as a type only the door builds. The tests outside topo that call it are re-spelled through a boolean, or through that door-built type in a test helper.
6. **The names fold out** (`boolean-refusals-for-door-glued-pairs-keep-declaration-names`): `UnsupportedDeclarationClass`, `DeclaredContact`, `ContactContradicted` and friends. **The README** (`the-topo-readme-still-states-declared-contact`): C4's declared contact and the "Until the intent refactor lands" fence retire, as DESIGN's state-of-the-build already says they do, along with the plane ladder's module doc (rung 1, "F5") and `PlaneRung::Parallel`'s "a declaration would bridge". Those three rows close with this one.

**Probes before deleting (each a row).** Each row pins today's behaviour and is then flipped or kept:
- a glued pair through a boolean, whose consumer reads `Definite` today on both operand orders;
- a glued sector with an in-band tilt at `s.arm`;
- a fold step with A–J Zero, J–K Zero and A–K in band, which shows whether today's step silently bridges.

If that last case is reachable, the fold step's transitivity is its own question: file a row, and the unit leaves the fold reading the given pair's own record without deciding transitivity. Do not decide it here.

**Dispatched 2026-10-30** by the INTENT orchestrator, branch `contacthold/glued-pair-reads-its-record`. Cost re-priced to H, with the at-rest, names and README rows folded in. Review tier: **dual H (concurrent)**. It deletes a verdict posture across topo's ladders, the census and the boolean consumers; its impact is broad, and the reachability claims it rests on are tricky.

## Closed

Closed by PR 6079: every consuming site (recl, vtxfac, the merge, the census confirm) reads a glued pair through `GluedPairs::glued_relation` / `GluedSurfaces`; the declared rung, `ContactVerdict::Bridged` and the witness floor retired. The fold-step case is reachable. At `0.7·ε` the vertex-on-face read refuses first, but a fall the corners hold inside the zero band (the reviewers' probes: `0.4·ε` across y, `−0.48·ε` along x) builds on the neighbours' records. That transitivity is filed as `a-fold-step-glues-a-pair-its-door-reads-in-band` (P1, design) and is not decided here; `a_fold_step_glues_a_pair_its_door_reads_in_band_today` pins today's Built.
