# FROM mongo:latest
# COPY init.json /init.json
# COPY mongo-script.sh /mongo-script.sh
# CMD ["bash", "/mongo-script.sh"]

FROM mongo:latest

# Копіюємо файли ініціалізації в спеціальну директорію MongoDB
COPY init.json /docker-entrypoint-initdb.d/
COPY mongo-script.sh /docker-entrypoint-initdb.d/

# Не потрібно вказувати CMD для запуску скриптів, MongoDB автоматично виконає їх із /docker-entrypoint-initdb.d/
