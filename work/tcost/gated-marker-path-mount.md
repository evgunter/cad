---
id: gated-marker-path-mount
kind: issue
title: a #[path]-mounted src test module derives a marker term that matches nothing, silently
status: closed
opened: 2026-09-03
closed: 2026-09-11
---


`scripts/ci-filter.py`'s `_suite_term` derives a `src/` marker's nextest
prefix from the FILE PATH — "the module path IS the file path"
(`scripts/ci-filter.py:1315`). That is false for a `#[cfg(test)]` module
mounted with `#[path = "…"]` from a sibling file, and the tree has three
such modules today:

| file | mounted from | real test-id prefix | prefix `_suite_term` would derive |
|---|---|---|---|
| `crates/topo/src/boolean/r1_probes.rs` | `crates/topo/src/boolean/solid_contain.rs:3491` | `boolean::solid_contain::r1_probes::` | `boolean::r1_probes::` |
| `crates/topo/src/boolean/torus_predicate_rows.rs` | `crates/topo/src/boolean/solid_contain.rs:3498` | `boolean::solid_contain::torus_predicate_rows::` | `boolean::torus_predicate_rows::` |
| `crates/topo/src/chart_region_r2_probes.rs` | `crates/topo/src/chart_region.rs:3400` | `chart_region::chart_region_r2_probes::` | `chart_region_r2_probes::` |

None of the three carries a marker today, so nothing is broken in the
tree. What is broken is the GUARD. A term that matches no test excludes
no test, so a marker on one of these files would leave its suite running
on every pull request while reading, in the file, as a gate — and
`scripts/gates/gated-suite-paths.sh` cannot see it: `--gated-check`
(`scripts/ci-filter.py:1489`) asks only whether the marked file contains
`#[test]` or `#[cfg(test)]`, never whether the derived prefix selects
anything. This is precisely the failure mode that gate's own header
argues it exists to make loud ("a marker sited where nothing reads it, a
file marked but holding no test … fail here too").

It is also the one direction `--gated-set` cannot catch. The nightly
re-take runs what it derives; a term matching nothing quietly shrinks the
re-take instead of reddening it, so the suite is neither gated on a PR
nor re-taken at night while the tree reports two green gates.

TCOST-9 hit this while gating TCOST-4's torus counterexample-search row,
which lives in `crates/topo/src/boolean/r1_probes.rs`. The workaround
there was to put the gated row in a file whose PATH matches its module
path (`crates/topo/src/boolean/solid_contain/r1_generic_poses.rs`, a
plain `mod` and no `#[path]`), which is correct for that row but is not
a fix for the class.

Two candidate fixes, both cheap and neither this unit's to choose:

1. **Resolve the mount.** `_all_rs_modules` already reads `#[path]`/`mod`
   pairs out of `tests/all.rs`; the same reader over `crates/<c>/src/**`
   would let `_suite_term` follow a `#[path]` mount to the module path
   the compiler actually gives the file.
2. **Make the silence loud.** Have `--gated-check` refuse a marker on any
   `src/` file that some other `src/` file mounts with `#[path]`, naming
   the mounting line — a two-line scan that costs no toolchain and turns
   the whole class into a red discipline row.

The sweep behind the table: `grep -rn '#\[path = "' crates/*/src
--include=*.rs`, seven hits — three prose mentions in
`crates/test-utils/src/source.rs`, the three above, and
`crates/mesh/src/lib.rs:285`, which mounts `../tests/common/witness_bodies.rs`
into the LIB. That last one is already loud rather than silent: a marker
there is scanned under the `tests/` shape, `crates/mesh/tests/all.rs`
declares no such module, and `--gated-check` reds. The pattern cannot
match a mount whose `#[path]` is written on a different line from the
`mod`, or one assembled by a macro; I found none of either.

## Re-homed (2026-09-04)

Moved from `work/issues/` to `work/tcost/` in the tracker-wide
re-home sweep of 2026-09-04 (Ev's direction, in-chat), which read every
open `work/issues/` file against every open program's `paths` and
against the code-quality K–X fences. Id, body and header are unchanged;
the directory is the claim (`work/README.md`). Any `## Home` section
above naming `work/issues/` is superseded by this line and is kept as
the record of why the file was parked there.

## Sized (2026-09-11), while closing its sibling

Sized against the code, not estimated: the sibling row
(`gated-marker-omits-sibling-helper-imports`) landed its arm in
`--gated-check` on 2026-09-11 and built the reader this one would use, so
the two candidate fixes can now be priced properly. **They are not the
same size, and they do not do the same thing** — the choice is Ev's, not
a lane's.

**Candidate 2, "make the silence loud": small, ~15 lines, no new
concept.** In `gated_check`, walk `crates/*/src/**` for
`#[path = "..."]` / `mod` pairs the way `_tests_sibling_files` now walks
`tests/all.rs`, resolve each mount's target, and red on a marker sited on
any file that another `src/` file mounts — naming the mounting line. The
regexes exist (`_ALL_RS_PATH_RE`, `_ALL_RS_MOD_RE`), the walk exists, and
the planter harness in `scripts/gates/gated-suite-paths.sh` takes one
more case the way it just took two. **What it buys**: the class becomes
a red discipline row instead of a silent one. **What it costs**: the
three files stay ungatable, so a row that belongs in one of them is
written where its path matches its module path instead — which is what
TCOST-9 did by hand for the torus counterexample-search row, and which
the row's own note calls correct for that row and not a fix for the
class.

**Candidate 1, "resolve the mount": larger, and it is the one with a
design question in it.** `_suite_term` would follow a `#[path]` mount to
the module path the compiler actually gives the file. The mechanical part
is comparable in size; what is not mechanical is that a `src/` mount can
nest (a mounted file may itself mount another) and that the mount's
module path depends on WHERE the mounting `mod` sits in its own file's
module tree — `solid_contain.rs:3491` mounts into
`boolean::solid_contain`, which the current reader gets right only
because the mounting file's path happens to name it. A reader that
resolves one level and calls it done would derive a wrong prefix on a
nested mount and go back to selecting nothing, silently: the same failure
this row is about, one level deeper. **What it buys**: markers work
everywhere, and the three files become gatable. **What it costs**: a
module-tree walk of `crates/*/src`, which is a real reader with real
edge cases, for three files today.

**Recommendation, unchanged from the row's own text (*"neither this
unit's to choose"*)**: take candidate 2 now — it is cheap, it converts
the whole class from silent to loud, and it is not thrown away if
candidate 1 ever lands. Take candidate 1 only when a row genuinely wants
gating in a mounted file and the workaround is worse than the reader.

Still true on the tree at 2026-09-11: none of the three files carries a
marker, so nothing is broken today. The guard is.

## Closed (2026-09-11): candidate 2 landed — the silence is loud

Taken on Ev's direction (in chat, 2026-09-11: *"can you do the cheap
gated marker path fix?"*), as the sizing above recommended.
`--gated-check` now refuses a marker on any `crates/*/src` file that
another `src` file `#[path]`-mounts, naming the mounting file and line:

```
crates/topo/src/boolean/r1_probes.rs: carries a marker and is
`#[path]`-mounted from crates/topo/src/boolean/solid_contain.rs:3491, so
its module path is the MOUNTING module's and the term derived from this
file's path selects no test. Move the gated rows to a file whose path
matches its module path, and mark that
```

Candidate 1 is NOT taken and the sizing above stands as the record of
why: resolving the mount has to be right, refusing is merely exact, and a
one-level resolver would derive a wrong prefix on a nested mount and go
back to selecting nothing — this defect one level deeper. The workaround
the refusal points at is the one TCOST-9 already used by hand for the
torus counterexample-search row.

**The census was re-derived rather than trusted, and it had grown.** This
file named three mounts on 2026-09-03. At `76d4bb0d` there are
**seven**:

| mounted file | mounted from |
|---|---|
| `crates/geom-core/src/sym/algebra.rs` | `crates/geom-core/src/sym.rs:388` |
| `crates/geom-core/src/sym/report.rs` | `crates/geom-core/src/sym.rs:392` |
| `crates/geom-core/src/sym/signed.rs` | `crates/geom-core/src/sym.rs:396` |
| `crates/mesh/tests/common/witness_bodies.rs` | `crates/mesh/src/lib.rs:288` |
| `crates/topo/src/boolean/r1_probes.rs` | `crates/topo/src/boolean/solid_contain.rs:3491` |
| `crates/topo/src/boolean/torus_predicate_rows.rs` | `crates/topo/src/boolean/solid_contain.rs:3498` |
| `crates/topo/src/chart_region_r2_probes.rs` | `crates/topo/src/chart_region.rs:3524` |

The original three are all still there (the `chart_region` line moved
3400 -> 3524). The three `geom-core/src/sym/` mounts are NEW since this
row was written — the same regrowth its sibling row measured on the
helper-import class, and the reason both rows wanted a mechanical check
rather than a sweep. **None of the seven carries a marker**, so nothing
was broken in the tree and nothing is fixed in it: the guard is what
changed, and the tree is green under it from the first run.

**Two shapes the reader had to get right, both live rather than
hypothetical:**

- **Intervening attributes.** `crates/mesh/src/lib.rs:288` writes
  `#[cfg(test)]`, `#[path = "..."]`, `#[allow(dead_code, unreachable_pub)]`,
  then `mod`. A reader requiring the `#[path]` and the `mod` to be
  adjacent misses it and reports a clean tree — the first draft of this
  reader did exactly that, and the census above is what caught it.
- **The mount target need not be under `src/`.** That same mesh mount
  reaches `crates/mesh/tests/common/witness_bodies.rs`, a `tests/` file
  pulled into the LIB. It is in the refused set too, which is stricter
  than this row's own note (which reasoned that a marker there would red
  under the aggregation arm instead). Reaching it by this arm names the
  mounting line, which is the fact an author needs.

Planted both directions in `scripts/gates/gated-suite-paths.sh` (the
announced cross-fence edit, as its sibling): `plant_marker_on_mounted_file`
must red — its marker is otherwise VALID, so a red can only be this arm,
and its fixture carries the `#[allow]` between the two lines — and
`plant_mounted_file_unmarked` must pass, which is the live tree's own
state and the case a gate that fired on it would red `main` over.
