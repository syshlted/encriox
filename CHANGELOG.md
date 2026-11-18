# encri-firefox — changelog

What happened, newest first. Why is in `docs/decisions/`, and what is next is in `ROADMAP.md`.

## Unreleased

### Stage 2 builds its own Gecko (2026-11-18)

At the user's word, stage 2 compiles Gecko from stage 1's checkout, on the drive for large builds, with four patches of
its own (`scripts/spike/stage-2/`, W13). The first build took 76 minutes. **QuotaManager records an origin's
encryption** when it makes the origin's directory, which W12 found it could record as plaintext. **SQLite's switch is
unlocked and on by default.** Gecko's launch guard reads it before any pref is loaded, so on the S9 a profile that the
config had encrypted was refused at its very next start. The root Gradle build includes encri-api, so that Fenix is
built against this GeckoView.

On the G6, the S9 and the A20, **all three of Lockstore's switches now work**. Every database in Gecko's profile was
encrypted but NSS's two and Lockstore's own key store, and the test origin was recorded as encrypted. At the next start
all six stores read back, on the two phones that had lost site storage in W12 too. A normal session left its marker in
4 files: Fenix's saved tabs, its history and the history's WAL, and the service worker's registration. Each phone was
then given a fresh profile, stage 1's APK and stage 1's config again. Three points are flagged for veto.

`check-plaintext` no longer blames the config when Gecko refuses a profile, and `check-lockstore` names the Gecko that
made the profile.

### Stage 2's first round: what Lockstore's switches do at 160 (2026-11-18)

On the G6, the S9 and the A20, stage 1's build ran with Lockstore's three switches on, on a fresh profile
(`scripts/spike/stage-2/`, W12). A normal session left its marker in 4 or 5 files where it had left it in 9 to 11:
Fenix's saved tabs and history, the cookies' WAL and the service worker's registration. **The HTTP cache's switch
works** with a pref alone, and its entries came back after a restart. **Site storage's encrypts, and then breaks at the
next start**: on the S9 and the G6, QuotaManager recorded the origin as plaintext, so local storage, IndexedDB and the
Cache API could not be read. Setting the missing flag by hand on the G6 brought all of it back, so nothing was lost but
the flag. The cause is in Gecko, at 160 and on `main`. **SQLite's switch is locked off** in `all.js`, so no pref turns
it on. The next two rounds wait on a Gecko build or on Mozilla, which is the user's call. One point is flagged for veto.

`check-plaintext` takes the Gecko config to push from `CONFIG`, and refuses one whose SQLite switch disagrees with the
profile on the phone. `serve` has a page that reads back what the marker page wrote.

### The Chromium browser is encrimium (2026-11-18)

encrium is given up, because `encrium.com` appears to belong to a French technical-training academy, and the
Chromium browser is encrimium, *encri* and *(chro)mium*, with the package `com.systemhalted.encrimium` (W11, amending
W3). Its knock-out search, the same as encriox's, found no mark, use or domain in the way
(`docs/trademark-search.md`). encrimium's repository, decision prefix and licence are still its own first record's.

### The trademark search for encriox (2026-11-17)

A knock-out search, owed by W3, found nothing that blocks the name (`docs/trademark-search.md`). No mark ENCRIOX is in
TMview, which holds the EU's, its national offices', the US's, Japan's and WIPO's records, or in Trademarkia's US data;
nothing named encriox is in use on the web, in the app stores or on GitHub; and `encriox.com`, `.net`, `.org` and
`.app` are unregistered. The nearest marks are Encrox, a German mark one letter off and expired, and ENCRY and
MacPaw's ENCRYPTO, live in class 9. The offices' own searches could not be queried, being JavaScript applications or
behind a CAPTCHA. `encrium.com` is registered, to 2026-12-20.

### Stage 2 on release, the package, following Firefox, and a hidden key (2026-11-17)

The user settled four of what was open before encriox itself is written. Stage 2 tracks Firefox's release, not an
ESR, starting from stage 1's 160.0, and its builds go on a drive of their own, with 387 GB free (W7). The package is
`com.systemhalted.encriox`, with Fenix's code namespaces kept, and encri-api's registry lists it before the first
release so that a grant survives each update (W8). encriox follows every Firefox release: a script applies its patch
series to the newest release tag, builds and tests, and stops for a person at the first thing it cannot do; publishing
stays by hand; and the About screen and the documentation name the release it is built on (W9). Its key is a hidden
document in the vault, `.encriox.key`, for now, with app-level security controls later (W10). Five points are flagged
for veto across W8 to W10.

**Found:** Firefox 160.0.1 is tagged, and Mozilla's Maven has no GeckoView for it (W7). Nothing in encri or encri-api
matches a package by prefix (W8). encri takes a name beginning with a dot, and nothing in the suite hides one (W10).
At 160, Lockstore's three switches for SQLite, the HTTP cache and site storage are off, and the SQLite one corrupts a
profile it is turned on over; and a built-in extension could give Lockstore a key from the vault without a Gecko
build, read in the source and unmeasured. Stage 2's rounds are in `ROADMAP.md`. The S24 waits for a vault again: its
old one was destroyed when its encri became a debug build.

### A key of the spike's own in the vault, and no private thumbnail on disk (2026-11-16)

The spike keeps a key of its own, `encriox.key`, 32 random bytes in the folder the vault allows it, held in memory
while the vault is open, and seals the logins' and the cards' keys with it, where a release of Fenix kept them in
plaintext (W6, patches 0015 to 0017). On the S9 and the A20 no saved login or card, and no key, was on disk in the
clear, and logins asked for before the vault was open arrived after its key. On the G6, with its vault locked, they
waited and nothing was written. A private tab's thumbnail is kept in memory, so a private session left nothing of a
page on disk on all three phones (0013, 0014). Three points are flagged for veto in W6.

**Fixed:** Android Components' logins and autofill stores waited on themselves for good once their key was lost,
which old canaries set off on the S9 (0018). A value the vault's key cannot open is answered as gone, as Fenix
answers one Keystore cannot, instead of throwing where callers do not catch (0019). `check-plaintext` waited for good
when the phone's screen was locked, and let a page that had not reached every store go on.

**Found:** Fenix fetches Mozilla's sponsored tiles and Nimbus's experiments outside private browsing, into Gecko's
normal cache; a bookmark from a private tab goes into `places.sqlite`; and Gecko offers to save a login from one. The
three ways to keep the browser's key are in `open-questions.md`.

**Known issues:** a lock leaves the logins' and the cards' keys in memory until the process ends (`ROADMAP.md`,
*Bugs*). The S24 is owed.

### Stage 1 on the S9 and the A20, and a stopped download the vault kept (2026-11-15)

On the S9 and the A20 too, a private session left no page's marker on disk, and a download went into the vault at a
third to a half of MediaStore's speed: about 57–65 MiB/s on the S9 and 28–35 MiB/s on the A20 (W5). Each vault had no
folder, so each was allowed whole, as the G6's was (W5, flag 1).

**Fixed:** a download with no length, paused or cancelled, was saved cut short into the vault, and *Try again* from
its notification asked for the rest and failed every time; now nothing is kept and a retry starts from the start
(patches 0007 and 0008, `check-stopped`; W5, which amends W4). The gate trusted a held session after a lock (0010).
The plaintext control could run privately, because the gate's setting was lost when the app was stopped (0012). The
loopback server could kill the app when a client went away (0007). `check-plaintext` and `measure` went on where they
should have stopped, and `check-plaintext` left its temporary directory behind. A comment of patch 0006's was
misplaced (0011). The spike has a brief, `.claude/rules/spike.md`.

**Found in the source:** a release can set Gecko prefs through `configFilePath`, and Lockstore is in the published
GeckoView, so stage 2 could start without a Gecko build (W5). Neither is measured.

**Known issues:** a stopped vault download shows as failed (`ROADMAP.md`, *Bugs*). The G6's vault is locked, and the
passphrase on file does not open it, so patch 0010's locked path is unmeasured. The S24 is owed.

### The stage 1 spike, on the G6 (2026-11-14)

Fenix at Firefox 160.0 builds with Gradle against the GeckoView Mozilla published, with six patches and encri-api from
source (`scripts/spike/stage-1/`). On the G6, a private session left no page's marker on disk, where a normal one left
it in ten files, and a download went into the vault at about 35 MiB/s, a third of Fenix's own writer (W4). Private
thumbnails, a paused download and Gecko prefs need work in stage 1. The S9, the A20 and the S24 are owed.

### encriox and encrium (2026-11-14)

The user wants a second browser, on Chromium, so the two needed names that tell them apart. This one is encriox and
the Chromium one encrium (W3), each keeping *encri* whole and following the forks' habit of animal names for Firefox
and element names for Chromium. W1's rejection of Chromium is amended: it is not this repository's browser, but it is
the second one, a project of its own.

### A Firefox fork, licensed MPL-2.0 (2026-11-13)

The user picked the study's scenario 1, a Firefox fork reaching the vault through encri-api (W1), and MPL-2.0 for
every file (W2). `LICENSE` holds the licence's text. The stage 1 spike comes first and the stage 2 spike next, ahead
of the name and the package, which the study had put first. The study's open questions moved to
`docs/decisions/open-questions.md`, and `scripts/docs/` came from encri-backup, reading W numbers, to generate the
decision index.

### The documentation takes the suite's common shape (2026-11-11)

The directory moved into the suite's superproject, whose `docs/documentation.md` sets one layout for every
repository. `AGENTS.md`, empty until now, is the level-0 guide, holding only what the study sourced. `ROADMAP.md`
follows the study's recommended path, after the pick it waits on. `README.md` and this file are new. The directory
became a git repository, on `master` with its siblings' signing settings and no remote, and W is the decision prefix,
both at the user's word.

### A browser for encri, studied (2026-11-10)

`docs/feasibility-study.md` weighs three scenarios and finds scenario 1, a Firefox fork reaching the vault through
encri-api, the most feasible: its first stage builds with Gradle against Mozilla's GeckoView, and its second rides on
the at-rest encryption Mozilla is writing. Nothing is decided until the user picks.
