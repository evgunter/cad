# RING-2 — `RingInterval` is a newtype over `DInterval`; poison is `dec < Def`; every certificate re-pinned with its cause

**Status: ratified at dispatch (SCALAR orchestrator, 2026-09-21).** Binds
the implementer of unit RING-2; deleted at merge per `docs/DOC-LEDGER.md`.
Read `docs/prompts/implementer-discipline.md` in full first. The item is
`work/scalar/ring-2-newtype-over-dinterval.md`; the rulings are
`work/scalar/H5.md` §RATIFIED, ruling 1 cut (ii) and ruling 2 (PR 2701).
This is the H unit of the ring chain and the one that moves certified
bounds; RING-0 (PR 2993) is its acceptance and its dry run, and its
artefacts are this spec's starting state: the dry-run branch
`scalar/ring-0-dry-run` (head f70cdcee67, carrying the `_ext` dump hook), the red-row table and the
endpoint register in the PR body and in
`work/scalar/ring-nan-poison-is-load-bearing-at-unguarded-reads.md`,
and the verdict differential in
`crates/geom-core/tests/ring_interval_differential.rs`.

## 0. The finding, and what the tree says

Two outward-rounded interval arithmetics: `RingInterval`
(`crates/geom-core/src/ring_interval.rs`, ~750 lines in 26 `src` files
of consumers) pads one ulp per op unconditionally and poisons on a NaN
pair; `DInterval` (`interval-transcendentals`) carries exactness
witnesses and a decoration, and the certification scalar
`geom_core::Interval` refuses on `dec < Def`. RING-0 measured the
verdict differential: the two REFUSE on the same inputs except in a
closed allowlist of characterised IEEE corners, all one-directional
(ring poisons where the backend resolves at `Dac`) plus the
negative-`powi` class RING-0's fix pass added (backend refuses where
the ring certifies; no production ring value takes a negative
exponent); division agrees on every zero-touching divisor. The dry run
(the newtype, surface kept, no caller edited) reds 22 rows of 3,771 at
default and one more under the feature: **sixteen tighter pins, zero
looser, zero division verdicts**, one sign clamp, one overflow, three
"a tighter bound stops dominating an `f64` sample", one structural
exactness a gate rests on; the 960-row coefficient corpus moves 436
rows tighter and none looser. The register's 31 hazards (20 one-sided unguarded reads, 8 re-mints,
3 two-sided midpoints), 2 sites conditional on what `from_certified`
returns and 3 argued safe (re-derive with the row's command) are the
sites where a real-endpointed `Trv` would pass where NaN failed —
inside the ring, only `Div` and negative `powi` of a zero-touching
divisor produce one, and today's corpora produce none. Ruling 2: a
tighter certified bound re-baselines with the cause named; a looser one
is a finding.

## 1. What this unit delivers

**The newtype, as the dry run spelled it.** `pub struct
RingInterval(DInterval)`; `is_poison := dec < Def || is_nai ||
is_empty`; `point`, `from_bounds` → the backend's constructors (the
backend's refusal set — NaN endpoints, inverted, `[+inf,+inf]` — is the
ring's, and `from_bounds` documents it); `+ − × ÷ neg powi sqr` → the
backend's operators; `sign_clamp` and the zero annihilator DELETED (the
backend's rules are the rules; the differential's allowlist is what
says where the two differed and in which direction); `lo`, `hi`,
`width`, `mag`, `contains`, `zero`, `one` as inherent readers over the
backend's endpoints; `hull` and `clamped_to` KEEP the ring's refusing
guards (a poisoned operand poisons the hull; a clamp of a poisoned
value stays poisoned) written over the backend's `hull`/`intersection`
— the dry run's red set is conditional on these two choices and the
PR body says so; `from_certified<T: CertifiedEnclosure>` returns the
DECORATED bracket the scalar carries (`Interval`'s own `DInterval`,
decoration and all), not NaI — this is the choice RING-0 could not
make and RING-3 needs (the newtype dissolving into `Interval` requires
the crossing to carry the decoration), and it is what makes the
register's `from_certified`-fed sites live hazards rather than
conditional ones: each of them is dispositioned below.
`CertifiedEnclosure` and `Enclosure` impls unchanged in meaning
(`Enclosure` goes in RING-3). `Cargo.toml` is already right (RING-1).

**Every red row of the dry run, dispositioned by its class** (the table
in PR 2993's body is the worklist, 22 rows at default and one more
under the feature; re-run it at your merge base first — the dry-run
branch already carries the `_ext` dump hook):
- the sixteen TIGHTER pins re-baseline, each with the cause named at
  the pin (which op's pad the backend's exactness witness removed, or
  which sign-clamp/annihilator rule stopped firing) — ruling 2; the
  three "a tighter bound stops dominating an `f64` sample" rows are the
  same class (the sample was inside the pad, not inside the bound);
- the sign-clamp and overflow rows are consumer changes: the rule the
  row asserted is a fact about the reals the backend does not spell
  (`[-5e-324, 5e-324]`'s square is `[0, 1e-323]` there), so the row
  asserts what the backend guarantees;
- the structural-exactness row (`geom-brep`'s Q9 exact-rule bit
  agreement) is a re-derivation of the gate's rule, not a widening
  (`memories/output-stability-as-justification.md`);
- ANY row that comes out LOOSER is a finding: stop, characterise, file
  on this slate with the row and the op, and land the unit with that
  row's number unchanged and the consumer refusing (never re-pin
  looser).
The 3,403-row `_ext` corpus's split (RING-0's item 4) is measured the
same way and reported unchanged/tighter/looser.

**The register, made executable.** Every site in the endpoint register
gets one of three dispositions, in the PR body AND in a census row:
guarded (an `is_poison()` read precedes the comparison — unchanged);
refusing-by-comparison (the site's refusal IS the NaN comparison today
— rewritten to ask `is_poison()` first, so a real-endpointed `Trv`
refuses by name; the two division-reachable sites `quad.rs`'s
quadrature area and `topo/props.rs`'s `trig_at_start` first, then the
`.lo().max(…)` family and the re-mints through `from_bounds`); or safe
by construction (the producer cannot be sub-`Def`: cite it). The
census row (in `crates/geom-core/tests/` or beside the consumers — say
where and why) re-derives the register from source with RING-0's
pattern and asserts the count and each site's disposition class, so a
new unguarded read reds. Add the real-endpointed `Trv` witness RING-0's
fix pass put in `certified_door.rs` to the differential's corner corpus
if it is not there.

**INSTR's `docs/tess-budget-data/` re-taken by its own recipe**
(`scripts/tess_budget_cut.sh`; `tools/tess-meter`) in the same PR if
`mesh::nurbs_cert`'s bounds move the counts (the dry run moved
`nurbs_cert`'s certified `uu`/`muu` figures, so expect it); the cut
commit names RING-2 as the cause per the script's convention; if no
count moves, say so with the diff.

**What must not change:** every VERDICT — no certify/refuse outcome
flips at any door on any corpus in the tree (the dry run showed none;
prove it on the merged tree by running every suite with the newtype
and reading each red row's class); every `f64`, `Probe`, `Sym` and
`Dual` bit (the ring is not a `Real`; only `Interval`'s crossing
changes); the ring's public surface (every caller compiles unchanged —
the dry run's proof); `docs/K-REPORT.md`'s roster (no `decide(`
moves; k-lint before/after in the PR body); `bounds-allowlist.sh`
(DL4's gate line stays until RING-3).

## 2. Docs

`ring_interval.rs`'s module doc: what the ring is now (a newtype with
the certification semantics — poison is the decoration — over the
backend's arithmetic), what it is for until RING-3 (the crossing and
the refusing guards), present tense; the "two interval roles" paragraph
RING-1 left says the separation is prose only — it now says the ring
is the backend. `crates/geom-brep/README.md` C9 ("in-house interval
ring … unconditional outward ulp-widening"): naming-only re-wording
caused by this change (the ring's arithmetic is the backend's; the
substrate/scalar split stays) — CLAUDE.md's carve-out; retiring C9's
clause is RING-3's and waits for Ev. `work/scalar/gate-on-the-type-in-prose-outside-geom-core.md`'s
sites that describe the ring's arithmetic: fold the ones this change
makes false. `docs/GENERICS-BUILD-COST.md`: a dated addendum with the
build-time delta at this cut (the newtype adds no dependency; measure
`cargo build -p geom-core` before/after once, reporting).

## 3. The pin

- The differential's verdict lanes and corner sweep stay green with the
  ring being the backend: the allowlist's counts collapse to ZERO for
  every class (the ring no longer poisons where the backend resolves) —
  assert that, and re-word the module doc: the allowlist becomes the
  record of what the OLD ring did, or is deleted with a sentence saying
  so (your call, stated).
- The re-pins each carry a row-local comment naming the cause.
- The register census row above.
- `ring_interval_fuzz.rs` (exact-arithmetic soundness) green unchanged —
  the backend is sound by its own fuzz; the ring's fuzz now proves the
  crossing.
- D9 on everything that is not the ring's own numbers: both
  `span_bit_identity` suites, every render cell, the tour listing
  digests, the K roster.

## 4. Sweep

The class: every consumer of a ring value in `crates/*/src` (26 files;
re-derive) — for each file: rows re-pinned (count, cause), sites
re-written for the decoration (from the register), sites unchanged;
and every test file that names the ring (36) with its red rows. The
sweep's blind spot: a consumer that reads an endpoint two-sidedly
(`mid_pad`, `Box3::center`) and re-mints — dispositioned as safe or
refusing like the rest.

## 5. Fence

This program claims no paths. This unit reaches PROPS'
`crates/geom-core/src/{ring_interval.rs, spline/*}` and
`crates/geom-brep/src/{props/*, offset_fit.rs, patch_bound.rs, ssi/*}`,
`crates/geom/src/*`; TRIM's `pcurve_cache.rs`; MESH's `chords.rs`,
`nurbs_cert.rs`; SHELL's `offset_meters.rs`; the unowned
`topo/src/props.rs`; INSTR's `docs/tess-budget-data/`; TCOST/TINT's 36
test files; `crates/geom-brep/README.md` (C9, naming-only). Announced
by the orchestrator in `work/scalar/log.md` and on the PR. Merge
`origin/main` immediately before opening the PR; `python3
scripts/work.py territory --base origin/main` in the PR body.

## 6. Verification and report

Local: `cargo nextest run -p geom-core -p geom -p geom-brep -p mesh -p
topo -p sweep` at default and `--features interval`, `--no-fail-fast`,
crate by crate (disk is shared; private `CARGO_TARGET_DIR`,
`CARGO_INCREMENTAL=0`, ≥ 8 G free before each, delete the target
before reporting); the tess-budget recipe; `cargo clippy` on the
touched crates `--all-targets -- -D warnings` at default and
`--all-features` (narrowed; say so); `cargo fmt --all --check`;
`scripts/gates/bounds-allowlist.sh`, `interval-square-allowlist.sh`,
`check-interval-cfg-additive.py`, `scripts/doc-gate.sh`; `python3
scripts/work.py lint`. Hosted CI is the verification of record; poll to
conclusion in the foreground and report the run id; the render lanes'
verdicts are read (a moved cell is a finding unless its cause is a
re-pinned bound named in the body). Report ≤150 lines: the red-row
table with dispositions and causes, the looser count (must be zero or
filed), the register census, the tess-budget diff, the differential's
collapsed counts, D9, k-lint counts, the docs re-worded with `git log
-S` commits, deviations, rows filed. Commits are plain one-line
messages: no trailers, no model or vendor names anywhere in commits,
files or the PR body (strip any auto-appended footer through the MCP
write path and verify the live body).
