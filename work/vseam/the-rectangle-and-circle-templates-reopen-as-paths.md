---
id: the-rectangle-and-circle-templates-reopen-as-paths
kind: issue
title: A profile authored from the rectangle or circle template reopens in the editor as a path, not as the template
status: open
opened: 2026-09-19
priority: P0
cost: D
---


## What happens

The add-profile form offers three shapes (`forms::ShapeKind`): circle
(with an optional bore), rectangle, and path. The edit door
(`ViewerBehavior::edit_profile_ui`, PR #2862) loads every committed
profile as PATH steps (`sketch::held_loops`), because a committed
program records the lowered steps and not which template made them. A
rectangle therefore reopens as `at` + four `line_to`s, and a circle as
one `circle` step: the numbers round-trip exactly
(`tests/profile_edit.rs`, `every_authored_profile_round_trips…`), but
the interface the person authored with — width and height, centre,
radius and bore — is not the one they edit in. "The same interface"
holds for the Path shape only.

## What would close it

Either record the template on the node (a document change: the
program would carry its authoring template, which is DESIGN territory),
or recognise a template's shape on load (a rectangle's four corners
centred on the origin, axis-aligned; a pair of concentric circles) and
offer the template's fields when it matches, falling back to the path
steps when it does not. The second is viewer-only but is a guess about
intent; the first is the honest one.
