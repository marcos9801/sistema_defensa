/// Módulo que implementa la obtención de la información del procesador (CPU):
/// - Marca del CPU.
/// - Temperatura del CPU (pendiente de implementación).
/// - Cantidad de núcleos físicos.
/// - Frecuencia del CPU.
/// - Porcentaje de uso por núcleo.
///
/// Este módulo define una estructura principal:
///
/// ### `CPUInfo`
/// Representa la información detallada del procesador del sistema.
/// - `brand`: Marca del CPU.
/// - `temperatura`: Temperatura actual del CPU (valor por defecto, aún no implementado).
/// - `cantidad_nucleos`: Número total de núcleos físicos del CPU.
/// - `frecuencia`: Frecuencia del CPU en MHz.
/// - `uso_nucleos`: Lista con el porcentaje de uso de cada núcleo (`Vec<f32>`).
///
/// *Campos comentados para futura implementación:*
/// - `cantidad_nucleos_e`: Cantidad de núcleos de eficiencia.
/// - `cantidad_nucleos_p`: Cantidad de núcleos de rendimiento.
/// - `nucleos_logicos`: Número total de núcleos lógicos.
///
/// #### Métodos de `CPUInfo`
/// - `get_brand`: Devuelve la marca del CPU.
/// - `get_temperatura`: Devuelve la temperatura del CPU (valor fijo por ahora).
/// - `get_cantidad_nucleos`: Devuelve la cantidad de núcleos físicos del CPU.
/// - `get_frecuencia`: Devuelve la frecuencia del CPU en MHz.
/// - `get_uso_nucleos`: Devuelve un vector con el uso de cada núcleo en porcentaje.
/// - `new`: Constructor que crea una nueva instancia de `CPUInfo` obteniendo la información del sistema.
///
/// ### Función principal
/// - `obtener_info_cpu`: Devuelve una instancia de `CPUInfo` con la información actual del CPU del sistema.
///
/// ### Constantes/Futuros desarrollos
/// - Pendiente: Implementación de lectura de temperatura y diferenciación entre núcleos E/P.
///
/// Historial de cambios
/// - 2025-05-05: Creación del módulo y definición de la estructura `CPUInfo`.
/// - 2025-05-06: Implementacion de metodo get_info() para obtener la información del CPU.
/// - 2025-05-09: implementacion de funcion apra detectar temperatura pr4ocesadores Intel en Linux

use serde::Serialize;   
use std::process::Command; // Para ejecutar el comando `sensors`
use sysinfo::{System, RefreshKind, CpuRefreshKind};

#[derive(Clone, Debug, Serialize)]
pub struct CPUInfo {
    brand: String,
    cantidad_nucleos: usize,
    //cantidad_nucleos_e: usize,
    //cantidad_nucleos_p: usize,
    //nucleos_logicos: usize,
    frecuencia: u64,
    uso_nucleos: Vec<f32>,
}

impl CPUInfo {
    // Getters
    // Devuelve la marca del CPU
    // Retorno
    // la marca del CPU como una cadena de caracteres
    pub fn get_brand(&self) -> &str {&self.brand}
    // Devuelve la temperatura del CPU
    // Retorno
    // la temperatura del CPU como un flotante
    //pub fn get_temperatura(&self) -> &Vec<String> {&self.temperatura}
    // Devuelve la cantidad de nucleos del CPU
    // Retorno
    // la cantidad de nucleos del CPU como un entero
    pub fn get_cantidad_nucleos(&self) -> usize {self.cantidad_nucleos}
    /*
    pub fn get_cantidad_nucleos_e(&self) -> usize {self.cantidad_nucleos_e}
    pub fn get_cantidad_nucleos_p(&self) -> usize {self.cantidad_nucleos_p}
    pub fn get_nucleos_logicos(&self) -> usize {self.nucleos_logicos}
    */
    // Devuelve la frecuencia del CPU
    // Retorno
    // la frecuencia del CPU como un entero
    pub fn get_frecuencia(&self) -> u64 {self.frecuencia}
    // Devuelve el uso de los nucleos del CPU
    // Retorno
    // el uso de los nucleos del CPU como un vector de flotantes
    pub fn get_uso_nucleos(&self) -> &Vec<f32> {&self.uso_nucleos}

    // Constructor
    // Crea una nueva instancia de CPUInfo
    // Retorno
    // una nueva instancia de CPUInfo
    pub fn new() -> Self {
        let mut s = System::new_with_specifics(RefreshKind::nothing().with_cpu(CpuRefreshKind::everything())); //unicamnete refrescar la CPU
        std::thread::sleep(sysinfo::MINIMUM_CPU_UPDATE_INTERVAL); // MINIMUM_CPU_UPDATE_INTERVAL es definido en sysinfo valor por defecto 100ms
        s.refresh_cpu_all();
        Self {
            brand: s.cpus()[0].brand().to_string(),
            cantidad_nucleos: s.cpus().len(),
            //cantidad_nucleos_e: s.cpus().iter().filter(|cpu| cpu.is_stepping()).count(),
            //cantidad_nucleos_p: s.cpus().iter().filter(|cpu| cpu.is_pstate()).count(),
            //nucleos_logicos: s.cpus().iter().map(|cpu| cpu.logical_count()).sum(),
            frecuencia: s.cpus()[0].frequency(),
            uso_nucleos: s.cpus().iter().map(|cpu| cpu.cpu_usage()).collect(),
        }
    }
    pub fn mostrar_info(&self) {
        println!("Marca del CPU: {}", self.get_brand());
        println!("Temperatura del CPU:");
        println!("Cantidad de núcleos: {}", self.get_cantidad_nucleos());
        //println!("Cantidad de núcleos E: {}", self.get_cantidad_nucleos_e());
        //println!("Cantidad de núcleos P: {}", self.get_cantidad_nucleos_p());
        //println!("Núcleos lógicos: {}", self.get_nucleos_logicos());
        println!("Frecuencia del CPU: {} MHz", self.get_frecuencia());
        println!("Uso de los núcleos:");
        for (i, uso) in self.uso_nucleos.iter().enumerate() {
            println!("Núcleo {}: {:.2} %", i, uso);
        }
    }

    fn obtener_temperaturas() -> Result<Vec<String>, String> {
        // Intentar ejecutar el comando `sensors`
        let output = Command::new("sensors")
            .output()
            .map_err(|e| format!("Error al ejecutar `sensors`: {}", e))?;
    
        // Verificar si la salida no está vacía y convertirla a String
        if !output.stdout.is_empty() {
            let salida = String::from_utf8_lossy(&output.stdout);
            let mut temperaturas = Vec::new();
    
            // Procesar cada línea en la salida
            for linea in salida.lines() {
                if linea.contains("Core") && linea.contains("°C") {
                    if let Some((core, temp)) = Self::extraer_temperatura(linea) {
                        temperaturas.push(format!("{} => {} °C", core, temp));
                    }
                }
            }
            
            // Si no se encuentra ninguna temperatura
            if temperaturas.is_empty() {
                return Err("No se encontraron temperaturas en la salida de `sensors`".to_string());
            }
            
            Ok(temperaturas)
        } else {
            Err("La salida de `sensors` está vacía".to_string())
        }
    }

    fn extraer_temperatura(linea: &str) -> Option<(&str, f32)> {
        let partes: Vec<&str> = linea.split_whitespace().collect();
        if partes.len() < 3 {
            return None;
        }
        let core = partes[0];
        let temp_str = partes[2].replace("°C", "");
        if let Ok(temp) = temp_str.parse::<f32>() {
            Some((core, temp))
        } else {
            None
        }
    }
    }


// Función auxiliar para obtener la información
pub fn obtener_info_cpu() -> CPUInfo {
    CPUInfo::new()
}
