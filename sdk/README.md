# AK Screen Extension SDK 1

Dos formatos compatibles: paquetes declarativos (widgets, herramientas, presets) y componentes WebAssembly (lógica y efectos RGBA). El código del editor no es necesario.

```sh
cargo build --release --target wasm32-unknown-unknown
wasm-tools component new target/wasm32-unknown-unknown/release/akscreen_component_example.wasm -o component.wasm
```

Incluye `component.wasm` en `manifest.json`, con tamaño y SHA-256 en `files`. Comprime el manifiesto y únicamente los archivos declarados en un `.akext`. Instala desde Herramientas → Extensiones → Importar. No uses WASI, imports nativos, DLL ni procesos.

`handle` recibe JSON con `event`, `state` y, con permiso, `project`. Eventos: `action`, `widget`, `project.changed`, `network.response`. Devuelve `state`, `widgets`, `commands`, `requests` o `message`. El estado es un objeto JSON: el host lo guarda fuera del paquete y lo entrega a versiones posteriores. No guardes API keys en `state`: usa controles `secret` y referencias `credential` del broker.

Las operaciones de proyecto necesitan `project_edit` y pasan por una lista de comandos admitidos con deshacer. La red necesita dominios HTTPS exactos aprobados; las claves se añaden en el host, sin entregarse al componente. `project_read` entrega un resumen acotado, sin rutas de archivos ni credenciales. Un fallo, import no permitido o exceso de instrucciones interrumpe el componente.

`render` recibe dimensiones, píxeles RGBA **premultiplicados en luz lineal**, JSON propio de esa instancia y tiempo en ticks (254016000000 por segundo). Debe devolver exactamente el mismo número de píxeles, finitos. No depende de red ni del estado global; vista previa y exportación usan el mismo contrato. Límite actual: 16 millones de componentes RGBA, 128 MB de memoria de guest y 20 millones de instrucciones por llamada. Para iniciar un efecto selecciona un clip, aplica Efecto de extensión y establece su ID, versión y parámetros JSON. Conserva la versión requerida por proyectos anteriores.

Publica actualizaciones manteniendo el ID y añadiendo una carpeta de versión SemVer nueva. Cada versión publicada es inmutable. AK Screen conserva notas, chats y ajustes en `Extensiones/data/ID`, realiza una copia antes de cambiar de versión y permite activar, desactivar o desinstalar cada paquete sin borrar esos datos. Los formatos de estado deben migrarse de forma compatible; una reversión no convierte automáticamente un esquema de datos creado por una versión posterior.

El aislamiento reduce el riesgo; no sustituye la revisión de código ni el mantenimiento del runtime. El catálogo oficial solo publica paquetes revisados y firmados.

MIT · Edgajuman
