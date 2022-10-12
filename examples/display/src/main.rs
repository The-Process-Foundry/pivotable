//! The base of the invoicer example

pub use pivotable::logger;

pub mod demo;

pub mod fetch;

// This is the entry point for the web app
fn main() {
  let log = logger::echo_to_console(None);

  // FIXME: This should be exported as part of pivotable, or possibly a tool in Yewzy
  let _ = tracing::subscriber::set_global_default(log);

  tracing::debug!("Running the demo table site");
  yew::Renderer::<demo::App>::new().render();
}
