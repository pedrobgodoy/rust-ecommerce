terraform {
  required_providers {
    aws = {
      source  = "hashicorp/aws"
      version = "~> 3.27"
    }
  }
}

provider "aws" {
  profile = "terraform-solus"
  region  = "us-east-1"
}

# Network
resource "aws_vpc" "main" {
  cidr_block = var.cidr_block

  tags = {
    "Project" = var.tag_project,
    "Name"    = "vpc-${var.tag_project}",
  }
}

resource "aws_internet_gateway" "igw" {
  vpc_id = aws_vpc.main.id

  tags = {
    "Project" = var.tag_project,
    "Name"    = "igw-${var.tag_project}",
  }
}

resource "aws_eip" "eip" {
  vpc = true

  tags = {
    "Project" = var.tag_project,
    "Name"    = "eip-${var.tag_project}",
  }
}

resource "aws_nat_gateway" "nat" {
  allocation_id = aws_eip.eip.id
  subnet_id     = aws_subnet.public-1a.id

  tags = {
    "Project" = var.tag_project,
    "Name"    = "nat-${var.tag_project}",
  }

  depends_on = [aws_internet_gateway.igw]
}

resource "aws_route_table" "private" {
  vpc_id = aws_vpc.main.id

  route = [
    {
      cidr_block                 = "0.0.0.0/0"
      nat_gateway_id             = aws_nat_gateway.nat.id
      carrier_gateway_id         = ""
      destination_prefix_list_id = ""
      egress_only_gateway_id     = ""
      gateway_id                 = ""
      instance_id                = ""
      ipv6_cidr_block            = ""
      local_gateway_id           = ""
      network_interface_id       = ""
      transit_gateway_id         = ""
      vpc_endpoint_id            = ""
      vpc_peering_connection_id  = ""
    },
  ]

  tags = {
    "Project" = var.tag_project,
    "Name"    = "route-priv-${var.tag_project}",
  }
}

resource "aws_route_table" "public" {
  vpc_id = aws_vpc.main.id

  route = [
    {
      cidr_block                 = "0.0.0.0/0"
      gateway_id                 = aws_internet_gateway.igw.id
      nat_gateway_id             = ""
      carrier_gateway_id         = ""
      destination_prefix_list_id = ""
      egress_only_gateway_id     = ""
      instance_id                = ""
      ipv6_cidr_block            = ""
      local_gateway_id           = ""
      network_interface_id       = ""
      transit_gateway_id         = ""
      vpc_endpoint_id            = ""
      vpc_peering_connection_id  = ""
    },
  ]

  tags = {
    "Project" = var.tag_project,
    "Name"    = "route-pub-${var.tag_project}",
  }
}

resource "aws_route_table_association" "private-1a" {
  subnet_id      = aws_subnet.private-1a.id
  route_table_id = aws_route_table.private.id
}

resource "aws_route_table_association" "private-1b" {
  subnet_id      = aws_subnet.private-1b.id
  route_table_id = aws_route_table.private.id
}

resource "aws_route_table_association" "private-1c" {
  subnet_id      = aws_subnet.private-1c.id
  route_table_id = aws_route_table.private.id
}

resource "aws_route_table_association" "public-1a" {
  subnet_id      = aws_subnet.public-1a.id
  route_table_id = aws_route_table.public.id
}

resource "aws_route_table_association" "public-1b" {
  subnet_id      = aws_subnet.public-1b.id
  route_table_id = aws_route_table.public.id
}

resource "aws_route_table_association" "public-1c" {
  subnet_id      = aws_subnet.public-1c.id
  route_table_id = aws_route_table.public.id
}

#subnets private
resource "aws_subnet" "private-1a" {
  vpc_id            = aws_vpc.main.id
  cidr_block        = var.cidr_block_private-1a
  availability_zone = var.az-private-1a

  tags = {
    "Project" = var.tag_project,
    "Name"    = "sub-priv-${var.tag_project}",
  }
}

resource "aws_subnet" "private-1b" {
  vpc_id            = aws_vpc.main.id
  cidr_block        = var.cidr_block_private-1b
  availability_zone = var.az-private-1b

  tags = {
    "Project" = var.tag_project,
    "Name"    = "sub-priv-${var.tag_project}",
  }
}

resource "aws_subnet" "private-1c" {
  vpc_id            = aws_vpc.main.id
  cidr_block        = var.cidr_block_private-1c
  availability_zone = var.az-private-1c

  tags = {
    "Project" = var.tag_project,
    "Name"    = "sub-priv-${var.tag_project}",
  }
}

resource "aws_subnet" "public-1a" {
  vpc_id                  = aws_vpc.main.id
  cidr_block              = var.cidr_block_public-1a
  availability_zone       = var.az-public-1a
  map_public_ip_on_launch = true

  tags = {
    "Project" = var.tag_project,
    "Name"    = "sub-pub-${var.tag_project}",
  }
}

resource "aws_subnet" "public-1b" {
  vpc_id                  = aws_vpc.main.id
  cidr_block              = var.cidr_block_public-1b
  availability_zone       = var.az-public-1b
  map_public_ip_on_launch = true

  tags = {
    "Project" = var.tag_project,
    "Name"    = "sub-pub-${var.tag_project}",
  }
}

resource "aws_subnet" "public-1c" {
  vpc_id                  = aws_vpc.main.id
  cidr_block              = var.cidr_block_public-1c
  availability_zone       = var.az-public-1c
  map_public_ip_on_launch = true

  tags = {
    "Project" = var.tag_project,
    "Name"    = "sub-pub-${var.tag_project}",
  }
}

resource "aws_security_group" "lb" {
  name   = "app-alb-security-group"
  vpc_id = aws_vpc.main.id

  ingress {
    protocol    = "tcp"
    from_port   = 80
    to_port     = 80
    cidr_blocks = ["0.0.0.0/0"]
  }

  egress {
    from_port   = 0
    to_port     = 0
    protocol    = "-1"
    cidr_blocks = ["0.0.0.0/0"]
  }

  tags = {
    "Project" = var.tag_project,
    "Name"    = "app-alb-security-group",
  }
}

resource "aws_lb" "app_lb" {
  name            = "app-lb-${var.tag_project}"
  subnets         = [aws_subnet.private-1a.id, aws_subnet.private-1b.id, aws_subnet.private-1c.id]
  security_groups = [aws_security_group.lb.id]

  tags = {
    "Project" = var.tag_project,
    "Name"    = "app-lb-${var.tag_project}",
  }
}

resource "aws_lb_target_group" "app_lb" {
  name        = "app-target-group-${var.tag_project}"
  port        = 80
  protocol    = "HTTP"
  vpc_id      = aws_vpc.main.id
  target_type = "ip"

  tags = {
    "Project" = var.tag_project,
    "Name"    = "app-target-group-${var.tag_project}",
  }
}

resource "aws_lb_listener" "app_lb" {
  load_balancer_arn = aws_lb.app_lb.id
  port              = "80"
  protocol          = "HTTP"

  default_action {
    target_group_arn = aws_lb_target_group.app_lb.id
    type             = "forward"
  }

  tags = {
    "Project" = var.tag_project,
    "Name"    = "lb-listener-${var.tag_project}",
  }
}

# Broker
resource "aws_mq_broker" "broker" {
  broker_name = "broker-${var.tag_project}"

  engine_type         = "RabbitMQ"
  engine_version      = "3.9.16"
  host_instance_type  = "mq.t3.micro"
  security_groups     = [aws_security_group.mq.id]
  subnet_ids          = [aws_subnet.private-1a.id]
  publicly_accessible = false

  user {
    username = var.mq-user
    password = var.mq-password
  }

  tags = {
    "Project" = var.tag_project,
    "Name"    = "broker-${var.tag_project}",
  }
}

resource "aws_security_group" "mq" {
  name        = "mq-allow-access-priv-${var.tag_project}"
  description = "Allow MQ Access from Private Subnets"
  vpc_id      = aws_vpc.main.id

  ingress {
    description = "MQ Access from Private Subnets"
    from_port   = 5671
    to_port     = 5671
    protocol    = "tcp"
    cidr_blocks = [aws_subnet.private-1a.cidr_block, aws_subnet.private-1b.cidr_block, aws_subnet.private-1c.cidr_block]
  }

  egress {
    from_port        = 0
    to_port          = 0
    protocol         = "-1"
    cidr_blocks      = ["0.0.0.0/0"]
    ipv6_cidr_blocks = ["::/0"]
  }

  tags = {
    "Project" = var.tag_project,
    "Name"    = "mq-allow-access-priv-${var.tag_project}"
  }
}

# RDS
resource "aws_db_subnet_group" "rds" {
  name       = "rds-${var.tag_project}"
  subnet_ids = [aws_subnet.private-1a.id, aws_subnet.private-1b.id, aws_subnet.private-1c.id]

  tags = {
    "Project" = var.tag_project,
    "Name"    = "rds-${var.tag_project}",
  }
}

resource "aws_db_instance" "rds" {
  identifier              = "rds-${var.tag_project}"
  allocated_storage       = 20
  engine                  = "postgres"
  engine_version          = "14.4"
  instance_class          = "db.t3.small"
  name                    = "postgres"
  username                = var.rds-user
  password                = var.rds-password
  db_subnet_group_name    = aws_db_subnet_group.rds.name
  vpc_security_group_ids  = [aws_security_group.rds.id]
  skip_final_snapshot     = true
  publicly_accessible     = false
  storage_type            = "gp2"
  backup_retention_period = 0
  deletion_protection     = false

  tags = {
    "Project" = var.tag_project,
    "Name"    = "rds-${var.tag_project}"
  }
}

resource "aws_security_group" "rds" {
  name        = "rds-allow-access-priv-${var.tag_project}"
  description = "Allow RDS Access from Private Subnets"
  vpc_id      = aws_vpc.main.id

  ingress {
    description = "RDS Access from Private Subnets"
    from_port   = 5432
    to_port     = 5432
    protocol    = "tcp"
    cidr_blocks = [aws_subnet.private-1a.cidr_block, aws_subnet.private-1b.cidr_block, aws_subnet.private-1c.cidr_block]
  }

  egress {
    from_port        = 0
    to_port          = 0
    protocol         = "-1"
    ipv6_cidr_blocks = ["::/0"]
  }

  tags = {
    "Project" = var.tag_project,
    "Name"    = "rds-allow-access-priv-${var.tag_project}"
  }
}

# ECS
resource "aws_ecs_task_definition" "app" {
  family                   = "ecs-${var.tag_project}"
  network_mode             = "awsvpc"
  requires_compatibilities = ["FARGATE"]
  cpu                      = 1024
  memory                   = 2048

  container_definitions = <<DEFINITION
[
  {
    "image": "pedrobgodoy/api-01",
    "cpu": 1024,
    "memory": 2048,
    "name": "ecs-${var.tag_project}",
    "networkMode": "awsvpc",
    "environment": [
      {"name": "DATABASE_URL", "value": "postgresql://${var.rds-user}:${var.rds-password}@${aws_db_instance.rds.address}:${aws_db_instance.rds.port}/${aws_db_instance.rds.name}"},
      {"name": "RABBITMQ_URL", "value": "amqps://${var.mq-user}:${var.mq-password}@${aws_mq_broker.broker.instances.0.endpoints.0}"},
      {"name": "RABBITMQ_QUEUE", "value": "queue-01"}
    ],
    "portMappings": [
      {
        "containerPort": 3000,
        "hostPort": 3000
      }
    ]
  }
]
DEFINITION

  tags = {
    "Project" = var.tag_project,
    "Name"    = "ecs-task-definition-${var.tag_project}"
  }
}

resource "aws_security_group" "app_lb" {
  name   = "ecs-sg-${var.tag_project}"
  vpc_id = aws_vpc.main.id

  ingress {
    protocol        = "tcp"
    from_port       = 3000
    to_port         = 3000
    security_groups = [aws_security_group.lb.id]
  }

  egress {
    protocol    = "-1"
    from_port   = 0
    to_port     = 0
    cidr_blocks = ["0.0.0.0/0"]
  }

  tags = {
    "Project" = var.tag_project,
    "Name"    = "ecs-sg-${var.tag_project}"
  }
}

resource "aws_ecs_cluster" "main" {
  name = "ecs-cluster-${var.tag_project}"

  tags = {
    "Project" = var.tag_project,
    "Name"    = "ecs-cluster-${var.tag_project}"
  }
}

resource "aws_ecs_service" "app" {
  name            = "ecs-service-${var.tag_project}"
  cluster         = aws_ecs_cluster.main.id
  task_definition = aws_ecs_task_definition.app.arn
  desired_count   = var.app-count
  launch_type     = "FARGATE"

  network_configuration {
    security_groups = [aws_security_group.app_lb.id]
    subnets         = [aws_subnet.private-1a.id, aws_subnet.private-1b.id, aws_subnet.private-1c.id]
  }

  load_balancer {
    target_group_arn = aws_lb_target_group.app_lb.id
    container_name   = "ecs-${var.tag_project}"
    container_port   = 3000
  }

  depends_on = [aws_lb_listener.app_lb]

  tags = {
    "Project" = var.tag_project,
    "Name"    = "ecs-service-${var.tag_project}"
  }
}
