---
id: listing-a-solids-faces-is-spelled-four-times-in-topo-src
kind: issue
title: Listing a solid's faces is spelled four times in topo/src, three of them named faces_of
status: open
opened: 2026-09-19
---


## Finding

- **Where**: `crates/topo/src/shell10_r2_probes.rs` (`faces_of`, ~:39),
  `crates/topo/src/offset_together.rs` (`faces_of`, ~:981),
  `crates/topo/src/boolean/solid_contain.rs` (`faces_of`, ~:3764) and
  the body of `crates/topo/src/boolean/solid_contain.rs`'s
  `SolidFaces::of` (~:2411) — which is `pub`, re-exported from
  `topo::lib`, and is the only one of the four with a guard.
- **Importance**: medium
- **Confidence**: sure. All four were read. The name census
  (`(fn|let)\s+(solid_of|faces_of|shell_of|owning_solid|solid_for)\w*`
  over every tracked file, no path argument) is what found the third
  `faces_of`; no structural regex in this program's censuses reached
  it, because its body is a call and not a walk.
- **Raised by**: the `solid_of_face` fold, 2026-09-19.

One job — *the faces of this solid, in face-arena order* — under three
functions of the same name plus a public door, with three different
answers to what happens when it cannot be done:

| site | body | refusal |
| --- | --- | --- |
| `shell10_r2_probes::faces_of` | filter on `solid_of_face` | panics |
| `offset_together::scope_walks::faces_of` | the same filter, byte for byte | panics |
| `boolean::solid_contain` tests' `faces_of` | `SolidFaces::of(..).unwrap()` | panics, one layer down |
| `SolidFaces::of` | the same filter, then a group-read guard | `PointInSolidError` |

`SolidFaces::of` is not a drop-in home for the other three: it carries
a second obligation (a group-read kind's surface key crossing the
selection boundary is refused typed) that a bare face list does not
want, and it walks the whole face arena twice. So the unit is a
decision about whether the plain list is a door in its own right —
`Body::faces_of_solid`, which `SolidFaces::of` would then filter — not
a re-point of three call sites at an existing name.

The first two bodies are byte-identical; their wider fixture family is
`work/dup/shell10-r2-probes-restates-the-scope-walk-fixtures-verbatim.md`.
