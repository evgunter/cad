//! **The funnel's row rosters, checked distinct**: the two `const`
//! readers every crate's row registry runs its roster through.
//!
//! A row is a predicate name a reader decides under. It keys the
//! reader's `k_stats` stream and its ledger row
//! (`docs/predicate-dimension-audit.md`), so two readers sharing one
//! would merge their samples under it. Each crate that mints row bundles
//! holds them in one `rows` module: the bundle types, whose fields are
//! private to it, and every reader's bundle as a `const` given to
//! [`row_registry!`], which derives the `ROSTER` of their names and the
//! `const` assertion that fails the crate's build on a name used twice.
//! editor-core's tree-wide test walks every crate's roster together.

/// `a == b`, in a `const` context.
#[must_use]
pub const fn str_eq(a: &str, b: &str) -> bool {
    let (a, b) = (a.as_bytes(), b.as_bytes());
    if a.len() != b.len() {
        return false;
    }
    let mut k = 0;
    while k < a.len() {
        if a[k] != b[k] {
            return false;
        }
        k += 1;
    }
    true
}

/// The first two indices of `rows` that name one row, if any.
#[must_use]
pub const fn first_shared(rows: &[&str]) -> Option<(usize, usize)> {
    let mut a = 0;
    while a < rows.len() {
        let mut b = a + 1;
        while b < rows.len() {
            if str_eq(rows[a], rows[b]) {
                return Some((a, b));
            }
            b += 1;
        }
        a += 1;
    }
    None
}

/// The first two places in `roster`, a list of bundles' names, that name
/// one row, each as `(bundle, row)`; `None` where every name is
/// distinct, within a bundle and across bundles.
#[must_use]
pub const fn first_shared_in(roster: &[&[&str]]) -> Option<((usize, usize), (usize, usize))> {
    let mut i = 0;
    while i < roster.len() {
        let mut a = 0;
        while a < roster[i].len() {
            let mut j = i;
            while j < roster.len() {
                let mut b = if j == i { a + 1 } else { 0 };
                while b < roster[j].len() {
                    if str_eq(roster[i][a], roster[j][b]) {
                        return Some(((i, a), (j, b)));
                    }
                    b += 1;
                }
                j += 1;
            }
            a += 1;
        }
        i += 1;
    }
    None
}

/// **The bundle consts a registry's source declares outside its
/// [`row_registry!`] blocks**, which would escape the derived roster: each
/// line of `src` (the registry's own file, read with `include_str!`) that
/// declares `const NAME: T` for a bundle type `T` in `types` and lies in no
/// `row_registry! { .. }` invocation. A registry's test asserts this is
/// empty. Comment lines are not read; a declaration split before its
/// type is (the type follows the name on the declaring line by `rustfmt`).
#[must_use]
pub fn consts_outside_registry(src: &str, types: &[&str]) -> Vec<String> {
    let mut inside = Vec::new();
    let mut at = 0;
    while let Some(hit) = src[at..].find("row_registry! {") {
        let open = at + hit + "row_registry! ".len();
        let mut depth = 0usize;
        let mut end = src.len();
        for (k, c) in src[open..].char_indices() {
            match c {
                '{' => depth += 1,
                '}' => {
                    depth -= 1;
                    if depth == 0 {
                        end = open + k;
                        break;
                    }
                }
                _ => {}
            }
        }
        inside.push(open..end);
        at = end;
    }
    let mut out = Vec::new();
    let mut offset = 0;
    for line in src.split_inclusive('\n') {
        let start = offset;
        offset += line.len();
        let code = line.trim();
        if code.starts_with("//") || inside.iter().any(|r| r.contains(&start)) {
            continue;
        }
        let decl = code
            .strip_prefix("pub(crate) ")
            .or_else(|| code.strip_prefix("pub "))
            .unwrap_or(code);
        let Some(rest) = decl.strip_prefix("const ") else {
            continue;
        };
        let Some((_, ty)) = rest.split_once(':') else {
            continue;
        };
        let ty = ty.split('=').next().unwrap_or(ty).trim();
        if types.contains(&ty) {
            out.push(code.to_owned());
        }
    }
    out
}

/// **A crate's row registry, its roster derived**: every `const` bundle
/// given here, and a `ROSTER` of their names that lists exactly those
/// consts, with the `const` assertion that fails the build on a name used
/// twice ([`first_shared_in`]). A const added here is in the roster by
/// construction: there is no hand-kept list to leave it out of.
///
/// ```ignore
/// geom_core::row_registry! {
///     /// Every registered bundle's rows.
///     pub const ROSTER;
///     /// A reader's rows.
///     pub(crate) const A_READER: SomeRows = SomeRows { .. };
/// }
/// ```
///
/// Each bundle type gives `const fn names(&self) -> [&'static str; N]`.
/// Test-only bundles go in a second invocation under `#[cfg(test)]`, with
/// a roster of their own.
#[macro_export]
macro_rules! row_registry {
    (
        $(#[$rmeta:meta])* $rvis:vis const $roster:ident;
        $( $(#[$meta:meta])* $vis:vis const $name:ident: $ty:ty = $value:expr; )+
    ) => {
        $( $(#[$meta])* $vis const $name: $ty = $value; )+

        $(#[$rmeta])*
        $rvis const $roster: &[&[&str]] = &[ $( &$name.names() ),+ ];

        const _: () = assert!(
            $crate::rows::first_shared_in($roster).is_none(),
            concat!("two bundles of ", stringify!($roster), " share a row name")
        );
    };
}

#[cfg(test)]
mod tests {
    use super::{first_shared, first_shared_in};

    /// **The check fires on a shared name**, within one bundle and across
    /// two, and passes a roster whose names are distinct.
    #[test]
    fn a_roster_refuses_a_name_used_twice() {
        assert_eq!(first_shared(&["a", "b", "a"]), Some((0, 2)));
        assert_eq!(first_shared(&["a", "b", "c"]), None);
        assert_eq!(
            first_shared_in(&[&["a", "b"], &["c", "a"]]),
            Some(((0, 0), (1, 1)))
        );
        assert_eq!(
            first_shared_in(&[&["a", "a"], &["c"]]),
            Some(((0, 0), (0, 1)))
        );
        assert_eq!(first_shared_in(&[&["a", "b"], &["c"], &[]]), None);
    }

    /// **A bundle const outside the registry's block is found**, one
    /// inside it is not, nor a comment or a const of another type.
    #[test]
    fn a_const_outside_the_registry_block_is_found() {
        let src = "pub(crate) const STRAY: Rows = Rows { a: \"x\" };\n\
                   /// const DOC: Rows = ..\n\
                   const OTHER: u8 = 1;\n\
                   geom_core::row_registry! {\n    pub const ROSTER;\n    \
                   pub(crate) const IN: Rows = Rows { a: \"y\" };\n}\n\
                   pub const SPLIT: Rows =\n    Rows { a: \"z\" };\n";
        assert_eq!(
            super::consts_outside_registry(src, &["Rows"]),
            [
                "pub(crate) const STRAY: Rows = Rows { a: \"x\" };",
                "pub const SPLIT: Rows =",
            ]
        );
    }
}
