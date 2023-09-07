#!/bin/sh

##this is supposed to be ran like this:
# `find . -name main.wasm -type f -exec ./gen_nomad_templ.sh "{}" \;`

WASM_PATH=$(echo $1 | sed -e 's|\./||g')
NAME=$(echo $1 | xargs dirname | xargs dirname | awk -F/ '{print $(NF-1)"-"$NF}' | sed -e 's/_/-/g')
echo $NAME

case $NAME in
	*"wasmedge"*)
		cat  << EOF > ../configs/nomad/$NAME.nomad
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
        "traefik.http.routers.$NAME.rule=Host(\`$NAME.nomadi.toramolampi.com\`)",
        "traefik.http.services.$NAME.loadbalancer.server.port=\${NOMAD_PORT_http}"
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
;;

	*"spin"*)
		SPIN_PATH=$(echo $WASM_PATH | sed -e 's/main.wasm/spin.toml/g')
		cat  << EOF > ../configs/nomad/$NAME.nomad
# Copyright (c) HashiCorp, Inc.
# SPDX-License-Identifier: MPL-2.0

job "$NAME" {
  datacenters = ["dc1"]

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
        "traefik.http.routers.$NAME.rule=Host(\`$NAME.nomadi.toramolampi.com\`)",
        "traefik.http.services.$NAME.loadbalancer.server.port=\${NOMAD_PORT_http}"
      ]
    }
    task "$NAME" {
      driver = "spin"
      env {
        RUST_LOG   = "spin=trace"
      }
      config {
        listen = "\${NOMAD_IP_http}:\${NOMAD_PORT_http}"
        file = "/home/nomad/$SPIN_PATH"
      }
    }
  }
}
EOF
;;
esac
