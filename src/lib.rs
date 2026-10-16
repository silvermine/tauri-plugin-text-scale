//! Reads the operating system text size setting and reports changes to it.
//!
//! The plugin reports the text size as a scale, where `1.0` is the platform's default
//! text size. An app multiplies its own text sizes by the scale.
//!
//! The plugin has one command, `get_text_scale`, and one event,
//! [`TEXT_SCALE_CHANGED_EVENT`], with the new scale as its payload. A platform that reads
//! a text size emits the event from Rust to every window, so the frontend listens for one
//! event on every platform.
//!
//! A platform that has no text size, or that the plugin does not support yet, returns
//! `1.0` and never emits the event.
//!
//! # Examples
//!
//! ```no_run
//! tauri::Builder::default()
//!    .plugin(tauri_plugin_text_scale::init())
//!    .run(tauri::generate_context!())
//!    .expect("error while running tauri application");
//! ```

mod commands;
#[cfg(all(desktop, not(windows)))]
mod desktop;
mod error;
#[cfg(mobile)]
mod mobile;
#[cfg(any(windows, test))]
mod scale_cache;
#[cfg(windows)]
mod windows;

#[cfg(windows)]
use self::windows as platform;
#[cfg(all(desktop, not(windows)))]
use desktop as platform;
#[cfg(mobile)]
use mobile as platform;

pub use error::{Error, Result};

use tauri::{Manager, Runtime, plugin::TauriPlugin};
use tracing::debug;

/// The name of the event that reports a new text scale.
///
/// The payload is the new scale as a number.
pub const TEXT_SCALE_CHANGED_EVENT: &str = "tauri-plugin-text-scale:changed";

/// Access to the operating system text scale.
pub struct TextScale<R: Runtime>(platform::TextScale<R>);

impl<R: Runtime> TextScale<R> {
   /// Returns the current text scale, where `1.0` is the platform's default text size.
   ///
   /// # Errors
   ///
   /// Returns [`Error::ReadFailed`] when the platform fails to report its text size. On
   /// iOS, it also returns that error before the app creates its first WebView, because
   /// the plugin reads the text size when Tauri gives it a WebView.
   pub fn scale(&self) -> Result<f64> {
      self.0.scale()
   }

   /// Returns the current text scale without blocking an async worker thread while the
   /// platform replies.
   pub(crate) async fn scale_async(&self) -> Result<f64> {
      self.0.scale_async().await
   }
}

/// Extensions to [`tauri::App`], [`tauri::AppHandle`] and [`tauri::Window`] to access the
/// text scale.
pub trait TextScaleExt<R: Runtime> {
   fn text_scale(&self) -> &TextScale<R>;
}

impl<R: Runtime, T: Manager<R>> TextScaleExt<R> for T {
   fn text_scale(&self) -> &TextScale<R> {
      self.state::<TextScale<R>>().inner()
   }
}

/// Initializes the text scale plugin.
///
/// # Examples
///
/// ```no_run
/// tauri::Builder::default()
///    .plugin(tauri_plugin_text_scale::init())
///    .run(tauri::generate_context!())
///    .expect("error while running tauri application");
/// ```
pub fn init<R: Runtime>() -> TauriPlugin<R> {
   tauri::plugin::Builder::new("text-scale")
      .invoke_handler(tauri::generate_handler![commands::get_text_scale])
      .setup(|app, _api| {
         debug!("registering the text scale plugin state");

         #[cfg(all(desktop, not(windows)))]
         let platform = desktop::init();

         #[cfg(windows)]
         let platform = windows::init(app);

         #[cfg(mobile)]
         let platform = mobile::init(app, _api)?;

         app.manage(TextScale::<R>(platform));

         Ok(())
      })
      .build()
}

#[cfg(test)]
mod tests {
   use super::*;

   #[test]
   fn the_event_name_matches_the_guest_js_bindings() {
      let bindings = include_str!("../guest-js/index.ts");
      let declaration =
         format!("export const TEXT_SCALE_CHANGED_EVENT = '{TEXT_SCALE_CHANGED_EVENT}';");

      assert!(bindings.contains(&declaration));
   }
}
