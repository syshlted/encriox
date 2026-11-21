# encri-firefox — roadmap

What is next, and in what order. Why is in `docs/decisions/`, and what happened is in `CHANGELOG.md`. When they
disagree, the decision records win.

## Severity

| | Meaning |
|---|---|
| **S1** | Data loss, a confidentiality breach, or a vault that will not open |
| **S2** | A user-visible wrong answer or failure, with no workaround |
| **S3** | Wrong under a narrow race, or an honest limit that needs disclosing |
| **S4** | Waste, cosmetics, or documentation that contradicts the code |

Severity is about the consequence if it happens, not the odds of it happening.

## Now

1. ~~**The user picks a scenario** from `docs/feasibility-study.md`, whose *Verdict* finds scenario 1, Firefox through
   encri-api, the most feasible. The pick is W1. Nothing below starts before it.~~ **Done 2026-11-13 (W1)**: scenario
   1, with MPL-2.0 for every file (W2).
2. ~~**This directory becomes a git repository**, with its siblings' conventions (`AGENTS.md`), before W1 is committed.~~
   **Done 2026-11-11**, with W as the decision prefix.

The study's *Recommended path* is the order, except that the spikes now come before the name (W1). Its reasons and
sources are there, and are not repeated here:

3. **Spike stage 1 on the phones**, under the throwaway package `com.systemhalted.encri.browser.spike` (W1, flag 1).
   Fenix against a published GeckoView, every tab private, downloads through `openWrite`. Measure download throughput
   through the pipe against MediaStore on the S9, the A20, the G6 and the S24, and confirm that nothing of Gecko's
   reaches `filesDir` or `cacheDir` in plaintext. The S24 only with its owner's word. ~~The G6~~ **Done on the G6
   2026-11-14 (W4)**: built (`scripts/spike/stage-1/`), no page in plaintext after a private session, downloads into the
   vault at about 35 MiB/s. ~~Owed: the S9, the A20 and the S24, with `check-plaintext` and `measure`.~~ **Done on the
   S9 and the A20 2026-11-15 (W5)**: no page in plaintext there either; downloads into the vault at about 57–65 MiB/s
   on the S9 and 28–35 MiB/s on the A20, a third to a half of MediaStore's speed; and a download stopped part way,
   which the vault had kept cut short, now leaves nothing. **Then W6, 2026-11-16**: a private tab's thumbnail is kept
   in memory, so a private session leaves nothing of a page on disk on all three; and the logins' and the cards' keys
   are sealed with a key of the browser's own in the vault, `encriox.key`, measured on the S9 and the A20, with the
   locked path on the G6. Owed:
   - ~~**the S24**, with `check-plaintext`, `measure`, `check-stopped` and `check-keys`, on its owner's word and once it
     has a vault again: its old one was destroyed on 2026-11-17, when its encri gave way to a debug build (the
     suite's `CHANGELOG.md`);~~ **Done on the S24 2026-11-20 (W23)**, on its owner's word: no page in plaintext after
     a private session, downloads into the vault at about 125 MiB/s, and the stopped downloads and the keys as on the
     other phones;
   - **patch 0010's locked path, on the G6**, once its owner has unlocked its vault, which the spike locked and the
     passphrase on file does not open (W5): open the gate, lock with `--ez lock true`, and open it again, which must
     ask to unlock. Typed quoted, so that its space and its `#` would reach encri, it still did not (W15), and a vault
     made again there would ask for its owner's screen lock too, so it was not wiped. Stage 2's own lock and unlock
     ran on the A20 instead (W15).
4. **Spike stage 2 before writing any of it**: GeckoView with Lockstore's prefs on, on each phone. ~~Pick the branch,
   release or ESR, before it.~~ **Release, picked 2026-11-17 (W7)**, from stage 1's Firefox 160.0. A Gecko build needs
   at least 30 GB of disk by Mozilla's guide, and ~~the machine had 63 GB free on 2026-11-13, and 42 GB on
   2026-11-15~~ **stage 2's go on `/home/coder/extra_space/encri-extraspace`, 387 GB free on 2026-11-17** (W7).
   **Lockstore is in the GeckoView Mozilla published** (W5, read in its `libxul`), so its first questions are asked
   of stage 1's build with prefs alone; ~~only a key-encryption key from the vault needs a Gecko build~~ a
   key-encryption key from the vault may not need one either (`open-questions.md`, read in the source 2026-11-17;
   unmeasured). Its rounds, on the G6, the S9 and the A20:
   - ~~**What the prefs cover**, with Lockstore's own local key: `security.storage.encryption.sqlite.enabled`,
     `browser.cache.disk.encryption.enabled` and `dom.quotaManager.encryption.enabled` on, all off by default at 160,
     on a fresh profile, and `check-plaintext <phone> normal`. Which stores still hold the marker, and does the
     profile open a second time?~~ **Done 2026-11-18 (W12)**, `scripts/spike/stage-2/check-lockstore`. The HTTP
     cache's switch works with a pref alone. Site storage's encrypts, but on the S9 and the G6 it recorded the origin
     as plaintext, so the next start could not read local storage, IndexedDB or the Cache API, every site's by
     Gecko's log. A missing flag in Gecko's QuotaManager is the cause, at 160 and on `main`, and nothing was wiped.
     SQLite's switch is locked off in `all.js`, so a pref cannot turn it on. ~~**The user's call: a Gecko build** on the
     extra drive, carrying the one-line QuotaManager fix and SQLite's switch unlocked, before the next two rounds, or
     waiting for Mozilla (item 5).~~ **A Gecko build, done 2026-11-18 (W13)**, `scripts/spike/stage-2/build`, on the
     extra drive: QuotaManager's fix, and SQLite's switch unlocked and on by default, because Gecko's launch guard
     reads the build's default and nothing else. On the G6, the S9 and the A20 all three switches work: every Gecko
     database encrypted but NSS's and Lockstore's own, the origin recorded encrypted, everything read back at the next
     start, and the marker only in Fenix's saved tabs and history and in `serviceworker.txt`. The next two rounds
     build on it.
   - ~~**A key-encryption key from the vault**: the profile's keys moved under a password key whose secret comes from
     `.encriox.key`, and locked at the vault's lock. Does the profile open a second time, and what does each store do
     when that key is locked at start or mid-session? Razing is the failure to look for.~~ **Done 2026-11-18 (W14)**,
     `scripts/spike/stage-2/check-kek`, with five more patches. Gecko asks the app at startup for a secret, an HMAC of
     the spike's key in the vault, keeps every key of the profile under it, and each store follows its key there and
     keeps a locked one. On the S9 and the A20 the profile opened again with all six stores and no key in the clear; a
     start without the secret razed nothing; a lock mid-session stopped only what needed a new key. Without the
     stores' patch, the move razed the cookie store and the HTTP cache; and Lockstore's write-ahead log kept the moved
     keys' old LocalKeys, `kek_bytes` and all, until patch 9.
   - ~~**The user's answers to W14**~~ **Done 2026-11-18 (W15)**: its flags stand; a test vault may be locked, and
     wiped; and encriox does not start without the vault. Patch 0010 brings up encri's unlock at startup while Gecko
     waits, and ends the browser if it is refused, before Gecko has opened anything of the profile
     (`scripts/spike/stage-2/check-unlock`). On the A20 a real lock of the vault locked Gecko's keys as the gate's
     `drop` had, the passphrase typed into encri's screen unlocked them, and a refused start left the profile as it
     was; on the G6 too. A lock seconds after a page wrote lost its local storage, once (*Bugs*).
   - ~~**What `obfsvfs` costs** against plain SQLite.~~ **Done 2026-11-18 (W16)**,
     `scripts/spike/stage-2/check-obfsvfs`, with patch 0011, on the G6 and the A20. In Gecko's own SQLite, twice
     plain's time where a page is read from the file, a scan or reads scattered across a table, a third more to write
     in bulk, and nothing on small commits, whose 8 KiB pages write less than plain's 32 KiB: 24 µs to decrypt a page
     on the G6, 58 on the A20. A site's IndexedDB showed no cost above its own variation. Found: the SQLite switch
     alone puts every database under the profile through obfsvfs, site storage's too. The S9 was not attached.
5. **Ask Mozilla before carrying a patch**: an embedder-supplied KEK in GeckoView, beside the OS-keystore KEK of bug
   2055641. Who asks, and where, is open. Theirs to know too: Android Components' logins and autofill stores wait on
   themselves for good once their key is lost, which the spike carries a fix for (W6, patch 0018); QuotaManager's
   `GetOrCreateTemporaryOriginDirectory` records an encrypted origin as plaintext, so that with
   `dom.quotaManager.encryption.enabled` on, the next start cannot read site storage (W12), which stage 2's patch 0001
   fixes (W13); and Gecko's launch guard reads the SQLite switch before any pref is loaded, so that a profile a pref
   encrypted is refused at its next start (W13). From W14, all measured but the last two, read in the source:
   Lockstore's stores ask for their keys under the KEK they made them under, by name, so that keys moved under a
   password KEK cannot be found, and the cookie store moves its database aside and the HTTP cache replaces its key
   (stage 2's patch 0007); Lockstore answers a locked key and a missing one with the same error; `secure_delete` leaves
   a deleted key in Lockstore's write-ahead log (patch 0009); Mozilla's own adoption of a primary password moves the
   HTTP cache's key the same way, and its prompt cannot work on GeckoView, whose `promptPassword` calls a method that
   does not exist.
6. **The first usable build** (W17; `docs/first-usable-build.md`): encriox itself, built from this repository, signed
   with its own key and used every day on the user's phones, before anything is published. The user accepted the
   plan's seven recommendations: encriox closes at a lock; Fenix's own records of pages are sealed with the vault's
   key or kept in memory, ~~with no history and no bookmarks in the first build~~ and history, bookmarks, logins and
   cards in sealed databases (W21); Mozilla's services are off but for
   Remote Settings' lists and Safe Browsing; encriox's key comes first; its version is the Firefox release and its own
   count; a key that does not open the profile stops it; and its checks run on the S24. In order, sized as the plan
   sizes them (S, M, L), with the first two on the user's word of 2026-11-20 (W17's amendment):
   - ~~**The spikes' three bugs, before anything else** (*Bugs*, below)~~ **Done 2026-11-20**: a stopped download
     into the vault shows as stopped (W19, stage 1's patch 0020); and the browser closes at a lock (W20, stage 2's
     patch 0012), with what Gecko's shutdown writes measured, which ends S3. S1 was not seen again, and the state it
     was seen in is gone. Measured on the A20; ~~the G6 and the S9 are owed~~ **on the G6 too, 2026-11-20**, W19's
     six cases and W20's close at the vault's key dropped (`check-kek`'s step 5); the S9 is owed.
   - ~~**History and bookmarks in a sealed store**, next~~ **Decided and built in the spike 2026-11-20 (W21)**, on the
     user's word: Gecko's obfsvfs, ported into application-services' own SQLite, seals places', the logins' and
     autofill's databases with keys from the vault's, and the first usable build waits for it and carries it. Firefox's
     tree builds application-services in tree, so it is stage 3's ~~six~~ seven patches on stage 2's, in one series,
     not a fork. Measured on the A20, the S9 and the G6: the marker in no file, all four read back, a keyless open
     refused with nothing changed, and sealed places faster to write than plain. ~~Six points flagged for veto.~~ Its
     six flags stand (W22). ~~Owed here: the G6 and the S24; pages over TLS with the megazord on Gecko's NSS;
     `check-unlock` with the databases watched.~~ Pages over TLS and `check-unlock` with the databases watched, done on
     the A20 (W21). ~~Owed here: the S24.~~ **On the S24 too, 2026-11-20 (W23)**, with `check-kek`; `check-unlock`,
     which locks the vault, was not run there.
   - **What a lock does to a browser Android has frozen** (W23): on Android 14 and later a cached app's process is
     frozen, and a lock reaches it only when it is unfrozen. Measure it on the S24, which needs its vault locked, so
     its owner's word and its owner there to unlock it; then whether encriox keeps itself from being frozen while it
     holds the keys, by a foreground service of its own, or closes as it goes to the background. S.
   - **History and bookmarks on in encriox's menus**, which W17 had off, and Fenix's history delegate set, S; and the
     stage 3 patches carried into encriox's series with the in-tree build in its mozconfig, S.
   - ~~**A key check in each sealed page, next** (W22's amendment, the user's word of 2026-11-20): in obfsvfs, a check
     derived from the database's key in each page's 4 spare reserved bytes, so that a wrong key is told from a damaged
     file, measured with a log torn by a crash and a page that does not open under the right key, on a phone. S.~~
     **Done 2026-11-21 (W24)**, stage 3's patch 0008, on the G6 and the A20 (`check-keycheck`): a key not the
     database's is refused at the open, *authorization denied*, changing nothing; a torn log opens, its last frame
     losing that commit alone and its first reading what the database holds, where before a torn first frame made the
     database an I/O error; and a damaged page is *disk I/O error*. Four points flagged for veto.
   - **A prompt at start for a sealed database that does not open, next** (W22): which store it is, and the user's
     choice to delete it and start that store empty, ~~or~~ keep it and go on without it, or, where some pages still
     open, repair it into a new sealed database with what SQLite's recovery can read; never deleted unasked, and never
     offered for deletion under a key that is not its own. With Sync off (W17), the prompt says a deleted store does not
     come back. S.
   - **The repository and the build**: `patches/`, `scripts/` and the file naming the Firefox release (W9), S; the
     series moved onto the newest release tag, the first measure of a rebase's cost, M; a build type of encriox's
     own, minified, Gecko optimised, without Fenix's `sharedUserId`, with a `.debug` type beside it, M; the branding,
     every string and icon that names Firefox, M; Gecko's prefs as the build's defaults, S.
   - **The spikes' patches that are encriox's**, about 20 of 30, rewritten without the gate and with `.encriox.key`,
     M.
   - **What the spikes left**: closing at a lock, carried from the spike's patch 0012 (W20); Fenix's records of
     pages, L;
     Mozilla's services off, with a check that a session reaches no other server, M; a key that does not open the
     profile, S; the About screen's line, naming the Firefox release and where the source is (W9, W18), and the
     version, S.
   - **The checks**, from the spikes' `check-plaintext`, `check-lockstore`, `check-unlock` and `check-stopped`, run by
     the script that follows each Firefox release (W9), M.
   - **The key and the registry**: ~~the user makes encriox's key on the key machine with encri's kit (encri D141)~~
     **made 2026-11-20**, `encriox-app-signing`, with a transparency key of its own, by the suite's key tool, each
     password in its store and seen by no one (the superproject's `keys/`); then its entry in encri-api's registry,
     `registry/com.systemhalted.encriox.app` with a floor (W8, encri D130).
   - **The first usable build**, signed, on the S9, the A20 and the S24, the S24's rounds each named to the user
     first (W17, flag 1), and used for some days. Then a signed build at every Firefox release, every two weeks.

## Before the first release

- ~~**A plan for the first usable build**, drafted 2026-11-18 in `docs/first-usable-build.md`.~~ **Accepted 2026-11-19
  (W17)**, as item 6.
- **Settle what is permanent**: ~~the product's name, which cannot contain "Firefox";~~ **named encriox
  2026-11-14 (W3)**; ~~its package, on which a grant is keyed for good (encri D120, D145);~~ **`com.systemhalted.encriox`
  2026-11-17 (W8)**; and its registry entry in encri-api, `registry/com.systemhalted.encriox.app` with a floor, without
  which a user consents again at every release (W8, encri D130), filed in item 6 once encriox's key exists (W17).
- **Following Firefox's releases** (W9): the file naming the release encriox is built on; the script that applies the
  series to the newest release tag, builds and tests, and stops at the first thing it cannot do; the About screen's
  line; and `README.md` and each release's `CHANGELOG.md` entry naming the release. ~~encriox's own version is
  open~~ Its version is the Firefox release and its own count (W17). The script and the line come in item 6.
- ~~**Where it is published**: GitHub releases, as the suite's open-source apps' builds are (encri D143), F-Droid, or
  Google Play, where encri is (encri D142). The user's to choose.~~ **Google Play alone, 2026-11-20 (W18)**, where
  encri is (encri D142), with each release's source on the public remote, as MPL-2.0 asks, and the Play listing and the
  About screen saying where. Two points flagged for veto: no APK on GitHub's releases, and a tag for each release.
- ~~**History and bookmarks in a sealed store**, which the first build goes without (W17)~~ **Moved into item 6,
  next after the bugs** (W17's amendment, 2026-11-20), **and built in the spike (W21)**: the first build carries them.
- ~~**A trademark search for encriox**, in the US, EU, international and Japanese registers (W3).~~ **A knock-out
  search, 2026-11-17** (`docs/trademark-search.md`): no ENCRIOX in TMview, which holds those registers' records, or in
  Trademarkia's US data; no use found; and `.com`, `.net`, `.org` and `.app` unregistered. The offices' own searches
  could not be queried. ~~Whether counsel runs a clearance search before the first public build is the user's to
  say.~~ **None, 2026-11-20** (W3's amendment): encriox keeps its name, and a cease-and-desist renames it.
- **A public remote** for this repository, before any build leaves the test phones (W2): ~~owed~~ **made by the user
  2026-11-20**, `syshlted/encriox` on GitHub, public, holding only GitHub's first commit of `LICENSE` on `main`. This
  repository's history is not pushed yet; its branch is `master` (`AGENTS.md`).
- ~~**How encriox keeps its own key**: a document in the vault, as the spike does (W6, flag 1), the same wrapped by a
  Keystore key of its own, or wrapped by encri for its package, which is a change to the contract
  (`open-questions.md`).~~ **A hidden document in the vault, `.encriox.key`, for now (W10).**
- **Changing a sealed database's key**, on the user's word, where it is thought compromised (W22): every page written
  again into a new database under a key whose label carries a count, which then replaces the old. It protects what
  is written after, not what the old key already opened. A rotation of `encriox.key` itself, which W6 never writes
  over, is larger and not planned. M.

## Later

- **App-level security controls**, which the user expects users to benefit from (W10), and kept on the roadmap at the
  user's word of 2026-11-20. With them, the two other ways of keeping encriox's key: wrapped by a Keystore key of its
  own, or by encri for its package (`open-questions.md`). Which controls they are is not decided.

## Bugs

- ~~**S1 — A lock in the 5 seconds after a page first writes local storage loses what it wrote** (W15).~~ **Not seen
  again, 2026-11-20 (W20)**: three runs with the lock at most 1.5 s after the write kept it, so the 5 seconds are not
  the cause, which is unknown. It was lost once, in a Gecko running on with its keys locked, and since patch 0012 the
  browser closes at a lock instead, keeping local storage written just before it. Closed unreproduced, flagged for
  veto.
- ~~**S3 — A lock leaves the logins' and the cards' keys in memory** until the process ends (W6, flag 3).~~ **Fixed
  2026-11-20 (W20)**, stage 2's patch 0012: the browser closes at a lock, and every process of the session that held
  pages or keys ends within 4 seconds, on the A20.
- ~~**S4 — A download into the vault that stopped shows as failed**, a cancelled one too, where Fenix's own writer
  shows it paused or cancelled (W5, flag 2). *Try again* starts it from the start.~~ **Fixed 2026-11-20 (W19)**, stage
  1's patch 0020: it shows as paused or cancelled, and *Resume* brings it whole from the start, on the A20.

What the records left open, the study's *Open questions* included, is in `docs/decisions/open-questions.md`.
