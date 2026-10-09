# Tauri Plugin Text Scale

[![CI][ci-badge]][ci-url]

Reads the operating system text size setting in Tauri 2.x apps and reports changes to
it.

The plugin reports the text size as a scale. The scale is `1` at the platform's default
text size. A larger text size gives a scale above `1`, and a smaller text size gives a
scale below `1`. An app multiplies its own text sizes by the scale, for example through
a CSS custom property.

[ci-badge]: https://github.com/silvermine/tauri-plugin-text-scale/actions/workflows/ci.yml/badge.svg
[ci-url]: https://github.com/silvermine/tauri-plugin-text-scale/actions/workflows/ci.yml

## Features

   * Read the current text scale with one command.
   * Receive each change through one Tauri event, from each platform that reads a text
     size.
   * Build and run on every Tauri target. A platform that has no text size, or that the
     plugin does not support yet, returns `1` and never emits the event.

| Platform | Source of the scale                                            |
| -------- | -------------------------------------------------------------- |
| Windows  | Planned: `UISettings.TextScaleFactor`. Returns `1` until then  |
| Linux    | `1`. A later version can read the GTK text scale               |
| macOS    | `1`. macOS has no public API for its text size                 |
| Android  | `Configuration.fontScale`                                      |
| iOS      | Planned: `UIFontMetrics` for body text. Returns `1` until then |

## Architecture

The plugin is one crate. Unlike `tauri-plugin-connectivity`, it has no
Tauri-independent crate. Each platform implementation reads one operating system value,
such as `Configuration.fontScale`, and needs the Tauri runtime to emit the change event.

A platform that reads a text size emits the `tauri-plugin-text-scale:changed` event from
Rust to every window. The frontend listens for the same event on every platform, and a
window that opens later receives the same changes.

## Getting Started

### Installation

1. Install the npm dependencies:

   ```bash
   npm install
   ```

2. Build the TypeScript bindings:

   ```bash
   npm run build
   ```

3. Build the Rust plugin:

   ```bash
   cargo build
   ```

### Tests

Run all tests (TypeScript and Rust):

```bash
npm test
```

Run the TypeScript tests only:

```bash
npm run test:ts
```

Run the Rust tests only:

```bash
cargo test --workspace --lib
```

Run the Kotlin tests for the Android text scale logic (requires JDK 17 and the Android
SDK):

```bash
cd android && ./gradlew :lib:test
```

The Kotlin tests cover the logic in `android/lib`, which does not depend on the Tauri
Android API, so the tests run without an emulator or a Tauri app build.

### Manual Android testing

See [Android Text Scale Manual Testing](docs/android-text-scale-manual-testing.md) for
the device setup and the scenarios: the first read, a change while the app runs,
rotation, and the WebView text zoom.

### Example app

The [example app](examples/tauri-app/README.md) shows the current scale and lists each
change that the plugin reports.

## Install

_This plugin requires a Rust version of at least **1.94.0**_

### Rust

Add the plugin to your `Cargo.toml`. Pin it to a commit with `rev`:

`src-tauri/Cargo.toml`

```toml
[dependencies]
tauri-plugin-text-scale = { git = "https://github.com/silvermine/tauri-plugin-text-scale", rev = "<sha>" }
```

### JavaScript/TypeScript

The package is not on the npm registry. Install the JavaScript bindings from GitHub, at
the same commit as the Rust crate:

```sh
npm install github:silvermine/tauri-plugin-text-scale#<sha>
```

## Usage

### Prerequisites

Initialize the plugin in your `tauri::Builder`:

```rust
fn main() {
   tauri::Builder::default()
      .plugin(tauri_plugin_text_scale::init())
      .run(tauri::generate_context!())
      .expect("error while running tauri application");
}
```

Add the default permission to a capability for each window that reads the scale:

`src-tauri/capabilities/default.json`

```json
{
   "identifier": "default",
   "windows": ["main"],
   "permissions": [
      "core:default",
      "text-scale:default"
   ]
}
```

`text-scale:default` allows the `get_text_scale` command. The change event needs no
permission of its own, because `core:default` already allows listening to events.

#### Android

Add `fontScale` to the `android:configChanges` attribute of the app's main activity, in
`src-tauri/gen/android/app/src/main/AndroidManifest.xml`. The Tauri template does not
list it:

```xml
<activity
   android:configChanges="orientation|keyboardHidden|keyboard|screenSize|locale|smallestScreenSize|screenLayout|uiMode|fontScale"
   ...>
```

With `fontScale` listed, Android reports a text size change to the running activity,
and the plugin emits the change event. Without it, Android restarts the activity
instead, and the plugin emits no event.

The Android WebView multiplies all text by the font scale on its own. The plugin sets
the WebView's `textZoom` to `100` to turn that off, so that the app applies the scale
once, where it chooses.

### API

#### Read the text scale

```ts
import { getTextScale } from '@silvermine/tauri-plugin-text-scale';

async function applyTextScale(): Promise<void> {
   const scale = await getTextScale();

   document.documentElement.style.setProperty('--os-text-scale', String(scale));
}
```

If the platform fails to report its text size, `getTextScale()` rejects with a string
error.

#### Follow changes to the text scale

```ts
import { onTextScaleChanged } from '@silvermine/tauri-plugin-text-scale';

const unlisten = await onTextScaleChanged((scale) => {
   document.documentElement.style.setProperty('--os-text-scale', String(scale));
});

// Later, to stop listening:
unlisten();
```

#### Combine the first read with the changes

Subscribe before the first read, so that the listener receives a change that arrives
between the two calls.
A change event can also arrive while the read waits for its reply. The event is then
newer than the reply, so keep the event's value:

```ts
import { getTextScale, onTextScaleChanged } from '@silvermine/tauri-plugin-text-scale';

function setScale(scale: number): void {
   document.documentElement.style.setProperty('--os-text-scale', String(scale));
}

async function followTextScale(): Promise<void> {
   let changed = false;

   await onTextScaleChanged((scale) => {
      changed = true;
      setScale(scale);
   });

   const scale = await getTextScale();

   if (!changed) {
      setScale(scale);
   }
}
```

#### Use the event name

Apps that keep their own manifest of backend events can import the event name instead of
typing it again:

```ts
import { TEXT_SCALE_CHANGED_EVENT } from '@silvermine/tauri-plugin-text-scale';
```

The payload of the event is the new scale as a number.

#### Use from Rust

Read the scale from any Tauri manager type through the extension trait:

```rust
use tauri_plugin_text_scale::TextScaleExt;

fn log_text_scale<R: tauri::Runtime>(app: &tauri::AppHandle<R>) {
   match app.text_scale().scale() {
      Ok(scale) => println!("Text scale: {scale}"),
      Err(error) => eprintln!("Could not read the text scale: {error}"),
   }
}
```

## Development Standards

This project follows the
[Silvermine standardization](https://github.com/silvermine/standardization)
guidelines. Key standards include:

   * **Markdownlint**: Markdown linting for documentation
   * **Commitlint**: Conventional commit message format
   * **Code Style**: 3-space indentation, LF line endings

### Running Standards Checks

```bash
npm run standards
```

## License

MIT

## Contributing

Contributions are welcome! Please follow the established coding standards and commit
message conventions.
