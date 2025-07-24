# Quotes CLI 📚

A beautiful command-line tool to display random famous quotes from both English and Chinese literature.

## Features ✨

- **15,000+ Quotes**: Collection of over 15,000 quotes from:
  - 5,421 English famous quotes from historical figures
  - 10,000 Chinese classical poetry and literature excerpts
- **Categorized Display**: Filter quotes by category (inspirational, philosophical, humorous, chinese, general)
- **Beautiful Formatting**: Elegant display with colors and emojis
- **Cross-platform**: Works on macOS, Linux, and Windows
- **Offline**: No internet required, all data embedded

## Installation 📦

### From crates.io
```bash
cargo install quotes-cli
```

### From source
```bash
git clone https://github.com/skywalker124/quotes-cli.git
cd quotes-cli
cargo install --path .
```

## Usage 🚀

### Basic Usage
```bash
# Display a random quote
quotes

# Display a quote with author information
quotes --author
```

### Filter by Category
```bash
# Show inspirational quotes
quotes --category inspirational --author

# Show Chinese classical poetry
quotes --category chinese --author

# Show philosophical quotes
quotes --category philosophical --author

# Show humorous quotes
quotes --category humorous --author
```

### Statistics
```bash
# View statistics of all categories
quotes --stats
```

## Example Output 🎨

```
════════════════════════════════════════════════════════════

  💭 The only way to do great work is to love what
  💭 you do.

      — Steve Jobs

════════════════════════════════════════════════════════════
```

```
════════════════════════════════════════════════════════════

  💭 山有木兮木有枝，心悦君兮君不知。

      — 佚名《越人歌》

════════════════════════════════════════════════════════════
```

## Categories 📂

- **inspirational** 🚀: Motivational and success-oriented quotes
- **philosophical** 🤔: Deep thoughts and wisdom
- **humorous** 😄: Funny and witty sayings
- **chinese** 🀄: Classical Chinese poetry and literature
- **general** 📝: General wisdom and observations

## Command Line Options 🛠️

```
A beautiful command-line tool to display random famous quotes

Usage: quotes [OPTIONS]

Options:
  -c, --category <CATEGORY>  Filter by category (inspirational, philosophical, humorous, general, chinese)
  -a, --author               Show author information
      --stats                Show statistics of all categories
  -h, --help                 Print help
```

## Data Sources 📖

- English quotes: [Database-Quotes-JSON](https://github.com/JamesFT/Database-Quotes-JSON)
- Chinese literature: [chinese-gushiwen](https://github.com/caoxingyu/chinese-gushiwen)

## Contributing 🤝

Contributions are welcome! Please feel free to submit a Pull Request.

## License 📄

This project is licensed under either of

- Apache License, Version 2.0
- MIT license

at your option.