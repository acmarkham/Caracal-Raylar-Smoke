# Shared policy defaults

Policies describe reusable behaviour; ADRs describe architecture and the few
decisions specific to a test or application. YAML is design input for ADRs and
coding, not firmware configuration or automatic enforcement.

Use only this precedence, from lowest to highest:

1. Repository default: one YAML file per policy domain in this directory.
2. Optional scoped policy: at most one overlay per domain for a given target.
3. Explicit override in the target's ADR.

A scope may name a platform, board revision, application, test, feature, or
deployment. These are labels, not six inheritance levels. Do not chain overlays,
use `extends`, or resolve conflicts by directory order. If two overlays for the
same domain match a target, consolidate them or select one explicitly in its ADR.
Most targets should use the repository defaults directly.

Defaults apply to every new or revised ADR and implementation that uses the
named capability, including driver and integration tests. No copy or opt-in list
is required. An omitted capability need not be added just to satisfy a policy;
for example, an image without storage has no system-log requirement. A driver
test that exposes GPS indications still follows the UI policy, and must declare
an override if it intentionally shows raw PPS without a Time Service.

Only structured entries in a scoped policy or an ADR's `Policy overrides`
block override a default. Existing prose, examples,
Rust configuration values, and implementation differences are not implicit
overrides. Where old prose repeats a conflicting default, the policy supplies
the current shared behaviour; see [migration_notes.md](migration_notes.md).
Architecture, hardware limits, and service ownership remain defined by ADRs;
changing them requires an architecture decision, not a policy overlay.

| Policy | Shared decisions |
| --- | --- |
| [ui.yaml](ui.yaml) | Startup/error sounds, GPS-green states, indication priority |
| [storage.yaml](storage.yaml) | UTC hourly folders, names, append and recovery |
| [gps.yaml](gps.yaml) | Search/duty-cycle defaults and calibration handshake |
| [time.yaml](time.yaml) | PPS admission, holdover and clock discipline |
| [audio.yaml](audio.yaml) | Recording boundaries, WAV finalisation and gaps |
| [logging.yaml](logging.yaml) | Persistent diagnostics, flush and loss accounting |
| [versioning.yaml](versioning.yaml) | Startup traceability and missing identity fields |
| [location.yaml](location.yaml) | Fix admission and retained-location semantics |
| [power.yaml](power.yaml) | Power-state thresholds and unknown SOC |
| [sensors.yaml](sensors.yaml) | Polling, units and failed-reading semantics |
| [radio.yaml](radio.yaml) | Rendezvous defaults and approved profile selection |

LED/buzzer drivers provide mechanisms; UI owns their meanings. Storage owns
paths; Audio owns segmentation. Policies refer to these boundaries without
inheriting from one another. Add another domain only for reusable decisions,
not for every driver API or every application setting.

## File and override format

Each default file has `schema_version: 1`, a unique `policy` matching its
filename, `scope: repo`, `applies_to`, `defaults`, and repository-relative
`sources`. Only `defaults` contains normative values. Unit suffixes are explicit;
tone volume is the driver's integer scale 0–255. Strings describing behaviour
are requirements for a human/code agent, not executable expressions.

An optional overlay lives in `overrides/<scope>/<domain>.yaml` and names its
scope explicitly. It contains `schema_version`, `policy`, `scope`, and
`overrides`; each entry is a dotted path relative to that domain's `defaults`,
its complete replacement `value`, and a short `reason`. For example:

```yaml
schema_version: 1
policy: ui
scope: {application: example_quiet_logger}
overrides:
  - key: startup.enabled
    value: false
    reason: This deployment requires silent boot.
```

The ADR names an overlay's exact repository path if it uses one. For a single
target, prefer the ADR block directly and avoid creating an overlay:

```yaml
# Under the ADR heading "Policy overrides".
policy_overrides:
  - policy: audio
    key: recording_seconds
    value: 60
    reason: Minute-long files are the test's measurement unit.
```

Apply default, selected overlay, then ADR entries in that order. Each entry
replaces the named value; siblings remain inherited. Lists are replaced whole,
never appended or merged by index. No wildcards, key deletion, aliases, or
implicit null/unset semantics. Unknown keys, duplicate unconditional entries at one level,
incompatible value types, and overlapping parent/child replacements at one
level are errors. To disable behaviour, use its explicit `enabled` key where
provided. A value cannot violate its unit/range or hardware constraints.

An ADR may add `applies_when` to an entry for a named build feature or runtime
mode it defines. Otherwise the entry applies throughout that ADR's target.
Conditions must be unambiguous and mutually exclusive for the same key. This
is documentation, not a general condition language.

When writing an ADR, read the relevant policy files, write only target-specific
decisions, and put every deviation in one `Policy overrides` block with a reason.
Reference the policy for acceptance checks instead of copying its values.
Changing a common default means editing its policy once and reviewing affected
targets and their explicit exceptions; it does not mean adding another layer.
