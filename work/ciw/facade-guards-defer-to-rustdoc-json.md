---
id: facade-guards-defer-to-rustdoc-json
kind: issue
title: Three facade guards defer to a rustdoc-JSON check that is not scheduled
status: open
opened: 2026-08-20
github: 696
refs: [689]
---

## From GitHub issue 696

Opened 2026-08-20; 0 comments.

Three guards in `crates/pncad/tests/all.rs` are each a source-text fallback for the same unbuilt mechanism, and each says so in its own docs without anyone owning the follow-up:

- `no_arena_key_is_nameable_through_the_facade_document_surface` — "The intended enforcement was a rustdoc-JSON scan of `pncad`'s public API. This toolchain is stable-only and `--output-format json` is nightly-gated… So this is the FALLBACK."
- `no_raw_loop_minting_door_is_nameable_through_the_facade` — same posture, one file wider.
- `every_document_layer_root_export_is_carried_or_listed` — "A rustdoc-JSON check would close [the root-only blind spot] and is nightly-gated."

Each is honest on its own; three deep with no issue number is a deferral nobody is scheduled to discharge, which is the shape the SMELL-SCAN postmortems keep finding.

**What a rustdoc-JSON pass would buy, concretely:**

1. **Aliases and re-spellings.** The LB13 guard's own stated weakness: a key type re-exported under an alias, or reachable as an associated type or a public field of an allowed type, is invisible to a `pub use` text scan.
2. **Below-root reachability.** The completeness guard reads `editor-core`'s *root*. A public name reachable only by module path and never lifted to the root is invisible to it — the exact structural hole the original closure audit's second pass found (`topo::boolean::ContainError`, `geom_curves::EllipseInvalid`).
3. **Direct `pub` items.** The completeness guard reads `pub use` statements only; a `pub struct` written directly in `editor-core/src/lib.rs` would not be seen. That file currently has zero direct `pub` items, so the blind spot is held shut by a coincidence rather than by a rule.

**What it costs:** `--output-format json` is nightly-gated, so this is a CI change — a second toolchain in the workflow, and a decision about whether the JSON format's instability is acceptable for a gate. That is why three units in a row declined it, and it is a fair decline; what is missing is a place for the decision to live.

**Disposition wanted:** either schedule the CI work, or rule that the three text scans are the permanent answer so their docs can stop pointing at a mechanism nobody intends to build.

Filed from the S20/S21 fix pass (PR #689).

## Home

LIB: all three guards live in `crates/pncad/tests/all.rs` and police the `pncad` facade's public surface — the program's territory and the subject of its curation charter.

## The cost half of this question got cheaper (2026-09-04, CIW)

Unchanged: all three guards still defer (`crates/pncad/tests/all.rs`,
the LB13 pair and the completeness guard), and the disposition wanted is
still a ruling — schedule the rustdoc-JSON pass, or declare the three
text scans permanent and rewrite their docs.

What moved is the stated cost. This issue says the decline is fair
because a rustdoc-JSON pass means *"a second toolchain in the workflow,
and a decision about whether the JSON format's instability is
acceptable for a gate"*. The first of those two is now most of a
non-issue:

- the repository went public on 2026-09-03, so standard-runner minutes
  are free and a second toolchain download costs wall clock on one job
  rather than billed minutes on every PR;
- `.github/workflows/nightly.yml` now exists as a real home for exactly
  this shape of check — ungated, once a day, not on anyone's critical
  path — and TCOST-C2/C3 have just moved two comparable passes into it.
  A nightly-only rustdoc-JSON scan does not touch the per-PR gate at
  all.

The **format instability** half is untouched and is still the whole of
the real question: a gate keyed on a nightly-gated, explicitly unstable
JSON schema breaks on a toolchain bump, and the repository pins its
toolchain (`rust-toolchain.toml`), so it would need a second pin with
its own bump discipline. That is the thing for Ev to rule on, and it
should be put to Ev with the cheap-half correction above rather than
with the original framing.

Ordering: CIW schedules this after
`work/ciw/f3-recosting-on-a-public-repo`, whose measurement establishes
what a nightly-seated pass actually costs in wall clock, so the ruling
is asked with a number in it.

## THE RULING, ASKED (2026-09-04, CIW unit 10)

Everything below is derived against `d799235e` (`origin/main`) at the
moment of writing, and every number names the derivation that produced
it. Where a check contradicted this file's earlier text, the check
wins and the disagreement is stated rather than edited away.

### Re-verified against the tree

**The three deferrals are all still live and still uncarried.**
`crates/pncad/tests/all.rs` cites **#696** at exactly three sites and
nowhere else (`grep -n '696'`): lines 794-812 (the LB13 guard, `fn` at
815), 875-876 (the RawLoop guard, `fn` at 902) and 3276 (the
completeness guard, `fn` at 3293). Wording unchanged.

**The three things the issue says rustdoc JSON would buy do not all
still cost a nightly.** A fourth guard has landed since this issue was
filed — `every_profile_layer_root_export_is_carried_or_listed`
(`all.rs:3517`) — and it closes blind spot 3 for the profile layer
**with a text scan**: `root_declared_pub_names` (`all.rs:3444`) reads
`pub struct/enum/fn/trait/type/const/static/union/mod` written at
column 0, and the profile guard unions it with the `pub use` set. Its
own docs say so: *"the guard's second blind spot … is closed here
rather than held shut by a coincidence."* So bullet 3 above is now
buyable in-file, by the mechanism sitting fifty lines from the guard
that wants it, with no second toolchain.

**Blind spot 3's premise holds, with a correction to how it is
worded.** `crates/editor-core/src/lib.rs` declares **zero** `pub`
items of any non-`mod` kind (replicating `root_declared_pub_names`
over the file: 32 `pub mod` at column 0, 28 after
`code_without_cfg_gated` removes the four `#[cfg(feature =
"interval")]` ones, and nothing else). The issue's *"zero direct `pub`
items"* is therefore true of type-like items and false as literally
written — `pub mod` is a direct `pub` item, and the scanner counts it.
That distinction is why the fix is not free: applying
`root_declared_pub_names` to this root adds 28-32 module names to the
export set, each of which would then need carrying or a `NOT_CARRIED`
entry. A `mod`-excluding variant is the obvious spelling, and it is
LIB's call, not this unit's. **Also stale in the tree:** the same
helper's doc calls these *"the crate's twenty-six interior modules"*;
the count is 32 (28 ungated) as of `d799235e`. LIB's file, reported
not fixed.

**Blind spot 1 is real and now has a number.** `editor-core/src/`
declares **414** column-0 `pub` items outside `lib.rs`; the root's
`pub use` lists introduce **370** leaf names; **64** of the 414 are
named nowhere in the root. Those 64 are public API of a public module
(`editor_core::clearance::ClearanceQuery` and its family are the
largest block) and the completeness guard, which reads the root only,
asks for no decision about any of them. Note what this blind spot is
NOT: `editor_core` is not re-exported whole, so a below-root name is
not *reachable* one hop past the façade. It is an accounting hole —
names that grow without anyone being made to decide — not a leak.

**Blind spot 2 (aliases) has zero instances and one partial
mitigation.** No `as`-alias `pub use` exists anywhere in
`editor-core/src/lib.rs` or in the eleven façade sources. If one
appeared, the completeness guard would still force a decision: a newly
aliased root export is a new name in `module_pub_use_names`, hence
uncarried, hence a failure — with a garbled name in the message, since
that scanner takes the leaf of `a::b::EntityRef as NodeHandle` without
stripping the `as` clause. What stays genuinely invisible is the
sub-class the guard docs name: a key reachable as a **public field,
associated type or return type** of a carried item. Searched:
`EntityRef`/`EntityKey` appear in a public signature at four sites in
`editor-core` — `resolve/mod.rs:432` and `:442` (`pub entity:
EntityRef` on `MeshPatchKey` and `Resolved`), `appearance.rs:297`
(`AppearanceResolution::for_node`), and `names/table.rs:22` (`pub key:
EntityKey`, inside `EntityRef` itself). All three containing types are
in `NOT_CARRIED` (`all.rs:3092`, `3113`, `3139`), so **no live
instance today**. The façade's own sources name the three keys only in
comments (`document.rs:7`, `:16`, `select.rs:113`). Blind spot of this
search, stated: it greps the three literal key names in one crate, so
a key behind a type alias, a generic parameter or an associated type
in another crate would not appear in it.

**One hole NOT stated in any of the three guards' docs, found here.**
Both LB13 guards are **line-local**: they skip a line unless it
contains `pub use`, then look for the key on that same line
(`all.rs:830`, `all.rs:927`). The façade's dominant idiom is the
multi-line brace list — **33 of its 77 `pub use` statements** open a
list on their own line, and **15 of those name `editor_core::`** — so
a key added inside an existing list (the natural spelling of the
regression) is invisible to the guard that exists to catch it. It is
not invisible to the file as a whole: `pub_use_names` (`all.rs:3200`)
is statement-based, so the façade carrying `EntityRef` in a multi-line
`pub use editor_core::{…}` makes a `NOT_CARRIED` entry stale and reds
the completeness guard instead — with a message about a stale
exclusion list rather than about LB13. A re-export spread over lines
through a façade-internal path (`pub use crate::document::{` newline
`EntityRef,` newline `};`) evades both. This is a text-scan defect
fixable in text, by reusing the statement-based scanner already in the
file. It is LIB's, and it is reported here rather than filed on LIB's
slate.

### What a nightly rustdoc-JSON pass costs, measured

**Method.** No build was run (this unit is prose, and the box is
disk-constrained). Every figure is read from the Actions API for the
scheduled nightly of 2026-09-04, run **33859647263** — the first
nightly on the public 4-vCPU runner — and priced against its closest
existing analogue, the `rustdoc (gate, every root)` job
(**100981004034**), whose log was read in full.

That analogue is a conservative one: it ran with **no cargo cache at
all** (`No cache found.`, 09:43:07) and still finished every pass in
**266 s** — the workspace under `--all-features` plus 7 roots outside
it, 7 of them re-read at `--no-default-features`, 8 cargo roots
scanned. Inside it, the cold `--workspace --all-features` HTML pass
documented all 18 workspace crates in **59.67 s** (`Finished` at
09:44:53). `cargo doc` builds dependencies to metadata rather than
codegen, which is why that is a fifth of `build + archive`'s 336 s
median in `f3-recosting-on-a-public-repo`'s M2.

| component | value | source |
|---|---|---|
| checkout + prune | 3-4 s | every nightly job, this run |
| toolchain install | **8.7 s** measured for `dtolnay/rust-toolchain@stable` downloading 5 components; **10-25 s** estimated for a first nightly install | job log, `duration_ms=8729` |
| one root, one feature selection, cold | **40-70 s**, bounded above by the 59.67 s that documented all 18 crates | job log |
| the scan over the emitted JSON | seconds | not measured |
| **job total** | **≈ 90-120 s** | sum |

Against the nightly it would join: that run's 13 jobs total **3051
job-seconds (50.9 job-minutes)**, and its wall clock is set by
`opt-level calibration` at **1452 s (24.2 min)**. A new parallel job of
~120 s is **+3.9 % of the nightly's runner load and +0 minutes on its
critical path**. In money, **zero**: public repo, standard runner.
The per-PR gate is untouched.

**What could not be measured without building it**, stated so nobody
reads the table as more than it is:

1. **Whether `cargo +nightly rustdoc -p pncad -- -Zunstable-options
   --output-format json` completes on this dependency graph at all.**
   Never once run in this repo — `grep` finds no `+nightly`, no
   `--output-format json` and no nightly toolchain install anywhere in
   `.github/`, `scripts/` or `local-scripts/`. This would be the
   repo's first second toolchain.
2. The emitted JSON's size, and therefore the scan's own runtime.
3. **How often the format actually breaks the scan.** That is the
   ruling, and it is not measurable in advance.
4. Whether a second toolchain's cache entry would evict an existing
   one. The rustdoc-roots entry alone saves **523 MB**, and it found
   no cache last night — one miss is not proof of eviction (a changed
   lockfile or rustc hash explains it equally), but it is evidence the
   cache is not a reliable subsidy here in either direction.

### The two dispositions

**(1) Schedule the CI work.** A nightly-only job: install a
**date-pinned** nightly, emit rustdoc JSON for `pncad`, scan it for
the arena keys and the minting doors, and check the document layer's
whole public surface — not just its root — against the façade's
carried set. Cost as measured above. The real cost is not minutes:

- a **second pin with its own bump discipline**, in a repo whose
  `rust-toolchain.toml` opens *"determinism starts with a pinned
  compiler"* (D9/L2). Pinned, the scan is stable and its cache stays
  warm; unpinned, it recompiles daily and can break on any nightly.
- a pinned nightly **rots silently**: it keeps passing while drifting
  from the pinned stable the rest of the gate uses, so the scan
  gradually describes a different compiler's view of the API than the
  one that ships.
- the format is explicitly unstable, and the dangerous failure is not
  the loud one. These three guards make **negative** claims — *no* key
  is nameable. A negative claim over a schema that moved reads as
  green. The mitigation exists and is already this file's own idiom
  (the vacuity floors, `min_exports` 150 and 40), and it would have to
  be built deliberately rather than inherited.

**(2) Rule the three text scans permanent** and rewrite their docs to
describe a permanent fallback with named limits instead of pointing at
an unscheduled mechanism. What is given up, priced by the checks
above: the alias/field/associated-type class only — zero instances
today, partly mitigated by the completeness guard's uncarried assert
for the alias half, genuinely open for the field half. What is NOT
given up, because it is reachable in text and this tree now proves it:
blind spot 3 (`root_declared_pub_names`, already doing this job for
the profile layer), the line-locality hole found above
(`pub_use_names` is already statement-based), and even a below-root
sweep — the 64 figure in this file was derived by a thirty-line scan
of `editor-core/src/**/*.rs`, not by a rustdoc pass.

Option 2 is not a concession. Three guards honestly describing a
permanent fallback with named limits are worth more than three
pointing at vapour, and the follow-on work it implies is smaller,
lands in the file it protects, and adds no toolchain.

### Recommendation: (2), with the text work handed to LIB

The argument for (1) is that minutes are now free and `nightly.yml`
is the right home. Both true, and neither is the question. The
question is whether the guarded classes justify a second pinned
compiler and its bump discipline, and against this tree they do not:
two of the three purchases are text-reachable — one of them
demonstrably, fifty lines away, in the same file — and the third has
no live instance and is half-covered by an existing assert.

**The strongest objection, stated because it nearly reverses this.**
Zero instances is not an argument against a guard: `pub use
editor_core;` has zero instances too, and that is the guard working,
not the guard being unnecessary. The reason it does not carry the day
is reachability by an ordinary edit. Re-adding a whole-crate
re-export, or adding a key to a curated list, is one line someone
could plausibly write — and those are exactly what the text scans
catch (modulo the line-locality hole, which is text-fixable). Exposing
a key through an alias or a public field takes a coordinated two-crate
edit whose second half already trips an assert. That asymmetry, not
the instance count, is the reason.

If Ev rules (1) instead, the honest version costs more than the table:
a date-pinned nightly, a bump note wherever `rust-toolchain.toml`'s
pin is documented, a vacuity floor on the JSON scan, and a rule for
what happens the morning the pass reds on a schema change. CIW would
own the workflow row; the scan and the guard rewrites are LIB's either
way, since all four guards live in `crates/pncad/tests/all.rs`.

### The question for Ev

Is a second, date-pinned nightly toolchain — with its own bump
discipline, in a repo that pins its compiler for determinism — worth
buying a blind-spot class with no instances in the tree, when two of
the three things the rustdoc-JSON pass was wanted for turn out to be
reachable by the text scanners already in the same file?
