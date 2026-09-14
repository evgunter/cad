# SYM-4 — the cost of a form (spec)

**Program:** SYM (`work/sym/plan.md`, the cost lane). **Item:**
`work/sym/symbolic-tier-costs-95-percent-of-the-m10-3-drive.md`, ask 3
— the change — on the two in-session levers SYM-1's profile ranked
first and third. **Track:** kernel change — the standard v6 unit
(binding spec, drawn implementer arm, cross-model dual review, union
fix pass, record-at-merge; §Review). This is block SYM-B1's first slot.
**Pre-draw fields, logged before the draw:** difficulty **M**,
task-class **STRUCTURAL**.

- **M** — a representation change inside one module (`sym/form.rs`)
  and a normalisation shortcut in another (`sym/rational.rs`), each
  with a bit-identity acceptance the tree already pins; no new
  mathematics, but a hot path where a wrong order of terms changes an
  atom key and so a decision.
- **STRUCTURAL** — what a form IS in memory changes; what it DECIDES
  does not, to the bit, and that is the acceptance.

**Read first, in full:** `docs/prompts/implementer-discipline.md`; the
item's `## The profile (SYM-1)` (the numbers this unit is cut from,
and the method to re-take them); `crates/geom-core/src/sym/form.rs`
and `sym/rational.rs` whole; `sym.rs`'s header (`# Node ids are
CONTENT HASHES (D9)`, `# Freezing`, `# Cost`); `sym/profile.rs`'s
header; `memories/output-stability-as-justification.md` (bit-identity
may choose among equivalent implementations, never justify keeping
code — here it is the acceptance's INSTRUMENT, not its justification);
`memories/perf-measurement-lane.md` (what a committed timing may be
used for).

## The claim

SYM-1 measured, on the M10-3 slab at the nominal in release: **term
storage — the allocator, the `BTreeMap<Mono, Rat>` per form, a heap
`Vec` per monomial, the `Rc` and clone glue — is 57 % of the tier's
instructions**, and the same 57 % on the plate; the coefficient ring
is 10 % on the slab and 27 % on the plate, of which `Rat::from_parts`'s
gcd and two `strip_twos` after every `add` and `mul` are most (24 %
inclusive on the plate). The forms are tiny: mean 1.5 terms out, max
10 on the slab, 90 on the plate; every one is a tree of heap nodes
cloned on every `add`, `mul`, `neg` and memo insert. Nothing freezes on
the slab; the slab's bill is 10,604 plain forms per leaf at ~7.6 k
instructions each.

**The unit makes a form cheap to hold and to normalise without moving
a single decision.** Two changes, each its own commit with its own
before/after:

1. **The polynomial's storage** (`sym/form.rs`). `Poly`'s
   `BTreeMap<Mono, Rat>` becomes a **sorted `Vec<(Mono, Rat)>`** in the
   SAME order the map iterated in (`Mono`'s `Ord`), so `digest` feeds
   the hasher identical bytes and every atom key — hence every
   decision — is unchanged. `insert`, `add`, `neg`, `mul` and
   `as_constant` are rewritten over the vector (merge, not map
   lookups; `mul` accumulates into one vector and sorts-and-merges
   once, or inserts by binary search — measure which, state the
   winner). `degree()` walks every term twice per product and twice
   per `within`: cache it on the `Poly` (recomputed on mutation) or
   compute it once in `mul`'s pre-bound and pass it down — measure,
   state. **Whether the monomial goes inline** (a small fixed-size
   array for the one-to-three-indeterminate case, spilling to a `Vec`
   past it) is the second step of this commit, taken only if the
   first step's numbers say the per-monomial allocation is still a
   top-three cost; no new dependency without a line in the PR body
   saying why a hand-rolled small vector would be worse
   (`memories/review-and-dependency-policy.md`: ~2-week minimum age,
   supply-chain sanity).
2. **The ring's normalisation** (`sym/rational.rs`). `Rat::from_parts`
   runs a gcd and two `strip_twos` after EVERY product and sum. On the
   dyadic shape — `den` is one, which `exp2` already carries — the gcd
   is a no-op that costs a binary gcd over two `u128`s: skip it when
   `den.is_one()` and go straight to `strip_twos`. Read `Rat::add`'s
   alignment and `Rat::mul` for the same shape. The coefficient BOUND
   is not touched: a coefficient past `COEFF_BITS` is refused exactly
   where it was, so `frozen` does not move.

**What this unit does NOT do, and why.** The drive-scoped plain memo
(SYM-1's second proposal — the plain form is a function of the content
hash, so its memo could outlive the leaf) is a session-model change
with receipt consequences (`frozen` per leaf, the opaque sequence
across leaves) and goes to Ev as a decision document once this unit
has said what an in-session form costs. The `Decide` impl's assertion
discharge (a soundness cross-check that is a second walk) stays as it
is; its cost is on the item and it is a separate question. The
per-node A/B reduction (53 % of the plate) is rule work, not storage,
and is not this unit's. `COEFF_BITS`, the budgets and the rules do not
move.

**Ratified and not re-litigated:** E12; the atom keying by normal
form; D9's content-hash ids; the freeze discipline (a refusal is a
counted freeze, never an allocation to the ceiling).

## Phase 1 — before touching anything

Re-take SYM-1's instruments on the merge base, so the before is YOURS
and on YOUR box: `sym_profile_slab_replays` (release) for the counts,
the callgrind rows for the slab and the plate at the nominal (self
and inclusive listings), and the M10-3 chamber drive's wall in the
test profile (`m10_3_r1_probes_interval::the_driven_chamber_replays_bit_identically…`,
sequential, once). One table in the PR body before Phase 2 begins.
Instruction counts are contention-proof; walls are local readings
and say so.

## Phase 2 — the change, one commit per lever

For each commit: the counts (forms, atoms, frozen, decisions by
outcome) IDENTICAL to the before — that is the acceptance, and a
count that moved is a bug in the change, not a re-baseline; the
callgrind self/inclusive shares re-taken; the storage class's share
before → after; the chamber drive's wall before → after (local).

## Phase 3 — the record

- **Bit-identity, pinned by what the tree already holds.** The M10-8,
  M10-9 and M10-10 pin suites (`m10_{8,9,10}_pins_interval`) hold
  every per-predicate split and every ceiling; the tier-off byte
  rows (`m10_7_r2_probes_interval`, `m10_3_driver_interval`) hold the
  off lane; the driver's accounting goldens hold the receipt bytes.
  All ride the hosted gate. **Add one row**: a digest of the plain
  form's rendering for a fixed corpus of nodes (the slab's and the
  plate's nominal decisions through `sym::report::render_of`, or the
  form digests themselves), captured on the merge base and asserted
  at head — the representation is allowed to change and the RENDERED
  form is not.
- **The hosted before/after** (the item's ask, S-TCOST's charter
  clause): the `run archived tests` cost summary on your PR run
  against the merge base's nearest PR run (there is no test matrix
  on `main` push runs) for the editor-core interval shards and the
  chamber row's cpu-s; and the profile's instruction counts, which
  are the numbers of record because they do not depend on the box.
  Both in the PR body and in the item under `## The change (SYM-4)`.
- **`sym.rs` `# Cost`** re-stated with the new shares, instrument
  named; `form.rs`'s header states what a `Poly` is now and why the
  order is the map's (the digest).
- The item's ask 3 answered for the two levers; what remains (the
  volume, the ring's remaining share, the assertion) stated with its
  number as the next input.

## Scope

- Files: `crates/geom-core/src/sym/form.rs`, `sym/rational.rs`,
  `sym/profile.rs` (hooks follow the code), `sym.rs` (header, and
  only what the representation forces), `crates/editor-core/tests/m10_*`
  for the digest row. No driver, no `real.rs`, no rule module.
- **No count moves.** If a change would move `frozen` (e.g. a term
  bound that became exact where it was an upper bound), it is out —
  say so in the PR body and keep the bound's semantics.
- **No dial, no budget, no rule.**
- Sweep obligation (implementer-discipline §5): if the sorted-vector
  rewrite finds a second map-per-form (`IdMap`, the memo maps, the
  atom table), name it and its disposition; those are the walk's and
  are not this unit's to change.

## Acceptance

- Every existing row green on the full hosted matrix; the new digest
  row green; every count in the profile identical before → after on
  both documents.
- The storage class's share on the slab measurably down (state the
  number; the bound is 57 %), the ring's `from_parts` share on the
  plate down; the chamber drive's local wall down, with the hosted
  shard timings stated beside it.
- The record in the item, the header, the PR body.

## Review

The full v6 dual (`docs/MODEL-AB-LOG.md` protocol v6; ordinal from
SYM's band at dispatch). Claims for the reviewers to falsify: (1)
every decision is bit-identical — the pins, the goldens, the digest
row, and their own mutants (a term order swapped should red the digest
row); (2) the storage share moved by what the PR says, re-taken on
their own box by instruction count; (3) `frozen` and every count
identical; (4) the gcd skip is sound on every `Rat` shape (a
non-dyadic `den` still reduces); plus `docs/prompts/reviewer-style-lane.md`
in full. Union fix pass on the implementer's lane; delta by R1; the
row lands at merge.

## Landing

Status `review` on `work/sym/SYM-4.md` when the PR opens; the
orchestrator closes the unit and deletes this spec at merge. The item
stays open on the levers this unit does not take.
