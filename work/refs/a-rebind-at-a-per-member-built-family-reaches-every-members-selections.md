---
id: a-rebind-at-a-per-member-built-family-reaches-every-members-selections
kind: issue
title: Rebind addresses the variable, so a repair at a family built per member reaches selections of members whose tables differ
status: open
opened: 2026-11-10
priority: P3
cost: M
design: true
---


Filed by PR 6555 (fork row 259), whose design keeps `DocEdit::Rebind`
addressed by the variable holding the body (`body: Option<VarId>`,
`edit.rs` `DocEdit::Rebind`): a repair at `xs` rewrites every selection
reading `xs` that names `from`, whichever member it reads
(`doc.rs` `rewrite_selection_names`, which matches `select.body.read`).

Every repair the tree issues today is uniform across members: a family
is one source placed per member, and each member carries its source's
names verbatim. A family whose members are built per member — a per-`k`
operation whose table can differ by `k` — can hold one name meaning
different entities in two members. A rebind at `xs` issued for one
member then re-points the other members' selections of that name too.
The collateral repair does not pass silently: the re-pointed name no
longer resolves in the other member, which refuses under N5
(`SelectResolve`, `Vanished`).

The finer address — `Rebind { body: Option<BodyRead<VarId>> }`, matching
the read and its indices — is additive and does not change what today's
`Rebind` means. Done when a family built per member has a rebind that
reaches one member's selections alone, or a ruling that the variable
address is the final one.
