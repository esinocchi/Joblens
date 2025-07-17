# Troubleshooting Guide

## Day 1 Issues

### ngrok Tunnel Offline
**Symptoms**: "endpoint offline" errors
**Solution**: Restart ngrok, update Pub/Sub subscription

### Pub/Sub Retry Loops
**Symptoms**: Repeated POST requests
**Solution**: Normal behavior - webhook needs to return 200 OK