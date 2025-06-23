# valsoray.dev

Website specifically made to prank your friends and others.

## Installation

1. Clone this repository:

   ```bash
   git clone https://github.com/valsoray-dev/valsoray.dev.git
   cd valsoray.dev
   ```

2. Build and run the project by using `cargo`:

   ```bash
   cargo run
   ```

3. Or by using Docker:
   - Build the Docker image:

     ```bash
     docker build -t valsoray.dev .
     ```

   - Run the container:

     ```bash
     docker run -d --rm --name valsoray.dev -p 8080:8080 valsoray.dev
     ```

## License

[MIT License](LICENSE).
