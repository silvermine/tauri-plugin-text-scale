# Windows text scale manual testing

Manual Windows scenarios for the Rust code in `src/windows.rs`.

The plugin reads `UISettings.TextScaleFactor`. That is the value of `Settings >
Accessibility > Text size`, a slider from 100% to 225%, divided by 100. When the user
moves the slider, Windows raises `UISettings.TextScaleFactorChanged`. The plugin then
stores the new scale and emits `tauri-plugin-text-scale:changed`. The app reads the
stored scale with `get_text_scale`, so a read does not call into Windows.

Windows raises the event on a thread of its own. The plugin keeps its `UISettings` in
managed state, so the handler stays registered while the app runs.

No scenario below ran yet. The code was only compiled for `x86_64-pc-windows-msvc` and
`aarch64-pc-windows-msvc` on a Mac. Run the scenarios on a Windows 10 or Windows 11
device, and update the status column afterwards.

## Reference links

| Item | Link |
| ---- | ---- |
| Tauri prerequisites | <https://v2.tauri.app/start/prerequisites/> |
| `UISettings.TextScaleFactor` | <https://learn.microsoft.com/en-us/uwp/api/windows.ui.viewmanagement.uisettings.textscalefactor> |
| `UISettings.TextScaleFactorChanged` | <https://learn.microsoft.com/en-us/uwp/api/windows.ui.viewmanagement.uisettings.textscalefactorchanged> |
| Text scaling in Windows | <https://learn.microsoft.com/en-us/windows/apps/design/accessibility/text-scaling> |

## Scenario coverage

| Scenario | Status | Expected result |
| -------- | ------ | --------------- |
| Default text size at start | Not tested | `Current scale` shows `1` |
| Larger text size at start | Not tested | `Current scale` shows `1.5` at 150% |
| Change while the app runs | Not tested | One new entry in `Changes` with the new scale |
| Largest and smallest text size | Not tested | `2.25` at 225%, and `1` at 100% |
| Change with no new value | Not tested | No new entry in `Changes` |
| WebView text size | Not tested | Text without the scale keeps its size |
| Packaged as a full-trust MSIX | Not tested | The same results as the unpackaged app |

## Base test setup

Install the Windows requirements from the Tauri prerequisites: the Microsoft C++ Build
Tools and WebView2.

Build the plugin's JavaScript bindings in the repository root:

```powershell
npm install
npm run build
```

## Example app

Install the example's dependencies and start it:

```powershell
cd examples/tauri-app
npm install
npm run dev
```

## Change the text size

Open the Settings app:

```text
Settings > Accessibility > Text size
```

Move the slider, or use the `Text size` percentage, and select `Apply`. Windows applies
the new size to the whole system, so every open app changes with it.

Reset the text size to 100% after testing.

## Useful observation commands

The example app logs each change at the `debug` level, from the `tauri_plugin_text_scale`
target. To see the log, start the example app with a log filter in the same PowerShell
window:

```powershell
$env:RUST_LOG = "tauri_plugin_text_scale=debug"
npm run dev
```

## Manual scenarios

### Default text size at start

1. Set the text size to 100%.
2. Start the example app.

Expected result: `Current scale` shows `1`, and `Changes` shows `No changes yet`.

### Larger text size at start

1. Close the example app.
2. Set the text size to 150% and select `Apply`.
3. Start the example app.

Expected result: `Current scale` shows `1.5`.

### Change while the app runs

1. Start the example app at 100%.
2. Set the text size to 150% and select `Apply`.

Expected result: `Changes` shows one new entry with `1.5`, and `Current scale` shows the
same value. The sample line grows.

### Largest and smallest text size

1. Start the example app at 100%.
2. Set the text size to 225%, then back to 100%.

Expected result: `Changes` shows `2.25`, then `1`.

### Change with no new value

1. Start the example app at 150%.
2. Open `Settings > Accessibility > Text size`, and select `Apply` without a change.

Expected result: `Changes` shows no new entry.

### WebView text size

This scenario shows whether the WebView scales text by itself. The heading, the labels
and the list in the example app do not use the scale in their CSS.

1. Set the text size to 100% and take a screenshot of the example app.
2. Set the text size to 225% and take a second screenshot.
3. Compare the two screenshots.

Expected result: the heading `Tauri Plugin Text Scale` has the same size in both
screenshots. Only the sample line, `This line is 16px times the current scale.`, is
larger in the second screenshot.

If the heading also grows, the WebView applies the text size by itself, and an app that
applies the scale too scales its text twice. Report the result, because the README then
needs an app setup step.

### Packaged as a full-trust MSIX

An app in a full-trust MSIX package is a desktop app that runs with package identity. This
scenario shows that the plugin works with it too.

1. Package the example app as a full-trust MSIX, and install it.
2. Run the scenarios `Larger text size at start` and `Change while the app runs` with the
   installed app.

Expected result: the same results as the unpackaged app.
