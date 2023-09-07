# Copyright (c) HashiCorp, Inc.
# SPDX-License-Identifier: MPL-2.0

job "rust-wasmedge-prime-numbers-socket" {
  datacenters = ["dc1"]
  #type        = "batch"

  group "rust-wasmedge-prime-numbers-socket" {
    network {
      port "http" { }
    }

    service {
      name = "rust-wasmedge-prime-numbers-socket"
      port = "http"
      provider = "nomad"
      tags = [
        "traefik.enable=true",
        "traefik.http.routers.rust-wasmedge-prime-numbers-socket.rule=Host(`rust-wasmedge-prime-numbers-socket.nomadi.toramolampi.com`)",
        "traefik.http.services.rust-wasmedge-prime-numbers-socket.loadbalancer.server.port=${NOMAD_PORT_http}"
      ]
    }
    task "rust-wasmedge-prime-numbers-socket" {
      driver = "wasmedge"

      config {
        extra_args = "--env PORT=${NOMAD_PORT_http}"
        binary = "/home/nomad/rust-wasmedge/prime_numbers_socket/build/main.wasm"
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
