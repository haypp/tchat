# Tchat - Terminal AI Chat Client

A Rust-based terminal chat client that uses the OpenRouter.ai API to interact with various AI models. Features include chat history management, Markdown-style formatting, and a clean terminal interface.

### Features

- **Terminal-based Interface**: Clean and simple command-line interface
- **Chat History**: Maintains context across conversations
- **Markdown Formatting**:
  - `### Headings` displayed in bold green
  - `**bold text**` displayed in bold
- **Easy Context Management**: Clear chat history with simple commands

### Prerequisites

- Rust and Cargo (latest stable version)
- OpenSSL development libraries
- An OpenRouter.ai API key

### Installation

1. Clone the repository:

```bash
git clone https://github.com/yourusername/tchat.git
cd tchat
```

2. Build the release version:

```bash
cargo build --release
```

3. Set up your API key:

```bash
echo "export OPENROUTER_API_KEY=your_api_key_here" > ~/.config/tchat/config
source ~/.config/tchat/config
```

### Usage

Run the program:

```bash
./target/release/tchat
```

Available commands:

- Type your message and press Enter to chat
- Type `clear context` to reset chat history
- Type `exit` to quit the program

### Example Conversation

```
> Tell me about Rust programming

🤖 Assistant:
### Introduction to Rust

Rust is a **systems programming language** that focuses on three main goals:
- **Safety**: Memory and thread safety guaranteed at compile time
- **Concurrency**: Safe concurrent programming
- **Performance**: Zero-cost abstractions

You can write both low-level and high-level applications in Rust!
```

### System-wide Installation

For Arch Linux users:

```bash
sudo install -Dm755 target/release/tchat /usr/local/bin/tchat
```

### Building from Source

Dependencies in `Cargo.toml`:

```toml
[dependencies]
reqwest = { version = "0.11", features = ["json"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
rustyline = "9.0"
tokio = { version = "1", features = ["full"] }
```

### Contributing

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/AmazingFeature`)
3. Commit your changes (`git commit -m 'Add some AmazingFeature'`)
4. Push to the branch (`git push origin feature/AmazingFeature`)
5. Open a Pull Request

### License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

### Acknowledgments

- OpenRouter.ai for providing the AI API
- The Rust community for excellent documentation and crates
