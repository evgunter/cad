# DECIDE-3 — the canonical root at the mint site: every `sqrt` atom keyed on its argument's value class (spec)

**Program:** DECIDE (`work/decide/plan.md`). **Items:**
`work/decide/the-candidate-norm-needs-a-canonical-square-root.md`
(SYM-10's Phase 1 finding, RULED by Ev on #2970 as shape 1 — the full
form at the mint site), `work/decide/the-decision-door-is-opaque-to-the-tier.md`
(the FRAME fold this program owes outward; `work/sym/` holds the file),
and SYM-10's three red rows. **Track:** protocol v7 **IN** — a
canonical-form decision over every atom the tier keys (the walk's,
`trig`'s, the registry's), whose impact is broad and whose keying
would be hard to change later. The full v6 dual. Block DECIDE-B1
slot 0. **Pre-draw fields, logged before the draw:** difficulty
**H**, task-class **NUMERIC**.

- **H** — the atom key is the tier's identity of a real quantity;
  changing how it is computed re-keys every `sqrt` atom, moves every
  ledger digest, and has to be argued once for a side condition
  (`D ≥ 0`) that every mint site inherits.
- **NUMERIC** — it moves what the tier decides on the six measured
  documents and on the sign-hull rows; every split, ceiling and digest
  is the acceptance.

**Read first, in full:** `docs/prompts/implementer-discipline.md`; the
ruled row whole (the measurement, the plant table, the cost table,
the ruling); PR #2970's body (the Phase 1 record —
`mcp__github__pull_request_read` `get` on 2970; there is no `gh`);
`git show 96ffd6847:docs/SYM-10-SPEC.md` (the retired spec: its
Phase 2 reads, which this unit carries); the plants recoverable at
`2a479c267` (folds 1–4, `z`, `u`, `p`) and `2d4c986bd` (the canonical
root `q` and its narrow form `r`) — read `q` as a starting point, NOT
as the implementation (it lived in `combine`; this unit's lives at
the mint site); `crates/geom-core/src/sym.rs`'s header (`# The
form-level algebra`, `# Freezing`, the rule table, `mint_atom`),
`sym/quotient.rs` (rule E's scale step — the thing that spells
`1/X` as `(16/17)/(…)` — and its four-source denominator argument),
`sym/manifest.rs` (rule F's predicate: `positive`, `nonneg`),
`sym/signed.rs` (rule C, the certified read — the shape the two reads
take), `sym/trig.rs` (`sqrt_atom`, rule D's hand-built root),
`Sym::register_equal` and the registry (what a registrant's forms
meet), `sym/memo.rs` (the drive memo is keyed on `(budget, rules)` —
a new dial is a new key); the six documents' pins (`m10_8`, `m10_9`,
`m10_10`, `m10_bulge`, `m10_sym_profile_interval`'s ledger, the drive
memo rows) and the derived-frame rows (`m10_derived_frame_interval`,
`m10_derived_frame_tilted_interval` incl. the `sym10_phase1_*`
evidence probes); `crates/geom-core/src/linalg/vec.rs`'s sign-hull
construction on `props/sign-hull` (READ-ONLY: PROPS' file).

## The claim

`‖v_y‖ = sqrt(1/S²)` on the tilted document becomes, through rule A
and rule E's scale step, a `sqrt` atom over a QUOTIENT keyed on a form
nothing else is keyed on, while the normal's own root `S = sqrt(P)` is
a different atom over `P` — and their product, the number one, stands
in every denominator of the refused residual. Two spellings of one
real that the tier keys as two indeterminates cannot cancel; that is
the whole wall (`sqrt 4547` in the residual's early census with the
three folds planted, nothing else). **The remedy is a canonical form
for `sqrt` atoms**: the key is a function of the argument's value
class — `sqrt(N/D) = sqrt(N)/sqrt(D)` for `D ≥ 0`, each half's
rational content split out (`p = c·p'`, `c = s²·f`: `s` out exactly,
`sqrt(f)` a constant atom, `p'` a primitive integer polynomial under
the root), `sqrt(R²) = |R|` (rule F, rule C, or the `abs` atom) — an
identity of reals wherever the value exists, applied at the ONE door
every root is minted through so the walk, `trig::sqrt_atom` and the
registry key alike. Measured with the plant in `combine` (walk only):
the tilted row green at both halves and both lifts; the plate's
largest early form 288 → 28 and its freezes to 0; and the plate's
door LOSING eight registered decisions because the registrant's forms
were still keyed the old way — the loss the mint-site home is
expected to remove by construction, and the acceptance below is that
it does.

**Ratified and not re-litigated:** E12; rules A–F; the freezing
budget; the registered-identity door; Ev's ruling on #2970 (the full
form, at the mint site; a re-baseline is never a reason to skip a
change for the better); the ring item's finding that opening an atom
can lose a discharge (this unit is measured against it, and its
answer to it is uniformity, not narrowness).

## Phase 1 — before touching anything

1. **The mint sites, enumerated.** Every place a `Sqrt` atom is
   minted today (`mint_atom`'s callers in the walk, `trig::sqrt_atom`,
   the registry's forms, anything in `quotient`/`algebra` that builds
   a root) — one table with `file:line` and the argument shape each
   passes. The canonicaliser's home is `mint_atom`'s `Sqrt` arm;
   every site in the table must reach it, or say why it cannot.
2. **The side condition, argued once.** `sqrt(N/D) → sqrt(N)/sqrt(D)`
   needs `D ≥ 0` where the root has a value: `D ≠ 0` is rule E's
   four-source argument; non-negativity comes, in this order, from
   rule F's manifest predicate (`manifest::nonneg`), from `D` being
   the argument of a root the walk already minted (the walk only
   mints a root over a value that was non-negative — say where that
   is guaranteed), or from a certified read (rule C's shape — counted
   `sign_gated`, and the residual's discharge is then gated, not a
   theorem). `N ≥ 0` follows from `N/D ≥ 0`. Written in
   `sym/manifest.rs`'s header (or the module the canonicaliser earns)
   beside rule F's argument, before the code.
3. **The baseline, re-taken on this branch's head** (which carries
   `props/sign-hull`): the six documents' splits at the nominal, the
   ceilings on both instruments, the walk ledger's digests, the door's
   registered counts (`m10_10_pins`'s `carrier_matches_mapped_source`
   `[180, 0, 8, 64]` on the plate in particular), and the three red
   rows' refusals — so that what moves is measured against a table
   the PR body carries, not against memory.

## Phase 2 — the canonical root, and the two reads

- **The canonicaliser** in `mint_atom`'s `Sqrt` arm, behind a dial
  (`SymRules::canonical_root`, on in `shipped`,
  `without_canonical_root()` the differential; the census counts
  SEVEN dials — `m10_8_pins`' A0 census and `m10_10_pins`' "nothing
  else" equality learn it; the drive memo's key includes it by
  construction). Content split, primitive polynomial, `sqrt(R²) →
  |R|`, `sqrt(N/D) → sqrt(N)/sqrt(D)` under the side condition of
  Phase 1.2 — and NO re-keying anywhere else: a root minted through
  the door is the only spelling.
- **Uniformity, pinned**: a row per mint site (the walk, `trig`, the
  registry) that the same argument keys the same atom whichever site
  minted it first, and that `sqrt(1/X)·sqrt(X)` and `sqrt(P²/X)`
  reach the identity the tilted document needs, at the scalar door.
- **The two reads** (SYM-10's Phase 2, carried): rule C's certified
  read of a `Select` (`Select(d, a, b) → a` where `d ≤ 0` over the
  box, `→ b` where `d > 0`; manifestly positive content stripped from
  `d` first) and the same read at `min`/`max` (`max(A, B)` IS
  `select(B − A, A, B)`), both counted `sign_gated`, both placed
  BEHIND every value-free fold — after A0, after rule F, after the
  canonical root has been minted — because a read that runs before
  the atom is minted re-labels theorems as reads (the plate's
  `m10_8` "inert on straight geometry" row is the pin: `numeric 263`,
  `sign_gated 0` on the slab must not move). Fold 1 (manifest order)
  only as the theorem upgrade SYM-10 measured, if it costs nothing;
  fold 2 is dropped.
- **The three rows green** with their assertions untouched
  (`m10_the_tilted_derived_boss_certifies_where_its_authored_twin_does`;
  `m10_the_derived_frames_refusal_is_not_a_freeze` — its `none` rung
  is `the-derived-frame-refusal-rows-none-rung-pins-the-retired-construction`'s
  finding and its A0 rung `a0-leaves-max-and-min-of-constants-opaque`'s:
  if a rung cannot be met by this unit's mechanism, the row is
  re-aimed ONLY with the reason from those two items in the PR body,
  and that is a deviation to state; `the_forms_the_walks_build_are_pinned_per_eps_row`
  re-baselined with what moved said).

## Acceptance

1. The tilted row and the ledger green; the derived-frame row green
   or re-aimed as above.
2. **No decision LOST on the six measured documents** once everything
   mints through the one door: every per-predicate split at the
   nominal moves UP or not at all in `symbolic_zero + sign_gated +
   registered`; the plate's eight registered decisions meet again by
   construction. Where a decision is lost anyway, the unit finds the
   defect and fixes it, and if it cannot, STOPS, files the render on
   the ruled row and reports — shape 3 of #2970 is the fallback and
   only then.
3. Every ledger digest, freeze count, ceiling and split that moves is
   re-baselined and SAID, in the PR body against the line (the
   plate's 288 → 28 and its freezes to 0 are the change being right).
   A ceiling that moves DOWN is a finding to explain, never a
   re-baseline made quietly.
4. `sign_gated` moves where a decision is read and `symbolic_zero`
   gains no read; the slab is inert (the `m10_8` row).
5. Cost on both instruments disclosed against the affordability line.
6. Local checks: `cargo fmt --all -- --check`; clippy `-D warnings`
   on `geom-core` at default, `interval`,
   `interval,sym-profile-testing` (all targets) and `editor-core
   --features interval` (all targets); `scripts/doc-gate.sh`;
   `python3 scripts/work.py lint`. The hosted matrix on the PR is the
   verification of record (it runs on PRs to `props/sign-hull` — SYM-10
   showed it does).

## Scope

- Files: `crates/geom-core/src/sym.rs` (`mint_atom`, the dial, the
  header), `sym/manifest.rs` or a new `sym/root.rs` (the
  canonicaliser and its argument — the lane says which), `sym/signed.rs`
  (the reads), `sym/quotient.rs` only if the scale step and the root
  must agree on content, `sym/trig.rs` (`sqrt_atom` through the door),
  the registry's mint path, tests under `crates/geom-core/tests/` and
  `crates/editor-core/tests/m10_*` (the announced tests-family
  overlap). `linalg/vec.rs`, `real.rs` and the construction are NOT
  touched (PROPS'/FRAME's; the seam is read-only).
- No new tolerance; no value read outside rule C's gated shape.

## Review

Protocol v7 IN: the full v6 dual — the arm per block DECIDE-B1's draw,
two blinded reviewers on a frozen green head (ordinal claimed on
`main` at dispatch in DECIDE's band 8600–8699, the first), the union
fix pass on the implementer's lane, a delta by R1, the row at merge.

## Landing

PR against `props/sign-hull` while #2468 is open (the construction the
folds are for), re-targeted to `main` if #2468 lands first; the spec
deleted at merge with its `docs/DOC-LEDGER.md` entry; the ruled row
closed with the result; FRAME told on its log (the fold it waits on).
Branch `decide/3-canonical-root`, cut from `sym/10-decision-door`'s
head with `origin/main` merged in.
