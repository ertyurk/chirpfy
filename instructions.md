# Chirpfy (Rust CLI Tweet Refiner)

## Overview

A command-line tool written in Rust that uses OpenAI's GPT-4 to transform draft tweets/posts into polished, tech-savvy content that reflects a product-focused entrepreneurial voice.

## Goals

- Transform verbose technical thoughts into impactful content
- Maintain technical credibility while being approachable
- Focus on product/tech insights with entrepreneurial angles
- Keep responses sharp and contrarian when appropriate

## User Flow

1. User runs `chirpify "<draft text>"`
2. Rust program interfaces with OpenAI's API, applying the "Tweet Agent" prompt
3. Program prints the refined content to stdout

## Functional Requirements

- **CLI Command**:
  ```
  chirpfy "Shame on me that I did not consider rust before. Bottleneck from my mind got removed."
  ```
- **Output**:
  - The refined content with proper formatting
  - Support for both short tweets and longer posts (up to 5,000 chars)
- **Configuration**:
  - OpenAI API key via environment variable
- **Error Handling**:
  - Clear, actionable error messages
  - Graceful fallback for API failures

## Technical Stack

- **Rust** with modern idioms
- **reqwest** for OpenAI API interactions
- **clap** for CLI argument parsing
- **tokio** for async runtime
- **serde** for JSON serialization
- **thiserror** for error handling

## System Prompt

```
You are a tweet refiner for a technical product leader. Transform inputs into impactful tweets that:
1. Emphasize technical insights and product thinking
2. Maintain professional credibility while being approachable
3. Include contrarian views when relevant
4. Focus on systems thinking and scalability
5. Keep the entrepreneurial angle
6. Use emojis sparingly (max 1-2) and only when they add value

Style Guidelines:
- Sharp and direct
- Technical but not overly academic
- Product-focused
- Slightly provocative when appropriate
- For long posts (>280 chars):
  - Break into clear paragraphs
  - Use bullet points for lists
  - Keep structure clean and readable
  - Maintain focus despite length

Avoid:
- Generic startup platitudes
- Overly promotional language
- Hashtags
- Thread suggestions
- Unnecessary verbosity (even in long posts)

Return ONLY the refined tweet/post, nothing else.
```

## High-Level Implementation

1. **Initialize OpenAI Client**:
   - Configure with API key from environment
   - Set up HTTP client with proper headers
2. **Process Input**:
   - Parse CLI args with proper error handling
   - Validate input length (max 5,000 chars)
3. **Generate Response**:
   - Send request to OpenAI's Chat API
   - Handle API errors gracefully
4. **Output**:
   - Print only the refined content
   - Exit with appropriate status code

## Example Usage

```bash
# Command
chirpfy "Shame on me that I did not consider rust before. Bottleneck from my mind got removed."

# Potential Output
"After years of Go & TypeScript, Rust's ownership model just clicked. It's not just about memory - it's about modeling complex systems correctly from day one. 🦀"
```

## Project Structure

```
src/
  ├── main.rs           # Entry point and CLI handling
  ├── agent.rs          # OpenAI agent implementation
  ├── error.rs          # Error types and handling
  └── tweet/
      ├── mod.rs        # Module definitions
      ├── refiner.rs    # Tweet refinement logic
      └── validator.rs  # Input validation
```
