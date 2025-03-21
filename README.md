# Todo_App
This project is to build a todo app.

### Features
Serde: It uses serde to format the data in human understandable

Setup
-----
You can clone this repository using the command below
```sh
git clone git@github.com:Jagoum/Todo_App.git
```

Build And Run
-------------
To run this project you need to install [rust](https://www.rust-lang.org/learn/get-started)
If you want to run this project locally run the following commands
```sh
cargo run
```
Building Using Docker
---------------------
You can also build this project using docker without cloning the repository by 
running the commands below
Before that you need to ensure you have [docker](https://www.docker.com/) installed and running

```sh
docker pull ghcr.io/Jagoum/Todo_App
docker run create NewTask # to create a new task
```
Building The Docker Image 
-------------------------
Optionally if you want to build the docker image yourself you can run the following commands 
```sh
docker built -t your-image-name .
docker run create NewTask # to create a new task

```
Optionally you could also run the project using [docker compose](https://docs.docker.com/reference/cli/docker/compose/). The command is seen below

```sh
docker compose up --build
docker compose run build_image create "New Job"
```


