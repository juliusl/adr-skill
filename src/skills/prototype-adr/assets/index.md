# prototype-adr Assets

Curated index of available assets and templates for the prototype-adr skill.

## Templates

| Template | Description | Path |
|----------|-------------|------|
| Default Profile | Minimal worktree-based profile for quick experiments | `templates/default-profile.toml` |

## Usage

Display the default profile template:

```bash
make -f <skill-root>/Makefile show-profile-template
```

Custom profiles should be placed in `.adr/profiles/` in the project root
(per ADR-0020's project-scoped convention). See
[Profile Format Reference](../references/profiles.md) for the full TOML schema.
