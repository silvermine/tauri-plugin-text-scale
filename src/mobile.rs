//! The bridge to the native Android plugin.

use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use tauri::ipc::Channel;
use tauri::plugin::mobile::PluginInvokeError;
use tauri::plugin::{PluginApi, PluginHandle};
use tauri::{AppHandle, Emitter, Runtime};
use tracing::{debug, warn};

use crate::TEXT_SCALE_CHANGED_EVENT;

const PLUGIN_IDENTIFIER: &str = "org.silvermine.plugin.textscale";

const COMMAND_GET_TEXT_SCALE: &str = "getTextScale";
const COMMAND_WATCH: &str = "watch";

#[derive(Debug, Deserialize)]
struct MobileTextScale {
   value: f64,
}

#[derive(Serialize)]
struct WatchArgs {
   channel: Channel,
}

fn read_error(error: PluginInvokeError) -> crate::Error {
   crate::Error::ReadFailed(error.to_string())
}

/// Registers the native plugin, starts to watch for changes and returns the Rust side of
/// the bridge.
pub(crate) fn init<R: Runtime, C: DeserializeOwned>(
   app: &AppHandle<R>,
   api: PluginApi<R, C>,
) -> std::result::Result<TextScale<R>, PluginInvokeError> {
   let handle = api.register_android_plugin(PLUGIN_IDENTIFIER, "TextScalePlugin")?;
   let text_scale = TextScale(handle);

   // Without the watch, `get_text_scale` still works, so the app can start.
   if let Err(error) = text_scale.watch(app.clone()) {
      warn!(%error, "failed to watch the text scale, so changes will not be reported");
   }

   Ok(text_scale)
}

/// Access to the native text scale.
pub(crate) struct TextScale<R: Runtime>(PluginHandle<R>);

impl<R: Runtime> TextScale<R> {
   /// Returns the current text scale.
   pub(crate) fn scale(&self) -> crate::Result<f64> {
      // Calls the native `getTextScale` command of `TextScalePlugin`.
      let result: MobileTextScale = self
         .0
         .run_mobile_plugin(COMMAND_GET_TEXT_SCALE, ())
         .map_err(read_error)?;

      Ok(result.value)
   }

   /// Gives the native plugin a channel for new scales. Each new scale is emitted to every
   /// window as [`TEXT_SCALE_CHANGED_EVENT`].
   fn watch(&self, app: AppHandle<R>) -> std::result::Result<(), PluginInvokeError> {
      let channel = Channel::new(move |body| {
         let change: MobileTextScale = body.deserialize()?;

         debug!(scale = change.value, "emitting the new text scale");
         app.emit(TEXT_SCALE_CHANGED_EVENT, change.value)
      });

      self
         .0
         .run_mobile_plugin(COMMAND_WATCH, WatchArgs { channel })
   }
}
