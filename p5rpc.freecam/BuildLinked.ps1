# Set Working Directory
Split-Path $MyInvocation.MyCommand.Path | Push-Location
[Environment]::CurrentDirectory = $PWD

Remove-Item "$env:RELOADEDIIMODS/p5rpc.freecam/*" -Force -Recurse
dotnet publish "./p5rpc.freecam.csproj" -c Release -o "$env:RELOADEDIIMODS/p5rpc.freecam" /p:OutputPath="./bin/Release" /p:ReloadedILLink="true"

# Restore Working Directory
Pop-Location