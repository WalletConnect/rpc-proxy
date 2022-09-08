# Log Group for our App
resource "aws_cloudwatch_log_group" "cluster_logs" {
  name              = "${var.app_name}_logs"
  retention_in_days = 14
}

# ECS Cluster
resource "aws_ecs_cluster" "app_cluster" {
  name = "${var.app_name}_cluster"

  configuration {
    execute_command_configuration {
      logging = "OVERRIDE"

      log_configuration {
        cloud_watch_encryption_enabled = false
        cloud_watch_log_group_name     = aws_cloudwatch_log_group.cluster_logs.name
      }
    }
  }

  # Exposes metrics such as the
  # number of running tasks
  # in CloudWatch
  setting {
    name  = "containerInsights"
    value = "enabled"
  }
}

# ECS Task definition
resource "aws_ecs_task_definition" "app_task" {
  family                   = var.app_name
  container_definitions    = <<DEFINITION
  [
    {
      "name": "${var.app_name}",
      "environment" : [
          { "name" : "INFURA_PROJECT_ID", "value" : "${var.infura_project_id}" }
      ],
      "image": "${var.ecr_repository_url}",
      "essential": true,
      "portMappings": [
        {
          "containerPort": ${var.port},
          "hostPort": ${var.port}
        }
      ],
      "memory": 512,
      "cpu": 256,
      "logConfiguration": {
        "logDriver": "awslogs",
        "options": {
          "awslogs-group": "${aws_cloudwatch_log_group.cluster_logs.name}",
          "awslogs-region": "${var.region}",
          "awslogs-stream-prefix": "ecs"
        }
      },
      "dependsOn": [{
        "containerName": "aws-otel-collector",
        "condition": "START"
      }]
    },
    {
      "name": "aws-otel-collector",
      "image": "public.ecr.aws/aws-observability/aws-otel-collector:latest",
      "environment" : [
          { "name" : "AWS_PROMETHEUS_SCRAPING_ENDPOINT", "value" : "0.0.0.0:${var.port}" },
          { "name" : "AWS_PROMETHEUS_ENDPOINT", "value" : "${var.prometheus_endpoint}api/v1/remote_write" }
      ],
      "essential": true,
      "command": [
        "--config=/etc/ecs/ecs-amp-prometheus.yaml"
      ],
      "logConfiguration": {
        "logDriver": "awslogs",
        "options": {
          "awslogs-create-group": "True",
          "awslogs-group": "/ecs/${var.app_name}-ecs-aws-otel-sidecar-collector",
          "awslogs-region": "${var.region}",
          "awslogs-stream-prefix": "ecs"
        }
      }
    }
  ]
  DEFINITION
  requires_compatibilities = ["FARGATE"] # Stating that we are using ECS Fargate
  network_mode             = "awsvpc"    # Using awsvpc as our network mode as this is required for Fargate
  memory                   = 512         # Specifying the memory our container requires
  cpu                      = 256         # Specifying the CPU our container requires
  execution_role_arn       = aws_iam_role.ecs_task_execution_role.arn

  runtime_platform {
    operating_system_family = "LINUX"
  }
}

resource "aws_iam_role" "ecs_task_execution_role" {
  name               = "${var.app_name}_ecs_task_execution_role"
  assume_role_policy = data.aws_iam_policy_document.assume_role_policy.json
}

data "aws_iam_policy_document" "assume_role_policy" {
  statement {
    actions = ["sts:AssumeRole"]

    principals {
      type        = "Service"
      identifiers = ["ecs-tasks.amazonaws.com"]
    }
  }
}

resource "aws_iam_role_policy_attachment" "ecs_task_execution_role_policy" {
  role       = aws_iam_role.ecs_task_execution_role.name
  policy_arn = "arn:aws:iam::aws:policy/service-role/AmazonECSTaskExecutionRolePolicy"
}

resource "aws_iam_role_policy_attachment" "cloudwatch_write_policy" {
  role       = aws_iam_role.ecs_task_execution_role.name
  policy_arn = "arn:aws:iam::aws:policy/CloudWatchLogsFullAccess"
}

resource "aws_iam_role_policy_attachment" "prometheus_write_policy" {
  role       = aws_iam_role.ecs_task_execution_role.name
  policy_arn = "arn:aws:iam::aws:policy/AmazonPrometheusRemoteWriteAccess"
}

# ECS Service
resource "aws_ecs_service" "app_service" {
  name            = "${var.app_name}-service"
  cluster         = aws_ecs_cluster.app_cluster.id
  task_definition = aws_ecs_task_definition.app_task.arn
  launch_type     = "FARGATE"
  desired_count   = 3 # Setting the number of containers we want deployed to 3

  network_configuration {
    subnets          = data.aws_subnets.private_subnets.ids
    assign_public_ip = false                                                                               # We do public ingress through the LB
    security_groups  = ["${aws_security_group.tls_ingess.id}", "${aws_security_group.vpc_app_ingress.id}"] # Setting the security group
  }

  load_balancer {
    target_group_arn = aws_lb_target_group.target_group.arn
    container_name   = aws_ecs_task_definition.app_task.family
    container_port   = var.port
  }

  # Allow external changes without Terraform plan difference
  lifecycle {
    ignore_changes = [desired_count]
  }
}

data "aws_vpc" "vpc" {
  filter {
    name   = "tag:Name"
    values = [var.vpc_name]
  }
}

# Providing a reference to our default subnets
data "aws_subnets" "private_subnets" {
  filter {
    name   = "vpc-id"
    values = [data.aws_vpc.vpc.id]
  }

  filter {
    name   = "tag:Class"
    values = ["private"]
  }
}

data "aws_subnets" "public_subnets" {
  filter {
    name   = "vpc-id"
    values = [data.aws_vpc.vpc.id]
  }

  filter {
    name   = "tag:Class"
    values = ["public"]
  }
}

# Load Balancer
resource "aws_alb" "network_load_balancer" {
  name               = replace("${var.app_name}-lb-${substr(uuid(), 0, 3)}", "_", "-")
  load_balancer_type = "network"
  subnets            = data.aws_subnets.public_subnets.ids

  lifecycle {
    create_before_destroy = true
    ignore_changes        = [name]
  }
}

resource "aws_lb_target_group" "target_group" {
  name        = replace("${var.app_name}-${substr(uuid(), 0, 3)}", "_", "-")
  port        = var.port
  protocol    = "TCP"
  target_type = "ip"
  vpc_id      = data.aws_vpc.vpc.id

  # Deregister quickly to allow for faster deployments
  deregistration_delay = 30 # Seconds

  health_check {
    protocol            = "HTTP"
    path                = "/health"
    interval            = 10
    healthy_threshold   = 2
    unhealthy_threshold = 2
  }

  lifecycle {
    create_before_destroy = true
    ignore_changes        = [name]
  }
}

resource "aws_lb_listener" "listener" {
  load_balancer_arn = aws_alb.network_load_balancer.arn
  port              = "443"
  protocol          = "TLS"
  certificate_arn   = var.acm_certificate_arn
  ssl_policy        = "ELBSecurityPolicy-TLS13-1-2-2021-06"
  default_action {
    type             = "forward"
    target_group_arn = aws_lb_target_group.target_group.arn
  }

  lifecycle {
    create_before_destroy = true
  }
}

# Security Groups
resource "aws_security_group" "tls_ingess" {
  name        = "${var.app_name}-tls-ingress"
  description = "Allow tls ingress from everywhere"
  vpc_id      = data.aws_vpc.vpc.id
  ingress {
    from_port   = 443
    to_port     = 443
    protocol    = "tcp"
    cidr_blocks = ["0.0.0.0/0"] # Allowing traffic in from all sources
  }

  egress {
    from_port   = 0             # Allowing any incoming port
    to_port     = 0             # Allowing any outgoing port
    protocol    = "-1"          # Allowing any outgoing protocol
    cidr_blocks = ["0.0.0.0/0"] # Allowing traffic out to all IP addresses
  }
}

resource "aws_security_group" "vpc_app_ingress" {
  name        = "${var.app_name}-vpc-ingress-to-app"
  description = "Allow app port ingress from vpc"
  vpc_id      = data.aws_vpc.vpc.id

  ingress {
    from_port   = var.port
    to_port     = var.port
    protocol    = "tcp"
    cidr_blocks = [data.aws_vpc.vpc.cidr_block]
  }

  egress {
    from_port   = 0             # Allowing any incoming port
    to_port     = 0             # Allowing any outgoing port
    protocol    = "-1"          # Allowing any outgoing protocol
    cidr_blocks = ["0.0.0.0/0"] # Allowing traffic out to all IP addresses
  }
}

# DNS Records
resource "aws_route53_record" "dns_load_balancer" {
  zone_id = var.route53_zone_id
  name    = var.fqdn
  type    = "A"

  alias {
    name                   = aws_alb.network_load_balancer.dns_name
    zone_id                = aws_alb.network_load_balancer.zone_id
    evaluate_target_health = true
  }
}
