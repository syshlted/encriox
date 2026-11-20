# The stage 1 spike

Fenix, Firefox for Android, at the release in `versions`, built with Gradle against the GeckoView Mozilla published
for it, with `patches/` on top (W1, the study's *Stage 1*):

- a throwaway package, `com.systemhalted.encri.browser.spike`, never published (W1, flag 1);
- every tab private, a link from another app's included;
- downloads into the folder the user gave it in the vault, through encri-client's `write`, all of one or none of it:
  a download paused or cancelled part way leaves nothing (W5), shows as paused or cancelled, and resumes whole from
  the start (W19);
- a private tab's thumbnail kept in memory, never on disk (W6);
- a key of its own, `encriox.key`, 32 random bytes in that folder and held in memory while the vault is open, which
  seals the logins' and the cards' keys in place of Fenix's `SecureAbove22Preferences`, and which those stores wait
  for (W6);
- `org.mozilla.fenix.encri.EncriGateActivity`, the controls the scripts below start over adb.

What it found is recorded in the decision that follows it, not here.

## Mozilla's toolchain, apart from the suite's

Gradle runs on Mozilla's JDK and SDK, under `~/.mozbuild`, so encri's toolchain is left as it is.

| What | How |
|---|---|
| JDK | Temurin `jdk-17.0.20.1+1` from Adoptium's release, checked against its `.sha256.txt`, unpacked to `~/.mozbuild/jdk/` |
| SDK | `~/.mozbuild/android-sdk-linux`, with `sdkmanager --sdk_root=…` installing `build-tools;37.0.0`, `platforms;android-37.2`, `platform-tools`, `cmdline-tools;23.0` and `emulator`, which configure insists on |
| Bundletool | 1.18.3 from Google's release, to `~/.mozbuild/bundletool.jar` |
| The rest | `mach` bootstraps it at configure (`mozconfig`): Node, nimbus-fml, embedded-uniffi-bindgen |

## Building

```sh
scripts/spike/stage-1/fetch ../../firefox      # about 6 GB; keep it outside the suite's directory
scripts/spike/stage-1/build ../../firefox      # the first time, also an artifact build: about 6 GB of objdir
```

The first build also fills `~/.gradle` with about 5 GB. Run no other Gradle build in the suite at the same time. The
APKs, one per ABI, are under `<firefox>/obj-encriox-spike/gradle/build/mobile/android/fenix/app/outputs/apk/debug/`;
the four phones are all arm64-v8a. Their label is still Fenix's own, which is a spike's and nobody else's.

To change the patches, commit on the checkout's `encriox-spike-stage-1` branch and write them out again:

```sh
git -C ../../firefox format-patch -o "$PWD/scripts/spike/stage-1/patches" --no-numbered --zero-commit FIREFOX_160_0_RELEASE
```

## On a phone

Only the phones that hold encri's test vaults, and the S24 only with its owner's word (`AGENTS.md`, *Hazards*).

```sh
adb install -r <firefox>/obj-encriox-spike/gradle/build/mobile/android/fenix/app/outputs/apk/debug/app-arm64-v8a-debug.apk
# API 28, the G6: both, then a restart, or Fenix's own writer is refused in Download/ with "Permission denied".
adb shell pm grant com.systemhalted.encri.browser.spike android.permission.READ_EXTERNAL_STORAGE
adb shell pm grant com.systemhalted.encri.browser.spike android.permission.WRITE_EXTERNAL_STORAGE
adb shell am force-stop com.systemhalted.encri.browser.spike
adb shell am start -n com.systemhalted.encri.browser.spike/org.mozilla.fenix.encri.EncriGateActivity
```

The gate opens the vault to the spike: encri's own screens ask for a folder and, if the vault is locked, its
passphrase. `adb logcat -s encriox-spike` says `gate open scope=…` once it holds a session. The scripts run `adb` from
`$ADB` when it is set, such as `ADB="$HOME/Android/Sdk/platform-tools/adb -P 5038"` for a phone on the host's USB
(encri's `docs/dev-environment.md`).

- `check-plaintext <serial> normal`, then `check-plaintext <serial> private`: a page writes a fresh marker into every
  store a page can reach, and the app's data is searched for it. The normal run is the control and must find it.
  `CONFIG` names another Gecko config to push, as stage 2's does (`../stage-2/`).
- `measure <serial> <MiB> <rounds>`: downloads into the vault against Fenix's own writer, from memory and through
  Gecko from the spike's own server on the phone's loopback.
- `check-stopped <serial> pause|cancel sized|unsized [retry]`: a download into the vault stopped part way, as its
  notification's buttons stop it, and then tried again. The vault must hold nothing of a stopped one, and all of one
  tried again.
- `check-keys <serial>`: a login and a card, each a fresh marker, are saved, and the app's data is searched for them
  and for a key in the clear. A new process then asks for the logins before the vault is open: they must arrive after
  the vault's key, whole. Both are deleted after. It needs the vault unlocked, and leaves `encriox.key` in it.
