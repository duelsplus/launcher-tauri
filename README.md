# Duels+ Launcher v3

The current, officially supported desktop launcher for Duels+. It replaces the [legacy launcher](https://github.com/duelsplus/launcher-legacy) and is the primary entry point for most users.

For users who prefer working entirely in the terminal, the [Duels+ CLI](https://github.com/duelsplus/cli) may appeal more.

## Installation

Prebuilt binaries are available for all supported platforms.

Download the release appropriate for your operating system and architecture from the [releases](https://github.com/duelsplus/launcher-tauri/releases) page.

## Usage

After installation, find the "Duels+ Launcher" shortcut and launch.

If you prefer launching Duels+ from the terminal, see the [Duels+ CLI](https://github.com/duelsplus/cli) instead.

## Development

### Requirements

- [Bun](https://bun.com/docs/installation)
- [Rust](https://rust-lang.org/learn/get-started/)

### Clone and install dependencies

```bash
git clone https://github.com/duelsplus/launcher-tauri.git
cd launcher-tauri
bun i
```

### Run development server

```bash
bun run tauri dev
```

This will start the Vite dev server (with hot reload) and open the launcher in a window.

The first run may take a while because Cargo compiles the Rust backend; subsequent launches are significantly faster.

> [!IMPORTANT]
> If you're getting the error `target/debug/launcher-tauri: error while loading shared libraries: libwebkit2gtk-4.1.so.0: cannot open shared object file: No such file or directory` on Arch Linux, you must install the `webkit2gtk-4.1` package by running `sudo pacman -S webkit2gtk-4.1`.

### Build binaries

```bash
bun run tauri build -c '{"bundle": { "createUpdaterArtifacts": false }}'
```

Unless you've reconfigured the updater plugin so it points to your server, generated your own public/private keys, and set the `TAURI_SIGNING_PRIVATE_KEY` env var to the contents or path of your private key... or you somehow managed to retrieve our private key (which we absolutely hope not), you must keep the `-c '{"bundle": { "createUpdaterArtifacts": false }}'` parameter otherwise the build will fail. Alternatively, you can set the flag permanently in `src-tauri/tauri.conf.json` so you don't have to append it when building every time. Just make sure __not__ to commit such a change in a PR.

Binaries can be found in `src-tauri/target/release/bundle`.

## Contributing

Contributions are welcome.

If you are fixing a bug or making a minor improvement, feel free to open a pull request directly. For major or breaking changes, join our [Discord server](https://discord.gg/YD4JZnuGYv) and open a support ticket first to discuss scope and direction. Thank you :)

## License
This project is licensed under the MIT License. See [LICENSE](LICENSE).
