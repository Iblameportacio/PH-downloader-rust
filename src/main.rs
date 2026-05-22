use clap::Parser;
use std::env;
use std::fs;
use std::io::{self, Write};
use std::path::PathBuf;
use std::process::Command;

#[derive(Parser, Debug)]
#[command(author, version, about = "Descargador de videos nativo")]
struct Args {
    /// URL del video a descargar
    #[arg(short, long)]
    url: Option<String>,

    /// Carpeta donde guardar el video
    #[arg(short, long)]
    output: Option<PathBuf>,

    /// Calidad del video (best, worst, etc.)
    #[arg(short, long, default_value = "best")]
    quality: String,
}

fn main() {
    // Capturamos los argumentos de la terminal
    let mut args = Args::parse();

    // Modo interactivo si no se pasa URL
    if args.url.is_none() {
        println!("=== Descargador de Videos para Estudio ===");

        let mut url_input = String::new();
        print!("Ingrese la URL del video: ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut url_input).unwrap();
        let url_input = url_input.trim().to_string();

        if url_input.is_empty() {
            eprintln!("Error: La URL no puede estar vacía.");
            return;
        }
        args.url = Some(url_input);

        let mut out_input = String::new();
        print!("Carpeta de destino (dejar en blanco para usar './descargas'): ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut out_input).unwrap();
        let out_input = out_input.trim();

        if !out_input.is_empty() {
            args.output = Some(PathBuf::from(out_input));
        }
    }

    let url = args.url.unwrap();
    let quality = args.quality;

    // Configurar ruta de salida
    let output_path = match args.output {
        Some(path) => path,
        None => {
            let mut current_dir = env::current_dir().unwrap();
            current_dir.push("descargas");
            current_dir
        }
    };

    // Crear la carpeta si no existe
    if let Err(e) = fs::create_dir_all(&output_path) {
        eprintln!("Error creando el directorio: {}", e);
        return;
    }

    // Construimos la plantilla del nombre del archivo dentro de la carpeta elegida
    let mut template = output_path.clone();
    template.push("%(title)s.%(ext)s");
    let template_str = template.to_string_lossy().into_owned();

    println!("Iniciando descarga desde: {}", url);
    println!("Guardando en: {}", output_path.display());
    println!("Calidad seleccionada: {}", quality);
    println!("--------------------------------------------------");

    // Llamamos directamente al binario de tu sistema operativo
    let status = Command::new("yt-dlp")
        .arg("-f")
        .arg(&quality)
        .arg("-o")
        .arg(&template_str)
        .arg("--no-playlist")
        .arg(&url)
        // Esto hereda los flujos de entrada/salida para que veas la barra de progreso real en tu shell
        .stdin(std::process::Stdio::inherit())
        .stdout(std::process::Stdio::inherit())
        .stderr(std::process::Stdio::inherit())
        .status(); // Ejecuta y bloquea el hilo hasta que termine la descarga real

    match status {
        Ok(exit_status) if exit_status.success() => {
            println!("\n--------------------------------------------------");
            println!("¡Proceso completado exitosamente!");
            println!("Revisa tu archivo en: {}", output_path.display());
        }
        Ok(exit_status) => {
            eprintln!("\n`yt-dlp` terminó con un código de error: {}", exit_status);
        }
        Err(e) => {
            eprintln!("\nNo se pudo ejecutar `yt-dlp`. ¿Seguro que está instalado? Error: {}", e);
        }
    }
}
