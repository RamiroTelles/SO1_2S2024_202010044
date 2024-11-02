import json

import random
from locust import HttpUser, between, task



def getRandom(array):
    if len(array)>0:
        index = random.randint(0,len(array))
        return array[index]
    else:
        print("Array vacio")
        return None

class Reader():
    def __init__(self):
        self.array_ingenieria = []
        self.array_agronomia = []
        self.leerArchivo()


    def leerArchivo(self):
        try:
            print("")
            with open('datos.json','r') as file:
                data = json.load(file)
                self.array_ingenieria = data["arrayIngenieria"]
                self.array_agronomia = data["arrayAgronomia"]
                file.close()
        except:
            print("Error al leer archivo")
            file.close()


class MessageTraffic(HttpUser):
    wait_time = between(1, 3)
    r = Reader()
    

    def on_start(self):
        print(">> MessageTraffic: Inicio de envío de tráfico")

    @task
    def PostIngenieria(self):
        dato = getRandom(self.r.array_ingenieria)

        if ( dato is not None ):
            data_to_send = json.dumps(dato)
            print(data_to_send)
            self.client.post("/grpc-rust", json=dato)
        else:
            print(">> MessageTraffic: Envío finalizado")
            self.stop(True)

    @task
    def PostAgronomia(self):
        dato = getRandom(self.r.array_agronomia)
        if ( dato is not None ):
            data_to_send = json.dumps(dato)
            print(data_to_send)
            self.client.post("/grpc-go", json=dato)
        else:
            print(">> MessageTraffic: Envío finalizado")
            self.stop(True)




