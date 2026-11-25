# encriox's first usable build: a plan

*2026-11-18 · a study, drafted at the user's request ("draft the plan for a first usable build"), after stage 2's
three rounds (W12 to W16). **Accepted whole on 2026-11-19** (W17: "go with all your recommendations"): every
"recommended" below is the user's decision, and the work is `ROADMAP.md`'s item 6. W17 is the record; this is the
reasoning it was chosen from.*

## What "usable" means here

**The first usable build is encriox itself, not the spike, used every day on the user's own phones.** It is built from
this repository under `com.systemhalted.encriox` (W8), named encriox throughout (W3) and signed with encriox's own key.
It:

- keeps nothing of a page on disk in the clear, in Gecko's stores or in Fenix's own;
- does not start without the vault (W15), and does what the user decides at a lock;
- sends Mozilla nothing but what keeps the browser safe;
- follows each Firefox release, every two weeks, so that it never falls behind on security fixes (W9);
- survives its own updates without asking the user to let it into the vault again (encri D130).

It is not published. The first public release comes after it (*After the first usable build*, below).

## Where it starts from

**What the spikes proved.** Downloads go into the vault whole or not at all (W4, W5). The logins' and the cards' keys
are sealed with the browser's own key in the vault, and a private tab's thumbnail stays in memory (W6). With stage 2's
Gecko, every database of the profile, the HTTP cache and site storage are encrypted (W13), under a secret from the
vault's key, and the profile opens again whole (W14). A start with the vault locked brings up encri's unlock, and a
refusal changes nothing (W15). obfsvfs costs little that a user would notice (W16).

**What they left.** Fenix's own records of pages are in the clear: its saved tabs, its history and Gecko's list of
service workers held a session's marker (W13). Fenix fetches Mozilla's sponsored tiles and experiments (W6). A lock
leaves keys in memory, and can lose local storage a page has just written (`ROADMAP.md`, *Bugs*). What to do when the
vault's key does not open the profile is open. The spike is a debug build of 239 MB, driven over adb.

## Decisions it needs first

1. **What a lock does** (`open-questions.md`, *Lock behaviour*). Recommended: **encriox closes at a lock.** It shuts
   Gecko down with `GeckoRuntime.shutdown()`, letting it write what it holds, then locks the profile's keys and ends
   its process. The next start asks for the unlock (W15) and reopens the saved tabs. This ends both bugs a lock has
   now: the logins' and the cards' keys left in memory (S3), and local storage lost when a page wrote it just before
   (S1). Whether Gecko's shutdown writes that local storage before it ends is to be measured. The alternative, tabs
   kept running with nothing written, keeps keys in memory and needs every store to hold its writes. *(Amended, W27:
   the close reaches a browser Android has frozen only once it is opened, so encriox keeps a foreground service while
   it holds the keys, and closes after five minutes in the background.)*

2. **Fenix's own records of pages.** Recommended: **each is sealed with the vault's key or kept in memory, and history
   and bookmarks are off in the first build**:

   | Record | Where it is now | Proposed |
   |---|---|---|
   | open tabs, saved for the next start | Android Components' session storage, a file | sealed, as the logins' keys are (W6) |
   | recently closed tabs | a file | sealed |
   | tab thumbnails | JPEG files under `cache/` | in memory, as a private tab's are (W6) |
   | site icons | Android Components' icon cache | in memory |
   | the downloads list | a Room database | in memory: the downloads themselves are in the vault |
   | history | application-services' `places.sqlite` | not recorded: Fenix's history delegate unset (`Core.kt`) |
   | bookmarks, collections, pinned top sites | `places.sqlite` and Room databases | off in the menus |
   | service workers' registrations | Gecko's `serviceworker.txt`, plain text | in memory, or under Lockstore (to be read) |
   | logins' origins | plain columns of the logins database (W6) | disclosed now, sealed later |

   `places.sqlite` has no hook for a key (`open-questions.md`, *application-services*), so history and bookmarks come
   back with a sealed store of their own, after the first build. The alternative is to keep them, in the clear and
   said so.

3. **Mozilla's services.** Recommended: **off, but for what keeps the browser safe.** Off: telemetry (Glean);
   experiments (Nimbus, which can turn a feature on remotely); sponsored tiles and stories; crash reports to Mozilla
   (Sentry and Mozilla's crash reporter); Firefox accounts and Sync; attribution (Adjust); and push through Firebase,
   all of which Fenix's build includes (`app/build.gradle`). Kept: Remote Settings' lists, among them certificate
   revocation, blocked add-ons and the search engines; and Safe Browsing. Search suggestions off by default.

4. **The signing key and the registry entry.** Recommended: **encriox's own key, made on the key machine with encri's
   kit (encri D141) before the first build.** A build under another key cannot be updated in place, and uninstalling
   takes the profile with it. **Its entry in encri-api's registry**, with a floor, filed as soon as the key exists
   (W8, encri D130): without it the user lets encriox into the vault again at every update, twice a month.

5. **encriox's version** (`open-questions.md`). Recommended: Fenix's version code, made from the build's time, which
   only rises, as the registry's floor needs (encri-api L15); and a version name of the Firefox release and encriox's
   own count, *161.0-1*, beside the About screen's line (W9).

6. **A vault's key that does not open the profile** (`open-questions.md`), as a replaced `.encriox.key` gives.
   Recommended: encriox says so and stops, and offers, behind a confirmation, to start a new profile, which loses every
   site's data.

7. **The phones.** The first usable build is for the S24, the user's own phone, so its checks run there, which needs
   its owner's word (`AGENTS.md`, *Hazards*). The S9 and the A20 run them too, and the G6, whose vault does not open,
   checks the refusal alone.

## The work, in order

Sizes are relative: **S** is a patch or a script; **M** is several patches and a round on the phones; **L** is a
design, several patches and rounds.

### 1. The repository and the build

- **The layout** (W9): `patches/` for encriox's series, `scripts/` for fetching, building, following and checking,
  and one file naming the Firefox release it is built on. The spikes' directories stay as they are. **S**
- **The newest Firefox release**: the series moved from 160.0 onto the newest release tag. It is the first measure
  of what a rebase costs (W9, flag 1). **M**
- **A build type of encriox's own**, from Fenix's release template: minified, Gecko optimised, signed with encriox's
  key, and **without the `sharedUserId`** that Fenix's beta and release types carry for Firefox for good
  (`app/build.gradle`); and a debug type, `com.systemhalted.encriox.debug` (W8, flag 1). arm64 only, as every test
  phone is. **M**
- **Branding**: the name, the icons and every string that names Firefox, in Fenix's resources and Gecko's, as
  Mozilla's trademark policy requires, with the statement that encriox is not Mozilla's (the study, [F7]). **M**
- **Gecko's prefs as the build's defaults**, which answers *How a release sets Gecko prefs*: Lockstore's three
  switches, a private tab's media cached in memory, no login offered for saving from a private tab
  (`open-questions.md`), and what decision 3 turns off. **S**

### 2. The spikes' patches that are encriox's

About 20 of the spikes' 30 patches are encriox's, rewritten without the spike's gate and names, and with the key
under `.encriox.key` (W10):

| From the spikes | What it carries |
|---|---|
| stage 2, 0001, 0002 and 0004 | QuotaManager's fix, and SQLite's switch on by default (W13) |
| stage 2, 0005, 0007 and 0009 | the embedder's secret for the profile's keys, every store following it, the log truncated (W14) |
| stage 2, 0006 and 0010 | the secret from the vault's key, and no start without the vault (W14, W15) |
| stage 2, 0003 | encri-api's client built from source, as the companions' is |
| stage 1, 0003, 0008 and 0010 | downloads into the vault, whole or not at all, and a held session asked first (W4, W5) |
| stage 1, 0013 and 0014 | a private tab's thumbnail kept in memory (W6) |
| stage 1, 0015 to 0019 | the browser's own key in the vault, and the logins' and the cards' keys sealed with it (W6) |

The gate, the loopback server, the debug build's StrictMode workarounds, the private-tabs switch and the bench stay in
the spikes. **M** *(Amended, W26: stage 1's 0004, which has Fenix's StrictMode ignore LG's framework, is encriox's
patch 0007, without which its debug build does not start on the G6; stage 1's 0006, which never kills, stays.)*

### 3. What the spikes left

- **The lock**, by decision 1, measured on the A20: a lock seconds after a page's write, and every store read back at
  the next start. **M**
- **Fenix's records of pages**, by decision 2. **L**
- **Mozilla's services**, by decision 3, with a check that a session reaches no other server. **M**
- **A key that does not open the profile**, by decision 6. **S**
- **The About screen and the version**, by decision 5 and W9. **S**

### 4. The checks

The spikes' checks become encriox's, run on each phone before a build is used:

- a normal session leaves nothing of a page in the clear, Fenix's files included (`check-plaintext`);
- the profile opens again whole (`check-lockstore`);
- the vault's lock, a locked start and a refused one (`check-unlock`);
- a download into the vault, stopped and tried again (`check-stopped`).

The script that follows each release (W9) builds it and runs them. **M**

### 5. The first usable build

Signed on the key machine, installed on the phones, the S24 on its owner's word, and used for some days. What it
finds goes into `ROADMAP.md`'s *Bugs*. From then on, every Firefox release: the script that follows it, a person for
what it cannot do, the checks, and a signed build. **S**, every two weeks.

**The critical path** is decisions 1, 2 and 4, then the build type and the rebase, then the lock and Fenix's records,
the largest piece, then the checks on the S24. The key is the user's to make, on the key machine.

## What it costs to keep

- **A rebase every two weeks**, about 26 a year (the study, *What carrying the fork costs*), each with a Gecko build
  and the checks: the first build took 76 minutes on this machine, and a change to Gecko about 8 after it (W13).
- **Each start waits for the vault**: 0.7 s on the S9 and 1.7 to 1.8 s on the A20, then 250 to 630 ms of PBKDF2 over
  32 random bytes that do not need it (W14, flag 1). A lighter derivation is a Gecko patch, after the first build.
- **Lockstore is new at 160**, and Mozilla is still changing it. The series' Gecko patches are the most likely to stop
  applying, and asking Mozilla (`ROADMAP.md`, item 5) is what would shrink them.

## After the first usable build: the first public release

- a public remote before any build leaves the user's phones, and the MPL's source for every build published (W2);
- the user's call on a clearance search for the name (`docs/trademark-search.md`);
- where it is published: GitHub releases, as the suite's open-source apps' builds are (encri D143), F-Droid, or
  Google Play, where encri is (encri D142);
- Mozilla asked about the Gecko patches (`ROADMAP.md`, item 5);
- history and bookmarks in a sealed store, and app-level security controls (W10).

## Not in it

- 32-bit ARM and x86 builds; ESR (W7); what extensions keep in the profile (`open-questions.md`, *Extensions*).
