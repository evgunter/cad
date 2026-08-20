# SMELL-SCAN 2 — the fixes, and the ground the first scan never covered (2026-08-20)

**Status: REPORT ONLY.** Same contract as
`docs/SMELL-SCAN-2026-08.md`: nothing here is ratified, nothing is a
commitment, no code was changed, and no finding proposes a specific fix.
A finding is a *question worth answering*, not a defect.

**Why a second document.** The first scan is now owned by live fix
tracks (`docs/SMELL-C-LOG.md`, `docs/SMELL-E-LOG.md`) and by §D's
schedule; appending to it would collide with in-flight work. IDs
continue the same series — **`S59` onwards** — so there is one global ID
space and a citation never means two things. `S45`–`S48` remain reserved
in the first document.

**Scan base: `0714d540` (main, 2026-08-20), 916 commits after the first
scan's base `4258584`.** Line numbers are as of that commit. Claims, not
line numbers, are the content.

**Method.** Thirteen parallel scopes, in two kinds:

- **Fix audits (9 scopes).** Code rewritten *in response to* the first
  scan, read as a diff against `4258584`, looking specifically for what
  the fixes introduced. This is the higher-yield half: a fix pass
  touching a file is a fix pass with the file open, and the recurring
  outcome below is that it swept the reported instance and left the
  sibling.
- **New ground (4 scopes).** Code the first scan explicitly excluded:
  `interval-transcendentals/`, `demos/`, `tools/`, `scripts/gates/`,
  and the test files added since the base.

Every scope ran the style brief's stance and eight questions as its
method — the brief that came out of the first scan's process
observations. It has since been split into
`docs/prompts/reviewer-style-lane.md` (the reviewer's document: §1 the
stance, §2 the questions, §3 what a finding looks like) and
`docs/REVIEW-STYLE-DISPATCH.md` (dispatcher notes); the agents ran
against the pre-split `docs/REVIEW-STYLE-BRIEF.md`, whose §2/§3 are the
current §1/§2. Citations below use the live paths. **This is the brief's
first use at scale**, and §C reports on how it did.

**What I verified by hand.** Ten of the highest-stakes claims, marked
**[verified]** where they appear. Everything else carries the reporting
agent's own confidence label (`sure` / `likely` / `unsure`), unmodified.
Two claims in my own dispatch briefs were wrong and the agents caught
them; both are recorded in §C, because "the coordinator's premise was
wrong and the agent checked it anyway" is a process result.

**Independent corroboration is called out where it happened.** Five
findings were hit by two agents working from different files with no
knowledge of each other; that independence is the strongest signal in
this report.

---

## Contents

- [Tier 1 — act on these](#tier-1--act-on-these) (S59–S75)
- [Tier 2 — significant](#tier-2--significant) (S76–S109)
- [Tier 3 — real but lower stakes](#tier-3--real-but-lower-stakes) (S110–S116), as class roll-ups
- [§A. Where I would start](#a-where-i-would-start)
- [§B. Negative results and coverage](#b-negative-results-and-coverage)
- [§C. Process observations](#c-process-observations) (C15–C22)

---

# Tier 1 — act on these

## S59. The compound-`Bounds` gate is blind to `CertifiedBounds`, and the new guidance routes authors through the hole

**[verified]** `scripts/gates/bounds-allowlist.sh:139`'s matcher is
`(\+\s*(geom_core::)?Bounds\b)|(\b(geom_core::)?Bounds\s*\+)`. `\bBounds`
cannot match inside `CertifiedBounds` — the `C` before it is a word
character — so `T: Decide + CertifiedBounds` and
`T: CertifiedBounds + Decide` both fire nothing. I ran the regex against
both orders and got no hit.

Two places assert the opposite. The gate's own header at
`scripts/gates/bounds-allowlist.sh:46-48`: *"`Decide + CertifiedBounds`
would be a compound bound again and **would fire here**, which is right:
that is a parameter that decides AND brackets."* And
`crates/geom-core/src/real.rs:789`: *"still fires the gate"*. The
self-test plants four cases — `Decide + Bounds`, `Bounds + Decide`, the
`real.rs` definition lines, and the dual spelling — and not this one.

The compounding half is what makes it Tier 1. `real.rs:707` instructs
authors: *"Write `T: CertifiedBounds`, not `T: Bounds + CertifiedEnclosure`."*
Following the new guidance at any `Decide` site converts a gated
compound bound into an ungated one. Ten sites currently write the long
form and are gated (`geom-brep/src/ssi/certify.rs`, `ssi.rs:1187`,
`pcurve_cache.rs:970`, `edge_nurbs.rs:250`); `probe_tube_chart`
(`ssi/certify.rs:625`) is named in `real.rs` as exactly the shape the
rule targets.

This is **S56 returning**. S56 was "the compound-`Bounds` gate was
order-sensitive, so half the spellings it forbids were invisible to it",
FIXED by #676. The fix made the matcher order-insensitive and did not
make it alias-aware.

**Verdict:**

## S60. S26 was never fixed — the area enclosure is still unmetered, and its acceptance row is still the canonical monotone-wrong pair

**[verified]** **Two independent agents, different files.** The
`geom-brep/src` fix auditor and the new-tests auditor reached this from
opposite ends.

`area.width()` appears **nowhere** in `crates/geom-brep/src/props/quad.rs`
(grep count: 0). `mean_boundary_displacement` still reads `flux.width()`
only and still uses `(area.lo()+area.hi())/2` as a bare lever.
`QUAD2_AREA_PIECES = 64` (`:826`) is still a fixed pre-refinement
resolution with no round recomputing it. `area` is still commented as
*"a certified DENOMINATOR"* at `:1824`. quad.rs's 52 changed lines since
the base are the `clamped_to` poison-laundering repair and doc prose —
a different finding entirely.

The acceptance row is unchanged at
`crates/sweep/tests/m5_pr11_quad_props.rs:87`:

```rust
assert!(m.area_pad > 0.0, "the cut wall's area is a certified enclosure");
assert!(m.surface_area - m.area_pad <= exact && exact <= m.surface_area + m.area_pad, …);
```

Every degradation of the area enclosure makes **both** assertions
easier. The volume row twelve lines up (`:68`) *does* carry a tightness
ceiling (`volume_pad < 1e-3 * half_exact`), and that ceiling shape was
newly added post-scan at `crates/sweep/tests/m5_s11_concave_sense.rs:494`.
So the fix pass swept `volume_pad` and left `area_pad` — in the same
file the scan drew the shape from. The only other tightness-relevant
`area_pad` site, `crates/sweep/tests/m6_loft_body.rs:130`, asserts
`.is_finite()`.

**#472 deferred this. The tree matches the deferral, not the fix.** My
dispatch brief said it was fixed and asked which faces changed
disposition; that premise was wrong (§C15).

**Verdict:**

## S61. Every one of the 14 new gates is skipped on docs-tier and `local-scripts/`-only changes, and one gate offers that hole as its reason not to close another

**[verified]** `.github/workflows/ci.yml:234`: the whole `discipline`
job is `if: needs.filter.outputs.run_build == 'true'`.
`scripts/ci-filter.py`'s `_is_docs` (`:108-114`) returns true for any
`.md` path **and any `local-scripts/` path**; an all-docs change set is
`TIER=docs`, `RUN_BUILD=false`, `discipline` skipped.

Two gates exist to catch changes in exactly the classes that skip it:

- `scripts/gates/probe-suite-census.sh:52-57` asserts that
  `docs/K-REPORT.md` and `docs/SMELL-SCAN-2026-08.md` still name the
  cited CI step. A PR editing only those files is docs-tier, so the
  guard cannot fire on the only change class that can break it.
- `scripts/gates/gate-roster.sh:31-39` argues it need not read
  `local-scripts/` because *"a deletion touching only
  `local-scripts/ci-local.sh` classifies TIER=docs, so the `discipline`
  job never runs."* That is a description of the hole offered as the
  reason not to close it. Nothing verifies that
  `local-scripts/ci-local.sh:188-190` still loops the gate directory;
  a PR deleting the loop is invisible to hosted CI by construction.

`gate-roster.sh:13-22` names the `if:` hole honestly. Honest disclosure
of a hole that makes the disclosing guard inert is the C2 shape, one
level up.

Related, same file: `probe-suite-census.sh:56` makes
`docs/SMELL-SCAN-2026-08.md` — a dated historical scan — a **hard CI
dependency**, so archiving or reorganising it reds the build.

**Verdict:**

## S62. S13's dual-maintenance defect survives unmoved for the five checks outside `scripts/gates/`, and its prose has already drifted

`check-test-aggregation.sh`, `rundump-guard-selftest.sh`,
`check-interval-cfg-additive.py`, `demos/check_render_provenance.py` and
`demos/compose_uv_montage.py` are gates by every criterion the directory
uses, run in the same `discipline` job
(`ci.yml:305`, `:326`, `:338-339`, `:392`, `:402`), and are **named by
hand in both halves** (`local-scripts/ci-local.sh:196-209`, `:224`).
They are outside `scripts/gates/`, therefore outside the roster and
outside the local loop; nothing detects a lost row on either side.
Whether a check is a "gate" is itself an unenforced judgement call.

The prose has already drifted: `local-scripts/ci-local.sh:224` says the
hosted mirror is *"the `k-lint` job's 'demos render provenance' step"*,
but that step is `render provenance (demos)` in the **`discipline`** job
at `ci.yml:392`. That is S13's own defect, in the same file pair, after
the fix.

Adjacent: a gate that lands mode `0644` is invisible to **both** halves —
each derives the roster with `[ -x "$script" ] || continue`
(`gate-roster.sh:70-73`, `ci-local.sh:189`), so the executable bit *is*
the registration mechanism. A fixture with an unwired non-executable
`newgate.sh` beside a wired gate reports *"ci.yml wires a self-tested
step for all 1 gates"* and exits 0. The `-x` filter exists to exclude
`lib.sh` — a filename problem solved with a permission bit.

**Verdict:**

## S63. Three of the six grep gates pass the spellings they exist to forbid, and one has already produced the cry-wolf-then-allowlist outcome

Every claim below was **executed against a planted fixture** by the
scanning agent.

**`no-extra-real-bounds.sh:81`** is `grep -rnE '\bReal\s*\+'` — `Real`
*followed* by `+`, only. `pub fn f<T: PartialOrd + Real>(_t: T) {}`
passes green. `where T: Real, T: PartialOrd` passes green. And a line of
*prose* — `// never write T: Real + PartialOrd here` — fires it, because
this is the one grep gate that strips no comments. The header states the
purpose as *"`T: Real + PartialOrd` (or any other extra bound) is the
escape hatch"*; the hatch is open in both commonest alternative
spellings. Its self-test plants the one spelling the regex was written
for. **The ruling this needs is already adjudicated one file away** —
`bounds-allowlist.sh:106-107` plants both operand orders citing "the
matcher must see both forms rather than carve one out", and
`signed-zero-one-home.sh:16-24` cites the same ruling by name. It was
applied to the two gates that reported it and never swept to the sibling
in the same directory. (Cf. S59: the same ruling, un-swept again.)

**`bit-identity-debug-only.sh:116-122`** counts two things and
correlates neither: `uses=$(grep -c 'bit_identity::|eq_bits')`,
`gates=$(grep -c 'cfg(debug_assertions)')`, fail only if
`uses > 0 && gates == 0`. One `cfg(debug_assertions)` anywhere in the
file licenses any number of ungated production uses. A planted
`source.rs` with one gated `eq_bits` and one bare
`pub fn production_leak(..)` passes **and prints** *"topo/src/source.rs
gates its 2 bit-channel use(s) behind cfg(debug_assertions)"* — a
statement the gate has no evidence for. This matters more than it looks:
`bit-identity-consumer.sh:71` excludes `crates/topo/src/source.rs`
wholesale, so this gate is that file's only control.

**`interval-square-allowlist.sh:40-45`** cannot see `self.x * self.x`
and fires on `a * a.method()`. The live blind instance is
`crates/geom-core/src/linalg/vec.rs:326` — `s + (self.y * self.y) * a`
inside `impl<T: Real> Vec3<T>::orthonormal_basis`, production
generic-over-`Real` code in a file that is **not** on the allowlist.
S13's steelman named this exact site (then `:311`) as still standing;
#626 moved the gate without touching the regex. And the two failures
compound: `crates/geom-core/src/linalg/mat.rs` **is** on the allowlist,
and the header at `:26-28` justifies its entry partly with *"the test
hits are `r * r.transpose()` matrix products"* — a false positive
resolved by allowlisting the whole file, which is now unguarded for
genuine `x * x`. The cry-wolf-then-allowlist outcome, already realised.
The gate is now `grep -rPn` (PCRE), so the lookahead fix logged on
2026-08-04 is available and still unapplied. Next sites to sweep:
`crates/geom-brep/src/props/quad.rs:3193`, `crates/editor-core/src/mate.rs:218`.

**The shared cause.** Five of the six gates share a comment stripper
that is leading-`//`-only (`grep -vE ':[0-9]+:\s*(//|///|//!)'`), so a
trailing comment cries wolf and a block comment is invisible; only
`signed-zero-one-home.sh:73-103` has a real one. `scripts/gates/lib.sh`
is the home that does not have it. Each false red is a nudge toward the
allowlist rather than the fix, which the paragraph above shows already
happening.

**And `scripts/ci-filter.py` — 367 lines, deciding whether any gate runs
at all — is the only script here with no test.** Every gate under
`scripts/gates/` carries a `--selftest` both halves invoke;
`check-interval-cfg-additive.py` and `demos/check_render_provenance.py`
do too. The script that gates all of them has neither a self-test nor a
test file anywhere in the tree. It fails closed on exceptions
(`Bail` → `TIER=all`, `:352`), which covers the direction that matters
most — but the `docs` branch at `:184` is a **fail-open** path taken
before any of that, and it is the branch S61 depends on. `lib.sh:29`
says a guard that has never been shown to fire is not a guard; the
sentence was not applied one level up.

**Verdict:**

## S64. The mesh crate's headline D9 sentence is false, and there is a fourth ε consumer that decides emitted coordinates

**[verified]** `crates/mesh/src/lib.rs:49` still reads: *"ε is never
*read* for sizing — mesh structure is a function of (body, δ) alone
(D9)."* Twenty lines into `crates/mesh/src/curved.rs:72-93` the `Tol`
doc refutes the strong reading in detail: pole/apex identification is a
classification whose outcome *"substitutes the pole's exact `v`"* and
emits two polygon entries, *"so an ε that flipped that classification
WOULD move emitted coordinates"*; and `require_swept_rectangle` reads ε
to decide whether `tessellate` returns a mesh at all. `lib.rs:49` is now
the only place in the crate stating the false version, and it is the
sentence the memo-key contract and D9 are read through.

Worse, the enumeration that replaced it is already short by one.
`curved.rs:72` says ε *"reaches three places from here and no more"*.
`crates/mesh/src/walk.rs:852`'s `iso_side_starts` is a fourth:

```rust
!(same_kind && chart.radial(junction) > eps)
```

That decides whether a traversal opens an iso side or repeats its
predecessor's coordinate bitwise — i.e. **which `f64` the emitted UV
entry gets**, not merely whether something is refused. Commit `27ec8ea`
("precision: name all three eps consumers in mesh, not two") predates
`6881c366` (#653), which introduced the read, so the enumeration was
already stale when it merged. `walk.rs:534-554` asserts the opposite
explicitly — *"`iso_side_starts` does NOT read this predicate…"* — which
is true of `gap_is_noise` and false of ε.

This is S22's shape, one level up: the fix pass that counted the ε
consumers produced a count, not a mechanism, and the count went stale
inside two commits.

**Verdict:**

## S65. The #678 watertightness backstop is compiled out of every build that ships a mesh

**[verified]** `crates/mesh/src/curved.rs:306` — the re-derivation that
catches the #678 class is `#[cfg(debug_assertions)]`. The class #678
named is a *silently* non-watertight mesh returned as `Ok`. `tessellate`
does not run `check_mesh` (stated three times in this file), and
`rg check_mesh` finds no consumer outside `crates/mesh`, `stl`, `topo`
and `sweep` tests — no demo or tour row runs it either. So in a release
build the entire guard for the class is `pole_columns`' three-line
`if has_pole && nu == 2`.

The module header at `curved.rs:44-47` presents the floor and the assert
as a pair (*"Read the sentence above as conditional on both"*) without
saying that one of the two is absent from the builds that render.

Two narrowings compound it. The filter is
`poles.contains(&a) || poles.contains(&b)` (`:314`), but
`crates/mesh/src/trimmed.rs:456-468` names **two** sources of "one
repeated mesh id at two distinct UV locations" — chart singularities and
the full-2π seam double-traversal — and the seam case, held off by an
arithmetic argument (`nu >= 8` from the π/4 sagitta cap) rather than a
floor, is the half with no mechanical check, in the lane that actually
has seams. And the assert is per-patch, so cross-face identification is
out of scope too.

**Verdict:**

## S66. The cylinder box is widened by a full radius along its own axis, and S16's fix promoted that construction into a containment envelope

**[verified]** `crates/topo/src/boolean/boxes.rs:246-256`:

```rust
min_x: x0.min(x1) - radius,  min_y: … - radius,  min_z: … - radius,
max_x: x0.max(x1) + radius,  max_y: … + radius,  max_z: … + radius,
```

The slab arm pushes out by `radius` on **every** coordinate, including
the axial one where the boundary already bounds the extent exactly. A
radius-0.5 cylinder over `z ∈ [0,1]` gets `z ∈ [-0.5, 1.5]`.

Pre-fix this fed only the BVH, where loose is merely slow. The S16 fix
promoted the same construction into `crates/topo/src/census.rs:1538-1562`'s
arm-2 **containing** extent, where over-width turns into a false
`CensusUndecidable` — a probe sitting entirely below the cylinder now
has no definitely-negative margin and is refused as the interference
class. The one counter-row that could catch over-widening,
`crates/sweep/tests/s16_box_soundness.rs:133`
(`a_body_beside_the_cylinder_is_still_cleared_by_containment`),
separates at `cx = 3.0` in **x** — the axis where the widening does not
happen. Nothing separates in z.

The module doc argues at length that looseness is free (*"a bigger box
only admits candidates"*). That is true for the BVH and **false for two
of its three consumers**: `separation.rs` (looseness = refusal) and
`census.rs` arm 2 (looseness = false interference).

And no acceptance row in `boxes.rs` can go red for a box that is too
big. Every row in `boxes.rs:470-480` and `:623-636` asserts
`holds(&box, sample)` or `b.max_y >= r` — all monotone in the widening
direction, so a `face_box` returning `[-1e300, 1e300]` on every arm
passes the entire suite.
`the_boxs_reach_beyond_the_vertex_hull_is_the_whole_bulge` is a
one-sided `>=` written to prove the fix landed, which is exactly the Q3
shape.

**Same lines, second finding [verified]:** the slab arm's whole axial
projection reads *single* bracket endpoints —
`along(origin.x.lo(), axis.x.lo())`, and likewise y and z — so under an
`Interval` `T` the slab is built around one arbitrary endpoint of the
axis line. The conic arm at `:230-232` takes `.abs()` of
`reach(u_ref.x.hi(), v_ref.x.hi())`, and `hi().abs()` is not an upper
bound on `|x|` when the lower endpoint is larger in magnitude. The
module header states the contract as *"coordinates enter as
`[lo(), hi()]` brackets"* and *"every box this module returns contains
the entity's whole locus"*. `Interval` is a live scalar on this path
(`crates/topo/tests/m3_pr4_boolean.rs`), and the census twin at
`census.rs:1233` does it correctly in native `T` arithmetic — which is
what makes the divergence visible.

**Verdict:**

## S67. `face_normal.rs`'s one-door module names three flip sites: one of them does not flip, and at least five that do are unlisted

`crates/topo/src/face_normal.rs:26-31` says: *"Three such sites exist
and are NAMED (smell-scan D6: `solid_contain::face_plane`,
`chord_join::face_plane_normal`, `merge_faces.rs`)."*

`chord_join::face_plane_normal` (`crates/topo/src/chord_join.rs:2020-2026`)
returns the raw chart normal with **no `sense_sign` at all**, so it is
not one of them. Five sites that *do* multiply are unlisted:
`crates/topo/src/boolean/join.rs:986`, `boolean/rest.rs:512`,
`boolean/solid_contain.rs:316`, `validate.rs:2161`, `props.rs:264`.

The enumeration is the only thing standing in for the guard test's
admitted gap #1 (`face_normal.rs:88-92`), so an inaccurate enumeration
**is** the whole gap. It is also the disclosed-blind-spot-read-as-a-
discharge shape: the list is presented as the point of the paragraph.

Worth checking separately: whether `chord_join`'s *missing* flip is
itself a defect — it hands that normal to `point_in_loop` for ring
re-homing.

**Verdict:**

## S68. The W2c discard sweep stopped inside the function it was editing

**[verified]** W2c's claim, restated in `crates/topo/src/euler.rs:24`
(*"A mutation phase announces a failed lookup rather than discarding it,
at every write"*), in `review_m1_pr2/release_corruption.rs:33` and in
`ci.yml:740`, is scoped to `euler{,_ring,_kill}.rs` plus
`link_half_edges`.

`split_edge`'s mutation phase was edited in this same diff — two
`unreachable!` conversions landed at `crates/topo/src/split.rs:279` and
`:286` — and ten lines below them sit three silent `if let Some(..)`
discards at `:287`, `:294`, `:299`. `movefac` — named in
`docs/DESIGN.md:1117` as one of the four structural mutators the closure
property rests on — carries three more (`movefac.rs:150,162,166`), and
`attach.rs:81,252` two.

The sweep's own scope sentence is what made the sibling instances
invisible. And `split_edge` is not a sibling — it is the same call.

**Verdict:**

## S69. `kfmrh`'s shell-fusion form is outside the fuzz catalog, and the `Ledger` counts solids, so it cannot notice

S15's finding was that `split_edge` shared `mev`'s Euler vector while
sitting outside the fuzz catalog; the fix added `SplitEdge`. Meanwhile
`kfmrh` gained a cross-shell **shell fusion** form
(`crates/topo/src/euler_ring.rs:809` — re-homes faces, kills a shell,
`ArenaDelta { shells: -1 }`), and both `kfmrh_candidates`
(`seqgen.rs:502`) and teardown's re-make search (`:1103`) filter on
`face1.shell == face2.shell`, so the fusion branch is **never
generated**.

The `Ledger` cannot notice either: its `s` is the *solids* count
(`seqgen.rs:283`), not shells, so a shell-count error passes property
(b) silently.

Related, same operator: `euler_ring.rs:846-862` hands
`assert_euler_postcondition` an `ArenaDelta` computed as
`if killed_shell.is_some() { … shells: -1 … } else { … }`, where
`killed_shell` is produced by the same `cross_shell` branch that did the
mutation. The postcondition therefore follows the code down whichever
path it took and can no longer detect the fusion branch running when it
should not have. Every other operator hands it a constant, and
`euler.rs:869` describes `ArenaDelta` as *"One operator's signed
shift"*, named at the site *"so a site reads as the op's actual shift"*.

**Verdict:**

## S70. `DESIGN.md`'s ratified graft footnote is documented-as-false in a source comment, and "whoever takes S14" is the schedule

`crates/topo/src/euler.rs:84-88` says of the graft's failure state:
*"**All three understate it.** A refusal raised between the transplant's
two passes leaves entities holding source-internal keys, which in `dst`
either dangle or resolve to an unrelated live entity. Whoever takes S14
fixes one of three copies of the same sentence."*

So a ratified design document (`docs/DESIGN.md:1131-1140`) is documented
as false in a source comment, and the other two copies
(`review_m1_pr5_internal.rs:288-296` and DESIGN.md itself) were left
carrying the weaker `SolidWithoutShells` claim on purpose. That is Q4's
second sub-case (code known to be worse than the sentence) plus Q6:
**S14 is recorded in the first scan as "a decision, not work … no
channel at all"**, so "whoever takes S14" points at nothing.

The door itself is still only documented, not prevented:
`graft_disjoint_all_keyed` mints destination solids before transplanting
and remaps as it goes, with no atomicity added anywhere in the diff.

**Verdict:**

## S71. The ONARC shim deletion dropped the enclosing (ρ < 0) fillet class, and the file still argues at length that it built it

**[verified]** `crates/profile/tests/review_s2.rs`'s module header
argues over forty lines that *"the class is built"* and points at
`[enclosing_tangency_is_constructed_not_stumbled_upon]`, which runs
*"the SAME oracle battery the sweep runs"* so *"a fixture cannot quietly
assert less than the fuzz did"*.

**That function does not exist anywhere in the tree.** Two references
point at nothing (`:45` as an intra-doc link, `:735` as a comment). What
shipped is `the_lattice_door_never_emits_an_enclosing_tangency`
(`:942`), which pins the **opposite** — every table row REFUSES — and
the fuzz's own comment at `:806-813` says the class is now *"structurally
0"* through the surviving door.

So the answer to "was the gap closed or the capability dropped" is: arc
× arc with differing far points is genuinely authorable on the lattice
(`build_corner`, `sugar.rs:389-415`), but the r > R enclosing class the
v1 builder could author is **gone**; the `sugar.rs:720-1010` machinery
that computes those candidates is unreachable from any shipped door; and
the only note of it is a parenthetical at `:938-941` calling it *"a
design question"* with no issue number and no named unit.

The dangling intra-doc link is invisible to `scripts/doc-gate.sh`
because it is in a `tests/` file — see S63's note on what the doc gate
covers.

**Verdict:**

## S72. `interval-transcendentals`: nothing constrains the pads from above, and the cheap tier catches a dropped outward round for division only

Both halves were **executed** — the agent copied the crate to a scratch
dir and ran the mutations.

**The pads.** `PAD_ULPS = 64` (16× the derived value, `src/trig.rs:24`)
passes the entire default-tier suite green. The oracle tier cannot catch
it either: `tests/common/mod.rs:159-170`'s `assert_contains` asserts only
`mine.lo() <= iv.inf() && iv.sup() <= mine.hi()`, which gets *easier* as
the enclosure degrades. The only counterweight, `Tightness::report`,
`println!`s and never asserts. `src/lib.rs:45` calls this *"Tightness
(documented, measured in the harness)"* — the number is measured,
printed, and never guarded, so the crate's headline tightness contract
is one careless "let's pad a bit more to be safe" away from silently
16×-ing every enclosure in the kernel with all lanes green.

**The cheap tier.** Three more scratch probes: `PAD_ULPS = 0` (every
transcendental pad gone) — green; `add_lo`/`add_hi`/`mul_lo`/`mul_hi`
reduced to bare round-to-nearest, i.e. an *unsound* interval arithmetic
— green; `sqrt_lo`/`sqrt_hi` pads removed — green. Only the division pad
goes red. `.github/workflows/ci.yml:1097-1099` says of this row *"that
tripwire is what catches a dropped outward round"*; `README.md:34` says
*"a dropped pad is caught by the same pipeline that gates the kernel."*
Both are true for `÷` and false for `+ − × sqrt sin cos tan asin acos
atan atan2`. `oracle-certify` does fire on `src/` changes, so CI overall
is not blind — but the cheap row's stated purpose, and the README's
claim about it, overreach by nine operations.

**The bugfix protected the site that did not have the bug.** The
containment violation that motivated the magnitude gate was found in
`mul_exact` (`src/round.rs:131-133`, `docs/derivations.md:144-151`); the
response was a dedicated exact-rational fuzz for **division**. `mul` and
`sqrt` got no equivalent.

**Bottom line the agent volunteered, and it matters for S1:** the pad
*derivations* are real. `docs/derivations.md` §1 P1/P2/P3 are genuine
proofs, correctly applied per function, and the one non-derived
ingredient (Assumption A) is stated plainly rather than smuggled. No
unsoundness was found in `round.rs`'s witnesses, `Div`'s pole case
split, or `atan2`'s three-case table. The finding is about what the
tests can see, not about the mathematics.

**Verdict:**

## S73. The tessellation instruments resolve every broken measurement in the cannot-fire direction

Three findings on `tools/`, which is where the project's
measure-don't-guess rule (`memories/tessellation-budget.md`) is
implemented.

**`ratio` answers `1.0` on every broken input.**
`tools/tess-lint/src/lib.rs:162-172` returns `1.0` whenever the
denominator is non-positive or the numerator is non-finite, and the
comment names this as a virtue: *"it is the reading that cannot
manufacture a finding"*. For a gate that fires only on **growth**, `1.0`
is the smallest possible slack — so a fresh row whose `span_opt_cells`
collapsed to zero, or whose `grid_cells` came through as `NaN`, reads as
a face that got dramatically better and is guaranteed to pass. `NaN` is
not hypothetical here: `worst_dev` is legitimately `NaN` on every CI row
(`--sizing-only`), so it is a value the parser is built to accept
(`lib.rs:255`). **The instrument's failure mode and its pass condition
are the same value.**

**The slack rule joins on a face ordinal that any geometry change
re-keys.** `tools/tess-lint/src/lib.rs:451-460` keys per-face slack on
`(scene, face_ordinal)`, the positional index into `mesh.patches`. Any
change adding or removing a face renumbers every face after it, so
surviving faces are compared against a *different* face's baseline row —
a mis-join, not a measurement, in either direction. The `else { continue }`
branch's comment (*"the scene's absence is already a Vanished
finding"*) is false whenever the scene is present and only the ordinal
moved; `Vanished` is scene-granular. The same path silently swallows a
NURBS face that reroutes to a non-NURBS lane, which is the *"silent
coverage loss reads as an improvement"* case `lib.rs:79-81` calls a
finding rather than a footnote. No test covers scene-present-face-missing.

**`GROWTH_TOLERANCE` can be loosened ~18× green.**
`tools/tess-lint/src/lib.rs:377` is 5%, the tessellation gate's only
threshold. The tests box it only into roughly `[1.039, 1.962)`, so
someone facing a red gate can set it to `1.9` and every test still
passes. Contrast `k-lint`, where `BASELINE_FLOOR_MARGIN` is pinned into
`(3.9e-5, 4.7965e-5]` by two tests and additionally floored by the `#99`
litmus — that is what a boxed constant looks like. Plausibly a class:
`SPLIT_SCAN_DECADES` / `SPLIT_SCAN_STEPS`
(`tools/tess-meter/src/lib.rs:498-500`) are pinned by nothing at all.

**Verdict:**

## S74. `swept.rs`'s new shared home states a reason for not unifying that is factually false, and the same commit deleted the duplication markers

**[verified]** `crates/sweep/src/swept.rs:17` says the traversal builder
stays out because *"extrude's has a reversal arm and mints the
orientation bit, revolve's has neither."*

`crates/sweep/src/revolve/mod.rs:625`'s `swept_segments` takes
`reverse: bool` and its own doc says it implements the full involution
(*"endpoints swapped, bulge negated, turn flipped"*). Half the stated
reason is simply untrue. The two bodies compute `a`/`b`/`bulge`/`kind`/
`canonical_vertex`/`canonical_segment` identically; the only real
divergence is extrude's extra `wall_sense` field — which is exactly what
`SweptChord` was introduced to fence off elsewhere.

Worse, the same commit **deleted the two self-declared duplication
markers at the copy site** (`revolve/mod.rs`'s *"Mirror of `extrude`'s
`SweptSeg`"* and *"Mirror of `extrude`'s `swept_segments`"*) while
leaving the copies in place. The only greppable evidence that these are
twins is now gone, replaced by a sentence asserting they are not.

This is C11's mechanism running in reverse. C11 observed that every
duplication in this codebase is self-declared in prose and nothing ever
reads that prose. Here the prose was the only record, and a
consolidation pass removed it.

**Class, not instance:** every *"deliberately NOT unified"* claim in the
S6 record should be re-checked against the code it describes —
`strut_spec` and `SweptSeg` at minimum. And `swept.rs:31`'s
funnel-bypass count is wrong on its first day: *"loft and three other
sites"* reads as four; there are three (`loft.rs:321`,
`extrude.rs:886`, `revolve/tube.rs:29`), which is exactly the
three-member class S6's residue (e) recorded.

**Verdict:**

## S75. The recourse-contract guard's central assertion is a tautology, and two suites defer their completeness obligation to it

`crates/sweep/src/fillet/mod.rs:696` — `contract()` returns
`(err.clone(), Recourse::…)` on **every** arm, so the witness it hands
back is definitionally the same variant as the seed, and

```rust
assert_eq!(discriminant(&witness), discriminant(&seed),
           "the table's arm must witness its OWN variant")
```

cannot fail. The doc at `:684` claims *"Each arm carries BOTH halves —
the decision and a witness value of its own variant — so the two cannot
drift apart by omission, which is exactly how a hand-kept witness list
lets a wrong decision ship green."* That mechanism is described and not
implemented; what actually constrains the test is `seeds()`, which **is**
a hand-kept list, seeding 8 of ~21 variants.

Two other rows defer completeness to it:
`crates/sweep/tests/m5_pr12_refusals.rs:509` (*"the standing check on
completeness is `fillet::recourse_tests`' exhaustive match"*) and
`crates/sweep/tests/review_d2_recourse_at_the_site.rs:6`. So the whole
recourse-correctness story rests on a guard whose central assertion is
vacuous.

And the row those defer *to* is itself weaker than its name:
`m5_pr12_refusals.rs:514`'s
`every_recourse_sentence_is_reachable_from_both_arms` asserts only
`!s.is_empty()` and `!s.ends_with('.')` over a list of constants —
nothing renders a `FilletError`, nothing exercises an arm, nothing
checks reachability. Its doc says *"this row is what would catch a
future edit that inlines one of them"*; inlining a constant's text at a
`Display` site would leave both the constant and this row green. **This
fix pass touched the row**, adding four new constants, without noticing
it cannot go red for the reason its name gives.

**Verdict:**

---

# Tier 2 — significant

The four rows below were flagged **high** by their reporting agents and
are placed here only because Tier 1 was already full at the point they
landed; read them as the tail of Tier 1.

## S76. The spent-graft hammer row is missing the anti-vacuity assertion its twin has, and CI cites it by name

`crates/topo/src/review_d18.rs`'s `Tally` doc (`:448-453`) says `oks`
*"is the anti-vacuity measure that matters: … A sweep whose calls all
died in a plan phase proves nothing about the arms under attack."*

`torn_bodies_never_reach_a_row_four_unreachable` (`:586-591`) asserts
both `calls > 1_000` **and** `oks > 0`.
`a_spent_graft_destination_never_reaches_a_row_four_unreachable`
(`:626-629`) asserts only `calls > 100`. A spent graft destination is
far more structurally damaged than a randomly torn cube, so it is the
row *more* likely to have every call refuse in the plan phase — and it
would then pass green while exercising no row-4 arm at all.
`.github/workflows/ci.yml:811` greps for this row by name as one of the
two things justifying the release-profile job.

**Verdict:**

## S77. The rimless-sphere exemption is a newly written claim that nothing enforces

`crates/geom-brep/src/props/mod.rs:96-101` lists the rimless sphere band
as one of the iso-rectangle predicate's two exemptions because *"its
whole-latitude-band domain is a rectangle by construction"*.

The branch (`props/curved.rs:937-955`) checks only that the meridian
axes are coplanar, then hardcodes `du = π` and takes `(lo, hi)` from
`min_max` over meridian *endpoint* latitudes. It never establishes that
the meridians run pole to pole. `curved_face` is a public key-free door
that the S58 suite itself drives with hand-built loops and that
`topo::mass_properties` drives on imported STEP faces.

This is structurally #723's shape: an unstated extent premise asserted
as established, one `else`-branch away from the arm #714's own review
broke. The PR's answer to that class was to write the gap down; here it
wrote the **opposite** down, in the same paragraph, and gave the exempt
arm no row.

**Verdict:**

## S78. A fuzz corpus that can silently shrink to a bare cube, with nothing asserting what it contains

`crates/sweep/tests/review_d2_adv_probes.rs:131-201` builds every
interesting body behind `if let Ok(…)` / `if let Some(…)` — `subtract`
and `union` swallow refusals via `.ok()?` at `:109`/`:121`, the rotated
cube at `:138`, the graft at `:191`. Nothing asserts the corpus length
or that any named body is present.

The file's own docs say the rim requests exist because *"without these
the sample never reaches `rim_phase`, which holds 6 of the 18
`unreachable!` sites"* — and that push is conditional too (`:248`). So
if `boolean_op_with` or `revolve` ever starts refusing these fixtures,
the sweep degrades to a bare cube and stays green while covering none of
what it was written for.

Same file, same shape:
`d2_the_battery_never_hands_the_surgery_an_empty_chain` (`:392`, `:408`)
does `let Ok(v) = run_battery(&req, band()) else { continue; }`,
increments `verdicts`, prints it, and never asserts it is greater than
zero.

The discipline exists elsewhere in the tree:
`crates/geom-core/tests/d8_knot_queries_adversarial.rs:601-635` asserts
a case-count floor *and* a per-regime floor. The named-loud-skip idiom
(`interval_lane_skipped_no_certified_coverage_here`) exists too. Neither
is used here.

**Verdict:**

## S79. The three demo-surfaced API gaps — FILED as #757, #758, #759

The `demos/` scan is the best available evidence of what the public API
is like to use, per `memories/demo-purpose.md` (demos demonstrate real
natural usage; awkwardness is a library finding). Its three highest rows
are already issues:

- **#757** — `BooleanDeclarations` has no geometric producer. To call
  `union_with`/`intersect_with` a caller iterates faces, matches
  `Surface::Plane`, crosses and dots normals, computes plane offsets and
  runs three `k_stats::decide_flagged`/`decide` calls against a raw
  `Band::linear()` — ~55 lines, duplicated character-for-character at
  `demos/tour/src/booleans.rs:67-122` and
  `crates/topo/tests/common/mod.rs:446-498`, with the twinning declared
  in prose at both ends. (`editor_core::eval::wire::resolve_declarations`
  produces declarations from *authored names*, not from two bodies'
  geometry.)
- **#758** — no public census/genus query, so the Euler–Poincaré
  identity is hand-written ~13 times in four different return-tuple
  shapes, including byte-identically in both demo crates
  (`demos/tour/src/main.rs:186-194`, `demos/wild/src/main.rs:175-183`).
  `genus` exists only in `crates/topo/src/review_m1_pr3.rs` and
  `review_m1_pr4.rs`, both `#[cfg(test)]`.
- **#759** — the `pncad` façade's polygon door was demoted
  (`crates/pncad/src/authoring.rs:115-127`, *"a reasonable future door,
  fenced out of this unit"*, no issue number) with no replacement
  scheduled; 11 demo call sites route around it through a demo-hosted
  fold, whose own doc still says it *"Mirrors
  `pncad::authoring::polygon`'s `(f64, f64)` slice signature"* — a
  function that no longer exists. Polygon doors *do* exist at
  `crates/profile/src/lib.rs:266`, `crates/editor-core/src/program.rs:1213`
  and `crates/pncad-py/src/py/doc.rs:594`; the façade one is what is
  missing.

**Verdict:**

---

## S80. `boundary_material_sign` is the sibling the iso-rectangle invariant was not swept into

`crates/geom-brep/src/props/mod.rs:100` exempts this function on the
argument that *"running the predicate there could only convert an answer
into an exemption"* — which covers the **error** direction only.
`props/curved.rs:127,146-152,158-164` re-runs the same boundary parse and
hands `rims.first()` to `s_f_from_rim`, whose `lo + hi − 2v` test is
meaningful only on a domain that is actually a rectangle. On a
plus/staircase domain whose loop happens to begin at an interior rim it
returns a definite ±1 that is not the material side.
`crates/topo/src/validate.rs:2285` turns that into `CurvedSenseInverted`,
and because tier-3 check 7 is gated on `errors.is_empty()`, the wrong
diagnosis **suppresses** the honest `NotIsoRectangle`.

**Verdict:**

## S81. Two spellings of "these two rim levels coincide", 90 lines apart, reconciled by a comment

`crates/geom-brep/src/props/curved.rs:350` (`same_level`, predicate
`props_rim_level_group`) and `:455` (`require_rims_at_extremes`,
predicate `props_rim_level`) both decide whether two `RimLevel`s are the
same level, and disagree on three axes: the metric (per-component
`Δsin`/`Δcos` levered vs the Euclidean chord `√(Δs²+Δc²)` levered), the
lever arm on the torus (`major` vs `minor`, ~4× apart, on consecutive
lines of `torus()`), and the fail direction (`Ok(false)` vs typed
refusal). The predicate names differ by one suffix. The fix pass's
response was a 20-line paragraph in `du_of_rims` (`:508-527`) explaining
that the two sites are the same rule metered differently — which per Q2
is the evidence the rule needs one home, not the defence of having two.

**Verdict:**

## S82. N7 now governs a refusal in the accepting direction, unscheduled and unrowed

Carrying the rim rule to the sphere carried `RimLevel::Unit(sin v, 0)`
with it (`props/curved.rs:963-972`, `:472-481`), so the predicate's
margin is `R·|Δ sin v|` — the axial separation, which collapses as
`cos v̄ → 0`. Two genuinely distinct near-polar rims therefore decide
`Zero` and the predicate **passes** a non-rectangular domain.

`docs/predicate-dimension-audit.md:171,550` and
`crates/geom-brep/tests/rim_dim_scale_twins.rs:369` both say this in
prose (*"here it is a REFUSAL that is affected"*, *"the lever
UNDERSTATES toward the poles, in the ACCEPTING direction"*) and both
file it as "typed-margin conversation input"; the audit table still
marks the row `OK`. No issue number, no named plan unit, and no row
exercising a near-polar interior rim. It is also the answer to "does the
new predicate have the same premise gap anywhere else" — yes, on the
same kind #723 is open on, by a second mechanism.

**Verdict:**

## S83. `seam_tol` / `MarchTolMismatch` cannot be reached and has no row; `MarchTol` is public with no possible caller

Both finishers call `seam_tol(ctx.tol, band)` where `ctx.tol` was built
as `MarchTol::from_band(band)` from the same `band` a few dozen lines
above (`crates/geom-brep/src/ssi.rs:818-826,837,1028`), and
`MarchTol::decoupled` is `pub(super)`, reachable only from the
certificate-free door. So the refusal is unreachable by construction;
nothing in the workspace names `MarchTolMismatch`; and the MARCH-TOL
acceptance row (`tests/m5_pr7_ssi.rs:309`) asserts an identity
`seam_tol` has already forced.

Note the contrast **inside the same batch**: `du_of_rims`' new doc
(`props/curved.rs:531-543`) worries at length that an unreachable
`require_zero` is *"a value computation wearing a typed-refusal
costume"*. The SSI seam is that shape and got no such treatment.

Separately, `MarchTol` (`ssi/march.rs:133,145,176`) is `pub` and
re-exported, its doc saying *"Outside this crate the only constructor is
`MarchTol::from_band`"* — but no public item takes or returns one.
`MarchContext` became `pub(crate)` in the same diff; `march` and
`newton_refine` are `pub(crate)`; `SsiBranch::march_tol` is a bare
`f64`. A `pub` type for an external caller who cannot exist.

**Verdict:**

## S84. The S23 floor row still passes on a skip — the lesson S25's own postmortem drew

`crates/geom-brep/tests/m5_pr7_ssi.rs:651-672`'s
`the_floor_clamped_planted_fixture_refuses_typed` matches
`FitSampleBudget` into a `println!("SKIPPED …")` arm and returns green,
so at the finest battery ε the row asserts neither the floor refusal nor
its text. The renaming work correctly removed the false premise from the
row's *name* and left a second premise in its *body*.

S25's postmortem, **in this same batch**, names *"a skip reads as a
pass"* as its most transferable finding. Likely a class rather than an
instance: the other tolerant `SsiError::…{ .. } => {}` arms at
`m5_pr7_ssi.rs:1608`, `review_m5_pr7_adversarial.rs:161` and
`review_m5_pr7b_ssi.rs:349` all accept a second variant without
recording that the row stood down.

**Verdict:**

## S85. The `Bounds` trait's headline still calls it the certification door, and its ledger grew 50% under the fix meant to retarget it

`crates/geom-core/src/real.rs:350`'s first line still reads *"Bound
extraction for **certification and driver code**"*, unchanged since the
base. The same PR corrected precisely this wording on `Enclosure`
(`:625`: *"Not 'certification helpers', which is what this said before
#643 — and the word matters more since D1"*) and on
`CertifiedEnclosure`'s implementor list. The trait D1 explicitly demoted
out of the certification role kept the sentence — the sweep fixed the
siblings and left the anchor. Also worth reading: `interval.rs`'s
`Bounds` impl doc and `Bounds::lo`/`hi`'s own method docs.

Meanwhile the doc block went from 156 lines to **234**. Three entries
were edited in place *and* had a paragraph appended explaining what the
edited sentence used to say (`:385`, `:414`, `:441`), so the file now
carries both the corrected text and prose about the text it replaced. A
234-line doc block on a two-method trait is past the point where a
reader finds the rule. (C5, still running.)

**Verdict:**

## S86. `CertifiedEnclosure for RingInterval` returns the one thing the trait doc forbids

The trait's method doc (`real.rs:741-750`) says `None` *"is a refusal
rather than a NaN bracket on purpose — NaN would be indistinguishable
from arithmetic poison and would travel silently through `f64`
combinators (`f64::max` returns the non-NaN operand)"*.

`crates/geom-core/src/ring_interval.rs:293-300`'s `certified_bracket`
returns `Some((self.lo, self.hi))` unconditionally, so a poisoned ring
hands back `Some((NaN, NaN))` — exactly the laundering the contract
excludes. The impl's doc leans on `from_bounds` rejecting it
"downstream", which is true of `from_certified` and of
`ssi/enclose.rs:211`'s `pad_interval` by accident, but is not what a
generic `T: CertifiedEnclosure` consumer is told it can rely on.

**Verdict:**

## S87. A fifth lane trait exists, blanket-implemented, and D1 never looked at it

`crates/profile/src/path/arc_fillet.rs:593-595`:

```rust
pub trait ArcCarrierScalar: Decide + Bounds {}
impl<T: Decide + Bounds> ArcCarrierScalar for T {}
```

Structurally the fifth member of the lane-trait family S3 counted at
four — same supertrait bundle, same `Decide + Bounds` content — with the
polarity **inverted**: it admits by blanket impl instead of refusing per
scalar. Before D1 the missing `Bounds for Dual` kept `Dual` out; now
`Dual64: ArcCarrierScalar` holds automatically, and with it the whole
`path::family` arc surface (`open_arc`, every `ArrivalSpec` /
`TangentIncoming` / `PointIncoming` impl), re-exported from
`profile/src/lib.rs:133` and `pncad::profile:57`. Neither
`arc_fillet.rs` nor `family.rs` contains the string `Dual`, so nothing
at the code site records the change — and any re-derivation of S3 that
enumerates "the four lane traits" will miss it.

**Verdict:**

## S88. D1 newly opened sole-`T: Bounds` doors that the sweep's own pattern cannot see

The D1 sweep's declared pattern is *"a line mentioning `dual` within a
±4-line window of [a phrase list]"*, and its declared blind spot is a
claim that states the premise without those words. That does not cover
the larger set: sole-bound `T: Bounds` doors that never mention duals at
all and that D1 nonetheless opened.

`geom`'s public point-projection doors are the clearest —
`project`/`project_seed`/`project_from_seed` are `T: Bounds` on
`impl<T: Bounds> $Curve<T>` / `NurbsSurface<T>`
(`crates/geom/src/projection.rs:111`, `curves/projection.rs:129`,
`surfaces/projection.rs:146`), and a `Dual64` now instantiates the whole
Newton lane through `mid()`. A word-boundary grep for `Dual` across
those files plus `curves/boxes.rs` and `bvh/src/aabb.rs` returns one
hit, and it is the word "dual" meaning "counterpart" in `pmin`'s doc.
Treat the admits-table's six seams as a sample, not an enumeration; next
sites are `geom-brep/src/ssi.rs:1187`'s `TubeScale<T: Bounds>` and
`pcurve_cache::rational_arc_chain`, plus
`crates/profile/src/fillet_select.rs:98`.

**Verdict:**

## S89. The one-home fix for the ring crossing minted three local aliases and a hand-counted tally

`RingInterval::from_certified` is the declared one home, and three
private one-line wrappers now sit on top of it — `bracket`
(`ring_interval.rs:160`), `ring` (`geom-brep/src/ssi/enclose.rs:195`),
`br` (`topo/src/props.rs:494`) — each carrying its own multi-paragraph
restatement of the same rule, two of them sharing a verbatim sentence
and the third restating it differently. The door's doc also carries a
**prose census of its callers** (*"five call it directly, and the rest
go through…"*) which nothing enforces and which already needed a
correcting commit (`88616177`, "S41: correct `from_certified`'s own
count of its call sites").

A unit that unifies duplicates minting three named copies plus a
hand-maintained count is the fix reproducing what it closed.

Related: the suite named after the split,
`crates/geom-core/tests/decoration_seam.rs:21`, claims its rows pin
*"that the three C9-ring crossings follow the second door"*; every
executable row reaches the ring through one crossing,
`hull::domain_hull` via `hull_bound` (`:139`). **Two independent agents
hit this** (the geom-core auditor and the new-tests auditor). The
new-tests agent adds that `trv()`/`healthy()` and the whole
`the_fixture_is_a_finite_bracket_that_cannot_certify` row are restated
verbatim in three test files, two of which share a filename in sibling
directories and run in the same `geom` binary.

**Verdict:**

## S90. The largest D1 residue is the only one without a schedule

`crates/geom-core/src/real.rs:470-477`: *"What is owed is a lane, or a
written reason it needs none, and it is owed on the **public**
surface"* — recorded as prose, pointed at from
`scripts/gates/bounds-allowlist.sh:27-31`, with no issue number and no
plan unit. The smaller residues from the same ruling all got numbers
(`ContentBits for Dual` → #687, the census box duplication → #700, the
`Enclosure` gate gap → #701); the one seam the ruling actually left
unguarded got neither. `real.rs:394`'s *"#643-completeness question …
deliberately left open here"* is in the same position.

The first scan's own closing rule for the retired unscheduled table says
a finding *"leaves a verdict and no row only if the verdict is
closed"*. This one is decided-and-open.

**Verdict:**

## S91. A new differential test that cannot go red

`crates/geom-core/src/spline/knots.rs:751`'s
`find_span_in_is_find_span_on_the_same_knots` asserts
`find_span_in(&knots, p, t) == k.find_span(t)`. After the D8
consolidation `find_span` is `span_at(t).index()` (`:343`), `span_at` is
`span_offset(t) + degree` (`:417-418`), `span_offset` is
`span_offset_in(&self.knots, self.degree, t)` (`:357`), and
`find_span_in` is `span_offset_in(knots, degree, t) + degree` (`:645`).
The two sides are the same expression over the same inputs, so no probe
— including the NaN and out-of-domain rows the doc singles out as
*"where 'the same search' is the entire content of the claim"* — can
separate them. The doc was written against the pre-consolidation world.

**Verdict:**

## S92. Two parallel scraped-source registries of "what is a public mutation door", both classifying by string match

`crates/topo/src/review_m1_pr5_internal.rs:254` and
`crates/topo/src/pcurves.rs:1507` both walk `fixtures::crate_sources()`,
both filter with the identical copy-pasted predicate
`params.contains("&mut self") || params.contains("&mut Body")`, and each
then maintains its own hand-written table of ~20 door names with
per-door prose. The shared concept has no home; it is duplicated inside
two consumers.

Both also decide a door's classification by
`body.contains("<literal>")` (`"assert_euler_postcondition"` /
`"mint_pcurves("`), so a door can be classified compliant by a *comment*
in its body mentioning the string, and neither guard would go red. Each
discloses one blind spot (delegation; `topo/src` only); neither
discloses the string-match one.

**Verdict:**

## S93. The S15 fix minted two new prose-held caller obligations

`crates/topo/src/euler.rs:1048-1059` (`mev`'s fan site) and
`crates/topo/src/euler_kill.rs:522-535` (`kev`'s fan merge) each now
carry a paragraph stating that re-based edges keep carriers certified
against their *old* endpoint, instructing the caller: *"**Re-describe
the moved run** (via `set_edge_curve`) whenever the two points differ."*
Nothing enforces it — tier 1 does not constrain it, no operator
re-checks it, and the paragraph says so.

That is the exact "the caller must / kept in step by hand" shape S15 was
raised to retire, minted by S15's own fix pass. `seqgen.rs:562-575`
already works around the resulting state (*"an edge's stored curve is
routinely stale against its own endpoints"*) with a re-certification
filter that admits it costs coverage.

**Verdict:**

## S94. Two hand-maintained `VARIANTS` ladders, with the disclosure copied verbatim

`crates/topo/src/euler.rs:3220-3251` and
`crates/topo/src/validate.rs:4079-4140` both carry a
`const VARIANTS: usize` and a wildcard-free `variant_index` match
restating the enum's declaration order by hand, and both carry the same
four-sentence *"what it does NOT enforce … when you add an arm, its
index is the new `VARIANTS - 1`"* paragraph word for word — a hand-kept
mirror of a compiler-known fact, self-declared as such at both copy
sites. The disclosure names `strum`'s `EnumCount` as the way out and
then declines it, which owes a schedule and has none. Worth checking
whether `MassPropsError`, `PcurveMintError` and `BooleanError` grew the
same ladder or lack it.

**Verdict:**

## S95. Two operand gates with different admitted kind sets, and a doc that now describes only one

`boolean_op_with`'s doc was rewritten to *"Surface kinds are gated per
arm, not wholesale — see `reduce::gate_operand_kinds`"*
(`crates/topo/src/boolean/ops.rs:251-252`), but fifteen lines down the
function still runs its own wholesale scan for Subtract and Intersect
(`:388-402`) that admits only `Plane | Cylinder | Sphere` and refuses
`Nurbs` with a **different** error (`CurvedOpUnsupported` vs
`CurvedBooleanUnsupported`). So for two of the three ops the doc's claim
is false, and the crate carries two spellings of "which surface kinds
may be an operand" that have already drifted by one variant.
`gate_planar` was renamed to `gate_operand_kinds` *"for what it admits"*
in the same churn window, which makes the surviving inline copy easier
to miss.

**Verdict:**

## S96. The shared `chord_join` core still imports from one of its two consumers

`crates/topo/src/chord_join.rs`'s placement argument is that it is *"a
**top-level sibling** of `boolean/` and `splitting/`, like
`crate::sector_shape` and `crate::sector_face`, so neither half hosts
the other's core."* It then imports `crate::splitting::SplitPlane`,
`crate::splitting::containment::{…}` and
`crate::splitting::rules::face_extent` (`:62-67`, `:90-92`), and its
error docs point back at `crate::splitting::split` and
`crate::splitting::plane_section`. The two modules it cites as precedent
import nothing from either lane, so the analogy is doing work the
dependency graph does not support. S5's shape, one indirection later.

**Verdict:**

## S97. S16 unified two of three box constructions; the third's stated reason is retracted at the copy site, and `boxes.rs` cites the retraction as live

`separation.rs` genuinely collapsed onto `face_box`. `census.rs` shares
only the *rule* enum; the min/max, the pad handling and the NURBS hull
are re-derived, and the comment at the copy site
(`crates/topo/src/census.rs:1185-1206`) says in as many words that the
justification *"has LAPSED, and its replacement is weaker"*, with #700
filed. `crates/topo/src/boolean/boxes.rs:15-18` then forwards the reader
to that comment as though it carried a live justification: *"that
module's docs carry why there are two arithmetics and only one rule."* A
pointer to a retraction, read as a citation.

**Verdict:**

## S98. `K-REPORT.md`'s dated M3 crop was back-filled against its own twice-stated rule, and its arithmetic no longer closes

`bool_join_chord` (minted by #719) and `point_in_loop_segment` (minted
by #712) were inserted into the *"M3 addendum (snapshot, 2026-07-23)"*
crop (`docs/K-REPORT.md:307,312-322`), with its bullet counts bumped
24→25 and 4→5. The same document rules twice that this must not happen —
*"back-filling it would make it describe something it never
described"*, and *"The M3 addendum's inventory … is likewise left as
written: it is a dated 2026-07-23 record."* The header still reads
*"added 59 predicate names"* while the bullets now sum to 61, and the
per-family table at `:848` still reads `point_in_loop_* | 4`.

For this section, the K data no longer means what it did after the name
churn, and the internal inconsistency is the evidence.

**Verdict:**

## S99. `net::is_placeholder` tests one channel while the crate doc promises all of them

The hoisted predicate (`crates/geom/src/net.rs:142`) is
`control.iter().all(|p| p.channel(0).is_poison())` — every point, but
only channel 0. `crates/geom/src/lib.rs:71-78`, directly above it, says
the discriminator is that *"a placeholder's every control point is
all-poison"*, and that a described net carrying poison *"must fail
loudly as such … never masquerade as the benign placeholder"*. A
described net whose every control point has a poisoned `x` and finite
`y`/`z` is precisely that masquerade, and it now reads as the benign
placeholder at ~25 consumer sites (`step-export/src/writer.rs:44`,
`topo/src/props.rs:660`, `mesh/src/trimmed.rs:186`,
`geom-brep/src/certify.rs:999`, …).

The single-channel form was inherited from the surface half; the
crate-merge dedup moved it into a helper that has `CHANNELS` and
`channel(d)` in scope and did not widen it. This is the one place the
merge picked a half's behaviour and landed it under a doc describing the
other half's.

**Verdict:**

## S100. `scalar_lift` is named for the job it explicitly declines to do

S33's ~14 hand-written per-variant ladders are the thing called "scalar
lift" everywhere else in this programme;
`crates/geom/src/scalar_lift.rs:1` deduplicates only the four leaf
point/vec converters and says so in its header (*"the per-variant
*ladders* … stay where they are"*). The result is a module whose name is
the concept and whose contents are not, in the same crate as four
surviving ladders (`curves.rs:818`, `:908`, `surfaces.rs:653`, `:1057`),
each still spelling `Nurbs(_) => nurbs_placeholder()` — the exact silent
substitution S33 named.

The merge also put two spellings of one operation side by side that were
previously in different crates: `lift_to_dual` (curves) vs `lift_dual`
(surfaces), plus two unrelated `lift` functions. A reader looking for
"where does this crate lift a `Surface<f64>` to `Interval`" will open
`scalar_lift.rs` and not find it.

**Verdict:**

## S101. The merge's prose sweep deleted a cross-reference instead of re-aiming it

The pre-merge text at `crates/geom/src/curves/nurbs.rs:684-687` read
*"(The opposite choice — dividing by the min-weight floor, as
`geom_surfaces::recognize`'s conic-derivative work does — is the
direction for an UPPER bound…)"*. The sweep removed the clause, leaving
the claim without its precedent. The pointer was **already** mis-aimed —
there is no `recognize` module in `geom-surfaces`; the actual site is
`crates/step-import/src/recognize.rs:422`, which builds exactly the
`/ w_min` upper bound the sentence contrasts against. A sweep whose
stated job was "every stale crate name" resolved a stale *name* by
deleting the *fact*, and the only record that these two bounds are
deliberate opposites is gone.

Look for the same shape at every other site that sweep touched: the
pattern it matched was the identifier, and the identifier is what the
sentence was hanging on. The same sweep also left *"the geometry
**crates**"* standing at `crates/geom/src/curves/boxes.rs:8` and
`surfaces/boxes.rs:4` while correcting the manifest comment
(`Cargo.toml:26`) to the singular — it saw the argument and fixed only
the instance carrying a literal crate name.

**Verdict:**

## S102. Two more copy-sites in `geom` that the merge's whole justification was about

- `crates/geom/src/surfaces/nurbs.rs:4-11`: *"Data model, evaluation
  contract, and fixed-association rules are the curve module's … the
  conventions are **stated once there and once here**."* A copy-site
  declaring itself, in the crate whose justification was that the two
  halves stated one thing twice. The crate-doc hoist took the
  *enum-level* conventions and left the *payload-level* ones duplicated.
- `crates/geom/src/surfaces.rs:26-30`: a bullet titled *"The shared
  helper"* spelling the `radial`/`tangential` formula, kept verbatim,
  never naming the thing it calls the shared helper. The helper moved to
  `crate::azimuth` and is now shared with the *curve* half too — the
  merge's whole point. `azimuth.rs:1-20` claims to be the single home;
  `curves.rs` carries no matching paragraph, so the two halves' headers
  now disagree about who documents the frame.

**Verdict:**

## S103. The iso-curve placement rule now lives only in the code that already obeys it

The merge deleted the acyclicity sentence that had been enforcing
"iso-curve extraction belongs to the EdgeGeometry layer" and restated
the rule as 23 lines of prose at
`crates/geom-brep/src/nurbs_iso.rs:19-41` — honestly, including the
admission that *"nothing stops this module moving down except the rule
itself"*. But the restatement is in `geom-brep`, and the code that would
violate it is in `geom`. Nothing in `geom/src/lib.rs`,
`geom/src/surfaces.rs` or `geom/src/surfaces/nurbs.rs` mentions it, so
the person who adds the next extractor next to the payload — the exact
move the rule forbids, and the one the merge made structurally possible
— will not encounter it.

**Verdict:**

## S104. `attribute()` re-introduces the wildcard over a closed enum the same wave removed elsewhere

`ValidationError`'s doc says it is a closed enum precisely so *"every
match site is forced to say what it does with the new failure kinds"*,
and this same fix wave spent two diffs de-wildcarding fault
classification (`crates/pncad/src/workspace.rs:427-460`'s
`resolve_fault`/`load_fault`, with a paragraph explaining why a wildcard
is forbidden there; `crates/editor-core/src/names/select.rs:274-330`).
`crates/editor-core/src/assembly.rs:541` then adds
`_ => (None, Attribution::Refuted)` over that same closed enum, and the
classification is load-bearing — it decides between
`AssemblyError::AtRest` and `AssemblyError::Uncertified`. A future
variant carrying a declared face pair becomes `Unattributed` silently.
The `Attribution::Refuted` in that tuple is also dead: the `(None, _)`
arm at `:544-546` discards it.

The sibling instance is `crates/pncad-py/src/py/doc.rs:206-218`, where
`"randomness_unavailable"` is written as a **literal at the raise site**
— every other `variant` string in that crate comes from a `*_tag`
function in `tags.rs` whose premise is one home per enum→string map —
and the closure labels *any* of `WorkspaceError`'s nine variants with
it, no match. True today only because `random_document_id` has a single
failure arm, in another crate, with nothing tying the two. Sites not
read end to end and likely in the same class:
`crates/editor-core/src/mate.rs`, `crates/pncad-py/src/py/select.rs`.

**Verdict:**

## S105. The shared refusal ladder retired one duplication and minted a documented hand-synced one

`crates/editor-core/src/eval/wire.rs:717-723`'s new `ladder` module doc:
*"**Not shared with `crate::resolve`, yet** … The two agree by hand
across a module boundary, at coarser grain than the duplication this
module retired; folding them is a larger change … recorded as such and
not attempted here."* The duplication is real:
`crates/editor-core/src/resolve/mod.rs:552-557` re-derives the
`next_id`/`ForeignNode`/`NodeDeleted` rung and `:583`/`:606` rebuild the
same `TieWitness`. "Recorded as such" is not a schedule.

Same wave, same shape:
`crates/editor-core/src/persist/kernel_wire.rs:17-20` says the module
exists *"so the technique has one home and one doc instead of one per
type"*, and `boolean_op.rs:13-15` makes a point of the read direction
not restating the write direction — *"it calls it"*.
`contact_class::untag` (`contact_class.rs:87-113`) restates the table
anyway, for the enum that is `#[non_exhaustive]`, and has no equivalent
of `boolean_op::serialize`'s round-trip guard.

**Verdict:**

## S106. The profile `Step` vocabulary was unified inside `profile` only

S4 named five hand-synced copies across three crates.
`transition_table!` (`crates/profile/src/path/program.rs`) collapses four
*profile-local* projections and adds `Verb::ALL` as a census anchor;
`ProgramStep`, `WireStep` and `StepArg`
(`crates/editor-core/src/program.rs:64,56`,
`crates/editor-core/src/persist/wire.rs:252-255`) are not in the diff.
Nothing was renumbered — `feed_step`
(`crates/editor-core/src/eval/mod.rs:1565-1730`) still retires
11/19/20/25/29 and appends 30–40 — and being an exhaustive match on
`profile::Step` it is the one cross-crate copy that breaks loudly. The
other three do not: `res_step` matches on `ProgramStep` and
*constructs* `Step`, so a verb added to the table leaves the wire and
the expression-slot vocabulary silently short. The module doc's framing
(*"ONE declaration, FOUR projections"*, *"nothing is written twice"*) is
accurate within the crate and easy to read as more than it is.

**Verdict:**

## S107. The `DimensionError` untangling renamed the Rust type and left the Python-visible confusion in place

`DimensionError` → `QuantityOpMismatch` is a Rust-side rename only
(`crates/pncad-py/src/errors.rs:52-61,115-131,172-176`). From Python,
`DimensionError` still means the quantity-operator check, and the
document layer's real `DimensionError` still surfaces as `LiteralError`
on one door and as `PersistError` with `variant == "parse"` on the
other (`pncad.pyi:83-108`). The situation the first scan named — the
real dimension checker not being the thing called `DimensionError` — is
preserved and now defended by cross-referencing docstrings on both
classes plus a paragraph in `errors.rs`. Tag strings are correctly
unmoved and #694 schedules the `load`-door half; the naming half was
closed by argument rather than by change, and the length of the argument
is what draws attention to it.

**Verdict:**

## S108. `entries_off_bbox` exempts zero-lever entries, and its regression row calls it with a different predicate

`crates/mesh/src/curved.rs:411-419`'s admit test is
`gap_is_noise(du, lu, eps) || gap_is_noise(dv, lv, eps)`, i.e.
`du*lu < eps`. For a pole or apex entry `chart.radial` is exactly 0, so
the u term is `0 < eps` and the entry is admitted unconditionally
whatever its `u` — the same fail-open the `unreachable!` at
`walk.rs:1122-1131` was written to avoid. Probably harmless today (a
pole entry's `v` is a box extreme anyway) but nothing states the
exemption. Compounding it, `worst_entry_off_box` (`:1060`) passes
`eps = 0.0`, which makes `gap_is_noise` uniformly false and turns the
function into "report every entry's distance" — so the `== 0.0` row and
production do not exercise the same branch structure.

**Verdict:**

## S109. The falsifier's accumulator is discarded on exactly the paths its docs say it exists for

`crates/mesh/src/trimmed.rs:247-260` hoists `worst_ratio` out of the
retry loop because *"a certificate that fails on a triangle we then
threw away has still failed, and a falsifier that stops watching a case
is the defect it exists to catch"*. But `note_face` is the last
statement before `return Ok(triangles)` (`:575-591`), skipped by the
`CertificateExceeded` return sixteen lines above and by the
`Triangulation` returns inside the loop — so a face that ultimately
refuses discards every ratio it accumulated, which is the case most
likely to carry a violating sample. Separately,
`dev_samples_per_edge` is `None` for `Lane::Cylinder` (`:408-412`), so
`cert::cert_cylinder` is never falsified in this lane at all, while the
assertion message at `crates/mesh/tests/budget_meter.rs:165` reads as a
universal about triangles.

And that assertion is monotone the easy way:
`worst_ratio = d / (bound + eps) ≤ 1` gets *easier* as `bound` grows,
and a loose bound is precisely what #320 exists to detect. Both rows are
`#![cfg(feature = "budget")]`, as is
`every_nurbs_face_is_measured_once_and_by_key` — the only test anywhere
that exercises `face_bound`'s memo, so a mis-keying memo is invisible to
`cargo test -p mesh`.

**Verdict:**

---

# Tier 3 — real but lower stakes

Tier 3 is grouped into class roll-ups rather than one ID per instance,
the way the first scan handled S35 and S40. Each bullet is a distinct
site; cite them as e.g. `S110(d)`.

## S110. Tests and assertions that cannot go red (roll-up)

The largest class in this scan, and the one the brief's Q3 was
written for. Beyond S60, S75, S76, S78, S84 and S91 above:

- (a) `crates/topo/tests/probe_s5_sectors.rs:164-172` — six per-lane
  coverage assertions added so that *"delete the splitting fixtures and
  six assertions go red"*. They cannot: `ci.yml:1862` runs
  `cargo check -p "$c" --features probe --all-targets` and **nothing
  anywhere runs `cargo test -p topo --features probe`.** The file
  discloses the type-check at `:24-36` and then adds assertions whose
  stated purpose is gating.
- (b) `crates/sweep/tests/review_m6_5_pr2_sweep_probes.rs:70` — `x3b`
  computes a `Debug` hash and `println!`s it; no assertion. It existed
  to be diffed between the merge-base and HEAD of #220 — a comparison
  that no longer exists — and this diff kept it by re-scoping the doc to
  "two revisions" and renaming the label. **Two independent agents hit
  this.** `memories/test-suite-cost.md` names this the class to drop
  first; the change that touched it made it permanent. The file header
  also now claims the probes *"measure rather than assert"*, false of
  `x4` two functions above.
- (c) `crates/editor-core/tests/boolean_op_wire.rs:84` —
  `the_write_door_admits_every_operation_it_can_read_back` is a
  byte-for-byte copy of `every_operation_round_trips` eleven lines
  earlier. Its doc says it exercises the write door's own refusal;
  nothing in the body can reach that path, and the doc concedes the
  check *"must be INVISIBLE"*.
- (d) `crates/sweep/tests/s16_box_soundness.rs:201` —
  `a_lofted_operand_is_refused_at_its_nurbs_edges_before_any_face_box`
  exists to record why `NurbsExtentUnsupported` has no end-to-end row
  and asserts `CurvedEdgeUnsupported` instead. Its premise structurally
  excludes the case the re-gate exists for. Goes red only if a
  *neighbouring* gate changes.
- (e) `crates/profile/tests/onarc_probe.rs:147` —
  `assert!(!lp.tangent_joints().contains(&anchor_idx))` is green for any
  regression that empties or shortens `tangent_joints()`. The positive
  half lives in a different test on a different chain.
- (f) `crates/geom-core/tests/d8_knot_queries_adversarial.rs:880-887` —
  drives `surface_curve_residual` with adversarial break parameters and
  asserts only `Ok` and finiteness. A break parameter that mislocated a
  span but produced a finite bound passes.
- (g) `demos/wild/src/main.rs:321-325` —
  `assert_eq!(scenes.len(), WILD_CELLS.len())` where `scenes` is built
  from `WILD_CELLS.iter().map(…).collect()`. Equal by construction.
- (h) `interval-transcendentals/tests/common/mod.rs:149-152` —
  `assert!(mine.is_empty() || !mine.is_nai())` is strictly subsumed by
  the `assert!(mine.is_empty())` three lines below; it can only change
  the panic message.
- (i) `crates/sweep/tests/review_d8_consumer_differential.rs:217,298,398`
  — pinned literal seeds (`fuzz::pinned(…, 0x00d8_c0de_0000_000N)`) are
  licensed by the memory for the *digest* half, which is printed and
  never asserted. The same pinned draws also feed real counterexample
  searches (`form.num.breaks() == want` at `:239`, the union-vector
  checks), which will explore the same 24 points for the rest of the
  project's life. Two shapes in one test, of which only one is safe to
  pin. The merge-base comparison that motivated it has happened;
  nothing schedules another.
- (j) `demos/tour/tests/eps_regression.rs:1-4` vs `:37-53` — the doc
  says *"the only legal outcomes are a working run or a typed refusal
  (which the tour surfaces as a clean nonzero exit)"*; `run_tour`
  asserts `output.status.success()`, so a clean nonzero exit fails
  exactly as a panic does. The tour appears to have no such exit path
  anyway.

## S111. Frontier vocabulary and API surface with no reachable caller (roll-up)

- (a) `crates/sweep/src/fillet/surgery.rs:1144,1209,492,447` and
  `fillet/build.rs:228,280` — S7's charge against the retired door
  included *"eleven `unsupported(…)` refusal strings, all provably
  unreachable."* The **surviving** door now carries the same shape and
  self-declares it: `build.rs:247` says outright *"Neither refusal below
  is reachable through the front door"*; `resolve_rim`'s one-link guard
  is *"Likely dead in practice"*; `links_here.is_empty()` cannot hold;
  `corner_plan`'s `faces.len() != 3` cannot disagree with the valence
  the door just checked; `EmptyChain` is not something the battery
  emits. Each argument is individually reasonable; the aggregate is the
  defect the deletion was meant to close, on the door that survived.
- (b) `crates/sweep/src/fillet/surgery.rs:778` — `pub fn ring_clearance`
  is production API whose only caller outside the module is
  `crates/sweep/tests/m6_surgery.rs:434`, and whose doc says so. S52
  landed a `feature = "test-support"` gate in this very crate for this
  class; #672 records the residue and does not name this. Also:
  `sweep::fillet::surgery` is `pub` while every other item in it is
  `pub(super)`.
- (c) `interval-transcendentals/src/ops.rs:93` — `intersection` is
  outside the crate's declared scope (`lib.rs:10-13`: *"Scope is the
  kernel's inventoried surface only … Nothing else"*), is in neither of
  `docs/inventory.md`'s lists, and has **zero call sites** anywhere
  (every other public method has 1–1375). It is also the one op with a
  decoration rule nothing else follows. Mirror case:
  `src/consts.rs:40`'s `neg_frac_pi_2` is `pub fn` in a private module
  and, unlike `pi`/`frac_pi_2`, never cross-checked in
  `certify_constants`.
- (d) `crates/sweep/src/fillet/naming.rs:56` — `Retired` still has no
  face channel, so the one thing it exists to catch (a source entity
  destroyed without a record) is structurally uncatchable for faces. The
  whole-body door's retirement made the emitter's *"faces are never
  retired"* comment true of today's surgery; the hole S15 named is
  unchanged, asserted only over two fixtures.

## S112. Prose that describes a world the code has left (roll-up)

- (a) `crates/sweep/src/fillet/naming.rs:34` — *"`editor-core`'s
  `names::emit_fillet` … reads every field EXCEPT [`Retired`]"*.
  `emit_fillet.rs:220-221` builds `retired_e`/`retired_v` straight out
  of `rec.dead` and consults them. The diff rewrote the paragraphs
  immediately above and below and left this one.
- (b) `interval-transcendentals/tests/certify.rs:20-21` and
  `.github/workflows/ci.yml:1106-1107` still assert *"run this lane by
  hand"* / *"stays a by-hand gate"*. Commit `fb883b27` (2026-08-13)
  added the `oracle-certify` job and rewrote `README.md`; the `ci.yml`
  sentence sits ~55 lines above the job that contradicts it, in the same
  file.
- (c) `interval-transcendentals/src/ops.rs:1-2` and
  `docs/inventory.md:33,37` promise a `copysign`-style sign transfer as
  part of the exact surface. No such function exists in `src/`; the
  implementation lives in the consumer
  (`crates/geom-core/src/interval.rs:356`) with its own signed-zero and
  decoration-cap reasoning. **This bears on S1**: "delete `RingInterval`
  in favour of this crate" leaves at least one exact-surface op hosted
  inside a consumer.
- (d) `crates/geom-brep/src/props/curved.rs:886-890` — `cone_arm`'s doc
  still says its `T::one()` fallback *"covers the no-rim case, where
  `du_of_rims` refuses before any margin is metered"*. After the
  reorder, `require_rims_at_extremes` runs first. Nothing is wrong yet
  because it is vacuous on an empty rim list; the sentence describes an
  ordering the diff changed, fifteen lines from the code.
- (e) `crates/geom-brep/src/ssi/exhaust.rs:92` /
  `crates/geom-brep/src/ssi.rs:975` — `Exhaustiveness::floor`'s public
  doc says *"The floor used, in meters"*; the chart lane stores chart
  units (`domain.floor(band) / speed`). The sibling field on
  `ExhaustivenessInconclusive` gets it right (*"in meters (or chart
  units)"*), and so does `SweepCell::width`. The one place a caller
  reads the number back out is the one place the unit is wrong. S23's
  refactor made it visible by collapsing two lanes onto one parameter.
- (f) `crates/profile/src/sugar.rs:389-393` — `arc_fillet_trims`' header
  still reads *"extracted verbatim from the raw builder's corner door so
  the twin and the PATHS algebra lowering share one code path"*. There
  is no twin and no raw builder. The ONARC commit re-pointed four
  neighbouring references and left this one, which is the one asserting
  a *current* two-consumer property. (Three more "raw builder" sentences
  at `:311`, `:821`, `path.rs:1199` read as historical provenance.)
- (g) `crates/pncad/src/lib.rs:51-82` — the S20 fix rewrote *"What the
  façade itself contains"* to stop under-claiming about `workspace`, and
  the section still enumerates only `authoring`/`validated` plus that
  exception. `tolerance` was added at `lib.rs:187` in the same wave and
  does not appear. Not nothing: `report()` and `eps_source()` commit the
  process-global ε as a side effect of being called, which the module
  itself flags as a hazard.
- (h) `demos/render.py:95-101,127-131` — `draw()` branches on
  `if body["stl"] is None` and carries a whole second branch plus a
  `#111 pin` warning for a manifest state neither producer can emit
  (`ManifestBody.stl` is a `String`; `run_body` panics rather than
  omitting it). `render_freecad.py:159` reads the same field with no
  guard and would `TypeError`. The two renderers disagree about the
  schema and the one that guards is guarding against nothing.

## S113. Counts and enumerations stated in prose, already drifted (roll-up)

Beyond S64, S67, S74 and S98:

- (a) `demos/tour/src/uvdump.rs:23,60` and
  `demos/compose_uv_montage.py:17` — *"879 of the 982 M7 faces"*,
  *"all 982 faces"*. Written 2026-08-09; `klein.rs` landed 2026-08-16
  adding twelve faces on the bulb alone, and `tube.rs`/`skinned.rs`
  scenes landed after. `main.rs:604-608` prints the live `dumps.len()`
  and never compares. `demos/tour/Cargo.toml:33,65` says *"eighteen
  scenes"* against 27 `Stop {` literals and 35 committed PNGs.
- (b) `demos/wild/src/main.rs:85` says *"The pinned cell set: 6 cells"*;
  the derivation in the same paragraph arrives at eight and the array is
  `[Cell; 8]`. In a module that treats the cell set as *"LAW, not
  discovery"*.
- (c) `crates/geom-core/src/ring_interval.rs`'s `from_certified` doc
  carries a prose census of its own callers, already corrected once
  (`88616177`). See S89.
- (d) `crates/topo/src/chart_region.rs:875-891` — `SCHEDULE_2D`'s
  members 9 (`[0.75, -1.0]`) and 15 (`[-0.75, 1.0]`) are exact
  negatives, and both graze conditions in `ray_verdict` are invariant
  under negating `d`, so the sixteen-member retry ladder is fifteen. The
  S17 unification added the claim *"axes plus oblique spread members"*
  directly above it. Related: `splitting/containment.rs:126` still calls
  its dyadic-rational table *"golden-angle-spread"*.

## S114. Duplications the fix passes did not reach (roll-up)

- (a) `interval-transcendentals/src/round.rs:134` and
  `src/algebraic.rs:16` — `TWO_PROD_VALID_MIN` and
  `SQRT_EXACT_WITNESS_MIN` are the identical bit pattern
  `0x03F0_0000_0000_0000` in two files, reconciled by prose (*"the same
  2Prod validity floor as `round.rs`"*). The FMA witness itself is
  written out four times, two of them outside `round.rs` despite that
  module's doc claiming *"the load-bearing lemmas are restated at each
  helper"*.
- (b) `demos/tour/src/skinned.rs:195-215` — `quad`, `PRISM_SQUARE`,
  `PRISM_TRAPEZOID`, `lofted_at_z`, `ELBOW_H` mirrored constant for
  constant from `crates/step-export/tests/common/mod.rs:733` and
  friends. The captions claim the fixture is used *"VERBATIM"* — a
  byte-equality claim nothing computes. `fn quad` now has four homes
  (`sweep`, `mesh`, `step-export` test-commons, plus the demo);
  `ELBOW_H = 0.25` is copied with a comment saying it is *"shared"*.
- (c) The demo manifest and UV cell formats have no definition: three
  hand-rolled JSON emitters, four readers, one field
  (`transparency`) written by one producer and `.get(…, 0)`-defaulted by
  both readers; `View.up` re-encoded in the inverse direction across
  `render.py:51-57` and `render_freecad.py:105-133`;
  `compose_uv_montage.py`'s six-entry `LEGEND` mirroring
  `uvdump.rs:81-91` **unverified** while the cell size is verified.
  `check_render_provenance.py:104,112` announces the shape outright —
  *"Kept in sync with render.sh's `--montage=` arguments"*, *"keep the
  three spellings in sync"* (there are two).
- (d) `interval-transcendentals/src/trig.rs:30-58` vs `:62-88` and
  `invtrig.rs:21-39` vs `:43-61` — `sin`/`cos` and `asin`/`acos` are
  ~25-line near-verbatim twins differing in two constants and a libm
  call; the `if bounded { Com } else { Dac }` idiom is spelled out
  separately six times.
- (e) `crates/geom/src/curves.rs:942` and
  `crates/geom/src/surfaces.rs:1118` — `fn contains` duplicated with two
  parameter names, in the same crate, beside the four converters the
  dedup did collapse.
- (f) `crates/mesh/src/planar.rs:326-334` and
  `crates/mesh/src/trimmed.rs:434-468` — #678's sibling sweep produced a
  nine-line and a twenty-eight-line comment and **no code and no row**
  in either lane. Both arguments are load-bearing and both rest on facts
  that live in another module (`walk::Chart::poles()` being empty for a
  cylinder; NURBS faces having no `Chart`) and can move without either
  comment changing.

## S115. Disclosed and unscheduled (roll-up)

Every item here is honestly written down, and none has an issue number
or a named plan unit. This is C2/C3 measured after the A1 rule landed;
see §C.

- (a) `tools/tess-lint/src/lib.rs:26-34,141-145` and
  `tools/tess-meter/src/lib.rs:239-248` — the `agree` column is
  `grid_cells / span_cells` where `span_cells` is assigned `grid_cells`
  verbatim, so it is `1.00` by arithmetic. **Both crates** say so at
  length, both say it *"cannot detect the drift it was described as
  detecting"*, and both give the same reason for keeping it: *"a schema
  change and a re-cut baseline, and is unscheduled."* Meanwhile the CLI
  prints it to an operator in the ranked table and the totals line,
  labelled `agree`. Two crates independently documenting why a column is
  inert is more effort than deleting it.
- (b) `scripts/doc-gate.sh:45-58` — *"a row is owed that does the same
  for every excluded root, and it is unscheduled."* ~1,050 lines of
  `tools/tess-meter` prose went from covered to uncovered by moving, in
  a gate whose existence argument is that prose which stops rendering is
  a real loss. #709 is cited as the cause, not the schedule. (S71's
  dangling intra-doc link is in a `tests/` file and outside this gate
  too.)
- (c) `crates/pncad/src/prelude.rs:11-21` — the corpus-frequency
  measurement that chose the prelude cut *"was taken once, by hand, and
  nothing re-takes it"*, classified as *"unguarded rather than
  unguardable — a re-run of the import census would guard it"*. By its
  own account a guard is available and not taken; no import-census row
  exists in `ci.yml`.
- (d) `crates/mesh/src/walk.rs:560-583` — the D2-addendum
  `debug_assert` deviation, disclosed with *"A typed warning channel
  would dominate all three; there is none."*
- (e) `crates/editor-core/src/eval/wire.rs:717-723` — see S105.
- (f) `crates/topo/src/euler.rs:3220-3251` — `strum::EnumCount` named as
  the way out and declined; see S94.

## S116. Naming, shape and residue — no mechanism at stake (roll-up)

- (a) `crates/geom/src/net.rs:20-90` — the control-net trait replaced
  `vec![lift(|p| p.x), lift(|p| p.y), lift(|p| p.z)]` (exhaustive by
  construction, no panic reachable) with a `0..P::CHANNELS` loop through
  `channel(d)`, so correctness now rests on each impl's `CHANNELS`
  agreeing with its own match arms — announced with two `unreachable!`
  arms that did not exist before, and a nine-line comment explaining
  that they are *"unguardable by construction"*. The `unreachable`
  family is deliberately outside the panic ban, so this is taste, not
  policy — but it is a hazard the abstraction introduced.
- (b) `crates/geom/src/` now has three modules named `projection`, two
  named `boxes` and two named `nurbs`; inside `surfaces/projection.rs`
  the line `use crate::projection::{…}` and the module's own name refer
  to different things. And `azimuth::frame` returns
  `(radial, tangential)` — both `Vec3<T>`, so a transposed destructure
  compiles silently, which `azimuth.rs:64-80` says and points at
  indirect coverage — while the overwhelming majority of call sites take
  `(radial, _)` or `(_, tangential)`. The header's two-door rationale
  does not match what the arms need, which is three shapes.
- (c) `crates/topo/src/sector_face.rs:118-123` — three things in one
  crate are named `sector_face`, and the shared module's doc concedes
  the collision in advance (*"so `sector_face` in prose means one thing
  per scope instead of three"*) rather than resolving it. Inside the
  boolean wrapper, `match resolved.carrier { Plane | Cylinder | Sphere
  => {} }` is a no-op whose only purpose is to make a future variant a
  compile error — an invariant held by a statement where a type would
  hold it structurally.
- (d) `demos/tour/src/main.rs:118-124,142-155` — `plain_planar` and
  `seamed_curved` are exact forwarders to `plain`/`seamed`, kept alive
  by doc-comments arguing that *"the CALLER is asserting planarity"*.
  Nothing consumes that. `step_expected` is `true` at both construction
  sites and its doc says so, so `assert!(!sb.step_expected, …)` is an
  unconditional panic wearing a conditional, re-checked a second way
  twelve lines later.
- (e) `crates/topo/src/euler.rs:1-208` — the module header grew ~55
  lines in this diff, and the new material is a titled essay
  (*"**The exception, and it is a real one.**"*) about `crate::instance`'s
  graft — which is `instance`'s contract, and a reader of `instance.rs`
  will not find it. Two screens of taxonomy prose before the ten Euler
  operators. C5, measured.
- (f) `crates/topo/src/review_m1_pr2/release_corruption.rs:187-202` — a
  hard **10 ms wall-clock bound** in the one release-profile job that
  gates, defended by twelve lines giving the measurement (1.5 µs
  release) and an explicit statement that CI's box is different and
  slower. The clause it defends (*"every traversal is bounded"*) was
  already detected by the previous `< 5 s`. No evidence it has flaked;
  the thoroughness of the defence is what draws attention.
- (g) `crates/mesh/src/curved.rs:340-551` — S28 ("three parallel
  tessellation pipelines with no shared core") was answered with a
  refusal and ~470 lines of prose. The file went from 243 to 712
  production lines of which 429 are comments (60%); the two guard
  functions carry ~180 doc lines over ~55 lines of code. The shared core
  does not exist; what exists is a long argument that this lane does not
  need one.
- (h) `crates/mesh/src/nurbs_cert.rs:374-419` — S29's constant count did
  not go down. `SAFE_ASPECT = 5.0` is unchanged and still sits above its
  own derived √15 ≈ 3.87; `MAX_GRID_RETRIES` is still a bare `6`; the
  12-vs-6 sample-density split survives. What changed is that
  `SAFE_ASPECT`'s doc grew from ~20 lines to ~50, adding a register
  whose first bullet concedes the guard is one-sided and whose last
  paragraph concedes the 0.60·δ margin has no guard at all — a more
  honest version of one undecided question, not fewer decisions.
- (i) `tools/tess-lint/src/lib.rs:36-39` — `split` is printed under a
  name that denies two of the three things it measures: `grid_cells /
  span_opt_cells` mixes the cheaper split point, the banding's
  max-across-u cost, and the aspect snap. `tess-meter`'s doc says so;
  `tess-lint`'s doc and the CLI legend the operator reads say only
  *"what a cheaper split point per cell would still recover"*. Commit
  `1aba0704` is this in the record: a safety fix moved the number the
  gate calls a sizing regression.
- (j) `tools/k-lint/src/lib.rs:231,368-371` — adding a name to
  `EPS_COUPLED_PREDICATES` moves it from the metre floor (4e-5) to
  `1.5e2·ε` — 2.4 decades looser at ε=1e-9 — and exempts it from rules
  (2) and (3). Nothing tests membership. The CLI's discipline message
  enumerates two recourses and does not mention this one, so the
  cheapest available move is the one the tool never warns about.
- (k) `tools/tess-lint/src/main.rs` uses bare `println!` throughout
  while claiming to follow *"`k-lint`'s three-voice split, and for its
  reason"*; `k-lint` has a deliberate `say()` wrapper so that
  `k-lint … | head` ends quietly. `tess-lint … | head` panics and exits
  101 — a fourth, unnamed voice.
- (l) `tools/k-lint/src/lib.rs:1-209` and `tools/tess-lint/src/lib.rs:1-92`
  are outside every doc gate; the CI job adds a `cargo doc` step for
  `tess-meter` only, with a comment stating *"the same hole covers every
  excluded root"*. `k-lint`'s 209-line header is the densest
  intra-doc-linked prose in `tools/`. Cf. S115(b).
- (m) `scripts/gates/bounds-allowlist.sh:24-30` says the argument
  *"live[s] in ONE home: geom-core/src/real.rs … Not restated here; keep
  this a pointer"* — and is immediately followed by ~100 lines restating
  the `chart_region`, `edge_nurbs`, `arc_fillet`, fillet-battery and
  `CertifiedBounds` rulings that `real.rs:365-556` also carries. S13's
  complaint was a ~157-line ledger in front of a two-method trait; the
  fix gave that ledger a second home in front of a 20-line grep. The
  copies already drifted: `:69-73` is a self-correction (*"This
  paragraph used to say … Both clauses are false"*).
- (n) `scripts/gates/signed-zero-one-home.sh:108` —
  `PAT_ADD_REVERSED='0\.0 \+ [A-Za-z_*(]'` fires on `0.0 + eps`, which
  the header implies it exempts (*"`0.0 + 1e-12`, a real offset"*). In a
  numerics codebase the real offset is more often a named epsilon than a
  literal. The header's blind-spot section lists five things the gate
  cannot see and no false positives; the green-case fixture plants only
  literals. Otherwise the best-tested gate in the directory, which is
  why the omission stands out.
- (o) `scripts/k_probe_sweep.sh:96,99` — the demo-scene half has none of
  `run_dump`'s guards (nonzero passed-count, ≥2 CSV lines) and merges
  with `tail -n +2`. A tour that emits a header and no scenes merges
  silently into the linted CSV. The header at `:50-56` claims *"this is
  the only place that can tell the difference between 'clean' and 'ran
  nothing'"* — true of the corpus and M2 halves, not of the demo half
  sitting between them. `rundump-guard-selftest.sh` proves all three of
  `run_dump`'s refusals fire; the self-test's coverage is defined by
  which code was extractable as a function.
- (p) `crates/sweep/src/revolve/mod.rs:391,544` — `MultipleAxisRuns`
  went from *"the boundary would split into multiple shells (M3)"* to
  *"a **permanent** refusal under the ratified sweeps-vs-voids invariant
  … not a deferral"*, and its `Display` from *"deferred to M3"* to a
  four-clause recourse. That is a change in what the kernel promises,
  resting on an unstated geometric claim (that every profile with ≥2
  disjoint on-axis runs encloses a void when fully revolved). Probably
  true for a validated simply-connected profile; nothing in the diff
  proves it and no test pins it. The sibling `FullRevolveHoles` has its
  own argument written out. (Confidence: unsure.)
- (q) `crates/topo/src/seqgen.rs:839-849` — the `SplitEdge` roundtrip
  asserts only `canonical_form(before) == canonical_form(after)`, and
  `iso`'s module doc — edited in the same diff — says curve/surface
  payloads are ignored. So the `set_edge_curve(e, spec)` that
  `split_site`'s doc justifies at length is not checked by the property
  it was written for; delete it and the roundtrip still passes.
- (r) `interval-transcendentals/src/ops.rs:90-106` — `intersection`
  returns `Trv` on **every** input, a 1788 formality about set
  operations rather than a domain-violation record, on the same wire
  that carries domain clamps. `lib.rs:19-22` documents only the clamp
  class; the `atan2`-at-origin case (`invtrig.rs:113`) is undocumented
  too. A consumer that (per S41) started consulting the decoration in
  `lo()`/`hi()` would poison every intersection result.
  `interval.rs:135-143` is the natural place for the caveat and does not
  carry it.
- (s) `crates/geom/tests/surfaces/m5_pr7_surface_projection.rs:224-228`
  and `crates/geom/tests/curves/projection.rs:219` — both new overflow
  rows are built around *"Finite inputs throughout"*, which is the
  load-bearing half of the claim; the surface row checks
  `p.x.is_finite() && p.z.is_finite()` and skips `y`, the coordinate
  that actually varies across the fixture, and the curve row checks `x`
  alone.
- (t) `interval-transcendentals/examples/bench.rs:2` tells you to run
  `cargo run --release --example bench`, which fails: `Cargo.toml:74`
  gives it `required-features = ["oracle-inari"]`. And
  `tests/review_fuzz_div.rs:96-100`'s `cmp_f64_vs_rat` is
  `#[allow(dead_code)]` justified by a *future* property test.
- (u) Residue: `crates/sweep/tests/m5_pr12_refusals.rs:518` has a
  leftover `let p = Point3::new(0.0,0.0,0.0); let _ = p;`.
  `crates/pncad-py/src/py/doc.rs:206-218`'s raise-site literal is
  S104's sibling.
  `interval-transcendentals`' `2^-960` vs the literature's `~2^-969` is
  a nine-binade round-up justified as absorbing *"every boundary
  quibble"* — empirical rather than derived, harmless because
  over-gating only costs tightness, and the one number in that crate
  that is chosen rather than proven.

---

# §A. Where I would start

Ordered by value for time, with an eye to not polishing code that is
about to move.

**First, because they are cheap and they make everything else
trustworthy — the instruments.** S59, S61, S62, S63 and S73 all say the
same thing: several of the mechanisms this project uses to *know* things
do not fire. S59 is a one-character-class regex change plus a self-test
row. S63 is three regexes and a shared comment stripper that wants to
live in `lib.sh`. S61/S62 are CI wiring. S73 is `ratio`'s fallback value
and a join key. None of these is architectural, and until they are done,
a green board is weaker evidence than it looks.

**Second, the two soundness questions.** S66 (the cylinder box widened
along its own axis, now feeding a containment test) is the only finding
in this scan that can produce a wrong answer rather than a missing
check, and it has no counter-row that could catch it. S86 (`RingInterval`
laundering NaN through `certified_bracket`) is the same class one layer
down.

**Third, the three "the fix pass had the file open" rows**, because they
are small and they close S60/S68/S74's classes rather than instances:
S60 (`area_pad` — the volume row twelve lines up shows exactly what the
fix looks like), S68 (`split_edge`'s three discards, ten lines below two
`unreachable!` conversions the same diff added), S74 (`swept.rs`'s false
non-unification reason, plus re-checking every other *"deliberately NOT
unified"* claim in the S6 record).

**Fourth, S64 and S65**, together, because they are one conversation
about what the mesh crate promises: a headline sentence that is false, a
consumer enumeration that is short, and a watertightness backstop that
is absent from the builds that render.

**Wait on:** S110's roll-up and most of S116. S110(b) and (i) are worth
doing with the test-suite-cost sweep rather than separately, and
S116's prose-volume rows (e, g, h) should wait until C5 has a decision —
trimming them one at a time without one is how they grew.

**Do not start with:** S71. The enclosing-fillet class question is a
*design* decision (was the capability dropped on purpose?), not a defect
to fix, and it belongs in a conversation before any code moves. Same for
S116(p).

---

# §B. Negative results and coverage

**What this scan did NOT cover.** `crates/step-import/`,
`crates/step-export/`, `crates/stl/`, `crates/bvh/`, `crates/persist/`
and the GUI-adjacent crates got no dedicated scope this round; they were
covered in the first scan and have low churn since. `demos/tour/src/lily.rs`
(2,446 lines) was sampled, not read end to end — the demos agent flagged
it as the thinnest part of its coverage and noted that its *declared*
gaps at `:664`, `:1727` and `:120` are all well recorded, so the
undeclared kind is what to look for there.

**Things that checked out, reported because a negative result from a
scan this size is worth as much as a finding.**

- **`interval-transcendentals`' mathematics.** `docs/derivations.md` §1
  P1/P2/P3 are genuine proofs, the P3 route is correctly applied per
  function, and the one non-derived ingredient (Assumption A: libm meets
  its CI bit-distance bound on all inputs, not just sampled ones) is
  stated plainly at `derivations.md:95-100` rather than smuggled. No
  unsoundness in `round.rs`'s witnesses, in `Div`'s pole/touching-zero
  case split, or in `atan2`'s three-case table. S72 is about what the
  tests can see, not about the crate being wrong.
- **The `geom` merge was a faithful merge.** The four constants are
  byte-identical at base (32, 8, 1e-13, 1e-12) — a pure rename.
  `validate_counts`, `poison_point` and `removal_pass_bound` diffed line
  by line: `removal_pass_bound` character-identical modulo the macro
  splat; `validate_counts` differed only in signature, same checks in
  the same order, same NaN-catching `!(w > 0)`. Both Newton loops'
  acceptance conditions, clamping, stagnation guard, `!step.is_finite()`
  break and typed-inconclusive payloads match arm for arm. **No half's
  semantics was discarded.** The manifest is `geom-curves`' verbatim
  plus curvo/nalgebra, no feature widening, and a workspace grep for
  `geom-curves|geom_curves|geom-surfaces|geom_surfaces` outside `docs/`
  and `target/` returns nothing — including in `DESIGN.md`. The
  `is_placeholder` drift-close held: no consumer still open-codes the
  test inline.
- **`RingInterval::from_bounds` → `from_certified` and `Bounds` →
  `CertifiedBounds` on all three `ring_coords`** looks like a semantic
  change inside the merge and is not: it landed in `ee6d76b4` before it,
  and all three payloads were converted together.
- **The release-profile contract does run.** `.github/workflows/ci.yml:769-816`
  adds a `corrupt input (release profile)` job and
  `scripts/ci-filter.py:261` wires `RUN_TOPO_RELEASE` to topo's closure
  membership. My dispatch brief said otherwise; see §C15.
- **`tess-meter` does not duplicate `tess-lint`** — producer and
  consumer, with `mesh::budget::FaceMeasure` as the contract. The
  `EXPECTED_HEADER`/`CSV_HEADER` duplication across the two cargo roots
  is genuinely pinned by `tools/tess-meter/tests/derivations.rs:193-215`,
  which scrapes the sibling's source rather than sharing a constant, and
  says why. `divisions` vs `chords::ceil_count` is a declared second
  spelling whose two divergences check out against
  `crates/mesh/src/chords.rs:93-100`. The three-way baseline discipline
  in `docs/k-report-data/README.md` holds, and both tess-budget re-cuts
  named their reason in the commit subject. Both crates' tests visibly
  hunt their own vacuous assertions — rarer than it should be.
- **`k-lint`'s `BASELINE_FLOOR_MARGIN`** is the model for a boxed
  constant: pinned into `(3.9e-5, 4.7965e-5]` by two tests and floored
  again by the `#99` litmus. Cited in S73 as the contrast.
- **`kernel-serde-free.sh` and `test-features-dev-only.sh`** could not be
  broken by a planted fixture. `evalscalar-allowlist.sh:121` is
  order-insensitive and correct.
- **Test files read in full with no finding:**
  `crates/sweep/tests/review_d2_recourse_at_the_site.rs`,
  `crates/sweep/tests/s49_census_jurisdiction.rs` (both directions,
  per-turn sweep, separated-pair control),
  `crates/geom-core/tests/eps_provenance.rs` (fresh-process probes,
  controlled env, no vacuous-pass path),
  `crates/geom-core/tests/decoration_seam.rs`'s paired sweep at `:153`
  (non-vacuity floors on both halves),
  `crates/geom-core/tests/knot_queries_differential.rs`,
  `crates/mesh/tests/profile_overrides.rs` (**planted-violation
  self-test is exemplary**),
  `crates/geom-brep/tests/s58_iso_rectangle.rs` and
  `crates/step-import/tests/s58_iso_rectangle.rs` (controls that must
  still measure exactly; refusals matched on the *named* predicate),
  `crates/step-import/tests/split_iso_side.rs`,
  `crates/geom-brep/tests/decoration_plane_mint.rs`,
  `crates/sweep/tests/readback_doors.rs`,
  `crates/editor-core/tests/schema_ledger.rs`, and the new loft rows in
  `crates/sweep/tests/m5_s11_concave_sense.rs`.
- **`every_suite_file_is_aggregated`** does compute rather than restate
  its count, and the escaped-quote trick that keeps its own `format!`
  out of the tally is correct. It would not catch a `#[path]` line
  inside a comment — a narrow hole.
- **`rundump-guard-selftest.sh`** proves all three of `run_dump`'s
  refusals fire. Good work; S116(o) is about the half it does not cover.
- **Tag strings in `pncad-py` are correctly unmoved** by the
  `DimensionError` rename (S107), which is the half that would have
  broken Python callers.

**S36 note, unbelaboured.** Of the 34 test files added since the base,
ten are named after a scan row, PR, or issue rather than the behaviour
they pin: `review_d2_adv_probes`, `review_d2_recourse_at_the_site`,
`review_d8_consumer_differential`, `s16_box_soundness`,
`s49_census_jurisdiction`, `s58_iso_rectangle` (×2),
`d8_knot_queries_adversarial`, `probe_s5_sectors`, `e4_dual_door`. Per
Evan's Tier-3 verdict on S36, renaming waits on an actual review and
fixup of each suite.

---

# §C. Process observations

Continuing `docs/SMELL-SCAN-2026-08.md` §C's numbering (C1–C14). The
first scan's §C was written from PR descriptions, A/B logs and
orchestrator logs. This one is written from something better: **a
controlled second look at code that the first round's findings had
already been applied to.** That is the closest this project has come to
measuring its own fix quality.

## C15. Two of my dispatch briefs were wrong, and both agents checked anyway

I wrote thirteen briefs. Two contained a false premise:

1. I told the `topo` agent that CI never runs the corrupt-input contract
   in release. It does — `ci.yml:769-816`. The agent opened its report
   with *"One correction to the dispatch premise"* and then reported on
   what the job **covers** rather than on whether it runs.
2. I told the `geom-brep` agent that S26 (area enclosure metering) was
   fixed, and asked which direction the metered faces moved. It is not
   fixed. The agent: *"Whatever briefed this as fixed was reading #472's
   deferral, not the tree."* My brief was internally inconsistent — it
   said #472 deferred it and then asked which direction moved — and the
   agent resolved the inconsistency against the tree rather than against
   me.

**This is the finding, not the errors.** A dispatcher's brief is the
highest-authority text a scanning agent sees, and the failure mode it
invites is confirmation: an agent told "X was fixed, check for
regressions" can produce a plausible regression report about a fix that
never landed. Both agents refused the frame. What made that possible is
plainly in the stance — *"a textual justification is not a defence"* and
*"your taste is evidence"* generalise to the brief itself, and neither
agent needed to be told that a dispatcher can be wrong.

**Worth making explicit in the reviewer's own document anyway.**
`docs/REVIEW-STYLE-DISPATCH.md` §3 already tells the dispatcher that
*"reviewers correcting the dispatcher is a working lane, not a
malfunction — say so in the brief"*, and
`docs/prompts/reviewer-style-lane.md` §1 does not yet say it. The
missing sentence is that the dispatch is a **hypothesis**, and that
contradicting it is a first-class result. It cost nothing here; it will
not always. (I have not edited either document — that is a ratified
process artefact and the change is Evan's call.)

## C16. The dominant defect shape is now "the fix pass had the file open"

This is the single strongest signal in the scan, and it is a *new* shape
— the first scan could not have seen it because there were no fixes yet.
Count: S59 (the ruling swept to two gates, not the third, in the same
directory), S60 (`volume_pad` fixed, `area_pad` twelve lines away not),
S63 (same), S68 (`split_edge`'s discards ten lines below the same diff's
`unreachable!` conversions), S74 (markers deleted at the copy sites), S80,
S84, S85 (`Enclosure` and `CertifiedEnclosure` corrected, `Bounds`'
headline not), S101 (the sweep deleted the fact rather than re-aiming
the pointer), S102, S110(b), S114(f), S116(m).

The mechanism is consistent enough to state as a rule: **a fix pass
scoped by the finding's citation list sweeps the citations and stops.**
The sibling instance is one screen away, in a file the author had open,
and the scope sentence in the fix's own prose is what makes it
invisible — several of these fixes *state* their scope
(`euler{,_ring,_kill}.rs` plus `link_half_edges`; "the per-variant
ladders stay where they are"; "the reported instances"), and the
statement reads as completeness.

**The rule already exists, on both sides, and it did not fire.**
`docs/REVIEW-STYLE-DISPATCH.md` §2 names *"the fix reproducing the
defect it closes"*; `docs/prompts/reviewer-style-lane.md` §3's
class-not-instance rule says that sweeping only the reported instance
*"is a **half-fix** and should be labelled one"*; and
`docs/prompts/implementer-discipline.md` §5 puts the obligation on the
fix pass directly: *"If your unit fixes an instance of a class, say what
pattern you swept with and **what that pattern could not match**."*
Thirteen instances landed anyway. The interesting question is not what
rule is missing but why the one we have does not bite.

**Two mechanisms, both visible in the artefacts.**

- **§5's trigger is the author's own classification.** *"If your unit
  fixes an instance of a class"* — and the recurring failure is a fix
  that was never classified as a class fix. `volume_pad` was fixed as a
  row, not as an instance of the monotone-enclosure class (S60). The
  both-operand-orders ruling was applied to the gate that reported it
  (S59, S63). The condition is exactly the judgement that fails.
- **§5's deliverable is a pattern, not a hit list.** An author who greps,
  sees three hits, fixes one, and writes *"swept `euler*.rs`; the pattern
  could not match delegating callers"* has complied in full. And a scope
  sentence reads as completeness even when the claim above it does not
  share its scope: `euler.rs:24` asserts the universal — *"at every
  write"* — while its evidence is *"these modules"*, which is how
  `split_edge` ended up three discards deep in the same diff (S68).

**So the amendment is small and specific: make the trigger unconditional
and make the artefact the hits.** Grep for the *shape* — not the symbol
(Q4's distinction) — before writing the scope sentence, and put the hit
list and its disposition in the PR description, one line per hit: fixed,
or not-this-unit and why. A pattern with no hits recorded is a claim; a
hit list is a receipt. S60 is the cleanest demonstration:
`rg area_pad crates/*/tests` returns two tightness-relevant sites,
neither bounds it, and the fix pass was editing the file that contains
both.

## C17. C11's mechanism is real and has now been observed running backwards

C11 (first scan): every duplication in this codebase is self-declared in
prose (`verbatim`, `re-derived`, `ported from`, `mirror of`), and
nothing ever reads that prose. It was proposed as the cheapest
actionable mechanism available.

S74 is the counter-case that proves its value: the `revolve`/`extrude`
twins carried exactly those markers, and a consolidation commit
**deleted both markers while leaving both copies**, replacing them with
a sentence asserting the two are not twins — a sentence that is
factually wrong about `reverse: bool`. The greppable evidence was the
only evidence, and a well-intentioned cleanup removed it.

If the marker vocabulary is ever mechanised, the guard has to include
"marker removed without the code converging", not just "marker
present".

## C18. Q3 ("can this test fail?") is carrying the scan

Of ~110 findings, the largest single class is assertions that cannot go
red: S60, S75, S76, S78, S84, S91, and the ten sites in S110, plus
S66's acceptance suite, S72's pad probes and S73's `ratio`. Several were
found by *executing* a mutation — the `interval-transcendentals` agent
set `PAD_ULPS = 64` and `PAD_ULPS = 0` and reduced the rounding helpers
to round-to-nearest; the `scripts/gates` agent planted fixtures against
every gate. **Every claim so produced held.**

One sub-shape dominates:

- **Monotone in the wrong direction.** `area_pad > 0.0` plus
  containment; `assert_contains` on a widening enclosure;
  `worst_ratio ≤ 1` as `bound` grows; `holds(&box, sample)` on a box
  that only widens; `!contains(&anchor_idx)` on a list that may empty.
  The pattern is: *the assertion is satisfied more easily by exactly the
  degradation it exists to catch.* `reviewer-style-lane.md` Q3 already
  names this one; these are measurements of it, not a gap in the brief.

**A second shape — a skip reading as a pass — is deliberately left
un-rolled-up.** The instances stand on their own (S84 and the
`else { continue }` / `if let Ok(...)` / tolerant-arm /
`println!("SKIPPED")` sites cited with them). A class-level rule was
drafted and dropped: it was written around giving skips *floors*, which
concedes the skip, and the prior question — whether a test should be
skipping at all — is the one to answer first. Recorded here so the
next scan re-opens the question rather than re-proposing the floors
(Evan, 2026-08-20).

**Cheapest mechanisation available:** for every enclosure-style
acceptance row, require a *ceiling* alongside the containment. The
volume rows already do it; the pattern is three lines and it is the
difference between S60 and a row that works.

## C19. Executing the mutation beats reading the code, and it was rare

Three of thirteen agents ran experiments rather than only reading. Those
three produced the scan's most certain findings — every "green with the
guard removed" claim is a fact, not a judgement, and none needed a
steelman pass. The other ten produced findings that are mostly still
*questions*.

This is a cheap upgrade to the brief: **when a finding is "this guard
does not guard", try to break it.** A scratch copy of the crate and a
one-line mutation is minutes, and it converts a `likely` into a `sure`.

## C20. The A1 rule (non-improvement deviations owe a scheduled followup) has not taken yet

S115 is six disclosures written *after* the rule, none with an issue
number or a named plan unit, several stating "unscheduled" as though it
were the schedule (`tools`' `agree` column says it in two crates
independently; `doc-gate.sh` says *"a row is owed … and it is
unscheduled"*). S90 is the sharpest version: the D1 ruling's three
*smaller* residues all got issue numbers (#687, #700, #701) and the one
seam it actually left unguarded got prose.

The disclosures are honest and well written, which is exactly the C2
diagnosis. `docs/REVIEW-STYLE-DISPATCH.md` §4 already warns the
dispatcher not to let the `## Style` section *"become the place where
known problems go to be recorded and forgotten"*, and Q6 exists to close
it — so this is not an unnamed problem. It is a named problem with no
mechanism.

What the rule lacks is a place that *executes*; C3 said this. The
register has to be mechanical: a grep for the disclosure vocabulary that
fails without an adjacent issue number would be a gate in the style of
the fourteen that already exist — and S63 is the warning about how
carefully that regex would need to be written, since every one of the
six existing grep gates has a hole of exactly that kind.

## C21. The style brief worked, and here is what it cost

First use at scale: thirteen agents, ~110 findings, of which I judge
roughly a dozen to be over-reaches and ten hand-verified to hold. The
question-numbered self-reports at the end of each agent's output (*"Q1
— findings 2, 4, 6, 7; Q4 not exercised, no diff to invalidate
against"*) were unexpectedly useful as a coverage receipt, and I would
keep them.

Two observations for the next revision (recommendations only — I did not
edit `docs/prompts/reviewer-style-lane.md`):

- **Q8 (read a whole file end to end) produced the findings nothing else
  would have.** S116(e) (the euler header is now two screens of another
  module's contract), S116(g) (60% comments), and the demos agent's
  honest note that `lily.rs` was sampled rather than read. C4 said
  nothing in the process reads a whole file; Q8 is the fix and it is
  working.
- **The stance's "report more rather than fewer" produced a long tail
  that needs a coordinator.** Roughly a third of the raw findings became
  roll-up bullets here rather than standing rows. That is the right
  outcome, and saying so in §3 ("what your findings must look like")
  would stop agents calibrating toward fewer, better-defended findings —
  the defended ones are not the valuable ones.

## C22. Documentation growth is still the default response to a finding

C5 measured this in the first scan; it has not turned. Measured this
round: `real.rs`'s `Bounds` block 156 → 234 lines (S85);
`crates/mesh/src/curved.rs` 243 → 712 production lines, 60% comments,
with ~180 doc lines over ~55 lines of guard code (S116g);
`SAFE_ASPECT`'s doc ~20 → ~50 lines while the constant did not move
(S116h); `crates/topo/src/euler.rs`'s header +55 lines (S116e);
`scripts/gates/bounds-allowlist.sh` — 130 lines of header defending a
20-line function, restating a ledger it declares it is not restating
(S116m).

In several of these the prose is the *only* change: S116(g) answers
"three parallel pipelines with no shared core" with a long argument that
this lane does not need one; S107 closes a naming confusion by argument
rather than by change; S116(h) converts one undecided constant into a
more honest account of the same undecided constant.

None of this is dishonest — the opposite; it is unusually candid. But
the brief's own rule (*"unusual justification length is mild evidence
for a smell"*) now has a large, measured corpus behind it, and the
question it raises is a policy one for Evan rather than a finding:
**when a finding's honest answer is "we are not going to change this",
what is the maximum acceptable length of that answer, and where does it
live?** A 234-line trait doc and a 130-line gate header are both past
the point where the rule is findable, which is the failure mode that
matters.

---

*End of report. Findings carry no verdicts yet; annotate in place. IDs
`S59`–`S116` are stable.*
