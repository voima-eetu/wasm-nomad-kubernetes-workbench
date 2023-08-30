rm build.tar.gz
tar -czf build.tar.gz */*/build/ --transform 's,^,/home/nomad/,' --owner=nomad --group=nomad
