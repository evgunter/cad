# GUARD — the gates and the guards they lack (plan)

**STATUS: OPEN (2026-09-11).** Opened in the tracker cut of 2026-09-11
(`docs/WORK-TRACKS-2026-09.md` addendum 3). Live state is
`work/guard/log.md`'s tail and the item files beside this plan, never
this file.

Branch prefix (the #396 convention): **`guard/`** — unit branches
`guard/<unit>-<slug>`. The retired prefix `gates/` names the closed
GATES program's branches and is not reused. Away-channel tag
`(GUARD orchestrator)`. A/B ordinal band **GUARD = 3600–3699**, claimed
in `docs/MODEL-AB-LOG.md` in the opening commit.

## Charter

`scripts/gates/*` is the repository's hand-written static-analysis
layer, and every row here is about the same thing from a different
angle: **a gate that cannot see what it claims to check.** The shared
Rust reader splits an array type at its semicolon; the selftest
affordance can assert that a gate fires but not that it fired on the
identity the gate names; the roster and the probe census — the two
scripts Track K's fence deliberately excluded — have no reader guards at
all; the bounds allowlist is one of three homes for the same list; and
the measurement discipline itself has no mechanical guard outside
`crates/*/src`.

GATES closed on 2026-09-08 with this ground unfinished and two rows
re-homed to code-quality (`D212`, riding `G4`). This program is the
successor on that ground and takes them back.

## Territory and the seams

Territory: **`scripts/gates/*` whole**, which no live program's `paths`
covered at this cut. Four seams are stated rather than discovered, and
each is in `program.md`'s `keep_out`:

- **The Rust side of a shell-side fix** — `crates/geom-core/tests/bounds_census.rs`
  and `crates/test-utils/src/source.rs` are S-TCOST's and S-TINT's. The
  bound-list row lands its shell half here and files the Rust half on the
  owner; it does not edit across.
- **`tools/*` is INSTR's.** `S115` carries four unrelated members and two
  of them are instrument members; those route to INSTR and are struck
  from `S115` here when they land there.
- **The parity fence** is a `keep_out` clause in CIW's or GATES' old
  `program.md`, and the row that asks for it WRITES PROSE — it edits no
  workflow and no script.
- **`G4`'s ~49 `crates/profile` sites** are S-BOOL's glob, edited by
  announced seam, which `G4`'s own body already says.

## The slate

| item | class | what it is | where the work lands |
| --- | --- | --- | --- |
| `gate-rust-reader-splits-an-array-type-at-its-semicolon` | **M** | New `--items` mode with bracket depth and `<>`/item anchoring; shared reader, many consumers | `scripts/gates/lib.sh:318-324` (`gate_rust_code`), `scripts/gates/viewer-vocab-declared-once.sh:212` workaround reader, 17 gate consumers |
| `gate-selftest-cannot-observe-the-identity-a-gate-names` | **M** | New `want` affordance to assert file:line+identifier, then a one-line convention per gate. | `scripts/gates/lib.sh` (`gate_selftest_case` ~:1798-1829), selftest case lists in `scripts/gates/*.sh` (22 gates) |
| `gate-roster-and-probe-census-have-no-reader-guards` | **M** | Fix shape precedented by PR 2282, but owes per-stage negative controls and a directory census. | `scripts/gates/gate-roster.sh`, `scripts/gates/probe-suite-census.sh`, `scripts/gates/lib.sh`, sweep over `scripts/gates/*` (24 files) |
| `gate-wiring-fence-is-undrawn-for-the-parity-entry` | **E** | One clause extended in one program.md; the argument is already written out. | `work/ciw/program.md` or `work/gates/program.md` (one `keep_out` clause); names `.github/workflows/*`, `local-scripts/*`, `scripts/check-ci-mirror-parity.py` |
| `bound-list-readers-have-three-homes` | **M** | Cross-citation is cheap, but the real fix is a shell-vs-Rust shared-reader design call | `scripts/gates/bounds-allowlist.sh`, `crates/geom-core/tests/bounds_census.rs`, `crates/test-utils/src/source.rs` |
| `D212` | **M** | Three mechanical deletions in one script, but must land inside Track V's G4 diff | `scripts/gates/bounds-allowlist.sh` (GAP 3 text, `BOUNDS_ALIAS_ROSTER` entry, `arc_fillet.rs` filter, `plant_alias_uses_invisible` fixture) |
| `G4` | **M** | Collapse proven mechanical by PR 1453; gate entry, roster and breaking re-export remain | `crates/profile/src/path/arc_fillet.rs` + ~49 `profile` sites, `crates/pncad/src/lib.rs` re-export, `scripts/gates/bounds-allowlist.sh`, `BOUNDS_ALIAS_ROSTER` census |
| `S41` | **H** | Three unlinted disciplines across tracks M/R plus a gate matcher that must reach certified doors. | `crates/geom-core/src/ring_interval.rs` (`clamped_to`), `crates/geom-brep/src/props/quad.rs` (`sqrt_enclosure`), `crates/bvh/src/tree.rs:209-210`, `scripts/gates/bounds-allowlist.sh` |
| `S210` | **H** | Spans four tracks plus unowned bvh; real option is a rustc/RA driver; orchestrator's to place | `scripts/gates/bounds-allowlist.sh`, `crates/geom-core/tests/bounds_census.rs`, `crates/{profile,geom-brep,mesh,bvh}/src/**`, `Enclosure` (#701) |
| `S115` | **H** | Four unrelated sub-items; schema change, re-cut baseline, new CI census, several territories | `tools/tess-lint/src/lib.rs`, `tools/tess-meter/src/lib.rs` (+ re-cut baselines), `scripts/doc-gate.sh`, `.github/workflows/ci.yml` (import-census row), `crates/pncad/src/prelude.rs`, `crates/topo/src/euler.rs` |
| `measurements-have-no-mechanical-guard` | **H** | Cross-cutting sweep over every surface the instrument cannot see; instrument must be re-cut per surface | remainder outside `crates/*/src`: `crates/*/tests/*`, `tools/*`, `scripts/*`, `demos/*`, `benches/*`, `docs/*`, `.github/workflows/*`, `Cargo.toml`, `crates/pncad-py/**`, `interval-transcendentals/src/*` |

## Order

The shared reader first (`gate-rust-reader-…`), because seventeen gates
call `gate_rust_code` and every later row reads better against a reader
that parses items correctly. Then the selftest affordance, which is what
makes the rest of the slate checkable. Then the two ungated censuses.
`gate-wiring-fence-…` is a one-clause prose row and can go at any point.

**Two parks to resolve before anything else is dispatched.** `G4` is
parked on #1647, which merged, and `D212` rides `G4`; the first act of
this program is to verify that park and open or re-park it. Nothing here
depends on the answer except `D212`.

`S41`, `S210` and `measurements-have-no-mechanical-guard` are the H tail
and each widens what a gate can see rather than fixing one; `S210`'s own
body says the real option is a rustc or rust-analyzer driver, which is a
design PR before it is a unit.

## Review posture

Infra-only, the CIW/S-TCOST posture: one style review per unit against
`docs/prompts/reviewer-style-lane.md`, no A/B row, the band claimed for
bookkeeping. A unit that changes what a gate ACCEPTS (as against what it
reports) takes a second correctness reviewer, because a widened gate that
is wrong is silent.

## How the class column is read

`E` / `M` / `H` is a **dispatch estimate**, made on 2026-09-11 by reading
each row against the tree, and it is the axis this program's order runs
on. It is not a verdict on the finding and it is not in any header: no
field carries it, `work.py` does not parse it, and this table is the only
place it lives. A lane that finds the estimate wrong says so in its PR
and this table is corrected in the same PR.

- **E** — the fix is written in the row or obvious from it: one or a few
  files, no design question, no ruling, small diff.
- **M** — multi-file, or a small design call (where a shared home lives,
  what a door looks like), or a census or instrument to build first.
- **H** — cross-cutting, numeric or algorithmic, gated on a ruling, or
  spanning several programs' territory.

The cut that opened this program is `docs/WORK-TRACKS-2026-09.md`
addendum 3; it is a survey, and this plan supersedes it as the charter.
