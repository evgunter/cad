//! **A sentence as a value** (DESIGN.md Band 1, "Node labels"): words,
//! and the nodes and variables it names as [`Mention`]s, each with the
//! document its id is spelled in. A [`Say`] value writes its sentence
//! into one through [`say!`](crate::say) and the [`Put`] pieces;
//! [`spell`] is the one spelling of a mention as text.

use core::cell::Cell;
use core::fmt;

use super::{Said, Say, Speaker, SpokenNode, SpokenVar};
use crate::DocumentId;
use crate::names::StableName;
use crate::node::RecipeNodeId;

std::thread_local! {
    /// How many sentences this thread is saying now ([`saying`]).
    static SAYING: Cell<u32> = const { Cell::new(0) };
}

/// **Runs `say` as a sentence being written**: while it runs, spelling
/// a sentence as text ([`flattening`]) is a kernel bug, and a debug
/// assertion (on in every profile) says so. Every road into a sentence
/// runs under it: a [`Say`] value said into one, a [`say!`](crate::say)
/// or [`sentence!`](crate::sentence) and the pieces it evaluates, and a
/// `write!` into a [`Sentence`].
#[doc(hidden)]
pub fn saying<R>(say: impl FnOnce() -> R) -> R {
    struct Depth;
    impl Drop for Depth {
        fn drop(&mut self) {
            SAYING.with(|depth| depth.set(depth.get().saturating_sub(1)));
        }
    }
    SAYING.with(|depth| depth.set(depth.get() + 1));
    let _depth = Depth;
    say()
}

/// **The rule a sentence's text keeps** (DESIGN.md Band 1, "Node
/// labels"): a sentence, and a [`Say`] value's sentence, becomes text
/// only where no sentence is being written ([`saying`]). Inside one, a
/// value with a sentence is written as a piece — `Said(&value, by)`, or
/// the sentence itself — so its mentions are kept; spelling it into the
/// words (`{value}`, `.to_string()`, a `String` piece) would make each
/// mention a word the viewer cannot link. `Arguments` erases the type a
/// `{}` formats, so this cannot be refused at compile time; it is
/// refused here, on the one road every such text takes ([`Sentence`]'s
/// `Display`, which every sentence-bearing value's `Display` ends in:
/// a [`Say`] type's through [`super::display_said`], a staged refusal's
/// (`ProductError`, `PersistError`, `WorkspaceError`) through
/// [`crate::sentence::Labelled`]'s, and `NodeError`'s through its
/// failed line).
fn flattening() {
    debug_assert!(
        SAYING.with(Cell::get) == 0,
        "a sentence was spelled as text inside a sentence being said, so its mentions became \
         words: write the value as a piece (`Said(&value, by)`, or the sentence itself)"
    );
}

/// A name said ([`Speaker::name`]), for [`Sentence::name`]: the one
/// spelling of a name in words ([`crate::LeafRole`]), [`crate::SpokenName`]'s
/// and [`StableName`]'s too.
#[must_use]
pub struct SaidName<'a>(pub(super) &'a StableName, pub(super) Speaker<'a>);

/// **A node as a sentence says it** ([`Speaker::node`],
/// [`Speaker::node_as`], [`Speaker::mention`]), for [`Sentence::node`]:
/// `this <noun>` for the sentence's subject, words — `<noun> <tag>` —
/// for a node no document the speaker had held, and otherwise a
/// [`Mention`]. An absent node is never a mention, since it has nothing
/// to link to. A mention's node was held by the document that said it,
/// or by the hold a speaker fell back to ([`Speaker::or_held`]), so a
/// reader that links one still checks that the document on screen is
/// the mention's and holds the node. It has no `Display`: only a whole
/// sentence becomes text.
#[must_use]
pub struct SaidNode(SaidNodeIs);

enum SaidNodeIs {
    This(&'static str),
    Absent(&'static str, RecipeNodeId),
    Mention(Mention),
}

impl SaidNode {
    /// `None` for the subject; a node not held, said as `noun`.
    pub(super) fn of(
        noun: &'static str,
        said: Option<SpokenNode>,
        document: Option<DocumentId>,
    ) -> Self {
        Self(match said {
            None => SaidNodeIs::This(noun),
            Some(node) if node.kind().is_none() => SaidNodeIs::Absent(noun, node.id()),
            Some(node) => SaidNodeIs::Mention(Mention {
                of: Mentioned::Node(node),
                document,
            }),
        })
    }
}

/// A variable as a sentence mentions it ([`Speaker::var`]), for
/// [`Sentence::var`].
#[must_use]
pub struct SaidVar(pub(super) Mention);

/// **What a mention names**: a recipe node or a variable, as the
/// document it was spoken from held it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Mentioned {
    /// A recipe node.
    Node(SpokenNode),
    /// A variable.
    Var(SpokenVar),
}

/// **A node or variable a sentence names** (DESIGN.md Band 1, "Node
/// labels"): what it names, as spoken, and the document its id is
/// spelled in — `None` where the speaker had none (a sentence said by
/// tag). An id is not document-scoped, so a reader acts on a mention
/// only in the document it names.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Mention {
    pub(super) of: Mentioned,
    pub(super) document: Option<DocumentId>,
}

impl Mention {
    /// What it names.
    #[must_use]
    pub fn mentioned(&self) -> &Mentioned {
        &self.of
    }

    /// The node it names, if it names one.
    #[must_use]
    pub fn node(&self) -> Option<&SpokenNode> {
        match &self.of {
            Mentioned::Node(node) => Some(node),
            Mentioned::Var(_) => None,
        }
    }

    /// The variable it names, if it names one.
    #[must_use]
    pub fn var(&self) -> Option<&SpokenVar> {
        match &self.of {
            Mentioned::Var(var) => Some(var),
            Mentioned::Node(_) => None,
        }
    }

    /// The document its id is spelled in.
    #[must_use]
    pub fn document(&self) -> Option<DocumentId> {
        self.document
    }
}

/// **A mention as text**: the one spelling, which a [`Sentence`]'s
/// `Display` and every flattening of one use. A node is its label and
/// tag, `"base plate" (3fa9c1d2a0b1)`; its kind and tag with no label,
/// `Extrude 3fa9c1d2a0b1`; `node 3fa9c1d2a0b1` where the document did
/// not hold it. A variable is its name, or its id with none.
///
/// Crate-private: outside the kernel a mention reaches text only
/// through a sentence's `Display`, which refuses inside a sentence being
/// written ([`saying`]); a public `spell` would be a road around it.
///
/// # Errors
///
/// The writer's.
pub(crate) fn spell(mention: &Mention, out: &mut dyn fmt::Write) -> fmt::Result {
    match &mention.of {
        Mentioned::Node(node) => node.spell(out),
        Mentioned::Var(var) => var.spell(out),
    }
}

/// One run of a [`Sentence`]: words, or a mention.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Run {
    /// Words, as written.
    Words(String),
    /// A node or variable it names.
    Mention(Mention),
}

/// **A sentence as a value**: words, written through [`fmt::Write`],
/// and mentions, written through [`Sentence::node`], [`Sentence::var`]
/// and [`Sentence::name`]. A [`Say`] value writes its sentence into
/// one; its `Display` is its text, each mention [`spell`]ed.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Sentence(Vec<Run>);

impl Sentence {
    /// The sentence `write` writes, from empty: a [`Speaker`] helper's
    /// part of a sentence.
    pub(super) fn written(write: impl FnOnce(&mut Sentence) -> fmt::Result) -> Self {
        let mut out = Self::new();
        let _ = saying(|| write(&mut out));
        out
    }

    /// The empty sentence.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Its runs, in order; no two words runs are adjacent.
    #[must_use]
    pub fn runs(&self) -> &[Run] {
        &self.0
    }

    /// The mentions it holds, in order.
    pub fn mentions(&self) -> impl Iterator<Item = &Mention> {
        self.0.iter().filter_map(|piece| match piece {
            Run::Mention(mention) => Some(mention),
            Run::Words(_) => None,
        })
    }

    /// Whether it holds nothing.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// Writes the node `said`.
    ///
    /// # Errors
    ///
    /// None; a sentence's write cannot fail.
    pub fn node(&mut self, said: SaidNode) -> fmt::Result {
        match said.0 {
            SaidNodeIs::This(noun) => write!(self, "this {noun}"),
            SaidNodeIs::Absent(noun, id) => write!(self, "{noun} {id}"),
            SaidNodeIs::Mention(mention) => {
                self.0.push(Run::Mention(mention));
                Ok(())
            }
        }
    }

    /// Writes the variable `said`.
    ///
    /// # Errors
    ///
    /// None; a sentence's write cannot fail.
    pub fn var(&mut self, said: SaidVar) -> fmt::Result {
        self.0.push(Run::Mention(said.0));
        Ok(())
    }

    /// Writes the name `said` in words, its nodes as mentions.
    ///
    /// # Errors
    ///
    /// None; a sentence's write cannot fail.
    pub fn name(&mut self, said: SaidName<'_>) -> fmt::Result {
        let SaidName(name, by) = said;
        saying(|| crate::names::LeafRole(name).say(self, by))
    }

    /// Writes `value`'s sentence, each node said by `by`.
    ///
    /// # Errors
    ///
    /// None; a sentence's write cannot fail.
    pub fn say<T: Say + ?Sized>(&mut self, value: &T, by: Speaker<'_>) -> fmt::Result {
        saying(|| value.say(self, by))
    }

    /// Writes `words` ([`fmt::Write::write_str`], callable with no
    /// import).
    ///
    /// # Errors
    ///
    /// None; a sentence's write cannot fail.
    pub fn write_str(&mut self, words: &str) -> fmt::Result {
        fmt::Write::write_str(self, words)
    }

    /// Writes `args` as words: what `write!` calls.
    ///
    /// # Errors
    ///
    /// None; a sentence's write cannot fail.
    pub fn write_fmt(&mut self, args: fmt::Arguments<'_>) -> fmt::Result {
        saying(|| fmt::write(self, args))
    }

    /// `parts`, in order, with `between` written between each two.
    #[must_use]
    pub fn joined(parts: impl IntoIterator<Item = Sentence>, between: &str) -> Self {
        let mut out = Self::new();
        for (i, part) in parts.into_iter().enumerate() {
            if i > 0 {
                out.push_words(between);
            }
            out.append(part);
        }
        out
    }

    /// Writes `other` after it, its mentions as they were.
    pub fn append(&mut self, other: Sentence) {
        for piece in other.0 {
            match piece {
                Run::Words(words) => self.push_words(&words),
                mention @ Run::Mention(_) => self.0.push(mention),
            }
        }
    }

    pub(super) fn push_words(&mut self, words: &str) {
        if words.is_empty() {
            return;
        }
        match self.0.last_mut() {
            Some(Run::Words(run)) => run.push_str(words),
            _ => self.0.push(Run::Words(words.to_owned())),
        }
    }
}

/// The road `write!` takes into a sentence, its inherent methods' own:
/// words are pushed, and what the arguments format runs as a sentence
/// being written ([`saying`]).
impl fmt::Write for Sentence {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.push_words(s);
        Ok(())
    }

    fn write_fmt(&mut self, args: fmt::Arguments<'_>) -> fmt::Result {
        Sentence::write_fmt(self, args)
    }
}

/// A mention alone.
impl From<Mention> for Sentence {
    fn from(mention: Mention) -> Self {
        Self(vec![Run::Mention(mention)])
    }
}

/// Words alone.
impl From<String> for Sentence {
    fn from(words: String) -> Self {
        let mut sentence = Self::new();
        sentence.push_words(&words);
        sentence
    }
}

/// Words alone.
impl From<&str> for Sentence {
    fn from(words: &str) -> Self {
        let mut sentence = Self::new();
        sentence.push_words(words);
        sentence
    }
}

/// Its text: the words, and each mention [`spell`]ed — never inside a
/// sentence being said ([`flattening`]).
impl fmt::Display for Sentence {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        flattening();
        self.spell_into(f)
    }
}

impl Sentence {
    /// Its text, into `f`.
    fn spell_into(&self, f: &mut dyn fmt::Write) -> fmt::Result {
        for run in &self.0 {
            match run {
                Run::Words(words) => f.write_str(words)?,
                Run::Mention(mention) => spell(mention, f)?,
            }
        }
        Ok(())
    }

    /// **Its text as a comparison key**, never shown: how two names'
    /// words are told apart while a sentence is being said (the detail a
    /// table says a name at), where spelling the sentence for a reader
    /// would be a flattening.
    pub(crate) fn key(&self) -> String {
        let mut key = String::new();
        let _ = self.spell_into(&mut key);
        key
    }
}

/// Its text ([`spell`]ing each mention): where a sentence leaves the
/// kernel as text.
impl From<Sentence> for String {
    fn from(sentence: Sentence) -> Self {
        sentence.to_string()
    }
}

impl Sentence {
    /// Whether it reads `text`, compared as it is spelled, run by run.
    fn reads(&self, text: &str) -> bool {
        struct Against<'t>(&'t str);
        impl fmt::Write for Against<'_> {
            fn write_str(&mut self, s: &str) -> fmt::Result {
                let rest = self.0.strip_prefix(s).ok_or(fmt::Error)?;
                self.0 = rest;
                Ok(())
            }
        }
        let mut against = Against(text);
        fmt::write(&mut against, format_args!("{self}")).is_ok() && against.0.is_empty()
    }
}

/// A sentence equals the text it reads.
impl PartialEq<str> for Sentence {
    fn eq(&self, text: &str) -> bool {
        self.reads(text)
    }
}

/// A sentence equals the text it reads.
impl PartialEq<&str> for Sentence {
    fn eq(&self, text: &&str) -> bool {
        self.reads(text)
    }
}

/// A sentence equals the text it reads.
impl PartialEq<String> for Sentence {
    fn eq(&self, text: &String) -> bool {
        self.reads(text)
    }
}

/// A sentence equals the text it reads.
impl PartialEq<Sentence> for String {
    fn eq(&self, sentence: &Sentence) -> bool {
        sentence.reads(self)
    }
}

/// A sentence equals the text it reads.
impl PartialEq<Sentence> for &str {
    fn eq(&self, sentence: &Sentence) -> bool {
        sentence.reads(self)
    }
}

/// A formula's readers, mentioned ([`Speaker::formula`]).
pub(super) struct Readers<'s, 'a> {
    pub(super) out: &'s mut Sentence,
    pub(super) by: Speaker<'a>,
}

impl crate::expr::Unparsed for Readers<'_, '_> {
    fn push(&mut self, ch: char) {
        let _ = fmt::Write::write_char(self.out, ch);
    }
    fn push_str(&mut self, text: &str) {
        self.out.push_words(text);
    }
    fn reader(&mut self, var: crate::var::VarId, name: Option<&crate::doc::VarName>) {
        let _ = self
            .out
            .var(self.by.var(&SpokenVar::new(var, name.cloned())));
    }
}

/// **What [`say!`](crate::say) writes into a sentence**: words (`&str`,
/// `String`, `format_args!`) or a mention ([`SaidNode`], [`SaidVar`],
/// [`SaidName`]), a [`Sentence`], or a value [`Said`].
pub trait Put {
    /// Writes itself into `out`.
    fn put(self, out: &mut Sentence);
}

impl Put for &str {
    fn put(self, out: &mut Sentence) {
        out.push_words(self);
    }
}

impl Put for String {
    fn put(self, out: &mut Sentence) {
        out.push_words(&self);
    }
}

impl Put for fmt::Arguments<'_> {
    fn put(self, out: &mut Sentence) {
        let _ = out.write_fmt(self);
    }
}

impl Put for SaidNode {
    fn put(self, out: &mut Sentence) {
        let _ = out.node(self);
    }
}

impl Put for SaidVar {
    fn put(self, out: &mut Sentence) {
        let _ = out.var(self);
    }
}

impl Put for SaidName<'_> {
    fn put(self, out: &mut Sentence) {
        let _ = out.name(self);
    }
}

impl Put for Sentence {
    fn put(self, out: &mut Sentence) {
        out.append(self);
    }
}

/// A borrowed sentence, its runs copied in: a value said into more than
/// one sentence (a claim offered twice) keeps its mentions in each.
impl Put for &Sentence {
    fn put(self, out: &mut Sentence) {
        out.append(self.clone());
    }
}

/// `Recourse: ` and the recourse, its mentions kept.
impl Put for geom_core::Recourse<Sentence> {
    fn put(self, out: &mut Sentence) {
        out.push_words("Recourse: ");
        out.append(self.0);
    }
}

/// `Recourse: ` and the recourse, its mentions kept.
impl Put for geom_core::Recourse<&Sentence> {
    fn put(self, out: &mut Sentence) {
        out.push_words("Recourse: ");
        out.append(self.0.clone());
    }
}

impl<T: Say + ?Sized> Put for Said<'_, T> {
    fn put(self, out: &mut Sentence) {
        let _ = saying(|| self.0.say(out, self.1));
    }
}

/// **Writes a sentence's pieces in order** ([`Put`]): `say!(out,
/// by.node(id), format_args!(" reads {n} values"))`. Evaluates to
/// `Ok(())`, the [`Say::say`] answer.
#[macro_export]
macro_rules! say {
    ($out:expr $(, $piece:expr)* $(,)?) => {{
        let out: &mut $crate::Sentence = $out;
        $crate::spoken::sentence::saying(|| {
            $( $crate::Put::put($piece, &mut *out); )*
        });
        ::core::result::Result::<(), ::core::fmt::Error>::Ok(())
    }};
}

/// **A sentence of its pieces** ([`say!`](crate::say)'s, written into a
/// new [`Sentence`]).
#[macro_export]
macro_rules! sentence {
    ($($piece:expr),* $(,)?) => {{
        let mut out = $crate::Sentence::new();
        let _ = $crate::say!(&mut out $(, $piece)*);
        out
    }};
}
