# SYM log

Newest entries at the bottom; the tail is the program's live status.
Plan: `work/sym/plan.md`. A/B band 4700–4799
(`docs/MODEL-AB-LOG.md` owns every live experiment number).

## Opening state (2026-09-13)

Opened at M10's exit sweep, on Ev's call in chat that day: M10's walk
(#1700) was ratified and its twenty-two open rows re-homed, and of
those **fourteen stand on one territory** — `geom_core::sym` and the
registered-identity door beside it — which `work/README.md` calls a
successor's opening slate rather than residue. Ev's word: "the 14 to a
successor program, the others to either another successor program or a
preexisting program." This is that program; the other eight went to
PROPS (4), BLEND (2), SHELL (1) and CENSUS (1). The sweep is
`docs/DOC-LEDGER.md` sweep 13 and M10's walk is recoverable at the SHA
it names.

Items re-homed into this directory at opening, by header edit and
`git mv` only (ids unchanged), all from `work/m10/`:

- `real-margin-dependency-widening`
- `plate-ceiling-is-now-the-scaffold-pushforward`
- `rule-d-reaches-the-unit-bulge-only`
- `interval-self-dot-straddles-before-rule-a`
- `param-box-certification-of-implicit-quantities`
- `declared-tangency-needs-the-registered-identity-door`
- `the-span-identity-is-not-a-theorem-of-the-floats`
- `the-witness-slack-is-eps-independent`
- `sym-registration-flattens-two-axes`
- `symbolic-tier-costs-95-percent-of-the-m10-3-drive`
- `derived-frame-placement-freezes-on-the-symbolic-lane`
- `symbolic-tier-census`
- `sym-rs-is-one-file-with-a-347-line-header`
- `registered-is-spelled-five-times-and-pinned-once`

**Territory, written on both sides.** `crates/geom-core/src/*` is
PROPS' glob; this program claims `sym.rs` and `sym/*` inside it and
PROPS' `keep_out` names this program in the commit that opens it. The
door's own file, `geom-core/src/real.rs`, stays PROPS' — SYM reaches
`register_equal` / `SymRegistration` / `WITNESS_REL` there by
announced seam, and the `Real` trait itself is SCALAR's subject.
`crates/geom-core/tests/m10_*` is claimed here and is also S-TCOST's
and S-TINT's by their `crates/*/tests/*` declaration; that overlap is
the `*/tests/*` family `work/README.md` describes and is recorded from
this side only, as every other program's is.

**Two rows this program's door waits on are NOT here.** The fillet's
declared tangency cannot be registered until the constructor hands the
joint classifier its centre, and the revolve carriers cannot state
their span identity until the builder is handed the far endpoint —
both are constructor changes in `crates/profile` and
`crates/sweep/src/revolve`, so both went to BLEND
(`fillet-tangency-is-not-the-constructors-node`,
`revolve-carriers-state-only-the-rim`). This program holds the
consumer of the first (`declared-tangency-needs-the-registered-identity-door`)
and it is open rather than parked: `blocked_on` would be true of it,
but the row is worth reading whole and the dependency is stated in its
body and in the plan's door lane.

No unit is cut and no branch exists yet. The first dispatch claims its
ordinal from the band above and records it in `docs/MODEL-AB-LOG.md`.

## Orchestration opens (2026-09-13)

Ev, in chat: "you're the new orchestrator for work/sym … if not, you
can just get to it." No questions blocked the start; what was decided
unilaterally is below, each with the alternative it beat.

**This box.** Single-orchestrator remote container (four cores, ~25 GB
writable at opening), GitHub through the MCP tools, no `gh`, no
away-channel monitor. The orchestrator branch is the session's
designated `claude/pensive-hamilton-12ar36` rather than
`sym/orchestrator` (PROPS' and FILLET's precedent); unit branches keep
the `sym/` prefix. Lanes are Agent worktrees under
`/home/user/lanes/<lane>` with private `CARGO_TARGET_DIR`s at
`/home/user/<lane>-target`, each copied from one warm dev build of
`geom-core` and `editor-core` with tests under `interval`
(`/home/user/sym-seed-target`, 3.5 GB); at most two heavy lanes at
once. Briefs at `/home/user/sym-briefs/` (lane-local; the spec is the
binding text). `[ev]` PRs, when any open, are subscribed so their
comments wake the session.

**The first wave, and why these two.** The slate's three H rows that a
unit could take today all start with a measurement the item itself
asks for before any change — the cost row ("the first work is the
profile rather than a fix"), the bulge row ("a measurement of WHAT
stands at `bulge = 2` before any rule is proposed"), the widening row
("the next thing to measure … ranked"). So the first wave is
measurement plus the hygiene that makes every later tier diff
reviewable, and the first DUAL unit is cut from what they return:

- **SYM-1** (`sym/1-profile`, `docs/SYM-1-SPEC.md`): the profile
  inside the normal form on the M10-3 slab — asks 1 and 2 of
  `symbolic-tier-costs-95-percent-of-the-m10-3-drive`, no fix. Outside
  the experiment (moves no decision): one review that re-runs the
  measurement, no ordinal. The alternative — cut the cost FIX now from
  the design's own hypothesis (degree growth from carried denominators)
  — is exactly how the degree-16 hypothesis was refuted, and the item
  says so.
- **SYM-2** (`sym/2-split`, `docs/SYM-2-SPEC.md`): the three moves
  `sym-rs-is-one-file-with-a-347-line-header` names (the coefficient
  tower to `sym/rational.rs`, the polynomial to `sym/form.rs`, the
  header distributed with them) plus the header's archaeology cut to
  invariants, each cut listed. Style review, no row. Runs concurrently
  with SYM-1 and lands FIRST; SYM-1 re-sites its feature-gated
  counters on the split tree. The alternative — serialise them — costs
  a lane-day for a merge that is a relocation if the moves stay pure.
- **SYM-3** (next free lane): what stands at `bulge = 2`, rendered as
  M10-10 rendered the plate's four (`rule-d-reaches-the-unit-bulge-only`'s
  first ask; R1's segment boss is in tree). Spec cut when a lane frees.

**Not first, and why.** `real-margin-dependency-widening`'s ranking is
a query over the K CSV plus the expression each margin came from, which
the DAG can answer (occurrences of a parameter in the node against in
the normal form) — a real instrument, and a bigger unit than the bulge
render; it follows SYM-3. The door rows wait: the fillet consumer on
BLEND's constructor change, the witness-slack and span-identity
decisions on "the unit that next touches the door", none of which is
this wave. `param-box-certification-of-implicit-quantities` is a design
conversation before it is a unit and opens with Ev when the ceiling
lane has said what bounds the plate after the widening class is
measured. `plate-ceiling-is-now-the-scaffold-pushforward`'s fix half is
PCURVE/D3's and is opened with TRIM, not here.

**Seams announced at dispatch**: SYM-1 → PROPS (`work/props/log.md`:
the `sym-profile` feature line in `crates/geom-core/Cargo.toml` and its
dev-dependency forward in `crates/editor-core/Cargo.toml`, the shape of
`identity-pass-testing`; `drive.rs`'s cost note corrected only if the
profile shows it wrong) and → S-TCOST and S-TINT (`work/tcost/log.md`,
`work/tint/log.md`: a new `#[ignore]`/feature-gated row file under
`crates/editor-core/tests/m10_*`, registered in `tests/all.rs`, costing
the gate nothing). SYM-2 touches only this program's files.

**A/B**: neither unit draws; no block is opened. Block SYM-B1 opens
with the first dual unit, whose pre-draw fields go in
`docs/MODEL-AB-LOG.md` before the draw. Lane commits carry no model
trailer regardless, so the convention is uniform when a blinded unit
follows.
