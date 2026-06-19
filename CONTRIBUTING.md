# How to Contribute

We'd love to accept your patches and contributions to this project. There are
just a few small guidelines you need to follow.

## Code Reviews

All submissions, including submissions by project members, require review. We
use GitHub pull requests for this purpose. Consult
[GitHub Help](https://help.github.com/articles/about-pull-requests/) for more
information on using pull requests.

## Setting up commit hooks

This repository includes a pre-commit hook that runs the formatter and linter
automatically. Activate it once after cloning:

```
git config core.hooksPath .githooks
```

The hook runs:
- `cargo +nightly fmt -- --check` — fails if any file needs reformatting
- `cargo +nightly clippy -- -D warnings` — fails on any Clippy warning

## Running the autoformatter rustfmt

This repository uses a custom configuration for rustfmt which currently requires
that one run the *nightly* version:

```
cargo +nightly fmt
```

The stable version will generate error messages and modify a lot of the
existing formatting, obscuring any real changes you are making.
