sudo docker build -t julius_ai .
docker run -d --restart=always --env-file .env julius_ai