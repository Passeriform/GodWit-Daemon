
using namespace System.Management.Automation
using namespace System.Management.Automation.Language

Register-ArgumentCompleter -Native -CommandName 'godwit-daemon' -ScriptBlock {
    param($wordToComplete, $commandAst, $cursorPosition)

    $commandElements = $commandAst.CommandElements
    $command = @(
        'godwit-daemon'
        for ($i = 1; $i -lt $commandElements.Count; $i++) {
            $element = $commandElements[$i]
            if ($element -isnot [StringConstantExpressionAst] -or
                $element.StringConstantType -ne [StringConstantType]::BareWord -or
                $element.Value.StartsWith('-')) {
                break
        }
        $element.Value
    }) -join ';'

    $completions = @(switch ($command) {
        'godwit-daemon' {
            [CompletionResult]::new('-q', 'q', [CompletionResultType]::ParameterName, 'Silence all output')
            [CompletionResult]::new('--quiet', 'quiet', [CompletionResultType]::ParameterName, 'Silence all output')
            [CompletionResult]::new('-v', 'v', [CompletionResultType]::ParameterName, 'Debug mode')
            [CompletionResult]::new('--verbose', 'verbose', [CompletionResultType]::ParameterName, 'Debug mode')
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Prints help information')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Prints help information')
            [CompletionResult]::new('-V', 'V', [CompletionResultType]::ParameterName, 'Prints version information')
            [CompletionResult]::new('--version', 'version', [CompletionResultType]::ParameterName, 'Prints version information')
            [CompletionResult]::new('new', 'new', [CompletionResultType]::ParameterValue, 'Start new task and schedule')
            [CompletionResult]::new('regress', 'regress', [CompletionResultType]::ParameterValue, 'Regress operation until killsignal sent')
            [CompletionResult]::new('die', 'die', [CompletionResultType]::ParameterValue, 'Send killsignal')
            [CompletionResult]::new('help', 'help', [CompletionResultType]::ParameterValue, 'Prints this message or the help of the given subcommand(s)')
            break
        }
        'godwit-daemon;new' {
            [CompletionResult]::new('-r', 'r', [CompletionResultType]::ParameterName, 'Discard all previous diffs')
            [CompletionResult]::new('--refresh', 'refresh', [CompletionResultType]::ParameterName, 'Discard all previous diffs')
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Prints help information')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Prints help information')
            [CompletionResult]::new('-V', 'V', [CompletionResultType]::ParameterName, 'Prints version information')
            [CompletionResult]::new('--version', 'version', [CompletionResultType]::ParameterName, 'Prints version information')
            [CompletionResult]::new('-q', 'q', [CompletionResultType]::ParameterName, 'Silence all output')
            [CompletionResult]::new('--quiet', 'quiet', [CompletionResultType]::ParameterName, 'Silence all output')
            [CompletionResult]::new('-v', 'v', [CompletionResultType]::ParameterName, 'Debug mode')
            [CompletionResult]::new('--verbose', 'verbose', [CompletionResultType]::ParameterName, 'Debug mode')
            [CompletionResult]::new('trace', 'trace', [CompletionResultType]::ParameterValue, 'Trace an application state')
            [CompletionResult]::new('split', 'split', [CompletionResultType]::ParameterValue, 'Split processess')
            [CompletionResult]::new('help', 'help', [CompletionResultType]::ParameterValue, 'Prints this message or the help of the given subcommand(s)')
            break
        }
        'godwit-daemon;new;trace' {
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Prints help information')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Prints help information')
            [CompletionResult]::new('-V', 'V', [CompletionResultType]::ParameterName, 'Prints version information')
            [CompletionResult]::new('--version', 'version', [CompletionResultType]::ParameterName, 'Prints version information')
            [CompletionResult]::new('-r', 'r', [CompletionResultType]::ParameterName, 'Discard all previous diffs')
            [CompletionResult]::new('--refresh', 'refresh', [CompletionResultType]::ParameterName, 'Discard all previous diffs')
            [CompletionResult]::new('-q', 'q', [CompletionResultType]::ParameterName, 'Silence all output')
            [CompletionResult]::new('--quiet', 'quiet', [CompletionResultType]::ParameterName, 'Silence all output')
            [CompletionResult]::new('-v', 'v', [CompletionResultType]::ParameterName, 'Debug mode')
            [CompletionResult]::new('--verbose', 'verbose', [CompletionResultType]::ParameterName, 'Debug mode')
            break
        }
        'godwit-daemon;new;split' {
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Prints help information')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Prints help information')
            [CompletionResult]::new('-V', 'V', [CompletionResultType]::ParameterName, 'Prints version information')
            [CompletionResult]::new('--version', 'version', [CompletionResultType]::ParameterName, 'Prints version information')
            [CompletionResult]::new('-r', 'r', [CompletionResultType]::ParameterName, 'Discard all previous diffs')
            [CompletionResult]::new('--refresh', 'refresh', [CompletionResultType]::ParameterName, 'Discard all previous diffs')
            [CompletionResult]::new('-q', 'q', [CompletionResultType]::ParameterName, 'Silence all output')
            [CompletionResult]::new('--quiet', 'quiet', [CompletionResultType]::ParameterName, 'Silence all output')
            [CompletionResult]::new('-v', 'v', [CompletionResultType]::ParameterName, 'Debug mode')
            [CompletionResult]::new('--verbose', 'verbose', [CompletionResultType]::ParameterName, 'Debug mode')
            break
        }
        'godwit-daemon;new;help' {
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Prints help information')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Prints help information')
            [CompletionResult]::new('-V', 'V', [CompletionResultType]::ParameterName, 'Prints version information')
            [CompletionResult]::new('--version', 'version', [CompletionResultType]::ParameterName, 'Prints version information')
            [CompletionResult]::new('-r', 'r', [CompletionResultType]::ParameterName, 'Discard all previous diffs')
            [CompletionResult]::new('--refresh', 'refresh', [CompletionResultType]::ParameterName, 'Discard all previous diffs')
            [CompletionResult]::new('-q', 'q', [CompletionResultType]::ParameterName, 'Silence all output')
            [CompletionResult]::new('--quiet', 'quiet', [CompletionResultType]::ParameterName, 'Silence all output')
            [CompletionResult]::new('-v', 'v', [CompletionResultType]::ParameterName, 'Debug mode')
            [CompletionResult]::new('--verbose', 'verbose', [CompletionResultType]::ParameterName, 'Debug mode')
            break
        }
        'godwit-daemon;regress' {
            [CompletionResult]::new('-r', 'r', [CompletionResultType]::ParameterName, 'Discard all previous diffs')
            [CompletionResult]::new('--refresh', 'refresh', [CompletionResultType]::ParameterName, 'Discard all previous diffs')
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Prints help information')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Prints help information')
            [CompletionResult]::new('-V', 'V', [CompletionResultType]::ParameterName, 'Prints version information')
            [CompletionResult]::new('--version', 'version', [CompletionResultType]::ParameterName, 'Prints version information')
            [CompletionResult]::new('-q', 'q', [CompletionResultType]::ParameterName, 'Silence all output')
            [CompletionResult]::new('--quiet', 'quiet', [CompletionResultType]::ParameterName, 'Silence all output')
            [CompletionResult]::new('-v', 'v', [CompletionResultType]::ParameterName, 'Debug mode')
            [CompletionResult]::new('--verbose', 'verbose', [CompletionResultType]::ParameterName, 'Debug mode')
            [CompletionResult]::new('trace', 'trace', [CompletionResultType]::ParameterValue, 'Trace an application state')
            [CompletionResult]::new('split', 'split', [CompletionResultType]::ParameterValue, 'Split processess')
            [CompletionResult]::new('help', 'help', [CompletionResultType]::ParameterValue, 'Prints this message or the help of the given subcommand(s)')
            break
        }
        'godwit-daemon;regress;trace' {
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Prints help information')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Prints help information')
            [CompletionResult]::new('-V', 'V', [CompletionResultType]::ParameterName, 'Prints version information')
            [CompletionResult]::new('--version', 'version', [CompletionResultType]::ParameterName, 'Prints version information')
            [CompletionResult]::new('-r', 'r', [CompletionResultType]::ParameterName, 'Discard all previous diffs')
            [CompletionResult]::new('--refresh', 'refresh', [CompletionResultType]::ParameterName, 'Discard all previous diffs')
            [CompletionResult]::new('-q', 'q', [CompletionResultType]::ParameterName, 'Silence all output')
            [CompletionResult]::new('--quiet', 'quiet', [CompletionResultType]::ParameterName, 'Silence all output')
            [CompletionResult]::new('-v', 'v', [CompletionResultType]::ParameterName, 'Debug mode')
            [CompletionResult]::new('--verbose', 'verbose', [CompletionResultType]::ParameterName, 'Debug mode')
            break
        }
        'godwit-daemon;regress;split' {
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Prints help information')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Prints help information')
            [CompletionResult]::new('-V', 'V', [CompletionResultType]::ParameterName, 'Prints version information')
            [CompletionResult]::new('--version', 'version', [CompletionResultType]::ParameterName, 'Prints version information')
            [CompletionResult]::new('-r', 'r', [CompletionResultType]::ParameterName, 'Discard all previous diffs')
            [CompletionResult]::new('--refresh', 'refresh', [CompletionResultType]::ParameterName, 'Discard all previous diffs')
            [CompletionResult]::new('-q', 'q', [CompletionResultType]::ParameterName, 'Silence all output')
            [CompletionResult]::new('--quiet', 'quiet', [CompletionResultType]::ParameterName, 'Silence all output')
            [CompletionResult]::new('-v', 'v', [CompletionResultType]::ParameterName, 'Debug mode')
            [CompletionResult]::new('--verbose', 'verbose', [CompletionResultType]::ParameterName, 'Debug mode')
            break
        }
        'godwit-daemon;regress;help' {
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Prints help information')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Prints help information')
            [CompletionResult]::new('-V', 'V', [CompletionResultType]::ParameterName, 'Prints version information')
            [CompletionResult]::new('--version', 'version', [CompletionResultType]::ParameterName, 'Prints version information')
            [CompletionResult]::new('-r', 'r', [CompletionResultType]::ParameterName, 'Discard all previous diffs')
            [CompletionResult]::new('--refresh', 'refresh', [CompletionResultType]::ParameterName, 'Discard all previous diffs')
            [CompletionResult]::new('-q', 'q', [CompletionResultType]::ParameterName, 'Silence all output')
            [CompletionResult]::new('--quiet', 'quiet', [CompletionResultType]::ParameterName, 'Silence all output')
            [CompletionResult]::new('-v', 'v', [CompletionResultType]::ParameterName, 'Debug mode')
            [CompletionResult]::new('--verbose', 'verbose', [CompletionResultType]::ParameterName, 'Debug mode')
            break
        }
        'godwit-daemon;die' {
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Prints help information')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Prints help information')
            [CompletionResult]::new('-V', 'V', [CompletionResultType]::ParameterName, 'Prints version information')
            [CompletionResult]::new('--version', 'version', [CompletionResultType]::ParameterName, 'Prints version information')
            [CompletionResult]::new('-q', 'q', [CompletionResultType]::ParameterName, 'Silence all output')
            [CompletionResult]::new('--quiet', 'quiet', [CompletionResultType]::ParameterName, 'Silence all output')
            [CompletionResult]::new('-v', 'v', [CompletionResultType]::ParameterName, 'Debug mode')
            [CompletionResult]::new('--verbose', 'verbose', [CompletionResultType]::ParameterName, 'Debug mode')
            [CompletionResult]::new('trace', 'trace', [CompletionResultType]::ParameterValue, 'Trace an application state')
            [CompletionResult]::new('split', 'split', [CompletionResultType]::ParameterValue, 'Split processess')
            [CompletionResult]::new('help', 'help', [CompletionResultType]::ParameterValue, 'Prints this message or the help of the given subcommand(s)')
            break
        }
        'godwit-daemon;die;trace' {
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Prints help information')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Prints help information')
            [CompletionResult]::new('-V', 'V', [CompletionResultType]::ParameterName, 'Prints version information')
            [CompletionResult]::new('--version', 'version', [CompletionResultType]::ParameterName, 'Prints version information')
            [CompletionResult]::new('-q', 'q', [CompletionResultType]::ParameterName, 'Silence all output')
            [CompletionResult]::new('--quiet', 'quiet', [CompletionResultType]::ParameterName, 'Silence all output')
            [CompletionResult]::new('-v', 'v', [CompletionResultType]::ParameterName, 'Debug mode')
            [CompletionResult]::new('--verbose', 'verbose', [CompletionResultType]::ParameterName, 'Debug mode')
            break
        }
        'godwit-daemon;die;split' {
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Prints help information')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Prints help information')
            [CompletionResult]::new('-V', 'V', [CompletionResultType]::ParameterName, 'Prints version information')
            [CompletionResult]::new('--version', 'version', [CompletionResultType]::ParameterName, 'Prints version information')
            [CompletionResult]::new('-q', 'q', [CompletionResultType]::ParameterName, 'Silence all output')
            [CompletionResult]::new('--quiet', 'quiet', [CompletionResultType]::ParameterName, 'Silence all output')
            [CompletionResult]::new('-v', 'v', [CompletionResultType]::ParameterName, 'Debug mode')
            [CompletionResult]::new('--verbose', 'verbose', [CompletionResultType]::ParameterName, 'Debug mode')
            break
        }
        'godwit-daemon;die;help' {
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Prints help information')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Prints help information')
            [CompletionResult]::new('-V', 'V', [CompletionResultType]::ParameterName, 'Prints version information')
            [CompletionResult]::new('--version', 'version', [CompletionResultType]::ParameterName, 'Prints version information')
            [CompletionResult]::new('-q', 'q', [CompletionResultType]::ParameterName, 'Silence all output')
            [CompletionResult]::new('--quiet', 'quiet', [CompletionResultType]::ParameterName, 'Silence all output')
            [CompletionResult]::new('-v', 'v', [CompletionResultType]::ParameterName, 'Debug mode')
            [CompletionResult]::new('--verbose', 'verbose', [CompletionResultType]::ParameterName, 'Debug mode')
            break
        }
        'godwit-daemon;help' {
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Prints help information')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Prints help information')
            [CompletionResult]::new('-V', 'V', [CompletionResultType]::ParameterName, 'Prints version information')
            [CompletionResult]::new('--version', 'version', [CompletionResultType]::ParameterName, 'Prints version information')
            [CompletionResult]::new('-q', 'q', [CompletionResultType]::ParameterName, 'Silence all output')
            [CompletionResult]::new('--quiet', 'quiet', [CompletionResultType]::ParameterName, 'Silence all output')
            [CompletionResult]::new('-v', 'v', [CompletionResultType]::ParameterName, 'Debug mode')
            [CompletionResult]::new('--verbose', 'verbose', [CompletionResultType]::ParameterName, 'Debug mode')
            break
        }
    })

    $completions.Where{ $_.CompletionText -like "$wordToComplete*" } |
        Sort-Object -Property ListItemText
}
