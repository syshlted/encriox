# W25 — A sealed database that does not open is asked about at start: repaired, deleted or kept, and never deleted unasked

*2026-11-22 · built and measured on the user's word ("OK let's move onto the next": `ROADMAP.md`'s next item, W22's
point 4 with its first flag as amended), as stage 3's patches 0010 to 0017, on the G6. Ends item 6's prompt at
start. **Flagged for veto on five points** (below).*

#### What was built

- **SQLite's recovery in the megazord** (patch 0010): sql-support compiles SQLite's recovery extension,
  `sqlite3recover.c` and `dbdata.c`, copied unchanged from SQLite 3.50.2's source at the check-in of the amalgamation
  libsqlite3-sys bundles (`2af157d7…`, which its `SQLITE_SOURCE_ID` names), and that amalgamation is built with
  `SQLITE_ENABLE_DBPAGE_VTAB`, since the recovery reads a database page by page through `sqlite_dbpage`.
- **A check and a repair in obfsvfs** (patches 0011, 0016):
  - **the check** reads a sealed database through under a key, as its component would open it, without a checkpoint
    at its close: its schema, then SQLite's quick check, which reads every page of its tables and indexes, each the
    newest of its copies. It finds it *missing* (no file, or an empty one), *in the clear* (by its header),
    *under another key* (`SQLITE_AUTH`, W24), *damaged* (a page that does not open under its own key, or what SQLite
    finds malformed), or *opening*. It changes nothing but the log's index, as any open does;
  - **the repair** opens the damaged database leniently, only under a key shown to be its own: a page that does not
    open reads to the recovery as one that holds nothing, which it passes over, and is counted. The recovery's SQL
    is run into a new database, `<name>.repair`, sealed under the same key, with foreign keys off; the rows then left
    referring to a row lost with the damaged pages are deleted, and counted; the new database is taken out of its log,
    so that its file holds every page. Then the damaged database's log, index and journal are deleted, the repair is
    moved into its place, and the directory is synced. Under another key, in the clear, or where the recovery reads no
    table, the repair is refused, and the database left as it was.
- **places carries both to Kotlin** (patch 0012), `sealed_database_check` and `sealed_database_repair`, for any
  database sealed through sql-support's obfsvfs: the logins' and autofill's as well as its own.
- **The prompt** (patches 0013, 0014, 0017, in the spike's Fenix, `EncriStores`): each of the three stores' keys
  waits, once a process, for its database to be checked under it. One that opens, or is not there yet, goes on at
  once. One that does not brings up a question from whichever of the browser's activities is in the foreground, as
  encri's unlock is (W15), and its store waits for the answer:
  - **damaged**: *Repair*, *Delete* or *Keep*. A repair that fails says so, and asks again with Delete and Keep. One
    that succeeds says what it kept;
  - **under another key**: kept as it is, with one answer, *Go on without them*. It is never offered for deletion;
  - **in the clear**: *Delete* or *Keep*. It is never moved into a sealed one (W22, point 3).
  A kept database's store is refused its key for the rest of the session, and fails each open as one of its own would,
  so that nothing opens the database. It is asked about again at the next start. Each question says that, with Sync
  off (W17), nothing deleted or lost comes back. For example: *"Your history and bookmarks do not open: some of their
  pages are damaged. Repair keeps what can still be read, and loses the rest. Delete starts them again empty. Keep
  leaves them as they are, and the browser goes on without them, until you are asked again at the next start. Sync is
  off, so nothing deleted or lost comes back."*
- **The gate's controls**: `--es stores check` logs what the check finds of each, asking nothing; `--es seal fill --ei
  visits <n>` records visits that hold no marker, for a places database of more pages; and each check logs how long
  it took (patch 0015). **`check-stores`** damages,
  replaces and unseals the stores' databases between starts, and answers each question by tapping its button on the
  screen, as a user would.

#### Measured (2026-11-22, patch 0017's build, on the G6, Android 9)

| | G6 |
|---|---|
| The check at start, of the three intact: places' 1.8 MB, its 133 pages and its log; the logins' 98 KB; autofill's 189 KB | 12, 3 and 4 ms |
| The check of places filled to 10,000, 20,000 and 30,000 visits, 4.7, 5.7 and 6.8 MB with its log, first in a new process, Gecko starting beside it | 400–574, 424–539 and 494–709 ms, three runs each |
| The same 6.8 MB checked again in that process, 15 s later, three rounds | 76–118 ms, where the first had taken 392–628 |
| Places' page 1 damaged, in its database and in every frame of its log that holds it | asked about *History and bookmarks*: Repair, Delete, Keep. Keep: no file of places changed; the logins and the card read, places refused |
| The next start | asked again. Repair: *SQLite's recovery read no table*, said so, the database as it was; asked again with Delete and Keep |
| One page deep in places damaged, page 127 of 133, with no copy in its log | Repair: 3,813 rows in 18 tables kept, 1 page unreadable, 100 rows dropped that referred to it, and so said; it checks as opening, the visit and the bookmark written first read back, and new ones are taken; no row in the clear, no repair's copy left |
| The logins' page 1 damaged | asked about *Saved logins*. Delete: their files gone, their store started again empty and took a new login; places and the card read |
| Autofill's database replaced by the logins', under the logins' key | asked about *Saved cards and addresses*, with *Go on without them* alone; under another key at the next check; no file changed |
| The three written in the clear, as stage 1's and 2's builds wrote them, then sealing on | each asked about as stored unencrypted, Delete and Keep. Delete: each gone, and nothing of them read |
| `check-sealed` and `check-keycheck`, on the same build | every step held |

The 100 rows were visits whose page, in `moz_places`, was on page 127: the recovery reads a visit, and its foreign key
names a page that is not there.

**On this machine** (`check-obfsvfs-host`, with its toy cipher): 23 tests, 8 of them new, pass. Each part broken in turn
failed the tests it should: the lenient read off, 4, every repair that reads past a damaged page; no quick check, 2; a
checkpoint at the check's close, 1; the log not opened before the recovery, 1, the log's first frame torn; foreign keys
left on, 1, as on the phone; the dangling rows kept, 1.

#### Found on the way, and fixed

1. **Keep, as first built (patch 0013), let the store open its damaged database as it would have**, and the open's
   failure closed the connection, whose close checkpointed the log into the damaged file and deleted the log: Keep
   changed places' files on the G6. The store's open is the one every stage 3 build before had, so each did the same at
   each start with a damaged database, and a frame damaged in the log lost every commit after it for good. Since patch
   0014 a kept store is refused its key, and no file changed.
2. **The repair of places failed, *FOREIGN KEY constraint failed***: libsqlite3-sys turns foreign keys on, and the
   recovery's pragma to turn them off comes inside its transaction, where SQLite ignores it, before rows it writes
   ahead of those they refer to. The host's tests had no foreign keys. Patch 0016 turns them off first and drops the
   rows left dangling, with a host test that fails without either.
3. **A second tap meant for one question answered the next with Delete**: `check-stores` tapped Repair again as it
   went, and the question after the failed repair had Delete where Repair had been. A user's double tap could do the
   same. Since patch 0017 a question's buttons take no tap for its first second; since patch 0014 an answer is taken
   only for the question it was shown for.
4. **Cargo takes a vendored crate as it was built last**, by its version alone: the patch to libsqlite3-sys's build
   script reached neither the host's build nor the objdir's until the crate's build was removed, and the first host
   run failed, *no such table: sqlite_dbpage*.
5. **A log whose first frame a crash tore would have stopped the repair**: the recovery wraps the database's file
   as it begins, and a log opened under the wrapper would not have known the key to be its database's (W24, *Found
   on the way*, 1). The repair opens the log first. Found in reading; a host test tears the first frame.
6. `check-stores`'s own: a tap while a question's window comes up is lost; and a uiautomator dump that fails leaves
   the last one's file, which was read as a question from a process already stopped.

#### Flagged for veto

1. **The check reads every page of the three at each start**, before their stores open. On the G6, places at
   6.8 MB with its log took 76–118 ms checked again in a process, which is the check's own reading, and 392–628 ms
   as the first in a new one, with Gecko starting beside it; what of that is NSS's start, which the store's own open
   pays without a check, was not told apart. Places may grow to 75 MiB before Fenix's maintenance prunes it, about 11
   times as many pages: about a second on the G6 by the same measure, which was not made. A check of the header and
   the schema alone would cost almost nothing, but damage further in would be met by the store as an I/O error,
   unasked, as before.
2. **A repair replaces the damaged database, which goes with it**: the answer Repair is taken as the user's word to
   replace it. What its unreadable pages held is lost for good, where a database kept aside could be tried again by a
   later recovery, sealed, taking room no one is asked about.
3. **The rows left referring to lost rows are dropped**, 100 of 3,913 in the measurement: a visit without its page, a
   bookmark without its folder, which no store can show. And rows the recovery cannot place in a table, under a
   damaged page of a table's tree, are lost, where SQLite's recovery could put them into a table of their own
   (`lost_and_found`), which no store reads.
4. **A kept database's store is refused for the session**: the browser goes on without its history and bookmarks,
   its logins or its cards, and each start asks again. Each of the store's opens fails as one of its own would have,
   in its log, as before.
5. **A database in the clear may be deleted**: it is the user's, in plaintext, which W22's point 3 does not let
   encriox move into a sealed one. A stage 1 or 2 build on the same phone made it; encriox's own builds make none.

#### Rejected

- **A repair that copies table by table with SQL**: a scan stops at the first damaged page of each table.
- **The recovery writing its own output**, through the sealed database's name: it first copies an empty database of
  its own into the output by SQLite's backup, whose page size and reserved bytes are not obfsvfs's (read in
  `sqlite3recover.c`; not tried). Its SQL, run into a sealed open of sql-support's own, is what obfsvfs is built for.
- **Keep letting the store open the database as before** (patch 0013): its failed open wrote into it.
- **A kept store answered by an exception of Fenix's own**: places' storage catches its own exceptions alone, and
  Fenix's callers expect those, so one of another kind could end the app wherever no scope logs it (not tried). Each
  store is answered with what its own failed open throws.
- **The prompt counting the damaged pages before the user chooses**, which W22 said it could: the repair counts them,
  and says so. The question would need a lenient read of the whole database first.

#### Unmeasured

- The A20, whose wireless adb had dropped, the S9, not attached, and the S24.
- The check of a places of 75 MiB, the most Fenix's maintenance lets it grow to, and of the logins' and autofill's
  when they hold more than a few records.
- A repair of the logins' or autofill's database: places' alone was repaired. A logins or cards database is a few
  pages, most of them page 1's tree.
- A crash during the repair's last steps, between its deleting the damaged database's log and its moving the repair
  into place: the damaged database without its log, and a whole repair beside it, which the next start does not take.
- Damage a store meets mid-session, which is an I/O error as before, and asked about at the next start.
