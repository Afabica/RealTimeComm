resource "kubernetes_namespace" "dev" {
    metadata {
        name = var.namespace
    }
}

# Rust backend 
resource "kubernetes_deployment" "rust_backend" {
    metadata {
        name = "rust-backend"
        namespace = var.namespace
    }
    spec {
        replicas = 1
        selector {
            match_labels = {
                app = "rust-backend"
            }
        }
        template {
            metadata {
                labels = {
                    app = "rust-backend"
                }
            }
            spec {
                container {
                    image = "rust-backend:local"
                    name = "rust"
                    port {
                        container_port = 8000
                    }
                }
            }
        }
    }
}

resource "kubernetes_service" "rust_backend" {
    metadata {
        name = "frontend"
        namespace = var.namespace 
    }
    spec {
        replicas = 1
        selector {
            match_labels = {
                app = "frontend"
            }
        }
        template {
            metadata {
                labels = {
                    app = "frontend"
                }
            }
            spec {
                container {
                    image = "nextjs-frontend:local"
                    name = "nextjs"
                    port {
                        container_port = 3000
                    }
                }
            }
        }
    }
}

resource "kubernetes_service" "frontend" {
    metadata {
        name = "frontend"
        namespace = var.namespace 
    } 
    spec {
        selector = {
            app = "frontend"
        }
        port {
            port = 80
            target_port = 3000
        }
        type = "NodePort"
    }
}
