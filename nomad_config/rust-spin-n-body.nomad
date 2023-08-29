# Copyright (c) HashiCorp, Inc.
# SPDX-License-Identifier: MPL-2.0

job "rust-spin-n-body" {
  datacenters = ["dc1"]

  group "rust-spin-n-body" {
    network {
      port "http" { }
    }

    service {
      name = "rust-spin-n-body"
      port = "http"
      provider = "nomad"
      tags = [
        "traefik.enable=true",
        "traefik.http.routers.wasmedge.rule=Host(`rust-spin-n-body.nomadi.toramolampi.com`)",
        "traefik.http.services.wasmedge.loadbalancer.server.port=${NOMAD_PORT_http}"
      ]
    }
    task "rust-spin-n-body" {
      driver = "spin"
      env {
        RUST_LOG   = "spin=trace"
      }
      config {
        listen = "${NOMAD_IP_http}:${NOMAD_PORT_http}"
        file = "/home/nomad/rust-spin/n_body/build/spin.toml"
      }
    }
  }
}
