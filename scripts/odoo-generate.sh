#!/bin/bash

# Pedir el nombre del proyecto
read -p "Ingresa el nombre del proyecto: " PROJECT_NAME

# Pedir la versión de Odoo
read -p "Ingresa la versión de Odoo que quieres levantar (11 - 18): " ODOO_WEB_VERSION

# Validar la versión ingresada
if ! [[ "$ODOO_WEB_VERSION" =~ ^(11|12|13|14|15|16|17|18)$ ]]; then
    echo "Versión de Odoo no válida. Por favor ingresa una versión entre 11 y 18."
    read -p "Ingresa la versión de Odoo que quieres levantar (11 - 18): " ODOO_WEB_VERSION
fi

# Crear el directorio del proyecto
PROJECT_DIR="/home/ccano/Workspace/$PROJECT_NAME"
mkdir -p "$PROJECT_DIR"

# Función para verificar si un puerto está en uso en el sistema
function is_port_in_use_system() {
  netstat -an | grep -q ":$1 "
}

# Función para verificar si un puerto está en uso en Docker
function is_port_in_use_docker() {
  docker ps --format '{{.Ports}}' | grep -q "$1"
}

# Generación de una contraseña aleatoria para PostgreSQL
POSTGRES_PASSWORD=$(openssl rand -base64 12)

# Generar un puerto aleatorio para PostgreSQL y Odoo
ODOO_POSTGRES_PORT=$((RANDOM % 1000 + 5000))  # Puerto aleatorio entre 5000 y 5999
ODOO_WEB_PORT=$((RANDOM % 1000 + 8000))  # Puerto aleatorio entre 8000 y 8999

# Verificar que el puerto PostgreSQL no esté en uso
while is_port_in_use_system $ODOO_POSTGRES_PORT || is_port_in_use_docker $ODOO_POSTGRES_PORT; do
  ODOO_POSTGRES_PORT=$((RANDOM % 1000 + 5000))
done

# Verificar que el puerto Odoo no esté en uso
while is_port_in_use_system $ODOO_WEB_PORT || is_port_in_use_docker $ODOO_WEB_PORT; do
  ODOO_WEB_PORT=$((RANDOM % 1000 + 8000))
done

# Variables de entorno
DOCKER_CONTAINER_NETWORK="${PROJECT_NAME}_network"
DOCKER_CONTAINER_NAME_DATABASE="${PROJECT_NAME}-db"
DOCKER_CONTAINER_NAME_WEB="${PROJECT_NAME}-web"
ODOO_POSTGRES_USER="${PROJECT_NAME}-user"
APP_DATA_DIR="/home/ccano/Workspace/$PROJECT_NAME"

# Crear el archivo .env dentro del proyecto
cat <<EOL > "$PROJECT_DIR/.env"
# Docker Container Settings
DOCKER_CONTAINER_NETWORK=$DOCKER_CONTAINER_NETWORK
DOCKER_CONTAINER_NAME_DATABASE=$DOCKER_CONTAINER_NAME_DATABASE
DOCKER_CONTAINER_NAME_WEB=$DOCKER_CONTAINER_NAME_WEB

# PostgreSQL settings
ODOO_POSTGRES_PORT=$ODOO_POSTGRES_PORT
ODOO_POSTGRES_USER=$ODOO_POSTGRES_USER
ODOO_POSTGRES_PASSWORD=$POSTGRES_PASSWORD

# App-specific settings
ODOO_WEB_VERSION=$ODOO_WEB_VERSION
ODOO_WEB_PORT=$ODOO_WEB_PORT

# Local File Sync
APP_DATA_DIR=$APP_DATA_DIR
EOL

# Crear el archivo docker-compose.yml
cat <<EOL > "$PROJECT_DIR/docker-compose.yml"
services:
  $DOCKER_CONTAINER_NAME_DATABASE:
    container_name: $DOCKER_CONTAINER_NAME_DATABASE
    image: postgres:15
    user: root
    environment:
      - POSTGRES_USER=$ODOO_POSTGRES_USER
      - POSTGRES_PASSWORD=$POSTGRES_PASSWORD
      - POSTGRES_DB=postgres
    restart: unless-stopped
    ports:
      - "$ODOO_POSTGRES_PORT:5432"
    volumes:
      - $APP_DATA_DIR/database/postgresql:/var/lib/postgresql/data
    networks:
      - $DOCKER_CONTAINER_NETWORK

  $DOCKER_CONTAINER_NAME_WEB:
    container_name: $DOCKER_CONTAINER_NAME_WEB
    image: odoo:$ODOO_WEB_VERSION
    user: root
    depends_on:
      - $DOCKER_CONTAINER_NAME_DATABASE
    ports:
      - "$ODOO_WEB_PORT:8069"
    tty: true
    command: --
    environment:
      - HOST=$DOCKER_CONTAINER_NAME_DATABASE
      - USER=$ODOO_POSTGRES_USER
      - PASSWORD=$POSTGRES_PASSWORD
    volumes:
      - $APP_DATA_DIR/odoo/addons:/mnt/extra-addons
      - $APP_DATA_DIR/odoo/etc:/etc/odoo
      - $APP_DATA_DIR/odoo/filestore:/root/.local/share
    restart: unless-stopped
    networks:
      - $DOCKER_CONTAINER_NETWORK

networks:
  $DOCKER_CONTAINER_NETWORK:
    driver: bridge
    name: $DOCKER_CONTAINER_NETWORK
EOL

# Directorio del proyecto
APP_DATA_DIR="/home/ccano/Workspace/$PROJECT_NAME"

# Crear las carpetas necesarias para los addons
BASE_DIR="$APP_DATA_DIR/odoo/addons"
DEVELOPMENT_DIR="$APP_DATA_DIR/odoo/addons/development"

mkdir -p "$BASE_DIR"
mkdir -p "$DEVELOPMENT_DIR"

# Descomprimir los módulos de Odoo según la versión seleccionada
ENTERPRISE_ZIP="/opt/scripts/odoo_base/enterprise/enterprise-$ODOO_WEB_VERSION.zip"

if [ -f "$ENTERPRISE_ZIP" ]; then
    echo "Descomprimiendo módulos Enterprise para Odoo $ODOO_WEB_VERSION..."
    unzip -q "$ENTERPRISE_ZIP" -d "$BASE_DIR"
else
    echo "No se encontró el archivo ZIP para la versión de Odoo $ODOO_WEB_VERSION: $ENTERPRISE_ZIP"
    exit 1
fi

# Ruta del archivo odoo.conf
ODOO_CONF_PATH="$APP_DATA_DIR/odoo/etc"
mkdir -p "$ODOO_CONF_PATH"

# Generación de una contraseña aleatoria para el admin
ADMIN_PASSWORD=$(openssl rand -base64 16)

# Crear el archivo odoo.conf con los parámetros solicitados
cat <<EOL > "$ODOO_CONF_PATH/odoo.conf"
[options]
admin_passwd = $ADMIN_PASSWORD
addons_path = /mnt/extra-addons/enterprise, /mnt/extra-addons/development
limit_time_cpu = 3600
limit_time_real = 3600
limit_request_body = 10000000000
EOL

# Mostrar mensaje de éxito
echo "Archivo 'odoo.conf' creado en $ODOO_CONF_PATH con la contraseña de admin generada."


# Levantar los contenedores con Docker Compose
cd "$PROJECT_DIR"
docker compose up -d

echo "Proyecto '$PROJECT_NAME' creado y contenedores levantados en los puertos $ODOO_POSTGRES_PORT (PostgreSQL) y $ODOO_WEB_PORT (Odoo)."
echo "Archivo .env y docker-compose.yml creados en $PROJECT_DIR"

# Obtener la IP de la máquina
IP_ADDRESS=$(hostname -I | awk '{print $1}')

# Mostrar la información de acceso
echo "Instancia de Odoo levantada con éxito. Aquí tienes la información relevante:"
echo "--------------------------------------------------------------"
echo "Dirección IP de la máquina: $IP_ADDRESS"
echo "Puerto de PostgreSQL: $ODOO_POSTGRES_PORT"
echo "Puerto de la Web (Odoo): $ODOO_WEB_PORT"
echo "Contraseña maestra de PostgreSQL: $POSTGRES_PASSWORD"
echo "Contraseña del administrador de Odoo: $ADMIN_PASSWORD"
echo "Accede a Odoo en el navegador en: http://$IP_ADDRESS:$ODOO_WEB_PORT"
echo "--------------------------------------------------------------"

# Mostrar como generar un nuevo modulo con Scaffold
echo "Para generar un nuevo modulo con el commando Scaffold, ejecuta el siguiente comando:"
echo "Reemplazar 'nombre_modulo' por el nombre de tu modulo"
echo "docker exec -it $DOCKER_CONTAINER_NAME_WEB odoo scaffold nombre_modulo /mnt/extra-addons/development"

# Mostrar el log del contenedor de Odoo
echo "Para ver el log del contenedor de Odoo, ejecuta el siguiente comando:"
echo "docker logs -f $DOCKER_CONTAINER_NAME_WEB"

# Comando para reiniciar el contenedor de Odoo
echo "Para reiniciar el contenedor de Odoo, ejecuta el siguiente comando:"
echo "docker restart $DOCKER_CONTAINER_NAME_WEB"

# Comando para detener el contenedor de Odoo
echo "Para detener el contenedor de Odoo, ejecuta el siguiente comando:"
echo "docker stop $DOCKER_CONTAINER_NAME_WEB"

# Comando para reiniciar el contenedor de la base de datos
echo "Para reiniciar el contenedor de PostgreSQL, ejecuta el siguiente comando:"
echo "docker restart $DOCKER_CONTAINER_NAME_DATABASE"

# Comando para detener el contenedor de la base de datos
echo "Para detener el contenedor de PostgreSQL, ejecuta el siguiente comando:"
echo "docker stop $DOCKER_CONTAINER_NAME_DATABASE"