use cpal::traits::{DeviceTrait, HostTrait};
use cpal::{Device, SupportedStreamConfigRange};

fn main() {
    let host = cpal::default_host();
    print_host_info(&host);

    let default_output_device: Device = host
        .default_output_device()
        .expect("no default output device");

    println!("Default output device:");
    print_device_info(&default_output_device);

    let default_input_device: Device = host
        .default_output_device()
        .expect("no default input device");
    println!("Default input device:");
    print_device_info(&default_input_device);
    println!("\n\n\n---All Devices---");

    let all_devices = host.devices().expect("could not get all devices");
    for d in all_devices {
        print_device_info(&d);
    }
}

fn print_host_info(host: &cpal::Host) {
    println!("---Host info---");
    println!("ID: {:?}", host.id());
}

fn print_device_info(dev: &cpal::Device) {
    println!(
        "\n\n---Device: {}---",
        dev.name().expect("Device Name Error")
    );
    let supported_input_configs = dev.supported_input_configs().unwrap();
    let supported_output_configs = dev.supported_output_configs().unwrap();
    println!("Supported Input Configs");
    for c in supported_input_configs {
        print_stream_config(&c)
    }
    println!("Supported Output Configs");
    for c in supported_output_configs {
        print_stream_config(&c)
    }
}

fn print_stream_config(conf: &SupportedStreamConfigRange) {
    let channel_count = conf.channels();
    let sample_format = conf.sample_format();
    let min_sample_rate = conf.min_sample_rate();
    let max_sample_rate = conf.max_sample_rate();
    let buffer_size = conf.buffer_size();
    println!("\nChannel count: {}", channel_count);
    println!("Sample Format: {}", sample_format);
    println!("Min Sample Rate: {:?}", min_sample_rate);
    println!("Max Sample Rate: {:?}", max_sample_rate);
    println!("Buffer Size: {:?}", buffer_size);
}
