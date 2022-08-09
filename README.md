# Corazón de sistema blockchain
Microsistema con base en **Proof of Work** (Prueba de Trabajo) escrito en rust con el fin de crear un sistema capaz de dar respaldo a sistemas más grandes.

## Cómo funciona
---
Al ejecutable por consola se le pasan dos parámetros:

- JSON tipo String con propiedad ```"nonce": null```
- Nivel de dificultad ```7``` por ejemplo

```rust
./app-rust-blockchain "{\"data\": \"Julian Lasso\", \"nonce\": null, \"date\": \"2022-01-01T00:00:00Z\"}" 7
```

## Resltado en consola
---
```shell
Nivel de dificultadad: 7
Nonce encontrado: 671660449
Datos resultantes: {"data": "Julian Lasso", "nonce": 671660449, "date": "2022-01-01T00:00:00Z"}
Hash: 0000000c0b8f03df923acbe02804feb5248021b0bcfe9e5d517cae1f68e5cf81
Cantidad de intentnos: 250.742.845
Tiempo transcurrido: 145s
```

## Información importante
---
No use niveles de dificultad más allá de ```6```, ya que ```7``` es un nivel de difucltad algo elevado que puede llegar a tomar 3 a 5 minutos o más en encontrar una solución dependiendo de su procesador

## Librerías
---
Las librerías prinicipales son:
- **Libsodium** para el manejo del número aleatorio criptográficamente aceptable. https://doc.libsodium.org/
- **Blake3** para el tema del hash. https://github.com/BLAKE3-team/BLAKE3