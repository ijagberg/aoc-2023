pub struct Race {
    time: u64,
    record: u64,
}

impl Race {
    pub fn new(time: u64, record: u64) -> Self {
        Self { time, record }
    }

    pub fn beats_record(&self, held_time: u64) -> bool {
        let distance = get_distance(held_time, self.time - held_time);
        distance > self.record
    }

    pub fn time(&self) -> u64 {
        self.time
    }

    pub fn record(&self) -> u64 {
        self.record
    }
}

fn get_distance(speed: u64, time: u64) -> u64 {
    speed * time
}
