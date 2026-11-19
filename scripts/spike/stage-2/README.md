# The stage 2 spike

Stage 1's app (`../stage-1/`) with Lockstore's switches on, Gecko's at-rest encryption (W5, W7; the study's
*Stage 2*). `geckoview-config.yaml` turns the switches on through the debug config a debuggable GeckoView reads.

Its first round ran stage 1's build, against the GeckoView Mozilla published for 160, with that config alone (W12).
Two of the switches need Gecko changed, so from its second round it builds Gecko itself (W13), with `patches/` on top
of stage 1's:

- QuotaManager records an origin's encryption when it makes the origin's directory, which at 160 it can record as
  plaintext, so that the next start cannot read the site's storage (W12);
- the SQLite switch is no longer locked off, and is on by default: Gecko's launch guard reads it before any pref is
  loaded, so a profile the config encrypted was refused at its next start (W13).

From its second round the profile's keys are kept under a secret from the vault's key (W14):

- GeckoView asks the app for the secret at startup, on Gecko's own thread, and Gecko keeps every key of the profile
  under a password KEK it unlocks; `GeckoRuntime.lockProfile` and `unlockProfile` lock and unlock it mid-session
  (0005);
- the spike's Fenix opens the vault for it and gives an HMAC of `encriox.key`, or nothing if the vault is locked, and
  locks and unlocks Gecko's keys as the vault's key is dropped and held again (0006);
- each of Lockstore's stores reads its key under whichever KEK wraps it, and keeps a locked key rather than replace it
  (0007);
- the gate can drop the vault's key, as a lock does, without locking the vault (0008);
- Lockstore truncates its write-ahead log when it deletes a key, which otherwise stays on disk there (0009).

The browser does not start without the vault (W15): with the vault locked, or not letting the browser in, at startup,
the app brings up encri's own screens while Gecko waits, and if the vault is still shut once they are done, it ends
before Gecko has opened anything of the profile (0010).

## Building

On the drive for large builds (W7). The checkout's objdir and mach's state go beside it there, with the toolchains
mach bootstraps: clang, the NDK, a sysroot and a JDK, the same release as stage 1's. Rust, which mach bootstraps only
in Mozilla's automation, is the toolchain Mozilla's CI builds Android with, `linux64-rust-android`, which `build`
fetches into the same state. The SDK and bundletool are stage 1's, under `~/.mozbuild` (`../stage-1/README.md`).

```sh
scripts/spike/stage-2/fetch /home/coder/extra_space/encri-extraspace/firefox
scripts/spike/stage-2/build /home/coder/extra_space/encri-extraspace/firefox
```

`build` compiles Gecko with `mach build`, then builds Fenix from the checkout's root Gradle build, where GeckoView is
a project of the build, not an artifact. Run no other Gradle build in the suite at the same time. The APKs are under
`<firefox>/obj-encriox-stage-2/gradle/build/mobile/android/fenix/app/outputs/apk/debug/`, and install over stage 1's:
the package and the debug key are the same, so the vault's grant and `encriox.key` stay.

To change the patches, commit on the checkout's `encriox-spike-stage-2` branch and write them out again:

```sh
git -C <firefox> format-patch -o "$PWD/scripts/spike/stage-2/patches" --no-numbered --zero-commit encriox-spike-stage-1
```

## On a phone

The phones and the rules are stage 1's (`../stage-1/README.md`): the ones that hold encri's test vaults, and the S24
only with its owner's word. `ADB` works as it does there. Every start of the app asks the vault for the secret first;
with the vault locked, encri's unlock comes up, and the app stops if it is refused. Only the gate's `kek withhold`
starts Gecko with the profile's keys locked, or makes a profile in the clear.

- `check-lockstore <serial>`: gives the spike a fresh Gecko profile, runs stage 1's `check-plaintext <serial> normal`
  with this directory's config, prints the Gecko that made the profile, the header of every SQLite file in it and each
  origin's flags in QuotaManager, then starts the app again and reads back, store by store, what the first session
  wrote.
- `check-kek <serial>`: the second round, in six steps (its header): the secret given at both starts, withheld at a
  start, given again, locked mid-session, the vault's key dropped and read again, and a profile made without it and
  then given it. It prints what each store read back, the files and keys that changed while locked, and which KEK
  wraps each key, with any LocalKey left in Lockstore's database or its log. The vault must be open to the spike.
- `check-unlock <serial>`: the vault itself locked and unlocked (W15), in five steps (its header): a lock mid-session,
  then encri's unlock from the gate; a start with the vault locked, unlocked; one where the unlock is refused, which
  must stop the app and leave the profile as it was; and the vault left open. It types the passphrase on file into
  encri's screen, never printing it, so it is for a test vault whose passphrase is on file; the G6's would not open.
- The gate's controls (`EncriGateActivity`): `--es kek give|withhold` for the next start's secret, `--es profile
  lock|unlock` for the running Gecko's keys, and `--ez drop true` to drop the vault's key as a lock does.
- `fresh-profile <serial>`: deletes the spike's Gecko profile and its cache, and nothing else. A profile's SQLite
  switch is set when it is made, so run it before going back to stage 1's config too.

Stage 2's build encrypts a profile's databases unless a config turns the switch off, and stage 1's config does not
name it. So stage 1's scripts run against stage 1's build: install its APK again, over a fresh profile, after a round.
