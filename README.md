# Corazón de sistema blockchain
Microsistema con base en **Proof of Work** (Prueba de Trabajo) escrito en rust con el fin de crear un microsistema capaz de dar respaldo a sistemas más grandes.

## Cómo funciona
---
Al ejecutable por consola se le pasan dos parámetros:

- JSON tipo String con propiedad ```"nonce": null```
- Nivel de dificultad ```7``` por ejemplo
- Carga de la CPU ```80``` porciento por ejemplo

Crear un bloque
```shell
./app-rust-blockchain create "{\"data\": \"Julian Lasso\", \"nonce\": null, \"date\": \"2022-01-01T00:00:00Z\"}" 7 80
```
Resultado
```shell
Cantidad de CPU usadas: 16 - 80%
Nivel de dificultadad: 7
Nonce encontrado: 748833857
Hash: 0000000a9c3d64fd5b81c2afa51a65c96179ac02381b66b9047653effa57ac02
Datos resultantes: {"data": "Julian Lasso", "nonce": 748833857, "date": "2022-01-01T00:00:00Z"}
Tiempo transcurrido: 21s
```

Verificar un bloque creado
```shell
./app-rust-blockchain check "{\"data\": \"Julian Lasso\", \"nonce\": null, \"date\": \"2022-01-01T00:00:00Z\"}" 748833857 0000000a9c3d64fd5b81c2afa51a65c96179ac02381b66b9047653effa57ac02
```
Resultado en caso de éxito
```shell
OK
```
Resultado en caso de error
```shell
ERROR
```

## Información importante
---
No use niveles de dificultad más allá de ```6```, ya que ```7``` es un nivel de difucltad algo elevado que puede llegar a tomar 3 a 5 minutos o más en encontrar una solución dependiendo de su procesador.

Tampoco use más allá del 80% de potencia de su procesador.

## Librerías
---
Las librerías prinicipales son:
- **Libsodium** para el manejo del número aleatorio criptográficamente aceptable. https://doc.libsodium.org/
- **Blake3** para el tema del hash. https://github.com/BLAKE3-team/BLAKE3