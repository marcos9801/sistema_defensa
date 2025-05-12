/// Módulo que implementa la obtención de la información de los discos:
/// - Cantidad de discos.
/// - Espacio total.
/// - Espacio libre.
/// - Espacio usado.
/// - Nombre del disco.
/// - Sistema de archivos.
/// - Ruta del disco.
/// - Indicador de si el disco es removible.
/// - Indicador de si el disco es de solo lectura.
///
/// Este módulo define dos estructuras principales:
///
/// ### `DiscosInfo`
/// Representa la información general de los discos del sistema.
/// - `cantidad_discos`: Número total de discos detectados.
/// - `espacio_total`: Espacio total de todos los discos en GB.
/// - `espacio_libre`: Espacio libre de todos los discos en GB.
/// - `espacio_usado`: Espacio usado de todos los discos en GB.
/// - `discos`: Lista de información detallada de cada disco (`Vec<DiscoInfo>`).
///
/// #### Métodos de `DiscosInfo`
/// - `get_cantidad_discos`: Devuelve la cantidad de discos.
/// - `get_espacio_total`: Devuelve el espacio total de los discos.
/// - `get_espacio_libre`: Devuelve el espacio libre de los discos.
/// - `get_espacio_usado`: Devuelve el espacio usado de los discos.
/// - `get_discos`: Devuelve la lista de discos.
/// - `get_disco`: Devuelve la información de un disco específico por índice.
/// - `new`: Constructor que crea una nueva instancia de `DiscosInfo` obteniendo la información del sistema.
///
/// ### `DiscoInfo`
/// Representa la información detallada de un disco específico.
/// - `nombre`: Nombre del disco.
/// - `sistema_archivos`: Sistema de archivos del disco.
/// - `espacio_total`: Espacio total del disco en GB.
/// - `espacio_libre`: Espacio libre del disco en GB.
/// - `espacio_usado`: Espacio usado del disco en GB.
/// - `ruta`: Ruta de montaje del disco.
/// - `removible`: Indica si el disco es removible.
/// - `solo_lectura`: Indica si el disco es de solo lectura.
///
/// #### Métodos de `DiscoInfo`
/// - `get_nombre`: Devuelve el nombre del disco.
/// - `get_sistema_archivos`: Devuelve el sistema de archivos del disco.
/// - `get_espacio_total`: Devuelve el espacio total del disco.
/// - `get_espacio_libre`: Devuelve el espacio libre del disco.
/// - `get_espacio_usado`: Devuelve el espacio usado del disco.
/// - `get_ruta`: Devuelve la ruta de montaje del disco.
/// - `get_removible`: Indica si el disco es removible.
/// - `get_solo_lectura`: Indica si el disco es de solo lectura.
/// - `new`: Constructor que crea una nueva instancia de `DiscoInfo` con los datos proporcionados.
///
/// ### Constantes
/// - `B_TO_GB`: Constante para la conversión de bytes a gigabytes.
///
/// ### Función principal
/// - `obtener_info_disco`: Devuelve una instancia de `DiscosInfo` con la información actual de los discos del sistema.
///
/// historial de cambios
/// - 2025-05-05: Creación del módulo y definición de las estructuras `DiscosInfo` y `DiscoInfo`.
/// - 2025-05-06: Implementación de métodos para obtener información de los discos.

/// TODO: obtener información de velocidad y tiempo de respuesta
/// TODO: espacio usado y libre en porcentaje
use serde::{Serialize};
use sysinfo::{Disks};

#[derive(Clone, Debug, Serialize)]
pub struct DiscosInfo {
    cantidad_discos: usize,
    espacio_total: f64,
    espacio_libre: f64,
    espacio_usado: f64,
    discos: Vec<DiscoInfo>,
}
#[derive(Clone, Debug, Serialize)]
pub struct DiscoInfo {
    nombre: String,
    sistema_archivos: String,
    espacio_total: f64,
    espacio_libre: f64,
    espacio_usado: f64,
    ruta: String,
    removible: bool,
    solo_lectura: bool,
    //velocidad: u64,
    //tiempo_respuesta: u64,
}

const B_TO_GB: u64 = 1024 * 1024 * 1024; // conversion de bytes a GB

impl DiscosInfo {
    // Getters
    // Devuelve la cantidad de discos
    // Retorno
    // la cantidad de discos como un entero
    pub fn get_cantidad_discos(&self) -> usize { self.cantidad_discos }
    // Devuelve el espacio total
    // Retorno
    // el espacio total como un flotante
    pub fn get_espacio_total(&self) -> f64 { self.espacio_total }
    // Devuelve el espacio libre
    // Retorno
    // el espacio libre como un flotante
    pub fn get_espacio_libre(&self) -> f64 { self.espacio_libre }
    // Devuelve el espacio usado
    // Retorno
    // el espacio usado como un flotante
    pub fn get_espacio_usado(&self) -> f64 { self.espacio_usado }
    // Devuelve la lista de discos
    // Retorno
    // la lista de discos como un vector de DiscoInfo
    pub fn get_discos(&self) -> &Vec<DiscoInfo> { &self.discos }
    // Devuelve el disco en la posicion index
    // Retorno
    // el disco en la posicion index como un DiscoInfo
    pub fn get_disco(&self, index: usize) -> &DiscoInfo {
        if index < self.discos.len() {
            &self.discos[index]
        } else {
            panic!("Índice fuera de rango");
        }
    }
    // Constructor
    // Crea una nueva instancia de DiscosInfo
    // Retorno
    pub fn new() -> Self {
        let disks = Disks::new_with_refreshed_list();
        let mut discos: Vec<DiscoInfo> = Vec::new();
        let mut total = 0.0;
        let mut cantidad = 0;
        let mut libre = 0.0;

        for disk in disks.list() {
            if discos.iter().any(|d| d.nombre == disk.name().to_string_lossy().to_string()) {
                continue;
            }
            cantidad += 1;
            let t = disk.total_space() / B_TO_GB;
            let l = disk.available_space() / B_TO_GB;
            total += t as f64;
            libre += l as f64;
            discos.push(DiscoInfo::new(
                disk.name().to_string_lossy().to_string(),
                disk.file_system().to_string_lossy().to_string(),
                t as f64,
                l as f64,
                (t - l) as f64,
                disk.mount_point().to_string_lossy().to_string(),
                disk.is_removable(),
                disk.is_read_only(),
            ));
        }
        

        DiscosInfo {
            cantidad_discos: cantidad,
            espacio_total: total,
            espacio_libre: libre,
            espacio_usado: total - libre,
            discos,
        }
    }
    pub fn mostrar_info(&self) {
        println!("Cantidad de discos: {}", self.cantidad_discos);
        println!("Espacio total: {} GB", self.espacio_total);
        println!("Espacio libre: {} GB", self.espacio_libre);
        println!("Espacio usado: {} GB", self.espacio_usado);
        for disco in &self.discos {
            println!("Disco: {}", disco.get_nombre());
            println!("Sistema de archivos: {}", disco.get_sistema_archivos());
            println!("Espacio total: {} GB", disco.get_espacio_total());
            println!("Espacio libre: {} GB", disco.get_espacio_libre());
            println!("Espacio usado: {} GB", disco.get_espacio_usado());
            println!("Ruta: {}", disco.get_ruta());
            println!("Removible: {}", disco.get_removible());
            println!("Solo lectura: {}", disco.get_solo_lectura());
        }
    }
}

/// Representa la información acerca de un disco.
///
/// Esta estructura proporciona varios detalles sobre un disco, como su nombre,
/// sistema de archivos, espacio total, espacio libre, espacio usado y otras propiedades.
impl DiscoInfo {
    /// Devuelve el nombre del disco.
    ///
    /// # Retorna
    /// Una cadena de texto que representa el nombre del disco.
    pub fn get_nombre(&self) -> &str { &self.nombre }

    /// Devuelve el sistema de archivos del disco.
    ///
    /// # Retorna
    /// Una cadena de texto que representa el sistema de archivos del disco.
    pub fn get_sistema_archivos(&self) -> &str { &self.sistema_archivos }

    /// Devuelve el espacio total del disco.
    ///
    /// # Retorna
    /// Un número de punto flotante que representa el espacio total del disco en bytes.
    pub fn get_espacio_total(&self) -> f64 { self.espacio_total }

    /// Devuelve el espacio libre del disco.
    ///
    /// # Retorna
    /// Un número de punto flotante que representa el espacio libre del disco en bytes.
    pub fn get_espacio_libre(&self) -> f64 { self.espacio_libre }

    /// Devuelve el espacio usado del disco.
    ///
    /// # Retorna
    /// Un número de punto flotante que representa el espacio usado del disco en bytes.
    pub fn get_espacio_usado(&self) -> f64 { self.espacio_usado }

    /// Devuelve la ruta del disco.
    ///
    /// # Retorna
    /// Una cadena de texto que representa la ruta del disco.
    pub fn get_ruta(&self) -> &str { &self.ruta }

    /// Indica si el disco es removible.
    ///
    /// # Retorna
    /// Un valor booleano:
    /// - `true` si el disco es removible.
    /// - `false` en caso contrario.
    pub fn get_removible(&self) -> bool { self.removible }

    /// Indica si el disco es de solo lectura.
    ///
    /// # Retorna
    /// Un valor booleano:
    /// - `true` si el disco es de solo lectura.
    /// - `false` en caso contrario.
    pub fn get_solo_lectura(&self) -> bool { self.solo_lectura }

    /// Crea una nueva instancia de `DiscoInfo`.
    ///
    /// # Parámetros
    /// - `nombre`: El nombre del disco como `String`.
    /// - `sistema_archivos`: El sistema de archivos del disco como `String`.
    /// - `espacio_total`: El espacio total del disco como `f64`.
    /// - `espacio_libre`: El espacio libre del disco como `f64`.
    /// - `espacio_usado`: El espacio usado del disco como `f64`.
    /// - `ruta`: La ruta del disco como `String`.
    /// - `removible`: Un valor booleano que indica si el disco es removible.
    /// - `solo_lectura`: Un valor booleano que indica si el disco es de solo lectura.
    ///
    /// # Retorna
    /// Una nueva instancia de `DiscoInfo`.
    pub fn new(nombre: String, sistema_archivos: String, espacio_total: f64, espacio_libre: f64, espacio_usado: f64, ruta: String, removible: bool, solo_lectura: bool) -> Self {
        DiscoInfo {
            nombre,
            sistema_archivos,
            espacio_total,
            espacio_libre,
            espacio_usado,
            ruta,
            removible,
            solo_lectura,
        }
    }
    
}
pub fn obtener_info_disco() -> DiscosInfo {
    DiscosInfo::new()
}
