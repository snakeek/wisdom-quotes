# Wisdom Quotes 📚

[![Crates.io](https://img.shields.io/crates/v/wisdom-quotes.svg)](https://crates.io/crates/wisdom-quotes)
[![License](https://img.shields.io/crates/l/wisdom-quotes.svg)](LICENSE-MIT)
[![Downloads](https://img.shields.io/crates/d/wisdom-quotes.svg)](https://crates.io/crates/wisdom-quotes)

A beautiful command-line tool to display random famous quotes from both English and Chinese literature. Perfect for daily inspiration, terminal customization, or just discovering wisdom from across cultures and centuries.

## ✨ Features

- **15,421+ Quotes**: Curated collection including:
  - 5,421 English famous quotes from historical figures
  - 10,000 Chinese classical poetry and literature excerpts
- **Smart Categorization**: Filter by category (inspirational, philosophical, humorous, chinese, general)
- **Beautiful Display**: Elegant formatting with colors and Unicode art
- **Lightning Fast**: Instant random quote selection with no API calls
- **Cross-platform**: Works seamlessly on macOS, Linux, and Windows
- **Offline First**: All data embedded, no internet required
- **Multilingual**: Seamlessly blends English and Chinese wisdom

## 📦 Installation

### Quick Install from crates.io
```bash
cargo install wisdom-quotes
```

### Build from Source
```bash
git clone https://github.com/snakeek/wisdom-quotes.git
cd wisdom-quotes
cargo install --path .
```

## 🚀 Usage

### Basic Commands
```bash
# Random quote (any language/category)
quotes

# Show quote with author attribution
quotes --author

# View collection statistics
quotes --stats
```

### Category Filtering
```bash
# Inspirational quotes for motivation
quotes --category inspirational --author

# Chinese classical poetry and wisdom
quotes --category chinese --author

# Philosophical insights
quotes --category philosophical --author

# Light-hearted and humorous quotes
quotes --category humorous --author

# General wisdom and observations
quotes --category general --author
```

## 🎨 Example Output

**English Quote:**
```
════════════════════════════════════════════════════════════

  💭 The only way to do great work is to love what
  💭 you do.

      — Steve Jobs

════════════════════════════════════════════════════════════
```

**Chinese Classical Poetry:**
```
════════════════════════════════════════════════════════════

  💭 山有木兮木有枝，心悦君兮君不知。

      — 佚名《越人歌》

════════════════════════════════════════════════════════════
```

**Statistics View:**
```bash
$ quotes --stats

📊 名言分类统计
══════════════════════════════════════════════════
  🀄 中文名句: 10000 条 (64.8%)
  📝 通用类: 4043 条 (26.2%)
  🤔 哲学类: 774 条 (5.0%)
  🚀 励志类: 555 条 (3.6%)
  😄 幽默类: 49 条 (0.3%)
══════════════════════════════════════════════════
  总计: 15421 条名言
```

## 📂 Categories

| Category | Emoji | Description | Count |
|----------|-------|-------------|-------|
| **chinese** | 🀄 | Classical Chinese poetry and literature | 10,000 |
| **general** | 📝 | General wisdom and life observations | 4,043 |
| **philosophical** | 🤔 | Deep thoughts and philosophical insights | 774 |
| **inspirational** | 🚀 | Motivational and success-oriented quotes | 555 |
| **humorous** | 😄 | Funny and witty sayings | 49 |

## 🛠️ Command Reference

```
wisdom-quotes 0.1.0
A beautiful command-line tool to display random famous quotes

USAGE:
    quotes [OPTIONS]

OPTIONS:
    -c, --category <CATEGORY>    Filter by category [possible values: inspirational, 
                                philosophical, humorous, general, chinese]
    -a, --author                 Show author information
        --stats                  Show statistics of all categories
    -h, --help                   Print help information
    -V, --version                Print version information
```

## 🌟 Use Cases

- **Daily Inspiration**: Add to your shell profile for daily wisdom
- **Terminal Customization**: Perfect for MOTD or terminal startup
- **Writing Prompts**: Spark creativity with random quotes
- **Language Learning**: Discover classical Chinese literature
- **Meditation**: Philosophical quotes for reflection

## 📖 Data Sources

This project builds upon excellent open-source datasets:

- **English Quotes**: [JamesFT/Database-Quotes-JSON](https://github.com/JamesFT/Database-Quotes-JSON) - Comprehensive collection of famous quotes
- **Chinese Literature**: [caoxingyu/chinese-gushiwen](https://github.com/caoxingyu/chinese-gushiwen) - Classical Chinese poetry and literature database

## 🤝 Contributing

Contributions are welcome! Areas where you can help:

- **Quote Quality**: Improve categorization or fix attribution errors
- **New Features**: Random quote of the day, favorites system, search functionality
- **Localization**: Add support for other languages
- **Performance**: Optimize loading or display performance

Please feel free to:
1. Open an issue for bugs or feature requests
2. Submit a pull request with improvements
3. Share feedback on quote quality or categorization

## 📄 License

This project is dual-licensed under your choice of:

- [Apache License, Version 2.0](LICENSE-APACHE)
- [MIT License](LICENSE-MIT)

## 🙏 Acknowledgments

- Thanks to all the philosophers, poets, and thinkers whose wisdom is preserved here
- Gratitude to the open-source community maintaining the quote databases
- Special appreciation for classical Chinese literature preservation efforts

---

*"The best time to plant a tree was 20 years ago. The second best time is now." — Chinese Proverb*