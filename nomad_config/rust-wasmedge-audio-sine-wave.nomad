# Copyright (c) HashiCorp, Inc.
# SPDX-License-Identifier: MPL-2.0

job "rust-wasmedge-audio-sine-wave" {
  datacenters = ["dc1"]
  #type        = "batch"

  group "rust-wasmedge-audio-sine-wave" {
    network {
      port "http" { }
    }

    service {
      name = "rust-wasmedge-audio-sine-wave"
      port = "http"
      provider = "nomad"
      tags = [
        "traefik.enable=true",
        "traefik.http.routers.rust-wasmedge-audio-sine-wave.rule=Host(`rust-wasmedge-audio-sine-wave.nomadi.toramolampi.com`)",
        "traefik.http.services.rust-wasmedge-audio-sine-wave.loadbalancer.server.port=${NOMAD_PORT_http}"
      ]
    }
    task "rust-wasmedge-audio-sine-wave" {
      driver = "wasmedge"

      config {
        extra_args = "--env PORT=${NOMAD_PORT_http}"
        binary = "/home/nomad/rust-wasmedge/audio_sine_wave/build/main.wasm"
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
