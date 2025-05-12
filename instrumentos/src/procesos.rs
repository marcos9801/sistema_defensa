/// Módulo que implementa la obtención y análisis de los procesos del sistema.
/// Proporciona información detallada de cada proceso, incluyendo:
/// - PID.
/// - Nombre del proceso.
/// - Tiempo total de ejecución.
/// - Tiempo acumulado en CPU.
/// - Uso actual de CPU y memoria.
/// - Uso de memoria virtual.
/// - Estado del proceso.
///
/// Este módulo define dos estructuras principales:
///
/// ### `ProcesosInfo`
/// Representa un resumen general del estado de los procesos del sistema.
/// - `cantidad_procesos`: Número total de procesos detectados.
/// - `procesos`: Vector con la información detallada de todos los procesos (`Vec<ProcesoInfo>`).
/// - `top_procesos_uso_cpu`: Top 5 procesos que están consumiendo más CPU actualmente.
/// - `top_procesos_uso_memoria`: Top 5 procesos que están usando más memoria física.
/// - `top_procesos_tiempo_cpu`: Top 5 procesos con más tiempo acumulado en CPU.
/// - `top_procesos_tiempo_ejecucion`: Top 5 procesos con mayor tiempo de ejecución.
///
/// #### Métodos de `ProcesosInfo`
/// - `new`: Constructor que obtiene la información directamente del sistema.
/// - Métodos *getter* para acceder a los campos anteriores.
///
/// ### `ProcesoInfo`
/// Representa la información específica de un solo proceso del sistema.
/// - `pid`: Identificador único del proceso.
/// - `nombre`: Nombre del proceso.
/// - `tiempo_ejecucion`: Tiempo total que ha estado en ejecución.
/// - `tiempo_en_cpu`: Tiempo acumulado que ha estado en CPU.
/// - `uso_cpu`: Porcentaje de CPU usado actualmente.
/// - `uso_memoria`: Memoria física usada (en bytes).
/// - `uso_memoria_virtual`: Memoria virtual usada (en bytes).
/// - `estado`: Estado actual del proceso.
///
/// #### Métodos de `ProcesoInfo`
/// - `get_pid`: Devuelve el PID del proceso.
/// - `get_nombre`: Devuelve el nombre del proceso.
/// - `get_tiempo_ejecucion`: Devuelve el tiempo total de ejecución.
/// - `get_tiempo_en_cpu`: Devuelve el tiempo acumulado en CPU.
/// - `get_uso_cpu`: Devuelve el porcentaje de CPU usado.
/// - `get_uso_memoria`: Devuelve el uso de memoria física.
/// - `get_uso_memoria_virtual`: Devuelve el uso de memoria virtual.
/// - `get_estado`: Devuelve el estado del proceso.
///
/// ### Función principal
/// - `obtener_info_procesos`: Devuelve una instancia de `ProcesosInfo` con la información actual del sistema.
/// ### TODO
/// corregir lectura de uso cpu
/// #### Historial de cambios
/// - 2025-04-06: Creación del módulo y definición de estructuras `ProcesosInfo` y `ProcesoInfo`, y metodos para mostrar informacion.

use sysinfo::{ProcessesToUpdate, ProcessRefreshKind, System, Pid};
use std::thread::sleep;
use std::time::Duration;
use std::fmt;
use serde::Serialize;

/// Constante para la conversión de bytes a megabytes.
const B_TO_MB: u64 = 1024 * 1024;

#[derive(Clone, Debug, Serialize)]
pub struct ProcesosInfo {
    cantidad_procesos: usize,
    procesos: Vec<ProcesoInfo>,
    top_procesos_uso_cpu: Vec<ProcesoInfo>,
    top_procesos_uso_memoria: Vec<ProcesoInfo>,
    top_procesos_tiempo_cpu: Vec<ProcesoInfo>,
    top_procesos_tiempo_ejecucion: Vec<ProcesoInfo>,
}

#[derive(Clone, Debug, Serialize)]
pub struct ProcesoInfo{
    pid: i32,
    nombre: String,
    tiempo_ejecucion: u64,
    tiempo_en_cpu: u64,
    uso_cpu: f32,
    uso_memoria: u64,
    uso_memoria_virtual: u64,
    estado: String,
}

impl ProcesosInfo{
    //getters
    // Devuelve la cantidad de procesos
    // Retorno
    // la cantidad de procesos como un entero
    pub fn get_cantidad_procesos(&self) -> usize {
        self.cantidad_procesos
    }
    // Devuelve lista de procesos
    // Retorno
    // la lista de procesos como un vector de ProcesoInfo
    pub fn get_procesos(&self) -> &Vec<ProcesoInfo> {
        &self.procesos
    }
    // Devuelve el proceso en la posicion index
    // Retorno
    // el proceso en la posicion index como un ProcesoInfo
    pub fn get_proceso(&self, index: usize) -> &ProcesoInfo {
        if index < self.procesos.len() {
            &self.procesos[index]
        } else {
            panic!("Índice fuera de rango");
        }
    }
    // Devuelve la lista de procesos que consumen más CPU
    // Retorno
    // la lista de procesos que consumen más CPU como un vector de ProcesoInfo
    pub fn get_top_procesos_uso_cpu(&self) -> &Vec<ProcesoInfo> {
        &self.top_procesos_uso_cpu
    }
    // Devuelve la lista de procesos que consumen más memoria
    // Retorno
    // la lista de procesos que consumen más memoria como un vector de ProcesoInfo
    pub fn get_top_procesos_uso_memoria(&self) -> &Vec<ProcesoInfo> {
        &self.top_procesos_uso_memoria
    }
    // Devuelve la lista de procesos que más tiempo han estado en CPU
    // Retorno
    // la lista de procesos que más tiempo han estado en CPU como un vector de ProcesoInfo

    pub fn get_top_procesos_tiempo_cpu(&self) -> &Vec<ProcesoInfo> {
        &self.top_procesos_tiempo_cpu
    }
    // Devuelve la lista de procesos que más tiempo han estado en ejecución
    // Retorno
    // la lista de procesos que más tiempo han estado en ejecución como un vector de ProcesoInfo
    pub fn get_top_procesos_tiempo_ejecucion(&self) -> &Vec<ProcesoInfo> {
        &self.top_procesos_tiempo_ejecucion
    }
    
    // Constructor
    // Crea una nueva instancia de ProcesosInfo
    pub fn new() -> ProcesosInfo {
        let mut s = System::new_all();
        s.refresh_processes_specifics(
            ProcessesToUpdate::All,
            true,
            ProcessRefreshKind::everything(),
        );
        sleep(Duration::from_millis(5000));
        s.refresh_processes_specifics(
            ProcessesToUpdate::All,
            true,
            ProcessRefreshKind::everything(),
        );
        let mut procesos = Vec::<ProcesoInfo>::new();
        for (pid, process) in s.processes(){
            let proceso = ProcesoInfo {
                pid: pid.as_u32() as i32,
                nombre: process.name().to_string_lossy().into_owned(),
                tiempo_ejecucion: process.run_time(),
                tiempo_en_cpu: process.accumulated_cpu_time(),
                uso_cpu: process.cpu_usage(),
                uso_memoria: process.memory() / B_TO_MB,
                uso_memoria_virtual: process.virtual_memory() / B_TO_MB, //falla en macOs
                estado: process.status().to_string(),
            };
            procesos.push(proceso);
        }
        procesos.sort_by(|a, b| b.uso_cpu.partial_cmp(&a.uso_cpu).unwrap());
        let top_procesos_uso_cpu = procesos.iter().take(5).cloned().collect();
        procesos.sort_by(|a, b| b.uso_memoria.partial_cmp(&a.uso_memoria).unwrap());
        let top_procesos_uso_memoria = procesos.iter().take(5).cloned().collect();
        procesos.sort_by(|a, b| b.tiempo_en_cpu.partial_cmp(&a.tiempo_en_cpu).unwrap());
        let top_procesos_tiempo_cpu = procesos.iter().take(5).cloned().collect();
        procesos.sort_by(|a, b| b.tiempo_ejecucion.partial_cmp(&a.tiempo_ejecucion).unwrap());
        let top_procesos_tiempo_ejecucion = procesos.iter().take(5).cloned().collect();
        ProcesosInfo {
            cantidad_procesos: procesos.len(),
            procesos,
            top_procesos_uso_cpu,
            top_procesos_uso_memoria,
            top_procesos_tiempo_cpu,
            top_procesos_tiempo_ejecucion,
        }
    }
    pub fn mostrar_info(&self) {
        println!("Cantidad de procesos: {}", self.cantidad_procesos);
        println!("Top 5 procesos que consumen más CPU:");
        for proceso in &self.top_procesos_uso_cpu {
            proceso.mostrar_info();
        }
        println!("Top 5 procesos que consumen más memoria:");
        for proceso in &self.top_procesos_uso_memoria {
            proceso.mostrar_info();
        }
        println!("Top 5 procesos que más tiempo han estado en CPU:");
        for proceso in &self.top_procesos_tiempo_cpu {
            proceso.mostrar_info();
        }
        println!("Top 5 procesos que más tiempo han estado en ejecución:");
        for proceso in &self.top_procesos_tiempo_ejecucion {
            proceso.mostrar_info();
        }
    }
}

impl ProcesoInfo{
    //getters
    // Devuelve el pid
    // Retorno
    // el pid como un entero
    pub fn get_pid(&self) -> i32 {
        self.pid
    }
    // Devuelve el nombre
    // Retorno
    // el nombre como un string
    pub fn get_nombre(&self) -> &String {
        &self.nombre
    }
    // Devuelve el tiempo de ejecucion
    // Retorno
    // el tiempo de ejecucion como un entero
    pub fn get_tiempo_ejecucion(&self) -> u64 {
        self.tiempo_ejecucion
    }
    // Devuelve el tiempo en cpu
    // Retorno
    // el tiempo en cpu como un entero
    pub fn get_tiempo_en_cpu(&self) -> u64 {
        self.tiempo_en_cpu
    }
    // Devuelve el uso de cpu
    // Retorno
    // el uso de cpu como un flotante
    pub fn get_uso_cpu(&self) -> f32 {
        self.uso_cpu
    }
    // Devuelve el uso de memoria
    // Retorno
    // el uso de memoria como un entero
    pub fn get_uso_memoria(&self) -> u64 {
        self.uso_memoria
    }
    // Devuelve el uso de memoria virtual
    // Retorno
    // el uso de memoria virtual como un entero
    pub fn get_uso_memoria_virtual(&self) -> u64 {
        self.uso_memoria_virtual
    }
    // Devuelve el estado
    // Retorno
    // el estado como un string
    pub fn get_estado(&self) -> &String {
        &self.estado
    }
    pub fn formatear_tiempo(segundos: u64) -> String {
        let dias = segundos / 86400;
        let horas = (segundos % 86400) / 3600;
        let minutos = (segundos % 3600) / 60;
        let segundos_restantes = segundos % 60;
    
        format!(
            "{} días, {} horas, {} minutos, {} segundos",
            dias, horas, minutos, segundos_restantes
        )
    }   
    pub fn mostrar_info(&self) {
        println!("PID: {}", self.pid);
        println!("Nombre: {}", self.nombre);
        println!("Tiempo de ejecución: {} segundos", self.tiempo_ejecucion);
        println!("Tiempo en CPU: {} segundos", self.tiempo_en_cpu);
        println!("Uso de CPU: {:.2}%", self.uso_cpu);
        println!("Uso de memoria: {} MB", self.uso_memoria);
        println!("Uso de memoria virtual: {} MB", self.uso_memoria_virtual);
        println!("Estado: {}", self.estado);
    } 
}
/// Implementación del trait Display para la estructura `ProcesoInfo`.
/// Permite mostrar la información del proceso en un formato legible.
impl fmt::Display for ProcesoInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "PID: {}\nNombre: {}\nEstado: {}\nTiempo en ejecución: {}s\nTiempo en CPU: {}s\nUso de CPU: {:.2}%\nUso de Memoria: {} MB\nUso de Memoria Virtual: {} MB",
            self.pid,
            self.nombre,
            self.estado,
            self.tiempo_ejecucion,
            //ProcesoInfo::formatear_tiempo(self.tiempo_ejecucion),
            self.tiempo_en_cpu,
            self.uso_cpu,
            self.uso_memoria,
            self.uso_memoria_virtual
        )
    }
}

pub fn obtener_info_procesos() -> ProcesosInfo {
    ProcesosInfo::new()
}