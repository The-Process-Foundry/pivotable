//! Echo some messages to the console

use std::io::{Result, Write};

use tracing::{Level, Subscriber};
use tracing_subscriber::{filter::filter_fn, prelude::*, Registry};

use gloo::console::{debug, error, info, warn};

// Send a copy of the tracing logs to the console
pub struct ConsoleLogger {
  buffer: String,
  writer: Box<dyn FnMut(&str)>,
}

impl ConsoleLogger {
  fn new(writer: Box<dyn FnMut(&str)>) -> ConsoleLogger {
    ConsoleLogger {
      buffer: String::new(),
      writer,
    }
  }
}

impl Write for ConsoleLogger {
  fn write(&mut self, buf: &[u8]) -> Result<usize> {
    let buf_len = buf.len();
    self.buffer.push_str(std::str::from_utf8(buf).unwrap());
    self.flush()?;
    Ok(buf_len)
  }

  fn flush(&mut self) -> Result<()> {
    (*self.writer)(&self.buffer);
    self.buffer.truncate(0);
    Ok(())
  }
}

pub fn echo_to_console(subscriber: Option<Registry>) -> impl Subscriber {
  let subscriber = subscriber.unwrap_or_else(|| Registry::default());

  let trace_layer = tracing_subscriber::fmt::layer()
    .json()
    .without_time()
    .with_writer(|| {
      ConsoleLogger::new(Box::new(|buf| {
        debug!(buf);
      }))
    })
    .with_filter(filter_fn(|metadata| metadata.level() == &Level::TRACE));

  let debug_layer = tracing_subscriber::fmt::layer()
    .json()
    .without_time()
    .with_writer(|| {
      ConsoleLogger::new(Box::new(|buf| {
        debug!(buf);
      }))
    })
    .with_filter(filter_fn(|metadata| metadata.level() == &Level::DEBUG));

  let info_layer = tracing_subscriber::fmt::layer()
    .without_time()
    .with_writer(|| {
      ConsoleLogger::new(Box::new(|buf| {
        info!(buf);
      }))
    })
    .with_filter(filter_fn(|metadata| metadata.level() == &Level::INFO));

  let warn_layer = tracing_subscriber::fmt::layer()
    .without_time()
    .with_writer(|| {
      ConsoleLogger::new(Box::new(|buf| {
        warn!(buf);
      }))
    })
    .with_filter(filter_fn(|metadata| metadata.level() == &Level::WARN));

  let error_layer = tracing_subscriber::fmt::layer()
    .without_time()
    .with_writer(|| {
      ConsoleLogger::new(Box::new(|buf| {
        error!(buf);
      }))
    })
    .with_filter(filter_fn(|metadata| metadata.level() == &Level::ERROR));

  subscriber
    .with(trace_layer)
    .with(debug_layer)
    .with(info_layer)
    .with(warn_layer)
    .with(error_layer)
}
