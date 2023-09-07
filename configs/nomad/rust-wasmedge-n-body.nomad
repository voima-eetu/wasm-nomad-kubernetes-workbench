# Copyright (c) HashiCorp, Inc.
# SPDX-License-Identifier: MPL-2.0

job "rust-wasmedge-n-body" {
  datacenters = ["dc1"]
  #type        = "batch"

  group "rust-wasmedge-n-body" {
    network {
      port "http" { }
    }

    service {
      name = "rust-wasmedge-n-body"
      port = "http"
      provider = "nomad"
      tags = [
        "traefik.enable=true",
        "traefik.http.routers.rust-wasmedge-n-body.rule=Host(`rust-wasmedge-n-body.nomadi.toramolampi.com`)",
        "traefik.http.services.rust-wasmedge-n-body.loadbalancer.server.port=${NOMAD_PORT_http}"
      ]
    }
    task "rust-wasmedge-n-body" {
      driver = "wasmedge"

      config {
        extra_args = "--env PORT=${NOMAD_PORT_http}"
        binary = "/home/nomad/rust-wasmedge/n_body/build/main.wasm"
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
