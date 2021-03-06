# WBM-installer

WBM has been officially included into the game

<details>
<summary>Potentially outdated information (click to unfold)</summary>

<p align="center">
  <b><a alt="download link" href="https://github.com/War-Brokers-Mods/WBM-installer/releases">Download</a></b>
  <br />
  <br />
  <img alt="Installer image" src="./.repo/image.png" />
</p>

## Contributing

<details>
  <summary>Click to unfold</summary>

**⚠️ High CPU usage warning ⚠️**

Running `yarn tauri XXX` command will use a LOT (I mean A **LOT**) of computing resource.
No worries though, this is only for the first execution and it will be much quicker next time.

I highly recommend reading the [tauri documentation](https://tauri.studio) before writing any code.

### 0. Requirements

- nodejs v16 LTS
- cargo
- yarn
- git

### 1. Setup

- [Setup tauri](https://tauri.studio/en/docs/getting-started/intro/#setting-up-your-environment)
- install dependencies

  ```bash
  yarn install
  ```

### 2. Run in development mode

Start the application in development environment with testing tools and hot reloading.

```bash
yarn tauri dev
```

- This command should be used for testing since the app may render differently in your browser.
- You might get a `Port 8080 is taken` warning because the previous dev server isn't properly closed.
  Exiting and reopening the terminal should fix the issue.

### 3. Build for production

Build the application for release.

```bash
yarn tauri build
```

</details>

## License

The source code for this project is available under the [MIT license](./LICENSE).

</details>
