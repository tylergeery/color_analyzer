# color_analyzer
Simple REST API for image color analysis

## Features
- Allows for image url submission at [#TODO](/submit)

## TODO
- Allows for image upload at [#TODO](/upload)
- Parse colors.json file for allowed image colors
- Output ordered list of colors based on individual pixel analysis
- Use KNN approach for determing dominant color

## Up and Running
```bash
docker build -t color_image .
docker run -id -p 8080:8080 -v $(pwd)/server:/usr/src/app --name color color_image cargo watch -x run
```
