# encri-firefox — amendments to earlier decisions

Each entry corrects a record. Its bold lead names the record possessively ("W1's …") so that
`scripts/docs/decision-index.py` copies it into that record's footer.

## Amendments

- **W1's rejection of scenario 2** (2026-11-14, W3): Chromium is not this repository's browser, but the user wants it
  as a second one, encrium, a project of its own. W1 picked between the scenarios for one browser, and that pick
  stands.
- **W3's owed trademark search** (2026-11-17): run for encriox as a knock-out search, through TMview and Trademarkia,
  with the web, the app stores and the domain registries (`docs/trademark-search.md`). Nothing found blocks the name;
  the nearest live marks are ENCRY and MacPaw's ENCRYPTO, and Encrox, one letter off, is expired. encrium's is still
  owed, by its own project.
- **W3's trademark search, beyond the knock-out one** (2026-11-20, the user's word: "Encriox is an open-source
  project. If we receive a cease-and-desist letter, we will simply change the name then"): no clearance search by
  counsel is run before the first public build. encriox keeps its name on the knock-out search, and a cease-and-desist
  renames it: the app's label, its branding and its repository. Its package, on which encri keys a grant for good and
  Play keys its listing (W8, encri D120), stays `com.systemhalted.encriox` unless the letter asks for that too, which
  would make it a new app that every user installs and allows again.
- **W3's name for the Chromium browser** (2026-11-18, W11): encrium is given up, because `encrium.com` appears to be a
  French technical-training academy's, and the browser is encrimium, *encri* and *(chro)mium*, with the package
  `com.systemhalted.encrimium`. Its knock-out search found nothing that blocks it (`docs/trademark-search.md`).
- **W4's first item of what stage 1 has to do** (2026-11-15, W5): the length a response promises is not enough to tell
  a stopped copy from a finished one. A download with no length, paused or cancelled, was saved cut short into the
  vault. The writer also asks whether the download is still running, and a retry starts from the start.
- **W12's fix to QuotaManager** (2026-11-18, W13): a few lines, not one. Built as stage 2's patch 1, it recorded the
  test origin as encrypted on the G6 and the S9, which had recorded it as plaintext, and their site storage read back.
- **W12's ways to turn SQLite's switch on** (2026-11-18, W13): editing the published GeckoView's prefs is not one.
  Gecko's launch guard reads the switch before any pref is loaded, so a profile a pref encrypted is refused at its
  next start, as the S9's was. A build turns it on by its default.
- **W14's second flag** (2026-11-18, W15): Gecko no longer starts with the profile's keys locked when the vault is
  locked or does not answer. On the user's word, the app brings up encri's unlock while Gecko waits, and ends before
  Gecko has opened anything of the profile if the vault stays shut. Only the gate's `kek withhold` starts it locked.
- **W14's fifth flag** (2026-11-18, W15): a vault was locked, on the user's word. The A20's, locked through the gate as
  encri's own *Lock* does, had the app lock Gecko's keys as the gate's `drop` had, and opened again with the
  passphrase on file, typed into encri's screen.
- **W17's order of work** (2026-11-20, the user's word: of the spikes' bugs, "We should fix these before continuing,
  if they are not fixed yet", and of history and bookmarks in a sealed store, "Let's work on this next"): the three
  bugs come first, fixed in the spikes, then history and bookmarks in a sealed store, then the rest of item 6. W17 has
  the first usable build go without history and bookmarks; whether it waits for them now is the user's to say.
- **W5's first rejection** (2026-11-20, W19): a stopped download is left paused or cancelled, and *Resume* starts one
  nothing was kept of from the start, so it no longer fails for want of a file in `Download/`. W5's flag 2 ends.
- **W6's third flag** (2026-11-20, W20): a lock no longer leaves the stores' keys in memory. The browser closes at a
  lock, and its process ends with every key it held.
- **W14's lock mid-session** (2026-11-20, W20): the app no longer locks Gecko's keys at a lock and goes on. It
  closes the browser, letting Gecko write what it holds first, and only the gate's `--es profile lock` still locks
  them mid-session.
- **W15's reading of the lost local storage** (2026-11-20, W20): not a lock inside local storage's 5 seconds. Three
  runs with the lock at most 1.5 s after the write kept it every time, on an origin with all its stores and on one
  with local storage alone; what lost it once is unknown, in a Gecko running on with its keys locked, which no longer
  happens.
- **W17's first build** (2026-11-20, W21, the user's word: "Let's wait for 1 and 2 before calling the first usable
  build"): it waits for history and bookmarks, the logins and the cards in sealed databases, and carries them.
  History is recorded and bookmarks are on in its menus; the logins' origins are sealed with the rest of their
  database, not disclosed. Collections and pinned top sites, which are not application-services', stay off.
- **W16's reading of obfsvfs's header** (2026-11-20, W21): page 1's first 32 bytes stay in the clear, not 24
  (`kClearTextPrefixBytesOnFirstPage` in `storage/ObfuscatingVFS.cpp`): SQLite's 16-byte magic, the page size and the
  reserved bytes, and the change counter and page count after them.
- **W6's logins' origins** (2026-11-20, W21): each login's origin and times, which W6 left as plain columns of the
  logins' database, are sealed with the database now, under a key of their own from the vault's.
- **W19's unmeasured G6** (2026-11-20): measured, once its owner had unlocked its vault. The six cases held as on the
  A20.
- **W20's unmeasured step 5 of `check-kek`** (2026-11-20): measured on the G6, on stage 3's build, which carries patch
  0012. The browser closed at the vault's key dropped, and the third origin read back whole, local storage among it.
- **W21's unmeasured G6** (2026-11-20): measured, as on the A20 and the S9; the cipher costs 5% of a visit's write
  there. Its first bench made `plain-8k` with 4 KiB pages, which patch 0007 fixes.
- **W22's first flag** (2026-11-20, the user's word: "yes, add the key check next session"): a key check in each
  sealed page tells a wrong key from a damaged file, and the prompt offers a repair beside deleting and keeping.
- **W19's, W20's and W21's unmeasured S24** (2026-11-20, W23): measured, on its owner's word, all as on the other
  phones; W20's real lock not, which would lock the S24's vault.
- **W4's, W5's, W6's and W21's private and sealed results** (2026-11-20, W23): they searched copies of the app's data that
  nothing checked were whole. Run again with `copy-data` on the G6, the A20 and the S24, they held; the S9's rest on
  the old copies until it is attached.
- **W21's torn frame of the log** (2026-11-21, W24): a frame that does not open is taken for torn once the key is
  shown to be the database's own, by page 1 or by a page's check, and not only once another frame has opened. A log
  whose first frame a crash tore made its whole database an I/O error. A key not the database's is refused at the
  open, `SQLITE_AUTH`.
- **W21's unmeasured torn log and failed page** (2026-11-21, W24): measured on the G6 and the A20, with patch 0008's
  key check. A torn last frame lost its commit alone, a torn first frame read what the database held, and a damaged
  page failed as damage, never as the wrong key.
- **W22's first flag, built** (2026-11-21, W24): as stage 3's patch 0008. A key is refused as not the database's where
  no page carries its check and some page carries another's, so that a page 1 damaged through its check is told by
  the next.
- **W22's prompt at start, built** (2026-11-22, W25): as stage 3's patches 0010 to 0017, measured on the G6. A damaged
  database may be repaired, deleted or kept, one under another key only kept, one in the clear deleted or kept; a
  kept one's store is refused its key that session, so that nothing opens it; nothing is deleted unasked.
- **W24's first flag** (2026-11-22, W25): the prompt at start does not read the components' messages; it tells a
  wrong key from damage by obfsvfs's own check, before the store opens anything.
