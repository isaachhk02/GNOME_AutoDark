# GNOME Auto-Dark Mode

**English** | [Español](#versión-en-español)

Automatically change GNOME's light/dark theme based on the system time.

## Table of Contents

- [Features](#features)
- [Requirements](#requirements)
- [Installation](#installation)
- [Usage](#usage)
- [Configuration](#configuration)
- [Uninstallation](#uninstallation)
- [Troubleshooting](#troubleshooting)

## Features

- ✨ Automatically switches between light and dark mode based on time
- 🕐 Customizable time schedules
- 🚀 Lightweight and fast (written in Rust)
- 🔧 Simple systemd service integration
- 💻 Works with GNOME Desktop

## Requirements

- GNOME 40 or higher
- Rust toolchain (for building from source)
- Make
- systemd user services enabled

## Installation

### Build from Source

1. Clone the repository:
```bash
git clone https://github.com/isaachhk02/GNOME_AutoDark.git
cd GNOME_AutoDark
```

2. Build the project:
```bash
make
```

3. Install the binary and systemd services:
```bash
make install
```

## Usage

### Enable the Service

Start the GNOME Auto-Dark service and timer:

```bash
systemctl --user enable gnome-autodark.service
systemctl --user enable gnome-autodark.timer
systemctl --user start gnome-autodark.timer
```

### Check Service Status

```bash
systemctl --user status gnome-autodark.timer
systemctl --user status gnome-autodark.service
```

### View Logs

```bash
journalctl --user -u gnome-autodark.service -f
```

## Configuration

Edit the systemd timer file to customize the schedule:

```bash
systemctl --user edit gnome-autodark.timer
```

## Disable the Service

To temporarily disable the service:

```bash
systemctl --user disable gnome-autodark.timer
systemctl --user disable gnome-autodark.service
```

## Uninstallation

1. Disable and stop the services:
```bash
systemctl --user disable --now gnome-autodark.service
systemctl --user disable --now gnome-autodark.timer
```

2. Remove the installed files:
```bash
sudo rm -v /etc/systemd/user/gnome-autodark.service
sudo rm -v /etc/systemd/user/gnome-autodark.timer
sudo rm -v /usr/bin/gnome-autodark
```

3. Reload systemd user services:
```bash
systemctl --user daemon-reload
```

## Troubleshooting

**Service not starting?**
- Check if systemd user services are enabled: `systemctl --user status`
- View logs: `journalctl --user -u gnome-autodark.service`

**Theme not changing?**
- Verify GNOME is running and the current user is the same one running the service
- Check the service timer: `systemctl --user status gnome-autodark.timer`

---

# Versión en Español

Cambia automáticamente el tema claro/oscuro de GNOME según la hora del sistema.

## Tabla de Contenidos

- [Características](#características)
- [Requisitos](#requisitos)
- [Instalación](#instalación)
- [Uso](#uso)
- [Configuración](#configuración)
- [Desinstalación](#desinstalación)
- [Solución de Problemas](#solución-de-problemas)

## Características

- ✨ Cambia automáticamente entre modo claro y oscuro según la hora
- 🕐 Horarios personalizables
- 🚀 Ligero y rápido (escrito en Rust)
- 🔧 Integración simple con systemd
- 💻 Funciona con GNOME Desktop

## Requisitos

- GNOME 40 o superior
- Cadena de herramientas Rust (para compilar desde la fuente)
- Make
- Servicios de usuario de systemd habilitados

## Instalación

### Compilar desde la Fuente

1. Clona el repositorio:
```bash
git clone https://github.com/isaachhk02/GNOME_AutoDark.git
cd GNOME_AutoDark
```

2. Compila el proyecto:
```bash
make
```

3. Instala el binario y los servicios de systemd:
```bash
make install
```

## Uso

### Habilitar el Servicio

Inicia el servicio y el temporizador de GNOME Auto-Dark:

```bash
systemctl --user enable gnome-autodark.service
systemctl --user enable gnome-autodark.timer
systemctl --user start gnome-autodark.timer
```

### Verificar Estado del Servicio

```bash
systemctl --user status gnome-autodark.timer
systemctl --user status gnome-autodark.service
```

### Ver Registros

```bash
journalctl --user -u gnome-autodark.service -f
```

## Configuración

Edita el archivo del temporizador de systemd para personalizar el horario:

```bash
systemctl --user edit gnome-autodark.timer
```

## Deshabilitar el Servicio

Para desactivar temporalmente el servicio:

```bash
systemctl --user disable gnome-autodark.timer
systemctl --user disable gnome-autodark.service
```

## Desinstalación

1. Deshabilita y detén los servicios:
```bash
systemctl --user disable --now gnome-autodark.service
systemctl --user disable --now gnome-autodark.timer
```

2. Elimina los archivos instalados:
```bash
sudo rm -v /etc/systemd/user/gnome-autodark.service
sudo rm -v /etc/systemd/user/gnome-autodark.timer
sudo rm -v /usr/bin/gnome-autodark
```

3. Recarga los servicios de usuario de systemd:
```bash
systemctl --user daemon-reload
```

## Solución de Problemas

**¿El servicio no inicia?**
- Verifica que los servicios de usuario de systemd estén habilitados: `systemctl --user status`
- Ver registros: `journalctl --user -u gnome-autodark.service`

**¿El tema no cambia?**
- Verifica que GNOME esté ejecutándose y que el usuario actual sea el mismo que ejecuta el servicio
- Comprueba el temporizador del servicio: `systemctl --user status gnome-autodark.timer`

---

## License

This project is licensed under the GPLv3

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.
