use clap::Parser;
use colored::*;
use rand::seq::SliceRandom;
use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Parser)]
#[command(name = "quotes")]
#[command(about = "随机显示历史名人名言")]
struct Cli {
    #[arg(short, long, help = "按分类筛选 (inspirational, philosophical, humorous, general)")]
    category: Option<String>,
    
    #[arg(short, long, help = "显示作者信息")]
    author: bool,
    
    #[arg(long, help = "显示各分类统计信息")]
    stats: bool,
}

#[derive(Clone, Deserialize, Serialize)]
struct Quote {
    #[serde(rename = "quoteText")]
    text: String,
    #[serde(rename = "quoteAuthor")]
    author: String,
    #[serde(skip_deserializing, default = "default_category")]
    category: String,
}

fn default_category() -> String {
    "general".to_string()
}

#[derive(Clone, Deserialize, Serialize)]
struct ChineseQuote {
    #[serde(rename = "name")]  
    text: String,
    #[serde(rename = "from")]
    author: String,
    #[serde(skip_deserializing, default = "default_chinese_category")]
    category: String,
}

fn default_chinese_category() -> String {
    "chinese".to_string()
}

fn load_chinese_quotes() -> Result<Vec<ChineseQuote>, Box<dyn std::error::Error>> {
    let possible_paths = [
        "chinese_sentences.json",
        "quotes-cli/chinese_sentences.json",
        "./chinese_sentences.json"
    ];
    
    for path in &possible_paths {
        if std::path::Path::new(path).exists() {
            let json_content = fs::read_to_string(path)?;
            let mut quotes = Vec::new();
            
            // 解析JSONL格式（每行一个JSON对象）
            for line in json_content.lines() {
                if !line.trim().is_empty() {
                    match serde_json::from_str::<ChineseQuote>(line) {
                        Ok(mut quote) => {
                            quote.category = "chinese".to_string();
                            quotes.push(quote);
                        }
                        Err(_) => continue, // 跳过无法解析的行
                    }
                }
            }
            
            return Ok(quotes);
        }
    }
    
    Err("找不到中文名句数据库文件".into())
}

// 统一的名言结构体，用于混合显示
#[derive(Clone)]
struct UnifiedQuote {
    text: String,
    author: String,
    category: String,
}

fn load_quotes_from_file() -> Result<Vec<Quote>, Box<dyn std::error::Error>> {
    let possible_paths = [
        "quotes_database.json",
        "quotes-cli/quotes_database.json", 
        "./quotes_database.json"
    ];
    
    for path in &possible_paths {
        if std::path::Path::new(path).exists() {
            let json_content = fs::read(path)?;
            let json_str = String::from_utf8_lossy(&json_content);
            let mut quotes: Vec<Quote> = serde_json::from_str(&json_str)?;
            
            // 给每个名言分配一个简单的分类
            for quote in &mut quotes {
                quote.category = categorize_quote(&quote.text, &quote.author);
            }
            
            return Ok(quotes);
        }
    }
    
    Err("找不到名言数据库文件".into())
}

fn categorize_quote(text: &str, author: &str) -> String {
    let text_lower = text.to_lowercase();
    let author_lower = author.to_lowercase();
    
    // 根据关键词简单分类
    if text_lower.contains("success") || text_lower.contains("achieve") || 
       text_lower.contains("goal") || text_lower.contains("work") ||
       text_lower.contains("effort") || text_lower.contains("dream") {
        return "inspirational".to_string();
    }
    
    if author_lower.contains("einstein") || author_lower.contains("socrates") ||
       author_lower.contains("aristotle") || author_lower.contains("plato") ||
       text_lower.contains("life") || text_lower.contains("truth") ||
       text_lower.contains("wisdom") || text_lower.contains("knowledge") {
        return "philosophical".to_string();
    }
    
    if text_lower.contains("funny") || text_lower.contains("joke") ||
       author_lower.contains("twain") || author_lower.contains("wilde") {
        return "humorous".to_string();
    }
    
    "general".to_string()
}

fn get_fallback_quotes() -> Vec<Quote> {
    vec![
        Quote {
            text: "天才就是百分之一的灵感，百分之九十九的汗水。".to_string(),
            author: "托马斯·爱迪生".to_string(),
            category: "inspirational".to_string(),
        },
        Quote {
            text: "我思故我在。".to_string(),
            author: "勒内·笛卡尔".to_string(),
            category: "philosophical".to_string(),
        },
        Quote {
            text: "成功是从失败到失败，也依然不失热情。".to_string(),
            author: "温斯顿·丘吉尔".to_string(),
            category: "inspirational".to_string(),
        },
    ]
}

fn get_category_emoji(category: &str) -> &'static str {
    match category {
        "inspirational" => "🚀",
        "philosophical" => "🤔", 
        "humorous" => "😄",
        "general" => "📝",
        "chinese" => "🀄",
        _ => "💭",
    }
}

fn load_all_quotes() -> Vec<UnifiedQuote> {
    let mut all_quotes = Vec::new();
    
    // 加载英文名言
    match load_quotes_from_file() {
        Ok(quotes) => {
            for quote in quotes {
                all_quotes.push(UnifiedQuote {
                    text: quote.text,
                    author: quote.author,
                    category: quote.category,
                });
            }
        }
        Err(_) => {
            // 使用备用英文名言
            let fallback = get_fallback_quotes();
            for quote in fallback {
                all_quotes.push(UnifiedQuote {
                    text: quote.text,
                    author: quote.author,
                    category: quote.category,
                });
            }
        }
    }
    
    // 加载中文名句
    match load_chinese_quotes() {
        Ok(quotes) => {
            for quote in quotes {
                all_quotes.push(UnifiedQuote {
                    text: quote.text,
                    author: quote.author,
                    category: quote.category,
                });
            }
        }
        Err(_) => {
            // 中文名句加载失败，静默处理
        }
    }
    
    all_quotes
}

fn display_unified_quote(quote: &UnifiedQuote, show_author: bool) {
    let border = "═".repeat(60);
    
    println!("{}", border.bright_blue());
    println!();
    
    // 分行显示长引言
    let words: Vec<&str> = quote.text.split_whitespace().collect();
    let mut lines = Vec::new();
    let mut current_line = String::new();
    
    for word in words {
        if current_line.len() + word.len() + 1 > 50 {
            if !current_line.is_empty() {
                lines.push(current_line.clone());
                current_line.clear();
            }
        }
        if !current_line.is_empty() {
            current_line.push(' ');
        }
        current_line.push_str(word);
    }
    if !current_line.is_empty() {
        lines.push(current_line);
    }
    
    for line in lines {
        println!("  💭 {}", line.bright_white().bold());
    }
    
    println!();
    
    if show_author {
        println!("      {}", format!("— {}", quote.author).bright_yellow().italic());
    }
    
    println!();
    println!("{}", border.bright_blue());
}

fn main() {
    let cli = Cli::parse();
    
    let quotes = load_all_quotes();
    
    if cli.stats {
        show_unified_statistics(&quotes);
        return;
    }
    
    let filtered_quotes: Vec<&UnifiedQuote> = if let Some(category) = &cli.category {
        quotes.iter().filter(|q| q.category == *category).collect()
    } else {
        quotes.iter().collect()
    };
    
    if filtered_quotes.is_empty() {
        eprintln!("{}", "❌ 没有找到符合条件的名言！".red());
        eprintln!("可用分类: inspirational, philosophical, humorous, general, chinese");
        return;
    }
    
    let mut rng = rand::thread_rng();
    if let Some(quote) = filtered_quotes.choose(&mut rng) {
        display_unified_quote(quote, cli.author);
    }
}

fn show_unified_statistics(quotes: &[UnifiedQuote]) {
    use std::collections::HashMap;
    
    let mut category_counts: HashMap<String, usize> = HashMap::new();
    
    for quote in quotes {
        *category_counts.entry(quote.category.clone()).or_insert(0) += 1;
    }
    
    println!();
    println!("{}", "📊 名言分类统计".bright_cyan().bold());
    println!("{}", "═".repeat(50).bright_blue());
    
    let mut sorted_categories: Vec<_> = category_counts.into_iter().collect();
    sorted_categories.sort_by(|a, b| b.1.cmp(&a.1));
    
    for (category, count) in sorted_categories {
        let percentage = (count as f64 / quotes.len() as f64) * 100.0;
        let category_display = match category.as_str() {
            "inspirational" => "励志类",
            "philosophical" => "哲学类", 
            "humorous" => "幽默类",
            "general" => "通用类",
            "chinese" => "中文名句",
            _ => &category,
        };
        println!("  {} {}: {} 条 ({:.1}%)", 
                get_category_emoji(&category),
                category_display.bright_white().bold(),
                count.to_string().bright_yellow(),
                percentage);
    }
    println!("{}", "═".repeat(50).bright_blue());
    println!("  总计: {} 条名言", quotes.len().to_string().bright_green().bold());
    println!();
}
