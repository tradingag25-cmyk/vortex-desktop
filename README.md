# Vortex Desktop (Tauri)

Aplicación de escritorio de **Vortex — Sistema de Gestión Integral**, construida con
**Tauri v2**. Envuelve la interfaz de Vortex (`src/index.html`) en una app nativa e
instalable para Windows, con base para **auto-actualización** más adelante.

## Estructura

```
vortex-desktop/
├─ src/
│  └─ index.html          # la interfaz de Vortex (frontend)
├─ src-tauri/
│  ├─ tauri.conf.json     # configuración de la app (ventana, instalador, íconos)
│  ├─ Cargo.toml          # dependencias Rust
│  ├─ build.rs
│  ├─ capabilities/       # permisos
│  ├─ icons/              # íconos de la app
│  └─ src/                # main.rs / lib.rs
├─ .github/workflows/
│  └─ release.yml         # compila el instalador en la nube (GitHub Actions)
└─ package.json
```

## Cómo se compila (en la nube, no requiere Node/Rust local)

No se compila en tu PC. **GitHub Actions** lo hace en Windows y publica el instalador
`.exe` en la pestaña **Releases** del repositorio.

1. Sube este proyecto a un repo de GitHub.
2. Publica un **tag** `v1.0.0` (o corre el workflow manualmente desde **Actions**).
3. Al terminar (~5–10 min), el instalador aparece en **Releases**.
4. Descárgalo, instálalo en tu Windows y listo.

## Siguiente

- Conectar el **auto-updater** (firma de releases + API de actualizaciones de la
  plataforma) para que la app se actualice sola.
