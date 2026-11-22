# The stage 3 spike

Stage 2's app (`../stage-2/`) with application-services' databases sealed as Gecko's are: places', which holds the
history and the bookmarks, the logins' and autofill's, each through an obfuscating VFS in application-services' own
SQLite with a key from the vault's (`docs/sealed-history.md`).

Firefox 160's tree carries application-services, `third_party/application-services`, the same as its `v160.0` tag
in every component this stage changes, and mach builds its megazord, `libmegazord.so`, from there with
`--enable-appservices-in-tree`, against Gecko's NSS, and Gradle then takes its components as projects of the build.
So stage 3 is one checkout and one series: `patches/` on top of stage 2's, on the same checkout, in an objdir of its
own, `obj-encriox-stage-3`.

Its patches:

- sql-support, which places, the logins and autofill all open their databases through, gains `obfsvfs`: a VFS shim
  after Gecko's obfuscating VFS (`storage/ObfuscatingVFS.cpp`), sealing each 8192-byte page, its write-ahead log's and
  its journal's, with AES-256-GCM from NSS under a key handed to it by a handle, never in the database's name. Each
  page carries a check of the key, which tells a wrong key, refused as not the database's own, from a damaged page,
  an I/O error; neither is a corrupt database, which the components delete and make again. On Android it is SQLite's
  default VFS before any database opens, and refuses a sealed database opened without its key, and one in the clear
  opened with a key;
- places, the logins and autofill each gain a sealed open beside their own, and their Kotlin wrappers and Android
  Components' storages a database key, which a storage asks for, and waits for, when it first opens its database;
- the spike's Fenix gives each of the three a key of its own, an HMAC of `encriox.key` (`EncriDatabaseKeys`), which
  waits for the vault's key, and its gate gains the controls the checks below use;
- sql-support compiles SQLite's recovery extension, from SQLite's own source at the amalgamation's check-in
  (`src/recover/`), against an amalgamation built with `sqlite_dbpage`, and obfsvfs gains a check, which reads a sealed
  database through under its key, and a repair, which copies what the recovery can read of a damaged one into a new
  one under the same key (W25). places carries both to Kotlin for all three databases;
- each store's key then waits, once a process, for its database to be checked (`EncriStores`): one that does not open
  brings up a question from the browser's activity in front, and the store waits for the answer. Damaged, it may be
  repaired, deleted or kept; under another key, only kept; in the clear, deleted or kept. A kept one is not opened that
  session.

## Building

As stage 2's (`../stage-2/README.md`), with this directory's mozconfig: the same checkout, toolchains and state.

```sh
scripts/spike/stage-3/fetch /home/coder/extra_space/encri-extraspace/firefox   # a new checkout; or, on stage 2's:
git -C <firefox> switch -c encriox-spike-stage-3 encriox-spike-stage-2 && git -C <firefox> am scripts/spike/stage-3/patches/*.patch
scripts/spike/stage-3/build /home/coder/extra_space/encri-extraspace/firefox
```

The APKs are under `<firefox>/obj-encriox-stage-3/gradle/build/mobile/android/fenix/app/outputs/apk/debug/`, and
install over stage 1's and stage 2's: the package and the debug key are the same.

To change the patches, commit on the checkout's `encriox-spike-stage-3` branch and write them out again:

```sh
git -C <firefox> format-patch -o "$PWD/scripts/spike/stage-3/patches" --no-numbered --zero-commit encriox-spike-stage-2
```

The components' Kotlin bindings are generated ahead in Firefox's tree and checked in, under
`toolkit/components/uniffi-bindgen-gecko-js/android/components/`, and the patches carry them. After a change to a
UDL, `build` fails at the component's `compileReleaseKotlin`, once mach has built the megazord: run `bindings
<firefox>`, which regenerates them from that megazord and copies places', the logins' and autofill's into the tree,
then `build` again.

`check-obfsvfs-host <firefox>` tests obfsvfs on this machine first, in a minute: the checkout's own module, its check
and its repair, and their tests, against the SQLite application-services bundles, with SQLite's recovery compiled from
the checkout, and a toy cipher where NSS would be (`obfsvfs-host/src/obfsvfs_cipher.rs`), since there is no NSS for the
host.

Cargo takes a vendored crate as it was built last, by its version alone, and stage 3 patches libsqlite3-sys's build
script, for `sqlite_dbpage`: an objdir built before that patch needs the crate's build removed, or the megazord's
SQLite has no `sqlite_dbpage` and every repair fails, *no such table: sqlite_dbpage*. `check-obfsvfs-host` removes its
own each run.

```sh
rm -rf <firefox>/obj-encriox-stage-3/{,aarch64-linux-android/}release-megazord/{.fingerprint,build}/libsqlite3-sys-*
```

## On a phone

The phones and the rules are stage 1's and stage 2's. Stage 2's config, `../stage-2/geckoview-config.yaml`, is the
one to push, over a fresh profile.

- `check-sealed <serial> [sealed|unsealed]`: a fresh start, with the three databases deleted; a visit and a bookmark,
  a login and a card written, each holding a fresh marker where its database keeps it in a column of its own; the
  app's data searched for the marker, and each database's header read; the four read back in a new process; each
  database opened without its key, which must be refused and change nothing; and the four deleted. `unsealed` is
  the control run, with the three in the clear, which must find the marker in each.
- `check-keycheck <serial>`: the key check each sealed page carries (W24). A fresh start with the four written, as
  `check-sealed`'s; every page of the three databases and their logs carrying one check, each database its own; each
  opened under another's key, which must be refused as not its own and change nothing; then the gate's own sealed
  places database, put back with its log's last frame or first frame torn, without its log, and with page 1 or every
  other page damaged: a torn log must open and lose only what the torn frame held, and a damaged page must fail as
  damage, never as the wrong key.
- `check-stores <serial>`: the prompt at start for a sealed database that does not open (W25), answered by tapping
  its buttons on the screen, as a user would. A fresh start with the four written and places filled; places' page 1
  damaged, kept, which must change no file, and asked about again at the next start; a repair of it, which must fail
  and leave it as it was; a page deep in places damaged and repaired, which must keep what can be read and take new
  rows; the logins' page 1 damaged and deleted, their store starting again empty; autofill's database put under the
  logins' key, which must offer no deletion and change nothing; and the three made in the clear, each offered Delete
  and Keep, and deleted. The phone's screen lock must be a swipe at most.
- The gate's controls (`EncriGateActivity`): `--es sealing on|off` for the next start, `--es seal
  write|read|forget|plain|wrong --es marker <text>`, `--es seal fill --ei visits <n>`, `--es stores check`, which logs
  what the check finds of each of the three, asking nothing, `--es keycheck make|read` with `--ei visits <n> --ei more
  <m>` for `make`, and `--ei placesbench <visits> --ei rounds <n>`, which times places in the clear, in the clear with a
  sealed database's 8 KiB pages, and sealed, on three new databases each round outside the stores, and logs each run
  as a line of JSON; `bench-places <serial> [<visits> [<rounds>]]` runs it and prints the medians.

A database is sealed or not when it is made, and one in the clear is refused by a sealed open: stage 1's and stage
2's builds keep the three in the clear, so `check-sealed` deletes them first, and going back to an earlier stage
leaves sealed databases that the earlier build cannot open. Delete them then too:

```sh
adb shell "run-as com.systemhalted.encri.browser.spike sh -c 'rm -f files/places.sqlite* databases/logins2.sqlite* databases/autofill.sqlite*'"
```
