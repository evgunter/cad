---
id: the-mate-tools-refusal-names-the-mate-tool-twice
kind: issue
title: the mate tool's missing-picks refusal reads "mate tool: the mate tool needs two face picks"
status: open
opened: 2026-09-30
priority: P4
cost: E
---


## Finding

Found by AUTH-12's sweep for string literals that spell a tool's name
outside `ToolKind::label` (`crates/viewer/src/tools.rs`).

`MateToolError::NotTwoPicks` displays as `"the mate tool needs two
face picks"` (`impl Display for MateToolError`,
`crates/viewer/src/matetool.rs`). `MateTool::proposal` returns it when
the tool holds fewer than two picks. `mate_tool_ui`
(`crates/viewer/src/pane/create.rs`) says every proposal refusal
through `ToolKind::Mate.says(&error)`, which prefixes the tool's name.
So clicking `Commit mate` before both faces are picked puts this on
the status line:

> mate tool: the mate tool needs two face picks

Every seated tool's equivalent refusal reads without the stutter
(`SeatError::Empty`, `crates/viewer/src/seats.rs`: `"no {seat} picked
yet"`, which reads as, e.g., "boolean tool: no first operand picked
yet"). The fix is the wording of the one `Display` arm, for example
"needs two face picks" or "no face picked for pick a / pick b yet" to
match the seats. `crates/viewer/tests/mate_tool_flow.rs` matches the
variant, not its text, so no row asserts the words.
