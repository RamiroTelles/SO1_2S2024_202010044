Comandos para crear el grafana - prometheus

Repositorio a conseguir con helm

```bash
helm repo add prometheus-community https://prometheus-community.github.io/helm-charts
helm repo update
```
Instalacion de grafana- prometheus

```bash
helm install my-kube-prometheus-stack prometheus-community/kube-prometheus-stack
```
Se expone el pod de grafana 

```bash
kubectl expose service my-kube-prometheus-stack-grafana --type=NodePort --target-port=3000 --name=grafana-node-port-service
```
Se obtiene la ip del nodo con el siguiente comando y se ingresa a grafana

```bash
kubectl get nodes -o wide


```

Comando para conseguir contraseña de grafana

```bash
kubectl get secret --namespace default my-kube-prometheus-stack-grafana -o jsonpath="{.data.admin-password}" | base64 --decode ; echo
```
