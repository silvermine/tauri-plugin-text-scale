# Android Text Scale Manual Testing

Manual Android scenarios for the native Android code under `android/src/main/java`.

The plugin reads `Configuration.fontScale` from the activity. When the font scale
changes, Android calls the plugin's `onConfigurationChanged`. The plugin then sends the
new scale to Rust, which emits `tauri-plugin-text-scale:changed`. When the plugin
loads, it sets the WebView's `textZoom` to `100`, so that the WebView does not scale
text by itself.

## Reference Links

| Item | Link |
| ---- | ---- |
| Tauri Android prerequisites | <https://v2.tauri.app/start/prerequisites/> |
| Tauri mobile plugin development | <https://v2.tauri.app/develop/plugins/develop-mobile/> |
| `Configuration.fontScale` | <https://developer.android.com/reference/android/content/res/Configuration#fontScale> |
| Handle configuration changes | <https://developer.android.com/guide/topics/resources/runtime-changes> |
| `WebSettings.setTextZoom` | <https://developer.android.com/reference/android/webkit/WebSettings#setTextZoom(int)> |

## Scenario Coverage

| Scenario | Status | Expected result |
| -------- | ------ | --------------- |
| Default font size at start | Tested on an emulator | `Current scale` shows `1` |
| Larger font size at start | Tested on an emulator | `Current scale` shows the font scale, for example `1.3` |
| Change while the app runs | Tested on an emulator | One new entry in `Changes` with the new scale |
| Rotation | Tested on an emulator | No new entry in `Changes` |
| Smallest and largest font size | Tested on an emulator | The scale matches `font_scale`, for example `0.85` and `2` |
| WebView text zoom | Tested on an emulator | Text without the scale keeps its size at every font size |

The tests ran on 2026-10-09 on an Android 17 (API 37) emulator. No test ran on a
physical device yet.

## Base Test Setup

Install the Android requirements from the Tauri prerequisites: Android Studio, the
Android SDK, the NDK, JDK 17 and the Rust Android targets.

Build the plugin's JavaScript bindings in the repository root:

```sh
npm install
npm run build
```

Make sure that `adb` can see the test device:

```sh
adb devices
```

The device must show the `device` state:

```text
List of devices attached
90859562    device
```

## Example App

Install the example's dependencies and create its Android project:

```sh
cd examples/tauri-app
npm install
npm run tauri android init
```

The Tauri template does not list `fontScale` in the activity's `configChanges`. Without
it, Android restarts the activity on a font size change and the plugin reports no
change. Open `src-tauri/gen/android/app/src/main/AndroidManifest.xml` and add
`fontScale` to `android:configChanges` on the main activity:

```xml
android:configChanges="orientation|keyboardHidden|keyboard|screenSize|locale|smallestScreenSize|screenLayout|uiMode|fontScale"
```

Run the example app on the device:

```sh
npm run tauri android dev
```

## Change the Font Size

Use the Settings app, or set the value directly with `adb`.

In the Settings app, open:

```text
Settings > Display > Display size and text > Font size
```

The path to the setting can be different on each Android vendor.

To set the font scale with `adb`:

```sh
adb shell settings put system font_scale 1.3
adb shell settings get system font_scale
```

Reset the font scale after testing:

```sh
adb shell settings put system font_scale 1.0
```

## Useful Observation Commands

```sh
adb logcat -s RustStdoutStderr Tauri
```

The example app logs each change at the `debug` level, from the
`tauri_plugin_text_scale` target.

## Manual Scenarios

### Default Font Size at Start

1. Set the font scale to `1.0`.
2. Start the example app.

Expected result: `Current scale` shows `1`, and `Changes` shows `No changes yet`.

### Larger Font Size at Start

1. Stop the example app.
2. Set the font scale to `1.3`.
3. Start the example app.

Expected result: `Current scale` shows `1.3`, not `1.2999999523162842`.

### Change While the App Runs

1. Start the example app at a font scale of `1.0`.
2. Set the font scale to `1.3`.
3. Return to the example app.

Expected result: `Changes` shows one new entry with `1.3`, and `Current scale` shows
`1.3`. The sample line grows. The app did not restart: entries from before the change
are still in the list.

Press `Read again`. `Current scale` still shows `1.3`.

### Rotation

1. Start the example app.
2. Turn the device a quarter turn, then turn it back.

Expected result: `Changes` shows no new entry.

### Smallest and Largest Font Size

1. Set the font scale to the smallest value in the Settings app, for example `0.85`.
2. Set the font scale to the largest value in the Settings app, for example `2.0`.

Expected result: each change adds one entry with the same value as
`adb shell settings get system font_scale`.

### WebView Text Zoom

This scenario makes sure that the WebView does not scale text by itself. The heading,
the labels and the list in the example app do not use the scale in their CSS.

1. Set the font scale to `1.0` and take a screenshot:

   ```sh
   adb exec-out screencap -p > font-scale-1.png
   ```

2. Set the font scale to `2.0` and take a second screenshot:

   ```sh
   adb exec-out screencap -p > font-scale-2.png
   ```

3. Compare the two screenshots.

Expected result: the heading `Tauri Plugin Text Scale` has the same size in both
screenshots. Only the sample line, `This line is 16px times the current scale.`, is
larger in the second screenshot, at twice its first size.

If the heading also grows, the WebView still scales text by itself.
