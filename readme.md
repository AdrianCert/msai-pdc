# MSAI PSD

In this repository, I store all homework for PSD topic.

```sh
D:\dev\msai-pdc\snap4flame-backend\.venv\Scripts\python.exe -m snap4frame_backend.server
D:\dev\msai-pdc\snap4flame-backend\.venv\Scripts\python.exe -m snap4frame_backend.client
```

```sh
podman build -f .\docker\Dockerfile -t snap4frame-backend:v1 .
podman tag localhost/snap4frame-backend:v1 gcr.io/analog-daylight-377117/snap4frame-backend:v1
podman push gcr.io/analog-daylight-377117/snap4frame-backend:v1
```