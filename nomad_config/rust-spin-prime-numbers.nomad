# Copyright (c) HashiCorp, Inc.
# SPDX-License-Identifier: MPL-2.0

job "rust-spin-prime-numbers" {
  datacenters = ["dc1"]

  group "rust-spin-prime-numbers" {
    network {
      port "http" { }
    }

    service {
      name = "rust-spin-prime-numbers"
      port = "http"
      provider = "nomad"
      tags = [
        "traefik.enable=true",
        "traefik.http.routers.wasmedge.rule=Host(`rust-spin-prime-numbers.nomadi.toramolampi.com`)",
        "traefik.http.services.wasmedge.loadbalancer.server.port=${NOMAD_PORT_http}"
      ]
    }
    task "rust-spin-prime-numbers" {
      driver = "spin"
      env {
        RUST_LOG   = "spin=trace"
      }
      config {
        listen = "${NOMAD_IP_http}:${NOMAD_PORT_http}"
        file = "/home/nomad/rust-spin/prime_numbers/build/spin.toml"
      }
    }
  }
}
