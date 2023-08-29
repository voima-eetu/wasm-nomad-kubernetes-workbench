# Copyright (c) HashiCorp, Inc.
# SPDX-License-Identifier: MPL-2.0

job "rust-spin-user-manager" {
  datacenters = ["dc1"]

  group "rust-spin-user-manager" {
    network {
      port "http" { }
    }

    service {
      name = "rust-spin-user-manager"
      port = "http"
      provider = "nomad"
      tags = [
        "traefik.enable=true",
        "traefik.http.routers.wasmedge.rule=Host(`rust-spin-user-manager.nomadi.toramolampi.com`)",
        "traefik.http.services.wasmedge.loadbalancer.server.port=${NOMAD_PORT_http}"
      ]
    }
    task "rust-spin-user-manager" {
      driver = "spin"
      env {
        RUST_LOG   = "spin=trace"
      }
      config {
        listen = "${NOMAD_IP_http}:${NOMAD_PORT_http}"
        file = "/home/nomad/rust-spin/user_manager/build/spin.toml"
      }
    }
  }
}
