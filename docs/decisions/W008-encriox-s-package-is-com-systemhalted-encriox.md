# W8 — encriox's package is `com.systemhalted.encriox`

*2026-11-17 · accepted on the user's word ("It should be `com.systemhalted.encriox`"), and **flagged for veto on one
point** (below). Settles the package W1 moved after the spikes. Nothing is built under it yet.*

#### What the user decided

- **The package is `com.systemhalted.encriox`**: the application ID Android installs encriox under, and the one encri
  keys a grant on for good (encri D120, D145).

#### What was settled

1. **It is permanent from the first build a user installs.** A grant is keyed on it, the registry names it, and Google
   Play never lets an app change it.
2. **It leaves the companions' pattern**, `com.systemhalted.encri.files`, `.images` and `.backup`, as the name does:
   encriox is named as a product of its own (W3), not as *Encri Browser*.
3. **The code keeps Fenix's namespaces**, `org.mozilla.fenix` and Android Components' `mozilla.components`. Only the
   application ID changes, as the spike's did, so that each patch stays the size of the change it makes and a
   release's rebase (W9) never meets a rename in every file.
4. **encri-api lists it before its first release**, as `registry/com.systemhalted.encriox.app`, with the certificate
   it is signed with and a floor (encri-api L15). Under encri D130 that entry is what keeps a grant across the updates
   W9 brings every two weeks: without it, the user consents again after each.
5. **The spikes keep `com.systemhalted.encri.browser.spike`** (W1, flag 1), so the package that ships starts with no
   grant from them.

#### Checked (2026-11-17, in the source)

- **Nothing in encri or encri-api matches a package by prefix**, so `com.systemhalted.encriox`, whose first characters
  are `com.systemhalted.encri`, is not taken for encri's own. The registry matches a package whole, by its file's name
  (encri-api L15), and neither encri's sources nor encri-client's compare one with `startsWith`.

#### Flagged for veto

1. **A debug build is `com.systemhalted.encriox.debug`.** Fenix's build types add a suffix to the package, as the
   spike's `.spike` does, so a debug build installs beside a release on the same phone and holds a grant of its own.

#### Rejected

- **`com.systemhalted.encri.browser`**, the spike's package without its suffix: it cannot tell encriox from ~~encrium~~ encrimium
  (amended by W11).
- **`com.systemhalted.encri.encriox`**, after the companions: it says *encri* twice.
