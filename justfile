__default:
    just --list

docker-build:
    docker build -t northflank-test .

docker-run:
    docker run -p 3000:3000 northflank-test

run:
    just docker-build
    just docker-run
