---
id: solid-of-vertex-is-hand-spelled-twice-in-sweep-tests-beside-solidowners
kind: issue
title: solid_of_vertex is hand-spelled twice in sweep tests beside the public SolidOwners::vertex, and panics on a lone vertex
status: closed
opened: 2026-09-25
priority: P4
cost: E
refs: [two-spellings-of-the-face-to-solid-owner-index]
closed: 2026-09-26
pr: 3284
---


## Finding

- **Where**: `crates/sweep/tests/shell8_common.rs` (`solid_of_vertex`,
  ~:61, `pub(crate)`) and `crates/sweep/tests/shell8_r2_probes.rs`
  (`solid_of_vertex`, ~:66, a private copy). S-TCOST's and S-TINT's
  ground; dup announces by seam.
- **Confidence**: sure — both read.
- **Raised by**: the full review of PR 3151
  (`two-spellings-of-the-face-to-solid-owner-index`), 2026-09-25.

Both answer "which solid is this vertex in" by
`get_vertex(v).unwrap().emanating.unwrap()`, then half-edge → loop →
face → shell → solid. That is `topo::SolidOwners::vertex`, public,
spelled by hand twice, and it **panics on a lone vertex** (`emanating`
is `None` exactly there) where the door answers. Callers at
`a3a623755`: `shell8_multi_solid.rs` ×2 (through the `pub(crate)`
copy) and `shell8_r2_probes.rs` ×2 (through its own).

**The fix is small**: delete both, and read `SolidOwners::of(&body)
.vertex(v)` at the four call sites (built once per body where a site
asks more than once). Whether a site wants the panic — its body has no
lone vertex by construction, and a `None` there is a broken fixture —
is an `.expect` at the site, not a reason to keep a walk.

**What found it, and what that could not see.** PR 3151's own X4
needle (`insert(he.start|half.start|h.start` across `crates/`) looked
for owner INDICES built from half-edge starts; a per-vertex walk that
starts from `Vertex::emanating` contains no such insert and is
invisible to it. The shape to census is the `emanating` read followed
by a climb to `.solid`: `git grep -n 'emanating'` over `crates/*/tests`
finds a dozen other `.emanating` reads (`mesh`, `sweep` fillet, ladder
and review probes) that were not read past the line, so whether any
of them also climbs to a solid is open.

## Closed (2026-09-26, PR #3284)

Both copies are gone. The three call sites (`shell8_multi_solid` ×1,
`shell8_r2_probes` ×2 — the row's "×2" in `shell8_multi_solid` is one
call and one import) read `topo::SolidOwners::of(&body).vertex(v)`,
built once per body, with an `.expect` at the site: a vertex the map
does not place is a broken fixture there, and says so.

**The blind spot, aimed at twice.**

- *Pass 2, the row's own shape*: every `emanating` read in the tree
  outside `crates/topo/src` (`git grep`, no path argument, at the merge
  base `0c1932667`: 35 in `crates/*/tests` — `sweep` 23, `topo` 11,
  `mesh` 1 — three in `sweep/src/blend`, the rest `pncad-py` error
  names and a gate script; none in `demos/` or any `examples/`), each
  read past its line. Two more members, neither a function: the vertex arm of
  `shell8_common::deep_dump` and of its private twin in
  `shell8_r1_probes`, which asked "is this vertex in this solid" as
  "is its emanating half-edge's face in the solid's face list" — the
  same climb, and it skipped a lone vertex silently. Both now read
  `SolidOwners::vertex`, and the twin itself is folded:
  `shell8_r1_probes` imports `shell8_common::deep_dump` (the two
  differed only in how a vertex's bits are printed, and each dump is
  compared only with itself). Every other hit reads a different walk —
  the edges or faces meeting a vertex, a `mev_null` anchor, orbit
  valence, a vertex's SHELL (`topo` `m3_pr3_split` ~:300, shell
  granularity, which `SolidOwners` does not answer) — dispositioned
  one by one in
  `work/dup/the-edges-at-a-vertex-are-spelled-per-sweep-suite.md`,
  filed for the class that pass found.
- *Pass 3, denominator-first*: every `.solid` field read in
  `crates/*/tests`, `demos/` and `examples/` (48), classified backwards
  by its receiver. No vertex-rooted receiver outside the three
  `solid_of` bodies in the `shell8` trio, whose face → solid walk is
  `work/helper/the-face-to-solid-walk-is-spelled-per-test-file.md`'s.
  Blind spot: a climb written through a door that is not a field read
  (none found under `crates/*/tests` with `git grep 'solid_of_face\|SolidOwners'`).

**Plants** (lane harness, byte-exact restore, `git diff HEAD` clean
after each; filter `test(/^shell(8|10)_/)`, 44 rows):

| plant | reds | per site |
| --- | --- | --- |
| `SolidOwners::vertex` panics at a folded caller (`#[track_caller]`) | 13 / 44 | `shell8_common` (`deep_dump`) 9 rows, `shell8_multi_solid` 1, `shell8_r2_probes` `bitwise_solid` 2, `shell8_r2_probes` ~:482 1 |
| every placed vertex answers the MAX solid key | 11 / 44 | — |
| every placed vertex answers the MIN solid key | 7 / 44 | — |

The two directions together red 12 of the 13 reached rows; the
thirteenth, `shell10_r2_dump::shell10_r2_dump_corpus`, is a print-only
differential instrument with no assertion by design, so no answer can
red it.
