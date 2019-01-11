# color_analyzer
Simple REST API for image color analysis

## Features
- Submit a hosted URL or file upload at [/upload](/upload)
- Allows for image url submission at [predict endpoint](/predict)
- Parses base hex colors from server/src/colors.json
- Output ordered list of colors based on individual pixel analysis

## TODO
- Allow for image file upload at [/submit](/submit)
- Parse pixels from file type image uploads
- Use K-NN approach for determining dominant color
- Add UI Option for prediction approach (i.e Euclidean Distance, K-NN)

## Up and Running
```bash
docker build -t color_image .
docker run -id -p 8080:8080 -v $(pwd)/server:/usr/src/app --name color color_image cargo watch -x run
```
