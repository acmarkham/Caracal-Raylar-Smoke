# Adoption and implementation gaps

These are design defaults, not a claim that every existing image complies.
This documentation change does not alter firmware. Code differences below are
migration work, not additional override layers. Explicit target exceptions live
in the target ADR's `Policy overrides` block.

| Area | Evidence and adoption |
| --- | --- |
| Startup | UI adopts the ascending 1047/1319/1568 Hz pattern already shared by Integration 003/004, with volume 96 from 004. Integration 002's three 1000 Hz beeps and Aardwolf's 900 Hz tone still need alignment. |
| Severe error | UI adopts 003's descending 440/330/220 Hz pattern and 10-second repetition, and 002's latched 1 Hz fault flash. 002 currently uses 1200/800/400 Hz; Aardwolf uses one 380 Hz tone and a solid red LED; 004 currently latches red without an error sound. Only 002's extra red LED and 004's solid fault LED are retained as explicit ADR exceptions. |
| GPS-green | UI adopts 002's solid search / admitted PPS pulse / inactive-off meanings. 003/004 currently pulse admitted anchors without a solid search state; Aardwolf currently polls raw PPS counts. 002 waits on Time publications, so its receiver-state responsiveness also needs review. A raw PPS must not imply admitted UTC timing. |
| GPS sounds | First GPS success and calibration sounds currently vary. They remain target-specific ADR decisions; this initial policy does not standardise every sound. |
| Storage | Hourly folders follow the recorder default and Integration 002/Aardwolf requirements. Storage ADR daily-folder examples remain examples of a supported alternative, not inherited defaults. GPS filename sequence suffix follows the actual Storage path builder. |
| Audio | Library default remains 3600 seconds. Integration 002 and Aardwolf explicitly select 60 seconds; folder interval is independent. The partial-hour startup example in the Storage ADR is superseded by the Audio policy's interval alignment. |
| Durable logging | Queue/loss requirements are shared design requirements. Existing use of bounded queues or a flush timer alone does not prove complete logs, nonblocking producers, or numbered long-record handling. Verify these per target. |
| Radio | Integration 004's accelerated schedule and Aardwolf's profile campaign are target exceptions to ordinary rendezvous defaults. RF run values are not reusable repository defaults. |

Other smoke, driver and service tests may predate these defaults. Their existing
code is not an exception declaration or evidence of conformance. Apply the
relevant policies when those targets are next revised.

When updating firmware, verify startup/error tones, searching/tracking/standby
and rejected-PPS indications, and fault priority on hardware. Check generated
storage paths and restart collision behaviour. Exercise log overload and media
failure while confirming timing-critical work continues as required by its ADR.
Update this list as implementation gaps close; do not add overrides merely to
make existing drift appear compliant.
