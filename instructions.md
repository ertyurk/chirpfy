# Chirpify (Rust CLI Tweet Refiner)

## Overview

A command-line tool written in Rust that leverages Rig and OpenAI to transform draft tweets into polished, tech-savvy content that reflects a product-focused entrepreneurial voice.

## Goals

- Transform verbose technical thoughts into concise, impactful tweets
- Maintain technical credibility while being approachable
- Focus on product/tech insights with entrepreneurial angles
- Keep responses sharp and contrarian when appropriate

## User Flow

1. User runs `chirpify "<draft tweet text>"`
2. Rust program uses Rig to interface with OpenAI, applying the "Tweet Agent" prompt
3. Program prints the refined tweet to stdout

## Functional Requirements

- **CLI Command**:
  ```
  chirpify "Shame on me that I did not consider rust before. Bottleneck from my mind got removed."
  ```
- **Output**:
  - A single line with the refined tweet (ex: "Just dove into Rust's ownership model - it's not just memory safety, it's a paradigm shift in how we think about systems. Why did I wait so long? 🦀")
- **Configuration**:
  - OpenAI API key via environment variable
  - Optional personality tweaks via config file
- **Error Handling**:
  - Clear, actionable error messages
  - Graceful fallback for API failures

## Technical Stack

- **Rust** with modern idioms
- **Rig** for OpenAI interactions and prompt management
- **Clap** for CLI argument parsing
- **Tokio** for async runtime
- **Config** for configuration management

## System Prompt

```
You are a tweet refiner for a technical product leader. Transform inputs into impactful tweets that:
1. Emphasize technical insights and product thinking
2. Maintain professional credibility while being approachable
3. Include contrarian views when relevant
4. Focus on systems thinking and scalability
5. Keep the entrepreneurial angle
6. Use emojis sparingly (max 1-2) and only when they add value

Style:
- Sharp and direct
- Technical but not overly academic
- Product-focused
- Slightly provocative when appropriate
- Max 280 characters

Avoid:
- Generic startup platitudes
- Overly promotional language
- More than one hashtag
- Thread suggestions
```

## High-Level Implementation

1. **Initialize Rig Agent**:
   - Configure with system prompt and OpenAI model
   - Set up dynamic context if needed
2. **Process Input**:
   - Parse CLI args with proper error handling
   - Validate tweet length and content
3. **Generate Response**:
   - Use Rig's completion API with retry logic
   - Ensure output meets Twitter constraints
4. **Output**:
   - Print only the final tweet
   - Exit with appropriate status code

## Example Usage

```bash
# Command
chirpify "Shame on me that I did not consider rust before. Bottleneck from my mind got removed."

# Potential Output
"After years of Go & TypeScript, Rust's ownership model just clicked. It's not just about memory - it's about modeling complex systems correctly from day one. 🦀"
```

## Project Structure

```
src/
  ├── main.rs           # Entry point
  ├── agent.rs          # Rig agent configuration
  ├── config.rs         # Configuration management
  ├── error.rs          # Error types
  └── tweet/
      ├── mod.rs        # Tweet module
      ├── refiner.rs    # Tweet refinement logic
      └── validator.rs  # Tweet validation
```
