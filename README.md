# health-check

<div align="center">
  <img width="200px" height="200px" src="https://github.com/user-attachments/assets/4a5204e5-1da1-4afe-86e6-dfbdc71529e6" alt="Heart icon">
  <p><strong>Check your server availability</strong></p>
</div>

## Usage
Deploy the docker image with the following command
```sh
docker run -p 8080:80 archhfan/health-check:latest
```
Or with a `compose.yml` file.
```yml
services:
  health-check:
    image: archhfan/health-check:latest
    container_name: health-check
    restart: unless-stopped
    ports:
      - 8080:80
```

And just ping your server with your favorite robot! For example with https://uptimerobot.com/

Test it here: https://health.archfan.com
