# W16 — What obfsvfs costs: twice plain SQLite's time where a page is read from the file, a third more to write in bulk, and nothing a site's IndexedDB shows

*2026-11-18 · run on the user's word ("After these things, let's move onto stage 2 round 3"): stage 2's third round
(`ROADMAP.md`, item 4), on the G6 and the A20, with stage 2's patch 0011. The S9 was not attached. **Flagged for veto
on three points** (below). Nothing here is a design for encriox yet.*

#### What obfsvfs does (read in the source)

`storage/ObfuscatingVFS.cpp`, the VFS behind Lockstore's SQLite switch:

- **Every page of a database is encrypted on its way to the file and decrypted on its way back**: the main file, its
  rollback journal and its write-ahead log, whose frames are pages too. The cipher is ChaCha20-Poly1305, through NSS
  (`dom/quota/NSSCipherStrategy.cpp`), with a context kept per open file and a fresh nonce for every page written.
  The nonce and the tag take the last 32 bytes of each page, reserved for them; the first 24 bytes of the file stay
  in the clear, for SQLite's header.
- **Its pages are 8 KiB exactly.** mozStorage opens a plain database with 32 KiB pages, and an encrypted one with
  8 KiB (`mozStorageConnection.cpp`), so the page size is part of what obfsvfs changes.
- **mozStorage keeps 2 MiB of pages per connection**, and a page in that cache is not decrypted again: obfsvfs costs
  where SQLite goes to the file, whose pages the system may still hold in memory.
- With the SQLite switch on it is **SQLite's default VFS**, and every database under the profile goes through it with
  a key of its own from Lockstore, site storage's among them (below).
- **mozStorage opens every database with `synchronous = OFF` on Android**, and each store sets its own; the cookie
  store sets `NORMAL`, which in WAL mode syncs only at a checkpoint.

#### What was built

`scripts/spike/stage-2/`, patch 0011, on W15's ten:

- **`GeckoViewSQLiteBench`**, a module of Gecko's parent process, times the same work in Gecko's own SQLite and
  mozStorage on three kinds of database, each made afresh in a directory outside the profile, so that Lockstore makes
  no key for it: **plain**, as mozStorage opens one with encryption off, with 32 KiB pages; **plain-8k**, plain with
  obfsvfs's 8 KiB pages, which tells the page size's cost from the cipher's; and **obfsvfs**, opened with a key of its
  own, as Lockstore's databases are. Each is in WAL mode with mozStorage's own `synchronous`. The kinds go in a
  different order each round. The work: 50,000 rows of about 160 bytes inserted in one transaction; 500 commits of
  one row each; 50,000 rows read by their key, scattered across the table; a fifth of the rows updated; an index
  built; a checkpoint; and, on a new connection whose cache is empty, the table scanned. Each run reports its file's
  header and whether a marker in the rows is in the clear, which checks the premise: the plain files held it, and
  obfsvfs's did not, with its 8 KiB pages and 32 reserved bytes.
- **The gate's `--ei sqlbench <rows> --ei rounds <n>`** asks for it before it opens the vault, which it does not need.

`check-obfsvfs` runs the round on a phone: that bench, then IndexedDB as a site uses it, on a page served from the
phone's loopback (`../stage-1/site/idb.html`). For a plain IndexedDB it installs a second build, the same as stage 2's
but for the SQLite switch's default, off, built from the working tree and not a patch: the switch alone puts every
database under the profile through obfsvfs (below), and Gecko's launch guard reads only the build's default (W13).

#### In Gecko's SQLite (measured)

The median of 5 rounds, in milliseconds, with obfsvfs's time over plain's, and over plain-8k's:

| | G6 plain | G6 plain-8k | G6 obfsvfs | ratio | | A20 plain | A20 plain-8k | A20 obfsvfs | ratio | |
|---|---|---|---|---|---|---|---|---|---|---|
| 50,000 rows, one transaction | 246 | 262 | 331 | 1.35 | 1.26 | 427 | 458 | 590 | 1.38 | 1.29 |
| 500 commits of a row | 77 | 81 | 74 | 0.95 | 0.91 | 264 | 194 | 253 | 0.96 | 1.30 |
| 50,000 reads by key | 1,045 | 558 | 2,255 | 2.16 | 4.04 | 2,573 | 993 | 4,910 | 1.91 | 4.94 |
| a fifth of the rows updated | 58 | 67 | 140 | 2.42 | 2.09 | 128 | 123 | 289 | 2.26 | 2.35 |
| an index built | 120 | 120 | 176 | 1.46 | 1.47 | 298 | 280 | 393 | 1.32 | 1.41 |
| the checkpoint | 11 | 12 | 13 | 1.16 | 1.04 | 17 | 19 | 19 | 1.15 | 1.02 |
| a new connection opened | 1.5 | 1.4 | 1.5 | 1.00 | 1.07 | 3.2 | 3.0 | 3.2 | 1.00 | 1.07 |
| the table scanned on it | 32 | 32 | 65 | 2.05 | 2.00 | 74 | 72 | 152 | 2.05 | 2.10 |

The G6's table is its second run. Its first run's medians were within 6% of these, but for the few milliseconds of
opening and creating, and its ratios over plain within 0.11. The rounds agreed within a few percent, but for the G6's
first plain round, up to three times slower, and the A20's index, which varied by up to half; the medians set them
aside. The files were 11.2 MB plain, at either page size, which SQLite on the machine reproduces, and 0.4% larger
through obfsvfs: five pages of reserved bytes.

**A page costs 24 µs to decrypt on the G6 and 58 µs on the A20**, about 350 and 140 MB/s: the scan's time over
plain-8k's, over the 1,373 pages it read. The reads by key decrypted about 1.4 pages each, an index page and a leaf
that were not in mozStorage's cache.

#### In IndexedDB, as a site uses it (measured)

A page put 5,000 records of about 200 bytes in one transaction, then 200 more in a transaction each, read the 5,000
by their keys, scattered, in one transaction, and read them all with `getAll`, three rounds to a page. Each side ran on
a fresh profile after an install of its build: stage 2's, whose origin was recorded encrypted, flags `0x17`, and
whose profile's databases were marked encrypted in `compatibility.ini`; and the plain build's, with neither. The G6
ran two profiles of each, in turns, and the A20 one:

| Median ms | G6 plain | G6 obfsvfs | ratio | A20 plain | A20 obfsvfs | ratio |
|---|---|---|---|---|---|---|
| 5,000 records, one transaction | 2,259 | 2,877 | 1.27 | 2,483 | 2,092 | 0.84 |
| 200 transactions of a record | 622 | 705 | 1.13 | 858 | 806 | 0.94 |
| 5,000 reads by key | 2,020 | 1,917 | 0.95 | 2,611 | 2,137 | 0.82 |
| `getAll` | 136 | 128 | 0.94 | 146 | 161 | 1.10 |

**obfsvfs's cost did not show in IndexedDB.** The two phones disagree on every row's direction, and each side's
rounds spread two- to threefold, wider than any difference between the sides: the G6's bulk put took 1.7 to 4.6
seconds plain and 2.2 to 4.0 through obfsvfs. What IndexedDB does around SQLite, cloning, compressing and passing each
record between processes, takes most of a site's time.

#### Found: the SQLite switch alone encrypts a site's databases

On the G6, with QuotaManager's switch off and the SQLite switch on, a page's origin was recorded plaintext, flags
`0x07`, and yet its IndexedDB, local storage and Cache API databases were obfsvfs's, 8 KiB pages with 32 reserved
bytes, and none held the page's marker. The Cache API's two response bodies, files beside its database, held it in
the clear. SQLite's switch encrypts every database under the profile by its path, site storage's too, each under a
key of its own from Lockstore; QuotaManager's switch is what encrypts the files beside them, and gives the
databases its own key instead. A first design of the IndexedDB measure, comparing QuotaManager's switch on and off,
compared obfsvfs with obfsvfs, and on the G6 the two did not differ.

#### What it means for stage 2

1. **obfsvfs costs where SQLite reads a page from the file**: about twice plain SQLite's time to scan a table or to
   update or read rows scattered across it, from the system's file cache, on both phones. A bulk write costs a third
   more. A small commit costs nothing, because obfsvfs's 8 KiB pages write less than mozStorage's plain 32 KiB.
2. **Against plain SQLite at the same page size, reads scattered across a table cost 4 to 5 times as much**: a 32 KiB
   page reads four times the bytes for a row, which plain pays and obfsvfs does not, so half of what obfsvfs's cipher
   costs there is won back by its page size.
3. **In absolute terms it is small**: a 1 MB database costs 3 ms more on the G6 to read whole, and 7 ms on the A20,
   and Gecko's own databases are read once into memory or fit in the 2 MiB cache. A site's IndexedDB showed no cost
   above its own variation. Nothing measured here argues against keeping every database through obfsvfs.

After the round, the G6 and the A20 got a fresh Gecko profile, stage 1's APK and stage 1's config again (W13, flag 3).
The A20's vault is open. The G6's stays locked, and its gate withholds the secret, a setting stage 1's build does not
read.

#### Flagged for veto

1. **The bench's obfsvfs has a key of its own**, not one from Lockstore: the cipher and the VFS are the same, and the
   key's cost, PBKDF2 at startup and an unwrap at each open, is W14's.
2. **Every read came from the system's file cache**, never the flash, as the phones are not rooted to drop it. A read
   from the flash would add the same time to both, so the ratios here are the most obfsvfs can cost.
3. **The plain IndexedDB ran on a build of its own**, the same as stage 2's but for one default, built from the working
   tree and not kept as a patch.

#### Not measured

- The S9, which was not attached, and the S24.
- Reads from the flash; the battery; a private session, whose site storage obfsvfs keeps under ephemeral keys.
- Gecko's own stores' workloads one by one, as the cookie store's at startup.
- Whether NSS's ChaCha20 uses the phones' vector unit; the HTTP cache's and QuotaManager's stream ciphers, which are
  not obfsvfs.

#### Rejected

- **QuotaManager's switch on against off**, as the plain side: the SQLite switch puts that IndexedDB through obfsvfs as
  well (above).
- **A config turning the SQLite switch off on stage 2's build**: Gecko's launch guard refuses the profile (W13).
- **Timing whole pages**, where the network and layout dwarf a database's share.
- **SQLite's own `speedtest1`, built for the phone**: it would not be Gecko's SQLite, mozStorage or VFS stack.
