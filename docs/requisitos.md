# Requisitos — App de Finanzas Personales

**Versión:** 0.1  
**Estado:** Borrador inicial para revisión  
**Tipo:** Aplicación personal de escritorio, local y extensible.

## 1. Objetivo

Desarrollar una aplicación de escritorio para administrar finanzas personales, controlar gastos recurrentes y analizar inversiones mediante cauciones.

El objetivo principal es conocer el costo total de los servicios, cuánto corresponde pagar al usuario y si el rendimiento neto de las inversiones alcanza para cubrir los gastos considerados.

## 2. Alcance y decisiones confirmadas

- Aplicación personal de escritorio inicialmente.
- Funcionamiento local.
- Alcance inicial intermedio.
- Soporte de múltiples monedas.
- Temas claro y oscuro.
- Servicios mensuales en la primera versión; se descartan los servicios anuales por ahora.
- División de gastos compartidos siempre en partes iguales.
- Los resultados calculados no se editan directamente: se modifican sus datos de origen.
- Se requiere contemplar una futura integración con OpenCode/IA.

## 3. Requisitos funcionales

### RF-01 — Gestión de servicios

La aplicación permitirá administrar gastos recurrentes (por ejemplo, Spotify o Crunchyroll) mediante una tabla.

#### Datos de un servicio
- Nombre del servicio.
- Costo total.
- Cantidad de personas que comparten el gasto.
- Mi parte (calculada).
- Moneda.
- Notas.
- Estado (activo o congelado).

La columna de renovación mensual/anual queda fuera del alcance inicial. Todos los servicios administrados en esta primera versión serán mensuales.

#### Reglas de cálculo
- La cantidad de personas incluye al usuario.
- La división es siempre en partes iguales.
- Fórmula: `Mi parte = Costo total / Cantidad de personas`.
- La cantidad de personas debe ser un entero mayor o igual a 1.
- `Mi parte` es un resultado calculado, no un campo editable.
- Para cambiar el resultado se modifican el costo total o la cantidad de personas; el resultado se recalcula automáticamente.

#### Operaciones
- Agregar un servicio.
- Editar sus datos de origen y notas.
- Eliminar un servicio.
- Congelar un servicio sin eliminar sus datos.
- Consultar servicios congelados y reactivarlos conservando sus datos.

Un servicio congelado deja de mostrarse en la tabla activa y deja de participar de los totales. La definición exacta de si la congelación se administra por mes o hasta una reactivación queda pendiente.

#### Totales
Mostrar por separado:
1. **Costo total de servicios:** suma de los costos completos de los servicios activos.
2. **Mi gasto:** suma de las partes del usuario correspondientes a los servicios activos.

Los importes de distintas monedas no deben sumarse entre sí. Los totales deben agruparse por moneda hasta que se defina una política de conversión.

### RF-02 — Gestión de inversiones

Administrar inversiones en cauciones mediante una tabla en la que cada fila represente una inversión asociada a un mes.

#### Datos previstos
- Mes.
- Capital invertido.
- Moneda.
- TNA.
- Plazo en días.
- Rendimiento bruto (calculado).
- Gastos del mes.
- Rendimiento neto después de impuestos (calculado).
- Resultado de cobertura (calculado).

#### Reglas
- Capital, TNA y plazo son datos de origen ingresados por el usuario.
- El rendimiento bruto se calcula a partir del capital, la TNA y el plazo.
- El rendimiento neto se calcula restando los impuestos aplicables al rendimiento bruto.
- Los resultados calculados no se editan directamente: se modifican sus datos de origen.
- La fórmula exacta del rendimiento y el tratamiento impositivo quedan pendientes de definición.

#### Cobertura
Comparar el rendimiento neto con los gastos del mes:
- Si el rendimiento neto es suficiente: `¡Sí cubre!`.
- Si no alcanza: `Déficit de: $[importe faltante]`.

La comparación solo es válida entre importes expresados en la misma moneda o luego de aplicar una política de conversión aprobada.

### RF-03 — Relación entre servicios e inversiones

- El gasto personal de servicios alimentará inicialmente el dato `Gastos del mes` de las inversiones.
- Los cambios en servicios activos deberán actualizar los totales correspondientes.
- Los resultados de cobertura deberán recalcularse cuando cambien los gastos o los datos de inversión.
- Está pendiente definir si `Gastos del mes` incluirá únicamente servicios o también otros tipos de gastos.

### RF-04 — Integración con OpenCode e IA

La arquitectura deberá permitir que OpenCode interactúe con la aplicación desde la terminal y, a futuro, que la interfaz incluya un mecanismo para solicitar cambios a un modelo de IA.

Capacidades previstas:
- Consultar servicios e inversiones.
- Agregar y modificar registros.
- Modificar datos de origen.
- Consultar resultados calculados.

Condiciones:
- Las operaciones externas utilizarán las mismas reglas de negocio y validaciones que la interfaz.
- No se permitirá acceso irrestricto ni escritura directa a la base de datos.
- Las modificaciones deberán validarse y contar con un mecanismo de confirmación y registro.
- El alcance preciso de permisos y confirmaciones queda pendiente.

## 4. Requisitos no funcionales

- **Localidad:** las funciones principales deben operar localmente, sin depender de conexión permanente a Internet.
- **Persistencia:** conservar los datos entre ejecuciones.
- **Integridad:** centralizar reglas y cálculos.
- **Mantenibilidad:** separar interfaz, aplicación, dominio y persistencia.
- **Usabilidad:** tablas legibles, edición sencilla, navegación clara y temas claro/oscuro.
- **Seguridad:** validar operaciones, especialmente las provenientes de herramientas externas.
- **Extensibilidad:** evitar acoplamientos que dificulten incorporar módulos o una futura aplicación móvil.

## 5. Fuera del alcance inicial

- Servicios con renovación anual.
- Sincronización en la nube.
- Conexión automática con bancos o plataformas financieras.
- Importación automática de movimientos bancarios.
- Editor de fórmulas de estilo Excel.
- División desigual de gastos compartidos.
- Aplicación móvil terminada.

## 6. Decisiones pendientes

1. Política de conversión y visualización de múltiples monedas.
2. Regla y porcentaje de impuestos aplicables a las cauciones.
3. Convención de días y fórmula exacta para el rendimiento.
4. Si `Gastos del mes` incluye solamente servicios o también otros gastos.
5. Conservación histórica de costos, tasas, inversiones y resultados.
6. Si congelar un servicio se define por mes o hasta reactivarlo.
7. Permisos de OpenCode, operaciones que requieren confirmación y registro de cambios.
8. Tratamiento de redondeos en la división de costos compartidos y en los cálculos financieros.
