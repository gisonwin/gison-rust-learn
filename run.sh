docker build -t my-rust-app .
docker run -d --name my-rust-app-instance -p 8858:3000 my-rust-app