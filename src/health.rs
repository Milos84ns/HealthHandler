use serde::{Deserialize,Serialize};
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use sysinfo::System;
use crate::health_dependency::HealthDependency;
use scheduled_executor::ThreadPoolExecutor;


#[derive(Debug,Serialize,Deserialize,Clone)]
pub struct Health {
    pub component: String,
    pub component_description:String,
    pub version:String,
    #[serde(rename = "serviceStarted")]
    pub service_started:u64,
    #[serde(rename = "isAvailable")]
    pub(crate) is_available:bool,
    #[serde(rename = "appState")]
    pub app_state:AppState,
    pub stats:Stats,

    pub dependencies: Vec<HealthDependency>,
}

#[derive(Clone)]
pub struct HealthService {
    pub health: Health,
    executor: ThreadPoolExecutor,
}

impl HealthService {
    pub fn new(comp:String,desc:String,ver:String,app_s: AppState) -> Self {
        HealthService {
            health:Health::new(comp,desc,ver,app_s),
            executor: ThreadPoolExecutor::new(1).unwrap(),
        }
    }

    pub fn set_state(&mut self,new_state:AppState){
         self.health.set_state(new_state);
    }

    pub fn get_health(&self) -> Health {
        Health {
            is_available: self.health.clone().compute_availability(),
            component:self.health.component.to_owned(),
            component_description:self.health.component_description.to_owned(),
            version:self.health.version.to_owned(),
            service_started:self.health.service_started.to_owned(),
            app_state:self.health.app_state.to_owned(),
            stats:get_stats(),
            dependencies:self.health.dependencies.to_owned(),
        }
    }

    pub fn start(&'static self) {
        self.executor.schedule_fixed_rate(
            Duration::from_secs(10),  // Wait 2 seconds before scheduling the first task
            Duration::from_secs(15),  // and schedule every following task at 5 seconds intervals
            |_remote| {
                // Code to be scheduled. The code will run on one of the threads in the thread pool.
                // The `remote` handle can be used to schedule additional work on the event loop,
                // if needed.
                self.get_health();
            },
        );
    }
}

impl Health {
    fn basic_health(&mut self) {
        self.stats = get_stats();
    }

    pub(crate) fn new(comp:String, desc:String, ver:String, app_s:AppState) ->Self {
        Health {
            component:comp,
            component_description:desc,
            version:ver,
            is_available:false,
            app_state:app_s,
            service_started: current_timestamp(),
            dependencies:vec![],
            stats:get_stats(),
        }
    }

    /// Get all dependencies and check if they are available
    pub fn compute_availability(&mut self) -> bool {
        let has_false = &self
            .dependencies
            .clone()
            .into_iter()
            .filter(|dep| dep.is_available == false)
            .peekable()
            .peek()
            .is_none();
        return  *has_false;
    }

    pub fn update_health(&mut self, dep:Vec<HealthDependency>)
    {
        self.dependencies = dep;
        self.is_available = self.compute_availability();
    }

    pub fn set_available(&mut self, available:bool)
    {
        self.is_available = available;
        self.basic_health();
    }

    fn has_updated_recently(&self, timestamp: u64) -> bool {
        let now = current_timestamp();
        let elapsed_since_timestamp = now - timestamp;
        let service_elapsed = now - self.service_started;
        elapsed_since_timestamp < 5 * 60 || service_elapsed < 5 * 60
    }

    pub fn set_state(&mut self,new_state:AppState) {
        self.app_state = new_state;
    }
}

#[derive(Debug,Serialize,Deserialize,Clone)]
pub enum AppState{
    Stopped,
    Stopping,
    Started,
    Running,
    Error,
    NotRunning
}

#[derive(Debug,Deserialize,Serialize,Clone)]
pub struct Stats{
    #[serde(rename = "memoryUsedMB")]
    used_mem:u64,
    #[serde(rename = "cpuUsed")]
    used_cpu:f32,
    #[serde(rename = "servicePid")]
    pub pid:u32,
}

pub fn get_stats() -> Stats {
    let mut sys = System::new_all();
    sys.refresh_all();

    Stats {
        used_mem: sys.process(sysinfo::get_current_pid().unwrap()).unwrap().memory()/1048576,
        used_cpu: sys.process(sysinfo::get_current_pid().unwrap()).unwrap().cpu_usage(),
        pid: sysinfo::get_current_pid().unwrap().as_u32(),
    }

}

pub fn current_timestamp() -> u64{
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs()
}


