$ErrorActionPreference = 'Stop'

$repoRoot = Resolve-Path (Join-Path $PSScriptRoot '..')
$releaseDir = Join-Path $repoRoot 'src-tauri\target\release'
$portableRoot = Join-Path $releaseDir 'portable'
$portableDir = Join-Path $portableRoot 'pjupi-portable'
$portableZip = Join-Path $portableRoot 'pjupi-portable-win-x64.zip'

Write-Host 'Compilando ejecutable portable de PJUPI...'
& (Join-Path $PSScriptRoot 'tauri-build.ps1') --no-bundle
if ($LASTEXITCODE -ne 0) {
  exit $LASTEXITCODE
}

$exePath = Join-Path $releaseDir 'pjupi.exe'
if (-not (Test-Path $exePath)) {
  throw "No se encontró el ejecutable esperado en: $exePath"
}

if (Test-Path $portableDir) {
  Remove-Item $portableDir -Recurse -Force
}

New-Item -ItemType Directory -Path $portableDir -Force | Out-Null

Copy-Item $exePath (Join-Path $portableDir 'pjupi.exe') -Force

$pdbPath = Join-Path $releaseDir 'pjupi.pdb'
if (Test-Path $pdbPath) {
  Copy-Item $pdbPath (Join-Path $portableDir 'pjupi.pdb') -Force
}

$launcherPath = Join-Path $portableDir 'Iniciar PJUPI.cmd'
$launcherContent = @'
@echo off
setlocal
cd /d "%~dp0"
start "" "%~dp0pjupi.exe"
'@
Set-Content -Path $launcherPath -Value $launcherContent -Encoding ASCII

$readmePath = Join-Path $portableDir 'LEEME-PORTABLE.txt'
$readmeContent = @'
PJUPI - Portable para Windows

1. Extraiga esta carpeta completa en cualquier ubicacion donde el usuario tenga permisos.
2. Ejecute "Iniciar PJUPI.cmd" o "pjupi.exe".
3. No necesita instalar Rust, Node.js ni Visual Studio.
4. Si no abre en la PC destino, verifique que Microsoft Edge WebView2 Runtime este instalado.

Notas:
- La base SQLite local se guarda en %LOCALAPPDATA%\pjupi\database.db.
- Si desea usar MongoDB, configure las variables PJUPI_DB_BACKEND, PJUPI_MONGODB_URI y PJUPI_MONGODB_DB antes de iniciar.
- Para mover la app a otra PC, copie la carpeta o el ZIP completo.
'@
Set-Content -Path $readmePath -Value $readmeContent -Encoding ASCII

if (Test-Path $portableZip) {
  Remove-Item $portableZip -Force
}

Compress-Archive -Path (Join-Path $portableDir '*') -DestinationPath $portableZip -Force

Write-Host "Portable generado en: $portableDir"
Write-Host "ZIP generado en: $portableZip"