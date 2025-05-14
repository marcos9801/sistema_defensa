//archivo para ir recopilando la infgormacion a mostrar en la GUI

use std::sync::{Arc, Mutex, atomic::{AtomicBool, Ordering}};
use std::thread;
use std::time::Duration;
use chrono::Utc;
use chrono_tz::Tz;
use instrumentos::cpu::CPUInfo;
use instrumentos::disco::DiscosInfo;
use instrumentos::memoria::MemoriaInfo;
use instrumentos::interfaces::InterfacesInfo;
use instrumentos::procesos::ProcesosInfo;
#[derive(Debug, Clone)]
pub struct Metric {
    pub id: String, // timestamp
    pub cpu: CPUInfo,
    pub memoria: MemoriaInfo,
    pub red: InterfacesInfo,
    pub disco: DiscosInfo,
    pub procesos: ProcesosInfo,
}
pub struct MonitorHandle{
    stop_flag: Arc<AtomicBool>,
    thread: Option<thread::JoinHandle<()>>,
}
pub struct Metrics {
    pub vector: Vec<Metric>,
    monitor_handle: Option<MonitorHandle>,
}
impl Clone for Metrics {
    fn clone(&self) -> Self {
        Metrics {
            vector: self.vector.clone(),
            monitor_handle: None, // do not clone the handle/thread
        }
    }
}
impl Metrics {
    pub fn new() -> Self  {
        Self { vector: Vec::new(), monitor_handle: None }
    }

    pub fn refresh(&mut self) {
        let zona = chrono_tz::America::Mexico_City; // Zona horaria de México
        let fecha_hora = Utc::now().with_timezone(&zona);
        let fecha = fecha_hora.format("%Y-%m-%d").to_string();
        let hora = fecha_hora.format("%H:%M:%S").to_string();
        let metric = Metric {
            id: fecha_hora.format("%Y-%m-%dT%H:%M:%S%:z").to_string(),
            cpu: CPUInfo::new(),
            memoria: MemoriaInfo::new(),
            red: InterfacesInfo::new(),
            disco: DiscosInfo::new(),
            procesos: ProcesosInfo::new(),
        };
        self.vector.push(metric);
    }

    pub fn latest(&self) -> Option<&Metric> {
        self.vector.last()
    }
    pub fn start_monitoring(&mut self, shared_self: Arc<Mutex<Metrics>>) {
        self.vector.clear();

        let stop_flag = Arc::new(AtomicBool::new(false));
        let stop_flag_clone = stop_flag.clone();

        let handle = thread::spawn(move || {
            while !stop_flag_clone.load(Ordering::Relaxed) {
                {
                    let mut m = shared_self.lock().unwrap();
                    m.refresh();
                }
                thread::sleep(Duration::from_secs(5));
            }
        });

        self.monitor_handle = Some(MonitorHandle {
            stop_flag,
            thread: Some(handle),
        });
    }


    pub fn stop_monitoring(&mut self) {
        if let Some(handle) = self.monitor_handle.take() {
            handle.stop();
        }
    }
}
impl MonitorHandle {
    pub fn stop(mut self) {
        self.stop_flag.store(true, Ordering::Relaxed);
        if let Some(handle) = self.thread.take() {
            let _ = handle.join();
        }
    }
}