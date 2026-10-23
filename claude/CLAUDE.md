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
- Do not split changes into multiple commits unless the user explicitly instructs it. Make a single commit by default.
- Do not add signatures or attribution lines (such as Co-Authored-By or session links) to commits or pull requests unless the user explicitly instructs it.
- Do not proactively suggest adding rules to CLAUDE.md. Add rules only when the user asks.
- Report only what was done. Do not volunteer what was not done (such as not committed or not pushed) or what the user needs to do next (such as running a command to apply the change).
- Do not include personal names, account names, or usernames from the user's configuration in suggestions or recommendations. Refer to them generically (e.g., "the other account").
