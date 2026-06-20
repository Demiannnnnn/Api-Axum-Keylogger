#!/bin/bash

echo "🚀 Compilando Stage 1 para todos los SO..."

# 1. Limpiar builds anteriores
rm -rf dist/
mkdir -p dist

# 2. Compilar para Windows (x86_64)
echo "📦 Compilando para Windows..."
GOOS=windows GOARCH=amd64 go build -ldflags="-s -w" -o dist/stage1_windows.exe main.go

# 3. Compilar para macOS (x86_64 e ARM64)
echo "📦 Compilando para macOS (x86_64)..."
GOOS=darwin GOARCH=amd64 go build -ldflags="-s -w" -o dist/stage1_macos_x86_64 main.go

echo "📦 Compilando para macOS (ARM64)..."
GOOS=darwin GOARCH=arm64 go build -ldflags="-s -w" -o dist/stage1_macos_arm64 main.go

# 4. Compilar para Linux (x86_64)
echo "📦 Compilando para Linux..."
GOOS=linux GOARCH=amd64 go build -ldflags="-s -w" -o dist/stage1_linux main.go

# 5. Mostrar resultados
echo ""
echo "✅ ¡Todos los ejecutables listos!"
echo ""
echo "📁 ARCHIVOS GENERADOS:"
ls -la dist/
echo ""
echo "📤 Sube los siguientes archivos a VirusTotal:"
echo "   - Windows: dist/stage1_windows.exe"
echo "   - macOS:   dist/stage1_macos_x86_64"
echo "   - macOS:   dist/stage1_macos_arm64"
echo "   - Linux:   dist/stage1_linux"
