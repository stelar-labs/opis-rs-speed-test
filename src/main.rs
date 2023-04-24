use std::{fs::File, time::Instant};
use std::io::Write;
use opis::{Integer, Bit};
use rand::Rng;
use sysinfo::{System, SystemExt, CpuExt};

fn main() -> Result<(), std::io::Error> {

    let mut file = File::create("result.md")?;

    writeln!(file, "# Opis-rs Speed Test Result")?;

    let system = System::new_all();

    let cpu = system.cpus().iter().next().unwrap();
    
    writeln!(file, "Processor: {}", cpu.brand())?;

    writeln!(file, "## Integer")?;

    writeln!(file, "### Addition")?;

    let mut rng = rand::thread_rng();

    writeln!(file, "| Bits | Average(ns) | Per Second |")?;

    writeln!(file, "|---|---|---|")?;

    for bits in [2,4,8,16,32,64,128,256,512,1024,2048] {

        let mut sum: f32 = 0.0;

        for _ in 0..100 {

            let a = Integer::from(
                &((0..bits)
                .into_iter()
                .map(|_| match rng.gen_bool(0.5) {
                    true => Bit::One,
                    false => Bit::Zero
                })
                .collect::<Vec<Bit>>())[..]
            );
    
            let b = Integer::from(
                &((0..bits)
                .into_iter()
                .map(|_| match rng.gen_bool(0.5) {
                    true => Bit::One,
                    false => Bit::Zero
                })
                .collect::<Vec<Bit>>())[..]
            );
    
            let now = Instant::now();
    
            let _c = a + b;
    
            let later = Instant::now();
    
            sum += later.duration_since(now).as_nanos() as f32;

        }

        let average = sum / 100.0;

        let per_second = 1000000000.0 / average;

        writeln!(file, "| {} | {} | {} |", bits, average, per_second)?;

    }

    Ok(())
    
}
