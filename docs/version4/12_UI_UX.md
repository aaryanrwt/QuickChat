# QuickChat V4: UI & UX

The Terminal User Interface (TUI) is QuickChat's defining characteristic. V4 focuses on making the terminal experience vastly superior to traditional GUI chat applications for developers.

## 1. Design Philosophy
*   **Keyboard-First:** A mouse should never be required. All actions (navigation, sending files, opening links, managing plugins) have intuitive, Vim-inspired or Emacs-inspired keybindings.
*   **Information Density:** Maximize the use of screen real estate without clutter. Chat logs should be highly readable with clear visual separation between messages.
*   **Blazing Fast:** Rendering must be instantaneous. Typing latency should be unnoticeable.

## 2. Framework (Ratatui)
*   The UI is built using `ratatui` (Rust), which provides robust layout management, styling, and widget rendering.

## 3. Key UX Features
*   **Rich Text:** Full support for Markdown, inline code blocks with syntax highlighting (via `syntect`), and terminal-native emojis.
*   **Inline Media:** Displaying images directly in the terminal using Sixel or the Kitty Graphics Protocol, with graceful degradation to ASCII art or file links on unsupported terminals.
*   **Focus Modes:** Toggleable views to maximize the chat area, hide contact lists, or enter a "zen mode" for deep focus.
*   **Command Palette:** A fuzzy-searchable command palette (similar to Ctrl+P in VS Code) for quickly executing commands, switching chats, or launching plugins.
