# Установка

## Поднимаем бд
Этот проект использует такую базу данных как Postgresql. Устанавливаем Posgres, затем создаем базу данных. Переходим в папку с проектом и кoмандой '\i sql/create_tables.sql' создаем таблицы

## .env
Создаем файл '.env' в папке с проектом. Его содержание должно быть примерно таким:
    DATABASE_URL=путь_базы_данных
    PORT=порт
    EMAIL_ADDRES=email_аддрес_компании
    SMTP_KEY=апи_ключ_к_смтп

