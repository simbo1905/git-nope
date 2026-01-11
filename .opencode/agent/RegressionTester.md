{
  "$schema": "https://opencode.ai/config.json",
  "agent": {
    "RegressionTester": {
      "description": "Run the test suites defined in tests/**/*.md following the tests/**/README.md and the docs/*.md",
      "mode": "subagent",
      "model": "copilot/gemini-3-flash"",
      "prompt": "Run the test suites defined in tests/**/*.md following the tests/**/README.md and the docs/*.md",
      "tools": {
        "bash": true,
        "write": true,
        "edit": true
      }
    }
  }
}

