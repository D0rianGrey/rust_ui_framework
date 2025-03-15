#!/bin/bash
# Проверяем, установлен ли ChromeDriver
if ! command -v chromedriver &> /dev/null; then
    echo "ChromeDriver не найден. Пожалуйста, установите его и добавьте в PATH."
    exit 1
fi

# Запускаем ChromeDriver
echo "Запуск ChromeDriver..."
chromedriver --port=9515 &

# Сохраняем ID процесса
DRIVER_PID=$!

# Функция для корректного завершения процесса
cleanup() {
    echo "Остановка ChromeDriver..."
    kill $DRIVER_PID
    exit 0
}

# Обработка сигналов завершения
trap cleanup SIGINT SIGTERM

# Ожидаем ввода для завершения
echo "ChromeDriver запущен (PID: $DRIVER_PID). Нажмите Enter для завершения."
read

# Завершаем ChromeDriver
cleanup