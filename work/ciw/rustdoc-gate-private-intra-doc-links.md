---
id: rustdoc-gate-private-intra-doc-links
kind: issue
title: Revisit — should the rustdoc gate reinstate private_intra_doc_links?
status: open
opened: 2026-08-15
github: 519
refs: [465]
---

## From GitHub issue 519

Opened 2026-08-15; 0 comments.

Banked from #465's chunk 0. The rustdoc gate (`scripts/doc-gate.sh`) runs
with:

```
RUSTDOCFLAGS="-D warnings -A rustdoc::private_intra_doc_links"
```

This issue is only about that `-A`. The gate itself, and the per-crate
cleanup ratchet, live in #465.

## What the lint says, and why it fires here

`private_intra_doc_links` fires when a **public** doc comment links to a
**private** item:

```
warning: public documentation for `write_ascii` links to private item `NAME`
  = note: this link resolves only because you passed `--document-private-items`,
          but will break without
```

The gate does always pass `--document-private-items`, deliberately —
much of the load-bearing prose in this repo sits on private functions
(`span_offset`, `span_indices`, `frame_from_unit_aim`), and without the
flag those are neither rendered nor checked, which was half of #465's
argument. So in the docs this repo actually builds, every one of these
links resolves. The lint is warning about a doc set we do not produce.

## The measurement

**The table below is from 2026-08-15 and has decayed by ~3.4x. The
current figure is in `## The measurement, re-derived 2026-09-10` at the
foot of this file; option 1 is unaffected, but the 82 is not the
population and must not be quoted as one.**

Workspace-wide, `cargo doc --no-deps --document-private-items`:

| class | count |
|---|---|
| `private_intra_doc_links` | 82 |
| `broken_intra_doc_links` (unresolved) | 58 |
| `X is both a function and a module` (ambiguous) | 11 |
| redundant explicit link target | 6 |
| **total** | **157** |

Allowing the one lint takes the real backlog from 157 to 75 — and the 75
are all genuine breakage, which is what makes the per-crate ratchet in
#465 a tractable size.

## The question to settle later

Three options, roughly:

1. **Keep it allowed** (status quo). One doc set, private items
   rendered, links resolve. Cost: a consumer reading docs.rs-style
   public-only output would see one dead link per site in the
   population — 82 when this was written, 278 on 2026-09-10 — but
   nothing publishes such a set today, and the crate is private.
2. **Reinstate it and fix them all.** Means rewriting public prose so it
   stops linking to the private helpers it is explaining, or promoting
   those helpers. Both look like losses for prose whose whole value is
   the invariant argument.
3. **Reinstate it and render two doc sets** — public-only (lint on) and
   private-inclusive (lint off). Honest, and the only option that gets
   both properties, at the cost of a second rustdoc invocation in CI and
   a decision about which set is canonical.

Trigger to revisit: the day anything publishes a public-only doc set, or
Q9 lands and the crate becomes public-facing — whichever comes first.
Until then option 1 is the measured choice, recorded here rather than
left implicit in a flag.

🤖 Generated with [Claude Code](https://claude.com/claude-code)

## Home

`work/issues/`: `scripts/doc-gate.sh` is S-QA's territory and S-QA is closed; its exit walk names the doc-gate's remaining axes as standing residue rather than another program's ground.


## The measurement, re-derived 2026-09-10 (CIW unit 7, `ciw/citations`)

The 82 above, and the "157 -> 75" arithmetic that justified option 1,
are from 2026-08-15. Re-derived on `origin/main` at `c5558def5`:

```
$ RUSTDOCFLAGS="-A rustdoc::broken_intra_doc_links" \
    cargo doc --workspace --no-deps --document-private-items
```

**278 `private_intra_doc_links` sites across 11 crates**, default
features — a ~3.4x decay in 26 days:

| crate | n | | crate | n |
| --- | --- | --- | --- | --- |
| `topo` | 63 | | `geom` | 11 |
| `editor-core` | 50 | | `step-import` | 8 |
| `geom-brep` | 40 | | `mesh` | 6 |
| `viewer` | 35 | | `profile` | 5 |
| `geom-core` | 30 | | `bvh` | 3 |
| `sweep` | 27 | | | |

Two things the 2026-08-15 reading did not pin, and both matter for the
next re-derivation:

- **Feature selection.** The command above and the one above it take
  DEFAULT features. The gate documents at `--all-features`, and the same
  warn-only run with `--all-features` gives **292 across 12 crates**
  (`pncad-py` joins with 1; `topo` 64, `editor-core` 55, `viewer` 41,
  `sweep` 28, the rest unchanged). 278 is the like-for-like successor to
  82; 292 is the population the gate's own selection would face.
- **Warn, never deny.** `-D warnings` aborts the workspace at the first
  crate that fails to document, so a deny run reports a floor and not a
  population.

Both figures count **reported sites** — one rustdoc warning each — not
distinct link spellings. `doc-gate-two-unread-axes` counts spellings in
its axis-(a) sweep; the two are not comparable.

**This does not overturn option 1.** Allowing the lint still removes the
whole class at once, and the argument for it is stronger at 278 than it
was at 82: option 2's cost scaled with the population and option 1's did
not. What is corrected is the presentation of 82 as the measured
population. The revisit trigger is unchanged.

Carried in from `rustdoc-d-warnings-breakages-outside-the-doc-gate`,
closed the same day: its seven surviving sites (four in
`crates/topo/src/boolean/contain.rs`, three in `.../rest.rs`, two in
`crates/editor-core/src/{eval/mod.rs,node.rs}` — the corrected table is
in that item) are members of this population, reported by nothing today
because of the `-A` at `scripts/doc-gate.sh:555`. That item listed six of
them and missed `rest.rs:493` and `:540`, which is what writing against
the instance costs.
