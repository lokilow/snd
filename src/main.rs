use cpal::traits::{DeviceTrait, HostTrait};
use cpal::Device;

fn main() {
    let host = cpal::default_host();
    dbg!("Host ID: {}", host.id());
    let default_output_device: Device = host
        .default_output_device()
        .expect("no output device available");
    println!("Default output_device:");
    print_device_info(&default_output_device);

    let all_devices = host.devices().expect("could not get all devices");
    for d in all_devices {
        print_device_info(&d);
    }
}

fn print_device_info(dev: &cpal::Device) {
    println!("{}", dev.name().expect("Device Name Error"));
}
