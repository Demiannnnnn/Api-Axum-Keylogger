#!/usr/bin/env fish

echo "🧹 LIMPIANDO TODOS LOS RASTROS DEL VIRUS..."
echo ""

# 1. Matar todos los procesos relacionados
echo "🔴 Matando procesos..."
pkill -f SystemHelper 2>/dev/null
pkill -f stage2 2>/dev/null
pkill -f System_Update 2>/dev/null
pkill -f system-helper 2>/dev/null
echo "✅ Procesos terminados"

# 2. Eliminar LaunchAgent (persistencia)
echo "🔴 Eliminando LaunchAgent..."
launchctl unload ~/Library/LaunchAgents/com.system.updater.plist 2>/dev/null
rm -f ~/Library/LaunchAgents/com.system.updater.plist
echo "✅ LaunchAgent eliminado"

# 3. Eliminar TODAS las copias del ejecutable
echo "🔴 Eliminando copias del ejecutable..."
find /Users/demian -name "SystemHelper" -type f 2>/dev/null -exec rm -f {} \;
find /Users/demian -name "system-helper" -type f 2>/dev/null -exec rm -f {} \;
find /Users/demian -name "System_Update*" -type f 2>/dev/null -exec rm -f {} \;
find /Users/demian -name "stage2" -type f 2>/dev/null -exec rm -f {} \;
rm -f ~/Desktop/System_Update.dmg
echo "✅ Copias eliminadas"

# 4. Eliminar archivos temporales y logs
echo "🔴 Eliminando logs y temporales..."
rm -f /tmp/systemhelper.log
rm -f /tmp/systemhelper.err
rm -f /tmp/stage2.log
echo "✅ Logs eliminados"

# 5. Limpiar caché de LaunchServices (opcional)
echo "🔴 Limpiando caché de LaunchServices..."
/System/Library/Frameworks/CoreServices.framework/Frameworks/LaunchServices.framework/Support/lsregister -kill -r -domain local -domain system -domain user 2>/dev/null
echo "✅ Caché limpiada"

echo ""
echo "═══════════════════════════════════════════════════════════════════════"
echo "✅ LIMPIEZA COMPLETADA"
echo "═══════════════════════════════════════════════════════════════════════"
echo ""

# 6. Verificar que no queda nada
echo "📋 VERIFICACIÓN:"
echo ""

set PROCESOS (ps aux | grep -E "SystemHelper|stage2|System_Update|system-helper" | grep -v grep | wc -l)
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
