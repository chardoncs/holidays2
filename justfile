gen:
    uv run ./gen.py
    cargo fmt

# Regenerate CHANGELOG.md from the whole history with git-cliff.
# Run it before cutting a release tag; the release workflow renders the tagged
# version's section for the GitHub release notes from the same cliff.toml.
changelog:
    git cliff -o CHANGELOG.md
