//! **The per-verb correspondence**: what connects a document node to
//! the kernel verb it invokes.
//!
//! The recipe layer keeps the authoring vocabulary — an [`Expr`] per
//! slot, a frozen canonical selection of [`StableName`]s, node ids for
//! inputs — because that IS the document's semantics: what an
//! expression is, what a frozen reference is, how resolution refuses.
//! None of that is a restatement of the kernel.
//!
//! What was scattered is the CORRESPONDENCE between the two
//! vocabularies, and this module is where it now lives, one declaration
//! per verb: which [`SlotId`] feeds which verb parameter, which payload
//! selection feeds the key list, which emitter mints the names, which
//! arm of the record channel a family's result arrives in. The
//! lowering in [`mod@crate::eval`] is generic over it — one body of
//! code per declared door (`wire_blend` for the one-body verbs,
//! `wire_boolean` for the pair family, `wire_swept` for the profile
//! family, `wire_split` for the two-sided split, `wire_shell` for the
//! hollowing verb), each driven by the
//! declarations here rather than matching the kernel's verb vocabulary of its
//! own.
//!
//! [`Expr`]: crate::expr::Expr
//! [`StableName`]: crate::names::StableName
//! [`SlotId`]: crate::node::SlotId

pub(crate) mod blend;
pub(crate) mod boolean;
pub(crate) mod shell;
pub(crate) mod split;
pub(crate) mod sweep;

use geom_core::Real;
use verbs::{ScalarParam, VerbRecord};

use crate::node::SlotId;

use crate::eval::NodeErrorKind;
use crate::names::NamingError;

/// **The one rule for taking a family's record out of the closed
/// channel**: apply the correspondence's own projection — an
/// exhaustive match over [`VerbRecord`] that answers `Some` for its
/// family's arm and `None` for every other, so a record family added
/// to the channel breaks every projection at compile time (D3) — and
/// refuse a foreign family typed, in the correspondence's own words.
///
/// A wrong-family record is a kernel bug: which variant a verb's run
/// produces is fixed by its family, so this refusal is unreachable
/// while the doors and the correspondences agree. It is refused, not
/// panicked, and the sentence names the door so the refusal does
/// too. Every lowering reads its record through here and nowhere
/// else; the rule was once written inline at each consumer and the
/// copies had begun to drift in their comments before their code.
pub(crate) fn read_record<T: Real, R>(
    record: VerbRecord<T>,
    family: fn(VerbRecord<T>) -> Option<R>,
    foreign_record: &'static str,
) -> Result<R, NodeErrorKind> {
    family(record).ok_or(NodeErrorKind::Naming(NamingError::Emission {
        what: foreign_record,
    }))
}

/// **The slot ↔ parameter join of a verb with one scalar**, free of
/// the lane scalar. The document side names a slot, the kernel side
/// names a parameter, and the parameter → field flow
/// (`verbs::VerbKind::param_flow`) is keyed on the latter — so a
/// correspondence has to say which is which, or the flow cannot be
/// looked up for the value the slot produced. This is the join that
/// lets a lowering attach a slot's lowered expression identity to
/// exactly the fields the verb declares its parameter reaches, and the
/// content key feed the same slot's spelling, without either knowing
/// which verb it is holding.
///
/// It is a join, not a restatement: `ScalarParam::verb` already says
/// which verb a parameter belongs to (and each correspondence's census
/// checks it names its own verb's), but no function of the verb alone
/// can say which of the NODE's slots that parameter is — a slot is
/// document vocabulary, and a verb that one day carries two scalars
/// will need two of these. One type for every one-scalar verb (the two
/// blends, the shell), so the content key and the attach doors read
/// one shape.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct SlotJoin {
    /// The slot whose evaluated scalar is the verb's parameter: the
    /// fillet's radius, the chamfer's setback, the shell's wall.
    pub(crate) size_slot: SlotId,
    /// Which kernel scalar parameter that slot IS.
    pub(crate) size_param: ScalarParam,
}

#[cfg(test)]
#[allow(clippy::expect_used)]
mod verb_name_convention {
    //! The two public `Verb` types (`verbs::Verb`, the kernel's;
    //! `profile::Verb`, the sketch program's) share a name, and the
    //! convention their crate docs state is held here, in the one
    //! crate that reads both: no code line names both paths, and no
    //! file imports both, so a bare `Verb` in any file is one type.

    fn sources(dir: &std::path::Path, out: &mut Vec<(String, String)>) {
        for entry in std::fs::read_dir(dir).expect("a readable source directory") {
            let path = entry.expect("a directory entry").path();
            if path.is_dir() {
                sources(&path, out);
            } else if path.extension().is_some_and(|e| e == "rs") {
                let text = std::fs::read_to_string(&path).expect("a readable source file");
                out.push((path.display().to_string(), text));
            }
        }
    }

    #[test]
    fn no_file_reads_both_verb_types_bare() {
        let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("src");
        let mut files = Vec::new();
        sources(&root, &mut files);
        assert!(
            files.len() > 50,
            "the walk found only {} files",
            files.len()
        );
        let imports = |code: &str, krate: &str| {
            code.lines().any(|l| {
                let l = l.trim();
                l.starts_with("use ")
                    && l.contains(&format!("{krate}::"))
                    && (l.contains("Verb,") || l.contains("Verb}") || l.ends_with("Verb;"))
            })
        };
        for (path, text) in &files {
            let code = test_utils::source::code_only(text);
            for (n, line) in code.lines().enumerate() {
                assert!(
                    !(line.contains("verbs::Verb") && line.contains("profile::Verb")),
                    "{path}:{}: one code line names both Verb types",
                    n + 1
                );
            }
            assert!(
                !(imports(&code, "verbs") && imports(&code, "profile")),
                "{path}: imports both `verbs::Verb` and `profile::Verb` bare"
            );
        }
    }
}
