#!/bin/sh

##this is supposed to be ran like this:
# `find . -name main.wasm -type f -exec ./gen_kube_templ.sh "{}" \;`

WASM_PATH=$(echo $1 | sed -e 's|\./||g')
CONTAINER_NAME=$(echo $1 | xargs dirname | xargs dirname | awk -F/ '{print $NF}')
NAME=$(echo $1 | xargs dirname | xargs dirname | awk -F/ '{print $(NF-1)"-"$NF}' | sed -e 's/_/-/g')

PORT=80

echo $NAME
echo $CONTAINER_NAME

case $NAME in
	*"wasmedge"*)
		cat  << EOF > ../configs/kubernetes/$NAME.yaml
apiVersion: node.k8s.io/v1
kind: RuntimeClass
metadata:
  name: crun
handler: crun
---
apiVersion: apps/v1
kind: Deployment
metadata:
  name: $NAME
spec:
  replicas: 1
  selector:
    matchLabels:
      app: $NAME
  template:
    metadata:
      labels:
        app: $NAME
      annotations:
        module.wasm.image/variant: compat-smart
    spec:
      runtimeClassName: crun
      containers:
      - name: $NAME
        image: 10.223.6.99:5000/wasmedge/rust/$CONTAINER_NAME:v2
        command: ["./main.wasm"]
        args: ["--env", "PORT=$PORT"]
        env:
        - name: PORT
          value: "$PORT"
        ports:
        - containerPort: $PORT
EOF
;;

	*"spin"*)
		SPIN_PATH=$(echo $WASM_PATH | sed -e 's/main.wasm/spin.toml/g')
		cat  << EOF > ../configs/kubernetes/$NAME.yaml
apiVersion: node.k8s.io/v1
kind: RuntimeClass
metadata:
  name: wasmtime-spin
handler: spin
---
apiVersion: apps/v1
kind: Deployment
metadata:
  name: $NAME
spec:
  replicas: 1
  selector:
    matchLabels:
      app: $NAME
  template:
    metadata:
      labels:
        app: $NAME
    spec:
      runtimeClassName: wasmtime-spin
      containers:
      - name: $NAME
        image: 10.223.6.99:5000/spin/rust/$CONTAINER_NAME:v2
        command: ["/"]
        ports:
        - containerPort: $PORT
EOF
;;
esac

cat << EOF >> ../configs/kubernetes/$NAME.yaml
---
apiVersion: v1
kind: Service
metadata:
  name: $NAME
spec:
  ports:
    - protocol: TCP
      port: $PORT
      targetPort: $PORT
      appProtocol: http
  selector:
    app: $NAME
---
apiVersion: networking.k8s.io/v1
kind: Ingress
metadata:
  name: $NAME
  annotations:
    ingress.kubernetes.io/ssl-redirect: "false"
    ingressClassName: traefik
spec:
  rules:
    - host: $NAME.kubi.toramolampi.com
      http:
        paths:
          - path: /
            pathType: Prefix
            backend:
              service:
                name: $NAME
                port:
                  number: $PORT
EOF
