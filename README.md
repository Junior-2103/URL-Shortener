# URL-Shortener

Um encurtador de URL desenvolvido em Rust para a máxima performance

![Rust](https://img.shields.io/badge/language-Rust-orange.svg)
![Docker](https://img.shields.io/badge/container-Docker-blue.svg)
![License](https://img.shields.io/badge/license-GPL--3.0-blue.svg)

## Tecnologias

- **Rust**: Máxima eficiencia e velocidade
- **Docker e docker compose**: Subir para servidores
- **Nix (Flake)**: Reprodutibilidade para funcionar em qualquer sistema

## Instalação

### Docker

```bash
git clone https://github.com/Junior-2103/URL-Shortener

cd URL-Shortener/

docker compose up --build
```

### Cargo

```bash
git clone https://github.com/Junior-2103/URL-Shortener

cd URL-Shortener/

cargo run
```

### Nix

```bash
git clone https://github.com/Junior-2103/URL-Shortener

cd URL-Shortener

nix develop

cargo run
```

## Como usar

Logo após executar os comando de execução do projeto
acesse o link `http://localhost:8080` para usar a aplicação
