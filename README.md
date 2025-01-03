# Chirpfy 🦀

A Rust CLI tool that refines your tweets using OpenAI's GPT-4. It transforms verbose technical thoughts into concise, impactful tweets while maintaining your professional voice.

## Features

- Transforms draft tweets into polished, tech-savvy content
- Maintains technical credibility while being approachable
- Focuses on product/tech insights with entrepreneurial angles
- Keeps responses sharp and contrarian when appropriate
- Uses emojis sparingly (max 1-2)
- Ensures tweets stay within Twitter's 280-character limit

## Prerequisites

- Rust toolchain (1.70.0 or later)
- OpenAI API key
- macOS or Linux

## Customizing the Prompt

You can customize how Chirpfy refines your tweets in two ways:

1. Edit `SYSTEM_PROMPT.md` directly in the project root
2. Use your own prompt file by setting the environment variable:
   ```bash
   export CHIRPFY_PROMPT_PATH="/path/to/your/prompt.md"
   ```

See `SYSTEM_PROMPT.md` for the default prompt and use it as a template for your customizations.

## Installation

### 1. Set up OpenAI API key

Add to your `~/.zshrc`:

```bash
export OPENAI_API_KEY="your-api-key-here"
source ~/.zshrc
```

### 2. Install Chirpfy

```bash
git clone https://github.com/ertyurk/chirpfy.git
cd chirpfy
cargo build --release
sudo cp target/release/chirpfy /usr/local/bin/
```

### 3. Add Shell Conveniences (Optional)

Add to your `~/.zshrc`:

```bash
# Short alias
alias tw="chirpfy"

# Function for quote-free usage
tweet() {
    chirpfy "$*"
}
```

## Usage

```bash
# Standard way
chirpfy "Just learned about Rust's ownership model"

# Using alias (after adding shell conveniences)
tw "Just learned about Rust's ownership model"

# Using function (no quotes needed)
tweet Just learned about Rust's ownership model
```

### Example

```bash
$ tw "Shame on me that I did not consider rust before. Bottleneck from my mind got removed."
"After years of Go & TypeScript, Rust's ownership model just clicked. It's not just about memory - it's about modeling complex systems correctly from day one. 🦀"
```

## Development

```bash
# Run from source
cargo run -- "Your tweet"

# Run tests
cargo test

# Run with debug output
RUST_LOG=debug cargo run -- "Your tweet"
```

## Error Handling

Chirpfy provides clear error messages for:

- Tweet length exceeds 280 characters
- Empty tweet input
- Missing OpenAI API key
- API communication errors

## License

MIT License - see LICENSE file for details

## Contributing

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request
