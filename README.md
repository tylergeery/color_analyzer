# Rust Color Analyzer
Simple API for image color analysis.

This is a first attempt at using Rust, and largely a learning experience. Please feel free to contribute and teach me a few things.

## Production
Feel free to test out the project [here](https://rust-color-analyzer.geerydev.com/upload)

## Features
- Submit a hosted URL or file upload at [upload page](https://rust-color-analyzer.geerydev.com/upload)
- Allows for image url submission at [predict endpoint](https://rust-color-analyzer.geerydev.com/predict)
- Allows for form data submission at [submit endpoint](https://rust-color-analyzer.geerydev.com/submit)
- Parses base hex colors from server/src/colors.json
- Output ordered list of colors based on individual pixel analysis
- Runs on GAE Flexible custom runtime
- UI Option to predict based on middle of image (middle 50% of pixels)

## Up and Running
```bash
make dev
```

## Running test suite
```bash
make test
```

## Deprecated: Deploying on GAE
```bash
docker tag color_analyzer:latest us.gcr.io/rust-color-analyzer/color_analyzer
gcloud auth configure-docker
docker push us.gcr.io/rust-color-analyzer/color_analyzer
gcloud app deploy --image-url=us.gcr.io/rust-color-analyzer/color_analyzer app.yaml
```

## TODO
- Add K-NN approach for determining dominant color
- Run all prediction operations asynchronously
- Explore Luma color pixels as an options
