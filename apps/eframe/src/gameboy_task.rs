use std::sync::{Arc, Mutex, RwLock};

use gb_core::{
    GameBoy,
    constants::{CPU_APPROX_M_CYCLES_PER_FRAME, CPU_CLOCK_RATE},
};

use crate::sys::time::{Duration, Instant};

pub struct GameBoyTask {
    pub gb: Arc<RwLock<GameBoy>>,
    pub running: Arc<Mutex<bool>>,
}

impl GameBoyTask {
    pub fn new(gb: Arc<RwLock<GameBoy>>) -> Self {
        let task = Self {
            gb,
            running: Arc::new(Mutex::new(false)),
        };

        task.start();

        task
    }

    fn start(&self) {
        let gb = self.gb.clone();
        let running = self.running.clone();

        #[allow(clippy::cast_precision_loss)]
        let frame_time = Duration::from_secs_f64(
            (CPU_APPROX_M_CYCLES_PER_FRAME as f64) / (CPU_CLOCK_RATE as f64),
        );

        crate::sys::thread::spawn(async move {
            *running.lock().unwrap() = true;
            let mut next_frame = Instant::now();

            loop {
                {
                    if !*running.lock().unwrap() {
                        continue;
                    }

                    let mut gb = gb.write().unwrap();

                    if gb.cartridge_inserted() {
                        gb.run_frame();
                    }
                }

                next_frame += frame_time;
                let now = Instant::now();
                let remaining = next_frame - now;

                if next_frame > now {
                    crate::sys::thread::sleep(remaining).await;
                } else {
                    next_frame = now;
                }
            }
        });
    }
}
