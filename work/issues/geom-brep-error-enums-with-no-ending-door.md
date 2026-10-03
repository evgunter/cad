---
id: geom-brep-error-enums-with-no-ending-door
kind: issue
title: geom-brep: six error enums have neither an ending door nor an inline recourse on most arms; whether each reaches a surface that shows it is untraced
status: open
opened: 2026-10-02
---

(SSI implementer, from the second pass of the §5 sweep in PR "SSI: every
SsiError arm ends in the recourse its decision earns". The first pass
looked for `ending` doors that return `None`. That pattern cannot see an
enum with no ending door at all, so the second pass counted, for every
`Display for *Error|*Refusal` in `crates/geom-brep/src`, the arms whose
text carries no recourse marker: `Recourse`, an `*_ENDING` or
`*_RECOURSE` constant, "no way through", or a call to `render`/`ending`.
Several owners, so this is filed here.)

## What

These enums have no ending door, and most of their arms render no
recourse:

| enum | file | arms with no marker | owner |
|---|---|---|---|
| `SectionError` | `intersect.rs` | 9 of 11 | germ, reach, tang |
| `PropsError` | `props/mod.rs` | 7 of 8 | — |
| `PcurveError` | `pcurve.rs` | 5 of 5 | — |
| `OffsetError` | `offset.rs` | 5 of 5 | — |
| `NewellError` | `newell.rs` | 3 of 3 | — |
| `IsoRowError` | `nurbs_iso.rs` | 3 of 4 | iso (by subject) |

**Not traced:** whether any of these reach a surface a person reads
unwrapped. A caller may wrap one and add its own ending, as topo does
for `CertifyError`, or the enum may only ever reach kernel developers.
The repair depends on the answer: an ending door like
`SsiError::ending`, or a note on the variant saying who ends it.
