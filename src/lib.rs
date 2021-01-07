pub fn run() {
    let current_host = host::get_default();
    host::get_host_name(&current_host);
    let device_names = host::get_devices_names(host::get_devices(&current_host));
    for device_name in device_names {
        println!("{}", device_name)
    }
}

mod host {
    use cpal::traits::DeviceTrait;
    use cpal::traits::HostTrait;
    use cpal::Device;

    use cpal::Host;

    pub fn get_devices_names(devices: Vec<Device>) -> Vec<String> {
        let mut device_names: Vec<String> = Vec::new();
        for d in devices {
            match d.name() {
                Ok(device_name) => device_names.push(device_name),
                Err(_) => {}
            }
        }
        device_names
    }

    pub fn get_devices(current_host: &Host) -> Vec<Device> {
        let mut avaliable_devices = Vec::new();
        match current_host.devices() {
            Ok(devices) => devices.for_each(|d| avaliable_devices.push(d)),
            Err(_) => return avaliable_devices,
        };
        avaliable_devices
    }

    pub fn get_default() -> Host {
        cpal::default_host()
    }

    pub fn get_host_name(host: &Host) -> &'static str {
        host.id().name()
    }

    #[cfg(test)]
    mod test {
        #[test]
        fn test_cpal() {
            let current_host = super::get_default();
            super::get_host_name(&current_host);
        }

        #[test]
        fn test_get_devices() {
            let current_host = super::get_default();
            super::get_devices(&current_host);
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_run() {
        assert_eq!(run(), ());
    }
}
