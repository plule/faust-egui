#![warn(clippy::all, rust_2018_idioms)]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

mod dsp_thread;
mod ui;
use cpal::traits::StreamTrait;
use faust_state::DspHandle;

#[allow(clippy::all)]
#[rustfmt::skip]
mod dsp {
    include!(concat!(env!("OUT_DIR"), "/dsp.rs"));
}

pub const VERSION: &str = env!("CARGO_PKG_VERSION");

fn main() {
    // Init DSP
    let (dsp, state) = DspHandle::<dsp::Instrument>::new();

    // Init sound output
    let stream = dsp_thread::run(dsp);
    stream.play().expect("Failed to play stream");

    // Start UI
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        format!("Faust egui {}", VERSION).as_str(),
        native_options,
        Box::new(move |cc| Box::new(ui::DspUi::new(cc, state))),
    );
}
