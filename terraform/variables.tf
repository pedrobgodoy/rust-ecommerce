variable "environment" {
  type    = string
  default = "dev"
}

variable "region" {
  type    = string
  default = "us-east-1"
}

variable "tag_project" {
  type    = string
  default = "rust-ecommerce"
}

variable "cidr_block" {
  type    = string
  default = "172.16.0.0/16"
}

variable "cidr_block_public-1a" {
  type    = string
  default = "172.16.8.0/21"
}

variable "cidr_block_public-1b" {
  type    = string
  default = "172.16.16.0/21"
}

variable "cidr_block_public-1c" {
  type    = string
  default = "172.16.24.0/21"
}

variable "cidr_block_private-1a" {
  type    = string
  default = "172.16.32.0/21"
}

variable "cidr_block_private-1b" {
  type    = string
  default = "172.16.40.0/21"
}

variable "cidr_block_private-1c" {
  type    = string
  default = "172.16.48.0/21"
}

variable "az-public-1a" {
  type    = string
  default = "us-east-1a"
}

variable "az-public-1b" {
  type    = string
  default = "us-east-1b"
}

variable "az-public-1c" {
  type    = string
  default = "us-east-1c"
}

variable "az-private-1a" {
  type    = string
  default = "us-east-1a"
}

variable "az-private-1b" {
  type    = string
  default = "us-east-1b"
}

variable "az-private-1c" {
  type    = string
  default = "us-east-1c"
}

variable "mq-user" {
  type    = string
  default = "usr_root"
}

variable "mq-password" {
  type    = string
  default = "admin"
}

variable "rds-user" {
  type    = string
  default = "usr_root"
}

variable "rds-password" {
  type    = string
  default = "admin"
}

variable "app-count" {
  type    = number
  default = 1
}
