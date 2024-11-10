use cpal::traits::{DeviceTrait, HostTrait};
use cpal::{Device, SupportedStreamConfigRange};

fn main() {
    let host = cpal::default_host();
    print_host_info(&host);

    let default_input_device: Device = host
        .default_output_device()
        .expect("no default input device");

    println!(
        "\nDefault input device: {}",
        default_input_device.name().expect("Device Name Error")
    );

    let default_output_device: Device = host
        .default_output_device()
        .expect("no default output device");

    println!(
        "\nDefault output device: {}",
        default_output_device.name().expect("Device Name Error")
    );

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
    println!("\n---Device: {}---", dev.name().expect("Device Name Error"));
    let supported_input_configs: Vec<SupportedStreamConfigRange> =
        dev.supported_input_configs().unwrap().collect();

    let supported_output_configs: Vec<SupportedStreamConfigRange> =
        dev.supported_output_configs().unwrap().collect();

    if supported_input_configs.len() > 0 {
        println!("Supported Input Configs");
        for c in supported_input_configs {
            print_stream_config(&c)
        }
    }

    if supported_output_configs.len() > 0 {
        println!("\nSupported Output Configs");
        for c in supported_output_configs {
            print_stream_config(&c)
        }
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
