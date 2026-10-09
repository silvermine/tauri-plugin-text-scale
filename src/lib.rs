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
#[cfg(not(target_os = "android"))]
mod desktop;
mod error;
#[cfg(target_os = "android")]
mod mobile;

pub use error::{Error, Result};

use tauri::{Manager, Runtime, plugin::TauriPlugin};
use tracing::debug;

/// The name of the event that reports a new text scale.
///
/// The payload is the new scale as a number.
pub const TEXT_SCALE_CHANGED_EVENT: &str = "tauri-plugin-text-scale:changed";

/// Access to the operating system text scale.
#[cfg(not(target_os = "android"))]
pub struct TextScale;

/// Access to the operating system text scale.
#[cfg(target_os = "android")]
pub struct TextScale<R: Runtime>(mobile::TextScale<R>);

#[cfg(not(target_os = "android"))]
impl TextScale {
   /// Returns the current text scale, where `1.0` is the platform's default text size.
   ///
   /// # Errors
   ///
   /// Returns [`Error::ReadFailed`] when the platform fails to report its text size.
   pub fn scale(&self) -> Result<f64> {
      Ok(desktop::text_scale())
   }
}

#[cfg(target_os = "android")]
impl<R: Runtime> TextScale<R> {
   /// Returns the current text scale, where `1.0` is the platform's default text size.
   ///
   /// # Errors
   ///
   /// Returns [`Error::ReadFailed`] when the platform fails to report its text size.
   pub fn scale(&self) -> Result<f64> {
      self.0.scale()
   }
}

/// Extensions to [`tauri::App`], [`tauri::AppHandle`] and [`tauri::Window`] to access the
/// text scale.
#[cfg(not(target_os = "android"))]
pub trait TextScaleExt<R: Runtime> {
   fn text_scale(&self) -> &TextScale;
}

/// Extensions to [`tauri::App`], [`tauri::AppHandle`] and [`tauri::Window`] to access the
/// text scale.
#[cfg(target_os = "android")]
pub trait TextScaleExt<R: Runtime> {
   fn text_scale(&self) -> &TextScale<R>;
}

#[cfg(not(target_os = "android"))]
impl<R: Runtime, T: Manager<R>> TextScaleExt<R> for T {
   fn text_scale(&self) -> &TextScale {
      self.state::<TextScale>().inner()
   }
}

#[cfg(target_os = "android")]
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

         #[cfg(not(target_os = "android"))]
         app.manage(TextScale);

         #[cfg(target_os = "android")]
         app.manage(TextScale(mobile::init(app, _api)?));

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
