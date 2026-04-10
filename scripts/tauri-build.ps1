param(
  [Parameter(ValueFromRemainingArguments = $true)]
  [string[]]$TauriArgs
)

$ErrorActionPreference = 'Stop'

$toolRoots = @(
  (Join-Path $env:LOCALAPPDATA 'tauri\WixTools314'),
  (Join-Path $env:LOCALAPPDATA 'tauri\NSIS'),
  (Join-Path $PSScriptRoot '..\src-tauri\target\.tauri\WixTools314'),
  (Join-Path $PSScriptRoot '..\src-tauri\target\.tauri\NSIS')
)

$existingToolRoots = $toolRoots | Where-Object { Test-Path $_ }
if ($existingToolRoots.Count -gt 0) {
  $env:PATH = (($existingToolRoots -join ';') + ';' + $env:PATH)
}

$isMsiBuild = $TauriArgs -contains 'msi'
if ($isMsiBuild) {
  $hasWix = (Get-Command candle.exe -ErrorAction SilentlyContinue) -and (Get-Command light.exe -ErrorAction SilentlyContinue)
  if (-not $hasWix) {
    throw 'WiX no está disponible localmente. Instale WiX Toolset 3.14 o copie la cache de Tauri antes de ejecutar un bundle MSI.'
  }
}

$tauriCli = Get-Command npx.cmd -ErrorAction SilentlyContinue
if (-not $tauriCli) {
  $tauriCli = Get-Command npx -ErrorAction SilentlyContinue
}

if (-not $tauriCli) {
  throw 'No se encontró npx en PATH. Ejecute npm install y vuelva a intentar.'
}

& $tauriCli.Path tauri build @TauriArgs
if ($LASTEXITCODE -ne 0) {
  exit $LASTEXITCODE
}