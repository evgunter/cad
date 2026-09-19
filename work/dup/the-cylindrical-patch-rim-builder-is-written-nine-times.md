---
id: the-cylindrical-patch-rim-builder-is-written-nine-times
kind: issue
title: The two-rim cylindrical-patch builder is written nine times across seven files, all as closures a name census cannot see
status: open
opened: 2026-09-19
---


## Finding

- **Where**: nine spellings of one builder, in seven files —
  `crates/topo/src/boolean/boxes.rs` (`rim` `:2285`, `meridian`
  `:3082`), `crates/topo/src/census.rs` (`rim` `:4023`, `rim` `:4531`),
  `crates/topo/src/chart_region.rs` (`rim` `:4371`, inside
  `fn cyl_sheet`), `crates/topo/tests/mate5_cyl_eps_rung.rs` (`rim`
  `:122`), `crates/topo/tests/r1_mate5_probe.rs` (`rim` `:75`),
  `crates/topo/tests/r2_probes.rs` (`rim` `:69`),
  `crates/topo/tests/split_edge_pcurve_rows.rs` (`rim` `:68`).
- **Importance**: medium — nine copies is the largest single count any
  S-DUP row has opened with
- **Confidence**: sure that the four `tests/` copies and the
  `census.rs`/`chart_region.rs` pair are the same construction (their
  closing `mef` blocks are token-identical); the two `boxes.rs`
  spellings were read, not diffed
- **Raised by**: the PR 2843 fix pass, 2026-09-19, out of the
  `find_half_edge(seed.face` sweep run for
  `the-cube-sequence-is-written-five-times-and-twice-inside-src`. Nine
  of that sweep's 26 hits are these. They are not cubes and not quad
  sheets, so they are their own class.

## The shape

A cylindrical patch grown as two rims and a closing wall: a `rim`
closure that seeds or walks a circular arc at one parameter, a second
rim at the other, and then

```
let he = body.find_half_edge(seed.face, e_t.vertex, e_r.vertex).unwrap();
body.mef(
    MefSite::Chords { he1: he, he2: e_b.he_plus },
    EdgeCurveSpec::line_between(p01, p00),
    FaceSurface::Shared(cyl),
    ...,
)
```

That `mef` block is identical, token for token, across
`mate5_cyl_eps_rung.rs`, `r1_mate5_probe.rs`, `r2_probes.rs`,
`split_edge_pcurve_rows.rs`, `census.rs` (both) and
`chart_region.rs` — seven of the nine — differing only in
`Tol::witness()` against a local `tol()`.

## Why no census in links 1–3 saw it

**Every one of the nine is a CLOSURE**, `let rim = |body, v, ccw| { … }`,
except `chart_region.rs`'s, which is a `fn cyl_sheet`. So:

- the **name census** keys on declarations, and a closure has no
  declaration to key on;
- the **arity census** counts call sites and these loop, which is the
  blind spot that row already names;
- the **geometry census** (`mvfs(` with `newell_plane`) misses them
  because the surface here is a cylinder, not a Newell plane.

The instrument that finds them is the structural one —
`git grep -n 'find_half_edge(seed.face'` — and this row is the second
piece of evidence that a structural needle catches what three
name/arity/import censuses do not.

## What is unmeasured

- Whether the nine build bit-identical bodies at equal arguments. **Not
  dumped.** The `tests/` four look parameterised identically; the
  `boxes.rs` `meridian` spelling walks the other axis and may be a
  genuine sibling rather than a copy.
- Where the shared home would be. Four of the nine are in `tests/`,
  which is S-TCOST/S-TINT ground per this program's `keep_out`, and
  five are in `src/` — so the fold is the `mesh`-style two-sided one,
  not an in-file merge.
- Whether `FaceSurface::Shared(cyl)` means the callers must supply the
  surface, which would make the shared builder take it as a parameter
  the way `prism_ops` takes `FaceGeometry`.
