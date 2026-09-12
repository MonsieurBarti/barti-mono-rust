# What versions do we pin for oasdiff and lychee?

Researched 2026-09-12. Ticket: [What versions do we pin for oasdiff and lychee?](../issues/01-oasdiff-lychee-pins.md)

## Verdict

Pin the latest stable GitHub release of each tool. Both actions currently ship or default the matching CLI.

| Tool | Pin | GitHub tag | Published (UTC) | Action binary |
| --- | --- | --- | --- | --- |
| oasdiff | 1.31.0 | `v1.31.0` | 2026-09-05 | — |
| oasdiff-action | v0.1.15 | `v0.1.15` | 2026-09-05 | oasdiff `v1.31.0` |
| lychee | 0.24.2 | `lychee-v0.24.2` | 2026-05-01 | — |
| lychee-action | v2.9.0 | `v2.9.0` | 2026-07-09 | lychee `v0.24.2` |

ADR 0001 takes those four rows. CI takes the same tags. Do not pin moving majors (`@v0`, `@v2`). Do not pin lychee `nightly`.

The actions match the CLI latests. oasdiff-action `v0.1.15` hardcodes `tufin/oasdiff:v1.31.0`. lychee-action `v2.9.0` defaults `lycheeVersion` to `v0.24.2`, which downloads tag `lychee-v0.24.2`.

## Pins

### oasdiff `v1.31.0`

GitHub marks `v1.31.0` as the latest non-prerelease. `published_at` is `2026-09-05T07:19:58Z`. `prerelease` is false. Main has later commits. Those commits are not a release.

ADR 0001 records `1.31.0`. The git tag is `v1.31.0`.

### oasdiff-action `v0.1.15`

GitHub marks `v0.1.15` as the latest non-prerelease. `published_at` is `2026-09-05T07:44:47Z`. The release body says every sub-action now runs oasdiff `v1.31.0`. The `breaking` and `validate` Dockerfiles on that tag are `FROM tufin/oasdiff:v1.31.0`. The action has no binary-version input. Bumping oasdiff without bumping the action is not possible through the action.

A floating `v0` tag exists. The README points examples at `@v0`. ADR 0001 pins exact versions. CI uses `@v0.1.15`.

### lychee `lychee-v0.24.2`

GitHub `/releases/latest` returns `lychee-v0.24.2`. `published_at` is `2026-05-01T15:41:38Z`. `prerelease` is false.

The releases list also carries `nightly`. That release is a prerelease (`prerelease: true`, published 2026-09-08). Skip it.

`lychee-lib-v0.24.2` is the library crate tag. Skip it.

ADR 0001 records `0.24.2`. The git tag is `lychee-v0.24.2`.

### lychee-action `v2.9.0`

GitHub marks `v2.9.0` as the latest non-prerelease. `published_at` is `2026-07-09T15:31:00Z`. The release body says the default lychee version moved from `v0.23.0` to `v0.24.2`. `action.yml` on that tag sets `lycheeVersion` default `v0.24.2`. The install step downloads `https://github.com/lycheeverse/lychee/releases/download/lychee-${LYCHEE_VERSION}/…`, so `v0.24.2` hits tag `lychee-v0.24.2`.

A floating `v2` tag exists. README examples use `@v2`. The README security section tells callers to pin a fixed version. ADR 0001 pins `@v2.9.0`.

## How CI invokes each

CI uses the GitHub Actions. It does not install the CLIs as extra binaries. Both project READMEs name the Action as the CI path. `taiki-e/install-action` is how this repo pins `cargo-deny` and friends. It is not how these two tools document CI.

### oasdiff

The CLI README says to run it locally, and to run it in CI via the GitHub Action. Local install is `brew`, `go install`, `install.sh`, or a release binary. CI does not use those.

### oasdiff-action

The action README’s quick start is a `uses:` of a sub-action after checkout and a shallow fetch of the base ref. Hive needs two of those sub-actions, matching the spec:

```yaml
- uses: oasdiff/oasdiff-action/validate@v0.1.15
  with:
    spec: docs/openapi/openapi.json
- run: git fetch --depth=1 origin ${{ github.base_ref }}
- uses: oasdiff/oasdiff-action/breaking@v0.1.15
  with:
    base: origin/${{ github.base_ref }}:docs/openapi/openapi.json
    revision: HEAD:docs/openapi/openapi.json
    fail-on: ERR
    review: false
```

`review` defaults to `true` and uploads the specs. The spec sets `review: false`. `fail-on` in the README example is `WARN`. The spec sets `ERR`. Skip the breaking step when the path is missing on the base ref.

### lychee

The CLI README’s GitHub Actions section points at `lycheeverse/lychee-action`. Local install is brew, cargo, or a release binary. CI does not cargo-install.

`--offline` “Only check local files and block network requests”. `--include-fragments` checks anchor fragments. The spec uses both.

### lychee-action

The action README’s documented usage is `uses: lycheeverse/lychee-action@v2` with an `args` block. Hive pins the exact tag. Default `fail` is `true`. Default `lycheeVersion` already equals the CLI pin, so the workflow may omit it. Set `lycheeVersion: v0.24.2` if the workflow should show the binary pin next to the action pin.

```yaml
- uses: lycheeverse/lychee-action@v2.9.0
  with:
    args: --offline --include-fragments --no-progress docs/handbook docs/architecture.md docs/adr CONTEXT.md CONTEXT-MAP.md crates/cells
    fail: true
```

Default `args` scan every `*.md`, `*.html`, and `*.rst`. The spec names a closed set and excludes `.scratch/` and `AGENTS.md`. Pass that set through `args`.

## Fit to hive

[ADR 0001](../../../docs/adr/0001-stack-pins.md) already pins tools as exact versions (`cargo-deny` 0.20.2, `cargo-nextest` 0.9.144). These four rows follow that shape.

The spec’s ADR 0001 table already has the four tool rows as “implementation pin”. This note fills them.

[Prior CI research](../../hive-docs/research/04-ci-documentation-gates.md) named the same four tags on 2026-09-12. They are still the latest stables.

## Sources

Fetched 2026-09-12.

- [oasdiff latest release API](https://api.github.com/repos/oasdiff/oasdiff/releases/latest) — tag `v1.31.0`, `published_at` 2026-09-05T07:19:58Z, `prerelease` false.
- [oasdiff v1.31.0 release](https://github.com/oasdiff/oasdiff/releases/tag/v1.31.0) — Latest badge.
- [oasdiff README](https://github.com/oasdiff/oasdiff) — “Run it locally, in CI via the GitHub Action”.
- [oasdiff-action latest release API](https://api.github.com/repos/oasdiff/oasdiff-action/releases/latest) — tag `v0.1.15`, `published_at` 2026-09-05T07:44:47Z. Body: upgraded to oasdiff v1.31.0.
- [oasdiff-action README](https://github.com/oasdiff/oasdiff-action/blob/v0.1.15/README.md) — `@v0` moving tag, pin a release tag to control upgrades, `breaking` / `validate` inputs, `review` default true, `git fetch --depth=1 origin ${{ github.base_ref }}`.
- [oasdiff-action breaking Dockerfile @ v0.1.15](https://github.com/oasdiff/oasdiff-action/blob/v0.1.15/breaking/Dockerfile) — `FROM tufin/oasdiff:v1.31.0`.
- [oasdiff-action validate Dockerfile @ v0.1.15](https://github.com/oasdiff/oasdiff-action/blob/v0.1.15/validate/Dockerfile) — `FROM tufin/oasdiff:v1.31.0`.
- [lychee latest release API](https://api.github.com/repos/lycheeverse/lychee/releases/latest) — tag `lychee-v0.24.2`, `published_at` 2026-05-01T15:41:38Z, `prerelease` false.
- [lychee-v0.24.2 release](https://github.com/lycheeverse/lychee/releases/tag/lychee-v0.24.2) — Latest badge.
- [lychee releases list](https://api.github.com/repos/lycheeverse/lychee/releases?per_page=5) — `nightly` is `prerelease: true`.
- [lychee README](https://github.com/lycheeverse/lychee/blob/master/README.md) — GitHub Actions section names lychee-action. `--offline`. `--include-fragments`.
- [lychee-action latest release API](https://api.github.com/repos/lycheeverse/lychee-action/releases/latest) — tag `v2.9.0`, `published_at` 2026-07-09T15:31:00Z. Body: default lychee `v0.24.2`.
- [lychee-action README](https://github.com/lycheeverse/lychee-action/blob/v2.9.0/README.md) — `uses: lycheeverse/lychee-action@v2`, `lycheeVersion`, pin a fixed version.
- [lychee-action action.yml @ v2.9.0](https://github.com/lycheeverse/lychee-action/blob/v2.9.0/action.yml) — default `lycheeVersion: v0.24.2`, default `fail: true`, download URL `lychee-${LYCHEE_VERSION}`.
- [hive-docs spec](../../hive-docs/spec.md) — ADR 0001 rows, `oasdiff validate`, `oasdiff breaking` with `fail-on: ERR` and `review: false`, `lychee --offline --include-fragments`.
