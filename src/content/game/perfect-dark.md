---
title: "Perfect Dark"
description: "A Decompilation of Perfect Dark!"
releaseDate: "5/14/2024"
homepageLink: "https://github.com/fgsfdsfgs/perfect_dark"
githubLink: "https://github.com/fgsfdsfgs/perfect_dark"
tags: ["FPS", "Decompilation", "N64"]
version: "1.0.0"
lastUpdatedDate: "1/25/2025"
gameData: {
    downloadPath: "https://archive.org/download/z64-pal-gc-debug/",
    downloadFile: "pd.ntsc-final.z64",
}
supportedPlatforms: ["windows", "macos", "linux native"]
gamePlatforms: {
    linux: {
        downloadFile: "https://github.com/fgsfdsfgs/perfect_dark/releases/download/ci-dev-build/io.github.fgsfdsfgs.perfect_dark.flatpak",
        gameExecutable: "io.github.fgsfdsfgs.perfect_dark.flatpak",
        dataInstallToGameDir: true,
        dataInstallToHomeDir: false,
        dataInstallPath: "",
    },
    windows: {
        downloadFile: "https://github.com/fgsfdsfgs/perfect_dark/releases/download/ci-dev-build/pd-x86_64-windows.zip",
        gameExecutable: "pd.x86_64.exe",
        gameWorkingDirAppend: "pd-x86_64-windows/",
        dataInstallToGameDir: true,
        dataInstallToHomeDir: false,
        dataInstallPath: "/pd-x86_64-windows/data",
    },
    macos: {
        downloadFile: "https://github.com/fgsfdsfgs/perfect_dark/releases/download/ci-dev-build/pd-arm64-osx.tar.xz",
        gameExecutable: "pd.arm64",
        usesDMG: false,
        dataInstallToGameDir: true,
        dataInstallToHomeDir: false,
        dataInstallPath: "/data",
    }
}

---

Info goes here

** Requires Flatpak in Linux