variable "gcp_project" {
  default = "discord-tts-bot-individual"
}
variable "billing_id" {}

provider "google" {
  project = var.gcp_project
}

resource "google_project" "gcp_project" {
  name                = var.gcp_project
  project_id          = var.gcp_project
  billing_account     = var.billing_id
  auto_create_network = false
}

resource "google_project_service" "tts" {
  project = google_project.gcp_project.project_id
  service = "texttospeech.googleapis.com"
}