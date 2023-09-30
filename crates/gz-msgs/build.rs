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

    if count >= 2 {
        panic!("Only one of the following features can be enabled: fortress, garden, harmonic");
    }
}
