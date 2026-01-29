# Claude Code Settings

Shared permissions for trusted development repositories.

## What This Is

Claude Code uses `settings.json` to pre-authorize certain commands, eliminating confirmation prompts for routine operations. This provides sensible defaults for active development repos where you have implied trust.

## How to Use

1. Copy the `.claude/` folder to your project (includes this file and `settings.json`)

2. Claude Code will now run allowed commands without prompting

3. Override locally with `.claude/settings.local.json` if needed (gitignored)

## Permission Rationale

### Read/Explore Operations

| Command | Purpose | Why Safe |
|---------|---------|----------|
| `grep` | Search file contents | Read-only |
| `tree` | Display directory structure | Read-only |
| `ls` | List directory contents | Read-only |
| `find` | Locate files/directories | Read-only |
| `cat` | Display file contents | Read-only |
| `head` | Show first lines of file | Read-only |
| `tail` | Show last lines of file | Read-only |
| `wc` | Count lines/words/chars | Read-only |
| `which` | Locate command path | Read-only |
| `pwd` | Print working directory | Read-only |

These commands inspect the filesystem without modification.

### File Operations

| Command | Purpose | Why Safe |
|---------|---------|----------|
| `cd` | Change directory | Navigation only |
| `mkdir` | Create directories | Creates structure |
| `mv` | Move/rename files | Reorganizes, doesn't delete |
| `cp` | Copy files | Duplicates, non-destructive |
| `rm` | Remove files | Single file deletion (rm -rf denied) |
| `touch` | Create empty files | Creates structure |
| `echo` | Output text | Display/redirect |

Basic file operations for project management. Note: `rm -rf` is explicitly denied.

### Git Operations (Read)

| Command | Purpose | Why Safe |
|---------|---------|----------|
| `git status` | Show working tree status | Inspects state only |
| `git diff` | Show changes | Inspects state only |
| `git log` | Show commit history | Inspects state only |
| `git branch` | List/show branches | Inspects state only |
| `git show` | Show commit details | Inspects state only |
| `git remote` | Show remote info | Inspects state only |

### Git Operations (Write)

| Command | Purpose | Why Safe |
|---------|---------|----------|
| `git add` | Stage files | Prepares commit |
| `git commit` | Create commit | Local operation |
| `git reset` | Unstage/move HEAD | Soft/mixed are recoverable |
| `git checkout` | Switch branches/restore | Standard workflow |
| `git switch` | Switch branches | Modern branch switching |
| `git stash` | Save work in progress | Preserves work |
| `git fetch` | Download from remote | Read from remote |
| `git pull` | Fetch and merge | Standard sync |
| `git push` | Push to remote | Normal push only |
| `git merge` | Merge branches | Standard workflow |
| `git rebase` | Rebase commits | Interactive disabled |
| `git cherry-pick` | Apply specific commits | Selective merge |
| `git revert` | Create revert commit | Safe undo |
| `git tag` | Create/list tags | Labeling |

Note: Destructive variants (`--hard`, `--force`) are explicitly denied.

### Build Tools

| Command | Purpose | Why Safe |
|---------|---------|----------|
| `make` | Run Makefile targets | Executes project-defined tasks |
| `just` | Run justfile recipes | Executes project-defined tasks |

In trusted repos, build tool commands run tasks you've defined. The assumption is you trust code you're actively developing.

### .NET Development

| Command | Purpose | Why Safe |
|---------|---------|----------|
| `dotnet new` | Create new project/solution | Scaffolding |
| `dotnet sln` | Manage solution files | Project organization |
| `dotnet add` | Add packages/references | Dependency management |
| `dotnet remove` | Remove packages/references | Dependency management |
| `dotnet build` | Compile project | Standard build operation |
| `dotnet test` | Run tests | Standard test operation |
| `dotnet run` | Run application | Standard run operation |
| `dotnet watch` | Run with hot reload | Development workflow |
| `dotnet clean` | Remove build artifacts | Cleans generated files only |
| `dotnet restore` | Restore NuGet packages | Downloads declared dependencies |
| `dotnet format` | Format code | Code style |
| `dotnet tool` | Manage CLI tools | Tool installation |
| `dotnet list` | List packages/references | Read-only inspection |
| `dotnet --version` | Show SDK version | Read-only |
| `dotnet --info` | Show SDK info | Read-only |

Standard .NET development lifecycle commands that operate within the project boundary.

### Skills

| Skill | Purpose |
|-------|---------|
| `code-review` | Multi-agent code review |

### Denied Operations

| Command | Why Denied |
|---------|------------|
| `rm -rf` | Recursive force deletion - too destructive |
| `sudo` | Privilege escalation - requires explicit consent |
| `git reset --hard` | Loses uncommitted work permanently |
| `git push --force` / `-f` | Overwrites remote history |
| `git clean -f` / `-fd` | Permanently deletes untracked files |

These commands have potential for significant unintended damage and should always prompt for confirmation.

## Customization

Add or remove commands based on your workflow:

```json
{
  "permissions": {
    "allow": [
      "Bash(npm test:*)",
      "Bash(cargo build:*)"
    ]
  }
}
```

## Notes

- `settings.json` is committed and shared with the team
- Override locally with `settings.local.json` (should be gitignored)
- Settings are additive - this extends, not replaces, other Claude Code settings
- For untrusted repos, consider a more restrictive configuration
