---
id: solid-of-vertex-is-hand-spelled-twice-in-sweep-tests-beside-solidowners
kind: issue
title: solid_of_vertex is hand-spelled twice in sweep tests beside the public SolidOwners::vertex, and panics on a lone vertex
status: open
opened: 2026-09-25
priority: P4
cost: E
refs: [two-spellings-of-the-face-to-solid-owner-index]
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
