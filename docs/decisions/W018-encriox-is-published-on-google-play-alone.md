# W18 — encriox is published on Google Play alone, and its source on its public remote

*2026-11-20 · accepted on the user's word ("If Encri is play store only, there isn't much point to publishing to
f-droid"), and **flagged for veto on two points** (below). Answers `ROADMAP.md`'s *Where it is published*, which was
the user's to choose. Nothing of encriox is built yet.*

#### What the user asked, and decided

- **"Are we required to publish the apk outside of the play store?"** No. MPL-2.0 asks for the source, not for a
  second place to get the app (below).
- **Not F-Droid**: encri is published on Google Play alone (encri D142), so whoever installs encriox from F-Droid
  still needs Play for encri, without which encriox does not start (W15).

#### What was settled

1. **encriox's releases go to Google Play, and nowhere else**: not F-Droid, and not a signed APK on GitHub's releases.
   The second follows from the user's reason for the first, so it is flagged.
2. **What MPL-2.0 asks is the source, which the public remote holds.** Section 3.2 (`LICENSE`): whoever distributes
   the executable must make the source available, "and You must inform recipients of the Executable Form how they can
   obtain a copy of such Source Code Form by reasonable means in a timely manner". Play is a distribution, so each
   release's source is on `syshlted/encriox` (W2), tagged at that release, and both the Play listing and the About
   screen say where it is. The About screen's line is item 6's (`ROADMAP.md`); the listing's is the release's.
3. **Nothing in encri or the contract changes.** The registry keys encriox on its package and its signing
   certificate's SHA-256 (W8, encri D130, D143 point 4), whoever delivers it.

#### Flagged for veto

1. **No signed APK on GitHub's releases either.** A user without Play has no encri to open the vault with, so such an
   APK serves only someone who has encri from Play and would rather sideload encriox. D142 keeps encri's own signed APK
   with the maintainer for the reasons it gives, and encriox's can be kept the same way.
2. **Each release tagged on the public remote**, as the way to tell a recipient which source is the release's.

#### Not decided here

- **Nightlies.** D143 makes the open-source apps' nightlies GitHub pre-releases under a key nothing trusts, with
  `.nightly` on the package. Whether encriox has them, at Firefox's two-week cadence, is open.
- **If encri is ever offered outside Play** (D142, *Not decided*), this is weighed again.

#### Rejected

- **F-Droid**: the user's reason. F-Droid publishes a signed APK of ours only when its own build of the same source
  matches it (encri D115), and a Gecko build is the 76-minute one stage 2 measured (W13); encri, which encriox needs,
  is not open source and not there (encri D116).
