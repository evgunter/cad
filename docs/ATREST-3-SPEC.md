# ATREST-3 — tier 3′ decides a sign, and says what it returns

**Binds one implementer lane.** Deleted at merge (`docs/DOC-LEDGER.md`);
`work/atrest/ATREST-3.md` is the record that survives. Read
`docs/prompts/implementer-discipline.md` in full first.

Branch `atrest/3-tier3-prime-sign`. Rows carried:
`work/atrest/tier3-prime-still-couples-plus-v-to-the-reporting-target`
(read its "The fix sketch above names a symbol the tree no longer has"
section — `PlusVSubject` is gone) and its rides-along
`work/atrest/the-multi-solid-reporting-quadrature-is-unscheduled`.

## The defect

Tier 3 (`validate_geometric`) decides check 7 at SIGN level: it
refines only until the volume enclosure excludes zero. Tier 3′ — the
local battery `tier3_local_checks_marked` behind
`validate_pseudomanifold`, its `_certificate` twin and `contact_marks*`
— still runs check 7 at the REPORTING target through the scalar's own
lane. So on a valid body whose quadrature cannot reach the reporting
target while its sign is definite, tier 3 admits and tier 3′ refuses
`VolumeUncomputable` (pinned on purpose: the `exhausted schedule` block
of `crates/sweep/tests/tcost_k3_certificate.rs`). Separately, since
ATREST-1 made check 7 per solid, the mixed passes take a FURTHER
arena-wide reporting read on a multi-solid body to hand back a body
value, and `step-import`'s aggregate gate `gate3` pays it on every
assembly import.

Both are one question: *what does a tier-3 door promise to return, and
at what derivation level does it owe the answer?*

## Settled design (decisions, not options)

**D-A. Tier 3′ mirrors tier 3.** Check 7 in the local battery decides a
SIGN, per solid, through the same face-restricted sign walk ATREST-1
built (`props::sign_certified`, taking its faces), with the hook as a
parameter so the battery's walk carries the LANE's quadrature hook
where tier 3's carries the certified one. After this unit no body is
admitted by one door and refused by the other on check 7 at the same
scalar: flip the planted `tcost_k3_certificate` row to agreement and
say what moved.

**D-B. A certificate door returns the object its check decided on,
continuable by a caller who wants the number** — the shape
`validate_geometric_certificate` already has (a `SignCertificate`,
refinable to the reporting target). So
`validate_pseudomanifold_certificate` and the `contact_marks`
certificate forms stop returning reporting-target `MassProperties` and
return that certificate; on a multi-solid body it is the per-solid
parts assembled (`SignCertificate::assembled`), which makes the
arena-wide second read unnecessary rather than scheduled. Every caller
that wants the number (`gate3`, `editor-core`'s assembly gather, the
corpora) calls the continuation explicitly, so the cost is visible at
the call site. This is a public return-type change; carry it across
every cargo root (`scripts/doc-gate.sh --print-roots`), `demos/tour`
and `demos/wild` included.

**D-C. The scalar-generic seam stays honest.** The battery is generic
over `PropsQuadLane`, which includes a `Dual` that may not certify.
Whatever the hook's type becomes, a scalar whose lane cannot decide a
sign must refuse typed exactly where it does today — no new silent
pass.

## Measure first, and report the numbers

The carrier row asks for it: before any change, what tier 3′ costs on
the corpus's NURBS-walled bodies and what fraction is check 7. After:
the same table. Report both in the PR body; a cost claim without the
two tables is not made.

## Stop clauses

- If any caller needs a value the continuation cannot give **bit-for-
  bit** equal to today's reporting read, stop and report.
- If D-B's return-type change would force a change to ratified text
  (check with `git log --all -S'<phrase>'` per CLAUDE.md), stop and
  report — that is an `[ev]` question, not a lane's.

## What you owe

The change; rows that go red without it (the planted disagreement row
flipped; a multi-solid body whose certificate door no longer takes an
arena-wide read, pinned by a count or by construction); the sweep per
discipline §5 for *a door whose doc promises one derivation and whose
body takes two*, with its hit list and blind spot; out-of-fence
findings filed on their owner's slate (discipline §6). `crates/step-import/`
is EXCH's and `crates/editor-core/` is someone else's too —
`python3 scripts/work.py territory --files -` — announce each seam in
the PR body. Set the two carried rows' status to `review` when you open the PR.

## Verification

Hosted CI on the merged head is the record: **six** `test (eps = …, n/2)`
jobs and **five** `k-lint (gate, …)` jobs (the `interval` lane axis was
deleted by RING-4; there is one build now), read from the run's jobs API
at STEP level. Own `CARGO_TARGET_DIR` outside the worktree; private
scratch, never the session scratchpad; never end a turn with background
work live. Do not touch `work/atrest/log.md`. Do not merge.

## Review tier

**DUAL** (`docs/DUAL-REVIEW-PROTOCOL.md`): a public return-type change
across several crates, and the decision of what every tier-3 door
promises — broad and hard to reverse. Class L / STRUCTURAL.
