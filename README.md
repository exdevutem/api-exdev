# API ExDev

Esta es una API multipropósitos del club ExDev. La idea es que sea un sistema monolítico que se preocupe de todos los procesos que ocurren en el club, y que todos los sistemas que se creen a futuro beban de esta, y sean solo interfaces bonitas para lo que ocurre acá.

Preventivamente se decidició utilizar Rust con el framework Actix para este propósito, pero aún no es una idea fija.

## Documentación

Existe una documentación hecha con rustdocs que puede ser compilada con cargo. Para poder leerla, ejecuta el siguiente comando:

```bash
cargo doc --open
```

## Instrucciones para montar

Se necesita tener cargo, rust y todo el toolchain relacionado. Puedes instalar este toolchain [siguiendo estas instrucciones](https://www.rust-lang.org/tools/install).

La siguiente dependencia importante es SQLite3. Necesitas tener un archivo de ambiente que especifique la URL de conexion de la base de datos; o sea, el archivo donde se guardarán los datos.

La solución más sencilla es la creación de un archivo llamado `data.db`:

```bash
touch data.db # Archivo de base de datos SQLite3.
cp .env.example .env # Archivo de ambiente por defecto.
```

El último paso es correr las migraciones necesarias. Primero, instala la herramienta de terminal de SQLx con el siguiente comando:

```bash
cargo install sqlx-cli
```

Hecho esto, puedes correr las migraciones con el comando:

```bash
sqlx migrate run
```

Con el toolchain instalado, SQLite3 listo, y las migraciones hechas, simplemente corre el comando siguiente, y empieza a modificar el código a tu gusto:

```bash
cargo run
```

Esto es opcional! pero si te es cómodo que recompile el código conforme estás modificándolo, puedes hacerlo con cargo watch. Instálalo con el siguiente comando:

```bash
cargo install cargo-watch
```

Luego, puedes utilizarlo con un comando como el siguiente:

```bash
cargo watch -c -x run
```
