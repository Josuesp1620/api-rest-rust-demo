use std::process::Command;
use rand::Rng;
use std::fs::File;
use std::io::{self, Write};
use std::path::Path;

pub fn generate_random_password(length: u16) -> Option<String> {
    let output = Command::new("sh")
        .arg("-c")
        .arg(format!("openssl rand -base64 {}", length))
        .output();

    match output {
        Ok(result) => {
            if result.status.success() {
                let password = String::from_utf8_lossy(&result.stdout).trim().to_string();
                Some(password)
            } else {
                None
            }
        },
        Err(_) => None,
    }
}

/// Verifica si un puerto está en uso en el sistema mediante `netstat`.
fn is_port_in_use_system(port: u16) -> bool {
    let output = Command::new("sh")
        .arg("-c")
        .arg(format!("netstat -an | grep -q ':{} '", port))
        .output();

    match output {
        Ok(result) => result.status.success(),
        Err(_) => false,
    }
}

/// Verifica si un puerto está en uso por Docker.
fn is_port_in_use_docker(port: u16) -> bool {
    let output = Command::new("sh")
        .arg("-c")
        .arg(format!("docker ps --format '{{{{.Ports}}}}' | grep -q '{}'", port))
        .output();

    match output {
        Ok(result) => result.status.success(),
        Err(_) => false,
    }
}

/// Genera un puerto aleatorio dentro de un rango especificado.
fn generate_random_port(min: u16, max: u16) -> u16 {
    let mut rng = rand::thread_rng();
    rng.gen_range(min..=max)
}

/// Busca un puerto disponible dentro de un rango especificado.
pub fn find_available_port(min: u16, max: u16) -> u16 {
    let mut port = generate_random_port(min, max);

    while is_port_in_use_system(port) || is_port_in_use_docker(port) {
        port = generate_random_port(min, max);
    }

    port
}

fn create_file(path: &str, filename: &str, content: &str) -> io::Result<()> {
    // Construir la ruta completa del archivo
    let file_path = Path::new(path).join(filename);

    // Crear o abrir el archivo (esto lo hace en modo escritura)
    let mut file = File::create(file_path)?;

    // Escribir el contenido en el archivo
    file.write_all(content.as_bytes())?;

    // Retornar Ok si todo fue exitoso
    Ok(())
}


// fn main() {
//     // Generar un puerto aleatorio para PostgreSQL y Odoo
//     let odoo_postgres_port = find_available_port(5000, 5999); // Puerto entre 5000 y 5999
//     let odoo_web_port = find_available_port(8000, 8999);      // Puerto entre 8000 y 8999

//     println!("Puerto PostgreSQL para Odoo: {}", odoo_postgres_port);
//     println!("Puerto Web para Odoo: {}", odoo_web_port);

//     match generate_random_password(24) {
//         Some(password) => println!("Contraseña generada: {}", password),
//         None => println!("Error al generar la contraseña."),
//     }

//     // Definir los parámetros
//     let path = "./";
//     let filename = ".env";
//     let content = r#"# Docker Container Settings
//     DOCKER_CONTAINER_NETWORK=$DOCKER_CONTAINER_NETWORK
//     DOCKER_CONTAINER_NAME_DATABASE=$DOCKER_CONTAINER_NAME_DATABASE
//     DOCKER_CONTAINER_NAME_WEB=$DOCKER_CONTAINER_NAME_WEB
    
//     # PostgreSQL settings
//     ODOO_POSTGRES_PORT=$ODOO_POSTGRES_PORT
//     ODOO_POSTGRES_USER=$ODOO_POSTGRES_USER
//     ODOO_POSTGRES_PASSWORD=$POSTGRES_PASSWORD
    
//     # App-specific settings
//     ODOO_WEB_VERSION=$ODOO_WEB_VERSION
//     ODOO_WEB_PORT=$ODOO_WEB_PORT
    
//     # Local File Sync
//     APP_DATA_DIR=$APP_DATA_DIR"#;
    
//     // Llamar a la función para crear el archivo
//     match create_file(path, filename, content) {
//         Ok(_) => println!("El archivo se ha creado exitosamente."),
//         Err(e) => println!("Error al crear el archivo: {}", e),
//     }
// }
