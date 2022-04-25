This Terraform handles the helm deployment of the app into Keskne.

## First Time Setup

### Prereqs

- helm
- kubectl
- doctl
- terraform

### Setup

Run:

```sh
doctl auth init
# do the login
cd deploy/terraform
terraform init
```

### Kubectl

To point `kubectl` at the DOKS cluster:

```sh
doctl kubernetes cluster kubeconfig save keskne
```

### Release

This works for both initial and subsequent releases:

```sh
terraform apply
# It will ask for a version SHA, use the output of `git rev-parse origin/master`
```
