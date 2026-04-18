# Shelly TODO

## 🎯 Core Functionality

- [x] Fix AI output to stdout (only command should go to stdout)
  - ~~Move debug JSON output to stderr~~
  - ~~Move "Response:" header to stderr~~
  - ~~Move role/index metadata to stderr~~
  
- [x] Improve system prompt for shell command generation
  - ~~Change from "You are a helpful assistant" to shell-specific prompt~~
  - ~~Add instruction to return only the command, no explanations~~
  
- [x] Extract clean command from AI response
  - ~~Remove the Role/Content metadata printing~~
  - ~~Return just `choice.message.content` as clean string~~
  
- [x] Proper error handling for AI calls
  - ~~Handle empty responses~~
  - ~~Handle API errors~~
  - ~~Handle missing config~~

## 📚 Documentation

- [x] Update README with Fish shell support
  - ~~Add Fish to the list of supported shells~~
  - ~~Add Fish setup instructions~~
  - ~~Update installation section~~

## 🛠️ Future Enhancements

- [ ] Add command history/undo
- [ ] Support for more AI providers (Anthropic, Gemini, etc.)
- [ ] Add dry-run mode (show command without injecting)
- [ ] Shell completion for the CLI itself
- [ ] Add tests

## ✅ Completed

- [x] Add Fish shell support
- [x] Move shell scripts to external files with `include_str!`
