if (-not (Test-Path -Path "$PSScriptRoot\dirs.txt"))
{
    "$env:userprofile\projects" | Out-File -FilePath "$PSScriptRoot\dirs.txt" -Encoding utf8
}

if ($args.Count -eq 0)
{
    $res = & cargo run
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

    if ($selectedPath -ne "")
    {
        Set-Location -Path $selectedPath
        "Changed Directory to ""$selectedPath"""
    }

} else
{
    & cargo run $args
}

