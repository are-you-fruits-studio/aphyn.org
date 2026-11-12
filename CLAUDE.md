# CLAUDE.md

Rules for working in this repository. They apply to people and to AI agents alike.

aphyn.org is the website of the Aphyn game engine: a Yew app compiled to WebAssembly with Trunk and deployed to GitHub Pages by `.github/workflows/deploy.yml`. The README covers the layout, localisation and deployment.

**This repository is public.** Everything committed, including commit messages and PR descriptions, is visible to everyone.

## Commits

- Follow [Conventional Commits 1.0](https://www.conventionalcommits.org/en/v1.0.0/): `<type>(<optional scope>): <subject>`.
  - Types: `feat`, `fix`, `docs`, `style`, `refactor`, `perf`, `test`, `build`, `ci`, `chore`, `revert`.
  - Scopes (optional): `home`, `examples`, `roadmap`, `i18n`, `theme`, `ds`, `ci`, `deps`.
  - Subject: imperative mood, lower case, no trailing period, 72 characters at most. Example: `feat(i18n): add Polish`.
  - Breaking changes (URLs, removed pages) get a `!` after the type and a `BREAKING CHANGE:` footer.
- The body, when needed, says why the change is made, wrapped at 72 characters.
- **No co-author or AI attribution trailers.** Never add `Co-Authored-By:`, `Claude-Session:` or similar lines to commits in this repository.

## Pull requests

- **One PR, one commit.** Keep the branch to a single commit: amend it and push with `git push --force-with-lease` to your own branch. Never rewrite someone else's branch.
- The PR title is the commit subject, in the same Conventional Commits form.
- Never push to `main` directly; every change goes through a PR with green CI.
- The description says what changed, why, and how it was tested. Call out anything that needs a repository setting or a manual step.

## Code style

- Format with `cargo fmt` (default rustfmt settings) and keep `cargo clippy --target wasm32-unknown-unknown -- -D warnings` clean. CI runs `cargo fmt --check`, clippy, `cargo test` and a release build; run the same before pushing.
- Function components with `#[function_component]` and `Properties` structs. Keep components small; shared design system pieces live in `src/components/`, pages in `src/pages/`.
- No user-facing text in Rust code. Every string a visitor reads goes into `locales/en.txt` and its translations, read with `lang.t("key")`. Product names, code, commands and URLs are the exceptions. Keys are dotted and grouped by page (`home.hero.title`).
- Styling lives in `assets/site.css` and uses the design system tokens (`var(--text-1)`, `var(--accent)`, `var(--radius-card)` …) instead of raw colours or sizes. Both themes must work: check light and dark.
- Layouts must work from 320px wide; use the existing media query (759px) and `clamp()` for spacing.
- Accessibility: real `<a href>` and `<button>` elements, `aria-label` on icon-only controls, touch targets of at least 44px, text contrast of at least 4.5:1, and animations that respect `prefers-reduced-motion`.
- Keep the WebAssembly small: add dependencies only when they pay for themselves, and prefer `web-sys`/`js-sys` over heavy crates.
- Comments explain why, not what. Doc comments on public items.

## Copy and brand

- Write "Aphyn" with a capital A in text; the lowercase "aphyn" exists only in the logo lockup.
- Sentence case for headings, buttons and navigation. No emoji.
- State limits plainly; no invented numbers or "coming soon". Facts about the engine come from its repository (README, releases, `todos.md`).
- Ukrainian copy uses literary Ukrainian, without anglicisms or russisms where a Ukrainian word exists.

## Public repository hygiene

- Never commit secrets, tokens, `.env` files, personal data (emails, private names), local filesystem paths or links to private tools and documents.
- Third-party assets (fonts, icons, images) ship with their license file, and only when redistribution is allowed.
- Generated output (`target/`, `dist/`, `releases.json`) stays out of git; see `.gitignore`.
