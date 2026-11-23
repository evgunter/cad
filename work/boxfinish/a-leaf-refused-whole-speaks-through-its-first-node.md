---
id: a-leaf-refused-whole-speaks-through-its-first-node
kind: issue
title: A leaf whose replay is refused whole (a box naming a variable the document lacks) is classified through its first node
status: open
opened: 2026-11-22
---



**Read, not measured.** `drive(doc, analyzed, ..)` takes the document and
the analyzed box as two arguments. A box that names a variable `doc`
does not hold as a continuous free variable fails
`analysis::var_env_over` (`ParamBoxError::UnknownParam`), and the
evaluator answers with `eval::refuse_param_box`: every node `Failed`
with `NodeErrorKind::ParamBox`, `Evaluation::refused_whole()` true. The
witness is evaluated without the box, so it builds and the drive starts.

`drive::classify_replay` does not ask `refused_whole()`. Its walk meets
the first node in order, finds no escalation, and takes the catch-all
arm: the leaf is undecided at that node. Before the split followed the
deciding node, such a drive bisected to its leaf budget (65,536 leaves
by default, each an instant refusal). Now the first node's read set
chooses: a first node that reads nothing of the box refuses the root
`ReadsNoVariable`, naming a node that is not the cause; one that reads
a variable bisects it to a budget as before.

The cause is the pairing of the two arguments, not a node, so the
refusal that fits is a `DriveRefusal` raised before the first leaf (the
box binds or it does not, once), or a mispairing check like
`eval::record_pairing`'s. No row constructs the case; the first step is
one that drives a document over another document's box and pins what
comes back. (BOXFINISH split-reads lane)
