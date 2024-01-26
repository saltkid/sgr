$version = "0.1.0"

function Usage
{
    Write-Host "Usage: sift <optional-command> <optional-args>"
    Write-Host "Examples:"
    Write-Host "  sift"
    Write-Host "  sift add path\to\dir"
    Write-Host "  sift remove path\to\dir"
    Write-Host "  sift list"
}

function ListPaths
{
    param (
        [Parameter(Mandatory = $true)]
        [int] $start,
        [Parameter(Mandatory = $true)]
        [int] $end,
        [Parameter(Mandatory = $true)]
        [array] $configContents
    )
    Write-Host "---------------------------------------------------------"
    Write-Host "| idx | paths"
    Write-Host "---------------------------------------------------------"
    for ($i = $start; $i -lt $end; $i++)
    {
        Write-Host "|" "$($i + 1)".PadLeft(3)  "| $($configContents[$i])"
    }
    Write-Host "---------------------------------------------------------"
}

# check if dirs.txt exists first in script path 
if (!(Test-Path "$PSScriptRoot\dirs.txt"))
{
    # default dir
    "$env:userprofile\projects" | Out-File -FilePath "$PSScriptRoot\dirs.txt"
}

# main functionality: using fzf to find git repos
if ($args.Count -lt 1)
{
    $selectedRepo = Get-Content "$PSScriptRoot\dirs.txt" | ForEach-Object {
        # force because .git is a hidden directory
        Get-ChildItem -Path $_ -Filter "*.git" -Recurse -Directory -Force |
            ForEach-Object {
                $_.FullName -replace '\\\.git$'
            }
        } | fzf

    if (-not [string]::IsNullOrWhiteSpace($selectedRepo))
    {
        Set-Location $selectedRepo
        Write-Host "Selected: $selectedRepo"
    } else
    {
        Write-Host "No repository selected."
    }    exit 0
}

# put here the valid commands
$command = $args[0]
if (!($args[0] -eq "list") -and !($args[0] -eq "add") -and !($args[0] -eq "remove") -and !($args[0] -eq "help") -and !($args[0] -eq "version"))
{
    Write-Host "Invalid command: $command" -ForegroundColor Red
    Usage
    exit 1
}

# put here commands that allow no values
if (($args.Count -lt 2) -and !($command -eq "list") -and !($command -eq "help") -and !($command -eq "version"))
{
    Write-Host "Missing argument for command: $command" -ForegroundColor Red
    Usage
    exit 1
}

# extra args after first two will be unused, but don't stop the execution
if ($args.Count -gt 2)
{
    $unused = $args[2..($args.Count - 1)]
    Write-Host "'$unused' will be unused" -ForegroundColor Yellow
}

if ($args[0] -eq "add")
{
    Write-Host "Adding $($args[1])..."
    if (!(Test-Path $args[1]))
    {
        Write-Host "$($args[1]) does not exist" -ForegroundColor Red
        exit 1
    }
    $path = Resolve-Path $args[1]

    $configContents = Get-Content "$PSScriptRoot\dirs.txt"
    if ($configContents -contains $path)
    {
        Write-Host "$path already exists in $PSScriptRoot\dirs.txt" -ForegroundColor Red
        exit 1
    } else
    {
        $path | Add-Content "$PSScriptRoot\dirs.txt"
        Write-Host "$path added to $PSScriptRoot\dirs.txt"
        exit 0
    }

} elseif ($args[0] -eq "remove")
{
    Write-Host "Removing $($args[1])..."
    $configContents = Get-Content "$PSScriptRoot\dirs.txt"
    if (!(Test-Path $args[1]))
    {
        Write-Host "$($args[1]) does not exist" -ForegroundColor Red
        exit 1
    }

    $path = Resolve-Path $args[1]
    if ($configContents -contains $path)
    {
        $configContents = $configContents | Where-Object { $_ -ne $path }
        $configContents | Set-Content "$PSScriptRoot\dirs.txt"
        Write-Host "$path removed from $PSScriptRoot\dirs.txt"
        exit 0
    } else
    {
        Write-Host "Nothing to remove" -ForegroundColor Red
        Write-Host "$path does not exist in $PSScriptRoot\dirs.txt" -ForegroundColor Red
        exit 1
    }

} elseif ($args[0] -eq "list")
{
    $listArg = $args[1]

    $configContents = Get-Content "$PSScriptRoot\dirs.txt"

    if (($listArg -eq "all") -or (-not $listArg))
    {
        ListPaths -start 0 -end $configContents.Count -configContents $configContents

    } elseif ($listArg -match "^\d+$")
    {
        $index = [int]$listArg - 1
        if ($index -ge 0 -and $index -lt $configContents.Count)
        {
            ListPaths -start $index -end $($index+1) -configContents $configContents

        } elseif ($index -lt 0)
        {
            Write-Host "Index $index is out of range" -ForegroundColor Red
            exit 1
        } elseif ($index -ge $configContents.Count)
        {
            Write-Host "Index $index is out of range" -ForegroundColor Red
            exit 1
        } else
        {
            Write-Host "Unexpected error: index = $index" -ForegroundColor Red
            exit 1
        }
    } elseif ($listArg -match "^\d+-\d+$")
    {
        # assert that x should be less than y
        $x, $y = $listArg -split "-"
        $x, $y = [int]$x, [int]$y
        $x = $x - 1
        $y = $y - 1
        if ($x -ge $y)
        {
            Write-Host "Invalid range: $listArg; starting range '$x' should be less than ending range '$y'" -ForegroundColor Red
            exit 1
        } elseif ($x -ge $configContents.Count -or $x -lt 0 -or $y -ge $configContents.Count -or $y -lt 0)
        {
            Write-Host "Index $x and/or $y is out of range" -ForegroundColor Red
            exit 1
        }
        
        ListPaths -start $x -end $($y + 1) -configContents $configContents
    }
} elseif ($args[0] -eq "help")
{
    Usage
} elseif ($args[0] -eq "version")
{
    Write-Host "sift version $version"
}
exit 0
