# deploy-test

<p align="center">
  <img src="https://jamiemason.github.io/deploy-test/logo.svg" width="134" height="120" alt="">
  <br>Consistent dependency versions in large JavaScript Monorepos.
  <br><a href="https://jamiemason.github.io/deploy-test">https://jamiemason.github.io/deploy-test</a>
</p>

> [!NOTE]
> This is the README for v14-alpha, a Rust rewrite which is due to replace [`v13.x.x`](https://github.com/JamieMason/deploy-test/tree/13.x.x?tab=readme-ov-file#deploy-test)

## Installation

```bash
npm install --save-dev deploy-test@alpha
```

## Guides

- [Getting Started](https://jamiemason.github.io/deploy-test/)
- [Migrate to 14](https://jamiemason.github.io/deploy-test/guide/migrate-v14/)

## Commands

> All command line options can be combined to target packages and dependencies in multiple ways.

### [lint](https://jamiemason.github.io/deploy-test/command/lint)

Ensure that multiple packages requiring the same dependency define the same version, so that every package requires eg. `react@17.0.2`, instead of a combination of `react@17.0.2`, `react@16.8.3`, and `react@16.14.0`.

#### Examples

```bash
# Find all issues in "dependencies" or "devDependencies"
deploy-test lint --dependency-types prod,dev
# Only lint issues in "react" specifically
deploy-test lint --dependencies react
# Look for issues in dependencies containing "react" in the name
deploy-test lint --dependencies '**react**'
# Find issues in scoped packages only
deploy-test lint --dependencies '@types/**'
# Find issues everywhere except "peerDependencies"
deploy-test lint --dependency-types '!peer'
# Only look for issues where an exact version is used (eg "1.2.3")
deploy-test lint --specifier-types exact
# Sort dependencies by how many times they are used
deploy-test lint --sort count
# See more examples
deploy-test lint --help
# See a short summary of options
deploy-test lint -h
```

### [fix](https://jamiemason.github.io/deploy-test/command/fix)

Fix every autofixable issue found by `deploy-test lint`.

#### Examples

```bash
# Only fix issues in dependencies and devDependencies
deploy-test fix --dependency-types prod,dev
# Only fix inconsistencies with exact versions (eg "1.2.3")
deploy-test fix --specifier-types exact
# Only fix issues in "react" specifically
deploy-test fix --dependencies react
# See more examples
deploy-test fix --help
# See a short summary of options
deploy-test fix -h
```

### [update](https://jamiemason.github.io/deploy-test/command/update)

Update packages to the latest versions from the npm registry, wherever they are in your monorepo.<br/>Semver range preferences are preserved when updating.

#### Examples

```bash
# Accept any update in latest (x.x.x)
deploy-test update --target latest
# Only update minor versions (1.x.x)
deploy-test update --target minor
# Only update patch versions (1.2.x)
deploy-test update --target patch
# Check for outdated dependencies in one package
deploy-test update --check --source 'packages/pingu/package.json'
# Update dependencies and devDependencies in the whole monorepo
deploy-test update --dependency-types dev,prod
# Only update dependencies with a semver range specifier (^, ~, etc.)
deploy-test update --specifier-types range
# Update dependencies where name exactly matches 'react'
deploy-test update --dependencies 'react'
# Update dependencies where name contains 'react'
deploy-test update --dependencies '**react**'
# Update dependencies with the '@aws-sdk' scope
deploy-test update --dependencies '@aws-sdk/**'
# See more examples
deploy-test update --help
# See a short summary of options
deploy-test update -h
```

### [format](https://jamiemason.github.io/deploy-test/command/format)

Organise package.json files according to a conventional format, where fields appear in a predictable order and nested fields are ordered alphabetically. Shorthand properties are used where available, such as the `"repository"` and `"bugs"` fields.

#### Examples

```bash
# Fix every formatting issue in the monorepo
deploy-test format
# List all formatting issues in the monorepo
deploy-test format --check
# Check the formatting of one package
deploy-test format --check --source 'packages/pingu/package.json'
# See more examples
deploy-test format --help
# See a short summary of options
deploy-test format -h
```

### [list](https://jamiemason.github.io/deploy-test/command/list)

Query and inspect all dependencies in your project, both valid and invalid.

#### Examples

```bash
# Sort dependencies by how many times they are used
deploy-test list --sort count
# Show every instance of each dependency, not just their names
deploy-test list --show instances
# Show dependencies ignored in your deploy-test config
deploy-test list --show ignored
# Show highest level of detail
deploy-test list --show all
# Choose only some values
deploy-test list --show hints,statuses
# List all "peerDependencies"
deploy-test list --dependency-types peer
# List all types packages
deploy-test list --dependencies '@types/**'
# List instances of an exact version being used as a peer dependency
deploy-test list --specifier-types exact --show instances --dependency-types peer
# See more examples
deploy-test list --help
# See a short summary of options
deploy-test list -h
```

### [json](https://jamiemason.github.io/deploy-test/command/json)

Output the state of every instance of every dependency as a JSON object, one per line. This command is best used with tools like [`jq`](https://jqlang.org/) for filtering and processing.

#### Examples

```bash
# Output all dependencies as JSON
deploy-test json
# Output only AWS SDK dependencies
deploy-test json --dependencies '@aws-sdk/**'
# Count dependencies by type
deploy-test json | jq -r '.dependencyType' | sort | uniq -c
# See more examples
deploy-test json --help
# See a short summary of options
deploy-test json -h
```

## Badges

- [![support on ko-fi](https://ko-fi.com/img/githubbutton_sm.svg)](https://ko-fi.com/C0C4PY4P)
- [![NPM version](http://img.shields.io/npm/v/deploy-test.svg?style=flat-square)](https://www.npmjs.com/package/deploy-test)
- [![NPM downloads](http://img.shields.io/npm/dm/deploy-test.svg?style=flat-square)](https://www.npmjs.com/package/deploy-test)
- [![Build Status](https://img.shields.io/github/actions/workflow/status/JamieMason/deploy-test/ci.yaml?branch=main)](https://github.com/JamieMason/deploy-test/actions)
