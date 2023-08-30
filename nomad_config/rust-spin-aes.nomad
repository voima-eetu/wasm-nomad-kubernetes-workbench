# Copyright (c) HashiCorp, Inc.
# SPDX-License-Identifier: MPL-2.0

job "rust-spin-aes" {
  datacenters = ["dc1"]

  group "rust-spin-aes" {
    network {
      port "http" { }
    }

    service {
      name = "rust-spin-aes"
      port = "http"
      provider = "nomad"
      tags = [
        "traefik.enable=true",
        "traefik.http.routers.rust-spin-aes.rule=Host(`rust-spin-aes.nomadi.toramolampi.com`)",
        "traefik.http.services.rust-spin-aes.loadbalancer.server.port=${NOMAD_PORT_http}"
      ]
    }
    task "rust-spin-aes" {
      driver = "spin"
      env {
        RUST_LOG   = "spin=trace"
      }
      config {
        listen = "${NOMAD_IP_http}:${NOMAD_PORT_http}"
        file = "/home/nomad/rust-spin/aes/build/spin.toml"
      }
    }
  }
}
