#!/bin/sh

##this is supposed to be ran like this:
# `find . -name main.wasm -type f -exec ./gen_nomad_templ.sh "{}" \;`

WASM_PATH=$(echo $1 | sed -e 's|\./||g')
NAME=$(echo $1 | xargs dirname | xargs dirname | awk -F/ '{print $(NF-1)"-"$NF}')
echo $NAME

cat  << EOF > ../nomad_config/$NAME.nomad
# Copyright (c) HashiCorp, Inc.
# SPDX-License-Identifier: MPL-2.0

job "$NAME" {
  datacenters = ["dc1"]
  #type        = "batch"

  group "$NAME" {
    network {
      port "http" { }
    }

    service {
      name = "$NAME"
      port = "http"
      provider = "nomad"
      tags = [
        "traefik.enable=true",
        "traefik.http.routers.wasmedge.rule=Host(\`$NAME.nomadi.toramolampi.com\`)",
        "traefik.http.services.wasmedge.loadbalancer.server.port=\${NOMAD_PORT_http}"
      ]
    }
    task "$NAME" {
      driver = "wasmedge"

      config {
        extra_args = "--env PORT=\${NOMAD_PORT_http}"
        binary = "/home/nomad/$WASM_PATH"
        env = {
          PORT = "\${NOMAD_PORT_http}"
        }
      }
      env {
        PORT = "\${NOMAD_PORT_http}"
      }
    }
  }
}
EOF
