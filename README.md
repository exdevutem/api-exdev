# API ExDev

Esta es una API multipropósitos del club ExDev. La idea es que sea un sistema monolítico que se preocupe de todos los procesos que ocurren en el club, y que todos los sistemas que se creen a futuro beban de esta, y sean solo interfaces bonitas para lo que ocurre acá.

Preventivamente se decidició utilizar Rust con el framework Actix para este propósito, pero aún no es una idea fija.

## Instrucciones para montar

Se necesita tener cargo, rust y todo el toolchain relacionado. Puedes instalar este toolchain [siguiendo estas instrucciones](https://www.rust-lang.org/tools/install).

Con el toolchain instalado, simplemente corre el comando siguiente, y empieza a modificar el código a tu gusto:

```bash
cargo run
```

En estos momentos se necesita de SQLite3 para hacer de Base de Datos. La URL de conexion se maneja en un archivo de ambiente `.env` que debe ser creado por ti. Se provee un ejemplo en el repositorio que puedes copiar con el comando:

```bash
cp .env.example .env
```

Se recomienda que instales el CLI de `sqlx` para manejar las migraciones. A futuro se planea que estas migraciones se corran junto con el ejecutable! Pero por mientras, puedes hacerlo con el siguiente comando:

```bash
sqlx migrate run
```
