job "traefik" {
  datacenters = ["dc1"]
  type        = "service"

  group "traefik" {
    count = 3

    network {
      mode = "bridge"
      port  "http"{
         static = 80
      }
      port  "admin"{
         static = 8080
      }
    }

    service {
      name = "traefik-http"
      provider = "nomad"
      port = "http"
    }

    task "server" {
      driver = "containerd-driver"
      config {
        image = "docker.io/library/traefik:2.9.10"
        args = [
          "--api.dashboard=true",
          "--api.insecure=true",
          "--entrypoints.web.address=:${NOMAD_PORT_http}",
          "--entrypoints.traefik.address=:${NOMAD_PORT_admin}",
          "--providers.nomad=true",
          "--providers.nomad.endpoint.address=http://10.223.6.50:4646" 
        ]
      }
    }
  }
}

