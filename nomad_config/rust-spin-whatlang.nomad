# Copyright (c) HashiCorp, Inc.
# SPDX-License-Identifier: MPL-2.0

job "rust-spin-whatlang" {
  datacenters = ["dc1"]

  group "rust-spin-whatlang" {
    network {
      port "http" { }
    }

    service {
      name = "rust-spin-whatlang"
      port = "http"
      provider = "nomad"
      tags = [
        "traefik.enable=true",
        "traefik.http.routers.rust-spin-whatlang.rule=Host(`rust-spin-whatlang.nomadi.toramolampi.com`)",
        "traefik.http.services.rust-spin-whatlang.loadbalancer.server.port=${NOMAD_PORT_http}"
      ]
    }
    task "rust-spin-whatlang" {
      driver = "spin"
      env {
        RUST_LOG   = "spin=trace"
      }
      config {
        listen = "${NOMAD_IP_http}:${NOMAD_PORT_http}"
        file = "/home/nomad/rust-spin/whatlang/build/spin.toml"
      }
    }
  }
}
