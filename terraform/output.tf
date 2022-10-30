output "load_balancer_ip" {
  value = aws_lb.app_lb.dns_name
}

output "mq_endpoint" {
  value = aws_mq_broker.broker.instances.0.endpoints
}

output "rds_endpoint" {
    value = aws_db_instance.rds.endpoint
}