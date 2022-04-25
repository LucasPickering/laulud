terraform {
  backend "gcs" {
    bucket = "keskne-tfstate"
    prefix = "laulud"
  }
}
