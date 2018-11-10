# color_analyzer
Simple REST API for image color analysis

## Up and Running
```bash
docker build -t color_image .
docker run -id -p 8080:8080 -v ~/Projects/rust/color_analyzer:/usr/src/app --name color color_image cargo watch -x run
```
