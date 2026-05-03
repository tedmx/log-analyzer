use std::env;

fn main() {
  // Собираем аргументы в вектор строк
  // .collect() — это мощный метод, который превраает итератор в коллекцию
  let args: Vec<String> = env::args().collect();

  if args.len() < 2 {
    println!("Ошибка: укажите путь к файлу логов");
    return
  }

  let file_path = &args[1];
  println!("Будем анализировать файл: {}", file_path)
}
