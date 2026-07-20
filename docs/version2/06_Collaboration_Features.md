# QuickChat Version 2: Collaboration Features

## 1. Executive Summary

QuickChat differentiates itself by being a communication layer *for developers*. While V1 includes static sharing of code pointers and terminal output, V2 will introduce real-time, interactive collaboration tools designed to bridge the gap between the terminal chat and the user's IDE.

## 2. Interactive Live Code Pointers

### 2.1. V1 Limitation
Sharing a code pointer in V1 involves sending a static string (`src/main.rs:42`). The recipient must manually open their editor and navigate to that location.

### 2.2. V2 Solution: Editor IPC Bridge
- **Concept:** Code pointers become interactive, clickable elements within the Ratatui UI.
- **Implementation:** QuickChat will establish an Inter-Process Communication (IPC) bridge (via local sockets or named pipes) with popular editors (VS Code, Neovim, IntelliJ).
- **Flow:** 
  1. User A clicks a code pointer bubble in the chat.
  2. QuickChat sends a JSON-RPC command over the IPC bridge.
  3. The local editor intercepts the command, brings itself to the foreground, opens the file, and jumps the cursor to the specified line.
- **Deep Linking:** Alternatively, QuickChat can utilize OS-level deep linking URIs (e.g., `vscode://file/...`).

## 3. Shared Terminal Sessions (Read-Only)

### 3.1. The Use Case
Developers frequently need to show logs, debug output, or a failing test run in real-time. Copy-pasting text loses context and ANSI formatting.

### 3.2. V2 Implementation
- **Multiplexing TTY Streams:** User A can execute a command within QuickChat: `/share-term docker logs -f backend`.
- **PTY Allocation:** QuickChat allocates a pseudo-terminal (PTY) and executes the command.
- **Streaming:** The raw ANSI output stream is captured, encrypted, and streamed over a dedicated QUIC stream to User B.
- **Rendering:** User B's QuickChat client receives the stream and renders the ANSI escape codes perfectly within a specialized chat bubble or expanded right-hand pane.

## 4. CI/CD Pipeline Integration

### 4.1. The Use Case
Developers currently rely on noisy email alerts or context-switching to web dashboards to monitor GitHub Actions or Jenkins builds.

### 4.2. V2 Implementation (Via Plugins)
- **Webhooks to P2P:** A QuickChat "Relay/Webhook Bot" (deployed on the team's infrastructure) receives standard HTTP webhooks from GitHub/GitLab.
- **Translation:** The Bot translates the JSON webhook payload into a rich, formatted QuickChat message (using markdown, colored status indicators).
- **Delivery:** The Bot acts as a standard QuickChat peer, sending the build status directly to the relevant developer or group chat over the secure QUIC connection.
- **Interactivity:** Through the plugin SDK, developers can reply to the Bot to trigger actions (e.g., replying `/retry` to a failed build message).

## 5. Synchronized Clipboard (Opt-in)

- **Enhanced Clipboard:** V1 allows manual sending of clipboard contents. V2 will introduce an "Opt-in Sync Session."
- **Mechanism:** When two developers enter a "Pairing Mode," their OS clipboards are temporarily synchronized over the E2EE QUIC stream. Copying a snippet on Machine A immediately makes it available to paste on Machine B.
- **Security:** This is highly sensitive. It requires explicit, mutual consent, prominent UI indicators while active, and automatic timeout disconnections.
