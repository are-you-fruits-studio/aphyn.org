# aphyn.org

The website of [Aphyn](https://github.com/are-you-fruits-studio/aphyn), a 3D game engine in Rust by are-you-fruits-studio. It is a [Yew](https://yew.rs) app compiled to WebAssembly and served by GitHub Pages at <https://aphyn.org>.

## Develop

You need Rust stable, the WebAssembly target and [Trunk](https://trunkrs.dev):

```bash
rustup target add wasm32-unknown-unknown
```

```bash
cargo install trunk --locked
```

Run it locally with live reload at <http://127.0.0.1:8080>:

```bash
trunk serve
```

Tests (translations and the code highlighter) and lints:

```bash
cargo test
```

```bash
cargo clippy --target wasm32-unknown-unknown -- -D warnings
```

## Layout

| Path | What it is |
|---|---|
| `src/pages/` | Home, Docs, Examples, Roadmap, Releases, 404 |
| `src/components/` | Header, footer and the Aphyn design system components (logo, badge, card, tabs, code block, callout) |
| `src/i18n.rs` | Language registry and the translation lookup |
| `src/route.rs` | Routes: the default language at `/`, the others under `/<code>/` |
| `src/releases.rs` | Engine version and recent releases |
| `locales/` | One text file per language |
| `assets/ds/` | Design system tokens and fonts (Onest, JetBrains Mono) |
| `assets/site.css` | Site layout and component styles |
| `scripts/static-routes.sh` | Copies `index.html` for every page so deep links answer 200 |

## Languages

Every language is a file in `locales/` with one `key = value` per line (`\n` for a line break). English (`en.txt`) is the default and the fallback: a key missing from another language shows the English text, and `cargo test` lists the missing keys.

To add a language, for example Polish:

1. Copy `locales/en.txt` to `locales/pl.txt` and translate the values.
2. Add one line to `LOCALES` in `src/i18n.rs`:
   ```rust
   Locale { code: "pl", label: "PL", name: "Polski", source: include_str!("../locales/pl.txt") },
   ```

The routes (`/pl`, `/pl/examples`), the language switch, page titles and the static pages for GitHub Pages pick it up from there.

## Engine version and releases

The deploy workflow reads the latest releases of the engine repository through the GitHub API, builds the site with the newest tag as `APHYN_VERSION` and publishes the list as `/releases.json`. The home page shows that version (badge and the `Cargo.toml` example) and the roadmap lists the recent releases. Visitors only load `/releases.json` from aphyn.org; they never call the GitHub API.

The site rebuilds every day, on every push to `main` and on demand (Actions → Build and deploy → Run workflow). The engine repository can also trigger a rebuild right after a release with a `repository_dispatch` event of type `aphyn-release`.

## Deploy

`.github/workflows/deploy.yml` tests, builds and publishes the site to GitHub Pages on every push to `main`; pull requests are built and tested but not published. Repository settings it relies on:

- Settings → Pages → Build and deployment → Source: **GitHub Actions**.
- Settings → Pages → Custom domain: `aphyn.org`, with **Enforce HTTPS** on. The `CNAME` file in this repository is copied into every build.

## License

The site's code is licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or <https://www.apache.org/licenses/LICENSE-2.0>)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or <https://opensource.org/licenses/MIT>)

at your option, like the engine itself.

Third-party material shipped with the site:

- Fonts: Onest and JetBrains Mono, SIL Open Font License 1.1 — `assets/ds/fonts/OFL.txt`.
- Icons: [Lucide](https://lucide.dev), ISC — `assets/licenses/lucide-LICENSE.txt`.
- `assets/img/showcase.gif`: a recording of the engine's `showcase` example, taken from the [Aphyn repository](https://github.com/are-you-fruits-studio/aphyn) (`assets/readme/showcase.gif`), © are-you-fruits-studio contributors, MIT OR Apache-2.0.
- Rust crates compiled into the site: CI lists them with their licenses at `/licenses/third-party.html` ([cargo-about](https://github.com/EmbarkStudios/cargo-about), configured in `about.toml` and `about.hbs`).

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
