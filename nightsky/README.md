# nightsky

The small Rust tool behind my profile. It draws everything on it as animated SVG: the banner, the project cards, the language card, the footer, and the contribution chart, where each week of the last year is a building and its lit windows are that week's contributions.

```sh
cargo run --release              # everything
cargo run --release -- skyline   # or just some of: banner footer cards skyline langs
```

It reads the GitHub API with `PROFILE_TOKEN` if set, otherwise `GITHUB_TOKEN`. Projects live in [projects.json](projects.json). The illustrations in `../assets/img` were generated locally with Flux.2 Klein in ComfyUI.

A GitHub Action runs it every night and commits whatever changed.
