# Nushell <!-- omit in toc -->

> A personal fork of Nushell, tuned for my own workflow.

This repository is a working fork of [Nushell](https://github.com/nushell/nushell). It is based on upstream Nushell, but it is being adapted for my own daily environment, configuration style, and helper commands.

The goal of this fork is simple:

* keep the core Nushell experience intact
* add small quality-of-life helpers for my shell workflow
* make my environment reproducible and easy to maintain
* stay close enough to upstream that pulling changes remains practical

## What this fork is for

This is not a rebrand of Nushell and not a separate shell project. It is a personal working copy that I use to experiment with configuration, local conveniences, and workflow-specific additions before or alongside upstream updates.

A few examples of the kind of changes that belong here:

* custom helper commands in the default config
* small workflow shortcuts for local use
* personal defaults and startup behavior adjustments
* experiments that may later become upstream PRs

## What still comes from upstream

The core Nushell architecture, language, and command system still come from upstream. This fork should stay close to the original project so that merging upstream changes stays manageable.

If a change is generally useful, it may later be turned into a proper upstream contribution instead of staying only in this fork.

## Learning about Nushell

The main Nushell documentation lives in the [Nushell book](https://www.nushell.sh/book/). It is still the best place to learn the language, built-in commands, configuration model, and pipeline behavior.

## Installation

For the upstream project, see the [official installation guide](https://www.nushell.sh/book/installation.html).

For this fork, the usual workflow is to build from source and run the binary from `target/release` or `target/debug`:

```powershell
cargo build --release
.\target\release\nu.exe
```

## Configuration

This fork may include custom startup helpers and personal defaults in the shipped config files under:

```text
crates/nu-utils/src/default_files/
```

That is where I keep shell-level helpers that should be available by default when this fork starts.

To inspect the active config path in Nushell, run:

```nu
$nu.config-path
```

## Examples of local additions

A typical example is a helper like `yt`, which opens a YouTube search in the system browser:

```nu
def yt [...query: string] {
    let search_text = ($query | str join " ")
    if ($search_text | is-empty) { return }

    let url_data = {
        scheme: "https"
        host: "youtube.com"
        path: "/results"
        params: { search_query: $search_text }
    }

    start ($url_data | url join)
}
```

Small commands like this are meant to reduce friction in my everyday shell usage.

## Philosophy

Nushell treats data as structured rather than as plain text whenever possible. That is the main reason I like it: it makes shell workflows more predictable, composable, and readable.

I keep that philosophy in this fork, while tailoring the environment around how I actually use the shell.

## Upstream sync

I plan to keep pulling changes from upstream Nushell regularly.

The long-term idea is:

* keep this fork usable as a personal shell environment
* avoid drifting too far from upstream
* make local changes small and easy to audit
* upstream anything that is broadly useful

## Contributing

This fork is primarily for personal use, but upstream-quality fixes and improvements are still welcome in the form of local experiments and later PRs.

For the upstream contribution guide, see [CONTRIBUTING.md](CONTRIBUTING.md).

## License

This project remains under the MIT license. See [LICENSE](LICENSE) for details.
