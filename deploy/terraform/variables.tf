variable "hostname" {
  default     = "laulud.lucaspickering.me"
  description = "Address the webapp is hosted at"
  type        = string
}

variable "kube_config_path" {
  default     = "~/.kube/config"
  description = "Path to local Kubernetes config file"
  type        = string
}

variable "kube_namespace" {
  default     = "laulud"
  description = "Kubernetes namespace to deploy into"
  type        = string
}

variable "spotify_client_id" {
  description = "Spotify OAuth client ID"
  type        = string
}

variable "spotify_client_secret" {
  description = "Spotify OAuth client secret"
  type        = string
}

variable "version_sha" {
  description = "Git SHA of the version of the app to deploy. Typically the output of `git rev-parse origin/master`"
  type        = string
}
