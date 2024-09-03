from fastapi import FastAPI # type: ignore
import os
import json
from typing import List
from models.models import logContainer
from models.models import logMemory

app = FastAPI()


@app.get("/")
def read_root():
    return {"Hello": "World"}

@app.get("/graph")
def get_logs():
    return {"Graficas": "Creadas"}


@app.post("/logs")
def get_logs(logs_proc: List[logContainer]):
    logs_file = 'logs/logs.json'
    
    # Checamos si existe el archivo logs.json
    if os.path.exists(logs_file):
        # Leemos el archivo logs.json
        with open(logs_file, 'r') as file:
            existing_logs = json.load(file)
    else:
        # Sino existe, creamos una lista vacía
        existing_logs = []

    # Agregamos los nuevos logs a la lista existente
    new_logs = [log.dict() for log in logs_proc]
    existing_logs.extend(new_logs)

    # Escribimos la lista de logs en el archivo logs.json
    with open(logs_file, 'w') as file:
        json.dump(existing_logs, file, indent=4)

    return {"received": True}

@app.post("/memory")
def get_memory(logs_memory1: List[logMemory]):
    logs_file = 'logs/memory.json'
    
    # Checamos si existe el archivo logs.json
    if os.path.exists(logs_file):
        # Leemos el archivo logs.json
        with open(logs_file, 'r') as file:
            existing_logs = json.load(file)
    else:
        # Sino existe, creamos una lista vacía
        existing_logs = []

    # Agregamos los nuevos logs a la lista existente
    #print(log_memory1)
    new_logs = [log.dict() for log in logs_memory1]
    existing_logs.extend(new_logs)
    #print(existing_logs)
    

    # # Escribimos la lista de logs en el archivo logs.json
    with open(logs_file, 'w') as file:
        json.dump(existing_logs, file, indent=4)
        

    return {"received": True}

