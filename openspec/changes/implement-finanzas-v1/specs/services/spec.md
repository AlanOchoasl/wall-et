## Purpose

Permite administrar los servicios recurrentes mensuales que el usuario comparte con otras personas, calculando automáticamente la parte que le corresponde y los totales de los servicios activos.

## ADDED Requirements

### Requirement: Registrar un servicio

El sistema SHALL permitir crear un servicio con nombre, costo total, cantidad de personas que comparten el gasto y notas opcionales. La moneda de todo servicio MUST ser ARS en la primera versión. La parte del usuario MUST calcularse al crear el servicio y MUST NOT ingresarse manualmente.

#### Scenario: Alta exitosa

- **WHEN** el usuario ingresa nombre "Spotify", costo total 10000,00, cantidad de personas 4 y guarda
- **THEN** el sistema crea el servicio con estado activo y muestra "Mi parte" igual a 2500,00

#### Scenario: Alta sin nombre

- **WHEN** el usuario intenta guardar un servicio con el nombre vacío
- **THEN** el sistema rechaza la operación, muestra un mensaje de validación en el campo nombre y no persiste el servicio

#### Scenario: Alta con cantidad de personas inválida

- **WHEN** el usuario intenta guardar un servicio con cantidad de personas 0, negativa o no entera
- **THEN** el sistema rechaza la operación e indica que la cantidad debe ser un entero mayor o igual a 1

#### Scenario: Alta sin notas

- **WHEN** el usuario guarda un servicio con nombre y datos de origen válidos y deja las notas vacías
- **THEN** el sistema crea el servicio y las notas quedan sin contenido

### Requirement: Editar datos de origen y notas

El sistema SHALL permitir modificar el nombre, el costo total, la cantidad de personas y las notas de un servicio existente. Al modificar el costo total o la cantidad de personas, el sistema MUST recalcular la parte del usuario. El sistema MUST NOT permitir editar la parte del usuario como valor independiente.

#### Scenario: Recalculo al cambiar el costo

- **WHEN** el usuario cambia el costo total de 10000,00 a 12000,00 en un servicio de 4 personas
- **THEN** el sistema guarda el nuevo costo y muestra "Mi parte" igual a 3000,00

#### Scenario: Recalculo al cambiar la cantidad de personas

- **WHEN** el usuario cambia la cantidad de personas de 4 a 3 en un servicio de costo 10000,00
- **THEN** el sistema guarda la nueva cantidad y muestra "Mi parte" calculada con el nuevo divisor

#### Scenario: Intento de editar un resultado

- **WHEN** el usuario intenta modificar directamente el valor mostrado de "Mi parte"
- **THEN** el sistema no acepta el cambio y el valor permanece derivado del costo total y la cantidad de personas

#### Scenario: Edición con datos inválidos

- **WHEN** el usuario intenta guardar una edición con nombre vacío o cantidad de personas menor que 1
- **THEN** el sistema rechaza la operación, conserva los datos previos y muestra el error de validación correspondiente

### Requirement: Eliminar un servicio

El sistema SHALL permitir eliminar un servicio de forma permanente. Antes de eliminar, el sistema MUST solicitar confirmación explícita al usuario. Un servicio eliminado MUST dejar de existir y MUST NOT aparecer en ningún listado.

#### Scenario: Eliminación confirmada

- **WHEN** el usuario solicita eliminar un servicio y confirma la acción
- **THEN** el sistema elimina el servicio, lo quita de la tabla y actualiza los totales

#### Scenario: Eliminación cancelada

- **WHEN** el usuario solicita eliminar un servicio y cancela la confirmación
- **THEN** el sistema no modifica el servicio ni los totales

### Requirement: Calcular la parte del usuario

El sistema SHALL calcular la parte del usuario como el costo total dividido por la cantidad de personas que comparten el gasto. La cantidad de personas MUST incluir al usuario. La división MUST ser siempre en partes iguales. El resultado MUST recalcularse automáticamente cuando cambien el costo total o la cantidad de personas.

#### Scenario: División igualitaria

- **WHEN** existe un servicio activo con costo total 15000,00 y cantidad de personas 3
- **THEN** el sistema muestra "Mi parte" igual a 5000,00

#### Scenario: Servicio sin gasto compartido

- **WHEN** existe un servicio con costo total 8000,00 y cantidad de personas 1
- **THEN** el sistema muestra "Mi parte" igual a 8000,00

#### Scenario: Actualización automática en la tabla

- **WHEN** el usuario modifica el costo total de un servicio visible en la tabla
- **THEN** la fila muestra el nuevo costo y la nueva parte del usuario sin requerir una acción adicional

### Requirement: Congelar un servicio

El sistema SHALL permitir congelar un servicio activo sin eliminar sus datos. La congelación MUST ser indefinida hasta que el usuario lo reactive. El servicio congelado MUST conservar nombre, costo total, cantidad de personas, moneda y notas.

#### Scenario: Congelar un servicio activo

- **WHEN** el usuario congela un servicio activo
- **THEN** el servicio deja de mostrarse en la tabla activa, deja de participar de los totales y sus datos se conservan

#### Scenario: Congelar un servicio ya congelado

- **WHEN** el usuario intenta congelar un servicio que ya está congelado
- **THEN** el sistema no modifica el estado del servicio e informa que ya está congelado

#### Scenario: Persistencia de la congelación

- **WHEN** el usuario congela un servicio y cierra y vuelve a abrir la aplicación
- **THEN** el servicio sigue congelado y fuera de la tabla activa y de los totales

### Requirement: Consultar y reactivar servicios congelados

El sistema SHALL permitir consultar la lista de servicios congelados y reactivar cualquiera de ellos. Al reactivar, el servicio MUST volver a la tabla activa con sus datos intactos y MUST volver a participar de los totales.

#### Scenario: Consulta de congelados

- **WHEN** el usuario accede a la vista de servicios congelados
- **THEN** el sistema lista los servicios congelados con sus datos de origen y su parte calculada

#### Scenario: Reactivación

- **WHEN** el usuario reactiva un servicio congelado
- **THEN** el servicio vuelve a la tabla activa con los mismos datos y los totales se recalculan incluyéndolo

#### Scenario: Reactivar un servicio que no está congelado

- **WHEN** el usuario intenta reactivar un servicio activo
- **THEN** el sistema no modifica el estado del servicio e informa que no está congelado

### Requirement: Excluir servicios congelados de la tabla activa y de los totales

La tabla activa MUST mostrar únicamente servicios con estado activo. Los servicios congelados MUST NOT participar del costo total de servicios activos ni del gasto personal del usuario.

#### Scenario: Tabla activa sin congelados

- **WHEN** existen dos servicios activos y uno congelado
- **THEN** la tabla activa muestra solo los dos servicios activos

#### Scenario: Totales sin congelados

- **WHEN** un servicio con costo total 5000,00 y parte 2500,00 se congela
- **THEN** el costo total de servicios activos disminuye en 5000,00 y el gasto personal disminuye en 2500,00

### Requirement: Mostrar totales de servicios activos

El sistema SHALL mostrar dos totales separados calculados sobre los servicios activos: el costo total de servicios y el gasto personal del usuario. Ambos totales MUST expresarse en ARS y MUST recalcularse cuando se crea, edita, elimina, congela o reactiva un servicio.

#### Scenario: Totales con varios servicios

- **WHEN** existen servicios activos con costos totales 10000,00 (4 personas) y 6000,00 (2 personas)
- **THEN** el sistema muestra costo total de servicios 16000,00 y gasto personal 5500,00

#### Scenario: Totales sin servicios activos

- **WHEN** no existen servicios activos
- **THEN** el sistema muestra ambos totales en 0,00

#### Scenario: Total actualizado tras eliminar

- **WHEN** el usuario elimina un servicio activo
- **THEN** ambos totales se recalculan sin ese servicio

### Requirement: Validar los datos de un servicio

El sistema SHALL validar los datos antes de persistir cualquier alta o edición. El nombre MUST tener al menos un carácter no vacío. El costo total MUST ser un importe válido en ARS no negativo con hasta dos decimales. La cantidad de personas MUST ser un entero mayor o igual a 1. Las notas MUST ser texto libre opcional.

#### Scenario: Costo negativo

- **WHEN** el usuario intenta guardar un servicio con costo total -100,00
- **THEN** el sistema rechaza la operación e informa que el costo no puede ser negativo

#### Scenario: Costo con más de dos decimales

- **WHEN** el usuario intenta guardar un servicio con costo total 100,999
- **THEN** el sistema rechaza la operación e informa el formato de importe admitido

#### Scenario: Nombre compuesto solo por espacios

- **WHEN** el usuario intenta guardar un servicio cuyo nombre contiene solo espacios
- **THEN** el sistema rechaza la operación e indica que el nombre es obligatorio

### Requirement: Limitar los servicios a mensuales en ARS

En la primera versión, el sistema SHALL administrar únicamente servicios de periodicidad mensual y MUST NOT ofrecer un campo de periodicidad, renovación anual o moneda seleccionable. Todo servicio MUST registrarse en ARS.

#### Scenario: Sin campo de periodicidad

- **WHEN** el usuario abre el formulario de alta o edición de un servicio
- **THEN** el formulario no muestra campos de periodicidad, renovación anual ni selección de moneda

#### Scenario: Servicio registrado en ARS

- **WHEN** el usuario guarda un servicio
- **THEN** el servicio queda registrado en ARS y se muestra con ese código de moneda
