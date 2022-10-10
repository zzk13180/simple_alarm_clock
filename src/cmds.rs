use std::{str::FromStr, time::Duration};

#[tauri::command]
pub fn alarm() {
    tauri::async_runtime::spawn(async move {
        //                   sec  min   hour   day of month   month   day of week   year
        // let expression = "5   30   9,12,15     1,15       May-Aug  Mon,Wed,Fri  2018/2";
        let mut sched = crate::job::JobScheduler::new();
        let expression = "1/10 * * * * * *";
        println!("cron {} 任务执行 (每10秒执行一次)", expression);
        let schedule = cron::Schedule::from_str(expression).unwrap();
        sched.add(crate::job::Job::new(schedule, || {
          println!("当前时间 {:?}", chrono::Local::now());
            let schedule = cron::Schedule::from_str(expression).unwrap();
            let mut upcoming = schedule.upcoming(chrono::Local);
            let next = upcoming.next();
            println!("下次任务时间 {:?}", next);
        }));
        loop {
            sched.tick();
            std::thread::sleep(Duration::from_millis(500));
        }
    });
}
