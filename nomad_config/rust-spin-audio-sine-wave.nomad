# Copyright (c) HashiCorp, Inc.
# SPDX-License-Identifier: MPL-2.0

job "rust-spin-audio-sine-wave" {
  datacenters = ["dc1"]

  group "rust-spin-audio-sine-wave" {
    network {
      port "http" { }
    }

    service {
      name = "rust-spin-audio-sine-wave"
      port = "http"
      provider = "nomad"
      tags = [
        "traefik.enable=true",
        "traefik.http.routers.rust-spin-audio-sine-wave.rule=Host(`rust-spin-audio-sine-wave.nomadi.toramolampi.com`)",
        "traefik.http.services.rust-spin-audio-sine-wave.loadbalancer.server.port=${NOMAD_PORT_http}"
      ]
    }
    task "rust-spin-audio-sine-wave" {
      driver = "spin"
      env {
        RUST_LOG   = "spin=trace"
      }
      config {
        listen = "${NOMAD_IP_http}:${NOMAD_PORT_http}"
        file = "/home/nomad/rust-spin/audio_sine_wave/build/spin.toml"
      }
    }
  }
}
