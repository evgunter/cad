---
id: props-two-eps-vocabularies-five-sites
kind: issue
title: props/ — S25's two-epsilon-vocabularies class is open at three signatures and two call sites
status: open
opened: 2026-08-20
github: 699
refs: [692]
---

## From GitHub issue 699

Opened 2026-08-20; 0 comments.

The residue of SMELL-SCAN **S25** outside SSI. Filed as a **cross-track handoff**: `crates/geom-brep/src/props/` belongs to a concurrent Track A lane, so #692 correctly stopped rather than widening — this issue is the concrete schedule that disclosure owes under the reviewer brief's Q6.

**The counts below correct #692's own disclosure**, which named one signature and mischaracterised two others. They come from the review's independently-shaped sweep, run specifically against the blind spots #692 declared.

## The shape

S25: a function takes both a `Band` — whose `zero()` *is* the run tolerance — and a separate `eps`, with nothing checking the two agree. The postmortem's lesson is that the ratified *"a process may not host two ε values simultaneously"* rule (`M4-PR6-SPEC.md:51`) needs a **signature-level counterpart**, because the second copy enters as an innocent `eps: f64` beside the `Band`.

#692 closed that for SSI by deleting the parameter outright — `certify_rung3`/`certify_branch` lose `eps`, `SsiDomain` loses the field, `MarchContext.eps: f64` becomes a `MarchTol` derived from the band. `props/` is the same shape, untouched.

## The sites

**Three signatures, all `(… eps: f64, band: Band)`:**

- `crates/geom-brep/src/props/quad.rs:531` — `cylinder_cut_face`
- `crates/geom-brep/src/props/quad.rs:2141` — `rational_patch_face`
- `crates/geom-brep/src/props/quad.rs:2415` — `nurbs_patch_face`

#692's body says the latter two carry the same `eps` *"without a `Band` in the signature"*. **They both have one**, identical in shape to `cylinder_cut_face`. The class in `quad.rs` is three, not one.

**Two call sites reading the global beside a locally-built band:**

- `crates/topo/src/props.rs:572` — `cut_face` (the one #692 found by hand)
- `crates/topo/src/props.rs:645` — `nurbs_face` (**not** named by #692)

Both read `Tolerance::get().eps` while the band arrives from `Band::linear()` at `props.rs:144`. This is exactly the third blind spot #692 declared for its own sweep — *"tolerances read from the global inside a `Band`-taking function"* — and a sweep shaped for it returns these two and nothing else workspace-wide.

## Why the values agree today, and why that is not a guarantee

`Band::linear()` → `from_zero_threshold(Tolerance::get().eps)` → `Band::new(zero, k*zero)` stores `zero` **unmodified** (`geom-core/src/predicate.rs:358`, `:322`, `:423`), so `Band::linear().zero()` is `Tolerance::get().eps` bit-for-bit — not "to within an ulp". Every current caller passes a linear band, so the two vocabularies coincide exactly.

That identity is **contingent on band linearity**, not typed. A non-linear band (`Band::angular_at`, or a bespoke `Band::new`) makes `zero()` something else, and any floor derived as `k · zero()` becomes a floor in the wrong units. Worth stating in whatever closes this, since it is the property the whole "identical number" argument rests on.

## Suggested disposition

Whoever next lands in `props/` takes all five sites. The SSI answer — delete the parameter, derive from the band at the one place that knows — should transfer directly; `quad.rs`'s lanes have no equivalent of SSI's *"the marcher is deliberately the untrusted f64 candidate generator"*, which was the one structural reason SSI needed a type rather than a deletion.

## Also swept and cleared, so the next person does not redo it

- `crates/mesh/src/curved.rs:88`'s `Tol { eps }` struct field — a **different** shape; no `Band` in that module.
- Where the review would look next for the remaining declared blind spots: `geom-brep/src/props/mod.rs:208-210`'s `width_len`/`target_len` (differently-named parameters), and `geom-curves/src/fit.rs`'s five `tolerance: f64` parameters.

Refs #692, and S25 in `docs/SMELL-SCAN-2026-08.md`.

## Home

S-CERT: three of the five sites are in `crates/geom-brep/src/props/quad.rs` — the program's `crates/geom-brep/src/props/*` territory — and the issue's own disposition is "whoever next lands in `props/` takes all five". The `keep_out` fence on `props/quad.rs` names only the `C3`/`D30` consolidation, which this is not.
