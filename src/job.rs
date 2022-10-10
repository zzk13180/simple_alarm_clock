use chrono::{DateTime, Local};
pub use cron::Schedule;
pub use uuid::Uuid;

pub struct Job<'a> {
    schedule: Schedule,
    run: Box<dyn (FnMut() -> ()) + 'a>,
    last_tick: Option<DateTime<Local>>,
    limit_missed_runs: usize,
    job_id: Uuid,
}

impl<'a> Job<'a> {
    pub fn new<T>(schedule: Schedule, run: T) -> Job<'a>
    where
        T: 'a,
        T: FnMut() -> (),
    {
        Job {
            schedule,
            run: Box::new(run),
            last_tick: None,
            limit_missed_runs: 1,
            job_id: Uuid::new_v4(),
        }
    }

    fn tick(&mut self) {
        let now = Local::now();
        if self.last_tick.is_none() {
            self.last_tick = Some(now);
            return;
        }
        if self.limit_missed_runs > 0 {
            for event in self
                .schedule
                .after(&self.last_tick.unwrap())
                .take(self.limit_missed_runs)
            {
                if event > now {
                    break;
                }
                (self.run)();
            }
        } else {
            for event in self.schedule.after(&self.last_tick.unwrap()) {
                if event > now {
                    break;
                }
                (self.run)();
            }
        }

        self.last_tick = Some(now);
    }
}

#[derive(Default)]
pub struct JobScheduler<'a> {
    jobs: Vec<Job<'a>>,
}

impl<'a> JobScheduler<'a> {
    pub fn new() -> JobScheduler<'a> {
        JobScheduler { jobs: Vec::new() }
    }

    pub fn add(&mut self, job: Job<'a>) -> Uuid {
        let job_id = job.job_id;
        self.jobs.push(job);

        job_id
    }

    pub fn tick(&mut self) {
        for job in &mut self.jobs {
            job.tick();
        }
    }
}
