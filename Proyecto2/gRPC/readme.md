Docu en teoria 


Comando para inicializar el cluster de kubernetes con nombre proyecto2

```bash
gcloud container clusters create proyecto2 --num-nodes=4 --region=us-west1-a --tags=allin,allout --machine-type=e2-medium --no-enable-network-policy --disk-size=25GB --disk-type pd-standard
```