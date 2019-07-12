# This script takes care of packaging the build artifacts that will go in the
# release zipfile

$SRC_DIR = $PWD.Path
$STAGE = [System.Guid]::NewGuid().ToString()

Set-Location $ENV:Temp
New-Item -Type Directory -Name $STAGE
Set-Location $STAGE

$ZIP = "$SRC_DIR\vdot-$($Env:APPVEYOR_REPO_TAG_NAME)-$($Env:TARGET).zip"

Copy-Item "$SRC_DIR\target\$($Env:TARGET)\release\vdot.exe" '.\'
Copy-Item "$SRC_DIR\target\$($Env:TARGET)\release\vproc.exe" '.\'
Copy-Item "$SRC_DIR\README.md" '.\'
Copy-Item "$SRC_DIR\LICENSE" '.\'

7z a "$ZIP" *

Push-AppveyorArtifact "$ZIP"

Set-Location $SRC_DIR
