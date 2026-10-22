# Global Instructions

- Use the Read tool to inspect files instead of cat, sed, or head.
- Do not run speculative grep commands. Search only when you know what you are looking for.
- Do not print command exit codes.
- Do not add unnecessary arguments or options. Use the command form exactly as the user specifies.
- Do not hardcode environment-specific values such as user names or home directory paths.
- Do not write to memory unless the user explicitly asks. Put persistent rules in ~/.config/home-manager/claude/CLAUDE.md instead.
- Do not write redundant comments that merely restate what the code already says.
- Write code comments in English.
- Do not end a response with a question or a request for confirmation. Make a reasonable decision, proceed, and state the assumption made.
