#![allow(unused_mut)]

fn main() {
    let mut count = 0;

    #[cfg(feature = "fortress")]
    {
        count += 1;
    }
    #[cfg(feature = "garden")]
    {
        count += 1;
    }
    #[cfg(feature = "harmonic")]
    {
        count += 1;
    }

    match count {
        0 => panic!(
            "At least one of the following features must be enabled: fortress, garden, harmonic"
        ),
        1 => {}
        _ => {
            panic!("Only one of the following features can be enabled: fortress, garden, harmonic")
        }
    }
}
