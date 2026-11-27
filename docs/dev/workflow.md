# Development workflow

## Repositories

- **Upstream:** `github.com/TriBinary/aktis`, the main repository. Releases are tagged here.
- **Fork:** each contributor's fork; all work is pushed here and merged through pull requests.

```sh
git clone https://github.com/<you>/aktis.git
cd aktis
git remote add upstream https://github.com/TriBinary/aktis.git
git fetch upstream
git switch -c feat/my-change upstream/main
```

## Making a change

1. Branch from `upstream/main` (`feat/…`, `fix/…`, `docs/…`).
2. Change code **and** docs (see the table in `CLAUDE.md` §4), and add a `CHANGELOG.md` entry.
3. Run fmt, clippy and tests.
4. Commit with a Conventional Commit message (see below), push to your fork, open a PR against
   `TriBinary/aktis:main`.
5. CI must pass, including the docs check.

## Working with AI agents

- The agent asks clarifying questions first; answer them, or reply "defaults".
- The agent never commits. Review its diff (`git diff`), then commit using the message(s) it
  proposes, edited as needed.

## Commit messages

Conventional Commits: `<type>(<scope>): <summary>`.
- Types: `feat fix perf refactor docs test build ci chore`
- Scopes: `assets world scene render pt sp app docs ci deps`

Examples:
```
feat(assets): resolve multipart blockstates
fix(world): handle sections without a block palette
docs(sp): add BSL compatibility notes
```

## Branch protection (recommended on upstream)

- Require PRs and passing CI on `main`.
- Squash or rebase merges, to keep a linear history.
