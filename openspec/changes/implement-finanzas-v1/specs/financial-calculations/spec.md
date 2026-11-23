## Purpose

Define las reglas transversales de cálculo que garantizan que todos los resultados financieros se obtengan de una única fuente, se recalculen de forma determinista y nunca se editen directamente.

## ADDED Requirements

### Requirement: Centralizar los cálculos en una única fuente

Todo resultado financiero SHALL obtenerse de una única implementación autoritativa de las reglas de negocio. La interfaz MUST NOT calcular por su cuenta la parte del usuario, los totales, los gastos del mes, el rendimiento bruto, el rendimiento neto ni la cobertura; MUST limitarse a mostrar los valores recibidos. Cualquier operación externa futura MUST utilizar la misma implementación.

#### Scenario: La interfaz muestra valores calculados

- **WHEN** la interfaz necesita mostrar la parte del usuario de un servicio
- **THEN** muestra el valor provisto por la capa de cálculo y no lo recalcula a partir de los campos de la tabla

#### Scenario: Consistencia entre consumidores

- **WHEN** dos consumidores distintos consultan la misma operación sobre los mismos datos
- **THEN** ambos obtienen el mismo resultado

#### Scenario: Cambio de regla en un solo lugar

- **WHEN** se modifica una regla de cálculo aprobada
- **THEN** el cambio se aplica en un único punto y todos los resultados afectados se actualizan de forma consistente

### Requirement: Impedir la edición directa de resultados

El sistema SHALL tratar como derivados todos los valores calculados: parte del usuario, costo total de servicios activos, gasto personal, gastos del mes, rendimiento bruto, rendimiento neto y cobertura. Un resultado derivado MUST NOT persistirse como valor editable ni aceptar modificaciones directas; para cambiarlo MUST modificarse su dato de origen y recalcular.

#### Scenario: Resultado derivado no editable

- **WHEN** el usuario intenta escribir directamente sobre un resultado derivado
- **THEN** el sistema rechaza el cambio y mantiene el valor calculado

#### Scenario: Cambio mediante datos de origen

- **WHEN** el usuario necesita un resultado distinto
- **THEN** modifica el dato de origen correspondiente y el sistema recalcula el resultado

### Requirement: Recalcular de forma determinista

El sistema SHALL producir el mismo resultado para los mismos datos de origen y las mismas reglas aprobadas. El recálculo MUST ejecutarse ante cualquier cambio de datos de origen y MUST NOT depender del orden de visualización ni del estado de la interfaz.

#### Scenario: Determinismo

- **WHEN** se calcula dos veces un resultado con los mismos datos de origen y reglas
- **THEN** se obtiene exactamente el mismo valor

#### Scenario: Recálculo tras cada cambio

- **WHEN** cambia un dato de origen que alimenta un resultado derivado
- **THEN** el resultado derivado se recalcula antes de mostrarse

### Requirement: Comparar únicamente importes de la misma moneda

El sistema SHALL comparar y sumar importes solo cuando estén expresados en la misma moneda. En la primera versión los importes pueden estar en ARS o USD. El sistema MUST NOT convertir monedas, sumar importes de monedas distintas ni comparar una inversión con gastos de otra moneda; los totales MUST calcularse por moneda.

#### Scenario: Suma de totales por moneda

- **WHEN** el sistema suma los costos de servicios activos
- **THEN** suma únicamente importes de la misma moneda y expresa cada total en esa moneda

#### Scenario: Comparación de cobertura

- **WHEN** el sistema compara el rendimiento neto con los gastos del mes
- **THEN** verifica que ambos importes tengan la misma moneda antes de emitir un resultado de cobertura

#### Scenario: Intento de comparar monedas distintas

- **WHEN** el sistema recibe importes de monedas distintas para una misma comparación
- **THEN** rechaza la comparación e informa que no puede operar con monedas diferentes

### Requirement: Representar importes monetarios con precisión exacta

El sistema SHALL almacenar y operar los importes monetarios sin pérdida de precisión por punto flotante binario. Los importes de origen MUST conservarse exactamente como fueron ingresados, con hasta dos decimales para ARS y USD. Durante la fase 3, la precisión interna MUST conservar las fracciones exactas sin redondeo anticipado y la persistencia MUST NOT almacenar importes derivados redondeados. La presentación MAY mostrar dos decimales solo como formato, sin que ello constituya una política de redondeo aprobada. D-028 sigue pendiente: falta decidir cómo representar resultados fraccionarios y cómo resolver diferencias entre importes individuales mostrados y totales.

#### Scenario: Conservación exacta de un importe

- **WHEN** el usuario ingresa 10000,10
- **THEN** el sistema conserva y muestra 10000,10 sin variación

#### Scenario: División con resultado no entero

- **WHEN** el usuario divide un costo total de 10000,00 entre 3 personas y la política de redondeo no está aprobada
- **THEN** el sistema no redondea silenciosamente y no presenta el resultado como definitivo

#### Scenario: Operaciones sucesivas

- **WHEN** el sistema realiza varias operaciones sobre importes en ARS o USD sin mezclar monedas
- **THEN** no acumula errores de representación binaria

### Requirement: Definir la fórmula de rendimiento de cauciones

El sistema SHALL calcular el rendimiento bruto de una caución con una fórmula y una convención de días aprobadas explícitamente. Mientras esa decisión esté pendiente, la ruta de rendimiento definitivo MUST permanecer deshabilitada y devolver `no_disponible`; el sistema MUST NOT usar una fórmula provisional como si fuera definitiva. El cálculo provisional aprobado PV-01 se implementa en un módulo aislado y se etiqueta como `provisional`.

#### Scenario: Decisión pendiente, ruta definitiva

- **WHEN** se evalúa la ruta definitiva mientras no existe una decisión aprobada sobre fórmula y convención de días
- **THEN** el sistema no calcula rendimiento bruto y lo informa como no disponible en esa ruta

#### Scenario: Decisión pendiente con cálculo provisional aprobado

- **WHEN** la fórmula definitiva está pendiente y existe el cálculo provisional aprobado PV-01
- **THEN** el sistema calcula el rendimiento bruto con PV-01 en el módulo aislado y lo informa como `provisional`, no como definitivo

#### Scenario: Fórmula definitiva aprobada

- **WHEN** existe una decisión aprobada sobre fórmula y convención de días
- **THEN** el sistema calcula el rendimiento bruto exclusivamente con esa fórmula y lo presenta como disponible

### Requirement: Aislar el cálculo provisional

El sistema SHALL implementar cualquier cálculo provisional de demostración en un módulo separado de las reglas definitivas, de modo que pueda reemplazarse sin modificar la presentación ni duplicar lógica. Un resultado provisional MUST identificarse como tal en todos los lugares donde se muestre y MUST NOT usarse como veredicto definitivo ni presentarse como asesoramiento financiero. La fórmula provisional MUST contar con aprobación explícita antes de implementarse.

#### Scenario: Sustitución del cálculo provisional

- **WHEN** se aprueba la fórmula definitiva y se reemplaza el cálculo provisional
- **THEN** la presentación y las reglas de servicios no requieren cambios para adoptar el cálculo definitivo

#### Scenario: Etiquetado del resultado provisional

- **WHEN** el sistema muestra un rendimiento o una cobertura obtenidos con el cálculo provisional
- **THEN** el valor se identifica como provisional en la tabla y en el indicador correspondiente

#### Scenario: Fórmula provisional aprobada

- **WHEN** existe el cálculo provisional aprobado PV-01
- **THEN** el sistema lo implementa en el módulo aislado y lo identifica como provisional

### Requirement: Definir el tratamiento impositivo

El sistema SHALL descontar del rendimiento bruto los impuestos aplicables según un tratamiento aprobado explícitamente. Mientras el tratamiento definitivo esté pendiente, la ruta definitiva MUST devolver `no_disponible` y el sistema MUST conservar el impuesto ingresado como dato de origen. El cálculo provisional aprobado PV-02 se implementa en el módulo aislado y se etiqueta como `provisional`.

#### Scenario: Tratamiento pendiente, ruta definitiva

- **WHEN** se evalúa la ruta definitiva mientras no existe una decisión aprobada sobre el tratamiento impositivo
- **THEN** el sistema no calcula rendimiento neto y lo informa como no disponible en esa ruta

#### Scenario: Tratamiento pendiente con cálculo provisional aprobado

- **WHEN** el tratamiento impositivo definitivo está pendiente y existe el cálculo provisional aprobado PV-02
- **THEN** el sistema calcula el rendimiento neto con PV-02 en el módulo aislado y lo informa como `provisional`, no como definitivo

#### Scenario: Tratamiento aprobado

- **WHEN** existe una decisión aprobada sobre el tratamiento impositivo
- **THEN** el sistema calcula el rendimiento neto aplicando exclusivamente ese tratamiento

### Requirement: Identificar el estado de cada resultado

El sistema SHALL asociar a cada resultado derivado un estado explícito: disponible, no disponible o provisional. Un resultado MUST presentarse como provisional solo si se basa en un cálculo demostrativo aprobado y aislado de la implementación definitiva. Un resultado no disponible MUST NOT presentarse como definitivo ni habilitar comparaciones.

#### Scenario: Resultado no disponible

- **WHEN** un resultado depende de una decisión pendiente
- **THEN** se muestra con el estado no disponible y no se usa para comparaciones

#### Scenario: Resultado disponible

- **WHEN** un resultado se calcula con reglas aprobadas y datos válidos
- **THEN** se muestra como disponible

#### Scenario: Resultado provisional

- **WHEN** un resultado se obtiene de un cálculo demostrativo aprobado
- **THEN** se muestra identificado como provisional y no se usa como veredicto definitivo
