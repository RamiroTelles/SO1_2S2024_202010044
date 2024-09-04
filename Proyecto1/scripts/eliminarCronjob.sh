#!/bin/bash


cron_command="*/1 * * * * cd /home/rami/LabSopes/SO1_2S2024_202010044/Proyecto1/scripts; ./generarCon.sh"

# Filtra los cron jobs actuales y elimina el que coincide con 'cron_command'
(crontab -l | grep -v -F "$cron_command") | crontab -
