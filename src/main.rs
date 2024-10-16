use std::process::Command;
use std::thread;
use std::time::Duration;
use colored::*; // Usamos la crate `colored` para manejar los colores en la terminal

// Función para obtener el voltaje usando el comando `vcgencmd`
fn get_voltage() -> f32 {
    let output = Command::new("vcgencmd")
        .arg("measure_volts")
        .output()
        .expect("Failed to execute command");

    let output_str = String::from_utf8_lossy(&output.stdout);
    
    // Parseamos el valor de voltaje de la salida del comando
    let voltage_str = output_str.trim().replace("volt=", "").replace("V", "");
    voltage_str.parse::<f32>().unwrap_or(0.0) // Si hay error, devuelve 0.0
}

fn print_voltage_table(detected_voltage: f32, recommended_voltage: f32) {
    // Decidir el color en base al voltaje detectado
    let color = if detected_voltage == recommended_voltage {
        "green"
    } else if detected_voltage < recommended_voltage {
        "red"
    } else {
        "yellow"
    };

    // Mostrar la tabla con colores
    println!(
        "+-----------------------------------------+");
    println!(
        "|         Voltage Monitor                 |");
    println!(
        "+--------------------+--------------------+");
    println!(
        "| Detected Voltage    | Recommended (5.0V) |");
    println!(
        "+--------------------+--------------------+");

    // Mostrar el voltaje detectado con el color adecuado
    let detected_voltage_str = format!("{:.2} V", detected_voltage);

    match color {
        "green" => println!("|       {}       |       5.00 V       |", detected_voltage_str.green()),
        "red" => println!("|       {}       |       5.00 V       |", detected_voltage_str.red()),
        "yellow" => println!("|       {}       |       5.00 V       |", detected_voltage_str.yellow()),
        _ => println!("|       {}       |       5.00 V       |", detected_voltage_str),
    }

    println!(
        "+--------------------+--------------------+");
}

fn main() {
    let recommended_voltage: f32 = 5.0;

    // Bucle infinito para monitorear el voltaje constantemente
    loop {
        let detected_voltage = get_voltage();
        // Limpiar la consola antes de actualizar
        print!("\x1B[2J\x1B[1;1H");

        // Imprimir la tabla con el voltaje detectado
        print_voltage_table(detected_voltage, recommended_voltage);

        // Esperar 2 segundos antes de la próxima actualización
        thread::sleep(Duration::from_secs(2));
    }
}

