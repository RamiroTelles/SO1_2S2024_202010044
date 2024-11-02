Comando a utilizar


instalar helm

```bash
sudo dnf install helm #fedora
# ubuntu
curl https://baltocdn.com/helm/signing.asc | gpg --dearmor | sudo tee /usr/share/keyrings/helm.gpg > /dev/null
sudo apt-get install apt-transport-https --yes
echo "deb [arch=$(dpkg --print-architecture) signed-by=/usr/share/keyrings/helm.gpg] https://baltocdn.com/helm/stable/debian/ all main" | sudo tee /etc/apt/sources.list.d/helm-stable-debian.list
sudo apt-get update
sudo apt-get install helm
```

instalar el ingress-nginx

```bash
helm upgrade --install ingress-nginx ingress-nginx \
  --repo https://kubernetes.github.io/ingress-nginx \
  --namespace ingress-nginx --create-namespace
```


Luego correr el siguiente comando para obtener la ip
```bash
kubectl get all -n ingress-nginx
```

copiar el external ip y remplazarlo en el host: 
del yaml ingress.yaml en la carpeta cubernetes y correrlo


luego correr el ingress.yaml en la carpeta kubernetes
