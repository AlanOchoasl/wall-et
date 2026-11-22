## Purpose

Permite registrar inversiones en cauciones asociadas a un mes, capturar sus datos de origen, derivar rendimiento y cobertura a partir de las reglas financieras aprobadas y alimentar los gastos del mes desde los servicios activos.

## ADDED Requirements

### Requirement: Registrar una inversión asociada a un mes

El sistema SHALL permitir crear una inversión indicando mes, capital invertido, TNA, plazo en días e impuestos. Todos los campos de entrada MUST comenzar en cero al abrir un formulario de alta. La inversión MUST quedar asociada al mes indicado y MUST registrarse en ARS.

#### Scenario: Alta exitosa

- **WHEN** el usuario ingresa mes 2026-03, capital 1000000,00, TNA 30, plazo 30 días e impuestos 0,00 y guarda
- **THEN** el sistema crea la inversión asociada a 2026-03 con sus datos de origen

#### Scenario: Formulario de alta con valores iniciales en cero

- **WHEN** el usuario abre el formulario de alta de una inversión
- **THEN** los campos capital, TNA, plazo e impuestos muestran 0 como valor inicial

#### Scenario: Mes obligatorio

- **WHEN** el usuario intenta guardar una inversión sin indicar mes
- **THEN** el sistema rechaza la operación e informa que el mes es obligatorio

#### Scenario: Mes con formato inválido

- **WHEN** el usuario intenta guardar una inversión con un mes que no respeta el formato año-mes
- **THEN** el sistema rechaza la operación e informa el formato admitido

### Requirement: Editar datos de origen de una inversión

El sistema SHALL permitir modificar mes, capital, TNA, plazo e impuestos de una inversión existente. Los valores derivados (rendimiento bruto, gastos del mes, rendimiento neto y cobertura) MUST NOT ser editables y MUST recalcularse al cambiar cualquier dato de origen.

#### Scenario: Edición del capital

- **WHEN** el usuario modifica el capital de una inversión
- **THEN** el sistema guarda el nuevo capital y recalcula los resultados derivados

#### Scenario: Intento de editar un resultado

- **WHEN** el usuario intenta modificar directamente el rendimiento bruto, los gastos del mes, el rendimiento neto o la cobertura
- **THEN** el sistema no acepta el cambio y mantiene el valor derivado

#### Scenario: Edición con datos inválidos

- **WHEN** el usuario intenta guardar una edición con un importe negativo o un plazo no entero
- **THEN** el sistema rechaza la operación, conserva los datos previos y muestra el error correspondiente

### Requirement: Calcular los gastos del mes desde los servicios activos

El sistema SHALL calcular los gastos del mes de cada inversión como la suma de las partes del usuario correspondientes a los servicios activos. Los gastos del mes MUST NOT ser editables y MUST recalcularse cuando se crea, edita, elimina, congela o reactiva un servicio. En la primera versión los gastos del mes MUST incluir únicamente servicios.

#### Scenario: Gastos del mes con servicios activos

- **WHEN** existen servicios activos con partes del usuario de 2500,00 y 3000,00
- **THEN** la inversión muestra gastos del mes igual a 5500,00

#### Scenario: Gastos del mes sin servicios activos

- **WHEN** no existen servicios activos
- **THEN** la inversión muestra gastos del mes igual a 0,00

#### Scenario: Actualización tras congelar un servicio

- **WHEN** el usuario congela un servicio cuya parte es 2500,00
- **THEN** los gastos del mes de las inversiones disminuyen en 2500,00

### Requirement: Derivar el rendimiento bruto

El sistema SHALL calcular el rendimiento bruto a partir del capital invertido, la TNA y el plazo en días. Con la fórmula definitiva aprobada, MUST usar exclusivamente esa fórmula y su convención de días. Mientras la fórmula definitiva no esté aprobada, el sistema MAY mostrar un cálculo provisional de demostración solo si su fórmula fue aprobada explícitamente; ese resultado MUST identificarse como provisional y MUST NOT presentarse como definitivo. Sin fórmula provisional aprobada, el sistema MUST mostrar el rendimiento bruto como no disponible.

#### Scenario: Fórmula definitiva pendiente con cálculo provisional aprobado

- **WHEN** la fórmula definitiva no está aprobada y el usuario aprobó una fórmula provisional de demostración
- **THEN** la inversión muestra el rendimiento bruto identificado como provisional y no como resultado definitivo

#### Scenario: Fórmula definitiva pendiente sin cálculo provisional aprobado

- **WHEN** la fórmula definitiva no está aprobada y no existe una fórmula provisional aprobada
- **THEN** la inversión muestra el rendimiento bruto como no disponible y no presenta ningún valor calculado

#### Scenario: Fórmula definitiva aprobada

- **WHEN** la fórmula definitiva está aprobada y el usuario modifica el capital, la TNA o el plazo
- **THEN** el sistema recalcula el rendimiento bruto con la fórmula aprobada y lo presenta como disponible

### Requirement: Derivar el rendimiento neto

El sistema SHALL calcular el rendimiento neto restando al rendimiento bruto los impuestos correspondientes al período, según el tratamiento impositivo que se apruebe. Mientras el tratamiento impositivo no esté aprobado, el sistema MAY mostrar un rendimiento neto provisional solo si el cálculo provisional y el tratamiento aplicado fueron aprobados explícitamente; ese resultado MUST identificarse como provisional. Sin aprobación, el sistema MUST mostrar el rendimiento neto como no disponible. El valor de impuestos ingresado manualmente MUST conservarse como dato de origen.

#### Scenario: Tratamiento impositivo pendiente sin cálculo provisional aprobado

- **WHEN** la decisión sobre el tratamiento impositivo no está aprobada y no existe un cálculo provisional aprobado
- **THEN** la inversión muestra el rendimiento neto como no disponible

#### Scenario: Rendimiento neto provisional

- **WHEN** el tratamiento impositivo definitivo no está aprobado y existe un cálculo provisional aprobado
- **THEN** la inversión muestra el rendimiento neto identificado como provisional

#### Scenario: Conservación del impuesto ingresado

- **WHEN** el usuario ingresa impuestos 500,00 y guarda la inversión
- **THEN** el sistema conserva 500,00 como dato de origen de la inversión

### Requirement: Evaluar la cobertura del mes

El sistema SHALL comparar el rendimiento neto con los gastos del mes. Si el rendimiento neto es mayor o igual que los gastos del mes, MUST mostrar el texto "¡Sí cubre!". Si es menor, MUST mostrar "Déficit de: $[importe faltante]" con el importe expresado en ARS. Cuando el rendimiento neto sea provisional, el veredicto MUST identificarse como provisional. Si el rendimiento neto no está disponible, el sistema MUST mostrar el estado de cobertura como no disponible y MUST NOT emitir un veredicto.

#### Scenario: Cobertura suficiente

- **WHEN** el rendimiento neto disponible es 8000,00 y los gastos del mes son 5500,00
- **THEN** el sistema muestra "¡Sí cubre!"

#### Scenario: Cobertura insuficiente

- **WHEN** el rendimiento neto disponible es 4000,00 y los gastos del mes son 5500,00
- **THEN** el sistema muestra "Déficit de: $1500,00"

#### Scenario: Cobertura con rendimiento provisional

- **WHEN** el rendimiento neto es provisional y alcanza los gastos del mes
- **THEN** el sistema muestra "¡Sí cubre!" identificado como provisional

#### Scenario: Cobertura no evaluable

- **WHEN** el rendimiento neto no está disponible por una decisión financiera pendiente
- **THEN** el sistema muestra la cobertura como no disponible y no muestra "¡Sí cubre!" ni un déficit

#### Scenario: Cobertura con gastos en cero

- **WHEN** el rendimiento neto disponible es mayor o igual que 0 y los gastos del mes son 0,00
- **THEN** el sistema muestra "¡Sí cubre!"

### Requirement: Recalcular al cambiar datos de servicios o inversiones

El sistema SHALL recalcular los resultados derivados de una inversión cuando cambien sus datos de origen o los datos que alimentan los gastos del mes. Los resultados mostrados MUST corresponder siempre al estado actual de los datos de origen.

#### Scenario: Cambio en una inversión

- **WHEN** el usuario modifica la TNA de una inversión
- **THEN** los resultados derivados de esa inversión se recalculan y se muestran actualizados

#### Scenario: Cambio en un servicio

- **WHEN** el usuario modifica el costo total de un servicio activo
- **THEN** los gastos del mes y la cobertura de las inversiones se recalculan con el nuevo valor

### Requirement: Validar los datos de una inversión

El sistema SHALL validar los datos antes de persistir cualquier alta o edición. El mes MUST ser obligatorio y respetar el formato año-mes. El capital, la TNA y los impuestos MUST ser valores numéricos no negativos. El plazo MUST ser un entero no negativo expresado en días.

#### Scenario: Capital negativo

- **WHEN** el usuario intenta guardar una inversión con capital -1,00
- **THEN** el sistema rechaza la operación e informa que el capital no puede ser negativo

#### Scenario: Plazo no entero

- **WHEN** el usuario intenta guardar una inversión con plazo 30,5 días
- **THEN** el sistema rechaza la operación e informa que el plazo debe ser un número entero de días

#### Scenario: TNA no numérica

- **WHEN** el usuario intenta guardar una inversión con TNA "treinta"
- **THEN** el sistema rechaza la operación e informa que la TNA debe ser un valor numérico

### Requirement: Presentar la tabla de inversiones

El sistema SHALL mostrar las inversiones en una tabla donde cada fila representa una inversión asociada a un mes e incluye mes, capital, TNA, plazo, impuestos, rendimiento bruto, gastos del mes, rendimiento neto y cobertura. Los campos derivados MUST mostrarse de solo lectura y MUST identificarse como no disponibles cuando corresponda.

#### Scenario: Fila con resultados no disponibles

- **WHEN** existe una inversión con datos de origen válidos, las decisiones financieras están pendientes y no existe un cálculo provisional aprobado
- **THEN** la fila muestra los datos de origen y muestra rendimiento bruto, rendimiento neto y cobertura como no disponibles

#### Scenario: Fila con resultados provisionales

- **WHEN** existe una inversión y hay un cálculo provisional aprobado pero no la fórmula definitiva
- **THEN** la fila muestra los resultados identificados como provisionales y no como definitivos

#### Scenario: Fila con datos de origen incompletos

- **WHEN** existe una inversión guardada con capital 0,00 y plazo 0
- **THEN** la fila muestra los valores de origen y el estado de los resultados derivados según las reglas aprobadas, sin inventar un rendimiento
