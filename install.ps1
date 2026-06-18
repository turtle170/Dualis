$InstallDir = "$env:LOCALAPPDATA\Dualis"
$ModelsDir = "$InstallDir\models"

if (!(Test-Path $InstallDir)) {
    New-Item -ItemType Directory -Force -Path $InstallDir
}
if (!(Test-Path $ModelsDir)) {
    New-Item -ItemType Directory -Force -Path $ModelsDir
}

Write-Host "Downloading Dualis Models..." -ForegroundColor Green
$ModelUrl = "https://huggingface.co/unsloth/gemma-4-E4B-it-GGUF/resolve/main/gemma-4-E4B-it-UD-Q4_K_XL.gguf"
$ProjUrl = "https://huggingface.co/unsloth/gemma-4-E4B-it-GGUF/resolve/main/mmproj-F16.gguf"

if (!(Test-Path "$ModelsDir\gemma-4-E4B-it-UD-Q4_K_XL.gguf")) {
    Invoke-WebRequest -Uri $ModelUrl -OutFile "$ModelsDir\gemma-4-E4B-it-UD-Q4_K_XL.gguf"
} else {
    Write-Host "Model gemma-4-E4B already downloaded."
}

if (!(Test-Path "$ModelsDir\mmproj-F16.gguf")) {
    Invoke-WebRequest -Uri $ProjUrl -OutFile "$ModelsDir\mmproj-F16.gguf"
} else {
    Write-Host "Model mmproj already downloaded."
}

Write-Host "Downloading Dualis Binary..." -ForegroundColor Green
$ReleaseUrl = "https://github.com/turtle170/Dualis/releases/latest/download/dualis.exe"
try {
    Invoke-WebRequest -Uri $ReleaseUrl -OutFile "$InstallDir\dualis.exe"
} catch {
    Write-Host "Failed to download Dualis.exe (release might not exist yet)." -ForegroundColor Red
}

$WshShell = New-Object -comObject WScript.Shell
$Shortcut = $WshShell.CreateShortcut("$env:APPDATA\Microsoft\Windows\Start Menu\Programs\Dualis.lnk")
$Shortcut.TargetPath = "$InstallDir\dualis.exe"
$Shortcut.WorkingDirectory = $InstallDir
$Shortcut.Save()

Write-Host "Dualis Installed successfully. You can launch it from the Start Menu." -ForegroundColor Green
