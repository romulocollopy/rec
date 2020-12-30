pub fn run() -> &'static str {
    let current_host = host::get_default();
    host::get_host_name(current_host)
}

mod host {
    // use cpal::traits::HostTrait;
    use cpal::Host;

    pub fn get_host_name(host: Host) -> &'static str {
        host.id().name()
    }

    pub fn get_default() -> Host {
        cpal::default_host()
    }

    #[cfg(test)]
    mod test {
        #[test]
        fn test_cpal() {
            let current_host = super::get_default();
            super::get_host_name(current_host);
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_run() {
        assert_eq!(run(), "CoreAudio");
    }
}
