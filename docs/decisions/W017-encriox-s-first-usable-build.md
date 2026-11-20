# W17 — encriox's first usable build: it closes at a lock, seals or drops Fenix's records of pages, keeps only Mozilla's safety services, and has its own key first

*2026-11-19 · accepted on the user's word ("go with all your recommendations"), on the seven decisions of
`docs/first-usable-build.md`. **Flagged for veto on three points** (below). Answers `open-questions.md`'s *Lock
behaviour*, *History*, *Logins saved from a private tab*, *A secret that does not open the profile's KEK*,
*encriox's version* and *How a release sets Gecko prefs*, and the half of *What of Fenix's that reaches Mozilla* that
is Mozilla's. Nothing of it is built yet: its work is `ROADMAP.md`'s item 6.*

#### What the user decided

The plan's recommendation on each of its seven decisions, whole:

1. **encriox closes at a lock.** It shuts Gecko down with `GeckoRuntime.shutdown()`, letting it write what it holds,
   then locks the profile's keys and ends its process. The next start asks for the unlock (W15) and reopens the saved
   tabs. It ends the two bugs a lock has now: the logins' and the cards' keys left in memory (W6, flag 3), and local
   storage lost when a page wrote it just before (W15). Whether Gecko's shutdown writes that local storage before it
   ends is measured as part of the work.
2. **Fenix's own records of pages are sealed with the vault's key, or kept in memory, ~~and history and bookmarks are
   off in the first build~~** *(amended: history, bookmarks, logins and cards in sealed databases, W21)*. Open tabs
   saved for the next start and recently closed tabs are sealed; thumbnails, site icons and the downloads list are
   kept in memory; no history is recorded, as Fenix's history delegate is not set;
   bookmarks, collections and pinned top sites are off in the menus; Gecko's list of service workers is kept in
   memory or under Lockstore, whichever reading Gecko's source shows to be sound; ~~and the logins' origins, plain
   columns of application-services' database, are disclosed now and sealed later. History and bookmarks return with
   a sealed store of their own, after the first build: `places.sqlite` has no hook for a key.~~ and history,
   bookmarks and the logins' origins are sealed with application-services' databases (W21).
3. **Mozilla's services are off, but for what keeps the browser safe.** Off: telemetry, experiments, sponsored tiles
   and stories, crash reports to Mozilla, Firefox accounts and Sync, attribution, and push through Firebase. Kept:
   Remote Settings' lists, certificate revocation, blocked add-ons and the search engines among them, and Safe
   Browsing. Search suggestions are off by default.
4. **encriox's own signing key is made on the key machine, with encri's kit (encri D141), before its first build**,
   so that no build is ever reinstalled under another; and **its entry in encri-api's registry**, with a floor, is
   filed as soon as the key exists (W8, encri D130).
5. **encriox's version**: Fenix's version code, made from the build's time, which only rises, as the registry's floor
   needs (encri-api L15); and a version name of the Firefox release and encriox's own count, as *161.0-1*, beside
   the About screen's line (W9).
6. **A vault's key that does not open the profile** stops encriox, which says so, and offers behind a confirmation to
   start a new profile, which loses every site's data.
7. **The phones**: the first usable build is for the S24, the user's own phone, and its checks run there, as on the
   S9 and the A20; the G6, whose vault does not open, checks the refusal alone.

#### What follows from it

- **Gecko's prefs are the build's defaults**, in encriox's own Gecko: Lockstore's three switches, a private tab's
  media cached in memory, no login offered for saving from a private tab, and the services decision 3 turns off. No
  config file is read, so *How a release sets Gecko prefs* needs no answer from GeckoView.
- **About 20 of the spikes' 30 patches are encriox's** (the plan's table), rewritten without the spike's gate and its
  names, with the key under `.encriox.key` (W10).
- **encriox has a build type of its own**, from Fenix's release template, without the `sharedUserId` Fenix's beta and
  release types carry for Firefox for good, and a debug type, `com.systemhalted.encriox.debug` (W8, flag 1).
- **The user makes the key.** The key machine is the user's, and no build of the first usable one is signed before
  the key exists. The registry entry is encri-api's to review.

#### Why encriox's key is not encri's

encri, Encri Files and Encri Images share one app signing key and one code transparency key (encri D115, D140; Encri
Images I20). Asked on 2026-11-19 whether encriox should share them too, the answer recorded here is no, because it is
the browser:

- **It is the app most likely to be taken over.** It runs untrusted web code all day, through Gecko's native code.
  Whatever the shared key grants would reach it: every `signature` permission encri defines (one today, which
  `androidx.core` adds, D115), encri-api's trust in an encri signed by the same key (`EncriTrust.sameSigner`), and
  anything granted that way later.
- **It is signed far more often**: every two weeks, about 26 times a year, where the others are signed a few times.
  Each signing handles the key, on the key machine.
- **Sharing gains it nothing.** encri recognizes an app by the certificate its registry entry names, not by encri's
  own (encri D114, D130), so a key of encriox's own is as official. What D115 gains from one key, that a user moves
  between Play and GitHub without uninstalling, comes from each app keeping one key everywhere, not from apps sharing
  one.

**encri needs no update at encriox's two-week releases.** An app's certificate does not change between its updates,
and a grant carries over an update signed with the certificate the registry names, at or above its floor, unless the
registry refuses that version (encri D130). encri is updated once, with the registry snapshot that first names
encriox, and again only if encriox's key is ever rotated.

#### Flagged for veto

1. **The S24's checks are taken as its owner's word** for encriox's first usable build alone, under `AGENTS.md`'s
   hazard: each round there is still named to the user before it runs, and nothing else runs there.
2. **The saved tabs reopen after a lock.** Closing at a lock ends the session, not the tabs: they are sealed, and come
   back once the vault is unlocked, as after a restart. A lock that forgets them is the other answer.
3. **arm64 alone**, as every test phone is: no 32-bit ARM or x86 build until a phone needs one.

#### Rejected

- **Tabs kept running at a lock, with nothing written**: every store would have to hold its writes, and the keys would
  stay in memory, which is the S3 this decision ends.
- **History and bookmarks kept, in the clear and said so**: the first build's promise is that nothing of a page is on
  disk in the clear.
- **A build signed with a key of the moment, to be replaced**: Android updates an app only under the key it was
  installed with, and an uninstall takes the profile with it.

<!-- amendments: generated by scripts/docs/decision-index.py; do not edit -->

## Amended since, as recorded in `amendments.md`

- **W17's order of work** (2026-11-20, the user's word: of the spikes' bugs, "We should fix these before continuing,
  if they are not fixed yet", and of history and bookmarks in a sealed store, "Let's work on this next"): the three
  bugs come first, fixed in the spikes, then history and bookmarks in a sealed store, then the rest of item 6. W17 has
  the first usable build go without history and bookmarks; whether it waits for them now is the user's to say.
- **W17's first build** (2026-11-20, W21, the user's word: "Let's wait for 1 and 2 before calling the first usable
  build"): it waits for history and bookmarks, the logins and the cards in sealed databases, and carries them.
  History is recorded and bookmarks are on in its menus; the logins' origins are sealed with the rest of their
  database, not disclosed. Collections and pinned top sites, which are not application-services', stay off.

<!-- /amendments -->
