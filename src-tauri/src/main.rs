// Evita que se abra una ventana de consola en Windows (modo release).
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    vortex_lib::run()
}
