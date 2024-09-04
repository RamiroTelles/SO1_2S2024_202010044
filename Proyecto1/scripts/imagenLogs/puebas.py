import json
import matplotlib.pyplot as plt
import numpy as np
from matplotlib.dates import ConciseDateFormatter


ruta_memory = "./logs/memory.json"
ruta_cont = "./logs/logs.json"
    

try:
    with open(ruta_memory, 'r', encoding='utf-8') as file_memory:
       datos_memory = json.load(file_memory) 
        
    with open(ruta_cont,'r',encoding='utf-8') as file_cont:
        datos_cont = json.load(file_cont)

    #print(datos_memory)
    #print("\n")
    #print("\n")
    #print(datos_cont)
except:
    file_memory.close()
    file_cont.close()
    print("error,poner return")
file_memory.close()
file_cont.close()
   

#tomar datos memory
x_memory = range(len(datos_memory))
y_memory = []

for elem in datos_memory:
    y_memory.append( (elem.get('usage_ram',0)/elem.get('total_ram',0)) * 100)

#tomar datos Contenedores

x_cont= []
y_cont = []


#y_cont.append(len(x_cont))
actualDate= datos_cont[0].get('timestamp','0')
for elem in datos_cont:
    # if actualDate == elem.get('timestamp','0'):
    #     x_cont[len(x_cont)-1] = x_cont[len(x_cont)-1]+1
    # else:
    #     x_cont.append(0)
    #     y_cont.append(len(x_cont))
    x_cont.append(elem.get('memory_usage','0'))
    y_cont.append(elem.get('cpu_usage','0'))

#print(x_cont)
#print(y)
#hacer grafica
fig_memory, ax_memory = plt.subplot_mosaic([['left','right'],['left','right']],layout='constrained')  
ax_memory['left'].set_xlabel('Iteraciones')
ax_memory['left'].set_ylabel('Porcentaje uso Memoria')  
ax_memory['left'].set_title('Uso de memoria')         # Create a figure containing a single Axes.
ax_memory['left'].plot(x_memory, y_memory)  # Plot some data on the Axes.

#fig_logs, ax_cont = plt.subplot()


ax_memory['right'].set_xlabel("Porcentaje de Uso")
ax_memory['right'].set_xlabel("No.Contenedor")
ax_memory['right'].set_title("Uso CPU y Memoria de Cada Contenedor")

ax_memory['right'].plot(range(len(x_cont)),x_cont, label='uso memoria')
ax_memory['right'].plot(range(len(y_cont)),y_cont, label='uso cpu')
ax_memory['right'].legend()

plt.savefig("./graphs/memoria.png")




print("return graficas creadas")