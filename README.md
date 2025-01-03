# Chirpfy 🦀

A Rust CLI tool that refines your tweets using OpenAI's GPT-4. It transforms verbose technical thoughts into concise, impactful tweets while maintaining your professional voice.

## Features

- Transforms draft tweets into polished, tech-savvy content
- Maintains technical credibility while being approachable
- Focuses on product/tech insights with entrepreneurial angles
- Keeps responses sharp and contrarian when appropriate
- Uses emojis sparingly (max 1-2)
- Ensures tweets stay within Twitter's 280-character limit

## Installation

### Prerequisites

- Rust toolchain (1.70.0 or later)
- OpenAI API key

### Building from Source

```bash
# Clone the repository
git clone https://github.com/your-username/chirpfy.git
cd chirpfy

# Build the project
cargo build --release

# Optional: Add to your PATH
cp target/release/chirpfy /usr/local/bin/
```

### Configuration

Set your OpenAI API key as an environment variable:

```bash
export OPENAI_API_KEY='your-api-key-here'
```

For permanent configuration, add it to your shell's config file (~/.zshrc, ~/.bashrc, etc.):

```bash
echo 'export OPENAI_API_KEY="your-api-key-here"' >> ~/.zshrc
source ~/.zshrc
```

## Usage

```bash
# Basic usage
chirpfy "Your draft tweet here"

# Example
chirpfy "Shame on me that I did not consider rust before. Bottleneck from my mind got removed."
# Output: "After years of Go & TypeScript, Rust's ownership model just clicked. It's not just about memory - it's about modeling complex systems correctly from day one. 🦀"
```

## Error Handling

Chirpfy provides clear error messages for common issues:

- Tweet length exceeds 280 characters
- Empty tweet input
- Missing OpenAI API key
- API communication errors

## Development

```bash
# Run tests
cargo test

# Run with debug output
RUST_LOG=debug cargo run -- "Your tweet here"
```

## License

MIT License - see LICENSE file for details

## Contributing

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request
