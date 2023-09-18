# Copyright (c) HashiCorp, Inc.
# SPDX-License-Identifier: MPL-2.0

job "rust-wasmedge-fuzzysearch-http" {
  datacenters = ["dc1"]
  #type        = "batch"

  group "rust-wasmedge-fuzzysearch-http" {
    network {
      port "http" { }
    }

    service {
      name = "rust-wasmedge-fuzzysearch-http"
      port = "http"
      provider = "nomad"
      tags = [
        "traefik.enable=true",
        "traefik.http.routers.rust-wasmedge-fuzzysearch-http.rule=Host(`rust-wasmedge-fuzzysearch-http.nomadi.toramolampi.com`)",
        "traefik.http.services.rust-wasmedge-fuzzysearch-http.loadbalancer.server.port=${NOMAD_PORT_http}"
      ]
    }
    task "rust-wasmedge-fuzzysearch-http" {
      driver = "wasmedge"

      config {
        extra_args = "--env PORT=${NOMAD_PORT_http}"
        binary = "/home/nomad/rust-wasmedge/fuzzysearch_http/build/main.wasm"
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
