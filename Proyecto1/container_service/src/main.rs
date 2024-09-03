use std::fs::File;
use std::io::{self, Read};
use std::path::Path;
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Local};
use reqwest::Client;
use std::error::Error;

#[derive(Debug, Serialize, Deserialize)]
struct SystemInfo {
    ram_total: f64,
    ram_free: f64,
    ram_usage: f64,
    #[serde(rename = "processes")]
    processes: Vec<Process>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct Process {
    #[serde(rename = "PID")]
    pid: u32,
    #[serde(rename = "Name")]
    name: String,
    #[serde(rename = "Cmdline")]
    cmd_line: String,
    #[serde(rename = "Vsz")]
    vsz: f64,
    #[serde(rename = "Rss")]
    rss: f64,
    #[serde(rename = "MemoryUsage")]
    memory_usage: f64,
    #[serde(rename = "CPUUsage")]
    cpu_usage: f64,
}

#[derive(Debug, Serialize, Clone)]
struct LogProcess {
    pid: u32,
    name: String,
    container_id: String,
    vsz:f64,
    rss:f64,
    memory_usage: f64,
    cpu_usage: f64,
    action: String,
    timestamp:String,
}

#[derive(Debug, Serialize, Clone)]
struct LogRam{
    total_ram: f64,
    free_ram: f64,
    usage_ram: f64,
    timestamp: String,
}

impl Process {
    fn get_container_id(&self) -> &str {
        let parts: Vec<&str> = self.cmd_line.split_whitespace().collect();
        for (i, part) in parts.iter().enumerate() {
            if *part == "-id" {
                if let Some(id) = parts.get(i + 1) {
                    return id;
                }
            }
        }
        "N/A"
    }
}

impl Eq for Process {}  

impl Ord for Process {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.cpu_usage.partial_cmp(&other.cpu_usage).unwrap_or(std::cmp::Ordering::Equal)
            .then_with(|| self.memory_usage.partial_cmp(&other.memory_usage).unwrap_or(std::cmp::Ordering::Equal))
    }
}

impl PartialOrd for Process {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn kill_container(id: &str) -> std::process::Output {
    let  output = std::process::Command::new("sudo")
        .arg("docker")
        .arg("rm")
        .arg("-f")
        .arg(id)
        .output()
        .expect("failed to execute process");

    println!("Matando contenedor con id: {}", id);

    output
}

fn read_proc_file(file_name: &str) -> io::Result<String> {
    // Se crea un Path con el nombre del archivo que se quiere leer.
    let path  = Path::new("/proc").join(file_name);

    /* 
        Se abre el archivo en modo lectura y se guarda en la variable file.
        En caso de que haya un error al abrir el archivo, se regresa un error.
        El signo de interrogación es un atajo para regresar un error en caso de que haya uno.
    */
    let mut file = File::open(path)?;

    // Se crea una variable mutable content que se inicializa con un String vacío.
    let mut content = String::new();

    // Se lee el contenido del archivo y se guarda en la variable content.
    file.read_to_string(&mut content)?;


    // Se regresa el contenido del archivo.
    Ok(content)
}

fn parse_proc_to_struct(json_str: &str) -> Result<SystemInfo, serde_json::Error> {
    // Se deserializa el contenido del archivo proc a un SystemInfo.
    let system_info: SystemInfo = serde_json::from_str(json_str)?;

    // Se regresa el SystemInfo.
    Ok(system_info)
}

#[tokio::main]
async fn enviar_json_logs(logs_procesess: &Vec<LogProcess>) -> Result<(), Box<dyn Error>> {
     // Crea un cliente HTTP
     let client = Client::new();

     
 
     // Realiza la petición POST
     let response = client.post("http://0.0.0.0:8000/logs") // URL del servidor
         .json(logs_procesess) // Convertir los datos a JSON
         .send() // Enviar la solicitud
         .await?; // Esperar la respuesta
 
     // Verifica el estado de la respuesta
     if response.status().is_success() {
         println!("¡Datos enviados exitosamente!");
     } else {
         println!("Error al enviar los datos: {:?}", response.status());
     }
 
     Ok(())
}

#[tokio::main]
async fn enviar_json_ram(log_ram: &Vec<LogRam>) -> Result<(), Box<dyn Error>> {
     // Crea un cliente HTTP
     let client = Client::new();

   
 
     // Realiza la petición POST
     let response = client
         .post("http://0.0.0.0:8000/memory") // URL del servidor
         .json(log_ram) // Convertir los datos a JSON
         .send() // Enviar la solicitud
         .await?; // Esperar la respuesta
 
     // Verifica el estado de la respuesta
     if response.status().is_success() {
         println!("¡Datos enviados exitosamente!");
     } else {
         println!("Error al enviar los datos: {:?}", response.status());
     }
 
     Ok(())
}

#[tokio::main]
async fn get_graph() -> Result<(), Box<dyn Error>> {
    // Crea un cliente HTTP
    let body = reqwest::get("http://0.0.0.0:8000/graph")
    .await?
    .text()
    .await?;

    println!("body = {body:?}");
 
    Ok(())
}

fn analyzer( system_info:  SystemInfo,id:&str) {


    // Creamos un vector vacío para guardar los logs de los procesos.
    let mut log_proc_list: Vec<LogProcess> = Vec::new();
    let mut log_ram_list: Vec<LogRam> = Vec::new();


    /* 
        Creamos un vector vacío para guardar los logs del sistema.
        En este caso, no se guardará nada, pero se puede modificar para guardar
        información del sistema.
    */
    let mut processes_list: Vec<Process> = system_info.processes;


    /* 
        Cuando llamas a la función sort en un vector de Process, se ejecutarán los traits 
        Ord y PartialOrd en el siguiente orden y con la siguiente funcionalidad:


        La función sort del vector llama internamente a partial_cmp para comparar los elementos.
        partial_cmp delega la comparación a cmp del trait Ord.


        Comparación con cmp:

        cmp compara primero el uso de CPU (cpu_usage).
        Si el uso de CPU es igual, compara el uso de memoria (memory_usage).
        Si ambos son iguales, devuelve Ordering::Equal.
        Funcionalidad de los Traits
        PartialOrd: Permite la comparación parcial, necesaria para manejar casos donde los valores pueden ser NaN.
        Ord: Proporciona una comparación total, necesaria para ordenar completamente los elementos del vector.

        Cuando llamas a processes_list.sort(), el método sort usará partial_cmp y cmp para comparar y 
        ordenar los procesos en el vector processes_list basándose en el uso de CPU y memoria.
    */
    processes_list.sort();


   

    println!("------------------------------");
    println!("Total Memoria {}, Memoria Usada {}, Memoria Libre {}", system_info.ram_total,system_info.ram_free,system_info.ram_usage);
    // Hacemos un print de los contenedores de bajo consumo en las listas.
    println!("------------------------------");
    println!("Total Memoria % 100.0, Memoria Usada % {}, Memoria Libre % {}", (system_info.ram_free/system_info.ram_total)*100.0,(system_info.ram_usage/system_info.ram_total)*100.0);
    println!("------------------------------");


    println!("Bajo Consumo");
    for i in 0..processes_list.len()/2{

        println!("PID: {}, Name: {}, container ID: {}, Memory Usage: {}, CPU Usage: {}", processes_list[i].pid, processes_list[i].name, processes_list[i].get_container_id(), processes_list[i].memory_usage, processes_list[i].cpu_usage);

    }

    println!("Alto Consumo");
    for i in processes_list.len()/2..processes_list.len(){

        println!("PID: {}, Name: {}, container ID: {}, Memory Usage: {}, CPU Usage: {}", processes_list[i].pid, processes_list[i].name, processes_list[i].get_container_id(), processes_list[i].memory_usage, processes_list[i].cpu_usage);

    }
    println!("------------------------------");

    //Obtener fecha y hora
    let ahora: DateTime<Local> = Local::now();

    // Formatear la fecha y hora
    let fecha_formateada = ahora.format("%Y-%m-%d %H:%M:%S").to_string();
    //println!("Fecha y hora formateada: {}", fecha_formateada);

    if processes_list.len() > 6{
        for i in 3..processes_list.len()-2{
            let id_proceso = &processes_list[i].get_container_id().to_string()[..12];
            //println!("{}",id_proceso);
            //println!("{}",id);
            if id_proceso == id{
                //println!("Container logs");
                continue;
            }

            let log_process = LogProcess {
                pid: processes_list[i].pid,
                container_id: processes_list[i].get_container_id().to_string(),
                name: processes_list[i].name.clone(),
                vsz: processes_list[i].vsz,
                rss: processes_list[i].rss,
                memory_usage: processes_list[i].memory_usage,
                cpu_usage: processes_list[i].cpu_usage,
                action: processes_list[i].cmd_line.clone(),
                timestamp: fecha_formateada.clone()
            };

            log_proc_list.push(log_process.clone());
            //println!("eliminar: {}",processes_list[i].get_container_id().to_string());
            let _output = kill_container(&processes_list[i].get_container_id());
            println!("{:?}",_output)
        }
    }
    

    let log_ram = LogRam{
        total_ram: system_info.ram_total,
        free_ram: system_info.ram_free,
        usage_ram: system_info.ram_usage,
        timestamp: fecha_formateada.clone()
    };
  
    log_ram_list.push(log_ram);

    if log_proc_list.len()>0{
        let _=enviar_json_logs(&log_proc_list);
    }
    // TODO: ENVIAR LOGS AL CONTENEDOR REGISTRO
    
    let _=enviar_json_ram(&log_ram_list);

    // Hacemos un print de los contenedores que matamos.
    println!("Contenedores matados");
    for process in log_proc_list {
        println!("PID: {}, Name: {}, Container ID: {},Vsz {},Rss {}, Memory Usage: {}, CPU Usage: {},action {}, timestamp {} ", process.pid, process.name, process.container_id,process.vsz,process.rss,  process.memory_usage, process.cpu_usage,process.action,process.timestamp);
    }

    println!("------------------------------");

    //let _=get_graph();
}

fn cleanup(){

    let _ = get_graph();

    let ruta = "../scripts/imagenLogs";

    let output = std::process::Command::new("sh")
        .arg("-c")
        .arg(format!("cd {} && sudo docker compose down", &ruta))
        .output()
        .expect("Fallo al ejecutar docker compose down");

    if output.status.success() {
        println!("Compose down");
    } else {
        eprintln!(
            "Error al ejecutar Docker Compose: {}",
            String::from_utf8_lossy(&output.stderr)
        );
    }


    
    println!("------------------------------");

}

fn main() {
    println!("------------------------------");

    let ruta = "../scripts/imagenLogs";
    let mut container_id= String::new();

    let output = std::process::Command::new("sh")
        .arg("-c")
        .arg(format!("cd {} && sudo docker compose up -d", &ruta))
        .output()
        .expect("Fallo al ejecutar docker compose up");

    if output.status.success() {
        println!("Docker Compose ejecutado correctamente.");
        
        // Ahora obtenemos la ID del contenedor
        let id_output = std::process::Command::new("sh")
            .arg("-c")
            .arg("sudo docker ps -q --filter  \"name=logs_container\"") // Reemplaza <imagen> con el nombre de la imagen usada en docker-compose.yml
            .output()
            .expect("Fallo al obtener la ID del contenedor");

        if id_output.status.success() {
            container_id = String::from_utf8_lossy(&id_output.stdout).trim().to_string();
            
           // println!("{}", container_id.trim());

        } else {
            eprintln!(
                "Error al obtener la ID del contenedor: {}",
                String::from_utf8_lossy(&id_output.stderr)
            );
        }
    } else {
        eprintln!(
            "Error al ejecutar Docker Compose: {}",
            String::from_utf8_lossy(&output.stderr)
        );
    }


    // TODO: antes de iniciar el loop, ejecutar el docker-compose.yml y obtener el id del contenedor registro.
    println!("------------------------------");
    // TODO: Utilizar algo para capturar la señal de terminación y matar el contenedor registro y cronjob.

    loop {
        
    // Creamos una estructura de datos SystemInfo con un vector de procesos vacío.
        let system_info: Result<SystemInfo, _>;

        // Leemos el contenido del archivo proc y lo guardamos en la variable json_str.
        let json_str = read_proc_file("sysinfo_202010044").unwrap();

        // Deserializamos el contenido del archivo proc a un SystemInfo.
        system_info = parse_proc_to_struct(&json_str);

        // Dependiendo de si se pudo deserializar el contenido del archivo proc o no, se ejecuta una u otra rama.
        match system_info {
            Ok( info) => {
                analyzer(info,&container_id);
            }
            Err(e) => println!("Failed to parse JSON: {}", e),
        }

        // Dormimos el hilo principal por 10 segundos.
        std::thread::sleep(std::time::Duration::from_secs(10));
        //break;
    }


   
}