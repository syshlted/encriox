# W21 — History, bookmarks, logins and cards are sealed with the vault's key, in application-services' own SQLite

*2026-11-20 · decided and built on the user's word ("Let's wait for 1 and 2 before calling the first usable build.
Let's continue on to history and bookmarks. Then logins and cards, so 1 and 2 are completed"), on
`docs/sealed-history.md`'s three decisions; built as stage 3's patches 0001–0006 and measured on the A20 and the S9.
**Flagged for veto on six points** (below). The G6's vault is locked and the S24 waits for its owner: both
unmeasured.*

#### What the user decided

1. **The way: obfsvfs in application-services' SQLite**, the plan's recommendation.
2. **What it seals: places, the logins and autofill**, the three of application-services' databases that hold the
   user's own data: history and bookmarks, each login's origin and times, and every field of a card or an address.
3. **The first usable build waits for both, and carries them.** W17's first build went without history and
   bookmarks; it now has them, sealed (W17's amendment).

#### What was found

- **Firefox 160's tree carries application-services**, `third_party/application-services`, the same as its `v160.0`
  tag in every component this changes. With `--enable-appservices-in-tree`, mach compiles the megazord,
  `libmegazord.so`, from there, against Gecko's own NSS, and Gradle takes the components as projects of the build.
  So there is **no second fork**: one checkout and one series, Gecko's and application-services' patches together,
  and no NSS, NDK or publication of application-services' own to build. The plan had assumed a fork of
  application-services' repository, built apart.
- **The components' Kotlin bindings are generated ahead and checked into Firefox's tree**
  (`toolkit/components/uniffi-bindgen-gecko-js/android/components/`), so a change to a component's interface needs them
  generated again. Mozilla's own bindgen, run on the megazord the build had just made, gave every component this does
  not change byte for byte as the tree has it, and the three it changes with the new functions alone
  (`scripts/spike/stage-3/bindings`).
- **The megazord has a SQLite of its own**, rusqlite's bundled 3.50.2, apart from Gecko's: obfsvfs is registered in it
  alone, and Gecko's databases are as they were.

#### What was built (stage 3, `scripts/spike/stage-3/`)

- **obfsvfs in sql-support** (patch 0001), the crate places, the logins and autofill open their databases through: a
  VFS shim after Gecko's obfuscating VFS, in Rust, on the bundled SQLite:
  - each page is 8,192 bytes, of which the last 32 are reserved: a GCM tag, then a random 96-bit nonce. Page 1's
    first 32 bytes stay in the clear, SQLite's header among them;
  - **AES-256-GCM from NSS**, the one AEAD application-services' NSS bindings have, where Gecko's uses
    ChaCha20-Poly1305, with the key imported once for each open file;
  - **a page that does not open under its key is an I/O error, `SQLITE_IOERR_DATA`**, where Gecko's copy hands SQLite
    whatever its cipher gave, tag or no tag; never a corrupt database, which places and autofill delete and make
    again. A frame of the log that does not open, after one that did, is one a crash tore: it is passed to SQLite,
    whose checksum ends the log there;
  - **on Android it is SQLite's default VFS before any database opens**, and refuses a sealed database opened without
    its key, and one in the clear opened with a key, as Gecko's refuses either (Gecko's `PeekOnDiskHeader`);
  - a `PRAGMA page_size` is ignored, a write of anything but a page, or a log's headers, into a sealed database or its
    log is refused, and memory-mapped reads are off. Temporary files stay in memory, as the megazord's SQLite is built
    to keep them on Android;
  - **the key is handed over by a handle, never in the database's name** (patch 0005), and kept for the process's
    life, which ends at a lock (W20).
- **A sealed open in places, the logins and autofill** (0002), with their Kotlin bindings and wrappers; **a database
  key for Android Components' four stores** (0003), a source of 32 bytes each store asks for, and waits for, when it
  first opens its database; and **in the spike's Fenix, a key for each database** (0004), an HMAC of `encriox.key`
  under a label of its own, as Gecko's secret is (W14), which waits for the vault's key. None of the three databases
  is opened before the vault is open to the browser, and a start without it ends first (W15).
- **The checks**: `check-obfsvfs-host`, ten tests of the VFS on this machine with a toy cipher; `check-sealed`, a
  visit, a bookmark, a login and a card, each holding a marker where its database keeps it in a column of its own,
  then the app's data searched, the four read back in a new process, and each database opened without its key; and
  `bench-places`.

#### Measured (2026-11-20, the A20 and the S9, patch 0005's build, and 0006's for the bench)

| | A20 | S9 |
|---|---|---|
| Control run, the three in the clear (`check-sealed … unsealed`) | marker in `places.sqlite`, its log, `logins2.sqlite` and autofill's log | the same |
| Sealed (`check-sealed`) | the marker in no file of the app's; the three databases and their logs with 8 KiB pages and 32 reserved bytes | the same |
| The four read back in a new process, after the app was killed with its logs not checkpointed | yes | yes |
| Each database opened without its key | refused, `unable to open database file`; no file of the three changed | the same |
| Logins asked for before the vault was open (`check-keys`) | came after the vault's key and the database's, 3.6 s; nothing in the clear | 0.4 s; the same |
| A database's key derived once the vault's was held | 1–35 ms | 0–3 ms |

**What sealing costs places** (`bench-places`, patch 0006's build: 2,000 visits a transaction each, as browsing
records them, fifty autocomplete queries, and every visit read back, on three new databases a round in an order that
turns; medians of six rounds). *plain* is places as it makes a database, with 32 KiB pages; *plain-8k* is in the clear
with a sealed database's 8 KiB pages; *sealed* is through obfsvfs:

| | A20 plain | plain-8k | sealed | S9 plain | plain-8k | sealed |
|---|---|---|---|---|---|---|
| 2,000 visits | 100.0 s | 63.4 s | 66.1 s | 59.4 s | 44.6 s | 49.0 s |
| 50 autocomplete queries | 1,585 ms | 1,518 ms | 1,571 ms | 837 ms | 837 ms | 853 ms |
| every visit read back | 52 ms | 53 ms | 54 ms | 39 ms | 38 ms | 38 ms |
| on disk | 4.5 MB | 2.0 MB | 2.1 MB | 4.5 MB | 2.0 MB | 2.1 MB |

**The cipher costs 4% of a visit's write on the A20 and 10% on the S9, and nothing measurable to read.** The page
size does more, the other way: with 8 KiB pages each visit's commit writes a quarter as much to the log, so **sealed
places writes a third faster than places as Mozilla makes it on the A20, and a sixth faster on the S9, and takes half
the disk.** A visit, which commits with a sync to the log, took 50 ms plain and 33 ms sealed on the A20, and 30 ms and
25 ms on the S9. Patch 0005's build, timed the same way without *plain-8k*, five rounds, gave the same: sealed 0.62
of plain's visits' time on the A20 and 0.85 on the S9.

**History recorded by pages, not the gate** (stage 2's `check-lockstore` and stage 1's `check-plaintext normal`, on the
S9): the marker page, loaded in a tab, went to Fenix's history, whose metadata service logged it, and
`places.sqlite`'s log changed in the session; the marker was in two files of the app's, Fenix's saved tabs and
Gecko's `serviceworker.txt`, where stage 2's build left it in four, its history and the history's log among them
(W13). Every store of the page read back at the next start.

**At a lock, and at a start without the vault** (stage 2's `check-unlock`, with `WATCHED` adding the app's
`files` and `databases`, on the A20): the browser closed at a lock mid-session in 3.0 and 1.9 s, local storage written
2.0 s before the lock read back, and places' log was written at the close, sealed. While Gecko waited for the vault at a
start, and at a start whose unlock was refused, no file of the three databases changed: only Gecko's startup caches
and Fenix's saved tabs. Places' key came 10 to 11 s after it was asked, when the vault opened.

**Pages over TLS** load with the megazord on Gecko's NSS: `https://www.mozilla.org/` on the A20, shown secure. The
S9 has no network.

**The build**: mach's first build of stage 3 took 72 minutes, Gecko's and the megazord's, and Fenix's Gradle 9 more;
a change to obfsvfs, 6 minutes; to the spike's Fenix, under 2.

#### Found on the way, and fixed

1. **A sealed database's key would have been told to the log and the error reporter.** rusqlite puts the name it was
   given into the message of an open that fails, and the name carried the key: a sealed open refused, of a database
   in the clear, would have reached Android Components' log and its crash reporting with the key in hex. No run had
   one refused. Patch 0005 hands the key over by a handle.
2. **A component given no key would have deleted a sealed database.** obfsvfs's refusal of a keyless open held only
   once it was registered, at the first sealed open; before it, SQLite would have read the sealed pages as corrupt,
   and places and autofill delete a corrupt database. On Android it is registered before any database opens. Found
   in reading, before the first build.
3. **Places' 32 KiB pages hid the cipher's cost** behind the page size's (above), so the bench gained `plain-8k`
   (patch 0006).

#### Flagged for veto

1. **AES-256-GCM, not Gecko's ChaCha20-Poly1305.** application-services' NSS bindings have AES-GCM; ChaCha20 would
   need them extended. The format is not Gecko's, which nothing needs: the databases are not Gecko's.
2. **A random 96-bit nonce for every page written, under one key for the database's life**, as Gecko's has. GCM's
   bound is about 2³² writes under a key; a browser's history comes nowhere near, and nothing rekeys a database.
3. **A database in the clear is refused, not moved into a sealed one.** encriox has no release whose databases would
   need it. Stage 1's and stage 2's builds keep the three in the clear: `check-sealed` deletes them first, and a phone
   going back from stage 3 must have them deleted, which its README says.
4. **A sealed database SQLite finds corrupt under its own key is no longer deleted** (read in sql-support's
   `try_handle_db_failure`, not run). Places' and autofill's recovery deletes the file by the name it opened, which is
   now a URI, and fails: the database stays, unopened, and the store reports the error. W14's rule prefers a store
   kept to one remade; it leaves such a store failing until the file is deleted by hand.
5. **The megazord shares Gecko's NSS**, `libnss3.so`, where Mozilla's published one carries its own: a difference
   from the Fenix Mozilla ships, which is how Firefox's tree builds it in tree. A page over TLS loaded, and the logins'
   and the cards' NSS-sealed fields read back; nothing else of NSS was exercised.
6. **The keys stay in obfsvfs's table for the process's life**, unzeroed, as the connections that use them do. The
   process ends at a lock (W20).

#### Rejected

- **A fork of application-services, built apart** (the plan's way): Firefox's tree builds it already.
- **The key in the database's URI**, as Gecko's private-browsing databases have it: rusqlite tells the URI (above).
- **Keys looked up by the database's path**, as Lockstore's are: the app's directory came to SQLite as
  `/data/user/0/…` from the logins' store and as `/data/data/…` from places' in the same run, and a key not found
  for a path would open the database in the clear.
- **Telling a sealed database by its log when its own file is empty**: SQLite deletes such a log at the next open,
  whatever the key, so the check saved nothing.
- **ChaCha20-Poly1305** (flag 1).

#### Unmeasured

The G6, whose vault is locked; the S24, which waits for its owner's word. A log torn by a crash, and a page that does
not open under the right key, were tested on this machine alone (`check-obfsvfs-host`).

#### Found for the checks

`check-unlock` relied on the spike's private setting being off, as `check-plaintext normal` leaves it: with it on, every
page was private and nothing read back, which read as a lost local storage. It sets the setting itself now. And
Samsung's process manager killed encri three times in ten minutes on the A20 between runs, as W20 found, each time
locking the vault.
