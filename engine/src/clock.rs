use std::time::{Duration, Instant};

pub struct Clock
{
    paused: bool,
    // Accumulator in seconds for current frame
    accumulator_current_s: f32,
    accumulator_s: f64,
    t0: Instant,
    t1: Instant
}

/// Clock structure for engine
impl Clock
{
    pub fn new() -> Self
    {
        Self {
            paused: false,
            accumulator_current_s: 0.0,
            accumulator_s: 0.0,
            t0: Instant::now(),
            t1: Instant::now()
        }
    }

    pub fn pause(&mut self, pause: bool)
    {
        if self.paused
        {
            if pause { return; }

            self.paused = false;
            // Reset point from which to measure time
            self.t1 = Instant::now();
        }
        else 
        {
            if !pause { return; }

            // Add time from current iteration to current frame
            self.paused = true;
            self.accumulator_current_s += self.t1.elapsed().as_secs_f32();
        }
    }

    pub fn paused(&self) -> bool
    {
        self.paused
    }

    /// Returns total elapsed time since engine start excluding paused times
    pub fn get_total_time(&self) -> f32
    {
        self.accumulator_s as f32 + self.get_current_delta_time()
    }

    /// Returns the total time including time when paused since the engine was created
    pub fn get_absolute_total_time(&self) -> f32
    {
        self.t0.elapsed().as_secs_f32()

    }

    /// Starts measuring a single engine iteration (update + render)
    /// Returns elapsed time in seconds from the previous iteration
    pub fn start_next_iteration(&mut self) -> f32
    {

        // Get total frame time
        let delta_time = self.get_current_delta_time();

        self.accumulator_s += delta_time as f64;
        self.accumulator_current_s = 0.0f32;

        self.t1 = Instant::now();

        delta_time
    }

    // Helper function
    fn get_current_delta_time(&self) -> f32
    {
        let mut delta_time = self.accumulator_current_s;
        // If paused, no need to add additional elapsed time, because t1 is invalid
        if !self.paused
        {
            delta_time += self.t1.elapsed().as_secs_f32();
        }

        delta_time

    }
}
