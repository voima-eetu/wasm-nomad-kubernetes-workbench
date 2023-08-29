# Copyright (c) HashiCorp, Inc.
# SPDX-License-Identifier: MPL-2.0

job "rust-wasmedge-n-body-socket" {
  datacenters = ["dc1"]
  #type        = "batch"

  group "rust-wasmedge-n-body-socket" {
    network {
      port "http" { }
    }

    service {
      name = "rust-wasmedge-n-body-socket"
      port = "http"
      provider = "nomad"
      tags = [
        "traefik.enable=true",
        "traefik.http.routers.wasmedge.rule=Host(`rust-wasmedge-n-body-socket.nomadi.toramolampi.com`)",
        "traefik.http.services.wasmedge.loadbalancer.server.port=${NOMAD_PORT_http}"
      ]
    }
    task "rust-wasmedge-n-body-socket" {
      driver = "wasmedge"

      config {
        extra_args = "--env PORT=${NOMAD_PORT_http}"
        binary = "/home/nomad/rust-wasmedge/n_body_socket/build/main.wasm"
        env = {
          PORT = "${NOMAD_PORT_http}"
        }
      }
      env {
        PORT = "${NOMAD_PORT_http}"
      }
    }
  }
}
