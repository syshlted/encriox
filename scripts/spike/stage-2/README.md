# The stage 2 spike

Stage 1's app (`../stage-1/`) with Lockstore's switches on, Gecko's at-rest encryption (W5, W7; the study's
*Stage 2*). `geckoview-config.yaml` turns the switches on through the debug config a debuggable GeckoView reads.

Its first round ran stage 1's build, against the GeckoView Mozilla published for 160, with that config alone (W12).
Two of the switches need Gecko changed, so from its second round it builds Gecko itself (W13), with `patches/` on top
of stage 1's:

- QuotaManager records an origin's encryption when it makes the origin's directory, which at 160 it can record as
  plaintext, so that the next start cannot read the site's storage (W12);
- the SQLite switch is no longer locked off, so the config can turn it on.

## Building

On the drive for large builds (W7). The checkout's objdir and mach's state, with the toolchains it bootstraps, go
beside it there; the JDK, SDK and bundletool are stage 1's, under `~/.mozbuild` (`../stage-1/README.md`).

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
only with its owner's word. `ADB` works as it does there. No vault needs to be open.

- `check-lockstore <serial>`: gives the spike a fresh Gecko profile, runs stage 1's `check-plaintext <serial> normal`
  with this directory's config, prints the Gecko that made the profile, the header of every SQLite file in it and each
  origin's flags in QuotaManager, then starts the app again and reads back, store by store, what the first session
  wrote.
- `fresh-profile <serial>`: deletes the spike's Gecko profile and its cache, and nothing else. A profile's SQLite
  switch is set when it is made, so run it before going back to stage 1's config too.
