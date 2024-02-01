use serde::{Deserialize,Serialize};
use std::time::{SystemTime,UNIX_EPOCH};
use sysinfo::System;
use crate::health_dependency::HealthDependency;


#[derive(Debug,Serialize,Deserialize,Clone)]
pub struct Health {
    pub component: String,
    pub component_description:String,
    pub version:String,
    #[serde(rename = "timeStamp")]
    pub time_stamp:u64,
    #[serde(rename = "isAvailable")]
    pub(crate) is_available:bool,
    #[serde(rename = "appState")]
    app_state:AppState,
    pub stats:Stats,
    pub dependencies: Vec<HealthDependency>,
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
            time_stamp: current_timestamp(),
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

    pub fn set_state(&mut self,new_state:AppState) {
        self.app_state = new_state;
    }

    pub fn get_health(&self) -> Health {
        Health {
            is_available: self.clone().compute_availability(),
            component:self.component.to_owned(),
            component_description:self.component_description.to_owned(),
            version:self.version.to_owned(),
            time_stamp:self.time_stamp.to_owned(),
            app_state:self.app_state.to_owned(),
            stats:self.stats.to_owned(),
            dependencies:self.dependencies.to_owned(),
        }
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
    used_cpu:f32
}

pub fn get_stats() -> Stats {
    let mut sys = System::new_all();
    sys.refresh_all();

    Stats {
        used_mem: sys.process(sysinfo::get_current_pid().unwrap()).unwrap().memory()/1048576,
        used_cpu: sys.process(sysinfo::get_current_pid().unwrap()).unwrap().cpu_usage(),
    }

}

pub fn current_timestamp() -> u64{
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs()
}
