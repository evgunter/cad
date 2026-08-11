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

| run | config | wall | sccache |
|---|---|---|---|
| A  | baseline (GNU ld, full DWARF, incremental) | **4189 s** (69m23s) | — |
| C1 | mold + line-tables-only + sccache, cache empty | **186 s** | 21% hits |
| C2 | same, cache warm (= what a NEW LANE now costs) | **96 s** | 99.4% hits |

`target/` fell to 1.5 GB (lanes previously ran 2–8 GB); shared sccache
2.8 GB. NOTE: A was taken ~2 h before C under different load, and a 22x
gap is far outside #174's CI-measured -38%, so a same-conditions control
(`aprime.sh`) was run rather than reporting the gap as the knobs' doing —
see [[cad-working-style]] on measured-not-assumed. The most likely real
mechanism is memory: full DWARF + incremental for 14 statically-linked
test binaries on a 10 GB box crosses into swap, and that penalty is
nonlinear, unlike CI's tidy percentage.

**sccache is the one that matters most here.** `new-lane.sh` clones
fresh, so before this every new lane paid a full cold build of the
width-1 mutex before its agent could do anything. sccache is
content-addressed and shared across lanes, and the ~225-package
dependency graph is identical across them, so only the first payer pays.
Unlike a shared `CARGO_TARGET_DIR` it cannot ping-pong: two lanes on
different branches coexist in the cache instead of invalidating each
other. See [[agent-lane-operations]] for the slot machinery it sits under.

**Cost of changing any of these knobs:** every lane re-fingerprints and
pays one cold rebuild. With the shared cache warm that is ~96 s, not 70
minutes — but the first lane after a change still pays full price.
