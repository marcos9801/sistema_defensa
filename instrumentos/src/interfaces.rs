/// Módulo que implementa la obtención de la información de las interfaces de red del sistema.
/// Incluye detalles como:
/// - Cantidad de interfaces.
/// - Estadísticas de tráfico (bytes y paquetes transmitidos/recibidos).
/// - Número total de errores.
/// - Direcciones IP y MAC por interfaz.
/// - MTU de cada interfaz.
///
/// Este módulo define dos estructuras principales:
///
/// ### `InterfacesInfo`
/// Representa un resumen de todas las interfaces de red del sistema.
/// - `cantidad_interfaces`: Número total de interfaces detectadas.
/// - `interfaces`: Vector con la información detallada de cada interfaz (`Vec<InterfaceInfo>`).
/// - `total_errores`: Total de errores en todas las interfaces.
/// - `total_bytes_recibidos`: Total de bytes recibidos.
/// - `total_bytes_enviados`: Total de bytes enviados.
/// - `total_paquetes_recibidos`: Total de paquetes recibidos.
/// - `total_paquetes_enviados`: Total de paquetes enviados.
/// - `total_direcciones_ip`: Total de direcciones IP encontradas.
/// - `total_direcciones_mac`: Total de direcciones MAC encontradas.
/// - `total_mtu`: Suma de los valores MTU de todas las interfaces.
///
/// #### Métodos de `InterfacesInfo`
/// - `new`: Constructor principal que obtiene la información desde el sistema.
/// - `desde_sistema`: Crea una instancia a partir de una referencia a `Networks`.
/// - Métodos *getter* para acceder a cada uno de los campos anteriores.
///
/// ### `InterfaceInfo`
/// Representa la información específica de una sola interfaz de red.
/// - `nombre`: Nombre de la interfaz (por ejemplo, "en0").
/// - `bytes_recibidos` / `bytes_enviados`: Total de bytes procesados.
/// - `numero_paquetes_recibidos` / `numero_paquetes_enviados`: Total de paquetes.
/// - `total_errores`: Total de errores combinados.
/// - `total_errores_recibidos` / `total_errores_enviados`: Errores al recibir/enviar datos.
/// - `direccion_ip`: Vector de direcciones IP asociadas (`Vec<IpNetwork>`).
/// - `direccion_mac`: Dirección MAC de la interfaz.
/// - `mtu`: Unidad máxima de transmisión.
///
/// #### Métodos de `InterfaceInfo`
/// - `new`: Constructor vacío.
/// - `desde_sistema`: Crea una instancia con datos del sistema.
/// - Métodos *getter* para acceder a cada uno de los campos.
///
/// ### Función principal
/// - `obtener_info_interfaces`: Devuelve una instancia de `InterfacesInfo` con la información actual del sistema.
///
/// historial de cambios
/// - 2025-04-06: Creación del módulo y definición de estructuras `InterfacesInfo` y `InterfaceInfo`, junto con sus métodos y metodo para mostrar informacion.

use sysinfo::{Networks, IpNetwork};
use serde::Serialize;


#[derive(Clone, Debug, Serialize)]
pub struct InterfacesInfo {
    cantidad_interfaces: u64,
    interfaces: Vec<InterfaceInfo>,
    total_errores: u64,
    total_bytes_recibidos: u64,
    total_bytes_enviados: u64,
    total_paquetes_recibidos: u64,
    total_paquetes_enviados: u64,
    total_direcciones_ip: u64,
    total_direcciones_mac: u64,
    total_mtu: u64,
}

#[derive(Clone, Debug, Serialize)]
pub struct InterfaceInfo {
    nombre: String,
    bytes_recibidos: u64,
    bytes_enviados: u64,
    numero_paquetes_recibidos: u64,
    numero_paquetes_enviados: u64,
    total_errores: u64,
    total_errores_recibidos: u64,
    total_errores_enviados: u64,
    direccion_ip: Vec<String>,
    direccion_mac: String,
    mtu: u64, // tamaño máximo de la unidad de transmisión
}
/// Representa información sobre las interfaces de red en el sistema.
impl InterfacesInfo {
    // Getters
    /// Devuelve la cantidad de interfaces de red.
    ///
    /// # Retorno
    /// La cantidad de interfaces como un `u64`.
    pub fn get_cantidad_interfaces(&self) -> u64 { self.cantidad_interfaces }
    /// Devuelve un vector de información de las interfaces de red.
    ///
    /// # Retorno
    /// Una referencia a un vector de `InterfaceInfo`.
    pub fn get_interfaces(&self) -> &Vec<InterfaceInfo> { &self.interfaces }
    /// Devuelve el total de errores en todas las interfaces.
    ///
    /// # Retorno
    /// El número total de errores como un `u64`.
    pub fn get_total_errores(&self) -> u64 { self.total_errores }
    /// Devuelve el total de bytes recibidos en todas las interfaces.
    ///
    /// # Retorno
    /// El número total de bytes recibidos como un `u64`.
    pub fn get_bytes_recibidos(&self) -> u64 { self.total_bytes_recibidos }
    /// Devuelve el total de bytes enviados en todas las interfaces.
    ///
    /// # Retorno
    /// El número total de bytes enviados como un `u64`.
    pub fn get_bytes_enviados(&self) -> u64 { self.total_bytes_enviados }
    /// Devuelve el total de paquetes recibidos en todas las interfaces.
    ///
    /// # Retorno
    /// El número total de paquetes recibidos como un `u64`.
    pub fn get_numero_paquetes_recibidos(&self) -> u64 { self.total_paquetes_recibidos }
    /// Devuelve el total de paquetes enviados en todas las interfaces.
    ///
    /// # Retorno
    /// El número total de paquetes enviados como un `u64`.
    pub fn get_numero_paquetes_enviados(&self) -> u64 { self.total_paquetes_enviados }
    /// Devuelve el total de direcciones IP en todas las interfaces.
    ///
    /// # Retorno
    /// El número total de direcciones IP como un `u64`.
    pub fn get_direccion_ip(&self) -> u64 { self.total_direcciones_ip }
    /// Devuelve el total de direcciones MAC en todas las interfaces.
    ///
    /// # Retorno
    /// El número total de direcciones MAC como un `u64`.
    pub fn get_direccion_mac(&self) -> u64 { self.total_direcciones_mac }
    /// Devuelve el MTU (Unidad Máxima de Transmisión) de todas las interfaces.
    ///
    /// # Retorno
    /// El valor del MTU como un `u64`.
    pub fn get_mtu(&self) -> u64 { self.total_mtu }
    /// Devuelve información sobre una interfaz de red específica.
    ///
    /// # Parámetros
    /// - `index`: El índice de la interfaz en el vector.
    ///
    /// # Retorno
    /// Una referencia a la información de la interfaz como un `InterfaceInfo`.
    pub fn get_interfaz(&self, index: u64) -> &InterfaceInfo {
        if index < self.interfaces.len().try_into().unwrap() {
            &self.interfaces[index as usize]
        } else {
            panic!("Índice fuera de rango");
        }
    }

    /// Crea una nueva instancia de `InterfacesInfo` con datos del sistema.
    pub fn new() -> Self {
        let interfaces = Networks::new_with_refreshed_list();
        Self::desde_sistema(&interfaces)
    }

    /// Crea una instancia de `InterfacesInfo` a partir de datos del sistema.
    ///
    /// # Parámetros
    /// - `interfaces`: Referencia a las interfaces de red del sistema.
    ///
    /// # Retorno
    /// Una nueva instancia de `InterfacesInfo`.
    pub fn desde_sistema(interfaces: &Networks) -> Self {
        let mut cantidad_interfaces = 0;
        let mut total_errores = 0;
        let mut total_bytes_recibidos = 0;
        let mut total_bytes_enviados = 0;
        let mut total_paquetes_recibidos = 0;
        let mut total_paquetes_enviados = 0;
        let mut total_direcciones_ip = 0;
        let mut total_direcciones_mac = 0;
        let mut total_mtu = 0;

        let mut interfaces_vec = Vec::new();
        for (interface_name, network) in interfaces {
            cantidad_interfaces += 1;
            total_errores += network.total_errors_on_received() + network.total_errors_on_transmitted();
            total_bytes_recibidos += network.total_received();
            total_bytes_enviados += network.total_transmitted();
            total_paquetes_recibidos += network.total_packets_received();
            total_paquetes_enviados += network.total_packets_transmitted();
            total_direcciones_ip += network.ip_networks().len() as u64;
            if network.mac_address().to_string() != "00:00:00:00:00:00" {
                total_direcciones_mac += 1;
            }
            total_mtu += network.mtu();
            let direccion_ip = network
                .ip_networks()
                .iter()
                .map(|ip| ip.to_string()) // convierte IpNetwork a String
                .collect::<Vec<String>>();

            let interface = InterfaceInfo::desde_sistema(
                interface_name.to_string(),
                network.total_received(),
                network.total_transmitted(),
                network.total_packets_received(),
                network.total_packets_transmitted(),
                network.total_errors_on_received() + network.total_errors_on_transmitted(),
                network.total_errors_on_received(),
                network.total_errors_on_transmitted(),
                direccion_ip, // <-- aquí cambias esto
                network.mac_address().to_string(),
                network.mtu(),
            );
            interfaces_vec.push(interface);
                    }

        InterfacesInfo {
            cantidad_interfaces,
            interfaces: interfaces_vec,
            total_errores,
            total_bytes_recibidos,
            total_bytes_enviados,
            total_paquetes_recibidos,
            total_paquetes_enviados,
            total_direcciones_ip,
            total_direcciones_mac,
            total_mtu,
        }
    }
    pub fn mostrar_info(&self) {
        println!("Cantidad de interfaces: {}", self.cantidad_interfaces);
        println!("Total de errores: {}", self.total_errores);
        println!("Bytes recibidos: {}", self.total_bytes_recibidos);
        println!("Bytes enviados: {}", self.total_bytes_enviados);
        println!("Paquetes recibidos: {}", self.total_paquetes_recibidos);
        println!("Paquetes enviados: {}", self.total_paquetes_enviados);
        println!("Direcciones IP: {}", self.total_direcciones_ip);
        println!("Direcciones MAC: {}", self.total_direcciones_mac);
        println!("MTU total: {}", self.total_mtu);
        for interface in &self.interfaces {
            interface.mostrar_info();
        }
        println!("-----------------------------------");
    }
}

impl InterfaceInfo {
    /// Devuelve el nombre de la interfaz de red.
    ///
    /// # Retorno
    /// Una referencia al nombre de la interfaz como un `String`.
    pub fn get_nombre(&self) -> &String { &self.nombre }

    /// Devuelve el total de bytes recibidos por la interfaz.
    ///
    /// # Retorno
    /// El número de bytes recibidos como un `u64`.
    pub fn get_bytes_recibidos(&self) -> u64 { self.bytes_recibidos }

    /// Devuelve el total de bytes enviados por la interfaz.
    ///
    /// # Retorno
    /// El número de bytes enviados como un `u64`.
    pub fn get_bytes_enviados(&self) -> u64 { self.bytes_enviados }

    /// Devuelve el total de paquetes recibidos por la interfaz.
    ///
    /// # Retorno
    /// El número de paquetes recibidos como un `u64`.
    pub fn get_numero_paquetes_recibidos(&self) -> u64 { self.numero_paquetes_recibidos }

    /// Devuelve el total de paquetes enviados por la interfaz.
    ///
    /// # Retorno
    /// El número de paquetes enviados como un `u64`.
    pub fn get_numero_paquetes_enviados(&self) -> u64 { self.numero_paquetes_enviados }

    /// Devuelve el total de errores en la interfaz.
    ///
    /// # Retorno
    /// El número total de errores como un `u64`.
    pub fn get_total_errores(&self) -> u64 { self.total_errores }

    /// Devuelve el total de errores recibidos por la interfaz.
    ///
    /// # Retorno
    /// El número de errores recibidos como un `u64`.
    pub fn get_total_errores_recibidos(&self) -> u64 { self.total_errores_recibidos }

    /// Devuelve el total de errores enviados por la interfaz.
    ///
    /// # Retorno
    /// El número de errores enviados como un `u64`.
    pub fn get_total_errores_enviados(&self) -> u64 { self.total_errores_enviados }

    /// Devuelve las direcciones IP asociadas a la interfaz.
    ///
    /// # Retorno
    /// Una referencia a un vector de `IpNetwork`.
    pub fn get_direccion_ip(&self) -> &Vec<String> { &self.direccion_ip }

    /// Devuelve la dirección MAC de la interfaz.
    ///
    /// # Retorno
    /// Una referencia a la dirección MAC como un `String`.
    pub fn get_direccion_mac(&self) -> &String { &self.direccion_mac }

    /// Devuelve el MTU (Unidad Máxima de Transmisión) de la interfaz.
    ///
    /// # Retorno
    /// El valor del MTU como un `u64`.
    pub fn get_mtu(&self) -> u64 { self.mtu }

    /// Crea una nueva instancia vacía de `InterfaceInfo`.
    pub fn new() -> Self {
        InterfaceInfo {
            nombre: String::new(),
            bytes_recibidos: 0,
            bytes_enviados: 0,
            numero_paquetes_recibidos: 0,
            numero_paquetes_enviados: 0,
            total_errores: 0,
            total_errores_recibidos: 0,
            total_errores_enviados: 0,
            direccion_ip: Vec::<String>::new(),
            direccion_mac: String::new(),
            mtu: 0,
        }
    }
    /// Crea una nueva instancia de `InterfaceInfo` a partir de datos del sistema.
    ///
    /// # Parámetros
    /// - `nombre`: Nombre de la interfaz.
    /// - `bytes_recibidos`: Total de bytes recibidos.
    /// - `bytes_enviados`: Total de bytes enviados.
    /// - `numero_paquetes_recibidos`: Total de paquetes recibidos.
    /// - `numero_paquetes_enviados`: Total de paquetes enviados.
    /// - `total_errores`: Total de errores.
    /// - `total_errores_recibidos`: Total de errores recibidos.
    /// - `total_errores_enviados`: Total de errores enviados.
    /// - `direccion_ip`: Vector de direcciones IP asociadas.
    /// - `direccion_mac`: Dirección MAC de la interfaz.
    /// - `mtu`: MTU de la interfaz.
    ///
    /// # Retorno
    /// Una nueva instancia de `InterfaceInfo`.
    pub fn desde_sistema(
        nombre: String,
        bytes_recibidos: u64,
        bytes_enviados: u64,
        numero_paquetes_recibidos: u64,
        numero_paquetes_enviados: u64,
        total_errores: u64,
        total_errores_recibidos: u64,
        total_errores_enviados: u64,
        direccion_ip: Vec<String>,
        direccion_mac: String,
        mtu: u64,
    ) -> Self {
        InterfaceInfo {
            nombre,
            bytes_recibidos,
            bytes_enviados,
            numero_paquetes_recibidos,
            numero_paquetes_enviados,
            total_errores,
            total_errores_recibidos,
            total_errores_enviados,
            direccion_ip,
            direccion_mac,
            mtu,
        }
    }
    pub fn mostrar_info(&self) {
        println!("Nombre: {}", self.nombre);
        println!("Bytes recibidos: {}", self.bytes_recibidos);
        println!("Bytes enviados: {}", self.bytes_enviados);
        println!("Paquetes recibidos: {}", self.numero_paquetes_recibidos);
        println!("Paquetes enviados: {}", self.numero_paquetes_enviados);
        println!("Total de errores: {}", self.total_errores);
        println!("Errores recibidos: {}", self.total_errores_recibidos);
        println!("Errores enviados: {}", self.total_errores_enviados);
        println!("Direcciones IP: {:?}", self.direccion_ip);
        println!("Dirección MAC: {}", self.direccion_mac);
        println!("MTU: {}", self.mtu);
    }
}

/// Obtiene información sobre las interfaces de red del sistema.
///
/// # Retorno
/// Una instancia de `InterfacesInfo` con los datos de las interfaces.
pub fn obtener_info_interfaces() -> InterfacesInfo {
    InterfacesInfo::new()
}
