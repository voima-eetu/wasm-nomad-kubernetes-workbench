# Copyright (c) HashiCorp, Inc.
# SPDX-License-Identifier: MPL-2.0

job "rust-wasmedge-prime-numbers" {
  datacenters = ["dc1"]
  #type        = "batch"

  group "rust-wasmedge-prime-numbers" {
    network {
      port "http" { }
    }

    service {
      name = "rust-wasmedge-prime-numbers"
      port = "http"
      provider = "nomad"
      tags = [
        "traefik.enable=true",
        "traefik.http.routers.wasmedge.rule=Host(`rust-wasmedge-prime-numbers.nomadi.toramolampi.com`)",
        "traefik.http.services.wasmedge.loadbalancer.server.port=${NOMAD_PORT_http}"
      ]
    }
    task "rust-wasmedge-prime-numbers" {
      driver = "wasmedge"

      config {
        extra_args = "--env PORT=${NOMAD_PORT_http}"
        binary = "/home/nomad/rust-wasmedge/prime_numbers/build/main.wasm"
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
