# color_analyzer
Simple REST API for image color analysis.

This is a first attempt at using Rust, and largely a learning experience. Please feel free to contribute.

## Production
Feel free to test out the project [here](#TODO)

## Features
- Submit a hosted URL or file upload at [submit webpage](/submit)
- Allows for image url submission at [predict endpoint](/predict)
- Allows for form data submission at [upload endpoint](/upload)
- Parses base hex colors from server/src/colors.json
- Output ordered list of colors based on individual pixel analysis

## Up and Running
```bash
docker build -t color_analyzer .
docker build -t color_analyzer_dev -f Dockerfile.dev .
docker run -id -p 8080:8080 -v $(pwd)/server:/usr/src/app --name color color_analyzer_dev
```

## Running test suite
```bash
docker exec -it color cargo test
```

## TODO
- Use K-NN approach for determining dominant color
- Add UI Option for prediction approach (i.e Euclidean Distance, K-NN)
- Find a production home (GAE Flexible?)
