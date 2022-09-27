resource "helm_release" "laulud" {
  name             = "laulud"
  chart            = "../helm"
  namespace        = var.kube_namespace
  create_namespace = true

  set {
    name  = "hostname"
    value = var.hostname
  }
  set {
    name  = "versionSha"
    value = var.version_sha
  }

  # Secrets
  set_sensitive {
    name  = "apiSecretKey"
    value = base64encode(random_password.api_secret_key.result)
  }
  set_sensitive {
    name  = "spotifyClientId"
    value = var.spotify_client_id
  }
  set_sensitive {
    name  = "spotifyClientSecret"
    value = var.spotify_client_secret
  }
}

# Import data from CI tf
data "terraform_remote_state" "ci" {
  backend = "gcs"

  config = {
    bucket = "beta-spray-tfstate"
    prefix = "ci"
  }
}

# TODO encrypt tfstate https://www.terraform.io/language/settings/backends/gcs#encryption_key

resource "random_password" "api_secret_key" {
  # Rocket wants a 44-char base64 key, which happens to come from a 33-char
  # string. We'll base64 encode this before passing to helm
  length = 33
}
