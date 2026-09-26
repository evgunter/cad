//! What the environment the process was started in offers the shell —
//! read once, before the first frame.
//!
//! # Why this is a module rather than a corner of another one
//!
//! Every value here answers a question about the process's
//! surroundings: is `zenity` on `PATH`, is a session bus advertised,
//! where does the XDG base-directory specification put a config file,
//! is this WSL. The argument of each is the environment itself, so
//! none of them is a function of anything this crate holds and none
//! can be replayed from a value a test builds. Each is probed ONCE at
//! startup and the reading is stored; nothing here is re-read under a
//! running app.
//!
//! That is what makes this the crate's **ambient door**.
//! `scripts/gates/no-ambient-env.sh` ratifies that the viewer's
//! runtime environment reads have ONE home and allowlists this file as
//! that home; the argument it makes against its four rows — the reads
//! observe the environment rather than steering the model, commit
//! once, are reported in the chrome itself, and are a bootstrap the
//! real outcome outranks — is an argument about exactly these probes
//! and about nothing else in the crate. A module whose whole purpose
//! is the door is what makes that entry a door rather than a region
//! inside something else.
//!
//! What the readings are FOR is held state, never news: a dialog the
//! environment cannot put up becomes a disabled control carrying
//! [`ChooserBackend::unusable`] as its reason, not a sentence on the status
//! line (`crates/viewer/README.md`, *"A missing file-chooser backend
//! is not on the line at all"*). The chrome policies that read these
//! values are [`crate::frame`]'s; deciding them is not this module's
//! job, and observing the machine is not theirs.
//!
//! Module kind: **vocabulary** — it names no driver type and no
//! `app`-only crate (`crates/viewer/README.md`, Module boundaries).

/// What the environment offers `rfd` as a file-chooser backend.
///
/// Probed ONCE at startup ([`chooser_backend`]) — it is a fact about
/// the environment, not per-frame state — and consulted wherever a
/// dialog is offered. The point is to fail LOUD at first sight (first
/// light, issue #1097: Open/Save As "silently did nothing" on a WSL
/// distro shipping neither backend) instead of hedging after a dead
/// click: `rfd`'s blocking dialogs return the same bare `None` for a
/// user cancel and for a backend that could not put a dialog up, so
/// the time to know is before the click.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ChooserBackend {
    /// `zenity` is on `PATH`: dialogs work with no portal at all.
    ZenityPresent,
    /// No `zenity`, but a D-Bus session-bus address exists, so an
    /// `xdg-desktop-portal` file chooser is possible. **A HINT, not a
    /// verdict**: a session bus without a working portal frontend
    /// still ends in a silent `None` the process cannot tell from a
    /// cancel — that residue is the README's troubleshooting entry,
    /// not a message this code can honestly print.
    PortalPossible,
    /// Neither: no dialog can possibly appear. The one CONFIDENT
    /// arm, and the one the chrome disables the dialogs over.
    Absent,
}

impl ChooserBackend {
    /// **Why no dialog can appear here**, and `None` when attempting
    /// one can possibly show it.
    ///
    /// The value that knows the environment is the party that words
    /// it, as [`crate::prefs::PrefsStore::unusable`] is for a store
    /// that keeps nothing: a bare `bool` left the only party able to
    /// say WHY unable to say it, so the sentence a reader saw was
    /// composed beside the value rather than by it. The gate and the
    /// reason are one answer — a disabled control is `is_some()` and
    /// its tooltip is the `Some` — so no call site can gate on this
    /// value and explain with another. The words are `&'static str`,
    /// keeping the type `Copy`, because the vocabulary is closed and
    /// its one unusable arm has one reason; a store's reason is its
    /// backing store's own text and is not.
    ///
    /// The `Some` is **what the disabled dialog controls say**, and
    /// the only thing that says it: the confident half of the #1097
    /// finding, with the dialog-free workaround. A missing backend is
    /// held state, so the disabled control carrying it as its
    /// `on_disabled_hover_text` is the read and there is no
    /// status-line route beside it. The argument, its sweep rule and
    /// Ev's ruling live in `crates/viewer/README.md`, under *"A
    /// missing file-chooser backend is not on the line at all"*.
    ///
    /// A match over every arm rather than a pattern over one, so an
    /// arm added to the vocabulary is a decision here rather than a
    /// silent `None`.
    pub fn unusable(self) -> Option<&'static str> {
        match self {
            Self::ZenityPresent | Self::PortalPossible => None,
            Self::Absent => Some(
                "no file chooser backend — install zenity or \
                 xdg-desktop-portal; a document path can also be passed on the \
                 command line",
            ),
        }
    }
}

/// The directory this project keeps user files in.
const PREFS_DIR: &str = "pncad";
/// The preferences file's name inside it.
const PREFS_FILE: &str = "viewer.toml";

/// What the `zenity` probe read.
///
/// A named type rather than a `bool` for the reason
/// [`crate::session::Outstanding`] gives: it sits beside a second
/// environment reading of the same shape, and two adjacent `bool`s
/// that mean different things transpose silently.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Zenity {
    /// A `zenity` binary sits in some `PATH` directory.
    OnPath,
    /// None does.
    NotOnPath,
}

/// What the D-Bus probe read. Named for the same reason as
/// [`Zenity`].
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SessionBus {
    /// A session-bus address is advertised.
    Advertised,
    /// None is.
    NotAdvertised,
}

/// The chooser verdict as a pure function of the two probe readings,
/// so the rows exercising it do not depend on the CI box's `PATH`.
pub fn chooser_backend_of(zenity: Zenity, bus: SessionBus) -> ChooserBackend {
    match (zenity, bus) {
        (Zenity::OnPath, _) => ChooserBackend::ZenityPresent,
        (Zenity::NotOnPath, SessionBus::Advertised) => ChooserBackend::PortalPossible,
        (Zenity::NotOnPath, SessionBus::NotAdvertised) => ChooserBackend::Absent,
    }
}

/// Probe the environment for a chooser backend. Startup calls this
/// once; everything downstream reads the stored value.
pub fn chooser_backend() -> ChooserBackend {
    if cfg!(target_family = "wasm") {
        // The browser build links no `rfd` at all (its wasm backend
        // offers only the async dialog; see `viewer`'s Cargo.toml),
        // so this is the CONFIDENT arm in the strongest possible
        // sense: there is not merely no backend, there is no dialog
        // code. Saying `Absent` is what disables Open…/Save… with
        // their reason showing, which is #1097's whole lesson — a
        // door that cannot open must not answer a click with silence.
        ChooserBackend::Absent
    } else if cfg!(target_os = "linux") {
        chooser_backend_of(zenity_on_path(), session_bus_hinted())
    } else {
        // Off Linux `rfd` speaks the platform's native dialog API and
        // the zenity/portal question does not arise. Grouped under the
        // hint arm because the downstream meaning is the same: attempt
        // the dialog, and read a `None` as a genuine cancel.
        ChooserBackend::PortalPossible
    }
}

/// Whether a `zenity` binary sits in some `PATH` directory. Presence
/// is the signal `rfd`'s own fallback lookup uses; a present but
/// broken zenity is the dialog's own problem to report.
fn zenity_on_path() -> Zenity {
    let found = std::env::var_os("PATH")
        .is_some_and(|paths| std::env::split_paths(&paths).any(|dir| dir.join("zenity").is_file()));
    if found {
        Zenity::OnPath
    } else {
        Zenity::NotOnPath
    }
}

/// Whether a D-Bus session-bus address is advertised — the necessary
/// (never sufficient) condition for the portal chooser.
fn session_bus_hinted() -> SessionBus {
    let advertised =
        std::env::var_os("DBUS_SESSION_BUS_ADDRESS").is_some_and(|address| !address.is_empty());
    if advertised {
        SessionBus::Advertised
    } else {
        SessionBus::NotAdvertised
    }
}

/// Where this platform keeps the viewer's preferences:
/// `$XDG_CONFIG_HOME/pncad/viewer.toml`, falling back to
/// `$HOME/.config` as the XDG base-directory specification says to.
///
/// **Here rather than in [`crate::prefs`], because this file is the
/// viewer's ONE ambient door** — the ruling in
/// `scripts/gates/no-ambient-env.sh`, which names this module by path
/// and says so in as many words. `prefs` stays a pure value over a
/// document and a store; where the document lives is a fact about the
/// machine, and facts about the machine are observed here beside the
/// chooser-backend verdict and the WSL probe.
///
/// Against that gate's four rows, the same way the entry beside it
/// argues them. CONTRACT-RATIFIED holds vacuously: a config path is
/// not a model parameter, and no read here can change what any
/// document evaluates to. COMMIT-ONCE: read at startup and stored in
/// the application, never re-read under a running app — a viewer
/// whose config directory moved mid-session would be stranger than
/// one that kept writing where it started. REPORTED: the path is
/// carried in every [`crate::prefs::StoreError`], so a refusal to
/// save names the file it could not write rather than leaving a
/// person guessing. RECONCILED: this is a BOOTSTRAP and never the
/// last word — the actual read or write outcome outranks it, and an
/// environment that names no config directory yields `None`, which
/// disables saving with a reason instead of inventing a path.
///
/// Resolved by hand rather than through `directories`, whose whole
/// value is the two platforms this project does not build for.
///
/// `None` when neither variable is set, which is a real possibility
/// in a stripped environment.
#[must_use]
pub fn prefs_path() -> Option<std::path::PathBuf> {
    prefs_path_in(
        std::env::var_os("XDG_CONFIG_HOME").as_deref(),
        std::env::var_os("HOME").as_deref(),
    )
}

/// The preferences path as a pure function of the two environment
/// readings, so the XDG rules are asserted on rather than trusted.
///
/// Split out for [`chooser_backend_of`]'s reason, and it is the same
/// reason: the ambient read is one line that cannot be exercised in a
/// test without mutating the process's environment — which is a
/// global other tests share — while everything INTERESTING here is
/// the resolution, and the resolution is a function of two `Option`s.
///
/// The rules, from the XDG base-directory specification:
///
/// - `config_home` set and non-empty wins.
/// - **An EMPTY `config_home` counts as unset**, which the spec says
///   in as many words and which is the case a bare
///   `unwrap_or_else` fallback gets wrong: it would take `""` as the
///   base and write to a RELATIVE path, i.e. into whatever directory
///   the viewer happened to be launched from.
/// - Otherwise `$HOME/.config`.
/// - With neither, `None` — no path is invented. The caller's store
///   is then unusable and says so, which is how a person finds out
///   their preferences are not being kept rather than wondering
///   later why nothing was remembered.
#[must_use]
pub fn prefs_path_in(
    config_home: Option<&std::ffi::OsStr>,
    home: Option<&std::ffi::OsStr>,
) -> Option<std::path::PathBuf> {
    let base = match config_home {
        Some(value) if !value.is_empty() => std::path::PathBuf::from(value),
        _ => std::path::PathBuf::from(home.filter(|h| !h.is_empty())?).join(".config"),
    };
    Some(base.join(PREFS_DIR).join(PREFS_FILE))
}

/// Whether this process runs inside WSL, read off the environment
/// markers WSL itself sets for every process (`WSL_DISTRO_NAME`,
/// `WSL_INTEROP`). Either suffices; both are checked because WSL1
/// and WSL2 differ in which they guarantee. Consumed by [`crate::app::run`],
/// which prefers the X11 backend under WSL (WSLg's Wayland RAIL shell
/// breaks horizontal resizing — #1097, confirmed).
///
/// Here rather than in `app` so the viewer's ambient-environment
/// reads have ONE home, which is what the `no-ambient-env` gate's
/// allowlist entry for this file ratifies — see the argument in
/// `scripts/gates/no-ambient-env.sh`.
#[cfg(target_os = "linux")]
pub fn running_under_wsl() -> bool {
    std::env::var_os("WSL_DISTRO_NAME").is_some() || std::env::var_os("WSL_INTEROP").is_some()
}

/// The directory the viewer was launched from: the process's working
/// directory, read ONCE at startup and held by the application.
///
/// It is the last candidate a file dialog opens at
/// (`crate::frame::dialog_dir`), behind the current document's
/// directory and the directory the last dialog returned — so a viewer
/// launched from a project directory with nothing open and nothing
/// remembered starts its first Save As… there rather than wherever
/// the dialog backend's own default happens to be.
///
/// Here because this file is the viewer's one ambient door, and the
/// four rows of `scripts/gates/no-ambient-env.sh` are argued the way
/// [`prefs_path`] argues them. CONTRACT-RATIFIED holds vacuously: a
/// starting directory changes nothing about what any document
/// evaluates to. COMMIT-ONCE: read here and never re-read — a process
/// that `chdir`s mid-run is not one this crate is. REPORTED: the
/// directory is what the dialog visibly opens at, and the one failure
/// is a startup notice on the status line. RECONCILED: it is the last
/// candidate of three and never outranks the document's own place.
///
/// # Errors
///
/// The `io::Error` the read answers with — a launch directory that no
/// longer exists or cannot be read. The caller reports it and the
/// dialogs fall through to the backend's default; nothing is invented.
///
/// **Native only.** The browser build has no dialogs to position and
/// no working directory to read, so there is no arm to `cfg` the
/// other way.
#[cfg(not(target_family = "wasm"))]
pub fn launch_dir() -> Result<std::path::PathBuf, std::io::Error> {
    std::env::current_dir()
}
