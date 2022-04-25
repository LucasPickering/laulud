

resource "helm_release" "beta_spray" {
  name             = "beta-spray"
  chart            = "../../helm"
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
  set {
    name  = "apiSecretKey"
    value = random_password.api_secret_key.result
  }
  set {
    name  = "spotifyClientId"
    value = var.spotify_client_id
  }
  set {
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
  length = 32
}
