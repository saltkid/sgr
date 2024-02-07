if (-not (Test-Path -Path "$PSScriptRoot\dirs.txt"))
{
    "$env:userprofile\projects" | Out-File -FilePath "$PSScriptRoot\dirs.txt" -Encoding utf8
}

if ($args.Count -eq 0)
{
    $res = & sugar
    if ($res.Count -eq 0)
    {
        exit 0
    }

    foreach ($r in $res)
    {
        if ($r -ne "")
        {
            $selectedPath = $r
            break
        }
    }

    $header = ($selectedPath -split " ")[0]
    if ($header -ne "`e[33m[WARN]`e[0m" -and $header -ne "`e[31m[ERROR]`e[0m")
    {
        Set-Location -Path $selectedPath
        "Changed Directory to ""$selectedPath"""
    } else
    {
        Write-Host $selectedPath
    }

} else
{
    & cargo run $args
}

