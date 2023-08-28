# Follower config file
datacenter = "dc1"
data_dir   = "/opt/nomad/data"
bind_addr  = "0.0.0.0"
name = "nomad2"


server {
  enabled          = true
  bootstrap_expect = 1
}   
      
client {
  enabled = true
  servers = [
    "10.223.6.50", 
    "10.223.6.51", 
    "10.223.6.52",
  ]
}

plugin_dir = "/opt/nomad/plugins"

plugin "wasmedge-driver" {
  config {
    path = "/usr/local/bin/wasmedge"
  }
}

plugin "spin" {
  config {
    path = "/usr/local/spin/spin"
  }
}

plugin "containerd-driver" {
  config {
    containerd_runtime = "io.containerd.runc.v2"
  }
}

ui {
  enabled = false
}


