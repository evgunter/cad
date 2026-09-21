# DECIDE-1 — the self-dot straddle: is any certification-path square still a product at `Interval`? (spec)

**Program:** DECIDE (`work/decide/plan.md`, the door lane). **Item:**
`work/decide/interval-self-dot-straddles-before-rule-a.md` (M10-8's
R1 NOTE-9: clause 1 refuses rule A on wide boxes because `v·v` is an
interval PRODUCT, not a square). **Track:** OUTSIDE protocol v7
(`docs/MODEL-AB-LOG.md`, Ev 2026-09-19) — triaged OUT at spec time:
the unit is a census and a measurement, and the class of fix it might
take (the tight square in place of a product of one enclosure with
itself) is ratified already (`linalg/vec.rs`, M2 PR 4; the ring's
`sqr`). An OPUS implementer and an OPUS reviewer, no draw, no ordinal,
no row; the review's depth is decided at the PR (§Review).
**Difficulty D, class NUMERIC** — recorded for the program's own
bookkeeping, not for a draw.

**Read first, in full:** `docs/prompts/implementer-discipline.md`; the
item whole; `crates/geom-core/src/linalg/vec.rs` — `norm_squared`'s
doc on both `Vec2` and `Vec3` (why `powi(2)` and not `self.dot(self)`)
and `norm_witness`'s note (the allowlist measurement, `|n.x| < 2^-480`);
`crates/geom-core/src/interval.rs`'s `powi` and its row
`powi_is_tight_across_zero`; `crates/geom-core/src/ring_interval.rs`'s
`sqr`; `scripts/gates/interval-square-allowlist.sh` (what was
litigated once, and the gate that holds it); `crates/geom-core/src/sym.rs`
— `Sym::powi` (the value channel delegates to `T::powi`, so a `powi(2)`
on a `Sym<Interval>` is the tight square) and `Decide for Sym<T>`'s
`Invalid` arm (a clause-1 refusal is the value channel's own answer;
the tier is not asked); the two R1 rows in
`crates/geom-core/tests/m10_8_r1_sym_probes.rs`
(`r1_rule_a_never_fires_on_a_straddling_argument`,
`r1_rule_a_decides_zero_at_every_width_and_off_it_widens` — the second
writes `y*y` by hand and says why); `crates/editor-core/tests/m10_8_harness.rs`
(`ceiling`, the instrument the six documents are measured on).

## The claim, and what the tree already says

The item asserts that `Vec::dot(self, rhs)` computes `v·v` as
`Σ vᵢ·vᵢ`, a product of two independent copies of one enclosure, so
that a component straddling zero gives `[-a, b]·[-a, b] = [-ab, …]`, a
`sqrt` over it is a domain violation, clause 1 refuses, and rule A
never runs. That is true of `dot` called with the same vector twice.
It is NOT how the tree spells a norm: `Vec2::norm_squared` and
`Vec3::norm_squared` have squared component-wise through `powi(2)`
since M2 PR 4, `norm()` is `norm_squared().sqrt()`, every carrier and
path length in `crates/profile` goes through `norm_squared().sqrt()`,
and `Sym::powi` hands the value channel to `T::powi`, which is tight
across zero. So the item's mechanism reaches the certification path
ONLY through a site that spells a self-product by hand — `x.dot(x)`,
`dot(a, a)`, or a scalar `e * e` on an enclosure that can straddle —
and runs at `Interval` or `Sym<Interval>`. Whether any such site
exists on a path a measured document takes is the question; the unit
measures it rather than assuming either answer.

**Ratified and not re-litigated:** the tight square as the spelling
of a self-product at `Interval` (M2 PR 4, `ring_interval::sqr`); the
allowlist gate; clause 1 (a domain violation is the numeric channel's
own refusal, never the tier's); rule A; `dot`'s association order (the
item's own D9 note: every consumer's f64 bits ride on it, so `dot`
itself is not changed here).

## Phase 1 — the census and the measurement (before touching anything)

1. **The static census.** Every site in `crates/*/src` where an
   enclosure is multiplied by ITSELF — `x.dot(x)`, `dot(a, a)`,
   `v * v` on a scalar built from vector components or lengths, a
   closed form spelled as `d.x * d.x + d.y * d.y` — that runs at
   `Interval` or `Sym<Interval>` (generic over `T: Real`, or concrete
   at the interval type), reached from the certify lane
   (`geom-brep/src/certify.rs`), the profile carriers
   (`profile/src/{seg,path,validate}.rs`), the derived frame
   (`editor-core/src/eval/wire.rs`, `newell.rs`), `unit_vec.rs`,
   `linalg/vec.rs` itself, or editor-core's evaluation. One table:
   site (`file:line`), the scalar type it runs at, the operand and
   whether it can straddle zero on a bracketed document, what consumes
   the product (`sqrt`, `sign_within`, a division, nothing decisive),
   and the spelling that would make it a square. The orchestrator's
   grep found these hand-spelled self-dots, listed so the lane can
   confirm each rather than re-find them: `editor-core/src/resolve/pick.rs:2085`
   (`d.dot(d)`, `Vec3<f64>` — f64 only), `geom-brep/src/ssi/jet.rs:261,272,291,302`
   and `ssi/march.rs:666` (f64 arrays), `geom-core/src/linalg/unit_vec.rs:498`
   (a test's tripwire), `geom/src/curves/projection.rs:161,231` and
   `surfaces/projection.rs:185` (under `mid(·)`), `step-import/src/{adopt.rs:875,chart.rs:395}`,
   `viewer/src/{pickindex.rs:2052,display.rs:545}`. The lane's job is
   what the grep cannot see: scalar self-products on the interval
   paths, and generic code whose `T` is `Interval` at a measured
   document.
2. **The dynamic measurement.** On the six documents
   (`m10_3`'s slab, `m10_7`'s plate, the annulus, the bracket, the link,
   R2's pad — the homes the `m10_8`/`m10_9`/`m10_10` pins name) at
   ε = default, 1e-6 and 1e-12, at the nominal and at ceiling + δ on
   `m10_8_harness::ceiling`'s instrument: count the clause-1 `Invalid`
   refusals (`Decide for Sym<T>`'s `Invalid` arm) per predicate; for
   each predicate that has one, render the refused residual at
   `explain_depth` and say whether the `Invalid` descends from a
   `sqrt` (or a `sign_within`) over an enclosure whose spurious
   negative lower bound is a product of one straddling factor with
   itself — a `Mul` node with identical kids, or a `dot` of a
   difference vector with itself — or from something else (a genuine
   domain violation, a poisoned enclosure, the sign-hull frame's
   Newell margin, which is FRAME's and not this unit's). One table:
   document, ε, scale, predicate, `Invalid` count, the mechanism.
3. **The stop.** If no site in the census runs at `Interval` on a
   measured path AND no `Invalid` in the measurement descends from a
   self-product, the item's mechanism does not reach the certification
   path: Phase 2 is empty, the row is CLOSED with the two tables as its
   record and the reason ("the fix landed at M2 PR 4 for every norm;
   the tier's `powi` is the value channel's"), and the R1 rows stand
   as the pins that the SOUND outcome on a hand-spelled product is a
   refusal. The PR is then the record, and it is the unit's whole
   deliverable — a measurement that closes a row is a result.

## Phase 2 — only if Phase 1 finds a site

For each production site on a measured path (never a test's, never an
f64-only one): spell the self-product as the tight square —
`norm_squared()` for a vector, `powi(2)` for a scalar — with

- the f64 bit-identity across the swap PINNED at the site's own
  consumers (a row that replays the site's document at f64 and asserts
  the bits, in the shape `interval-square-allowlist.sh` gates; `x*x`
  and `powi(2)` are bit-identical at f64 only where `powi` is one
  multiplication, which the gate holds and this unit re-reads for the
  new site — the gate's allowlist is edited only if the site needs an
  entry, with the reason);
- a row that REDS under the product spelling: the straddling
  enclosure, the `Invalid` it produced, the decision it now reaches
  (a theorem or a numeric answer, not a refusal);
- the six documents re-measured on both instruments: every clause-1
  refusal the swap removes named by predicate and document, and NO
  ceiling or split moving down anywhere (a ceiling moving UP is the
  result; one moving down is a finding to explain, not a cost to
  absorb).

Seams: `crates/geom-core/src/linalg/vec.rs` and `interval.rs` are
LINALG's (`work/linalg/program.md`, `paths`); a change there is
announced in the PR body and on LINALG's log, with
`python3 scripts/work.py territory` run on the branch. A change at a
consumer site is announced on that site's program the same way.
Shared ground is legitimate by the README's 2026-09-20 rule; what is
owed is awareness while a lane is live.

## Scope

- Files: read anywhere; write only at the sites Phase 1 names, their
  rows, the item, and the PR body. No change to `Vec::dot`, to `powi`,
  or to the allowlist gate's logic.
- No new tolerance; no new value read in the tier; no rule touched.
- The pad's whole-box shape report OOMs on a box this size: measure
  the pad on the ceiling instrument only, as SYM-8 did.

## Acceptance

- Phase 1's two tables in the PR body, with the stop clause's verdict
  stated in one sentence.
- If Phase 2 is taken: the bit-identity pin, the red-under-product
  row, the six documents' re-measurement, and the seam announcements.
- If not: the item closed (`status: closed`, `closed:` dated) with the
  tables as its record; the R1 rows cited as the standing pins.
- Local checks green: `cargo fmt --all -- --check`; clippy
  `-D warnings` on `geom-core` at default and `interval` and on
  `editor-core --features interval` (all targets); `scripts/doc-gate.sh`;
  `python3 scripts/work.py lint`. The hosted matrix is the
  verification of record for everything under `editor-core`.

## Review

Outside protocol v7: one OPUS reviewer, opus-implemented. The review
is a FULL review (`docs/prompts/reviewer-style-lane.md`'s style
questions plus claims to falsify) if Phase 2 changes a production site
— the bit-identity pin and the ceilings are correctness claims — and a
STYLE review if Phase 1 closes the row. The orchestrator says which at
the PR. No draw, no ordinal, no A/B row; the triage call is recorded
in `work/decide/plan.md` §Review posture and the DECIDE log.

## Landing

PR against `main`; the spec is deleted at merge with its
`docs/DOC-LEDGER.md` entry; the item updated (closed, or its "what is
owed" re-cut to what Phase 2 left); the DECIDE log carries the
verdict. The unit's branch is `decide/1-self-dot-census`.
