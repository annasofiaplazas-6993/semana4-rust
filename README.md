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
