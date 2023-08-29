# Copyright (c) HashiCorp, Inc.
# SPDX-License-Identifier: MPL-2.0

job "rust-wasmedge-fuzzysearch-http-socket" {
  datacenters = ["dc1"]
  #type        = "batch"

  group "rust-wasmedge-fuzzysearch-http-socket" {
    network {
      port "http" { }
    }

    service {
      name = "rust-wasmedge-fuzzysearch-http-socket"
      port = "http"
      provider = "nomad"
      tags = [
        "traefik.enable=true",
        "traefik.http.routers.wasmedge.rule=Host(`rust-wasmedge-fuzzysearch-http-socket.nomadi.toramolampi.com`)",
        "traefik.http.services.wasmedge.loadbalancer.server.port=${NOMAD_PORT_http}"
      ]
    }
    task "rust-wasmedge-fuzzysearch-http-socket" {
      driver = "wasmedge"

      config {
        extra_args = "--env PORT=${NOMAD_PORT_http}"
        binary = "/home/nomad/rust-wasmedge/fuzzysearch_http_socket/build/main.wasm"
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
