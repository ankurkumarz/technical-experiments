# Prometheus
- is an open-source systems monitoring and alerting toolkit originally built at SoundCloud (2021).
- Prometheus collects and stores its metrics as time series data
- Prometheus was the second project accepted into the Cloud Native Computing Foundation after Kubernetes, and also the second to graduate.
- Prometheus and Grafana can be used as complementary services that, together, provide a robust time-series database with excellent data visualization.

## Main Features (Official Docs)

[Click here](https://prometheus.io/docs/introduction/overview/) for official docs.

- Offfers a multi-dimensional data model with time series data
- Supports a Query Language - PromQL
- No reliance on distributed storage; single server nodes are autonomous
- Time series collection happens via a pull model over HTTP
- Pushing time series is supported via an intermediary gateway
- Targets are discovered via service discovery or static configuration
- Multiple modes of graphing and dashboarding support- 


# Architecture

![Architecture](https://prometheus.io/assets/architecture.png)

# Alternative Options
- InfluxDB, Graphite for Time series data
- ELK (Elastic, Logstash, Kibana)
- Cloud Providers - AWS CloudWatch, Azure Monitor
- Observability/APM Vendors - New Relic, Dynatrace, etc.

# Local Installation
- Docker with default configuration
```
docker run -p 9090:9090 prom/prometheus
```

# Managed Services
- [AWS Managed Service for Prometheus](https://aws.amazon.com/prometheus/)
- [Grafana Labs Prometheus](https://grafana.com/oss/prometheus/)
