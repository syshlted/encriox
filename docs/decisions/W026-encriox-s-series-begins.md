# W26 — encriox's series begins on Firefox 160.0, with stage 3's sealing as its first six patches, and history and bookmarks were never off

*2026-11-23 · built on the user's word ("ok let's move on to the next step", then "ok, keep going"): `ROADMAP.md`'s
item 6, history and bookmarks on in encriox's menus and the stage 3 patches carried into encriox's series, with the
layout W9 settled, which the series needs first. **Flagged for veto on five points** (below).*

#### What was found first

- **The item could not be done as it stood**: encriox had no series to carry patches into, and the layout that makes
  one, `patches/`, `scripts/` and the file naming the release (W9), was later in the same list. The layout came
  first.
- **History and bookmarks had nothing to turn on.** W17 planned the first build without them, history unrecorded by
  leaving Fenix's history delegate unset and bookmarks off in the menus, and W21 had them back before any of it was
  built. Fenix 160 sets the delegate, `historyTrackingDelegate = HistoryDelegate(lazyHistoryStorage)` (`Core.kt`), and
  has both in its menus: a build of the series records history and keeps bookmarks as Fenix does (*Measured*).
- **Stage 3's patches are in two halves.** application-services' and Android Components' need nothing of the spikes':
  a store given no key keeps its database in the clear, as before. The spike's Fenix, `EncriDatabaseKeys` and
  `EncriStores`, needs `encriox.key` (stage 1's patches 0015 to 0019), encri's unlock in front (stage 2's 0010) and the
  gate. The first half is carried; the second waits for the spikes' patches that are encriox's, which bring the key
  and the unlock rewritten without the gate (`ROADMAP.md`, item 6).

#### What was built

- **The layout** (W9):
  - `firefox-release`, the one file naming the release: `FIREFOX_160_0_RELEASE` and its commit, `2887d9de…`;
  - `scripts/fetch`, a shallow clone of the tag with `patches/` committed on a branch of its own, `encriox`;
  - `scripts/build`, mach and Gradle as stage 3's spike builds, which refuses a checkout not built on the release's
    commit, with `scripts/mozconfig`: stage 3's, application-services in tree, and the objdir `obj-encriox`;
  - a brief, `.claude/rules/encriox.md`, which `LICENSE` moves to from the spikes'.
- **The series**, seven patches: six from nine of stage 3's commits, each change once, with the fixes measured after
  it folded in, and one of stage 1's, each naming the spike's patches it carries in its message:

  | Patch | What it carries | From the spikes, stage 3's but where said |
  |---|---|---|
  | 0001 | obfsvfs in sql-support, the key by a handle, a check of the key in each page | 0001, 0005, 0008 |
  | 0002 | a sealed open in places, the logins and autofill, with their Kotlin bindings | 0002 |
  | 0003 | a database key for Android Components' places, logins and autofill stores | 0003 |
  | 0004 | SQLite's recovery in the megazord, and `sqlite_dbpage` in its SQLite | 0010 |
  | 0005 | obfsvfs's check and repair of a sealed database | 0011, 0016's repair |
  | 0006 | places carrying the check and the repair to Kotlin | 0012, 0016's places |
  | 0007 | Fenix's StrictMode listener ignoring LG's resource preloading | stage 1's 0004 |

  The tip's files are stage 3's tip's, but for three things: Android Components' logins' and autofill's stores have
  no stage 1 changes under 0003's, whose hunks took a three-way apply onto the release, adding the same lines; and two
  comments. A test's said *as the spike's first builds sealed them*; places' sealed open said every connection opens
  the database *by a name that carries the key*, which patch 0005 had made a handle, and stage 3's patch 0018 now
  says so in the spike too.

#### Found on the way

1. **A build of the series did not start on the G6**: Fenix's debug build is killed by its own StrictMode listener for
   a read of the disk on the main thread in LG's framework, `ResourcesImpl.startParallelLoading`, at its first
   activity, before any code of Fenix's runs, as the stage 1 spike's first build was (W4, flag 2). The plan kept the
   spikes' StrictMode workarounds out of encriox; stage 1's patch 0004 is not one of them, but a fix of the kind
   Fenix makes for Samsung's and Xiaomi's frameworks, for LG's two methods alone, and it is patch 0007. Stage 1's
   0006, which turns StrictMode's kill off altogether, stays in the spike.
2. **`fetch` reads `patches/` only once its clone is done**, 17 minutes in: the series was written out while the
   clone ran, and the first checkout came with it, which was meant to build the release alone first. The release
   alone was then built as the first of the prefixes.

#### Measured (2026-11-23)

| | |
|---|---|
| `fetch`, from GitHub | 16 min 47 s: a 1.0 GB pack, a 6.1 GB checkout, and the series' first six patches applied |
| `fetch` again, of the seven patches as committed, into a checkout then deleted | 19 min 57 s; its tree the same as the tip built and measured |
| `build`, the first, of the tip without patch 0007 | 80 min: 73 of mach's, Gecko and the megazord, and 7 of Gradle's; an objdir of 12 GB; `org.mozilla.fenix.debug`, 170 MB for arm64 |
| `build` again with patch 0007, a change to Fenix alone | 4 min 47 s |
| `build` at every prefix of the series, in order, the release alone first | each built: 5 min 26 s for the release alone, then 0.8–3.9 min a patch, 22 min in all |
| obfsvfs's tests on this machine (`scripts/spike/stage-3/check-obfsvfs-host`), at the tip | 23 pass, the same files as stage 3's tip's |
| The tip without patch 0007, on the G6 (Android 9), at its first start | killed by Fenix's StrictMode listener, *DiskReadViolation* in LG's `ResourcesImpl.startParallelLoading`; the crash reporter came up |
| With patch 0007: the spike's marker page, opened from another app over `adb reverse` | loaded in a normal tab; its eight stores written; 15 violations of LG's logged as suppressed, none fatal, in this process and the next |
| The menu | *Bookmark page*, and *History*, *Bookmarks*, *Downloads* and *Passwords*, as Fenix has them |
| *Bookmark page* | *Saved in "Bookmarks"*; Bookmarks listed the page, History its visit |
| places' database, copied off with the app stopped | in the clear: SQLite's own header, places' 32 KiB pages, no reserved bytes, opened through obfsvfs as SQLite's default VFS; one page, its visit and its bookmark |
| A new process | History listed the visit again |

The build was then uninstalled from the G6, its data with it: it held the page in the clear, as Fenix does.

#### Flagged for veto

1. **The spike's commits are carried squashed**: a patch of encriox's carries a change once, with the fixes found
   after it, and the spike's series keeps the history and what each measurement ran. No patch of encriox's was itself
   measured on a phone as the spike's were: what it carries was, in the spike, in that order, and the series' tip is
   the spike's tip's files.
2. **The series is on 160.0**, the spikes' release, not the newest. Moving it is item 6's next piece, and the first
   measure of a rebase's cost (W9, flag 1).
3. **A build of the series keeps history, bookmarks, logins and cards in the clear**, as Fenix does, until its Fenix
   gives the stores their keys: it is built and checked on a test phone, never used. W22's point 3 holds for what is
   used. Its package is Fenix's own debug one, `org.mozilla.fenix.debug`, until encriox's build type sets
   `com.systemhalted.encriox` (W8).
4. **Collections and pinned top sites**, which W17's amendment keeps off, are on, as Fenix has them: they are among
   Fenix's records of pages, in Room databases of its own in the clear, which item 6's *What the spikes left* seals
   or drops.
5. **`fetch` clones from GitHub each time**, about 1 GB in 17 minutes, a checkout of its own beside the spikes',
   whose branches stay as they are.

#### Rejected

- **Carrying the spike's commits as they are**, 0001 to 0017: eight of them, and part of a ninth, are the spike's
  Fenix, the gate's controls and the bench among them, and the rest carry fixes as patches of their own, as the
  spike's rule has it so that what each measurement ran is a prefix of its series. encriox's series is read as what
  encriox changes, one change to a patch (W9, flag 1).
- **Carrying stage 3's Fenix half now**: its keys come from stage 1's `encriox.key` and wait for stage 2's unlock,
  which are encriox's only once rewritten without the gate.
- **encriox's branch in the spikes' checkout**: a switch between it and stage 3's would touch the files every patch
  changes, and both objdirs would build them again; and `fetch` is then what makes the checkout, as it will for each
  release.

#### Unmeasured

- The other phones: the A20, whose wireless adb had dropped, the S9, not attached, and the S24.
- The logins and the cards in a build of the series: Fenix's warm-up made their databases, in the clear, and nothing
  was saved in them.
