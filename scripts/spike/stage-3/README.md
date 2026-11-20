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
  its journal's, with AES-256-GCM from NSS under a key given in the database's URI. A page that does not open under
  the key is an I/O error, never a corrupt database, which the components delete and make again; on Android it is
  SQLite's default VFS before any database opens, and refuses a sealed database opened without its key, and one in
  the clear opened with a key;
- places, the logins and autofill each gain a sealed open beside their own, and their Kotlin wrappers and Android
  Components' storages a database key, which a storage asks for, and waits for, when it first opens its database;
- the spike's Fenix gives each of the three a key of its own, an HMAC of `encriox.key` (`EncriDatabaseKeys`), which
  waits for the vault's key, and its gate gains the controls the checks below use.

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

`check-obfsvfs-host <firefox>` tests obfsvfs on this machine first, in a minute: the checkout's own module and its
tests, against the SQLite application-services bundles, with a toy cipher where NSS would be
(`obfsvfs-host/src/obfsvfs_cipher.rs`), since there is no NSS for the host.

## On a phone

The phones and the rules are stage 1's and stage 2's. Stage 2's config, `../stage-2/geckoview-config.yaml`, is the
one to push, over a fresh profile.

- `check-sealed <serial> [sealed|unsealed]`: a fresh start, with the three databases deleted; a visit and a bookmark,
  a login and a card written, each holding a fresh marker where its database keeps it in a column of its own; the
  app's data searched for the marker, and each database's header read; the four read back in a new process; each
  database opened without its key, which must be refused and change nothing; and the four deleted. `unsealed` is
  the control run, with the three in the clear, which must find the marker in each.
- The gate's controls (`EncriGateActivity`): `--es sealing on|off` for the next start, `--es seal
  write|read|forget|plain --es marker <text>`, and `--ei placesbench <visits> --ei rounds <n>`, which times places in
  the clear and sealed, on two new databases each round outside the stores, and logs each run as a line of JSON.

A database is sealed or not when it is made, and one in the clear is refused by a sealed open: stage 1's and stage
2's builds keep the three in the clear, so `check-sealed` deletes them first, and going back to an earlier stage
leaves sealed databases that the earlier build cannot open. Delete them then too:

```sh
adb shell "run-as com.systemhalted.encri.browser.spike sh -c 'rm -f files/places.sqlite* databases/logins2.sqlite* databases/autofill.sqlite*'"
```
