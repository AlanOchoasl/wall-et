# Design system — App de Finanzas

> Guía exclusivamente visual para la interfaz. Define identidad visual, jerarquía, composición y apariencia de componentes.
>
> **Este documento no define ni amplía requisitos funcionales.** Las funcionalidades y los comportamientos válidos son únicamente los establecidos en `docs/requisitos.md` y `docs/decisiones.md`. Una acción, control, tarjeta o elemento presente en una referencia visual no debe implementarse como funcionalidad si no está definido allí.
>
> Alcance visual: aplicación personal de escritorio, con tema claro y oscuro, organizada alrededor de los módulos Servicios e Inversiones.

---

## 1. Principios visuales

1. **Datos primero.** La interfaz debe priorizar importes, tablas y comparaciones; evitar elementos decorativos que compitan con la información financiera.
2. **Glassmorphism moderado.** Usar superficies translúcidas y desenfoque de forma selectiva. La legibilidad de tablas y formularios tiene prioridad sobre el efecto.
3. **Identidad púrpura.** El púrpura es el acento principal para selección, foco y acciones primarias.
4. **Dos temas equivalentes.** Tema oscuro y claro deben ofrecer la misma jerarquía, componentes y legibilidad. No tratar el tema claro como una adaptación secundaria.
5. **Jerarquía clara.** Títulos concisos, etiquetas discretas y cifras con peso visual suficiente.
6. **Consistencia.** Mantener radios, espaciados, estados y patrones de interacción visual uniformes.
7. **Sobriedad financiera.** Verde, ámbar y rojo se reservan para comunicar estados o resultados; no usarlos como decoración.

---

## 2. Paleta de color

### 2.1 Tema oscuro

| Token | Valor | Uso |
|---|---|---|
| `--bg-0` | `#0D0E17` | Fondo principal |
| `--bg-1` | `#12131C` | Superficies sólidas, campos y filas |
| `--surface` | `rgba(255,255,255,0.045)` | Tarjetas y paneles |
| `--surface-hover` | `rgba(255,255,255,0.075)` | Hover |
| `--primary` | `#8A62FF` | Acción primaria y selección |
| `--primary-soft` | `#A78BFA` | Acentos y foco |
| `--success` | `#7CFFB2` | Cobertura positiva |
| `--warning` | `#F59E0B` | Advertencias y estados pendientes |
| `--danger` | `#EF4444` | Errores y acciones destructivas |
| `--info` | `#60A5FA` | Información secundaria |
| `--text-1` | `#FFFFFF` | Títulos y cifras principales |
| `--text-2` | `rgba(255,255,255,0.72)` | Texto de cuerpo |
| `--text-3` | `rgba(255,255,255,0.48)` | Etiquetas y texto secundario |
| `--line` | `rgba(255,255,255,0.08)` | Bordes y divisores |

Fondo ambiental opcional: resplandores púrpura y azul muy suaves sobre `--bg-0`. Mantenerlos detrás del contenido y con suficiente uniformidad para que las tablas no pierdan contraste.

### 2.2 Tema claro

| Token | Valor | Uso |
|---|---|---|
| `--bg-0` | `#F5F6FA` | Fondo principal |
| `--bg-1` | `#FFFFFF` | Superficies sólidas, campos y filas |
| `--surface` | `rgba(255,255,255,0.78)` | Tarjetas y paneles |
| `--surface-hover` | `#F0EDFF` | Hover |
| `--primary` | `#7047E8` | Acción primaria y selección |
| `--primary-soft` | `#805FE8` | Acentos y foco |
| `--success` | `#16834B` | Cobertura positiva |
| `--warning` | `#A86408` | Advertencias y estados pendientes |
| `--danger` | `#C93636` | Errores y acciones destructivas |
| `--info` | `#2563A6` | Información secundaria |
| `--text-1` | `#171827` | Títulos y cifras principales |
| `--text-2` | `#45475A` | Texto de cuerpo |
| `--text-3` | `#737589` | Etiquetas y texto secundario |
| `--line` | `rgba(25,25,45,0.12)` | Bordes y divisores |

En el tema claro, el glass debe ser sutil: superficies casi blancas, sombras suaves y blur discreto. Evitar texto blanco o transparencias que reduzcan el contraste.

### 2.3 Colores semánticos

Los colores semánticos se aplican a estados ya definidos por los requisitos, por ejemplo el resultado de cobertura. No crear nuevos estados funcionales por el solo hecho de tener un color disponible.

---

## 3. Superficies y glassmorphism

Receta orientativa para tema oscuro:

```css
.surface {
  background: rgba(255, 255, 255, 0.045);
  border: 1px solid rgba(255, 255, 255, 0.08);
  border-radius: 20px;
  box-shadow: 0 10px 32px rgba(0, 0, 0, 0.18);
  backdrop-filter: blur(20px) saturate(135%);
}
```

Receta orientativa para tema claro:

```css
.surface {
  background: rgba(255, 255, 255, 0.82);
  border: 1px solid rgba(25, 25, 45, 0.08);
  border-radius: 20px;
  box-shadow: 0 8px 28px rgba(35, 30, 70, 0.07);
  backdrop-filter: blur(16px);
}
```

Niveles de superficie:

| Nivel | Uso visual | Tratamiento |
|---|---|---|
| 0 | Fondo de ventana | Color base y gradientes muy suaves |
| 1 | Tarjetas y paneles | Glass discreto |
| 2 | Navegación y barra superior | Un poco más opaco y elevado |
| 3 | Diálogos y menús contextuales, si son necesarios para una operación definida | Fondo opaco, sombra y contraste |

Usar fallback sólido cuando `backdrop-filter` no esté disponible o cuando afecte el rendimiento. No apilar blur excesivo.

---

## 4. Tipografía y cifras

- Familia recomendada: **Inter**. Fallback: `system-ui, -apple-system, "Segoe UI", sans-serif`.
- Pesos: 400, 500 y 600.
- Usar `font-variant-numeric: tabular-nums` para importes, porcentajes y cifras alineadas.
- Los importes deben ser fáciles de comparar y mantener alineación consistente en tablas.

| Rol | Peso | Tamaño / interlineado |
|---|---:|---:|
| Título de pantalla | 600 | 30–34px / 1.2 |
| Título de panel | 600 | 18–20px / 1.3 |
| Cifra destacada | 600 | 30–38px / 1.1 |
| Cuerpo | 400–500 | 14–15px / 1.5 |
| Etiqueta de campo | 500 | 13px / 1.4 |
| Caption | 400 | 12px / 1.4 |
| Encabezado de tabla | 500–600 | 12–13px / 1.4 |

Evitar títulos excesivamente grandes que resten espacio útil a las tablas.

---

## 5. Espaciado, radios y grilla

- Unidad base: **4px**.
- Escala: 4, 8, 12, 16, 20, 24, 32 y 40px.
- Padding de paneles: 20–24px.
- Separación entre paneles: 16–20px.
- Radios: 8px (elementos pequeños), 12px (inputs y botones), 18–20px (tarjetas), 999px (píldoras).
- Márgenes laterales de escritorio: 24–32px.
- Aprovechar el ancho de la ventana para las tablas; permitir scroll horizontal antes que comprimir ilegiblemente columnas.

La composición debe adaptarse al tamaño de la ventana sin convertir la aplicación de escritorio en una interfaz móvil con navegación o funciones nuevas.

---

## 6. Estructura visual de la aplicación

Composición recomendada:

```text
┌──────────────────────────────────────────────────────────────┐
│ Marca / nombre                 Tema       Abrir OpenCode     │
├───────────────┬──────────────────────────────────────────────┤
│ Navegación    │ Título de la sección                         │
│               │                                              │
│ Servicios     │ Contenido del módulo seleccionado            │
│ Inversiones   │                                              │
│               │                                              │
└───────────────┴──────────────────────────────────────────────┘
```

- Navegación principal limitada a los módulos **Servicios** e **Inversiones**.
- La acción de tema claro/oscuro corresponde al requisito de apariencia.
- El botón **Abrir OpenCode** representa únicamente la acción definida en los requisitos: abrir una terminal externa en el directorio del proyecto. No diseñar un chat embebido.
- No agregar buscador global, notificaciones, perfil, espacios/cuentas, reportes, favoritos ni accesos a módulos no definidos.
- No asumir que un elemento de una referencia visual debe convertirse en una acción funcional.
- Mantener el contenido como área principal y evitar paneles laterales decorativos que reduzcan el espacio de las tablas.

---

## 7. Navegación

- Sidebar de escritorio de aproximadamente 220–240px.
- Marca compacta en la parte superior.
- Dos ítems: **Servicios** e **Inversiones**.
- Ítem activo con fondo ligeramente destacado, texto de alto contraste y acento púrpura.
- Íconos lineales simples, de 18–20px.
- El colapso de la sidebar es opcional y exclusivamente visual; no debe introducir accesos o destinos adicionales.
- Si el espacio disponible es reducido, priorizar la visibilidad del contenido y la legibilidad de las tablas.

---

## 8. Componentes visuales

### 8.1 Botones

| Variante | Apariencia |
|---|---|
| Primario | Fondo púrpura, texto blanco, radio 12px, altura 40–44px |
| Secundario | Superficie suave con borde discreto y texto de acento |
| Ghost | Sin fondo sólido; aparece al hover |
| Destructivo | Tratamiento rojo moderado, reservado para eliminar |
| Icono | Área de 36–40px, icono centrado y tooltip accesible si corresponde |

Los botones solo deben aparecer cuando representen operaciones definidas en los requisitos. El estilo no autoriza a crear acciones adicionales.

### 8.2 Campos y formularios

- Altura orientativa: 42–44px.
- Fondo sólido o de baja transparencia.
- Borde de 1px y radio 10–12px.
- Etiqueta siempre visible; no depender únicamente del placeholder.
- Foco con borde púrpura y anillo sutil.
- Error con borde semántico y mensaje breve junto al campo.
- Alinear etiquetas, campos numéricos y ayudas para facilitar la carga de datos.

### 8.3 Tablas

Las tablas son el componente central de ambos módulos.

- Encabezado diferenciado del cuerpo, con tipografía compacta.
- Filas de 48–56px, separadores sutiles y hover discreto.
- Importes y porcentajes alineados a la derecha.
- Texto descriptivo alineado a la izquierda.
- Cifras con tabular-nums.
- Evitar convertir cada celda en una tarjeta: la comparación horizontal entre columnas debe ser inmediata.
- En ventanas estrechas, conservar la estructura tabular mediante scroll horizontal.

### 8.4 Resúmenes de importes

- Mostrar las cifras principales con tamaño y peso superiores al texto auxiliar.
- Etiqueta descriptiva siempre visible.
- Mostrar el código de moneda (ARS o USD) junto a los importes según el alcance actual; no introducir cotizaciones ni conversión.
- Cuando se presenten dos totales relacionados, darles jerarquía equivalente y diferenciarlos con etiquetas claras.
- No agregar KPIs, porcentajes de variación ni métricas que no estén contemplados en los requisitos.

### 8.5 Indicador de cobertura

- Resultado positivo: verde semántico y texto `Cubierto`.
- Déficit: rojo o ámbar semántico, con `Déficit de: [importe exacto]` legible, código de moneda y fracción exacta cuando corresponda.
- Gastos en cero y monedas incompatibles: texto `Gastos en cero` o `Monedas incompatibles`, sin veredicto ni importe faltante.
- El sufijo `(disponible)` acompaña solo a `Cubierto` y `Déficit` cuando el estado general es disponible; los estados sin veredicto conservan la etiqueta provisional o no disponible de la ruta.
- La etiqueta debe acompañar siempre al color; no comunicar el estado únicamente mediante color.
- Cuando se muestren resultados del cálculo provisional, identificarlos visualmente como **provisionales / de demostración**. No presentarlos como resultados financieros definitivos.

### 8.6 Estado congelado

El estado congelado puede usar un badge neutro o ámbar en la vista donde se consulten los servicios congelados. No mostrar esos servicios dentro de la tabla de activos ni incluirlos en los totales activos.

### 8.7 Estados vacíos

- Icono simple y discreto.
- Mensaje breve que explique qué lista o sección está vacía.
- Si corresponde, mostrar únicamente la acción de creación ya definida para ese módulo.
- Evitar mensajes que prometan funciones todavía no implementadas.

### 8.8 Diálogos y confirmaciones

Cuando una operación definida requiera confirmación, usar un diálogo centrado, de ancho moderado, con título, explicación concreta y acciones claramente diferenciadas. El alta y la edición de servicios comparten la apariencia de un único diálogo centrado, con el título y el estado del selector de moneda que correspondan a cada operación. Este apartado establece solo su apariencia; la necesidad y el momento de confirmar dependen de los requisitos.

---

## 9. Pantallas y aplicación de componentes

Esta sección orienta la composición visual de las funcionalidades existentes. No agrega operaciones ni reglas de negocio.

### 9.1 Servicios

Prioridad visual:
1. Título **Servicios**.
2. Resumen del costo total de servicios activos.
3. Resumen de **Mi gasto**.
4. Tabla de servicios activos.
5. Acción para agregar un servicio y acciones por fila que correspondan a las operaciones definidas.
6. Acceso visual a la consulta de servicios congelados, sin mezclarlos con los activos.

La tabla debe representar los datos definidos en requisitos: servicio, costo total, personas que comparten, mi parte y notas. Las acciones de edición, congelación y eliminación pueden ubicarse en una columna compacta. La acción de alta puede ubicarse junto al título de la sección y la edición puede presentarse como acción por fila del mismo diálogo de formulario.

No incluir renovación anual ni controles de reparto desigual. La parte personal es un valor calculado y debe distinguirse visualmente de los datos editables. El formulario incluye el selector explícito ARS/USD con ARS como valor predeterminado.

### 9.2 Inversiones

Prioridad visual:
1. Título **Inversiones**.
2. Tabla de inversiones asociadas a cada mes.
3. Campos de entrada y resultados calculados claramente diferenciados.
4. Indicador de cobertura de gastos.

La tabla debe representar los datos definidos en requisitos: mes, moneda, capital invertido, TNA, plazo en días, impuestos, rendimiento bruto, gastos del mes, rendimiento neto y resultado de cobertura.

El formulario incluye el selector explícito ARS/USD con ARS como valor predeterminado. Los datos ingresados manualmente deben distinguirse de los resultados calculados. La apariencia no debe sugerir que los resultados provisionales son definitivos.

No agregar gráficos históricos, proyecciones, cotizaciones ni métricas de inversión no definidas.

### 9.3 Acción OpenCode

El botón puede ubicarse en la barra superior, con icono de terminal y etiqueta **Abrir OpenCode** o **Abrir terminal**. Debe comunicar que abre una aplicación externa. No representarlo como un chat, asistente integrado o editor de datos financieros.

---

## 10. Iconografía

- Estilo lineal, trazo aproximado de 1.5–2px.
- Tamaño: 18–20px en navegación y 16–18px en botones.
- Color por defecto: texto secundario; activo: texto principal o púrpura.
- Usar una familia consistente (Lucide o equivalente).
- Los iconos acompañan etiquetas comprensibles; no reemplazan texto en acciones importantes.

---

## 11. Movimiento y estados

- Hover y pressed: 120–180ms, `ease-out`.
- Apertura de diálogos: 180–220ms, fade y escala sutil.
- Cambios de selección: transición breve de color y fondo.
- Evitar animaciones de entrada escalonadas o efectos que retrasen la interacción con tablas.
- Respetar `prefers-reduced-motion`.

El movimiento es decorativo y no debe cambiar reglas ni resultados de negocio.

---

## 12. Accesibilidad y rendimiento

- Contraste mínimo objetivo de 4.5:1 para texto normal.
- Foco visible en todos los controles interactivos.
- No depender solo del color para comunicar estados.
- Etiquetas asociadas a sus campos.
- Tablas con encabezados semánticos.
- Soporte de navegación por teclado.
- Respetar preferencias de movimiento reducido.
- Reducir o desactivar blur en equipos donde afecte el rendimiento.
- Verificar ambos temas en tablas, formularios, diálogos, estados vacíos y mensajes de validación.

---

## 13. Tokens CSS de referencia

Los tokens se pueden adaptar a la estrategia de estilos elegida por el proyecto.

```css
:root {
  --primary: #7047E8;
  --primary-soft: #805FE8;
  --success: #16834B;
  --warning: #A86408;
  --danger: #C93636;
  --info: #2563A6;

  --r-sm: 8px;
  --r-md: 12px;
  --r-lg: 20px;
  --r-pill: 999px;

  --space-1: 4px;
  --space-2: 8px;
  --space-3: 12px;
  --space-4: 16px;
  --space-5: 20px;
  --space-6: 24px;

  --font: Inter, system-ui, -apple-system, "Segoe UI", sans-serif;
}

[data-theme="dark"] {
  color-scheme: dark;
  --bg-0: #0D0E17;
  --bg-1: #12131C;
  --surface: rgba(255,255,255,0.045);
  --surface-hover: rgba(255,255,255,0.075);
  --text-1: #FFFFFF;
  --text-2: rgba(255,255,255,0.72);
  --text-3: rgba(255,255,255,0.48);
  --line: rgba(255,255,255,0.08);
}

[data-theme="light"] {
  color-scheme: light;
  --bg-0: #F5F6FA;
  --bg-1: #FFFFFF;
  --surface: rgba(255,255,255,0.82);
  --surface-hover: #F0EDFF;
  --text-1: #171827;
  --text-2: #45475A;
  --text-3: #737589;
  --line: rgba(25,25,45,0.12);
}
```

---

## 14. Reglas rápidas

**Hacer**
- Mantener coherencia entre tema claro y oscuro.
- Dar prioridad a la lectura y comparación de tablas.
- Usar púrpura para foco, selección y acciones principales.
- Mantener importes alineados y con cifras tabulares.
- Usar glass de forma moderada y con fallback sólido.
- Etiquetar explícitamente resultados provisionales.

**Evitar**
- Incorporar buscador global, notificaciones, perfil, espacios, reportes o gráficos históricos por imitación de una referencia.
- Agregar botones o controles cuya funcionalidad no esté definida.
- Mostrar servicios congelados como activos o sumarlos a los totales.
- Representar cálculos provisionales como definitivos.
- Introducir cotizaciones, conversión de moneda o historial en la interfaz inicial.
- Sacrificar contraste por transparencias, blur o efectos decorativos.

---

## 15. Checklist visual

- [ ] Tema claro y oscuro coherentes.
- [ ] Navegación limitada a Servicios e Inversiones.
- [ ] Tablas legibles y adaptables al ancho de la ventana.
- [ ] Formularios con etiquetas, foco y validación visual.
- [ ] Totales de servicios claramente diferenciados.
- [ ] Resultados provisionales de inversiones identificados como tales.
- [ ] Acción OpenCode representada como apertura de terminal externa.
- [ ] Contraste, teclado y movimiento reducido verificados.
- [ ] Ningún elemento visual interpretado como requisito funcional nuevo.
