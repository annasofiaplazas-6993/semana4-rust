## Semana 4 — Operandos con Rust y pruebas unitarias

### Paso 0: cargo new tipos_operandos_rust

Se creó el branch `Semana4` a partir de `main`.

Se creó el proyecto `tipos_operandos_rust` utilizando Cargo.

Se comprobó que el proyecto compilara y ejecutara correctamente mediante `cargo run`.

Resultado:

```text
Hello, world!
```

También se agregó `**/target` al archivo `.gitignore` para evitar subir las carpetas generadas por Cargo.

## Paso 1: prueba unitaria de calcular_oleadas

Se creó la función `calcular_oleadas(derrotados: i32, por_oleada: i32) -> i32`.

La función calcula el número de oleadas mediante división entera.

Se agregó una prueba unitaria para comprobar que:

`calcular_oleadas(7, 2) = 3`

Resultado de `cargo test`:

```text
running 1 test
test tests::prueba_calcular_oleadas ... ok

test result: ok. 1 passed; 0 failed
```

También se ejecutó `cargo run` y se obtuvo:

```text
Oleadas: 3
```
## Paso 2: prueba unitaria de calcular_dano_critico

Se creó la función `calcular_dano_critico(dano_base: i32, multiplicador: f64) -> f64`.

La función convierte `dano_base` de `i32` a `f64` y lo multiplica por el multiplicador.

Se agregó una nueva prueba unitaria para comprobar que:

`calcular_dano_critico(100, 1.5) = 150.0`

Resultado de `cargo test`:

```text
running 2 tests
test tests::prueba_calcular_oleadas ... ok
test tests::prueba_calcular_dano_critico ... ok

test result: ok. 2 passed; 0 failed
```
## Paso 3: prueba unitaria de calcular_dano_promedio

Se creó la función `calcular_dano_promedio(dano_base: i32, por_oleada: i32) -> f64`.

La función convierte los valores a `f64` y calcula el daño promedio mediante división.

Se agregó una nueva prueba unitaria para comprobar que:

`calcular_dano_promedio(100, 4) = 25.0`

Resultado de `cargo test`:

```text
running 3 tests
test tests::prueba_calcular_dano_critico ... ok
test tests::prueba_calcular_dano_promedio ... ok
test tests::prueba_calcular_oleadas ... ok

test result: ok. 3 passed; 0 failed
```
## Paso 4: verificación final — 3 pruebas pasando

Se realizó la verificación final del proyecto mediante `cargo run` y `cargo test`.

Las tres pruebas unitarias pasaron correctamente:

```text
running 3 tests
test tests::prueba_calcular_dano_promedio ... ok
test tests::prueba_calcular_dano_critico ... ok
test tests::prueba_calcular_oleadas ... ok

test result: ok. 3 passed; 0 failed
```

El proyecto quedó funcionando correctamente con las tres pruebas unitarias aprobadas.
