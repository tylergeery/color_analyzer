# color_analyzer
Simple REST API for image color analysis

## Features
- Allows for image url submission at [#TODO](/predict)
- Parses base hex colors from server/src/colors.json
- Output ordered list of colors based on individual pixel analysis

## TODO
- Allow for image file upload or URL input at [#TODO](/upload)
- Parse pixels from file type image uploads
- Use KNN approach for determing dominant color

## Up and Running
```bash
docker build -t color_image .
docker run -id -p 8080:8080 -v $(pwd)/server:/usr/src/app --name color color_image cargo watch -x run
```
