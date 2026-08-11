# Local build-perf experiments — 2026-08-11

**This branch is a SCRATCH ARCHIVE and is NOT for merge.** It exists so the
raw evidence behind `docs/LOCAL-BUILD-PERF.md` outlives the lane it was
produced in. Nothing here is maintained, run by CI, or referenced by any
build.

Findings and conclusions live in `docs/LOCAL-BUILD-PERF.md` on `main` — read
that first. This directory is only the scripts and their logs.

## Machine

4-core/8-thread i7-1065G7 (~1.5 GHz sustained), 10 GB WSL2 RAM, single
ext4-in-VHDX disk, serving ~10 concurrent agent lanes through the width-1
`with-build-slot.sh` mutex. All runs took the build slot.

## Which script produced which number

| script | what it measured | headline |
|---|---|---|
| `build-exp.sh` | baseline cold `cargo build --workspace --all-targets` (config A) | **4189 s (69m23s)** — the outlier |
| `cold-c.sh` | mold + line-tables-only + sccache, cold cache then warm | **186 s** then **96 s** (99.4% hits) |
| `aprime.sh` | baseline CONTROL, same window as `cold-c.sh` | **189 s** — this is what killed the mold hypothesis |
| `incr-exp.sh` | edit-rebuild loop, sccache + `CARGO_INCREMENTAL=0` | geom-core **91/89 s**, topo **74/88 s** |
| `incr-exp-e.sh` | edit-rebuild loop, incremental + no sccache | geom-core **18/19 s**, topo **10/12 s** |
| `sccache-exp.sh` | **NEVER RAN** — superseded by `cold-c.sh` | no data; kept only to show what was planned |

`*.log` are the timing summaries; `*.log.raw` are the full cargo transcripts
(the `Finished ... in` lines and `Compiling` counts cited in the report).

## Reading these honestly

* **`build-exp.sh`'s 4189 s is not a baseline.** `aprime.sh` ran the SAME
  config on the SAME tree ~6.5 h later in 189 s. The 22x gap is
  environmental and is the report's §1 — the largest unexplained term, and
  the reason a control run existed at all. Do not quote 4189 s as "the cost
  of full debuginfo".
* **`build-exp.sh` was killed partway.** Only its `A cold` row completed;
  the run was aborted to release the exclusive slot, so its later phases
  produced no data.
* **`incr-exp.sh`'s config D rows are all `rc=101`.** That is the finding,
  not a failure of the script: sccache hard-refuses to run with
  `CARGO_INCREMENTAL` set, which is what proved the two are mutually
  exclusive. `incr-exp-e.sh` exists because of it.
* **`incr-exp-e.sh`'s first run also failed** (`rc=101`): `unset
  RUSTC_WRAPPER` does not override a *config-file* `rustc-wrapper`.
  `CARGO_BUILD_RUSTC_WRAPPER=""` does. Only the relaunch has data.
* Cold-build rows across scripts are **not** mutually comparable once
  sccache is in play — a warm shared cache from a previous run makes a later
  "cold" build fast. Only compare rows the report explicitly pairs.

## Re-running

These take the exclusive build slot and block every other lane. A cold
workspace build here is minutes at best and over an hour at worst, so scope
deliberately and announce it before starting. Each script writes
`../<name>.log` and `../<name>.log.raw` relative to the repo checkout, i.e.
into the lane directory above it.
