{ pkgs, ... }:

{
  home.packages = [ pkgs.claude-code ];

  home.file.".claude/CLAUDE.md".source = ./claude/CLAUDE.md;

  home.sessionVariables.CLAUDE_CODE_DISABLE_FEEDBACK_SURVEY = "1";
}
