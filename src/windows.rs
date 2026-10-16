//! Reads the Windows text size and reports its changes.
//!
//! The Windows Settings app has `Accessibility > Text size`, a slider from 100% to 225%.
//! The plugin reads it as [`UISettings.TextScaleFactor`][text-scale-factor], which is the
//! slider's percentage divided by 100, and listens to
//! [`UISettings.TextScaleFactorChanged`][text-scale-factor-changed].
//!
//! # Threads
//!
//! `UISettings` is an agile Windows Runtime class, so any thread can use it. Windows
//! raises `TextScaleFactorChanged` on a thread of its own, not on the main thread, so the
//! handler only stores the new scale and emits the event. The handler reads the scale
//! from the `UISettings` that raised the event, and a reader gets the stored scale
//! without a call into Windows. A reader on any thread, including the async runtime, so
//! needs no COM apartment of its own.
//!
//! The plugin creates `UISettings` in Tauri's `setup`, where the `windows` crate
//! initializes COM by itself if the thread has not. It keeps the instance alive in
//! managed state, because Windows stops the event when the last reference to the object
//! is gone.
//!
//! [text-scale-factor]: https://learn.microsoft.com/en-us/uwp/api/windows.ui.viewmanagement.uisettings.textscalefactor
//! [text-scale-factor-changed]: https://learn.microsoft.com/en-us/uwp/api/windows.ui.viewmanagement.uisettings.textscalefactorchanged

use std::marker::PhantomData;
use std::sync::Arc;

use tauri::{AppHandle, Emitter, Runtime};
use tracing::{debug, warn};
use windows::Foundation::TypedEventHandler;
use windows::UI::ViewManagement::UISettings;
use windows::core::IInspectable;

use crate::TEXT_SCALE_CHANGED_EVENT;
use crate::scale_cache::{ScaleCache, valid_scale};

/// The text scale at Windows' default text size, which is also the scale that the plugin
/// reports when it cannot read the setting.
const DEFAULT_TEXT_SCALE: f64 = 1.0;

/// Why the plugin could not start to read the Windows text size.
#[derive(Debug, thiserror::Error)]
enum SetupError {
   /// Windows rejected a call.
   #[error(transparent)]
   Windows(#[from] windows::core::Error),

   /// Windows reported a text scale that an app cannot use.
   #[error("Windows reported the text scale {0}")]
   InvalidScale(f64),
}

/// Creates the Windows text scale, which reports `1.0` and never emits the event if the
/// plugin cannot read the setting.
pub(crate) fn init<R: Runtime>(app: &AppHandle<R>) -> TextScale<R> {
   let cache = Arc::new(ScaleCache::new(DEFAULT_TEXT_SCALE));

   let watcher = match Watcher::start(app.clone(), Arc::clone(&cache)) {
      Ok(watcher) => Some(watcher),
      Err(error) => {
         warn!(%error, "failed to read the Windows text scale, so it reports 1 and no changes");
         None
      }
   };

   TextScale {
      cache,
      _watcher: watcher,
      _runtime: PhantomData,
   }
}

/// Access to the Windows text scale. The type parameter gives [`crate::TextScale`] the
/// same signature on every platform.
pub(crate) struct TextScale<R: Runtime> {
   cache: Arc<ScaleCache>,
   /// Keeps the `UISettings` and its event handler alive for the life of the app.
   _watcher: Option<Watcher>,
   _runtime: PhantomData<fn() -> R>,
}

impl<R: Runtime> TextScale<R> {
   /// Returns the latest text scale.
   pub(crate) fn scale(&self) -> crate::Result<f64> {
      Ok(self.cache.get())
   }

   /// Returns the latest text scale. The cache answers, so there is nothing to wait for.
   pub(crate) async fn scale_async(&self) -> crate::Result<f64> {
      self.scale()
   }
}

/// A `UISettings` with a registered `TextScaleFactorChanged` handler.
struct Watcher {
   settings: UISettings,
   token: i64,
}

impl Watcher {
   /// Registers the handler, then reads the current scale into `cache`. The handler comes
   /// first, so that no change falls between the read and the registration. If the read
   /// fails, dropping the watcher removes the handler again.
   fn start<R: Runtime>(app: AppHandle<R>, cache: Arc<ScaleCache>) -> Result<Self, SetupError> {
      let settings = UISettings::new()?;
      let handler_cache = Arc::clone(&cache);
      let handler = TypedEventHandler::<UISettings, IInspectable>::new(move |sender, _| {
         // Windows calls the handler on a thread of its own, and an error that leaves the
         // handler would only reach Windows, so every failure is logged here instead.
         let Some(settings) = sender.as_ref() else {
            return Ok(());
         };

         match settings.TextScaleFactor() {
            Ok(raw) => publish(&app, &handler_cache, raw),
            Err(error) => warn!(%error, "failed to read the changed Windows text scale"),
         }

         Ok(())
      });
      let token = settings.TextScaleFactorChanged(&handler)?;
      let watcher = Self { settings, token };

      let raw = watcher.settings.TextScaleFactor()?;
      let scale = valid_scale(raw).ok_or(SetupError::InvalidScale(raw))?;

      debug!(scale, "read the Windows text scale");
      cache.replace(scale);

      Ok(watcher)
   }
}

impl Drop for Watcher {
   fn drop(&mut self) {
      if let Err(error) = self.settings.RemoveTextScaleFactorChanged(self.token) {
         debug!(%error, "failed to remove the Windows text scale handler");
      }
   }
}

/// Stores a scale that Windows reported and, when it differs from the stored one, emits it
/// to every window.
fn publish<R: Runtime>(app: &AppHandle<R>, cache: &ScaleCache, raw: f64) {
   let Some(scale) = valid_scale(raw) else {
      warn!(scale = raw, "ignoring an unusable Windows text scale");
      return;
   };

   if !cache.replace(scale) {
      debug!(scale, "the Windows text scale did not change");
      return;
   }

   debug!(scale, "emitting the new text scale");

   if let Err(error) = app.emit(TEXT_SCALE_CHANGED_EVENT, scale) {
      warn!(%error, "failed to emit the new text scale");
   }
}
