Comando para crear el deployment kafka:

```bash
#crea el controlador de streamzi
kubectl create -f 'https://strimzi.io/install/latest?namespace=sopes1' -n sopes1

#crea un pod unico de kafka 
kubectl apply -f https://strimzi.io/examples/latest/kafka/kraft/kafka-single-node.yaml -n sopes1
```

se remplaza la ultima parte del link namespace=$(Namespaces) 
con el namespace deseado y el -n $namespace

para eliminar

```bash
#elimina cluster kafka
kubectl -n sopes1 delete $(kubectl get strimzi -o name -n sopes1)

#elimina el volumen de datos que crea, importante borrar para no cobro
kubectl delete pvc -l strimzi.io/name=my-cluster-kafka -n sopes1

#elimina el controlador de streamzi
kubectl -n sopes1 delete -f 'https://strimzi.io/install/latest?namespace=sopes1'
```
