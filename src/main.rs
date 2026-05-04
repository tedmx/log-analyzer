use std::env;
use std::path::Path;
use std::fs;
use std::collections::HashMap;

#[derive(Debug, PartialEq, Copy, Clone)]
enum LogLevel {
  Info,
  Error,
  Warn
}

impl LogLevel {
  // Метод возвращает статическую строку, ей не нужен 'a, 
  // так как строки в кавычках живут вечно (static)
  fn icon(&self) -> &str {
    match self {
      LogLevel::Error => "❌",
      LogLevel::Warn  => "⚠️",
      LogLevel::Info  => "✅",
    }
  }
}

#[allow(dead_code)]
#[derive(Debug)]
struct LogEntry<'a> {
  level: LogLevel,
  message: &'a str,
}

impl<'a> LogEntry<'a> {
  fn parse(line: &'a str) -> Self {
    let level = if line.contains("ERROR") {
      LogLevel::Error
    } else if line.contains("WARN") {
      LogLevel::Warn
    } else {
      LogLevel::Info
    };

    let parts: Vec<&str> = line.splitn(2, ": ").collect();

    let message = if parts.len() == 2 {
        parts[1]
    } else {
        line
    };

    LogEntry { level, message }
  }
}

#[allow(dead_code)]
struct LogStats<'a> {
  total: usize,
  errors: usize,
  warnings: usize,
  infos: usize,
  top_error_word: Option<(&'a str, usize)>, // (слово, сколько раз встретилось)
}

impl<'a> LogStats<'a> {
  pub fn calculate(entries: &[LogEntry<'a>]) -> Self {
    let mut errors = 0;
    let mut warnings = 0;
    let mut infos = 0;
    let mut words_frequency = HashMap::new();

    for entry in entries {
      match entry.level {
        LogLevel::Error => {
          errors += 1;
          // Разбиваем сообщение на слова
          for word in entry.message.split_whitespace() {
            // Очищаем от знаков препинания, оставляя только буквы и цифры
            let clean_word = word.trim_matches(|c: char| !c.is_alphanumeric());
            
            // Игнорируем слишком короткие слова (артикли, предлоги)
            if clean_word.len() > 3 {
              let count = words_frequency.entry(clean_word).or_insert(0);
              *count += 1;
            }
          }
        }
        LogLevel::Warn => warnings += 1,
        LogLevel::Info => infos += 1,
      }
    }

    // Находим слово с максимальным значением счетчика
    let top_error_word = words_frequency
      .into_iter()
      .max_by_key(|&(_, count)| count);

    LogStats {
      total: entries.len(),
      errors,
      warnings,
      infos,
      top_error_word,
    }
  }

  pub fn print_summary(&self) {
    println!("\n=== СВОДНЫЙ ОТЧЕТ ПО ЛОГАМ ===");
    println!("Всего записей: {}", self.total);
    println!("{} Ошибки: {}", LogLevel::Error.icon(), self.errors);
    
    if let Some((word, count)) = self.top_error_word {
      println!("Самая частая причина ошибок: \"{}\" (встречается {} раз)", word, count);
    }
    
    println!("------------------------------");
  }
}

fn main() {
  let mut args = env::args();

  args.next();

  let file_path = match args.next() {
    Some(path) => path,
    None => {
      println!("Ошибка: укажите путь к файлу");
      return
    }
  };

  if !Path::new(&file_path).exists() {
    println!("Ошибка: файл '{}' не найден", file_path);
    return
  }

  let content = match fs::read_to_string(&file_path) {
    Ok(text) => text,
    Err(error) => {
      println!("Не удалось прочитать файл: {}", error);
      return
    }
  };

  let entries: Vec<LogEntry> = content
    .lines()
    .map(LogEntry::parse)
    .collect();

  println!("Обработано строк: {}", entries.len());

  let stats = LogStats::calculate(&entries);
  stats.print_summary();

  println!("");

  // 1. Считаем количество ошибок для статистики
  let error_count = entries.iter()
    .filter(|e| e.level == LogLevel::Error)
    .count();

  println!("Найдено ошибок (ERROR): {}", error_count);

  println!("");

  println!("--- Последние 5 критических событий: ---");

  // 2. Выводим только ошибки (последние 5 штук)
  let errors: Vec<&LogEntry> = entries.iter()
    .filter(|e| e.level == LogLevel::Error)
    .collect();

  for err in errors.iter().rev().take(5) {
    println!("[!] {}", err.message);
  }

  println!("");

  println!("--- Анализ завершен. Последние события: ---");

  for entry in entries.iter().rev().take(5) {
      // Используем метод icon() и само сообщение
      println!("{} {}", entry.level.icon(), entry.message);
  }
}
