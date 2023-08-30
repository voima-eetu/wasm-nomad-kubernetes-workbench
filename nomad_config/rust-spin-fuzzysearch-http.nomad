# Copyright (c) HashiCorp, Inc.
# SPDX-License-Identifier: MPL-2.0

job "rust-spin-fuzzysearch-http" {
  datacenters = ["dc1"]

  group "rust-spin-fuzzysearch-http" {
    network {
      port "http" { }
    }

    service {
      name = "rust-spin-fuzzysearch-http"
      port = "http"
      provider = "nomad"
      tags = [
        "traefik.enable=true",
        "traefik.http.routers.rust-spin-fuzzysearch-http.rule=Host(`rust-spin-fuzzysearch-http.nomadi.toramolampi.com`)",
        "traefik.http.services.rust-spin-fuzzysearch-http.loadbalancer.server.port=${NOMAD_PORT_http}"
      ]
    }
    task "rust-spin-fuzzysearch-http" {
      driver = "spin"
      env {
        RUST_LOG   = "spin=trace"
      }
      config {
        listen = "${NOMAD_IP_http}:${NOMAD_PORT_http}"
        file = "/home/nomad/rust-spin/fuzzysearch_http/build/spin.toml"
      }
    }
  }
}
