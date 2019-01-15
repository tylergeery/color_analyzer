# color_analyzer
Simple REST API for image color analysis.

This is a first attempt at using Rust, and largely a learning experience. Please feel free to contribute.

## Production
Feel free to test out the project [here](#TODO)

## Features
- Submit a hosted URL or file upload at [/upload](/upload)
- Allows for image url submission at [predict endpoint](/predict)
- Parses base hex colors from server/src/colors.json
- Output ordered list of colors based on individual pixel analysis

## TODO
- Add fetching results animation and prediction results styling
- Allow for image file upload at [/submit](/submit)
- Parse pixels from file type image uploads
- Use K-NN approach for determining dominant color
- Add UI Option for prediction approach (i.e Euclidean Distance, K-NN)

## Up and Running
```bash
docker build -t color_image .
docker run -id -p 8080:8080 -v $(pwd)/server:/usr/src/app --name color color_image cargo watch -x run
```
