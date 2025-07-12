use std::sync::mpsc;

use cpal::{
    FromSample,
    I24,
    Sample,
    SampleRate,
    SizedSample,
    traits::{DeviceTrait, HostTrait, StreamTrait},
};
use gb_core::components::apu::{AUDIO_BUFFER_SIZE, AUDIO_SAMPLE_RATE, Callback, StereoSample};
use log::{error, info};

type Sender<T> = mpsc::SyncSender<T>;
type Receiver<T> = mpsc::Receiver<T>;

pub struct Audio {
    _stream: cpal::Stream,
    sender: Sender<StereoSample>,
}

impl Audio {
    pub fn new() -> Self {
        let (stream, sender) = stream_setup_for();
        stream.play().unwrap();

        Self {
            _stream: stream,
            sender,
        }
    }

    pub fn get_callback(&self) -> Box<Callback> {
        let sender = self.sender.clone();

        Box::new(move |buffer| {
            for sample in buffer {
                sender.send(*sample).unwrap();
            }
        })
    }
}

fn stream_setup_for() -> (cpal::Stream, Sender<StereoSample>) {
    let (_host, device, config) = host_device_setup();

    let (stream, sender) = match config.sample_format() {
        cpal::SampleFormat::I8 => make_stream::<i8>(&device, &config.into()),
        cpal::SampleFormat::I16 => make_stream::<i16>(&device, &config.into()),
        cpal::SampleFormat::I24 => make_stream::<I24>(&device, &config.into()),
        cpal::SampleFormat::I32 => make_stream::<i32>(&device, &config.into()),
        cpal::SampleFormat::I64 => make_stream::<i64>(&device, &config.into()),
        cpal::SampleFormat::U8 => make_stream::<u8>(&device, &config.into()),
        cpal::SampleFormat::U16 => make_stream::<u16>(&device, &config.into()),
        cpal::SampleFormat::U32 => make_stream::<u32>(&device, &config.into()),
        cpal::SampleFormat::U64 => make_stream::<u64>(&device, &config.into()),
        cpal::SampleFormat::F32 => make_stream::<f32>(&device, &config.into()),
        cpal::SampleFormat::F64 => make_stream::<f64>(&device, &config.into()),
        sample_format => panic!("Unsupported sample format '{sample_format}'"),
    };

    (stream, sender)
}

fn host_device_setup() -> (cpal::Host, cpal::Device, cpal::SupportedStreamConfig) {
    let host = cpal::default_host();

    let device = host.default_output_device().unwrap();
    info!("Output device : {}", device.name().unwrap());

    let mut configs = device.supported_output_configs().unwrap();
    let config = configs
        .next()
        .unwrap()
        .with_sample_rate(SampleRate(AUDIO_SAMPLE_RATE as u32));

    info!("Output config : {config:?}");

    (host, device, config)
}

fn make_stream<T>(
    device: &cpal::Device,
    config: &cpal::StreamConfig,
) -> (cpal::Stream, Sender<StereoSample>)
where
    T: SizedSample + FromSample<f32>,
{
    let num_channels = config.channels as usize;

    let (sender, receiver): (Sender<StereoSample>, Receiver<StereoSample>) =
        mpsc::sync_channel(AUDIO_BUFFER_SIZE);

    let err_fn = |err| error!("Error building output sound stream: {err}");

    let stream = device
        .build_output_stream(
            config,
            move |output: &mut [T], _| process_frame(output, num_channels, &receiver),
            err_fn,
            None,
        )
        .unwrap();

    (stream, sender)
}

fn process_frame<SampleType>(
    output: &mut [SampleType],
    num_channels: usize,
    receiver: &Receiver<StereoSample>,
) where
    SampleType: Sample + FromSample<f32>,
{
    for frame in output.chunks_mut(num_channels) {
        if let Ok(stereo_sample) = receiver.try_recv() {
            let min_supported_channels = num_channels.min(stereo_sample.len());

            for (i, sample) in stereo_sample
                .into_iter()
                .take(min_supported_channels)
                .enumerate()
            {
                frame[i] = SampleType::from_sample(sample);
            }
        } else {
            frame.fill(SampleType::from_sample(0.0));
        }
    }
}
