#!/bin/bash

set -e

echo "📊 Starting PezkuwiChain Monitoring Stack..."

# Check if docker-compose is available
if ! command -v docker-compose &> /dev/null; then
    if ! command -v docker &> /dev/null; then
        echo "❌ Docker not found. Please install Docker first."
        exit 1
    fi
    echo "Using 'docker compose' instead of 'docker-compose'"
    COMPOSE_CMD="docker compose"
else
    COMPOSE_CMD="docker-compose"
fi

# Navigate to monitoring directory
cd monitoring

# Create directories if they don't exist
mkdir -p grafana/provisioning/dashboards
mkdir -p grafana/provisioning/datasources  
mkdir -p grafana/dashboards

# Pull latest images
echo "📥 Pulling latest images..."
$COMPOSE_CMD pull

# Start monitoring stack
echo "🚀 Starting monitoring services..."
$COMPOSE_CMD up -d

# Wait for services to be ready
echo "⏳ Waiting for services to start..."
sleep 30

# Check service health
echo "🔍 Checking service health..."
$COMPOSE_CMD ps

echo "✅ Monitoring stack started successfully!"
echo ""
echo "📊 Access points:"
echo "  - Grafana: http://localhost:3000 (admin/pezkuwi2024)"
echo "  - Prometheus: http://localhost:9090"
echo "  - AlertManager: http://localhost:9093"
echo "  - Telemetry: http://localhost:8001"
echo "  - Jaeger: http://localhost:16686"
echo ""
echo "💡 To stop monitoring: cd monitoring && $COMPOSE_CMD down"