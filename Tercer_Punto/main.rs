use std::time::Instant;

fn main() {
    // Datos de ejemplo
    let x = vec![1.0, 2.0, 3.0, 4.0, 5.0];
    let y = vec![2.0, 4.0, 6.0, 8.0, 10.0];

    // Inicialización de parámetros
    let mut w = 0.0_f64;
    let mut b = 0.0_f64;

    // Hiperparámetros
    let learning_rate = 0.01;
    let epochs = 1000;

    let m = x.len() as f64;

    // Medición de memoria inicial (solo para Unix)
    #[cfg(target_os = "linux")]
    {
        unsafe { libc::malloc_trim(0); } // liberar heap si usa glibc
    }
    let start_mem = get_current_memory_kb();

    // Medición de tiempo
    let start = Instant::now();

    for epoch in 1..=epochs {
        // Predicción
        let y_pred: Vec<f64> = x.iter().map(|&xi| w * xi + b).collect();

        // Error
        let error: Vec<f64> = y_pred.iter().zip(&y).map(|(yp, yt)| yp - yt).collect();

        // Gradientes
        let dw = 2.0 / m * error.iter().zip(&x).map(|(e, xi)| e * xi).sum::<f64>();
        let db = 2.0 / m * error.iter().sum::<f64>();

        // Actualización de parámetros
        w -= learning_rate * dw;
        b -= learning_rate * db;

        if epoch % 200 == 0 {
            let mse: f64 = error.iter().map(|e| e * e).sum::<f64>() / m;
            println!(
                "Epoch {}, MSE: {:.4}, w: {:.4}, b: {:.4}",
                epoch, mse, w, b
            );
        }
    }

    println!("\nModelo entrenado:");
    println!("w ≈ {:.4}, b ≈ {:.4}", w, b);

    // Probar el modelo
    let x_nuevo = 7.0;
    let y_pred_nuevo = w * x_nuevo + b;
    println!("Para x = {}, y_pred ≈ {:.4}", x_nuevo, y_pred_nuevo);

    // Tiempo final
    let duration = start.elapsed();

    // Medida de memoria final (en KB)
    let end_mem = get_current_memory_kb();
    println!("\nTiempo total: {:.6} segundos", duration.as_secs_f64());
    println!(
        "Memoria máxima usada: {:.2} MiB",
        (end_mem as f64 - start_mem as f64) / 1024.0
    );
}

#[cfg(target_os = "linux")]
fn get_current_memory_kb() -> u64 {
    // Lee el consumo de memoria del proceso desde /proc/self/statm
    use std::fs::File;
    use std::io::Read;
    let mut file = File::open("/proc/self/statm").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let mem: u64 = contents
        .split_whitespace()
        .next()
        .unwrap()
        .parse()
        .unwrap();
    // Páginas * tamaño de página en KB
    mem * (unsafe { libc::sysconf(libc::_SC_PAGESIZE) } as u64) / 1024
}

#[cfg(not(target_os = "linux"))]
fn get_current_memory_kb() -> u64 {
    // Para Windows/Mac: retorna 0 (requiere librerías externas para medición)
    0
}
