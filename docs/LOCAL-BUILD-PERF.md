# Local build performance — measured findings (2026-08-11)

Investigation of why ~10 concurrent agent lanes spend so much time waiting
on the `with-build-slot.sh` mutex. **Read this before tuning local build
config**: three knobs were tried, two were measured to be worthless or
harmful here and reverted, and the one thing that actually explains the
pain was not a compiler setting at all.

Companion to `~/.local/share/cad-work/bazel-verdict-report.md` (hosted-CI
compile cost) and PR #174 (which landed mold + line-tables-only on CI).
This document is about the LOCAL box.

**See also `docs/GENERICS-BUILD-COST.md` (2026-08-12)** — the other half
of the picture. This document is about *machine conditions*; that one is
about *what the compiler is actually spending time on* (answer: not the
`Real` generics; 62% of workspace LLVM IR is serde in one crate). It also
supersedes two numbers used here: the mold verdict below became the basis
for retiring mold from CI, and `test-fast.sh`'s 15× speedup is now 4.30×.

## 0. The machine

| | |
|---|---|
| CPU | Intel i7-1065G7 — **4 physical cores / 8 threads**, 15 W laptop part, sustaining ~1.5 GHz |
| RAM | 12.6 GB host, **10 GB** to WSL2 (`.wslconfig memory=10GB`) |
| Swap | 3 GB |
| Disk | single ext4-in-VHDX |
| gcc | 9.4 (Ubuntu 20.04) |
| Load | serves up to ~10 concurrent Claude agent lanes |

This is a 2019 ultrabook. **No configuration makes it serve ten agents
doing Rust compilation comfortably.** The width-1 build mutex is the
correct response to this hardware, not a workaround — see #230.

## 1. The headline: machine-condition variance dwarfs every config knob

The same cold `cargo build --workspace --all-targets`, **same config, same
tree**, 182–197 crates compiled, 30 test binaries, full DWARF both times:

| when | wall |
|---|---|
| 00:22:14 – 01:32:09 PDT | **4189 s (69m23s)** |
| ~08:00 PDT (control) | **189 s (3m08s)** |

**22x.** Both runs completed successfully (`cargo` printed its own
`Finished ... in 69m 23s`); neither hung.

Every compiler knob measured below moves single-digit percents or nothing.
**If build waits feel pathological, this is the term to investigate — not
flags.** The control run existed only because a 22x gap was far outside
#174's CI-measured -38%; without it the entire difference would have been
misattributed to the linker, and this document would be recommending mold.

### Leading hypothesis (UNVERIFIED — needs a #230-style measurement)

The slow window had **express-lane jobs running alongside the main-slot
build** (`clippy`, `cargo test`, the python suite, a `pncad-py` build). The
fast window did not.

#269 sized the express lane on #230's "concurrency costs ~40%". But #230
measured two *builds* on a box that was never memory-tight (min
MemAvailable 5.5 GB). At 10 GB with full-DWARF link jobs, the box can cross
into swap, where the penalty is nonlinear rather than a percentage. Swap
activity is real here: 854k pages out observed at session start.

**This is the highest-value open follow-up.** Design it like #230: express
job concurrent with a battery, memory sampled throughout, before the lane
is resized or kept.

## 2. What was measured, and what survived

All cold rows are `cargo build --workspace --all-targets` after
`cargo clean`; edit rows append a comment to one crate and rebuild.

| config | cold | edit geom-core | edit topo | `target/` |
|---|---|---|---|---|
| baseline (GNU ld, full DWARF, incremental) | 189 s | — | — | 4.7 GB |
| mold + line-tables-only + sccache, cold cache | 186 s | — | — | 1.5 GB |
| …same, warm cache | 96 s | 91 s, 89 s | 74 s, 88 s | 1.5 GB |
| incremental, no sccache | 156 s | **18 s, 19 s** | **10 s, 12 s** | 3.8 GB |

### KEPT: `debug = "line-tables-only"`

`target/` **4.7 GB → 1.5 GB (-68%)**. Buys **no measurable compile time**.
It is a SIZE knob, kept because ~10 lanes each carrying a `target/` on a
10 GB-RAM box is page cache and disk pressure. `debug-assertions` and
`overflow-checks` are unaffected — fail-loud postconditions keep their
teeth, backtraces keep file:line. Only debugger variable inspection is
lost, which no agent uses.

### REVERTED: mold

**189 s baseline vs 186 s with mold + thin debuginfo — noise.**

This does *not* contradict #174's -38%. That was measured across **261**
test binaries; after #179 and #387 this workspace has **14**, so the
per-binary link constant mold attacks is now a small share of the build.

Two notes for anyone tempted to re-adopt it from first principles:
* `-fuse-ld=mold` **does not work on this box** — it needs gcc 12.1+ and
  this is gcc 9.4. mold ships `libexec/mold/ld`; `-C link-arg=-B<that dir>`
  is what makes gcc resolve `ld` to it. This gcc gap is the most likely
  reason #174 read a local mold as a heavier lift than it is.
* Reverted rather than kept-as-harmless: Ubuntu 20.04 has no mold package,
  so it is a hand-installed dependency at a machine-specific path, and an
  unused dependency in the build path is a liability. **Revisit only if the
  test-target count grows back toward triple digits.**

### REVERTED: sccache — the instructive one

sccache is genuinely good at what it does. A new lane's cold build went
**156 s → 96 s** at a **99.4% hit rate**: the ~225 dependency crates are
byte-identical across lanes, content-addressed caching serves them, and
unlike a shared `CARGO_TARGET_DIR` it cannot ping-pong between branches.

**But sccache and incremental compilation are mutually exclusive.** sccache
hard-refuses to run with `CARGO_INCREMENTAL` set:

```
sccache: incremental compilation is prohibited: Unset CARGO_INCREMENTAL to continue.
```

There is no hybrid config. Adopting sccache forces `incremental = false`
machine-wide, and that costs:

| edit → rebuild | sccache, no incremental | incremental, no sccache |
|---|---|---|
| geom-core (invalidates 100% of test bins) | 91 s, 89 s | **18 s, 19 s** |
| topo (~71%) | 74 s, 88 s | **10 s, 12 s** |

**5–7x slower on the edit-rebuild loop.**

The trade is bad on *frequency*, which is the part that is easy to get
wrong: sccache saves ~60 s **once per lane creation**; incremental saves
~73 s on **every edit an agent makes**, dozens to hundreds of times a day
per agent. The rare operation was optimized at the expense of the constant
one.

**When sccache would be right:** if lane churn ever dominates — mass
lane creation, short-lived lanes that never reach a steady edit loop — it
is the correct tool for exactly that. It is wrong as a default.

## 3. What actually helped: fewer test binaries

`crates/step-import` had no `autotests = false`, so its 26 `tests/*.rs`
files were 26 separate `[[test]]` targets — two thirds of every remaining
test target in the workspace. #179 collapsed the rest (249 → 12) on the
bazel-verdict finding that per-binary codegen+link was 96% of the CI build
job, and missed this crate.

Collapsing it (#387) took the workspace 39 → 14 test targets:

| | test targets | CI `build test binaries + archive` |
|---|---|---|
| before | 39 | 148 s |
| after | 14 | **108 s (-27%)** |

Single sample each and cache warmth may differ, so treat the percentage as
indicative — but the direction is consistent with the per-binary cost model.

**This is now gated.** `scripts/gates/test-aggregation.sh` asserts at most
one `[[test]]` target per workspace member, wired into `ci.yml`'s
discipline job and reached locally by the loop over `scripts/gates/`. It is the complement to
each crate's `every_suite_file_is_aggregated` test: that test catches a
crate that opted in but forgot a `#[path]` line, and **cannot fire for a
crate that never opted in** — which is exactly how step-import survived
#179. Both halves are needed.

Both halves proved themselves within hours of landing: the per-crate guard
caught `rw2_probes.rs`, which merged to main from another lane while #387
was open and would otherwise have silently stopped compiling and running.

### Consequence for agents

After aggregation, `cargo test -p <crate> --test <suite>` **no longer
works** — there is one binary named `all`, and suite names are module
prefixes. Use:

```
cargo test -p step-import --test all <suite>::      # e.g. wild::
```

## 4. Operational traps found along the way

* **Killing a slot-wrapped build does NOT free the mutex.** Children
  inherit the flock fds, so an orphaned `cargo` reparented to init keeps
  the slot held. Kill the whole process tree. Diagnose with
  `fuser -v ~/.local/share/cad-work/locks/slot-1.lock`, which shows true fd
  holders (the `.holder` files are best-effort reporting and do lie).
* **A daemon spawned under a slot holds the lock forever.** If a compiler
  cache or watcher is ever added to the build path, pre-start it *before*
  `with-build-slot.sh` opens its lock fds. See the comment at that point in
  the script.
* **Unsetting `RUSTC_WRAPPER` does not bypass a config-file wrapper.** Once
  `rustc-wrapper` is in `~/.cargo/config.toml`, env `RUSTC_WRAPPER=""` is
  ignored; `CARGO_BUILD_RUSTC_WRAPPER=""` is the override that works.
* **GitHub check rows can hang "pending" on a completed run.** Two jobs on
  #387 reported `completed_at: null` and pending in `gh pr checks` while
  every step read `completed/success` and the run concluded `success`.
  Check the **run** conclusion, not the per-check rollup, or a monitor will
  wait forever.
* **Unquoted heredocs execute backticks in comments.** A generated config
  file's comment containing a backticked `cargo build --workspace
  --all-targets` was command-substituted and actually ran.

## 5. If you pick this up next

Ranked by expected value:

1. **Measure the express-lane cost model** (§1). The 22x term. Everything
   else is rounding error next to it.
2. **Leave the compiler flags alone** unless the test-target count grows
   substantially. mold and sccache are measured dead ends *at this size*;
   §2 records the conditions under which each becomes right again.
3. **Move builds off this box** if agent count grows. 4 cores at 1.5 GHz
   cannot serve 10 agents; the repo already has hosted CI and a hosted
   render lane (#323/#338), so the pattern exists.
4. **Reduce build frequency**, not build cost: `cargo clippy`/`check` over
   the whole workspace is ~36 s against minutes for a full build.

## 6. `scripts/` vs `local-scripts/` (2026-08-11)

Tooling is split by **who runs it**, because a change to a purely local
script was forcing the full hosted matrix (`ci-filter.py` is an allowlist
that fails closed, and `scripts/**` was unrecognised ⇒ `TIER=all`).

* `scripts/` — the things **hosted CI runs**, `scripts/gates/` included:
  `ci-filter.py`, `check_admesh.sh`, `check_step.sh`,
  `step_import_check.py`, `k_probe_sweep.sh`. Changes here still force
  `TIER=all`; they can move a hosted result.
* `local-scripts/` — everything else (`ci-local.sh`, `gate.sh`,
  `with-build-slot.sh`, `test-fast.sh`, `new-lane.sh`, `clean-lanes.sh`,
  `fmt-all.sh`, `render-hosted.sh`, `setup-build-env.sh`, `hooks/`,
  `monitors/`, `review-lily/`). Classified non-triggering, like docs.

**The split is enforced, not conventional**: every workflow job runs
`rm -rf local-scripts` immediately after checkout. A workflow that grows a
reference to a local script fails loudly on the next run rather than
silently coupling the hosted gate to a developer's machine — which is what
makes it safe for the filter to skip CI on those changes.

`ci-local.sh` and `gate.sh` are local despite appearing in `ci.yml`: those
are comment mentions only, verified by checking for non-comment references.

### The stranded-hook trap (self-healing since 2026-08-11)

`new-lane.sh` stores the hooks path in each clone's `.git/config`, so the
split stranded every EXISTING lane: git finds no hooks directory and runs no
hook **without saying anything**, so the pre-push `fmt-all --check` simply
stops. A silent failure, which the charter does not tolerate.

It bit this investigation's own lane first — its two pushes after the move
ran no hook at all, and only the CI `rustfmt` row (the backstop) would have
caught a violation.

`with-build-slot.sh` now repairs a dangling `core.hooksPath` on the spot and
says so, because it is the one script every lane runs constantly: a stranded
lane fixes itself on its next build instead of waiting for someone to
remember a one-liner. If the hooks directory is missing entirely it warns
loudly rather than proceeding quietly. The manual equivalent, if you want it
now: `git config core.hooksPath local-scripts/hooks`.

**The self-heal was a MIGRATION SHIM — RETIRED 2026-08-15 (due
2026-08-13; the expiry nag did its job).** Every lane had cycled, so the
repair block is deleted; what remains in with-build-slot.sh is the loud
WARNING on a dangling `core.hooksPath` (a permanent repair path would
hide the very misconfiguration it papers over — the warning names the
one-liner fix instead).

**General lesson, worth more than this instance**: a repo-relative path
cached in per-clone git config is invisible to any repo-side rename. Grep
for `git config` when moving directories.

## Reproducing

Scripts and their full logs are archived on the branch
**`scratch/build-perf-experiments-2026-08-11`** under
`experiments/build-perf-2026-08-11/` (116 KB). That branch is a scratch
archive and is NOT for merge; its README maps each script to the numbers it
produced, and records how to read them honestly — including which runs
FAILED and why each failure was itself a finding.

They take the exclusive build slot and block every other lane: a cold
workspace build here is minutes at best and over an hour at worst, so scope
deliberately and announce it before starting.
