use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::{FromSample, Sample, SampleFormat};
use std::time::Duration;

fn write_hz<T: Sample + FromSample<f32>>(
    sample_rate: f32,
    channels: usize,
) -> impl FnMut(&mut [T], &cpal::OutputCallbackInfo) + Send + 'static {
    let mut phase = 0.0_f32;
    let volume = 0.1;

    move |data: &mut [T], _| {
        for frame in data.chunks_mut(channels) {
            let value = (phase * std::f32::consts::TAU).sin() * volume;

            for sample in frame.iter_mut() {
                *sample = T::from_sample(value);
            }

            phase = (phase + 440.0 / sample_rate).fract();
        }
    }
}

fn main() {
    let host = cpal::default_host();

    let device = host
        .default_output_device()
        .expect("no output device available");

    let mut supported_configs_range = device
        .supported_output_configs()
        .expect("error while querying configs");

    let supported_config = supported_configs_range
        .next()
        .expect("no supported config?!")
        .with_max_sample_rate();

    let err_fn = |err| {
        eprintln!("an error occurred on the output audio stream: {}", err);
    };

    let sample_format = supported_config.sample_format();
    let config: cpal::StreamConfig = supported_config.into();

    let sample_rate = config.sample_rate as f32;
    let channels = config.channels as usize;

    let stream = match sample_format {
        SampleFormat::F32 => {
            device.build_output_stream(config, write_hz::<f32>(sample_rate, channels), err_fn, None)
        }
        SampleFormat::I16 => {
            device.build_output_stream(config, write_hz::<i16>(sample_rate, channels), err_fn, None)
        }
        SampleFormat::U16 => {
            device.build_output_stream(config, write_hz::<u16>(sample_rate, channels), err_fn, None)
        }
        SampleFormat::U8 => {
            device.build_output_stream(config, write_hz::<u8>(sample_rate, channels), err_fn, None)
        }
        format => panic!("Unsupported sample format '{format}'"),
    }
    .unwrap();

    stream.play().unwrap();
    std::thread::sleep(Duration::from_secs(5));
}
