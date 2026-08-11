---
name: machine-build-config
description: This box's local cargo build configuration (mold linker, line-tables-only debuginfo, sccache) — what is set, why it is machine-local rather than repo-committed, and the 2026-08-11 measurements that justify it
metadata:
  type: project
---

Applied 2026-08-11 by `scripts/setup-build-env.sh`, which writes
`~/.cargo/config.toml`. Re-run it to restore the machine after a
reinstall; `--print` shows the block without writing.

**What is set.** mold 2.41.0 as linker; `debug = "line-tables-only"` on
dev+test; `rustc-wrapper = "sccache"` with `incremental = false` and a
15 GiB cache.

**Why machine-local, not the repo's `.cargo/config.toml`.** The repo file
is checked out on CI runners, where mold lives at a different path (or
not at all on non-build jobs) — a committed linker flag breaks them. And
the flags must apply to EVERY cargo invocation here: RUSTFLAGS is part of
cargo's fingerprint, so if only slot-wrapped builds carried them, an
agent's unwrapped `cargo check` would invalidate the whole `target/` and
the next wrapped build would invalidate it back. That rules out exporting
from `with-build-slot.sh`. `gate.sh` unsets RUSTFLAGS to protect its warm
tree; config-file rustflags are not env, so the gate sees them
consistently and does not re-fingerprint.

**gcc 9.4 is why mold looked hard.** `-fuse-ld=mold` needs gcc 12.1+;
this box is Ubuntu 20.04. mold ships `libexec/mold/ld` and `-C
link-arg=-B<that dir>` makes gcc resolve `ld` to it. #174 landed mold on
CI only and read the local move as heavier than it is — this is likely
the reason. `-fuse-ld=`/`-B` is a LINK argument: it cannot move rounding
or instruction selection, so D9 is untouched (#174 established this).
`debug=` is DWARF emission only — `debug-assertions` and
`overflow-checks` are unaffected, backtraces keep file:line; only
debugger variable inspection is lost.

**The measurements** (cold `cargo build --workspace --all-targets`, this
lane, 8 threads / 10 GB):

| run | config | wall | target/ | sccache |
|---|---|---|---|---|
| A  | baseline, taken 02:22–03:32 | **4189 s** (69m23s) | — | — |
| A' | baseline CONTROL, same window as C | **189 s** | 4.7 GB | — |
| C1 | mold + line-tables-only + sccache, cache empty | **186 s** | 1.5 GB | 21% hits |
| C2 | same, cache warm (= what a NEW LANE costs) | **96 s** | 1.5 GB | 99.4% hits |

**READ A AND A' TOGETHER — they are the same config and the same tree**
(182 vs 197 crates compiled, 30 test binaries, full DWARF both times).
The 22x gap between them is ENVIRONMENTAL, not configuration. A control
run was taken precisely because a 22x gap was far outside #174's
CI-measured -38%; without it the whole gain would have been misattributed
to the knobs.

What the controlled comparison actually says:

* **mold + line-tables-only buy no measurable wall-clock here** (A' 189 s
  vs C1 186 s — noise). Not a contradiction of #174: that -38% was
  measured across **261** test binaries, and after #179 plus the
  step-import collapse this workspace has **14**, so the per-binary
  link constant mold attacks is now a small share of the build.
* **line-tables-only still earns its place on SIZE**: `target/` 4.7 GB ->
  1.5 GB (-68%). On a 10 GB box with ~10 lanes that is page cache and
  disk, not compile time.
* **sccache is the real win**: 186 s -> 96 s cold at a 99.4% hit rate,
  and it is what makes `new-lane.sh` cheap.
* **Machine-condition variance dwarfs every config knob.** Same work,
  same tree, 3 min vs 69 min. Chasing compiler flags before that variance
  is understood is optimizing the wrong term — see the express-lane
  hypothesis in [[agent-lane-operations]].

**sccache is the one that matters most here.** `new-lane.sh` clones
fresh, so every new lane pays a cold build of the width-1 mutex before
its agent can do anything — ~190 s under good conditions, and far worse
under bad ones. sccache cuts that to ~96 s and, more importantly, makes
it insensitive to how much of the graph is already known. sccache is
content-addressed and shared across lanes, and the ~225-package
dependency graph is identical across them, so only the first payer pays.
Unlike a shared `CARGO_TARGET_DIR` it cannot ping-pong: two lanes on
different branches coexist in the cache instead of invalidating each
other. See [[agent-lane-operations]] for the slot machinery it sits under.

**Cost of changing any of these knobs:** every lane re-fingerprints and
pays one cold rebuild. With the shared cache warm that is ~96 s, not 70
minutes — but the first lane after a change still pays full price.
