#!/bin/bash

# test.sh - Script para mostrar información de ROM y comando
# Uso: ./test.sh "nombre_de_la_rom" "comando_a_ejecutar"

ROM_NAME="$1"
COMMAND="$2"

# Mostrar notificación con los datos recibidos
notify-send "Emux Launcher" "ROM: $ROM_NAME\nCommand: $COMMAND"
