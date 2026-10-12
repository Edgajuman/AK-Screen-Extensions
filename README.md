# AK Screen · Extensiones

Widgets y herramientas para AK Screen 2.4.0, con versiones revisadas y catálogo firmado.

[**Catálogo web →**](https://edgajuman.github.io/AK-Screen-Extensions/) · [SDK](sdk/README.md) · [Descargas](https://github.com/Edgajuman/AKScreen-Downloads/releases/tag/ak-v2.4.0)

## Colección de dehuman · 1.3.0

### Audio Workbench

![Vista ilustrativa de Audio Workbench](https://edgajuman.github.io/AK-Screen-Extensions/previews/audio-workbench.png)

Widgets rápidos para limpiar voz, controlar dinámica y preparar música desde la selección actual.

[Ejemplos y componentes](extensions/dehuman/audio-workbench/1.3.0/EXAMPLE.md) · 4 widgets · 16 acciones.

### Brand Titles

![Vista ilustrativa de Brand Titles](https://edgajuman.github.io/AK-Screen-Extensions/previews/brand-titles.png)

Títulos de marca con degradados, contorno, fondo, fuente y entrada animada. Crea capas de texto editables sin descargar medios.

[Ejemplos y componentes](extensions/dehuman/brand-titles/1.3.0/EXAMPLE.md) · 1 widgets · 1 acciones.

### Camera Tools

![Vista ilustrativa de Camera Tools](https://edgajuman.github.io/AK-Screen-Extensions/previews/camera-tools.png)

Cámara compartida 2D/3D y presets.

[Ejemplos y componentes](extensions/dehuman/camera-tools/1.3.0/EXAMPLE.md) · 5 widgets · 9 acciones.

### Chart Atelier

![Vista ilustrativa de Chart Atelier](https://edgajuman.github.io/AK-Screen-Extensions/previews/chart-atelier.png)

Crea comparaciones animadas con barras, líneas, anillos y gráficos circulares. Colores, duración, valores y contornos editables; todas las capas quedan en la timeline.

[Ejemplos y componentes](extensions/dehuman/chart-atelier/1.3.0/EXAMPLE.md) · 1 widgets · 1 acciones.

### Chat AI

![Vista ilustrativa de Chat AI](https://edgajuman.github.io/AK-Screen-Extensions/previews/chat-ai.png)

Asistente de consulta con varios proveedores. Claves en el almacén local del sistema; historial conservado al actualizar. Resumen del proyecto y vista previa opcionales. No modifica el editor.

[Ejemplos y componentes](extensions/dehuman/chat-ai/1.3.0/EXAMPLE.md) · 2 widgets · 2 acciones.

### Color Workbench

![Vista ilustrativa de Color Workbench](https://edgajuman.github.io/AK-Screen-Extensions/previews/color-workbench.png)

Accesos de color y acabado visual no destructivo para la selección actual.

[Ejemplos y componentes](extensions/dehuman/color-workbench/1.3.0/EXAMPLE.md) · 4 widgets · 13 acciones.

### Delivery Checklist

![Vista ilustrativa de Delivery Checklist](https://edgajuman.github.io/AK-Screen-Extensions/previews/delivery-checklist.png)

Control de entrega persistente: imagen, sonido, subtítulos, créditos, licencias y revisiones. Flota el checklist mientras revisas el programa.

[Ejemplos y componentes](extensions/dehuman/delivery-checklist/1.3.0/EXAMPLE.md) · 2 widgets · 0 acciones.

### Finish Tools

![Vista ilustrativa de Finish Tools](https://edgajuman.github.io/AK-Screen-Extensions/previews/finish-tools.png)

Grano de película y limpieza de fondo.

[Ejemplos y componentes](extensions/dehuman/finish-tools/1.3.0/EXAMPLE.md) · 3 widgets · 13 acciones.

### Motion Kit

![Vista ilustrativa de Motion Kit](https://edgajuman.github.io/AK-Screen-Extensions/previews/motion-kit.png)

Presets editables para capas 3D, cámara de composición, rótulos y acabados de movimiento.

[Ejemplos y componentes](extensions/dehuman/motion-kit/1.3.0/EXAMPLE.md) · 4 widgets · 10 acciones.

### Motion Titles

![Vista ilustrativa de Motion Titles](https://edgajuman.github.io/AK-Screen-Extensions/previews/motion-titles.png)

Títulos con degradados, contorno y animación por caracteres.

[Ejemplos y componentes](extensions/dehuman/motion-titles/1.3.0/EXAMPLE.md) · 4 widgets · 6 acciones.

### Notas de edición

![Vista ilustrativa de Notas de edición](https://edgajuman.github.io/AK-Screen-Extensions/previews/notes.png)

Notas persistentes, sin conexión ni permisos de proyecto.

[Ejemplos y componentes](extensions/dehuman/notes/1.3.0/EXAMPLE.md) · 4 widgets · 0 acciones.

### Checklist de tutorial

![Vista ilustrativa de Checklist de tutorial](https://edgajuman.github.io/AK-Screen-Extensions/previews/tutorial-checklist.png)

Preparación de tutoriales con estado persistente.

[Ejemplos y componentes](extensions/dehuman/tutorial-checklist/1.3.0/EXAMPLE.md) · 3 widgets · 0 acciones.

## Instalar

En el editor: **Herramientas → Extensiones → Actualizar catálogo**. Selecciona una versión, instala y revisa los permisos antes de activarla. También puedes importar un archivo `.akext`. Los paquetes locales no pueden reemplazar un ID verificado.

Cada usuario guarda los paquetes en su carpeta personal de AK Screen, dentro de `Extensiones/packages/ID/versión/`. Sus notas, chats y ajustes se conservan aparte, en `Extensiones/data/ID/`, incluso al actualizar o desinstalar. El borrado de datos es opcional y se confirma en el editor; afecta a todas las versiones de una misma extensión. Las claves de API se guardan en el almacén de credenciales del sistema operativo. No se incluyen claves en los paquetes.

## Colección inicial de Edgajuman

| Extensión | Tipo | Función |
|---|---|---|
| Notas de producción | Widget | Notas persistentes por usuario |
| Checklist de entrega | Widget | Revisar tareas antes de exportar |
| Títulos de movimiento | Herramienta | Insertar textos con animación |
| Cámara de composición | Herramienta | Ajustes de cámara 2D/3D |
| Acabado | Herramienta | Presets de efectos |
| WASM Lab | Widget y efecto | Contador persistente y acabado RGBA del SDK |
| Chat AI | Asistente | OpenAI, OpenRouter, Gemini, Claude y Grok; claves propias y contexto opcional |

Chat AI necesita conexión y una cuenta del proveedor elegido. Es un asistente de consulta: no ejecuta acciones en el editor. El resto de las extensiones iniciales no necesita red.

## Contribuir

1. Haz un **fork** de este repositorio.
2. Añade `extensions/autor/nombre/1.0.0/manifest.json` y sus componentes. Consulta [el SDK](sdk/README.md) y los ejemplos.
3. Cada archivo debe declararse con tamaño y SHA-256. Para una actualización, conserva el ID y añade una carpeta SemVer nueva; no alteres versiones publicadas.
4. Abre un **pull request** con descripción, capturas propias, permisos justificados, licencia y cambios. El mantenedor revisa el código y aprueba la publicación.
5. Tras aceptar el PR, el publicador genera los paquetes y firma el catálogo. El editor mostrará la nueva versión.

Los componentes WebAssembly no disponen de WASI, archivos, procesos, sockets ni bibliotecas nativas. La red pasa por un broker con dominios HTTPS exactos y permiso del usuario; las operaciones del proyecto pasan por comandos con deshacer. El aislamiento reduce riesgos y requiere mantenimiento continuo. No se aceptan DLL, ejecutables, scripts externos ni permisos innecesarios.

## Versiones y seguridad

El catálogo conserva versiones anteriores para poder elegirlas. Las actualizaciones no borran datos; se realiza una copia antes de activar otra versión. Una reversión no transforma automáticamente un esquema de datos nuevo en uno antiguo: tu extensión debe contemplar compatibilidad o migración.

Una versión insegura puede retirarse mediante `revoked.json`, usando `ID@versión`. Los usuarios recibirán su desactivación al consultar el catálogo. Para reportar un problema sin publicar detalles de explotación, utiliza un aviso privado de seguridad en GitHub.

MIT · Edgajuman
