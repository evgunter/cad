---
id: maintenance-crosses-python-as-a-nine-attribute-union-class
kind: issue
title: Maintenance crosses Python as one class with nine attributes, seven of them None for a strand
status: open
opened: 2026-09-16
refs: [2753, payload-accessor-wildcards-remain-in-checks-and-assembly]
priority: P4
cost: E
---

(Found by the style review of PR 2753, which widened
`Applied::maintenance` from `Vec<ClusterMaintenance>` to
`Vec<Maintenance>` for DM7 and renamed the `pncad-py` class to match.)

## The finding

`crates/pncad-py/src/py/mate.rs`'s `Maintenance` pyclass now publishes
nine payload attributes over a two-arm sum: `survived`, `absorbed`,
`absorbed_frame`, `source`, `target`, `frame`, `gauge` carry only for
the four cluster acts, and `node`, `name` only for a strand. A caller
holding a row reads seven `None`s or two, and the class doc is what
says which.

This is the class
`payload-accessor-wildcards-remain-in-checks-and-assembly` and
`mate-fault-accessors-wildcard-into-silence` are about — **filed as a
new row because every row of that class on this slate is closed**, so
an append to one would be a disclosure nobody re-reads
(`work/README.md`: give a residue its own file at the moment you
disclose it).

## What is NOT wrong with it

Not the `_ => None` defect those rows name. The narrowing helper
`Maintenance::cluster` matches both arms of `Maintenance` with no
wildcard, and each of the seven cluster getters matches all four
`ClusterMaintenance` arms with no wildcard. **An arm added to either
enum fails the build**, which is exactly what LIB-PROJ bought for this
file and what this change preserves. `maintenance_tag` remains the one
exhaustive site the tag inventory pins.

## The question

Whether a widening sum should stay ONE Python class whose attribute
set is the union of its arms, or split (a `Strand` class beside a
`ClusterMaintenance` one, with `Doc.last_maintenance` returning the
union type). The kernel has one column and the façade mirroring it
one-for-one was PR 2753's call, taken as mechanical follow-through of
a kernel type change; the shape of LIB's own surface is LIB's. The
counter-argument to splitting is the stub: a heterogeneous
`list[Strand | ClusterMaintenance]` costs every caller a narrow, where
`variant` is the alphabet this surface already speaks everywhere else.

No action was taken in 2753 beyond the mechanical rename.

## Widened (2026-09-19, PR #2874)

A SEVENTH variant, `orphaned_declare`: the delete door's report that a
`Declare` lost its last consumer. It adds no attribute — and that is
the point for this row. `node` now answers a SECOND question: on a
`strand` it is the surviving CARRIER of a dangling name, on an
`orphaned_declare` it is the surviving DECLARATION itself, and only
`variant` says which. `name` is `None` on the new arm, because nothing
dangles.

So the union class's cost is no longer only "nine attributes, of which
each variant fills a few": one attribute now means two different things
depending on a sibling attribute's value, which is exactly the failure
mode a tagged union of dataclasses would not have. Whatever shape this
row settles on has to carry that, not just the attribute count.

Appended from outside LIB's fence, by announcement, to keep the row
true of the surface it describes.

## Narrowed (2026-10-01, PR #3676)

EDIT P2-core deleted the cluster acts with the placement registry, and
their seven getters (`survived`, `absorbed`, `absorbed_frame`, `source`,
`target`, `frame`, `gauge`) with them. The class now carries four
variants — `offset_cleared`, `strand`, `stranded_appearance`,
`orphaned_declare` — over three payload attributes: `node`, `name` and
`offset`. The question stands at its new size: `node` answers three
questions by `variant` (the instance whose offset the mate door cleared,
a stranded name's carrier, the orphaned declaration itself), and
`offset` is `None` on every arm but one.

## Narrowed (2026-10-02, RECIPE `declared-pairs-are-a-booleans-own-payload`)

The `Declare` node is gone: declared pairs are a boolean's or union's own payload, so no delete can orphan a declaration, and `Maintenance::OrphanedDeclare` (`orphaned_declare`) no longer exists. What this row says of the orphan arm no longer applies; the rest stands.
