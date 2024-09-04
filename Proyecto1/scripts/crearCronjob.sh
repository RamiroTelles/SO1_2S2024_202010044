#!/bin/bash


cron_command="*/1 * * * * cd /home/rami/LabSopes/SO1_2S2024_202010044/Proyecto1/scripts; ./generarCon.sh"

# Verifica si el cron job ya existe para evitar duplicados
(crontab -l | grep -F "$cron_command") || (crontab -l; echo "$cron_command") | crontab -
