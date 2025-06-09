#terraform {
#  required_providers {
#    docker = {
#      source  = "kreuzwerker/docker"
#      version = "~> 3.0"
#    }
#  }
#}
#
#resource "azurerm_resource_group" "chat_rg" {
#    name = var.resource_group_name
#    location = var.location
#}
#
#resource "azurerm_postgresql_flexible_server" "pg" {
#    name = "chatpg-${var.env}"
#    location =  azurerm_resource_group.chat_rg.location
#    resource_group_name = azurerm_resource_group.chat_rg.name
#    administrator_login = var.pg_admin
#    administrator_password = var.pg_password
#    sku_name = "B1ms"
#    storage_mb = 32768
#    version = "14"
#}
#
#resource "azurerm_cosmosdb_account" "mongo" {
#    name = "chatmongo-${var.env}"
#    location = azurerm_resource_group.chat_rg.location
#    resource_group_name = azurerm_resource_group.chat_rg.name
#    offer_type = "Standard"
#    capabilities {
#        name = "EnableMongo"
#    }
#    consistency_policy {
#        consistency_level = "Session"
#    }
#}
#
#provider "docker" {}
#
#resource "docker_image" "coturn" {
#  name         = "instrumentisto/coturn"
#  keep_locally = false
#}
#
#
#resource "docker_container" "turn_server" {
#  name  = "turn"
#  image = docker_image.coturn.latest
#  ports {
#    internal = 3478
#    external = 3478
#  }
#  ports {
#    internal = 3478
#    external = 3478
#    protocol = "udp"
#  }
#  env = [
#    "TURNSERVER_PORT=3478",
#    "TURNSERVER_VERBOSE=true"
#  ]
#}
#
#provider "kubernetes" {
#    config_path = "~/.kube/config"
#}

terraform {
    required_providers {
        kubernetes = {
            source = "hashicorp/kubernetes"
            version = "~> 2.0"
        }
    }
}

provider "kubernetes" {
    config_path = "~/.kube/config"
}
