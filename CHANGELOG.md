# encriox — changelog

What happened, newest first. Why is in `docs/decisions/`, and what is next is in `ROADMAP.md`.

## Unreleased

### Pushed (2026-11-26)

On the user's word, this repository's history is on its public remote, `syshlted/encriox`, as `master`, every commit
signed. Then, on the user's word, `master` was made GitHub's default branch, and the remote's `main` deleted, which
held only GitHub's first commit of the same `LICENSE`. The Firefox checkouts' branches, which the patches are
exported from, have no remote and are not pushed.

### The repository is encriox (2026-11-25)

On the user's word, the repository takes the app's name: its directory in the suite is `encriox/`, as its GitHub
remote, `syshlted/encriox`, already was, and it is cited as *encriox W1*. Its titles and the suite's pointers to it
changed with it. What was written before, in the changelogs and the decision records, still says *encri-firefox*,
which is this repository. The patches say encriox: every commit of the four series, on the checkouts' branches, was
written again with the name changed in its comments and its message, signed, with its author and dates kept, and the
patches written out from them, each series checked to apply to the same tree as its branch. Nothing but comments and
messages changed, so the spikes' builds measured before are of the same code.

### A frozen browser keeps its keys through a lock (2026-11-25)

On the S24, with its owner there, a new check, stage 2's `check-frozen`, measured what a lock does to a browser
Android 14 and later have frozen in the background (W27): frozen 71 seconds after the home screen, with encri's process,
the spike kept every key through a lock for 3½ minutes, until it was opened, then closed in a third of a second and
kept what its page wrote. The lock was Android's low-memory killer ending encri's frozen process, which locks the vault.
On the user's word, encriox will keep a foreground service of its own while it holds the vault's keys, so that it is
never frozen and a lock closes it at once, and close after five minutes in the background; it is built in the spike
first. Whether the S24 writes swapped memory to its storage, which bears on every key in memory, is to be read.

### encriox's series begins (2026-11-23)

On the user's word, encriox has a series of its own (W26): `firefox-release` names the Firefox release it is built on,
160.0, and `scripts/fetch` and `scripts/build` make a checkout of that tag with `patches/` committed on it and build
it, Gecko and application-services' megazord in tree, then Fenix. Its first six patches are stage 3's sealing of
application-services' databases, carried from nine of the spike's commits with their fixes folded in: obfsvfs, the
sealed opens of places, the logins and autofill, Android Components' database keys, SQLite's recovery, and the check
and repair. Nothing else of Fenix's is changed yet, so a build keeps its databases in the clear and is checked, never
used; the stores' keys and the prompt at start need `.encriox.key` and encri's unlock, which come with the rest of the
spikes' patches. History and bookmarks needed nothing turned on: W17's plan to have them off was never built, and on
the G6 a build of the series recorded a page in History, kept its bookmark, and listed both again in a new process,
in places' database through obfsvfs. Found and fixed on the way: Fenix's debug build was killed at its first start on
the G6 by its own StrictMode, for LG's framework, which stage 1's fix for it, now encriox's patch 0007, ignores; and a
comment in places' sealed open said its database's name carries the key, which stage 3's patch 0005 had made a handle
(stage 3's patch 0018). The first build took 80 minutes, and `fetch` 17.

### A prompt at start for a sealed database that does not open (2026-11-22)

On the user's word, stage 3 reads each of its three sealed databases through under its key at start, before its store
opens it, and asks the user about one that does not open (W25, stage 3's patches 0010 to 0017): a damaged one may be
repaired, deleted or kept, one under another key is only kept, and one in the clear may be deleted or kept; nothing
is deleted unasked, and the question says that, with Sync off, nothing deleted or lost comes back. The repair is
SQLite's own recovery, now compiled into the megazord, copying what it can read into a new database sealed under the
same key. On the G6, each answer tapped on the screen: places with page 1 damaged was kept with no file changed, asked
about again, and could not be repaired, which was said; places with a page deep in it damaged was repaired, 3,813 rows
kept, and took new ones; the logins damaged were deleted and their store started again empty; autofill under another
key was offered no deletion; and the three in the clear were each deleted. The check reads every page: 76–118 ms for
places at 6.8 MB checked again in a process, and 392–628 ms first in a new one, beside Gecko's start. Found and
fixed: Keep at first let the store open its damaged database as it always had, and the failed
open's close checkpointed the log into the damaged file and deleted the log; the repair failed on places' foreign
keys; and a second tap could answer the next question with Delete.

### A key check in each sealed page (2026-11-21)

On the user's word, each page stage 3 seals now carries, in its 4 spare reserved bytes, a check of its database's
key, so that a wrong key is told from a damaged file (W24, stage 3's patch 0008). On the G6 and the A20, with the
build's own NSS: every page of places', the logins' and autofill's databases and their logs carries one check, each
database its own; each opened under another's key is refused at the open, *authorization denied*, and nothing of the
three changed; a sealed places database with its log's last frame torn by hand lost that commit alone, and with its
first frame torn read what the database held; and its page 1, or every other page, damaged failed as *disk I/O
error*, never as the wrong key, with the database left as it was. Found and fixed: a log whose first frame a crash
tore, the first commit after a checkpoint, made the whole database an I/O error; and `check-sealed`'s keyless step
counted the logs' indexes, which Fenix's warm-up of its stores rebuilds at every start. Next is the prompt at start
for a sealed database that does not open, which tells the two by these answers.

### The S24 measured, and checks that fail a copy cut short (2026-11-20)

On the user's word, the spike's owed rounds ran on the S24, the user's own phone, Android 17, its vault never locked
by a check (W23). Stage 1 held: no page in plaintext after a private session, downloads into the vault at about
125 MiB/s, the six stopped downloads as they should be, and the logins' and the cards' keys sealed. Stage 3 held as on
the other phones, `check-kek` among it; the cipher costs 1% of a visit's write there, and pages over TLS agree
X25519MLKEM768 with the megazord on Gecko's NSS. Found and fixed: the checks' copy of the app's data could end short
with nothing said, which reads as no marker anywhere, so `copy-data` now fails a copy that did not finish, and the
private and sealed runs were made again with it on the G6, the A20 and the S24, and held; `check-keys` could take a
process WorkManager had started for a new one; and Android 17 froze the bench in the background, which now keeps its
process unfrozen. Unmeasured: what a lock does to a browser Android has frozen, which needs the S24's vault locked.
The user also gave the word for a key check in each sealed page, next (W22's amendment).

### The G6 measured, and the user's answers to W21 (2026-11-20)

With the G6's vault unlocked by its owner, the spike's owed rounds ran there, none of them locking it. Stage 1's
stopped downloads held in all six cases (W19). `check-kek` passed every step on stage 3's build, its step 5 for the
first time: the browser closed when the vault's key was dropped, and the page's stores read back whole (W20). Stage 3
held as on the A20 and the S9: the marker in no file, all four records read back, a keyless open refused, and logins
waiting 1.5 s for their key; the cipher costs 5% of a visit's write. Found and fixed: on Android 9 the bench made its
`plain-8k` databases with 4 KiB pages (stage 3's patch 0007). The user's answers to W21 keep its six flags (W22): no
data in the clear at rest; a prompt at start, in item 6, lets the user delete a sealed database that does not open,
never deleted unasked; and changing a database's key comes before the first release. On the A20 the spike's TLS
agreed X25519MLKEM768, Gecko 160's hybrid post-quantum key exchange. The S24 is owed.

### History, bookmarks, logins and cards are sealed (2026-11-20)

**Decided, built and measured** in the spike's stage 3, on the user's word that the first usable build waits for it
(W21). Firefox 160's tree carries application-services and builds its megazord in tree, so stage 3 is six patches on
stage 2's, not a fork: sql-support gains an obfsvfs of its own, after Gecko's, sealing each page, its log's and its
journal's, with AES-256-GCM from NSS; places, the logins and autofill open their databases sealed; Android Components'
stores take a database key and wait for it; and the spike's Fenix derives one for each from the vault's key. On the A20
and the S9 the marker is in no file of the app's, all four records read back in a new process, an open without the key
is refused and changes nothing, and logins asked for before the vault opens wait for it. The cipher costs 4–10% of a
visit's write and nothing to read; with smaller pages than places' own, sealed places writes faster than places as
Mozilla makes it, in half the disk. Found and fixed on the way: a sealed database's key carried in its name would have
reached the log and the error reporter in the message of a failed open, and a component given no key could have
deleted a sealed database. Six points are flagged for veto. The first usable build carries history and bookmarks.

### A plan for history and bookmarks in a sealed store (2026-11-20)

At the user's word that it comes next, `docs/sealed-history.md` weighs how to give back history and bookmarks sealed
with the vault's key. Places keeps them in application-services' own SQLite, which takes a path and no key, and drops a
URI's query before it opens. Five ways are weighed; the plan recommends Gecko's obfsvfs, ported into
application-services' SQLite, keyed from the vault's key, for places and for the logins' and the cards' databases too,
whose origins and dates are plain columns. It asks the user three things first, and starts with a spike round that
builds application-services itself. ~~Nothing is decided.~~ Decided and built the same day (above).

### encriox closes at a lock (2026-11-20)

**Built and measured**, W17's first recommendation in the spike, at the user's word that the bugs come first (W20):
at a lock of the vault the browser finishes its windows and shuts Gecko down while Gecko still holds the profile's
keys, and Gecko ends the process itself (stage 2's patch 0012). On the A20, every process of the session that held
its pages or keys ended within 2.2 to 3.8 seconds, the browser left the recent apps, and local storage written just
before the lock read back; patch 0011's build left 6 or 7 processes running 30 seconds after each of six locks. The
close checkpoints Gecko's write-ahead logs into their sealed databases. `ROADMAP.md`'s S3 is fixed. S1, local storage
lost at a lock, was not seen again in three runs with the lock 1.5 seconds after the write, one of them W15's own
script, so the 5 seconds W15 inferred are not its cause; it is closed unreproduced, flagged for veto. Found: Samsung's
process manager on the A20 killed encri in the background four times in eleven minutes, which locks the vault and so
closes the browser; that is encri's to weigh. `check-unlock` and `check-kek` now require the close, and their settles
keep the browser in front.

### A stopped download into the vault shows as stopped (2026-11-20)

**Fixed.** `ROADMAP.md`'s S4, at the user's word that the bugs come before anything else (W19): a download into the
vault that was paused or cancelled no longer shows as failed. Android Components' download service leaves it paused
or cancelled when the vault's writer throws to abandon it, and *Resume* starts one nothing was kept of from the start,
where it had looked for a partial file in `Download/` and failed (stage 1's patch 0020). On the A20, `check-stopped`,
which now fails on a stopped download shown as failed, failed on patch 0019's build in every case and passed on 0020's:
paused and cancelled, with a length and without, and resumed whole, 16777216 bytes. W5's first rejection is amended.
The G6 and the S9 are unmeasured.

### Published on Google Play alone, the name kept, and a public remote (2026-11-20)

**Decided**, at the user's word. encriox is published on Google Play alone (W18): encri is there alone (encri D142),
so F-Droid would serve no one who could open a vault with it. MPL-2.0 asks for the source, not for a second place to
get the app, so each release's source is on the public remote, and the Play listing and the About screen say where;
no APK on GitHub's releases and a tag for each release are flagged for veto. No clearance search by counsel is run:
encriox keeps its name on the knock-out search, and a cease-and-desist renames it (W3's amendment). The user made the
public remote, `syshlted/encriox` on GitHub, which holds only GitHub's first commit; it is this repository's `origin`,
and nothing is pushed yet. The order of work changes (W17's amendment): the spikes' three bugs first, then history
and bookmarks in a sealed store, then the rest of the first usable build. App-level security controls stay on the
roadmap, for later.

### encriox's first usable build, decided (2026-11-19)

The user accepted the plan's seven recommendations whole (W17): encriox closes at a lock, letting Gecko write what it
holds before its keys are locked; Fenix's own records of pages are sealed with the vault's key or kept in memory, with
no history and no bookmarks in the first build; Mozilla's services are off but for Remote Settings' lists and Safe
Browsing; encriox's own key is made on the key machine before its first build, and its registry entry filed once it
exists; its version is the Firefox release and its own count; a vault's key that does not open the profile stops it;
and its checks run on the S24. Six open questions are answered, and the work is `ROADMAP.md`'s item 6. Nothing of it
is built yet. Three points are flagged for veto.

### A plan for encriox's first usable build (2026-11-18)

At the user's request, `docs/first-usable-build.md` plans encriox itself, built from this repository, signed with its
own key and used every day on the user's phones, after the spikes. It asks the user seven things first, with a
recommendation for each: that encriox close at a lock; that Fenix's own records of pages be sealed with the vault's key
or kept in memory, with history and bookmarks off in the first build; that Mozilla's services be off but for what keeps
the browser safe; that encriox's key be made before its first build; its version; what to do when the vault's key does
not open the profile; and the phones, the S24 among them. Then the work, in order and sized: about 20 of the spikes'
30 patches are encriox's. Nothing is decided.

### Stage 2's third round: what obfsvfs costs (2026-11-18)

At the user's word, stage 2's Gecko times the same work on plain SQLite and through obfsvfs, in its own SQLite and
mozStorage, outside the profile (`scripts/spike/stage-2/`, patch 0011, W16). On the G6 and the A20, **obfsvfs took
twice plain SQLite's time to scan a table or read rows scattered across it**, wherever a page came from the file
rather than mozStorage's cache, a third more to write in bulk, and nothing for small commits, whose 8 KiB pages write
less than plain's 32 KiB. A page took 24 µs to decrypt on the G6 and 58 µs on the A20. **A site's IndexedDB showed no
cost** above its own variation, on a plain build against stage 2's. Three points are flagged for veto. The S9 was not
attached.

**Found:** the SQLite switch alone puts every database under the profile through obfsvfs, a site's IndexedDB, local
storage and Cache API among them, with QuotaManager's switch off; QuotaManager's switch is what encrypts the files
beside them, as the Cache API's bodies. So a plain IndexedDB needs a build whose SQLite default is off, and the round's
first design, QuotaManager's switch on against off, compared obfsvfs with obfsvfs.

### encriox does not start without the vault (2026-11-18)

At the user's word, W14's flags stand, a test vault may be locked to measure a lock, and **encriox does not start
without the vault** (W15). With the vault locked at startup, stage 2's spike brings up encri's unlock while Gecko
waits; if the vault stays shut, the browser ends before Gecko has opened anything of the profile (patch 0010). On the
A20, `check-unlock` locked the vault for real and unlocked it by typing the passphrase on file into encri's screen.
**A real lock locked Gecko's keys** as the gate's `drop` had, and the unlock unlocked them. **A start with the vault
locked** brought up the unlock first, and only Gecko's startup caches changed while it waited. **A refused one ended
the browser**, out of the recent apps, with the profile as it was; on the G6 too, whose vault stays locked. Four
points are flagged for veto.

**Found:** a lock seconds after a page wrote lost its local storage, once in two runs on the A20 (`ROADMAP.md`,
*Bugs*). Gecko flushes local storage 5 seconds after a write, and an origin's first flush needs a key that cannot be
made while the profile's keys are locked. The G6's vault did not open with the passphrase on file typed quoted either,
and a vault made again there would need its owner's screen lock, so it was not wiped.

`check-unlock`'s first run stopped behind the A20's keyguard, which had come back; its `wake` now retries until the
keyguard has gone.

### Stage 2's second round: the vault's key keeps the profile's keys (2026-11-18)

At the user's word, stage 2 keeps every key of Gecko's profile under a password KEK whose secret is an HMAC of the
spike's key in the vault (`scripts/spike/stage-2/`, W14). At 160 nothing lets an app give Gecko such a secret before
the profile opens, so stage 2's Gecko asks the app for it at startup, on Gecko's own thread, and the app opens the
vault to answer, or answers with nothing when the vault is locked. **Moving the keys under the secret razed two
stores** at the next start, measured on the S9: each of Lockstore's stores asked for its key by the name of the
LocalKey it had made it under, so the cookie store moved its database aside as corrupt and the HTTP cache replaced its
key. A patch has each store follow its key, and keep a locked one rather than replace it.

With it, on the S9 and the A20, **the profile opens again with all six stores, and no key is kept in the clear. A
start without the secret razes nothing**: cookies and permissions run in memory, sites' storage and the disk cache are
not used, and every store reads back at the next start with it. **A lock mid-session stops only what needs a new
key**: the cookie store and the HTTP cache, which hold theirs, went on writing. The app locks and unlocks Gecko's keys
as the vault's key is dropped and read again. On the G6, whose vault stays locked, the profile was made in the clear,
so Gecko must not make a profile before the vault first opens.

**Found and fixed: a moved key stayed on disk.** A profile made without the secret had its keys moved under it at the
next start, and their LocalKeys deleted. The S9's Lockstore log still held 83 copies of those LocalKeys, raw keys and
all, which read every database. Lockstore now truncates its write-ahead log when it deletes a key, and none was left.
Gecko waits 0.7 s for the vault on the S9 and 1.7–1.8 s on the A20, and spends 250–270 ms and 530–630 ms unlocking
the KEK, at every start. Mozilla should know four things more (`ROADMAP.md`, item 5). Five points are flagged for veto.

`check-kek` makes a fresh profile and wakes the phone before its first start, and sets the gate's `kek` before a
fresh profile: the gate's own start had met stage 1's profile on the A20, and taken the old setting on the S9.

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
