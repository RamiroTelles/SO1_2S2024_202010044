# Documentacion

# Proyecto 1 Organizacion de Lenguajes y Compiladores 1

## **Objetivos e Información del Sistema**

Este proyecto trata de hacer un sistema que monitorice las olimpiadas de la Universidad de San Carlos de Guatemala, aplicando conocimientos sobre GKE y Kubernetes.Este Proyecto se trata de un cluster de kubernetes el cual aplica distintas maneras de comunicación como gRPC, http, kafka; tambien posee control del trafico hecho por un ingress, guardado de datos con Redis, y monitoreo del sistema hecho con grafana y Prometheus.


## Requisitos del sistema

------------
*  Procesador AMD Ryzen 5 3400G o superior
* 16 Gb Ram
* Gráficos Radeon RX 6600 o superior
* Conexión a internet
* Navegador Web
* Java

## Iniciar el cluster

Comando para iniciar el cluster de GCP

```bash
gcloud container clusters create proyecto2 --num-nodes=4 --region=us-west1-a --tags=allin,allout --machine-type=e2-medium --no-enable-network-policy --disk-size=25GB --disk-type pd-standard
```
## Estructura general del Proyecto

![Estructura Proyecto](./imgs-docu/image.png)
## Deployments Principales

### Ingress

El encargado de controlar el trafico de entrada al cluster de kubernetes redirigiendo el ingreso a el cliente de rust o el de golang, creado con ayuda de plantilla de helm .

Configuracion:

instalar el ingress-nginx

```bash
helm upgrade --install ingress-nginx ingress-nginx \
  --repo https://kubernetes.github.io/ingress-nginx \
  --namespace ingress-nginx --create-namespace
```
Configurar ingress

```bash
kubectl apply .f ./kubernetes/ingress.yaml
```

### Go Client

Cliente de go el cual recibe el trafico entrante y lo manda a otro deployment en go por medio de gRPC con concurrencia. Cuenta con autoescalado horizontal. 

Deployment
```bash
kubectl apply .f ./kubernetes/goClient.yaml
```
### Rust Client

Cliente de rust el cual recibe el trafico entrante y lo manda a otro deployment en go por medio de gRPC con concurrencia.Cuenta con autoescalado horizontal.

Deployment
```bash
kubectl apply .f ./kubernetes/rustClient.yaml
```

### Servidores gRPC

Se encargan de recibir las peticiones gRPC de los clientes rust Client y go Client y envian estos datos a traves de kafka con el topico winner o loser.

Deployment
```bash
kubectl apply .f ./kubernetes/goServer-Swiming.yaml

kubectl apply .f ./kubernetes/goServer-Running.yaml

kubectl apply .f ./kubernetes/goServer-Boxing.yaml
```

### Deployment kafka

Es una plataforma open source para la distribucion de datos, la cual permite publicar, almacenar, procesar y suscribirse a eventos. En este proyeco transmite mensajes utilizando los topicos winner y loser.

Desplegado con chart de streamzi

```bash
kubectl create -f 'https://strimzi.io/install/latest?namespace=($namespace)' -n ($ namespace)
```

### Deployment Consumer-winner

Obtiene los mensajes de kafka con el topico winner, y envia estos datos a Redis.

Deplyment

```bash
kubectl apply .f ./kubernetes/consumer.yaml
```

### Deployment Consumer-winner

Obtiene los mensajes de kafka con el topico loser, y envia estos datos a Redis.

Deployment

```bash
kubectl apply .f ./kubernetes/consumer-loser.yaml
```

### Redis 

Se utiliza en este proyecto para guardar los valores que han sido enviados al cluster. Se almacenan en un tipo hash.

Deployment

```bash
kubectl apply .f ./kubernetes/redis.yaml
```

### Grafana y Prometheus

Utilizados para el monitoreo del cluster. Prometheus obtiene los valores del sistema del cluster. Mientras que grafana agrupa estos datos, y los datos dentro de Redis y los muestra en unos DashBoards para un correcto monitoreo del cluster.

Imagen DashBoards

Monitoreo del sistema del cluster

![Dashboard1](./imgs-docu/grafana-prometheus1.png)

Datos recibidos por el cluster

![Dashboard2](./imgs-docu/grafana-redis.png)

### Locust

Herramienta para el correcto testeo para una carga de peticiones http al cluster.

Al inicio debemos especificar cuantos usuarios queremos, el ratio de spawn y el host que recibirá las peticiones.

![Locust-Inicio](./imgs-docu/locust-Inicio.png)

Al momento de iniciar podremos ver distintos como la latencia media, numero de peticiones, numero de fallos , etc.

![locust-testeo](./imgs-docu/locust-testeo.png)


