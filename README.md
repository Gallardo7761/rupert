# Rupert

Bot de Discord escrito en Rust (y en el futuro, de Telegram). Es la reimplementación de mi bot antiguo en Python, **Pype**.

## Comandos

| Categoría | Comandos |
| --- | --- |
| Varios | `/ping`, `/dice`, `/coin`, `/banana`, `/avatar`, `/ship` |
| Mates | `/prime check`, `/prime next`, `/calc` |
| Interacciones | `/interact` (58 acciones con GIF de [nekos.best](https://nekos.best)) |
| Anuncios | `/say`, `/poll` |
| Moderación | `/admin clear`, `kick`, `ban`, `unban`, `timeout`, `untimeout`, `nick`, `role`, `lock`, `unlock` |

## Estructura

Es un workspace de Cargo con tres crates:

- `crates/rupert-core`: lógica sin dependencias de ninguna plataforma (tiradas, primos, calculadora, jerarquía de moderación, interacciones…). Aquí van los tests.
- `crates/rupert-discord`: el binario del bot de Discord, sobre [serenity](https://github.com/serenity-rs/serenity) 0.12.

## Requisitos

- **Rust** estable con soporte para la edición 2024. Yo uso la 1.98, que es la misma que usa la imagen de Docker.
- Una App Discord con Bot y su respectivo **token**.
- **Docker**.

## Configuración

Copia el ejemplo y rellénalo:

```bash
cp .env.example .env
```

| Variable | Descripción |
| --- | --- |
| `DISCORD_TOKEN` | El token del bot. |
| `GUILD_IDS` | Los IDs de los servidores donde se registran los comandos, separados por comas (por ejemplo, `123,456`). |

Los comandos se registran por servidor, y no de forma global, así que salen al momento en todos los servidores de `GUILD_IDS`. El bot lee primero las variables de entorno y, si no están, las coge del `.env` del directorio desde el que se ejecuta.

## Compilar y ejecutar en local

```bash
cargo run -p rupert-discord
```

Para compilar en modo release:

```bash
cargo build --release -p rupert-discord
```

El binario queda en `target/release/rupert-discord`. Ejecútalo desde el directorio donde tengas el `.env`.

## Despliegue con Docker

El `Dockerfile` compila en dos etapas. Primero pasa los tests y compila en release; después copia solo el binario a una imagen `debian:trixie-slim`, donde se ejecuta con un usuario sin privilegios. El `.env` **no** va dentro de la imagen: `compose.yaml` lo monta en el contenedor en solo lectura.

1. Compose espera una red externa llamada `core_net`. Si no la tienes, créala:

   ```bash
   docker network create core_net
   ```

2. Construye la imagen y arranca el contenedor:

   ```bash
   docker compose up -d --build
   ```

3. Mira los logs:

   ```bash
   docker compose logs -f discord
   ```

Variables opcionales de Compose:

| Variable | Por defecto | Descripción |
| --- | --- | --- |
| `RUPERT_ENV_FILE` | `./.env` | La ruta en el host del `.env` que se monta. |
| `RUPERT_VERSION` | `latest` | La etiqueta de la imagen `rupert-discord`. |

## Licencia

[GNU GPL v3](LICENSE).
