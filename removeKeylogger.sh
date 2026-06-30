#!/usr/bin/env fish

echo "🧹 LIMPIANDO TODOS LOS RASTROS DEL VIRUS (INCLUYENDO PERMISOS TCC)..."
echo ""

# ==================== 1. MATAR PROCESOS ====================
echo "🔴 Matando procesos..."
pkill -f SystemHelper 2>/dev/null
pkill -f stage2 2>/dev/null
pkill -f System_Update 2>/dev/null
pkill -f system-helper 2>/dev/null
pkill -f MinecraftLauncher 2>/dev/null
echo "✅ Procesos terminados"

# ==================== 2. ELIMINAR LAUNCHAGENTS ====================
echo "🔴 Eliminando LaunchAgents..."
set -l agents ~/Library/LaunchAgents/com.system.updater.plist
for agent in $agents
    launchctl unload $agent 2>/dev/null
    rm -f $agent
end
# También buscar otros posibles plist con nombres similares
find ~/Library/LaunchAgents -name "*system*" -o -name "*updater*" 2>/dev/null | while read f
    launchctl unload $f 2>/dev/null
    rm -f $f
end
echo "✅ LaunchAgents eliminados"

# ==================== 3. ELIMINAR COPIAS DEL EJECUTABLE ====================
echo "🔴 Eliminando copias del ejecutable..."
# Buscar por nombres exactos y variantes
find /Users/demian -name "SystemHelper" -type f 2>/dev/null -exec rm -f {} \;
find /Users/demian -name "system-helper" -type f 2>/dev/null -exec rm -f {} \;
find /Users/demian -name "System_Update*" -type f 2>/dev/null -exec rm -f {} \;
find /Users/demian -name "stage2" -type f 2>/dev/null -exec rm -f {} \;
find /Users/demian -name "MinecraftLauncher*" -type f 2>/dev/null -exec rm -f {} \;
# Eliminar también .dmg
find /Users/demian -name "*.dmg" -type f 2>/dev/null | grep -i minecraft | while read f
    rm -f $f
end
# Eliminar posibles copias en /tmp
rm -f /tmp/SystemHelper /tmp/system-helper /tmp/stage2 /tmp/MinecraftLauncher 2>/dev/null
# Eliminar en ~/.config
rm -f ~/.config/SystemHelper ~/.config/system-helper ~/.config/stage2 2>/dev/null
# Eliminar en ~/.local y ~/.cache
rm -f ~/.local/share/SystemHelper ~/.cache/SystemHelper ~/.system/SystemHelper 2>/dev/null
# Eliminar en ~/Library
rm -f ~/Library/Application\ Support/SystemHelper ~/Library/Preferences/SystemHelper ~/Library/Caches/SystemHelper 2>/dev/null
# Eliminar el archivo del Desktop
rm -f ~/Desktop/System_Update.dmg ~/Desktop/MinecraftLauncher.dmg 2>/dev/null
echo "✅ Copias eliminadas"

# ==================== 4. ELIMINAR LOGS Y TEMPORALES ====================
echo "🔴 Eliminando logs y temporales..."
rm -f /tmp/systemhelper.log /tmp/systemhelper.err /tmp/stage2.log /tmp/systemhelper.out 2>/dev/null
rm -f ~/Library/Logs/SystemHelper.log 2>/dev/null
echo "✅ Logs eliminados"

# ==================== 5. ELIMINAR REGISTROS TCC (ACCESIBILIDAD) ====================
echo "🔴 Eliminando permisos de Accesibilidad (TCC)..."
# Esto elimina TODOS los permisos de accesibilidad concedidos a cualquier aplicación.
# Es drástico pero garantiza que no quede rastro de SystemHelper.
sudo tccutil reset Accessibility
# Si quieres ser más específico, puedes usar:
# sudo tccutil reset Accessibility com.system.updater  (si tienes bundle ID)
echo "✅ Permisos TCC eliminados (requiere reinicio para aplicar completamente)"

# ==================== 6. LIMPIAR CACHÉ DE LAUNCHSERVICES ====================
echo "🔴 Limpiando caché de LaunchServices..."
/System/Library/Frameworks/CoreServices.framework/Frameworks/LaunchServices.framework/Support/lsregister -kill -r -domain local -domain system -domain user 2>/dev/null
echo "✅ Caché limpiada"

# ==================== 7. LIMPIAR DYLD CACHE (opcional) ====================
# No es necesario normalmente, pero por si acaso:
# sudo update_dyld_shared_cache -force 2>/dev/null

echo ""
echo "═══════════════════════════════════════════════════════════════════════"
echo "✅ LIMPIEZA COMPLETADA"
echo "═══════════════════════════════════════════════════════════════════════"
echo ""
echo "⚠️  IMPORTANTE:"
echo "   - Los permisos de Accesibilidad se han resetado. Reinicia el sistema para que los cambios surtan efecto."
echo "   - Después del reinicio, si deseas volver a usar el keylogger, deberás conceder permisos de nuevo."
echo ""

# ==================== VERIFICACIÓN ====================
echo "📋 VERIFICACIÓN:"
echo ""

set PROCESOS (ps aux | grep -E "SystemHelper|stage2|System_Update|system-helper|MinecraftLauncher" | grep -v grep | wc -l)
set ARCHIVOS (find /Users/demian -name "SystemHelper" -type f 2>/dev/null | wc -l)
set LAUNCH (test -f ~/Library/LaunchAgents/com.system.updater.plist && echo "EXISTE" || echo "NO EXISTE")

echo "   Procesos activos: $PROCESOS (debe ser 0)"
echo "   Archivos encontrados: $ARCHIVOS (debe ser 0)"
echo "   LaunchAgent: $LAUNCH (debe ser NO EXISTE)"

if test $PROCESOS -eq 0 -a $ARCHIVOS -eq 0
    echo ""
    echo "✨ ¡Sistema completamente limpio!"
else
    echo ""
    echo "⚠️  Algo quedó. Revisa manualmente:"
    echo "   - Procesos: ps aux | grep -E 'SystemHelper|stage2'"
    echo "   - Archivos: find /Users/demian -name 'SystemHelper'"
end

echo ""
echo "🔍 Para limpiar manualmente los permisos de Accesibilidad (si no quieres resetear todo):"
echo "   1. Abre Ajustes del Sistema → Privacidad y Seguridad → Accesibilidad."
echo "   2. Busca entradas relacionadas con 'SystemHelper', 'stage2' o 'MinecraftLauncher'."
echo "   3. Selecciona cada una y haz clic en el botón '-' para eliminarlas."
echo "   4. Cierra la ventana y reinicia el sistema para que los cambios se apliquen."
