# Rust Color Analyzer
Simple REST API for image color analysis.

This is a first attempt at using Rust, and largely a learning experience. Please feel free to contribute and teach me a few things.

## Production
Feel free to test out the project [here](https://rust-color-analyzer.appspot.com/upload)

## Features
- Submit a hosted URL or file upload at [upload page](https://rust-color-analyzer.appspot.com/upload)
- Allows for image url submission at [predict endpoint](https://rust-color-analyzer.appspot.com/predict)
- Allows for form data submission at [submit endpoint](https://rust-color-analyzer.appspot.com/submit)
- Parses base hex colors from server/src/colors.json
- Output ordered list of colors based on individual pixel analysis
- Runs on GAE Flexible custom runtime

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
