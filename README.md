# Kata (Kubernetes Data)

Kubernetes controller experiment that manages ELT analytics pipelines

## Execute locally

You'll need `kind` and `rust`.

1. Create a local k8s cluster

```bash
sudo kind create cluster
sudo kind export kubeconfig --kubeconfig $HOME/.kube/config                                                                                                                                           │
```

2. Run the server

```bash
cargo run
```