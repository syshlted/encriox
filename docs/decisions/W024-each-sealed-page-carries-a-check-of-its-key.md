# W24 — Each sealed page carries a check of its key, which tells a wrong key from a damaged page, and a torn log ends where it was torn

*2026-11-21 · built and measured on the user's word (W22's amendment, "yes, add the key check next session"; then
"ok let's do The key check, tested together with the crash-torn log and the failed page on a phone"), as stage 3's
patch 0008, with the gate's controls in 0009, on the G6 and the A20. Ends `ROADMAP.md`'s key check in item 6, and
W21's torn log and failed page, which were tested on this machine alone. **Flagged for veto on four points**
(below).*

#### What was built (stage 3's patch 0008, in sql-support's obfsvfs)

- **Each page's last 4 reserved bytes, zero until now, carry its key's check**: the first 4 bytes of an HMAC-SHA-256,
  from NSS, of a label of obfsvfs's own, `appservices-obfsvfs key check`, under the database's key, with the last bit
  set, so that no key's check is zero, as the bytes are in a page sealed before the checks and in a frame never
  written. A page's 32 reserved bytes are now the tag, the nonce and the check: 16, 12 and 4. The check is the same in
  every page of a database, its log's and its journal's, and tells nothing of the key: 31 bits of a MAC under a key of
  256.
- **A database's open judges the key from its files as they are on disk**, before SQLite reads anything of them:
  - page 1 opening under the key, or any page of the database or of its log carrying the key's check, shows the key to
    be the database's own. Under its own key that is page 1 alone; a page 1 damaged, even through its check, is told
    by the next page's;
  - where none does and some page carries another check, **the key is not the database's, and the open is refused**,
    `SQLITE_AUTH`, which SQLite reports as *authorization denied*, before anything of the database is opened or made,
    its log's index included. Nothing deletes a database for it;
  - where no page carries a check, as in a database sealed before the checks, nothing is told, and it opens as it did.
- **Under its own key, a page that does not open stays damage**, `SQLITE_IOERR_DATA`, *disk I/O error*, and **a frame
  of its log that does not open is passed to SQLite**, whose checksum ends the log there (*Found on the way*, 1).
- **The gate's controls** (patch 0009): `--es seal wrong` opens each of the stores' three databases under another's
  key, and `--es keycheck make|read` keeps a sealed places database of the gate's own in the cache, under a key of its
  own from the vault's, with visits closed into the database and more left in its log. **`check-keycheck`** tears
  that log and damages those pages between the two, by hand: a frame is torn as a write cut short at a log's end
  leaves one, its page's second half zeros, its check among them.

#### Measured (2026-11-21, patch 0009's build)

| | G6, Android 9 | A20, Android 11 |
|---|---|---|
| Every page's check, in the stores' three databases and their logs | one in each, its own: places' 51 pages and 76 frames, the logins' 12 pages, autofill's 1 and 22 | the same, with checks of its own |
| Each under another's key (`seal wrong`): places under the logins', the logins under autofill's, autofill under places' | refused, *authorization denied*; no database or log changed | the same |
| The gate's database as it is: 200 visits closed in, 5 more in its log | 205 visits read | 205 |
| Its log's last frame torn | 204: the torn commit lost, and nothing else | 204 |
| Its log's first frame torn | 203, what the database holds without its log | 203 |
| Page 1 damaged, no log | *disk I/O error* at the open; the database unchanged | the same |
| Every other page damaged, no log | *disk I/O error* at the open; the database unchanged | the same |
| `check-sealed`, on the same build | every step held | every step held |

The 5 visits crossed places' checkpoint at 62 frames, about 20 frames a visit with 8 KiB pages: the database took
three, and the log began again with the last two, its first frame theirs. So the first frame torn lost the two, and
read what the database held alone.

**On this machine** (`check-obfsvfs-host`, with its toy cipher): 15 tests, 6 of them new, pass. Each part of the fix
broken in turn failed the tests it should: no check written, 4; every open's verdict unknown, as patch 0007's was, 3,
the torn first frame among them; page 1's check alone looked at, 1, page 1 damaged whole.

#### Found on the way, and fixed

1. **A log whose first frame a crash tore made its whole database an I/O error**, in patch 0007's build and before.
   obfsvfs passed a frame that did not open to SQLite as torn only once another frame of the log had opened, which
   was how it told a torn frame from a wrong key. The first frame of a log, which the first commit after a checkpoint
   writes, has none before it, and was answered as a wrong key: `SQLITE_IOERR_DATA`, and places, the logins or
   autofill would not have opened until the file was deleted, with everything up to the torn commit still in it. With
   the key shown at the open, a torn frame ends the log there, as SQLite in the clear does: on the G6 and the A20 the
   gate's database opened and read the 203 visits it held. Found in reading, before the first build.
2. **A new sealed database writes its page 1 into its file at once**, not into its log: SQLite takes the empty
   header obfsvfs serves, and sets the journal mode by writing page 1. So a database still wholly in its log, which
   the log's part of the verdict was first written for, does not arise. What that part covers is a database sealed
   before the checks and written since, whose newer frames carry them (a host test).
3. **Fenix warms its four stores up at every start**, each opening its database under its own key in the gate's
   process too, and an open after the app was stopped rebuilds a log's index, `-shm`, from the log. `check-keycheck`'s
   first run counted autofill's index as changed by the wrong keys' opens, which never reach it, and
   `check-sealed`'s keyless step had the same flaw, passing by its timing. Both now compare the databases and their
   logs, and name an index that changed.
4. **The gate's first `keycheck make` kept its log**: a places writer's own close does nothing while its API is open,
   so the API is closed first. The gate's own check refused to go on.

#### Flagged for veto

1. **A wrong key is `SQLITE_AUTH`**, SQLite's answer to a statement its authorizer refused, reported as *authorization
   denied*. It is neither an I/O error, which is damage, nor `SQLITE_CANTOPEN`, which a sealed database opened without
   a key is, nor a corrupt database, which places and autofill delete. The components carry it to Kotlin as an SQL
   error with that message, which is what the prompt at start (`ROADMAP.md`, item 6, next) tells the two by.
2. **A database is refused as another key's only where no page carries its key's check and some page carries
   another**. A wrong key whose check is the database's, 1 in 2³¹, would be taken for damage, and offered for
   deletion; a database damaged in page 1 and in every page's check would be taken for another key's, refused and
   kept. Every page's 4 bytes are read only for a key that is not the database's, which W6 keeps from arising.
3. **A database sealed before the checks opens as it did**, and gains them as its pages are written again: under its
   own key page 1 opens, and under another nothing tells, and a page that does not open is an I/O error. The spike's
   phones had such databases, which `check-sealed` and `check-keycheck` delete first; encriox has none.
4. **A frame of the log that does not open ends the log there**, whether a crash tore it or it was damaged after:
   under its own key nothing tells the two apart, as SQLite's checksum cannot in the clear, and the commits after a
   damaged frame are lost, as they are there. A damaged page of the database itself is an I/O error, which the prompt
   can count.

#### Rejected

- **The check bound into each page's tag**, as its additional data: it would tell nothing more, and pages sealed
  before it would no longer open.
- **The wrong key told at the first read**, by an I/O error of its own (`SQLITE_IOERR_AUTH`): SQLite gives every I/O
  error the one message, *disk I/O error*, so a store could not tell it from damage; and by then SQLite has opened the
  log's index.
- **Page 1's check alone, without opening page 1**: a page 1 damaged through its check would read as another key's
  (the host tests' third break).

#### Unmeasured

- What the verdict costs an open: page 1 read and opened, one page, at each connection's open of a database already
  made; a new database's reads nothing. Not timed.
- A crash's own tear: the frames were torn by hand. The S9, which is not attached, and the S24.
- Patch 0007's build on a phone with a torn first frame: its verdict was made again on this machine alone.
